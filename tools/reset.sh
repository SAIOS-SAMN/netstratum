#!/usr/bin/env bash
# CLEAN START (11_LIFECYCLE §8/§9): wipe the accreted universe.
# IRREVERSIBLE — the var* trees are single-copy and git-ignored.
set -euo pipefail
cd "$(dirname "$0")/.."

# Refuse to wipe while the board is beating.
if pgrep -f 'target/debug/conductor' >/dev/null 2>&1; then
  echo "reset: the conductor is running — stop it first (tools/stop.sh)"; exit 1
fi

echo "reset: removing accreted universe state —"
for d in var var-era1 var-era2 var-retired; do
  if [ -d "$d" ]; then
    echo "  $d ($(du -sh "$d" 2>/dev/null | cut -f1), $(ls "$d/worlds" 2>/dev/null | wc -l) worlds)"
    rm -rf "$d"
  fi
done

# Stale Apport crash reports (§9).
for c in /var/crash/_opt_netstratum_*.crash /var/crash/_opt_ipsal-autic-v0.5_*.crash; do
  [ -e "$c" ] && { echo "  $c"; sudo rm -f "$c" 2>/dev/null || rm -f "$c" 2>/dev/null || echo "    (needs sudo — skipped)"; }
done

echo "reset: the universe is clean. tools/start.sh founds it fresh."
