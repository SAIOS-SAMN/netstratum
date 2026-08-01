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

## §8 OPEN — needs the operator's word

**CLOSED by the ruling and its clarification (recorded):** Ω's shape
(§3), the scoping question (§4), D's arity — a shape, not a scalar
(D1) — and `∇V`'s fate (struck whole, D0).

Still open:

1. **The capacity receipt.** D3 says capacity rises BY RECEIPT, one
   index per deed. `GaugeShiftedAt` and `EntityRegistered` already
   carry `l` and `i/j/k` growth. Is any deed owed for a `k`
   (compositional) rise that neither covers, or is the vocabulary
   complete as it stands?
2. **The kernel invariant.** D2–D4 are inscribed in the ledger but are
   not yet invariants in `engine/kernel/CHARTER.md`. Landing admission
   and the contraction floor as a kernel-judged invariant is a charter
   amendment and would bump `kernel::VERSION` 2 → 3. Proposed, not
   ruled.
3. **`ε_mag`.** D3's growth is "bounded above by the magnitude gate
   (E4)", and E4's `ε_mag` has never been named. Growth is unbounded in
   practice until it is.
