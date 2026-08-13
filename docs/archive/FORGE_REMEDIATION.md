# FORGE REMEDIATION PLAN — the 94 %

> **ARCHIVED 2026-08-13.** Superseded/dead weight per the doc audit — kept for history, not live spec. See docs/BUILD_ORDER.md, docs/HANDOFF_2026-08-13.md, or the doc it names as its successor for current status.

**Lane:** `remediations-lane`. **Written:** 2026-08-02.
**Target:** `engine/solver/src/search.rs` — measured at **94.2 % of a
beat** (`REMEDIATIONS.md` §1).
**Companion:** `REMEDIATIONS.md` R1–R5. This file supersedes that
section's sequencing; §6 below records what changed and why.

---

## §1 THE SAFETY PROOF — read this before the changes

Everything proposed here is **prover-side only**, and that is not a
judgement call — it is readable in three places:

1. **`verify_work` never re-runs the search.**
   `solver-verify/src/work.rs:266` re-forges the task from its digest,
   re-applies the **presented** program to every training pair and the
   test input, and re-derives the certificate chain. It never
   enumerates a candidate. A node that searches differently — or not at
   all — verifies another node's proof identically. `[READ]`

2. **Any program that closes the task is a valid solution.**
   `forge_task` (`forge.rs:142`) samples a secret word, renders the
   pairs, and **discards the word**; its own comment: *"finding A word
   that closes them is the solver's work (it need not be the forged
   one; the verifier is the authority either way)."* So a change that
   makes the search return a *different but equally closing* program
   breaks nothing. `[READ]`

3. **The mint keys off `Difficulty` alone.**
   `kernel/src/lib.rs:845` — `mint_bound` is a table on the difficulty
   rung (levels 3/4/5 → 1, 4, 7; everything else 0). It does not read
   wall-clock, candidates tested, or which program was found. `[READ]`

And one fact that closes the economic question:

4. **`research_step` runs the search ONCE per world per beat**
   (`conductor/src/lib.rs:1072`) and mints at most once. A faster
   search does not produce more solves per beat — it produces the same
   solve, sooner. **Wall-clock has no path to the mint rate.**

> **Conclusion.** R1, R3, R4 and R5 change *how long* the same answer
> takes and *which of several equally valid programs* is returned.
> They change neither what verifies nor what mints. **They do not need
> Ruling Box 1.** `REMEDIATIONS.md` §8 said otherwise; §6 corrects it.

The one genuine economic lever in the forge is untouched here: the
`level → Difficulty` ladder (`forge.rs:41`) and the rung→reward table.
Those stay exactly as they are.

---

## §2 WHAT THE SWEEP ACTUALLY COSTS — the arithmetic

Steady state is level 5+ (`level = (1 + solved/3).min(5)`):
`Difficulty { tier: 3, word_len: 3, rows: 4, cols: 4, train_pairs: 3 }`.

Vocabulary (`search.rs:125-135`):

| source | count |
|---|---|
| symmetries | 6 |
| `OBJ_OPS` | 8 |
| `derived_recolors` | 0–7 |
| `derived_tiles` | 0 or 16 (`TileMirror::ALL`² ) |
| **V typical / worst** | **~21 / 37** |

Frontier, exhaustive to depth 3 at V = 37:

```
1 + 37 + 1369 + 50 653  =  52 060 candidates
```

**The budget never bites at live difficulty.** `SEARCH_BUDGET` is
200 000 (`search.rs:119`) and 52 060 < 200 000, so the sweep runs to
exhaustion and the space is fully covered. It bites only at
`word_len` 4 — reachable through `from_key` (`forge.rs:61`), which is
**not wired** (a read called from tests). That single fact reorders the
whole plan; see R4.

**Where the work goes.** `closes()` (`search.rs:105`) re-runs the whole
program **from the raw input** for every candidate, so a depth-`d`
candidate costs `d` op-applications per training pair:

```
current:  3 pairs × (37·1 + 1369·2 + 50 653·3)  =  464 202 op-applications
shared:   3 pairs × (1 + 37 + 1369 + 50 653)    =  156 180 op-applications
                                          ratio  =  2.97×
```

Every one of a node's V children re-derives that node's entire output
before applying its one new op. **That is the shear, unapplied.**

---

## §3 THE CHANGES

> ## Where the cost is — measured 2026-08-02, and it is not where the plan said

`engine/solver/examples/bench_solve.rs`, 120 forged tasks across rungs
3–5, three samples:

| | count | cost | share |
|---|---|---|---|
| solved | 85 (70 %) | 2.67 ms | 0.1 % |
| unsolved | 35 (30 %) | 2.825 s | **99 %** |

Per case that is 31 µs against 81 ms — **a factor of ~2 600.** The whole
forge cost is the exhaustive sweep over tasks that have no answer in the
searched vocabulary. A solved task is nearly free.

