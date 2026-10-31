# Concepts

The terms used across the code and the design documents, each with a plain
definition, its formula where it has one, and where it is implemented. Read
[ARCHITECTURE.md](ARCHITECTURE.md) first for how the pieces fit together.

All quantities are exact rationals. A *share* is a rational in `[0, 1]`. Paths
are relative to `engine/conductor/src/` unless stated otherwise.

## 1. Units and time (era 1)

| Term | Definition | Code |
|---|---|---|
| **grain** | The unit of amount: one nucleon's worth of matter. A universe is founded with an exact number of grains (e.g. `2^55`). | `pulse.rs` (`Manifest`) |
| **bell** | One tick of the simulation. Every law advances by whole bells; nothing is integrated continuously. | `breath.rs` |
| **ring** | The pulse a body emits each bell, travelling outward. A ring crosses distance `r` in `r` bells. | `ring.rs` |
| **echo** | A ring as received and re-emitted by the bodies it reaches. A body holds each arriving ring for its own period, then passes it on. | `ring.rs` |
| **universal bell period** `T_u` | The round trip of the universe's first echo at the founding: `2(R + H)` whole bells. Galaxies re-read their orbits once per universal echo. | `Universe::t_u` |
| **holding** `H` | How much a body holds back the rings passing through it; mass is holding (`M = H`), with no separate gravitational constant. `H(<r)` is the holding inside reach `r`. | `ring.rs`, `flows::Enclosed` |
| **deficit** `δ` | The share of a ring withheld at reach `r`: `δ = H / (r + H)`. Always below 1. A place's clock runs at `1 − δ`. | `ring::deficit` |
| **home year** | The home world's local year (one turn about its star), the time unit for life and peoples. | `life.rs` |

## 2. Matter

