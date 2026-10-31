# 51 — Cognition: the universe and the mind

**The rule that separates physics from thought. Downstream of `00_CHARTER`;
binding on the intelligence arc (`iae`, `substrate`) and everything that mints.**

The engine grows real intelligence, not a brute-force search wearing a
differential-geometry facade. It does so by splitting cognition into two tiers
with a single exact seam between them. This document is the law of that split.

---

## 1. The two tiers (binding)

> **Amended by `53_INTELLIGENCE_PORT`.** The Universe half of this section
> stands unchanged. The Mind half — "stochastic, float-tolerant,
> proposes/guesses candidate generators" — is retired: it described the
> search the opening paragraph rejects. The Mind does not sample; it
> observes. See §1a.

> **The Universe is exact and deterministic; it commits. The Mind is exact and
> observational; it reduces. Nothing the Mind reduces enters the Universe except
> through the exact verifier, and the verifier admits only what conserves.**

**The universe (the kernel).** Exact `ℚ/ℤ` (and later `ℚ[x₁…xₘ]`), deterministic,
replayable. It holds the state, computes the exact obstruction, runs the exact
verification (Smith Normal Form today; Gröbner bases / free resolutions when
multivariate generators appear), and is the **only tier that commits a deed**.
Conservation is enforced here (`kernel::admits`). No float, no threshold, no
heuristic score ever enters this tier — that law is unchanged and absolute.

### 1a. The mind is exact observation (binding, `53`)

**The mind (the entity-level observer).** Exact, learned, `grown_basis`-seeded —
**no float, no candidate loop.** It does not guess; it peels a pair (a grid at the
ARC seat, a book at the survival seat), canonicalizes-and-DIFFERENCES the pair
against the crystals it has already grown, and **the residual names the deed** —
the entity becomes the thing that recognized the leftover. Intelligence lives in
*which book or family is read first* (need orders; observation supplies), never in
*which candidate is sampled*. A mind whose biomass book is short builds or razes
from that residual, not from a score.

**Why the earlier split was wrong.** An exact engine that *enumerates* candidates
is brute force no matter how much cohomology it wears — but a *stochastic
proposer* that *samples* candidates is the same brute force wearing a noise
generator, and a *utility-weighted partition-draw* over candidates is the same
sweep-and-pick wearing a weight. The cognition was never going to live in a
proposal distribution; it lives in the reduction — peel → difference → drain
against grown crystals → decline through a deduced frame → compose only on the
remainder. Remove the observation and there is nothing to reduce; remove the exact
Universe and the render hallucinates unconserved state. Both, jointly — one
reduction organ at two seats — are bounded cognition. Floats are forbidden in
*both* tiers now; the thought is as exact as the world it commits to. `53` is the
carrier; this section is its seam.

## 2. Determinism is preserved by seeding (binding)

The Mind's stochasticity must never make the Universe unreplayable. Every
proposer is seeded strictly from state: `seed = BLAKE3(state_digest)` (the same
seeded-synthesis discipline the chronicle already uses for crystallization). The
sampling is therefore **replayable pseudo-randomness** — it explores
non-exhaustively and parallelizes across entities on the GPU, yet a replay from
the same chronicle reproduces the same guesses and the same mints. Only exact,
verified results are committed; the proposal path itself carries no state the
Universe depends on.

## 3. The propose → verify → mint loop (the seam)

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
   reductions eliminate the once-obstructed class in `O(1)`.

## 4. PMC — the exact skeleton, the continuum frame

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

## 5. What is too limited, and when it is replaced

SNF exists only over a PID (`ℤ`, single-variable `ℚ[x]`). It is the base verifier
and it is genuinely too limited for multivariate polynomic generators — there
Smith form does not exist and the exact tools are **Gröbner bases and free
resolutions** over `ℚ[x₁…xₘ]`. The engineering order is fixed: **build the
proposer seam first on the SNF base**, and swap in the Gröbner reducer only once
the proposer actually emits multivariate generators. A multivariate quotient
engine built before a system that proposes multivariate generators is untestable
dead code.

## 6. Bind, don't fold (binding, not subtractive collapse)

> **A manifold does not disappear when it solves a problem; it binds to it. The
> engine retains the binding — the overlap and its torsion twist — never
> collapsing a solved obstruction to a bare zero.**

Reducing an obstruction to a residual is a `ℤ`-module **decomposition**
question (is the cochain in the span of what the Mind knows), and that is
intrinsically what it is — no cup product decides membership. But returning
*only* the residual throws the solution's geometry away, and that discard is a
disguised collapse. So `overlay` (`iae::resolve`) returns the whole
[`Binding`]: the **coefficients** (the integer overlap — which generators, in
what combination, wrapped the obstruction), the **torsion** (the `ℤ/dᵢ` twist —
the flex that maps a manifold onto an imperfect match, which a `ℚ` vector space
could not carry), and the **residual** (only the genuinely unbound part). The
exact reconstruction `generators · coefficients + residual = obstruction` always
holds: nothing collapses.

The cup product `⌣` and the tensor/`Tor` interaction are the genuine
**composition** operations — how two learned patches overlay into a
higher-degree class — and they are added as such, never as a fake resolver.
Learning adjoins a manifold patch; solving twists and overlaps the internal
geometry until it wraps the external obstruction, and the wrapping is kept.
