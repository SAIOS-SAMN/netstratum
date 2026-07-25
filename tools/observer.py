#!/usr/bin/env python3
"""observer.py — the out-of-game observatory.

Watches every civilization mirrored under var/worlds/ and renders a
civilization-centric dashboard: resources, zones, clock, puzzles
confirmed, and the intelligence ratio. READ-ONLY BY CONSTRUCTION —
this script opens files for reading and never writes anything.

Standard library only. Exact fractions in, exact fractions out —
no float ever touches a game quantity.

Usage:
  python3 tools/observer.py                 # one shot, default root
  python3 tools/observer.py --watch 2       # refresh every 2 seconds
  python3 tools/observer.py --root var/worlds --sort intelligence

Chain check: '✓ linked' means every mirrored receipt's parent digest
equals its predecessor's digest and sequence numbers are contiguous —
reordering, deletion, or splicing shows as '✗ BROKEN'. (Full BLAKE3
recomputation lives in-game via `verify`/`replay`; the mirror is a
reflection, the chronicle is the authority.)
"""

import argparse
import json
import os
import sys
import time
from fractions import Fraction
from pathlib import Path

RESOURCES = ["iron", "biomass", "alloy", "diamond"]


def frac(text):
    """Parse an exact mirror quantity ('9/2' or '12') as a Fraction."""
    if isinstance(text, str) and "/" in text:
        numer, denom = text.split("/", 1)
        return Fraction(int(numer), int(denom))
    return Fraction(int(text))


def show(fraction):
    """Render a Fraction the netstratum way: whole if whole, else p/q."""
    if fraction.denominator == 1:
        return str(fraction.numerator)
    return f"{fraction.numerator}/{fraction.denominator}"


def chain_linked(chronicle_path):
    """Structural chain check over the mirrored receipt stream."""
    if not chronicle_path.exists():
        return True, 0
    previous_digest, previous_seq, count = None, None, 0
    with open(chronicle_path, encoding="utf-8") as stream:
        for line in stream:
            line = line.strip()
            if not line:
                continue
            receipt = json.loads(line)
            if previous_digest is not None and receipt.get("parent") != previous_digest:
                return False, count
            if previous_seq is not None and receipt.get("seq") != previous_seq + 1:
                return False, count
            previous_digest = receipt.get("digest")
            previous_seq = receipt.get("seq")
            count += 1
    return True, count


def collected_total(chronicle_path):
    """Sum of yield_collected amounts across all resources (the
    resources-through-the-economy denominator; per-resource weighting
    arrives with the disposition landings)."""
    total = Fraction(0)
    if not chronicle_path.exists():
        return total
    with open(chronicle_path, encoding="utf-8") as stream:
        for line in stream:
            line = line.strip()
            if not line:
                continue
            receipt = json.loads(line)
            if receipt.get("event") == "yield_collected":
                amount = receipt.get("detail", "").split(" ")[0]
                try:
                    total += frac(amount)
                except (ValueError, ZeroDivisionError):
                    pass
    return total


def read_civilization(civ_dir):
    """One dashboard row from one mirrored civilization."""
    state_path = civ_dir / "state.json"
    if not state_path.exists():
        return None
    try:
        state = json.loads(state_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return None  # a torn read can't happen (atomic rename), but a foreign file could sit here

    banked = state.get("banked", {})
    row = {
        "civilization": state.get("civilization", civ_dir.name),
        "clock": state.get("clock", 0),
        "zones": len(state.get("zones", [])),
        "famine": state.get("famine", []),
        "receipts": state.get("receipts", 0),
    }
    for resource in RESOURCES:
        row[resource] = frac(banked[resource]) if resource in banked else Fraction(0)

    solves_dir = civ_dir / "solves"
    row["puzzles"] = (
        sum(1 for p in solves_dir.iterdir() if p.suffix == ".json")
        if solves_dir.is_dir()
        else 0
    )

    linked, mirrored = chain_linked(civ_dir / "chronicle.jsonl")
    row["linked"] = linked
    row["mirrored"] = mirrored

    consumed = collected_total(civ_dir / "chronicle.jsonl")
    row["intelligence"] = (
        Fraction(row["puzzles"]) / consumed if consumed > 0 else None
    )
    return row


def color(text, code, plain):
    if plain:
        return text
    return f"\033[{code}m{text}\033[0m"


def render(rows, sort_key, plain):
    headers = [
        "CIVILIZATION", "IRON", "BIOMASS", "ALLOY", "DIAMOND",
        "ZONES", "TICKS", "PUZZLES", "INTELLIGENCE", "CHAIN",
    ]

    def sort_value(row):
        value = row.get(sort_key, 0)
        if sort_key == "intelligence":
            return value if value is not None else Fraction(-1)
        if sort_key == "civilization":
            return value
        return value

    rows = sorted(rows, key=sort_value, reverse=(sort_key != "civilization"))

    table = []
    for row in rows:
        chain = "✓ linked" if row["linked"] else "✗ BROKEN"
        if not plain:
            chain = color(chain, "32" if row["linked"] else "31", plain)
        name = row["civilization"]
        if row["famine"]:
            name += " ⚠" + ",".join(row["famine"])
        table.append([
            name,
            show(row["iron"]), show(row["biomass"]),
            show(row["alloy"]), show(row["diamond"]),
            str(row["zones"]), str(row["clock"]), str(row["puzzles"]),
            show(row["intelligence"]) if row["intelligence"] is not None else "—",
            chain,
        ])

    widths = [
        max(len(strip_ansi(cell)) for cell in [headers[i]] + [r[i] for r in table])
        for i in range(len(headers))
    ] if table else [len(h) for h in headers]

    lines = ["  ".join(h.ljust(widths[i]) for i, h in enumerate(headers))]
    for r in table:
        lines.append("  ".join(pad(cell, widths[i]) for i, cell in enumerate(r)))
    return "\n".join(lines)


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


def pad(cell, width):
    return cell + " " * (width - len(strip_ansi(cell)))


def observe(root, sort_key, plain):
    root = Path(root)
    if not root.is_dir():
        return f"(no worlds mirrored yet — {root} does not exist; play a session with the mirror wired)"
    rows = [
        row
        for civ_dir in sorted(root.iterdir())
        if civ_dir.is_dir()
        for row in [read_civilization(civ_dir)]
        if row is not None
    ]
    if not rows:
        return f"(no civilizations under {root} yet)"
    return render(rows, sort_key, plain)


def main():
    parser = argparse.ArgumentParser(description="netstratum observatory (read-only)")
    parser.add_argument("--root", default="var/worlds", help="mirror root (default var/worlds)")
    parser.add_argument("--watch", type=float, default=0, metavar="SECONDS",
                        help="refresh interval; omit for one shot")
    parser.add_argument("--sort", default="civilization",
                        choices=["civilization", "iron", "biomass", "alloy", "diamond",
                                 "zones", "clock", "puzzles", "intelligence"])
    parser.add_argument("--plain", action="store_true", help="no color (also honors NO_COLOR)")
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
            print(f"NETSTRATUM OBSERVATORY — {time.strftime('%H:%M:%S')} — root {args.root}\n")
            print(output)
            time.sleep(args.watch)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
