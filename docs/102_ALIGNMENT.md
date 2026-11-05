# 102 — Alignment: one vocabulary, one register of what is owed

This document held the vocabulary used at every tier and the register of
every item built or owed across the era-1 documents, with each item's
source, dependencies and open decision.

**Status:** superseded as a register and as an order (2026-11-05). The
register is [docs/119](119_REMEDIATION_REGISTER.md); the order of work is
[docs/PLAN.md](PLAN.md), which replaces §4 below (docs/118 Part IV had
replaced it without saying so; PLAN.md carries Part IV forward). This
document is kept for its vocabulary (§1), its rules of form (§2) and its
rows as history. Its register was last kept on 2026-10-31 and knows nothing
of docs/109–118. Each row in §3 now carries its state on 2026-11-05 and the
docs/119 row that carries it, or "retired" with the reason; "unverified"
where neither docs/119 nor the code establishes it. New items go to docs/119,
never here (docs/119 G11).

The commit ids in this document (e4b74a9 and those in §3 D) are from the
earlier working tree; they are not in this repository's history.

## 1. The vocabulary (one meaning a word, every tier)

| word | meaning | code |
|---|---|---|
| **core** | a body that holds matter in its throat and bounces each bell | `pulse::Body` |
| **wavefront** | the geodesic sphere of light-time `t` about a source in its enclosure; an arrival is a wavefront meeting an orbit (docs/110) | `ring.rs` |
| **equator** | the fixed hypersurface of the inversion `δ ↦ 1 − δ`, `δ = ½`, where the faces meet (docs/110) | `flows::through` |
| **shell** | a band of reach on one face, fibred by the phase torus and the R loop; its state a field of cells (docs/110) | `flows::fluid` |
| **cell** | a shell's bead along one plane's circle, parted at gaps wider than the mean; it owns its arc (docs/110) | `flows::phase_beads` |
| **orbit torus** | a body's invariant torus, its motion a linear flow `φ_k(n) = φ_k(0) + n·t_k/P` (docs/110) | `motion.rs` |
| **throat** | the surface at a body's equator `R = H`, the tube's mouth; a body has one iff `R_b ≤ H_b` (docs/111 §3.1). `Body.throat` holds its unreleased contents | `Body.throat` |
| **state** | what the throat held at the core's forming; set once, never drained | `Body.state` |
| **bounce / bell** | a core's re-pulse in full in its own units; one a bell | breath's arrivals |
| **push / unlocked face** | the bounce's carriage on a kind; the face the core does not lock, `1 − δ` | `push_cut`, release |
| **release** | the tide's share leaving the throat a bounce, `s = ½·c/(c + base)` | breath's release |
| **flow** (the filament) | a core's released matter turning on its jetstream loop; the ocean is the universal core's flow | `Body.flows` (`Universe.ocean` folds in, docs/101) |
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

The vocabulary is kept as it stood. The record does not yet agree with it
everywhere: "cell" against the "no cells" ruling (docs/119 G5), the home
(D14, I23), the faces (I7), and the terms docs/119 §I and H1–H2 list. The
GLOSSARY is where each is settled.

## 2. Rules of form (every item below is checked against them)

Phases, never frames. No leftovers. No statics (a constant is a placeholder
for missing physics). One writer per state. Exact and bounded, closed forms.
One grammar, coupled tiers, each tier its own mechanism. Formulas in the
docs before code; witnesses by re-derivation, never equal bytes.

## 3. The register (superseded by docs/119)

Each row as it stood on 2026-10-31, with a last column for its state on
2026-11-05. **(code)** marks a state read in the code at main `33a738eb`;
the rest are taken from docs/119, docs/PLAN.md or the documents named.
Paths are under `engine/conductor/src/`.

### A. Remediation (misaligned machinery; no new physics)

