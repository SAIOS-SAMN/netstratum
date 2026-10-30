#!/usr/bin/env python3
"""observer.py — the out-of-game observatory, traversable.

THE TOPOLOGY: a CIVILIZATION (registry identity, spans worlds) settles
COLONIES on PLANETS inside a WORLD (one clock, one chronicle, one
receipt chain). Worlds sit on a 3-torus starchart; galaxies are lazy
regions with a center and reach. The observatory renders VIEWS over
that topology — a universe dashboard by default, tables and
drill-downs by subcommand.

READ-ONLY BY CONSTRUCTION — opens files for reading, never writes.
Standard library only. Exact fractions in, exact fractions out.

AUTHORITY (docs/JSON_DISPLAY_ONLY.md): material fields (banks, famine,
zones, elements, STAMP, labor) decode from **state.tlv** only. Rows
without TLV are omitted. No JSON carriers are read. Chain presence
comes from **chronicle.tlv**. Registry / chart / universe are `.civ`
and `.card` TLV only.

Usage:
  python3 tools/observer.py                    # universe dashboard (default)
  python3 tools/observer.py --watch 2          # refresh every 2s
  python3 tools/observer.py civs [--tier T0]   # civilizations, by tier
  python3 tools/observer.py worlds             # settled planets table
  python3 tools/observer.py world <name>       # one world, whole
  python3 tools/observer.py civ <slug>         # one line, whole
  python3 tools/observer.py tiers              # the union board T0→T9
  python3 tools/observer.py stress             # condition roll-up only
  python3 tools/observer.py techs              # crystallized nodes
  python3 tools/observer.py chart              # the star chart
  python3 tools/observer.py market             # resting offers
  python3 tools/observer.py routes             # E3.8 element lanes (chronicle)
  python3 tools/observer.py refineries         # E3.8 element-refinery zones
  python3 tools/observer.py arc                # P8.2 trajectory · hunger · rail · labor
  python3 tools/observer.py ruins              # P8.4 closed-ruins history
  python3 tools/observer.py <view> --watch 2   # any view, refreshed
  # Live worlds only by default; --closed- ruins stay hidden (no cinder spam).
  # Use `ruins` for the closed archive history view.

Chain check: 'ok' means every exported receipt's parent digest equals
its predecessor's digest with contiguous sequence numbers — splicing,
deletion, or reordering reads 'BROKEN'. (Full BLAKE3 recomputation
lives in-game via `verify`/`replay`; the chronicle is the authority,
the lens is its write-only rendering.)
"""

import argparse
import os
import sys
import time
from fractions import Fraction
from pathlib import Path

# The seven resources, in wire-code order (cfe::dop::Resource). Crystal
# is code 7 — the resonance crystal, and the resource the PoUW mint pays
# into (`DiscoveryMinted`, tag 86, banks Resource::Crystal). It was
# missing here, so the one economic output of verified work was invisible
# on every board this file renders.
RESOURCES = ["iron", "biomass", "alloy", "diamond", "joule", "sink", "crystal"]
STAR_NAMES = {1: "O", 2: "B", 3: "A", 4: "F", 5: "G", 6: "K", 7: "M",
              8: "bd", 9: "wd", 10: "rg", 11: "ns", 12: "wr", 13: "lwh"}
