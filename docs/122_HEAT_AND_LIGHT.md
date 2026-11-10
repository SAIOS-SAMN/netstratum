# 122 — Heat and light: from bonds and friction, out by the face

This document derives where heat and light come from and where they go.
Heat is made by bonds forming and by friction, it leaves as light through a
body's unlocked face, and one ledger books it across bond, heat, motion,
potential and light. It replaces the frame-count readings that set cooling
and starlight today, and the two starlight laws.

**Status:** derived (2026-11-05); steps 1 (the read-only ledger), 2 (emission by the face), 3 (bonds into heat, nuclear) and 4 (friction booked) built (2026-11-10). It answers register docs/119
B8 and B9 and the open decision "hydrogen's hold" (docs/PLAN.md). The
operator said on 2026-11-05: "why are we still stuck in frames :: where
does irradiation as heat and light come from? bonds+friction - sounds like
we are missing physics". It supersedes docs/106 §2's heat pair where that
pair heats and cools "on the open frames"; the rest of docs/106 stands, and
this is its S1 and S2.

**Semantics (2026-11-10):** docs/124 places each form on its level and
names it by nature's term: heat is motion out of step (the spread of a
body's turning), bond energy is the quantum level's, potential the state
level's, and light is in transit on no lane. Its §5 derives the star core's
heat from contraction, which this document's build order now carries
(step 5).

**Prerequisites:** docs/106 (the substrate and its ledger), docs/117 §8.2
(the one face law, `polarity::faces_on`), docs/104 (the inverse), docs/97
(the nuclear rungs, `u(z)`).

## 0. What the code does today (read 2026-11-05)

- **No bond makes heat.** Fusion makes light (`made·rung_light(z)` in the
  live burn; `pairs·rung_light(z)` at the throat). Chemistry closing,
  peeling and dissociation release or absorb nothing.
- **Friction makes heat:** the torque's undelivered share
  (`spin_torque`: `p·δ·(1 − |k|)`) and stars' encounters (`collide_stars`,
  into the reservoir).
- **Frames set the cooling:** `keep = (1 − open)·(1 − δ)` with
  `flows::open = unpaired/weight`, so hydrogen's `open = 1` wipes a hydrogen
  mote's heat every bell; `formation::radiates = unpaired/hold`, 0 for
  hydrogen (`hold = weight − unpaired = 0`).
- **Two starlight laws:** the live `made·rung_light(z)` (booked in `Light`),
  and `formation::burn`'s `feed·fuel·hold(z)/total·dm` (0 for hydrogen, so
  ignition's heat is always 0; unbooked).
- **The throat's light is used twice:** spent on peels (`recycle`'s pool)
  and shone whole.
- **Rounding makes heat:** the enfold scars (phase-rounding residue) are
  added to agitation.
- **No energy ledger.** The light ledger `made = received + drained` closes
  by construction; heat made, cooled, trapped or fallen is booked nowhere.

## 1. The forms of energy (one book)

docs/106 §1 books three, heat `a·m`, motion `m·ℓ` and potential `m·Φ`. The
fourth is missing, and it is where heat and light come from:

- **Bond** `B`: the energy held in a body's bonds: the dipole coupling's
  closing work (docs/121 §9–§10). Magnetism is the open frames (ruling of
  2026-10-15), so a bond forming is open frames pairing, poles closing. Its
  energy is counted in the frames that close: a fusion pair closes
  `2u(z) − u(2z)`, and a chemical closing its grip. Frames enter here as the
  moments they are, never as a rate of radiation; radiation is by the face
  (law 3), which retires `radiates` and `open` in cooling (register K5).

Light is the fifth entry, energy in transit: made where heat leaves a
body, received where a body takes it, drained where nothing does.

## 2. The laws

1. **Bonds release into heat, never into light.** A bond formed releases its
   energy into the heat of the body it formed in: a fusion pair
   `2u(z) − u(2z)`, a chemical closing its grip. A bond broken takes its
   cost from that body's heat (a peel `u(z − k) + k·u(1) − u(z)`, a
   dissociation). `B + heat` is unchanged by a bond's change.
2. **Friction moves motion into heat.** The share of a motion not delivered
   in step leaves motion for heat (decoherence: heat is motion out of
   step, docs/124 §3): the torque's undelivered share, an
   encounter's grip, the shear's unkept share. `motion + heat` is unchanged
   (docs/106 §3: heat is never a residue).
3. **Heat leaves as light through the unlocked face.** A body's heat `a`
   emits `a·u` a ring, `u` its unlocked face read on its own axis (one face
   law: the continuum `1 − δ`, the vacuum `δ`) and keeps `a·g`. Every kind
   radiates by where it stands, never by its frames; hydrogen radiates.
   What leaves is light made.
4. **Light is taken on the unlocked face.** Light reaching a body enters its
   heat by that body's unlocked face (the existing `shine`); the rest
   drains. `light made = received + drained` (the existing ledger).
5. **Heat stands where its sources meet its emission.** With a body's heat
   in from bonds, friction and light received, `q`, a ring, and out `a·u`,
   its heat approaches `a* = q/u` in closed form, `a_n = a* + (a₀ − a*)·gⁿ`:
   the master equation of docs/105 with the face as its rate, no frames.
6. **One ledger closes each echo:** bond + heat + motion + potential + light
   in transit, over both faces. Its witness is that closure, read beside the
   spin identities.

## 3. What they replace

