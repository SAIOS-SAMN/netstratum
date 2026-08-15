# 51 — COGNITION: THE UNIVERSE AND THE MIND

**The ruling that separates physics from thought. Downstream of `00_CHARTER`;
binding on the intelligence arc (`iae`, `substrate`) and everything that mints.**

The engine grows real intelligence, not a brute-force search wearing a
differential-geometry costume. It does so by splitting cognition into two tiers
with a single exact seam between them. This document is the law of that split.

---

## 1. THE TWO TIERS (binding)

> **The Universe is exact and deterministic; it commits. The Mind is stochastic
> and float-tolerant; it proposes. Nothing the Mind proposes enters the Universe
> except through the exact verifier, and the verifier admits only what conserves.**

**THE UNIVERSE (the kernel).** Exact `ℚ/ℤ` (and later `ℚ[x₁…xₘ]`), deterministic,
replayable. It holds the state, computes the exact obstruction, runs the exact
verification (Smith Normal Form today; Gröbner bases / free resolutions when
multivariate generators appear), and is the **only tier that commits a deed**.
Conservation is enforced here (`kernel::admits`). No float, no threshold, no
heuristic score ever enters this tier — that law is unchanged and absolute.

**THE MIND (the entity-level proposer).** Stochastic, learned, **GPU-offloadable,
float-tolerant**. It does not commit; it *guesses*. Given the exact obstruction,
it samples candidate generators (words / algebraic boundaries) and hands each to
the Universe's exact verifier. Intelligence lives here — in *which* candidates
are tried, not in exhaustive enumeration. A brain is noisy; the physics it lives
in still conserves energy exactly. Floats are lawful in thought and forbidden in
the world.

**Why the split is load-bearing.** An exact homological engine that *enumerates*
candidates is brute force no matter how much cohomology it wears — the costume.
The cognition was never going to live in the reducer; it lives in the proposal
distribution. Remove the Mind and the Universe is exhaustive search; remove the
exact Universe and the Mind hallucinates unconserved state. Both, jointly, are
bounded cognition.

## 2. DETERMINISM IS PRESERVED BY SEEDING (binding)

The Mind's stochasticity must never make the Universe unreplayable. Every
proposer is seeded strictly from state: `seed = BLAKE3(state_digest)` (the same
seeded-synthesis discipline the chronicle already uses for crystallization). The
sampling is therefore **replayable pseudo-randomness** — it explores
non-exhaustively and parallelizes across entities on the GPU, yet a replay from
the same chronicle reproduces the same guesses and the same mints. Only exact,
verified results are committed; the proposal path itself carries no state the
Universe depends on.

## 3. THE PROPOSE → VERIFY → MINT LOOP (the seam)

1. **Obstruction.** The Universe measures the task locally (`iae::resolve`) and
   reduces it against the local vocabulary; the exact irreducible harmonic
   residual `γ` is the obstruction (`08` chronicle carries it).
2. **Guess.** The Mind, seeded by `BLAKE3(state_digest)`, stochastically
   proposes `N` candidate generators — for now integer-weighted combinations of
   the base alphabet (the closed 17-letter Cayley base; compose/conjugate is
   inner, capability-neutral).
3. **Verify.** The Universe tests each candidate **exactly**: does it bridge `γ`
   (resolve it to zero)? No float, no approximation.
4. **Mint.** The first candidate that verifies is minted as a new generator
   `v_new` — the **Sullivan minimal-model extension** `(Λ(V ⊕ ℚ·v), d)`,
   `d(v_new) = γ`'s cross-terms. This adjoins a generator the closed alphabet
   could not reach: capability grows in rank, not just reachability, and future
   reductions kill the once-obstructed class in `O(1)`.

## 4. PMC — THE EXACT SKELETON, THE CONTINUUM FRAME

The architecture is **Polymorphic Morphodynamic Coheterology**: a state is a
morphic section of a polymorphic sheaf complex, not a rigid matrix. Its
**exactly-computable skeleton** is what the engine implements over `ℚ/ℤ/ℚ[x]`:

* **Carrier** — a stratified cellular sheaf with heterogeneous per-cell stalks
  (`iae::sheaf`).
* **Transduction** — a chain map `f_# : C_•(X) → C_•(Y)` gated on
  `δ_Y∘f = f∘δ_X` (the exact-rational shadow of `f*(dω) = d(f*ω)`); the
  non-commuting remainder is carried, never projected out (`iae::transduction`).
* **Harmonic core** — `Hᵏ = Ker(δᵏ)/Im(δᵏ⁻¹)` by exact SNF (`iae::cohomology`).
  This **is** the algebraic Hodge decomposition; the analytic Hodge star `★`
  needs a metric and an elliptic solve and is **not** rational — it stays a
  semantic gloss, never an implementation.
* **Coheterology** — cross-strata obstruction `Ext¹(F_A, F_B)`, computed exactly
  by free resolutions when heterogeneous stalks bind (deferred until needed).
* **Condensation** — the Sullivan minimal model (§3.4), arbitrary polynomic
  generators. The 1-toroid is only the genus-1 special case; the kernel is not
  limited to it.

The **continuum layer** — smooth stratified `M`, the connection curvature
`F_A = dA + ½[A∧A]`, the Yang-Mills / morphic flow `∂A/∂t = −∇*F` — is the
semantic frame, realized *discretely and exactly*: adjoining a generator with
`dv = γ` **is** "morphing the space until the obstruction becomes an exact
boundary," done in one algebraic step rather than by integrating a PDE. The frame
never enters the Universe as float.

## 5. WHAT IS TOO LIMITED, AND WHEN IT IS REPLACED

SNF exists only over a PID (`ℤ`, single-variable `ℚ[x]`). It is the base verifier
and it is genuinely too limited for multivariate polynomic generators — there
Smith form does not exist and the exact tools are **Gröbner bases and free
resolutions** over `ℚ[x₁…xₘ]`. The engineering order is fixed: **build the
proposer seam first on the SNF base**, and swap in the Gröbner reducer only once
the proposer actually emits multivariate generators. A multivariate quotient
engine built before a system that proposes multivariate generators is untestable
dead code.