# 12_UNION §2a + 30_OFFICES (Wave-1): T4–T9 are deeper economic offices,
# not ranks above the progenitors. Keep in lockstep with
# ledger::registry::tier_of — one reading, two surfaces.
TIER_OF_STATION = {
    "progenitor": "T0",
    "watcher": "T1",
    "observer": "T1",
    "sovereign": "T2",
    # T3 = absence of a seat (handled in tier_of), never a station word.
    "caravan": "T4",
    "crucible": "T5",
    "exchange": "T6",
    "assay": "T7",
    "vault": "T8",
    "anchor": "T9",
}
TIER_TITLES = {
    "T0": "T0 — THE BOARD-MAKERS (progenitors)",
    "T1": "T1 — THE ARCHITECT OBSERVERS (watchers · observers)",
    "T2": "T2 — THE RECORD KEEPERS (sovereigns)",
    "T3": "T3 — THE BIOSPHERE NODES (the common, lawful state)",
    "T4": "T4 — CARAVAN (trade routes)",
    "T5": "T5 — CRUCIBLE (elemental refineries)",
    "T6": "T6 — EXCHANGE (STAMP market-making)",
    "T7": "T7 — ASSAY (element identity court)",
    "T8": "T8 — VAULT (custody and collateral)",
    "T9": "T9 — ANCHOR (external settlement seats)",
    "T?": "T? — UNREADABLE SEAT (not the common — refuse, never default)",
}
# Display order: top of the board down through deeper economic functions.
ALL_TIERS = ("T0", "T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "T9")


# ── exact arithmetic ────────────────────────────────────────────────

def frac(text):
    if isinstance(text, str) and "/" in text:
        numer, denom = text.split("/", 1)
        return Fraction(int(numer), int(denom))
    return Fraction(int(text))


def show(fraction):
    if fraction is None:
        return "—"
    if fraction.denominator == 1:
        return str(fraction.numerator)
    return f"{fraction.numerator}/{fraction.denominator}"


def show_board(fraction):
    """Compact bank figure for the watch board.

    Exact fractions stay exact when short. Very large reduced forms
    (long-run demurrage leftovers) floor to whole units with a trailing
    '+' so the dashboard stays readable without inventing floats.
    """
    if fraction is None:
        return "—"
    if fraction < 0:
        return "-" + show_board(-fraction)
    if fraction.denominator == 1:
        return str(fraction.numerator)
    if fraction.numerator < 1000 and fraction.denominator <= 64:
        return show(fraction)
    whole = fraction.numerator // fraction.denominator
    if fraction.numerator % fraction.denominator == 0:
        return str(whole)
    if whole == 0:
        return show(fraction) if fraction.denominator <= 256 else "<1"
    return f"{whole}+"


# ── the house wire (state.tlv; landed as "MIRROR-TLV"): the committed orb decoded native ──
# NS-1 TLV: tag u8 ‖ LE32(len) ‖ value; unknown tags skip (SEAL A).
# Ratio: sign u8 ‖ LE32(len) ‖ numer be ‖ LE32(len) ‖ denom be.

def tlv_records(data):
    at = 0
    while at + 5 <= len(data):
        tag = data[at]
        length = int.from_bytes(data[at + 1:at + 5], "little")
        value = data[at + 5:at + 5 + length]
        if len(value) < length:
            return  # torn tail — stop, never guess
        yield tag, value
        at += 5 + length


def tlv_ratio(value, at=0):
    sign = value[at]
    at += 1
    parts = []
    for _ in range(2):
        length = int.from_bytes(value[at:at + 4], "little")
        at += 4
        parts.append(int.from_bytes(value[at:at + length], "big") if length else 0)
        at += length
    numer, denom = parts
    if denom == 0:
        return Fraction(0), at
    return Fraction(-numer if sign == 1 else numer, denom), at


# The in-value codes (dop.rs: Resource = 1..=7). Crystal rides the
# existing banked entry (tag 18) — no new TLV tag — so a decoder that
# stops at 6 silently drops crystal banks instead of failing.
RESOURCE_NAMES = {1: "iron", 2: "biomass", 3: "alloy", 4: "diamond",
                  5: "joule", 6: "sink", 7: "crystal"}


def element_name(element_id: bytes) -> str:
    """Port of cfe::dop::element_name — family form + voice from hash.

    Family is read from byte[1] against a default G-star spine when the
    star is unknown at this layer (the board names matter; the assay
    already bound richness to the real class at mint time).
    """
    # G/F/K spine bands [2,1,1,1] — the common board default.
    bands = (2, 1, 1, 1)
    total = sum(bands) or 1
    pick = element_id[1] % total
    family = 4
    for at, weight in enumerate(bands):
        if pick < weight:
            family = at + 1
            break
        pick -= weight
    form = {1: "lode", 2: "marrow", 3: "rime"}.get(family, "veil")
    voices = (
        "adamant", "pale", "keening", "vaulted", "ember", "tidal",
        "hollow", "woven", "still", "spiral", "murmured", "resonant",
    )
    voice = voices[element_id[0] % len(voices)]
    return f"{voice} {form}"


def decode_orb_tlv(path):
    """The committed orb from state.tlv — banked, famine, zones,
    vented, elements, name — decoded from the canonical bytes.

    Returns None when no carrier stands. **state.tlv is authority** for
    material fields — no other export is consulted.
    """
    try:
        data = path.read_bytes()
    except OSError:
        return None
    orb = {
        "banked": {}, "famine": [], "zones": 0,
        "vented": Fraction(0), "elements": [],
        "stamp": Fraction(0), "name": None, "orb_id": None,
        "habitats": 0,
        "refineries": [],
        "zone_kinds": [],
    }
    for tag, value in tlv_records(data):
        if tag == 16 and len(value) >= 8:
            # Orb header: orb_id LE64 ‖ name bytes
            orb["orb_id"] = int.from_bytes(value[:8], "little")
            try:
                orb["name"] = value[8:].decode("utf-8", errors="replace")
            except Exception:
                orb["name"] = None
        elif tag == 17 and len(value) >= 3:
            # tile u16 LE ‖ kind u8
            orb["zones"] += 1
            tile = int.from_bytes(value[:2], "little")
            kind = value[2]
            # ZoneKind::Habitat = 2 · ElementRefinery = 9 (dop wire)
            if kind == 2:
                orb["habitats"] += 1
            if kind == 9:
                orb.setdefault("refineries", []).append(tile)
            orb.setdefault("zone_kinds", []).append((tile, kind))
        elif tag == 18 and len(value) > 1:
            name = RESOURCE_NAMES.get(value[0])
            if name:
                orb["banked"][name], _ = tlv_ratio(value, 1)
        elif tag == 19 and value:
            name = RESOURCE_NAMES.get(value[0])
            if name:
                orb["famine"].append(name)
        elif tag == 28:
            orb["vented"], _ = tlv_ratio(value)
        elif tag == 72 and len(value) >= 32:
            # element_id[32] ‖ ratio — the open matter vocabulary
            # (14_ALMANAC). Closed Resource enum is untouched.
            element_id = bytes(value[:32])
            try:
                amount, _ = tlv_ratio(value, 32)
            except (IndexError, ValueError):
                continue
            orb["elements"].append((element_id, amount))
        elif tag == 73:
            # STAMP-1 unit-of-account balance (TLV 73 on ColonyOrb).
            try:
                orb["stamp"], _ = tlv_ratio(value)
            except (IndexError, ValueError):
                pass
        # every other tag: skipped whole (SEAL A) — future fields
        # never break this reader.
    return orb


def yield_amount(tlv_hex):
    """The yield's amount from the receipt's CANONICAL bytes.

    Every exported receipt carries `tlv` — hex of the event's wire
    form (lens/render.rs: `hex(&receipt.event.wire_bytes())`). For
    YieldCollected that is tag 3, laid out by `dop.rs::amount_layout`
    as orb u64 LE (8) ‖ resource u8 (1) ‖ ratio. Reading it here is
    the MIRROR-TLV pattern applied to a receipt instead of an orb:
    the wire is the authority, the `detail` prose is a rendering.

    None when no tag-3 record stands — the caller then adds nothing
    rather than guessing a zero into a total."""
    try:
        data = bytes.fromhex(tlv_hex)
    except ValueError:
        return None
    for tag, value in tlv_records(data):
        if tag == 3 and len(value) > 9:
            return tlv_ratio(value, 9)[0]
    return None


# ── readers (the lens's data files — data, never prose) ─────────────

def chain_linked_tlv(world_dir):
    """Chain presence from chronicle.tlv (engine verifies; board trusts file)."""
    tlv = Path(world_dir) / "chronicle.tlv"
    if not tlv.exists():
        return False, 0
    try:
        size = tlv.stat().st_size
    except OSError:
        return False, 0
    # Approximate receipt count for display only (not SEAL B).
    return True, max(1, size // 80)


# Chronicle event tags (cfe::dop / vrp) — display scan only.
_EVT_ZONE_BUILT = 2
_EVT_ZONE_RAZED = 7
_EVT_ROUTE_LAPSED = 80
_EVT_ELEMENT_ROUTE = 110
_RECEIPT_FRAME = 20
_ZONE_ELEMENT_REFINERY = 9
_RAIL_WORD = {1: "R1", 2: "R2", 3: "R3", 4: "R4"}


def iter_chronicle_events(world_dir):
    """Yield (event_tag, event_value) from chronicle.tlv receipt frames.

    Skips torn/unknown frames (SEAL A). Not a full SEAL B verify — the
    engine owns that; the board only displays route/refinery history.
    """
    path = Path(world_dir) / "chronicle.tlv"
    if not path.exists():
        return
    try:
        data = path.read_bytes()
    except OSError:
        return
    for tag, value in tlv_records(data):
        if tag != _RECEIPT_FRAME or len(value) < 84:
            continue
        count = int.from_bytes(value[80:84], "little")
        at = 84 + count * 8
        if at > len(value):
            continue
        for etag, eval_ in tlv_records(value[at:]):
            yield etag, eval_
            break  # one event per receipt


def load_element_routes(world_dir):
    """Live element routes: charter (110) minus lapse (80), by deal id."""
    routes = {}
    for etag, value in iter_chronicle_events(world_dir):
        if etag == _EVT_ELEMENT_ROUTE and len(value) >= 40:
            deal = int.from_bytes(value[:8], "little")
            element = bytes(value[8:40])
            try:
                eta, used = tlv_ratio(value, 40)
            except (IndexError, ValueError):
                continue
            rail_at = 40 + used
            rail = value[rail_at] if rail_at < len(value) else 0
            routes[deal] = {
                "deal": deal,
                "element": element,
                "eta": eta,
                "rail": rail,
            }
        elif etag == _EVT_ROUTE_LAPSED and len(value) >= 8:
            deal = int.from_bytes(value[:8], "little")
            routes.pop(deal, None)
    return list(routes.values())


def load_refineries_from_chronicle(world_dir):
    """ElementRefinery zones still standing (ZoneBuilt kind 9 − ZoneRazed)."""
    # key (orb_id, tile) -> True
    standing = {}
    for etag, value in iter_chronicle_events(world_dir):
        if etag == _EVT_ZONE_BUILT and len(value) >= 11:
            orb_id = int.from_bytes(value[:8], "little")
            tile = int.from_bytes(value[8:10], "little")
            kind = value[10]
            if kind == _ZONE_ELEMENT_REFINERY:
                standing[(orb_id, tile)] = True
        elif etag == _EVT_ZONE_RAZED and len(value) >= 10:
            orb_id = int.from_bytes(value[:8], "little")
            tile = int.from_bytes(value[8:10], "little")
            standing.pop((orb_id, tile), None)
    return [
        {"orb_id": orb, "tile": tile}
        for (orb, tile) in sorted(standing)
    ]


def load_refineries_from_state(world_dir):
    """Live mirror: ElementRefinery tiles on state.tlv (preferred when present)."""
    out = []
    planets = Path(world_dir) / "planets"
    if not planets.is_dir():
        return out
    for planet_dir in sorted(p for p in planets.iterdir() if p.is_dir()):
        wire = decode_orb_tlv(planet_dir / "state.tlv")
        if not wire:
            continue
        for tile in wire.get("refineries") or []:
            out.append({
                "orb_id": wire.get("orb_id"),
                "tile": tile,
                "planet": wire.get("name") or planet_dir.name,
            })
    return out


def _parse_civ_tlv(data: bytes):
    """Parse registry .civ TLV (ledger::card::civ tags)."""
    card = {
        "name": "",
        "sigil": "",
        "species": "",
        "clade": "",
        "motto": "",
        "station": None,
        "founder": "",
        "axes": {},
    }
    at = 0
    while at + 5 <= len(data):
        tag = data[at]
        ln = int.from_bytes(data[at + 1 : at + 5], "little")
        val = data[at + 5 : at + 5 + ln]
        if len(val) < ln:
            break
        try:
            s = val.decode("utf-8")
        except Exception:
            s = ""
        if tag == 1:
            card["name"] = s
        elif tag == 2:
            card["sigil"] = s
        elif tag == 3:
            card["species"] = s
        elif tag == 4:
            card["clade"] = s
        elif tag == 5:
            card["motto"] = s
        elif tag == 6:
            card["axes"]["disposition"] = s
        elif tag == 7:
            card["axes"]["metabolism"] = s
        elif tag == 8:
            card["axes"]["perception"] = s
        elif tag == 9:
            card["founder"] = s
        elif tag == 11:
            card["station"] = s or None
        at += 5 + ln
    if not card["sigil"]:
        return None
    return card


def load_registry(var_root):
    registry = {}
    reg_dir = Path(var_root) / "registry"
    if not reg_dir.is_dir():
        return registry
    for path in sorted(reg_dir.glob("*.civ")):
        try:
            card = _parse_civ_tlv(path.read_bytes())
            if not card:
                continue
            registry[card.get("sigil", path.stem)] = card
        except OSError:
            continue
    return registry


# Engine-only or retired names that must not appear as peoples.
BOARD_EXCLUDE_CIVS = frozenset({"proofer"})
BOARD_EXCLUDE_WORLDS = frozenset({"visible-proof", "proofer-hearth"})


def ensure_registry_coverage(registry, rows):
    """Synthesize a card for any civilization living on a world without
    a registry file so the board never shows a blank species.

    Engine fixtures (proofer / visible-proof) are excluded entirely —
    they are not peoples of the universe.
    """
    for row in rows:
        slug = row.get("civilization") or "?"
        if slug in registry or slug == "?" or slug in BOARD_EXCLUDE_CIVS:
            continue
        if row.get("world") in BOARD_EXCLUDE_WORLDS:
            continue
        registry[slug] = {
            "name": slug,
            "sigil": slug,
            "species": "unregistered line",
            "clade": "—",
            "station": None,
            "founder": "unknown",
            "axes": {},
            "_synthetic": True,
        }
    # Drop engine fixtures if a card lingered on disk.
    for slug in BOARD_EXCLUDE_CIVS:
        registry.pop(slug, None)
    return registry


def tier_of(card):
    """12_UNION §2a + 30_OFFICES: tier is a derived reading, never stored.

    T3 is the ABSENCE of a seat. An unknown station word must NOT fall
    through to T3 (that is the hole a gate would leak through) — it
    renders as T? so the board refuses to launder an unreadable office
    into the common.
    """
    station = card.get("station") if card else None
    if not station:
        return "T3"
    return TIER_OF_STATION.get(station, "T?")


CREED_CONCEPTS = [
    # (concept, the receipted tendency it reads) — order breaks ties.
    # The hardship concept holds precedence 2 by operator ruling
    # (2026-07-30, upgraded two tiers): endurance outranks commerce
    # and rupture in a tied reading. Its NAME stands under operator
    # review (the crucible/Furnace rename law) — one word swaps it.
    ("doctrine", {"tech_discovered", "tech_licensed"}),
    ("hardship", {"famine_struck", "overheat_struck", "vent_settled",
                  "demurrage_settled"}),
    ("communion", {"resource_transferred", "overflow_absorbed",
                   "bond_sealed", "bond_settled"}),
    ("schism", {"covenant_slashed", "council_closed", "diverged",
                "absorption_lapsed"}),
    ("passage", {"aperture_sent", "aperture_received", "gauge_shifted",
                 "gauge_shifted_at", "era_annexed", "galaxy_observed",
                 "line_succeeded", "world_charted"}),
]
CREED_VOICES = ["quiet", "burning", "wandering", "deep", "risen", "iron",
                "tide-born", "unbowed", "patient", "hollow", "radiant",
                "grafted", "sleepless", "wintering", "manifold", "first"]


# The hardship gradient (operator-ruled 2026-07-30): the endured
# weight selects the word — tempering supports the base, anneal aligns
# the middle, athanor crowns the deep end. Floors are placeholder
# parameters; "crucible" is retired from the creed vocabulary.
HARDSHIP_GRADIENT = [(24, "athanor"), (8, "anneal"), (0, "tempering")]


def creed_of(events, digest_hex):
    """14_CREEDS §4 (CREED-1): the derived creed — never stored, never
    enumerated. CONCEPT reads the line's dominant receipted tendency
    (list order breaks ties, deterministic); VOICE draws from the
    chronicle's terminal digest — the 10 §2 naming law applied to
    belief. New creeds arise as chronicles diverge; a line with no
    deeds yet walks the unwritten way."""
    best, best_n = None, 0
    for concept, names in CREED_CONCEPTS:
        n = sum(1 for e in events if e.get("event") in names)
        if n > best_n:
            best, best_n = concept, n
    if best is None:
        return "the unwritten way"
    if best == "hardship":
        best = next(label for floor, label in HARDSHIP_GRADIENT if best_n >= floor)
    try:
        voice = CREED_VOICES[int(digest_hex[:2], 16) % len(CREED_VOICES)]
    except (ValueError, IndexError):
        voice = "quiet"
    return f"the {voice} {best}"


def function_of(card, agg):
    """12_UNION §4: the FUNCTION reading — derived at the observer from
    metabolism + observed niche (zones, banks, solves), never stored,
    confers no standing. First match wins (FUNCTION-1 v1 order)."""
    metabolism = card.get("axes", {}).get("metabolism", "")
    kinds = agg.get("zone_kinds", set())
    if metabolism == "plasmic":
        return "flux-harvester"
    if metabolism == "mineral":
        return "miner"
    if kinds & {"smelter", "synthesizer", "resonator"}:
        return "weaver"
    if "cryo-array" in kinds or agg.get("sink", Fraction(0)) > 0:
        return "sink-banker"
    if "encasement" in kinds:
        return "flux-harvester"
    if agg.get("puzzles", 0) > 0:
        return "prover"
    return "sky-condenser"


def is_closed_ruin(dir_name: str) -> bool:
    """Chronicle rotation / divergence ruins: `<base>--closed-<digest8>`.

    Archives on disk — never a second live solar system. Listing them
    on the board floods the SOLAR SYSTEMS table with cinder clones.
    """
    return "--closed-" in dir_name


def world_dirs(var_root, *, include_closed=False):
    """Live world directories only (default). Closed ruins opt-in."""
    worlds_dir = Path(var_root) / "worlds"
    if not worlds_dir.is_dir():
        return []
    out = []
    for p in sorted(worlds_dir.iterdir()):
        if not p.is_dir() or p.name in BOARD_EXCLUDE_WORLDS:
            continue
        if not include_closed and is_closed_ruin(p.name):
            continue
        out.append(p)
    return out


def count_closed_ruins(var_root):
    worlds_dir = Path(var_root) / "worlds"
    if not worlds_dir.is_dir():
        return 0
    return sum(
        1 for p in worlds_dir.iterdir()
        if p.is_dir() and is_closed_ruin(p.name)
    )


def world_card_sigil(world_dir):
    """The world's OWN civilization sigil, from `world.card` tag 30 — the
    AUTHORITATIVE world->civ link. World names are flavourful and do NOT
    prefix their sigil ('forge-reach' belongs to 'forgeborn', 'tun-reach'
    to 'tunborn'), so the dir name alone cannot identify the line; the
    scribe writes the sigil into the card exactly so it can. Returns None
    when the card is absent or torn (then the caller falls back to the
    dir-name guess)."""
    path = Path(world_dir) / "world.card"
    try:
        data = path.read_bytes()
    except OSError:
        return None
    at = 0
    while at + 5 <= len(data):
        tag = data[at]
        ln = int.from_bytes(data[at + 1 : at + 5], "little")
        val = data[at + 5 : at + 5 + ln]
        if len(val) < ln:
            break
        if tag == 30:  # scribe: put_str(30, civilization sigil)
            try:
                return val.decode("utf-8")
            except Exception:
                return None
        at += 5 + ln
    return None


def world_facts(world_dir):
    """Everything one world carries, gathered once.

    **Authority:** `state.tlv` (orb wire) + `chronicle.tlv` presence.
    Planet dir name + world dir name supply identity labels.
    """
    linked, receipts = chain_linked_tlv(world_dir)
    consumed = Fraction(0)  # yield total requires receipt stream; optional later
    events, terminal_digest = [], ""
    puzzles, depth = 0, 0
    solves = world_dir / "solves"
    if solves.is_dir():
        for path in solves.iterdir():
            if path.is_file() and path.suffix == ".sol":
                puzzles += 1
                # depth tag 4 = u64 LE in .sol payload
                try:
                    data = path.read_bytes()
                    at = 0
                    while at + 5 <= len(data):
                        tag = data[at]
                        ln = int.from_bytes(data[at + 1 : at + 5], "little")
                        val = data[at + 5 : at + 5 + ln]
                        if tag == 4 and len(val) == 8:
                            depth = max(depth, int.from_bytes(val, "little"))
                        at += 5 + ln
                except OSError:
                    pass
    planets = []
    planets_dir = world_dir / "planets"
    world_name = world_dir.name
    # Authoritative: the sigil the scribe wrote into world.card. The
    # dir-name split is only a fallback for a torn/absent card — it is
    # wrong for every flavourfully-named world (forge-reach → forgeborn).
    civ_guess = world_card_sigil(world_dir) or (
        world_name.split("-")[0] if world_name else "?"
    )
    if planets_dir.is_dir():
        for planet_dir in sorted(p for p in planets_dir.iterdir() if p.is_dir()):
            wire = decode_orb_tlv(planet_dir / "state.tlv")
            if wire is None:
                continue
            planet_label = wire.get("name") or planet_dir.name
            banked = {k: show(v) for k, v in wire["banked"].items()}
            famine = list(wire["famine"])
            traj = "decline" if famine else "growth"
            labor = wire.get("habitats", 0)
            state = {
                "civilization": civ_guess,
                "world": world_name,
                "planet": planet_label,
                "colony": planet_label,
                "clock": receipts,
                "banked": banked,
                "famine": famine,
                "zones": [],
                "_wire_zones": wire["zones"],
                "entropy_vented": show(wire["vented"]),
                "elements": [
                    {"id": eid.hex(), "name": element_name(eid), "amount": show(amt)}
                    for eid, amt in wire["elements"]
                ],
                "stamp": show(wire.get("stamp", Fraction(0))),
                "trajectory": traj,
                "hunger": [],
                "rail": "R1",
                "rail_routes": 0,
                "labor": labor,
                "habitats": labor,
                "ap_available": "—",
                "ap_capacity": "—",
                "star_class": None,
            }
            planets.append(state)
    pending = 0
    inbox = world_dir / "inbox"
    if inbox.is_dir():
        pending = sum(
            1 for p in inbox.iterdir() if p.is_file() and p.suffix == ".dir"
        )
    # ── the mind's own GROWTH (this version): tally the cognition deeds
    # from the chronicle. A line's intelligence shows here — the words it
    # minted (166, its own vocabulary), whether it has cleaved (43, spawned
    # a peer branch on reaching mastery), and the shared truths it reached
    # by converging with other independent lines (177, cross-line
    # consilience). These are what "watch the civs grow" means now.
    words, cleaved, axioms = 0, False, 0
    for etag, _eval in iter_chronicle_events(world_dir):
        if etag == 166:
            words += 1
        elif etag == 43:
            cleaved = True
        elif etag == 177:
            axioms += 1
    return {
        "name": world_dir.name, "linked": linked, "receipts": receipts,
        "events": events, "consumed": consumed, "puzzles": puzzles,
        "depth": depth, "planets": planets, "pending": pending,
        "digest": terminal_digest,
        "words": words, "cleaved": cleaved, "axioms": axioms,
    }


def read_all(var_root):
    """One row per settled planet + the per-world facts, once."""
    worlds, rows = {}, []
    for world_dir in world_dirs(var_root):
        facts = world_facts(world_dir)
        worlds[facts["name"]] = facts
        for state in facts["planets"]:
            banked = state.get("banked", {})
            elements = state.get("elements") or []
            element_total = Fraction(0)
            element_kinds = 0
            for entry in elements:
                try:
                    element_total += frac(str(entry.get("amount", "0")))
                    element_kinds += 1
                except (ValueError, TypeError):
                    pass
            try:
                stamp_amt = frac(str(state.get("stamp", "0")))
            except (ValueError, TypeError):
                stamp_amt = Fraction(0)
            hunger = state.get("hunger") or []
            if isinstance(hunger, str):
                hunger = [hunger]
            row = {
                "civilization": state.get("civilization", "?"),
                "world": state.get("world", facts["name"]),
                "planet": state.get("planet", "?"),
                "clock": state.get("clock", 0),
                "zones": state.get("_wire_zones", len(state.get("zones", []))),
                "zone_kinds": {z.get("kind") for z in state.get("zones", [])},
                "famine": state.get("famine", []),
                "puzzles": facts["puzzles"], "depth": facts["depth"],
                "consumed": facts["consumed"], "linked": facts["linked"],
                "star": state.get("star_class"),
                "vented": state.get("entropy_vented", "0"),
                "elements": elements,
                "element_kinds": element_kinds,
                "element_total": element_total,
                "stamp": stamp_amt,
                # P8.2 trajectory · hunger · rail (from state uplink).
                "trajectory": state.get("trajectory", "—"),
                "hunger": ",".join(hunger) if hunger else "—",
                "rail": state.get("rail", "—"),
                "rail_routes": state.get("rail_routes", 0),
                # P8.3 labor.
                "labor": state.get("labor", 0),
                "habitats": state.get("habitats", 0),
                "ap": (
                    f"{state.get('ap_available', '—')}/"
                    f"{state.get('ap_capacity', '—')}"
                ),
            }
            for resource in RESOURCES:
                row[resource] = frac(banked[resource]) if resource in banked else Fraction(0)
            row["intelligence"] = (
                Fraction(facts["puzzles"]) / facts["consumed"]
                if facts["consumed"] > 0 else None)
            # Engine fixtures are not peoples — never surface them.
            if row["civilization"] in BOARD_EXCLUDE_CIVS:
                continue
            if row["world"] in BOARD_EXCLUDE_WORLDS:
                continue
            rows.append(row)
    return rows, worlds


def _iter_tlv(data: bytes):
    at = 0
    while at + 5 <= len(data):
        tag = data[at]
        ln = int.from_bytes(data[at + 1 : at + 5], "little")
        val = data[at + 5 : at + 5 + ln]
        if len(val) < ln:
            break
        yield tag, val
        at += 5 + ln


def load_starchart(var_root):
    """starchart.card TLV (WORLD/X/Y/Z/G/END)."""
    chart = {}
    path = Path(var_root) / "worlds" / "starchart.card"
    if not path.exists():
        return chart
    try:
        data = path.read_bytes()
    except OSError:
        return chart
    cur = None
    coords = ["0", "0", "0", "0"]
    for tag, val in _iter_tlv(data):
        try:
            s = val.decode("utf-8")
        except Exception:
            s = ""
        if tag == 1:  # WORLD
            if cur is not None:
                chart[cur] = (
                    frac(coords[0]),
                    frac(coords[1]),
                    frac(coords[2]),
                    frac(coords[3]),
                )
            cur = s
            coords = ["0", "0", "0", "0"]
        elif tag == 2:
            coords[0] = s
        elif tag == 3:
            coords[1] = s
        elif tag == 4:
            coords[2] = s
        elif tag == 5:
            coords[3] = s
        elif tag == 0 and cur is not None:  # END
            chart[cur] = (
                frac(coords[0]),
                frac(coords[1]),
                frac(coords[2]),
                frac(coords[3]),
            )
            cur = None
    if cur is not None:
        chart[cur] = (
            frac(coords[0]),
            frac(coords[1]),
            frac(coords[2]),
            frac(coords[3]),
        )
    return chart


def universe_card(var_root):
    """Return (span, tuniversal) — the short pair every view already uses."""
    card = load_universe(var_root)
    return card["span"], card["tuniversal"]


def load_universe(var_root):
    """universe.card TLV with safe defaults."""
    defaults = {
        "topology": "torus",
        "span": Fraction(64),
        "tuniversal": "00:00:00:00:00:00",
        "ticks": 60,
        "fiber": "—",
    }
    path = Path(var_root) / "worlds" / "universe.card"
    if not path.exists():
        return defaults
    try:
        data = path.read_bytes()
    except OSError:
        return defaults
    out = dict(defaults)
    for tag, val in _iter_tlv(data):
        try:
            s = val.decode("utf-8")
        except Exception:
            s = ""
        if tag == 1:
            out["topology"] = s
        elif tag == 2:
            out["span"] = frac(s)
        elif tag == 3:
            out["tuniversal"] = s
        elif tag == 4 and len(val) == 8:
            out["ticks"] = int.from_bytes(val, "little")
        elif tag == 5:
            out["fiber"] = s
    return out


def load_galaxies(var_root):
    """galaxies.card TLV (NAME/CX/CY/CZ/REACH/END)."""
    path = Path(var_root) / "worlds" / "galaxies.card"
    galaxies = []
    if not path.exists():
        return galaxies
    try:
        data = path.read_bytes()
    except OSError:
        return galaxies
    cur = {"name": "?", "center": (Fraction(0), Fraction(0), Fraction(0)), "reach": Fraction(16)}
    c = ["0", "0", "0"]
    for tag, val in _iter_tlv(data):
        try:
            s = val.decode("utf-8")
        except Exception:
            s = ""
        if tag == 1:
            cur = {
                "name": s,
                "center": (Fraction(0), Fraction(0), Fraction(0)),
                "reach": Fraction(16),
            }
            c = ["0", "0", "0"]
        elif tag == 2:
            c[0] = s
        elif tag == 3:
            c[1] = s
        elif tag == 4:
            c[2] = s
        elif tag == 5:
            cur["reach"] = frac(s)
        elif tag == 0:
            cur["center"] = (frac(c[0]), frac(c[1]), frac(c[2]))
            galaxies.append(cur)
    return galaxies


def galaxy_membership(chart, galaxies, span):
    """Assign each charted world to the nearest galaxy whose reach
    covers it (wrapped L1). Uncovered worlds are intergalactic."""
    members = {g["name"]: [] for g in galaxies}
    intergalactic = []
    for world, coords in chart.items():
        best_name, best_d = None, None
        for galaxy in galaxies:
            d = toroidal_l1(coords[:3], galaxy["center"], span)
            if d <= galaxy["reach"] and (best_d is None or d < best_d):
                best_name, best_d = galaxy["name"], d
        if best_name is None:
            intergalactic.append(world)
        else:
            members[best_name].append((world, best_d))
    for name in members:
        members[name].sort(key=lambda pair: (pair[1], pair[0]))
    intergalactic.sort()
    return members, intergalactic


def toroidal_l1(a, b, span):
    """THE L1 LAW: exact distance, the short way around each axis,
    summed whole — no squaring in a toroid."""
    total = Fraction(0)
    for u, v in zip(a, b):
        d = abs(u - v) % span
        d = min(d, span - d)
        total += d
    return total


# ── table plumbing ──────────────────────────────────────────────────

def strip_ansi(text):
    out, skip = [], False
    for ch in text:
        if ch == "\033":
            skip = True
        elif skip and ch == "m":
            skip = False
        elif not skip:
            out.append(ch)
    return "".join(out)


def tabulate(headers, table):
    widths = [
        max(len(strip_ansi(cell)) for cell in [headers[i]] + [r[i] for r in table])
        for i in range(len(headers))
    ] if table else [len(h) for h in headers]
    lines = ["  ".join(h.ljust(widths[i]) for i, h in enumerate(headers))]
    for r in table:
        lines.append("  ".join(
            cell + " " * (widths[i] - len(strip_ansi(cell)))
            for i, cell in enumerate(r)))
    return "\n".join(lines)


class Paint:
    def __init__(self, plain):
        self.plain = plain

    def c(self, text, code):
        return text if self.plain else f"\033[{code}m{text}\033[0m"

    def head(self, text):
        return self.c(text, "1;36")

    def good(self, text):
        return self.c(text, "32")

    def bad(self, text):
        return self.c(text, "31")

    def warn(self, text):
        """Caution — yellow, not alarm red. Famine and heat use this."""
        return self.c(text, "33")

    def dim(self, text):
        return self.c(text, "2")

    def strong(self, text):
        return self.c(text, "1")


def display_name(name):
    """Display-only beautification — the chart KEY is the identity and
    never changes; only the rendering breathes."""
    for prefix, glyph in (("var-era1--", "e1·"), ("var-era2--", "e2·"),
                          ("var--", "v0·")):
        if name.startswith(prefix):
            name = glyph + name[len(prefix):]
            break
    if len(name) > 28:
        parts = name.split("-")
        if len(parts) > 3:
            name = f"{parts[0]}-{parts[1]}…{parts[-1]}⁽{len(parts) - 3}⁾"
        if len(name) > 28:
            name = name[:13] + "…" + name[-13:]
    return name


def civ_display(card, slug):
    """Display name only — founder is not a prefix (was 'auto', inconsistent)."""
    return card.get("name") or slug


def station_label(card):
    """BAND-1: station is an office or the common unseated state.

    Never blank: None reads 'common' (T3), never an empty cell.
    """
    station = card.get("station")
    if station:
        return station
    return "common"


def engine_pulse(var_root):
    """Is the conductor alive, and how fresh is the export?

    Returns (running: bool, age_seconds: float|None, newest_path: str|None).
    """
    running = False
    try:
        # Match the binary name without sweeping the whole process table
        # into a false positive on this script's own command line.
        import subprocess
        out = subprocess.check_output(
            ["pgrep", "-f", r"(^|/)(conductor|target/.*/conductor)( |$)"],
            text=True, stderr=subprocess.DEVNULL)
        running = bool(out.strip())
    except (subprocess.CalledProcessError, FileNotFoundError, OSError):
        running = False

    newest, newest_path = None, None
    worlds = Path(var_root) / "worlds"
    if worlds.is_dir():
        for pattern in ("**/state.tlv", "**/chronicle.tlv", "**/world.card"):
            for path in worlds.glob(pattern):
                try:
                    mtime = path.stat().st_mtime
                except OSError:
                    continue
                if newest is None or mtime > newest:
                    newest, newest_path = mtime, str(path)
    age = (time.time() - newest) if newest is not None else None
    return running, age, newest_path


def format_age(seconds):
    if seconds is None:
        return "no exports yet"
    seconds = int(seconds)
    if seconds < 60:
        return f"{seconds}s ago"
    if seconds < 3600:
        return f"{seconds // 60}m ago"
    if seconds < 86400:
        return f"{seconds // 3600}h ago"
    return f"{seconds // 86400}d ago"


def condition_parts(row):
    """Plain-language condition tokens — no emoji, no alarm glyphs.

    Returns a list of (severity, severity) where severity is one of
    'ok', 'caution', 'broken'. Empty list means steady / nothing to note.
    """
    parts = []
    if not row.get("linked", True):
        parts.append(("chain broken", "broken"))
    famine = row.get("famine") or []
    if famine:
        parts.append(("short " + "+".join(famine), "caution"))
    try:
        vented = frac(str(row.get("vented", "0")))
        if vented > 0:
            parts.append((f"venting {show_board(vented)}", "caution"))
    except (ValueError, TypeError):
        pass
    return parts


def condition_label(row, paint):
    """Single painted condition string for tables."""
    parts = condition_parts(row)
    if not parts:
        return paint.good("steady")
    painted = []
    for text, severity in parts:
        if severity == "broken":
            painted.append(paint.bad(text))
        else:
            painted.append(paint.warn(text))
    return ", ".join(painted)


def stress_flags(row, paint):
    """Back-compat name used by older views — now plain language."""
    parts = condition_parts(row)
    if not parts:
        return []
    out = []
    for text, severity in parts:
        if severity == "broken":
            out.append(paint.bad(text))
        else:
            out.append(paint.warn(text))
    return out


def tech_held_count(state):
    """How many base-stack conditions this planet has crystallized."""
    stack = state.get("constellation") or []
    if not isinstance(stack, list):
        return 0
    return sum(1 for node in stack if isinstance(node, dict) and node.get("held"))


def zone_summary(kinds):
    """Compact zone inventory: E×3 H×2 S×1 …"""
    if not kinds:
        return "—"
    # Accept either a set of kind names or a multiset dict.
    if isinstance(kinds, dict):
        counts = kinds
    else:
        counts = {}
        for kind in kinds:
            counts[kind] = counts.get(kind, 0) + 1
    short = {
        "extractor": "E", "habitat": "H", "smelter": "S",
        "synthesizer": "Y", "encasement": "C", "cryo-array": "K",
        "resonator": "R", "regenerator": "G",
        "element-refinery": "F",  # Wave-1 REFINE-1
    }
    parts = []
    for kind, n in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0])):
        glyph = short.get(kind, kind[:3])
        parts.append(f"{glyph}×{n}" if n != 1 else glyph)
    return " ".join(parts)


