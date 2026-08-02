# FORGE REMEDIATION PLAN — the 94 %

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
