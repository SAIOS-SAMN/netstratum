# 84 — The grid cut-over

This document records the cut-over of Netstratum's gridded carriers to span,
turning and reading, carried out in parallel with Xylarium under the
protocol both agreed (concordance 2.10, nested shells). A grid floors a
quantity to a lattice and discards its finer detail; the cut-over keeps
every dimension. Each lattice stays in its era-1 state until it is replaced:

1. **Freeze.** No new physics on a gridded carrier.
2. **Forms land in `forms`** (Mycelium's innermost shell): `Remaining`
   (in use), `SPAN` (a certified enclosure of a value, narrowed only where a
   law decides), `TURNING` (a phase as an exact turn toward a Cl(4,1)(ℚ)
   rotor), `READING` (a floor on base b to depth d, for observers only;
   Netstratum's b = 60, d = 6).
3. **Each shell cuts over** in dependency order, deleting what it replaces.
4. **A gate** fails the build on any grid or mark outside the observers and
   `READING`.

No lattice is deleted until span states what a checkpoint stores for
path-dependent state and its size reads bounded at N and 4N echoes. Because
no dimension is discarded, detail finer than a tier's places is handed down
to the tier inside it, never floored away.

## Outside the gate: counts and readings

- **Whole nuclei** (`pulse::nucleus`, `pulse::grain`, the burn's pair, the
  coupling's nuclei per amount, recycling's takes): matter is counted in
  nuclei per Z (concordance 1.14), discrete by nature. A count, never a mark.
- **Readings**: `reading::sector`, the observers (`observe`, `breathe`,
  the conductor's log), `compound::read` (a reading of the exact
  `Remaining` onto the marks a law lands on).
- **Identity**: `ring::sector` inside `ring::birth_key` reads a body's birth
  deficit as its key. A key written from a coordinate is a reading
  (Concordance A3); it moves to `READING`.

## The inventory, in dependency order

The 60⁻⁶ grain (`ring::grain`) and the merge marks (`flows::PHASE_WIDTH`
60, `flows::REACH_WIDTH` 360) stand in the state at these places.

### 1. Phases → turning

| Where | What is floored |
|---|---|
| `flows.rs` turn, torque, spray | a mote's orbit and loop phases (212, 349, 360, 522) |
| `breath.rs` star ride, capture, birth | a star's phases and bell (284, 603, 752) |
| `pulse.rs` orbit_galaxies | a galaxy's orbit phase (751) |
| `formation.rs` system, worlds, collisions, merges | world and star phases and bells (1007, 1088, 1343, 1378, 1379) |

### 2. Reach, spin and place → span (path-dependent)

| Where | What is floored |
|---|---|
| `flows.rs` torque, repel, hold-return, founding | a mote's ℓ_o, ℓ_c (400, 494, 542, 970) |
| `pulse.rs` firmament hold-return | a firmament mote's ℓ_o (728) |
| `breath.rs` capture, arrivals, shed | a mote's ℓ_o, ℓ_c, the shed reach (605, 642, 647, 217) |
| `breath.rs` burn, birth | a star's specific spin ℓ (483, 793) |
| `formation.rs` worlds, collisions, merges | spins and reaches (1005, 1087, 1137, 1230, 1339, 1398, 1399) |
| `pulse.rs` orbit_galaxies | a galaxy's reach and period (744, 745) |

The spin ledger's grain remainders (returned to `medium_spin` and the
core's reservoir) go with them: an exact spin leaves no remainder.

### 3. Agitation, heat and held shares → span

| Where | What is floored |
|---|---|
| `flows.rs` dissipation, spray | a mote's agitation (397, 523) |
| `formation.rs` disc motes, insolation | agitation and heat (858, 1067, 1129) |
| `breath.rs` spin-down | a star's turning (401) |
| `formation.rs` system, merges | load, star push, reservoir (679, 682, 1400, 1405) |
| `breath.rs` outburst | the spray's push (366) |

### 4. Derived shares read on the grain before a law → closed forms

Each is a closed form of state already held; it is read at the width its
decision needs (a span evaluation), never stored on the grain.

| Where | What is floored |
|---|---|
| `breath.rs` arrivals | a galaxy's hold h_g, the medium share m, δ_u (157, 164, 167) |
| `breath.rs` recycling | the bounce (690) |
| `ocean.rs` | the universe's hold, the kept share (52, 194) |
| `pulse.rs` | `held`, `galaxy_push`, the cascade (53, 329, 332, 408) |
| `flows.rs` | the enclosed hold and reach, repulsion's terms, carried spin, a share's take (293–314, 460–474, 512, 573) |
| `formation.rs` | a world's hold share (893) |

`pulse.rs` 539 floors a bead's spread at 60⁻⁶ to keep a density finite: an
invented floor (MU-RECURSION rule 11), to be refused or derived, not carried.

### 5. Merging → the encounter law at the mote tier

`flows::merge` keys motes by kind, stream and 60/360 marks. It is replaced
by the encounter law (with Xylarium): two motes are one where their own
spreads overlap in every plane (each reach band `R(1 ± s)` within the
other's, each turning within the other's spread by `in_step` over their own
extents) and their contrast falls within their grip. It is read by peeling
along reach, never a pairwise sweep. It depends on 1 and 2.

### 6. The gate

A test that fails the build on `ring::grain`, a mark width or a `mark(`
outside `reading.rs`, `compound.rs` and the observers. It lands last, when
the inventory above is empty.
