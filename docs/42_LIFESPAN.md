# 42 — Lifespan (the species clock, and what it does to a society)

**Status:** design, proposed 2026-08-13. Entirely **missing law** — no
age, mortality, senescence, or turnover exists in code today
(`world/labor.rs` holds a static headcount). Spec'd here as the
mortality term of the `41_PROVISION` §3 population, with the rulings §5
names. Not itself law — ruled section by section, the `40_REJUVENATION`
discipline.

---

## 0. Why

Different entities should not live alike. A crystal-mind or a plasma-
field is a slow, near-geological being; a biotic-swarm or a void-swarm
turns over in a breath. Today they turn over identically — not at all:
`labor_force = habitats + clones` is a static count with no birth, no
death, no generation. This chapter gives each species its own clock,
and makes that clock the engine of a real trade: **growth versus
continuity.**

It is the third face of the one tension the game already rules. The
Locust/Custodian split is extraction-vs-regrowth on a planet (#28) and
demand-vs-food on a population (`41_PROVISION` §3); lifespan is
turnover-vs-continuity on the generations. One law, three surfaces.

## 1. Lifespan is A species property — read, not chosen

A line's characteristic lifespan is a fact of what it is — its class /
clade — the same place `clade_preferred_region` and `economic_mode`
already read (`almanac::taxon_economy`). The ordering is the law; the
exact spans are §5:

| longer-lived (slow clock) | shorter-lived (fast clock) |
|---|---|
| crystal-mind (10), plasma-field (6), lithovore (2), thermal-banker (12) — mineral/energy/lattice beings | biotic-swarm (1), void-swarm (11), pelagic-aerial (5) — collective/eusocial biotic |

Long-lived lines are the mineral, plasmic, and lattice classes — beings
whose substrate is stone, energy, or crystal; short-lived lines are the
biotic collectives that trade individuals cheaply. **This is emergent
from the species roster (`15_TAXON`), not a per-line dial** — a clade's
lifespan derives from its class band exactly as its preferred region
does.

## 2. Lifespan → the mortality term — turnover in the population curve

Lifespan enters as the death rate in the `41_PROVISION` §3 population
logistic. A population of `labor_force` at lifespan `L` loses a `1/L`
share of its force per beat to natural turnover — the flat,
exact-rational shape (`1/L` is a clean ratio; no transcendental,
bounded denominator, the demurrage discipline). Net population change at
a touch is:

    ΔP = food_surplus_growth  −  turnover(1/L · P · Δt)

- A **short-lived** line (small `L`) has a large death term: it must
  keep breeding hard just to hold, so it grows fast when fed and
  collapses fast when starved — high-variance, adaptive, cheap.
- A **long-lived** line (large `L`) has a small death term: it holds
  its numbers on little food and barely grows even in surplus — low-
  variance, stable, expensive to expand.

Settled lazily at the touch over `Δt`, the same seam as demurrage,
rejuvenation, and provision — no per-beat deed.

## 3. The growth-VS-continuity trade — what it means for the society

Turnover is not only a population number; it is what a society keeps.
The `growth.rs` reading already carries continuity axes (time, testing,
scope) derived from the chronicle. Lifespan couples to them:

- **A long-lived society accumulates.** Its practices, tech, and
  discovered nodes persist across generations because the generation
  barely changes — knowledge compounds, the growth-reading's time and
  testing axes deepen. The Custodian's body: slow to grow, hard to
  displace, rich in retained law.
- **A short-lived society churns.** Each fast generation must RE-earn
  what the last knew, or it decays — knowledge is discontinuous unless
  actively renewed. The Locust's body: fast, adaptive, forgetful; it
  wins the sprint and loses the archive.

**The mechanic that makes this bite:** a share of a line's standing
advantages (crystallized boons, adopted practices) that must be
RE-confirmed within a lifespan or lapse — so a short-lived line pays a
continuity tax a long-lived one does not. This is the demurrage shape
applied to knowledge rather than stock, and it reuses the same
settle-at-touch seam. The exact decay and what lapses is a §5 ruling.

## 4. What this composes with

- **`15_TAXON` (class/clade roster)** — where lifespan is read from.
- **`41_PROVISION` §3** — the population logistic lifespan is the death
  term of.
- **`grained_logistic` (#28)** — the growth term turnover nets against.
- **`world/growth.rs`** — the continuity axes lifespan taxes.
- **`world/labor.rs`** — `labor_force`, the count turnover shrinks.
- **`42`↔`41`:** neither ships without the other — a mortality term
  needs a population to act on, and a population needs a mortality floor
  or it never turns over. Rule and land them together.

## 5. The rulings owed

1. **The per-class lifespan values** (§1) — the `L` per class band.
   Derived candidate: `L` scales with the class's substrate permanence
   (crystal/mineral/energy long, biotic collective short), banded so the
   longest is a small multiple of the shortest, not orders apart (a
   10000-beat vs 10-beat split would make short-lived lines unplayable).
2. **The starvation interaction** (§2) — does turnover add to the
   `41_PROVISION` §3 starvation decay, or is starvation just turnover
   with no births? One combined death term is simpler.
3. **The continuity tax** (§3) — what a short-lived line must re-confirm
   within a lifespan (crystallized boons? adopted practices? both?), and
   the decay rate if it lapses. Or defer §3 as a second slice, shipping
   §1–§2 (the population clock) first.
4. **Where lifespan is stored/derived** — a pure reading off class (no
   new state, the `clade_preferred_region` precedent) is strongly
   preferred over a per-orb stored age.