This single reading settles most of §3:

- **R4 is struck.** Ordering can only shorten the walk to an answer, and
  answers are 0.1 % of the time. Measured directly anyway: reordering the
  vocabulary observation-first left the solve rate at exactly 85/120 and
  the time marginally worse (min 2.695 s against 2.614 s). The premise
  was sound and the lever is attached to nothing.
- **R2 is struck for the same reason** — it prunes duplicate candidates
  inside a sweep whose cost is dominated by sweeps that must complete.
- **R5 becomes the interesting one**, because an unsolvable task re-drawn
  from an unchanged digest costs 81 ms to reach the identical `Unsolved`.

### The finding underneath: the forge can outrun its own solver

30 % of forged tasks are unsolvable *by the solver that must solve them*.
`forge_task` samples a secret word from the tier's families, but the
solver can only reach a `Tile` or `Recolor` operator if `derived_tiles` /
`derived_recolors` reconstruct its exact parameters from the observation
— and `derived_tiles` reads the ratio off the FIRST pair only, so a word
that disturbs the shape (a tile composed with a rotation, say) leaves the
operator unreachable and the task closed to the search.

Two costs, and the second is the one that matters:

1. **Compute.** The unreachable cases are 99 % of forge time.
2. **Economy.** A line drawing an unsolvable task burns the full sweep
   and mints nothing. At 30 % that is roughly one beat in three spent
   producing no work, and the line cannot tell the difference between
   "I was not good enough" and "there was nothing there".

Owed, and it is an operator question because it moves the difficulty
surface: should the forge emit only tasks its own vocabulary can close,
should the derived-operator coverage be widened to match the generative
families, or is an unsolvable draw a lawful part of the ladder?

## R1b — BUFFER REUSE: LANDED (2026-08-02)
>
> Operator: *"your heap on the Vec is too small."* Correct, and it is
> what R1 was missing.
>
> A grid at live difficulty is 9–16 bytes. `Op::apply` allocated a fresh
> `Vec<u8>` per call, so the payload was smaller than the allocator
> bookkeeping, and the sweep does this ~50 000 times per solve.
>
> `Op::apply_into(&self, g, out)` now holds the body and `apply` is a
> wrapper over it — one implementation, two entry points. `search::solve`
> carries two buffers for the whole sweep instead of allocating per
> candidate.
>
> Measured directly on `solve` (the beat loop swings ±4 % on identical
> code and cannot resolve this), 120 solves, five samples:
>
> | | min | mean | spread |
> |---|---|---|---|
> | before | 2.700 s | ~3.70 s | 2.70 – 4.31 s |
> | after | 2.736 s | **~2.78 s** | **2.74 – 2.85 s** |
>
> **The floor did not move; the mean fell ~25 % and the spread collapsed
> from 1.61 s to 0.12 s.** Allocation churn was not costing throughput at
> best case — it was causing allocator-pressure variance, and the mean
> carried the cost of the bad samples. For a loop that runs continuously,
> that is the number that matters.
>
> This also corrects the reading below: the first R1 attempt reused only
> one of the two buffers it needed, so it measured as a wash. The
> hypothesis was right and the implementation was half-done.
>
> ## ⛔ R1 (prefix images) + R3 (streamed level) — **STRUCK** (2026-08-02)
>
> **The predicted ≈3× did not appear. Measured A/B on one machine state,
> 40 cycles from one baseline universe:**
>
> | build | wall |
> |---|---|
> | committed (no R1/R3) | **56.45 s · 55.86 s** |
> | R1+R3, images collected | 56.96 s |
> | R1+R3, short-circuited | 56.43 s |
>
> **A wash.** The op-count arithmetic in §2 is correct — 464 202 → 156 180
> applications — and it buys nothing, because **op application is not the
> bottleneck.** A `Grid` is a heap `Vec<u8>`; carrying images trades
> cheap re-application on small grids for persistent allocation, and the
> two cancel.
>
> **AND MY OWN 48.24 s FIGURE DOES NOT REPRODUCE.** It was measured
> BEFORE the 66-commit merge; the same baseline on the same machine now
> reads ~56.2 s for the *unchanged* code. So the earlier "R1 is a
> regression" reading was an artefact of comparing across a merge —
> corrected here rather than left standing. Separately worth noting: the
> merge itself cost ≈ +17 % per beat.
>
> **Reverted.** `search.rs` stands as committed. Shipping complexity for
> no measured gain is the failure this board exists to catch, and the
> plan predicting 3× is exactly the kind of claim that has to be measured
> before it is believed.
>
> **KEPT: `solver/tests/search_identity.rs`** — 30 recorded answers
> spanning every rung, plus determinism and a closes-its-task check. It
> was written to guard the rewrite and it outlives it: any future change
> to `solve` now has a pin that catches a substituted program, which is
> what a property test cannot see.
>
> **What this does NOT settle:** where the 94 % actually goes. The
> op-count model was wrong, so the next move is to PROFILE `solve`
> (vocabulary construction, `OBJ_OPS` segmentation, allocator) rather
> than to reason about it again. Recorded as the reopened question.

