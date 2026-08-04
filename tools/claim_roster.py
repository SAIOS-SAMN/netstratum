#!/usr/bin/env python3
"""THE CLAIM PATH — merge the live civilization record into the durable roster.

Replaces `install.sh export-roster`, which could not do this job three
ways over (docs/ENDOWMENTS.md E4): it dies on a `var/polities.json` that
has not existed since 2026-07-30, it reads a file the civilizations are
not in, and it is a `cp` — so it would delete the durable-only lines
outright.

THE IDENTITY IS `slug`. Never `name`, never the filename. Measured
2026-08-03: every seeds entry and every registry file carries `slug`,
and the registry's slug equals its filename in all 33 cases. Comparing
on `name` manufactures deaths and births that never happened — three
lines have been renamed (Eightfold to Brine Cortex, Deep Chorus to
Echowright, Hivelight to Vitreous Hex) with the slug held on both sides.

NOTHING IS EVER DELETED. Every slug present in the durable roster
survives the merge, whether or not the live universe knows it. A claim
that can drop a civilization is a wipe wearing an export's name.

REFUSE, NEVER GUESS. `conductor::load_polities` requires name, sigil-or-
slug, disposition, metabolism, perception, world, planet, colony and a
parseable policy; an entry missing any of them is SKIPPED at load with
only a stderr note. So an entry this tool cannot complete from real data
is REFUSED and reported by name — never written with an invented world,
which would read as a claim while being silently dropped by the engine.

Dry run by default. `--write` is the only thing that touches the roster.
"""

import argparse
import json
import os
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SEEDS = os.path.join(REPO, "seeds", "aeon1-polities.json")

# cfe::thermal::StarClass is 1-BASED (thermal.rs: O = 1 .. M = 7), and
# lens/render.rs writes `class as u8` into the planet state. The word
# forms are conductor/src/polities.rs::star_word.
STAR_WORDS = [
    "o", "b", "a", "f", "g", "k", "m",
    "brown-dwarf", "white-dwarf", "red-giant",
    "neutron-star", "wolf-rayet", "luminal-white-hole",
]

# Every field conductor::load_polities takes with `?`. Missing any one of
# these means the entry is skipped at load, so we refuse it here instead.
REQUIRED = ("name", "slug", "disposition", "metabolism", "perception",
            "world", "planet", "colony", "policy")

# The order write_default_polities emits, so a merged file reads like a
# seeded one. Optional tail fields follow, emitted only when held.
FIELD_ORDER = ("name", "slug", "species", "clade", "motto",
               "disposition", "metabolism", "perception",
               "world", "planet", "colony", "policy",
               "x", "y", "z",
               "station", "lineage", "star", "albedo", "orbit", "gauge")


def star_word(code):
    """StarClass ordinal to its roster word; None if out of range."""
    if isinstance(code, int) and 1 <= code <= len(STAR_WORDS):
        return STAR_WORDS[code - 1]
    return None


def load_seeds(path):
    with open(path) as handle:
        entries = json.load(handle)
    return {entry["slug"]: dict(entry) for entry in entries}


def load_registry(var_root):
    """The live civilization record — one file per line, keyed by slug.

    Axes are NESTED under `axes` here and FLAT in the roster; flatten on
    the way in so the two shapes are compared on the same terms.
    """
    registry = {}
    directory = os.path.join(var_root, "registry")
    if not os.path.isdir(directory):
        return registry
    for name in sorted(os.listdir(directory)):
        if not name.endswith(".json"):
            continue
        with open(os.path.join(directory, name)) as handle:
            record = json.load(handle)
        slug = record.get("slug") or name[:-5]
        flat = {"slug": slug}
        for key in ("name", "species", "clade", "motto", "founder", "station"):
            if record.get(key):
                flat[key] = record[key]
        for key, value in (record.get("axes") or {}).items():
            if value:
                flat[key] = value
        lineage = record.get("lineage")
        if lineage:
            flat["lineage"] = lineage if isinstance(lineage, str) else " ".join(lineage)
        registry[slug] = flat
    return registry


