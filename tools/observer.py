#!/usr/bin/env python3
"""observer.py — the out-of-game observatory, traversable.

THE TOPOLOGY: a CIVILIZATION (registry identity, spans worlds) settles
COLONIES on PLANETS inside a WORLD (one clock, one chronicle, one
receipt chain). The observatory renders VIEWS over that topology —
a dashboard by default, tables and drill-downs by subcommand.

READ-ONLY BY CONSTRUCTION — opens files for reading, never writes.
Standard library only. Exact fractions in, exact fractions out.
Chronicle `detail` strings are DISPLAYED verbatim, never parsed
(the no-grepping law: a parse of a rendering is a second truth).

Usage:
  python3 tools/observer.py                    # the dashboard
  python3 tools/observer.py civs [--tier T0]   # civilizations, by tier
  python3 tools/observer.py worlds             # settled planets table
  python3 tools/observer.py world <name>       # one world, whole
  python3 tools/observer.py civ <slug>         # one line, whole
  python3 tools/observer.py tiers              # the union board T0→T3
  python3 tools/observer.py stress             # the t0 stress read
  python3 tools/observer.py techs              # crystallized nodes
  python3 tools/observer.py chart              # the star chart
  python3 tools/observer.py <view> --watch 2   # any view, refreshed

Chain check: '✓' means every exported receipt's parent digest equals
its predecessor's digest with contiguous sequence numbers — splicing,
deletion, or reordering reads '✗ BROKEN'. (Full BLAKE3 recomputation
lives in-game via `verify`/`replay`; the chronicle is the authority,
the lens is its write-only rendering.)
"""

import argparse
import json
import os
import sys
import time
from fractions import Fraction
from pathlib import Path

RESOURCES = ["iron", "biomass", "alloy", "diamond", "joule", "sink"]
STAR_NAMES = {1: "O", 2: "B", 3: "A", 4: "F", 5: "G", 6: "K", 7: "M",
              8: "bd", 9: "wd", 10: "rg", 11: "ns", 12: "wr", 13: "lwh"}
TIER_OF_STATION = {"progenitor": "T0", "watcher": "T1", "observer": "T1",
                   "sovereign": "T2"}
TIER_TITLES = {"T0": "T0 — THE BOARD-MAKERS (progenitors)",
               "T1": "T1 — THE ARCHITECT OBSERVERS (watchers · observers)",
               "T2": "T2 — THE RECORD KEEPERS (sovereigns)",
               "T3": "T3 — THE BIOSPHERE NODES (the common, lawful state)"}


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


# ── the house wire (state.tlv; landed as "MIRROR-TLV"): the committed orb decoded native ──
# NS-1 TLV: tag u8 ‖ LE32(len) ‖ value; unknown tags SKIP (SEAL A).
# Ratio: sign u8 ‖ LE32(len) ‖ numer BE ‖ LE32(len) ‖ denom BE.

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


RESOURCE_NAMES = {1: "iron", 2: "biomass", 3: "alloy", 4: "diamond",
                  5: "joule", 6: "sink"}


def decode_orb_tlv(path):
    """The committed orb from state.tlv — banked, famine, zones,
    vented — decoded from the canonical bytes. Returns None when no
    carrier stands (the JSON rendering then carries the display)."""
    try:
        data = path.read_bytes()
    except OSError:
        return None
    orb = {"banked": {}, "famine": [], "zones": 0, "vented": Fraction(0)}
    for tag, value in tlv_records(data):
        if tag == 17:
            orb["zones"] += 1
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
        # every other tag: skipped whole (SEAL A) — future fields
        # never break this reader.
    return orb


# ── readers (the lens's data files — data, never prose) ─────────────

def read_jsonl(path):
    if not path.exists():
        return
    with open(path, encoding="utf-8") as stream:
        for line in stream:
            line = line.strip()
            if not line:
                continue
            try:
                yield json.loads(line)
            except json.JSONDecodeError:
                # The torn-tail law (07 §1.2) + live-read safety: an
                # unparseable line is a torn or MID-WRITE append. STOP
                # here — skipping past it would shift the digest chain
                # and falsely report BROKEN on a healthy live world.
                return


def chain_linked(chronicle_path):
    previous_digest, previous_seq, count = None, None, 0
    for receipt in read_jsonl(chronicle_path):
        if previous_digest is not None and receipt.get("parent") != previous_digest:
            return False, count
        if previous_seq is not None and receipt.get("seq") != previous_seq + 1:
            return False, count
        previous_digest = receipt.get("digest")
        previous_seq = receipt.get("seq")
        count += 1
    return True, count


