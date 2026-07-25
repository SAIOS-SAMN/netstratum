#!/usr/bin/env python3
"""steward.py — the master control program, on a leash.

THE TOPOLOGY: directives target a WORLD (the container with the one
chronicle); the game loop executes them against the colonies inside.

Colloquially "the MCP"; named steward because it governs, it does not
own. THE IRONCLAD LAW (substrate contract §7): the steward NEVER edits
world state. It only drops directive files into a civilization's
inbox/. A directive becomes real ONLY when the game loop picks it up,
checks the steward entity's NS-6 grants, and executes it as an
ordinary, hash-chained, attributed receipt. `revoke steward <scope>`
in-game shuts this tool out of a world instantly.

Standard library only.

Usage:
  python3 tools/steward.py list                          # worlds under the root
  python3 tools/steward.py send <world> <verb> [args...] # queue a directive
  python3 tools/steward.py status <world>                # inbox: pending/applied/refused
  python3 tools/steward.py --root var/worlds ...

Examples:
  python3 tools/steward.py send sol zone 12 ext
  python3 tools/steward.py send sol collect
  python3 tools/steward.py send kepler-passage raze 3:4
"""

import argparse
import json
import sys
from pathlib import Path


def world_dirs(root):
    root = Path(root)
    if not root.is_dir():
        return []
    return sorted(d for d in root.iterdir() if d.is_dir() and (d / "world.json").exists())


def next_number(inbox):
    """Directives are numbered; order in the inbox IS the steward's
    stated order (the mirror reads name-ascending)."""
    highest = 0
    for pool in [inbox, inbox / "applied", inbox / "refused"]:
        if pool.is_dir():
            for path in pool.iterdir():
                stem = path.name.split("-", 1)[0]
                if stem.isdigit():
                    highest = max(highest, int(stem))
    return highest + 1


def cmd_list(root):
    worlds = world_dirs(root)
    if not worlds:
        print(f"(no worlds under {root})")
        return
    for world in worlds:
        card = json.loads((world / "world.json").read_text(encoding="utf-8"))
        planets = card.get("planets", [])
        pending = sum(1 for p in (world / "inbox").glob("*.json")) if (world / "inbox").is_dir() else 0
        print(f"  {card.get('world', world.name):<20} planets {', '.join(planets) or '(none)':<24} "
              f"inbox pending {pending}")


def cmd_send(root, world, verb, args):
    world_dir = Path(root) / world
    if not (world_dir / "world.json").exists():
        sys.exit(f"steward: no mirrored world '{world}' under {root} "
                 f"(known: {', '.join(d.name for d in world_dirs(root)) or 'none'})")
    inbox = world_dir / "inbox"
    inbox.mkdir(exist_ok=True)
    number = next_number(inbox)
    name = f"{number:06d}-{verb}.json"
    directive = {"verb": verb, "args": list(args), "as": "steward"}
    (inbox / name).write_text(json.dumps(directive) + "\n", encoding="utf-8")
    print(f"queued {name} → {world}/inbox/  (applies when the game loop "
          f"ticks, IF the steward entity holds the needed scope)")


def cmd_status(root, world):
    inbox = Path(root) / world / "inbox"
    if not inbox.is_dir():
        sys.exit(f"steward: no inbox for '{world}' under {root}")
    pending = sorted(p.name for p in inbox.glob("*.json"))
    applied = sorted(p.name for p in (inbox / "applied").glob("*.json")) if (inbox / "applied").is_dir() else []
    refused = sorted(p.name for p in (inbox / "refused").glob("*.json")) if (inbox / "refused").is_dir() else []
    print(f"{world}: {len(pending)} pending · {len(applied)} applied · {len(refused)} refused")
    for name in pending:
        print(f"  pending  {name}")
    for name in refused[-5:]:
        print(f"  refused  {name}")


def main():
    parser = argparse.ArgumentParser(description="netstratum steward (directives only, never state)")
    parser.add_argument("--root", default="var/worlds", help="the worlds root (default var/worlds)")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("list")
    send = sub.add_parser("send")
    send.add_argument("world")
    send.add_argument("verb")
    send.add_argument("args", nargs="*")
    status = sub.add_parser("status")
    status.add_argument("world")
    ns = parser.parse_args()

    if ns.command == "list":
        cmd_list(ns.root)
    elif ns.command == "send":
        cmd_send(ns.root, ns.world, ns.verb, ns.args)
    elif ns.command == "status":
        cmd_status(ns.root, ns.world)


if __name__ == "__main__":
    main()
