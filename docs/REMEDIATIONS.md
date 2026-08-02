# REMEDIATIONS — the deterministic-cost checklist

**Lane:** `remediations-lane` (worktree `.claude/worktrees/remediations-lane`).
**Opened:** 2026-08-02. **Base:** `main` at `540ac96`.

## §0bis SUMMARY — the board in one page

> A map, never a second authority (`EQUATIONS` §0.1). Restated facts
> rot; this section holds only **measured numbers** and **pointers**.

**THE MEASUREMENT.** Beat cost at 33 worlds: **1.52 s**, of which
`research_step` is **94.2 %** (1432.9 ms), `merchant_step` 4.0 %,
`chain_beat` 0.8 %. The forge search grew **≈2000× in 40 beats**;
everything else grew 3–13×. An 80-cycle run takes 198 s; 120 cycles does
not finish in ten minutes. Full tables: §1.

**THE FINDING.** The cartesian object is the forge search
(`solver/src/search.rs:124`) — breadth-first over the vocabulary
product, |V| 14–37, depth ≤3, ~52 060 candidates, each **re-derived from
the raw input**. `peel` and `shear` are the house cures and both exist;
this is where they were never applied.

**THE THREE DOCUMENTS.**

| file | what it settles |
|---|---|
| **this file** | R1–R18, the whole cost surface, each `[MEASURED]` / `[READ]` / `[INFERRED]` |
| **`FORGE_REMEDIATION.md`** | the plan for the 94 % — R1+R3 (≈3×, outcome-neutral), R4, R5, R2 |
| **`FORGE_BUDGET.md`** | `SEARCH_BUDGET` as a capacity reading — rows **B0–B8**, the six **axial capabilities** |

**WHAT LANDS WITHOUT A RULING.** R1+R3 behind one byte-identity pin
(≈3×); then R4; then the find-scans and re-derivations (R7–R13, R17–R18).
None carry economic content.

**WHAT IS BLOCKED, AND ON WHAT.**

- **R6** (stepping replay) rides `kernel-lane` **11a** — the same
  refactor as its #30. Do not design a second.
- **B0–B8** (the budget law) need `kernel-lane` **11a/11b** for the
  replay axis, and operator words for every value (`FORGE_BUDGET` §7).
- **R2** waits on the `from_key` wiring at N6 (~4 % today).

**THE STANDING LESSON OF THIS LANE.** Three times a conclusion was
reached from one axis and stated as if it covered the manifold — from
the *shape* of PoW rather than the code; from consensus-neutrality
rather than the six axes; from "leave coefficients unnamed" rather than
seeing that unnamed is how `200_000` was born. The cure is structural,
not resolve: **`FORGE_BUDGET` §9's axial capabilities** — a landing
names which capability it exercised, and satisfying one is never
satisfying the rest.

**TWO COLLAPSES ALREADY RECORDED AS DEFECTS, both inherited by the
budget law:** capacity read on the aggregate instead of per channel
(`bridge.rs:306`), and the four ideology axes collapsed into
`syntropy_ratio` (`EQUATIONS` §6 **S2**, against `09` §1.3). The
operator's *"multi-axial coordinated system"* is not a framing applied
from outside — it is the law's own shape, twice.

---

## §0 What this file is

The operator's standing question, in the operator's words: find the
**cartesian CRUD** — *iterative loops around n-dimensional objects that
operate on every potential of potential* — and every other place the
system pays a whole-history or whole-product price for an incremental
answer. `peel` (address one thing directly instead of scanning for it)
and `shear` (carry a cursor forward instead of re-sweeping) are the
house cures and already exist in the codebase; most of what follows is
a place they were named but not applied, or applied to the wrong twin.

This file is a **work board, not law.** An entry leaves it by landing
or by being struck. Every claim carries either `[MEASURED]` (a number
in §1 produced this session), `[READ]` (verified at the cited line, not
timed), or `[INFERRED]`.

**Status legend:** `[ ]` open · `[~]` in flight · `[x]` landed · `[!]`
blocked on a named ruling.

**Do not treat cost as the only axis.** §8 is load-bearing: this is a
proof-of-work system, and several of the largest wins below *reduce the
work performed per beat*. That is an economic change, not a free
optimization, and it is an operator ruling.

