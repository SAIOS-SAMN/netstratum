#!/usr/bin/env python3
"""observer.py — the out-of-game observatory.

THE TOPOLOGY: a CIVILIZATION (registry identity, spans worlds) settles
COLONIES on PLANETS inside a WORLD (one clock, one chronicle, one
receipt chain). This dashboard renders one row per settled planet,
then aggregates per civilization.

READ-ONLY BY CONSTRUCTION — opens files for reading, never writes.
Standard library only. Exact fractions in, exact fractions out.

Usage:
  python3 tools/observer.py                 # one shot, root ./var
  python3 tools/observer.py --watch 2       # refresh every 2 seconds
  python3 tools/observer.py --sort intelligence

Chain check: '✓' means every mirrored receipt's parent digest equals
its predecessor's digest with contiguous sequence numbers — splicing,
deletion, or reordering reads '✗ BROKEN'. (Full BLAKE3 recomputation
lives in-game via `verify`/`replay`; the chronicle is the authority,
the mirror is its reflection.)
"""

import argparse
import json
import os
import sys
import time
from fractions import Fraction
from pathlib import Path

RESOURCES = ["iron", "biomass", "alloy", "diamond", "joule", "sink"]


def frac(text):
    if isinstance(text, str) and "/" in text:
        numer, denom = text.split("/", 1)
        return Fraction(int(numer), int(denom))
    return Fraction(int(text))


def show(fraction):
    if fraction.denominator == 1:
        return str(fraction.numerator)
    return f"{fraction.numerator}/{fraction.denominator}"


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


def collected_total(chronicle_path):
    """Resources-through-the-economy: the sum of yield_collected
    amounts (per-resource disposition weights arrive with G-landings)."""
    total = Fraction(0)
    for receipt in read_jsonl(chronicle_path):
        if receipt.get("event") == "yield_collected":
            try:
                total += frac(receipt.get("detail", "").split(" ")[0])
            except (ValueError, ZeroDivisionError):
                pass
    return total


def load_registry(var_root):
    """slug -> civilization card."""
    registry = {}
    reg_dir = Path(var_root) / "registry"
    if reg_dir.is_dir():
        for path in sorted(reg_dir.glob("*.json")):
            try:
                card = json.loads(path.read_text(encoding="utf-8"))
                registry[card.get("slug", path.stem)] = card
            except json.JSONDecodeError:
                continue
    return registry


def read_worlds(var_root):
    """One row per settled planet, plus per-world facts."""
    rows = []
    worlds_dir = Path(var_root) / "worlds"
    if not worlds_dir.is_dir():
        return rows
    for world_dir in sorted(p for p in worlds_dir.iterdir() if p.is_dir()):
        linked, _ = chain_linked(world_dir / "chronicle.jsonl")
        consumed = collected_total(world_dir / "chronicle.jsonl")
        solves = world_dir / "solves"
        puzzles, depth = 0, 0
        if solves.is_dir():
            for path in solves.iterdir():
                if path.suffix != ".json":
                    continue
                puzzles += 1
                try:
                    depth = max(depth, json.loads(path.read_text(encoding="utf-8")).get("depth", 1))
                except json.JSONDecodeError:
                    pass
        planets_dir = world_dir / "planets"
        if not planets_dir.is_dir():
            continue
        for planet_dir in sorted(p for p in planets_dir.iterdir() if p.is_dir()):
            state_path = planet_dir / "state.json"
            if not state_path.exists():
                continue
            try:
                state = json.loads(state_path.read_text(encoding="utf-8"))
            except json.JSONDecodeError:
                continue
            banked = state.get("banked", {})
            row = {
                "civilization": state.get("civilization", "?"),
                "world": state.get("world", world_dir.name),
                "planet": state.get("planet", planet_dir.name),
                "clock": state.get("clock", 0),
                "zones": len(state.get("zones", [])),
                "famine": state.get("famine", []),
                "puzzles": puzzles,
                "depth": depth,
                "consumed": consumed,
                "linked": linked,
                "star": state.get("star_class"),
                "vented": state.get("entropy_vented", "0"),
                "syntropy": state.get("syntropy_ratio", "—"),
            }
            for resource in RESOURCES:
                row[resource] = frac(banked[resource]) if resource in banked else Fraction(0)
            row["intelligence"] = (
                Fraction(puzzles) / consumed if consumed > 0 else None
            )
            rows.append(row)
    return rows


