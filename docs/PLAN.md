# Plan

The order in which the physics is consolidated, the Mycelium integration
made correct, and the repository organized. It follows from
[PHYSICS.md](PHYSICS.md) and [MYCELIUM.md](MYCELIUM.md). Each step is a
GitHub issue; the milestones are releases
([v0.1.0](https://github.com/SAIOS-SAMN/netstratum/milestone/1),
[v0.2.0](https://github.com/SAIOS-SAMN/netstratum/milestone/2)).

## Principles

- **One law per quantity.** A quantity computed in several places is reduced
  to one function that every caller uses; the copies are deleted, never kept
  beside it.
- **Mechanical before physical.** A consolidation that changes no value
  lands first, on its own, so a later change of law is read against a
  known base.
- **Docs before code** for every change of law: the derivation in its design
  document and its row in [EQUATIONS.md](EQUATIONS.md), then the code with
  its witness.
- **Measured.** A change of law states a bounded read (identities,
  conservation, what moved) in its pull request.

## Steps

### v0.1.0: the founding and the pin (in progress)

| # | Step | Depends on |
|---|---|---|
| [#27](https://github.com/SAIOS-SAMN/netstratum/issues/27) | Fix the composition handed to era 0 (nucleon count read as proton count) | — |
| [#5](https://github.com/SAIOS-SAMN/netstratum/issues/5) | Bound the carrier's growth, so reads pass the third bounce | — |
| [#2](https://github.com/SAIOS-SAMN/netstratum/issues/2), [#3](https://github.com/SAIOS-SAMN/netstratum/issues/3), [#4](https://github.com/SAIOS-SAMN/netstratum/issues/4) | The founding's F4–F6: collections by weight, release by pressure, the contained period | #5 |
| [#6](https://github.com/SAIOS-SAMN/netstratum/issues/6) | The rim and the bell's length without founding shells | — |
| [#7](https://github.com/SAIOS-SAMN/netstratum/issues/7), [#8](https://github.com/SAIOS-SAMN/netstratum/issues/8) | Galaxies from released matter; bodies' extents and throats | #3 |
| [#9](https://github.com/SAIOS-SAMN/netstratum/issues/9) | Declare LAW-WAVEFRONT v1 | — |
| [#10](https://github.com/SAIOS-SAMN/netstratum/issues/10) | Pin Mycelium v0.3.0 and pass the conformance read (PR #24, held for rc.3) | Mycelium rc.3 |

### v0.2.0: one law per quantity

**Phase 1, mechanical (no value changes).**

| # | Step |
|---|---|
| [#28](https://github.com/SAIOS-SAMN/netstratum/issues/28) | One deficit, one rate, one face map, one universe hold, one grain mark, one ring-in-bells |
| [#37](https://github.com/SAIOS-SAMN/netstratum/issues/37) | Remove dead physics code and unused dependencies |

**Phase 2, physical (each a change of law, docs first).**

| # | Step | Depends on |
|---|---|---|
| [#29](https://github.com/SAIOS-SAMN/netstratum/issues/29) | One clock for a nested body; one coherence and one slip | #28 |
| [#30](https://github.com/SAIOS-SAMN/netstratum/issues/30) | One push, one heat, one release law | #28 |
| [#31](https://github.com/SAIOS-SAMN/netstratum/issues/31) | One burn and lifetime, one rim, one winding, one orbital motion | #29, #6 |
| [#12](https://github.com/SAIOS-SAMN/netstratum/issues/12) | The substrate's energy ledger and heat as state (S1, S2) | #30 |

**Phase 3, the eras.**

| # | Step | Depends on |
|---|---|---|
| [#32](https://github.com/SAIOS-SAMN/netstratum/issues/32), [#13](https://github.com/SAIOS-SAMN/netstratum/issues/13) | One model each for heat, star classes, orbits and elements; the imported rulers O1–O7 derived | #30, #31, #12 |

**Phase 4, Mycelium.**

| # | Step | Depends on |
|---|---|---|
| [#33](https://github.com/SAIOS-SAMN/netstratum/issues/33) | Decide what the court verifies (operator decision, with the coordinator) | — |
| [#34](https://github.com/SAIOS-SAMN/netstratum/issues/34) | The agreement check: an unmeasured law is a disagreement | — |
| [#35](https://github.com/SAIOS-SAMN/netstratum/issues/35) | Wire `tongue` into the seat, or remove it | — |
| [#36](https://github.com/SAIOS-SAMN/netstratum/issues/36) | `TPoint::parse` and one multivector | #10 |
| [#11](https://github.com/SAIOS-SAMN/netstratum/issues/11) | Attachment as a live service | #33, operator go-ahead |

**Phase 5, organization.**

| # | Step | Depends on |
|---|---|---|
| [#38](https://github.com/SAIOS-SAMN/netstratum/issues/38) | Crates grouped by role, the design record in `docs/design/` by era, the example collision, the build's disk cost stated | #37 |

### Decisions open

The `needs-decision` issues wait on the operator: #33 (what the court
verifies), #14–#17 (the register's open laws B5, B6, L5, L6), and #11 (a
running service).
