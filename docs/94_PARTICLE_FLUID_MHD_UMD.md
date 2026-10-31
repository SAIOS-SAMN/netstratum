# 94 — Particles in flow: fluid-MHD under the UMD overlay

This document derives fluid MHD and the unitary morphodynamics (UMD) overlay
for era 1's motes, so that the flow carries them as particles with pressure,
density, drag, field and current. Two constraints apply: no second-degree
exponents (squares), which make the medium rigid, and the vacuum–continuum
inversion is a Möbius inversion through the ocean as a jetstream current.
UMD is specified in ipsal-autic.

**Status:** derived, not built. Read against: ipsal-autic
`docs/implementation/phase_24_5_kinematic_substrate_layer_discipline.md`
(the thermodynamics → kinematics → UMD stack, K-step discreteness,
SUBSTRATE_C, rotor transport, the refused patterns) and
`docs/legacy/.../NEXT_LEG_BLUEPRINT (VERSION 6.1.0).md` (§0.5 the dual
ring: ℤ windings, ℚ gear ratios; §2.3 orbital mechanics; §2.4 L1 never L2;
§6 the physics extrapolation table).

## 0. What is missing, and why it matters

Netstratum's motes are already Lagrangian parcels: each carries mass (3Z),
spin in two planes, phases, heat, leg and sense, and every exchange between
them is booked exactly (docs/92 §1, the identities). What they lack is the field
side: no pressure, no density, no magnetic field, no current, no coupling of
field to flow. In nature those are what let a disc keep its dust (pressure
maxima) and move its turning outward (magnetic turbulence). This document
derives them for motes as particles, in the UMD grammar, first degree
throughout.

## 1. The UMD rules every component obeys

From the ipsal-autic stack, read onto Netstratum's grammar:

- **Two rings, two meanings.** Integers (ℤ) are windings: how many times a
  particle has wrapped an obstruction (whole orbits completed, rungs). They
  carry torsion. Rationals (ℚ) are gear ratios (phase-lock quotients ν = N/D):
  "for every D bells this particle completes N turns". Never magnitudes,
  never floats. (Blueprint §0.5.)
- **K-steps, never continuous time.** One bell is one K-step; there is no
  dt → 0, no derivative, no integral. A rate is a gear ratio per bell.
- **The luminal bound, SUBSTRATE_C = 1.** No state changes by more than one
  whole turn per K-step. Exceeding it is a luminal fracture, refused as a
  typed outcome, never a panic; the saturation escape is the flop.
- **Motion is transport, X_{k+1} = R·X_k·R⁻¹.** A particle moves by a rotor
  sandwich on its sphere, its phases turned, never by adding a velocity
  vector to a position.
- **L1, never L2.** Sums of absolute values, never sums of squares (the
  denominator hygiene and the no-flat-square law, one rule).
- **Selection by measurement, never if/then gates.** Where several outcomes
  could happen, each is read and the measurement decides.
- **Σ L = 0, unitary equilibrium.** Every exchange of turning is booked by
  both sides.

## 2. The luminal bound and the ring (the derivation that ties it together)

A particle at deficit δ about its centre turns `2δ` of its orbit a bell (the
ring law: two rings a bell). The luminal bound demands `2δ ≤ 1`, i.e.
`δ ≤ ½`. The ring, `δ = ½` (R = H), is exactly where a particle orbits at
the luminal bound. Inside it a particle would turn more than a whole orbit a
K-step: a luminal fracture. So nothing can orbit inside the ring. Matter
that reaches it inverts: the Möbius jetstream (docs/93, to be written; first
degree in the deficit):

    M: (δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)

the vacuum's pull is the continuum's push (`1 − δ ≤ ½`, back within the
bound), half a turn round the loop, its sense flipped; `M∘M = id`. The
transit is the current `δ(λ) = δ + λ(1 − 2δ)`, `ψ(λ) = ψ + λ/2`, passing the
ring at λ = ½ where its sense flips.

This is the UMD reading of three things previously held separately: "inside
the ring the vacuum takes" (Xylarium 11.0, 2026-10-15), the throat's inversion, and
why the founding (every kind inside the ring) left galaxies with no disc.

## 3. The particle's state (the overlay on `flows::Mote`)

Kept as now: kind, amount, `ℓ_o`, `ℓ_⊥` (spins), phases, agitation, leg,
born, levers.

Added, each a reading or an exact carrier:

