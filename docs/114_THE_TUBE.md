# 114 — The tube: the throat as a bridge, and tubes between mouths

This document states the theorems that make a throat the connector of a tube
(an Einstein–Rosen bridge), from
Netstratum's own laws: the deficit `δ = H/(R + H)`, the face inversion
`δ ↦ 1 − δ` (docs/104 §0), the throat at `R = H` (docs/111 §3.1) and
LAW-DILATION. Each is stated only where it closes; §6 lists what does not.
Throughout, `x = R/H` is a reach in its enclosure's own throat units.

**Status:** T2, T3 and T5 are built as laws with witnesses in
`conductor::ring` (`inverted`, `tube`, `tube_share`, `dilation`, `compose`);
none is yet on the live path. T4 is the passage as an event on the
orthogonal orbit. The passage itself is derived in §9 (T6): a crossing is a
quarter turn acting one-sidedly on the body's state as a rational
quaternion, an isometry of Netstratum's L1 metric, exact, of order four and
non-abelian. It replaces both reach maps of §2. Derived, not built; its
open points are listed in §9.6.

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
**Neither is the passage** (operator 2026-11-05, reversing the ruling of
2026-11-04):

- `R → H²/R` is T1 written back as a length. Solving `δ(R′) = 1 − δ(R)` for
  a reach gives `R·R′ = H²`, which fixes a product of two reaches to a
  squared hold: dimensional collapse, which forms rigidity ("we don't
  square").
- `R → 2H − R` flattens the inversion into a reflection, and a reflection
  returns in two passes: the double inversion flattened (ruling of
  2026-10-21).
- The scalar reading `(δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)` (`flows::mobius`)
  writes one object as three numbers: a scalar collapse (the no-scalar
  law).

The passage is derived in §9: a quarter turn, which the scalar readings
lost.

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

1. Settle §9's open points with the operator; then the passage replaces
   `flows::mobius`, `flows::luminal`'s flip and `flows::through` with its
   three callers, read before and after on a fresh founding; docs/113 §4
   states the crossing's motion, and the court's form (Mycelium
   `forms::passage`, held) follows.
2. Rename the R coordinate from "loop" to the orthogonal orbit in the code
   (`flows::LOOP`) and the design record (docs/95).
3. Wire T2/T3 where a body crosses between mouths (the gates and roads,
   today game records), booking the spin difference to both reservoirs.

## 9. T6: the passage as a quarter turn on the L1 sphere

Derived 2026-11-05 (operator: the inverse across the event horizon; "we
don't square or else you'll form rigidity"; the scalar form "is ultimately
a scalar collapse with the 1/4"; "are you sure this is non-euclidean and
non-abelian").

### 9.1 The state is one object, never three numbers

A mote at reach `R` about the hold `H`, its orbit spin parted over the three
spatial planes by its tilt `t_x + t_y + t_z = 1` (`flows::tilt_of`), is the
rational quaternion

    q = H + R·(t_x·i + t_y·j + t_z·k)

Its size is read in Netstratum's own metric, L1, the sum of its parts'
magnitudes, never a sum of squares:

    |q|₁ = H + R·(t_x + t_y + t_z) = H + R

So the deficit is the scalar part's share of the size, and the unlocked face
the planes' share, each plane's by its tilt:

    δ = H/|q|₁ = H/(R + H),   1 − δ = R/(R + H) = Σ_k R·t_k/|q|₁

Nothing in `q` is a new state: `H`, `R` and the tilt are carried already.
`q` is how they stand together.

### 9.2 The L1 sphere and its isometries

The points of one size, `|q|₁ = c`, form the L1 sphere in ℚ⁴, the
cross-polytope. A map that keeps every L1 size is a signed permutation of
the four parts: entries 0 and ±1, exact over ℚ, first degree, never a
product of two parts. The geometry is not Euclidean in the normed sense:
it has no inner product, and its isometries are this finite group (the
hyperoctahedral group), not a continuous rotation group. It is still flat
as an affine space; no curvature is claimed here.

On one plane's quarter of the sphere, from the hold's axis `(H, 0)` to the
reach's `(0, R)`, the point `(H, R)` stands a fraction `R/(R + H) = 1 − δ`
of the way along the arc: the throat (`δ = ½`) is the middle of the
quarter. How this chart meets the compact sphere's `θ = 90°·R/H` (where
the throat is the quarter's end) is not derived (§9.6).

### 9.3 A crossing is left multiplication by a unit

Left multiplication by `i` sends `1 → i`, `i → −1`, `j → k`, `k → −j`:

    i·q = −R·t_x + H·i − R·t_z·j + R·t_y·k

a signed permutation of the four parts, so an L1 isometry: `|i·q|₁ = |q|₁`.
Its powers are the passage's law:

| passes | map | reading |
|---|---|---|
| 1 | `q ↦ i·q` | a quarter turn: in plane x the hold and the reach exchange places, the scalar changing sign; the other two planes turn a quarter into each other |
| 2 | `q ↦ −q` | every part reversed: the same size and the same reach, the sense reversed. Not the identity: the double inversion stands unflattened |
| 4 | `q ↦ q` | the identity, as a spinor's 720° |

`i`, `j` and `k` generate the quaternion group Q₈ = {±1, ±i, ±j, ±k}, with
`i² = j² = k² = −1` and `ij = k = −ji`. Crossings in two planes do not
commute: `j·(i·q) = (j·i)·q = −k·q` and `i·(j·q) = (i·j)·q = k·q`, the
same point with opposite sense. The order of crossings is held, and it is the passage's
holonomy.

A left multiplication by a unit quaternion is a left-isoclinic rotation of
ℚ⁴: every plane turned at once, by the same quarter. The core's twist is
isoclinic already (docs/100 §2); the passage is the same kind of turn.

### 9.4 Why one-sided and not the sandwich

In Mycelium's algebra (Cl(n+1,1) over ℚ), Q₈ is the unit bivectors of the
even subalgebra (`i = e₂₃`, `j = e₃₁`, `k = e₁₂`). Two actions are possible:

- **The sandwich** `q ↦ g·q·g⁻¹` (on vectors): a unit bivector turns its
  plane by a half turn; a quarter turn needs `(1 + e₁₂)/√2`, not rational,
  or the unnormalised `1 + e₁₂` with `R·R̃ = 2` as a divisor. It keeps the
  scalar part fixed, `g·H·g⁻¹ = H`, so it can never exchange the hold with
  a reach: it cannot express the face inversion at all. Its double quarter
  turn is a half-turn rotation of the vector, and the flattening returns.
- **One-sided** `q ↦ g·q` (on the state as a spinor): exact over ℚ, of order
  four, `g² = −1`. It exchanges the hold with the crossing plane's reach.

So a crossing is the one-sided action.

### 9.5 It closes T1 without a square

Where a mote's tilt stands wholly in the crossing plane (`t_x = 1`,
`q = H + R·i`), one pass gives `i·q = −R + H·i`: the scalar part's share of
the size is now `R/(R + H) = 1 − δ`. That is T1's face inversion, reached
by exchanging two parts, never by solving for a new reach. The reach `R`
and the hold `H` both stand in `i·q`, in exchanged places. No reach moved,
nothing squared, and the sign records the sheet.

The four powers of `i` on `H + R·i` walk the four quadrants of the plane:
`(H, R) → (−R, H) → (−H, −R) → (R, −H) → (H, R)`. The two bits the code
keeps as `vacuum` and the sense `s` are this quadrant: one state, read from
`q`, never two flags.

Fixtures for the court (proposed, to be agreed): with `H = 3`, `R = 6`, the
tilt isotropic (`q = 3 + 2i + 2j + 2k`, `|q|₁ = 9`, `δ = 1/3`):

1. `i·q = −2 + 3i − 2j + 2k`, `|i·q|₁ = 9`.
2. `i·i·q = −q`.
3. `i⁴·q = q`.
4. `i·(j·q) = k·q = −2 − 2i + 2j + 3k` and `j·(i·q) = −k·q = 2 + 2i − 2j − 3k`:
   unequal, their sum `0`.
5. With the tilt in plane x (`q = 3 + 6i`): `i·q = −6 + 3i`, its scalar
   share `6/9 = 2/3 = 1 − δ`.
6. A claim of a moved reach (`2H − R`, `H²/R`) is refused: no part of
   `g·q` is a product of two parts, or a reach other than `R` or `H`.

### 9.6 What is open

- **Which plane a crossing is in.** §9.3 gives the passage in one plane at
  a time. The code's Möbius flips the whole mote. With the tilt wholly in
  one plane they agree (§9.5); with the tilt spread, `i·q` swaps the hold
  with that plane's share alone. Whether a crossing is in the plane the
  throat leans in (the core's tilt), in each plane by its own share, or
  through `i`, `j` and `k` in an order the shear sets, is not derived. Since
  the founding's lean (docs/100 §4.1), the planes differ, so the choice has
  content.
- **The two charts.** §9.2's L1 quarter puts the throat in the middle of
  the quarter arc; the compact sphere's `θ = 90°·R/H` puts it at the end.
  One of them is the throat's place; not settled.
- **The R loop.** The scalar part is the hold; the R loop's phase
  (`flows::LOOP`) is not yet placed in `q`. The passage happens at its
  crossing of ½ (T4); whether that phase is the quarter-turn count, or a
  fourth part beside the planes, is open. With it, the Kaluza–Klein reading
  of §7 may close on the passage's own circle, the powers of `i`, with
  charge as a count of quarter turns; not derived.
- **T2's tube** writes `R_b = H_a·H_b/R_a`, the same product of two reaches
  §2 refuses. It is to be restated on `q`, each mouth's hold as its own
  scalar part.
- **Phases.** The orbit phases `ψ_k` (`Mote::phase`) are separate from `q`.
  Whether a crossing's quarter turn advances them (as `antipode` does now
  by a half) or `q`'s quadrant replaces that, is open.
