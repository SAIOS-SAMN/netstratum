# 17 — THE CAPACITY LAW: Δ(Ω) = D

**Status:** DRAFTED 2026-08-01 (operator ruling, this session). Awaits
ratification. This document is the LAW; the amendments it owes to
`EQUATIONS.md`, `SYSTEM_SPEC.md`, and `engine/kernel/CHARTER.md` are
named in §5 and are NOT made here — one landing at a time.

Subordinate to `00_CHARTER.md`. Successor in force to the organising
role that `C(Δ)` held in `SYSTEM_SPEC.md` §2/§4; see §3 for the exact
demotion, which is narrower than "delete C" and must be read carefully.

---

## §1 THE RULING (operator, 2026-08-01 — recorded as given)

> kick C out entirely as the overall formation and start building
> around Δ(Ω) = D where D is dimensional capacity overlaid with the
> maximum potential of ∇V — the equations stay the same, but instead of
> forming around C we are looking at the D capacity and building
> towards its maximum potential in union with V as a whole — no two
> parts cut out, and the "descent" is only a uni-directional axial on
> the maximum dimensions of the polynomial union.

Everything below §1 is TRANSLATION, not the ruling. Where the
translation adds a form the ruling did not state, it is marked
**[DERIVED — ratify or strike]**. The relay law binds: translate,
never transcribe.

## §2 WHAT CHANGED, IN ONE LINE

The system formed around a scalar it was told to MINIMIZE. It now forms
around a graded quantity it is told to MAXIMIZE. The equations are
untouched; the objective is inverted and re-seated on the quantity that
actually carries value.

Why this is not cosmetic: `C(Δ) = ‖δΔ‖₁` is blind to the H¹ class by
construction (the 2026-08-01 correction, `60a5da4`, established this in
the standing law). The economy mints on the H¹ class. A system whose
organising quantity cannot see the thing it pays for will, under any
optimizer, walk away from the payment. Capacity is the class made
countable, so law and incentive now point the same direction.

**This completes a ruling already begun, and is not a reversal.** THE
PROPORTIONAL LAW (2026-07-31) barred squares as aggregators, and the
sealed reason is written in `coherence_functional`'s own header:
*"Squares are the mathematics of minimization — a loss shaped to be
differentiated and descended; this lens exists to be READ."* Descent was
already outlawed at the level of the AGGREGATOR. It survived at the
level of the FORMATION, in one sentence of `SYSTEM_SPEC.md` §2 that
scores operators by how much they reduce C. This law finishes the same
ruling one level up.

## §3 THE DEMOTION OF C — precise, and narrower than it sounds

**C is struck as THE FORMATION. C is NOT struck as a GATE, and cannot
be** — this is a mathematical constraint, not a preference, and the
operator's phrase "as the overall formation" is read as carrying it.

`H¹ = Z¹/B¹` is defined only on cocycles. If `C(Δ) > 0` then `Δ ∉ Z¹`,
so `Δ` has no cohomology class, so **D is undefined**. C is therefore
the admission predicate that makes D meaningful at all.

Its new standing, exactly:

- **Was:** the objective. "Operators are scored by how much they reduce
  C(Δ)" (`SYSTEM_SPEC.md` §2). This was greedy steepest descent on a
  class-blind scalar, and it is STRUCK.
- **Is:** a well-formedness predicate, ranking beside I1/I2/I3. `C(Δ) >
  0` means the relational data contradicts itself on some triple — the
  proposed state is malformed, not "far from done." It refuses, or its
  inconsistency is emitted as a receipt. Nothing is ever scored by it.

This is the kernel's own philosophy applied to the one place a score
had crept in: the kernel admits or refuses, it never optimizes
(`SYSTEM_SPEC.md` §0). C is a type check.

## §4 THE FORM

### 4a. Ω — the totality of the individual universe (RULED 2026-08-01)

`Ω` is **the totality of one individual universe**: its whole committed
relational state as a single object. Not an operator vocabulary, not a
selected subset, not a state-under-transformation. `Δ(Ω)` is the Delta
of the entire universe.

