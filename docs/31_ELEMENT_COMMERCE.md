# 31 — ELEMENT COMMERCE: routes, refineries, distribution

**Status:** DESIGN — operator directive 2026-08-07. Lifts deferred
slices of `14_ALMANAC` §5 (refinery derivation) and §5a (element
apertures) into a playable commerce board. Depends on GROUND STEP
(prospect/assay live) and composes with ROUTE-1, MARKET-1, MERCHANT-1,
APERTURE-1, TREATY-1.

**Companion:** `30_OFFICES_AND_RAILS.md` (T4 Caravan, T5 Crucible),
`32_STAMP_MARKET.md` (pricing and instruments).

---

## 0. Problem statement

Today:

| Surface | State |
|---------|--------|
| Closed carriers (iron…crystal) | Full zone economy + aperture trade |
| Elements (tag 72) | Prospect/assay bank; **no cross-world trade**; **no refinery zone in conductor** |
| Converters | Smelter/synthesizer for **closed** carriers only |
| Routes | ROUTE-1 melt on carrier shipments, not element ids |

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

### 3.1 One zone kind, many tunings

**ZoneKind::ElementRefinery** (name may be `Refinery` in UI):

| Field | Law |
|-------|-----|
| Tuning | `element_id` on ZoneBuilt (or additive rider) |
| Input | that element, rate = f(family, tier) |
| Output | **refined form** = new registry identity, predecessor-linked, tier+1 |
| Heat | converter-hot; thermal board prices it |
| Gate | Deepening (condition 3) or Crucible (T5) advantage — **Q-T2** |

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

`build_step` extension:

1. If banked raw element above threshold and Deepening held → prefer
   refinery tuning to richest held raw element.
2. Crucible (T5) lines get lower reserve factor for refinery builds.
3. One zone per beat law unchanged.

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
