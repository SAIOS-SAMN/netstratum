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

**Status:** derived. Readings built and witnessed (`conductor::strip`); the
laws are rewired onto them one at a time (§6), each with a founding read.

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

## 7. Witnesses

- **Wholly outside or inside**, a plane reads `λ = 0` or `1`, `s = δ` or `1 −
  δ`, `σ = 1` or `−1`.
- **The luminal bound** `s_k ≤ ½` for every plane, every reach, every extent.
- **On the ring** (`R_k = H`), `λ_k = ½`, `s_k = ½`, `σ_k = 0`, the fixed
  point.
- **A whole crossing** advances the phase by `½`, and two crossings by a whole
  turn, the winding kept.
- **No pressure leaves the share bare;** `(s ⊖ P) ⊕ P = s`.
