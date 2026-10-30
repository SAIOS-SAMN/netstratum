# 108 — Own rulers: retiring the imported calibrations

Ruling of 2026-10-30: no Earth-centric, human-centric or Milky-Way-centric
component in the code, the functions, the tests, the comments or the
documentation; each import is derived on Netstratum's own rulers, the
formula in the documents before the code. Status: scope. Builds on
docs/78 (design law 1: no table of Earth's measured values), docs/96 (the
hold law), docs/105 (the polarity equilibria), docs/106 (the substrate),
docs/102 (the register, items O1–O7 below).

## 0. The principle

An imported value is missing physics (docs/78, design law 1). It is
retired by the law it stands in for, read on the universe's own quantities
(grains, bells, rings, shares, holdings, grips), never by a renamed
constant. Until its law is derived and witnessed, each import stays
labelled as an import where it is used, so nothing passes for derived.

Three kinds of ruler are distinguished:

- **A display ruler** converts a quantity the physics already holds into an
  imported unit for reading (kelvin, years). It retires by reading the
  quantity on its own unit. The physics does not change.
- **A calibration** sets a share or scale from an observed system (the Sun's
  share of its system, a galaxy's gas fraction). It retires by the law that
  produces that share, which changes the physics and is read again.
- **A placeholder** stands for a law not yet stated (a pressure unit, a
  mind's size). It retires when the law is stated.

## 1. The imports in era 1

| # | Import | Kind | Stands for | Where |
|---|---|---|---|---|
| O1 | `HYDROGEN_K = 157,800` (hydrogen's ionization energy in kelvin) and `CONTACT_K = 2,760` (the contact anchor, from Xylarium's life law) | display ruler | heat read on the lightest kind's grip | `life.rs` (grips and world heat in kelvin), `bin/life`, docs/82 |
| O2 | `BELL_YEARS = 97,850` (a light round trip across a disc of the Milky Way's size, in Earth years; Xylarium's ladder) | display ruler | the bell counted in home years | `life.rs` (`BELL_YEARS`, the climb's clock), docs/82 |
| O3 | `K = 279 K`, the blackbody equilibrium of a Sol-type body at 1 AU (`cfe::thermal::thermal_k`) | calibration | a world's temperature against its star's burn and its reach | `seating::site_of`, `cfe::thermal`, docs/78 §"The refactor onto era 1's physics" |
| O4 | The star-class luminosity table (`cfe::thermal::StarClass`, the Sun as unit) used to site an era-1 world | calibration | a star's burn, read directly | `seating::site_of` |
| O5 | The tier shares of `pulse::tier`: a core's hole 1/1000 of its galaxy (the hole-to-bulge relation), its medium 1/10 (the Milky Way's gas to stars), a star's centre 9986/10000 (the Sun's share of its system), its medium 1/100, the orbs' half-angle `t = 1/283` (the Solar System's planets against the Sun) | calibration | the share each tier's centre holds and releases | `pulse::tier` (read by `pulse.rs:830`, `life.rs:316`), docs/78 |
| O6 | The placeholders of docs/82: the reference pressure (Earth's surface, 1 bar), `K = 60,000 K·bells` (an innermost world at 1,000 K), a mind's blueprint of order a genome (~6·10⁹ bits), a viable people of order 500, the orbs' spacing of ~10 reaches (Gyr-stable systems) | placeholder | a pressure unit, a radiation law, what a mind must hold, a population's floor, orbital stability | docs/82 §"Anchors" |
| O7 | The kelvin bands of era 0 read by era 1 (cryo, mesic, foundry) | calibration | the regimes of a world's heat | `cfe::anchor`, through `site_of` |

## 2. The derivation paths

Each is stated as the path to derive, not as derived; each derivation is
written here, with its witness, before its code.

- **O1, heat on its own ruler.** The physics holds heat as a share of a
  body's turning (`Body.heat`) and a kind's grip as its tightness τ. The
  lightest kind's grip τ₁ is the universe's own unit of binding: a world's
  heat reads `heat/τ₁` grips, and a kind holds where its own τ exceeds it.
  The kelvin factor is a display conversion only. Witness: every grip test
  (`a + τ > κ_w·h`) and every read of `life` unchanged when read in grips.
- **O2, the bell in home years.** A world's local year is its turn over the
  home world's, and a world turns once in `P = r / M(<r)` bells (docs/82),
  so one bell is `1/P_home` home years: a closed form on the home world's
  own reach and enclosure. The import counts the bell as 97,850 years, a
  scale this reading does not share; the climb's clock (`life.rs`) is
  re-read on the derived ruler, and the difference is recorded when read.
  Witness: the climb's clock in home years equals the count of the home
  world's turns over the same span.
- **O3 and O4, siting by the world's own heat and its star's own burn.**
  `site_of` maps an era-1 world onto era 0's thermal law through a star
  class and the 279 K anchor. A world's heat and its star's burn are already
  era-1 state (`Body.heat`, the burn), so the regime is read from the
  world's heat against the home world's heat (the ruler of O2), with no
  class table and no anchor. Depends on S2 (heat as state, docs/106).
  Witness: the regimes a world reads equal those of its heat against the
  home's, at every world.
- **O5, the tier shares from the polarity equilibria.** The share a centre
  keeps and the share it releases are a pair share of read quantities
  (docs/105: the equilibrium `α/(α + β)` of a two-pole exchange). At each
  tier the two poles are the centre's hold and its medium's release through
  the throat (docs/96, the hold law). Derived per tier, the five numbers of
  `pulse::tier` become each tier's equilibrium at its own throat, read from
  the founding, never a table. Depends on S1 (the energy ledger) and the
  equilibrium at the core (docs/105 §4.1). Witness: conservation exact; each
  tier's held share read as its equilibrium; no constant in `tier`.
- **O6, the placeholders.** Each retires with the law named beside it in
  docs/82: pressure with S3 (pressure as state, docs/106); the radiation law
  with S2; a mind's size and a people's floor with the gauges' own capacity
  (docs/82, the climb); spacing with the echo hold (docs/96 §8.2), which
  already decides which neighbours join.
- **O7, the regimes.** With O3 the regimes become bands of a world's heat
  against the home world's, their edges the heats at which the lightest
  bonds hold and part (the grips of O1), not the boiling points of Earth's
  chemistry.

## 3. Era 0

The era-0 game model is built on Earth physics throughout: the joule as its
energy commodity, kelvin anchors cited to Earth's chemistry, the
Morgan–Keenan star classes with the Sun as unit, the solar leap, and polity
and species names taken from the Solar System and from humans. Its names are
moved to Netstratum's own vocabulary (ruling of 2026-10-30); its values stay
as era 0's calibration until era 1 replaces each piece (ruling of 2026-10-14: era 0 a
building set for demolition), and none of them is read by era 1 except
through O3, O4 and O7.

