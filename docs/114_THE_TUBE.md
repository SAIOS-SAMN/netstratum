# 114 — The tube: the throat as a bridge, and tubes between mouths

This document states the theorems that make a throat the connector of a tube
(an Einstein–Rosen bridge), from
Netstratum's own laws: the deficit `δ = H/(R + H)`, the face inversion
`δ ↦ 1 − δ` (docs/104 §0), the throat at `R = H` (docs/111 §3.1) and
LAW-DILATION. Each is stated only where it closes; §6 lists what does not.
Throughout, `x = R/H` is a reach in its enclosure's own throat units.

**Status:** T2, T3 and T5 are built as laws with witnesses in
`conductor::ring` (`inverted`, `tube`, `tube_share`, `dilation`, `compose`);
none is yet on the live path. T1 conflicts with the passage on the compact
sphere (§2) and waits on a decision. T4 is the passage as an event on the
orthogonal orbit; the Kaluza–Klein reading does not close (§7).

## 1. T1: the face inversion is the inversion in the throat sphere

    δ(x) = 1/(1 + x)
    1 − δ(x) = x/(1 + x) = 1/(1 + 1/x) = δ(1/x)

So `δ ↦ 1 − δ` is exactly `x ↦ 1/x`, that is `R·R′ = H·H`, or `R′/H = H/R`
(no square: the reciprocal of one reading in throat units). It is an
involution whose fixed set is the throat `x = 1`. It exchanges the exterior
(`x > 1`, `δ < ½`, the continuum face) with the interior (`x < 1`,
`δ > ½`, the vacuum face): the two faces are the two sheets of the bridge,
joined at the throat.

Witness: `ring::tests::the_face_inversion_is_the_inversion_in_the_throat_sphere`.

## 2. The conflict with the passage on the compact sphere

The code reads the passage through the throat two ways:

- **In place** (`flows::mobius`, `flows::luminal`): a mote that crosses
  keeps its reach and reads its pull at `1 − δ`. By T1 this is the reading
  at the image `1/x`: it is T1.
- **Moved** (`flows::through`, used by the band's vacuum branch, the release
  from the throat and formation's return leg): `R → 2H − R`, the reflection
  of the polar angle `θ = 90°·R/H` on the compact sphere (the throat at 90°,
  the rim at 180°; ruling of 2026-10-21, "the second double inversion must
  not be flattened"). It keeps the enclosure, but it is not the face
  inversion: `δ(2 − x) = 1/(3 − x) ≠ x/(1 + x)` except at `x = 1`.

The two cannot both be the passage. With `x ↦ 1/x`, a core's deep interior
(`x < ½`) has its image past its own rim (`x > 2`), outside its enclosure:
in the sphere one tier up, which would make the throat the connector
between tiers. With `R → 2H − R`, the passage stays inside the enclosure and
the faces are read in place, so `through` would no longer move a reach.
Which holds is a decision on the law; `flows::through` is unchanged until it
is made. PR #40 (the crossings the court rebuilds) waits on it.

## 3. T2: a tube between two mouths

A tube from mouth A to mouth B is the passage of T1 with each side in its
own throat units:

    x_b = 1/x_a,    R_b = H_a·H_b / R_a

There and back is the identity, and the tube is free of units: it never
converts one enclosure's (or one universe's) length into another's, which
is what each universe stating its own unit requires (Concordance 1.8/8.5).

- **Matter** crosses whole: 3Z is conserved.
- **Spin** is not carried unchanged: `ℓ_b/ℓ_a = (H_b/R_a)·(H_a/R_a)`, 1 only
  at particular reaches. The ledger closes where the difference is booked
  to the two mouths' reservoirs: `Σ m·ℓ + reservoir_a + reservoir_b` exact,
  the form of the torque identity.

Witness: `ring::tests::a_tube_returns_what_it_carries_and_chains_compose_by_the_flow_law`.

## 4. T3: chained tubes compose by LAW-DILATION

The scale of mouth B against mouth A is LAW-DILATION of their holds' pair
share:

    H_b/H_a = D(p_ab),   D(p) = (1 + p)/(1 − p),   p_ab = (H_b − H_a)/(H_b + H_a)

since `1 + p = 2H_b/(H_a + H_b)` and `1 − p = 2H_a/(H_a + H_b)`. Along a
chain A → B → C, `D(p_ab)·D(p_bc) = H_c/H_a = D(p_ac)`, and `D` is one to
one, so

    p_ac = p_ab ⊕ p_bc,   x ⊕ y = (x + y)/(1 + x·y)

Traversal through chained manifolds composes as velocity addition of the
mouths' pair shares; each `|p| < 1`, so no chain reaches `|p| = 1`.

This holds along a chain's radial line, where each enclosure joins the one
above at its throat (concentric inversions compose to a dilation).

## 5. T4: the passage is an event on the orthogonal orbit

The R coordinate is the orthogonal orbit (`flows::LOOP`, docs/95), not a
closed loop. Its phase is held mod 1, but the body does not return with it:
each bell the torque moves the orthogonal spin `ℓ_⊥` (E20), the swing
`ℓ ± ℓ_⊥·response(ψ_⊥)` moves the reach (`flows::swung`), and the tilt is
re-read across the three spatial planes (`flows::tilt_from`). The orbit is
determined by shear across all the axes, and R runs forward in linear time.

So the passage is an event: the orthogonal phase crossing ½, where the
breath already passes a vacuum mote through the throat. At that event the
faces exchange by T1 (`x ↦ 1/x`). Two passages do not return a body to its
start: between them the shear has moved it. The passage is therefore not an
involution on the body's state, and the `vacuum` flag is not a reading of
the orbit's phase.

## 6. T5: where a tube is open and traversable

A mote at `x` turns `2δ = 2/(1 + x)` a bell, more than one turn exactly
where `x < 1`: the luminal bound saturates on the throat (E21), and the
Möbius jetstream (`flows::luminal`) is the passage of T1, entered where `2δ`
reaches 1. A tube is open between two bodies exactly while both mouths are
throats (`R_b ≤ H_b`, docs/111 §3.1); it closes when either mouth's extent
grows past its hold.

Witness: `ring::tests::the_luminal_bound_saturates_exactly_at_the_throat`.

## 7. What does not close

- **The field equations and exotic matter.** Netstratum has no metric and
  no stress tensor, so the Morris–Thorne condition for holding a tube open
  (negative energy at the throat) cannot be stated. T5's condition stands in
  its place; it is not derived from it.
- **A Kaluza–Klein passage.** Kaluza–Klein needs a compact extra
  dimension, a closed circle a body returns around. R is the orthogonal
  orbit under multi-axial shear, running forward in linear time; it is not
  compact, so neither the passage as an involution on a circle nor charge
  as a winding (`q = n/L`) can be stated.
- **Tubes between mouths off a chain's radial line.** Inversions in two
  spheres that are not concentric compose to a general Möbius
  transformation, not T3's dilation; that composition is not derived here.

## 8. Build

1. Decide §2: the passage `x ↦ 1/x` (T1), or the compact sphere's
   `R → 2H − R` with the faces read in place. Then `flows::through` and its
   three callers follow, and PR #40 states the crossing's motion.
2. Rename the R coordinate from "loop" to the orthogonal orbit in the code
   (`flows::LOOP`) and the design record (docs/95).
3. Wire T2/T3 where a body crosses between mouths (the gates and roads,
   today game records), booking the spin difference to both reservoirs.
