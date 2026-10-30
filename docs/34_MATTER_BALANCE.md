# 34 — Matter balance: elements · alchemy · fleets · entropy ∥ syntropy

**Status:** partial land — design lock + first coupling (session).  
**Authority for the dual books:** `EQUATIONS` §6, `05_THERMAL_BALANCE`,
`09_IDEOLOGY` (syntropy ratio), `45_ALMANAC`, `31_ELEMENT_COMMERCE`,
`33_TRADE_FLEET_AND_DESK`.  
**Does not** invent a second heat law or float energy. **Does not**
replace I1 resource conservation.

---

## 0. Why this file exists

Fleets, STAMP, and trade routes sat beside the element/alchemy loop
without a closed material story. Extraction without regeneration and
order-making (refine · compound · hull mint) without a thermal dual
would let play runaway. This document **ties one loop** and **caps it
with the manifold dual** already stored on the orb:

```text
entropy_vented      (tag 28)  — what the orb SHED
syntropy_contained  (tag 20)  — what the orb KEPT
```

---

## 1. The closed material loop

```text
                    ┌──────────────────┐
   prospect/assay → │ RAW ELEMENTS     │ ← ground_step / veins
                    │  (tag 72 ledger) │
                    └────────┬─────────┘
                             │ refine (ElementRefinery)
                             ▼
                    ┌──────────────────┐
                    │ REFINED FORMS    │
                    └────────┬─────────┘
              compound/alchemy│
                             ▼
                    ┌──────────────────┐
                    │ GEMS (order)     │
                    └────────┬─────────┘
                             │ salvage paths
          iron/carriers ─────┤
                             ▼
                    ┌──────────────────┐
   ShipYard         │ SCRAP (pool 74)  │
                    └────────┬─────────┘
                             │ mint_hull
                             ▼
                    ┌──────────────────┐
                    │ HULLS → capacity │ → trade routes (melt haul)
                    └──────────────────┘

EXTRACTION (Extractor / Assay) ── ration/raw income
REGENERATOR ─────────────────── restores ration (syntropic infrastructure)
```

**Law:** every arrow is a receipted deed. Nothing free-mints.

---

## 2. Entropy ∥ syntropy cap (the balance)

### 2.1 Reading (derived, never a third book)

```text
thermal_balance(orb) =
    syntropy_contained / (syntropy_contained + entropy_vented)
    when either arm > 0; else None (no thermal history yet)
```

Bands (exact, derived from 1+1):

| Band | Condition | Play meaning |
|------|-----------|--------------|
| **Orderly** | balance ≥ 1/2 | refine / compound / hull mint free of extra gate |
| **Strained** | 0 < balance < 1/2 | order-work **refuses** unless a Regenerator stands |
| **Unbooked** | None | first deeds allowed (bootstrap) |

Half is `1/(1+1)` — no dial.

### 2.2 Order-work (what the gate covers)

| Deed | Why it is order-work |
|------|----------------------|
| `ElementRefined` | concentrates matter |
| `GemCompounded` | alchemy / higher structure |
| `HullMinted` | structure from scrap |
| (optional later) heavy assay streaks | extraction stress |

**Recycling is not gated:** `ScrapSalvaged` (iron or element→scrap)
remains open under strain — scrap is the entropy-side recovery path.

### 2.3 What posts the dual (OrderWorked tag **118**)

After order-work and salvage, a neutral receipt **books both arms**
(call-not-copy of derived amounts; no Resource move):

| Trigger | entropy arm | syntropy arm |
|---------|-------------|--------------|
| Refine | spend / 4 (heat of conversion) | yielded if Regenerator stands, else 0 |
| Compound | 0 | gem yield (order) |
| Hull mint | scrap_spent / 4 | scrap_spent / 2 (structure) |
| Salvage (any) | amount / 8 | amount / 8 (neutral form-change) |

Halves/quarters from `1+1` sums. **Salvage is order-neutral recycle**
(both arms equal residual) so scrap→hull feed is not self-poisoning;
**refine without Regenerator** is the pure-entropy strain path.
Regenerator standing is the **infrastructure answer** to strain — not
a free pass forever: without encasement/regen income, entropy still
climbs via vent/collect.

### 2.4 Extraction and regeneration (already live, now named)

| Zone / deed | Role in the loop |
|-------------|------------------|
| Extractor | raw carrier income (iron/bio) |
| Assay / prospect | open element veins |
| Regenerator | restores ration; **unlocks order-work under strain** |
| Encasement | posts syntropy via ember capture (landed collect path) |
| Demurrage / vent | posts entropy (landed) |
| Melt void share | cargo lost to void (entropic transit — already receipted) |

---

## 3. Fleet ↔ alchemy coupling

1. **Scrap feedstocks (v1):** iron salvage (landed) **and** element
   salvage (burn banked element → scrap at identity amount; alchemy
   waste / surplus refined forms become ships).
2. **Hull capacity** gates positive-melt hauls (Phase 3) — so **element
   trade routes** need ships, which need scrap, which needs extraction
   or element surplus.
3. **Metabolism catalog** still filters hull roles (Phase 3); biotic
   lines prefer organic voice names but same physics.

---

## 4. Player surface

| Verb / pane | Balance role |
|-------------|--------------|
| `status` / desk grow | show thermal_balance + band |
| `refine` / alchemy compound | order-work gated |
| `fleet salvage` / `fleet salvage-element` | recycling path |
| `fleet mint` | order-work gated |
| Regenerator build | cure for Strained |

---

## 5. Explicit non-goals

- Float energy or continuous free energy
- Netting syntropy and entropy into one stored scalar
- Replacing resource I1 conservation with thermal books
- Forcing Regenerator to build ShipYard (optional advantage only)

---

## 6. Open / next

- Q-B1: whether assay streaks post OrderWorked entropy
- Q-B2: Caravan/Crucible modifiers on OrderWorked arms
- Wire balance into observer STAMP/element boards
- Phase 4 instruments may list only when balance ≥ 1/2 (local Exchange)

**Registers at land:** tag **118** OrderWorked; next free **119**;
kernel minor bump only if admits/covered grow.
