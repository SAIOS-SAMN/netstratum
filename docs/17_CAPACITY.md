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

### 4a. Ω — the operator manifold **[DERIVED — ratify or strike]**

`Ω` is the closed vocabulary of lawful operator shapes `N_i` together
with their admissible weights `ω_i = weight_i · confidence_i`. It is the
whole manifold, never a selected operator: `Δ(Ω)` is the state's
response to everything the law permits, not to one chosen deed.

### 4b. D — dimensional capacity **[DERIVED — ratify or strike]**

    D(Δ)  =  dim span_{H¹} { Π_G( N_i(Δ) ) : N_i ∈ Ω }

The number of independent irreducible planes the operator manifold can
reach from `Δ`, counted modulo gauge. Frame artifacts (`B¹`) contribute
nothing; only structural content counts.

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

### 4c. The ∇V overlay **[DERIVED — ratify or strike]**

D is not a bare count. Dimensions are not equal: a plane along which
the potential gradient still has something to give is worth more than a
spent one. The overlay grades each axis by the maximum potential
available along it:

    c_a  =  sup_{axis a} ‖∇V‖          (the maximum potential of that axis)

`∇V`'s ROLE INVERTS and this is the load-bearing change. It was demoted
to damping — "gradient demoted to a hint, ζ small, never the driver."
It is now re-promoted, but as a MAGNITUDE OF AVAILABLE POTENTIAL, never
as a direction of travel. It answers *how much is here*, never *which
way is downhill*. The old guard was placed on this term while descent
leaked in through the scoring rule; under this law there is no scoring
rule to leak through.

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

1. **§4a, Ω.** Is Ω the closed operator vocabulary as stated, or does it
   carry the weights as a manifold with its own geometry? The
   translation takes the former.
2. **§4c, the overlay.** `sup ‖∇V‖` per axis is the translation's
   reading of "maximum potential." A different reading (mean available
   potential, or potential integrated along the axis) changes the
   coefficients but not the shape.
3. **§4e.** "Maximum dimensions" is read as the TOP-DEGREE part of `U`.
   If it means the full set of maximal axes rather than the single
   leading one, admissible motion widens from one axis to a face.
4. **I7's numbering and the VERSION bump.** I7 is proposed, not ruled.
5. **C's survival as a gate (§3).** The translation holds that C cannot
   be removed entirely without leaving D undefined. If the ruling
   intends C gone from the mathematics and not merely from the
   formation, D needs a different foundation and this document is
   wrong at §4b.
