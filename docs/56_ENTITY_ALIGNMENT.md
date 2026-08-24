# 56 — ENTITY ALIGNMENT: THINKING DOWN, KNOWLEDGE UP

Operator framing (2026-08-24): *"thinking needs to be pushed down to the
entity level, knowledge needs to be consolidated at the civilizational
level."* This doc is the audit (`ENTITY-ALIGN-0`, task #193) and the
concrete design for the two slices that follow it (`#194`, `#195`).

## 1. THE MISALIGNMENT, GROUNDED

Three findings, each traced to exact call sites, not inferred from intent.

### 1a. Agency is civ-orchestrated, not entity-initiated

`Polity::research_step` (`engine/conductor/src/lib.rs:3470-4050`) is the
whole cognition beat, and it is ONE function that:

1. Reads `digest = receipt.own().0` — the CIV's latest chronicle digest.
2. Reads `difficulty = Difficulty::level(self.level)` — a property of the
   CIV's own rung (`solver-verify/src/forge.rs:41-48`), identical for
   every vessel.
3. Forges exactly ONE task: `forge_task(digest, difficulty)`.
4. Tells `self.automaton` to think it (`solver::think_task`).
5. Tells `self.sibling` to CONFIRM the SAME task
   (`self.sibling_deposit(&task, digest)`, line 4048) — not to think
   about anything of its own choosing.

Neither vessel has a loop, a task queue, or a "do I want to think about
this" gate of its own. `research_step` decides "we think about X now" for
the whole civilization, then dispatches both vessels at it. This is the
opposite of "entities think, civ consolidates" — today the civ thinks
(chooses the ONE task), and entities merely execute the choice.

### 1b. Entities are flat, not tiered

