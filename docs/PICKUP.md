# PICKUP — the state of the board (2026-07-27, coder-01; refreshed
# post-ideology-board by coder-02 the same day)

Cold-start orientation for any node resuming work. Read
`00_CHARTER.md` whole, then `BUILD_ORDER.md` for the current landing —
this file is the fast map, never the authority. The working discipline,
state-model laws, and earned gotchas live in `HANDOFF.md`.

## 1. Run it

```
cargo run -p netstratum-cli            # hands-on world (help lists all)
cargo run -p conductor -- --cadence 2  # the autonomous board (egregores)
python3 tools/observer.py --watch 2    # the ledger board
python3 -m http.server 8080 -d var/worlds   # serve wall.html (the Terminal Wall)
python3 tools/steward.py               # inject directives into live worlds
```

Conductor boot order: adopt legacy `bots.json` → CONVERGE archived eras
(idempotent, quarantines named) → genesis egregores → beat. Journals + the
wall regenerate every 60-tick year.

## 2. Landed (all witnessed; 158/158 green, clippy zero at `22a5995`)

| landing | what |
|---|---|
| N0–N3, P1, P2, D1, D2 | scaffold · VRP chain · first slice · entities · multi-planet charter · persistence · costs/famine · industry |
| CONVERGE-1 | era annexation onto TUniversal{S,G,T}; torn-tail law; quarantine; four-C placement |
| THERMAL-1 | `T_eq⁴` master equation drives the tick; StarClass; encase-first/vent-overflow; opt-in per orb |
| VENT-1 / ECON-J / SINK-1 | entropy dual ledger (`VentSettled`); heating burn + freeze-lock; bankable sink + FLAT demurrage |
| JOURNAL-1 | the Chronicle Feed: per-year journals + cyber-noir `wall.html`, pure rendering, deterministic |
| the ideology board, whole (coder-02) | TRADE-1 transfers/compassion (30) · ABSORB-1 absorption/morality windows (31–32) · BOND-1 standing bonds/love (33–34) · COVENANT-1 vitrified escrow + fidelity curve/integrity (35–37) · IDEO-1 friction (38) + orb cohesion entry (41) + `ideology` panel — eight books, ratios derived never stored; deferrals at Q4 |
| BOT-THERM | egregores thermally alive: Furnace mints, Frost banks sink and honestly freeze-locks |
| renames | Crucible→Furnace · bot→egregore/automaton (NS-6 §5.2) · themed daughter names |

Recent defect rulings (regression-pinned): **flat demurrage** (proportional
decay exploded canonical denominators to ~2^1100 — never proportional);
**chronicle rotation** (recurring founding names across eras interleaved two
timelines in one mirrored file — divergent residents rotate to
`<name>--closed-<digest8>`).

## 3. The laws in one breath

Exact rational arithmetic everywhere, no floats, roots never taken (work in
`T⁴`); receipts are the world (SEAL B), unknown tags skip (SEAL A), adapters
never hold logic (SEAL C); the mirror reflects, never authors; annex never
merge; one landing at a time, red-first, spec+mechanic+vectors together;
terminology law — no set-letter glyphs, Furnace never Crucible, no
"Goldilocks", egregore/automaton never "bot".

## 4. Wire tag registry state

1–38 and 41 assigned (see `engine/cfe/src/dop.rs` header — the one
authority); 8–9 reserved (D2 terrain, D3 milestones); 39–40 reserved
tech emergence; **next free 42**.

## 5. Lanes and next work

- **coder-01**: next = Phase 2 cross-world trade (egregores exchanging
  joules/`S_cool` — needs a cross-world carrier; transfers are intra-world
  today) or TECH-1 (`10_TECH_EMERGENCE.md`, opens SINK-1's deferred
  research/throttle gates).
- **coder-02**: ideology board LANDED whole (09 status LANDED;
  deferrals at Q4). Next candidates: the verification gaps named in
  `HANDOFF.md` §9 (the interleaving stress witness; the state.json
  cohesion pin — the latter a coder-01 relay), then D2/D3 or as
  assigned.
- Parallel worktrees per node; merge to `main` on green; BUILD_ORDER
  conflicts resolve by keeping both entries.

## 6. Known dark spots (all deliberate, none forgotten)

- 3 quarantined `--closed-*` corpses in `var/worlds` — real pre-fix
  corruption, preserved and named; leave as ruins.
- NS-2 / NS-4 / NS-5 / NS-7 are stubs by build order; N7 settlement gated
  by charter §7 + economy guardrail 5 (regulatory review). Nothing on any
  chain.
- SINK-1 research/throttle gates deferred to TECH-1.
- `World`→`Star` struct rename debt waits for the multi-planet landing
  (NS-6 §6).
- `var*/` archives are single-copy (git-ignored); operator declined/hasn't
  ordered a backup.
- Era carriers: mirror jsonl (observation) + cfe tlv (resume) coexist by
  design.

## 7. Design docs not yet built

`10_TECH_EMERGENCE.md` (TECH-1 spec) · 05 §1.4 sink trade equilibrium
pricing · 07 §3 derelict recovery · OPEN_QUESTIONS Q1–Q4
(rejected-with-reopening-paths). (01 §7's escrow mechanic BUILT as
COVENANT-1; its cross-boundary NS-7 settlement §7.3 still awaits N7.)