## §1 THE MEASUREMENT

Release build, `--cadence 0`, from the same 19-world founding baseline
each time (`install.sh --var <scratch>`), single machine, this session.

| cycles | worlds at end | wall clock | mean per beat |
|---|---|---|---|
| 20 | 24 | 13.06 s | 0.65 s |
| 40 | 33 | 59.29 s | 1.48 s |
| 80 | 44 | 197.98 s | 2.47 s |
| 120 | — | did not finish inside the remaining ~290 s of a 10-minute budget |

Marginal cost, beats 41–80: **3.47 s/beat.** Total wall grows about
quadratically in beats while the roster grows linearly — the per-beat
cost is rising, not flat.

**Phase attribution** (temporary instrumentation in `Polity::cycle` and
the beat loop; reverted after measuring — the reusable instrument that
remains is `engine/conductor/examples/cost_probe.rs`):

| phase | beat 1 (19 worlds) | beat 40 (33 worlds) | share at beat 40 |
|---|---|---|---|
| `research_step` | 0.7 ms | **1432.9 ms** | **94.2 %** |
| `merchant_step` | 0.13 ms | 60.6 ms | 4.0 % |
| `chain_beat` | 3.8 ms | 12.1 ms | 0.8 % |
| `scribe::record` | 3.1 ms | 7.3 ms | 0.5 % |
| `persist::sync` | 0.6 ms | 4.4 ms | 0.3 % |
| `market_step` | 0.32 ms | 1.5 ms | 0.1 % |
| tick · collect · inbox · build · survey · diverge | — | < 0.6 ms each | < 0.2 % |
| **beat total** | **9.5 ms** | **1.52 s** | |

**The forge search grew ≈2000× in 40 beats.** Everything else grew
between 3× and 13×. Any cost work that does not start at
`research_step` is optimising the 6 %.

**Single-world chain costs** (`cost_probe`, world `cinder-choir`):

| receipts | `persist::load` | chain verify | full `replay` | `state_bytes` | one chained beat (3 deeds) |
|---|---|---|---|---|---|
| 69 | 166.3 µs | 59.2 µs | 31.5 µs | 4.4 µs | 273.6 µs |
| 180 | 224.6 µs | 131.9 µs | 99.2 µs | 4.0 µs | 734.2 µs |

Per-beat chain cost is **linear in history** (2.6× the receipts → 2.7×
the cost). It is small today and it is the term that never stops
growing, because the roster plateaus and the chronicle does not.

## §2 THE FORGE SWEEP — the 94 %

> **SUPERSEDED IN PART by `docs/FORGE_REMEDIATION.md` (2026-08-02).**
> That file is the working plan for R1–R5 and corrects three things
> read wrong here: R1/R3/R4/R5 are **not** blocked on Ruling Box 1
> (the search is never re-run by a verifier, `mint_bound` keys off
> `Difficulty` alone, and the search runs once per world per beat — so
> speed has no path to the mint rate); **R4 is inert today** because
> `SEARCH_BUDGET` (200 000) is never reached at `word_len` 3
> (52 060 candidates), which makes now the free moment to fix it; and
> **R2 is worth ~4 %** at live depth, not the structural win implied
> below. The entries stand as findings; take the sequencing from the
> plan.

`engine/solver/src/search.rs:124` `solve()` is a breadth-first
enumeration over the **cartesian product of the vocabulary with
itself, `word_len` deep**. This is precisely the shape the operator
named: every potential of every potential.

The space, measured from the source:

- vocabulary = 6 symmetries (`search.rs:125-132`) + 8 `OBJ_OPS`
  (`solver-verify/src/objects.rs:147`) + ≤7 derived recolors
  (`search.rs:33`) + 0 or 16 derived tiles (`search.rs:79`,
  `TileMirror::ALL` is 4 → 4×4) = **|V| between 14 and 37** `[READ]`
- depth = `word_len`, capped at 3 by `Difficulty::level`
  (`solver-verify/src/forge.rs:41`) and 4 by `from_key` (`:61`) `[READ]`
- full frontier at |V|=37, depth 3 ≈ **52,060 candidate programs**,
  each tested by `closes()` re-running the whole program over every
  training pair `[READ]`