def load_registry(var_root):
    registry = {}
    reg_dir = Path(var_root) / "registry"
    if reg_dir.is_dir():
        for path in sorted(reg_dir.glob("*.json")):
            try:
                card = json.loads(path.read_text(encoding="utf-8"))
                registry[card.get("sigil", card.get("slug", path.stem))] = card
            except json.JSONDecodeError:
                continue
    return registry


def tier_of(card):
    """12_UNION §2a: the TIER is a derived reading, never stored."""
    return TIER_OF_STATION.get(card.get("station"), "T3")


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


# The hardship GRADIENT (operator-ruled 2026-07-30): the endured
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


def world_dirs(var_root):
    worlds_dir = Path(var_root) / "worlds"
    if not worlds_dir.is_dir():
        return []
    return sorted(p for p in worlds_dir.iterdir() if p.is_dir())


def world_facts(world_dir):
    """Everything one world's export carries, gathered once."""
    chronicle = world_dir / "chronicle.jsonl"
    linked, receipts = chain_linked(chronicle)
    consumed = Fraction(0)
    events, terminal_digest = [], ""
    for receipt in read_jsonl(chronicle):
        events.append(receipt)
        terminal_digest = receipt.get("digest") or terminal_digest
        if receipt.get("event") == "yield_collected":
            try:
                consumed += frac(receipt.get("detail", "").split(" ")[0])
            except (ValueError, ZeroDivisionError):
                pass
    puzzles, depth = 0, 0
    solves = world_dir / "solves"
    if solves.is_dir():
        for path in solves.iterdir():
            if path.suffix != ".json":
                continue
            puzzles += 1
            try:
                depth = max(depth, json.loads(
                    path.read_text(encoding="utf-8")).get("depth", 1))
            except json.JSONDecodeError:
                pass
    planets = []
    planets_dir = world_dir / "planets"
    if planets_dir.is_dir():
        for planet_dir in sorted(p for p in planets_dir.iterdir() if p.is_dir()):
            state_path = planet_dir / "state.json"
            if not state_path.exists():
                continue
            try:
                state = json.loads(state_path.read_text(encoding="utf-8"))
            except json.JSONDecodeError:
                continue
            # MIRROR-TLV: the COMMITTED fields prefer the house wire
            # (state.tlv — canonical bytes); the JSON rendering keeps
            # the context (clock, civ, star, offers, readings).
            wire = decode_orb_tlv(planet_dir / "state.tlv")
            if wire is not None:
                state["banked"] = {k: show(v) for k, v in wire["banked"].items()}
                state["famine"] = wire["famine"]
                state["zones"] = state.get("zones", [])
                state["_wire_zones"] = wire["zones"]
                state["entropy_vented"] = show(wire["vented"])
            planets.append(state)
    pending = 0
    inbox = world_dir / "inbox" / "pending"
    if inbox.is_dir():
        pending = sum(1 for p in inbox.iterdir() if p.suffix == ".json")
    return {
        "name": world_dir.name, "linked": linked, "receipts": receipts,
        "events": events, "consumed": consumed, "puzzles": puzzles,
        "depth": depth, "planets": planets, "pending": pending,
        "digest": terminal_digest,
    }


def read_all(var_root):
    """One row per settled planet + the per-world facts, once."""
    worlds, rows = {}, []
    for world_dir in world_dirs(var_root):
        facts = world_facts(world_dir)
        worlds[facts["name"]] = facts
        for state in facts["planets"]:
            banked = state.get("banked", {})
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
            }
            for resource in RESOURCES:
                row[resource] = frac(banked[resource]) if resource in banked else Fraction(0)
            row["intelligence"] = (
                Fraction(facts["puzzles"]) / facts["consumed"]
                if facts["consumed"] > 0 else None)
            rows.append(row)
    return rows, worlds


def load_starchart(var_root):
    chart = {}
    path = Path(var_root) / "worlds" / "starchart.json"
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                card = json.loads(line)
                chart[card["world"]] = (
                    frac(card["x"]), frac(card["y"]), frac(card.get("z", "0")),
                    frac(card.get("g", "0")),
                )
            except (json.JSONDecodeError, KeyError, ValueError):
                continue
    return chart


def universe_card(var_root):
    path = Path(var_root) / "worlds" / "universe.json"
    if path.exists():
        try:
            card = json.loads(path.read_text(encoding="utf-8"))
            return frac(card.get("span", "64")), card.get("tuniversal", "00:00:00:00:00:00")
        except (json.JSONDecodeError, ValueError):
            pass
    return Fraction(64), "00:00:00:00:00:00"


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
    name = card.get("name", slug)
    if card.get("founder") == "conductor":
        name = "⚙ " + name
    return name