(The first draft read Ω as the operator manifold. That is STRUCK by the
operator's clarification. The consequence is a simplification: D is not
a reachability computation over operators, it is a direct reading of one
universe's own structure.)

This seats capacity as a **per-unit quantity**, which is what
`16_UNIT.md` already requires — the kernel is the deployable unit, and
PoUW/PoWC are judged at that level. Each universe carries one D. That is
the number units compare, trade, and are weighted by across the mesh.

### 4b. D — dimensional capacity (the scoped structure)

    Δ(Ω)  =  D          D  =  the structure surviving Π_G( Δ(Ω) )

D is what coboundary reduction leaves standing when the whole universe's
Delta is scoped (§4c). Frame artifacts contribute nothing; only
structural content counts.

**This is computable today, from code that already exists.**
`rfa_core::coboundary_reduce` performs the scoping; `Delta::dim` and
`Delta::fiber` carry the arity; the surviving independent entries are
the capacity. No new mathematics is owed — only a caller.

**D is not a new quantity.** Three things already in the law are D
under other names, and this is the strongest evidence the form is
right:

- `PhiK I^(3)` — "rank: dim of the relational structure" — is D,
  specified and never measured.
- A refused fold "emits the residue as a NEW plane (the mint)"
  (`SYSTEM_SPEC.md` §4). A plane is a dimension. **The mint is a
  capacity increment.** D counts minted planes.
- `12_UNION.md` §3 and `11_AEON.md` §7 defer seat widths and occupancy
  to "capacity as a live fiber reading (minted-plane count)" — a
  reading that has never existed. It is D.

### 4c. THE SCOPING IS THE REDUCTION, NEVER ∇V (RULED 2026-08-01)

**Coboundary reduction scopes a dimension. `∇V` does not, and must not.**

The sealed code already performs the scoping in two phases, and both are
scoping acts (`engine/rfa-core/src/reduce.rs`, quoted from its header
and body):

1. **Extent** — `Δ'_ij,l = Δ_ij,l − Δ_0j,l + Δ_0i,l`. Anchoring at
   vertex 0 sends every frame-induced relation to exactly zero; "the
   surviving entries are the frame-free content." This fixes WHAT the
   dimension is: gauge vanishes, structure remains.
2. **Scale** — integer normalization by `lcm(denominators) /
   gcd(numerators)`, after which "every coordinate becomes an integer
   and the representative is scale-canonical." This fixes HOW FAR the
   dimension extends. The scale is derived, never chosen.

Extent and scale together ARE the scope of a dimension, and the
reduction produces both without ever consulting a potential.

**Why ∇V must be kept out of the scoping — the hole this closes.** `V`
is a CHOSEN potential. If `∇V` scoped the dimensions, structure would
become an artifact of that choice, and a dimension carrying little
potential could be scoped OUT of existence — silently, with no receipt.
That is precisely the discard the whole 2026-08-01 correction was
fighting, re-entering through the back door. Under this ruling a
dimension exists because the algebra found it, not because the potential
rewarded it.

**What `∇V` does instead.** Its maximum potential is OVERLAID onto
dimensions the reduction has already scoped — reporting what each still
offers, never defining or bounding it:

    on each scoped dimension a:   c_a  =  the maximum potential of ∇V on a

`∇V` inverts role: it was damping ("gradient demoted to a hint, ζ small,
never the driver"), and it returns as a MAGNITUDE OF AVAILABLE
POTENTIAL. It answers *how much is here*; it never answers *which way*,
and it never answers *how big the dimension is*.

### 4d. The polynomial union — "no two parts cut out" **[DERIVED]**

D and V are ONE graded object, never two terms handled separately:

    U(Δ)  =  Σ_a  c_a · x^a          (degree = the capacity count;
                                      coefficients = available potential)

The union is a polynomial in the dimensional generators. Its DEGREE is
how many irreducible planes stand; its COEFFICIENTS are what each still
offers.

The prohibition on cutting it in two is a direct repair of the recorded
failure mode: the previous formulation guarded one term (`ζ∇V`) and let
the objective leak through another (C-scoring). A guard on half of a
decomposable object is not a guard. `U` is not decomposable by law.

### 4e. The descent constraint **[DERIVED — ratify or strike]**

Descent is not abolished; it is confined. Motion is admissible only if
it is:

1. **AXIAL** — along a single axis of the top-degree part of `U`. Free
   motion in the space is not lawful movement.
2. **UNI-DIRECTIONAL** — monotone non-decreasing in D. Capacity never
   retreats.

This is not a new arrow. It is `ELEVATE-1`'s "the fiber never retreats"
(`BadClimb`) generalised from the gauge climb to capacity itself.

### 4f. Rest replaces "done"

The old cycle was "done when C(Δ)=0 AND orbit_lock stays ∅" — a
terminal condition defined by an absence. There is no done. The system
RESTS when the leading axis of `U` offers no further potential, and
rest is not emptiness: the H¹ core stands at full degree underneath it.
The fixed point is maximal capacity, not minimal inconsistency.

## §5 AMENDMENTS OWED (named here, made at ratification — NOT made now)

1. `SYSTEM_SPEC.md` §2 — strike "Operators are scored by how much they
   reduce C(Δ)." Replace with the axial/unidirectional admissibility
   test. This single sentence is the descent; it is the highest-value
   line in the amendment set, and it is still only a sentence — the
   echelon scoring it describes is on the Build list and was never
   written. **Strike it before anyone implements it.**
2. `SYSTEM_SPEC.md` §4 — the dimensional-resolution cycle restated
   around D: unfold RAISES capacity, fold may lower it only to the H¹
   floor, and the loop rests rather than completes.
3. `EQUATIONS.md` — a new row for `U` and D beside C2/C3/C5; C2's row
   re-marked as a gate, not a measure of progress.
4. `engine/kernel/CHARTER.md` — **I7, THE CAPACITY INVARIANT
   [DERIVED — ratify or strike]:**

       I7 capacity:  D(next) ≥ D(prior),  or the decrement is emitted
                     as a receipt (minted, or refused with the loss named)

   This is I6 (*no difference dies without account*) made MEASURABLE.
   I6 today guards only the fold; I7 guards every step. Landing it is a
   charter amendment and bumps `kernel::VERSION` 2 → 3.
5. `16_UNIT.md` §4 — the discovery mint pays the emitted residue;
   under §4b that is "the mint pays a capacity increment," in units.

## §6 WHAT THIS MAKES BUILDABLE — the instrument, not the ruling

D is the measurement three families of placeholders have been waiting
on. This is the answer to "how do we attack the placeholders": stop
ruling them one at a time and build the meter that makes whole families
rulable at once. Ruling Box 1 is the precedent — `calibrate.rs` was
built, the ladder was measured, and only then was `f` ruled.

Building the capacity meter:

- gives `rfa_core::orbit_lock` its first caller. orbit_lock is the local
  test for whether a difference is gauge (`B¹`, foldable) or
  torsion-locked (`H¹`, must mint) — it is the per-pair capacity test,
  and wiring it IS building the meter.
- gives `rfa_core::rcf_hash` its stated job: the canonical signature of
  the class, so D's identity is checkable across nodes.
- ends `engine/rfa-core`'s orphanhood (zero dependents today,
  `PICKUP.md` §8a) — it becomes the capacity engine rather than
  witnessed unused mathematics.
- retires by measurement, not by decree: seat widths
  (`ledger/src/registry.rs:37`), galaxy reach
  (`ledger/src/galaxy.rs:33`), occupancy caps (`11_AEON.md` §7) — every
  one of which names a minted-plane reading as its blocker.
- supplies the honest basis for the `ω` anchor: weight earned as
  verified capacity contributed, rather than assigned
  (`mesh/src/agree.rs:57`).

## §7 OPEN — needs the operator's word

**CLOSED 2026-08-01 by operator clarification (recorded, no longer
open):** Ω is the totality of the individual universe (§4a), and the
scoping of a dimension is coboundary reduction, never `∇V` (§4c). Both
were carried as questions in the first draft and are now ruled.

Still open:

1. **§4b, the arity of D.** `Delta` carries two counts — `dim`
   (entities) and `fiber` (coordinates per relation). Is D the count of
   surviving independent PLANES, or the pair (dim, fiber) of the reduced
   tensor carried whole? The translation takes the former; the latter
   would make D a vector and the polynomial union richer.
2. **§4c, the overlay's reading.** "The maximum potential" is read as a
   supremum of `‖∇V‖` on each scoped dimension. Mean-available or
   integrated-along-the-axis are the alternative readings; they change
   the coefficients, not the shape, and none of them scope.
3. **§4e.** "Maximum dimensions" is read as the TOP-DEGREE part of `U`.
   If it means the full set of maximal axes rather than the single
   leading one, admissible motion widens from one axis to a face.
4. **I7's numbering and the VERSION bump.** I7 is proposed, not ruled.
5. **C's survival as a gate (§3).** The translation holds that C cannot
   be removed entirely without leaving D undefined. If the ruling
   intends C gone from the mathematics and not merely from the
   formation, D needs a different foundation and this document is
   wrong at §4b.
