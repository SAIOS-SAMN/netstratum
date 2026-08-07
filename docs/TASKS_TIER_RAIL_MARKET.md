# TASKS — T4–T9 · R1–R4 · Element commerce · STAMP market

**Status:** IMPLEMENTATION BOARD (operator directive 2026-08-07).  
**Authority docs:** `30_OFFICES_AND_RAILS.md`, `31_ELEMENT_COMMERCE.md`,
`32_STAMP_MARKET.md`.  
**Discipline:** one landing at a time; red-first witnesses; spec + mechanic
+ vectors together (CHARTER / HANDOFF). Commit only on operator go for
production landings; this file is the backlog DAG.

**Legend:** `[ ]` pending · `[~]` in progress · `[x]` done · `!` blocks fun  
**Lanes (suggested):** registry · cfe · conductor · cli · lens/observer ·
strand-adapter · isthmus/datum-adapter

---

## Wave 0 — Spec lock (documentation)  `done this session`

| ID | Task | Exit |
|----|------|------|
| D0.1 | [x] `30_OFFICES_AND_RAILS` — T4–T9 + R1–R4 named | file exists |
| D0.2 | [x] `31_ELEMENT_COMMERCE` — routes + refineries | file exists |
| D0.3 | [x] `32_STAMP_MARKET` — currency + instruments | file exists |
| D0.4 | [x] Pointers from 12_UNION / 01_ECONOMY / 14_ALMANAC / OPEN_QUESTIONS | patches |
| D0.5 | [~] Q-T1/Q-T2 **coded as defaults** in Wave-1; Q-R/Q-S still open | recorded + landed defaults |

---

## Wave 1 — Registry: open T4–T9  `done 2109bf0`  (foundation green)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| T1.1 | [x] Extend `STATIONS` + `tier_of` → T4…T9 exhaustive | defaults Q-T1 | compile; recurrence/registry pins |
| T1.2 | [x] `ascend` preconditions per office (deed counts) | T1.1 | typed refusals with cures |
| T1.3 | [x] Seat width + NoSeatOpen for new stations | T1.1 | pins |
| T1.4 | [x] Founding or mid-era **NPC path** to at least T4/T6 (not only CLI) | T1.2 | conductor pin `wave1_era_pins` |
| T1.5 | [ ] Observer STATION column shows T4–T9 names | T1.1 | visual pin (observer dirty — other lane) |
| T1.R | [x] **REFINE-1** + E3.5 (pulled into Wave-1 exit) | ground_step | `ElementRefined` tag 107; pure NPC refine |

---

## Wave 2 — Rails R1–R4 as data  `! foundation`

| ID | Task | Depends | Exit |
|----|------|---------|------|
| R2.1 | [ ] `Rail` type u8 1..=4 on routes + instruments | D0.5 | wire tags claimed first |
| R2.2 | [ ] Default R1 on all local balances | R2.1 | replay |
| R2.3 | [ ] Promotion deeds R1→R2 (aperture-linked) | R2.1, E-APT-1 | pins |
| R2.4 | [ ] Feature-gated R3 strand adapter (no hard path dep) | R2.1 | `stamp-rail-strand` builds optional |
| R2.5 | [ ] Feature-gated R4 isthmus/datum adapter | R2.1 | `stamp-rail-court` builds optional |
| R2.6 | [ ] Observer RAIL column (max settled rail) | R2.1 | |

---

## Wave 3 — Element commerce  `! fun depth`

| ID | Task | Depends | Exit |
|----|------|---------|------|
| E3.1 | [ ] **E-APT-1** element aperture legs (id in value; conservation) | ground_step live | tag + pins |
| E3.2 | [ ] **E-ROUTE-1** element route charter / upkeep / lapse | E3.1, ROUTE-1 | CLI + pins |
| E3.3 | [ ] Conductor `element_trade_step` NPC | E3.2 | pure NPC route clear |
| E3.4 | [x] **REFINE-1** ElementRefinery zone; derive rates from family/tier | 14 §5 | two seeds different chains |
| E3.5 | [x] Conductor build prefers refinery when raw surplus | E3.4 | pin |
| E3.6 | [ ] **E-MKT-1** offers carry ElementId cargo | E3.1, MARKET-1 | NPC market_step |
| E3.7 | [ ] WANT-1 may target element ids | E3.6 | |
| E3.8 | [ ] Observer `routes` + `refineries` views | E3.2, E3.4 | |
| E3.9 | [ ] Treaty allow-lists for element ids | E3.1, TREATY | |

---

## Wave 4 — STAMP unit + spot  `! money`

