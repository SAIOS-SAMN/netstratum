# PICKUP — the state of the board (2026-07-27, coder-01; refreshed
# post-ideology-board by coder-02 the same day; refreshed 2026-07-30 by
# coder-02 — the galaxy law + the fifth progenitor seat)

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
| APERTURE-1 (2026-07-29) | the cross-world value carrier (NS-4 §1–§3, tags 47–48): two-legged deal-linked deed, demurrage at the touch both sides, per-world replay, pair conservation an N6 reading; the market board's gate is open |

Design authority now also includes `12_UNION.md` (the union world-book,
eight operator rulings 2026-07-29: bands as space, four progenitor
species ×8 seats, Watcher/Observer/Sovereign, the lineage law, discrete
gauge morphodynamics — Q3 resolved, the substrate law, the implementation
manifold). **The union board landed whole in first slices (2026-07-29,
BUILD_ORDER §3b):** SUBSTRATE-1 · LINEAGE-1 · TECH-1a (the solar leap) ·
THRESHOLD-1 (the crossing + shield) · AEON-1a (G, key changes, star
aging, the fold) · ELEVATE-1 (the three-gate climb) · BAND-1 (seats) ·
COUNCIL-1 (the multilateral covenant) · PLANNER-1 (the licensed leap;
the solar leap is never licensed). **Aeon 1 is live at the default `var/`**
(the old test universe deleted by ruling 2026-07-30; 15 civilizations,
5 progenitors seated — wave 2, 2026-07-30: **Tunborn**, the tardigrade
line's own head (12_UNION §2's fifth seat; Slagborn's lineage resolves to
it) — 2 pre-solar shielded worlds; seed roster in
`seeds/aeon1-egregores.json`, CREATED 2026-07-30: it had been referenced
but never written, and `var*/` is git-ignored, so the seeds copy is the
durable record — keep it appended on every wave). **The galaxy law is
ruled and specced** (`13_GALAXY.md`, 2026-07-30): ONE universe
structurally; the whole charted torus is the one founding galaxy; new
galaxies are never a static count — created lazily by the observation
deed (`GalaxyObserved`, tag 54 — INSCRIBED at GALAXY-1a's landing,
2026-07-30: `mirror::galaxy` — the deed `observe_galaxy`, the lazy
founding charter, derived membership `galaxy_of`, the `galaxies.json`
rendering; NS-1 §2.3 row 54; 6 witnesses in `galaxy_pins.rs`; the
frontier is a DISTANCE — wrapped-L1 to every center vs reach, never an
outside). GALAXY-1b LANDED
2026-07-30: the observation sweep rides the year voice in the conductor
(both beat loops), creator resolves through the lineage law, and `t0
galaxies` reads the charted skies (galaxy board rows: name · center ·
reach · derived member count · intergalactic tally). Operator consoles landed in the CLI:
`npcs pause|resume|speed N` (writes `var/conductor.ctl`, polled every
beat) and `civ seats` / `civ new` (custom civilizations with live
station-seat checks, rostered for the conductor). Convergence no longer
self-annexes the live root — era-qualified aliases of living worlds are
gone; only `<base>-eraN` archives converge (the fold is the one door). Open second slices: TECH-1b
(the crystallized tree), AEON-1b (conductor orchestration: voice
triggers, divergence speciation, fold rotation), the market board
(§3a — its APERTURE gate stands open), and the union CLI verbs.

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

1–48 and 52–58 assigned (see `engine/cfe/src/dop.rs` header — the one
authority): 39 TechDiscovered · 40 TechLicensed · 42–46 the AEON block ·
47–48 the APERTURE pair · 52–53 the COUNCIL pair · 54 GalaxyObserved
(GALAXY-1a, universe ledger only) · 55 CrystalDrawn + 56–58 the NS-2
protocol records (N4, coder-03 — in-value codes: Resource::Crystal = 7,
ZoneKind::Resonator = 7); 8–9 reserved (D2 terrain, D3 milestones);
49–51 reserved the market board (§3a); **next free 59** for anything
outside those reservations.

## 5. Lanes and next work

- **coder-01**: next = Phase 2 cross-world trade (egregores exchanging
  joules/`S_cool` — needs a cross-world carrier; transfers are intra-world
  today) or TECH-1 (`10_TECH_EMERGENCE.md`, opens SINK-1's deferred
  research/throttle gates).
- **coder-02**: ideology board LANDED whole (09, deferrals at Q4);
  GALAXY-1 LANDED whole (13_GALAXY, 2026-07-30); the interleaving
  stress witness LANDED (HANDOFF §9.1 struck, `world_interleave.rs`).
  Standing relay to coder-01: the state.json cohesion pin (HANDOFF
  §9.2) and the JOURNAL follow-on (§9.3). Next: the coder-03 N4
  boundary sign-off (the flagged excursions below), then D2/D3 or as
  assigned.
- **coder-03**: N4 (NS-2 RQP + the scanner) — LANDED on main
  2026-07-30 (33e6c73 + post-ruling inscriptions 7f739f4, after the
  fifth-set rulings): spec §1–§3 with extracted vectors, sigil index +
  scan (`engine/cfe/src/query.rs`), CrystalDrawn (tag 55), crystals +
  the resonator (02 §5.1 deferral paid), the `scan` CLI verb; 187/187
  workspace witnesses, clippy zero, arc played. Boundary excursions
  flagged for coder-02 sign-off: dop.rs body (resource/zone/event),
  world.rs (scan + replay arm), five mechanical renderer arms in
  mirror (the P1 relay).
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
- `var*/` archives are single-copy (git-ignored). The ORIGINAL test
  universe (124 worlds, its eras, its quarantined ruins) was DELETED by
  operator ruling 2026-07-30 — "we dont need to maintain any of the old
  system, it was just a test." Aeon 1 now lives at the default `var/`;
  its seed roster is `seeds/aeon1-egregores.json`.
- Era carriers: mirror jsonl (observation) + cfe tlv (resume) coexist by
  design.

## 7. Design docs not yet built

`10_TECH_EMERGENCE.md` (TECH-1 spec) · 05 §1.4 sink trade equilibrium
pricing · 07 §3 derelict recovery · OPEN_QUESTIONS Q1–Q4
(rejected-with-reopening-paths). (01 §7's escrow mechanic BUILT as
COVENANT-1; its cross-boundary NS-7 settlement §7.3 still awaits N7.)
