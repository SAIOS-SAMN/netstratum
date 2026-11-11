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
plane's pair at its own resisted share. §9 (the two halves) supersedes §1's
per-plane reach and step 3's rates: six spins, two rates, one law for the
motion and the book. It is built next (§9.6), then the faces' readers and
the pass (§6, #188).

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

## 9. The two halves: six spins, two rates, one law

**Operator 2026-11-11:** "is step 2 condensing what should be uncompacted
space into two planes which is just two dimensions in costume, or is there a
deeper issue that needs to be unfolded"; the halves below then ruled ("write
docs/128 §9 on the two chiral quaternion halves before writing code").

**Status:** derived; supersedes §1's per-plane reach and §8 step 3's rates.
Build follows (§9.6).

### 9.0 What was collapsed

- **The state.** A mote holds `spins`, three spatial spins, and `ell_orth`,
  one number for the whole R side: four spin numbers for the six planes of
  its orientation `G`.
- **The bell.** `Orientation::bell` turns the circulation "on each R-plane
  alike", so `e₁R`, `e₂R` and `e₃R` never part. The R partner, the plane's
  attached dimension (§8 item 1), has no direction of its own.
- **The rate.** The tilt's turning (`turning_of`) reads one `δ` from the
  summed reach. The spatial planes take `2δ·t_k`, and the loop takes `2δ` plus
  the push, the same `δ` again. A rotation of S³ has two rates (Spin(4) =
  H⊕H, rank 2), and they were tied to one number. This is why #157's planes
  stayed identical.
- **§1's unfolding was the wrong one.** It read the three spatial planes as
  three orbits, each at its own reach `R_k = ℓ_k/H(<ℓ_k) < R`, so `δ_k > δ`.
  It copied those rates onto the still-collapsed partners. The phases
  advanced near `6δ` a bell while `spin_torque` booked at `δ`: two laws for
  one motion. One founding showed it: at the echo of bell 558, one system
  of 1,535 bodies where step 2 forms three of 63, from the same
  checkpoint.

### 9.1 Six spins

A mote's spin is a bivector with one component on each plane of `G`:
`ℓ_k` on the spatial plane k and `ℓ_kR` on its partner `e_kR` (k = 1, 2,
3). No two planes share a slot. `ell_orth` becomes the three partner spins;
the swing's split `t_k·ℓ_⊥` (docs/120 §8b′) is their reading today, and it
is how a mote's existing loop spin is first carried onto them.

### 9.2 The two halves

The self-dual and anti-self-dual halves (docs/117 §11.7, `P± = (1 ± I)/2`),
first degree:

    ℓ±_k = ℓ_k ± ℓ_kR        ℓ± = Σ_k |ℓ±_k|        t±_k = ℓ±_k / ℓ±

Each half stands at its own reach by the ring law and turns at its own
deficit:

    R± = ℓ± / H(<ℓ±)        δ± = H(<ℓ±) / (R± + H(<ℓ±))

A half with no spin (`ℓ± = 0`) does not turn: its steps are zero, never `δ
= 1` with no direction.

### 9.3 The planes' steps, from the halves

A bell is two rings (E57), each half turning `δ±` a ring along its own
direction:

    Δθ_k  = δ+·t+_k + δ−·t−_k        (spatial plane k, turns a bell)
    Δθ_kR = δ+·t+_k − δ−·t−_k        (its partner)

- **The identity.** With no partner spin, `t+ = t− = t` and `δ+ = δ− = δ`,
  so `Δθ_k = 2δ·t_k`, the tilt's sum, and `Δθ_kR = 0`. The planes cannot
  over-turn, since `Σ_k |Δθ_k| ≤ δ+ + δ− ≤ 2·max δ±`.
- **The halves part** once the partners differ from their planes. With
  `ℓ_kR = c·ℓ_k` the halves stand at `ℓ(1 ± c)` and turn at two rates: the
  double rotation.
- **Step 2's loop is not recovered,** and should not be. Step 2 turned every
  partner at `2δ` plus the push, along the diagonal, whatever spin the
  partners held. Here a partner turns by its own spin.

### 9.4 The push along the loop

The parent's push advancing matter along the orthogonal orbit (`orth_push`,
`κ_⊥`) is a torque on the partner spins, booked by `spin_torque` as now. It
is shared onto the three partners by their own spins' shares,
`ℓ_kR / Σ_j |ℓ_jR|`; with no partner spin, by the tilt. It is never laid on
every partner alike.

### 9.5 One law for the motion and the book

`spin_torque`, the face (§2), the transit (§3), the pressure (§4) and the
drag (§4a) read the same halves. A plane's deficit a ring is half its bell
step, `d_k = Δθ_k/2` and `d_kR = Δθ_kR/2`, and the book takes that plane's
torque at that `d`. No law reads a third `δ`. §1–§3 are then read per half,
at `R±` and the parcel's extent there, in place of `R_k`.

### 9.6 Build order

1. The mote carries its six spins (`ell_orth` → three partner spins). This
   is a save-format change, so one new genesis checkpoint is laid once.
2. The halves (`ℓ±`, `t±`, `R±`, `δ±`) and the steps of §9.3 as readings,
   with their witnesses.
3. The bell turning each plane and partner by `Δθ`, retiring §8 step 3's
   `2·s_k′` and `turning_of`'s `2δ·t_k` together.
4. `spin_torque` at each plane's `d`, then the face, transit and drag per
   half.

Each step from 3 on is read as bell 558 from the new checkpoint against the
same bell under step 2 (`.testtmp/wt2`), with these counters:
- `Σ m·ℓ` given against the push;
- the phases' advance against `Σ d`;
- the systems and their bodies.

### 9.7 Witnesses

- **No partner spin** recovers the tilt's sum, `Δθ_k = 2δ·t_k`, `Δθ_kR = 0`.
- **The halves part:** `ℓ_kR ≠ 0` gives `δ+ ≠ δ−` and partners turning
  apart.
- **Bounded:** `Σ|Δθ_k| ≤ δ+ + δ−`, so a bell never turns more than the two
  halves carry.
- **One law:** the deficit the book reads for every plane is half its phase
  step.
- **Exchange of halves:** reversing every partner (`ℓ_kR ↦ −ℓ_kR`) exchanges
  `+` and `−` and leaves `Δθ_k` unchanged and `Δθ_kR` reversed: the
  inversion between the two halves.

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
