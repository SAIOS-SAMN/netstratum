# 103 — The layers: each its own mechanism, coupled by stated laws

Operator 2026-10-28: "bonding formula sounds too weak, or you are missing
sub-layers (underneath our known states)"; "pressure is kinematic, should be
formulaic"; "this entire universe is going to be inverted in order to
produce"; "we are combining multiple layers of physics here, where kinematic
and hydrofluids operate through different mechanisms"; "write docs/103 and
scope each layer". Status: SCOPE. Builds on docs/94 (fluid, MHD, UMD),
docs/96 (the enclosure, the hold law), docs/97 (Q, Z, R), docs/98 (the faces),
docs/99 (phases), docs/100 (twist, torsion, transitions), docs/101 (one
universal core), docs/102 (vocabulary, register).

## 0. The principle

A layer is a mechanism with its own state and its own law. Layers COUPLE:
one layer's state enters another's law through a coupling stated as a law of
its own. A layer never stands in for another (no pressure computed from
motion, no bond read from one layer alone), and every coupling is read on
the face where the matter actually stands (docs/98, the luminal bound).

| layer | mechanism | its state | code |
|---|---|---|---|
| L1 sub-atomic | the nucleus's convergence on S^11; the electrons' phase | Q, Z, place, R | `nuclides` |
| L2 chemical | open frames pairing into clusters | clusters, open frames | `chem`, `formation::layer_chemistry` |
| L3 kinematic | turning, torque, the loop, the faces | ℓ, phases, tilt, leg | `flows::turn`, `torque`, `luminal`, `mobius` |
| L4 hydrofluid (MHD) | the medium as a continuum: pressure, field, shear | per ring: density, pressure, field | `flows::fluid`, `flows::shear` |
| L5 gravitational | the enclosure: hold, deficit, rings, tide | H(<R), δ | `flows::Enclosed`, `band_pairs` |
| L6 the bond | the layers composed into one body | the bond | `formation::bond` |

## 1. L1 — sub-atomic (built; two transitions owed)

- **State:** each nuclide's Q (open protons), Z (fibred neutrons), place
  (turning, open degree, claimed and paired shares), R = κ_e·(1 − 2·frac(p)).
- **Laws (built):** grip `κ_w·h`, survival `κ + (1 − κ)·h`, captures to the
  densest, release where `a + τ > κ_w·h`, the burn's barrier by charge.
- **Owed:** the electrons' transition (R = 0) and the poles' (docs/100 §9.1,
  §9.3; R's scale for ruling), the coupling C (docs/100 §3, for ruling).
- **What it gives up:** each nuclide's GRIP (its fibres' bond) and its OPEN
  FRAMES at its pulse phase (R bound: its electrons hold; R free: open).

## 2. L2 — chemical (built in worlds only; owed in the medium)

- **State:** clusters (`chem::Kind` with parts) and their open frames.
- **Laws (built, inside worlds):** `layer_chemistry`: deep clusters part at
  their layer's heat; atoms with frames open close in their smallest whole
  cluster (`chem::closing`), the bond holding `min(1, τ_min/heat)` of the
  contact.
- **Misaligned:** the medium (a galaxy's flows, a system's) has no chemistry:
  motes never pair their frames while they travel, so every binding's
  magnetism is its members' raw open frames.
- **Scope:** chemistry of the medium, one law with the worlds': where two
  kinds' motes stand in step (their phases' in-step share, docs/99) and their
  open frames are bound (R > 0 at their pulse), their frames pair into the
  smallest closing cluster, the contact by the in-step share, the hold by the
  tightness against the heat. Nucleons conserved; light booked (the frames
  paired give their line, docs/100 §9.1).
- **What it gives up:** PAIRED frames (the chemical bond) and the frames left
  OPEN (magnetism).

## 3. L3 — kinematic (built)

- **State:** ℓ_o, ℓ_⊥, phases per plane and on the R loop, tilt (lead), leg.
- **Laws (built):** turning `2·share` a bell in closed form, the share δ on
  the continuum and `1 − δ` on the vacuum; the torque's hemisphere response;
  the route; the Möbius pass; the symmetric luminal bound; the attractor
  (disc and lead).
- **Misaligned:** nothing in its own law; its readings are taken by layers
  that do not ask which face the matter stands on (§5, §7).
- **What it gives up:** where each mote stands (reach, phase, face) and how
  it moves (its spin).

## 4. L4 — hydrofluid (built kinematically; to restate as a state law)

- **Now:** a ring's pressure is `P = (m·ā + M)/w` (holding times its members'
  mean AGITATION, plus its field M, over its width `2pR`): agitation is the
  torque's undelivered residue, so the pressure is a by-product of motion; and
  the pressure contrast between neighbouring rings moves SPIN outward by an
  exchange share each echo. Pressure is computed from motion and acts only
  as motion: no state of the medium stands.
