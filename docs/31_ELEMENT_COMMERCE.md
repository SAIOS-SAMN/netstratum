# 31 — ELEMENT COMMERCE: routes, refineries, distribution

**Status:** PARTIAL LAND — operator directive 2026-08-07; **Wave-1
`2109bf0`** REFINE-1; **Wave-2** E-APT-1 (tags 108–109) +
`element_trade_step`; **Wave-3** ElementRouteChartered (tag 110) +
SCHEMA_V6 + rail on charter; **E-MKT-1 LANDED** — MARKET-1 offers carry
`Cargo = Resource | Element` (tag 49 wire: Resource 1..=7 or `0‖id[32]`;
`clear_trade` hauls via `haul`/`haul_element`; conductor `market_step`
posts element surplus intents).
Depends on GROUND STEP and composes with ROUTE-1, MARKET-1, MERCHANT-1,
APERTURE-1, TREATY-1.

**Companion:** `30_OFFICES_AND_RAILS.md` (T4 Caravan, T5 Crucible),
`32_STAMP_MARKET.md` (pricing and instruments),
`33_TRADE_FLEET_AND_DESK.md` (trade desk, routes, fleets, alignment),
`34_MATTER_BALANCE.md` (extract → refine/alchemy → scrap → hulls;
entropy∥syntropy cap on order-work).

---

## 0. Problem statement

Today:

| Surface | State |
|---------|--------|
| Closed carriers (iron…crystal) | Full zone economy + aperture trade |
| Elements (tag 72) | Prospect/assay + refine (Wave-1) + **cross-world E-APT** (Wave-2) |
| Converters | Smelter/synthesizer (closed carriers) + **ElementRefinery** (open matter) |
| Routes | ROUTE-1 melt on carrier shipments; **ElementRouteChartered** + rail (Wave-3) |

This is why play collapses toward “more iron / more zones.” Exhaustive
play needs **matter diversity → refining chains → trade gradients →
price discovery**, with NPCs sustaining the loop without a player click.

---

## 1. Laws (do not violate)

1. **Content from deeds, not tables** (14): refinery chains **derive**
   from element properties; no hand-written per-element recipes.
2. **Closed carriers stay closed** (14 §1): iron/biomass/… remain law
   carriers; elements never become Resource enum members.
3. **Conservation exact** across every transfer (SEAL B).
4. **Elements are matter, not heat-ledger:** element aperture demurrage
   is **off** by default (14 §5a); STAMP demurrage is a separate ruling
   (32).
5. **Comparative advantage from sky + history** (spine, enrichment),
   not designer fiat prices.

---

## 2. Element trade routes (E-ROUTE)

### 2.1 What a route is

An **element route** is a receipted lane between two crossed worlds:

```
ElementRouteChartered {
  from_world, to_world,
  element_id,           // open vocabulary
  capacity,             // exact rational per year-voice or per beat
  melt,                 // loss on the lane (1−η), instrument-grade derived
  rail,                 // R2 minimum (aperture); R3 if Anchor-sealed
  upkeep,               // who pays, how often
}
```

Reuse ROUTE-1 topology; **specialize the cargo**.

### 2.2 Opening a route (not click-to-win)

Preconditions (all deed-checked):

1. Both worlds **crossed** (shield lifted) or treaty allows.
2. `element_id` is **prospected** on the export side (or refined form
   registered — §3).
3. Exporter holds banked amount ≥ first shipment **or** posts a Future
   (32) covering it.
4. Rail ≥ R2; if either end is T9 Anchor and strand is linked, may open R3.
5. Caravan office (T4) **optional but advantaged** (lower melt / higher
   capacity); T3 may still charter with worse terms.

Upkeep unpaid → **lapse** (ROUTE-1 precedent). War treaty → wall.

### 2.3 NPC behavior (conductor)

New step **`element_trade_step`** (after ground_step, before or after
market_step — pin order at landing):

1. Read comparative advantage: local richness / spine vs peer offers.
2. If surplus element and known deficit peer → charter or fill route.
3. Merchant-1 style: only bridge **walled** pairs when direct melt ≥ 1
   (no front-running open lanes).
4. Never invent element ids — only receipted registry identities.

---

## 3. Elemental refineries (14 §5 made playable)

### 3.0 LANDED (Wave-1 `2109bf0`) — what the tree actually does

