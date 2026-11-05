# 118 — The foundation, the audit and the trajectory

This document states what Netstratum's era-1 physics is founded on, audits
every law the conductor runs against that foundation, and lays out the
trajectory from the founding to what the universe is for. It exists because
the derivations of docs/114 §9–§10 and docs/117 were each sound locally and
each fitted onto structures the code already had, so every step found
another collapse beneath it (operator 2026-11-05: "you're looking to place
bandaids and do non-substantive work but somehow we have foundations
missing").

**Status:** proposed for the operator's ruling, 2026-11-05. Part I's axioms
are each marked **ruled** (an operator ruling stands behind it), **derived**
(a document derives it) or **proposed** (it needs a ruling). Part II reads
the conductor at main `8e6f3d32` plus PR #56. Part III extends docs/89 (one
story, 2026-10-22), whose breaks B1–B8 are cited, not re-read. Nothing here
changes code.

**How to read it:** Part I is what everything must follow from. Part II is
where the code stands against it, all at once, instead of one collapse at a
time. Part III is where each stage of the universe stands and what it waits
on. Part IV is the order of work that follows.

---

## Part I — The foundation

### F1. Exact, first degree, no scalar collapse — ruled

- Every quantity is an exact rational; nothing is approximate except a
  certified bound around an exact value (docs/78 design laws; the dynamic
  carriers ruling of 2026-10-24).
- Every law is first degree: no square, no cube, no product of a quantity
  with itself (design law 2; operator 2026-11-05, "we don't square or else
  you'll form rigidity"). A product of two different first-degree readings
  (a spin `ℓ = R·H`, a flow-law `1 + p·q`) is allowed; a quantity times
  itself is not.
- No scalar stands for a multivector (the no-scalar law): what is one object
  is carried as one object, never as separate numbers recombined.
- **Size is L1**: the sum of the magnitudes of a state's parts. L1 is what
  makes the spheres below compact whatever the algebra's signature.

### F2. The axes: four hyperbolic planes — derived (docs/117 §10), needs ruling

Eight base axes, in four planes:

| plane | its poles | what it is |
|---|---|---|
| axis 1, 2, 3 | `Z_k` (closed), `Q_k` (open) | a proton's frame on axis k, closed or open |
| R's plane | the universe's origin and infinity | reach: scale between the singularity and infinity |

- **The poles are the null directions** of their plane: `n_Z = e₊ + e₋`,
  `n_Q = e₊ − e₋`. A plane's orthonormal axes are `e₊` (square +1, the
  **whole**) and `e₋` (square −1, the **span**).
- The signature is fixed, (4, 4); which pole is called Z is a labelling the
  state carries (Mycelium review, 2026-11-05).
- It is Xylarium's grammar (one pole axis plane and isoclinic planes), with
  three axes for Xylarium's five.

### F3. The base and the state — derived in part (docs/117 §4, §10), needs ruling

- **A body's point on the machine** is a degree-1 element over the eight
  axes: on each plane, its closed and open counts on the poles,
  `A·n_Z + Q·n_Q = (A + Q)·e₊ + (A − Q)·e₋`, the whole and the span. Its
  L1 sphere is **S⁷**.
- **S⁷ fibres over S⁴** (the quaternionic Hopf fibration): over each base
  direction an S³ of turning.
- **The quantum state** rides on the machine: an even element `q` (S³ at its
  smallest, the even degrees 0, 2, 4, 6 over all eight axes at its fullest).
  **S¹¹ = S⁷ ∗ S³**, the join: a point of S¹¹ is a point of the machine, a
  state, and the share between them. The nuclide table's degrees split over
  the two exactly (witness in `nuclides`).
- **Open:** which of `q`'s components the existing data fixes and which are
  new (Part II shows the code carries almost none of them).

### F4. Motion: one generator, read in first degree — derived (docs/117 §8.2–§8.3)

- A motion is a degree-2 generator `B`: on eight axes, 28 components, each on
  a pair of axes, each with its own size.
- **Velocity is `B`; drag is its contraction** (the degree-lowering part of a
  product). Read from a plane's open pole, velocity is `δ` and drag `1 − δ`,
  the shares the code already carries.
- `B` is read component by component, never by its square: two axes of one
  sign make a turning, of opposite signs a boost; two components on disjoint
  axes are a screw pair; two sharing one axis, with the other two of opposite
  sign and equal size, are null.
- **Within one plane the only motion is its boost** (the dilation). A
  turning stands on two axes of one sign, which are always in two different
  planes.

### F5. Measure and conservation — ruled

- **Matter is counted in whole nuclei** (`pulse::nucleus`); nucleons are
  conserved through every capture, burn, peel and fission (docs/97 §1).