def stress_flags(row, paint):
    """The stress read: famine ⚠ · vented ♨ · broken chain ✗."""
    flags = []
    if row["famine"]:
        flags.append(paint.bad("⚠" + ",".join(row["famine"])))
    try:
        if frac(str(row["vented"])) > 0:
            flags.append(paint.c("♨" + str(row["vented"]), "33"))
    except ValueError:
        pass
    if not row["linked"]:
        flags.append(paint.bad("✗chain"))
    return flags


# ── views ───────────────────────────────────────────────────────────

def view_board(var_root, args, paint):
    """The dashboard: the whole board at a glance, then pointers."""
    rows, worlds = read_all(var_root)
    registry = load_registry(var_root)
    span, tuniversal = universe_card(var_root)
    lines = [paint.head("THE BOARD"), ""]
    tiers = {}
    for slug, card in registry.items():
        tiers.setdefault(tier_of(card), []).append(slug)
    tier_line = " · ".join(
        f"{t} {len(tiers.get(t, []))}" for t in ("T0", "T1", "T2", "T3"))
    lines.append(
        f"  TUniversal {paint.strong(tuniversal)} · span {show(span)} · "
        f"{paint.strong(str(len(registry)))} civilizations ({tier_line}) · "
        f"{paint.strong(str(len(worlds)))} worlds · "
        f"{paint.strong(str(len(rows)))} settled planets")
    stressed = [(r, stress_flags(r, paint)) for r in rows]
    stressed = [(r, f) for r, f in stressed if f]
    petitions = sum(w["pending"] for w in worlds.values())
    if stressed:
        lines += ["", paint.head("STRESS — the t0 read")]
        for row, flags in stressed:
            lines.append(f"  {row['world']}/{row['planet']}  {' '.join(flags)}")
    else:
        lines += ["", f"  stress: {paint.good('none — the board breathes')}"]
    lines.append(
        f"  petitions pending upward: {paint.strong(str(petitions))}"
        + ("" if petitions == 0 else "  (t0 inbox in the client)"))
    lines += ["", paint.head("VIEWS"),
              "  " + paint.dim("civs [--tier T0] · worlds · world <name> · civ <slug> · "
                               "market · tiers · stress · techs · chart — any view takes --watch N")]
    return "\n".join(lines)