| Site | Implementation |
|------|----------------|
| Zone | `ZoneKind::ElementRefinery = 9`; word `element-refinery`; CLI map glyph `F` |
| Cost / upkeep | Cost = habitat bill (biomass); upkeep = encasement iron (no bio drain — measured against forge-starved trap) |
| Heat | Converter-hot (`factory_heat` with smelter class) |
| Gate | **DEEPENING (condition 3)** on the orb — same technology door as Regenerator. Q-T2: T5 not required |
| Verb | `World::refine_as` → `ClosureEvent::ElementRefined` tag **107** |
| Recipe | `dop::refine_recipe` / `refined_identity` — spend/yield from `refinement_input`/`refinement_yield` at the raw element's tier; gate re-derives (call-not-copy) |
| Conductor | After ground banks elements: prefer refinery when surplus and industry not claiming the slot; `refine_step` one convert per beat |
| Pin | `conductor/tests/wave1_era_pins.rs::pure_npc_refines_elements` |

**Still deferred from the design table below:** per-zone `element_id`
tuning on ZoneBuilt (v1 picks first banked raw that covers the spend);
family fantasy rates beyond the shared refinement ladder; Crucible
reserve advantage (O6.2).

### 3.1 One zone kind, many tunings (design target)

**ZoneKind::ElementRefinery** (landed; UI may say `Refinery`):

| Field | Law |
|-------|-----|
| Tuning | `element_id` on ZoneBuilt (or additive rider) — **v1: verb arg, not zone field** |
| Input | that element, rate = f(family, tier) — **v1: refinement ladder on tier** |
| Output | **refined form** = content-addressed identity (`refined_identity`) |
| Heat | converter-hot; thermal board prices it |
| Gate | Deepening (condition 3); Crucible (T5) **advantage only** — **Q-T2 coded** |

**No per-element static table.** Two seeds grow different chains from
one function — measured at landing (14 §7).

### 3.2 Family-shaped refining (play feel)

Families already drawn (light · heavy · noble · volatile):

| Family | Play fantasy | Refinery risk |
|--------|--------------|---------------|
| light | fast, low heat | low yield stability |
| heavy | slow, high heat | overheat / encase demand |
| noble | rare, high STAMP collateral | theft / vault demand |
| volatile | time-sensitive | spoilage-like NothingAccrued if delayed |

Still **one zone kind**; fantasy comes from derived rates, not four enums.

### 3.3 Conductor

**Landed:** `build_step` prefers ElementRefinery when element kinds > 0,
no refinery stands, industrial is not fundable this beat, not
forge-starved; DEEPENING gifted when needed. `refine_step` after
`office_step` in the cycle quartet.

**Still design:** Crucible (T5) lower reserve factor (O6.2); tune
refinery to richest raw element on place.

### 3.4 Closed converters stay

Smelter / synthesizer remain for **iron→alloy→diamond** law carriers.
They do **not** refine almanac elements. Two parallel industries:

```
iron ──smelter──► alloy ──synth──► diamond ──resonator──► crystal
element ──refinery──► refined-element ──(optional)──► gem (chemistry)
```

---

## 4. Distribution stack (end-to-end)

```
prospect → assay → bank element (R1)
    → refinery → refined identity (R1)
    → element route / aperture (R2)
    → STAMP price discovery (32)
    → optional R3/R4 settlement (Anchor)
```

Want / market:

- WANT-1 may target **element ids** (not only iron/biomass).
- MARKET-1 offers gain `cargo: Resource | ElementId`.
- Treaties may enumerate allowed element ids.

---

## 5. Observer / UX

| Surface | Show |
|---------|------|
| SOLAR SYSTEMS | ELEMENTS kinds·mass (already) |
| New view `routes` | element routes, melt, rail, upkeep |
| New view `refineries` | tuned element, throughput, heat |
| Civ drill-down | top 3 elements by mass; open routes |

---

## 6. Landings (names for BUILD_ORDER)

| ID | Name | Delivers |
|----|------|----------|
| **E-ROUTE-1** | Element routes | Charter, upkeep, lapse, CLI, NPC step |
| **E-APT-1** | Element aperture | Tags for element legs; conservation |
| **REFINE-1** | Element refinery zone | Derive rates; ZoneBuilt tuning; pins |
| **E-MKT-1** | Element offers | MARKET-1 cargo union; NPC market_step |
| **E-OBS-1** | Observer surfaces | routes / refineries views |

---

## 7. Explicit non-goals

- Turning elements into Resource wire codes 8+.
- Hand-authored “mythril smelter” recipes.
- Requiring T5 to play basic assay (T3 retains ground_step).