- **The spin ledger closes:** `Σ m·ℓ + reservoir` exact (the torque
  identity); every share a law gives is booked (no leftovers, 2026-10-27).
- **A share is read in its own enclosure** (2026-10-16): a body's deficit
  against the hold of what stands inside it, in that enclosure's units.

### F6. The passage — derived (docs/114 §9, docs/117 §9–§10), formula open

- **The face inversion is the pole swap**: a reflection in `e₊` exchanging
  `n_Z` and `n_Q`, reversing the span. A crossing's sense is the span's sign.
- **The throat is the inversion's fixed sphere** (`A = Q`, span zero): the
  event horizon (operator 2026-11-05, "the inverse across the event
  horizon").
- **The poles are null** (`4AQ = 0`): the singularity and infinity, where a
  passage is one-way.
- **A crossing moves no reach** (LAW-PASSAGE v2, Mycelium's draft clause
  2.11, held): what changes is the state's face and parity.
- **Open:** the passage's formula on the pole basis (docs/117 §10.5 item 3),
  the size of a double pass, and ties.

### F7. The tiers — ruled in form (2026-10-16, 2026-10-18), needs restating

- Universe ⊃ galaxy ⊃ star system ⊃ world: each is an enclosure of the same
  state type, with its own throat and its own units, one grammar coupled
  across tiers (docs/78, not one law at every tier).
- Nesting composes by the flow law `p ⊕ q`, the dilation's composition; a
  tier's clock is the product of the shares between it and the source.
- **Restating owed:** each tier's enclosure on F2's planes, so a tier's
  throat is its R plane's fixed sphere.

### F8. Kinds — ruled (docs/97), with one collapse

- A kind is a nuclide: Q protons, Z neutrons, weight `Q + Z`, converged on
  S¹¹ (docs/97 §2); its place is a bidegree `(a, b, j)` over the machine and
  the state (docs/117 §5).
- R in docs/97 is the electrons' phase, where chemistry and light live.
- **Open:** how a kind's frames part over the three axes. docs/95 §6 proposed
  an even split, which is a collapse: no per-axis content.

### F9. The interfaces — aligned, needs ruling

- **The sensorium** (Mycelium MU-CLIENT §6): what a seated mind receives,
  built by the game. Aligned: the three planes' wholes plus R's plane,
  Cl(4, 1), Mycelium's own value; the spans withheld (docs/117, proposed
  §11). Readings on R belong to the sensorium and never enter identity.
- **The court** (docs/113): only the crossings at the universal core's
  vertex are stated as Mycelium transitions, under LAW-PASSAGE.
- **The hypha's resources**: a universe's manifest, its nuclei per Z,
  granted by the endowment (built in `ns-hypha::refound`).

### F10. One seed, laws that favour no axis — ruled (2026-11-04)

- The seed sets initial conditions only (the founding's orientation,
  `pulse::orient`); no law draws.
- Every law is equivariant under permuting the axes; only the seed tells them
  apart (witnessed by the permuted-seed read, docs/100 §4.1).

### What Part I needs ruled

1. F2 and F3: the four hyperbolic planes with their poles; the state as the
   machine's point plus the riding `q`.
2. F6: the passage's formula on the pole basis.
3. F8: whether frames part over the axes by a law, and which.
4. F9: the sensorium as the projection onto the wholes and R's plane.
5. Heat and energy: the thermodynamic substrate (docs/106) has no place yet
   in F1–F10. Whether agitation is a reading of the state or a component of
   it is not stated.

---

## Part II — The audit

Each law the conductor runs, against Part I. **Follows**: it is a reading or
a law of the foundation as it stands. **Through a collapse**: the law is
sound but reads a state the foundation says is collapsed. **Contradicts**:
it states something Part I refuses. **No basis**: Part I does not found it.

### II.1 The state the code carries