- `SEARCH_BUDGET = 200_000` (`search.rs:119`) is a candidate count, so
  the sweep runs to exhaustion at these sizes rather than being cut
  short `[READ]`

- [ ] **R1 — PREFIX-SHARED EVALUATION (the shear, applied to the search).**
  `closes()` (`search.rs:105`) re-runs the program **from the raw
  input** for every candidate. BFS already holds every prefix in
  `frontier`; each of a node's |V| children re-derives that node's
  entire output before applying its one new op. Carry the intermediate
  grids on the frontier entry and each test becomes **one** op
  application plus a compare. Saving ≈ `word_len`× on the dominant
  phase, with **zero change to what is found or to the certificate**.
  This is the single best cost-per-risk item on the board. `[READ]`

- [ ] **R2 — CANONICAL FORM ON GEOMETRY WORDS (peel, applied to the
  space).** The six symmetry generators generate the dihedral group of
  the square — **at most 8 distinct transforms.** Every word of length
  3 drawn from those six is one of 216 spellings of one of ≤8
  functions. Composing consecutive geometry ops into a normal form
  before testing removes provably-duplicate candidates without
  removing a single reachable function. `derived_recolors` already
  uses exactly this idea (its 7 normalization prefixes). Measure the
  collapse before claiming a factor. `[READ]`

- [ ] **R3 — STREAM THE FRONTIER, DO NOT MATERIALIZE IT.**
  `search.rs:161-173` builds `next: Vec<Program>` holding up to `room`
  (≈200k) fully-cloned programs, each a heap `Vec<Op>`. At |V|=37,
  depth 3 that is ~50k allocations per world per beat, ~1.65M per beat
  across a 33-world board. The k-th candidate is addressable
  arithmetically from a mixed-radix counter — **find the k-th
  potential directly, never build the product.** Peel, exactly. `[READ]`

- [ ] **R4 — THE BUDGET STARVES THE INFORMATIVE OPERATORS.**
  `vocabulary` is ordered `[symmetries, object ops, derived_recolors,
  derived_tiles]` (`search.rs:125-135`) and the growth loop breaks at
  `room` (`search.rs:165`). So when the budget bites, the operators
  **derived from the observation itself** — the ones most likely to
  close the task — are the first never enumerated. Ordering the
  derived ops FIRST is a one-line change that strictly improves the
  solve rate at identical cost. This is closer to a defect than an
  optimisation. `[READ]`

- [ ] **R5 — MEMOIZE `(digest, difficulty) → outcome`.**
  `research_step` (`conductor/src/lib.rs:1072`) seeds the forge from
  `receipts().last().own`. A world that appended nothing this beat
  re-forges the identical task and re-runs the identical exhaustive
  sweep for the identical answer. `solve` is pure and deterministic;
  the repeat is provably wasted. Bound the cache to the live roster.
  `[READ]`

## §3 THE CHAINED BEAT — the term that never stops growing

- [ ] **R6 — `replay()` CANNOT STEP, SO EVERY DEED REPLAYS ALL OF
  HISTORY.** `chain_beat` (`conductor/src/lib.rs:1365-1384`) rebuilds
  the prefix chronicle and replays it **whole, once per new deed**:
  `receipts[..=at].to_vec()` copies the prefix, `Chronicle::from_receipts`
  re-hashes the entire chain (`cfe/src/vrp.rs:201`), and
  `world::replay` (`cfe/src/world.rs:4193`) is one monolithic 113-arm
  loop from genesis. The code's own comment concedes it: *"O(deeds ×
  history) per beat … the cure when it bites is a stepping replay in
  cfe."* Measured at 734 µs/world at 180 receipts and rising linearly.
  **This is the same refactor `kernel-lane` needs for task #30 — see
  §9. Do not design a competing one.** `[MEASURED]`

- [ ] **R7 — `persist::sync` READS THE WHOLE FILE TO FIND THE TAIL.**
  `cfe/src/persist.rs:56` is an append-only sync that first calls
  `load(path)` (`:63`) — reading the entire file, parsing every
  receipt, and re-verifying the whole hash chain — then compares every
  on-disk receipt to memory, purely to learn where to append. Called
  per world per beat at `conductor/src/lib.rs:1289`. Measured 224.6 µs
  at 180 receipts. A carried append cursor plus a tail-digest check
  gives the same divergence guarantee at O(1). `[MEASURED]`

