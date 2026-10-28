# 102 — Alignment: one vocabulary, one register of what is owed

Operator 2026-10-28: "whats the coherent picture with aligned semantics for
implementation"; "lets find our alignment across all the topics we have
identified that need remediations or forward development". Status: REGISTER,
kept current; each item closes with its commit and witness.

## 1. The vocabulary (one meaning a word, every tier)

| word | meaning | code |
|---|---|---|
| **core** | a body that holds matter in its throat and bounces each bell | `pulse::Body` |
| **throat** | what a core holds unreleased | `Body.throat` |
| **state** | what the throat held at the core's forming; set once, never drained | `Body.state` |
| **bounce / bell** | a core's re-pulse in full in its own units; one a bell | breath's arrivals |
| **push / unlocked face** | the bounce's carriage on a kind; the face the core does not lock, `1 − δ` | `push_cut`, release |
| **release** | the tide's share leaving the throat a bounce, `s = ½·c/(c + base)` | breath's release |
| **flow** (the FILAMENT) | a core's released matter turning on its jetstream loop; the ocean is the universal core's flow | `Body.flows` (`Universe.ocean` folds in, docs/101) |
| **mote** | one kind's share of a flow, its state carried forward | `flows::Mote` |
| **R loop, legs** | the circulation; the continuum leg out along the axis, the vacuum leg home through the disc | `Mote.phase[LOOP]`, `Mote.vacuum` |
| **Möbius pass** | `(δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)`: a mote's leg turned | `flows::mobius` |
| **luminal bound** | every mote on the face whose turning share is at most ½ | `flows::luminal` |
| **deficit δ** | gravity's held share at reach R, `H/(R + H)`; the push's `1 − δ` | `Enclosed::reach` |
| **enclosure** | a core's hold read once: base `state/g`, ring = base, rim = `2·base`, the strip between | `flows::Enclosed` |
| **ring (of flow)** | a band pair on the strip, `k(ρ) = (S/G)(1 + ρ)/(1 − ρ)` | `flows::band_pairs` |
| **strand** | motes bound in a chain, judged by their bond (was "filament" in `flows::chains`) | `flows::Strand`, `flows::strands` |
| **bond** | `λ/(1 + λ)`, `λ = 2(m + μ + τ)/R`: holding, magnetism, torsion | `formation::bond` (read by the crest, strands, ignition, worlds) |
| **binding** | elements of one ring bound into one body where the bond meets heat and tide | band_pairs' output |
| **step** | a member's in-step share with its binding's phase, `C = Σ t_k·in_step` | `flows::step_with` |
| **heart** | what falls (C ≥ ½); its cut keeps what its hold holds | `formation::heart_cut` |
| **disc** | a binding's members off the held hemisphere, placed by their phase | `formation::system` |
| **ignition** | a heart that holds against its heat becomes a core one tier down | `ignition_share` |
| **firmament** | the universal core's held cut: the heaviest and node-shell kinds | `Universe.firmament` (folds in, docs/101) |
| **home** | the universal core's own disc, unique by its elements | core `[0]` today (rebuilt, docs/101) |
| **twist** | a core's winding per plane, `½·t_k` a ring | `quantum::twist` |
| **torsion τ** | the lead a body keeps between its planes against its enclosure's | `flows::torsion` |
| **nuclide** | weight `w = Q + Z` on the convergence; Q the open protons, Z the fibred neutrons, R the electrons' phase | `nuclides::*` |
| **place** | a nuclide's turning, open degree, claimed and paired shares | `nuclides::place` |
| **densest** | the founding's heaviest nuclide, where every climb stops | `Universe.densest` |
| **clock** | rings received a bell, nested down the wells; TUniversal the universal core's bell | `galaxy_rate`, `nested`, `t_u` |

## 2. Rules of form (every item below is checked against them)

Phases, never frames. No leftovers. No statics (a constant is a placeholder
for missing physics). One writer per state. Exact and bounded, closed forms.
One grammar, coupled tiers, each tier its own mechanism. Formulas in the
docs before code; witnesses by re-derivation, never equal bytes.

## 3. The register

### A. Remediation (misaligned machinery; no new physics)

| # | item | where | depends | ruling |
|---|---|---|---|---|
| A1 (CLOSED e4b74a9) | the leg flag's two writers: `stream` undoes `luminal` | `flows::stream`, breath | — | — |
| A2 (CLOSED e4b74a9) | tilt dropped on save for firmament and ocean motes | `pulse::write` tags 19/24 | — | — |
| A3 (CLOSED e4b74a9) | "filament" names a bound chain: rename to **strand** | `flows::chains`, `Chain`, `stream`, docs | — | — |
| A4 (CLOSED e4b74a9) | two impact laws: `life::climb` draws from the seed (with T_u stood in at 60 by `bin/life`); `life::advance` measures on the hidden phase | `life.rs`, `bin/life.rs` | — | — |
| A5 (INTO B1) | nine readings of the universal state; `is_home` branches; `core_state` 0; the release span branch | pulse, breath, ocean (docs/101 §1.2) | B1 | — |
| A6 (INTO B1, B4) | three homes: `is_home` (path [0]), `life::home` and `chronometry::keepers` (heaviest kept); `places.rs` | life, chronometry, places | B1, B4 | — |
| A7 (CLOSED e4b74a9) | `formation::echo_holds` duplicates the crest's law | formation | — | — |
| A8 | `bin/observe`'s "cascade" view recomputes the cascade beside it | observe | B3 | — |