def aggregate_by_civilization(rows, registry):
    """Sum a civilization's holdings across every world it settles."""
    civs = {}
    for row in rows:
        agg = civs.setdefault(row["civilization"], {
            "colonies": 0, "puzzles": 0, "depth": 0, "consumed": Fraction(0),
            "worlds": set(),
            **{r: Fraction(0) for r in RESOURCES},
        })
        agg["colonies"] += 1
        for resource in RESOURCES:
            agg[resource] += row[resource]
        if row["world"] not in agg["worlds"]:
            agg["worlds"].add(row["world"])
            agg["puzzles"] += row["puzzles"]
            agg["depth"] = max(agg["depth"], row["depth"])
            agg["consumed"] += row["consumed"]
    out = []
    for slug, agg in sorted(civs.items()):
        card = registry.get(slug, {})
        axes = card.get("axes", {})
        name = card.get("name", slug)
        if card.get("founder") == "conductor":
            name = "⚙ " + name
        if card.get("lineage"):
            name += " ↳ " + card["lineage"]
        # 12_UNION §2a: the TIER is a derived reading, never stored —
        # station progenitor→T0, watcher/observer→T1, sovereign→T2,
        # stationless→T3 (the biosphere, the common lawful state).
        tier = {"progenitor": "T0", "watcher": "T1", "observer": "T1",
                "sovereign": "T2"}.get(card.get("station"), "T3")
        out.append({
            "civilization": name,
            "tier": tier,
            "species": card.get("species", "?"),
            "axes": "·".join(axes.get(a, "?")[:3] for a in
                             ["disposition", "metabolism", "perception"]),
            "worlds": len(agg["worlds"]),
            "colonies": agg["colonies"],
            **{r: agg[r] for r in RESOURCES},
            "puzzles": agg["puzzles"],
            "depth": agg["depth"],
            "intelligence": (Fraction(agg["puzzles"]) / agg["consumed"]
                             if agg["consumed"] > 0 else None),
        })
    return out


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


def render(rows, registry, sort_key, plain):
    def paint(text, code):
        return text if plain else f"\033[{code}m{text}\033[0m"

    def sort_value(row):
        value = row.get(sort_key, 0)
        return value if value is not None else Fraction(-1)

    rows = sorted(rows, key=sort_value,
                  reverse=(sort_key not in ("civilization", "world", "planet")))

    colony_table = []
    for row in rows:
        planet = row["planet"]
        if row["famine"]:
            planet += paint(" ⚠" + ",".join(row["famine"]), "31")
        star_names = {1: "O", 2: "B", 3: "A", 4: "F", 5: "G", 6: "K", 7: "M",
                      8: "bd", 9: "wd", 10: "rg", 11: "ns", 12: "wr", 13: "lwh"}
        colony_table.append([
            row["civilization"], row["world"], planet,
            show(row["iron"]), show(row["biomass"]),
            show(row["alloy"]), show(row["diamond"]),
            show(row["joule"]), show(row["sink"]),
            star_names.get(row["star"], "—"),
            str(row["vented"]),
            str(row["zones"]), str(row["clock"]),
            paint("✓", "32") if row["linked"] else paint("✗ BROKEN", "31"),
        ])
    sections = ["SETTLED PLANETS", tabulate(
        ["CIVILIZATION", "WORLD", "PLANET", "IRON", "BIOMASS", "ALLOY",
         "DIAMOND", "JOULE", "SINK", "STAR", "VENTED", "ZONES", "TICKS",
         "CHAIN"],
        colony_table)]

    civ_table = [
        [agg["civilization"], agg["tier"], agg["species"], agg["axes"],
         str(agg["worlds"]), str(agg["colonies"]),
         show(agg["iron"]), show(agg["biomass"]),
         show(agg["alloy"]), show(agg["diamond"]),
         str(agg["puzzles"]), str(agg["depth"]),
         show(agg["intelligence"]) if agg["intelligence"] is not None else "—"]
        for agg in aggregate_by_civilization(rows, registry)
    ]
    sections += ["", "CIVILIZATIONS", tabulate(
        ["CIVILIZATION", "TIER", "SPECIES", "AXES", "WORLDS", "COLONIES", "IRON",
         "BIOMASS", "ALLOY", "DIAMOND", "PUZZLES", "DEPTH", "INTELLIGENCE"],
        civ_table)]
    return "\n".join(sections)