| Field | Ring | Meaning |
|---|---|---|
| `wound: u64` | ℤ | its winding: whole orbits completed about its centre (the floor of its orbit phase's advance since its birth) |
| `gear` | ℚ, a reading | its gear ratio ν = 2δ at its place, never stored (render on observation) |
| `field` | ℚ phase | its magnetic phase on S¹: the orientation of its open frames, carried with it (frozen into the particle) |

`u` (its open frames, the Q pole) is already its magnetic moment: the field
phase gives it a direction.

## 4. Thermodynamics: pressure and density, read by ring

A ring is a bead of the galaxy's or the system's disc (`flows::beads`, one
kind on one face, a fibre's run between its hold minima): the fluid's own
gathering, never a grid.

- **Density** of ring b: its particles' holding share over its band's width,
  `ρ_b = m_b / w_b`, the width the band's own spread `R(1 + p) − R(1 − p) =
  2pR` (first degree: a share over a length).
- **Pressure** of ring b: density times its holding-weighted heat,
  `P_b = ρ_b · a_b` (the ideal-gas form, a product of two distinct readings).
- **The pressure push** on ring b: the difference of its neighbours'
  pressures across it, `F_b = (P_{b−1} − P_{b+1}) / m_b`, outward where the
  inner ring presses harder. A difference, never a gradient squared. Booked:
  what pushes b outward pushes its neighbours inward by the same holding-
  weighted share (Σ L = 0).
- **Pressure support**: a ring's gas stands outward of its dust by its
  pressure share, `σ = F_b / (F_b + δ_b)` (the saturating share of its push
  against its pull), read per kind by inertia.
- **The pressure maximum traps dust**: where `F_b` changes sign between
  neighbours, the push is inward from outside and outward from inside: dust
  drifting by drag collects there. Measured, never gated.

## 5. Drag with crowding (the back-reaction the first attempt missed)

The earlier drag (set aside, `scratchpad/disc-pressure-drag.patch`) passed
spin from faster-turning dust to pressure-held gas and drove all dust into
the star: the radial drift barrier. The missing term is the back-reaction:

- In ring b, the drag's share is the gas's share of the ring's holding,
  `g_b = m_gas / (m_gas + m_dust)`.
- Dust gives spin to gas by the exchange law times `g_b`: where dust
  outweighs gas, `g_b` falls, the drag weakens, and the dust stops drifting.
  Dust crowds where it is already dense, and the crowd grows: the streaming
  concentration, first degree.

## 6. Electro-magnetics: field, current, induction, as particle readings

- **A ring's magnetization**: its particles' moments, each by its field
  phase's in-step with the ring's holding-weighted mean field phase,
  `M_b = Σ u_i · in_step(φ_i, φ̄_b)` (the coherence of their orientations;
  L1: a sum of shares).
- **The field across an interface** (FaradayTensor across the orb pair): the
  difference of two neighbouring rings' magnetizations, `B_bc = M_b − M_c`,
  first degree.
- **A current** is a field that does not close: around a loop of rings the
  interfaces' fields sum to a residual, `J = Σ_loop B` (the closed-loop
  commutator's reading); `J = 0` where the field is a pure gradient.
  Integer-layer: the number of whole field windings the loop encloses is its
  torsion (ℤ), the residual its rational part (ℚ).
- **Induction (the field frozen into the flow)**: each particle's field
  phase advances with its own orbit phase (`φ ← φ + 2δ` a K-step, the same
  rotor), so differential rotation winds the field: neighbours' field phases
  drift apart by their gear ratios' difference. The in-step between them
  falls; the magnetization falls with it: shear stretches the field.
- **Reconnection** (discrete torsional yielding, Theorem xxxiv's form): when
  a loop's whole windings change, the field reconnects: the winding's
  turning is released as heat to the loop's particles, by holding. A step,
  never a continuous slip.

## 7. MHD coupling: the field moves turning (what drives accretion)

- **Magnetic shear**: between neighbouring rings the field transfers turning
  by the exchange law, its share the interface's field over the pair's
  moment, `c_B = |B_bc| / (u_b + u_c)` (a saturating share, ≤ 1), times their
  gear ratios' difference. Outward, as the hydrodynamic shear, but in
  proportion to the field. A weakly magnetized disc spreads by it: the
  magnetorotational transport, first degree, no instability criterion
  squared.
- **Magnetic pressure**: a ring's field adds its in-step moment to its
  pressure, `P_b ← P_b + M_b / w_b` (a share over a width, as the gas's).
- **The star's field** (its magnetic braking, already built) is this law at
  the system tier: the star's moment and its disc's, read across their
  interface.

## 8. Kinematics: how a particle moves each K-step

One K-step for a particle, in order, each booked:

1. its ring's pressure push (§4), drag (§5) and magnetic shear (§7) give its
   spin change; its anchor's torque (docs/92) and its core's push give the
   rest;
2. its new spin's gear ratio `2δ` is checked against SUBSTRATE_C; a particle
   reaching the ring takes the Möbius current (§2) rather than fracture;
3. its phases (orbit and field) turn by the rotor of its gear ratio;
4. its winding counts the whole turns it completed;
5. it is held, released, settled or landed by the readings already built
   (the cut, the rim, capture, the ocean's landing).

No step stores a derived value; every density, pressure, field and gear
ratio is read from the particles at the K-step it is needed.

## 9. What it should do for worlds (the predictions to check)

- Matter founded in the disc (`[1, 2]`, from the ring) stays there: the
  luminal bound forbids orbiting inside the ring, the jetstream returns what
  crosses it.
- Gas stands outward of dust by its pressure; dust drifts inward by drag but
  crowds where the pressure peaks and where it is already dense.
- Magnetic shear spreads the disc and feeds its centre without emptying it.
- Crowded dust rings collapse (the binding law) into bodies inside their own
  rims: worlds.

## 10. Refused, per the UMD discipline

- squares and roots in any law (L2 norms, `v²` energies, `1/r²` falloffs,
  `√` reaches);
- continuous time, derivatives, floats;
- stored derived values (density, pressure, field, gear ratio are readings);
- if/then gates (every threshold is a measurement: a sign change, a
  winding count);
- unbooked exchanges (every spin moved is paid by its counterpart);
- grids (rings are the particles' own beads).

## 11. Build order and witnesses

1. The Möbius jetstream and the luminal bound (§2): witness `M∘M = id`,
   every particle's `2δ ≤ 1` after each K-step.
2. Pressure and density by ring (§4): witness Σ of pushes = 0.
3. Drag with crowding (§5): witness spin conserved per ring.
4. Field phase, magnetization, interface field, current (§6): witness a
   pure-gradient field gives `J = 0`.
5. Magnetic shear and pressure (§7): witness Σ L = 0.
6. The founding from the ring (scratchpad/founding-from-ring.patch, with the
   home's own frame).
7. Read: discs kept, dust crowding, worlds grown, every identity exact.

Each its own commit and read on the same founding.
