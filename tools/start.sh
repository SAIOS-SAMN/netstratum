#!/usr/bin/env bash
# BOOT (11_LIFECYCLE §8): conductor + Terminal Wall server, PID-tracked.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p var
CADENCE="${1:-2}"

if pgrep -f 'target/debug/conductor' >/dev/null 2>&1; then
  echo "start: conductor already running (tools/stop.sh to stop)"; exit 1
fi

echo "start: building…"; cargo build -p conductor -q
echo "start: conductor (cadence ${CADENCE}s) → var/conductor.log"
nohup target/debug/conductor --cadence "$CADENCE" > var/conductor.log 2>&1 &
echo $! > var/conductor.pid

# Wall server on 8080 (localhost only), once the wall exists.
if ! pgrep -f 'http.server 8080' >/dev/null 2>&1; then
  echo "start: Terminal Wall → http://127.0.0.1:8080/wall.html"
  ( cd var/worlds 2>/dev/null && nohup python3 -m http.server 8080 --bind 127.0.0.1 > ../wall.log 2>&1 & echo $! > ../wall.pid ) || true
fi
echo "start: up. observe: python3 tools/observer.py --watch 2"
