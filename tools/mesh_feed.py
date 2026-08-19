#!/usr/bin/env python3
"""Export netstratum forge solves as mycelium feed claims.

Kernel-side of the membrane: reads var/worlds/*/solves/*.sol, dedupes
the transform vocabulary, synthesizes one canonical pair-grid claim per
transform, and writes ARC-shaped JSON into var/mesh-feed/. Then:

  mycelium-uplink --kernel net --feed /opt/netstratum/var/mesh-feed

v1 exports the D4 family (rotate90/180/270, flip_h, flip_v, transpose)
and single-layer per_layer() of the same. recolor compositions and
tile*() claims are counted and skipped — named for a later pass, not
silently dropped.
"""
import glob
import json
import os
import re
from collections import Counter

SOLVES = "/opt/netstratum/var/worlds/*/solves/*.sol"
FEED = "/opt/netstratum/var/mesh-feed"

BASE = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

D4 = {
    "rotate90": lambda g: [list(r) for r in zip(*g[::-1])],
    "rotate180": lambda g: [r[::-1] for r in g[::-1]],
    "rotate270": lambda g: [list(r) for r in zip(*g)][::-1],
    "flip_h": lambda g: [r[::-1] for r in g],
    "flip_v": lambda g: g[::-1],
    "transpose": lambda g: [list(r) for r in zip(*g)],
}


def transforms():
    seen = Counter()
    for f in glob.glob(SOLVES):
        raw = open(f, "rb").read()
        text = "".join(chr(b) if 32 <= b < 127 else " " for b in raw)
        m = re.search(
            r"((?:rotate90|rotate180|rotate270|flip_h|flip_v|transpose"
            r"|per_layer\([a-z_0-9]+\)|tile\dx\d\([^)]*\)|recolor\[[0-9,]+\][^@]*))\s*@",
            text,
        )
        seen[re.sub(r"\s+", " ", m.group(1)).strip() if m else "UNPARSED"] += 1
    return seen


def pure_recolors(limit):
    """Distinct standalone recolor[perm] claims (not compositions)."""
    perms = []
    seen = set()
    for f in sorted(glob.glob(SOLVES)):
        raw = open(f, "rb").read()
        text = "".join(chr(b) if 32 <= b < 127 else " " for b in raw)
        for m in re.finditer(r"recolor\[([0-9,]+)\]", text):
            rest = text[m.end() : m.end() + 30]
            if re.search(r"(flip|rot|transpose|tile|per_layer)", rest):
                continue
            perm = tuple(int(x) for x in m.group(1).split(","))
            if len(perm) == 10 and sorted(perm) == list(range(10)) and perm not in seen:
                seen.add(perm)
                perms.append(perm)
                if len(perms) >= limit:
                    return perms
    return perms


def main():
    import sys

    recolor_limit = int(sys.argv[1]) if len(sys.argv) > 1 else 20
    os.makedirs(FEED, exist_ok=True)
    seen = transforms()
    wrote, skipped = 0, Counter()
    # Pure recolor claims: a 2x5 grid carrying every hue once, mapped.
    hues = [[0, 1, 2, 3, 4], [5, 6, 7, 8, 9]]
    for perm in pure_recolors(recolor_limit):
        out = [[perm[h] for h in row] for row in hues]
        claim = {"train": [{"input": hues, "output": out}]}
        name = "net_recolor_" + "".join(str(d) for d in perm)
        with open(os.path.join(FEED, f"{name}.json"), "w") as f:
            json.dump(claim, f)
        wrote += 1
    for name, count in seen.items():
        inner = name
        m = re.fullmatch(r"per_layer\(([a-z_0-9]+)\)", name)
        if m:
            inner = m.group(1)  # single layer: per_layer(f) == f
        if inner in D4:
            claim = {"train": [{"input": BASE, "output": D4[inner](BASE)}]}
            safe = re.sub(r"[^a-z0-9_]+", "_", name)
            with open(os.path.join(FEED, f"net_{safe}.json"), "w") as f:
                json.dump(claim, f)
            wrote += 1
        else:
            skipped[name.split("(")[0].split("[")[0]] += count
    print(f"mesh-feed: {wrote} claims written to {FEED}")
    for family, count in skipped.most_common():
        print(f"mesh-feed: skipped {count:5d} {family} (later pass)")


# ---- demand loop: search the mesh's walls with netstratum's library ----

def library():
    """Every transform netstratum's forge knows, as grid functions."""
    fns = dict(D4)
    for perm in pure_recolors(400):
        name = "recolor" + "".join(str(d) for d in perm)
        fns[name] = (lambda p: lambda g: [[p[h] for h in row] for row in g])(perm)
        for dname, d in D4.items():
            fns[f"{name}_{dname}"] = (
                lambda p, f: lambda g: f([[p[h] for h in row] for row in g])
            )(perm, d)
    return fns


def demand(walls_dir):
    """Read the mesh's omega walls in walls_dir, search the library,
    export any claim a known transform closes (all train pairs)."""
    omega = os.path.join(walls_dir, "omega.tsv")
    names = []
    if os.path.exists(omega):
        with open(omega) as f:
            names = [l.split("\t")[0] for l in f.readlines()[1:] if l.strip()]
    # The forge's own recolor search: infer a sitewise hue map from
    # pair 0, keep it only if consistent across every pair.
    def infer_recolor(pairs):
        m = {}
        for i, o in pairs:
            if len(i) != len(o) or any(len(r) != len(q) for r, q in zip(i, o)):
                return None
            for ri, ro in zip(i, o):
                for a, b in zip(ri, ro):
                    if m.setdefault(a, b) != b:
                        return None
        return m if any(a != b for a, b in m.items()) else None

    fns = library()
    os.makedirs(FEED, exist_ok=True)
    hits = 0
    for name in names:
        path = os.path.join(walls_dir, name + ".json")
        if not os.path.exists(path):
            continue
        import json as _json
        task = _json.load(open(path))
        pairs = [(p["input"], p["output"]) for p in task.get("train", [])]
        if not pairs:
            continue
        if infer_recolor(pairs):
            import json as _json
            with open(os.path.join(FEED, f"net_demand_{name}.json"), "w") as f:
                _json.dump({"train": task["train"]}, f)
            print(f"demand: {name} closed by recolor-search → claimed")
            hits += 1
            continue
        for fname, fn in fns.items():
            try:
                if all(fn(i) == o for i, o in pairs):
                    shutil_dst = os.path.join(FEED, f"net_demand_{name}.json")
                    with open(shutil_dst, "w") as f:
                        _json.dump({"train": task["train"]}, f)
                    print(f"demand: {name} closed by {fname} → claimed")
                    hits += 1
                    break
            except (IndexError, TypeError):
                continue
    print(f"demand: {hits} of {len(names)} walls closed by the library")


if __name__ == "__main__":
    import sys
    if len(sys.argv) > 2 and sys.argv[1] == "--demand":
        demand(sys.argv[2])
    else:
        main()
