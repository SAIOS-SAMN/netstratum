# NETSTRATUM

An AI MUD — a text-first, multi-planetary city-building game — that is also the
playable reference implementation of the dimensional web: the NS protocol stack
specified in Rational Field Algebra (RFA), executed by the Canonical Field
Engine (CFE), and settled to existing chains through the WWW3 bridge (NS-7).

**Start here:** `docs/00_CHARTER.md` (the founding document, read whole).
The current landing is always read at `docs/BUILD_ORDER.md`, never inferred.
Resuming work? `docs/PICKUP.md` is the fast map of the board; `docs/HANDOFF.md` is the working practice around it.

```
protocols/   NS-1 DOP · NS-2 RQP · NS-3 VRP · NS-4 APERTURE · NS-5 MESH · NS-6 IPSAL-OS · NS-7 ANCHOR
docs/        00 charter · 01 economy · 02 colony loop · 03 substrate contract ·
             04 civilization genesis · 05 thermal balance · 06 civilizations
             (the world-book) · 07 era convergence · 08 chronicle feed ·
             09 ideology · 10 tech emergence · BUILD_ORDER (the one carrier) ·
             OPEN_QUESTIONS
engine/      the Rust workspace: cfe · kernel · codec · rfa-core · physics ·
             solver · solver-verify · ledger · mesh · conductor
clients/     the CLI/MUD client
tools/       observer.py (the board) · steward.py (the inbox)
```

Set up: `./install.sh` prepares a universe — builds the binaries, seats the NPC
roster from `seeds/`, founds the worlds with one bounded beat, and exits with
nothing left running. `./install.sh run` starts the conductor's heartbeat,
`fresh` wipes and rebuilds, `status` reports what stands; `--help` lists the
rest. Var roots are single-copy and git-ignored, so `wipe` guards the live one.

Play: `cargo run -p netstratum-cli` (`help` lists everything — found, zone,
star, site, thermal, journal, save/resume). Autonomous board:
`cargo run -p conductor` (converges the archived eras, then beats; the
civilizations journal every 60-tick year onto `var/worlds/wall.html` — the
Terminal Wall — and `python3 tools/observer.py` prints the ledger board).

References live in `/opt/repos/` (the ancestry era — consult, never port
blindly) and `/opt/ipsal-autic-v0.4/` (the sealed engine build and its organ
map, `docs/GROUND.md`).
