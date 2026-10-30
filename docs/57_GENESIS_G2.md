# 57 — Genesis G2: the trajectory a fresh universe must show

**Status:** ruled by the operator 2026-10-07 ("a new genesis and start from
scratch to verify growth trajectory as anticipated"). The old universe
(`var/`, 67 worlds, last beat 1127 on 2026-09-15) stays untouched as the
archive. G2 lives in its own var root, `var-g2/`, founded by the compiled
22-seat roster (`engine/conductor/src/polities.rs`). There is no seeds
file and no roster on disk.

Each expectation is grounded in code, with its citation. A run is judged
against this list. "doc-only" marks a design claim the code does not
implement; G2 does not test those.

## How civilizations appear

- **Genesis:** the 22 compiled seats, founded in the first beat
  (`conductor/src/main.rs` startup, `Polity::genesis`).
- **Growth:** new civilizations appear only by divergence, the cleave
  (`lib.rs` `divergence_step`). A line cleaves once it holds the top of the
  difficulty ladder (L8) with at least 20 solves. Its branch is founded
  live, with its own world and chronicle, at a chart point beside the
  parent.
- **Fixed for G2:** a cleaved branch now records where it was founded
  (`registry/<sigil>.seat`). A restart resumes it in that world, at that
  point, under that temperament. Before the fix, a restart founded it a
  second time as `<sigil>-hearth` at a derived position (`bloom-call` vs
  `bloomcall-hearth` in the archive).
- **Founding is a choice (fixed for G2, direction of 2026-10-07: give the lines options, freedom and agency).** A pinned seat is the line's
  compiled home: one option, not an order. Beside it stand the regime's
  continuous draws and the open orbits (every twelfth of `k` out to 3)
  around every main-sequence sun, about 365 options. The physics scores
  each by `substrate_viability`, and the band draws among them,
  viability-weighted and chronicle-seeded (`siting::choose_site`). Better
  land is likelier, never guaranteed. A home the physics refuses (albedo 1:
  syzygy, deepmat) is reported, and a `site_planet` refusal is never
  discarded silently.
- **The founding band arrives with the colony, bounded by the land.** It
  is `⌊K_pop⌋` of the chosen site before anything is built, clamped to
  `[1, FOUNDING_BAND]`. It no longer waits on a first Habitat raised
  outside famine: that coupling locked every G2 cleave out of its own
  people.
- **A confirmed volumetric close counts.** It is recorded and counted,
  and the line climbs its own (volumetric) ladder and can cleave. The
  crystal mint stays deferred, as documented in the code.
- **Measured before the fixes** (`.testtmp/var-g2-baseline`, 1000 beats):
  7 of 22 lines collapsed within 17 ticks (6 sited on land that could not
  feed them, 2 with refused homes left unsited). 4 volumetric lines
  thought 845 times each and could never close. All 3 cleaves were born
  with no people.
- **Doc-only:** the "found a new civilization" flow (docs/20_FOUNDING §1-2:
  land, tithe) is not implemented. Neither is confluence (11_AEON). A
  CLI-made civ (`civ new`) joins only at the conductor's next boot.

## The checklist

| # | Quantity | Expected | Where it shows | Code |
|---|---|---|---|---|
| 1 | Founding bank | 20 Iron + 10 Biomass, plus a star-class top-up | first `ColonyFounded` | `cfe/src/dop.rs:982`, `:1000-1048` |
| 2 | Founding band | exactly one `PopulationSettled{+band}` per colony at genesis, band = clamp(⌊K_pop⌋, 1, 4) | chronicle, `genesis <sigil>: a founding band of N settles` | `conductor/src/lib.rs` `founding_band` |
| 3 | Population | logistic toward K, never above K; births scaled by contentment and cognition share | `PopulationSettled` / `PopulationAccrued` | `cfe/src/almanac/rejuvenation.rs:51-60` |
| 4 | Old-age deaths | none until a headcount reaches L (64; 256 for classes 2/6/10/12) | `PopulationSettled` < 0 while fed | `world/rejuvenation.rs:443`, `science_lab.rs:205` |
| 5 | The plasmic seat | grows by the logistic toward its own K, like every line: it eats through the element-eater path, and its hyperthermal clade band sits at ~(T_hot/T_mesic)⁴ the mesic flux, so its wild food and K run ~100× a mesic world's. Measured G2: 25,342 people under K = 58,782. (An earlier reading here, "4 forever", was wrong.) | `g2_k` | `world/distribution.rs` `primary_production`, `population_ceiling` |
| 6 | Famine | every `FamineStruck` paired by a `FamineRelieved`, unless the line is dying | chronicle | `world/tick.rs:575-596`, `:764-808` |
| 7 | Research | starts at L1; at most one solve per beat; climbs at most one rung per solve | `beat N: sigil(... sS LL)` every 10 beats | `lib.rs:4386`, `:4426-4431` |
| 8 | Volumetric seats (5) | close and climb their own ladder like planar lines; no crystal mint (deferred) | beat log | `lib.rs` volumetric arm, `ladder_climbs` |
| 9 | Crystal from discovery | a pulse while a line crosses L3-L5 (1, 4, 7), then nothing | `DiscoveryMinted` | `kernel/src/lib.rs:2428` |
| 10 | STAMP | rises, always ≤ crystal ever held; in code it only moves between lines, so total STAMP never falls | `StampMinted`, apertures | `lib.rs:5189-5215`, `world/mod.rs:663`, `:705` |
| 11 | First cleave | no sooner than 19 beats after the line's first solve, and only at L8 | `beat N: DIVERGENCE — X cleaves from the line of Y`, `Diverged` (tag 43) | `lib.rs:4458` |
| 12 | Cleave ceiling | at most 22 first-generation cleaves (one per line), fewer as name pools run dry | roster count | `lib.rs:4553-4565` |
| 13 | Collapse | possible, by a band's own poor choice of land; a collapsed line leaves the roster (its chronicle synced whole) and is never again a party to trade | `CivilizationCollapsed` (tag 174), `beat N: COLLAPSE — …` | `world/science_lab.rs:309-321`, `conductor::retire_collapsed` |
| 14 | Restart | resumes, never refounds: same worlds, cleaves not repeated, solves and levels kept | world dirs, `Diverged` count | `lib.rs:1477-1481`, `.seat` cards |

Stale docs found while writing this: 11_AEON §3 and the comment at
`lib.rs:4445` say a cleave needs "rung 5", but the code gates on L8.
50_WIN_CONDITIONS says no failure closure exists, but tag 174 does. The
STAMP sinks in 32_STAMP_MARKET §2.2 are doc-only.

## On the network

Every G2 world is a people under Netstratum's domain on the :7520 court.
`ns-hypha census var-g2/worlds/<world> --send` folds its new receipts per
receipt onto the colonies line and self-publishes. G2's compiled seats
reuse the archive's world names, so a people's network name carries its
universe (`var-g2/tun-reach`), and its identities, cursor and entries never
mix with the archive's. The archive (`var`) keeps its bare names, because
its folds were sealed under them.