| field | what it is in Part I | verdict |
|---|---|---|
| `Mote.key`, `amount` | the kind and its whole nuclei (F5, F8) | follows |
| `Mote.ell_o`, `ell_orth` | two magnitudes standing for the generator `B` (28 components) | through a collapse |
| `Mote.tilt` | a partition of one over three planes: magnitudes without signs, no component between planes | through a collapse |
| `Mote.phase` (3 planes + R loop) | a circle turned "in" each plane; F4 says a plane holds no turning, so these must be readings of turnings between planes | contradicts F4 |
| `Mote.vacuum` | one boolean for the face, the parity and the R route's sense | through a collapse (three meanings in one bit) |
| `Mote.agitation` | heat | no basis (Part I's open item 5) |
| `Mote.lever`, `lever_place` | the exact carrier's place on the grain (enfold) | follows, as a carrier |
| `Body.kept`, `medium`, `throat` | manifests | follows; frames per axis absent (F8) |
| `Body.ell`, `ell_orth`, `tilt`, `phase`, `bell` | as the mote's | as the mote's |
| `Body.state`, `reservoir` | the core's hold; the ledger's antipole (F5) | follows |
| `Body.heat`, `remnant`, `solid`, `coupled`, `gas`, `turning`, `load` | readings and flags of formation | heat: no basis; the rest follow as records |
| `Universe.t_u` | the stored 186-bell period; the established period is about 5 bells (docs/116) | contradicts (the schedule runs on the stored one) |

**What the state is missing:** each plane's whole and span as its own pair;
the 28 components of motion (the code carries two magnitudes and a
partition); the riding state `q`; R's plane as a plane; frames per axis.

### II.2 The laws

| law | in Part I | verdict |
|---|---|---|
| `ring::{rate, deficit, period, spin, radius}` | shares and readings of one plane's state (F4) | follows |
| `ring::{dilation, compose}` | the dilator on a plane's poles, its composition (F2, F7) | follows |
| `ring::{inverted, tube}` | `h·(h/r)`, `h_a·h_b/r_a`: a reach squared into the hold | contradicts F1, F6 |
| `polarity::{faces, throat_held, equilibrium, after}` | share readings and the two-pole exchange | follows |
| `flows::through` | the reach map `2H − R` | contradicts F6 |
| `flows::mobius`, `antipode` | `(δ, ψ + ½, −s)`: the passage as three scalars | contradicts F1, F6 |
| `flows::luminal` | the bound follows; its flip is of the whole mote | through a collapse |
| `flows::torque`, `turn`, `spin_torque`, `orth_push`, `swung`, `through_centre` | the push's response per plane, by tilt share, on the single face | through a collapse (tilt, phases, `vacuum`) |
| `flows::band`, `toward_rest` | first degree on shares; the vacuum branch reads `through` | follows, except the vacuum branch (contradicts F6) |
| `flows::settle_disc`, `tilt_from`, `blend` | the attractor on tilts; tilt arithmetic (now equivariant) | through a collapse |
| `flows::held`, `read_cut`, `exits`, `Enclosed` | shares in the one enclosure, sorted by whole spin | follows; each plane's own enclosure absent (F5) |
| `flows::{repel, bonded, collections, step_with, in_step}` | phases in step on circles per plane | through a collapse (phases) |
| `flows::{fluid, restoring, around, phase_beads}` | ring exchanges by shares; cells by phase | follows on shares, through a collapse on phases |
| `flows::{band_pairs, beads, strands, merge, hold_along, within_reach}` | the galaxy settling its stars by phases and reach | through a collapse (phases) |
| `flows::{spray, founding}` | matter thrown at capture phases | through a collapse (phases) |
| `breath::{galaxy_rate, nested, system_clock, slip, rungs}` | clocks as products of shares (F7) | follows |
| `breath::coherence` | `1 − 2·|Δ − round Δ|` on a phase | through a collapse |
| `breath::{shine, shine_systems}` | light on the unlocked face (F4 shares) | follows |
| `formation::{shells, collapse, bond, ignition_share, coupling, burn, tunnelling, heart_cut, carried}` | share laws on manifests | follows |
| `formation::{collide_stars, settle_system}`, layers and strikes | meetings by band and hemisphere (phases) | through a collapse (phases) |
| `recycle::*` | exact counts at the throat | follows |
| `ocean::{swing, pump, radial_share}` | a galaxy's swing on the scalar reach | through a collapse (R's plane not carried) |
| `pulse::{ocean_frame, orbit_galaxies}` | the galaxies' orbits, derived | follows, on the scalar reach |
| `nuclides::{place, protons, coherence, survival, captures, exchange, seal_window, grip, heat}` | the convergence on S¹¹ (F8) | follows; the bidegree is not read |
| `nuclides::mobius` | the conversion at a nucleon's pole as a pass | follows F6 once restated as the pole swap |
| `nuclides::electron_phase` | R = `κ_e·(1 − 2·frac(p))` | follows; docs/104 §1.1's correction of `(1 ± R)/2` is owed |
| `pulse::{founding, cascade, deploy, orient}` | the founding and its orientation (F10) | follows |
| `pulse::tier` | the tier shares, imports by its own account (docs/108 O5) | no basis |
| `pulse::{push_of, galaxy_push, home_push}` | the push `½·Q/(A + Q)`, a pole share | follows |
| `pulse::{established_period, contained_front}` | readings | follow; the schedule does not use them (II.1, `t_u`) |
| `enclosure::{route, place}` | routes through the throats (F7) | follows |
| `chem::*` | atoms from S¹¹ windings | follows; two counts of open ends (`chem::unpaired`, `L_e`) agree at 282 of 1495 weights (docs/115) |
| `horizons::read` | the planes' horizons on the orthonormal basis | superseded by F2 (docs/117 §10) |
| `ns-hypha::refound`, the manifest | the court and the endowment (F9) | follows |
| the sensorium | — | not built |
| life, the peoples and the economy | docs/89 B1–B8: life reads its own physics, worlds receive no starlight after birth, the era-0 box under the economy, two laws for one thing | per docs/89 (not re-read here) |

