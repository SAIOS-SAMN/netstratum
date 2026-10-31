# 41 — Provision (the food identity + the fed society)

**Status:** design, proposed 2026-08-13. Two halves: §1 (the food-identity
law) is **already true in code** and this doc pins it; §2–§5 (the fed
population, growth-from-surplus, distribution-by-archetype) are the
**missing law**, spec'd here with the open decisions §6 names. Not itself
law — a work-list decided section by section, the `40_REJUVENATION`
discipline.
Companion: `42_LIFESPAN.md` (the mortality term of the §3 population).

---

## 0. Why

Today food is produced, pooled, and burned by buildings. A world's
ration (`metabolic_ration`) is a single undifferentiated bank consumed
per-ZONE (`zone_upkeep`), and its people — `labor_force = habitats +
clones` (`world/labor.rs`) — are a *work input* that never eats.
Nothing scales demand to the size of the society, nothing grows or
starves a population, and no archetype changes how food is shared, only
how fast it is made. So there is no society for food to move through.

This chapter closes that: food feeds a population, the fed surplus grows
it and the deficit starves it, and the archetype decides how it is
distributed — the Locust/Custodian tension `40_REJUVENATION` gave the
planet, now given the people.

## 1. The FOOD-IDENTITY law — what a species eats (landed, pinned here)

**A civilization eats what its species is, never "food" in the
abstract.** The sealed metabolism is the identity, and the ration
follows it (`dop::metabolic_ration_of`):

| species (metabolism) | ration | farms | law |
|---|---|---|---|
| **biotic** | Biomass | at rate 1 | grows a field |
| **mineral** (lithovore) | Iron | at rate 1 | works ore bodies |
| **plasmic** (plasmoid) | Ember | rate **0** | captures heat — never a field |
| **minted element-eater** (#24) | its periodic element | at rate 1 | farms its own stone |

The plasmoid is the load-bearing case and it is law, not flavor: a
plasmic line's food is captured energy (`plasmic agriculture is
encasement of real heat`, 04 §1.2 / 05 §1.2), pinned by
`world_metabolism::the_mesic_band_starves_a_current_sheet` (a plasmoid
in a temperate band starves on ember) and
`plasmic_agriculture_is_encasement_of_real_heat`. Everything downstream
already reads the ration, not biomass: upkeep, regrowth (#28
`ration_touch_delta`), quarantine decay (#30), the element-eater stasis
(#24). **No `Biomass` is hardcoded in the mechanism** — the `f467876`
correction.

**This section adds no code; it pins the identity as law** so no future
mechanic may re-hardcode a single food. The witness owed: a pin that
`metabolic_ration_of` is total over the sealed metabolisms and that no
consumer of the food slot names `Biomass` directly (a grep-witness, the
`tuned_constant_ratchet` shape).

## 2. The fed population — demand follows the society, not the buildings

`labor_force` already derives the headcount from habitats + clones. This
section makes it a mouth:

- **Per-capita demand.** Each unit of `labor_force` levies a ration
  demand per beat, in the species' own ration (§1). This is added to
  the existing structural `zone_upkeep` (buildings still cost their
  own upkeep); the new term is the people's food.
- **Settled at the touch, lazily.** Like demurrage and rejuvenation
  (#28), the population's consumption settles at the collect touch over
  `Δt` since `last_touch` — no per-beat deed. It folds into the same
  `ration_touch_delta` seam: a world's net ration delta at a touch is
  `regrowth − population_demand` (free) or `−decay − population_demand`
  (quarantined).
- **Famine is now a society event.** When the ration cannot cover the
  population's demand, the shortfall drives §3 (the population shrinks),
  not merely a zoned bank going negative.

**Derivation, not dial:** the per-capita rate should derive — the
cleanest candidate is a fraction of the habitat's own food yield (a
habitat feeds the people it houses), so demand and supply share one
number. The exact fraction is an open decision (§6).

## 3. Growth from surplus — the population is a logistic on food

The population is the natural quantity to carry on the #28 curve, with
food as its carrying input:

- **Carrying capacity** `K_pop` = what the standing food surplus and
  the living footprint can sustain — the same `carrying_capacity`
  shape (`living_zones × habitability(clade, region)`), now bounding
  people, so a lush world in the clade's own band sustains a large
  population and a hostile one a small one.
- **Surplus grows, deficit shrinks.** A fed surplus climbs
  `labor_force` toward `K_pop` along `grained_logistic` (the #28
  rational sigmoid — slow near zero, saturating at K); a deficit is the
  quarantine-style decay on the population, starving it down. Same
  exact-rational curve, same lazy settle, now on the headcount.
- **The mortality floor** is `42_LIFESPAN` — even a fed population turns
  over at its species' rate, so the growth term nets against the
  lifespan death term.

**This is the Locust/Custodian tension on people:** over-work a world
(extract past its food's regrowth) and the population collapses and
scatters; keep demand under the food's growth rate and it holds.

## 4. Distribution by archetype — how the surplus is shared

The archetype decides not how much food exists but how it is allocated,
read from the disposition axis (expansionist · empiricist · epicurean,
`vocabulary::seed(Disposition)`) and the taxon `economic_mode`
(`almanac::economic_mode`, already Extractive/Regenerative/Balanced):

- **Expansionist** — lean-and-wide: low per-capita ration, fast
  population growth, thin surplus. Grows headcount, spends stability.
- **Epicurean** — rich-and-slow: high per-capita ration, slow growth,
  deep surplus. Buys resilience against famine, spends expansion speed.
- **Empiricist** — the surplus above subsistence funds research (feeds
  the novelty/discovery path, 38_COGNITION), not more mouths. Grows
  knowledge, spends population.

**Derivation:** these are the same disposition bindings the vocabulary
already mints over the landed channels — distribution is a factor set
over a `provision` channel, not a bespoke branch (the practice-channel
precedent, `28_VOCABULARY`). The exact factors are an open decision (§6), banded
by the fourfold law like every chartered factor.

## 5. What this composes with

- **`metabolic_ration_of` / `metabolic_harvest_of`** (§1) — the food
  identity, landed.
- **`world/labor.rs`** (`labor_force`, clones) — the population, made a
  consumer.
- **`grained_logistic` / `carrying_capacity`** (#28) — reused verbatim
  for the population curve.
- **`ration_touch_delta`** (#28/#30) — the settle seam demand folds
  into.
- **`economic_mode` / disposition bindings** — the distribution factors.
- **`42_LIFESPAN`** — the mortality term of §3.
- **Species-matched creation** — that a species is founded on a fitting
  world (a plasmoid on a hot one) is `TASKS_ALPHA_BETA` Phase 4 (#34,
  progenitor-gated star/planet creation + delegation); provision reads
  the fit (`planet_fit`, `habitability`) but does not create the world.

## 6. The open decisions

1. **The per-capita ration rate** (§2) — the fraction of a habitat's
   food yield one unit of `labor_force` eats. Derived candidate: half
   the habitat harvest (a habitat feeds twice what it houses works),
   leaving a structural surplus. The fraction is to be decided.
2. **The starvation target** (§3) — does a deficit shrink `clones`
   first (bio path, reversible), then habitat occupancy, or an explicit
   new population count? Reversible-clones-first is the least invasive
   and needs no new stored field.
3. **The distribution factors** (§4) — the per-disposition provision
   bindings (expansionist thin/fast, epicurean rich/slow, empiricist
   research-funded), banded fourfold.
4. **Whether §1 gets its grep-witness now** or rides the next test pass.
