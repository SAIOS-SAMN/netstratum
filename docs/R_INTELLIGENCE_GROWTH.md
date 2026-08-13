# R-D7 — what unlocks growth in intelligence: V grows

**Lane:** `remediations-lane`. **Written:** 2026-08-03.
**Status:** research directive. Unifies R-D4, R-D5/6 and the salvaged
nursery design against measurements this lane already took.
**2026-08-12:** the mint's gate design is now charter law —
`38_COGNITION_CONVERGENCE.md` §4 (NOVELTY-GATE, task #13): the four
structural gates (unique orbit · measured productivity · verifier
conservation · convergent derivation), consuming `rfa-core`'s
`rcf_hash` and `coherence_functional` as their first production
callers. §5–§6 there land the compounding and hunger halves.

---

## §1 The measurement that names the axis

`bench_solve`, 120 forged tasks: **85 solved, 35 not.** Every change this
lane made moved time and none moved that number — R1, R2, R3, R4, buffer
reuse, the digest memo, the reverted image dedup.

The reason, read from `search.rs` rather than guessed: `derived_recolors`
and `derived_tiles` derive an operator only when it is the LAST op of the
word with at most one geometric op before it. Anything else leaves that
operator **outside the vocabulary**, and no search depth reaches an
operator that is not there.

**The binding constraint is `V`, not `d`.** Reach is what is missing, and
reach is what "intelligence" names here.

## §2 The envelope already grows with V — that half is landed

```rust
pub fn frontier(vocabulary: usize, depth: usize) -> u128   // Σ_{k=0}^{d} V^k
```

`SEARCH_BUDGET = 200_000` is gone; the bound is derived from `V` and the
rung. So the machinery already expands when `V` expands. **Nothing grows
`V`.** The vocabulary is rebuilt identically on every call, from six
hardcoded symmetries, eight hardcoded object operators, and two
narrow derivations.

That is the same defect as `Resource` being a closed enum (R-D4): a fixed
set where there should be one that grows with capability. **The closed
operator vocabulary and the closed resource vocabulary are one problem.**

## §3 The design already exists, salvaged and unbuilt

From the discarded `parallel-agent` worktree (`engine/solver/src/mint.rs`,
never committed): **a civilization grows its OWN solver vocabulary.**
Five laws, and they are already the right ones:

1. **CLOSED ALGEBRA** — a minted operator is a composition of existing
   total primitives (a macro word, or a geometric prefix with a per-task
   re-derived family). No new semantics, no new code, no I/O. **A mint
   changes REACHABILITY, never capability class.**
2. **MEASURED NOVELTY** — the gate.
3. **BOUNDED** — library, macro and history caps.
4. **CONTAINED** — per-civilization libraries, never global.
5. **RECEIPTED AND KILLABLE** — every mint carries a certificate naming
   the triggering task and the base-failure witness; replay re-derives;
   a `--no-mint` flag freezes the door.

This is **vocabulary minting**, a different axis from the landed **value
minting** (`DiscoveryMinted`, `mint_bound`, U1 PoUW). Value minting pays
a line for work done. Vocabulary minting makes the line *able to do work
it could not do before*.

## §4 The reframe this lane owes: the exhaustive sweep is the certificate

Law 2 is the gate: a candidate operator mints only if **the base
vocabulary provably fails the triggering task at equal search depth AND
the candidate closes it.** Novelty measured, never asserted.

**That proof is exactly what the 35 unsolved tasks already are.** The
sweep is exhaustive over `frontier(V, d)`, so `Unsolved` is not a
timeout — it is a proof that no word in the current vocabulary closes the
task at that depth. A heuristic or sampled search could not produce that;
only an exhaustive one can.

I recorded those 35 cases as waste: *"99 % of forge time, a line burns
the sweep and mints nothing."* That reading was wrong. **They are the
trigger set.** The cost I spent the session trying to remove is the cost
of earning a novelty certificate, and the thing that makes the certificate
sound is the exhaustiveness I kept trying to prune.

This also settles a question §5 of `FORGE_REMEDIATION` left open — whether
an unsolvable draw is a lawful rung. It is: an unsolvable draw is where
intelligence grows.

## §5 Where the nursery already is in law

Vocabulary growth needs a container that cannot leak into consensus, and
the container is already ruled:

- `NS-5` §7.6 — the sandboxed nursery: a declared fork under its own
  branch coordinate. *"You can play in your nursery; you cannot rip a
  hole in the galaxy."* Penalty attaches only to presenting nursery state
  as mainline.
- `00_CHARTER` §11 — the nursery law: a world is born and raised on one
  node; a fork that forks openly is lawful, only fabricated confirmation
  is not.
- `48_CAPACITY_THREAT_MODEL` §11c — *"the nursery routes divergence
  instead of refusing it."* (moved from `17_CAPACITY` in the
  2026-08-13 docs split)

A minted operator lives in the minting civilization's own library on its
own branch. It never enters another line's vocabulary except by the
ordinary chain, and `verify_work` is untouched: the judge re-applies a
presented program, and a program composed of existing primitives replays
identically whether or not the verifier has heard of the macro.

## §6 What gates the mint — and this is where capacity enters

Not every line should grow. The nursery's law 3 (bounded) and law 4
(contained) need a rule for **who may hold how much vocabulary**, and
that is the budget work already open:

- `EQUATIONS` §7 rows **B1–B9** derive a search DEPTH from standing.
  The same reading gives a **library ceiling**: a line holds vocabulary
  in proportion to the structure it sustains.
- `kernel::bridge::open_axes_of` already names *"the DIRECTION A NEW
  RELATION WOULD EARN ON"* — the syntropic half, a reading that gives
  rather than withholds.
- `motion::envelope(n, rank) = n^rank` is already the kernel's statement
  that the budget is a function of standing rank rather than a constant.

So the ladder is one ladder: **structure earns depth, and structure earns
vocabulary.** A line at the ground gets the floor of both. A line
carrying real relational structure searches deeper AND may hold more
operators — and `frontier(V, d)` grows in both arguments at once, which
is why it is the right envelope.

## §7 The sequence

1. **Widen the derivations** (task #29). Invert the trailing ops before
   deriving: for each short candidate suffix, apply its inverse to the
   output and run the existing derivation against that. This lifts reach
   with no new law, and it is the honest first move because a mint should
   never pay for what plain derivation would have found.
2. **Re-measure the trigger set.** After 1, the remaining unsolved cases
   are the tasks no derivation reaches — the real novelty triggers, and a
   much smaller and more meaningful set than today's 35.
3. **Build the nursery mint** against those, with all five laws, and the
   certificate carrying the triggering task plus the base-failure
   witness.
4. **Gate the library ceiling on capacity** — B1–B9's reading, applied to
   vocabulary size rather than to search depth.
5. **Then reopen R-D4** for `Resource`: the same growth law, applied to
   the element vocabulary rather than the operator one. If the mechanism
   is right for one it should be recognisably the same for the other.

## §8 What this closes

- **The R1–R5 optimisation track** and the **3a–3d search track** are
  both closed. Both optimised `d`; the constraint is `V`.
- **R-D6 (cohomological convergence)** is withdrawn. It replaced a
  working search to chase the same 30 %, and the encoder built for it
  measured 7 % agreement — recorded and removed.
- **Task #26** ("the forge outruns its own solver") is reframed rather
  than closed: the mismatch is real, and after §7.1 what remains of it is
  the trigger set rather than a defect.

The measurement rule stands and now has a reason behind it: **solve rate
first, time second.** Time is `d`. Solve rate is `V`. Only one of them is
intelligence.