def load_siting(var_root):
    """Where each line actually sits, read from the live universe.

    The registry cannot express siting and the roster can, so for a line
    the roster has never heard of, the world itself is the only source:
    `var/worlds/<world>/planets/<planet>/state.json` names its
    civilization slug, and starchart.json carries the coordinates.
    """
    siting, chart, charted_dirs = {}, {}, set()
    worlds = os.path.join(var_root, "worlds")
    if not os.path.isdir(worlds):
        return siting, chart, charted_dirs

    chart_path = os.path.join(worlds, "starchart.json")
    if os.path.isfile(chart_path):
        with open(chart_path) as handle:
            for line in handle:
                line = line.strip()
                if line:
                    row = json.loads(line)
                    chart[row["world"]] = row

    for world in sorted(os.listdir(worlds)):
        planets = os.path.join(worlds, world, "planets")
        if not os.path.isdir(planets):
            continue
        charted_dirs.add(world)
        for planet in sorted(os.listdir(planets)):
            state_path = os.path.join(planets, planet, "state.json")
            if not os.path.isfile(state_path):
                continue
            with open(state_path) as handle:
                state = json.load(handle)
            slug = state.get("civilization")
            if not slug:
                continue
            found = {"world": state.get("world") or world,
                     "planet": state.get("planet") or planet,
                     "colony": state.get("colony")}
            word = star_word(state.get("star_class"))
            if word:
                found["star"] = word
            row = chart.get(found["world"])
            if row:
                for axis in ("x", "y", "z"):
                    if row.get(axis) is not None:
                        found[axis] = row[axis]
            siting[slug] = found
    return siting, chart, charted_dirs


def merge(seeds, registry, siting, default_policy):
    """Union on slug. Returns (entries, report).

    Precedence, and the reasoning for each:
      name        registry wins — the sigil rename landed live first, so
                  the live record is the LATER authority on display name.
      axes/lore   registry wins for the same reason, seeds as fallback.
      siting      SEEDS wins — the roster is the seating plan and its
                  coordinates are the deliberate ones; the live universe
                  only fills in for lines the roster never had.
      policy      seeds only. It is a conductor behaviour, not a fact
                  about the world, and nothing in live state records it.
    """
    report = {"merged": [], "claimed": [], "preserved": [], "refused": []}
    entries = {}

    for slug in sorted(set(seeds) | set(registry)):
        seed = seeds.get(slug)
        live = registry.get(slug)
        entry = dict(seed) if seed else {"slug": slug}

        if live:
            for key in ("name", "species", "clade", "motto", "founder",
                        "station", "lineage", "disposition", "metabolism",
                        "perception"):
                if live.get(key):
                    entry[key] = live[key]

        # Siting only from the live universe when the roster lacks it —
        # a deliberate seat is never overwritten by an observed one.
        if not seed:
            found = siting.get(slug)
            if found:
                for key, value in found.items():
                    if value is not None:
                        entry[key] = value
            entry.setdefault("policy", default_policy)

        missing = [key for key in REQUIRED if not entry.get(key)]
        if missing:
            # Refused, not written. A roster entry the engine would skip
            # is worse than an absent one: it reads as claimed.
            report["refused"].append((slug, entry.get("name", slug), missing))
            continue

        entries[slug] = entry
        if seed and live:
            renamed = seed.get("name") != entry.get("name")
            report["merged"].append((slug, seed.get("name"), entry["name"], renamed))
        elif live:
            report["claimed"].append((slug, entry["name"], entry["world"]))
        else:
            report["preserved"].append((slug, entry["name"], entry.get("world")))

    return entries, report