| Term | Definition | Code |
|---|---|---|
| **kind** | A type of matter: a nuclide or a cluster of nuclides. Its weight, charge, open frames and grip are computed from its parts, never read from a table. | `chem::Kind` |
| **nuclide** | A nucleus of `Q` protons and `Z` neutrons (Netstratum's convention; Xylarium uses the reverse). | `nuclides.rs` |
| **R** | The electron's phase state of a nuclide, `R = κ_e·(1 − 2·frac(p))`, where `p` is the pulse phase in turns and `κ_e = 2·Z(Q)/Q` its coherence. Bound electrons (`R > 0`) close its frames; free electrons open them. | `nuclides.rs` |
| **place** (of a nuclide) | Where its nucleons stand on the 12-dimensional sphere S^11: its turning, its open degree, and how many directions are claimed and paired. | `nuclides::Place` |
| **open frames** | The unpaired directions of a kind. They are its magnetism and decide how it bonds. | `flows::open` |
| **grip** | How tightly a kind holds against heat, from its tightness `τ`; a kind parts where `a + τ > κ_w·h`. | `nuclides::grip` |
| **densest element** | The heaviest nuclide the founding produced. Every later capture stops there. | `Universe::densest` |
| **manifest** | An exact amount of each kind, in grains. | `pulse::Manifest` |
| **mote** | A parcel of one kind in motion, carrying its amount, its spin in two planes, a phase per plane, its agitation, which leg it travels, a lever and a tilt. | `flows::Mote` |

## 3. Bodies and places

| Term | Definition | Code |
|---|---|---|
| **body** | A core, star or world. It keeps a manifest, has satellites and a medium, and turns about its centre. | `pulse::Body` |
| **core** | A galaxy's centre: a singularity whose *state* is set at the founding and never drained. The first core (path `[0]`) is the home galaxy. | `pulse::Body`, `pulse::is_home` |
| **universal core** | The home galaxy's core read at the universe tier, the source of the universal pulse. | `pulse.rs`, docs/101 |
| **throat** | The surface at a body's equator `R = H`, where its two faces meet: the mouth of the tube joining its enclosure to the one above. A body has one iff its extent lies within it (`R_b ≤ H_b`). `Body::throat` holds the tube's unreleased contents. | docs/111 §3.1, `enclosure.rs` |
| **extent** | A body's own size: the reach its held matter stands at about its centre, where its pressure share meets its potential share. | docs/111 §3.1 |
| **enclosure** | The closed sphere a body defines: each galaxy is its own S^360 and each star system its own S^60, hung inside the sphere above at its core. | `enclosure.rs` |
| **reach** `R` | A body's distance from its centre in that enclosure's units. | `Body::radius` |
| **medium** | A tier's unbound matter beyond its last orbit. | `Body::medium` |
| **ocean** | The matter between galaxies, as motes returning to the home galaxy. | `Universe::ocean`, `ocean.rs` |
| **firmament** | The heaviest kinds held at the universal core, which barely move under its pulse. | `Universe::firmament` |
| **tier** | A level of the hierarchy: universe, galaxy, star system, world. The same laws apply at each, read in that tier's own units. | `pulse::tier`, docs/89 |

## 4. Motion

| Term | Definition | Code |
|---|---|---|
| **turning** | Each received ring turns a body by its deficit: `ω = δ` turns per ring, so one turn takes `P = 1 + R / H` rings. A body's phase at bell `n` is `φ + n/P`, in closed form. | `motion.rs`, `flows::turn` |
| **spin** `ℓ` | Reach times holding, `ℓ = R · H`; angular momentum is mass times `ℓ`. Motes carry it in the orbit plane (`ell_o`) and across it (`ell_orth`). | `ring.rs`, `flows::Mote` |
| **phase** | A position on a turn, as a share in `[0, 1)`, one per plane. Membership and closeness are read from phases, never from lengths in a fixed frame. | `flows::Mote::phase` |
| **the three planes** | Space is three spatial planes plus the R coordinate; a mote's tilt is its spin's share in each plane. | docs/95, `flows::space_of` |
| **pulse** and **push** `p` | Each bell a core re-pulses at full strength. Its push share is `p = ½·Q/(A + Q)` (`Q` its open frames' magnetism, `A` its holding); matter is held where `H/(R + H) ≥ p`. | `pulse::galaxy_push` |
| **torque** | The pulse's effect on a mote's spin: it adds spin on one hemisphere of the bell and removes it on the other; what does not land in step becomes heat. Always paired with the hold's return. | `flows::torque`, `flows::spin_torque` |
| **agitation** | A mote's heat: the share of its turning absorbed and not re-locked. | `flows::Mote::agitation` |
| **lever** | Per plane, the part of a mote's holding standing one place past its phase mark, so that its centre is its true position. | `flows::true_phase` |

## 5. The faces (inverse physics)

Netstratum's universe is the inverse of Xylarium's: the shares a body holds
and releases are exchanged under the inversion `δ ↦ 1 − δ`.

| Term | Definition | Code |
|---|---|---|
| **held face** `g` | The share a body holds: `g = 1 − δ` (inverse gravity). | docs/104, `polarity.rs` |
| **unlocked face** `u` | The share it releases: `u = δ`. Pushes and heating act here. | docs/104 |
| **continuum / vacuum leg** | The two faces a mote can travel on. The core's outflow starts on the continuum; ocean arrivals travel the vacuum. | `flows::Mote::vacuum` |
| **luminal bound** | A mote turns `2·share` a bell; where its share would exceed ½ it would turn more than one orbit per step, so it crosses to the other face, where its share is at most ½. | `flows::luminal` |
| **Möbius pass** | The crossing between faces: `(δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)`. Applied twice it is the identity. | `flows::mobius` |
| **polarity equilibrium** | For a two-pole exchange with rates `α` (A→B) and `β` (B→A), the equilibrium share is `α/(α + β)`, approached in closed form. | `polarity::equilibrium`, docs/105 |

## 6. Formation

| Term | Definition | Code |
|---|---|---|
| **binding** | Matter on one band of a disc bound into one body, where its bond meets its heat and the tide across it. | `flows::band_pairs` |
| **bond** | A binding's strength, `λ/(1 + λ)` with `λ = 2(m + μ + τ)/R`: holding, open-frame magnetism and twist over reach (docs/103 composes it from all layers). | `formation::bond`, docs/103 |
| **shell / stretch** | A ring of a galaxy's gas, and a span of it that can collapse. | `formation::Shell`, `formation::Stretch` |
| **heart / ignition** | A collapsing binding's centre; it ignites as a star where it holds against its own heat. | `formation.rs` |
| **remnant** | A star whose burn has ended. Its extent is re-read without heat: within its equator its throat opens and it is a core whose tube's other face is its galactic core; outside, it stays closed. | `Body::remnant`, docs/111 §3.2 |
| **recycling** | Shed and spent matter returns through the cores and the ocean; total mass never changes. | `recycle.rs`, `ocean.rs` |
| **heat** (of a world) | The agitation of the bonds that built it, cooling through its unlocked face; its thermal band `r(1 ± heat)` decides which neighbours it meets. | `Body::heat` |

## 7. The game (era 0)

| Term | Definition | Code |
|---|---|---|
| **world** | One playable planet with its own chronicle. | `engine/cfe/src/world/` |
| **orb** | A self-contained game object with provenance (NS-1 DOP): a colony, sector or profile. | `engine/cfe/src/dop.rs` |
| **line** / **civilization** | A people playing one or more worlds, with its axes (disposition, metabolism, perception). | `ledger::registry`, `conductor::polities` |
| **beat** | One heartbeat of the conductor: every world is played and ticked once. | `conductor/src/main.rs` |
| **receipt** | The NS-3 record of one completed action, linked to its predecessor. | `engine/cfe` |
| **chronicle** | A world's append-only sequence of receipts; it is the world's state of record. | `engine/cfe` |
| **update function** | `kernel::admits(prior, next, step)`: decides whether a step lawfully produces `next` from `prior`. | `engine/kernel/src/lib.rs` |
| **STAMP** | The game's unit of account. | `engine/cfe` |
| **ember** | The stored-heat commodity, minted only by encasing excess heat. | `cfe::dop::Resource::Ember` |
| **sink** | Banked capacity to absorb heat. | `cfe::dop::Resource::Sink` |
| **demurrage** | A holding cost that decays a balance over time. | `engine/cfe` |
| **forge / rung / solve** | The puzzle forge, seeded by the latest receipt; a rung is a difficulty level, and a solve is a verified answer with its certificate. | `engine/solver`, `engine/solver-verify` |
| **mint** | The creation of new value or a new line, only through a lawful event (e.g. a solve, a cleave). | `engine/cfe`, `engine/kernel` |
| **bands** | The tiers of peoples: progenitors, sovereigns, stewards, ancients and others. | docs/12, docs/70 |
| **μ** | The reserve currency earned by court work on the Mycelium network. | `engine/hypha` |
