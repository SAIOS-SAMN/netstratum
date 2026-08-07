# TASKS — Trade desk · fleets · alignment · local capital (5 phases)

**Status:** IMPLEMENTATION BOARD — operator directive 2026-08-07.  
**Authority doc:** `33_TRADE_FLEET_AND_DESK.md`.  
**Matter loop:** `34_MATTER_BALANCE.md` (elements · alchemy · fleets ·
entropy∥syntropy — partial land with Phase 3 fleets).  
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
| P1.1 | [x] CLI **trade desk mode** (`desk` / `desk enter`, panes t/y/r/a/…) | D0.1 | colony strip; interactive loop |
| P1.2 | [x] TRADE pane: peer table `D_chart`, melt, treaty, WALL/SHIELD/OPEN | P1.1 | calls `pair_melt` only |
| P1.3 | [x] ROUTES pane: carrier + element lanes + upkeep hints | P1.1 | `routes` CLI also lists element lanes |
| P1.4 | [x] DIPLO pane: standing treaties + renounce hint | P1.1 | reads `treaties()` |
| P1.5 | [x] Arbitrage map (read-only): walled pairs with open M-bridge | P1.2 | merchant geometry |
| P1.6 | [x] Player **STAMP** verbs: balance, mint, aperture | S4.1 landed | S4.5; SEAL B pin |
| P1.7 | [x] Player **element** aperture + charter (rail default R2) | E3.1–E3.2 landed | mirrors NPC |
| P1.8 | [~] MARKET-1 cargo widen Resource\|STAMP\|ElementId | P1.6, P1.7 | **deferred** — clear via stamp/element verbs; offer widen next slice |
| P1.9 | [x] Desk loop dispatches world verbs to `command` (no shadow ledger) | P1.1 | P1.9 |
| P1.10 | [x] Observer STAMP (tag 73) + world STAMP column; lens JSON `stamp` | P1.6 | S4.6 partial; routes pane in desk/CLI |
| P1.11 | [x] `clients/cli/tests/desk_pins.rs` melt ≡ pair_melt + stamp mint | P1.2 | green |
| P1.12 | [x] Docs: Phase 1 land note + 33 PARTIAL | P1.11 | |

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
| P2.1 | [x] `effective_distance` / `d_orbit` / `route_melt` (33 §3.2) | P1.2 | cfe::world |
| P2.2 | [x] θ advanced pure-from-clock: `index + now/60` on Slot lattice | P2.1 | no receipt spam |
| P2.3 | [x] `d_orbit` via phased `separation`; 0 if unsited | P2.2 | |
| P2.4 | [x] `route_melt` + clear_trade / conductor / CLI use D_eff | P2.1, P2.3 | call-not-copy |
| P2.5 | [x] Alignment CHEAP/FAIR/DEAR/— from D_orbit thresholds | P2.4 | desk colors |
| P2.6 | [x] ROUTES pane: projected melt next 6 year-steps | P2.5 | |
| P2.7 | [x] TRADE peers sorted OPEN by rising melt | P2.5 | |
| P2.8 | [x] Q-F1 coded: `ORBIT_PHASE_TICKS = 60` year-voice | P2.2 | |
| P2.9 | [x] `phase_shifts_orbit_and_melt` pin | P2.4 | green |
| P2.10 | [x] Docs + BUILD_ORDER Phase 2 land | P2.9 | |

**Phase 2 done when:** a player can delay a haul one window and measure
lower void loss under pure law.

**Kernel:** bump only if admits/covered change; prefer pure derivation.

---

## Phase 3 — Fleet capital  `! ships as economy`

**Goal:** Haulers/semis/runners purchasable and buildable; scrap market;
route capacity limited by assigned hulls; shipwright civ interface.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P3.0 | [x] Tags 114–117 claimed; next free **118**; orb TLV **74** scrap | — | dop header |
| P3.1 | [x] Scrap pool on orb (tag 74); iron→scrap salvage | P3.0 | ScrapSalvaged |
| P3.2 | [x] ZoneKind::ShipYard = 10 (habitat cost) | P3.1 | yard required Q-F4 |
| P3.3 | [x] HullMinted + capacity/η from 1+1 ladder + Resonator eighth | P3.2 | |
| P3.4 | [x] HullAssigned; free_capacity; haul gate when melt>0 | P3.3 | |
| P3.5 | [x] HullScrapped → half scrap reverse | P3.3 | |
| P3.6 | [x] Metabolism catalog filter (mineral/biotic/plasmic) | P3.3 | |
| P3.7 | [x] hull_voice names per metabolism | P3.6 | |
| P3.8 | [x] Hull η stacks as product of complements in pair_melt | P3.4 | Q-F2 |
| P3.9 | [x] conductor `fleet_step` salvage/mint/assign | P3.3 | pure NPC |
| P3.10 | [x] `fleet` CLI + desk FLEET pane + help fleet | P3.4 | |
| P3.11 | [x] ROUTES/TRADE see capacity via free_capacity / η stack | P3.4 | |
| P3.12 | [x] Caravan O6.1 still stacks with hull η | P3.8 | |
| P3.13 | [x] SCHEMA_V7 hull trail + replay SEAL B | P3.3 | |
| P3.14 | [x] `pure_npc_fleet_hauls` green | P3.9 | |
| P3.15 | [x] Docs + kernel 0.23.0; ratchet ≤112 | P3.14 | |

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
