# Contributing to Netstratum

This document sets the standards a change follows: the environment, the
checks, how the design record is kept, and how commits are written. It
matches the standards of the projects Netstratum publishes with
([Mycelium](https://github.com/SAIOS-SAMN/mycelium) and Xylarium).

## Environment

- Rust, stable toolchain, with `rustfmt` and `clippy`.
- Network access to GitHub at build time: the Mycelium crates are fetched
  from their published repository.
- Linux or macOS. Era-1 readings are exact-rational and memory-hungry; long
  breaths want several GB free and are run one at a time.

## Checks every change passes

```
cargo check --workspace --all-targets   # no errors, no warnings
cargo test  --workspace                 # every test passes
```

A physics change also carries its witness: a test that proves the law it
builds (an identity, a conservation, a fixed point), never a test that pins
an incidental number.

## The design record

- **Formulas before code.** A law is written into `docs/` (derived, with
  its witness) before it is built, and the doc records its state as it is
  built (status lines, commits).
- **The design laws** of era 1 ([`docs/78_SPHERES.md`](docs/78_SPHERES.md),
  "Design laws") bind every new law: no imported tables, no squares, no fill
  orders, phases never frames, exact and bounded, one grammar with coupled
  tiers. Every share is a pair share of read quantities; ½ only as the
  inversion's fixed point at the throat.
- **No leftovers, no statics.** Every share of matter is placed by a law; a
  constant is a placeholder for missing physics and is named as such.
- **The register** ([`docs/102_ALIGNMENT.md`](docs/102_ALIGNMENT.md)) keeps
  the single vocabulary and every owed item, closed with its commit.

## Mycelium

Netstratum depends on `mycelium` and `forms` from
https://github.com/SAIOS-SAMN/mycelium at a tagged release, declared once in
the root `Cargo.toml` (`[workspace.dependencies]`). Bumping the tag is a
deliberate change of its own: read the release's wire and layout changes,
re-run every check, and record it.

## Commits

- **Author:** commits are authored and committed under the project's
  identity, `netstratum contributor <two.cats7701@gmail.com>` (one email
  for every repository in the published set, each under its own name:
  `mycelium contributor`, `xylarium contributor`). Personal names
  appear only in the copyright line (README, notice).
- **Message:** a subject line of at most 72 characters in the imperative
  ("Pin Mycelium v0.2.0 from GitHub"), a blank line, then a body that says
  what changed and why, wrapped at 72 columns.
- **AI assistance:** work produced with an AI model carries a trailer naming
  it, e.g. `Co-Authored-By: Claude <noreply@anthropic.com>`.
- One substantive change a commit; generated data (`target/`, `var*/`) is
  never committed.

## Data

Universe and game state lives in variable roots (`var/`, `var-*/`),
git-ignored and throwaway: a universe is re-founded, never migrated.

## License

By contributing, a contributor agrees that the contributions are licensed
under the Apache License, Version 2.0 ([LICENSE](LICENSE)).
