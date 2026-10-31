#!/usr/bin/env python3
"""webserve.py — the observatory's window: a read-only file server.

THERE IS NO JSON API. Binary cards and TLV are the protocol
(docs/JSON_DISPLAY_ONLY.md). Index is a plain-text listing, not JSON.

READ-ONLY BY CONSTRUCTION: GET only; anything else refuses 405.

Usage:
  python3 tools/webserve.py [--root var] [--port 8081]
"""

import argparse
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
# Binary + HTML only — no .json / .jsonl
ALLOWED_SUFFIXES = {
    ".tlv",
    ".card",
    ".civ",
    ".dir",
    ".sol",
    ".jrn",
    ".fiber",
    ".html",
}


def build_index(root):
    """Directory index as plain text lines (not JSON)."""
    lines = ["# netstratum data index", ""]
    worlds_dir = root / "worlds"
    if worlds_dir.is_dir():
        lines.append("[worlds]")
        for world_dir in sorted(p for p in worlds_dir.iterdir() if p.is_dir()):
            if "--closed-" in world_dir.name:
                continue
            planets = []
            planets_dir = world_dir / "planets"
            if planets_dir.is_dir():
                planets = sorted(p.name for p in planets_dir.iterdir() if p.is_dir())
            lines.append(f"world {world_dir.name}")
            for p in planets:
                lines.append(f"  planet {p}")
    reg_dir = root / "registry"
    if reg_dir.is_dir():
        lines.append("")
        lines.append("[registry]")
        for p in sorted(reg_dir.glob("*.civ")):
            lines.append(f"civ {p.stem}")
    return "\n".join(lines) + "\n"


class Window(BaseHTTPRequestHandler):
    root = Path("var")

    def log_message(self, *args):
        pass

    def do_GET(self):
        path = self.path.split("?", 1)[0]
        if path in ("/", "/index.html"):
            return self.send_file(TOOLS / "observatory.html", "text/html")
        if path == "/stream":
            return self.stream()
        if path in ("/data/index", "/data/index.txt"):
            body = build_index(self.root).encode()
            return self.send_bytes(body, "text/plain; charset=utf-8")
        # Reject JSON index legacy path explicitly.
        if path == "/data/index.json":
            self.send_response(410)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write("gone — use /data/index.txt (no JSON)\n".encode("utf-8"))
            return
        if path.startswith("/data/"):
            relative = path[len("/data/") :]
            target = (self.root / relative).resolve()
            if (
                self.root.resolve() in target.parents
                and target.suffix in ALLOWED_SUFFIXES
                and target.is_file()
            ):
                kind = (
                    "text/html"
                    if target.suffix == ".html"
                    else "application/octet-stream"
                )
                return self.send_file(target, kind)
        self.send_response(404)
        self.end_headers()

    def stream(self):
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        seen = {}
        try:
            while True:
                changed = []
                worlds = self.root / "worlds"
                if worlds.is_dir():
                    for state in worlds.glob("*/planets/*/state.tlv"):
                        stamp = state.stat().st_mtime_ns
                        if seen.get(state) != stamp:
                            seen[state] = stamp
                            rel = state.relative_to(self.root)
                            changed.append(str(rel))
                    for card in worlds.glob("*/world.card"):
                        stamp = card.stat().st_mtime_ns
                        if seen.get(card) != stamp:
                            seen[card] = stamp
                            changed.append(str(card.relative_to(self.root)))
                if changed:
                    payload = " ".join(changed)
                    self.wfile.write(f"event: tick\ndata: {payload}\n\n".encode())
                    self.wfile.flush()
                time.sleep(1)
        except (BrokenPipeError, ConnectionResetError):
            pass

    def send_file(self, path, content_type):
        data = path.read_bytes()
        self.send_bytes(data, content_type)

    def send_bytes(self, body, content_type):
        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


def main():
    parser = argparse.ArgumentParser(description="netstratum observatory file server")
    parser.add_argument("--root", default="var")
    parser.add_argument("--port", type=int, default=8081)
    args = parser.parse_args()
    Window.root = Path(args.root)
    httpd = ThreadingHTTPServer(("0.0.0.0", args.port), Window)
    print(f"netstratum window on http://localhost:{args.port}/ (no JSON)")
    httpd.serve_forever()


if __name__ == "__main__":
    main()
