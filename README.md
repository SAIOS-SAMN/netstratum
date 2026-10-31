# Netstratum

Netstratum is a universe and civilization engine written in exact rational
arithmetic. Beneath it runs a cosmology derived from one grammar of shares and
phases: an S^360 universe whose bounce founds its nuclides, whose galaxies,
stars and worlds form by flow rather than placement. On top of it runs a
text-first, multi-planetary city-building game whose civilizations, minds and
economy live on those worlds. It speaks to the
[Mycelium](https://github.com/SAIOS-SAMN/mycelium) network for its clock, its
atlas and its shared law forms.

**Status:** research software, in active development. Era 1 (the sphere
universe) is derived and built in [docs/78](docs/78_SPHERES.md) through
[docs/108](docs/108_OWN_RULERS.md); the register of what is built
and what is owed is [docs/102](docs/102_ALIGNMENT.md). See
[DISCLAIMER.md](DISCLAIMER.md): the code and documentation are AI-generated
under human editorial direction.

## Key documents

| Document | Purpose |
|---|---|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | How the workspace is organized: crates, the two models, state on disk, where to make a change |
| [docs/CONCEPTS.md](docs/CONCEPTS.md) | Every term used in the code and documents, with its definition, formula and location |
| [docs/README.md](docs/README.md) | The index of every design document |
| [docs/00_CHARTER.md](docs/00_CHARTER.md) | The founding charter, read whole |
| [docs/78_SPHERES.md](docs/78_SPHERES.md) | The sphere universe and its design laws, which bind every new law |
| [docs/102_ALIGNMENT.md](docs/102_ALIGNMENT.md) | One vocabulary for every tier, and the register of what is built and owed |
| [docs/105_POLARITY_EQUILIBRIA.md](docs/105_POLARITY_EQUILIBRIA.md) | The master equation of a two-pole exchange |
| [docs/106_THE_THERMODYNAMIC_SUBSTRATE.md](docs/106_THE_THERMODYNAMIC_SUBSTRATE.md) | The substrate (potential, pressure, energy) beneath motion; the next build |
| [docs/107_MYCELIUM_INTEGRATION.md](docs/107_MYCELIUM_INTEGRATION.md) | How Netstratum keeps time and law with the Mycelium network |
| [docs/108_OWN_RULERS.md](docs/108_OWN_RULERS.md) | Every imported Earth, Sun and galaxy calibration, and its derivation on Netstratum's own rulers |
| [docs/GAME_SPEC.md](docs/GAME_SPEC.md), [docs/SYSTEM_SPEC.md](docs/SYSTEM_SPEC.md) | The game and the system as specified |
| [docs/EQUATIONS.md](docs/EQUATIONS.md) | The equations in one place |
| [docs/BUILD_ORDER.md](docs/BUILD_ORDER.md), [docs/OPEN_QUESTIONS.md](docs/OPEN_QUESTIONS.md) | The build sequence and the open questions |
| [engine/kernel/CHARTER.md](engine/kernel/CHARTER.md) | The kernel's charter: the single validated update function |
| [protocols/](protocols/) | The NS protocol stack, NS-1 to NS-7 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Checks, the design record and the commit standard |

## Layout

| Path | Contents |
|---|---|
| [engine/](engine/) ([overview](engine/README.md)) | The Rust workspace |
| [engine/ratio](engine/ratio) | Exact rationals for the carrier |
| [engine/rfa-core](engine/rfa-core) | Rational Field Algebra: relational deltas, cocycles, coherence |
| [engine/cfe](engine/cfe) | The Canonical Field Engine: state transitions over RFA |
| [engine/codec](engine/codec) | The committed state's one canonical encoding |
| [engine/kernel](engine/kernel) | The single validated update function, `lawful(prior, next, step)` |
| [engine/ledger](engine/ledger) | The pre-chain ledger of record |
| [engine/lens](engine/lens) | Read-only projections from the chain |
| [engine/scribe](engine/scribe) | The display writers: journals, the Terminal Wall, exports |
| [engine/mesh](engine/mesh) | NS-5: the spine and the lawful chain remainder |
| [engine/physics](engine/physics) | Event-driven exact kinematics on the RFA carrier |
| [engine/solver](engine/solver) | The ARC solver harness: exact grid transformations as proofs |
| [engine/solver-verify](engine/solver-verify) | The sealed verifier half of the solver |
| [engine/substrate](engine/substrate) | The exact-ℚ algebra (fields, planes, the CGA multivector) |
| [engine/reduct](engine/reduct) | Exact algebraic cognition (the thought engine) |
| [engine/tongue](engine/tongue) | Solver programs stated as Mycelium rule elements |
| [engine/hypha](engine/hypha) | `ns-hypha`: the uplink to a Mycelium hypha |
| [engine/conductor](engine/conductor) | The heartbeat, and the era-1 universe (pulse, breath, flows, nuclides, polarity, formation) |
| [clients/cli](clients/cli) | The Netstratum game client |
| [protocols/](protocols/) | The NS protocol stack: [NS-1 DOP](protocols/NS-1_DOP.md), [NS-2 RQP](protocols/NS-2_RQP.md), [NS-3 VRP](protocols/NS-3_VRP.md), [NS-4 Aperture](protocols/NS-4_APERTURE.md), [NS-5 Mesh](protocols/NS-5_MESH.md), [NS-6 IPSAL-OS](protocols/NS-6_IPSAL-OS.md), [NS-7 Anchor](protocols/NS-7_ANCHOR.md) |
| [docs/](docs/) | The charter, the derivations and the register |
| [tools/](tools/) | The board observer and the steward inbox (Python), example service units |
| [xtask/](xtask/) | Workspace tasks |
| [install.sh](install.sh) | Prepares a game universe and the conductor's heartbeat |

## Building

The build needs stable Rust and network access to GitHub for the Mycelium
crates, which are pinned at a tagged release in the root
[Cargo.toml](Cargo.toml):

```
cargo build --release
cargo test --workspace
```

## Running the era-1 universe

The conductor's era-1 binaries found and breathe a sphere universe in a
variable root (`var-*/`, git-ignored and disposable):

```
target/release/pulse    <var> found 2^55        # found a universe of 2^55 grains
target/release/breathe  <var> 1700 _ write      # breathe it to bell 1700
target/release/observe  --var <var> universe    # read it (galaxy, star, world …)
```

`breathe` reports conservation, the identities (spin, light, torque), and the
stars, worlds and clocks of every galaxy.

## Playing

```
cargo run -p netstratum-cli
```

[install.sh](install.sh) prepares a game universe and the conductor's
heartbeat (`./install.sh --help`).

## Documentation

Start with [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and
[docs/CONCEPTS.md](docs/CONCEPTS.md). [docs/README.md](docs/README.md) indexes
the design documents: the charter, the era-1 derivations in order, and the
register of what is owed.
Contribution and commit standards are in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Copyright 2026 Daniel Edward McFarland. Licensed under the
[Apache License, Version 2.0](LICENSE); see [NOTICE](NOTICE) and
[DISCLAIMER.md](DISCLAIMER.md).
