# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html). Until 1.0.0, minor versions
may change public APIs, the wire, the ledger and the model's laws; every such
change is named here.

## Unreleased

### Added

- `docs/MYCELIUM.md`: what Netstratum takes from Mycelium, where each use
  runs and how complete it is, the defects (verification only as a
  diagnostic, unmeasured laws counted as agreeing, `tongue` half-wired), and
  the line between the two projects' physics. docs/109 A6 corrected to
  partial.
- `docs/PHYSICS.md`: the engines, the era-1 live path, the quantities
  computed in more than one place, the values not yet derived, the era-0
  bridges, and a ledger of what the engine's reads have established;
  EQUATIONS §8.1 adds the live-path laws E12–E30.
- The open-source layout: `AGENTS.md` (with a `CLAUDE.md` pointer),
  `MAINTAINERS.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, this changelog, and
  `scripts/check` (fmt, clippy `-D warnings`, tests, rustdoc `-D warnings`,
  `scripts/check-docs`).
- The founding with the whole table of elements in the universal core's
  throat (docs/112 F1), released inside the throat (F2), the band as an
  attractor (F3) and `τ` from the table's books (F5).
- Contract review: changes touching a shared contract are labelled
  `cross-project` and reviewed by Mycelium's maintainer, the coordinator
  (MAINTAINERS.md, CONTRIBUTING.md "Contracts").
- Getting involved: a README section, issue templates (defect, proposal,
  track) and a pull request template; open work tracked as GitHub issues
  under the v0.1.0 milestone.
- The universal core's band on the inverse faces (docs/104 §6).
- The firmament's collective: bonds, collections moving as one, its books
  and last phases saved (docs/111 W2, W3).
- The front's formulas: arrival, density, throat, surface share and
  strength (`conductor::ring`, docs/111 W1).
- Attachment of external agents through the hypha (docs/109) and the shell
  law (docs/110).

### Changed

- The workspace is formatted with rustfmt and clean under clippy
  `-D warnings`; `type_complexity`, `too_many_arguments` and
  `needless_range_loop` are allowed workspace-wide.

### API changes

- Mycelium pinned at `v0.3.0-rc.1` (was `v0.2.0`): `sdk::Kernel` is
  `sdk::Voice`; `HyphaKey::vk_bytes` is `verifying_bytes`;
  `forms::xylarium` is `forms::shares`. A seat's role is Mycelium's opaque
  tier, and Netstratum names its tiers in `ns_hypha::tiers` (progenitor 0,
  ancients 1, steward 2, sovereign 3, the bytes unchanged). Mycelium's task
  addresses change at this release; saved test data carrying old addresses
  is re-founded, never migrated.

### Layout changes

- `docs/STATUS.md` moved to `ROADMAP.md`, `docs/CONCEPTS.md` to
  `GLOSSARY.md`.
- The era-1 save gains tags 28 (the founded table's holding) and 29 (the
  firmament's last phases).