# ── views ───────────────────────────────────────────────────────────

def view_board(var_root, args, paint):
    """Universe dashboard: cosmos → galaxies → systems → civilizations.

    Built for `--watch`: someone sitting on the board should understand
    what is happening without already knowing which colony is starving.
    """
    rows, worlds = read_all(var_root)
    registry = ensure_registry_coverage(load_registry(var_root), rows)
    universe = load_universe(var_root)
    chart = load_starchart(var_root)
    galaxies = load_galaxies(var_root)
    span = universe["span"]
    members, intergalactic = galaxy_membership(chart, galaxies, span)
    running, age, _newest = engine_pulse(var_root)

    # Index planet state for tech holdings / zone kinds (from the
    # worlds already loaded — never re-read the chronicle).
    planet_state = {}
    for wname, facts in worlds.items():
        for state in facts["planets"]:
            key = (state.get("world", wname), state.get("planet", "?"))
            planet_state[key] = state

    # ── universe ────────────────────────────────────────────────────
    tiers = {}
    for slug, card in registry.items():
        tiers.setdefault(tier_of(card), []).append(slug)
    tier_line = " · ".join(
        f"{t} {len(tiers.get(t, []))}"
        for t in ALL_TIERS
        if tiers.get(t) or t in ("T0", "T1", "T2", "T3"))
    chains_ok = all(w["linked"] for w in worlds.values()) if worlds else True
    total_receipts = sum(w["receipts"] for w in worlds.values())
    total_puzzles = sum(w["puzzles"] for w in worlds.values())
    petitions = sum(w["pending"] for w in worlds.values())
    # This version's growth signals, summed across the board.
    minted_words = sum(w.get("words", 0) for w in worlds.values())
    cleaves = sum(1 for w in worlds.values() if w.get("cleaved"))
    shared_truths = sum(w.get("axioms", 0) for w in worlds.values())
    offers = 0
    for facts in worlds.values():
        for state in facts["planets"]:
            offers += len(state.get("offers") or [])

    totals = {r: Fraction(0) for r in RESOURCES}
    element_kinds = 0
    element_mass = Fraction(0)
    for row in rows:
        for r in RESOURCES:
            totals[r] += row[r]
        element_kinds += row.get("element_kinds", 0)
        element_mass += row.get("element_total", Fraction(0))
    active = sum(1 for row in rows if row["clock"] > 0)
    short_iron = sum(1 for row in rows if "iron" in (row.get("famine") or []))
    short_bio = sum(1 for row in rows if "biomass" in (row.get("famine") or []))
    venting = sum(1 for row in rows if frac(str(row.get("vented", "0") or "0")) > 0)
    broken = sum(1 for row in rows if not row.get("linked", True))
    t2_count = len(tiers.get("T2", []))
    t1_count = len(tiers.get("T1", []))

    if running:
        pulse = paint.good("LIVE") + f" · last export {format_age(age)}"
    elif age is not None and age < 120:
        pulse = paint.warn("export fresh, conductor not seen") + f" · {format_age(age)}"
    else:
        pulse = paint.bad("STALE") + f" · last export {format_age(age)} · conductor not running"

    lines = [
        paint.head("UNIVERSE"),
        f"  engine {pulse}",
        f"  topology {paint.strong(universe['topology'])} · "
        f"span {paint.strong(show(span))} · "
        f"TUniversal {paint.strong(universe['tuniversal'])} · "
        f"{universe['ticks']} ticks/year",
        f"  fiber {paint.dim(universe['fiber'])}",
        f"  {paint.strong(str(len(registry)))} civilizations ({tier_line})",
        f"  {paint.strong(str(len(worlds)))} worlds · "
        f"{paint.strong(str(len(rows)))} settled planets · "
        f"{paint.strong(str(len(galaxies)))} galaxies · "
        f"{paint.strong(str(len(chart)))} charted positions"
        + (
            f" · {paint.dim(str(count_closed_ruins(var_root)) + ' closed ruins hidden')}"
            if count_closed_ruins(var_root)
            else ""
        ),
        f"  activity: {paint.strong(str(active))} systems ticking · "
        f"{paint.strong(str(total_receipts))} receipts · "
        f"{paint.strong(str(total_puzzles))} puzzles solved · "
        f"{paint.strong(str(offers))} market offer(s) · "
        f"{paint.strong(str(petitions))} petition(s)",
        f"  cognition: {paint.strong(str(minted_words))} words minted "
        f"(each line's own vocabulary) · "
        f"{paint.strong(str(cleaves))} lines CLEAVED "
        f"(spawned a peer branch at mastery) · "
        f"{paint.strong(str(shared_truths))} shared truth(s) "
        f"(independent lines converged)",
        f"  matter — elements {paint.strong(str(element_kinds))} kinds "
        f"({paint.strong(show_board(element_mass))} total mass) · "
        f"law carriers: biomass {paint.strong(show_board(totals['biomass']))} · "
        f"joule {paint.strong(show_board(totals['joule']))} · "
        f"crystal {paint.strong(show_board(totals['crystal']))} · "
        f"alloy {paint.strong(show_board(totals['alloy']))} · "
        f"iron(legacy) {paint.dim(show_board(totals['iron']))}",
        f"  chain {paint.good('all intact') if chains_ok else paint.bad('BROKEN somewhere')}"
        f" · condition roll-up: "
        f"{short_iron} short iron · {short_bio} short biomass · "
        f"{venting} venting heat · {broken} broken",
    ]
    if t2_count == 0:
        lines.append(
            "  " + paint.warn(
                "BOARD GAP: zero sovereigns (T2 empty) — no line holds the "
                "record-keeper seat; ascend via `ascend <sigil> to sovereign` "
                "or seat founding T2 lines"))
    if t1_count == 0:
        lines.append(
            "  " + paint.warn(
                "BOARD GAP: zero T1 seats (watcher/observer) — the architect band is empty"))

    # ── galaxies ────────────────────────────────────────────────────
    lines += ["", paint.head("GALAXIES")]
    if not galaxies:
        lines.append("  " + paint.dim("no galaxies charted yet — observation founds them"))
    else:
        g_table = []
        for galaxy in galaxies:
            name = galaxy["name"]
            roster = members.get(name, [])
            # Busiest member by clock among this galaxy's worlds.
            lead = "—"
            lead_clock = -1
            for world_name, _dist in roster:
                for row in rows:
                    if row["world"] == world_name and row["clock"] > lead_clock:
                        lead_clock = row["clock"]
                        lead = f"{display_name(world_name)} t{row['clock']}"
            cx, cy, cz = galaxy["center"]
            g_table.append([
                display_name(name),
                f"({show(cx)}, {show(cy)}, {show(cz)})",
                show(galaxy["reach"]),
                str(len(roster)),
                lead,
            ])
        lines.append(tabulate(
            ["GALAXY", "CENTER", "REACH", "WORLDS", "MOST ACTIVE"],
            g_table))
        if intergalactic:
            lines.append(
                "  intergalactic (outside every reach): "
                + ", ".join(display_name(w) for w in intergalactic))

    # ── solar systems ───────────────────────────────────────────────
    lines += ["", paint.head("SOLAR SYSTEMS — each world is one star system")]
    # Sort: most active first (clock desc), then name.
    system_rows = sorted(rows, key=lambda r: (-r["clock"], r["world"], r["planet"]))
    s_table = []
    for row in system_rows:
        state = planet_state.get((row["world"], row["planet"]), {})
        # Prefer multiset from state zones; fall back to kind set.
        kind_counts = {}
        for zone in state.get("zones") or []:
            k = zone.get("kind", "?")
            kind_counts[k] = kind_counts.get(k, 0) + 1
        if not kind_counts and row.get("zone_kinds"):
            kind_counts = {k: 1 for k in row["zone_kinds"]}
        held = tech_held_count(state)
        civ_slug = row["civilization"]
        civ_card = registry.get(civ_slug, {})
        civ_name = civ_card.get("name", civ_slug)
        # Galaxy of this world (short label for the column).
        g_name = "—"
        for galaxy in galaxies:
            roster = members.get(galaxy["name"], [])
            if any(w == row["world"] for w, _ in roster):
                g_name = galaxy["name"]
                if g_name.startswith("the-"):
                    g_name = g_name[4:]
                # archon-sky-1 → archon·1
                g_name = g_name.replace("-sky-", "·").replace("-", "·")
                break
        else:
            if row["world"] in intergalactic:
                g_name = "intergalactic"
            elif row["world"] not in chart:
                g_name = "uncharted"
        ek = row.get("element_kinds", 0)
        elem_cell = (
            f"{ek}·{show_board(row.get('element_total', Fraction(0)))}"
            if ek else "—")
        s_table.append([
            display_name(row["world"]),
            row["planet"],
            STAR_NAMES.get(row["star"], "—"),
            civ_name[:18],
            g_name[:14],
            zone_summary(kind_counts),
            str(row["clock"]),
            elem_cell,
            show_board(row["biomass"]),
            show_board(row["crystal"]),
            show_board(row["joule"]),
            f"{held}" if held else "—",
            str(worlds.get(row["world"], {}).get("puzzles", 0) or "—")
            if worlds.get(row["world"], {}).get("puzzles", 0) else "—",
            condition_label(row, paint),
        ])
    lines.append(tabulate(
        ["WORLD", "PLANET", "STAR", "CIVILIZATION", "GALAXY", "ZONES",
         "CLOCK", "ELEMENTS", "BIO", "CRYSTAL", "JOULE", "TECH", "PUZZLES",
         "CONDITION"],
        s_table))
    lines.append(paint.dim(
        "  zones: E extractor · H habitat · S smelter · Y synthesizer · "
        "C encasement · K cryo · R resonator · G regenerator"))
    lines.append(paint.dim(
        "  ELEMENTS = N kinds · total mass (almanac matter, tag 72) · "
        "BIO/CRYSTAL/JOULE are closed law carriers · iron is legacy ration only"))
    lines.append(paint.dim(
        "  condition: steady = banks feeding · short X = famine on resource X · "
        "venting = excess heat sealed as entropy · large banks floor with '+'"))

    # ── civilizations ───────────────────────────────────────────────
    lines += ["", paint.head("CIVILIZATIONS")]
    # Aggregate per civ from rows.
    civ_agg = {}
    for row in rows:
        agg = civ_agg.setdefault(row["civilization"], {
            "clock": 0, "zones": 0, "worlds": set(),
            "iron": Fraction(0), "biomass": Fraction(0), "crystal": Fraction(0),
            "element_kinds": 0, "element_total": Fraction(0),
            "puzzles": 0, "famine": set(), "venting": False,
        })
        agg["clock"] = max(agg["clock"], row["clock"])
        agg["zones"] += row["zones"]
        agg["worlds"].add(row["world"])
        agg["iron"] += row["iron"]
        agg["biomass"] += row["biomass"]
        agg["crystal"] += row["crystal"]
        agg["element_kinds"] += row.get("element_kinds", 0)
        agg["element_total"] += row.get("element_total", Fraction(0))
        for f in row.get("famine") or []:
            agg["famine"].add(f)
        try:
            if frac(str(row.get("vented", "0") or "0")) > 0:
                agg["venting"] = True
        except (ValueError, TypeError):
            pass
    # puzzles double-count if multi-planet; fix by world once
    for slug, agg in civ_agg.items():
        agg["puzzles"] = sum(
            worlds.get(w, {}).get("puzzles", 0) for w in agg["worlds"])

    unregistered = sorted(
        slug for slug in civ_agg if slug not in registry)
    for tier in ALL_TIERS:
        members_t = sorted(
            (slug for slug, card in registry.items() if tier_of(card) == tier),
            key=lambda s: (-civ_agg.get(s, {}).get("clock", 0), s))
        # Unregistered holdings (player/test lines) land under T3.
        if tier == "T3":
            members_t = members_t + [
                s for s in unregistered if s not in members_t]
        # Always show the classical band; show economic bands only when
        # someone holds a seat there (keeps the watch board scannable).
        if not members_t and tier not in ("T0", "T1", "T2", "T3"):
            continue
        lines.append(paint.strong(f"  {TIER_TITLES[tier]}"))
        if not members_t:
            lines.append("    " + paint.dim("(empty)"))
            continue
        c_table = []
        for slug in members_t:
            card = registry.get(slug, {})
            agg = civ_agg.get(slug, {
                "clock": 0, "zones": 0, "worlds": set(),
                "iron": Fraction(0), "biomass": Fraction(0), "crystal": Fraction(0),
                "element_kinds": 0, "element_total": Fraction(0),
                "puzzles": 0, "famine": set(), "venting": False,
            })
            home = ", ".join(sorted(display_name(w) for w in agg["worlds"])) or "—"
            cond_bits = []
            if agg["famine"]:
                cond_bits.append("short " + "+".join(sorted(agg["famine"])))
            if agg["venting"]:
                cond_bits.append("venting")
            if not cond_bits:
                cond = paint.good("steady") if agg["worlds"] else paint.dim("unseated")
            else:
                cond = paint.warn(", ".join(cond_bits))
            c_table.append([
                civ_display(card, slug)[:22],
                (card.get("species") or "?")[:24],
                station_label(card),
                home[:28],
                str(agg["zones"]),
                str(agg["clock"]),
                show_board(agg.get("element_total", Fraction(0)))
                if agg.get("element_kinds") else "—",
                show_board(agg["biomass"]),
                show_board(agg["crystal"]),
                str(agg["puzzles"]) if agg["puzzles"] else "—",
                cond,
            ])
        lines.append(tabulate(
            ["CIVILIZATION", "SPECIES", "STATION", "WORLD(S)",
             "ZONES", "CLOCK", "ELEMENTS", "BIO", "CRYSTAL", "PUZZLES",
             "CONDITION"],
            c_table))
        lines.append("")

    # ── NAV ─────────────────────────────────────────────────────────
    lines += [
        paint.head("DRILL DOWN"),
        "  " + paint.dim(
            "worlds · world <name> · civs [--tier T0] · civ <slug> · "
            "chart · market · techs · tiers · stress"),
        "  " + paint.dim(
            "watch: python3 tools/observer.py --watch 2 --root var"),
    ]
    return "\n".join(lines).rstrip()


