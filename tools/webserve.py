#!/usr/bin/env python3
"""webserve.py — the observatory's window: a read-only file server.

THERE IS NO API. The mirror's own files ARE the protocol — the web
observatory reads the same renderings the terminal observer reads
(`state.json`, `starchart.json`, `chronicle.jsonl`, `universe.json`),
served verbatim. One truth, zero translation layers (the reflection
law over HTTP). The single synthesized route, `/data/index.json`, is a
directory INDEX rendered as data — a listing, not an endpoint.

READ-ONLY BY CONSTRUCTION: GET only; anything else refuses 405. The
future push stream (NS-5) will speak NS-1 TLV frames over a socket —
the house wire, never a third format.

Usage:
  python3 tools/webserve.py [--root var] [--port 8081]
  → http://localhost:8081/
"""

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
ALLOWED_SUFFIXES = {".json", ".jsonl", ".html"}


def build_index(root):
    """The directory index as data: worlds, planets, registry sigils."""
    worlds = []
    worlds_dir = root / "worlds"
    if worlds_dir.is_dir():
        for world_dir in sorted(p for p in worlds_dir.iterdir() if p.is_dir()):
            planets = []
            planets_dir = world_dir / "planets"
            if planets_dir.is_dir():
                planets = sorted(
                    p.name for p in planets_dir.iterdir() if p.is_dir())
            worlds.append({"world": world_dir.name, "planets": planets})
    registry = []
    reg_dir = root / "registry"
    if reg_dir.is_dir():
        registry = sorted(p.stem for p in reg_dir.glob("*.json"))
    return json.dumps({"worlds": worlds, "registry": registry})


class Window(BaseHTTPRequestHandler):
    root = Path("var")

    def log_message(self, *args):
        pass

    def do_GET(self):
        path = self.path.split("?", 1)[0]
        if path in ("/", "/index.html"):
            return self.send_file(TOOLS / "observatory.html", "text/html")
        if path == "/data/index.json":
            body = build_index(self.root).encode()
            return self.send_bytes(body, "application/json")
        if path.startswith("/data/"):
            relative = path[len("/data/"):]
            target = (self.root / relative).resolve()
            # Stay inside the root; only mirror renderings pass.
            if (self.root.resolve() in target.parents
                    and target.suffix in ALLOWED_SUFFIXES
                    and target.is_file()):
                kind = ("application/json" if target.suffix == ".json"
                        else "text/plain" if target.suffix == ".jsonl"
                        else "text/html")
                return self.send_file(target, kind)
        self.send_response(404)
        self.end_headers()

    def do_POST(self):
        # Observation and intervention are different verbs — the
        # window only ever looks. Petitions go through the steward.
        self.send_response(405)
        self.end_headers()

    do_PUT = do_DELETE = do_PATCH = do_POST

    def send_file(self, path, kind):
        try:
            body = path.read_bytes()
        except OSError:
            self.send_response(404)
            self.end_headers()
            return
        self.send_bytes(body, kind)

    def send_bytes(self, body, kind):
        self.send_response(200)
        self.send_header("Content-Type", f"{kind}; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)


def main():
    parser = argparse.ArgumentParser(description="the observatory's window (read-only)")
    parser.add_argument("--root", default="var")
    parser.add_argument("--port", type=int, default=8081)
    args = parser.parse_args()
    Window.root = Path(args.root)
    server = ThreadingHTTPServer(("127.0.0.1", args.port), Window)
    print(f"observatory window on http://localhost:{args.port}/ — root {args.root}/ (read-only)")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