The native vocabulary (wire codes unchanged):

| Was | Is |
|---|---|
| the joule (`Resource::Joule`, wire 5) | the ember (`Resource::Ember`) |
| kelvin, `KelvinAnchor`, the Sol equilibrium | thermal degrees, `ThermalAnchor`, the home equilibrium |
| star classes O, B, A, F, G, K, M; Wolf–Rayet | Blue, BlueWhite, White, YellowWhite, Yellow, Orange, Red; WindStripped |
| the solar leap; a world's sun | the stellar leap; a world's star |
| terraform, TERRA | worldshape, WORLDSHAPE |
| ECON-J | ECON-E |
| the clade aliases of Earth's taxonomy (felid, tardigrade, cetacean, cephalopod, hymenopteran, amphibian, cnidarian, avian, teleost, gastropod and others), and human, humanoid | retired; the coined clade names stand alone |
| polity seeds europa, jovian-reach, sun-crown, sunwell, kepler-passage | rime-moon, giant-reach, star-crown, starwell, lantern-passage |
| the geologic medium; the geoid | the lithic medium; the equipotential figure |
| the torus in megaparsecs (docs/58) | the torus in cells |

## 4. Build order

1. **O1** (display only; no physics change).
2. **O2** (closed form on the home world's period).
3. **O3, O4, O7** with S2.
4. **O5** with S1 and the core's equilibrium.
5. **O6** with S2, S3 and the gauges.

Each closes in the register (docs/102) with its commit.
