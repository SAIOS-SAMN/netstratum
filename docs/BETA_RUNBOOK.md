# THE BETA RUNBOOK — watching netstratum play itself

**Status: operational, verified 2026-08-13.** This documents a
capability that already exists — `engine/conductor` and
`tools/observer.py` were both landed before this doc was written; this
is the missing "how do I actually run one" page, not a design.

## 0. What this is, and what it isn't

`conductor` runs every civilization in its roster autonomously —
building zones, collecting yields, researching, tithing — beat by
beat, with **no human input**. `observer.py` watches the result live,
**strictly read-only** (it parses `chronicle.tlv`/`state.tlv`; it never
writes).

**This is NOT solver/ARC-AGI-piloted cognition.** `engine/solver/`
proves useful work by solving ARC-AGI puzzles for the PoW layer — it
does not choose what a civilization builds, tithes, or researches, and
has no read/write path into a `Polity`'s decisions. What plays a beta
civ today is `conductor`'s own scripted/procedural policy
(`Policy::Steady` and its siblings in `polities.rs`), not learned or
inferred strategy. If "AI-piloted" is read as "cognition-driven
strategy," that link does not exist yet — this runbook demonstrates
the autonomous-PLAY loop, not an autonomous-THINKING one.

## 1. Run it — zero setup required

The roster ships with a compiled default set of founding civilizations
(`polities.rs::founding_polities` — Tunborn and its siblings). No
`civ new`, no world-founding, no genesis-sealing needed for a first
run:

```sh
cargo run -p conductor -- --var var --cycles 200 --cadence 2
```

- `--cycles 200` — stop after 200 beats (omit, or pass `0`, to run
  forever at the cadence until killed).
- `--cadence 2` — 2 seconds of wall-clock time per beat (tune to taste;
  lower is faster, not "more accurate" — beats are exact-rational, not
  time-sensitive).
- `--var var` — the state root; `var/worlds/<sigil>-hearth/` holds each
  polity's `chronicle.tlv`/`state.tlv`.

## 2. Watch it, in a second terminal

```sh
python3 tools/observer.py --watch 2 --root var
```

Re-reads and re-renders every 2 seconds. It is read-only by
construction (`observer.py`'s own header states this) — it can be left
running indefinitely alongside a live conductor without any risk of
the two processes racing on writes.

For the 3-torus visual board instead of the terminal view:

```sh
python3 tools/webserve.py --root var
# then open tools/observatory.html in a browser
```

## 3. Add more civilizations to the beta

`civ new` registers a line into the compiled roster via a receipted
door (never a hand-edited file — see `polities.rs`'s definition law).
From the CLI:

```
civ new <sigil> | <Name> | <species> | <motto> | <disposition> <metabolism> <perception> [station]
```

`conductor` derives the new polity's world/planet/colony names and
starting position automatically (`registered_polities()`) — nothing
else to configure. Restart (or the next boot of) `conductor` picks up
the addition; the roster is compiled fresh from the registry each run,
never cached in a file that can drift.

## 4. What a beta run demonstrates today, and what it doesn't

**Demonstrates:** the full landed economic loop running unattended —
founding, zone build/upkeep/yield (now metabolism-aware, this
session's `zone_cost` work), population growth/starvation, the
founding tithe and seat treasuries, taxation levies, civic
stability/welfare, tech crystallization, ideology/cohesion — for
however many civilizations are in the roster, for as long as
`--cycles`/wall-clock allows.

**Does not demonstrate:** an end to any of it. Per
`docs/50_WIN_CONDITIONS.md` (written alongside this doc), there is no
win/loss condition anywhere in the engine — a beta run will play
forever (or until `--cycles` runs out) with no closure, no matter what
happens to any civilization's population, economy, or stability. Watch
for a civilization visibly collapsing (population to 0, chronic
famine) as a live argument for landing the loss condition that doc
proposes first.

**Also does not demonstrate:** kernel enforcement being LIVE. Per
`#31` (`docs/39_PHASE3_FLIP.md`, still the top item on
`docs/GAME_SPEC.md`'s open-work queue), the kernel judges every deed
but its verdict is still advisory — `replay`, not the kernel, is the
live transition. A beta run today is not yet proof that an illegal
deed COULD NOT have applied; it only shows that none happened to be
proposed by conductor's own (lawful-by-construction) policy.