def render(entries):
    """Emit the flat one-object-per-line shape write_default_polities
    writes and load_polities parses (brace-counted, so no nesting and no
    braces inside strings)."""
    lines = ["["]
    slugs = sorted(entries)
    for index, slug in enumerate(slugs):
        entry = entries[slug]
        pairs = []
        for key in FIELD_ORDER:
            value = entry.get(key)
            if value is None or value == "":
                continue
            text = str(value)
            if '"' in text or "{" in text or "}" in text:
                raise ValueError(f"{slug}.{key} carries a brace or quote: {text!r}")
            pairs.append(f'"{key}":"{text}"')
        # Any field the roster held that this tool does not know about is
        # carried through rather than dropped — the record loses nothing
        # by passing through a merge.
        for key in sorted(entry):
            if key not in FIELD_ORDER and entry.get(key) not in (None, ""):
                text = str(entry[key])
                if '"' not in text and "{" not in text and "}" not in text:
                    pairs.append(f'"{key}":"{text}"')
        comma = "," if index + 1 < len(slugs) else ""
        lines.append("  {" + ",".join(pairs) + "}" + comma)
    lines.append("]")
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description="Claim the live civilization record into the durable roster.")
    parser.add_argument("--var", default="var", help="universe root (default: var)")
    parser.add_argument("--roster", default=SEEDS, help="durable roster to merge into")
    parser.add_argument("--policy", default="steady",
                        help="policy for claimed lines that have none (default: steady)")
    parser.add_argument("--write", action="store_true", help="write the roster (default: dry run)")
    args = parser.parse_args()

    var_root = args.var if os.path.isabs(args.var) else os.path.join(REPO, args.var)

    if args.policy not in ("steady", "aggressive", "industry"):
        print(f"claim_roster: policy '{args.policy}' is not one conductor::Policy::parse accepts")
        return 0
    if not os.path.isfile(args.roster):
        print(f"claim_roster: no roster at {args.roster}")
        return 0

    seeds = load_seeds(args.roster)
    registry = load_registry(var_root)
    siting, chart, world_dirs = load_siting(var_root)
    if not registry:
        print(f"claim_roster: no registry under {var_root}/registry — nothing to claim")
        return 0

    entries, report = merge(seeds, registry, siting, args.policy)

    print(f"roster:    {args.roster} ({len(seeds)} lines)")
    print(f"registry:  {var_root}/registry ({len(registry)} lines)")
    print(f"result:    {len(entries)} lines\n")

    renamed = [row for row in report["merged"] if row[3]]
    print(f"MERGED    {len(report['merged'])} lines held by both, {len(renamed)} renamed")
    for slug, was, now, changed in report["merged"]:
        if changed:
            print(f"    {slug:<16} {was}  ->  {now}")
    print(f"\nCLAIMED   {len(report['claimed'])} live lines the roster never recorded")
    for slug, name, world in report["claimed"]:
        print(f"    {slug:<16} {name:<16} {world}")
    print(f"\nPRESERVED {len(report['preserved'])} roster lines the live universe does not hold")
    for slug, name, world in report["preserved"]:
        print(f"    {slug:<16} {name:<16} {world or '(unsited)'}")

    if report["refused"]:
        print(f"\nREFUSED   {len(report['refused'])} lines — NOT written, and not lost:")
        for slug, name, missing in report["refused"]:
            print(f"    {slug:<16} {name:<16} missing {', '.join(missing)}")
        # A charted world with no directory is the likely home of a
        # refused line, but likely is not measured — so it is reported
        # for a ruling and never silently adopted.
        orphan_charts = sorted(set(chart) - world_dirs)
        if orphan_charts:
            print(f"    charted worlds with no directory: {', '.join(orphan_charts)}")
            print("    (a candidate home for the refused lines — NOT assumed here)")

    if not args.write:
        print("\ndry run — nothing written. Pass --write to claim.")
        return 0

    text = render(entries)
    # Re-read what we are about to replace and prove nothing vanishes.
    survivors = {entry["slug"] for entry in json.loads(text)}
    lost = sorted(set(seeds) - survivors)
    if lost:
        print(f"\nREFUSING TO WRITE — the merge would drop {lost}")
        return 0
    with open(args.roster, "w") as handle:
        handle.write(text)
    print(f"\nwritten: {args.roster}")
    print("review with 'git diff seeds/' before committing")
    return 0


if __name__ == "__main__":
    sys.exit(main())
