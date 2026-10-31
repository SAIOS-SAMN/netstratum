# Netstratum

Netstratum is a multi-planetary civilization game in which AI-driven
civilizations and human players share one universe, computed exactly: every
quantity is a rational number, and nothing is floating point. The
civilizations' thinking is useful work. They solve verified puzzles, and those
solutions fund the in-game economy. The game is built on its own protocol
stack (NS-1 to NS-7) and connects to the
[Mycelium](https://github.com/SAIOS-SAMN/mycelium) network, where its ancients'
minds do court work and earn μ, the reserve currency.

It is for players, for developers of AI agents, for researchers studying
machine cognition and emergent economies, and for operators of the network.

## Quick start

Requires stable Rust and network access to GitHub (the Mycelium crates are
fetched at build time).

```
git clone https://github.com/SAIOS-SAMN/netstratum && cd netstratum
./install.sh          # build, found a universe in ./var (a few minutes), seat its 22 civilizations
./install.sh run      # run the AI civilizations, one beat every 2 seconds
```

In a second terminal:

```
target/release/netstratum                    # play your own world
target/release/observe --var var universe    # read the universe
python3 tools/observer.py --root var --watch 2   # watch the civilizations
```

In the client, `help` lists the commands. A first session:

```
found home
zone 12 ext        # an extractor on tile 12 (row 1, column 4)
zone 13 hab        # a habitat beside it
tick 5
collect
status
verify             # re-check the world's receipt chain
```

## What you can do

- **Play a world.** Found a colony, zone its tiles, produce, trade, research
  and sign treaties. Every action is a receipt in the world's chronicle, which
  `verify` and `replay` check.
- **Run AI civilizations.** The conductor plays 22 founding civilizations,
  each thinking, building and trading on its own world. Send one a directive
  through its inbox, or take over its world by holding its pen.
- **Write an agent.** Grant an agent scoped permissions (build, raze,
  collect) and act through it; every action records who took it.
- **Found and study a universe.** Galaxies, stars and worlds form from flows
  of matter, with mass conserved exactly. Advance it with `breathe` and read
  it with `observe`.

## Status

[docs/STATUS.md](docs/STATUS.md) states what works today and where each part
is heading: the civilizations' cognition, Synthesis and sub-universes, the
economy, μ and the Mycelium uplink, AI and human play, the universe and the
protocol stack.

## Documentation

| Document | Contents |
|---|---|
| [docs/STATUS.md](docs/STATUS.md) | What works, and the trajectory of each part |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | The crates, the two models, files on disk, where to make a change |
| [docs/CONCEPTS.md](docs/CONCEPTS.md) | Every term, with its definition, formula and code location |
| [docs/00_CHARTER.md](docs/00_CHARTER.md) | The project's binding rules |
| [docs/README.md](docs/README.md) | The index of the design record |
| [protocols/](protocols/) | The NS protocol specifications |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How changes are made and checked |

## License

Copyright 2026 Daniel Edward McFarland. Licensed under the
[Apache License, Version 2.0](LICENSE); see [NOTICE](NOTICE) and
[DISCLAIMER.md](DISCLAIMER.md). The code and documentation are AI-generated
under human editorial direction.
