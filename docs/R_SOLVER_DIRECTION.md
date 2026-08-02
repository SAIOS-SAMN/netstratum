# R-D5 — the solver is a brute-force enumerator, and that is the defect

**Lane:** `remediations-lane`. **Written:** 2026-08-03.
**Status:** research directive. Supersedes the R1–R5 optimisation track.

> Operator, 2026-08-02: *"you burned all of your resources churning on
> something that is unsolvable by your own brute force reasoning where
> you should have researched new technologies to unlock the higher order
> of intelligence."*

Upheld. I measured that 99 % of forge time is exhaustive sweeps over
tasks the solver cannot solve, and I responded by caching those sweeps.
The measurement was the answer and I treated it as a starting point for
more of the same. This file is the direction I should have taken from it.

---

## §1 What the solver actually is

`solve` enumerates the cartesian product of a 14–37 operator vocabulary
to depth ≤ 3 and tests every word forward against the training pairs.
That is generate-and-test. It has no model of the task, learns nothing
across candidates, and cannot use the structure of a near-miss.

Three consequences, all measured:

- 30 % of forged tasks are unreachable — no word in the product closes
  them, so the sweep runs to exhaustion and returns `Unsolved`.
- Those cases are 99 % of all forge time (81 ms against 31 µs).
- Every optimisation in the R1–R5 track (buffer reuse, prefix carrying,
  frontier streaming, vocabulary ordering, memoisation) changes the
  constant in front of `V^d`. None of them changes `V^d`.

**The system is named for cognition and the cognition is a product
sweep.** This is precisely the shape ARC exists to defeat.

## §2 The codebase already contains the better method, applied to 2 of 9

`derived_recolors` (`search.rs:33`) does not guess a recolor. It reads
the hue map **off the observation** — for each of seven geometric
normalisations, it checks whether the pairs are cell-wise consistent and
extracts the exact mapping. `derived_tiles` (`:79`) reads the tiling
ratio off the shapes.

That is inversion: *solve for the operator given input and output*,
rather than propose an operator and check. It costs `O(pairs · cells)`
where enumeration costs a factor of `V`.

**It is applied to two operator families out of nine.** The other seven —
six symmetries and eight object operators — are guessed. And the 30 %
unreachable cases are exactly where inversion failed and guessing could
not reach: `derived_tiles` reads the ratio from the **first pair only**,
so a word that disturbs the shape before tiling leaves the operator
outside the vocabulary entirely.

The remediation is not a faster sweep. It is to extend the method the
codebase already proved, and to stop enumerating what can be derived.

## §3 Four directions, cheapest first

### 3a. Deduplicate the frontier by IMAGE, not by program

Two words that produce the same grids are interchangeable for everything
downstream. The search enumerates **program space**; what matters is
**state space**, and state space is dramatically smaller:

- the six symmetry generators produce at most 8 distinct transforms
  (the dihedral group), so 216 length-3 geometry words collapse to ≤ 8;
- composed recolors are a single recolor;
- many object operators are idempotent (`CropContent` twice is
  `CropContent`).

Keep a set of image-tuples already reached; skip any child whose images
are already present. This is the peel/shear idea applied correctly —
collapse by what a thing **is**, not by how it was spelled. It needs no
new theory, preserves exhaustiveness over reachable states, and shrinks
the effective branching factor rather than the constant.

**Note this subsumes R2 and does it properly**: R2 proposed canonicalising
geometry words syntactically and measured ~4 %. Deduplicating by image
catches the same collapse plus every other one, structurally.

### 3b. Search backward from the output, and meet in the middle

Most of the vocabulary is invertible: all six symmetries are their own
group, a bijective recolor inverts, `Rotate90⁻¹ = Rotate270`. Run the
sweep forward from the input to depth `d/2` and backward from the output
to depth `d/2`, and join on matching images.

`V^d` becomes `2·V^(d/2)`. At `V = 37, d = 3` that is ~52 000 candidates
against ~450. **This is the exponent, not the constant** — the thing
every R1–R5 change failed to touch.

The non-invertible arm (`Tile` expands, `CropLargest` discards) simply
does not participate in the backward pass; it stays forward-only. A
partial meet-in-the-middle is still most of the win.

### 3c. Derive every operator family that can be derived

Extend `derived_*` to the whole vocabulary:

- **symmetries** — the shape change alone (`rows×cols` → `cols×rows`)
  identifies transpose-class operators; a cell-wise check confirms which.
- **tiles** — read the ratio from **every** pair, not the first, and
  admit the case where a shape-changing op precedes the tiling.
- **object operators** — segment both sides and compare component
  counts; `KeepLargest` and `DeleteLargest` are identifiable from the
  segmentation, not from trying them.

Every family moved from guessed to derived removes a factor of `V`.

### 3d. Intersect consistent-program sets across pairs (version spaces)

The standard technique for programming-by-example: for each training
pair compute a compact representation of **all** programs consistent with
it, then intersect across pairs. The intersection is usually tiny even
when each individual set is astronomically large.

This is the principled form of what 3a and 3c approximate, and it is
where the design should end up. It is also the largest change, so it
belongs after them.

## §4 What must not move

- **`verify_work` and the certificate.** The judge re-applies the
  presented program; it never enumerates. Every direction here is
  prover-side and consensus-free — `FORGE_BUDGET` §1 established that
  and it still holds.
- **Determinism.** Same task, same answer, on every machine. All four
  directions are deterministic; none needs sampling or a clock.
- **The honest withhold.** `Unsolved` stays a lawful reading. A smarter
  search must never guess.

## §5 Sequencing

1. **3a (image dedup)** — no new theory, subsumes R2, measurable
   immediately with `bench_solve`. Note it changes which program is
   returned, so `search_identity.rs` is re-recorded deliberately.
2. **3b (bidirectional)** — the exponent. Needs an `Op::invert` returning
   `Option<Op>`, which is a small addition to the sealed vocabulary.
3. **3c (derive the rest)** — removes a factor of `V` per family and
   directly attacks the 30 % unreachable.
4. **3d (version spaces)** — the principled endpoint.

Measure each with `bench_solve` (min and spread over samples), reporting
**solve rate first and time second**. The R1–R5 track optimised time
while the solve rate never moved off 85/120; a change that lifts the
solve rate and costs time is a better change than the reverse, because
an unsolvable draw mints nothing.

## §6 The lesson this file records

I had the measurement that mattered by mid-session — 99 % of the cost is
tasks that cannot be solved — and spent the rest of the budget making
those failures cheaper. Making a brute-force search faster is not the
same problem as not needing one. The measurement said the search was
inadequate; I read it as saying the search was slow.
