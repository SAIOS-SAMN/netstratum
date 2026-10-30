# R-D5/6 — the solver must read geometry, not enumerate programs

**Lane:** `remediations-lane`. **Written:** 2026-08-02, rewritten
2026-08-03 after consulting `/opt/repos/convergence-engine` and
`/opt/ipsal-autic-v0.1/saios-arc`.
**Status:** research directive. Closes the R1–R5 optimisation track and
the 3a–3d search track with it.
**2026-08-12:** the convergence-engine concept map is charter law at
`38_COGNITION_CONVERGENCE.md` — perspective compounding (§5) is the
ARC-AGI-2 device this file's geometry reading feeds into.

---

## §0 The incident this file opens with

An out-of-memory kill took down the operator's machine. The cause was
mine: `solve` was given a `HashSet<Vec<Grid>>` of every distinct image
tuple reached, cloned in, unbounded. Reverted at `HEAD`; the tree carries
none of it.

The failure is not incidental to the subject. **Raw images have no
floor**, so a set of them grows without bound. A cohomology class does
have a floor — §2. Deduplicating by hashed grid was flat indexing, which
is precisely what `DIMENSIONAL_RESOLUTION.md` names as the thing that
destroys the geometry. AI implementor identified: it had built brute force
presented as geometry, and it ran the machine out of memory.

## §1 The solver imports the relational engine and never calls it

```
engine/solver/src/search.rs:231   impl rfa_core::Exact for SolveRead {}
engine/solver/src/search.rs:298   rfa_core::consult((task, max_depth), ...)
```

That is the entire use of `rfa-core` in the solver: a marker trait and
the cage door. Not `Delta`. Not `coboundary_reduce`. Not `holonomies`,
not the residual field, not H¹.

The relational engine sits in the solver's own dependency graph, built
and witnessed, while the solver enumerates a cartesian product beside it.

## §2 What the two consulted repositories say

### 2a. `saios-arc` already retired this architecture

`saios-arc/src/lib.rs` lists, under **retired**:

- `solve_pair<N, M>` **Compositor::search dispatch**
- `derivation/*` — `ValueDerivation`, `PositionDerivation`,
  `GeometricDerivation`, `CardinalityDerivation`, `HomogeneityDerivation`
  — *"substrate cognition replaces adapter-side rule derivation"*

The retired list is this lane's plan. The search dispatch is netstratum's
`solve`; the derivation producers are §3c, which the AI implementor proposed the day
before as the fix. They built both, and moved past both.

**The live path:**

```
cells → RuntimeDelta → cohomology_class_from_runtime
      → traverse_for_convergence → BundleEliminator
```

Elimination against a cohomology class. Not enumeration over a product.
And the entry point is `think_with_walk_backwards` — backward reasoning,
which §3b proposed and which is already built there.

### 2b. `refinement.rs` is what "stacking axioms" means concretely

`OperatorHypothesis` carries `{ operator_handle, claimed_post_fingerprint }`
— **a cohomology fingerprint, not a program.** The loop refines one
hypothesis across the training pairs, and crystallisation is gated on a
stability witness: *"PathIndependent attraction across at least 2 pairs.
Mid-process crystallisation refused; only stability-witnessed convergence
crystallises."*

Each pair constrains the hypothesis. Agreement across pairs is what
promotes it. That is learning from the observation, and it is the
opposite of testing 52 060 candidates against it.

### 2c. `DIMENSIONAL_RESOLUTION.md` gives the governing law

> Controlled expansion: propose operators that **increase**
> dimensionality when the residual says the current dimensions are not
> enough. Controlled collapse: `coboundary_reduce` **projects back** to
> the irreducible core after every expansion.
>
> Expansion bounded by validation gates. Collapse bounded by
> cohomological projection — **you cannot fold past the irreducible form;
> the H¹ class is the floor.**
>
> The cycle: unfold → fold → measure the residual → unfold again if
> needed.

Both bounds matter, and the lower one is why §0 happened. A search over
raw images is unbounded in both directions. A search over cohomology
classes cannot fall below H¹, and the residual is what tells it when to
expand.

## §3 Netstratum already holds both halves

| the law needs | Netstratum has | where |
|---|---|---|
| the relational field `Δ(V)` | `rfa_core::Delta` | `rfa-core` |
| controlled collapse | `coboundary_reduce` | `rfa-core/src/reduce.rs` |
| the irreducible core | `holonomies` — H¹ representatives | `rfa-core/src/reduce.rs` |
| the residual that says "expand" | `symmetry_foundation` — *"the ground new structure rises from"* | `kernel/src/bridge.rs` |
| both poles of the reading | `dual_pole`, `Flux { sources, sinks, balanced }` | `rfa-core` |
| a projection from state to `Δ` | `bridge::project_complex` | `kernel` |

The one thing missing is a projection **from a grid pair to a `Δ`** —
`saios-arc`'s `pair_encoding` / `cells_to_runtime_delta`. Everything the
convergence cycle needs downstream of that is built, witnessed, and in
the solver's dependency graph already.

## §4 The direction

**Do not optimise the search. Replace what it searches over.**

1. **Encode the pair as a relation, not a picture.** A grid pair becomes
   a `Δ` over cell/region entities — `saios-arc::pair_encoding` is the
   reference. Antisymmetry is the curvature; this is where the geometry
   enters and where flat hashing loses it.
2. **Take the cohomology class.** `coboundary_reduce` strips coordinate
   distortion; what survives is what no frame explains — the invariant
   the transformation actually is.
3. **Match classes, not images.** Two pairs of the same task should
   present the same class. That is the constraint, and it is bounded
   below by H¹ rather than by a growing set.
4. **Refine across pairs, crystallise on stability.** One hypothesis,
   constrained pair by pair, promoted only on a path-independence
   witness over ≥ 2 pairs.
5. **Expand only on the residual.** When the class does not resolve, the
   residual field names which triple failed to compose — that is the
   signal to unfold a dimension, and it is already computed.

## §5 What holds unchanged

- `verify_work` re-applies a presented program and never enumerates, so
  all of this stays prover-side and consensus-free.
- Determinism: exact rationals, no float, no clock, no sampling.
- The honest withhold: `Unsolved` stays lawful. A solver that reasons
  must still never guess.
- **Bounded memory is now a stated requirement, not an assumption.** Any
  structure that accumulates per candidate must have a named ceiling
  before it is built.

## §6 What was closed to get here

- **R1, R2, R3, R4** — struck. Each changed the constant in front of
  `V^d`; the solve rate never moved off 85/120.
- **R5** — landed (a 31 % digest-repeat memo), and it caches failures
  rather than removing them.
- **3a** — built, measured, reverted. It collapsed 56 % of the frontier
  and the solve rate still did not move, because deduplication shrinks
  the search without widening its reach. Then it ran the machine out of
  memory, for the reason in §0.

The measurement rule that survives all of it: **solve rate first, time
second.** Every change in this lane's history improved or preserved time
and none of them moved 85/120.