- [ ] **R8 — `scribe::record` READS `chronicle.jsonl` TWICE PER BEAT.**
  `rotate_divergent` (`scribe/src/lib.rs:124-131`) reads the whole
  export and splits it into lines; then `:101-105` reads the very same
  file **again** just to `.lines().count()` an append cursor. Two full
  reads and parses per world per beat, growing with the export. Same
  cure as R7: carry the cursor. Already named as inventoried debt in
  PICKUP §8d. `[READ]`

## §4 THE FIND-SCANS — peel that does not peel

- [ ] **R9 — `peel` IS ITSELF A FIND-SCAN.** `cfe/src/world.rs:4939`:
  ```rust
  fn peel(orbs: &mut [ColonyOrb], orb: u64) -> Option<&mut ColonyOrb> {
      orbs.iter_mut().find(|held| held.orb_id == orb)
  }
  ```
  Its own doc comment says it exists so there is *"no cartesian
  find-scan repetition; peels and shears where there were iterations"*
  — and the body is a linear find. There are **67 scan sites in
  `world.rs` alone** and `peel`/`peek` account for most of them. The
  name is right and the implementation has not caught up: an
  `orb_id → slot` index (rebuilt on the rare membership change) makes
  every peel O(1) and makes the name true. `[READ]`

- [ ] **R10 — THE WHOLE ENGINE IS Vec-AS-MAP.** Across ~46k lines,
  only **two** files use a keyed map at all (`mesh/src/arbor.rs`,
  `cfe/src/query.rs`); everything else is `Vec` plus linear lookup.
  This is the structural root of R9 and of R11. It does not need a
  wholesale rewrite — it needs an index on the collections that are
  hot and keyed (orbs, offers, techs, debts, covenants). `[READ]`

- [ ] **R11 — `deposit_receipt` IS O(receipts × orbs).** Its inner
  `deposit` helper (`cfe/src/world.rs:4144-4150`) does
  `sheet.iter().position(...)` per receipt to find the orb's slot in a
  `Vec<(u64, CrystalDeposit)>`. The shear cursor above it is correct;
  the sheet lookup inside it is not. `[READ]`

- [ ] **R12 — `verify_orbit_witness` LINEAR-FINDS THE OPERATOR.**
  `rfa-core/src/fold.rs:249` — `operators.iter().find(|op| op.id == …)`
  inside the judge's half. Small today (operator tables are short) and
  it is the one scan sitting on a constitutional path, so it should be
  a keyed lookup on principle. `[READ]`

## §5 THE FULL RE-DERIVATIONS

- [ ] **R13 — THE NON-INCREMENTAL TWIN IS STILL LIVE.**
  `cfe/src/world.rs` has both `crystal_deposits_at` (`:1079`, the
  correct #41 incremental shear) and `crystal_deposits` (`:1094`,
  a full chronicle rescan that opens `let _ = now;` and never prunes
  the window). `constellation()` (`:1182`) calls **the full-rescan
  twin.** So the one read a player hits most often pays the whole
  history every call, and — separately — diverges from what actually
  crystallizes. PICKUP §8e already records the divergence as a bug;
  it is also the cost item. One cure closes both: delete the twin and
  route `constellation` through the cursor. `[READ]`

- [ ] **R14 — `committed()` CLONES 22 COLLECTIONS.**
  `cfe/src/world.rs:2905` deep-clones every committed family to
  produce a snapshot; `chain_beat` calls it again for
  `codec::state_bytes` at `conductor/src/lib.rs:1403`, and
  `Polity::cycle` calls it once more just to list orb ids
  (`:1276`). The orb-id listing needs no snapshot at all — that one
  is free to fix today. `[READ]`

- [ ] **R15 — THE FULL STATE IS SERIALIZED AND WRITTEN EVERY BEAT.**
  `codec::state_bytes` is cheap per call (4 µs measured) but the
  arbor writes a **whole-state blob per commitment**
  (`mesh/src/arbor.rs:371`, `:622`). Disk grows with beats × state, not
  with change. A delta-or-blob choice at the commitment is a wire
  question, not a code question — record it, do not act alone. `[MEASURED]`

