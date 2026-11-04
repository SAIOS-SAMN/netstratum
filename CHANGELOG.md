# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html). Until 1.0.0, minor versions
may change public APIs, the wire, the ledger and the model's laws; every such
change is named here.

## Unreleased

### Added

- The founding is oriented (docs/100 §4.1; operator 2026-11-04): the
  universal core is founded a grain off isotropic, the seed choosing which
  plane leads and which lags (`pulse::orient`; `pulse … found <grains>
  [seed phrase]`). Until now every law and the founding treated the three
  planes alike, so they carried one disc's state three times. Read: the lean
  grows from 1 grain to 910 by the third bounce, and 1,930 of 2,246 motes
  stand apart in phase; a seed leaning other planes reads the same universe
  permuted. `breathe` prints the lean at each echo.
- The front's periods read live at each universal echo (E51–E53): the
  contained front per plane per cell, the rim `R_rim = H(<R_rim)`, and the
  established period `T_u = 4·R_rim`, so `T_u/4` is the rim (docs/112 §7.1
  closed). Read: `T_u` 4.54 and 5.00 bells at the first two echoes against
  the stored 186. The schedule does not use them yet.
- docs/116 and EQUATIONS §8.5 (E47–E50): every placeholder in PHYSICS §5
  read in its code and deduced where a law closes (the isotropic body,
  compounding for `ln 2`, the tier shares retired, the bell's years and
  kelvin read on own rulers), with what stands, what needs a read and what
  does not close.
- docs/115 and EQUATIONS §8.4 (E40–E44): bonds from nucleons and electrons,
  derived from the electrons' count, the phase transitions, the fibre and
  transduction: a bond pairs a held lone end with a free one, whole
  clusters by integer count, the light of a bond, a bond parting by heat
  and torsion against its grip. Open: one count of open ends (two in the
  code, agreeing at 282 of 1495 weights); clusters that stay open.
- docs/114 and EQUATIONS §8.2 (E31–E35): the throat as a tube's connector
  (the face inversion as `x ↦ 1/x`, a tube between mouths, chained tubes
  composing by LAW-DILATION, the Kaluza–Klein passage on the R loop, an
  open tube), with `ring::{inverted, tube, tube_share, dilation, compose}`
  and their witnesses; the conflict with the compact sphere's passage
  stated (docs/114 §2).
- `docs/PLAN.md`: the work in order under the v0.1.0 and v0.2.0
  milestones, each step an issue (#27–#38 filed for the consolidation, the
  Mycelium integration and the repository's organization).
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

- No plane is favoured by its index (docs/78, no fill orders): a tilt's
  rounding grains return to the planes that lost most, not the last plane;
  a star's pull reads every plane by its tilt, not the first; a binding's
  arcs are read in every plane tied for the disc; a core with nothing
  turning keeps its tilt. Unchanged while the planes are alike (control
  read: the same universe to every count).
- The galaxy's band pairs read their sums certified: the strip's step and
  the crest's count of bindings each from narrow bounds on their sums,
  with the exact sum only where the bounds decide differently. The same
  outcomes (witnessed); the third bounce of a fresh 2^40 universe arrives
  at 187 s against 272 s. `breathe` reports each step's time per echo
  under `NETSTRATUM_PROFILE` (#5).
- The firmament's collective push (`flows::repel`) sums its pairs exactly on
  the phases' common 60-adic denominator instead of term by term in reduced
  rationals: the same values (witnessed), and a fresh 2^40 universe now
  reaches its third bounce (#5).
- A world's composition handed to era 0 is keyed by proton count (it was
  the nucleon count, which era 0 read as Z); clusters are split among
  their atoms' elements by nucleons instead of dropped (#27).
- docs/114 T4 and E34: the passage restated as an event on the orthogonal
  orbit (sheared across the axes, forward in time). The Kaluza–Klein
  reading is withdrawn: R is not compact.
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
