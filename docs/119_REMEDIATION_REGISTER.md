# 119 — The remediation register: the audits of 2026-11-05

Every remediation the audits of 2026-11-05 found, one row each, with where it
stands, what is wrong and what remedies it. It is the register docs/118 Part IV
works from, and it corrects docs/118 where the audits contradict it.

**Status:** 7 of 9 audits recorded (§A–§H). The audit of the design record
(docs/78–118) and the audit against MU-STANDARDS and MU-RECURSION are added
as §I and §J when they report. Nothing here changes code.

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
| A11 | S1 | flows.rs:1785-1792 (`strands`) | Extent as a bounding box over axes, and it ignores the wrap at phase 0: a strand straddling 0 reads an extent near 1 | Circular spread; fix the wrap now |
| A12 | S2 | breath.rs:556-570, 1680-1687, 1584-1591; formation.rs:1841-1850 | Landing, capture, orphan placement and system joins by radial length differences, never phases (against "phases, not frames", 2026-10-27) | Membership by in-step phase and the hemisphere law |
| A13 | S1 | breath.rs:2112, 2149, 2153, 2159 | Every throat puncture, seal and cauterize booked on plane 0 | Book on the plane the crossing is in |
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
| A22 | `ring::inverted`, `tube` (unused) | `flows::through` (live) | Two throat inversions coexist |
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
| B3 | formation.rs:1715-1716, 2078 | `settle_system` clears a system's motes and writes back only a manifest: the spray, captured motes, birth discs, warming and residue shifts lose their state at each settle | Keep the system's motes through the settle |
| B4 | breath.rs:1341-1343 vs 1036, 1129, 1562; 1305, 1365; 1048-1078 | Spin books disagree: the slip shed books twice or the other three book too little; decohered and spent stars credit spin to the medium while their motes also carry it; an emptied remnant credits nothing | One outflow book; an identity on every passing path |
| B5 | breath.rs:1985-1994, 2231-2233 | Arrivals and exits re-set spin with no identity | Book them under the torque identity |
| B6 | breath.rs:1044 with 2016 | Remnant infall's phase is pushed as a 2-vector and replaced by zero | Carry the star's phases |
| B7 | polarity.rs:32-34 vs breath.rs:391, formation.rs:684-687, 1013, 1020 | The unlocked face is `δ` in `polarity::faces` (and everything nuclear) and `1 − δ` in light, systems and strikes: both at once | One face law, read from the plane's axis (docs/117 §8.2) |
| B8 | chem.rs (`hold`); formation.rs:436-441, 1009-1014, 61-67 | Hydrogen's hold is 0 under `weight − open`, so `burn` makes no light for hydrogen, ignition heat is always 0, and `radiates(H) = 0` against its own doc | Settle hydrogen's hold; one starlight law (B9) |
| B9 | breath.rs:1433 vs formation.rs:441 | Two starlight laws: live light is `made·rung_light(z)`; `burn`'s own light feeds only `system()` and printouts | One |
| B10 | life.rs:205 | `chem::closing(a.z, …)` passes protons where weights are expected | Pass weights |
| B11 | flows.rs:1191 | `toward_rest` gets the swung reach and the stored `ell_o`: before and after are different quantities | One reach |
| B12 | pulse.rs:2037; breath.rs:484 | A tape with `t_u = 0` loops forever | Refuse it on read |
| B13 | Body.load; breath.rs:2701 | `load` is read in the period law and never set by a law (always 0) | Derive its writer or retire it |
| B14 | pulse.rs:2047-2056 | The legacy tag-19 read counts the firmament twice until the next cut | Retire the legacy read (§H, superseded layouts) |
| B15 | formation.rs:2079-2086 | `read.residual` is zero by construction: a tautology, not a witness | Witness the books from both sides |
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
| G11 | docs/102's register last edited 10-31; knows nothing of docs/109-118; D1 implemented while marked open; B7 and C3 no longer valid; docs/118 replaces §4 without saying so | Re-cut docs/102 from this register |
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

## §I. The design record (docs/78–118): pending

## §J. MU-STANDARDS and MU-RECURSION: pending
