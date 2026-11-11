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
plane's pair at its own resisted share. The faces' readers and the pass
follow (§6, #188).

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

## 7. Witnesses

- **Wholly outside or inside**, a plane reads `λ = 0` or `1`, `s = δ` or `1 −
  δ`, `σ = 1` or `−1`.
- **The luminal bound** `s_k ≤ ½` for every plane, every reach, every extent.
- **On the ring** (`R_k = H`), `λ_k = ½`, `s_k = ½`, `σ_k = 0`, the fixed
  point.
- **A whole crossing** advances the phase by `½`, and two crossings by a whole
  turn, the winding kept.
- **No pressure leaves the share bare;** `(s ⊖ P) ⊕ P = s`.