def view_civs(var_root, args, paint):
    """The civilizations, grouped by tier, filterable."""
    rows, worlds = read_all(var_root)
    registry = ensure_registry_coverage(load_registry(var_root), rows)
    civs = {}
    for row in rows:
        agg = civs.setdefault(row["civilization"], {
            "colonies": 0, "puzzles": 0, "depth": 0, "consumed": Fraction(0),
            "worlds": set(), "zone_kinds": set(),
            **{r: Fraction(0) for r in RESOURCES}})
        agg["colonies"] += 1
        agg["zone_kinds"] |= row.get("zone_kinds", set())
        agg.setdefault("events", [])
        agg.setdefault("digest", "")
        for resource in RESOURCES:
            agg[resource] += row[resource]
        if row["world"] not in agg["worlds"]:
            agg["worlds"].add(row["world"])
            agg["puzzles"] += row["puzzles"]
            agg["depth"] = max(agg["depth"], row["depth"])
            agg["consumed"] += row["consumed"]
            facts = worlds.get(row["world"], {})
            agg["events"] = agg["events"] + facts.get("events", [])
            # Deterministic: the lexicographically first world's
            # terminal digest carries the voice.
            if not agg["digest"] or row["world"] < agg.get("digest_world", "￿"):
                agg["digest"] = facts.get("digest", "")
                agg["digest_world"] = row["world"]
    by_tier = {}
    for slug in sorted(set(list(civs) + list(registry))):
        card = registry.get(slug, {})
        tier = tier_of(card)
        if args.tier and tier != args.tier.upper():
            continue
        agg = civs.get(slug, {
            "colonies": 0, "puzzles": 0, "depth": 0, "consumed": Fraction(0),
            "worlds": set(), "zone_kinds": set(),
            **{r: Fraction(0) for r in RESOURCES}})
        by_tier.setdefault(tier, []).append([
            civ_display(card, slug),
            card.get("species", "?"),
            station_label(card),
            function_of(card, agg),
            creed_of(agg.get("events", []), agg.get("digest", "")),
            str(len(agg["worlds"])), str(agg["colonies"]),
            show_board(agg["biomass"]),
            show_board(agg["joule"]), show_board(agg["sink"]),
            show_board(agg["crystal"]),
            show_board(agg["iron"]),
            str(agg["puzzles"]),
            show(Fraction(agg["puzzles"]) / agg["consumed"])
            if agg["consumed"] > 0 else "—",
        ])
    lines = []
    for tier in ALL_TIERS:
        if tier not in by_tier:
            continue
        lines += [paint.head(TIER_TITLES[tier]), tabulate(
            ["CIVILIZATION", "SPECIES", "STATION", "FUNCTION", "CREED",
             "WORLDS", "COLONIES",
             "BIOMASS", "JOULE", "SINK", "CRYSTAL", "IRON(leg)", "PUZZLES",
             "INTELLIGENCE"],
            by_tier[tier]), ""]
    if "T?" in by_tier:
        lines += [paint.head(TIER_TITLES["T?"]), tabulate(
            ["CIVILIZATION", "SPECIES", "STATION", "FUNCTION", "CREED",
             "WORLDS", "COLONIES",
             "BIOMASS", "JOULE", "SINK", "CRYSTAL", "IRON(leg)", "PUZZLES",
             "INTELLIGENCE"],
            by_tier["T?"]), ""]
    return "\n".join(lines).rstrip() or "(no civilizations match)"


