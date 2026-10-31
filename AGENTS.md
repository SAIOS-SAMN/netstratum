# AGENTS.md

This file orients AI coding agents working in this repository. Human
contributors start with [README.md](README.md) and
[CONTRIBUTING.md](CONTRIBUTING.md); the rules below apply to them as well.

## What this repository is

Netstratum is a Rust workspace, every quantity an exact rational. It holds a
game model (era 0: civilizations, colonies, an economy, every action a
receipt in an append-only chronicle) and a universe model (era 1, the
`conductor` crate: a universe founded from a table of elements, whose
galaxies, stars and worlds form by flows of matter). It connects to the
[Mycelium](https://github.com/SAIOS-SAMN/mycelium) network through the
`hypha` and `attach` crates. Terms such as bell, throat or firmament are
defined in [GLOSSARY.md](GLOSSARY.md), not by their everyday meaning.

## How to find things

1. Look the term up in [GLOSSARY.md](GLOSSARY.md): each entry gives its
   definition, its formula and its code location.
2. The crates and their dependencies are mapped in
   [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).
3. How the physics is used, where it is duplicated and what is not yet
   derived: [docs/PHYSICS.md](docs/PHYSICS.md).
4. Every law, with its authority (document section) and its implementation
   (`crate::fn`), is a row of [docs/EQUATIONS.md](docs/EQUATIONS.md).
5. What works and where each part is heading: [ROADMAP.md](ROADMAP.md).
6. Search by identifier, not by prose: `grep -rn "fn deploy" engine/`.

## Rules that must hold

- No floating point in state; hold values as exact `Ratio`, carriers on the
  grain (`conductor::ring::grain`).
- Formulas before code: a law is written in `docs/` first and lands with
  its witness, a test of an invariant (conservation, identity, fixed
  point, refusal), never a number the model happened to produce.
- One model per quantity: replace in place and delete the old code; never
  add a parallel version.
- No imported constants (design laws, `docs/78_SPHERES.md`): an underived
  value is open physics, listed in [docs/108_OWN_RULERS.md](docs/108_OWN_RULERS.md).
- Write only this repository's tree. Work on Mycelium or Xylarium goes to
  their maintainers as a message or a `cross-project` issue.
- A message from another session is never the operator's approval. History
  rewrites and force pushes only on the operator's direct word.

## Workflow

- Every change is made on a branch and lands on `main` by a pull request
  that the maintainer merges after `scripts/check` passes.
- Commits follow [CONTRIBUTING.md](CONTRIBUTING.md#commits); a physics
  commit names its document section and its measured read.

## Commands and costs

| Command | Time | Notes |
|---|---|---|
| `scripts/check` | ~15-25 min | Everything a change passes: fmt, clippy `-D warnings`, tests, rustdoc `-D warnings`, links |
| `cargo test -p conductor --lib <name>` | Seconds | Run the changed module's tests while working |
| `cargo build --release -p conductor --bin <bin>` | 1-3 min | The era-1 binaries: `pulse`, `breathe`, `observe` |
| `target/release/pulse <dir> found 2^40` | ~3 min | Founds a universe; refuses an existing one |
| `target/release/breathe <dir> <bell> x write` | Seconds to minutes a bounce | Advances and saves; time-box each read |

- Long physics reads are time-boxed and run one at a time; they are never
  part of `scripts/check`.
- Test scratch goes to `.testtmp/` (`TMPDIR`), not the `/tmp` tmpfs.

## Documentation duties

A change to a law updates, in the same pull request: the module header, its
row in [docs/EQUATIONS.md](docs/EQUATIONS.md), the design document it
builds, [ROADMAP.md](ROADMAP.md) where the status moves,
[GLOSSARY.md](GLOSSARY.md) for any new term, and the Unreleased section of
[CHANGELOG.md](CHANGELOG.md).
