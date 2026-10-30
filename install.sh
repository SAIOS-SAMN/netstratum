#!/usr/bin/env bash
#
# install.sh — stand up a clean Netstratum universe.
#
# A universe is one directory (its "var root"):
#
#   registry/        one card per civilization registered — the compiled
#                    seats at genesis, every cleave, every registration
#                    (`<sigil>.civ`), and where a cleave was founded
#                    (`<sigil>.seat`)
#   worlds/          one directory per world — chronicle, receipts, lens exports
#   conductor.ctl    the operator control line (paused / cadence N)
#
# The founding roster is compiled (engine/conductor/src/polities.rs, 22
# seats); no roster file is read. New civilizations arise by divergence.
#
# Nothing here leaves a process running. `install` prepares the directory
# and founds the worlds with a single bounded beat, then exits. `run` is
# the only command that hands off to the conductor's heartbeat.
#
# Usage:
#   ./install.sh                       prepare ./var  (refuses if it exists)
#   ./install.sh --var demo            prepare ./demo
#   ./install.sh fresh --var demo      wipe ./demo, then prepare it again
#   ./install.sh wipe  --var demo      remove ./demo
#   ./install.sh status --var demo     what stands at ./demo
#   ./install.sh run   --var demo      start the conductor against ./demo
#
# Options:
#   --var root       universe root, repo-relative unless absolute (default: var)
#   --no-build       skip cargo build; use whatever already sits in target/
#   --no-genesis     prepare files only, do not found the worlds
#   --cadence N      seconds between beats, for `run` (default: 2)
#   --force          act on a root that already holds a live universe
#   --yes            never prompt (for non-interactive use)
#
# Var roots are git-ignored and single-copy. `wipe` deletes real history.

set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

command_name="install"
var_root="var"
do_build=1
do_genesis=1
cadence=2
force=0
assume_yes=0

say()  { printf '%s\n' "$*"; }
step() { printf '\n== %s\n' "$*"; }
warn() { printf 'install.sh: %s\n' "$*" >&2; }
die()  { printf 'install.sh: %s\n' "$*" >&2; exit 1; }

usage() {
  cat <<'USAGE'
netstratum install.sh — stand up a clean universe

  ./install.sh [install]        prepare a universe (default command)
  ./install.sh fresh            wipe the root, then prepare it again
  ./install.sh wipe             remove a universe root
  ./install.sh status           report what stands at a root
  ./install.sh run              start the conductor against a root

Options:
  --var ROOT      universe root, repo-relative unless absolute (default: var)
  --no-build      skip cargo build
  --no-genesis    prepare files only; do not found the worlds
  --cadence N     seconds between beats, for run (default: 2)
  --force         act on a root that already holds a live universe
  --yes           never prompt
USAGE
  exit "${1:-0}"
}

parse_args() {
  if [ $# -gt 0 ]; then
    case "$1" in
      install|fresh|wipe|status|run)
        command_name="$1"; shift ;;
      -h|--help)
        usage 0 ;;
    esac
  fi
  while [ $# -gt 0 ]; do
    case "$1" in
      --var)     [ $# -ge 2 ] || die "--var needs a directory"; var_root="$2"; shift 2 ;;
      --roster)  die "--roster is gone: the founding roster is compiled (engine/conductor/src/polities.rs)" ;;
      --cadence) [ $# -ge 2 ] || die "--cadence needs a number"; cadence="$2"; shift 2 ;;
      --no-build)   do_build=0; shift ;;
      --no-genesis) do_genesis=0; shift ;;
      --force)      force=1; shift ;;
      --yes|-y)     assume_yes=1; shift ;;
      -h|--help)    usage 0 ;;
      *) die "unknown argument '$1' (try --help)" ;;
    esac
  done
}

# Absolute path for the root; relative roots resolve against the repo,
# so the script behaves the same from any working directory.
resolve_root() {
  case "$var_root" in
    /*) printf '%s' "$var_root" ;;
    *)  printf '%s/%s' "$REPO" "$var_root" ;;
  esac
}

# A directory counts as a universe when it carries any of the three
# things only the engine writes. `wipe` will not touch anything else.
# The roster's two earlier names are still recognised: an existing
# universe must not stop looking like one because the name changed.
looks_like_universe() {
  local root="$1"
  [ -e "$root/polities.json" ] || [ -e "$root/egregores.json" ] \
    || [ -e "$root/bots.json" ] || [ -d "$root/worlds" ] || [ -d "$root/registry" ]
}

# An empty directory carries no history, so it is safe to remove without
# the universe checks below.
dir_is_empty() {
  local root="$1" entry
  for entry in "$root"/* "$root"/.[!.]*; do
    [ -e "$entry" ] && return 1
  done
  return 0
}

world_count() {
  local root="$1" n=0 entry
  if [ -d "$root/worlds" ]; then
    for entry in "$root/worlds"/*/; do
      [ -d "$entry" ] && n=$((n + 1))
    done
  fi
  printf '%s' "$n"
}

