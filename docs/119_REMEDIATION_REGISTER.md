# 119 — The remediation register: the audits of 2026-11-05

Every remediation the audits of 2026-11-05 found, one row each, with where it
stands, what is wrong and what remedies it. It is the register docs/118 Part IV
works from, and it corrects docs/118 where the audits contradict it.

**Status:** all 9 audits recorded (§A–§J). Nothing here changes code.

**Method:** nine readers, read-only, against main `8e6f3d32` (the code is the
same on `law/poles`), each reading its slice in full and citing file and
line. Paths are under `engine/conductor/src/` unless given.

**Severity:**
- **S1, wrong behaviour:** a bug, or a law that does not do what it is for.
  Fixed whatever the redesign.
- **S2, contradicts the foundation or a ruling:** squares, scalar
  collapse, cartesian structure, imports, a reach map.
- **S3, drift:** one law written twice, a doc the code does not do, dead or
  orphaned code, a stale record.
- **S4, standards and naming:** MU-STANDARDS, the GLOSSARY, file layout.

Remediations marked **(F)** wait on the foundation's state type (docs/118
Part IV step 2); the rest can be done before it.

**Done:** B6, B10, B12, A11, A13, J2, J3, J5 (PR #58). The order of the rest
is [docs/PLAN.md](PLAN.md).

---

## §A. Cartesian structure in expansion and contraction (the deepest)

The state is a coordinate tuple, and every law reads it axis by axis. This is
the root under §C's phase collapse, and every row here waits on one carrier
for the plane state and one generator for motion (docs/118 F2–F4).

| id | sev | where | finding | remediation |
|---|---|---|---|---|
| A1 | S2 | flows.rs:52-57, :164-194; pulse::Body | The state is a tuple: `SPACE = [0, 2, 3]`, `LOOP = 1`, phases indexed per axis, tilt a partition over three axes, two spin magnitudes | (F) One carrier per plane (its whole and its span) and one generator for motion; per-axis numbers become readings |
| A2 | S2 | flows.rs:887-903 (`turn`), :1069-1071, :1208-1221 (`spin_torque`) | The per-bell law advances each axis on its own, coupled only through the scalar δ; a plane torqued below zero turns half a turn on its own | (F) One generator acting on the plane state |
| A3 | S3 | flows.rs:957-964 and 75-81; flows.rs:1213-1220; breath.rs:969-972 | "Through the centre" written three times | One law, called at every tier |
| A4 | S3 | breath.rs:897-898, 965-994 | The star's pull and ride re-implement `spin_torque`'s orthogonal torque inline, without the band | Stars and motes through one torque law |
| A5 | S2 | breath.rs:916-954; flows.rs:1927-1942 (`settle_disc`); formation.rs:2055-2065 | Phases returned, tilts relaxed and worlds turned component by component | (F) As A2 |
| A6 | S1 | pulse.rs:1347-1350 (`orbit_galaxies`) | Every spatial plane gets the same `+4δ`, ignoring tilt; inconsistent with `turn`'s `2δ·t_k` | One turning law for cores and motes |
| A7 | S2 | flows.rs:441-450 (`step_with`), 604-609 (`bonded`), 413-422 (`torsion`), 152-154 (`space_mean`), 1326-1334 (`shear`); formation.rs:1159-1169 (`meetings`) | Closeness as sums of per-axis differences (L1 sums, means over three axes) | (F) Closeness as one reading of two plane states |
| A8 | S2 | flows.rs:556-571 (`pair_sums`), 497-507 (`repel`); breath.rs:697-709, 1234-1236, 1271-1273 | Products over four planes (a fourth power when the planes are alike); `repel` clamps each axis to ±½, a box | (F) As A7 |
| A9 | S2 | flows.rs:1669-1680 (`beads`) | Fibres keyed by 16 orthants; a fibre straddling ½ on any axis is cut. Feeds `merge` and `fluid` | Gather by the plane state's hemispheres, not each axis's |
| A10 | S2 | flows.rs:2191-2210 (`band_pairs`) | The disc picked by largest tilt index; members binned on `⌊phase·count⌋`, a fixed grid anchored at phase 0 (against the 2026-10-23 "no grids" ruling) | Arcs by gaps on the plane state (as `phase_beads` does) |
| A11 | **done (#58)** S1 | flows.rs:1785-1792 (`strands`) | Extent as a bounding box over axes, and it ignores the wrap at phase 0: a strand straddling 0 reads an extent near 1 | Circular spread; fix the wrap now |
| A12 | S2 | breath.rs:556-570, 1680-1687, 1584-1591; formation.rs:1841-1850 | Landing, capture, orphan placement and system joins by radial length differences, never phases (against "phases, not frames", 2026-10-27) | Membership by in-step phase and the hemisphere law |
| A13 | **done (#58)** S1 | breath.rs:2112, 2149, 2153, 2159 | Every throat puncture, seal and cauterize booked on plane 0 | Book on the plane the crossing is in |
| A14 | S3 | flows.rs:2558, 2737 | `phase.get(1)` instead of `LOOP` | Name the plane |
| A15 | S2 | flows.rs:638-668; pulse.rs:1032-1039; flows.rs:89 | `[f64; 3]`, `(0..3)`, a hand-written table of six axis pairs, a `1/3; 3` default | Planes named by role, not position |
| A16 | S2 | horizons.rs:28-29, 67-76, 143 | `[usize; 8]` orthant bitmask; hard-coded three-way tie test | Superseded (docs/117 §10); restate on the pole basis or retire |

**Expansion and contraction written as two pieces of code, not one law and its
inverse (S2 unless marked):**

| id | outbound | inbound | what differs |
|---|---|---|---|
| A17 | galaxy to ocean, breath.rs:2215-2246 | ocean to galaxy, breath.rs:548-590, 1967-2003 | Not inverse: an absolute value drops the sign, landing re-places matter by the founding formula, both legs discard phases and tilt |
| A18 | throat release, breath.rs:1877-1958 | throat entry, breath.rs:2048-2105 | Different criteria and code |
| A19 | rim exit, flows.rs:1459-1471 | arrival, breath.rs:1979-1986 | Different criteria |
| A20 | star outflow book, breath.rs:131-160, 737-747 | star capture book, breath.rs:1732-1789 | Outflow books `ℓ` only, capture `ℓ` and `ℓ_⊥`; phases relative coming in, absolute going out |
| A21 | continuum band, flows.rs:1190-1191 | vacuum band, flows.rs:1192-1198 | Two laws for one band; the vacuum one unguarded and through the reach map |
| A22 | `ring::inverted`, `tube` (unused) | `flows::through` (live) | Two throat inversions coexist. **`inverted` and `tube` removed (#63)**; `through` (2H − R, ruled out 11-05) remains, replaced by the passage on the poles basis (PLAN step 3) |
| A23 | nuclide release, nuclides.rs:284-309 | captures, nuclides.rs:367-397, 404-429 | Three pieces with opposite fill orders; `nuclides::exchange` is the one-law form |
| A24 | recycle press, recycle.rs:149-184 | recycle peel, recycle.rs:185-262, trap 284-320 | Two laws, each with a fill order |
| A25 | outward transport: `shear`, `fluid`'s field share, `formation::exchange` | none | Hard-coded "outer gains"; only `restoring` is signed both ways |
| A26 | dilation into a frame, breath.rs:1185, 1764-1769; formation.rs:1806 | out of it, formation.rs:2018, 1452; pulse.rs:1340 | Hand-multiplied at every site; `ring::dilation` unused |
| A27 | `merge`, flows.rs:2277-2366 | none but `recycle::part` | One-way |

**All-pairs and cross-product loops (S3):**

| id | where | size |
|---|---|---|
| A28 | flows.rs:547-582 `pair_sums` (via `repel`) | n(n−1)·4 over the firmament, BigInt |
| A29 | flows.rs:677-694 `collections` | n²/2, screened |
| A30 | pulse.rs:746-785, 906-913 `cascade` | K²/2 over founding kinds; Σ run² for density (once) |
| A31 | breath.rs:548-570 | ocean motes × galaxies inside the arrival loop |
| A32 | breath.rs:1225-1240 | stars × every galaxy mote, a linear filter (`within_reach` would bisect) |
| A33 | flows.rs:2020-2025, 2086-2095 `band_pairs` | up to 256 strip steps × bounds × n |
| A34 | flows.rs:808-817 `hold_within`, 783-792 `rim` | O(n) per call, per kind and band (`hold` bisects) |
| A35 | breath.rs:1776 | a new system `Enclosed` per captured take |
| A36 | formation.rs:1826-1950 `settle_system` | up to n rounds × n log n |

**Era 0 (S2, demolition):** the 3-torus of coordinate triples
(`ledger::torus_distance`, `lib::spatial_l1` and their ~20 call sites);
`seating`'s 3×3×3 lattice boxing the era-1 universe; `orbit.rs`'s L1 diamond;
`commissions`' grid; the 64 tiles; one n³ and four n² loops (lib.rs:6779-6787,
2607-2615, 6007-6011, 7190-7203).

---

## §B. Wrong behaviour (S1, fixed whatever the redesign)

| id | where | finding | remediation |
|---|---|---|---|
| B1 | pulse.rs:1005 | No law ever creates a galaxy: only the founding makes a core (the home), so every non-home branch (galaxy orbits, tides, galaxy clocks) is dead in a fresh universe. Every read so far happened inside the home galaxy | Derive and build galaxy formation from released matter (issue #7), or state that one galaxy is the scope |
| B2 | breath.rs:556-570; ocean.rs:69-80 | The ocean never lands: the home's reach is 0, so its swing is `(0, 0)`; matter exiting to the ocean never returns | Landing on the plane state (§A12), or a home swing derived from its own reach |
| B3 | formation.rs:1715-1716, 2078 | `settle_system` clears a system's motes and writes back only a manifest: the spray, captured motes, birth discs, warming and residue shifts lose their state at each settle | Keep the system's motes through the settle. **done (#66)**: each mote read with its own state; the belts written back as motes |
| B4 | breath.rs:1341-1343 vs 1036, 1129, 1562; 1305, 1365; 1048-1078 | Spin books disagree: the slip shed books twice or the other three book too little; decohered and spent stars credit spin to the medium while their motes also carry it; an emptied remnant credits nothing | One outflow book; an identity on every passing path. **done (#67)**: the slip shed booked once; a passing credits the medium only what its motes and worlds do not carry (`carried_spin`). The decohered, spent and emptied paths are not yet exercised by a read |
| B5 | breath.rs:1985-1994, 2231-2233 | Arrivals and exits re-set spin with no identity. Read 2026-11-05: an exit carries a mote to the universe as `ℓ_g + ℓ·f` (`f` the galaxy's share), but an arrival is not its inverse: it reads `\|ℓ_m − ℓ_g\|/f` (the sign lost) and adds a fresh landing balance `r·H(<r)`, so a round trip makes spin and nothing pays it | A derivation first: the arrival as the exit's inverse (signed), with the landing's balance paid by the galaxy's reservoir under the torque identity, witnessed by a round trip. **done (#69) for the spins** (docs/120 §5): the arrival is the exit's signed inverse and places nothing; the phases' re-set at both crossings stays open |
| B6 | **done (#58)** breath.rs:1044 with 2016 | Remnant infall's phase is pushed as a 2-vector and replaced by zero | Carry the star's phases |
| B7 | polarity.rs:32-34 vs breath.rs:391, formation.rs:684-687, 1013, 1020 | The unlocked face is `δ` in `polarity::faces` (and everything nuclear) and `1 − δ` in light, systems and strikes: both at once | One face law, read from the plane's axis (docs/117 §8.2) |
| B8 | chem.rs (`hold`); formation.rs:436-441, 1009-1014, 61-67 | Hydrogen's hold is 0 under `weight − open`, so `burn` makes no light for hydrogen, ignition heat is always 0, and `radiates(H) = 0` against its own doc | Settle hydrogen's hold; one starlight law (B9) |
| B9 | breath.rs:1433 vs formation.rs:441 | Two starlight laws: live light is `made·rung_light(z)`; `burn`'s own light feeds only `system()` and printouts | One |
| B10 | **done (#58)** life.rs:205 | `chem::closing(a.z, …)` passes protons where weights are expected | Pass weights |
| B11 | flows.rs:1191 | `toward_rest` gets the swung reach and the stored `ell_o`: before and after are different quantities | One reach |
| B12 | **done (#58)** pulse.rs:2037; breath.rs:484 | A tape with `t_u = 0` loops forever | Refuse it on read |
| B13 | Body.load; breath.rs:2701 | `load` is read in the period law and never set by a law (always 0) | Derive its writer or retire it |
| B14 | pulse.rs:2047-2056 | The legacy tag-19 read counts the firmament twice until the next cut | Retire the legacy read (§H, superseded layouts). **done (#64)**: tags 2 and 19 refused |
| B15 | formation.rs:2079-2086 | `read.residual` is zero by construction: a tautology, not a witness | Witness the books from both sides. **done (#68)**: the input partition of every mote and world against the spin before, and the matter identity (held after + escaped = held before), IDENTITY system and settle matter |
| B16 | flows.rs (`luminal`, `shear`, `settle_disc`, `held`, `exits`, `beads`, `strands`, `band_pairs`, `fluid`) | Nine laws read vacuum motes on the continuum δ, while `turn` and `spin_torque` invert it | One face reading (B7) |

---

## §C. The physics against the foundation (S2)

| id | where | finding | remediation |
|---|---|---|---|
| C1 | all laws meeting bodies in step (§A7-A12) | The phases: circles turned inside planes, which a hyperbolic plane cannot hold (docs/117 §10.3); the largest collapse | (F) Derive which turnings between planes the phases read; move the laws onto them |
| C2 | flows.rs:182 and 10 sites; recycle.rs:341, 354; breath.rs | `Mote.vacuum`: one bit, four meanings (face of the pull, route direction, parity, merge label), six writer sites in three modules, documented "never reassigned" | (F) The face as the axis a plane is read from; the route and the label separate |
| C3 | flows.rs:282-298, 1187 (`band`, `toward_rest`); docs/104 §6.3 | Second degree in R: the rest point `R·R = 2q` (and `R*·R* = w̄_c/w`) is a square; `toward_rest` tests on it | Restate the band's rest in first degree |
| C4 | flows.rs:1173 (`spin_torque`) | Cooling `keep·keep` | First-degree cooling, or rule the two passes (see C11) |
| C5 | pulse.rs:209 (`reach_of`) | `2t/(1 + t·t)`; used only by `tier` | Retire with `tier` (C8) |
| C6 | flows.rs:826-827; breath.rs:789; pulse.rs:499 | `δ` read from spin is `H²/(ℓ + H²)`: an implicit square in every `reach` | Rule whether reading δ from ℓ is allowed (docs/118 F1 silent) |
| C7 | flows.rs:1386; breath.rs:1918; formation.rs:1767; flows.rs:2461, 75 | The reach map `through` (live) and the scalar Möbius and antipode | (F) The passage on the pole basis (docs/118 F6) |
| C8 | pulse.rs:219-247 (`tier`); life.rs:45-52, 383, 445; seating.rs:138 | Imports still live: `tier`'s shares, `HYDROGEN_K`, `CONTACT_K`, `BELL_YEARS`, the 279° anchor, ln 2 ≈ 693/1000 and 1442/1000. CHANGELOG says docs/116 retired them | Retire as docs/116 derived; correct CHANGELOG |
| C9 | pulse.rs:410, 466; breath.rs:1907, 1924 | docs/116's placeholders still in code: the ½ home push, `push_of` ½, the ½ release, the isotropic ⅔ release phase | Build docs/116's derivations |
| C10 | ~100 sites (`ratio(1, 2)`: flows 53, pulse 20, breath 16, formation 15) | ½ as a factor, against the 10-28 rule (½ only at the throat's fixed point) | Each to a pair share or the throat |
| C11 | compound.rs; polarity::after; formation cooling; burn | Compounding `(1 − x)ⁿ`, a quantity times itself n times, under F1's "no product of a quantity with itself" | Rule compounding (it was ordered, 2026-10-20, over halving) |
| C12 | flows.rs:286-290 (`toward_rest`); formation.rs `collapse` (24-step bisection) | Search by halving and bisection, against construction not search | Closed forms |
| C13 | 97 `ring::grain` sites; flows.rs:103-130 | No grid gate (the 2026-10-20 cut-over ruling); `tilt_from` grains and redistributes | Build the gate, or rule the carrier |
| C14 | Universe.t_u; breath.rs:470, 484, 512, 800, 1284, 1392; pulse.rs:1117; clocks; life | The schedule runs on the stored 186-bell period at nine sites; the established period is a printout | Schedule on the established period |
| C15 | pulse.rs:971, 631-953 | `unit` from the retired dispersal founding; `cascade` survives only to make `t_u` | Retire with C14 |
| C16 | siting.rs:82-86 | Era-0 founding on a Kepler track with squares and cubes, still the fallback | Demolition (§F) |
| C17 | flows (`Σ m·(ℓo + ℓ⊥)`), `ℓo = Σ|lk|` | Two planes' spins summed into one ledger number; plane spins summed in L1 into one reach | (F) |
| C18 | ring (the turning law); flows::{shear, repel, fluid}; formation::Stretch::mu; the heart's push | Orbits are not defined by shear from the magnetic attractor with its inverse in repulsion: an orbit is the turning law alone (gravity's held share); attraction (opposite spins pairing in gas) and repulsion (open frames pushing, in the firmament and the heart) are separate laws in separate places, neither shaping an orbit; the shear that reaches the orbit is gravity's contrast and runs outward only | One signed magnetic coupling, attraction and repulsion one law and its inverse, its exchange on the turning lane the shear, the orbit its first-degree balance with gravity (docs/121) |

---

## §D. One law written more than once (S3)

| id | the law | its copies |
|---|---|---|
| D1 | the face pair | `flows::face`, `polarity::faces` |
| D2 | the holding | `flows::holding`, `pulse::holding`; `universal_state`, `universal_ledger`, `whole_u`, `whole` |
| D3 | `H(<·)` | `Enclosed::hold` (by spin order), `hold_within` (by place): they disagree |
| D4 | the rim | `Enclosed::rim`, `exits`' `2·base`, `whole()` |
| D5 | the capture phase | `spray` Z·p, `founding` Z/2, the release's own |
| D6 | the tilt attractor `2δ/(1+δ)` | `settle_disc`, the star return near breath.rs:870 |
| D7 | the band overlap share | `held_share`, `band_pairs::share_in` |
| D8 | the spin exchange `u = s·gap/m` | `shear`, `fluid`, `around` (only `shear` guards overdraw) |
| D9 | closeness on the sphere | `repel`/`pair_sums` (Π over four planes), `step_with`/`bonded` (Σ over three) |
| D10 | coherence | `breath::coherence`, `flows::in_step` (one formula), the clock coherence, `Π in_step` |
| D11 | the ring law | inline at formation.rs:83, 602, 686, 820, 1837, 1963, ocean.rs:107 (δ); 1056, 2023 (ℓ); 601, 652, 709 (R), while `ring::spin`, `radius` are orphans |
| D12 | the rung law `2u(z) > u(2z)` | recycle.rs:62-65, formation::burn 402-411 |
| D13 | own reach | `ring::pinch`, breath's Apollonian capture, formation's tether sphere |
| D14 | the home | `pulse::is_home`, `chronometry` (heaviest core), `life::home` |
| D15 | the bulge | `ancients::in_bulge`, `places::in_bulge` |
| D16 | the push | `push_of`, `galaxy_push`/`push_cut` |
| D17 | the galaxy frame | built five times in breath (89, 680, 1838, 2671, 2615) |
| D18 | the system clock | `system_clock`, breathe's own copy |
| D19 | recomputed in place | `heart_cut` twice (1155, 1175), `ignition_share` twice, `read_cut` twice an echo |
| D20 | the founding's law | `flows::founding` (test-only), breath.rs:1974-1995 |
| D21 | `carried`, `heart_cut` | the same ℓ formula copied; the weight enters twice |
| D22 | phase and orbit | `motion::Turning`, `quantum::hidden_phase`, the stepped `Body.phase`, `orbit::Orbit` (era 0, the one that moves lines) |
| D23 | distance | `spatial_l1`/`torus_distance` (era 0), `places::distance`, `nearest`, the unused `enclosure::route` |
| D24 | clocks | the bell and `T_u`, galaxy terms, local years, the keepers; era 0's universal clock, 60-tick year, per-world `now()` |

---

## §E. What the docs say and the code does not (S3)

| id | where | the claim | the code |
|---|---|---|---|
| E1 | flows.rs:29-32, 1454-1456; docs/104 §6.3 | two exits, R ≥ H | one exit, on `ℓ_o` at `2·base` (docs/96 §7) |
| E2 | flows.rs:175-181 | `vacuum` never reassigned | four laws write it |
| E3 | flows.rs:185 | the lever never more than half | capped at the amount |
| E4 | flows.rs:614 | collections from `repel`'s moves | from the whole bounce's phase deltas; `repel`'s return discarded (pulse.rs:1264) |
| E5 | flows.rs:2130 | opposed leads cancel | tilts are unsigned partitions |
| E6 | flows.rs:972-973 | the orbit phase stands half a turn on | only the reading is antipoded |
| E7 | flows.rs:2499-2500 (`fluid`) | a pressure contrast | signed `restoring` plus a column |
| E8 | flows.rs:1167 | never a square | `keep·keep` |
| E9 | flows.rs:846-851, 920 (`turn`) | the route, not the inversion | both; `felt` reads the uninverted δ |
| E10 | flows.rs:1415 (`spray`) | never two antipodes | false at p = ½ |
| E11 | flows.rs:1295, 2473 (`shear`, `luminal`) | at the torque's rates | the unswung reach |
| E12 | flows.rs:1521-1536, 1899 | — | doc blocks misplaced (on `Work`, `settle_disc`) |
| E13 | nuclides.rs:207-211, 311-313 | `exchange` reads `p·δ`; `neutron_share` is `2δ/(1+δ)` | `p·(1−δ)`; returns `1−δ` |
| E14 | nuclides `grip`, `survival`, captures, `release`; breath.rs:1274-1277, 1308-1311 | a pulse phase `p` | a held share |
| E15 | formation `bond`, `burn`, `heart_cut`, `collapse` docs | old laws (no τ, ν; `3Z_product`; cumulative hold; "smallest" vs "largest") | the current laws |
| E16 | breath.rs:5-27, 495-496, 769-773, 1003-1004, 1165-1167, 1256-1258, 640-641, 1987-1988, 2210-2212 | the module header's shedding and compression; phases advance by the clock; infall loop ½; spin by the square; spin-down covers outflow | none of these as stated |
| E17 | pulse.rs:16-17, 1053-1067, 1732-1734, 291, 329 | era-0 tiering; the tape's tags; state and heat zero for other bodies | stale |
| E18 | ocean.rs:11-21; places.rs:13-14; recycle.rs:24-25; life.rs:21; motion.rs ("nothing is stepped") | retired crests; distance by route; peel cost at z 67, 106; mass by A = 2Z | stale |
| E19 | Cargo.toml | flows reads Mycelium's `physics::source` | no such call |
| E20 | Event fields `impacts`, `grown.1`, `fates.2`/`.3`, `weights[0]`/`[1]`; `Write::Parallel` | reported | always zero; never built |

---

## §F. What runs, era 0 against era 1 (S2–S3)

| id | finding | remediation |
|---|---|---|
| F1 | The live binary (`main.rs`) requires an era-1 universe, breathes it each beat, then runs the whole era-0 economy on top; the economy "runs again: operator 2026-10-21" (main.rs:862-863). Era 0 is about half the crate and the body of each beat | Decide the beat: era 1 alone, with the economy as demolition (foundations first) |
| F2 | A ~1.5k-line bridge (seating, places, waking, attachment, gauges) turns era-1 worlds into era-0 box positions, seven star classes, L1 orbits and a 60-tick year | Replace each translation with the era-1 reading (docs/89 converged order) |
| F3 | The conductor is one crate with every era-0 dependency; era 1 cannot be built alone | An era-1 physics crate |
| F4 | Orphans: `ocular.rs` (empty), `lookahead.rs`, `enclosure::place`, `route`, `pulse::from_nuclei` (the court-to-universe manifest: the hypha's resources cannot enter), `flows::manifest`, `founding`, `torsion_signed`, `soc::choose_build`, `nuclides::{place, binds, mobius, swapped, exchange, electron_phase}`, most of `ring`, `polarity::after`, `chem::closed`, `formation::extent`, `Body.gas` (no producer) | Wire or delete; wire `from_nuclei` |
| F5 | Untested era-1 modules: `enclosure`, `ocean`, `waking`; `breath` 5 tests in 2,965 lines, `pulse` 9 in 2,405, `formation` 5 in 2,312; `lib.rs` none in 8,133 | Witnesses with the state type |
| F6 | docs/89's breaks: B2 fixed; B1 and B3 mostly fixed (B1 with B10's new bug); B5 and B7 changed; B4, B6, B8 present | Carry B4, B6, B8 forward here |
| F7 | `ns-hypha::refound` still originates "the flat L1 torus, periods 64 cells" (refound.rs:192) | The era-1 origin |
| F8 | `enfold::residue` never receives the tier map's k (breath.rs:932): `Residue::Frame` never produced | The tier map (docs/89 B6) |

---

## §G. Rulings, the task list and the process (S3)

| id | finding | remediation |
|---|---|---|
| G1 | Five documents compete as "what is next": docs/PLAN.md, docs/102 §4, the 10-28 resume memory, docs/118 Part IV, the 11-03 memory (the table is the product). None says which governs | One governing plan; the rest marked superseded |
| G2 | The verification standard drifted: PRs #50 and #54 verified by byte-identical readings and wall-clock time, through the `NETSTRATUM_PROFILE` env var, against the 2026-10-20 ruling (re-derivation, N vs 4N counters, no env-var switches) | Re-verify #5's work by re-derivation and counters; retire the env-var profiler |
| G3 | Laws judged on reads of 380-580 bells, against "judge no law on 24-600-echo reads" (2026-10-19) | State the read length a law needs |
| G4 | Rulings with no memory file: 10-26 (R the orthogonal orbit), 10-31 (the court's option (a)), 11-04 (the seeded founding), 11-05 (the passage rulings, "we don't square", the foundation) | Record each |
| G5 | Rulings the record contradicts: "no cells" (docs/110, 111, 102, issue #8, CHANGELOG); the unlocked face swapped in code while docs/102 D1 says "confirm"; "no draws at the founding" (10-16) superseded only in docs | Reconcile each |
| G6 | Rulings with no code: trunks refill the reservoir (10-18); one congruent orbit spacing (10-19, `formation::dance` gone, nothing replaced it); the vessel; the home unique by elements | Track each |
| G7 | Issues stale: #5 (the third bounce is reached), #6 (the rim is read; the schedule switch remains), #18 (done), #33 (decided, docs/113 merged), #44 (superseded by F2 if ruled); #4 partly done; #9 may need a v2 under F2/F6 | Close or re-scope |
| G8 | Issues overlapping: #13, #32 and docs/116; #30 and #3; #29 and docs/118 II.3; #8 and F6; #18 and #38; #12 and the heat item; #14/#15 and R's plane | Merge |
| G9 | The consolidation track (#28-#31) would consolidate laws docs/118 marks contradicting or collapsed | Hold behind the foundation |
| G10 | Untracked work: PR #56 (the foundation, no issue, no `cross-project` label though it holds LAW-PASSAGE); branches `law/electron-band` (docs/104 §1.1's correction, parked) and `law/torsion-attractor` (held), no PR or issue; merged-then-reversed `ring::{inverted, tube}` (#42) and `horizons::read` (#55) with no clean-up issue; PRs #52, #53 contract-touching without the label | An issue each; labels |
| G11 | **done (#65)** docs/102's register last edited 10-31; knows nothing of docs/109-118; D1 implemented while marked open; B7 and C3 no longer valid; docs/118 replaces §4 without saying so | Re-cut docs/102 from this register |
| G12 | AGENTS.md:50, 58 and CONTRIBUTING.md:19, 32 require `scripts/check`, which the operator called theatre (11-04); the done-when memory needs a live fleet while services are down | Amend both |
| G13 | Memory: ~20 stale entries still read as entry points (revival, resume, core reflection); three files unindexed (economy-doing-layer-gap, live-verification-methodology, no-biologic-naming); `opt-mycelium-antiquated` names the old vendor pin | Prune and correct |
| G14 | `/opt/netstratum` (no remote, HEAD d88fc8f, Mycelium v0.2.0) is still this session's working directory and the memory key, while the truth is `/opt/netstratum-public` | Retire or mark it |

---

## §H. Naming and MU-STANDARDS (S4, the physics ambiguities S2)

| id | finding | remediation |
|---|---|---|
| H1 | **The physics' letters are ambiguous:** Z is protons in `pulse::nuclei_by_z` and neutrons in `nuclides` and the GLOSSARY; R, p and Q each mean two things in Netstratum's own GLOSSARY (S2) | One meaning each; restate the GLOSSARY |
| H2 | One word, many things: "tier" (4), "frame" (5, in a codebase whose law is "phases, never frames"), "line" (2), "ring" (2), "plane" (2), `hold` (5 functions), "bounce" (4), "fold" (2), "seat" (2), "census" (2), `founding` and `mobius` (2 each), `grip` (3), `World`, `Place` (3), `Reading` (3), `Shell`, `Keeper`, two `Holding` types | One name, one meaning (Mycelium AGENTS) |
| H3 | Collisions with Mycelium's meanings: the `hypha` crate (Mycelium's hypha is the server; this crate is the house and its uplink, and its `judge` files an answer), Breath, Ascension, Orbit, Shell, Horizon, Gauge, torsion, residue/residual, peel, mint (μ only), Judge, Act, stroke, Seat, Site, chart, fibre, Frame, epoch, Cover, `unfold`; Xylarium's conductor | Rename to the function |
| H4 | Files named for a story, not their contents: `breath.rs` (one 2,283-line function), `pulse.rs` (the universe model, founding, tiers and codec), `waking`, `ascension`, `soc`, `civic` (the economy), `realms`, `ocular` (empty), `refound`, `siting`/`seating`/`places` (three near-synonyms); the `pulse` binary founds a universe | One organ per file, named for it (MU-STANDARDS 1.2) |
| H5 | Over 2,500 lines (1.1): lib.rs 8,133, flows.rs 3,930, breath.rs 2,965, cfe/dop.rs 8,938, kernel/lib.rs 3,315, cli main.rs 9,517, cli hubs.rs 3,269 | Split by organ |
| H6 | `main.rs` and `lib.rs` holding logic (1.3, 1.5): conductor `main` 1,390 lines, `observe` 1,148, `breathe` 898; hypha's nine commands in one file | Argument parsing, boot order and loop only; commands under `src/bin/` |
| H7 | Names the rules refuse: `*_CAP`, `*_MAX`, `PATIENCE`, `LATTICE` (2.6); `*Refused`, `*Error`, `*Fault` instead of `*Refusal` (2.10); `bool` functions without `is_` (2.3); `*Context`, tuple aliases (2.1); generic `f`, `of`, `u`, `step` ×4 (2.2); acronyms `soc`, `dop`, `vrp`, `rcf` (2.7); American spellings (`center`, `fiber`, `color`) | Rename |
| H8 | Headers: 103 of 243 carry dates, rulings or commit ids (2.8); none carry the AI-generated header | History to the record; add the header |
| H9 | Tests: 259 of 385 named as sentences with articles (3.1); retired verbs (`reads_back`, `exactly`); "replay" as a claim | `<unit>_<behaviour>` |
| H10 | `snapshot.rs` still reads superseded layouts v1 and v2 (§10) | Refuse by name |
| H11 | Person words in names, logs and lore (2.11); era and version words (`era1`, `aeon`, `g2_*`, `MAGIC_V1`, `LegacyReceipt`, `_v2`) | Remove |

---

## Corrections to docs/118 the audits require

- **II.2 audits orphans as running laws** (most of `ring`, the nuclide pass functions, `enclosure::place`): §F4.
- **`band`, `toward_rest` "follow"**: they square (§C3). `pulse::{push_of, galaxy_push, home_push}` "follow": they are docs/116's placeholders (§C9). `founding`, `cascade`, `deploy` "follow": `cascade` lives only for the contradicted `t_u`, `deploy`'s unit from the retired dispersal (§C15). `Enclosed`, `exits` "follow": two `H(<·)` and three rims (§D3-D4).
- **Part III T3 "galaxies built"**: no galaxy is ever made (§B1).
- **Missing**: the unlocked face's two definitions (§B7), hydrogen's light (§B8), the discarded system motes (§B3), the spin books (§B4-B5), compounding (§C11), the implicit square in `δ(ℓ)` (§C6), the fourth power of `Π in_step` (§A8), siting's Kepler track and life's imports (§C8, §C16), the duplicated laws (§D).
- **II.2's last row** restates docs/89 B1 and B2 as open; both converged (§F6).

---

## §I. The design record against itself (docs/78–118)

All 40 documents were read in full. The record contradicts itself on its most
basic laws; every row below is S2 or S3 and is remedied by one statement
in the foundation that the others are corrected to.

**Contradictions on the base laws (S2):**

| id | the question | the documents that disagree |
|---|---|---|
| I1 | One law at every tier? | docs/78 DL4 "One law at every tier" (never amended); docs/85 §5 "too linear"; docs/89 "one law ✔"; docs/118 F7 attributes "not one law" to docs/78, a misquote |
| I2 | Stepped or closed form? | docs/78 DL5 and Decision 2 "never stepped", "circuitry, never steps"; docs/89 step 4 "the stepped `Body.phase`"; docs/94 K-steps |
| I3 | The period | docs/83 `P = 1 + R/H`; docs/108 O2 still `P = r/M(<r)`; docs/101 §0, docs/107 still `r/(r + M(<r))` |
| I4 | The exit | docs/96 §7 `R ≥ rim = 2·base` (built); docs/92, 104, 111, 112 `R ≥ H` |
| I5 | Where matter stands | docs/92 inside the throat; docs/94 nothing orbits inside the ring; docs/99 outside; docs/103 most inside |
| I6 | The release | docs/104 §3 heavy kinds more; docs/104 §1.2 and docs/112 §3.1 heavy kinds more slowly |
| I7 | The faces | velocity `1 − δ` (docs/98) or `δ` (docs/117, 118); drag `1 − δ` (docs/83, 117) or `δ` (docs/104); the unlocked face `½(1 − δ)` (92, 96), `1 − δ` (102), `δ` (104) |
| I8 | Which leg is the disc | docs/99 the disc is the continuum; docs/102 the vacuum leg runs home through the disc |
| I9 | Does a star's heart have a throat | yes (docs/83 FL, 96 §2, 99); no (docs/91 C6, 92 §2, 111 §3.1) |
| I10 | The throat | `R_b ≤ H_b` (docs/102, 111); `A = Q` (docs/117 §10, 118 F6); how the two relate is never derived |
| I11 | Möbius and the passage | `M∘M = id` (docs/94, 97); "not an involution" (docs/114 T4); `−q` after two (docs/114 §9); `R → 2H − R` built (docs/89, 112), refused (docs/113, 114); `i·q` (docs/114 §9) against `v_k·q` (docs/117 §9.4) |
| I12 | The bound share | phase-dependent count (docs/104 §1.1); the held share `g`, phase out (docs/105 §3) |
| I13 | The neutron share | ¼ retired (docs/97 §9); "the constant ¼" (docs/105 §4.1); docs/97 §9's 6/7 and 1/6 inconsistent as written |
| I14 | The push | five forms: `held/(1 + held)` (89), `½·Q/(A + Q)` (95), `½·u/hold` (96), `Q/(A + Q)` (104), `H_disc/(1 + H_disc)` (83) |
| I15 | Twist | stands (docs/100 §2); grows outward (docs/104 §2–3) |
| I16 | Closeness | two-sided owed (docs/85 §6b, before docs/99); one-sided built (docs/99 §2.2); a product over planes ruled (78 N1) and refused (85 §2) |
| I17 | Planes | one turning per plane (85, 110); a plane holds no turning (117 §10); 180 planes and "capacity never restricted" (85) against three (95) against four over eight axes (117, 118) |
| I18 | R | ten meanings (§I-terms); paired with none (95, 117 §1) against its own plane (117 §10, 118); backward on the vacuum leg (95) against always forward (114 T4) |
| I19 | Z | the charge (docs/78, still the manifest's "nuclei per Z" in 84, 109, 118 F9) against the neutrons (97 onward) against the closed pole (95, 117, 118) |
| I20 | Weight | `3Z` (95), `Q + Z` (97), `3Z − unpaired` (104) |
| I21 | Coherence | a share (78 Br5) against an integer W (81) |
| I22 | The rim and the universal period | rim `H`, `2·base`, `H(<R_rim)`; `T_u` 60, `2·base`, `2·a(R_rim)`, `4·R_rim` ≈ 5, stored 186 |
| I23 | The home | the universal core's disc at `[]` (101) against core `[0]` (111, 112) against the heaviest core (89 B5) |
| I24 | Galaxies | by the crest grammar (101) against by the pinch test (111) |
| I25 | Flatness and squares | "never through flat space", "non-Euclidean" (78, 85) against "still flat" (114 §9.2); `ℓ·ℓ = r·H` rejected (83) but `R·R′ = H·H` "no square" (114 T1); docs/83's "no squares is geometry, not arithmetic" against docs/118 F1's arithmetic |
| I26 | Rotors | the sandwich (docs/94) against one-sided, never the sandwich (114, 117) |
| I27 | The signature | state (117 §2) against fixed (4, 4) (118 F2) |
| I28 | Spin per plane | built (docs/96 §6) against two magnitudes and a tilt (docs/118 II.1) |
| I29 | Wormholes and gates | none between galaxies (83) against throats between cores (78) against gates to other systems and the universal tier (109) |

**Built, then called wrong by a later document, with the code still in place
(S2):** `conductor::motion` (ω = M/r), the band atom and `A = 2Z`, the
tidal mass split, the 279 K climate and seating, the seed-drawn torsion
phase, life's K and grips, collisions by `s·D ≥ T`, the dispersal founding,
the ½ regeneration, `life::wild`, `flows::{through, mobius, rest}`,
`formation::infall`, the docs/92 reactor, the two-plane leftovers, the ½
release, the libration, `echo_holds`, the static quarter and the 1200 cap,
`held = (1 + R)/2`, the whole-mote luminal flip, the one-sided step,
torsion's `/2`, `toward_rest` "compared on the squares", `ring::{inverted,
tube}`, `horizons::read`, and the band's rest `R*·R*`. **Remediation:** each
retired in code or restored in the record, never both standing.

**Treated as settled while open, or the reverse (S3):** docs/79's
Calabi–Yau "unproven" listed as established (docs/101 §0); the docs/80 Q
lane (superseded) cited as a carrier; docs/100 §3's open coupling used in
docs/101; docs/103 §5's inversion awaiting confirmation but applied as law
in docs/104–111 (and docs/102 D1 still "confirm"); docs/95 §6's proposed
even split built; docs/106's heat-as-state open but used in docs/110–111;
docs/111 "not built" though docs/112 built §5.1; docs/112 §7.1 "closed"
while the schedule runs on 186; docs/117 "changes no live physics" though
`horizons::read` was built on it; docs/118 F1 "ruled" though newer than the
laws it forbids and silent on compounding; docs/102 L5, L6 open but decided
in docs/103 §9; docs/101 "nothing built"; docs/92 unbuilt but read as
present by docs/94; docs/83's `L = r·v` refuted by its own SP1 and never
withdrawn. **Remediation:** every document's status line re-read against
the code.

**Terms with two or more meanings across the record (S2 where the physics
reads them):** R (ten), Z (three), Q (six), A (three), plane (eight),
frame (nine), vacuum (eight), horizon (six), throat (eleven), ring (ten),
coherence (four), κ, σ, τ, w, μ, λ, p (four or more each), push (five
forms), unlocked face (five), held share (three), rim (four), shell (six),
cell (forbidden, then defined), filament/strand/flow, binding/bond/clump,
heart/reactor, home (four), bell (six), grain (three), equator (three),
degree (two), rigidity (two), loop, state (three). **Remediation:** one
meaning each in the GLOSSARY, the rest renamed (with §H1–H2).

**Foundations the record assumes and never states (S2), each owed to the
foundation (docs/118):**
1. a body's full state (which fields are state, which readings);
2. where heat lives and its unit;
3. what the phases are;
4. the bell and ring arithmetic ("two rings a bell", `ν = 2·r_echo/T_u`,
   "a core ring = `T_u/4` bells");
5. the unit of mass and hold at each tier;
6. the enclosure's order (inside out as built, outside in as decided);
7. what `T_u` is;
8. whether the S^60 and S^360 tier spheres still stand beside the S⁷ tower;
9. how the throat `R = H` relates to `A = Q`;
10. the potential's zero and the energy ledger's unit;
11. the manifest's key under the inverted nucleus (the hypha counts nuclei
    "per Z" while Z is the neutrons);
12. where electrons live, against `chem::unpaired`;
13. what a pulse phase p is (`Body.bell`, `φ_LOOP − bell_LOOP`, `φ_⊥`);
14. the court's algebra (Cl(n+1, 1) against Netstratum's (4, 4); docs/84's
    `TURNING` and the sensorium on Cl(4, 1));
15. who or what is an observer under "render on observation";
16. which time is primary (closed forms, K-steps, proper time, the beat);
17. whether compounding `(1 − s)ⁿ` is allowed under F1.

**docs/118's own errors:** it misquotes docs/78 DL4 (I1); it cites
"docs/117, proposed §11", which does not exist; its II.2 marks `push_of`
and `band` "follows" (a ½ halving and a square); it audits neither the
anchors (docs/92), θ (docs/81) nor the gauges of mind (docs/82).



## §J. MU-STANDARDS and MU-RECURSION

MU-RECURSION's laws that bind Netstratum, as a kernel or at what crosses the
universal core, and how the code stands against them. MU-STANDARDS and
MU-RECURSION's coding rules bind Mycelium's own tree; they bind Netstratum
only by adoption, which CONTRIBUTING.md claims ("matches the standards") and
the code does not do (§H).

| id | sev | law | where | finding | remediation |
|---|---|---|---|---|---|
| J1 | S2 | law 8, 78.62 L1, 78.59 Ruling 1, 78.169(3) | era-1 physics; `ns-hypha::probe` | Nothing of the era-1 universe reaches the court; only the fixed probe builds a transition. A body is scalars, not ground plus modes plus momentum, so a crossing cannot be composed under the court's algebra | (F) The state type in the court's form; crossings stated at the vertex (docs/113) |
| J2 | **done (#58)** S1 | law 4, 78.130, 78.45 | flows.rs:637-676 (`collections`, PR #54) | A float screen (`MARGIN = 1e-6`) in a physics decision; "certified" only by a comment, with no test of the bound | Mycelium's certified enclosures (`Encl`) in place of f64, or the exact test alone |
| J3 | **done (#58)** S1 | law 9, 78.21, 78.152 | hypha/src/house.rs:662-666 | The SEATS epoch posted on the wire is `SystemTime::now()`, a wall clock; beats paced by `sleep` (main.rs:345, 1388; hypha main.rs:402, 679, 741) | The epoch from the ledger's TUniversal point |
| J4 | S2 | law 12, 78.43 | pulse.rs:595; breath.rs:470-490; main.rs:368, 911 | A stored, frozen period drives the schedule against the derived one (§C14), and is sealed into the origin's radius, which can never be edited | The established period; the origin's radius from it at the next founding |
| J5 | **done (#58)** S1 | 78.121 | hypha/src/lib.rs:80-102 (`agree`) | Drops unmeasured laws, so "unmeasured" on one side and "breached" on the other reads as agreement | Unread is not agreement |
| J6 | S2 | 78.154, §1.5 | hypha/src/refound.rs:96-108 | The ledger commitment is over file paths and blake3 digests of file bytes; the court can check bytes, never reconstruct the state | Canonical records with exact values |
| J7 | S2 | §42.7 | flows.rs:1386, 3645-3660 | The passage a scalar reach map; the tests pin LAW-PASSAGE v1's fixtures, which Mycelium has since struck; the dead `H²/R` model still cited in EQUATIONS.md:1711 | (F) The passage `v_k·q`; tests to v2's fixtures; EQUATIONS corrected |
| J8 | S2 | 78.45(2), law 1, rule 11 | formation.rs:243-259; flows.rs:292-308 | Fixed-count forward guesses: 24 bisections for the collapse width, 64 halvings then 0 | Closed forms (§C12) |
| J9 | S2 | law 2, rule 15 | ring.rs:231-237 (`birth_key`) | A body's identity is the bell plus a 60⁶-mark sector, bumped +1 on a collision: it depends on the reading's resolution and on processing order | Identity as the body's content |
| J10 | S2 | laws 4, 7 | ring.rs:192-195; enfold.rs:28-55, 134-138; flows.rs:226-231; pulse.rs:964-969; breath.rs:480 | Rounding to fixed sexagesimal places; residue below one nucleus leaves the state with only a tally, and the tally is floored | Remainders carried, never discarded (§C13) |
| J11 | S3 | §0.0 | nuclides.rs:21 (`READ = 8192`); ring.rs:193, enfold.rs:167 (60, 60⁶); pulse.rs:40-44, 66-69 (1024·60³⁰); the 24 and 64 counts; ocean.rs:72-74 (default 2) | Underived numbers not named as open physics; `tier`'s imports registered (docs/108) but live | Register each or derive it |
| J12 | S3 | 78.122, 78.152 TU-5 | hypha/src/lib.rs:174-201; main.rs:916-931; ledger/src/lib.rs:46, 169, 212; chem.rs:116 | The lexicon version is returned and compared with nothing; `TPoint` hand-parsed, keeping `h` only; an in-game `TUniversal{S,G,T}` collides with the clock; `protons(w).unwrap_or(0)` files weights past 8192 under Z = 0 | Compare the version; `TPoint::parse`; rename; refuse past the read |
| J13 | S2 | 78.165, 78.169 | refound.rs:491-493; house.rs:96-98 | A local throat's keeper is `line_key`, blake3 of a name: a key nobody holds | A held key |
| J14 | S3 | 78.62 L0, 78.169(3) | EQUATIONS.md:1726 | Only LAW-DILATION is declared, with no version; the hypha never reads `LAW_VERSION` | Declare each law with its version; read L0's |
| J15 | S3 | 78.132, .139, .140 | refound.rs; fold.rs:254-282 | The origin commits the seed only, not the laws as ratios or the drawn parameters | The Big Bang record whole |
| J16 | S3 | 78.31, 76.1 | pulse.rs:219-247; breath.rs:1584-1591 | Tiers as a `match depth` table; orphaned worlds to the "nearest" star by `min_by_key` | Recursion through the core; placement by law |
| J17 | S4 | rules 4, 15, 11; MU-STANDARDS 2.4 | flows.rs:1307, 1684, 1699; pulse.rs:1120; `unwrap_or` 374 times | Index tie-breaks (`.then(a.cmp(b))`); invented values for missing ones | Ties held; refusals, never defaults |

**Honoured:** polytonic lanes never summed (law 5); amendment versus fork
and the origin never edited (78.59 Ruling 2, 78.132); the shared law forms
passed in (78.131); the manifest as nuclei per Z, unit-free (78.155b,
with J12's defect); the Concordance signed (78.169(4)); the reserve's
designation (78.138); conservation witnessed internally (78.89, though
never stated as L0 `Flux`). Of J1–J17, three were not in Netstratum's own
record: J2, J3 and J13.

