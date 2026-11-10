# 120 — The coordinate: TUniversal inside the universe

This document derives the coordinate every body in the universe stands at,
in the form of Mycelium's TUniversal point, so that a body's state is its
coordinate, its motion is the closed-form advance of that coordinate, and
every reading (closeness, membership, a neighbour, a crossing) is taken by
addressing coordinates, never by iterating a catalogue of bodies.

**Status:** derived 2026-11-05, proposed for the operator's reading
(operator: "start unpacking this from an advanced spreadsheet or card
catalogue and start using the manifold coordinate system we have been
building since day 1 ... derive the algorithmic coordinates and immediately
reference that without cartesian iterations that explode on dimensional
expansion or contraction: use the TUniversal coordinate system and move
directly via the computational physics we define and derive"). It answers
docs/119 §A, whose rows all stand on the state as a coordinate tuple, and it
is the state type docs/118 Part IV step 2 asks for. Open points are in §9.

## 0. What the record already holds

- **The point** (Mycelium `chronometry::tuniversal`, 78.152 TU-5; docs/80):
  `origin / h : S : G : T : H : K`. `origin` is a genesis and an epoch, and
  two points of two origins are never compared; `h` is the tick; the four
  lanes are exact rationals, each the cumulative measured change of one kind:
  **S** boosts and dilations, **G** hue moves, **T** null fluxes, **H**
  turns; **K** counts the orbits closed. The point is ordered as it is
  written, "a canonical key, never a measure". Its navigation (TU-6):
  *unpack* (the coordinates by name), *branch* (a named child of a point),
  *superpose* (two points joined), *reset* (a new epoch re-originated at a
  point).
- **Time is a coordinate** (docs/80): an event is a place and its time;
  the interval between two events is a distance every observer agrees on;
  proper time is one observer's worldline; cosmic time is measured from the
  singularity's bounce.
- **Circuitry, never steps** (docs/78, design law 5): a state at t is its
  closed form, `φ + ω·t`, never stepped bell by bell.
- **The forms** (docs/84, concordance 2.10): SPAN, a certified enclosure of
  an exact value, narrowed only where a law decides; TURNING, a phase as an
  exact turn; READING, a floor on base 60 to depth d, **for observers
  only**. Nothing in the state is a grid.
- **The motion types** (docs/117 §3, §8.3): a generator's components are
  turnings (two axes of one sign), boosts (two of opposite signs), and null
  pairs (a horizon), read in first degree.

These are one structure. The point's lanes are the motion types.

## 1. The coordinate of a body

Every body (a mote, a star, a world, a galactic core) stands at

    origin / h : S : G : T : H : K

| field | what it is inside the universe | its kind |
|---|---|---|
| `origin` | the body's enclosure: the throat it stands about, named by its path from the universal core | a path, never compared across enclosures |
| `h` | the bell, the universe's tick, from the founding | a count |
| `S` | its cumulative boost: the dilation of its reach, R's plane's span (docs/117 §10.2) | exact, signed |
| `G` | its cumulative hue move: the change of its kind, its place on S¹¹ (docs/97 §2, docs/117 §5) | exact |
| `T` | its cumulative null flux: each passage through the throat writes its span's change, `−2·(A − Q)` (docs/117 §12) | exact, signed |
| `H` | its cumulative turning: the turnings between planes the code's phases read (docs/117 §10.3) | exact, signed |
| `K` | the orbits it has closed | a count |

- **The coordinate is the state.** A body's reach is read from `S`, its
  kind from `G`, its sheet from the sign of `S`'s span (docs/117 §12), its crossings' flux from `T`, its place on its orbit from
  `H`, its completed turns from `K`, each at its enclosure. The fields a
  mote carries now (reach, spin magnitudes, tilt, phases, `vacuum`, kind)
  become readings of its coordinate.
- **Lanes are never summed into one another** (MU-RECURSION law 5): two
  coordinates are compared lane by lane, and no reading adds a boost to a
  turning. This is what removes the scalar collapse of docs/119 §C17 (two
  planes' spins summed into one number) and the cartesian closeness of
  §A7–A8 (per-axis differences summed or multiplied).
- **Signed lanes.** Mycelium's lanes are non-negative issuance. Inside a
  universe a turning and a boost run in both senses (docs/78 design law 6),
  so S, T and H are signed. What crosses to the court is read from them
  (§8).

## 2. Motion is the closed-form advance of the lanes

A body's motion over a span of bells is its generator's components, by
type, times the bells, added to its lanes:

    h ↦ h + n,   S ↦ S + n·b_S,   H ↦ H + n·b_H,   T ↦ T + n·b_T

with `b_S`, `b_H`, `b_T` its generator's boost, turning and null components
(docs/117 §8.3), read in first degree from its coordinate and its
enclosure's, and `K` the completed turns of `H`. It is docs/78's `φ + ω·t`
with the turning as `ω` and the lanes as `φ`.

- **No bell is stepped.** A body is advanced from the bell its coordinate
  was last written to the bell a reading asks for, in one addition per lane.
  Nothing is iterated bell by bell or body by body.
- **A law changes a generator, never a coordinate directly.** An encounter,
  a capture, a burn or a torque changes what a body's lanes advance by from
  that bell on; the coordinate between events is closed form.
- **An event happens at a coordinate.** Two bodies meet, a plane reaches its
  horizon, a body completes a turn, where their lanes reach a value a law
  names. The bell of that event is solved from the lanes' closed forms (one
  first-degree solve per pair of lanes), not found by stepping until it
  happens. A turn completes where `H` reaches its next whole turn; a body
  crosses its throat where `T`'s reading flips.

## 3. Neighbours are found by addressing, not by scanning

The point is ordered as it is written: origin, tick, each lane in lane
order, `K`. Bodies of one enclosure are held in that order, as an ordered
index keyed by coordinate.

- **A body's neighbours are its neighbours in the order,** read by bisection
  on the lane a law reads: bodies close in reach are close in `S`, bodies
  close on their orbit close in `H`. A law that reads closeness on one lane
  reads it from the index by a range on that lane, in `log n`, never by
  testing every pair (docs/119 §A28–A36, the standing ruling "peels and
  shears, not sweeps").
- **A law reads only the lane it needs.** Membership in a body's sphere is a
  range on `S`; being in step is a range on `H`; a crossing is a sign on
  `T`. No law reads every axis to decide one thing (docs/119 §A7–A12).
- **Two lanes, two ranges.** Where a law reads two lanes (in reach and in
  step), it reads the range on one and filters by the other within it. The
  lanes are never combined into one distance.

## 4. Expansion and contraction are depth, not iteration

The sexagesimal reading at depth d (pure base 60, LAW-COORDINATE v1; degrees
are a caller's unit, so Xylarium's depth d on turns is this reading's depth
d − 1 on degrees) places a lane's exact value in a **sector**, the exact
interval `[m·60⁻ᵈ, (m + 1)·60⁻ᵈ)` that holds it, decided by
exact comparison and never rounded (Mycelium's frame law: "never a floor on
a grid"). The coordinate itself is kept whole. The sectors nest: a node at
depth d holds its sixty children at depth d + 1, so
they form a tree, and a node is a range on the lane:

- **Contraction** (zooming out, the dynamic carriers ruling of 2026-10-24:
  "zoom out to certified sectors") is reading to fewer places: bodies whose
  coordinates share their first d places stand in one node. Nothing is
  merged or recomputed; the node is the bodies under it.
- **Expansion** (zooming in, "render exact only under observation") is
  reading to more places: the node's children, down to the exact coordinate.
  Nothing is iterated; a deeper node is a longer path.
- **The cost of a reading is its depth, never the number of bodies.** A
  region of the universe is a node; a reading of it walks d levels; a body
  is found by its path. Dimensional expansion and contraction change d, and
  d is at most the six places the universe reads (docs/78, the TUniversal
  coordinate's six sexagesimal places), so neither explodes.
- **The tree is a reading, never the state** (docs/84's gate): the state is
  the exact coordinate; the places are how an observer, and a law that
  decides at a width, address it.

## 5. The tiers are origins

A body's `origin` is its enclosure. The tiers nest as origins nest:

- **Branch is a birth.** A star system born in a galaxy is a branch of the
  galaxy's point: a new origin named by its parent, at the bell it was
  born, its bodies' coordinates written at that origin (Mycelium TU-6
  BRANCH). A world is a branch of its system.
- **Superpose is a merger.** Two bodies that join stand at one coordinate
  read from both (Mycelium TU-6 SUPERPOSE): their lanes joined by the law
  of the encounter (holding-weighted, docs/96), at one origin. No residual
  is discarded: the merger keeps each body's offset from the merged point
  per lane (their holding-weighted sum is zero), so the merged body's
  internal spread is read back and **fission is the inverse** (agreed with
  Mycelium 2026-11-05, fixtures 9 and 10).
- **Reset is a re-origin across a throat.** A body crossing out of its
  enclosure into the one above is written at the new origin, its lanes read
  across the passage (docs/117 §9.3–§9.4): the null flux `T` records the
  crossing, and no reach moves (LAW-PASSAGE).
  - **Between a galaxy and the ocean** (register B5, 2026-11-05): an exit
    carries a mote's spin into the universe by its galaxy's share `f`,
    `ℓ_u = ℓ_g + ℓ·f` in each orbit (`ℓ_g` the galaxy's own); the arrival is
    its inverse, signed, `ℓ = (ℓ_u − ℓ_g)/f`. Nothing is placed and nothing
    is paid: the mote's spin about the universal core is kept exactly, a
    round trip returns it, and a mote counter-turning against the galaxy it
    lands in lands counter-turning (the dynamic signature, docs/117). It
    replaced an arrival that read `|ℓ_m − ℓ_g|/f` (the sign lost) under the
    galaxy's clock and added a fresh landing balance `r·H(<r)` (a placement)
    and the swing's radial share across the orbit. A galaxy holding nothing
    (`f = 0`) takes no arrival. The phases are still re-set at both
    crossings (the exit writes the galaxy's, the arrival `½`): open.
- **Coordinates of two origins are never compared** (TU-5). A reading across
  tiers goes through the origins: a body is compared with another in a
  different enclosure by carrying one to the other's origin, through the
  throats between them (`enclosure::route`'s path), never by subtracting
  their numbers. This is docs/89 B6's missing tier map, and its form.
- **The clocks are branches of TUniversal** (docs/101 §0): a tier's tick is
  its origin's, nested as the shares between it and the source.

## 6. What this replaces

| now | becomes |
|---|---|
| `Mote` and `Body` fields (reach, `ell_o`, `ell_orth`, tilt, phase per index, `vacuum`) | one coordinate per body; the fields are readings of its lanes |
| `SPACE = [0, 2, 3]`, `LOOP = 1`, `phase[k]` | lanes named by what they measure |
| per-axis updates each bell (`turn`, `spin_torque`) | the closed-form advance of §2 |
| bells stepped one by one in `breathe` | bodies advanced to the bell a reading or an event asks for |
| all-pairs loops (`pair_sums`, `collections`, `cascade`) and linear filters over every mote | ranges on the ordered index (§3) |
| orthant keys, phase grids, bounding boxes (`beads`, `band_pairs`, `strands`, `horizons`) | ranges on one lane; nodes of the reading (§4) |
| radial differences for landing, capture and joins | ranges on `S` and `H` within one origin |
| expansion and contraction coded twice (docs/119 §A17–A27) | one advance with its sign, and one re-origin across a throat |
| the 60⁻⁶ grain in the state | exact lanes; the places a reading only |
| `birth_key` (a sector plus a collision bump) | a body's identity is its origin and its birth coordinate, its content (MU-RECURSION law 2) |
| the stored `t_u` and the schedule on it | events solved from the lanes (§2); the established period a reading |

## 7. What it fixes in the register

- **§A (cartesian structure):** A1–A16 by the coordinate and its lanes;
  A17–A27 by one advance and one re-origin; A28–A36 by the ordered index.
- **§B:** B1 (no galaxy forms) becomes a branch at the bell a binding
  closes; B2 (the ocean never lands) a re-origin into the galaxy whose range
  on `S` holds the parcel; B3 (system motes discarded) cannot happen, since
  a settle changes generators, not coordinates; B4–B5 (the spin books)
  become one book per lane.
- **§C:** C1 (the phases) and C2 (`vacuum`) become `H` and `T`; C14
  (the stored period) goes; C13 (the grain in the state) goes.
- **§J:** J1 (the court's form) through the point's own form; J9 (identity
  by resolution and order) and J10 (rounding with residue dropped) by exact
  lanes.

## 8. The court

The point's form is the court's already. What crosses the universal core's
vertex (docs/113) is a body's coordinate at the core's origin: its lanes
read as the court reads them (issuance per lane, non-negative), its passage
a T-lane event under LAW-PASSAGE. The court verifies a crossing by
reconstructing the coordinate from the generator and the bells, which is
what MU-RECURSION law 8 asks.

## 8a. Alignment across the nodes (2026-11-05)

- **The lane meanings** are the network's (Mycelium's point; Xylarium's
  network.md): S spatial, G gauge, T temporal, H holonomic. Xylarium's
  kernel lanes map to them: its S (a phase on the anchor's ring) is a
  one-plane reading of H; its G (holding as a share of the anchor's, with
  `D(δ)` its dilation) is a reading of S; its T (the beat) is T. Its G from
  the transductions (fusion, fission, frame pairing) is planned.
- **The shared form:** proposed to Mycelium that the coordinate and its
  sexagesimal reading be one form all kernels use, Xylarium's
  `xyl-core::coordinate` the base. Xylarium agrees and deletes its copy
  when the form lands; Netstratum's `conductor::coordinate` is built in its
  shape to be replaced by it.
- **Xylarium's kernel takes no Mycelium dependency** (only its private
  uplink links the SDK), so it conforms to LAW-COORDINATE v1 by reading
  proposal 78.173's fixtures as its own tests, its lanes renamed in place.
  Netstratum links Mycelium's forms already and imports `forms::coordinate`
  when it is released at a tag; until then `conductor::coordinate` carries
  the form's names, writing and reading (pure base 60, `Sector`, `within`
  and `Side`, `OriginsDiffer`, `Bodies::range`) and reads 78.173's fixtures
  1–6 as tests, with the motion stated to Mycelium as fixtures 7–10 (the
  advance, the event's bell, the merger with its offsets, fission).
- **Xylarium's condition:** a lane's advance is a sheared return, never a
  closed repeat, so the form holds each lane exactly and assumes no period.
  §2's advance does: `K` counts H's whole turns; no cycle is assumed.

- **LAW-FIELD v1 (Mycelium 78.171), 2026-11-05:** read against docs/114–121,
  amended by Mycelium. The pole basis is its null pair (`e₀/e∞` written
  `n_Z/n_Q`, docs/117 §10), the throat's fixed sphere its membrane. Its
  signature is read per plane (`Cl(p, q)`). A point holds its winding with
  each phase, so `H` stays exact and `K = ⌊|H|⌋`. Mycelium's operator agreed
  that a sphere enters the field's law only as a holding's own enclosure,
  never as a reference surface, so docs/117's `S⁷ ⊂ S¹¹` stands as nested
  enclosures. Its fixtures 1–6 hold by hand; they are read as tests when its
  form has an API.

## 8b. A lane is a vector at every tick (operator, 2026-11-05)

The operator ruled: "for each sexigesimal tick you contain your entire
universe on a dimension as a vector". A lane is never one number:

- **Each lane holds a vector on the axes,** the universe's whole content on
  that dimension. `H` holds the per-plane phases and the turnings between
  planes as one degree-2 generator on the poles basis (docs/117 §8, §10),
  never a phase folded into a rational; `S`, `G` and `T` likewise hold their
  components on the axes they move on.
- **The sexagesimal reading reads the vector at each tick:** a sector at
  depth `d` is the exact cell holding every component, never a reading of
  one component or of a norm (no square, no scalar collapse).
- **Order and ranges stay first degree:** the canonical key orders a lane by
  its components in axis order, exactly; a range on a lane is a range on one
  of its components, read by bisection as now.
- **What this changes:** LAW-COORDINATE v1 (78.173) holds each lane as one
  exact rational, so `H` across three planes is a scalar collapse. Brought to
  Mycelium before v0.4.0's tag; `conductor::coordinate` follows the agreed
  form.

## 8b′. A plane closes on itself (operator, 2026-11-05)

"Plane needs closure": a plane is a closed circle on the manifold (`S¹`
inside `S⁷`), never an open plane. Its phase wraps at 360°, its whole turns
counted by `K`, and a spin on it is a turning around that circle whose sign
is the sense of travel. With per-plane spins (§6), each plane answers on
its own circle. A component that passes zero carries the body through the
centre on that plane alone (its phase there half a turn on, its magnitude
kept), never on every plane at once by the summed spin
(`flows::through_centre` today).

## 8c. A body's own enclosure: the volume a point collapsed (operator, 2026-11-05)

The operator ruled: "identify the collapse and add the volume back both
semantically, and dimensionally". (Never "cells": ruling 2026-10-16.)

- **The collapse.** A mote carries its holding as an amount at one reach,
  one spin per orbit and one phase per plane: a point. The matter it stands
  for occupies a volume, which the record already states. The dispersal
  ruling (2026-10-16) has every kind spread over `R(1 ± p)`, and
  `flows::held` reads a mote's held share as the part of `R(1 ± p)` inside
  the throat. The volume lives in that one reading and is dropped
  everywhere else. So every law that sets one reach for matter with extent
  flattens it: the release's mirror (the inner edge), and either edge.
- **Semantically: a mote is a body,** holding with its own enclosure, in the
  grammar of stars and worlds (one grammar, coupled tiers). Its extent is
  its own sphere about its place, the holding's own enclosure (the one
  sphere LAW-FIELD admits). That sphere is the dispersal's half-width, `R·p`.
- **Dimensionally: the extent is state,** carried on `S` beside the reach,
  a component per plane of the base. A law reads how much of a body's
  enclosure stands where: the held share, a capture, a bond and an exit are
  each the part of the enclosure on one side of a boundary. None of them
  compares a single reach.
- **The release:** the core's released parcel stands at the throat (`R = 1`)
  with its own enclosure spanning `± span·c`, both sides of the throat at
  once, as the dispersal law states. Nothing is placed and nothing mirrored.
  Its held share is read from it.
- **A passage** exchanges reach and hold and carries the enclosure whole:
  `H + R` is kept, so the volume is conserved through the crossing.
- **The extent is derived, never set** (operator 2026-11-05: "let the
  codebase derive the degrees of separation"): `flows::own_enclosure` reads
  a body's extent on each plane of its orbit, where its own pull meets its
  enclosure's, the pinch share `m/(m + H(<R))` of its reach
  (`ring::pinch_share`). Nothing stores a width. Its reading in degrees is
  open: a ratio of reaches is not a fraction of a turn (register K3). The
  cut's held share (`flows::held`) reads the part of that enclosure within
  the hold; captures and bonds follow.
- **Built on every gathering** (`own-extents`): the beads, the strands, the
  band pairs and the fluid's rings and shells read each mote over its own
  extent `R(1 ± e)` (`e` its pinch share), never over one band `R(1 ± p)`
  for the galaxy. The push `p` no longer enters a width anywhere. The
  shear's payments are held on the grain beside its step: each range pays
  `given·total/whole`, and a neighbour's exact payments from many ranges
  compounded their denominators until one echo would not complete.
  Read on one 2^40 founding over 580 bells (an observation of that run):
  every identity exact; 19 worlds (14 before); 20 systems (30), none stuck
  at its rim (443).

## 8d. The torus found flat, and re-volumized (operator, 2026-11-06)

**Found:** the engine's positions and separations were non-volumetric.
- A body's place was one reach on a radial line plus three independent
  phases, one per plane: a flat 3-torus `S¹ × S¹ × S¹` times a line, never
  a sphere.
- Separations were read plane by plane: the coupling's alignment, the bond's
  in-step product, and `|R_i − R_j|` on reach. On a sphere they are one
  geodesic that couples the planes.

The operator ruled the torus be re-volumized in non-Euclidean spheres:
- **Degrees stay the state** (round rotation angles on the turning planes,
  §8b; the L1 projection a named reading only).
  **Amended 2026-11-10 (docs/117 §11.7, operator: "derive the answer …
  rationale for opening up that vent"):** the stored state is the
  generator `G`, the hyper-dimensional object degrees are a reading of.
  Its components are the round turnings' Cayley rates, `tan(θ/4) = g`, and
  it also holds the turnings between planes and the volume, which degrees
  cannot. Rational degrees have no exact rotor except at `0°` and `180°`
  (78.173 §8.3). Degrees remain every reading, rendered as certified
  sectors.
- **A body's orientation is one rotor on `S³`,** its three turnings
  (`e₊₂e₊₃`, `e₊₃e₊₁`, `e₊₁e₊₂`, cyclic) composed in a fixed order, which does
  not commute.
- **The separation of two bodies is read by isoclinic degrees** (docs/117
  §11.6): on each dual pair, the left and right angles `(θ ± θ′)/2`, and
  their exact differences `(Δα, Δβ)`, with no cosine and no square (the
  operator refused the squared cosine of the relative rotor). Where a law
  needs a value from a certified interval, it acts on the bound the caller
  states, the one nearer zero, and never on a sexagesimal sector (base 60 is
  display only); where the interval straddles zero, it does not act.

**Built so far:** none in the laws. The certified geodesic module is in
progress. The coupling's alignment, its range and the bond test move onto it
in turn, each with a read.

## 9. What is open

1. **G's content.** That a hue move is a change of kind (the place on S¹¹)
   is this document's reading; Mycelium's G lane counts hue moves of
   symbols. Whether G is the kind's bidegree `(a, b, j)` or the kind's
   composition change alone is not derived.
2. **H across three planes:** answered by Xylarium (2026-11-05). `H` holds
   the per-plane phases and the turnings between planes together; a phase
   on one plane's ring is a one-plane reading of `H`, not the whole lane.
3. **The generator's reading from the coordinate.** §2's `b_S`, `b_H`,
   `b_T` are read from the body's coordinate and its enclosure's. Each law
   that sets a generator now (the torque, the band, the push) is to be
   restated as that reading.
4. **The signed lanes against the court's non-negative ones** (§1, §8).
5. **Heat.** Answered by docs/124 §3 (2026-11-10): heat is a reading, the spread of a body's turning about its own step, never a lane. Agitation has no lane here. Whether heat is a reading of the
   coordinate (the spread of a body's lanes) or a lane of its own is the
   same open question docs/118 left (Part I item 5).
6. **The index's own cost** when a body's lanes advance at different rates:
   the order is kept at the bell a reading asks for, so a reading may reorder
   bodies that crossed one another since; the reorder is local (only crossed
   neighbours move) and is not yet stated as a law.
