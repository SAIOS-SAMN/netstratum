# 106 — The thermodynamic substrate: potential, pressure and energy beneath motion

Design direction (2026-10-28): the obstacle to stability and equilibrium is a
missing substrate; this document scopes it. Status: scope. Read against
ipsal-autic
`docs/implementation/phase_24_5_kinematic_substrate_layer_discipline.md` (the
operator-named stack, 2026-05-17), as docs/94 was. Builds on docs/94 (UMD,
SUBSTRATE_C), docs/103 (the layers), docs/104 (the inverse duals), docs/105
(polarity equilibria).

## 0. The stack, and how ours stands inverted

```
UMD / form           bodies, bindings, hearts, worlds       shaped by
KINEMATICS           motion, transport, SUBSTRATE_C          propagated FROM
THERMODYNAMICS       pressure, potential, energy             ← the substrate
```

Ours runs upside down. Motion is the floor: torques give spin (`flows::torque`,
`spin_torque`), and heat and pressure are read off afterwards as its residue
(agitation the torque's undelivered share; pressure `ρ·ā` of that residue;
cooling a share that leaves no book). Nothing holds potential or energy as
state, and nothing propagates motion from it. That one gap is the main
problem with stability (operator, 2026-10-28):

- **No restoring force:** equilibrium lives in the substrate (pressure
  against the pull), and there is no potential or energy state to restore to.
- **Heat leaves no book:** cooling's share is lost (docs/104 §2 owed).
- **Pressure is kinematic:** a by-product of motion (docs/103 §4).
- **docs/103 started at the wrong floor:** kinematics and hydrofluid as peers,
  nothing beneath.

## 1. The substrate's state (per place, first degree, shares)

At each place (a mote's reach in its enclosure, a ring, a body):

- **Potential `Φ`:** the hold the enclosure gives there, read on the face
  inverse gravity holds, `Φ = g = 1 − δ` (docs/104 §0). The deeper the hold,
  the more energy a body has given up standing there.
- **Pressure `P`:** what the medium bears there,
  - hydrostatic: the column beyond it, `Σ m_j·δ_j` (built, aa776c9);
  - thermal: its heat's share of its density, `ρ·a`, `a` heat as state (§2);
  - magnetic: its open frames in step over its width (built).
- **Energy, three forms, one book:** heat `a·m` (the substrate's own),
  motion `m·ℓ` (the spin the kinematics carries), potential `m·Φ` (where it
  stands). In grains of holding, on the one ledger unit (`universal_ledger`).

## 2. Its laws

- **Conservation, one ledger:** heat + motion + potential, summed over both
  faces and booked across every exchange, is exact each echo. Light made is
  booked leaving (received or drained, the existing light ledger); nothing
  else leaves. Cooling's pull channel is heat moved to the other face
  (docs/104 §2), never lost.
- **The heat pair (docs/105 §3):** heating in at our unlocked face on the
  open frames, `u·e`; cooling out by the pull back and the light,
  `g + e − g·e`; heat stands at the master equation's equilibrium
  `a* = u·e/(u·e + g + e − g·e)`, approached in closed form.
- **Equilibrium, the restoring force:** a place stands where its pressure
  balances the pull across it, the hydrostatic balance on its face, as pair
  shares: `P_in/(P_in + P_out) = Φ_in/(Φ_in + Φ_out)` across a ring's two
  sides. A place pushed off it is pushed back by the difference: the
  substrate restores.
- **Saturation (the flop):** a state that would carry more than SUBSTRATE_C
  per K-step escapes as a typed outcome, the luminal bound's own, never a
  panic (docs/94 §1).

## 3. Kinematics propagated from it

- **Motion is the substrate's difference transported:** a body's spin changes
  by the pressure and potential contrast across it, read as pair shares, the
  rotor carrying it (docs/94 §1), bounded by SUBSTRATE_C. The torque stops being
  a source of its own: the bounce's push enters as the core's potential
  re-pulsed (its state against its throat, docs/105 §4.1), and what the torque
  gives is what the substrate's potential pays.
- **Heat is never a residue:** what the transport does not deliver in step is
  energy moved from motion to heat, booked on the one ledger (it was a share
  computed and dropped into agitation).
- **The coupling up to form:** a binding holds where its bond (docs/103 §6)
  meets the substrate's pressure across it (its thermal and magnetic pressure
  against its density), never its members' agitation alone.

## 4. What it replaces

| now | becomes |
|---|---|
| `flows::torque` / `spin_torque` as a source | the substrate's contrast transported; the torque the potential's payment |
| agitation as the torque's residue | heat as state, its pair equilibrium, booked |
| cooling `keep·keep`, no counterweight | the heat pair with the counterweight on the other face |
| `flows::fluid`'s pressure from agitation | P as state (hydrostatic, thermal, magnetic) |
| the spin exchange by pressure contrast as a law | the coupling's reading (motion from the substrate) |
| docs/103's layers as peers | stacked: substrate → kinematics → form |

## 5. The inverse

Every substrate reading names its face through docs/104's map: potential `g`
(held from outside in), heating by `u`, cooling by `g`. Its fixed point is the
ring, `g = u`, where pressure and pull balance with no contrast: the throat is
the substrate's still point.

## 6. Build order (each with its witness)

1. **The energy ledger** (heat, motion, potential, both faces): read only.
   Witness: its closure per echo reported beside the existing identities
   (expected open today: the size of what leaks is the first reading).
2. **Heat as state with its counterweight** (docs/104 §2, docs/105 heat pair).
   Witness: the ledger's heat term closes.
3. **Pressure as state; the hydrostatic equilibrium as the restoring law.**
   Witness: a medium at its balance moves nothing; displaced, it returns.
4. **Kinematics from the substrate:** the torque restated as the potential's
   payment and the contrast's transport. Witness: the spin ledger exact; the
   energy ledger closes.
5. **Form's coupling:** bindings against pressure. Witness: bindings counted
   where the bond meets the substrate.
6. **Read in short runs:** each layer's ledger and steady state, then the
   worlds.

## 7. Open

- The potential's zero (where `Φ` is read from: the throat's fixed point is
  the natural still point).
- Its coupling across tiers (a star's substrate inside its galaxy's).
- The flop's typed outcome in our code (today the luminal pass).
