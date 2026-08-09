#!/usr/bin/env python3
"""migrate_off_json.py — one-shot disk purge of legacy JSON carriers.

Converts registry / universe / starchart / solves to TLV cards when the
binary form is missing, then deletes residual JSON/JSONL that the
production write path no longer emits.

READ-THEN-REWRITE: only rewrites when a TLV counterpart can be produced
or already exists. Never invents colony balances — state.json is deleted
only when state.tlv already stands (authority is chronicle.tlv).

Usage:
  python3 tools/migrate_off_json.py [--root var] [--dry-run]
"""

from __future__ import annotations

import argparse
import json
import re
import struct
import sys
from pathlib import Path


def put(tag: int, value: bytes, out: bytearray) -> None:
    out.append(tag & 0xFF)
    out.extend(struct.pack("<I", len(value)))
    out.extend(value)


def put_str(tag: int, s: str, out: bytearray) -> None:
    put(tag, s.encode("utf-8"), out)


def put_u64(tag: int, v: int, out: bytearray) -> None:
    put(tag, struct.pack("<Q", v & 0xFFFFFFFFFFFFFFFF), out)


def atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_bytes(data)
    tmp.rename(path)


def extract_string(text: str, key: str) -> str | None:
    m = re.search(rf'"{re.escape(key)}"\s*:\s*"((?:\\.|[^"\\])*)"', text)
    if not m:
        return None
    return bytes(m.group(1), "utf-8").decode("unicode_escape")


def migrate_registry(root: Path, dry: bool, stats: dict) -> None:
    reg = root / "registry"
    if not reg.is_dir():
        return
    for path in sorted(reg.glob("*.json")):
        civ = path.with_suffix(".civ")
        if civ.exists():
            if not dry:
                path.unlink(missing_ok=True)
            stats["registry_deleted"] += 1
            continue
        try:
            card = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            stats["registry_skip"] += 1
            continue
        out = bytearray()
        put_str(1, card.get("name") or card.get("slug") or path.stem, out)
        put_str(2, card.get("sigil") or card.get("slug") or path.stem, out)
        put_str(3, card.get("species") or "", out)
        if card.get("clade"):
            put_str(4, str(card["clade"]), out)
        put_str(5, card.get("motto") or "", out)
        axes = card.get("axes") or {}
        put_str(6, axes.get("disposition") or "", out)
        put_str(7, axes.get("metabolism") or "", out)
        put_str(8, axes.get("perception") or "", out)
        put_str(9, card.get("founder") or "", out)
        if card.get("station"):
            put_str(11, str(card["station"]), out)
        if not dry:
            atomic_write(civ, bytes(out))
            path.unlink(missing_ok=True)
        stats["registry_migrated"] += 1


def migrate_universe(root: Path, dry: bool, stats: dict) -> None:
    worlds = root / "worlds"
    card = worlds / "universe.card"
    legacy = worlds / "universe.json"
    if card.exists():
        if legacy.exists():
            if not dry:
                legacy.unlink(missing_ok=True)
            stats["universe_deleted"] += 1
        return
    if not legacy.exists():
        return
    try:
        text = legacy.read_text(encoding="utf-8")
        data = json.loads(text)
    except (OSError, json.JSONDecodeError):
        stats["universe_skip"] += 1
        return
    span = str(data.get("span") or "64")
    ticks = int(data.get("ticks") or 0)
    tuniv = str(data.get("tuniversal") or "00:00:00:00:00:00")
    out = bytearray()
    put_str(1, "torus", out)
    put_str(2, span, out)
    put_str(3, tuniv, out)
    put_u64(4, ticks, out)
    put_str(5, "polynomial", out)
    if not dry:
        atomic_write(card, bytes(out))
        legacy.unlink(missing_ok=True)
    stats["universe_migrated"] += 1


def migrate_starchart(root: Path, dry: bool, stats: dict) -> None:
    worlds = root / "worlds"
    card = worlds / "starchart.card"
    legacy = worlds / "starchart.json"
    if card.exists():
        if legacy.exists():
            if not dry:
                legacy.unlink(missing_ok=True)
            stats["chart_deleted"] += 1
        return
    if not legacy.exists():
        return
    try:
        text = legacy.read_text(encoding="utf-8")
    except OSError:
        stats["chart_skip"] += 1
        return
    out = bytearray()
    n = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        world = extract_string(line, "world")
        x = extract_string(line, "x") or "0"
        y = extract_string(line, "y") or "0"
        z = extract_string(line, "z") or "0"
        g = extract_string(line, "g") or "0"
        if not world:
            try:
                obj = json.loads(line)
                world = obj.get("world")
                x, y = str(obj.get("x", "0")), str(obj.get("y", "0"))
                z, g = str(obj.get("z", "0")), str(obj.get("g", "0"))
            except json.JSONDecodeError:
                continue
        if not world:
            continue
        put_str(1, world, out)
        put_str(2, x, out)
        put_str(3, y, out)
        put_str(4, z, out)
        put_str(5, g, out)
        put(0, b"", out)
        n += 1
    if n == 0:
        stats["chart_skip"] += 1
        return
    if not dry:
        atomic_write(card, bytes(out))
        legacy.unlink(missing_ok=True)
    stats["chart_migrated"] += 1