- [ ] **R16 — `mesh::walk` RE-WALKS THE CHRONICLE FOR PEN AUTHORITY.**
  `mesh/src/lib.rs:234`, called from `:267`, `:280`, `:308` — each a
  full pass to learn who holds the pen. Cheap now (measured inside a
  3.7 µs `pen_service`), listed for completeness so it is not
  rediscovered later. `[READ]`

## §6 THE CARTESIAN PRODUCTS OVER THE ROSTER

- [ ] **R17 — `merchant_step` IS A TRIPLE LOOP OVER WORLDS, O(W³).**
  `conductor/src/lib.rs:1445`, loops at `:1453` / `:1457` / `:1461`.
  At 44 worlds that is 85,184 cells per beat. Measured 60.6 ms at 33
  worlds — 4 % today, and it is **cubic**, so it overtakes everything
  else as the roster grows. Two cures, both pure:
  1. **The `m` loop is outermost but the pair qualification does not
     depend on `m`.** `ask_a`, `ask_b`, `d_ab` and `pair_melt(a,b)`
     (`:1465-1474`) are recomputed W times for the same pair. Hoisting
     the pair test out of the `m` loop is a mechanical restructure:
     O(W³) → O(W²) + O(qualified pairs × W).
  2. The pair test is a **key match** (`ask_b.want == ask_a.give &&
     ask_b.give == ask_a.want`, `:1469`). Index the roster's first
     offer by `(give, want)` once per beat — O(W) — and complementary
     pairs are *found*, never swept. Most beats have none. `[MEASURED]`

- [ ] **R18 — `market_step` IS O(W² × offers²).**
  `conductor/src/lib.rs:285`; pair loops at `:316-317`, offer loops at
  `:330-331`, and it calls the **mutating** `clear_trade` on every
  combination to discover whether it was viable — operating on every
  potential of every potential, literally. Cheap today (1.5 ms) purely
  because books are short. Same index cure as R17. `[MEASURED]`

## §7 NOT A REMEDIATION — these are already right

Recorded so they are not "fixed" by a later reader:

- **The prover/judge split at the fold gate.** `orbit_lock`
  (`rfa-core/src/fold.rs:205`) sweeps operators and is **prover-side
  by construction**; the judge calls `verify_orbit_witness` (`:239`),
  O(step), no search. This is the house pattern (`solver` /
  `solver-verify`) correctly applied. PICKUP §8a's altitude correction
  is satisfied in the code.
- **The #41 shear cursor** (`cfe/src/world.rs:1079`) is a correct
  incremental sweep with a pruned window. R13 is about its abandoned
  twin, not about it.
- **`Chronicle::receipts()`** returns a slice, not a copy. The
  incremental shear is genuinely incremental on the receipt side.
- **`SEARCH_BUDGET` as a candidate count rather than a clock**
  (`search.rs:119`) is correct and must stay: a deterministic bound is
  what makes the forge replayable. R1–R4 change what a candidate
  costs, never what counts as one.

## §8 THE CAVEAT — WITHDRAWN ON EVIDENCE 2026-08-02

**This section originally read: "This is a proof-of-work system. R1–R5
make the forge cheaper, and the forge is the work" — and concluded that
R1–R3 were blocked on Ruling Box 1. That conclusion was wrong, and it
was wrong because it reasoned from the shape of the system instead of
reading the consensus path.** Struck rather than rewritten (the history
law); the correction and its evidence are `docs/FORGE_REMEDIATION.md`
§1, in one line each:

- `verify_work` (`solver-verify/src/work.rs:266`) re-applies the
  **presented** program and never enumerates a candidate, so the search
  is prover-side and consensus-free;
- `mint_bound` (`kernel/src/lib.rs:845`) is a table on `Difficulty`
  alone — not on cost, not on wall-clock, not on which program was
  found;
- `research_step` (`conductor/src/lib.rs:1072`) runs the search **once
  per world per beat** and mints at most once, so a faster search
  yields the same solve sooner, never more solves.