def view_worlds(var_root, args, paint):
    """The settled planets table (the old front page), sortable."""
    rows, _ = read_all(var_root)

    def sort_value(row):
        value = row.get(args.sort, 0)
        return value if value is not None else Fraction(-1)

    rows = sorted(rows, key=sort_value,
                  reverse=(args.sort not in ("civilization", "world", "planet")))
    table = []
    for row in rows:
        ek = row.get("element_kinds", 0)
        elem = (
            f"{ek}·{show_board(row.get('element_total', Fraction(0)))}"
            if ek else "—")
        traj = row.get("trajectory", "—")
        if traj == "decline":
            traj = paint.bad(traj)
        elif traj == "stagnation":
            traj = paint.warn(traj)
        elif traj == "growth":
            traj = paint.good(traj)
        table.append([
            row["civilization"], row["world"], row["planet"],
            elem,
            show_board(row.get("stamp", Fraction(0))),
            show_board(row["biomass"]),
            show_board(row["alloy"]), show_board(row["diamond"]),
            show_board(row["joule"]), show_board(row["sink"]),
            show_board(row["crystal"]),
            show_board(row["iron"]),
            STAR_NAMES.get(row["star"], "—"),
            str(row["zones"]), str(row["clock"]),
            traj,
            str(row.get("hunger", "—")),
            str(row.get("rail", "—")),
            str(row.get("labor", 0)),
            paint.good("ok") if row["linked"] else paint.bad("BROKEN"),
            condition_label(row, paint),
        ])
    return "\n".join([paint.head("SETTLED PLANETS"), tabulate(
        ["CIVILIZATION", "WORLD", "PLANET", "ELEMENTS", "STAMP", "BIOMASS", "ALLOY",
         "DIAMOND", "JOULE", "SINK", "CRYSTAL", "IRON(leg)", "STAR", "ZONES",
         "TICKS", "TRAJ", "HUNGER", "RAIL", "LABOR",
         "CHAIN", "CONDITION"],
        table)])