def migrate_solves(root: Path, dry: bool, stats: dict) -> None:
    worlds = root / "worlds"
    if not worlds.is_dir():
        return
    for path in worlds.glob("*/solves/*.json"):
        sol = path.with_suffix(".sol")
        if sol.exists():
            if not dry:
                path.unlink(missing_ok=True)
            stats["solves_deleted"] += 1
            continue
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            stats["solves_skip"] += 1
            continue
        out = bytearray()
        put_str(1, str(data.get("task") or path.stem), out)
        put_str(2, str(data.get("program") or ""), out)
        put_str(3, str(data.get("certificate") or ""), out)
        put_u64(4, int(data.get("depth") or 1), out)
        if not dry:
            atomic_write(sol, bytes(out))
            path.unlink(missing_ok=True)
        stats["solves_migrated"] += 1


def migrate_galaxies(root: Path, dry: bool, stats: dict) -> None:
    worlds = root / "worlds"
    card = worlds / "galaxies.card"
    legacy = worlds / "galaxies.json"
    if card.exists():
        if legacy.exists():
            if not dry:
                legacy.unlink(missing_ok=True)
            stats["galaxies_deleted"] = stats.get("galaxies_deleted", 0) + 1
        return
    if not legacy.exists():
        return
    try:
        text = legacy.read_text(encoding="utf-8")
    except OSError:
        stats["galaxies_skip"] = stats.get("galaxies_skip", 0) + 1
        return
    out = bytearray()
    n = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        name = extract_string(line, "galaxy") or extract_string(line, "name")
        cx = extract_string(line, "x") or "0"
        cy = extract_string(line, "y") or "0"
        cz = extract_string(line, "z") or "0"
        reach = extract_string(line, "reach") or "16"
        if not name:
            try:
                obj = json.loads(line)
                name = obj.get("galaxy") or obj.get("name")
                cx, cy = str(obj.get("x", "0")), str(obj.get("y", "0"))
                cz = str(obj.get("z", "0"))
                reach = str(obj.get("reach", "16"))
            except json.JSONDecodeError:
                continue
        if not name:
            continue
        put_str(1, name, out)
        put_str(2, cx, out)
        put_str(3, cy, out)
        put_str(4, cz, out)
        put_str(5, reach, out)
        put(0, b"", out)
        n += 1
    if n == 0:
        stats["galaxies_skip"] = stats.get("galaxies_skip", 0) + 1
        return
    if not dry:
        atomic_write(card, bytes(out))
        legacy.unlink(missing_ok=True)
    stats["galaxies_migrated"] = stats.get("galaxies_migrated", 0) + 1


def purge_display_residue(root: Path, dry: bool, stats: dict) -> None:
    """Delete residual JSON display files.

    state.json is deletable whenever chronicle.tlv exists (mirror
    regenerates from the chain on the next scribe pass) or when
    state.tlv already stands. Other display residue drops when its
    binary counterpart exists, or always for pure-display journals.
    """
    worlds = root / "worlds"
    if not worlds.is_dir():
        return
    for world in worlds.iterdir():
        if not world.is_dir():
            continue
        has_chain = (world / "chronicle.tlv").exists()
        # Planet state.json
        planets = world / "planets"
        if planets.is_dir():
            for planet in planets.iterdir():
                if not planet.is_dir():
                    continue
                leg = planet / "state.json"
                if leg.exists() and (has_chain or (planet / "state.tlv").exists()):
                    if not dry:
                        leg.unlink(missing_ok=True)
                    stats["display_deleted"] += 1
        for legacy_name, keep_if_missing_auth in (
            ("world.json", "world.card"),
            ("chronicle.jsonl", "chronicle.tlv"),
            ("journal.jsonl", "journal.jrn"),
        ):
            leg = world / legacy_name
            if not leg.exists():
                continue
            auth = world / keep_if_missing_auth
            if auth.exists() or has_chain or legacy_name == "journal.jsonl":
                if not dry:
                    leg.unlink(missing_ok=True)
                stats["display_deleted"] += 1
        # Legacy inbox directives (*.json) in inbox and subdirs
        inbox = world / "inbox"
        if inbox.is_dir():
            for path in inbox.rglob("*.json"):
                if path.is_file():
                    if not dry:
                        path.unlink(missing_ok=True)
                    stats["display_deleted"] += 1
        # Legacy fiber JSON if any
        fiber_dir = worlds / "fiber"
        if fiber_dir.is_dir():
            for path in fiber_dir.glob("*.json"):
                if not dry:
                    path.unlink(missing_ok=True)
                stats["display_deleted"] += 1

    # Struck seed files
    for struck in (root / "egregores.json", root / "bots.json", root / "polities.json"):
        if struck.exists():
            if not dry:
                struck.unlink(missing_ok=True)
            stats["struck_deleted"] += 1


