# 99 — The binding's dynamics, end to end, with the inverse faces

This document traces a binding's dynamics end to end, from the founding
through capture to the formed body, including the inverse faces, and derives
each misalignment behind the measured break (§0).

**Status (2026-10-28):** §2.1–§2.5 built (32a7dfc, 3400588, bc2d32c); §5
open.

**Prerequisites:** docs/96 (founding as flow; §7 the galaxy enclosure; §8
the hold law), docs/97 (Q, Z, R), docs/98 (the inverse faces).

## 0. The measured break (2^55 grains, bells 0–1500)

- 12 systems, 10 companions, 8 worlds by bell 1500; galaxy 163 has heard no
  echo (it stands ~3780 bells out).
- No binding was refused at ignition (0 by heat, 0 by heart).
- 49683 members returned from bindings in which nothing fell; by bell 900,
  59 such bindings, 16876 members; the nearest member a median 118 heart
  extents out (10^1 to 10^5).

## 1. The chain, stage by stage

Each stage: its law, what it reads by (phases, or a frame still to retire), the face it reads (G gravity's
held share `δ = H/(R + H)`; P the push's unlocked share `1 − δ`), its inverse
reading, and its state.

| # | stage | law | reads by | face | inverse reading | state |
|---|---|---|---|---|---|---|
| 1 | release | `s_k = ½·c/(c + base)`, placed `R = ring + span·c` | galaxy | P (tide on the unlocked face) | released on this universe's continuum leg = the true vacuum; the luminal bound sorts the face | built |
| 2 | luminal | share > ½ ⇒ Möbius | mote | either | symmetric (docs/98 §4) | built fa7378d |
| 3 | torque, turning | `2·share` a bell, route by face | mote | G on its face | vacuum face reads `1 − δ`, route backward | built |
| 4 | disc settles | tilt returns `turned/(d + turned)` | galaxy | G (`turned = 2δ`), P (`d = 1 − δ`) | admitted face only (outside the ring) | built |
| 5 | rings | band pairs `k(ρ) = (S/G)(1+ρ)/(1−ρ)` on the strip | galaxy | G/P balance | ring fixed (½ both faces) | built |
| 6 | the crest | bond `λ/(1+λ)`, `λ = 2(m+μ)/R` ≥ heat + tide `w/(2(R+H))`; `⌊R/w_max⌋` bindings by disc-plane phase | the binding's own enclosure | G contrast (tide) | tide symmetric (`|Δδ| = |Δ(1−δ)|`) | built |
| 7 | membership | ring band × disc-plane arc | galaxy | — | — | built — **misaligned (§2.1, §2.2)** |
| 8 | infall | member falls where its distance ≤ `λ_f·R_max` (the heart's extent), fixed point from all fallen down | a dilated frame (`× g/m`) | G (heart cut `δ ≥ ½`) | the held heart is the vacuum face (containment), coherent | built — **broken; to phases (§2.2)** |
| 9 | ignition | `a < ½`, share `(1−2a)/(1+2a)` | binding | — | — | built |
| 10 | heart cut | kind held where `H/(R + H) ≥ ½` | the star's own | G inside the throat | inside the throat the vacuum face holds (docs/98 §5) | built |
| 11 | disc → system | disc motes at `ℓ = r·H(<r)`, warmed by light × unlocked + galaxy flux `1 − δ_gal` | system | P (heat) | heat on the push face | built |
| 12 | worlds | echo hold `max m/(r + max m) ≥ |Δω|`, solid orbits never meet | system | G (`ω = δ`) | admitted face | built 40b4ad5 |
| 13 | burn | barrier `Z + Z`, `τ = 1/(1 + 2z)` | star | — | **reads weight where its law says charge** (§2.5) | built — **misread** |
| 14 | captures, release | `σ = κ + (1−κ)h`, grip `κ·h`, h at the pulse | star | R | electrons' phase | built 785a074 |
| 15 | death, recycle | spent / decohered return; remnant drains `δ/2` | galaxy | G | — | built |

## 2. The misalignments, derived

### 2.1 Membership takes members from the axial leg

`band_pairs` gathers every mote of a ring band whose disc-plane phase falls
in the arc, on either leg of the R loop. Members on the axial leg (loop
phase in `[½, 1)`) are in transit (docs/94 §11: "the axial leg carries
matter in transit; the disc leg is where the cut and collapse are read"),
half a loop from the disc. **Law:** a binding gathers on the disc leg only
(`on_disc_leg`), as the beads and the cut already do.

### 2.2 No frames: the infall is read in phases

The infall turns each member's phase offsets into lengths (`r_c·|Δψ_k|`),
dilates them into a frame (`× g/m`, the binding's "own units") and compares
them with a heart's extent read in that frame. Both the frame and the
dilation retire: a member's place in its binding is its phase, and closeness
is the in-step share, the same measure the chains, the coherence and the
torque already read (S¹'s linear measure, `in_step = 1 − 2|Δψ|`).

- **The binding's phase:** its members' holding-weighted circle mean in each
  plane, `ψ̄_k` (`flows::circle_mean`), as now.
- **A member's step with it:** across the spatial planes by its orbit's own
  shares (its tilt, L1: a plane its orbit does not turn in carries no
  weight), `C = Σ_k t_k·in_step(ψ_k, ψ̄_k)`, and on the R loop on the disc
  leg (§2.1).
- **It falls on the held hemisphere:** where `C ≥ ½` (the hemisphere law's
  boundary, a quarter turn; the phase twin of the throat condition, deficit
  ½ at the ring). Below ½ it stands on the open hemisphere: the disc.
- **No fixed point to empty:** the fall is read once, each member against
  the binding's phase, order-free; no extent shrinks with what fell.
- **The disc's places:** a disc member's place about its star is its phase
  offset from the star's (the fallen members' mean), its turning read by the
  ring law at the star's hold; never a length in a frame. (The current
  `ℓ_o = r·H(<r)` from a dilated distance retires with the frame; its phase
  form is owed in the build of step 3.)

### 2.3 (retired) The runaway's contraction

Read in phases the binding needs no contraction from its arc to its throat:
the hemisphere law decides the fall at any width.

### 2.4 (retired) The extent's fixed point

Retired with the frame (§2.2).

### 2.5 The burn's barrier reads weight

`formation::burn` states its barrier as the charge two nuclei bring,
`Z + Z`, and tunnelling `τ = h/(h + 2Z)`; it is handed the nuclide's weight
w. In the inverse nucleus the charge is the protons, `Q(w)`. **Law:**
`τ = 1/(1 + 2·Q(w))` (weight 4: 1/5, as the doc's He + He; it reads 1/9).
The driver stays over the product's nucleons `2w`.

## 3. The inverse physics in this chain

- **Faces.** The luminal bound admits only the face whose share is at most
  ½ (docs/98): outside the ring the continuum face (δ < ½), inside it the
  vacuum face (1 − δ < ½). This universe's released matter stands outside
the ring, so
  bodies form on the admitted face and read δ; the hearts hold inside their
  own throats, where the admitted face is the vacuum's: containment is the
  vacuum's kind (Xylarium: "a star is quantum gravity containment, its
  holding face the vacuum's"). So in this universe the heart is the vacuum
  and the disc the continuum, each read on its admitted face: the inversion
  is carried by where matter stands, never by a second law.
- **What the inversion changes in a law:** only where a law reads a face
  without asking which one the matter stands on. In this chain: none beyond
  the luminal bound (built). Gravity (G), magnetism (the open frames' push,
  P) and velocity (`2·share`) all read the admitted face.
- **The nucleus** stays as built (Q the open proton, Z the bound neutron,
  pole = leg, R the electrons' phase).

## 4. Build order (one commit each, its witness read before the next)

1. **§2.1 disc-leg membership** (built). Witness: no member on the axial leg; the
   ring's holding conserved.
2. **§2.2 the infall in phases** (built with step 3; `flows::step_with`; the
   frame, `g/m`, `formation::infall` and the newborn's rim by distance
   retire; the system's rim is the whole turn). Witness: a
   member displaced only in a plane of tilt 0 is in step; a member a quarter
   turn off in its orbit's plane stands at the boundary; a binding whose
   members share its phase falls whole.
3. **The disc's places in phases** (built with step 2: a disc member's place
   is its phase off step, `1 − C ∈ (½, 1]`, read by the ring law in
   `formation::system`; the star's own spin reads each fallen member's
   phase place times its hold as a share of what fell, `d·H_c/λ`, the
   place's `/λ` dilation retired). Witness: the system's
   spin booked exactly; every disc member's phase offset from its star kept.
4. **§2.5 the burn's barrier by charge** (built, `formation::tunnelling`). Witness: H + H 1/3, He + He 1/5.
5. **Read:** re-found, breathe to bell 1500 (and 3000 for galaxy 163's first
   echoes): bindings formed, hearts formed, stars, worlds per star, every
   identity.

## 5. Open

- The timescale: galaxy 163 hears its first echo near bell 3780; a reading
  of the whole universe needs bells to ~4000 (cost grows with the galaxy
  motes, ~200 s an echo at bell 1500).
- Whether the R loop's offset should read the circulation's length (a loop
  of `2·base`) rather than the orbit's.