### B. Forward development (new physics, in order)

| # | item | doc | depends | ruling |
|---|---|---|---|---|
| B1 | the universal core as one body: firmament, home core, ocean fold into its held cut, its disc, its flows | docs/101 §2 | A1, A2, A5, A6 | — |
| B2 | the founding as its throat; `cascade` and `deploy` retire; `T_u` its echo (hypha reads tag 17) | docs/101 §2, §6 | B1 | **mycelium** (owner of TUniversal), `T_u` |
| B3 | galaxies formed from its flow (the universe tier's mechanism, the crest's grammar); `orbit_galaxies`' rings retire | docs/101 §2 | B2 | — |
| B4 | the home as its disc, unique by elements; the firmament the heavy base | docs/78, 83, 101 | B3, B5 | — |
| B5 | carriage by harmonic, the coupling C | docs/100 §3 | — | **operator**: C ((a) `q = l + 1` per xylarium's reading) |
| B6 | the electrons' transition (R = 0) on the light and chemistry ledgers; the poles' transition | docs/100 §9.1, §9.3 | — | **operator**: R's scale |
| B7 | the founding's slight beads stop standing at rest | docs/100 §6 | B2 | — |
| B8 | cost at the universe tier: every grain in motion; the zoom ruling (certified sectors, exact under observation) | ruling 2026-10-24 | B1 | — |

### B′. The layers (docs/103)

| # | item | doc | depends | ruling |
|---|---|---|---|---|
| L5 | production read on the matter's own face (the inverted strip) | docs/103 §5 | — | **operator**: both faces or the whole frame |
| L4 | pressure restated as the medium's state (hydrostatic, thermal, magnetic); the gradient's push the coupling | docs/103 §4 | — | — |
| L2 | chemistry of the medium: frames pair in step where R holds them | docs/103 §2 | B6 | — |
| L6 | the bond composed through the layers (nuclear, chemical, magnetic, torsional, gravitational) | docs/103 §6 | L2, L4 | **operator**: additive or nested |

### B″. The inverse duals (docs/104)

| # | item | doc | depends | ruling |
|---|---|---|---|---|
| D1 | the face map in one place (`held = 1 − δ`, `unlocked = δ`) | docs/104 §4 | — | **operator**: confirm the map (§5) |
| D2 | heating, cooling and the counterweight on a heat ledger | docs/104 §1–§2 | D1 | — |
| D3 | push and release on our unlocked face | docs/104 §1 | D1 | — |
| D4 | twist and its release by our held share | docs/104 §1 | D1 | — |
| D5 | the quantum state inverted, `h = (1 − R)/2` | docs/104 §1 | D1 | — |

### B‴. The substrate (docs/106), beneath every layer

| # | item | doc | depends | ruling |
|---|---|---|---|---|
| S1 | the energy ledger (heat, motion, potential, both faces), read | docs/106 §6 | — | — |
| S2 | heat as state with its counterweight | docs/106 §2 | S1 | — |
| S3 | pressure as state; hydrostatic equilibrium the restoring law | docs/106 §2 | S1 | — |
| S4 | kinematics propagated from the substrate | docs/106 §3 | S2, S3 | — |
| S5 | form's coupling: bindings against pressure | docs/106 §3 | S3 | — |

### C. Reading and after

| # | item | depends |
|---|---|---|
| C1 | re-found and breathe past bell 7000: galaxies formed, compositions, the home, stars and worlds per galaxy, identities, cost | B1–B4 |
| C2 | worlds with life: the gauges and the climb on a stable universe | C1, A4 |
| C3 | the live G2 universe (var-e2): re-founded only when cleared; services stay down; restarts coordinated with mycelium | C1 |
| C4 | vendor mycelium past 287cf10c (forms carry geometric elements, tag 16) before any restart | C3 |

### D. Closed this cycle (for the record)

Echo hold and solid orbits (40b4ad5); R at the pulse, the face as heat
(785a074); the founding's phase (cd16484); the symmetric luminal bound
(fa7378d); disc-leg membership, phase infall, burn by charge (32a7dfc,
3400588, bc2d32c); no fixed cap, densest 1612 (ff0c985); the twist, the
locked lead, torsion, twist in the bond and the fibres (65348e2, 4ffd2e7,
2db928e, a12af8c).

## 4. Order

1. **A1–A4, A7** (no physics change; each with its witness).
2. **A5–A6 fold into B1** (2026-10-28): folding nine readings over four holders now is rewritten when they become one body; pointing the homes at today's empty home core would break the standard year until B4.
3. **Rulings in hand:** C (B5), R's scale (B6), `T_u` with mycelium (B2).
4. **B1 → B2 → B3 → B4 with B5, B7**, then **B8** as the cost demands.
5. **C1**, then **C2–C4**.
