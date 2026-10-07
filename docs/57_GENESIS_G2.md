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
- **Growth:** new civilizations appear ONLY by divergence, the cleave
  (`lib.rs` `divergence_step`). A line cleaves once it holds the top of the
  difficulty ladder (L8) with at least 20 solves. Its branch is founded
  live, with its own world and chronicle, at a chart point beside the
  parent.
- **Fixed for G2:** a cleaved branch now records where it was founded
  (`registry/<sigil>.seat`). A restart resumes it in that world, at that
  point, under that temperament. Before the fix, a restart founded it a
  second time as `<sigil>-hearth` at a derived position (`bloom-call` vs
  `bloomcall-hearth` in the archive).
- **Doc-only:** the "found a new civilization" flow (docs/20_FOUNDING §1-2:
  land, tithe) is not implemented. Neither is confluence (11_AEON). A
  CLI-made civ (`civ new`) joins only at the conductor's next boot.

## The checklist

| # | Quantity | Expected | Where it shows | Code |
|---|---|---|---|---|
| 1 | Founding bank | 20 Iron + 10 Biomass, plus a star-class top-up | first `ColonyFounded` | `cfe/src/dop.rs:982`, `:1000-1048` |
| 2 | Founding band | exactly one `PopulationSettled{+4}` per colony, with the first Habitat, never in famine | chronicle | `conductor/src/lib.rs:310`, `:2013`, `:2037` |
| 3 | Population | logistic toward K, never above K; births scaled by contentment and cognition share | `PopulationSettled` / `PopulationAccrued` | `cfe/src/almanac/rejuvenation.rs:51-60` |
| 4 | Old-age deaths | none until a headcount reaches L (64; 256 for classes 2/6/10/12) | `PopulationSettled` < 0 while fed | `world/rejuvenation.rs:443`, `science_lab.rs:205` |
| 5 | The plasmic seat | headcount 4 forever: no harvest, never starves, always fed | its census | `dop.rs:1837`, `lib.rs:3127` |
| 6 | Famine | every `FamineStruck` paired by a `FamineRelieved`, unless the line is dying | chronicle | `world/tick.rs:575-596`, `:764-808` |
| 7 | Research | starts at L1; at most one solve per beat; climbs at most one rung per solve | `beat N: sigil(... sS LL)` every 10 beats | `lib.rs:4386`, `:4426-4431` |
| 8 | Volumetric seats (5) | never solve: `s0 L1` forever (known dead end, task #173) | beat log | `lib.rs:4122-4155` |
| 9 | Crystal from discovery | a pulse while a line crosses L3-L5 (1, 4, 7), then nothing | `DiscoveryMinted` | `kernel/src/lib.rs:2428` |
| 10 | STAMP | rises, always ≤ crystal ever held; in code it only moves between lines, so total STAMP never falls | `StampMinted`, apertures | `lib.rs:5189-5215`, `world/mod.rs:663`, `:705` |
| 11 | First cleave | no sooner than 19 beats after the line's first solve, and only at L8 | `beat N: DIVERGENCE — X cleaves from the line of Y`, `Diverged` (tag 43) | `lib.rs:4458` |
| 12 | Cleave ceiling | at most 17 first-generation cleaves (22 minus 5 volumetric), fewer as name pools run dry | roster count | `lib.rs:4553-4565` |
| 13 | Collapse | none in a healthy run (starvation deaths are fractional) | `CivilizationCollapsed` (tag 174) | `world/science_lab.rs:309-321` |
| 14 | Restart | resumes, never refounds: same worlds, cleaves not repeated, solves and levels kept | world dirs, `Diverged` count | `lib.rs:1477-1481`, `.seat` cards |

Stale docs found while writing this: 11_AEON §3 and the comment at
`lib.rs:4445` say a cleave needs "rung 5", but the code gates on L8.
50_WIN_CONDITIONS says no failure closure exists, but tag 174 does. The
STAMP sinks in 32_STAMP_MARKET §2.2 are doc-only.

## On the network

Every G2 world is a people under netstratum's domain on the :7520 court.
`ns-hypha census var-g2/worlds/<world> --send` folds its new receipts per
receipt onto the colonies line and self-publishes. G2's compiled seats
reuse the archive's world names, so a people's network name carries its
universe (`var-g2/tun-reach`), and its identities, cursor and entries never
mix with the archive's. The archive (`var`) keeps its bare names, because
its folds were sealed under them.