| now | becomes |
|---|---|
| live burn `made·rung_light(z)` as light | the burn's bond energy into the star's heat (law 1); its starlight is its emission (law 3) |
| `formation::burn`'s `fuel·hold(z)/total·dm` | retired: one starlight law (B9) |
| ignition's `heat = light·unlocked` (always 0) | the binding's bonds and its infall friction into heat (laws 1, 2) |
| `keep = (1 − open)(1 − δ)` | emission by the face, `a·u` (law 3) |
| `radiates = unpaired/hold` | retired: emission is by the face (B8 dissolves: hydrogen's hold no longer enters heat or light) |
| the throat's light spent on peels and shone | the press's bond energy into heat; the peel's cost from it; what remains emits (laws 1, 3) |
| the enfold scars as heat | remainders carried on the coordinate (register J10), never heat |
| no energy ledger | law 6 |

The frames keep their other roles (magnetism and the push, the bonding
gates, the collapse bond `μ`); none is heat or light.

## 4. The inverse

On the vacuum's axis the faces exchange (docs/117 §8.2): a vacuum mote
emits `a·δ` and keeps `a·(1 − δ)`. The throat (`δ = ½`) emits and keeps
alike: the still point of docs/106 §5.

## 5. Build order (each with its witness)

1. **The ledger, read only** (docs/106 S1, with the bond form). Witness: its
   closure per echo reported; the first reading is the size of today's
   leak. **Built (2026-11-10):** `conductor::energy::read`, an `energy` line
   per universal echo and over the run in `breathe`; heat, potential and bond
   booked, motion left to the spin identities; no sum claimed to close until
   the forms share a unit (docs/124 §8). First read (2^40, 580 bells): heat
   0 → 0.0265 unit masses, of which light received is 0.00032 (1.2 %), the
   other 98.8 % from channels booked nowhere (the torque, rounding,
   encounters); potential 0 → 0.123; the bond (open frames) *rose* 0.302 →
   0.420 (+39 %) while 0.0037 of light was made: frames are unpaired far
   faster than fusion closes them (the throat's peels, `recycle`); light
   made = received + drained exactly.
2. **Emission by the face** (law 3), retiring `open` in `keep` and
   `radiates`. Witness: a body's heat approaches `q/u` in closed form, at N
   and 4N rings; hydrogen emits. **Built (2026-11-10):** `flows::emission`, `flows::emit`,
   `flows::heat_after` (EQUATIONS E54): the torque only makes heat; galaxy
   and ocean motes emit over the torque's two rings, a system's motes and
   worlds on its star's bell, the light shone on the continuum face and
   booked; `keep = (1 − open)(1 − δ)` and `formation::radiates` retired (the
   gas shells keep `δ` a ring; their saved record moved from tag 6, now
   refused, to 31). Witnesses: the closed form at N and 4N rings; every grain
   of heat booked as light; hydrogen emits; the vacuum stream keeps the
   exchanged face. Read on the same founding (2^40, 580 bells) before and
   after: light made 0.0037 → 0.110 unit masses (received 0.018, drained
   0.092, balanced); the heat the unbooked sources make, now visible beside
   its emission, 0.026 → 0.126; stars born 26 → 11, those holding hydrogen
   1 of 21 → 8 of 9 (births with no hydrogen falling in 17 → 2); hydrogen
   burned 0 in both (the star core's heat is step 5); every identity exact.
3. **Bonds into heat** (law 1): fusion's rung energy and the throat's press
   into heat; peels and dissociation from it; one starlight law. Witness:
   `B + heat` unchanged by every bond change; the ledger's bond and heat
   terms close.
4. **Friction booked** (law 2): the torque's undelivered share, encounters
   and the shear's unkept share, motion to heat. Witness: `motion + heat`
   unchanged by each. **Built (2026-11-10):** motion is booked as `Σ m·ℓ`, which
   is angular momentum and stays exact under every spin identity, so the
   payer is read from docs/124 §3 instead: friction turns motion *out of
   step* into heat. The torque's undelivered share is paid by the push (the
   bounce carries on `p − Σ m·|Δℓ| − heated`, EQUATIONS E20); an encounter's
   heat is the relative motion its grip ends, the pair's first-degree spread
   about their mean turning (E56); the ocean's torque, with no bounce
   downstream, is a named source. `Event::heat_made` names every source and
   `breathe`'s energy line reports the heat still unaccounted. Open: the
   payer of a bounce's parting (the state level), the shear's unkept share
   (the shear keeps `Σ m·ℓ` and makes no heat today).
5. **Contraction heating** (docs/124 §5): a collapse's released
   potential, less the motion kept in step, into the star core's heat; the
   ignition cut and the outburst re-cut read `open_at` at that heat,
   retiring `ionizing = 1`, the re-cut's heat 0 and the `hot-heart`
   constant. Witness: `potential + motion + heat` unchanged across a
   collapse; a core deep enough strips its hydrogen and keeps it, one too
   shallow does not ignite.
6. **Scars to remainders** (J10). Witness: no rounding enters heat.
7. A read of one founding before and after each step.

## 6. Open

- The chemical grip's energy in ledger units (`chem::closing` gives a
  tightness, a share; its energy is that share of what).
- Where a star's emitted light starts its passage: at its surface (its
  reach) or at its star core.
- The scale on which heat meets a winding's tightness (docs/124 §8 item 1).
- Potential's zero (docs/106 §7).
