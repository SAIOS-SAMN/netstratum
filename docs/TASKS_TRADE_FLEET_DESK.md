# TASKS — Trade desk · fleets · alignment · local capital (5 phases)

**Status:** IMPLEMENTATION BOARD — operator directive 2026-08-07.  
**Authority doc:** `33_TRADE_FLEET_AND_DESK.md`.  
**Consumes:** `30_OFFICES_AND_RAILS`, `31_ELEMENT_COMMERCE`, `32_STAMP_MARKET`,
`TASKS_TIER_RAIL_MARKET` (S4.5–S4.6, E3.6, Wave 5 instruments, O6).  
**Discipline:** one landing at a time; red-first witnesses; claim tags
in `dop.rs` header before bumps; SEAL B; ratchet does not rise for
convenience; menu state never outruns the chronicle (33 L4).

**Legend:** `[ ]` pending · `[~]` in progress · `[x]` done · `!` blocks fun  
**Lanes:** cfe · kernel · codec · conductor · cli · lens · observer · docs

**Baseline:** kernel `0.22.0`, next free tag **114**, SCHEMA_V6, STAMP NPC
spot green. Re-measure before each phase land.

---

## Phase map

```
P1 Sightlines & parity  ──►  P2 Alignment (D_eff)
                                │
                                ▼
                         P3 Fleet capital
                                │
                                ▼
                         P4 Local capital markets
                                │
                                ▼
                         P5 Defense & deep rails
```

| Phase | Codename | Depends | Fun exit |
|-------|----------|---------|----------|
| **1** | Sightlines & parity | STAMP/E-APT engine (landed) | Desk + player STAMP/element trade |
| **2** | Alignment economics | P1 readings useful | Orbit windows move melt |
| **3** | Fleet capital | P2 D_eff; yard design | Hulls limit and optimize routes |
| **4** | Local capital markets | P1 STAMP player; `32` Wave 5 | Civ-filtered Note/Fut/Share/Perp |
| **5** | Defense & deep rails | P3 hulls; R-line | Escort + R3/R4 capital exits |

---

## Phase 0 — Spec lock (this documentation)

| ID | Task | Exit |
|----|------|------|
| D0.1 | [x] `33_TRADE_FLEET_AND_DESK.md` design lock | file exists |
| D0.2 | [x] This board (`TASKS_TRADE_FLEET_DESK.md`) Phases 1–5 | file exists |
| D0.3 | [x] Pointers from BUILD_ORDER / PICKUP / 30–32 / OPEN_QUESTIONS | patches |
| D0.4 | [x] Q-F1…Q-F6 recorded in OPEN_QUESTIONS | rows present |

---

## Phase 1 — Sightlines & player parity  `! blocks desk fun`

**Goal:** Interactive ASCII trade surface over **existing** physics;
player can act on money/cargo paths NPCs already use.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P1.1 | [ ] CLI **trade desk mode** (persistent frame + pane keys t/y/r/…) | D0.1 | `trade` or desk enter; colony strip visible |
| P1.2 | [ ] TRADE pane: peer table `D_chart`, melt, treaty, WALL/SHIELD/OPEN | P1.1 | pure reading; no new dial |
| P1.3 | [ ] ROUTES pane: list live lanes + upkeep/lapse hints | P1.1 | uses `routes` / element_routes |
| P1.4 | [ ] DIPLO pane: treaty status + offer seal/renounce shortcuts | P1.1 | wraps existing treaty verbs |
| P1.5 | [ ] Arbitrage map (read-only): show A–M–B triangles merchant law would take | P1.2 | matches `merchant_step` geometry |
| P1.6 | [ ] Player **STAMP** verbs: balance, mint (crystal identity), aperture | S4.1 landed | S4.5 exit; SEAL B |
| P1.7 | [ ] Player **element** haul + optional element charter verb | E3.1–E3.2 landed | mirrors NPC steps |
| P1.8 | [ ] MARKET-1 cargo widen: offers may give/want **Resource \| STAMP \| ElementId** | P1.6, P1.7 | E3.6 + BOOK-S real; pins |
| P1.9 | [ ] Desk actions dispatch to deeds only (post/fill/charter/aperture) | P1.1–P1.8 | no shadow state |
| P1.10 | [ ] Observer: routes + STAMP treasury columns (reconcile dirty lane) | P1.3, P1.6 | S4.6 / E3.8 |
| P1.11 | [ ] Harness: player-path or pure-NPC pin that desk readings match engine melt | P1.2 | green board |
| P1.12 | [ ] Docs: Phase 1 land note in BUILD_ORDER + 33 status PARTIAL | P1.11 | |

**Phase 1 done when:** operator can open the desk, see peers/routes/melt,
post/clear STAMP and element cargo, charter a lane, without inventing
hulls or orbital phase.

