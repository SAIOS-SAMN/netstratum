# Architecture

This document describes how the Netstratum workspace is organized: the
crates and their dependencies, the two models the code contains, where state
lives, and where to make a change. For the meaning of the terms used here,
see [GLOSSARY.md](../GLOSSARY.md).

## 1. Overview

Netstratum is a Rust workspace of about 120,000 lines, every quantity held as
an exact rational number (no floating point anywhere in the state). It
contains two models that share the engine crates:

- **The game model (era 0).** Civilizations play colonies on worlds: zoning,
  production, trade, research and the economy. Every action is validated by
  a single update function and recorded as a receipt in an append-only
  chronicle. This model is complete enough to play through the CLI client.
- **The universe model (era 1).** A cosmology the game is moving onto: a
  universe founded from an exact amount of each element, whose galaxies,
  stars and worlds form by flows of matter rather than being placed. It
  lives in the `conductor` crate and is run and read with its own binaries.

Era 1 is replacing era 0 piece by piece. Where era 1 needs a quantity era 0
still provides (a world's thermal regime, for example), it reads it through a
bridge (`conductor::seating::site_of`); these bridges are listed in
[108_OWN_RULERS.md](108_OWN_RULERS.md).

The workspace also connects to [Mycelium](https://github.com/SAIOS-SAMN/mycelium),
a separate network project, for a shared clock, a shared atlas and shared law
forms; see §6.

## 2. Crates

Layered from the bottom up (an arrow means "depends on"):

```
ratio ─────────────┬──────────────────────────────────────────────┐
                   ▼                                              │
cfe (world model, TLV wire, receipts) ◄── rfa-core                │
  ▲     ▲      ▲                             ▲                    │
codec  kernel  ledger ◄── physics            │                    │
  ▲     ▲  ▲     ▲                           │                    │
  │    mesh lens  scribe       solver-verify ◄── solver ◄── reduct ◄── substrate
  │     ▲    ▲     ▲                 ▲           ▲
  └─────┴────┴─────┴──── conductor ──┴───────────┴── tongue, hypha (Mycelium)
                              ▲
                        clients/cli
```

| Crate | Lines | Responsibility |
|---|---|---|
| `ratio` | 0.6k | The exact-rational carrier (`Ratio`, canonical coprime form) and its wire encoding |
| `rfa-core` | 3k | Rational Field Algebra: antisymmetric relational tensors (`Delta`), cocycles, coherence |
| `cfe` | 36k | The Canonical Field Engine: the era-0 world model (colonies, zones, resources, thermal law, economy), the TLV wire format, receipts and the chronicle |
| `codec` | 1.4k | The single canonical encoding of the committed state; the bytes that digests bind |
| `kernel` | 4k | The update function `admits(prior, next, step)`: pure, total and free of I/O; every state change passes through it |
| `ledger` | 2.3k | The pre-chain ledger of record and the civilization registry |
| `lens` | 1.8k | Read-only projections of the chain |
| `scribe` | 1.2k | Display writers: journals, the Terminal Wall, exports (write-only) |
| `mesh` | 1k | NS-5: the spine and the in-chain remainder of the mesh protocol |
| `physics` | 0.5k | Event-driven exact kinematics on the rational carrier |
| `substrate` | 4k | Exact-rational algebra: fields, planes, the conformal geometric algebra multivector |
| `reduct` | 2.8k | Exact algebraic reduction, the reasoning engine used by the minds |
| `solver-verify` | 4k | The verifier half of the puzzle solver: the grid vocabulary, the task forge and certificates; everything a replay needs |
| `solver` | 3.3k | The search half of the solver (the ARC task harness) |
| `tongue` | 0.2k | Solver programs expressed as Mycelium rule elements |
| `hypha` (`ns-hypha`) | 4k | The uplink to a Mycelium hypha: verification and settlement with the network, and the house that seats council minds and attached minds |
| `attach` | 0.6k | Attachment (docs/109): the interface's constants, the file exchange between the house and the conductor, and the client library an external agent uses |
| `conductor` | 28k | The heartbeat that plays the game's civilizations, and the whole era-1 universe model |
| `clients/cli` (`netstratum-cli`) | 15k | The game client: the map, the commands, presentation only |
| `xtask` | 0.1k | Workspace tasks |

`engine/pen` is an empty, retired crate kept only as a placeholder; it is not
a workspace member.

## 3. The universe model (era 1)

All of era 1 is in `engine/conductor/src`. It is a simulation of exact
quantities advanced in discrete **bells** (ticks), with no floating point and
no random number generator in the physics.

### 3.1 Data model

```
Universe                       pulse.rs
├── cores: Vec<Body>           one per galaxy; the first is the home galaxy
│   └── Body                   path, radius, period, kept (a Manifest), medium,
│       │                      ell (spin), state, heat, tilt, throat, gas, …
│       ├── flows: Vec<Mote>   the galaxy's matter in transit (flows.rs)
│       └── satellites: Vec<Body>   star systems → stars → worlds
├── ocean: Vec<Mote>           matter between galaxies, returning home
├── unit, t_u, densest         the unit of mass, the universal bell period,
│                              the heaviest nuclide the founding produced
```

- A **`Manifest`** maps each kind of matter to an exact amount in grains.
- A **`Kind`** (`chem.rs`) is a nuclide or a cluster of nuclides. Its weight,
  charge, open frames and grip are computed from its parts, never looked up
  in a table.
- A **`Mote`** (`flows.rs`) is a parcel of one kind in motion: its amount,
  its spin in the orbit plane (`ell_o`) and across it (`ell_orth`), a phase
  per plane, agitation (heat), which leg it travels (continuum or vacuum),
  a lever and a tilt.

### 3.2 Lifecycle

1. **Founding** (`bin/pulse.rs` → `pulse::deploy`). An exact manifest of
   nucleons (for example `2^55` grains) is turned into a universe: the
   nucleosynthesis of the bounce (`nuclides.rs`), the cores and their
   throats, and the initial motes.
2. **Breathing** (`bin/breathe.rs` → `breath::breathe`). The universe
   advances bell by bell. Each bell, in order:
   - on each universal echo, every galaxy re-reads its orbit
     (`pulse::orbit_galaxies`);
   - the echoes of each core's pulse arrive at its bodies (`ring.rs`, the
     ring-density law);
   - stars shed what they cannot hold;
   - ocean motes land in the galaxy whose pull is greatest;
   - the core's pulse applies torque to the motes, and they turn
     (`flows::torque`, `flows::turn`);
   - the motes form bindings, and bindings form stars and worlds
     (`flows::band_pairs`, `formation.rs`);
   - stars burn and capture nucleons, and spent stars become remnants;
   - shed matter is recycled through the cores (`recycle.rs`, `ocean.rs`).
3. **Persistence** (`pulse::write` / `pulse::read`). The universe is
   serialized to `<var>/pulse.bin` (TLV records). `<var>/conductor.beat`
   records the bell reached, so a run resumes from its checkpoint.
4. **Reading** (`bin/observe.rs`). Read-only views of a saved universe:
   `universe`, `galaxy`, `star`, `world`, `flows`, `census`, `formation`,
   `ledger`, `recycle` and others. `bin/life.rs` reads what each world
   offers life, and `bin/atoms.rs` builds the periodic chemistry from the
   atom law.

### 3.3 Invariants checked on every run

`breathe` reports the conservation of mass across the run and the spin,
light and torque identities: every share of matter is placed by a law, so
the totals before and after must match exactly. The design laws every era-1
law must satisfy are in [78_SPHERES.md](78_SPHERES.md), "Design laws": no
imported tables, no squares, first-degree shares, phases rather than
reference frames, and bounded exact arithmetic.

### 3.4 Where the derivations are

How the physics is used on the live path, where it is duplicated and what
it has produced is in [PHYSICS.md](PHYSICS.md).

Each era-1 law is derived in a design document before it is coded, starting
at [78_SPHERES.md](78_SPHERES.md) and continuing in order through
[108_OWN_RULERS.md](108_OWN_RULERS.md). The register of what is built and
what is open is [102_ALIGNMENT.md](102_ALIGNMENT.md).

## 4. The game model (era 0)

### 4.1 State and the update rule

The game keeps one machine-readable copy of state, and every change goes
through one function ([00_CHARTER.md](00_CHARTER.md) §11):

```
request ──► inbox ──► kernel::admits(prior, next, step) ──► Ok │ Refused(typed)
                              │
                              ▼
               receipt (NS-3) appended to the world's chronicle
```

- `cfe` holds the world model and computes `next` from `prior` and a step.
- `kernel::admits` checks the transition: totals balance, permissions hold,
  arithmetic is exact. It is pure and never reads or writes bytes.
- `codec` produces the canonical bytes of the committed state; receipts bind
  their digests.
- Randomness, where a mechanic needs it, is drawn from a generator seeded by
  the previous receipt's digest, so replaying the chronicle reproduces every
  outcome (Charter §9).

### 4.2 The heartbeat

The `conductor` binary (`engine/conductor/src/main.rs`) runs the game. Each
beat, for every world, it drains the steward inbox, plays the
civilization's policy through a contained agent, ticks the world, collects,
runs the puzzle forge (seeded by the world's latest receipt) and keeps the
ledger. In era 1 the conductor also breathes the era-1 universe one bell a
beat and seats the founding civilizations on its star systems.

### 4.3 The client

`clients/cli` is the player's interface. It shows the map, takes commands
and renders the engine's typed refusals as readable text. Its `verify` and
`replay` commands read the chain as any peer would. Presentation lives only
here.

## 5. Files on disk

A universe lives in a **variable root** (`var/` or `var-*/`), which is
git-ignored and disposable: a universe is re-founded, never migrated.

| Path | Contents |
|---|---|
| `<var>/pulse.bin` | The era-1 universe (TLV records) |
| `<var>/conductor.beat` | The bell an era-1 run has reached (its checkpoint) |
| `<var>/universe.jrn` | The conductor's own readings, appended as framed TLV |
| `<var>/registry/` | One card per registered civilization (`<sigil>.civ`, `<sigil>.seat`) |
| `<var>/worlds/` | One directory per era-0 world: its chronicle, receipts and display exports |
| `<var>/conductor.ctl` | The operator's control line (`paused`, `cadence N`) |
| `<var>/*.bin` | Further era-1 records: councils, realms, ascension, ancients, accounts |

`install.sh` prepares a game root and founds its worlds; `pulse` founds an
era-1 universe in an empty root.

## 6. Mycelium

Netstratum depends on Mycelium at a tagged release, pinned in the root
`Cargo.toml`. What it takes from Mycelium, how complete each use is, and
where the line falls between Netstratum's physics and Mycelium's are in
[MYCELIUM.md](MYCELIUM.md); how it keeps time on the network is in
[107_MYCELIUM_INTEGRATION.md](107_MYCELIUM_INTEGRATION.md).

External agents and sandboxes attach through the house in `ns-hypha`
([109_ATTACHMENT.md](109_ATTACHMENT.md)): the house turns their seat calls
into files under the variable root (`attach/`, and directives in the worlds'
inboxes), and the conductor admits them, places them and applies their
actions each beat (`conductor::attachment`).

## 7. Where to make a change

| To change | Look in |
|---|---|
| How matter moves, turns or is held | `conductor/src/flows.rs`, `ring.rs`, `motion.rs` |
| How stars and worlds form | `conductor/src/formation.rs` |
| The founding and the nuclides | `conductor/src/pulse.rs`, `nuclides.rs`, `chem.rs` |
| What happens each bell | `conductor/src/breath.rs` |
| What a world offers life | `conductor/src/life.rs` |
| A game mechanic (colonies, zones, economy) | `engine/cfe/src/world/`, `engine/cfe/src/dop.rs` |
| What counts as a lawful state change | `engine/kernel/src/lib.rs` |
| The wire format | `engine/cfe/src/tlv.rs`, `engine/codec` |
| A command or its display | `clients/cli/src/main.rs` |
| A read-only view of the universe | `conductor/src/bin/observe.rs` |

Before changing a law in era 1, read its derivation (§3.4) and the design
laws; a new law is written into its design document, with the test that
proves it, before the code ([CONTRIBUTING.md](../CONTRIBUTING.md)).

## 8. Tests

`cargo test --workspace` runs every test. Tests live beside their code
(`#[cfg(test)]` modules) and in each crate's `tests/` directory. A physics
test proves a law (a conservation, an identity, a fixed point) rather than
pinning an incidental number. Long reads of a universe are done with the
binaries, not with tests: found with `pulse`, run with `breathe`, inspect
with `observe`.
