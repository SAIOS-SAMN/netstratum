# 128 — The pass per plane: velocity, pressure resistance, phase and the parcel's boundary

This document corrects docs/126 §2. The strip's half twist `½·t_k` was a
flat plane: `t_k` is the tilt, each plane's L1 share of one summed spin
(planes on a simplex, never on their own circles), and `½` is the throat's
fixed point carried to every pass as a constant. Read per plane, the pass
comes from four parts of the record: the plane's own velocity, the medium's
pressure resisting it, its phase state, and the share of the parcel that
has crossed its boundary. The face becomes per plane and continuous.

**Operator 2026-11-11:** "this is a flat plane — stop your build and derive
the physics as velocity / pressure resistance / phase state / parcel
boundaries: ½·t_k"; then "derive the correction and add
multi-dimensionality".

**Status:** derived. Readings built and witnessed (`conductor::strip`). §8's
order built: motes carry `G`; `strip::PlaneForm`; the torque turning each
plane's pair at its own resisted share. §9 (the two halves on the group)
supersedes §1's per-plane reach and step 3's rates: six spins, one motion a
half composed on S³ × S³, one law for the motion and the book. It is built
next (§9.7), then the faces' readers and the pass (§6, #188).

**Prerequisites:** docs/94 §2 (the transit), docs/120 §8b′ (a plane closes
on itself) and §8c (a mote's own enclosure), docs/126 (the strip), the twist's
pressure (`torsion`), docs/104 §0 (`I`).

## 0. What was flat

- **`t_k` collapses the planes onto one spin.** The tilt reads every plane's
  rate as a share of the summed orbit spin, `2δ·t_k`, with `δ` the deficit at
  the summed reach `R = Σℓ_k/H`. No plane is read on its own circle.
- **The face is one flag per mote** (`Mote::vacuum`), flipped whole by the
  luminal bound: the parcel crosses the ring all at once, on every plane.
- **`½` stands for the crossing.** docs/94 §2 holds the transit `ψ(λ) = ψ +
  λ/2` over the crossing `λ ∈ [0, 1]`; the pass took the whole `λ = 1` in one
  step.

## 1. Velocity: each plane on its own circle

A plane closes on itself (docs/120 §8b′). Its spin `ℓ_k`, read by the ring
law at the enclosure's hold at that spin, gives the plane its own reach and
deficit:

    R_k = ℓ_k / H(<ℓ_k)        δ_k = H(<ℓ_k) / (R_k + H(<ℓ_k))

No tilt share enters. Planes with different spins stand at different reaches
and turn at different rates.

## 2. The parcel's boundary: the face as the share crossed

A mote is a parcel with its own enclosure (docs/120 §8c). On plane k its
extent is `R_k·(1 ± e_k)`, with `e_k = m/(m + H(<ℓ_k))` its pinch share
(`ring::pinch_share`). The ring on plane k stands at `R = H(<ℓ_k)` (`δ_k =
½`). The plane's **face** is the share of the parcel's extent inside the ring,
first degree, on the extent's linear measure:

    λ_k = clamp( (H(<ℓ_k) − R_k(1 − e_k)) / (2·R_k·e_k), 0, 1 )

- `λ_k = 0` is the continuum face (the parcel wholly outside the ring);
  `λ_k = 1` is the vacuum face (wholly inside).
- With no extent (`e_k = 0`), the face is `0` outside the ring, `1` inside and
  `½` on it.
- **The face is a reading, multi-dimensional:** one share per plane, never
  one flag for the mote.

## 3. Phase state: the transit read on each plane

docs/94 §2's transit is the face-reading for any crossing `λ`:

    s_k = δ_k + λ_k·(1 − 2δ_k)        (the plane's share on its face)
    σ_k = 1 − 2·λ_k                   (its sense, flipping at λ_k = ½)

- **The luminal bound holds by construction:** wholly outside, `s_k = δ_k ≤
  ½`; wholly inside, `s_k = 1 − δ_k ≤ ½`. Nothing orbits faster than a turn a
  bell, so no flip event is needed.
- **The half twist emerges per plane.** As the parcel crosses, the plane's
  phase advances by the transit, `Δψ_k = ½·Δλ_k`: half a turn over a whole
  crossing, taken by each plane as its own parcel crosses its own ring. It
  replaces docs/126 §2's `½·t_k`.
- `I` is the face-reading at `λ = 1`: `δ ↦ 1 − δ`, sense flipped (docs/104
  §0).

## 4. Pressure resistance: the medium against the plane

The twist's pressure on plane k (`torsion`, `P_k = P_kj ⊕ P_kl`) resists the
plane's motion by the flow law, the same pressure the release composes and
the vacuum consumes under (docs/126 §3):

    s_k′ = s_k ⊖ P_k        (resisted on the continuum, P reversed past ½: s_k ⊕ P_k)

read with the plane's sense: on the continuum side of its crossing the
pressure resists, past it (the helix reversed) it aids. The pressure is zero
where the planes do not differ (an isotropic twist), so it acts once the
planes part.

## 4a. Drag: the jetstream's counterflow on the traversal

**Operator 2026-11-11:** "there's a missing traversal force of the jetstream
on the continuum/vacuum that is the inverse reciprocal of the forward linear
movement"; the names below agreed the same day.

§4's `P_k` is the twist between planes. It is zero when the planes do not
differ. The medium also resists a parcel by its own motion along the plane,
whatever the planes do: the jetstream's other leg, the vacuum drawn in along
the axis against the continuum's outflow (docs/94 §2). Nature calls this
force **drag**, and calls its source **counterflow**. The record already
names drag: "velocity is the generator; drag is its contraction" (docs/117
§8.2; operator 2026-11-05: "increase to degrees as velocity with inverse as
drag").

**Its measure: the inverse reciprocal is the strip's pass.** A plane's
forward traversal at share `s_k` moves at rate `b_k = s_k/(1 − s_k)` (E57).
The pass `M` (docs/94 §2) turns the face over (`s ↦ 1 − s`), which takes the
rate to its reciprocal, and reverses the sense, which takes it to its
inverse:

    M: b_k ↦ −1/b_k        M∘M: b_k ↦ b_k

So the counterflow's drag is the forward movement read from the closed axis.
It is the same jetstream on the strip's other face, not a second force. Its
share is `d_k = 1 − s_k`, and its ratio to the velocity is the reciprocal
rate, `d_k/s_k = 1/b_k` (Xylarium's pole ratio `t = d/c`). The product
`b_k·(1/b_k) = 1` holds at every reach. No constant enters.

**What it acts on: the parcel's departure from the medium's flow.** A
parcel feels drag only by moving against the medium at its place. The
medium's transit on plane k is the holding-weighted transit of the range
meeting the place (docs/121 §9.2, docs/127 §1), with each mote's sense, so
the counterflow leg's inbound motes read against the outbound. The range is
taken without the parcel:

    s̄_k = Σ_j w_j·σ_j·s_j′ / Σ_j w_j        (j over the range, not i)
    D_k = σ_i·s_i′ ⊖ s̄_k                   (the departure, flow law)

**The law, first degree.** Each ring the drag takes from the departure at
its share `d_k`, by the medium's crowding share of the holding at the place
(docs/94 §5, the back-reaction):

    g_k = W/(W + w_i)        (W the range's holding without i)
    T_k = d_k·g_k·w_i·D_k    (the transit the parcel gives the medium)

The parcel's transit falls by `T_k/w_i`. The range gains `T_k`, shared by
holding. This is the manifold's conformation on the transit (docs/127 §3):
`Σ w·σ·s` is kept, so drag is never a source. A parcel moving with the
medium (`D_k = 0`) feels no drag. An empty medium (`g_k = 0`) leaves the
parcel bare. A medium lighter than the parcel weakens the drag, as dust
crowding stops its drift (docs/94 §5). An outbound parcel in an inbound
stream departs by more than its own share, and so it is dragged harder.

**Order with §4:** the twist's pressure first (`s_k′`), then the drag on
`s_k′`'s departure. The two are distinct: `P_k` comes from the planes'
differences, `T_k` from the legs' motion.

**Open:** the friction's heat. The heat comes from bonds and friction
(operator 2026-11-05). Drag keeps the transit, and what friction turns to
heat is the transit lane's restoration in docs/127 §4. Its keep must be
read first degree: kinetic energy as a square of the share is refused
(docs/78). It is derived with #184.

## 4b. Drag's orthogonal axis: repulsion, the inverse velocity of pressure

**Operator 2026-11-11:** "drag needs an orthogonal axis on repulsion as
inverse velocity of pressure".

§4a reads drag on one axis, the traversal, so it is flat. A jetstream has two
legs on two axes. The continuum flows out in the ring's plane, and the
vacuum is drawn in along the ring's axis (docs/94 §2). The counterflow
therefore also stands orthogonal to the traversal, on the ring's axis `a±`
(§11.1). There the medium does not carry the parcel along: its pressure
pushes back against the parcel's crossing. That push is repulsion, the
restoring force toward the hydrostatic balance (docs/106 S3, docs/110).

**The repulsion's measure: the pass of pressure's velocity.** A pressure
share `P` on the ring moves at the flow law's rate, `b_P = P/(1 − P)` (E57).
Read through the strip's pass, as §4a reads the forward rate:

    M: b_P ↦ −1/b_P        share r = 1 − P, sense reversed

So the repulsion is the inverse velocity of pressure. Its share is `1 − P`
on the closed face, and its sense opposes the crossing. `b_P·(1/b_P) = 1` at
every pressure, and no constant enters. The ring's pressure is the twist's
pressure on its planes, read along its own direction (§9.3), first degree:

    P± = Σ_k |t±_k|·P_k

**What it acts on: the crossing.** The ring's motion on its transit `x =
σ·s′` splits by its tilt (§11.3). It moves along the invariable plane on
`ς(1 − τ)·x`, and it crosses the plane on `τ·x`. The along-part takes
§4a's counterflow. The crossing takes the repulsion, by §4a's law on the
orthogonal axis, with the medium's crossing over the range meeting it:

    x⊥ = τ·x        x̄⊥ = Σ_j w_j·x⊥_j / Σ_j w_j        D⊥ = x⊥ ⊖ x̄⊥
    T⊥ = (1 − P)·g·w·D⊥

- **One law on two axes.** The drag's share is the axis's own velocity
  turned over: `1 − s′` along the traversal, `1 − P` across it
  (`strip::drag` with that axis's share).
- **Never a source:** the exchange keeps `Σ w·x⊥` (§4a's exchange on the
  crossing), so repulsion moves crossing between the parcel and the medium,
  and none is made.
- **What it does:** a ring crossing faster than the medium loses its
  crossing to it, so the medium's rings settle toward the invariable plane.
  That is the disc's flattening, read from pressure, not imposed.
- **Its limits:** an untilted ring (`τ = 0`) has no crossing and feels no
  repulsion. Pressure at the luminal bound (`P → 1`) turns at full velocity,
  and its inverse falls to nothing. At `P = ½` the pressure's rate is `1`
  and is its own inverse, the fixed point. With no pressure the repulsion
  takes the whole departure, as §4a's drag does on a parcel at rest.
- **Order:** the twist's pressure resists the traversal first (§4, `s′`),
  then the along-drag (§4a) and the repulsion (here) read `s′`.

## 5. Every face-reading reads its plane's face

Each law that names a face reads it per plane, at `λ_k`, through the transit:
- the torque's share `d_o` becomes `s_k′`;
- emission's held face is `faces_on` at `s_k`, per plane;
- the shear's dipole inverts by `σ_k`;
- the magnet's frames, by the plane's face;
- the push's antipole, by the face of the plane it acts on;
- the luminal bound is §3's identity, with no flip.

What names no face is unchanged: turning on its own leg, phases, in-step and
heat (docs/104 §0).

## 6. Build order (one change each, a founding read each)

1. **The readings** (`conductor::strip`): `δ_k`, `e_k`, `λ_k`, `s_k`, `σ_k`,
   `s_k′`, with their witnesses. Built.
2. **The torque on `s_k′`** per plane, retiring the tilt's `2δ·t_k`.
3. **The faces' readers** (emission, shear, magnet, push) on `λ_k`.
4. **The pass retired as an event:** `luminal` and `mobius` become the
   transit's readings, the phase advancing `½·Δλ_k`; `Mote::vacuum` stays only
   as the stream's kind (where it entered), never the face.
5. **docs/126 §2's `½·t_k` reverted** in the same change as step 4.
6. **Drag** (§4a): the readings `strip::drag` (`d_k`, `1/b_k`, `s̄_k`,
   `D_k`, `T_k`) with their witnesses, then the exchange in the torque after
   `s_k′`.

## 8. The plane's form: the dimension attached, without collapse

**Operator 2026-11-11:** "plane needs a dimension attached to it with form:
how do we represent that without collapse"; then "derive it and build in
that order".

A plane's share `s_k` is a number standing in for a two-dimensional form:
the same collapse one level down. The record already gives each plane its
attached dimension in three places, and together they are its form.

1. **Its turning pair** (docs/117 §11.6, §11.7). Every spatial plane has its
   R partner, its circulation: `e₂₃ ↔ e₁R`, `e₃₁ ↔ e₂R`, `e₁₂ ↔ e₃R`, the dual
   pairs of S³. Spin(4) is left × right isoclinic, so a pair is held exactly
   as two degrees, with no square and no cosine:
   `α_k = (θ_k + θ_kR)/2`, `β_k = (θ_k − θ_kR)/2`. In the orientation's
   generator `G` (docs/117 §11.7, `conductor::orientation`) the pair is its
   two components `(g_k, g_kR)`, each with its winding `K`. On Xylarium's join
   (#183) it is the axis share against the isoclinic share.
2. **Its poles on its own axis** (docs/95 §0): the neutrons' held dimensions
   `3Z_k` and the protons' open dimensions `3Q_k` on axis k, the plane's
   nucleon dipole. An atom names no axis, so a nuclide's dimensions part
   evenly over the three axes; each plane carries `3Z/3` and `3Q/3`.
3. **Its radial dimension on S** (§1–§2): its own reach `R_k`, its parcel's
   extent `e_k`, and its face `λ_k`.

So a plane's form is one record per plane:

    PlaneForm_k = { turning: (g_k, g_kR),  poles: (3Z_k, 3Q_k),  reach: (R_k, e_k, λ_k) }

Nothing in it is a share. `s_k`, `σ_k` and `P_k` are readings of it, taken
where a law needs a velocity share for the flow law (a share is the flow
law's own carrier) and never stored back.

**The laws on the form:**
- **The torque turns the pair.** Its push composes `(g_k, g_kR)` together
  through the generator, so the circulation moves with its plane, never
  beside it. `s_k′` sets only the rate of that composition.
- **The pressure is read from the forms' differences.** Each pair's twist is
  the plane forms' `(α_j, β_j)` against `(α_k, β_k)`, so the twist presses
  once the planes' pairs differ. This is the asymmetry #157 lacked.
- **The face is the plane's,** from its reach and extent (§2); the pass is
  the transit on each plane (§3).

**Build order:**
1. Motes carry the orientation `G`, the six components with their windings,
   in place of per-plane phases (the #167 work, here on the per-plane
   velocity and face, never on the tilt).
2. `strip::PlaneForm`, read from a mote: its pair from `G`, its poles from
   `chem::dimensions`, its reach and face from `strip::plane_face`.
3. The torque composing on each plane's pair at `s_k′`, retiring `2δ·t_k`.

## 9. The two halves on the group: six spins, two motions, one law

**Operator 2026-11-11:** "is step 2 condensing what should be uncompacted
space into two planes which is just two dimensions in costume, or is there a
deeper issue that needs to be unfolded"; then "are you writing this in
non-euclidean geometric space"; then "rewrite entirely in non-euclidean
space — then build".

**Status:** derived; supersedes §1's per-plane reach and §8 step 3's rates.
Built in the order of §9.7.

### 9.0 The space

A body's turning state is a point of the group, not of a flat chart. The
orientation `G` is a rotor of `Cl(4,0)`'s even algebra, the direct sum of
two quaternion algebras (`P± = (1 ± I)/2`, docs/117 §11.6), so a state is a
pair of unit quaternions `(q+, q−)` on S³ × S³, one cover (`R ~ −R`). It is
curved and does not commute: two motions composed in the other order give
another state.

What stands on the group:
- **Composition is the algebra's product.** A motion is applied as `q± ↦
  q±·m±` on each half (`orientation::Motion::then`), never by adding angles.
- **The chart is Cayley's.** A motion is written as a pure quaternion `u`
  per half and lifted as `m = (1 + u)(1 − u)⁻¹` (`Half::motion`). Its
  components are the Cayley rates on the half's three units.
- **Magnitudes read first degree (design law 2).** A pure quaternion's size
  is read in the L1 metric, `|u|₁ = Σ_k |u_k|` (docs/120 §8d). Its square,
  the round norm, never states a law. The algebra's own product uses it
  only inside composition, which is the group itself.

### 9.1 What was collapsed, and what was flat

- **The state.** A mote holds `spins`, three spatial spins, and `ell_orth`,
  one number for the whole R side: four spin numbers for the six planes of
  `G`.
- **The bell.** `Orientation::bell` turned the circulation "on each R-plane
  alike", so `e₁R`, `e₂R` and `e₃R` never parted. The halves had nothing to
  part them, which is #157.
- **The rate.** `turning_of` read one `δ` from the summed reach: the spatial
  planes `2δ·t_k` and the loop `2δ` plus the push. A motion of S³ × S³ has
  two (one per half), and they were tied to one number.
- **The chart, flat.** The bell took each plane's share of a turn through
  `rate()` on its own (`signed_rate(Δθ_k/2)`), a rate per component. On the
  group the rate belongs to the half's motion as one vector. A component's
  rate of a share is the flat chart applied plane by plane.
- **§1's unfolding.** It read the three spatial planes as three orbits, each
  at its own reach `R_k < R`, so `δ_k > δ`. It copied those rates onto the
  still-collapsed partners. The phases advanced near `6δ` a bell while
  `spin_torque` booked at `δ`. One read: at the echo of bell 558 from one
  checkpoint, one system of 1,535 bodies where step 2 forms three of 63.

### 9.2 Six spins

A mote's spin is a bivector, an element of the algebra the group turns by,
with one component on each plane of `G`: `ℓ_k` on spatial plane k and
`ℓ_kR` on its partner `e_kR`. No two planes share a slot. `ell_orth`
becomes the three partner spins, its sum the loop spin. A loop spin carried
alone (a record before the partners) goes onto them by the tilt, `ℓ_kR =
t_k·ℓ_⊥`, the swing's split (docs/120 §8b′). A law that changes the loop
spin as a whole keeps it on the partners by their own shares `ℓ_kR/Σ_j ℓ_jR`,
or by the tilt where they sum to nothing.

### 9.3 Each half's spin, reach and rate

The halves of the spin, as the orientation splits a generator
(`orientation::halves`; `e₁R = −e₂₃I`, `e₂R = −e₃₁I`, `e₃R = −e₁₂I`, so on
`P+` a partner enters with its sign reversed):

    ℓ+_k = ℓ_k − ℓ_kR        ℓ−_k = ℓ_k + ℓ_kR        (pure quaternions)

Each half is one spin with its own size, reach, deficit and rate, the ring
law read first degree:

    ℓ± = |ℓ±|₁ = Σ_k |ℓ±_k|        R± = ℓ± / H(<ℓ±)
    δ± = H(<ℓ±) / (R± + H(<ℓ±))     b± = δ±/(1 − δ±) = H(<ℓ±)/R±

and its direction is the spin's own, on the L1 sphere:

    t±_k = ℓ±_k / ℓ±        Σ_k |t±_k| = 1

A half with no spin does not move.

### 9.4 The motion: one vector a half, composed on the group

The ring's motion on each half is the Cayley vector of its rate along its
direction:

    u±_k = b±·t±_k        m± = (1 + u±)(1 − u±)⁻¹

The bell is two rings, composed on each half:

    q± ↦ q±·m±·m±

so `G` takes the pair `(m+², m−²)`. Fed to the orientation as a generator on
the six planes, by the halves' inverse (`joined`): `b_k = (u+_k + u−_k)/2`,
`b_kR = (u−_k − u+_k)/2`, then `advance_by(b, 2)`. The windings count the
planes' passes as now.

- **No partner spin.** `ℓ+ = ℓ− = ℓ`, so the halves move alike, `b_kR = 0`
  and `b_k = b·t_k`. Equal halves are a rotation of the three spatial
  axes alone (the diagonal SO(3)), with R held: the tilt's motion at the
  body's rate, as one vector rather than a rate per plane.
- **The halves part** once the partners hold spin: `ℓ+ ≠ ℓ−`, two reaches,
  two rates and two directions, the double rotation.
- **Nothing over-turns.** Each half moves by one rate `b±` a ring, never by
  one rate a plane.
- **The swing's response retires.** `ℓ_k + t_k·ℓ_⊥·response(ψ_kR)` put the
  partner's phase into the reach. The halves carry the partner spin into
  the reach directly (`ℓ±`), and R stays the axis plane's phase in the
  turning, never a factor in a share (Xylarium #183).

### 9.5 The R side: orbits, never a loop

**Operator 2026-11-11:** "there should be no loop — there should be orbits
with shear from the manifold produced by gravity and physics".

The partner planes hold orbits like every plane. The parent's push laid
"along the orthogonal orbit" (`orth_push`, `κ_⊥`) is a driven loop and
retires. What turns the R side is the twist of the unitary morphodynamic
state, sourced by the manifold's shear (§10).

### 9.6 One law for the motion and the book

**Built (`flows::ring_motions`, read by `flows::torque` at each bell's
start):** each half moves at its ring's transit after the pressure and the
drag on both axes (§4, §4a, §4b, §11), read for the whole manifold before
any mote turns, the exchange with the range on the grain. Each half reads
its transit on the face it stands on: the face carries the inversion (§3,
`I` is the face-reading at `λ = 1`), so `Mote::vacuum` names the stream and
no longer sets its deficit, and no transit passes ½. A ring wholly inside
its ring reads `1 − δ` and one wholly outside `δ`, whichever stream it
rides; one straddling it crosses continuously. The ring's pressure is the
manifold's own twist over the motes it moves, `ρ` the enclosure's base over
its whole.

**Corrected (the ablation probe, `observe ablate`, 2026-11-11).** The first
build inverted the vacuum stream's face, `λ ↦ 1 − λ`. By the transit's
identity `s(δ, 1 − λ) = 1 − s(δ, λ)` that turns each share to its
complement: the disc's vacuum motes, which stand inside their rings, read
`δ > ½`, past the luminal bound, with rates up to ~900 against the old
motion's ~1. The face alone gives the old motion on every half standing on
its own side.

The book takes each half's torque at that half's `δ±`, the same deficit its
motion reads. The face (§2), the transit (§3), the pressure (§4) and the
drag (§4a) are read per half, at `R±` and the parcel's extent there, in place
of `R_k`. A plane's own reading is the orientation's reading of `G`, never a
rate of its own. No law reads a third `δ`.

### 9.7 Build order

1. **The mote carries its six spins** (§9.2). The save keeps the partners'
   shares; an older record loads by the tilt.
2. **The halves as readings** (`orientation`): `ℓ±`, `R±`, `δ±`, `b±`, `t±`,
   `u±`, with their witnesses.
3. **The bell on the halves** (§9.4): `G ← G·(m+², m−²)`. This retires §8 step
   3's `2·s_k′`, `turning_of`'s `2δ·t_k` and circulation, and the bell's
   rate per component.
4. **`spin_torque` at each half's `δ±`,** then the face, transit and drag
   per half.

From step 3 on, each change is read as bell 558 from the checkpoint against
the same bell under step 2 (`.testtmp/wt2`), with these counters:
- `Σ m·ℓ` given against the push;
- each half's motion against its `δ±`;
- the systems and their bodies.

### 9.8 Witnesses

- **The group, not the chart.** Two rings composed give `m·m`, which is not
  the motion of the doubled rate (`(1 + 2u)(1 − 2u)⁻¹`); the bell is the
  product.
- **No partner spin** moves both halves alike, and the partners not at all.
- **The halves part:** partner spin gives `b+ ≠ b−` and partners turning.
- **The ring is the fixed point:** `R± = H` gives `b± = 1`.
- **The inversion:** `δ ↦ 1 − δ` gives `b ↦ 1/b` on each half (E57).
- **Exchange of halves:** reversing every partner spin exchanges `+` and
  `−`. The six-plane generator keeps its spatial components and reverses
  its partners.
- **One law:** the deficit the book reads for each half is the one its
  motion reads.

## 10. The unitary morphodynamic state: automorphism and isoclinic

**Operator 2026-11-11:** "add automorphism to isoclinic — and derive from
that point as a unitary morphodynamic state".

**Status:** derived. Steps 1–2 built: the form and twist as readings
(`Orientation::form_twist`), and the partner orbits conformed to the
manifold's shear with the driven loop retired (`flows::conform_partners`).
Step 3 built: a core's twist pressure is read from the manifold's twist
over its disc's motes (`torsion::manifold_twist`, each mote's `C` generator
as a share on the units, holding-weighted); the wound plane stays the
release's lead. Bodies carry no `G` yet (#159), so a body's own twist and the
fibre's tension (`torsion::tension`, a system's tilt against its core's)
follow with it. Step 4's law built: the book reads the passage each
spatial plane took on the group this bell (its reading's change with its
windings'), so `spin_torque`'s torque `p·(Δψ_k/2)·resp_k` and its heat
`p·(Σ|Δψ_k|/2)·(1 − |k|)` read the motion itself; the old `2δ·t_k` was that
passage under the tilt's turning, so the law's form is kept. The reach
reads the form's orbit (the spatial spin), and the swing's response
retires. The face, transit, pressure and drag per half are built (§9.6,
`flows::ring_motions`), with the ring's tilt (§11) and drag's orthogonal axis
(§4b).

### 10.0 The factorization

A rotation of S³, the continuum's four axes `e₊₁, e₊₂, e₊₃, e₊R` held as the
quaternions with `e₊R` the real unit, is `x ↦ p·x·q̄`, with `p, q` unit
quaternions, the two halves (`P±`). Two kinds of motion stand inside it:

- **The automorphisms** of the quaternions are the conjugations `x ↦ a·x·ā`.
  They fix the real unit, R, and rotate the three spatial units. They are
  the equal halves, `p = q`: §9.4's "no partner spin", the three spatial
  axes turned with R held.
- **The isoclinic motions** multiply on one side, `x ↦ c·x`. They turn both
  planes of every dual pair (`e₂₃, e₁R`), (`e₃₁, e₂R`), (`e₁₂, e₃R`) by one
  angle (docs/117 §11.6; Xylarium T18). They are the turnings that involve
  R.

Every motion factors uniquely into the two, exactly:

    p·x·q̄ = c·(q·x·q̄),    c = p·q̄

### 10.1 The state

A body's turning state is the pair of unit quaternions:

    F = q        its **form**: the automorphism, its orientation in space, R held
    C = p·q̄      its **twist**: the isoclinic turning against R

It is the same point as `G`'s halves, `(p, q) = (C·F, F)`, one cover, with
`p` the `P+` half and `q` the `P−` half of `orientation::Motion`. The
factorization is algebraic in the pair, so it holds for either labelling; the
labelling is fixed by the face (below). Both
factors are unitary (unit quaternions, `F·F̄ = C·C̄ = 1`), so the state keeps
its measure under every motion, and an inverse is a conjugate: no norm is
read, and no root is taken.

- **Morphodynamic.** `F` is the form a body holds, the orientation of its
  axes and its orbits in space. `C` is how that form stands deformed against
  R. `C = 1` is a body in pure orbit, undeformed. `C ≠ 1` is the manifold's
  twist at the body, the deformation docs/127 §2 names on the H lane
  ("twist", the quantum torsion), now one unitary factor, not a scalar per
  pair.
- **The face chooses the chirality.** The continuum carries its form on one
  half and its twist as the other's motion relative to it. The vacuum, the
  continuum inverted (docs/104 §0), exchanges the halves: form on the other
  half, twist `C̄`. Reversing every partner spin exchanges the halves
  (§9.8), so the inversion is the exchange of chirality, and #241's "the
  inversion is the twist" is this exchange read on `C`.

### 10.2 One bell from that point

Each half moves by its own motion (§9.4), `p ↦ p·m+`, `q ↦ q·m−`, with
`m± = (1 + u±)(1 − u±)⁻¹` twice a bell. On the state:

    F ↦ F·m−
    C ↦ p·m+·m̄−·q̄ = C·F·(m+·m̄−)·F̄

- **The form moves by the automorphism part of the motion,** `m−`, its
  orbit.
- **The twist moves by the halves' relative motion,** `m+·m̄−`, carried into
  the body's own axes by its form (conjugation by `F`). It is measured
  against the body, not against a fixed axis. Equal halves (`m+ = m−`) leave
  `C` unchanged: a body in pure orbit keeps its twist.
- **Exact:** products of the integer halves (`orientation::Half`) and their
  conjugates.

### 10.3 What sources the twist: the manifold's shear

The halves differ by the partner spins (`ℓ+ − ℓ− = −2·ℓ_R`, §9.3). So the
twist's motion is sourced by what gives the partners spin. That is the
manifold's shear, never a push:

- **Gravity's differential motion across a parcel.** The ring law's speed at
  reach `r` is the hold there, `b·r = H(<r)`. A parcel spanning `R(1 ± e)`
  (docs/120 §8c) has edges moving by different amounts. The difference is
  the hold enclosed across its own extent:

      ΔH = H(<R(1 + e)) − H(<R(1 − e))

  It is first degree and exact, with no rate taken and no square.
- **The shear deforms the parcel's form against R.** A difference of the
  motion on plane k across the reach turns the plane k with its R partner,
  the dual pair: an isoclinic increment, the twist. The shear's share on
  plane k is the half's own direction there, `t±_k` (§9.3).
- **The shear's rate (built, `flows::shear_rate`).** A rigidly turning
  chord's edges part by `2e·H` (it turns at `b = H/R`); the shear is the
  departure from it:

      σ = (2e·H − ΔH) / (2e·R)

  `b` where nothing stands across the parcel, zero where the hold grows in
  step with the reach (rigid), zero with no extent. `σ` is itself the
  departure from rigid, so `σ`, not `σ − b`, sources the partners (the
  coordinator's ruling 2 read `σ − b`, which would give a rigid parcel
  partner spin `−b`; its intent, rigid gives none, is `σ`).
- **The target (built, `flows::partner_target`).** The halves' rates split
  about the orbit's own, `b± = b ± σ/2`, so their mean is the spatial orbit
  and their difference the shear (the vacuum's halves exchanged, `σ ↦ −σ`).
  Each half's spin is the one standing at its rate's reach,
  `ℓ±* = R±*·H(<R±*)` with `b±·R±* = H(<R±*)` (`Enclosed::reach_at_rate`,
  the inverse through the hold's steps). The partners take half the
  difference along the tilt: `ℓ_kR* = t_k·(ℓ−* − ℓ+*)/2`.
- **Conformation (built, `flows::conform_partners`).** Each mote's partner
  spins relax toward the target by the attractor's keep at its deficit
  (E38), `ℓ_kR′ = ℓ* + k·(ℓ_kR − ℓ*)`; the range meeting it on reach pays
  the change by holding, so `Σ w·ℓ_kR` stands on every partner plane but
  for the grain's remainder (returned, as the shear's). The shear is never
  a source. The twist `C` then moves by the halves those spins give
  (§10.2).
- **Restoration and dissipation.** The friction's heat from the
  conformation is open with §4a's (#184): spin is kept, and what of the
  relaxation becomes heat must be read first degree before it enters the
  manifold's dissipation (E62).

### 10.4 What the laws read

- **Orbits and the reach** read the form `F`: the spatial orientation, the
  tilt, the orbit planes.
- **The torsion, the face's twist and the pressure** read the twist `C`. Its
  generator is equal on each dual pair (isoclinic), so the pressure between
  planes is read from `C`'s components on the three units. This is §4's
  `P_k`, now from the unitary twist.
- **Heat** names no face and is unchanged (docs/104 §0).

### 10.5 What retires

- the driven loop on the R side (`orth_push`, `κ_⊥`, §9.5);
- the twist as a scalar per pair of planes (`torsion` pressures `ρ·(x_j ⊖
  x_k)` on phase readings), replaced by the unitary twist `C`;
- "loop" and "circulation" as names for the turning that involves R: it is
  the isoclinic twist, carried on the partner orbits.

### 10.6 Witnesses

- **The factorization is exact:** `(C·F)·x·F̄ = p·x·q̄` for every `x`, and `(p,
  q) ↦ (F, C) ↦ (p, q)` is the identity.
- **Unitary:** `F·F̄ = C·C̄ = 1` after any bell, exact before the grain.
- **Pure orbit:** equal halves leave `C` unchanged and move `F`.
- **The inversion is the exchange:** exchanging the halves takes `C ↦ C̄`.
- **Shear is never a source:** `Σ w·ℓ_kR` kept by conformation; a parcel with
  no extent (`e = 0`) has no shear.
- **Restoration closes the book:** what the twist's restoration removes
  appears as the manifold's dissipation (E62).

### 10.7 Build order

1. **The state's two factors as readings** (`orientation`): `F`, `C` from
   `G`'s halves, and back, with the factorization and unitary witnesses.
2. **The twist sourced by the shear** (§10.3), retiring `orth_push` and `κ_⊥`.
   The partner spins are conformed to the manifold's shear and restored
   into the dissipation.
3. **`torsion` from `C`** (§10.4), retiring the per-pair pressures on phase
   readings.
4. **§9.7 step 4 on the factors:** `spin_torque` at `δ±`, the face, transit
   and drag per half.

## 11. The ring: volume, axis and tilt

**Operator 2026-11-11:** "your reference to a 'ring' needs volume, axis, and
tilt".

**Status:** derived; the readings are built (`strip::Ring`,
`strip::half_rings`, `strip::invariable_axes`, `flows::ring_volume`,
`orientation::tilt`, `Orientation::half_axes`) with §11.5's witnesses. The
motion already turns about the axis by the rate; the face, transit,
pressure and drag per half read the rest with §9.6.

§9.4 moved each half "a ring" by a rate along a direction. That treats the
ring as a circle with a speed, which is flat. A ring is the solid a half's
motion sweeps on S³. An isoclinic motion's orbits are great circles that
fibre S³ (the Hopf fibration). The fibres through a parcel's extent fill a
solid torus. At `δ = ½` (`R = H`), the Clifford torus splits S³ into two
solid tori, and the inversion `I` exchanges them (docs/104 §0, §10.1). So a
ring has three readings, each on its half and each exact.

### 11.1 Axis

The half's spin in the body's own axes is `ℓ±` (§9.3). Its motion `m±`
composes on the right (`h± ↦ h±·m±`, §9.4), so in space it turns about the
spin carried by its half:

    a+ = p·ℓ+·p̄        a− = q·ℓ−·q̄

Each is a conjugation by a unit, so it is exact and takes no norm. The axis is
fixed by its own motion: `m±` commutes with `ℓ±`, so `h±·m±·ℓ±·m̄±·h̄± =
h±·ℓ±·h̄±`. Equal halves (`C = 1`, no partner spin) give one axis on both,
which is the spatial orbit's.

### 11.2 Volume

The ring's tube is the parcel's extent about its reach, `R±(1 ± e±)`, with
`e± = m/(m + H(<ℓ±))` the pinch share at the half's hold (§2,
`ring::pinch_share`). Design law 2 refuses a volume taken as a squared or
cubed length. A tube is therefore read by two first-degree measures:

- **Its width,** `2·R±·e±`. The face `λ±` is the share of the width standing
  inside the ring `R = H` (§2's law on the half), and the transit and sense
  follow from it (§3).
- **Its content,** the holding inside the tube, the manifold's own measure of
  the volume it encloses:

      V± = H(<R±(1 + e±)) − H(<R±(1 − e±))

  This is §10.3's `ΔH`. The shear reads the ring's content against the
  holding a rigid tube of that width would enclose, so it is this ring's
  reading (`σ± = (2e±·H − V±)/(2e±·R±)`), not a law of its own.

With no extent, the ring is a circle: no width, no content, no shear, and the
face is `0`, `½` or `1` by place.

### 11.3 Tilt

A ring tilts against the plane of the manifold's own turning. In nature this
is the invariable plane: the plane normal to the total angular momentum of
everything enclosed, the plane inclinations are measured from. On each half
it is the holding-weighted sum of the rings' axes in space:

    A± = Σ_i w_i·a±_i

It is conserved wherever the spins are kept (`Σ w·ℓ` stands under
conformation and the torque's exchange), so it is the manifold's reference,
not a fixed axis. The tilt of a ring is read from the product of its axis
with that reference on the half, `Q = A±·ā±`. For pure quaternions, the
scalar part of `Q` is `A·a`, the axis along the reference, and its vector
part is `A×a`, the line of nodes. First degree, as every share in the model
(`v/(1 + |v|₁)`):

    τ± = |A×a|₁ / (|A·a| + |A×a|₁)     its inclination share, in [0, 1]
    ς± = sign(A·a)                     its sense: +1 prograde, −1 retrograde
    n± = A×a                           its line of nodes

- `τ = 0` is a ring in the invariable plane, prograde (`ς = +1`) or
  retrograde (`ς = −1`). `τ = 1` is a polar ring (`A·a = 0`).
- It is scale-free: a ring's tilt does not change with its spin's size or
  the manifold's.
- A lone ring is its own reference: `τ = 0`, `ς = +1`.

### 11.4 What reads the ring

- **The motion** reads its axis and rate (§9.4): it turns the tube about
  `a±` by `b±`, one ring.
- **The face and transit** read its width (§2–§3, per half: §9.6).
- **The shear** reads its content (§10.3).
- **The pressure and the drag** read its tilt. The medium's transit lies in
  the invariable plane. A ring meets the medium along that plane on the share
  `1 − τ` of its motion, with its sense `ς`, and crosses it on the share `τ`.
  The along-part takes the counterflow (§4a). The crossing, on the ring's
  axis, takes the repulsion at the inverse velocity of the ring's pressure
  (§4b). These enter with §9.6's pipeline.

### 11.5 Witnesses

- **The axis is fixed by its own motion:** a half moved about its own spin
  keeps `a±`. Equal halves give one axis.
- **Content and shear are one reading:** `V = 0` gives `σ = b`, and no extent
  gives no width, no content and no shear.
- **Tilt limits:** an aligned ring reads `τ = 0, ς = +1`, a reversed ring
  `τ = 0, ς = −1`, and a perpendicular ring `τ = 1`. Scaling either axis
  leaves the tilt unchanged.
- **A lone ring** is untilted against its own invariable axis, within the
  grain its terms stand on.

## 12. The ring in every dimension: capacity, volume, and shear out of the plane

**Operator 2026-11-11:** "open the vents on rings in multi-dimensional
capacity with volume and derive shear for orbital pathways outside of
planar view".

§11 read a ring at one reach with one tube. Two planar collapses remain.
The shear and the partner target were read once, at the orbit's summed
reach, and laid onto the planes by the tilt's shares. The disc formed by
pulling every tilt toward one global plane (`settle_disc`), a share vector
relaxed toward a blend, with no shear in it. This section opens both.

### 12.1 Capacity: each plane its own ring

Each isoclinic plane `p`, the dual pair `(e_k, e_kR)`, stands on its own
circle (§1). Its spin `ℓ_k` is read by the ring law, lifted, at its own
reach `r_k`, with hold `H_k = H(<r_k)`. Its ring has:

- **its capacity** `W_k = 3W·α_k`, the plane's first-degree hold share
  (LAW-COUPLING v2, docs/129 §1.2, `α_k` its spin's part on the pair);
- **its tube** `r_k(1 ± e_k)`, with `e_k = m/(m + H_k)` its pinch share at
  that plane's hold;
- **its volume,** the holding inside the tube, `V_k = H(<r_k(1 + e_k)) −
  H(<r_k(1 − e_k))` (§11.2 per plane);
- **its shear across its own reach,** `σ_k = (2e_k·H_k − V_k)/(2e_k·r_k)`.

The capacity is read per face and never summed (Mycelium 78.181 §7,
Xylarium #197): a body's three rings are three readings, not one.

### 12.2 In-plane shear per plane: the partner orbits

The partner target (§10.3) is read on each plane at its own rate,
`b_k = H_k/r_k`, split about it by its own shear, `b±_k = b_k ± σ_k/2` (the
vacuum's halves exchanged). Each half's spin stands at its rate's reach,
`ℓ±*_k = R±*_k·H(<R±*_k)` with `b±_k·R±*_k = H(<R±*_k)`, and the partner
takes half the difference with the plane's own sense:

    ℓ_kR* = sign(ℓ_k)·(ℓ−*_k − ℓ+*_k)/2

This retires `t_k·(ℓ−* − ℓ+*)/2`, the orbit's one shear laid onto the planes
by the tilt. A plane with no spin has no ring and no partner target. The
conformation is §10.3's unchanged.

### 12.3 Shear out of the plane: rings against the rings their tubes meet

A ring's tube meets the rings of the range around it (the motes whose
extents meet its own, `meeting`). Where their axes differ, the manifold
turns about different axes across the tube's volume. That is a shear out
of the ring's plane: the warp of a disc, which in nature damps inclination
and carries the disc to one plane by contact, never by a global
attractor.

- **The volume's axis.** The holding-weighted spin of what the ring's tube
  holds: the ring itself and the range meeting it, `Ā_i = w_i·ℓ_i + Σ_j
  w_j·ℓ_j`, the spins read on the planes they are booked on. A heavy ring
  among light ones is its volume's axis, and barely moves.
- **The shear's reading.** The product of the ring's axis with its volume's,
  `Q = Ā·ℓ̄`, as §11.3: its inclination share against what it meets,
  `τ_i = |Ā×ℓ|₁/(|Ā·ℓ| + |Ā×ℓ|₁)`, its sense `ς_i = sign(Ā·ℓ)`, its line of
  nodes `Ā×ℓ`. The out-of-plane shear rate is the ring's rate on that
  share, `σ⊥_i = τ_i·b_i`, zero for a ring coaxial with its volume.
- **What it does.** The shear conforms the ring's axis toward its volume's,
  keeping its sense and its size: the target `ℓ*_i = ς_i·|ℓ_i|₁·Ā_i/|Ā_i|₁`,
  relaxed by the attractor's keep at its deficit (E38), `ℓ_k′ = ℓ*_k +
  k·(ℓ_k − ℓ*_k)`. The range pays the change by holding on every plane, so
  `Σ w·ℓ_k` stands per plane but for the grain's remainder. The shear is
  never a source.
- **Its limits.** A lone ring is its own volume's axis and is not moved, nor
  is a coaxial one. A ring at right angles to its volume's axis (`ς = 0`,
  `Ā·ℓ = 0`, possible only where the range cancels the ring's own part)
  stands at the unstable fixed point and is not moved. Two equal rings at a
  right angle each hold both in their tubes and turn toward their diagonal.
  A retrograde ring aligns antiparallel and stays retrograde: shear does not
  flip a sense.
- **What it replaces.** `settle_disc`'s pull of every tilt toward the
  galaxy's blended plane. The galaxy's plane stays a reading (`blend`), the
  core's lean. Since the range pays, `Σ w·ℓ_k` per plane is kept inside the
  disc, so the core's antipole takes nothing from it. The disc's angular
  momentum is held by the disc itself.
- **The heat** of the out-of-plane shear is open with §4a's and §10.3's
  (#184).

### 12.4 Witnesses

- **Per-plane capacities sum to the whole:** `Σ W_k = 3W`. A plane with no
  spin has no ring.
- **Per-plane partners:** a body whose planes stand at different reaches
  takes a different partner target on each, never one laid by its tilt.
- **Shear out of the plane is never a source:** `Σ w·ℓ_k` is kept per plane
  within the grain.
- **It damps inclination:** a ring tilted against the rings it meets ends
  the bell less tilted against them. A coaxial ring and a lone ring do not
  move, equal rings at a right angle meet between, and a retrograde ring
  keeps its sense.

## 13. Tilt from quantum torsion: the axial rotations

**Operator 2026-11-11** (the derivation approved, with its four rulings):
the ring had two tilt dampers (§4b, §12.3) and no tilt generator. Its tilt
from quantum torsion (docs/100 §8, Einstein–Cartan: torsion couples to spin)
enters as rotations of its axis, automorphisms on the group, unitary and
first degree.

**The basis.** The plane laws read a mote's spins as its components on the
planes (`torsion::plane_spin`, the antipole, the disc's plane). The two
rotations below turn the spins by the group's action, `ℓ ↦ m·ℓ·m̄`
(`orientation::turned`). This keeps the spin's size on the group and moves
spin between the planes.

### 13.1 Precession about the core's axis

- **The twist at a body's place.** "Twist per turn is the deficit" (docs/100
  §2): the core winds each plane, each ring, by the deficit at the ring, ½
  at its own, shared by its tilt. A body takes it at its own place by its
  own deficit and its share of the core's planes, on its galaxy's clock:

      φ_i = δ_i·Σ_k t_c,k·t_i,k·ν_g        (`quantum::twist_at`; `quantum::twist` is δ = ½)

  So precession falls with reach as the deficit falls, as frame-dragging
  does.
- **The rotor.** `Â_c` is the core's lead (its wound plane, `torsion::wound`)
  as an L1 direction, and `m_c = Cayley(rate(φ_i)·Â_c)`. Each bell, `ℓ_i ↦
  m_c·ℓ_i·m̄_c`. A rotation about `Â_c` keeps the ring's part along it,
  `Â_c·ℓ`, exactly, so the inclination to the core's axis is not inflated,
  and the line of nodes advances: Lense–Thirring precession. (The tilt's L1
  share `τ` reads the nodes on the L1 measure and so moves as they turn;
  the part along the axis is the invariant.)
- **The book.** The spin moved between the planes, `ΔS_k`, is taken by the
  core's antipole with the opposite sign, `A_k′ = A_k − ΔS_k`, so `S_k + A_k`
  is constant on every plane (docs/100 §8.2, E39). This restores the wound
  plane's lead that §12 starved. `Σ_k A_k` need not vanish under a rotation,
  so the wound plane is the core's planes' spin with its antipole,
  normalized, `tilt_from(S + A)`. That is `t_k + A_k/ΣS` wherever `ΣA = 0`,
  and the torque's ledger identity counts the antipole.

### 13.2 Tipping by the twist's pressure

- **The axial vector.** The pair pressures `P_jk = ρ·(x̄_j ⊖ x̄_k)` (§10.4)
  are a bivector. Pair `(j, k)` turns plane `j` toward plane `k`, which is a
  rotation about the third axis, so the axial vector is `p_l = P_jk` with
  `l` the third. Read on the twist of what the ring's tube holds (the ring
  and the range it meets, each mote's `C` generator as a share
  `v/(1 + |v|)`, holding-weighted), `ρ` the enclosure's base over its
  whole.
- **The rotor.** `m_p = Cayley(p)`, and `ℓ_i ↦ m_p·ℓ_i·m̄_p` each bell. Where
  `p` is transverse to the ring's axis, it tips the ring out of its plane:
  torsion generates inclination. Since `p` is local, rings at different
  places tip differently. A disc-wide `p` would only rotate the disc whole.
- **The book.** As §13.1: the core's antipole takes `ΔS_k`.

### 13.3 The warp's balance

With §13.1–§13.2 beside §4b and §12.3, a disc no longer flattens to `τ = 0`.
Near the core (deep deficit, strong twist), precession and tipping hold the
rings to the core's wound axis. Far out, the out-of-plane shear and the
repulsion settle them toward the invariable plane. Between, tipping and
damping balance on a stationary warp, the analogue of a Bardeen–Petterson
disc, read where the motions balance, never solved for.

### 13.4 The chiral split between the halves

`a+ = C·(F·ℓ+·F̄)·C̄` and `a− = F·ℓ−·F̄` (§11.1): the twist turns the `+`
half's axis away from the `−` half's. The ring reads the split as the tilt
between them from their product, `Q = a−·ā+` (§11.3): its share, sense and
line of nodes `a−×a+`, the isoclinic torsion the parcel carries, in space
(`strip::Ring::chiral`). With `C = 1` and no partner spin the halves are one
axis, and the split is none.

### 13.5 Witnesses

- **The antipole's book:** `S_k + A_k` is unchanged on every plane by the
  precession and the tipping.
- **Precession keeps the inclination to the core's axis:** `Â_c·ℓ` is
  unchanged exactly, while the axis turns; an axis along `Â_c` is fixed.
- **Transverse pressure tips:** a ring whose `p` is transverse to its axis
  ends the bell tilted against its former axis; `p` along the axis does not
  move it.
- **Deeper precesses faster:** `φ` rises with the deficit.
- **The chiral split vanishes** with `C = 1` and no partner spin, and
  stands with them.

## 14. The jetstream's return: the exit across the passage

**Operator 2026-11-11:** zero stars in the home galaxy is "an error or
miscalculation of distribution properties through the jetstream of the
vacuum and continuum"; the fix below approved.

**What the probe measured** (`observe streams`, the g557 checkpoint, laid
before §9): 3,773 of 3,827 motes on the vacuum stream, holding 0.256 of the
galaxy, all inside the core's ring at median `R/H = 0.06`. The continuum
was a shell of 54 motes just outside it (`R/H ≈ 1.04`, holding 0.002). The
galaxy bound no band (`band_pairs`), so no star formed.

**Why.** Inside the ring a continuum share passes ½, so the luminal bound
sends the mote through the Möbius pass onto the vacuum leg
(`flows::luminal`). The vacuum leg returns only through the throat, and
what passes it leaves on the continuum (operator 2026-10-19). It left with
its spin, so it stood inside the ring again, and the next bell sent it back
to the vacuum. The matter cycled between the legs inside the ring, and the
outbound leg never carried it into the disc.

### 14.1 The exit across the passage: measured, not adopted

Across the passage a body stands at reach `H` about hold `R` (docs/103 §5;
Mycelium 78.181, settled). Read in the galaxy's own enclosure, the exit
stands where the ring law turns at `R/H` (`Enclosed::reach_at_rate`), its
spin `ℓ′ = R′·H(<R′)`. On g557 (`observe streams`) that carried the
continuum outside the ring from 54 motes (holding 0.002) to 259 (0.059)
in one bell. But every one of the 215 exits landed beyond the galaxy's rim
(reaches about 9 to 356 against a rim of 1.256): the reading throws the
matter out of the home, never into the disc between its ring and its rim.
Read literally, `(R, H) ↦ (H, R)` keeps `ℓ = R·H`, so the body returns to
its spin balance inside the ring at the next bell.

**So the passage is not the missing piece alone.** The home's matter
carries too little spin to stand outside the core's ring at all: the
bounce gives a nucleus `ℓ = p·w̄/w` with `p = ½` against the singularity's
base 1, which puts it at `R/H < 1`, and the home's rim is only `1.256` of
its base. The spin the bounce distributes through the jetstream is what is
read next (the probe first), before any exit law is adopted.

**Read (g557, `observe streams 0`).** The ring (the least reach turning at
rate 1, `R = H(<R)`) stands at `1.256`, the rim at `1.256`: with every
body inside, the ring's hold is the whole hold, so the ring is the rim and
the disc band between them has no width. To open it a body must carry
more than `ℓ = 1.256·1.256 ≈ 1.578`; by weight the vacuum carries 0.10
(p10), 0.27 (p50) and 0.37 (p90) of that, and the disc band holds nothing.
The bounce's `ℓ = p·w̄/w` is read against the singularity's base 1, not
against the home's whole hold (base plus the 0.256 its own matter adds),
so even at `p = 1` a mean nucleus stands at `R ≤ 1` inside the ring. The
shortfall is the bounce's ruler, not the passage.

### 14.2 The throat test reads the R side's passage, signed

A vacuum mote reaches the throat when its R-side phase crosses ½ from
above. The test read `frac(was − now)` as the distance travelled backward,
which holds only while the R side is driven backward (the retired loop).
With the R side turned by the shear either way (§10.3), a forward move `f`
read as `1 − f` backward, so one bell sent 2,139 of 3,773 vacuum motes
"into the throat". The test now reads the R side's passage with its
windings, the partner planes' mean of their readings' change and their
windings' change (as the spatial passage, §10.7 step 4), signed:

    Δψ_R = mean_j (ψ′_jR − ψ_jR + K′_jR − K_jR)
    enters  where  Δψ_R < 0  and  frac(ψ_R − ½) ∈ (0, −Δψ_R]

A forward passage never enters.

### 14.3 Witnesses

- **Forward never enters:** a vacuum mote whose R side moves forward is not
  taken into the throat, however far it moves; a backward passage through ½
  is.

## 7. Witnesses

- **Wholly outside or inside**, a plane reads `λ = 0` or `1`, `s = δ` or `1 −
  δ`, `σ = 1` or `−1`.
- **The luminal bound** `s_k ≤ ½` for every plane, every reach, every extent.
- **On the ring** (`R_k = H`), `λ_k = ½`, `s_k = ½`, `σ_k = 0`, the fixed
  point.
- **A whole crossing** advances the phase by `½`, and two crossings by a whole
  turn, the winding kept.
- **No pressure leaves the share bare;** `(s ⊖ P) ⊕ P = s`.
- **Drag is the pass's image:** `M(b) = −1/b`, and `M∘M` returns `b`;
  `b·(1/b) = 1` at every reach.
- **Drag is never a source:** `Σ w·σ·s` is kept by the exchange.
- **No departure and no medium mean no drag:** `D_k = 0` or `g_k = 0` gives
  `T_k = 0`.
