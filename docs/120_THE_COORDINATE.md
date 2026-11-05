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
| `T` | its cumulative null flux: its passages through the throat and at the poles (docs/117 §9.3, §10.1) | exact, signed |
| `H` | its cumulative turning: the turnings between planes the code's phases read (docs/117 §10.3) | exact, signed |
| `K` | the orbits it has closed | a count |

- **The coordinate is the state.** A body's reach is read from `S`, its
  kind from `G`, its face and sheet from `T`, its place on its orbit from
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

READING floors a coordinate on base 60 to depth d (docs/84). The place of a
body at depth d is the first d sexagesimal places of each lane. That is a
path in a tree whose every node has sixty children:

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
  of the encounter (holding-weighted, docs/96), at one origin.
- **Reset is a re-origin across a throat.** A body crossing out of its
  enclosure into the one above is written at the new origin, its lanes read
  across the passage (docs/117 §9.3–§9.4): the null flux `T` records the
  crossing, and no reach moves (LAW-PASSAGE).
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
- **Xylarium's condition:** a lane's advance is a sheared return, never a
  closed repeat, so the form holds each lane exactly and assumes no period.
  §2's advance does: `K` counts H's whole turns; no cycle is assumed.

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
5. **Heat.** Agitation has no lane here. Whether heat is a reading of the
   coordinate (the spread of a body's lanes) or a lane of its own is the
   same open question docs/118 left (Part I item 5).
6. **The index's own cost** when a body's lanes advance at different rates:
   the order is kept at the bell a reading asks for, so a reading may reorder
   bodies that crossed one another since; the reorder is local (only crossed
   neighbours move) and is not yet stated as a law.