### II.3 What the audit shows

- **One collapse runs through most of the physics: the phases.** Every law
  that meets bodies by being in step (collections, band pairs, collisions,
  coherence, spray) reads circles turned inside planes, which F4 says cannot
  turn. This is the largest single gap.
- **The face is one bit carrying three meanings** (`vacuum`): the face, the
  parity and the R route's sense.
- **The reach maps and the scalar Möbius contradict F6** and are few:
  `through`, `mobius`, `antipode`, `ring::inverted`, `ring::tube`.
- **The share laws follow the foundation as they stand**: the ring law, the
  push, the light, formation's shares, the clocks, recycling, the nuclear
  convergence. Most of the built physics is sound; it reads a collapsed
  state.
- **Heat has no foundation yet**, and the schedule runs on a stored period
  the universe's own reading contradicts.

---

## Part III — The trajectory

The universe from its founding to what it is for, stage by stage. The end it
serves, as the record states it: Netstratum builds the periodic table, the
hypha assigns resources (operator 2026-11-03), and seated minds live in it
through the sensorium (Mycelium MU-CLIENT §6). A stable universe built on
the inverted nucleons is open research.

| stage | what it is | where it stands | waits on |
|---|---|---|---|
| T0 the founding | the table in the universal core's throat; the seed's orientation | built (docs/112, docs/100 §4.1) | — |
| T1 the bounce and release | the core's push releases matter by the tide | built; the release folds matter back through `through` | F6's passage |
| T2 the table | nucleosynthesis, captures, burns, recycling, to the densest element | built on S¹¹ | F8 (frames per axis) |
| T3 the firmament and galaxies | the collective, its collections and order; the galaxies' orbits | built; costly (#5) | the phases (II.3) |
| T4 stars and systems | binding, ignition, the heart, burning, capture, settling | built | the phases; F6 |
| T5 worlds | layers, strikes, chemistry inside a world | built in part | the phases; light to worlds (docs/89 B2) |
| T6 the disc | the planes coming apart; whether a disc forms | the lean grows (docs/100 §4.1); a disc not read | cost (#5); F2–F4 |
| T7 chemistry and bonds | bonds from nucleons and electrons (docs/115) | derived, not built | one count of open ends; F8 |
| T8 light and R | the electrons' phase, the light ledger | built in part (docs/104 §1.1 owed) | F9 |
| T9 life and minds | a world's media, the climb, a mind waking | built on its own physics (docs/89 B1) | T5, T8; the sensorium |
| T10 the sensorium | what a seated mind receives | not built | F9 |
| T11 crossings on the court | the vertex's crossings as Mycelium transitions | specified (docs/113), law drafted (LAW-PASSAGE v2, held) | F6 |
| T12 the hypha's resources | a universe's manifest granted by the endowment | built (`refound`, nuclei per Z) | — |

**Two constraints run through every stage:**

- **Cost** (#5): a bounce takes minutes and grows with the motes; the read
  that decides T6 needs about ten bounces.
- **The schedule** runs on the stored period, not the universe's own (about
  5 bells against 186).

**Where the trajectory is weakest:** T3–T6 all read the phases, so the
largest gap (II.3) sits under the stages that make galaxies, stars, worlds
and the disc. T9–T10 sit on physics of their own (docs/89 B1) and on an
interface not yet built.

---

## Part IV — The order of work that follows

1. **Rule Part I** (its five open items). Nothing below starts before.
2. **The state type**: one carrier for a body's point on the machine (each
   plane's whole and span), its motion (the generator's components) and its
   riding state, replacing the scattered fields in place (no parallel copy,
   2026-10-14). The existing fields become readings of it, so every law in
   II.2 marked *follows* keeps working unchanged.
3. **The phases**, the largest gap: derive which turnings between planes the
   code's phases read (docs/117 §10.5 item 1), then move the laws that meet
   bodies in step onto them.
4. **The face and the passage**: replace `vacuum`, `through`, `mobius`,
   `antipode` and `ring::{inverted, tube}` with F6 on the state.
5. **Each later stage in trajectory order**, each with a read before and
   after on a fresh founding, conservation and every identity exact: T6's
   disc, T7's bonds, T8's light, then T9–T10 on the sensorium.
6. **Cost and the schedule** alongside, since every read depends on them.
