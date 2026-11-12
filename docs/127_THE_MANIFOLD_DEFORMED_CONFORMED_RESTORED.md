# 127 — The manifold deformed, conformed and restored: the connecting tissue

This document names the component every heat, pressure and twist law was
missing. A body's exuded properties are not quantities it holds. They are a
deformation of the manifold it stands in. The manifold conforms its other
bodies to that deformation and restores it. What restoration removes is
dissipated into the manifold and recycled, never deleted and never parked
in a body.

The document gives the history of the finding, the laws with every factor
derived, and the order of the build.

**Operator 2026-11-11:** "all of this needs to be a property of the manifold
as a deformation, conformation, or restoration — that's the missing
component; you are putting the phase state in the star as it exudes
properties — without the connecting tissue"; then "derive all factors and
formulas and update equations to complete accuracy — then begin development
on heat transfers as recycling energy across manifold dissipation".

**Status:** derived (2026-11-11). Built in steps (§6), each with its witness
and a founding read. Step 1 built: heat dissipated through the manifold
(`flows::emit`, #182).

**Prerequisites:**
- docs/120: the coordinate as the state, its lanes S, G, T, H. The manifold
  here is those lanes read over one enclosure.
- docs/124 §3: heat as motion out of step.
- docs/104 §0 and §2: the inversion `I` and the counterweight.
- docs/105: polarity equilibria.
- docs/121 §9: the range's field as one.
- docs/100 §8: the quantum torsion.
- docs/126: the vacuum as a Möbius strip.

## 0. History: how the missing component was found

1. **docs/126's half twist** (the Möbius pass on the twist) was read on one
   founding (2^40, 580 bells). The universe moved: 6 stars born against 14,
   250 bodies in systems against 15. The heat the energy book could not
   account for at bell 558 grew from −[2;08) to −[11;20)·60⁻².
2. **Isolated to the half twist:** the two-face pressure was inert (the core's
   tilt is isotropic, so its twist pressure is zero, the same root as #157).
3. **Traced to heat crossing body boundaries unbooked.** A star's outburst,
   shed layers, dissolutions and infall made motes at heat 0; escaping motes
   and motes falling into a star dropped theirs. Carrying each source's heat
   per holding (docs/126 §3a) brought the gap to −[4;25)·60⁻², still twice
   main's.
4. **The remainder had the same cause:**
   - a star's heat is the scalar `(agitation_of(reservoir) + h)·kept`, which
     changes whenever its reservoir moves, with nothing to take the
     difference;
   - contraction at ignition is booked as no source (#142).
5. **The operator named the cause:** each fix moved a number between body
   scalars. The state lives on the manifold, but no law carries a body's
   change into it. Every body boundary is therefore a place the books can
   leak. The leak is the missing connecting tissue, not a missing patch.

## 1. The manifold and a body on it

- **The manifold `M_E`:** an enclosure `E` (a galaxy's core, a star's system)
  read as the coordinate's lanes over everything it holds (docs/120). Its
  bodies are its motes, stars and worlds, each standing at a place in `E`.
- **Its conformed state at a place:** what the manifold holds there with no
  body departing from it. This is the holding-weighted reading of the range
  that meets the place (docs/121 §9.2, the field as one), on each lane and
  each component:

      x̄(place) = Σ_j w_j·x_j / Σ_j w_j        (j over the range)

- **Its held share at a place:** `g_M = 1 − δ`, `δ = H/(R + H)`, the
  manifold's own face (docs/105 §3; the hold inside `R`, docs/103 §5).

## 2. Deformation

A body's **deformation** is its departure from the conformed state at its
place, per lane and per component, first degree and never squared (register
C17):

| lane | deformation | already in the record as |
|---|---|---|
| H (phase) | `D^H_i = x_i − x̄` per component; its magnitude `Σ |x_i − x̄|` | heat, motion out of step (docs/124 §3, `dispersion`) |
| H (twist) | `d_i = t_i − t̄`, per pair `d_j ⊖ d_k` | the quantum torsion (docs/100 §8) |
| S (reach) | the lift `u_i`, reach displaced from its balance | docs/121 §14.6 |
| G (bond) | the open dimensions still to close | the bond (docs/124 §2) |

A body's heat is its H-deformation weighted by its holding, `w_i·a_i`. That
is the book's `a·m`, read as a deformation of the manifold rather than a
quantity the body holds.

**A change of a body's own state is a deformation of the manifold.** A star
whose reservoir moves changes `agitation_of(reservoir)`. That change is the
manifold deformed at the star's place, by

    ΔD_star = (A′ − A)·K        (A = agitation_of(reservoir), K its kept)

and it enters §4's dissipation, never vanishing.

## 3. Conformation

The manifold **conforms** its bodies to a deformation: each body's state
moves toward the conformed state by the coupling that already acts.
- the shear's range answering its field as one (docs/121 §10);
- in-step coherence;
- bonds.

Conformation moves state between bodies with `Σ w·x` kept: conformation is
never a source. Its witness is the existing one: the shear books what it
moves, `Σ m·ℓ` exact.

## 4. Restoration and dissipation

The manifold **restores** a deformation. Each ring, a body's deformation
relaxes by its keep `k_i`, the law that already stands for that lane:

| lane | keep a ring | law |
|---|---|---|
| H (heat) | `k_i = g_i`, its held face | emission (E54) |
| H (twist) | `k_i = 2δ/(1 + δ)` | the attractor (E38) |

What restoration removes from body `i` this ring is

    Δ_i = (1 − k_i)·w_i·D_i

**Dissipation is the missing law.** `Δ_i` leaves the body and enters the
manifold. It is not deleted, and it is not parked in another body's scalar.
The manifold's dissipation this ring is

    Δ_M = Σ_i Δ_i + Σ ΔD_state        (bodies' restoration, bodies' own state changes)

and the manifold returns it by its own faces at the place it was given
(docs/104 §2's counterweight, "cooling here is heating there", read through
the tissue):

- **Radiated:** the unlocked share `u_M·Δ_M` crosses on no lane as light
  (docs/124 §2). It is absorbed by the bodies it falls on (irradiance,
  E27) and the rest drains to the sink, the vacuum's face. This is E54's
  light, now the manifold's.
- **Recycled:** the held share `g_M·Δ_M` stays in the manifold as heat. It
  is returned to the bodies that conform at that place, each by its holding
  at its own held share:

      to body j:  g_M·Δ_M · w_j·g_j / Σ_l w_l·g_l

  This is "heat transfers as recycling energy across manifold dissipation":
  what one body restores, its neighbours take up.

**The book closes by construction:**

    Σ_i w_i·a_i (after) = Σ_i w_i·a_i (before) + named sources − Δ_M + recycled + absorbed

with `Δ_M = radiated + recycled` and `radiated = absorbed + drained`. So the
only heat that leaves the bodies is the light drained to the sink, and there
is no unaccounted term left to grow.

**Factors, all derived:**
- **`g_M`, `u_M`:** the face law at the dissipating body's place (docs/105
  §3), `g_M = 1 − δ`, `u_M = δ`, a pair share with no ½.
- **`k_i`:** each lane's standing keep (E54, E38).
- **The recycling's weights:** holding times held share, `w_j·g_j`, the same
  pair-share form as everything in docs/104 §1.2.
- **No constant enters.**

**What it replaces:**
- E54's "the rest emitted as light" becomes the manifold's radiated share,
  with its held share now recycled;
- docs/126 §3a's carriage at heat 0 → per holding was an interim, and stays
  as the matter-carries-its-own-deformation rule;
- the reservoir's unbooked change becomes `ΔD_star`.

## 5. Witnesses

- **Each ring the manifold's book closes:** `Δ_M = radiated + recycled`, and
  `radiated = absorbed + drained`, exactly.
- **A star whose reservoir moves** gives `ΔD_star` to the manifold, and the
  bodies' heat plus the manifold's pending dissipation is unchanged.
- **Conformation is never a source:** `Σ w·x` kept.
- **The founding read:** heat unaccounted at 0 at every echo, or reduced to
  the light drained; plus stars, hydrogen and motes against main.

## 6. Build order (one change each)

1. **The manifold's dissipation on heat (H)** (#182): emission through the
   manifold (radiated at `u_M`, recycled at `g_M` to the conforming bodies)
   for each enclosure's motes; the book reads `Δ_M`, radiated and recycled.
2. **A body's own state change as deformation** (#183): the star's reservoir
   (`ΔD_star`) and contraction at ignition (#142) into `Δ_M`.
3. **Heat by component on the manifold** (#143): `D^H` per component, the
   recycling per component.
4. **The twist and the reach** (#184): the attractor's restoration (E38) and the
   lift's into the same dissipation.
5. **The bond (G)** (#185): chemistry's heat (docs/122 §6) as the bond's
   restoration.

## 7. Open

- Whether the radiated and recycled split is read at the dissipating body's
  place, as here, or over the range it meets. This document reads it at the
  place, first degree, and the read decides.
- docs/122 law 3 (all cooling as light) is restated by §4. docs/125 §4.1 item
  3 is superseded: the counterweight is the recycled share, carried by the
  manifold.