### R1 — CARRY THE PREFIX OUTPUTS (the shear) · **land first**

**What.** The BFS already holds every prefix in `frontier`. Store with
each frontier entry the grid it produces **for each training pair**
(and for `test_in`). Testing an extension becomes: apply one op to the
cached parent grids, compare to the expected outputs. `closes()` stops
re-running from the input.

**Expected effect.** 2.97× fewer op-applications on the dominant layer
(§2). On a 94 %-of-beat phase that is roughly **1.52 s → 0.6 s per
beat** at 33 worlds, if op-application is the bulk of candidate cost.
Measure, do not assume — `OBJ_OPS` are segmentation-based and cost far
more per application than a symmetry, so the realised factor will
differ from the count ratio.

**Semantics.** Identical. Same enumeration order, same first match,
same program, same certificate.

**Risk.** Memory. A `Tile` op multiplies grid area by up to 9×, so a
depth-2 cache can hold 36×36 grids: 1369 nodes × 3 pairs × ~1.3 kB
≈ 5 MB per world. Mitigated entirely by R3 — stream the parent nodes
one at a time and the peak cache is one node.

**Witness.** `solver` gains a red-first pin: for a fixed digest and
difficulty, the program returned by the new `solve` is **byte-identical**
to the old one. That pin is the whole safety argument for R1 and R3, and
it must be written before either lands.

---

### R3 — STREAM THE FRONTIER, NEVER MATERIALIZE IT (peel) · **land with R1**

**What.** `search.rs:161-173` builds `next: Vec<Program>` holding up to
`room` fully-cloned programs — ~50 k heap `Vec<Op>` per world per beat,
~1.65 M per beat across a 33-world board. Replace with a generator:
iterate the depth-(d−1) frontier one node at a time; for each, walk the
vocabulary and test in place; only nodes that must survive to the next
depth are retained.

Combined with R1 the natural shape is one loop:

```
for each parent node (program, cached grids):
    for each op in vocabulary:
        test = apply(op, cached grids) vs expected      # R1
        if closes -> return                             # same order
    # nothing materialized; the parent's cache dies here
```

**Expected effect.** Removes the allocation storm; peak memory becomes
one node's cache. Allocation is not the dominant term — expect
**10–20 %**, not a multiple. Its real value is making R1 affordable.

**Semantics.** Identical, same order, same first match.

**Risk.** The budget accounting (`tested`, `room`, `SEARCH_BUDGET`)
must count **exactly** as it does today. `SEARCH_BUDGET` is an ENGINE
LAW (`search.rs:111-119`) and a deterministic bound is what makes the
forge replayable. The rewrite must not drift the count by one.

**Witness.** Extend R1's identity pin to assert the **candidate count**
is unchanged for a fixed digest, not only the returned program.

---

### R4 — ORDER THE VOCABULARY BY WHAT THE OBSERVATION SAID · **the largest win**

**What.** `vocabulary` is built in this order (`search.rs:125-135`):

```
symmetries (6) · OBJ_OPS (8) · derived_recolors (≤7) · derived_tiles (0|16)
```

The **derived** operators are read off the training pairs themselves —
`derived_recolors` (`search.rs:33`) finds the exact hue map under each
of seven geometric normalizations, and `derived_tiles` (`:79`) reads
the exact tiling ratio from the first pair. Meanwhile `forge_task`
samples the secret word from the very same families
(`forge.rs:76-124`). **The ops most likely to be in the answer are
enumerated last.**

The search returns at the **first** match, so enumeration order sets
the *expected* number of candidates tested. Put the derived ops first.

**Expected effect.** Large and expected-case, not worst-case. At V=37
the answer currently sits at an arbitrary index in a 50 653-wide
lexicographic layer; ordering the observation-derived ops into the low
indices should collapse the expected search to a small fraction of the
layer. **This is a prediction, not a measurement — R4's whole
deliverable is the measured before/after.**

**It is also a latent defect fix.** The growth loop breaks at `room`
(`search.rs:165`), so when the budget *does* bite the derived ops are
the first never enumerated — the search would starve exactly the
operators that can close the task. It does not bite today (§2), which
is precisely why this is the cheap moment to fix it: **today the change
is provably outcome-neutral, because the space is covered either way.**
Once `from_key`'s `word_len` 4 is wired, it stops being neutral.

