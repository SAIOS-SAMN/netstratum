# 43_DISTRIBUTION — The provisioning layer

*The bridge between subsistence and a civilization-scale economy. Stated as
charter law; grounded in the landed
population loop (#40–42, `41_PROVISION`) and the settle-at-touch discipline.*

## 0. Why a layer at all

If food goes straight from harvest to stomach the result is a biomass ledger,
not a market. The population loop landed in #40–42 is exactly that ledger:
`population_touch_delta` reads `available = bank + harvest + regrowth` and
feeds the people directly off it. That is subsistence. It has no place for
logistics, no place for demand above calories, and no lever for a
disposition to express itself economically.

The depot is the missing node: an intermediate provisioning stock between
extraction and consumption. Interposing it makes three things load-bearing
that today are inert — **logistics** (food must move to where the mouths
are), **stratified demand** (fed people want leisure, which pulls
specialised labour), and **disposition** (each archetype runs the layer
differently). It is also the seed of every later trade/merchant system:
once food buffers and moves, the market machinery has something to price.

## 1. The depot — a buildable provisioning node (`ZoneKind::Granary`)

*(The depot reuses the raw ration bank — no separate stock, no new deed.
The granary flips the consumption law on; it does not introduce a second
quantity. Logistics/transport (§6) therefore stays inert for this layer,
for the same reason.)*

A new zone, deployed like any other structure the player builds, per orb (a
world's provisioning centre), typed to the sitting clade's ration. Its
stock is the orb's existing banked ration (`banked_amount`) — the granary
is not a second buffer but the infrastructure that changes how the standing
bank is consumed.

**Presence changes the law, not the numbers.** With no granary a world runs
subsistence — the #40–42 model stands verbatim, harvest→stomach, byte
identical (a world without a granary replays exactly as today). Build one
and the provisioned economy switches on: the caloric surplus is allowed to
compound into growth under a leisure/stability discipline (§3–4) and the
disposition factors (§5) come into force. The granary is the technology that
turns a ledger into an economy — the REGEN-1 precedent (a zone whose
placement unlocks a law), and it stores nothing new, mints no deed, so the
whole layer stays SEAL-B-free on the state side. All new quantities
(stability, coverage, leisure demand) are derived at the touch (§4/§8).

## 2. The consumption cycle — draw from the depot, not the harvest

Where a depot stands, `population_touch_delta` is rewired: the harvest of
the span flows **into** the depot (up to its capacity), and the population
eats **from** the depot's standing fill. The depot decouples production
from consumption in time — a full granary feeds the people through a lean
span; an empty one cannot, however much the fields yielded that beat.

Two severities, and the milder fires first:

- **Depot below caloric baseline → starvation.** The existing #40–42 cull:
  the unfed die. Death is the last stage, only when calories run out.
- **Depot fed but lacking leisure/variety → stall.** Growth stops and
  stability falls (§4) before anyone starves. A provisioned-but-joyless
  population holds or dwindles slowly; it does not die of it.

## 3. Stratified demand — calories, then leisure

Two tiers, drawn in order:

1. **Caloric baseline** — the R6 per-capita ration (`harvest/2 · p · Δt`),
   the #40 demand, now drawn from the depot's food slot.
2. **Leisure / recreation** — a second provision good the depot stocks,
   demanded only once calories are met. It is produced by non-food labour
   (the pull for specialised work) and is clade-typed like the ration
   (a `Recreation` provision good; **proposed**: a derived leisure invariant
   per clade, minted the way a ration is, not a new hardcoded resource).

Growth requires both tiers. Fed + entertained → the #41 logistic births.
Fed + bored → stall (no births, slow stability bleed). Unfed → cull.

## 4. Stability — a derived contentment reading (no new field, no deed)

A reading in `[0,1]`, computed at the touch from the depot's fill against
the two tiers — high when both calories and leisure are stocked, low when
leisure is short, zero-adjacent when calories fail. Growth scales by it;
below a floor, births halt. It is **derived at read**, exactly as the
regrowth scan is (`regrown_ration`) — it stores nothing and mints no deed,
so it is SEAL-B-free. (Contrast depot stock in §1, which is real state and
must be receipted.)

## 5. Disposition expression — R8, realised over the depot

Distribution is a factor set over a `provision` channel keyed on the
disposition axis (`vocabulary::seed(Disposition)`: 1 expansionist ·
2 empiricist · 3 epicurean) crossed with `economic_mode` — the same
bindings the vocabulary already mints, never a bespoke branch (the
practice-channel precedent, `28_VOCABULARY` / `41_PROVISION §4`). Each
disposition runs the layer differently:

- **Expansionist** — lean and central. Low depot buffer target, low leisure
  demand, fast headcount growth on thin stability (military rations). Grows
  wide, spends resilience.
- **Epicurean** — deep and multi-tier. High buffer + high leisure demand,
  slow growth, deep famine resilience; the leisure pull drives internal
  trade. Grows rich, spends speed.
- **Empiricist** — the depot's surplus margin above subsistence siphons to
  research (feeds the novelty/discovery path, `38_COGNITION`), not mouths
  or leisure. Grows knowledge, spends population.

The exact factors are banded by the fourfold law like every chartered
factor (§9), and must not add dial literals (derive-not-dial).

## 6. Logistics friction — food must move (layer 2, deferred)

Extraction sites and population centres are different places; food crosses
from one to the other. Transport capacity, trade routes, and their security
become load-bearing — a granary far from its fields, or cut off, starves
though the harvest is plentiful. This is the seam that seeds the merchant
systems (#33). Layer 1 is single-orb (harvest and depot co-located);
cross-orb transport is layer 2, built on the existing bond/transfer rails.

## 7. The workbench — design, deploy, and inspect a deployable system

*(Decision: build a general, bench-type-agnostic workbench abstraction up
front, not a depot-only view.)*

The player builds the distribution engine (and every other deployable
system) to their civilisation's requirements, and a **workbench** displays
any bench's components, health, throughput, and unmet requirements — so
each layer is legible, not a black box. The abstraction is one shape over
many bench types: a bench reports its component zones, each with a
fill/health/throughput reading and its unmet demands; the console renders
any bench uniformly. The bench types are the deployable systems already in
the engine — **depot** (granary zones; food/leisure coverage, stability,
disposition factors), **refinery** (ElementRefinery), **ship-yard**
(ShipYard), **lab** (research) — with the depot wired in as the first
concrete bench that exercises the abstraction. It is read-only (a `world`
query + a CLI panel) over deployed state. The workbench is how "grow/design/
deploy via civilisational requirements" becomes visible and steerable at
the console.

## 8. Settle discipline

*(Depot stock = the raw bank, so there is no new stored quantity and
no `DepotSettled` deed. Everything this layer adds is derived at the touch.)*

- **Depot stock** — the orb's existing banked ration; settles through the
  ration deeds already in the collect touch (`YieldCollected`/`UpkeepSettled`),
  no new carrier.
- **Stability, coverage, leisure demand, factors** — all derived at the
  touch/read; no stored field, no deed (the `regrown_ration` precedent).
- **The touch order** — harvest banks → population draws its calories, then
  its leisure, from the bank (granary present) → stability gates the births/
  cull that settle → research siphon (empiricist). One touch, existing
  receipts.
- **No granary → no change** — the #40–42 path is untouched where no granary
  stands, so every existing world and pin replays byte-identical.

## 9. Decisions

1. **The depot's nature: reuse the raw bank.** No separate stock, no
   `DepotSettled` deed; the granary flips the consumption law on. Logistics
   (§6) stays inert this layer for the same reason.
2. **The leisure good: a derived per-clade recreation invariant**,
   minted like a ration (`metabolic_ration_of` precedent) — species-
   specific (a swarm's leisure ≠ a stone-being's), no new closed resource.
3. **Stability semantics: a derived-at-touch reading** in `[0,1]`
   gating growth; stores nothing, mints no deed (the `regrown_ration`
   precedent). May be promoted to stored state later if it needs momentum.
4. **The severity ordering:** leisure-short stalls (no
   death), calorie-short starves; stability bleeds in the stall band before
   any cull.
5. **The disposition factors — landed (L4)** as a factor set derived from
   `0`/`1`/`2` (no new dial), keyed on `civ_axes().disposition`: expansionist
   leisure ×0 (lean — grows on food alone), epicurean ×2 (deep — the granary
   becomes a luxury engine that stalls without production), empiricist ×1 but
   half its surplus births siphon to research (`research_siphon`, the metric;
   the `DiscoveryMinted` deed is the follow-up). This is the mechanic of R8's
   ¾/5⁄4/¾+research, expressed without importing those literals; the exact
   bands remain an open balance decision if R8's numbers are wanted.
6. **The workbench scope: a general workbench abstraction up front**
   — bench-type-agnostic (depot, refinery, ship-yard, lab all as benches),
   the depot wired in as the first concrete bench. See §7.

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