**Pulls forward from tier board:** S4.5, S4.6, E3.6, E3.8.

---

## Phase 2 — Alignment economics  `! distance drama`

**Goal:** Planetary orbits change `D_eff` so routes get cheaper/dearer
by law; desk shows windows.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P2.1 | [ ] Spec pin: `D_eff = D_chart + D_orbit` (33 §3.2); single producer | P1.2 | doc + function site named |
| P2.2 | [ ] Drive θ (and ψ if required) from clock/site — exact lattice | P2.1, PlanetSited | deterministic replay |
| P2.3 | [ ] `D_orbit` chord/phase reading; zero if unsited | P2.2 | unit pins |
| P2.4 | [ ] `pair_melt` / `route_melt` consumes `D_eff` only | P2.1, P2.3 | call-not-copy; old tests updated |
| P2.5 | [ ] Alignment band CHEAP / FAIR / DEAR derived (no new melt dial) | P2.4 | desk + pins |
| P2.6 | [ ] ROUTES pane: projected melt over next K beats | P2.5, P1.3 | optimize “wait window” |
| P2.7 | [ ] Candidate route list includes alignment-sensitive sort | P2.5, P1.2 | |
| P2.8 | [ ] Year-voice or tick policy locked (Q-F1) | P2.2 | OPEN_QUESTIONS resolved or default coded |
| P2.9 | [ ] Harness: same chart, different phase ⇒ different melt | P2.4 | green |
| P2.10 | [ ] Docs + BUILD_ORDER Phase 2 land | P2.9 | |

**Phase 2 done when:** a player can delay a haul one window and measure
lower void loss under pure law.

**Kernel:** bump only if admits/covered change; prefer pure derivation.

---

## Phase 3 — Fleet capital  `! ships as economy`

**Goal:** Haulers/semis/runners purchasable and buildable; scrap market;
route capacity limited by assigned hulls; shipwright civ interface.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P3.0 | [ ] Claim tags for hull/scrap deeds in `dop.rs` header | — | 114+ reserved in comment |
| P3.1 | [ ] Scrap ledger (v1 one pool — Q-F3 default) | P3.0 | bank + conservation |
| P3.2 | [ ] Yard zone or converter deed (yard required — Q-F4) | P3.1 | ZoneKind or mint path |
| P3.3 | [ ] `HullMinted` + derived capacity/η/upkeep from grade/role | P3.2 | no undervied stat block |
| P3.4 | [ ] `HullAssigned` / clear assign; route free capacity reading | P3.3, ROUTE-1 | haul refuses over capacity |
| P3.5 | [ ] `HullScrapped` → scrap reverse fraction | P3.3 | conservation pin |
| P3.6 | [ ] Archetype `hull_catalog` + recipes (metabolism/disposition) | P3.3, genesis axes | two civs different catalogs |
| P3.7 | [ ] Vocabulary names per civ (voice only) | P3.6 | |
| P3.8 | [ ] η stack route×hull — one formula (Q-F2) | P3.4, P2.4 | pins |
| P3.9 | [ ] NPC shipwright: scrap bids + hull asks + commission | P3.3, market_step | pure NPC build/buy |
| P3.10 | [ ] Player FLEET pane + commission/buy/assign verbs | P3.4, P1.1 | desk complete for fleet |
| P3.11 | [ ] Route optimize uses hull mix (capacity + η) | P3.4, P1.3 | |
| P3.12 | [ ] Caravan office advantage composes (assign / η) | P3.8, O6.1 | |
| P3.13 | [ ] SCHEMA bump if hulls committed (V7 trail) + replay SEAL B | P3.3 | codec pins |
| P3.14 | [ ] Harness: pure NPC mints hull, assigns, haul capacity-gated | P3.9–P3.4 | green |
| P3.15 | [ ] Docs Phase 3 land; ratchet re-measured | P3.14 | |

**Phase 3 done when:** infinite aperture spam is impossible without hulls;
player can buy scrap from a shipwright archetype and assign a hauler.

**Pulls:** conductor shipwright step; optional T4/T5 yard advantages.

---

## Phase 4 — Local capital markets  `! money games`

