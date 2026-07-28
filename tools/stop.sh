#!/usr/bin/env bash
# SHUTDOWN (11_LIFECYCLE §8): SIGTERM both processes by PID, wait, report.
cd "$(dirname "$0")/.."
for name in conductor wall; do
  pid_file="var/${name}.pid"
  if [ -f "$pid_file" ] && kill -0 "$(cat "$pid_file")" 2>/dev/null; then
    pid="$(cat "$pid_file")"
    echo "stop: SIGTERM $name ($pid)"
    kill -TERM "$pid" 2>/dev/null || true
    for _ in 1 2 3 4 5; do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
    kill -0 "$pid" 2>/dev/null && { echo "stop: $name lingered — SIGKILL"; kill -KILL "$pid" 2>/dev/null || true; }
    rm -f "$pid_file"
  fi
done
# Sweep any stragglers not tracked by pid file.
pkill -f 'target/debug/conductor' 2>/dev/null || true
echo "stop: down."