| # | item | where | depends | decision | state (2026-11-05) |
|---|---|---|---|---|---|
| A1 (closed e4b74a9) | the leg flag's two writers: `stream` undoes `luminal` | `flows::stream`, breath | — | — | Closed then; the flag has many writers again: `Mote.vacuum` is written at six sites in three modules and carries four meanings. Now [docs/119](119_REMEDIATION_REGISTER.md) C2, E2 |
| A2 (closed e4b74a9) | tilt dropped on save for firmament and ocean motes | `pulse::write` tags 19/24 | — | — | Closed; not re-verified |
| A3 (closed e4b74a9) | "filament" names a bound chain: rename to **strand** | `flows::chains`, `Chain`, `stream`, docs | — | — | Closed: `flows::Strand` and `flows::strands` stand **(code)**. The words filament, strand and flow still overlap across the record: docs/119 §I (terms), H2 |
| A4 (closed e4b74a9) | two impact laws: `life::climb` draws from the seed (with T_u stood in at 60 by `bin/life`); `life::advance` measures on the hidden phase | `life.rs`, `bin/life.rs` | — | — | Closed; not re-verified. Life's imported constants are docs/119 C8 |
| A5 (into B1) | nine readings of the universal state; `is_home` branches; `core_state` 0; the release span branch | pulse, breath, ocean (docs/101 §1.2) | B1 | — | Open. Now docs/119 D2 (the holding and `universal_state`, `universal_ledger`, `whole_u`, `whole`) |
| A6 (into B1, B4) | three homes: `is_home` (path [0]), `life::home` and `chronometry::keepers` (heaviest kept); `places.rs` | life, chronometry, places | B1, B4 | — | Open. Now docs/119 D14 (the three homes) and I23 (the record's three readings of the home) |
| A7 (closed e4b74a9) | `formation::echo_holds` duplicates the crest's law | formation | — | — | Closed then; `formation::echo_holds` still stands (formation.rs:1645) **(code)**, and docs/119 §I lists it among laws a later document called wrong with the code still in place |
| A8 | `bin/observe`'s "cascade" view recomputes the cascade beside it | observe | B3 | — | Open: the view stands (`bin/observe.rs`, `"cascade"`) **(code)**. It goes with `cascade`, which lives only to make `t_u`: docs/119 C15 |

### B. Forward development (new physics, in order)

| # | item | doc | depends | decision | state (2026-11-05) |
|---|---|---|---|---|---|
| B1 | the universal core as one body: firmament, home core, ocean fold into its held cut, its disc, its flows | docs/101 §2 | A1, A2, A5, A6 | — | Open, not built: `Universe` still holds the ocean apart from the cores, and the home is core `[0]` (`pulse::deploy`) **(code)**. Now docs/119 D2, D14, I23; the body's state is [docs/PLAN.md](PLAN.md) step 3 |
| B2 | the founding as its throat; `cascade` and `deploy` retire; `T_u` its echo (hypha reads tag 17) | docs/101 §2, §6 | B1 | **mycelium** (owner of TUniversal), `T_u` | In part. The founding is the table in the universal core's throat, nothing placed outside it (docs/112 §2; `pulse::deploy`) **(code)**. `cascade` and `deploy` have not retired: `cascade` makes only `t_u`, and the schedule runs on the stored period. Now docs/119 C14, C15, J4; what `T_u` is, docs/119 §I item 7 |
| B3 | galaxies formed from its flow (the universe tier's mechanism, the crest's grammar); `orbit_galaxies`' rings retire | docs/101 §2 | B2 | — | Open: no law makes a galaxy (docs/119 B1); `pulse::orbit_galaxies` stands **(code)**, its turning law docs/119 A6; the two accounts of galaxy formation, docs/119 I24. Ordered in PLAN step 2 behind the coordinate |
| B4 | the home as its disc, unique by elements; the firmament the heavy base | docs/78, 83, 101 | B3, B5 | — | Open. Now docs/119 D14, I23, and G6 ("the home unique by elements", a ruling with no code) |
| B5 | carriage by harmonic, the coupling C | docs/100 §3 | — | **open**: C ((a) `q = l + 1` per Xylarium's reading) | Open, not built: no carriage by harmonic in the conductor **(code)**. docs/119 §I records the coupling as open but used in docs/101; no row of its own |
| B6 | the electrons' transition (R = 0) on the light and chemistry ledgers; the poles' transition | docs/100 §9.1, §9.3 | — | **open**: R's scale | Open: `nuclides::electron_phase` stands with no caller outside its tests **(code)**, docs/119 F4. R's meanings, docs/119 H1, I18; R's plane, PLAN step 3 |
| B7 | the founding's slight beads stop standing at rest | docs/100 §6 | B2 | — | **Retired.** Since docs/112 §2 the founding places nothing outside the throat: the firmament, the galaxies and the ocean begin empty (`pulse::deploy`) **(code)**, so there are no founding beads to stand at rest. What remains of it is the phases themselves, docs/119 C1 |
| B8 | cost at the universe tier: every grain in motion; the zoom rule (certified sectors, exact under observation) | — | B1 | — | Open. The cost is docs/119 A28–A36 (all-pairs and cross-product loops), remedied by ranges on the coordinate ([docs/120](120_THE_COORDINATE.md); PLAN principles). The zoom rule has no docs/119 row |

### B′. The layers (docs/103)

| # | item | doc | depends | decision | state (2026-11-05) |
|---|---|---|---|---|---|
| L5 | production read on the matter's own face (the inverted strip) | docs/103 §5 | — | **open**: both faces or the whole frame | Decided in docs/103 §9 (the whole frame inverted; its meaning, the enclosure counted from the outside in, awaits confirmation), as docs/119 §I records. Built: unverified. The enclosure's order, docs/119 §I item 6 |
| L4 | pressure restated as the medium's state (hydrostatic, thermal, magnetic); the gradient's push the coupling | docs/103 §4 | — | — | Open. The restoring law stands (S3); `fluid` reads it plus a column: docs/119 E7, A25 |
| L2 | chemistry of the medium: frames pair in step where R holds them | docs/103 §2 | B6 | — | Open: derived, not built (docs/118 Part III T7, on docs/115). No docs/119 row |
| L6 | the bond composed through the layers (nuclear, chemical, magnetic, torsional, gravitational) | docs/103 §6 | L2, L4 | **open**: additive or nested | Decided additive in docs/103 §9, as docs/119 §I records. Built: unverified |

### B″. The inverse duals (docs/104)

| # | item | doc | depends | decision | state (2026-11-05) |
|---|---|---|---|---|---|
| D1 | the face map in one place (`held = 1 − δ`, `unlocked = δ`) | docs/104 §4 | — | **open**: confirm the map (§5) | **Implemented, not in one place.** `polarity::faces` returns `(1 − δ, δ)` and the nuclear laws read it **(code)**; `flows::face` is a second copy **(code)**, docs/119 D1; light to worlds reads the unlocked face as `1 − δ` (breath.rs:390) and the strikes' face as `½(1 − δ)` (formation.rs:684-687) **(code)**; docs/119 B7, B16 list these and the other sites. The map was applied without the confirmation this row asked for: docs/119 G5, I7 |
| D2 | heating, cooling and the counterweight on a heat ledger | docs/104 §1–§2 | D1 | — | Open, not built: no heat ledger or counterweight in the conductor **(code)**. Where heat lives is open (PLAN, "Decisions open"; docs/119 §I item 2); cooling `keep·keep`, docs/119 C4 |
| D3 | push and release on the unlocked face | docs/104 §1 | D1 | — | Open. Now docs/119 B7 (one face law), D16 (the push written twice), I14 (five forms of the push), C9 (the ½ release) |
| D4 | twist and its release by the held share | docs/104 §1 | D1 | — | Unverified. The record disagrees on twist: docs/119 I15 |
| D5 | the quantum state inverted, `h = (1 − R)/2` | docs/104 §1 | D1 | — | Open: docs/104 §1.1's correction is parked on the branch `law/electron-band`, docs/119 G10 |

### B‴. The substrate (docs/106), beneath every layer

| # | item | doc | depends | decision | state (2026-11-05) |
|---|---|---|---|---|---|
| S1 | the energy ledger (heat, motion, potential, both faces), read | docs/106 §6 | — | — | Open, not built: no energy ledger in the conductor **(code)**. The potential's zero and the ledger's unit, docs/119 §I item 10 |
| S2 | heat as state with its counterweight | docs/106 §2 | S1 | — | Open, not built (as D2). Where heat lives, docs/119 §I item 2; PLAN, "Decisions open" |
| S3 | pressure as state; hydrostatic equilibrium the restoring law | docs/106 §2 | S1 | built (`flows::restoring`) | Built: `flows::restoring` and its witness `a_medium_at_its_balance_moves_nothing_and_displaced_it_returns` stand **(code)**. docs/119 A25 (the one transport signed both ways), E7 |
| S4 | kinematics propagated from the substrate | docs/106 §3 | S2, S3 | — | Unverified. No docs/119 row |
| S5 | form's coupling: bindings against pressure | docs/106 §3 | S3 | — | Unverified. No docs/119 row |

### B⁗. Own rulers (docs/108): the imported calibrations retired

| # | item | doc | depends | decision | state (2026-11-05) |
|---|---|---|---|---|---|
| O1 | heat read in grips (the lightest kind's τ), `HYDROGEN_K` retires | docs/108 §2 | — | — | Open: `life::HYDROGEN_K` still read (life.rs:45) **(code)**. Now docs/119 C8; docs/116 §4; PLAN step 5 (#32) |
| O2 | the bell in home years (`1/P_home`), `BELL_YEARS` retires | docs/108 §2 | — | — | Open: `life::BELL_YEARS` still read (life.rs:386) **(code)**. Now docs/119 C8; docs/116 §4 |
| O3 | a world sited by its own heat against the home's, the 279 K anchor retires | docs/108 §2 | S2, O2 | — | Open: the 279° anchor stands in siting (seating.rs:138) **(code)**. Now docs/119 C8 |
| O4 | a star read by its own burn, the class table leaves era-1 siting | docs/108 §2 | O3 | — | Open: siting still reads `cfe::thermal::StarClass` (seating.rs:144) **(code)**. Now docs/119 F2 (the bridge's seven star classes), C16 |
| O5 | the tier shares as each tier's equilibrium at its throat; `pulse::tier`'s constants retire | docs/108 §2 | S1, docs/105 §4.1 | — | Open: `pulse::tier` is still read by `life::window_echoes` (life.rs:448) **(code)**. docs/116 §4 retires it; now docs/119 C5, C8, J16 |
| O6 | docs/82's placeholders (pressure, radiation, a mind's size, a people's floor, spacing) with their laws | docs/108 §2 | S2, S3 | — | Unverified. No docs/119 row; docs/118 did not audit docs/82 (docs/119 §I, "docs/118's own errors") |
| O7 | the thermal regimes as bands of heat against the home's, edges at the grips | docs/108 §2 | O1, O3 | — | Open. docs/116 §4 retires kelvin with era 0's thermal model (#32); PLAN step 5; docs/119 C8. Not read in code |

### C. Reading and after

| # | item | depends | state (2026-11-05) |
|---|---|---|---|
| C1 | re-found and breathe past bell 7000: galaxies formed, compositions, the home, stars and worlds per galaxy, identities, cost | B1–B4 | Open. The read length a law needs, docs/119 G3; reads on a fresh founding (PLAN principles); the stages, docs/118 Part III |
| C2 | worlds with life: the gauges and the climb on a stable universe | C1, A4 | Open. docs/118 Part III T9; the era-0 bridge under life, docs/119 F1, F2 |
| C3 | the live G2 universe (var-e2): re-founded only when cleared; services stay down; restarts coordinated with Mycelium | C1 | **Retired.** The G2 universe on var-e2 was a data directory of the earlier working tree, `/opt/netstratum`, which docs/119 G14 marks for retirement in favour of this repository; nothing here refers to var-e2 but docs/101 and this document **(code)**. Reads are taken on a fresh founding (PLAN principles), and what the live beat runs is docs/119 F1. Coordination with Mycelium stands as a principle of PLAN.md, not as a row |
| C4 | vendor Mycelium past 287cf10c (forms carry geometric elements, tag 16) before any restart | C3 | **Done, by another route:** the workspace pins `mycelium-sdk` from GitHub at tag `v0.3.0` (Cargo.toml) and vendors no copy **(code)**. No docs/119 row |

### D. Closed this cycle (for the record)

Echo hold and solid orbits (40b4ad5); R at the pulse, the face as heat
(785a074); the founding's phase (cd16484); the symmetric luminal bound
(fa7378d); disc-leg membership, phase infall, burn by charge (32a7dfc,
3400588, bc2d32c); no fixed cap, densest 1612 (ff0c985); the twist, the
locked lead, torsion, twist in the bond and the fibres (65348e2, 4ffd2e7,
2db928e, a12af8c).

## 4. Order (superseded by docs/PLAN.md)

The order of work is [docs/PLAN.md](PLAN.md). The list below is kept as it
stood on 2026-10-31.

1. **A1–A4, A7** (no physics change; each with its witness).
2. **A5–A6 fold into B1** (2026-10-28): folding nine readings over four holders now is rewritten when they become one body; pointing the homes at today's empty home core would break the home year until B4.
3. **Open decisions:** C (B5), R's scale (B6), `T_u` with Mycelium (B2).
4. **B1 → B2 → B3 → B4 with B5, B7**, then **B8** as the cost demands.
5. **O1 → O2**, then **O3, O4, O7** with S2, **O5** with S1, **O6** with S2–S3 (docs/108 §4).
6. **C1**, then **C2–C4**.
