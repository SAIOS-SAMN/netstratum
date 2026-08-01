# 17 — THE CAPACITY LAW: Δ(Ω) = D

**Status:** SUBORDINATE to `EQUATIONS.md` §1 (D0–D4, THE FOUNDATION
RULING, `4232434`). That ledger carries the FORM; this document carries
the operator's ruling verbatim, the reasoning behind two of its clauses,
the record of what an earlier draft got wrong, and the build
consequences. **Where this document and `EQUATIONS.md` §1 disagree, the
ledger governs** (§0.1: a disagreement with an authority is a DEFECT).

Subordinate to `00_CHARTER.md`.

---

## §1 THE RULING (operator, 2026-08-01 — recorded as given)

The formation:

> kick C out entirely as the overall formation and start building
> around Δ(Ω) = D where D is dimensional capacity overlaid with the
> maximum potential of ∇V — the equations stay the same, but instead of
> forming around C we are looking at the D capacity and building
> towards its maximum potential in union with V as a whole — no two
> parts cut out, and the "descent" is only a uni-directional axial on
> the maximum dimensions of the polynomial union.

The clarification, same day:

> the shape of Ω is the totality of the individual "universe" — and
> coboundary reduction is the scoping of a dimension (D) instead the
> "V" nabla

THE FOUNDATION RULING (`EQUATIONS.md` §1) then took both further:
gradient descent is a FAILED TOPOLOGY, struck whole rather than
confined; E1, the master equation, struck with it; D is a SHAPE
`{i, j, k, l}`, never a scalar.

Everything below §1 is TRANSLATION and REASONING, never the ruling.

## §2 WHAT THE FIRST DRAFT GOT WRONG — recorded, never silently dropped

This document's first draft (`f0a3d43`, `b9988a1`) was written before the
foundation landed and read four things wrongly. Each is recorded here
rather than quietly deleted, per the ledger's own drift discipline.

| the draft said | the law says | why the draft was wrong |
|---|---|---|
| Ω is the operator manifold — the closed vocabulary of `N_i` with their weights | Ω is the totality of the individual universe (§3) | it made D a reachability computation over operators when D is a direct reading of one universe's own structure |
| D is a count — the number of surviving independent planes, the DEGREE of a polynomial | D is a SHAPE `{i, j, k, l}`, four extents, never a scalar (D1) | collapsing four extents into one number is the same error as the C it replaced: a scalar cannot say WHERE capacity is strained |
| `∇V` returns as a magnitude of available potential, overlaid on scoped dimensions | `∇V` is STRUCK WHOLE; V carries no continuous gradient (D0) | re-promoting the gradient in any role keeps a failed topology alive; and V is not a scalar field, so `∇V` has no referent |
| "the equations are untouched; only the objective inverts" | E1 is STRUCK — there is no master equation | the composition was refused, not merely re-aimed; many algorithms at many scales replace the one master |

The draft's surviving contributions are §3–§6 below.

## §3 Ω ≡ V — the unification, and what "no two parts cut out" means

The operator's clarification says Ω is **the totality of the individual
universe**. D0 says V is **the committed state read WHOLE: indivisible,
unpartitioned, never truncated to a subspace.** These are the same
object under two names.

    Ω  ≡  V        the individual universe, whole
    Δ(Ω) = Δ(V)    the relation it expresses    (`rfa_core::Delta`)
    Δ(Ω) = D       that relation's extent IS its dimensional capacity

This settles what "in union with V as a whole — no two parts cut out"
means, and the first draft read it wrongly. It does NOT mean "keep D and
a potential term joined in one objective." It means **the universe is
never cut** — read whole, or not read. That is the totality law already
landed at MESH-1d (every committed field measured or censused whole, the
UNMEASURED tripwire standing), restated at the foundation.

The prohibition has teeth because partition is exactly how the previous
frame failed: it guarded one term (`ζ∇V` demoted to damping) while the
objective leaked through another (C-scoring in `SYSTEM_SPEC` §2). A
guard on half of a partitioned object is not a guard. An unpartitionable
V has no halves to guard.

## §4 WHY THE REDUCTION SCOPES, AND WHY ∇V COULD NOT HAVE

The operator ruled that coboundary reduction is the scoping of a
dimension, not `∇V`. The sealed code already performs exactly that
scoping, in two phases, and both are scoping acts
(`engine/rfa-core/src/reduce.rs`):

1. **Extent** — `Δ'_ij,l = Δ_ij,l − Δ_0j,l + Δ_0i,l`. Anchoring at
   vertex 0 sends every frame-induced relation to exactly zero; "the
   surviving entries are the frame-free content." This fixes WHAT the
   dimension is: gauge vanishes, structure remains.
2. **Scale** — integer normalization by `lcm(denominators) /
   gcd(numerators)`, after which "every coordinate becomes an integer
   and the representative is scale-canonical." This fixes HOW FAR it
   extends. The scale is DERIVED, never chosen.

Extent and scale together are the scope of a dimension, and the
reduction produces both without consulting any potential. This is why
D4's floor is written `ext_x( Π_G(Δ(V)) )` — the reduction is what says
how far capacity may fall, because the reduction is what knows which
extent was frame and which was structure.

