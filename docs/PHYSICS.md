# Physics

This document describes how Netstratum's computational physics is used: the
engines in the workspace, the code each one runs on the live path, where the
same quantity is computed in more than one place, the values that are not
yet derived, and what the engine has produced. The laws themselves are
listed in [EQUATIONS.md](EQUATIONS.md); how Netstratum uses Mycelium is in
[ARCHITECTURE.md](ARCHITECTURE.md) §6.

It reflects `main` at the time of writing, from a read of the code (file and
line references are given so each statement can be checked).

## 1. The engines

| Engine | Crate | What it computes | On the live path |
|---|---|---|---|
| The universe (era 1) | `conductor` | A universe founded from a table of elements: nucleosynthesis, the cores' pulse, motes in flow, torque, the fluid and shell balance, star and world formation, burning, recycling, clocks | Yes: `bin/pulse` founds it, the conductor advances it every beat (`breath::breathe`, `main.rs:357`) |
| The game's world physics (era 0) | `cfe` | A world's thermal law `T = 279·L(1 − A)/r`, star classes, orbits (Kepler), element properties, refining, matter balance, resource regrowth | Yes: every world tick (`world.tick()`) and the kernel's judge (`kernel::admits`) |
| Exact kinematics | `physics` | `FlowState` (exact recurrences with inverses), `exhale`/`inhale`, `step_witness`, `Beacon` (seeded exact draws), `AxisVector`/`Triple` | Only `Beacon` and the carrier types: `FlowState`, the breath pair and the witness have no callers |
| Relation algebra | `rfa-core` | Antisymmetric relational tensors, reduction, holonomy | Yes, in the kernel's judgement of discoveries (`kernel/src/lib.rs:866-880`); not physics |
| Exact algebra | `substrate` | Rationals, a Clifford multivector, Smith normal form, volumes | Through `reduct` (cognition); the conductor's own use is in tests only |
| Mycelium's laws | `mycelium` (pinned) | Transition verification, the share-compounding law, rule elements, the network clock | The compounding law is live (`compound.rs`); verification runs only as a diagnostic (`ns-hypha probe`) |

Two models therefore describe the same worlds. Era 1 is replacing era 0, and
where era 1 does not yet provide a quantity it reads era 0's through a bridge
(§4).

## 2. The era-1 live path

### 2.1 Founding (`bin/pulse.rs`)

1. `pulse::founding`: bounce nucleosynthesis on the nucleon count gives the
   table of nuclides by weight (`nuclides.rs`).
2. `nuclides::beside_nature`: the table read beside nature's ratios, never
   fitted.
3. `pulse::deploy`: the whole table in the universal core's throat
   ([112_THE_FOUNDING.md](112_THE_FOUNDING.md)).
4. `pulse::write`: the universe to `<var>/pulse.bin`.

### 2.2 Each beat (`main.rs`)

1. **`breath::breathe`** (`main.rs:357`), in order:
   - on each universal echo, `pulse::orbit_galaxies`: the galaxies re-read
     their orbits; the firmament's collective, its collections and its order
     (`flows::repel`); the ocean's torque;
   - each arrival: the ocean's hold and swing (`ocean`), the core's push
     (`pulse::home_push`, `pulse::galaxy_push`), star encounters
     (`formation::collide_stars`);
   - each star: its turning and place (`flows::turn`, `enfold`), settling
     (`formation::settle_system`), its heart (`formation::heart_cut`),
     shedding by its faces (`polarity::faces`, `nuclides::heat`,
     `nuclides::release`, `nuclides::capture_rapid`), burning
     (`formation::burn`, `compound`, `recycle::rung_light`,
     `nuclides::capture_slow`);
   - the galaxy's motes: the release from the throat, the torque with the
     universal core's band (`flows::torque`, `flows::toward_rest`), the disc
     (`flows::settle_disc`), recycling (`recycle`), the shear and the fluid
     (`flows::shear`, `flows::fluid`), the luminal bound, exits, strands,
     streams and bindings (`flows::band_pairs`);
   - births: `formation::system`, then `flows::beads` and `flows::merge`.
2. A ledger readout every `T_u` beats (`main.rs:370-516`).
3. The worlds' life readings and the seating of peoples (`life`, `seating`,
   `waking`), then attachment (`attachment::read`).
4. Clocks (`clocks`), the era-0 orbit step (`orbit::Orbit`), the keeper chain
   (`chronometry`), substrata and realms.

### 2.3 Reading (`bin/breathe.rs`, `bin/observe.rs`)

`breathe` advances a saved universe and prints its readings: births,
conservation, circulation, the collective, the identities. `observe` reads
a saved universe without advancing it. Some laws are reached only from these
readers: `formation::{shells, stretches, collapse, extent, lightest}`,
`ocean::pump`, `motion::years_closing`, `flows::band` (also from `bin/life`).

### 2.4 Built and not yet on the live path

- `ring::{arrival, density, open, surface, strength}`: the front and the
  throat (docs/111 W1), witnessed but not yet called.