def view_civs(var_root, args, paint):
    """The civilizations, grouped by tier, filterable."""
    rows, worlds = read_all(var_root)
    registry = load_registry(var_root)
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
        axes = card.get("axes", {})
        by_tier.setdefault(tier, []).append([
            civ_display(card, slug),
            card.get("species", "?"),
            "·".join(axes.get(a, "?")[:3]
                     for a in ["disposition", "metabolism", "perception"]),
            card.get("station", "—") or "—",
            function_of(card, agg),
            creed_of(agg.get("events", []), agg.get("digest", "")),
            str(len(agg["worlds"])), str(agg["colonies"]),
            show(agg["iron"]), show(agg["biomass"]),
            show(agg["joule"]), show(agg["sink"]),
            str(agg["puzzles"]),
            show(Fraction(agg["puzzles"]) / agg["consumed"])
            if agg["consumed"] > 0 else "—",
        ])
    lines = []
    for tier in ("T0", "T1", "T2", "T3"):
        if tier not in by_tier:
            continue
        lines += [paint.head(TIER_TITLES[tier]), tabulate(
            ["CIVILIZATION", "SPECIES", "AXES", "STATION", "FUNCTION", "CREED", "WORLDS", "COLONIES",
             "IRON", "BIOMASS", "JOULE", "SINK", "PUZZLES", "INTELLIGENCE"],
            by_tier[tier]), ""]
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
        planet = row["planet"]
        flags = stress_flags(row, paint)
        if flags:
            planet += " " + " ".join(flags)
        table.append([
            row["civilization"], row["world"], planet,
            show(row["iron"]), show(row["biomass"]),
            show(row["alloy"]), show(row["diamond"]),
            show(row["joule"]), show(row["sink"]),
            STAR_NAMES.get(row["star"], "—"),
            str(row["zones"]), str(row["clock"]),
            paint.good("✓") if row["linked"] else paint.bad("✗ BROKEN"),
        ])
    return "\n".join([paint.head("SETTLED PLANETS"), tabulate(
        ["CIVILIZATION", "WORLD", "PLANET", "IRON", "BIOMASS", "ALLOY",
         "DIAMOND", "JOULE", "SINK", "STAR", "ZONES", "TICKS", "CHAIN"],
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
    chain = paint.good("✓ intact") if facts["linked"] else paint.bad("✗ BROKEN")
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
        star = STAR_NAMES.get(state.get("star_class"), "—")
        lines += ["", paint.head(
            f"  PLANET {state.get('planet', '?')} — {state.get('civilization', '?')}"),
            f"    star {star} · clock {state.get('clock', 0)} · "
            f"{len(state.get('zones', []))} zone(s) · vented {state.get('entropy_vented', '0')}",
            f"    {holdings}"]
        if state.get("famine"):
            lines.append("    " + paint.bad("⚠ famine: " + ", ".join(state["famine"])))
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
                 + f" · axes {'·'.join(axes.get(a, '?') for a in ['disposition', 'metabolism', 'perception'])} · "
                 f"station {card.get('station', '—') or '—'}")
    if card.get("motto"):
        lines.append(f"  {paint.dim(chr(0x201C) + card['motto'] + chr(0x201D))}")
    # The lineage walk (LINEAGE-2): the pointer is a space-joined
    # LIST — walk the PRIMARY chain (first parent per hop), and mark
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
            flags = stress_flags(row, paint)
            lines.append(
                f"    {row['world']}/{row['planet']} · "
                f"{STAR_NAMES.get(row['star'], '—')} star · "
                f"clock {row['clock']} · {row['zones']} zone(s)"
                + ("  " + " ".join(flags) if flags else ""))
    else:
        lines.append("  no settled planets on the board")
    return "\n".join(lines)


def view_tiers(var_root, args, paint):
    """The union board T0→T3 — who stands where, seats by station."""
    registry = load_registry(var_root)
    lines = []
    for tier in ("T0", "T1", "T2", "T3"):
        members = sorted(
            slug for slug, card in registry.items() if tier_of(card) == tier)
        lines.append(paint.head(TIER_TITLES[tier]))
        if members:
            for slug in members:
                card = registry[slug]
                lines.append(
                    f"  {civ_display(card, slug):<24} "
                    f"{paint.dim(card.get('species', '?'))}")
        else:
            lines.append("  " + paint.dim("(no one stands here yet)"))
        lines.append("")
    lines.append(paint.dim("seats are space, not rank — the rooms and their "
                           "widths live in the client: civ seats"))
    return "\n".join(lines)


# MANU-1 (ruled 2026-07-31): reach = BASE_REACH + instrument grade —
# the global constant is struck; the unaided horizon is the g=0 read.
# Until InstrumentForged receipts land engine-side, every world reads
# unaided (grade 0) through the same law.
BASE_REACH = 16


def epistemic_reach(state):
    """The world's scan horizon: base + its highest instrument grade
    (a data field on state.json once MANU-1's engine deed lands)."""
    grades = [int(i.get("grade", 0)) for i in state.get("instruments", [])]         if isinstance(state, dict) else []
    return BASE_REACH + max(grades, default=0)


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
        linked, _ = chain_linked(world_dir / "chronicle.jsonl")
        if not linked:
            corrupt.append(name)
    reads = []
    for name, coords in chart.items():
        if name in corrupt:
            continue
        # MANU-1: this world's OWN horizon — its instruments decide.
        state = {}
        for state_path in sorted((Path(var_root) / "worlds" / name / "planets").glob("*/state.json")):
            try:
                state = json.loads(state_path.read_text())
            except (OSError, ValueError):
                state = {}
            break
        reach = epistemic_reach(state)
        near = [
            other for other in corrupt
            if other in chart
            and toroidal_l1(coords[:3], chart[other][:3], span) <= reach
        ]
        if near:
            reads.append(
                f"{name}  {paint.bad('◌epistemological')} — "
                f"{len(near)} corrupted chronicle(s) in reach: "
                + paint.dim(", ".join(display_name(n) for n in near)))
    return reads


def view_stress(var_root, args, paint):
    """The t0 stress read, alone and whole."""
    rows, worlds = read_all(var_root)
    lines = [paint.head("STRESS — the t0 read")]
    any_stress = False
    for line in epistemic_reads(var_root, paint):
        any_stress = True
        lines.append(f"  {line}")
    for row in rows:
        flags = stress_flags(row, paint)
        if flags:
            any_stress = True
            lines.append(f"  {row['world']}/{row['planet']}  {' '.join(flags)}")
    for name, facts in sorted(worlds.items()):
        if facts["pending"]:
            any_stress = True
            lines.append(
                f"  {name}  {paint.strong(str(facts['pending']))} petition(s) pending")
    if not any_stress:
        lines.append("  " + paint.good("none — the board breathes"))
    lines.append(paint.dim(
        "  deeds: t0 move (separate) · licensed leap (relieve) · t0 contract (seal)"))
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


VIEWS = {
    "board": view_board, "civs": view_civs, "worlds": view_worlds, "market": view_market,
    "world": view_world, "civ": view_civ, "tiers": view_tiers,
    "stress": view_stress, "techs": view_techs, "chart": view_chart,
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
                        help="filter civs by tier: T0..T3")
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
            print(f"NETSTRATUM OBSERVATORY — {args.view} — "
                  f"{time.strftime('%H:%M:%S')} — {args.root}/\n")
            print(output)
            time.sleep(args.watch)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