**Why `∇V` could not have done this job — the hole the strike closes.**
A potential is CHOSEN. Had `∇V` scoped the dimensions, structure would
have become an artifact of that choice, and a dimension carrying little
potential could be scoped OUT of existence silently, with no receipt.
That is precisely the discard the 2026-08-01 correction was fighting,
re-entering through the back door. Under the standing law a dimension
exists because the algebra found it, not because a potential rewarded
it — and D4 makes that enforceable: capacity holding genuine difference
(`orbit_lock ≠ ∅`) never contracts at all.

## §5 WHY C CANNOT LEAVE THE MATHEMATICS — support for its survival as semantics

The foundation keeps C "only as the SEMANTICS of the component readings"
(§3 of the ledger: congruence over `(i,j)`, coherence over `(i,j,k)`,
cohesion over `(l)`). That survival is not a courtesy; it is forced, and
the reason is worth stating once:

`H¹ = Z¹/B¹` is defined only on cocycles. Where the coherence reading
over `(i,j,k)` is non-zero, `Δ ∉ Z¹`, so `Δ` has no cohomology class —
and D4's floor `ext_x(Π_G(Δ(V)))` is then reading a reduction of
something that has no well-defined class to reduce to.

So the coherence reading is not a progress measure and not a target; it
is the **well-formedness condition that makes the capacity floor
meaningful**. A non-zero coherence reading says the relational data
contradicts itself on some triple — the state is malformed, not
"far from done." It refuses, or the inconsistency is receipted. It is
never scored, never minimized, never descended.

This is the kernel's own philosophy applied to the one place a score had
crept in: the kernel admits or refuses, it never optimizes.

## §6 THE LINEAGE — this completes a ruling, it does not reverse one

THE PROPORTIONAL LAW (2026-07-31) barred squares as aggregators, and the
sealed reason sits in `coherence_functional`'s own header:

> *"Squares are the mathematics of minimization — a loss shaped to be
> differentiated and descended; this lens exists to be READ."*

Descent was already outlawed at the level of the AGGREGATOR in July. It
survived at the level of the FORMATION — in `SYSTEM_SPEC` §2's sentence
scoring operators by how much they reduce C, and in E1's `ζ∇V` term.
THE FOUNDATION RULING finishes the same ruling one level up. Three
strikes, one law, seven months apart.

## §7 WHAT THIS MAKES BUILDABLE — the instrument

D is the measurement whole families of placeholders have been waiting
on. The method is the one Ruling Box 1 proved: build the meter, measure,
then rule — never rule a placeholder blind.

**The capacity meter is small, and every piece exists.** D is
`{i, j, k, l}`; `rfa_core::for_each_residual` already iterates exactly
that index space; `Delta { dim, fiber }` already bounds i/j/k by `dim`
and l by `fiber`. The meter is `ext_x(Δ(V))` and `ext_x(Π_G(Δ(V)))` —
the extent of each index before and after reduction. Both functions are
written. What is owed is a caller.

Building it:

- gives `rfa_core::coboundary_reduce` and `orbit_lock` their first
  production callers. `orbit_lock` is the per-index test D4 already
  names as its contraction veto — wiring it IS building the floor.
- gives `rfa_core::rcf_hash` its stated job: the canonical signature of
  the class, so a capacity claim is checkable across nodes.
- ends `engine/rfa-core`'s orphanhood — zero dependents today
  (`PICKUP.md` §8a) — turning witnessed unused mathematics into the
  capacity engine.
- retires placeholders BY MEASUREMENT: seat widths
  (`ledger/src/registry.rs:37`), galaxy reach
  (`ledger/src/galaxy.rs:33`), and occupancy caps (`11_AEON.md` §7) all
  name a minted-plane reading as their blocker. That reading is `l`,
  the fiber extent, and D3 already says how it grows — `GaugeShiftedAt`
  opens a fiber plane, earned by receipted mints.
- supplies the honest basis for the `ω` anchor: weight earned as
  verified capacity contributed rather than assigned
  (`mesh/src/agree.rs:57`) — the load-bearing gap of `SYSTEM_SPEC` §5.

## §8 THE THREE QUESTIONS — resolved 2026-08-01

**CLOSED by the ruling and its clarification (recorded):** Ω's shape
(§3), the scoping question (§4), D's arity — a shape, not a scalar
(D1) — and `∇V`'s fate (struck whole, D0).

The three carried questions were answered by relay. Two of the three
answers do not survive the sealed code, and the relay law binds —
translate, never transcribe; correct the source's violations rather than
inscribe them. The resolutions below are what the code supports.

### 8a. `k` growth — NO deed is owed. `k` has no independent extent.

The relayed answer was that a receipted deed is strictly required for
compositional growth, lest capacity drift through a back door. **The
premise is false, and the sealed iterator says so:**

```rust
let n = delta.dim;
for i in 0..n { for j in 0..n { ... for k in 0..n { ... for l in 0..delta.fiber {
```

