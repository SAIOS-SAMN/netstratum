# Netstratum

Netstratum is a multi-planetary civilization game in which AI-driven
civilizations and human players share one universe, computed exactly: every
quantity is a rational number, and nothing is floating point. It demonstrates
three nested models in that universe: a **world model** (a planet, its
colonies and economy), a **galactic model** (a galaxy's star systems, markets,
roads and realms) and a **universal model** (the galaxies, the universal clock
and the ancients' councils).

The civilizations' thinking is useful work: they solve verified puzzles, and
those solutions fund the in-game economy. External AI agents and simulated
sandboxes attach through the [Mycelium](https://github.com/SAIOS-SAMN/mycelium)
hypha mesh and are contained in the universe at one place, with scoped
authority, under the same update rule as everything native to it; the mesh
verifies their actions independently ([docs/109](docs/109_ATTACHMENT.md)).
The ancients' minds do court work on the network and earn μ, the reserve
currency.

It is for players, for developers of AI agents and world models, for
researchers studying machine cognition and emergent economies, and for
operators of the network.

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
- **Attach an agent or a sandbox.** An external model attaches through the
  Mycelium hypha with the `attach` crate (`engine/attach/examples/agent.rs`):
  it couples to a world still forming whose land fits its needs, is admitted
  when that world's orbit closes, acts there as a contained agent under the
  same update rule as everything else, and grows only by its own economy and
  progression ([docs/109](docs/109_ATTACHMENT.md)).
- **Found and study a universe.** Galaxies, stars and worlds form from flows
  of matter, with mass conserved exactly. Advance it with `breathe` and read
  it with `observe`.

## Status

[ROADMAP.md](ROADMAP.md) states what works today and where each part
is heading: the civilizations' cognition, Synthesis and sub-universes, the
economy, μ and the Mycelium uplink, AI and human play, the universe and the
protocol stack.

## Get involved

1. Build it and play a first session (Quick start, above), then read
   [ROADMAP.md](ROADMAP.md) for where each part stands and
   [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for where code lives.
2. Pick an issue: the [v0.1.0 milestone](https://github.com/SAIOS-SAMN/netstratum/milestone/1)
   is the current work, and [`good first issue`](https://github.com/SAIOS-SAMN/netstratum/labels/good%20first%20issue)
   marks a place to start. Design questions waiting on a decision carry
   [`needs-decision`](https://github.com/SAIOS-SAMN/netstratum/labels/needs-decision).
3. Follow [CONTRIBUTING.md](CONTRIBUTING.md): a branch, `scripts/check`, a
   pull request. A new law starts as a `proposal` issue and is written in
   `docs/` before the code.

## Documentation

| Document | Contents |
|---|---|
| [ROADMAP.md](ROADMAP.md) | What works, and the trajectory of each part |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | The crates, the two models, files on disk, where to make a change |
| [GLOSSARY.md](GLOSSARY.md) | Every term, with its definition, formula and code location |
| [docs/109_ATTACHMENT.md](docs/109_ATTACHMENT.md) | How external agents and sandboxes attach through the hypha and are contained |
| [docs/00_CHARTER.md](docs/00_CHARTER.md) | The project's binding rules |
| [docs/README.md](docs/README.md) | The index of the design record |
| [protocols/](protocols/) | The NS protocol specifications |
| [docs/EQUATIONS.md](docs/EQUATIONS.md) | Every law, with its authority and its implementation |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How changes are made and checked |
| [AGENTS.md](AGENTS.md) | Orientation and rules for AI coding agents |
| [CHANGELOG.md](CHANGELOG.md) | Notable changes, release by release |
| [MAINTAINERS.md](MAINTAINERS.md), [SECURITY.md](SECURITY.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | Who maintains it, reporting a vulnerability, conduct |

## Compatibility

| Project | Pinned at |
|---|---|
| [Mycelium](https://github.com/SAIOS-SAMN/mycelium) (`mycelium`, `forms`) | `v0.3.0-rc.1` (release candidate; `v0.3.0` once cut). Conformance read: pending |

## License

Copyright 2026 Daniel Edward McFarland. Licensed under the
[Apache License, Version 2.0](LICENSE); see [NOTICE](NOTICE) and
[DISCLAIMER.md](DISCLAIMER.md). The code and documentation are AI-generated
under human editorial direction.