**Goal:** Notes / Futures / Shares / Perps on **civ-filtered** books;
not one global static board; planet + archetype matter.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P4.0 | [ ] Claim instrument tags per `32` Wave 5 | P3.0 discipline | header first |
| P4.1 | [ ] **NOTE-1** issue / coupon / default (≠ ideology bond) | P1.6 STAMP, T8 optional | pins; Vault seize sketch |
| P4.2 | [ ] **FUT-1** margin + last-clear mark + delivery/cash | P4.1, P1.8 cargo | no float mark-to-model |
| P4.3 | [ ] **SHARE-1** issue / transfer / dividend from receipt window | P4.1, production evidence | |
| P4.4 | [ ] **PERP-1** funding from last-clear premium; liquidation | P4.2, Q-S4 | |
| P4.5 | [ ] Listing filter: archetype + evidence + office (T6 full stack; Q-F5 Spot-only without) | P4.1–P4.4, genesis | two civs different books |
| P4.6 | [ ] Planetary/local book identity (per world Exchange seat) | P4.5 | no global blob |
| P4.7 | [ ] `instrument_step` NPC lifecycle (funding, expiry, auctions) | P4.2–P4.4 | pure NPC |
| P4.8 | [ ] Freight/alignment seasons: Future on cargo or melt-index from last clears | P4.2, P2.5 | |
| P4.9 | [ ] EXCH pane lists **only** local listable instruments | P4.5, P1.1 | |
| P4.10 | [ ] Shipwright finance: Note/Future commission for hull build | P4.1, P3.9 | |
| P4.11 | [ ] FORWARD-1 coexistence map (Q-S3) documented + coded boundary | P4.1 | |
| P4.12 | [ ] Harness: NPC Note or Future lifecycle + player open/close | P4.7 | green |
| P4.13 | [ ] Docs Phase 4 land; `32` status update | P4.12 | |

**Phase 4 done when:** two civilizations show different listable books;
a Future prices freight around an alignment window without float marks.

**Aligns with:** `TASKS_TIER_RAIL_MARKET` Wave 5 (I5.*) — land once, check both boards.

---

## Phase 5 — Defense & deep rails

**Goal:** Protect capital in transit; optional R3/R4 exits for claims.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.1 | [ ] Escort role becomes active: convoy risk reading | P3.3 | |
| P5.2 | [ ] Interdiction / loss law (typed residual, not silent delete) | P5.1 | pins |
| P5.3 | [ ] DEFEND pane: assign escort, risk band | P5.1, P1.1 | |
| P5.4 | [ ] R2.3 promotion R1→R2 claims (tier board) | R2.1 landed | |
| P5.5 | [ ] Feature-gated R3 strand adapter for capital packets | P5.4, X7.1 | optional build |
| P5.6 | [ ] Feature-gated R4 court/datum path | P5.5 | optional build |
| P5.7 | [ ] T9 Anchor exit verbs for instruments/hull claims | P5.5–P5.6, O6.6 | |
| P5.8 | [ ] Lease hulls optional (Q-F6) | P3.3 | |
| P5.9 | [ ] End-to-end demo script: mint scrap→hull→route→Future→(optional R3) | all | documented run |
| P5.10 | [ ] Docs Phase 5 land; program success metric (33 §12) checked | P5.9 | |

**Phase 5 done when:** trade is defendable and deep-rail exits exist
without being required for R1–R2 fun.

---

## Cross-cutting checklist (every phase land)

- [ ] Tags claimed in `dop.rs` header before use  
- [ ] `covered` / `admits` / taxon `every_deed` / lens render  
- [ ] Replay SEAL B on touched chronicles  
- [ ] Codec schema trail if committed state grows  
- [ ] Ratchet ≤ ceiling (derive hull/instrument numbers)  
- [ ] Pure-NPC pin + player desk path where Phase requires  
- [ ] BUILD_ORDER + 33 status line + this board rows flipped  
- [ ] No `git add -A` over unrelated dirty lanes (observer)  

---

## Suggested first vertical slice (minimum fun)

```
[ ] P1.1–P1.4  desk + TRADE/ROUTES/DIPLO panes
[ ] P1.6–P1.8  STAMP + element + widened offers
[ ] P1.5       arbitrage map read-only
[ ] P2.1–P2.6  D_eff + alignment on route panel
[ ] P3.1–P3.4  scrap/yard/hull assign (one role: Hauler)
[ ] P4.1–P4.2  Note + Future only (defer Share/Perp)
```

Do **not** start Phase 5 combat before Phase 1 desk is playable.

---

## Dependency sketch vs tier board

```
TASKS_TIER_RAIL_MARKET          TASKS_TRADE_FLEET_DESK
─────────────────────          ──────────────────────
S4.5–S4.6 STAMP CLI/obs  ──►  P1.6, P1.10
E3.6 ElementId offers    ──►  P1.8
Wave 5 I5.* instruments  ──►  P4.* (same land)
O6.1 Caravan             ──►  P3.12
R2.3 / R3–R4 / X7        ──►  P5.4–P5.7
33 design                ──►  this entire board
```

---

## Definition of done (program)

Matches `33_TRADE_FLEET_AND_DESK.md` §12:

1. Desk sightlines + melt/alignment  
2. Player cargo clears (resource, element, STAMP)  
3. Route optimize with hulls  
4. Scrap / shipwright / hull commission  
5. Local Note or Future differs by civ evidence  
6. Zero extra human; R1–R2 offline from strand/datum  