def load_starchart(var_root):
    """world -> (Fraction x, Fraction y) from the chart's JSONL."""
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
    """span + TUniversal from universe.json."""
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


def render_starchart(chart, span, tuniversal):
    """An ascii x/y projection of the 3-torus (edges wrap) plus exact
    TUniversal{S, G} coordinates and nearest neighbors by L1 (no
    squaring in a toroid)."""
    if not chart:
        return ""
    names = sorted(chart)
    lines = ["", f"STAR CHART — 3-torus, span {show(span)} (edges wrap) — TUniversal {tuniversal}"]

    # The ascii canvas, worlds marked a, b, c…
    width, height = 49, 13

    def label(i):
        """Unambiguous marks: a..z, then aa, ab, … — never recycled."""
        out = ""
        i += 1
        while i > 0:
            i, r = divmod(i - 1, 26)
            out = chr(ord("a") + r) + out
        return out

    canvas = [[" "] * width for _ in range(height)]
    occupied = {}
    marks = []
    for i, name in enumerate(names):
        x, y = chart[name][0], chart[name][1]
        col = int((x % span) / span * width) % width
        row = int((span - (y % span)) / span * height) % height
        mark = label(i)
        if (row, col) in occupied:
            # More than one world in this cell: the map shows a stack
            # marker; the legend carries the truth.
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

    # Exact coordinates and nearest neighbor by squared distance.
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
               if best is not None else "")
        )
    return "\n".join(lines)


def display_name(name):
    """Display-only beautification — the chart KEY is the identity and
    never changes. Annexation prefixes compact to era glyphs; the old
    pre-themed-naming ancestry chains (archived eras only) ellipsize to
    first…last segment. `07` annex law: names are preserved on disk;
    only the rendering breathes."""
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


def observe(var_root, sort_key, plain):
    rows = read_worlds(var_root)
    if not rows:
        return (f"(nothing mirrored under {var_root}/worlds yet — play a session "
                "with the mirror wired, or run the mirror_demo)")
    out = render(rows, load_registry(var_root), sort_key, plain)
    span, tuniversal = universe_card(var_root)
    chart = render_starchart(load_starchart(var_root), span, tuniversal)
    return out + chart if chart else out


def main():
    parser = argparse.ArgumentParser(description="netstratum observatory (read-only)")
    parser.add_argument("--root", default="var",
                        help="var root holding registry/ and worlds/ (default var)")
    parser.add_argument("--watch", type=float, default=0, metavar="SECONDS")
    parser.add_argument("--sort", default="civilization",
                        choices=["civilization", "world", "planet"] + RESOURCES
                        + ["zones", "clock", "intelligence"])
    parser.add_argument("--plain", action="store_true")
    args = parser.parse_args()
    plain = args.plain or bool(os.environ.get("NO_COLOR")) or not sys.stdout.isatty()

    if args.watch <= 0:
        print(observe(args.root, args.sort, plain))
        return
    try:
        while True:
            output = observe(args.root, args.sort, plain)
            if not plain:
                print("\033[2J\033[H", end="")
            print(f"NETSTRATUM OBSERVATORY — {time.strftime('%H:%M:%S')} — {args.root}/\n")
            print(output)
            time.sleep(args.watch)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