**Semantics.** The searched *set* is unchanged; only the order. Where
several programs close a task, a different (equally valid) one may be
returned — see §1.2.

**Risk.** This is the one change that alters the returned program, so
R1/R3's byte-identity pin cannot cover it. Land R1+R3 **first**, prove
identity, **then** R4 as a deliberate order change with its own pin
(same task still solved; candidates-tested recorded).

---

### R5 — MEMOIZE `(digest, difficulty) → outcome` · **measure before building**

**What.** `solve` is pure and deterministic. `research_step` seeds it
from `receipts().last().own`, so a world that appended nothing since
the last beat re-forges an identical task and re-runs an identical
sweep for an identical answer.

**Honest expectation: the hit rate is probably low.** Most worlds
append receipts every beat (tick, collect, trade), so the digest
usually moves. The clear wins are penned worlds and quiet worlds.

**Do this first:** instrument the digest-repeat rate for one 80-cycle
run. If repeats are rare, **strike R5** rather than carry a cache
nobody hits. A cache that never hits is pure complexity, and the
lane's own law is that entries leave the board by landing *or by being
struck*.

---

### R2 — CANONICAL FORM ON GEOMETRY RUNS · **deprioritized, with the arithmetic**

`REMEDIATIONS.md` R2 proposed collapsing the six symmetry generators to
the ≤8 dihedral elements they generate. Having done the counting, the
payoff at live depth is small:

length-3 words over V=37 containing an adjacent geometry pair —

```
positions (1,2) geometry:  6·6·37 = 1332  → 8·37 = 296
positions (2,3) geometry:  37·6·6 = 1332  → 37·8 = 296
all three geometry:            216        →     8   (double-counted above)
saved ≈ 2 000 of 50 653  ≈  4 %
```

**~4 % at word_len 3.** It grows with depth (runs get longer) and would
matter at `word_len` 4+, but it is not worth spending a correctness
budget on now. **Held, not struck** — re-open it with the `from_key`
wiring at N6.

---

## §4 SEQUENCING

| # | change | gate | outcome-neutral? | expected |
|---|---|---|---|---|
| 1 | **R1 + R3** together, behind one byte-identity pin | none | **yes, provably** | ~3× on 94 % of the beat |
| 2 | **R4** vocabulary order | after 1's pin is green | yes *today* (space fully covered) | large, expected-case; measure |
| 3 | **R5** measure repeat rate | after 2 | yes | build only if the rate justifies it |
| 4 | **R2** canonical form | N6 / `from_key` wiring | yes | ~4 % now, more at depth 4+ |

**Nothing here is gated on Ruling Box 1.** §1 is the argument. Ruling
Box 1 still governs the `level → Difficulty` ladder and the rung→reward
table, and this plan touches neither.

**Definition of done for step 1:** workspace green, clippy zero, the
byte-identity pin red-first then green, and a re-run of the §1
measurement in `REMEDIATIONS.md` recording the new per-beat figure
beside the old.

---

## §5 WHAT IS NOT PROPOSED

- **Touching `SEARCH_BUDGET`.** A candidate count, not a clock, is what
  makes the forge replayable across machines (`search.rs:111-119`, an
  ENGINE LAW and explicitly never a test knob). It stays.
- **Skipping depths below `word_len`.** The forge always uses exactly
  `word_len` ops, so depths 0–2 look wasteful — they are 1 407 of
  52 060 candidates (**2.7 %**), and searching them is what lets a
  shorter program legitimately close a task. Not worth the generality.
- **Heuristics, sampling, beam search, floats.** The sector door
  (`search.rs:203`) permits them — *"the search inside may become as
  dirty as it likes"* — but every change here is exact and
  deterministic, and there is no reason to spend that licence on a
  phase that has a clean 3× available.
- **The volumetric path.** `solve_volume` is the decorative tier
  (operator C8); it cannot mint and is out of scope.

---

## §6 CORRECTION TO `REMEDIATIONS.md` §8

That section sequenced R1–R3 as **blocked on Ruling Box 1**, reasoning
that the forge is the proof-of-work so making it cheaper re-prices a
mint. Having since read `verify_work` (`work.rs:266`), `mint_bound`
(`kernel/src/lib.rs:845`) and `research_step`
(`conductor/src/lib.rs:1072`) whole, that block does not apply:

- the search is never re-run by any verifier;
- the reward is a table on `Difficulty`, not on cost;
- the search runs once per world per beat, so speed cannot change how
  often a mint occurs.

**R1, R3, R4 and R5 are unblocked.** Two further corrections from the
same reading: **R4 is inert today** rather than actively starving the
search (the budget does not bite at `word_len` 3), which makes now the
safe moment to fix it; and **R2 is ~4 %**, not the large structural win
its entry implied. `REMEDIATIONS.md` §2 and §8 are amended to point
here.