def view_world(var_root, args, paint):
    """One world, whole: planets, techs, chronicle tail, petitions."""
    target = args.target
    if not target:
        return "usage: observer.py world <name>"
    match = next((d for d in world_dirs(var_root) if d.name == target), None)
    if match is None:
        names = ", ".join(d.name for d in world_dirs(var_root))
        return f"no world '{target}' — the board holds: {names}"
    facts = world_facts(match)
    chart = load_starchart(var_root)
    span, _ = universe_card(var_root)
    lines = [paint.head(f"WORLD {facts['name']}")]
    chain = paint.good("intact") if facts["linked"] else paint.bad("BROKEN")
    lines.append(
        f"  chain {chain} · {facts['receipts']} receipts · "
        f"{facts['puzzles']} puzzles (depth {facts['depth']}) · "
        f"{facts['pending']} petition(s) pending")
    if facts["name"] in chart:
        x, y, z, g = chart[facts["name"]]
        nearest, best = None, None
        for other, coords in chart.items():
            if other == facts["name"]:
                continue
            d = toroidal_l1((x, y, z), coords[:3], span)
            if best is None or d < best:
                nearest, best = other, d
        lines.append(
            f"  S=({show(x)}, {show(y)}, {show(z)}) · G={show(g)}"
            + (f" · nearest {display_name(nearest)} at L1 {show(best)}"
               if nearest else ""))
    for state in facts["planets"]:
        banked = state.get("banked", {})
        holdings = " · ".join(
            f"{r} {banked[r]}" for r in RESOURCES if r in banked) or "empty banks"
        stamp = state.get("stamp", "0")
        star = STAR_NAMES.get(state.get("star_class"), "—")
        traj = state.get("trajectory", "—")
        hunger = state.get("hunger") or []
        if isinstance(hunger, list):
            hunger_s = ",".join(hunger) if hunger else "—"
        else:
            hunger_s = str(hunger)
        rail = state.get("rail", "—")
        labor = state.get("labor", 0)
        habitats = state.get("habitats", 0)
        ap_a = state.get("ap_available", "—")
        ap_c = state.get("ap_capacity", "—")
        lines += ["", paint.head(
            f"  PLANET {state.get('planet', '?')} — {state.get('civilization', '?')}"),
            f"    star {star} · clock {state.get('clock', 0)} · "
            f"{len(state.get('zones', []))} zone(s) · vented {state.get('entropy_vented', '0')}",
            f"    STAMP {stamp} · {holdings}",
            f"    traj {traj} · hunger {hunger_s} · rail {rail} "
            f"({state.get('rail_routes', 0)} routes) · "
            f"labor {labor} (hab {habitats}) · AP {ap_a}/{ap_c}"]
        if state.get("famine"):
            lines.append(
                "    " + paint.warn("short on: " + ", ".join(state["famine"])))
        elements = state.get("elements") or []
        if elements:
            shown = []
            for entry in elements[:12]:
                if isinstance(entry, dict):
                    name = entry.get("name") or (entry.get("id") or "?")[:8]
                    amt = entry.get("amount", "?")
                    shown.append(f"{name} {amt}")
                else:
                    shown.append(str(entry))
            more = f" · +{len(elements) - 12} more" if len(elements) > 12 else ""
            lines.append("    elements: " + " · ".join(shown) + more)
        zones = state.get("zones", [])
        if zones:
            kinds = {}
            for zone in zones:
                kinds[zone.get("kind", "?")] = kinds.get(zone.get("kind", "?"), 0) + 1
            lines.append("    zones: " + " · ".join(
                f"{k}×{v}" for k, v in sorted(kinds.items())))
    # CONSTEL-1: the base-7 strip from the uplink reading.
    for state in facts["planets"]:
        stack = state.get("constellation")
        if stack:
            glyphs = []
            for node in stack:
                if node.get("held"):
                    glyphs.append("✶" if node.get("condition") != 6 else "◆")
                elif node.get("threshold", 1) > 1 and node.get("condition") != 6:
                    glyphs.append(f"{node.get('progress', 0)}/{node.get('threshold')}")
                else:
                    glyphs.append("○")
            held = sum(1 for n in stack if n.get("held"))
            lines.append(
                f"  base-7: {' '.join(glyphs)}  ({held}/7 held)")
            break
    techs = [e for e in facts["events"] if e.get("event") in
             ("tech_discovered", "tech_licensed")]
    if techs:
        lines += ["", paint.head("  TECHNOLOGY (this line's own tongue)")]
        for event in techs:
            lines.append(f"    t{event.get('stamp', '?')}  "
                         + paint.strong(event.get("detail", "")))
    tail = facts["events"][-args.tail:]
    if tail:
        lines += ["", paint.head(f"  CHRONICLE — the last {len(tail)} receipts")]
        for event in tail:
            lines.append(
                f"    t{event.get('stamp', '?'):>4}  "
                f"{event.get('event', '?'):<22} {paint.dim(event.get('detail', ''))}")
    return "\n".join(lines)