`i`, `j` AND `k` are all bounded by `delta.dim`; only `l` is bounded by
`fiber`. **D = {i, j, k, l} is four INDICES over exactly TWO independent
EXTENTS.** `k` is the intermediate vertex of the cocycle triple, drawn
from the same entity set as `i` and `j` — it is not a degree of freedom
that can move on its own.

So `k` cannot drift: it cannot rise unless `dim` rises, and `dim` rises
only through `EntityRegistered`, which is already a receipt. A separate
`k`-deed would mint a degree of freedom the tensor does not have, and a
receipt for a movement that cannot occur is worse than no receipt — it
is a false entry in the ledger.

**The real defect this question surfaces.** D3 states "one index per
deed (the gauge law's shape — declaration == effect, every other index
held still)." That clause is **UNSATISFIABLE for `i`/`j`/`k`**: no deed
can raise `i` while holding `j` and `k` still, because all three read one
bound. Amendment owed to D3: *one EXTENT per deed*, the extents being
`dim` (carrying i, j, k together) and `fiber` (carrying l).

### 8b. Kernel invariants — YES in principle, NOT YET, and the bump must wait

The relayed answer was to elevate D2–D4 and execute the VERSION 2→3 bump.
The direction is right. The sequencing is not, and bumping now would
damage the chain.

**The blocker is a missing map.** D2–D4 are stated over `Δ(V)`. The
kernel judges `lawful(prior, next, step)` over `cfe::world::Committed`.
**No function anywhere in the workspace maps `Committed → Delta.**
Without it the kernel cannot compute `ext_x(Δ(V))` at all, so the
invariant is not merely unlanded — it is unenforceable.

**Why the premature bump is harmful.** `VERSION` binds into every
commitment (E5). Bumping 2→3 to declare an invariant the kernel does not
actually judge writes a false claim into the chain permanently: every
commitment would assert it was cut under a law that never ran. That is
the exact failure `judged = 0` exists to prevent — the hole stamped
visibly rather than hidden.

**The lawful sequence:** build `Committed → Δ(V)` · build the extent
meter (`ext_x` before and after `Π_G`) · witness red-first · take the
`rfa-core` dependency into the kernel (lawful — rfa-core imports only
`cfe`, so the arrow stays one-way, and `U_DIRECTIVES` §4 already
anticipates this at U2) · THEN amend the charter and bump VERSION **in
the same landing that makes the judgment real.**

### 8c. `ε_mag` — struck from D3. The conclusion holds; the apparatus does not.

The relayed answer was to eliminate `ε_mag` and bound growth by a
"Non-Vacuous Torsion Egress Boundary (S_out⁺)", permitting growth iff
`∮Θ ≠ 0`, with an "Eclipse operator" denying "cosmetic" shifts, citing
"Constitutional Law Article I / Article II".

**The conclusion is right. The apparatus is refused on two grounds.**

1. **The cited authorities do not exist.** `00_CHARTER.md` has no
   Article I or Article II; no `Θ` form, Eclipse operator, or
   `Ξ_cosmetic` appears anywhere in the tree. Inscribing invented
   citations is precisely the drift `EQUATIONS.md` §0 was written to
   prevent — a restatement with nothing behind it.
2. **The mechanism already exists under landed names.** "Growth
   permitted iff a non-zero cohomological defect" IS `orbit_lock ≠ ∅`.
   "Denied where the shift yields no structural uplift" IS
   `PlaneUnearned` (landed, `engine/cfe/src/world.rs`). D3 already
   requires opening grade *k* to carry *k* receipted mints. Nothing new
   is needed; the law is already topological.

**The real defect is a conflation of units.** E4's `ε_mag` bounds
`‖Δ‖₁` — an L1 MASS. D3 bounds an EXTENT — a count of indices. These
are different quantities in different units, and D3's clause "bounded
above by the magnitude gate (E4)" is a category error importing one into
the other. **Amendment owed to D3:** capacity growth is bounded by the
earned-mint gate (`orbit_lock ≠ ∅`, `PlaneUnearned` refusing), never by
a magnitude epsilon. `ε_mag` remains E4's own open question about
per-step L1 mass — a separate constant, still unnamed.

### 8d. Amendments owed to `EQUATIONS.md` §1 (named, not made here)

1. **D3** — "one index per deed" → "one EXTENT per deed" (§8a).
2. **D3** — strike "bounded above by the magnitude gate (E4)"; the bound
   is the earned-mint gate (§8c).
3. **D1** — record that the four indices carry two independent extents
   (`dim` for i/j/k, `fiber` for l), so `∀x ∈ {i,j,k,l}` in D2 is two
   independent constraints, not four.

## §9 OPEN — still needs the operator's word

1. **`ε_mag` itself** (E4, unchanged by §8c): the per-step L1 mass bound
   has never been named.
2. **The seat policy.** A worktree per seat is recommended and sound —
   `git worktree add ../seat-<n>` with committing from the root
   prohibited while seats are concurrent. Not executed here: another
   seat is live in the root tree, and restructuring under an active
   session is how work gets lost. Adopt at a quiet point.