Exactly two vessels ever call `think_task`: `automaton` and `sibling`
(`steward` is explicitly excluded — `ipsal.rs:1-14`, "the steward is no
longer a thinker," directive-drain only). Their only differentiation is
COMM-1's discover/confirm split (`sibling_deposit`'s own doc comment,
line 3322: "deposit-only — no value mint, the automaton is the line's
value producer"). There is no growth in entity count, no distinct
cognitive role beyond that one axis, and no analogue of convergence-
engine's founder (absorbs/integrates), derived (refines), emergent
(explores) tiers (`/opt/ipsal-autic-v0.1/repos/convergence-engine`,
`src/lineage.rs`).

### 1c. Consolidation is a binary gate, not a graded process

`mint_solver_word_as` (`engine/cfe/src/world/tech.rs:261-299`) mints the
instant `cited.len() >= 2` — two distinct vessels, once, forever, no
further curation. `consilience_deposit` (`lib.rs:1117-1168`, twice-fixed
this session for correctness) already tracks vessel identity correctly;
what it does NOT track is how INDEPENDENT those two vessels' origins
were. Convergence-engine's `MeshAxiom.hardness`
(`src/membrane.rs:279-313`) grades confidence by mean pairwise
`origin_drift` among confirming entities — agreement between entities
that started from genuinely different places counts for more than
agreement among near-clones. Netstratum has no equivalent: two
automaton-class vessels agreeing counts exactly the same as two
genuinely differently-seeded ones would, because there is currently only
ever one of each.

## 2. WHY #194 AND #195 ARE ORDERED THIS WAY

`#194` (graded consolidation) is deliberately BEFORE `#195` (entity
tiers), not after, even though tiers sound more foundational. Reason:
grading requires SOMETHING to measure diversity BY, and today's two-
vessel model already has a real, measurable diversity signal — `automaton`
discovers, `sibling` confirms, and their basins (`self.grown`) accumulate
independently from different task histories (the sibling's `absorb_autic`
call and its own `self.grown.entry(sibling)` basin diverge from the
automaton's over time). Grading can land and be TESTED against the
existing two-vessel population before the population itself grows —
proving the grading law works on the simple case is the reference oracle
for the harder N-vessel case `#195` introduces.

## 3. #194 DESIGN — GRADED CONSOLIDATION

**The measure.** Define `origin_drift(a, b)` between two confirming
vessels as a function of what's already resident and cheap to read: the
SYMMETRIC DIFFERENCE of their grown basins (`self.grown.get(&vessel)` is
`Vec<Crystal>`, `Crystal { class: ChainDelta }` — `ChainDelta` derives
`PartialEq, Eq` but not `Hash`, so the set operation is a linear
`contains` scan, not a `HashSet`, bounded by `AUTIC_BASIN_CAP = 4096` per
vessel). Two vessels that have independently proven mostly-disjoint
crystal sets have genuinely diverged; two vessels whose basins are
near-identical (e.g. one only ever confirms, never discovers anything
the other didn't already have) drifted less.

```text
drift(a, b) = |basin(a) Δ basin(b)| / |basin(a) ∪ basin(b)|   ∈ [0, 1]
```

Exact-rational (a ratio of set sizes, both `usize` — no float, matching
[[no-scalar-law]]).

**The gate, extended not replaced.** `mint_solver_word_as` keeps its
`cited.len() >= 2` floor (identity still gates whether a mint is even
POSSIBLE — #194 does not relax that). What changes: the CALLER
(`deposit_toward_word`) computes `drift` between the two confirming
vessels and threads it onto the `SolverWordMinted` receipt as a new
field (`hardness: Ratio`, next-free tag — claim before landing, mirroring
[[register-claim-discipline]]). This is purely ADDITIVE to the wire (a
new field on an existing receipt, not a new gate), so it cannot break
replay of any existing chronicle — old receipts simply have no
`hardness` reading (or a schema migration default), matching every prior
additive landing this session (`RegionCharted`, `ChronicleCheckpointed`).

**What hardness DOES, once it exists.** Not scoped for #194 itself
(mint still succeeds identically) — but the field becomes the resource a
LATER slice can spend: word `pre_reach` (the "try a library word whole
before searching," `lib.rs:3924`) could prefer higher-hardness words when
several candidates tie, or a low-hardness word could be a candidate for
re-confirmation rather than treated as settled forever. Recording the
measurement is #194's whole job; spending it is deliberately deferred so
#194 stays small and independently testable.

**Test shape** (mirrors this session's `crossing_candidates_pins.rs`
reference-oracle discipline): a fixture with two vessels sharing an
IDENTICAL basin should measure `drift ≈ 0`; two vessels with disjoint
basins should measure `drift ≈ 1`; a differential test against a naive
set-difference reference, not a re-derivation of the same formula.

## 4. #195 DESIGN — ENTITY TIERS

**Do not copy convergence-engine's population mechanics wholesale.**
Their model (spawn caps, DRAIN/CREATE) is tuned to ITS OWN economy; the
right question for netstratum is what a founder/derived/emergent
DISTINCTION would concretely change about `think_task` dispatch, not
whether to import their exact numbers.

**Proposed roles, mapped onto EXISTING primitives (no new type needed
for the first landing):**

- **Founder** — reads the FULL accumulated basin before choosing what to
  attempt (closest to today's `automaton`: it's the "value producer,"
  `sibling_deposit`'s own words). No change needed here — automaton
  already plays this role.
- **Derived** — works the SAME task as the founder but on a NARROWER
  slice (today's `sibling`, restricted to confirm-only). The gap: give
  it occasional CHOICE — a beat where, instead of confirming the
  automaton's task, it independently forges its OWN task off a
  DIFFERENT digest (e.g. its own basin's latest crystal hash rather than
  the civ's chronicle digest) and pursues it. This is the FIRST real
  entity-autonomy: one vessel occasionally picks its own problem.
- **Emergent** — does not exist today. Population growth (a THIRD
  vessel) is the part of this slice that's genuinely new construction,
  not a role relabel — gate it on something already civ-derived and
  earned (e.g. `evolution_stage` or `civ_reach`, matching how every
  other growth axis in this codebase is earned, never granted), so a
  young civ stays at two vessels and a mature one can afford a third.

**Order of landing within #195** (so each piece is independently
testable): (a) let `sibling` occasionally forge its own task instead of
confirming — behavior change, no new entity, smallest possible slice;
(b) measure whether that alone produces a measurable hardness lift via
#194's new field; (c) only then add the third (emergent) vessel, gated
on an earned civ property.

## 5. WHAT THIS DOC DELIBERATELY DOES NOT DO

No code changes ship with this doc. `mint_solver_word_as` and
`consilience_deposit` were both touched twice already in the session
that wrote this audit (the digest-vs-identity bug, then the
`word_pending` leak) — a third edit to the same functions, in the same
sitting, without this design existing in writing first, is exactly the
kind of rushed change [[entity-align-arc]] was created to prevent.
