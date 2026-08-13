# 43_DISTRIBUTION — THE PROVISIONING LAYER

*The bridge between subsistence and a civilization-scale economy. Translated
from operator directive 2026-08-13 into charter law; grounded in the landed
population loop (#40–42, `41_PROVISION`) and the settle-at-touch discipline.*

## 0. Why a layer at all

If food goes straight from harvest to stomach you have a biomass LEDGER,
not a market. The population loop landed in #40–42 is exactly that ledger:
`population_touch_delta` reads `available = bank + harvest + regrowth` and
feeds the people directly off it. That is subsistence. It has no place for
logistics, no place for demand above calories, and no lever for a
disposition to express itself economically.

The DEPOT is the missing node: an intermediate provisioning stock between
extraction and consumption. Interposing it makes three things load-bearing
that today are inert — **logistics** (food must move to where the mouths
are), **stratified demand** (fed people want leisure, which pulls
specialised labour), and **disposition** (each archetype runs the layer
differently). It is also the seed of every later trade/merchant system:
once food buffers and moves, the market machinery has something to price.

## 1. THE DEPOT — a buildable provisioning node (`ZoneKind::Granary`)

A new zone, deployed like any other structure the player builds, per orb
(a world's provisioning centre), typed to the sitting clade's ration. It
holds **depot stock** — an intermediate quantity DISTINCT from the raw
extraction bank (`banked_amount`), so that the raw harvest at an extraction
site and the provisioned food at a population centre are different things
that a transport step (layer 2, §6) must bridge.

**Presence changes the law, not just the numbers.** With no granary a world
runs subsistence — the #40–42 model stands verbatim, harvest→stomach, byte
identical (a world without a depot replays exactly as today). Build one and
the provisioned economy switches on: the population draws from the depot,
the caloric surplus buffers against famine, and the leisure/stability tiers
(§3–4) come alive. The granary is the technology that turns a ledger into
an economy — the REGEN-1 precedent (a zone whose placement unlocks a law).

Depot stock is a NEW STORED QUANTITY, so — like `PopulationSettled` (#39) —
it rides a carrier deed **`DepotSettled { orb_id, good, delta }`** (tag
168) so replay banks it from the receipt. It settles at the collect touch,
folded in beside the population delta (§8).

## 2. THE CONSUMPTION CYCLE — draw from the depot, not the harvest

Where a depot stands, `population_touch_delta` is rewired: the harvest of
the span flows **into** the depot (up to its capacity), and the population
eats **from** the depot's standing fill. The depot decouples production
from consumption in time — a full granary feeds the people through a lean
span; an empty one cannot, however much the fields yielded that beat.

Two severities, and the milder fires FIRST (the operator's ordering):

- **Depot below caloric baseline → STARVATION.** The existing #40–42 cull:
  the unfed die. Death is the LAST stage, only when calories run out.
- **Depot fed but lacking leisure/variety → STALL.** Growth stops and
  stability falls (§4) BEFORE anyone starves. A provisioned-but-joyless
  population holds or dwindles slowly; it does not die of it.

## 3. STRATIFIED DEMAND — calories, then leisure

Two tiers, drawn in order:

1. **Caloric baseline** — the R6 per-capita ration (`harvest/2 · p · Δt`),
   the #40 demand, now drawn from the depot's food slot.
2. **Leisure / recreation** — a SECOND provision good the depot stocks,
   demanded only once calories are met. It is produced by non-food labour
   (the pull for specialised work) and is clade-typed like the ration
   (a `Recreation` provision good; **PROPOSED**: a derived leisure invariant
   per clade, minted the way a ration is, not a new hardcoded resource).

Growth requires BOTH tiers. Fed + entertained → the #41 logistic births.
Fed + bored → stall (no births, slow stability bleed). Unfed → cull.

## 4. STABILITY — a derived contentment reading (no new field, no deed)

A reading in `[0,1]`, computed at the touch from the depot's fill against
the two tiers — high when both calories and leisure are stocked, low when
leisure is short, zero-adjacent when calories fail. Growth scales by it;
below a floor, births halt. It is **derived at read**, exactly as the
regrowth scan is (`regrown_ration`) — it stores nothing and mints no deed,
so it is SEAL-B-free. (Contrast depot STOCK in §1, which is real state and
must be receipted.)

## 5. DISPOSITION EXPRESSION — R8, realised over the depot

Distribution is a factor set over a `provision` channel keyed on the
DISPOSITION axis (`vocabulary::seed(Disposition)`: 1 expansionist ·
2 empiricist · 3 epicurean) crossed with `economic_mode` — the SAME
bindings the vocabulary already mints, never a bespoke branch (the
practice-channel precedent, `28_VOCABULARY` / `41_PROVISION §4`). Each
disposition runs the layer differently:

- **Expansionist** — lean and central. Low depot buffer target, low leisure
  demand, fast headcount growth on thin stability (military rations). Grows
  wide, spends resilience.
- **Epicurean** — deep and multi-tier. High buffer + high leisure demand,
  slow growth, deep famine resilience; the leisure pull drives INTERNAL
  TRADE. Grows rich, spends speed.
- **Empiricist** — the depot's surplus margin above subsistence siphons to
  RESEARCH (feeds the novelty/discovery path, `38_COGNITION`), not mouths
  or leisure. Grows knowledge, spends population.

The exact factors are banded by the fourfold law like every chartered
factor (the §9 ruling), and must not add dial literals (derive-not-dial).

## 6. LOGISTICS FRICTION — food must move (layer 2, deferred)

Extraction sites and population centres are different places; food crosses
from one to the other. Transport capacity, trade routes, and their security
become load-bearing — a granary far from its fields, or cut off, starves
though the harvest is plentiful. This is the seam that seeds the merchant
systems (#33). Layer 1 is single-orb (harvest and depot co-located);
cross-orb transport is layer 2, built on the existing bond/transfer rails.

## 7. THE WORKBENCH — design, deploy, and INSPECT a deployable system

The player builds the distribution engine to their civilisation's
requirements, and a **workbench view** displays the bench's
components, health, throughput, and unmet tiers — so the layer is legible,
not a black box. The depot is the FIRST bench: its view shows the granary
zones, each good's fill vs capacity, the stability reading, the caloric and
leisure coverage, and the disposition factors in force. It is a READ-ONLY
inspection surface (a `world` query + a CLI panel) over a deployed system,
generalising to other benches (refinery, ship-yard, lab) later. The
workbench is how "grow/design/deploy via civilisational requirements"
becomes visible and steerable at the console.

## 8. SETTLE DISCIPLINE

- **Depot stock** — real state, per `(orb, good)`. Carrier deed
  `DepotSettled` (tag 168), settled at the collect touch, folded beside the
  population delta and the ration delta. Replay banks from the receipt.
- **Stability, coverage, factors** — all DERIVED at the touch/read; no
  stored field, no deed (the `regrown_ration` precedent).
- **The touch order** — harvest banks → flows into depot (capped) →
  population draws from depot (calories then leisure) → births/cull settle →
  research siphon (empiricist). One touch, one coherent set of receipts.
- **No depot → no change** — the #40–42 path is untouched where no granary
  stands, so every existing world and pin replays byte-identical.

## 9. RULINGS OWED (the forks the translation surfaces)

1. **The depot's nature** — a `ZoneKind::Granary` holding a SEPARATE depot
   stock (proposed, enables §6 transport), vs reusing the raw bank as the
   depot (simpler, but no logistics seam). Buildable/workbench-able either
   way.
2. **The leisure good** — a derived per-clade recreation invariant minted
   like a ration (proposed), vs reusing an existing resource (e.g. a luxury
   Joule draw), vs a new closed `Resource::Recreation`.
3. **Stability semantics** — derived-only reading gating growth (proposed),
   vs a stored, receipted stability quantity that itself decays/recovers.
4. **The severity ordering** — confirm: leisure-short STALLS (no death),
   calorie-short STARVES; stability bleeds in the stall band before any
   cull. (Operator stated this; pinning it.)
5. **The disposition factors** — the fourfold-banded numbers for lean/deep/
   research over the provision channel (extends R8), derive-not-dial.
6. **The workbench scope now** — the depot's read-only inspection view this
   layer (proposed), vs a general workbench abstraction across all bench
   types up front.

## 10. What this composes with

- `population_touch_delta` / `collect_as` (#40–42) — the consumption cycle
  rewires here; no-depot path unchanged.
- `ZoneKind` minted-building + REGEN-1 (a zone that unlocks a law) — the
  granary's precedent.
- `DepotSettled` mirrors `PopulationSettled` (#39) — carrier-deed for new
  stored state; next-free tag **168**.
- `vocabulary::seed(Disposition)` / `economic_mode` — the §5 factors.
- bond/transfer rails — the §6 transport layer and the #33 merchant seam.
- `38_COGNITION` — the empiricist research siphon's sink.