def view_civ(var_root, args, paint):
    """One civilization, whole: card, lineage walk, holdings."""
    target = args.target
    registry = load_registry(var_root)
    if not target or target not in registry:
        names = ", ".join(sorted(registry))
        return f"no civilization '{target or ''}' — the registry holds: {names}"
    card = registry[target]
    rows, worlds = read_all(var_root)
    axes = card.get("axes", {})
    lines = [paint.head(f"CIVILIZATION {civ_display(card, target)} [{tier_of(card)}]")]
    clade = card.get("clade", "")
    lines.append(f"  species {paint.strong(card.get('species', '?'))}"
                 + (f" · clade {clade}" if clade else "")
                 + f" · station {station_label(card)}")
    # The three sealed identity axes (G1): disposition · metabolism ·
    # perception — what the line wants, how it feeds, how it senses.
    # Shown only on the drill-down, never as a bare "axes" column.
    if any(axes.get(a) for a in ("disposition", "metabolism", "perception")):
        lines.append(
            "  identity: "
            f"wants={axes.get('disposition', '—')} · "
            f"feeds={axes.get('metabolism', '—')} · "
            f"senses={axes.get('perception', '—')}")
    if card.get("motto"):
        lines.append(f"  {paint.dim(chr(0x201C) + card['motto'] + chr(0x201D))}")
    # The lineage walk (LINEAGE-2): the pointer is a space-joined
    # list — walk the primary chain (first parent per hop), and mark
    # the confluences; headless is lawful.
    walk, seen, confluent = [target], set(), False
    while True:
        parents = (registry.get(walk[-1], {}).get("lineage") or "").split()
        if len(parents) > 1:
            confluent = True
        parent = parents[0] if parents else None
        if not parent or parent in seen or parent not in registry:
            break
        seen.add(walk[-1])
        walk.append(parent)
    if len(walk) > 1:
        note = "  (head: " + walk[-1] + (", ⋔ confluent — every root via resolve_heads" if confluent else "") + ")"
        lines.append("  lineage: " + " ↳ ".join(walk) + paint.dim(note))
    else:
        lines.append("  lineage: " + paint.dim("headless — a lawful state"))
    civ_worlds = sorted({r["world"] for r in rows if r["civilization"] == target})
    civ_events = [e for w in civ_worlds for e in worlds.get(w, {}).get("events", [])]
    civ_digest = worlds.get(civ_worlds[0], {}).get("digest", "") if civ_worlds else ""
    lines.append("  creed: " + paint.strong(creed_of(civ_events, civ_digest))
                 + paint.dim("  (derived from the chronicle — 14_CREEDS)"))
    held = [r for r in rows if r["civilization"] == target]
    if held:
        lines += ["", paint.head("  HOLDINGS")]
        for row in held:
            lines.append(
                f"    {row['world']}/{row['planet']} · "
                f"{STAR_NAMES.get(row['star'], '—')} star · "
                f"clock {row['clock']} · {row['zones']} zone(s) · "
                f"{condition_label(row, paint)}")
    else:
        lines.append("  no settled planets on the board")
    return "\n".join(lines)


def view_tiers(var_root, args, paint):
    """The union board T0→T9 — who stands where, seats by station."""
    registry = load_registry(var_root)
    lines = []
    for tier in ALL_TIERS:
        members = sorted(
            slug for slug, card in registry.items() if tier_of(card) == tier)
        # Skip empty economic bands on the tiers view unless every band
        # is wanted later; always print the classical four.
        if not members and tier not in ("T0", "T1", "T2", "T3"):
            continue
        lines.append(paint.head(TIER_TITLES[tier]))
        if members:
            for slug in members:
                card = registry[slug]
                station = station_label(card)
                lines.append(
                    f"  {civ_display(card, slug):<24} "
                    f"{paint.dim((card.get('species') or '?')[:20]):<22} "
                    f"{paint.strong(station)}")
        else:
            lines.append("  " + paint.dim("(no one stands here yet)"))
        lines.append("")
    # Unreadable seats (T?) if any registry card holds an unknown office.
    unknown = sorted(
        slug for slug, card in registry.items() if tier_of(card) == "T?")
    if unknown:
        lines.append(paint.head(TIER_TITLES["T?"]))
        for slug in unknown:
            card = registry[slug]
            lines.append(
                f"  {civ_display(card, slug):<24} "
                f"{paint.bad(str(card.get('station')))}")
        lines.append("")
    lines.append(paint.dim(
        "seats are space, not rank — T4–T9 are deeper economic offices "
        "(Wave-1); rooms and widths: CLI `civ seats` · registry seat_width"))
    return "\n".join(lines)


# MANU-1 (ruled 2026-07-31): reach = BASE_REACH + instrument grade —
# the global constant is struck; the unaided horizon is the g=0 read.
# Until InstrumentForged receipts land engine-side, every world reads
# unaided (grade 0) through the same law.
BASE_REACH = 16


def epistemic_reach(_wire):
    """The world's scan horizon: base + highest instrument grade.

    Instrument grades ride the engine World (MANU-1); the TLV mirror
    does not yet export them. Unaided horizon = BASE_REACH until a
    wire tag lands for instruments.
    """
    return BASE_REACH


def epistemic_reads(var_root, paint):
    """WANT-1 kind 1 (board-side): truth-starvation — a world whose
    REACH (wrapped L1 on the chart) contains broken chains or ruins
    reads the epistemological want. Worlds cannot see neighbors; the
    board can — this reading lives here by design."""
    chart = load_starchart(var_root)
    span, _ = universe_card(var_root)
    corrupt = []
    for world_dir in world_dirs(var_root):
        name = world_dir.name
        if "--closed-" in name:
            corrupt.append(name)
            continue
        linked, _ = chain_linked_tlv(world_dir)
        if not linked:
            corrupt.append(name)
    # Closed ruins on disk (even if excluded from live listing).
    worlds_dir = Path(var_root) / "worlds"
    if worlds_dir.is_dir():
        for p in worlds_dir.iterdir():
            if p.is_dir() and is_closed_ruin(p.name) and p.name not in corrupt:
                corrupt.append(p.name)
    reads = []
    for name, coords in chart.items():
        if name in corrupt:
            continue
        reach = epistemic_reach(None)
        near = [
            other for other in corrupt
            if other in chart
            and toroidal_l1(coords[:3], chart[other][:3], span) <= reach
        ]
        if near:
            reads.append(
                f"{name}  {paint.warn('truth-starved')} — "
                f"{len(near)} corrupted chronicle(s) in reach: "
                + paint.dim(", ".join(display_name(n) for n in near)))
    return reads


def view_stress(var_root, args, paint):
    """Condition roll-up only — shortages, heat, petitions, truth-starvation.

    Prefer the default board for the full picture; this view is the
    narrow t0 checklist.
    """
    rows, worlds = read_all(var_root)
    lines = [paint.head("CONDITION ROLL-UP")]
    any_stress = False
    for line in epistemic_reads(var_root, paint):
        any_stress = True
        lines.append(f"  {line}")
    short = [r for r in rows if condition_parts(r)]
    if short:
        any_stress = True
        lines.append(paint.strong("  systems under pressure"))
        for row in short:
            lines.append(
                f"    {row['world']}/{row['planet']:<16}  "
                f"{condition_label(row, paint)}")
    for name, facts in sorted(worlds.items()):
        if facts["pending"]:
            any_stress = True
            lines.append(
                f"  {name}  {paint.strong(str(facts['pending']))} petition(s) pending")
    if not any_stress:
        lines.append("  " + paint.good("all systems steady — the board breathes"))
    lines.append(paint.dim(
        "  deeds: t0 move (separate) · licensed leap (relieve) · t0 contract (seal)"))
    lines.append(paint.dim(
        "  for the full universe view: python3 tools/observer.py  (no subcommand)"))
    return "\n".join(lines)


def view_techs(var_root, args, paint):
    """Every crystallized and licensed node on the board, verbatim —
    the detail line is displayed, never parsed."""
    lines = [paint.head("TECHNOLOGY — each line's own tongue")]
    found = False
    for world_dir in world_dirs(var_root):
        facts = world_facts(world_dir)
        techs = [e for e in facts["events"] if e.get("event") in
                 ("tech_discovered", "tech_licensed")]
        for event in techs:
            found = True
            kind = "⌁" if event.get("event") == "tech_licensed" else "✶"
            lines.append(f"  {kind} {facts['name']:<18} t{event.get('stamp', '?'):>4}  "
                         + paint.strong(event.get("detail", "")))
    if not found:
        lines.append("  " + paint.dim("nothing crystallized yet — pressure and "
                                      "surplus write this page"))
    return "\n".join(lines)