- `enclosure::route`, `pulse::from_nuclei`, `nuclides::{binds, swapped,
  paired}`: no callers.

## 3. Piecemeal implementations

The same quantity is computed in more than one place. Each entry lists the
places. Reducing each to one law is tracked as issues labelled `law`.

| Quantity | Places | Note |
|---|---|---|
| The deficit `δ = H/(R + H)` | `ring::deficit` (4 callers) and written inline 17 times: `flows.rs:259, 687, 725, 1504, 1616, 1659`; `formation.rs:83, 602, 686, 820, 1013, 1951`; `pulse.rs:474, 707, 1288`; `ocean.rs:107`; `breath.rs:564, 587` | One formula, many copies |
| The rate `r/(r + H)` | `ring::rate`; inline at `ocean.rs:71`, `breath.rs:2651` | |
| The faces `(1 − δ, δ)` | `flows::face`, `polarity::faces` | Identical |
| A system's clock | `breath::system_clock`; inline at `breath.rs:759`; `places.rs:96` (`1 − δ`, no galaxy factor); `life.rs:868` (`ring::rate` by radius, stored as `World.coupling`) | Four readings; `World.coupling` also names a different quantity than `formation::coupling` |
| Coherence `1 − 2·off` | `breath::coherence`, `flows::in_step`, `flows::response`, `chronometry::Keeper::read`, `nuclides::electron_phase` | Breath uses two coherences in one loop: an `in_step` product for shedding, `coherence(slip)` for burning |
| Slip behind the universal clock | `breath::slip`, `chronometry::Keeper::read`, `life.rs:880` | |
| The universe's hold | `pulse::universal_state`, `pulse::universal_ledger` (the same body), `ocean::universe_hold` (normalised by core states) | |
| The core's push | `pulse::push_of` (½·Q/(A + Q)), `pulse::push_cut` (held/(1 + held)), literal ½ at `home_push`, the collective push, `repel` and the ocean's torque; ½·c/(c + 1) at `breath.rs:1872`; `flows::band` | |
| Heat | `nuclides::heat` (max(0, ν·δ − 1)), `breath::shine` (l·(1 − δ)/M), `formation.rs:820` (heat·δ), `formation.rs:1013`, `life.rs:871` (heat × `HYDROGEN_K` in kelvin), `life.rs:852` (a star's luminosity ½(1 + M/M_home)) | |
| Release | `nuclides::release`, the throat release (`breath.rs:1866-1890`), `recycle::recycle`, `formation::heart_cut`, the remnant drain δ/2 (`breath.rs:1006`) | |
| Burn and lifetime | `breath.rs:1397` (live, per rung, in phase); `main.rs:478` and `formation.rs:1009` (Z = 1, coherence 1) | The readout's lifetime is not the live burn |
| The rim | `enclosure::rim_of`, `Enclosed::whole` (rim = H), `settle_system` (2H), `chronometry` (2 × the last satellite's radius) | `chronometry` also picks the home by kept mass, not `pulse::is_home` |
| A ring in bells (`T_u/4`) | `breath.rs:510`, `pulse.rs:1064`, `bin/breathe.rs:341` | |
| A galaxy's winding | `quantum::twist` (½ a turn a ring), `pulse.rs:1291` (4δ an echo) | |
| Orbital motion | `motion::Turning` (φ₀ + n/P), `orbit::Orbit` (era 0's L1 diamond, on the live path) | |
| The grain mark | `ring::grain_step`, `enfold::mark` | |
| Seeded draws | `physics::Beacon`, `pulse::draw` (unkeyed, no rejection) | |
| Shedding | `breath.rs:1310` (by coherence), `life::window_echoes` (by `tier(1).medium`, stale) | |

## 4. Era 0 and era 1 together

The worlds the game plays on are era-0 worlds whose position, composition
and land come from era 1:

- **Era 1 into era 0.** `seating::site_of` matches an era-1 world to an
  era-0 star class by `burn` (a stand-in from the star's mass, `life.rs:852`)
  and sets its orbit `r = 279·L/T`, with `T` the era-1 heat already in
  kelvin through `HYDROGEN_K`. Its composition is sealed into the era-0 world
  (`lib.rs:2114`) and drawn as veins (`cfe/src/world/survey.rs:643-669`); its
  land overrides era 0's wild flow; it moves on era-1 periods.
- **Era 0 read by the conductor directly.** The kelvin bands at `lib.rs:306`
  and `lib.rs:1624-1636`; `siting` (`cfe::thermal::t_eq4`, founding sites);
  the star class passed to gauge occupancy (`soc.rs:103, 127`,
  `lib.rs:4354`); star classes written into `polities.rs` and
  `attachment.rs:299`.
- **Quantities modelled in both eras:** heat, star classes, orbits (three
  models: Kepler `r³/M`, era-1 periods, the Diophantine tracks `r = M·k²`),
  elements (era 0's hashed properties, era 1's nuclides), food and land, and
  the contact anchor (`CONTACT_K = 2760` in both `life.rs:50` and
  `cfe/src/almanac/periodic.rs:128`).
- **A likely defect:** `composition_of` keys the composition by
  `chem::atom_w`, the nucleon count `w`, and era 0 reads it as the proton
  count `Z` (`survey.rs:643, 669`).

[108_OWN_RULERS.md](108_OWN_RULERS.md) lists these imports as O1–O7; the
read corrects it: O5's only caller is `life.rs:445`; O7's bands are also read
directly by the conductor; `2760` and `2273` stand as bare literals in
`cfe/src/almanac/periodic.rs:128, 133`.

## 5. Values not yet derived

Each is a placeholder for physics still owed, never a tuned number.

| Value | Where |
|---|---|
| The tier shares (1/1000, 1/10, 1/6; 9986/10000, 1/100, 1/283; medium ½) | `pulse.rs:205-217` (marked as imports in its own comment) |
| `HYDROGEN_K` 157 800, `CONTACT_K` 2 760, `BLUEPRINT_DOUBLINGS` 26, `BELL_YEARS` 97 850, `VIABLE_PEOPLE` 500, the genome sizes | `life.rs:45-394` |
| ln 2 and 1/ln 2 as 693/1000 and 1442/1000 | `life.rs:278, 301, 446` |
| The literal ½ pushes; the double pass `ν = 2·r/T_u`; `source_clock` 2 | §3; `breath.rs:1272`; `nuclides.rs:265` |
| 2/3, 4δ, `T_u/4`, the isotropic tilt 1/3, the /3 spreads | `breath.rs:1889`; `pulse.rs:1291`; `flows.rs:89, 1194`; `formation.rs:1169` |
| The enfold cap d/4 and its ladder of 60 | `enfold.rs:30-33` |
| The era-0 anchors: 279 K, the band edges, the solar class tables | `cfe/src/anchor.rs` |

The grains (60⁶, 60³⁰ nuclei a grain, ×1024) are the carrier's resolution,
not physics.

## 6. What the engine has produced

Each read is a bounded run of the universe at a commit, with what it
measured and what it established. Every read since 10-23 reports total
charge (3Z) conserved and the spin, torque and light identities exact.

| Commit | Founding | Measured | Established |
|---|---|---|---|
| `70e343bd` (10-24) | 2^55, 210 kinds | The founding in 7.8 s and 21 MB; the sequential filter before it did not finish in 300 s | The founding's filter read once, not pair by pair |
| `bd7cebd5` (10-24) | 2^55, 510 bells | The home's reservoir grew 1 → 6946 bits (~240 bits an echo); after the fix, level at 276–278 bits | The scalar-collapse law: a ledger booked in a moving unit widens without bound |
| `e98eb9d5` (10-23) | 2^55, 340 bells | Motes 807, 858 under the certified push, identical to the grained push | Certified intervals reproduce the exact push |
| `86379cd7` (10-23) | 2^55, 340 bells | 10 worlds; 99.4 % of a newborn star's disc stands at once | A star system read by one enclosure |
| `0e1c697e` (10-24) | 2^55, 340 bells | Worlds grown 8 → 64, bodies ignited 1 → 18 | Capture as settling into a star's sphere |
| `e7562cac` (10-24) | 2^55, a new founding | 3 galaxies, the home's share 0.065 (was 0.45) | Mass as the one weight changes the founding |
| `f78aed79` (10-25) | 2^55 | 3 galaxies, 58 stars, 108 worlds | Placing the first systems (no longer done: matter is released, docs/112) |
| `a47938bd` (10-27) | 2^55 | The densest element read by the formula, 1612 | No fixed cap on the table |
| `ab58abff` (10-31) | 2^40, 300 bells | The restoring balance moves spin; nothing created | S3, the substrate's restoring force |
| `48f3ce9d` (10-31) | 2^40, 300 bells | 12 shell planes part into cells; the balance around them moves 6.6× the radial; +23 % time | The shell law acts where the radial bins could not |
| `a9bb47f8` (10-31) | 2^40 | The 48 firmament motes founded at one point stay coincident under the order law | The order law alone cannot part a coincident collective |
| `b763cbd8` (10-31) | 2^40 | The wholly held firmament falls 48 → 17 → 0 in three bounces | The firmament is not contained: no restoring pull acted |
| `df4fa7ed` (10-31) | 2^40 | With the band on the inverse faces the firmament gathers matter (48 → 245 → 59) | The band acts, as a raw force it overshoots |
| `5cd7dbb8` (10-31) | 2^40, 1495 nuclides | The table in the core: 997 motes released at the first bounce, 6 systems in the zero galaxy; τ 0.070 at the second | Matter emerges from the core's release, nothing placed; the third bounce did not finish (carrier growth) |

**What this has and has not shown.** The engine holds its accounting
exactly: matter and the spin, torque and light identities balance on every
read, and each read that failed a law has changed it. It now forms systems
from released matter rather than placing them. It has not yet produced a
stable universal state: the firmament is not yet a container, reads beyond
a few bounces are stopped by the growth of the carrier, no universe is
running, and no era-1 state has been verified by the Mycelium court.