registry_count() {
  local root="$1" n=0 entry
  if [ -d "$root/registry" ]; then
    for entry in "$root/registry"/*.civ; do
      [ -f "$entry" ] && n=$((n + 1))
    done
  fi
  printf '%s' "$n"
}

confirm() {
  local prompt="$1"
  [ "$assume_yes" -eq 1 ] && return 0
  if [ ! -t 0 ]; then
    die "refusing to proceed without a terminal — pass --yes to confirm"
  fi
  printf '%s [type yes to continue] ' "$prompt"
  local answer=""
  read -r answer || true
  [ "$answer" = "yes" ]
}

preflight() {
  command -v cargo >/dev/null 2>&1 \
    || die "cargo not found — install Rust from https://rustup.rs and re-run"
  [ -f "$REPO/Cargo.toml" ] \
    || die "$REPO does not look like the netstratum workspace (no Cargo.toml)"
  command -v python3 >/dev/null 2>&1 \
    || warn "python3 not found — the engine runs fine, but tools/observer.py will not"
}

# Building is a side effect with its own output; keep it out of any
# command substitution or its progress text lands in the caller's
# variable instead of on the terminal.
ensure_build() {
  [ "$do_build" -eq 1 ] || return 0
  step "Building the workspace binaries, release (the first run compiles from scratch)"
  ( cd "$REPO" && cargo build --release -p conductor -p netstratum-cli -p ns-hypha )
}

# The conductor binary. Prints the path and nothing else.
conductor_bin() {
  local built="$REPO/target/release/conductor"
  [ -x "$built" ] \
    || die "no conductor binary at target/release/conductor — re-run without --no-build"
  printf '%s' "$built"
}

cmd_install() {
  local root; root="$(resolve_root)"
  preflight

  if [ -e "$root" ]; then
    if looks_like_universe "$root" && [ "$force" -eq 0 ]; then
      die "$root already holds a universe ($(world_count "$root") worlds).
       Use a different --var, or 'fresh --var ${var_root}' to wipe and rebuild."
    fi
    if [ "$force" -eq 0 ]; then
      die "$root already exists — pass --force to install into it anyway"
    fi
  fi

  ensure_build
  local bin; bin="$(conductor_bin)"

  step "Preparing $root"
  mkdir -p "$root"
  say "roster: the 22 compiled founding seats; each band chooses where it settles"

  if [ "$do_genesis" -eq 1 ]; then
    step "Founding the worlds (one bounded beat, then exit)"
    "$bin" --var "$root" --cycles 1 --cadence 0
  else
    say "genesis skipped — the worlds are founded on the conductor's first boot"
  fi

  step "Ready"
  say "  ./install.sh status --var $var_root      what stands here"
  say "  ./install.sh run    --var $var_root      start the heartbeat"
  say "  python3 tools/observer.py --root $var_root --watch 2"
  say "  python3 tools/webserve.py --root $var_root --port 8081"
  say "  target/release/netstratum                the hands-on client"
  say "  target/release/ns-hypha follow $var_root  fold this universe onto the :7520 court"
  say ""
  say "  ./install.sh fresh  --var $var_root      wipe and start over"
}

cmd_wipe() {
  local root; root="$(resolve_root)"

  [ -e "$root" ] || { say "nothing at $root — already clean"; return 0; }
  if dir_is_empty "$root"; then
    rmdir "$root"
    say "removed the empty directory $root"
    return 0
  fi
  looks_like_universe "$root" \
    || die "$root does not look like a netstratum universe (no polities.json,
       worlds/ or registry/) — refusing to delete it"

  local worlds; worlds="$(world_count "$root")"
  if [ "$root" = "$REPO/var" ] && [ "$force" -eq 0 ]; then
    die "$root is the default live universe ($worlds worlds).
       Var roots are single-copy and git-ignored: this history exists nowhere else.
       Pass --force if you truly mean to delete it."
  fi

  confirm "Delete $root — $worlds worlds, $(registry_count "$root") civilizations, permanently?" \
    || die "wipe cancelled — nothing was deleted"

  rm -rf "$root"
  say "wiped $root"
}

cmd_fresh() {
  cmd_wipe
  cmd_install
}

cmd_status() {
  local root; root="$(resolve_root)"
  say "root:         $root"
  if [ ! -e "$root" ]; then
    say "state:        absent — ./install.sh --var $var_root to create it"
    return 0
  fi
  if ! looks_like_universe "$root"; then
    if dir_is_empty "$root"; then
      say "state:        prepared and empty — the conductor seeds and founds it on first run"
    else
      say "state:        present, but holds no universe"
    fi
    return 0
  fi

  local worlds; worlds="$(world_count "$root")"
  say "state:        $( [ "$worlds" -gt 0 ] && echo founded || echo "prepared, not founded" )"
  say "roster:       22 compiled founding seats"
  say "registry:     $(registry_count "$root") civilizations registered (founding seats, cleaves, registrations)"
  say "worlds:       $worlds"
  [ -f "$root/conductor.ctl" ] && say "control:      $(cat "$root/conductor.ctl")"
  [ -d "$root/spine" ] && say "spine:        present"
  return 0
}

cmd_run() {
  local root; root="$(resolve_root)"
  # The conductor seeds and founds an empty root itself, so `run` only
  # needs the directory to exist — install.sh made it deliberately.
  [ -d "$root" ] \
    || die "no universe at $root — run ./install.sh --var $var_root first"
  ensure_build
  local bin; bin="$(conductor_bin)"
  step "Conductor beating every ${cadence}s against $root (Ctrl-C to stop)"
  exec "$bin" --var "$root" --cadence "$cadence"
}

main() {
  parse_args "$@"
  case "$command_name" in
    install)       cmd_install ;;
    fresh)         cmd_fresh ;;
    wipe)          cmd_wipe ;;
    status)        cmd_status ;;
    run)           cmd_run ;;
    *)             die "unknown command '$command_name'" ;;
  esac
}

main "$@"
