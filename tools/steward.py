#!/usr/bin/env python3
"""steward.py — the steward program, scope-bound.

Directives are binary TLV (*.dir), never JSON. THE IRONCLAD LAW
(substrate contract §7 / docs/JSON_DISPLAY_ONLY.md): the steward NEVER
edits world state. It only drops directive files into a world's inbox/.

World presence is detected via world.card or chronicle.tlv (not world.json).

Usage:
  python3 tools/steward.py list
  python3 tools/steward.py send <world> <verb> [args...]
  python3 tools/steward.py status <world>
"""

import argparse
import struct
import sys
from pathlib import Path

# TLV: tag u8 ‖ LE32(len) ‖ value  (ledger::card::dir)
DIR_VERB, DIR_ARG, DIR_AS = 1, 2, 3


def put(tag: int, value: bytes, out: bytearray):
    out.append(tag & 0xFF)
    out.extend(struct.pack("<I", len(value)))
    out.extend(value)


def put_str(tag: int, s: str, out: bytearray):
    put(tag, s.encode("utf-8"), out)


def write_directive(path: Path, verb: str, args, acting_as: str = "steward"):
    out = bytearray()
    put_str(DIR_VERB, verb, out)
    for a in args:
        put_str(DIR_ARG, str(a), out)
    put_str(DIR_AS, acting_as, out)
    path.write_bytes(bytes(out))


def world_dirs(root):
    root = Path(root)
    if not root.is_dir():
        return []
    return sorted(
        d
        for d in root.iterdir()
        if d.is_dir()
        and (
            (d / "world.card").exists()
            or (d / "chronicle.tlv").exists()
        )
        and not d.name.startswith(".")
        and "--closed-" not in d.name
    )


def next_number(inbox):
    highest = 0
    for pool in [inbox, inbox / "applied", inbox / "refused"]:
        if pool.is_dir():
            for path in pool.iterdir():
                stem = path.name.split("-", 1)[0]
                if stem.isdigit():
                    highest = max(highest, int(stem))
    return highest + 1


def parse_world_card(path: Path):
    """Read world name + planet list from world.card TLV."""
    try:
        data = path.read_bytes()
    except OSError:
        return None, []
    name, planets = path.parent.name, []
    at = 0
    while at + 5 <= len(data):
        tag = data[at]
        ln = int.from_bytes(data[at + 1 : at + 5], "little")
        val = data[at + 5 : at + 5 + ln]
        if len(val) < ln:
            break
        if tag == 1:
            try:
                name = val.decode("utf-8")
            except Exception:
                pass
        elif tag == 2:
            try:
                planets.append(val.decode("utf-8"))
            except Exception:
                pass
        at += 5 + ln
    return name, planets


def cmd_list(root):
    worlds = world_dirs(root)
    if not worlds:
        print(f"(no worlds under {root})")
        return
    for world in worlds:
        card = world / "world.card"
        if card.exists():
            wname, planets = parse_world_card(card)
        else:
            wname, planets = world.name, []
        inbox = world / "inbox"
        pending = 0
        if inbox.is_dir():
            pending = sum(
                1
                for p in inbox.iterdir()
                if p.is_file() and p.suffix == ".dir"
            )
        print(
            f"  {wname:<20} planets {', '.join(planets) or '(none)':<24} "
            f"inbox pending {pending}"
        )


def cmd_send(root, world, verb, args):
    world_dir = Path(root) / world
    if not (
        (world_dir / "world.card").exists()
        or (world_dir / "chronicle.tlv").exists()
    ):
        sys.exit(
            f"steward: no exported world '{world}' under {root} "
            f"(known: {', '.join(d.name for d in world_dirs(root)) or 'none'})"
        )
    inbox = world_dir / "inbox"
    inbox.mkdir(exist_ok=True)
    number = next_number(inbox)
    name = f"{number:06d}-{verb}.dir"
    write_directive(inbox / name, verb, args)
    print(
        f"queued {name} → {world}/inbox/  (applies when the game loop "
        f"ticks, IF the steward entity holds the needed scope)"
    )


def cmd_status(root, world):
    inbox = Path(root) / world / "inbox"
    if not inbox.is_dir():
        sys.exit(f"steward: no inbox for '{world}' under {root}")

    def names(exts, pool):
        if not pool.is_dir():
            return []
        return sorted(
            p.name
            for p in pool.iterdir()
            if p.is_file() and p.suffix in exts
        )

    pending = names((".dir",), inbox)
    applied = names((".dir",), inbox / "applied")
    refused = names((".dir",), inbox / "refused")
    print(
        f"{world}: {len(pending)} pending · {len(applied)} applied · {len(refused)} refused"
    )
    for name in pending:
        print(f"  pending  {name}")
    for name in refused[-5:]:
        print(f"  refused  {name}")


def main():
    parser = argparse.ArgumentParser(
        description="netstratum steward (directives only, never state; TLV .dir)"
    )
    parser.add_argument(
        "--root", default="var/worlds", help="the worlds root (default var/worlds)"
    )
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("list", help="list worlds")
    p_send = sub.add_parser("send", help="queue a directive")
    p_send.add_argument("world")
    p_send.add_argument("verb")
    p_send.add_argument("args", nargs="*")
    p_status = sub.add_parser("status", help="inbox status")
    p_status.add_argument("world")
    args = parser.parse_args()
    if args.command == "list":
        cmd_list(args.root)
    elif args.command == "send":
        cmd_send(args.root, args.world, args.verb, args.args)
    elif args.command == "status":
        cmd_status(args.root, args.world)


if __name__ == "__main__":
    main()
