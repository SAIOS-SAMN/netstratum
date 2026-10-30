# netstratum

netstratum is a universe and civilization engine written in exact rational
arithmetic. Beneath it runs a cosmology derived from one grammar of shares and
phases: an S^360 universe whose bounce founds its nuclides, whose galaxies,
stars and worlds form by flow rather than placement. On top of it runs a
text-first, multi-planetary city-building game whose civilizations, minds and
economy live on those worlds. It speaks to the
[mycelium](https://github.com/SAIOS-SAMN/mycelium) network for its clock, its
atlas and its shared law forms.

**Status:** research software, in active development. Era 1 (the sphere
universe) is being derived and built in `docs/78`–`docs/106`; the register of
what is built and what is owed is [`docs/102_ALIGNMENT.md`](docs/102_ALIGNMENT.md).
See [DISCLAIMER.md](DISCLAIMER.md): the code and documentation are
AI-generated under human editorial direction.

## Layout

```
engine/      the Rust workspace
  ratio          exact rationals for the carrier
  rfa-core       Rational Field Algebra: relational deltas, cocycles, coherence
  cfe            the Canonical Field Engine: state transitions over RFA
  codec          the committed state's one canonical encoding
  kernel         the one validated update function, lawful(prior, next, step)
  ledger         the pre-chain ledger of record and its lens
  lens           read-only projections from the chain
  scribe         the display writers: journals, the Terminal Wall, exports
  mesh           NS-5: the spine and the lawful chain remainder
  physics        event-driven exact kinematics on the RFA carrier
  solver         the ARC solver harness: exact grid transformations as proofs
  solver-verify  the sealed verifier half of the solver
  substrate      the exact-ℚ algebra (fields, planes, the CGA multivector)
  reduct         exact algebraic cognition (the thought engine)
  tongue         solver programs stated as mycelium rule elements
  hypha          ns-hypha: the uplink to a mycelium hypha
  conductor      the heartbeat, and the era-1 universe (pulse, breath, flows,
                 nuclides, polarity, formation)
clients/cli  the netstratum game client
protocols/   the NS protocol stack (NS-1 … NS-7)
docs/        the charter, the derivations, the register (see docs/README.md)
tools/       the board observer and the steward inbox (Python), service units
xtask/       workspace tasks
```

## Building

Rust (stable) and network access to GitHub, for the mycelium crates, which are
pinned at a tagged release in the root `Cargo.toml`:

```
cargo build --release
cargo test --workspace
```

## Running the era-1 universe

The conductor's era-1 binaries found and breathe a sphere universe in a
variable root (`var-*/`, git-ignored, throwaway):

```
target/release/pulse    <var> found 2^55        # found a universe of 2^55 grains
target/release/breathe  <var> 1700 _ write      # breathe it to bell 1700
target/release/observe  --var <var> universe    # read it (galaxy, star, world …)
```

`breathe` reports conservation, the identities (spin, light, torque), the
stars, worlds and clocks of every galaxy.

## Playing

```
cargo run -p netstratum-cli
```

`./install.sh` prepares a game universe and the conductor's heartbeat
(`./install.sh --help`).

## Documentation

Start with [`docs/README.md`](docs/README.md): the charter, the era-1
derivations in order, the vocabulary and the register of what is owed.
Contribution and commit standards: [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Copyright 2026 Daniel Edward McFarland. Licensed under the
[Apache License, Version 2.0](LICENSE); see [NOTICE](NOTICE) and
[DISCLAIMER.md](DISCLAIMER.md).