- [x] **R19 — STRUCK.** There is no forge-cost ruling to wait on.
  R1, R3, R4 and R5 are **unblocked** and sequenced in
  `FORGE_REMEDIATION.md` §4; R2 is held for the `from_key` wiring at
  N6 on its own (small) merits. Ruling Box 1 still governs the
  `level → Difficulty` ladder and the rung→reward table — this lane
  proposes no change to either.

  §3–§6 (R6–R18) carry **no** economic content — they are pure
  overhead around the work, and none of them need a ruling. That part
  of the original reading stands.

## §9 CROSS-LANE ALIGNMENT — verified 2026-08-02

`git worktree list` plus `main..<branch>` this session:

| lane | ahead | behind | territory now |
|---|---|---|---|
| `kernel-lane` | 19 | 19 | kernel parameters, `C_bound` containment, ruleset epochs, task #30 |
| `lens-lane` | 5 | 18 | the display peel, the luminal layer, kinematic pressure |
| `capacity-lane` | 5 | 18 | `19_REGISTRY`, `20_FOUNDING`, the purse |
| `syntropy-inscription` | 1 | 0 | the value-loss book |
| `ground-floor` | 0 | 18 | legacy — nothing to merge |
| `remediations-lane` | this | 0 | this file, `cost_probe` |

**The honest answer to "is everyone working toward the kernel/mesh
integration":** not directly, and that is mostly correct division of
labour — but three things need saying.

1. **R6 AND `kernel-lane` #30 ARE THE SAME REFACTOR.** `kernel-lane`
   reverted P3 at `df48786` after it exposed a live/replay divergence:
   *"the transition is defined twice, in replay's 113 arms and in the
   live proposer, and they agree only while the shared rules stay
   still."* Its cure (`2f40fed`, 11a) makes `apply()` a zero-logic
   wrapper over `judge()`. **That is exactly the restructure R6 needs
   to make `replay()` steppable.** One refactor pays a correctness debt
   and a cost debt. This lane must not design a second one — R6 rides
   11a. Relay owed to `kernel-lane`.

2. **FOUR LANES ARE 18–19 COMMITS BEHIND `main`, and `kernel-lane` is
   19/19 diverged.** That is the largest integration risk on the board
   and it is nobody's assigned task. `capacity-lane` is already gating
   its own §6a on kernel #30, so the coupling is live, not theoretical.

3. **VOCABULARY COLLISION — "peel" now names two different things.**
   `lens-lane` landed `THE PEEL — magnification is a depth` (`4af07c5`),
   a display/magnification concept. `cfe::world::peel` is the
   data-access idiom in R9. Both are legitimate; the collision should
   be recorded before a third reader merges them by name.

## §10 PICKUP CORRECTIONS — verified against the code this session

PICKUP §8 is dated 2026-08-01 and three of its load-bearing claims have
been overtaken. Corrections are carried into `PICKUP.md` §8a/§8b
alongside this file.

- **§8a "The judge does not judge. Nothing in production calls
  `lawful`." — NOW FALSE.** `kernel::lawful` is called in production at
  `conductor/src/lib.rs:1380`, per deed, every beat, with refusals
  collected into `self.refused`. `[READ]`
- **§8a "`engine/rfa-core` has ZERO dependents." — NOW FALSE.** Two
  crates depend on it: `kernel` (`engine/kernel/Cargo.toml:18`) and
  `solver` (`engine/solver/Cargo.toml:12`). `[READ]`
- **§8a's real remainder STANDS, and should be restated narrowly:**
  `mesh::arbor::extend` is still passed `None` at
  `conductor/src/lib.rs:1403`, so every commitment is still stamped
  `judged = false`. The judge now runs **beside** the chain, not
  **inside** it. The Phase-3 flip is genuinely open; the sweeping
  version of the claim is not. `[READ]`
- **§8b's codec entry is STALE.** It reports `SCHEMA_V2` half-sealed
  while `state_bytes` emits v2. `state_bytes` emits `SCHEMA_V3`
  (`codec/src/lib.rs:68`) and v1/v2/v3 all decode (`:320`). The
  lingering item is only the `CODER-04 TO RATIFY` comment still
  attached to V2 at `:37`. `[READ]`
- **§8e's duplicate `constellation` key is CONFIRMED STILL LIVE** —
  `lens/src/render.rs:121` and `:204` both emit the key into the same
  object. `[READ]`
