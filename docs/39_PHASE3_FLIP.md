# 39 — The PHASE-3 flip (landed)

**Status: landed 2026-08-13.** The operator ruled §3 option A
(sub-coordinate index) and signed off §4. `mesh::arbor::Coordinate`
gained `sub: u32`, hashed into every commit like every other field —
several deeds sharing one world tick now address distinctly. §5's
sequence landed in full: the sub-coordinate wire amendment (with its
own round-trip coverage via `arbor.rs`'s existing open()/extend()
tests), per-deed commitment in `chain_beat` (each deed threads
`Some(&step)` into `extend`, gated live), and the demotion itself —
`conductor::chain_beat` only advances `arbor_committed` past a deed
the kernel actually admitted; a refused deed gets no arbor
commitment and blocks further commits that beat until it resolves.
`world/replay.rs`'s banner records the new role. See
`docs/archive/U_DIRECTIVES.md` §7's roadmap line for the cross-reference.

---

## 1. What the flip is

Today the kernel judges **beside** the chain: every deed is run through
`kernel::admits` every beat, refusals are recorded and read back through
`Polity::refusals` and announced once — but the deed **stands
regardless**, because `cfe::world::replay` is still the live transition.
The commitment the conductor writes carries `judged = false`
(`arbor::extend(.., None)`).

The flip demotes `replay` to a proposer and makes the kernel the live
gate: a deed the judge refuses **no longer stands**. This is the one
defect between "having a judge" and "having a kernel" (`46_KERNEL`
§3), and it is the PoUW precondition — a distributed economy cannot
anchor value on a transition each node defines for itself.

## 2. What is already done (no ruling needed, landed)

- **The judge runs live, per deed** — the stepper (task #9) makes
  prior/next a clone of state, O(state), not a prefix replay. The
  conductor's `chain_beat` judges every new receipt each beat.
- **The judge agrees with the law** — task #7 closed the endowment
  and founding-bootstrap disagreements; the live board re-judges to
  **zero refusals** on a 15-world sample.
- **Coverage is total, and the compiler holds it so** — task #17:
  `kernel::covered` is a wildcard-free match; the unjudged set is
  empty and a new deed breaks the build until classified.
- **The verdict is durable** — the judged watermark persists beside
  the exports; a boot re-derives only the recorded refusals.

So the preconditions the §0 audit named as blocking are all measured
met. What remains is purely the wire shape below.

## 3. The one ruling — the addressing law (operator)

The arbor addresses one record per coordinate, and the coordinate's
`t` is the world clock. **Several deeds share one tick**, so a
per-deed commitment refuses `Occupied` on the second deed of any beat
(measured). Judgment is per-deed; the commitment is per-beat; the
`judged` flag cannot honestly straddle them.

Three ways forward — the ruling picks one:

| Option | Wire cost | What `judged` then means |
|---|---|---|
| **A. Sub-coordinate index** | Add a `u32` sub-index to `Coordinate` (hashed into every commit — a breaking wire change, `SCHEMA` of the spine). | One commitment per deed, each carrying its own verdict. The true flip; the honest one. |
| **B. Beat-clean stamp** | `extend` gains a self-verifying multi-step form: it carries the beat's receipts, re-derives each intermediate state, stamps `judged = true` only if every deed admits. Additive wire (the beat's receipts ride the commit). | The beat as a whole was clean. Coarser, but no coordinate change. |
| **C. Tick-per-deed** | Advance the world clock per deed rather than per beat. No wire change, but it re-times the whole economy (demurrage, spans, year voices all read the clock). | One commitment per deed by construction — but the blast radius is the entire tick law. |

**Recommendation: A.** It is the shape the spine was built for (the
`judged` bool already exists per record); B stores redundant receipts
the chronicle already holds; C moves a load-bearing constant nothing
else asked to move. A is a bounded, reviewed wire amendment — exactly
the class the tag-claim convention governs.

## 4. The sign-off — what a refusal comes to mean (operator)

Independently of §3: the day `replay` is demoted, a refused deed stops
standing on the live board. That changes play — a proposer that
emitted an unlawful deed now has it rejected mid-beat rather than
recorded-and-overruled. The live `var/` re-judges clean today, so the
immediate blast radius is nil; but the semantic is permanent and is
yours to authorize.

## 5. The landing, once ruled (sequence)

1. **(ruling A)** the sub-coordinate wire amendment in `mesh::arbor`
   (`Coordinate`, `seal`, `present`'s tag-88 frames) + its round-trip
   pins; spine `SCHEMA` bump.
2. per-deed commitment in `chain_beat` — the stepper already produces
   the per-deed prior/next; each threads `Some(&step)` into `extend`.
3. **(sign-off)** demote `replay`: the conductor applies a deed only
   if `admits` passes; a refused deed does not enter the chronicle.
   `world/replay.rs`'s banner updated; `U_DIRECTIVES` §5 marked done.
4. escrow generalization (`8067f60`) unlocked — an escrowed emitter
   settles only on `admits`, now safe because coverage is total.

## 6. What this document does not change

Nothing lands from it without the §3 ruling and §4 sign-off. The
judge continues to run beside the chain until then, exactly as it
does today — visible, recorded, and overruled.