| ID | Task | Depends | Exit |
|----|------|---------|------|
| S4.1 | [ ] **STAMP-1** balances on orb/world; mint sources; transfer deed | D0.5 Q-S1 | pins; SEAL B |
| S4.2 | [ ] Crystal→STAMP convert schedule (placeholder curve) | S4.1 | |
| S4.3 | [ ] **BOOK-S** STAMP spot against Resource \| ElementId | S4.1, E3.6 | |
| S4.4 | [ ] **MM-1** conductor `stamp_market_step` two-sided NPC quotes | S4.3 | **no player required for clears** |
| S4.5 | [ ] CLI `stamp` verbs | S4.1 | |
| S4.6 | [ ] Observer treasury includes STAMP | S4.1 | |

---

## Wave 5 — Instruments (stocks, notes, futures, perps)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| I5.1 | [ ] **SHARE-1** issue / transfer / dividend from receipt window | S4.3, T6 optional | pins |
| I5.2 | [ ] **NOTE-1** issue / coupon / default / Vault seize (≠ ideology bond) | S4.1, T8 | pins |
| I5.3 | [ ] **FUT-1** margin, last-clear mark only, delivery/cash | S4.1, Vault | pins |
| I5.4 | [ ] **PERP-1** funding from last-clear premium; liquidation | I5.3, Q-S4 | pins |
| I5.5 | [ ] Conductor `instrument_step` (funding, expiry, auctions) | I5.2–I5.4 | NPC-only lifecycle |
| I5.6 | [ ] Integrity book hooks on default / false assay | I5.2, T7 | |

---

## Wave 6 — Offices as play (T4–T9 gameplay, not titles)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| O6.1 | [ ] T4 Caravan: route melt/capacity advantage + lapse office | E3.2, T1.x | |
| O6.2 | [ ] T5 Crucible: refinery reserve advantage | E3.4, T1.x | |
| O6.3 | [ ] T6 Exchange: only seated MM may list Share/Note/Fut/Perp | Wave 5, T1.x | |
| O6.4 | [ ] T7 Assay: consilience challenge / slash path | E3.1 | |
| O6.5 | [ ] T8 Vault: custody inventory + margin accounts | I5.x | |
| O6.6 | [ ] T9 Anchor: only exit R3/R4 | R2.4–R2.5 | |

---

## Wave 7 — Cross-repo settlement

| ID | Task | Depends | Exit | Repo |
|----|------|---------|------|------|
| X7.1 | [ ] Strand schema for STAMP/Note/Share packets | R2.4, S4.1 | round-trip | strand |
| X7.2 | [ ] Isthmus opaque frame for court claims | R2.5 | no path dep | isthmus |
| X7.3 | [ ] Datum verify path for R4 work-backed STAMP | X7.2 | reward only on multi-axial close | datum |
| X7.4 | [ ] Netstratum Anchor import refuses bad proofs | X7.1–X7.3 | pin | netstratum |
| X7.5 | [ ] End-to-end demo: NPC mint → R2 trade → R3 strand → R4 optional | all | scripted | multi |

---

## Wave 8 — Exhaustive play / anti click-to-win

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P8.1 | [ ] Shared AP budget across build / prospect / research / quote | conductor | opportunity cost pin |
| P8.2 | [ ] Trajectory + hunger + rail on observer | R2.6, research | |
| P8.3 | [ ] Labor/population derived from habitats (optional design land) | 02 | design then code |
| P8.4 | [ ] Closed-ruins history view (already hidden live) | observer | |
| P8.5 | [ ] Balance pass: placeholder curves → ruled tables | all | |

---

## Suggested first vertical slice (minimum fun)

```
[x] T1.1–T1.4 stations T4–T9 + NPC seats     ← Wave-1 2109bf0
[x] E3.4–E3.5 REFINE-1 + NPC build           ← Wave-1 2109bf0
[ ] E3.1–E3.3 element aperture + route + NPC trade   ← Wave-2
[ ] S4.1–S4.4 STAMP + spot + NPC MM                  ← Wave-4 (after E3)
[ ] R2.1–R2.3 rails R1–R2 only                       ← Wave-2/3 parallel OK
```

**Wave-2 order (see `RELAY_WAVE2_IMPLEMENTOR.md`):** claim tags → keep
done-bar red for route/STAMP → E-APT-1 → E-ROUTE-1 → `element_trade_step`
→ pure NPC route clear turns harness green. Perps and R3/R4 stay deferred.

---

## Dependency sketch

```
D0 ──► T1 (offices) ✓ ──► O6 (office gameplay)
 │
 ├──► R2 (rails data) ──► X7 (cross-repo)
 │
 ├──► E3: REFINE ✓ ──► E-APT/E-ROUTE (Wave-2) ──► S4 STAMP ──► I5
 │                                    │
 └──► ground_step ✓ ──────────────────┘
```

---

## Definition of done (program)

1. Pure NPC era produces: refined elements, element route clears, STAMP
   spot clears, at least Notes or Futures lifecycle — **zero player**.
2. Player can compete on the same books without admin faucets.
3. Optional feature flags build strand/datum adapters without forcing
   them into default `cargo test -p conductor`.
4. Observer shows T/R, elements, routes, STAMP without closed-ruin spam.
5. All new tags claimed in `dop.rs` header before implementation commits.