- **Scope, the medium's state:** each ring of a face (`beads`) is a parcel of
  the continuum with
  - its DENSITY `ρ = m/w` (holding over width, as now);
  - its HYDROSTATIC PRESSURE, the hold of every ring outside it on its face
    borne inward, `P_h(R) = Σ_{rings beyond} m_j·δ_j` (the column's weight,
    gravity's held share at each; a reading of state, no motion);
  - its THERMAL PRESSURE `ρ·a`, a its heat as a state (the R crossings' line
    and the fibres' release, docs/100 §9, booked on the light ledger), never
    the torque's residue alone;
  - its MAGNETIC PRESSURE from its field `M` (open frames in step, as now)
    over its width.
- **Scope, the coupling L4 → L3 (a law of its own):** the medium pushes a
  body moving through it by the pressure GRADIENT across it, the contrast of
  the two sides it spans, `(P_in − P_out)/(P_in + P_out)` (first degree,
  saturating), times the body's own cross share; the spin it moves is booked
  on the ledger. The exchange of spin by contrast retires as the law; it
  stays the coupling's reading.
- **Scope, the coupling L4 → L6:** pressure SUPPORTS against collapse: a
  binding's heat in the bond's test is its thermal and magnetic pressure
  against its density, not its members' agitation alone (§6).
- **Open:** whether hydrostatic support is read per face (the inverse faces'
  columns) and how the two faces' pressures meet at the ring.

## 5. L5 — gravitational (built; read on the wrong face for production)

- **State:** each core's enclosure (base, hold within, whole), δ at each reach.
- **Laws (built):** `Enclosed`, the ring law, band pairs on the strip,
  the tide's contrast.
- **Misaligned (the operator's 3):** the strip (`1 < R ≤ 2`), the band pairs,
  the crest and the disc read matter on the CONTINUUM face, outside the ring,
  while the luminal bound now keeps most matter on the VACUUM face, inside the
  ring (47,577 passes by bell 1700). Production is read where matter does not
  stand; this is not a relabel (docs/98 §3 is corrected here: the faces are
  symmetric for motion, not for production).
- **Scope:** production reads matter on its own face: on the vacuum face the
  enclosure is read through the throat's inversion, `R → H²/R` (`δ ↦ 1 − δ`),
  so the vacuum's strip is the image of the continuum's, `H/2 ≤ R < H`, its
  band pairs, crest and disc read there by the same laws at the inverted reach.
  **For ruling:** production on each face at its own inverted strip (both
  faces produce), or the universe's whole frame inverted (what we call outside
  is inside, every tier).

## 6. L6 — the bond, composed through the layers

- **Now:** `bond(λ) = λ/(1 + λ)`, `λ = 2(m + μ + τ)/R`: gravity's holding,
  the raw open frames' magnetism, the twist. One layer between bodies; nothing
  beneath.
- **Scope, the composition (bottom up, each layer's bond in the same form, a
  body's share over its reach):**
  - **nuclear** `ν`: the members' grip, `Σ m·κ_w·h` (L1), the matter that holds
    as nuclei at the binding's heat;
  - **chemical** `χ`: the frames paired among the members in step (L2);
  - **magnetic** `μ_open`: only the frames left open after pairing (L2), in
    step (the field, L4);
  - **torsional** `τ`: the members' net lead (L3, built);
  - **gravitational** `m`: the holding (L5).
- **The rule, for ruling:**
  - (a) the layers add to the line density, `λ = 2(m + ν + χ + μ_open + τ)/R`,
    each a share of the same reach;
  - (b) the layers nest, each holding what the one beneath leaves free,
    `B = 1 − Π_layers (1 − b_layer)` (a body holds by any layer that holds).
- **Against it:** heat as pressure (L4 thermal and magnetic) and the tide
  (L5), read on the matter's own face (§5).

## 7. The couplings (each a law of its own)

| from → to | coupling | state |
|---|---|---|
| L1 → L2 | R decides which frames are open (bound or free) | owed (§2, docs/100 §9.1) |
| L1 → L6 | nuclear grip `ν` | owed (§6) |
| L2 → L6 | paired frames `χ`; open frames `μ_open` | owed (§6) |
| L3 → L1 | twist and torsion part fibres (`a + τ > κ_w·h`) | built (a12af8c) |
| L3 → L4 | members' motion sets the medium's rings | built |
| L4 → L3 | pressure gradient pushes bodies | owed (§4) |
| L4 → L6 | pressure supports against collapse | owed (§4, §6) |
| L5 → L3 | turning `2δ`, by face | built |
| L5 → production | strip, crest, disc on the matter's own face | owed, for ruling (§5) |
| light (L1 burn, L2 pairing) → L4 heat | the light ledger warms the medium | built in part |

## 8. Build order (each with its witness)

1. **L5 on the matter's face** (§5), after the ruling (both faces, or the
   whole frame). Witness: bindings formed on the vacuum face; the ring fixed
   (½ both faces).
2. **L4 restated** (§4): hydrostatic, thermal, magnetic pressure as state;
   the gradient's push as the coupling. Witness: the spin ledger exact; a
   medium at rest in its own pressure moves nothing.
3. **L2 in the medium** (§2): frames pair in step where R holds them.
   Witness: nucleons conserved; light booked; a cold in-step medium pairs.
4. **L6 composed** (§6), after the rule's ruling. Witness: a binding of
   sealed nuclei and paired frames holds where the one-layer bond did not.
5. **L1's owed transitions** (§1), after R's scale.
6. **Read:** re-found, breathe to bell 1700 and past 7000.

## 9. For ruling

- **§5:** production on each face's own strip, or the universe's frame
  inverted whole.
- **§6:** the bond's composition, additive (a) or nested (b).
- Standing: the coupling C (docs/100 §3); R's scale (docs/100 §9).