def view_market(var_root, args, paint):
    """BOARD-1: the trade board — standing offers from the state
    uplink (data fields, never prose), clears counted by event kind."""
    lines = [paint.head("THE TRADE BOARD — resting intents, cleared crossings")]
    resting = 0
    for world_dir in world_dirs(var_root):
        for state in world_facts(world_dir)["planets"]:
            for offer in state.get("offers", []):
                resting += 1
                lines.append(
                    f"  ⇋ {world_dir.name:<18} offer {offer.get('offer', '?')}: "
                    f"{paint.strong(offer.get('give_amount', '?'))} {offer.get('give', '?')} "
                    f"for {paint.strong(offer.get('want_amount', '?'))} {offer.get('want', '?')}"
                )
    cleared = 0
    for world_dir in world_dirs(var_root):
        cleared += sum(
            1 for e in world_facts(world_dir)["events"]
            if e.get("event") == "offer_cleared")
    if resting == 0:
        lines.append("  " + paint.dim("no intents rest — the market waits on surplus and want"))
    lines.append(
        f"  crossings cleared to date: {paint.strong(str(cleared // 2))}"
        + paint.dim("  (each clear is two books)"))
    lines.append(paint.dim("  the price is the ratio — no absolute price exists (guardrail 6)"))
    lines.append(paint.dim("  element lanes: observer routes · refineries: observer refineries"))
    return "\n".join(lines)


def view_routes(var_root, args, paint):
    """E3.8 — element trade routes (E-ROUTE-1) from chronicle.tlv.

    Live set = ElementRouteChartered (tag 110) minus RouteLapsed (80).
    Display only; engine owns SEAL B verify.
    """
    lines = [paint.head(
        "ELEMENT ROUTES — open matter lanes (deal · rail · η · element)")]
    total = 0
    by_rail = {1: 0, 2: 0, 3: 0, 4: 0}
    for world_dir in world_dirs(var_root):
        routes = load_element_routes(world_dir)
        if not routes:
            continue
        lines.append(paint.strong(f"  {display_name(world_dir.name)}"))
        for r in sorted(routes, key=lambda x: x["deal"]):
            total += 1
            rail = int(r.get("rail") or 0)
            by_rail[rail] = by_rail.get(rail, 0) + 1
            eid = r["element"]
            ename = element_name(eid)
            lines.append(
                f"    deal {paint.strong(str(r['deal'])):<8} "
                f"{_RAIL_WORD.get(rail, f'R{rail}')}  "
                f"η={show(r['eta']):<8}  "
                f"{ename}  "
                f"{paint.dim(eid[:4].hex())}"
            )
    if total == 0:
        lines.append("  " + paint.dim(
            "no element routes charted — charter after E-APT / E-ROUTE deeds"))
    else:
        rail_bits = " · ".join(
            f"{_RAIL_WORD.get(k, k)} {v}" for k, v in sorted(by_rail.items()) if v)
        lines.append(
            f"  {paint.strong(str(total))} live lane(s)"
            + (f"  ({rail_bits})" if rail_bits else "")
        )
    lines.append(paint.dim(
        "  authority: chronicle.tlv (charter 110 · lapse 80) · melt rides the haul"))
    return "\n".join(lines)


def view_refineries(var_root, args, paint):
    """E3.8 — ElementRefinery zones (Wave-1 REFINE-1).

    Prefer state.tlv mirror (live tiles). Fall back to chronicle
    ZoneBuilt(kind=9) − ZoneRazed when the mirror is empty.
    """
    lines = [paint.head(
        "ELEMENT REFINERIES — open-matter converters (zone kind 9)")]
    total = 0
    for world_dir in world_dirs(var_root):
        from_state = load_refineries_from_state(world_dir)
        if from_state:
            rows = from_state
            source = "state.tlv"
        else:
            rows = load_refineries_from_chronicle(world_dir)
            source = "chronicle"
        if not rows:
            continue
        lines.append(
            paint.strong(f"  {display_name(world_dir.name)}")
            + paint.dim(f"  [{source}]")
        )
        for row in rows:
            total += 1
            planet = row.get("planet") or f"orb {row.get('orb_id', '?')}"
            lines.append(
                f"    {planet:<20} tile {paint.strong(str(row['tile']))}  "
                f"element-refinery"
            )
    if total == 0:
        lines.append("  " + paint.dim(
            "no element-refineries standing — raise ZoneKind::ElementRefinery "
            "(DEEPENING / T5 eases spend)"))
    else:
        lines.append(f"  {paint.strong(str(total))} refinery zone(s) standing")
    lines.append(paint.dim(
        "  throughput derives from element family/tier (14 §5) — no hand recipes"))
    return "\n".join(lines)


def view_chart(var_root, args, paint):
    """The star chart: ascii torus + exact coordinates."""
    chart = load_starchart(var_root)
    span, tuniversal = universe_card(var_root)
    if not chart:
        return "(no chart yet)"
    names = sorted(chart)
    lines = [paint.head(
        f"STAR CHART — 3-torus, span {show(span)} (edges wrap) — TUniversal {tuniversal}")]
    width, height = 49, 13

    def label(i):
        out = ""
        i += 1
        while i > 0:
            i, r = divmod(i - 1, 26)
            out = chr(ord("a") + r) + out
        return out

    canvas = [[" "] * width for _ in range(height)]
    occupied, marks = {}, []
    for i, name in enumerate(names):
        x, y = chart[name][0], chart[name][1]
        col = int((x % span) / span * width) % width
        row = int((span - (y % span)) / span * height) % height
        mark = label(i)
        if (row, col) in occupied:
            canvas[row][col] = "*"
            occupied[(row, col)] += 1
        else:
            canvas[row][col] = mark[0] if len(mark) == 1 else "+"
            occupied[(row, col)] = 1
        marks.append((mark, name))
    lines += ["  ·" + "-" * width + "·"]
    lines += ["  |" + "".join(r) + "|" for r in canvas]
    lines += ["  ·" + "-" * width + "·",
              "  (* = several worlds share this cell · + = world beyond z on the legend)"]
    for mark, name in marks:
        x, y, z, g = chart[name]
        nearest, best = "—", None
        for other in names:
            if other == name:
                continue
            d = toroidal_l1((x, y, z), chart[other][:3], span)
            if best is None or d < best:
                nearest, best = other, d
        lines.append(
            f"  {mark:<3} {display_name(name):<28} S=({show(x)}, {show(y)}, {show(z)}) G={show(g)}"
            + (f"   nearest {display_name(nearest)} at L1 {show(best)}"
               if best is not None else ""))
    return "\n".join(lines)


def view_ruins(var_root, args, paint):
    """P8.4 — closed-ruins history (archives rotated on divergence).

    Live board hides `--closed-` worlds. This view lists them as
    history: name · receipts · terminal digest · age of export.
    """
    worlds_dir = Path(var_root) / "worlds"
    if not worlds_dir.is_dir():
        return paint.dim("(no worlds dir)")
    ruins = []
    for p in sorted(worlds_dir.iterdir()):
        if not p.is_dir() or not is_closed_ruin(p.name):
            continue
        facts = world_facts(p)  # works on closed dirs too
        # Split live stem from closed marker.
        base, _, digest8 = p.name.partition("--closed-")
        mtime = p.stat().st_mtime if p.exists() else 0
        age = time.time() - mtime
        ruins.append([
            display_name(base),
            digest8[:8] if digest8 else "?",
            str(facts.get("receipts", 0)),
            str(facts.get("puzzles", 0)),
            format_age(age),
            paint.good("ok") if facts.get("linked") else paint.bad("BROKEN"),
            (facts.get("digest") or "")[:12] or "—",
        ])
    if not ruins:
        return "\n".join([
            paint.head("CLOSED RUINS"),
            paint.dim("  (none — no divergence rotations under this var/)"),
            paint.dim("  live board hides these; this view is the history door"),
        ])
    return "\n".join([
        paint.head(f"CLOSED RUINS — {len(ruins)} archive(s)"),
        paint.dim("  rotated on chronicle divergence · not live solar systems"),
        tabulate(
            ["LINE", "CLOSED", "RECEIPTS", "PUZZLES", "AGE", "CHAIN", "DIGEST"],
            ruins,
        ),
    ])


def view_arc(var_root, args, paint):
    """P8.2 compact board: trajectory · hunger · rail · labor only."""
    rows, _ = read_all(var_root)
    table = []
    for row in sorted(rows, key=lambda r: r.get("civilization", "")):
        traj = row.get("trajectory", "—")
        if traj == "decline":
            traj = paint.bad(traj)
        elif traj == "stagnation":
            traj = paint.warn(traj)
        elif traj == "growth":
            traj = paint.good(traj)
        table.append([
            row["civilization"], row["world"],
            traj,
            str(row.get("hunger", "—")),
            str(row.get("rail", "—")),
            str(row.get("labor", 0)),
            str(row.get("ap", "—")),
            condition_label(row, paint),
        ])
    return "\n".join([
        paint.head("ARC — trajectory · hunger · rail · labor (P8.2/P8.3)"),
        tabulate(
            ["CIVILIZATION", "WORLD", "TRAJ", "HUNGER", "RAIL", "LABOR", "AP", "CONDITION"],
            table,
        ) if table else paint.dim("(no settled planets)"),
    ])


VIEWS = {
    "board": view_board, "civs": view_civs, "worlds": view_worlds, "market": view_market,
    "routes": view_routes, "refineries": view_refineries,
    "world": view_world, "civ": view_civ, "tiers": view_tiers,
    "stress": view_stress, "techs": view_techs, "chart": view_chart,
    "ruins": view_ruins, "arc": view_arc,
}


def main():
    parser = argparse.ArgumentParser(
        description="netstratum observatory (read-only, traversable)")
    parser.add_argument("view", nargs="?", default="board",
                        choices=sorted(VIEWS), help="which view to render")
    parser.add_argument("target", nargs="?", default=None,
                        help="world name / civ slug for the drill-downs")
    parser.add_argument("--root", default="var",
                        help="var root holding registry/ and worlds/ (default var)")
    parser.add_argument("--watch", type=float, default=0, metavar="SECONDS")
    parser.add_argument("--sort", default="civilization",
                        choices=["civilization", "world", "planet"] + RESOURCES
                        + ["zones", "clock", "intelligence"])
    parser.add_argument("--tier", default=None,
                        help="filter civs by tier: T0..T9 (or T? for unreadable)")
    parser.add_argument("--tail", type=int, default=8,
                        help="chronicle receipts shown in world view (default 8)")
    parser.add_argument("--plain", action="store_true")
    args = parser.parse_args()
    plain = args.plain or bool(os.environ.get("NO_COLOR")) or not sys.stdout.isatty()
    paint = Paint(plain)
    render_view = VIEWS[args.view]

    if not (Path(args.root) / "worlds").is_dir():
        print(f"(nothing exported under {args.root}/worlds yet — play a session "
              "with the ledger wired, or boot the conductor)")
        return

    if args.watch <= 0:
        print(render_view(args.root, args, paint))
        return
    try:
        while True:
            output = render_view(args.root, args, paint)
            if not plain:
                print("\033[2J\033[H", end="")
            title = "universe" if args.view == "board" else args.view
            print(f"NETSTRATUM OBSERVATORY — {title} — "
                  f"{time.strftime('%H:%M:%S')} — {args.root}/\n")
            print(output)
            time.sleep(args.watch)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