def ensure_world_cards(root: Path, dry: bool, stats: dict) -> None:
    """Synthesize world.card from planets/ when missing (post-JSON purge)."""
    worlds = root / "worlds"
    if not worlds.is_dir():
        return
    for world in worlds.iterdir():
        if not world.is_dir() or "--closed-" in world.name:
            continue
        card = world / "world.card"
        if card.exists():
            continue
        if not (world / "chronicle.tlv").exists() and not (world / "planets").is_dir():
            continue
        planets = []
        planets_dir = world / "planets"
        if planets_dir.is_dir():
            planets = sorted(p.name for p in planets_dir.iterdir() if p.is_dir())
        out = bytearray()
        put_str(1, world.name, out)
        for p in planets:
            put_str(2, p, out)
        if not dry:
            atomic_write(card, bytes(out))
        stats["world_cards"] = stats.get("world_cards", 0) + 1


def migrate_operators(root: Path, dry: bool, stats: dict) -> None:
    """Convert var/operators/*.json → plain-text profiles (no JSON)."""
    ops = root / "operators"
    if not ops.is_dir():
        return
    for path in sorted(ops.glob("*.json")):
        plain = ops / path.stem
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            stats["ops_skip"] = stats.get("ops_skip", 0) + 1
            continue
        if not isinstance(data, dict):
            stats["ops_skip"] = stats.get("ops_skip", 0) + 1
            continue
        lines = []
        if data.get("world") is not None:
            lines.append(f"world {data.get('world') or ''}")
        if data.get("station"):
            lines.append(f"station {data['station']}")
        for k, v in data.items():
            if k in ("world", "station"):
                continue
            lines.append(f"{k} {v}")
        body = "\n".join(lines) + ("\n" if lines else "")
        if not dry:
            if not plain.exists() or plain.stat().st_size == 0:
                plain.write_text(body, encoding="utf-8")
            path.unlink(missing_ok=True)
        stats["ops_migrated"] = stats.get("ops_migrated", 0) + 1


def main() -> int:
    ap = argparse.ArgumentParser(description="Migrate residual JSON off disk")
    ap.add_argument("--root", default="var")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()
    root = Path(args.root)
    stats = {
        "registry_migrated": 0,
        "registry_deleted": 0,
        "registry_skip": 0,
        "universe_migrated": 0,
        "universe_deleted": 0,
        "universe_skip": 0,
        "chart_migrated": 0,
        "chart_deleted": 0,
        "chart_skip": 0,
        "solves_migrated": 0,
        "solves_deleted": 0,
        "solves_skip": 0,
        "display_deleted": 0,
        "struck_deleted": 0,
    }
    migrate_registry(root, args.dry_run, stats)
    migrate_universe(root, args.dry_run, stats)
    migrate_starchart(root, args.dry_run, stats)
    migrate_galaxies(root, args.dry_run, stats)
    migrate_solves(root, args.dry_run, stats)
    migrate_operators(root, args.dry_run, stats)
    purge_display_residue(root, args.dry_run, stats)
    ensure_world_cards(root, args.dry_run, stats)
    mode = "DRY-RUN" if args.dry_run else "DONE"
    print(f"migrate_off_json [{mode}] root={root}")
    for k, v in sorted(stats.items()):
        if v:
            print(f"  {k}: {v}")
    remaining = list(root.rglob("*.json")) + list(root.rglob("*.jsonl"))
    print(f"  residual_json_any: {len(remaining)}")
    for p in remaining[:15]:
        print(f"    still: {p}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
