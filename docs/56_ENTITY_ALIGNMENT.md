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

**The measure.** Corrected during implementation to mirror
`solver::consilience`'s ALREADY-DEPLOYED algorithm exactly
(`selfmap.rs:225-231`: `hardness = orders.dedup().len()` — the count of
DISTINCT curiosity-orders among the converged lines), rather than a
freshly-invented continuous ratio. At vessel scope: read each confirming
vessel's grown basin (`self.grown.get(&vessel)` is `Vec<Crystal>`,
`Crystal { class: ChainDelta }`), reduce it to its DISTINCT classes (a
linear `contains` scan — `ChainDelta` derives `PartialEq, Eq` but not
`Hash`, bounded by `AUTIC_BASIN_CAP = 4096` per vessel), then count how
many vessels among the group carry a genuinely DIFFERENT set of classes
(`same_basin_knowledge`: two readings are the same iff they contain
exactly the same classes, order and duplicate count ignored — a basin is
a proven SET, not a sequence). `hardness: u32`, matching
`KnowledgeConverged`'s own field type exactly — no float, no Ratio, an
integer count like its precedent. For today's two-vessel population this
is always 1 (identical knowledge, no diversity bridged) or 2 (genuinely
different knowledge) — it generalizes to `[1, vessel_count]` once #195
grows the population.

**The gate, extended not replaced.** `mint_solver_word_as` keeps its
`cited.len() >= 2` floor (identity still gates whether a mint is even
POSSIBLE — #194 does not relax that). What changes: the CALLER
(`deposit_toward_word`) computes hardness between the confirming vessels
and closes it as a SEPARATE receipt alongside the mint — **NOT** a new
field on `SolverWordMinted` itself. Corrected during implementation
(2026-08-24): `SolverWordMinted`'s `expansion` is encoded as "everything
remaining in the record, opaque, no length prefix" (`dop.rs:5673`,
`value.get(at..)?.to_vec()`) — there is no safe place to append a
trailing field without either being silently swallowed into `expansion`
or shifting the read offset for every OLD receipt already on disk. It
also collides with the wire law directly: "tags are permanent: one tag,
one meaning, forever" (`tlv.rs:4`) — reshaping tag 166's payload IS
changing its meaning. `hardness` lands as its OWN new tag
(`SolverWordHardness { orbit: [u8; 32], hardness: u32 }`), closed as a
second deed right after the mint succeeds — purely additive, cannot
touch any existing receipt's bytes, matching how `KnowledgeConverged`
(tag 177) already keeps its own `hardness: u32` as a field on ITS OWN
event rather than retrofitted onto something else's.

**What hardness DOES, once it exists.** Not scoped for #194 itself
(mint still succeeds identically) — but the field becomes the resource a
LATER slice can spend: word `pre_reach` (the "try a library word whole
before searching," `lib.rs:3924`) could prefer higher-hardness words when
several candidates tie, or a low-hardness word could be a candidate for
re-confirmation rather than treated as settled forever. Recording the
measurement is #194's whole job; spending it is deliberately deferred so
#194 stays small and independently testable.

**Test shape** (mirrors this session's `crossing_candidates_pins.rs`
reference-oracle discipline): two vessels sharing an IDENTICAL basin
measure `hardness == 1`; two vessels with any genuinely different class
between them measure `hardness == 2`; the wire round-trips
(`SolverWordHardness` encode/decode); `SolverWordMinted`'s own bytes are
BYTE-IDENTICAL before and after this landing, proven by re-running the
existing mint-shape tests unchanged.

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
