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
| **5** | Defense axioms · escort · covert terraform · espionage · deep rails | P3 hulls; `36_DEFENSE_AXIOMS` | Species-relative defense + KIN convoy + R3/R4 |

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
| P4.0 | [x] Tags **120–131**; SCHEMA_V9; next free **132** | P3.0 | dop header |
| P4.1 | [x] **NOTE-1** issue / redeem (collateral ≥ half face; ≠ BondSealed) | P1.6 STAMP | instrument_pins |
| P4.2 | [x] **FUT-1** margin both sides + cash settle vs **last clear** only | P4.1 | no float mark |
| P4.3 | [x] **SHARE-1** issue / transfer / dividend (⅛ pool × units/total) | P4.1 | instrument_pins |
| P4.4 | [x] **PERP-1** open / fund (⅛ premium) / close / liquidate | P4.2 | instrument_pins |
| P4.5 | [x] Listing filter: evidence · disposition · Exchange | P4.1–P4.4 | two books pin |
| P4.6 | [x] Local book identity (SCHEMA_V9 shares/perps trail) | P4.5 | |
| P4.7 | [x] `instrument_step`: clears, notes, perps fund, empiricist shares | P4.1–P4.4 | pure NPC |
| P4.8 | [x] Last-clear marks power Fut/Perp; alignment seasons use same marks | P4.2 | mark API |
| P4.9 | [x] Desk EXCH + CLI `note|future|share|perp` list open books | P4.5 | |
| P4.10 | [x] Shipwright finance: `mint_hull_financed` + `fleet mint … finance` | P4.1, P3.9 | HullFinanced 131 |
| P4.11 | [x] FORWARD-1 coexistence: Notes parallel (Q-S3) | P4.1 | 32 §3.5 |
| P4.12 | [x] Harness Note/Future/Share/Perp/HullFinance SEAL B | P4.7 | instrument_pins 5 |
| P4.13 | [x] Docs Phase 4 complete land | P4.12 | this board |

**Phase 4 done when:** two civilizations show different listable books;
a Future prices freight around an alignment window without float marks.
**Exit met** for Note/Fut/Share/Perp local books (2026-08-08).

**Aligns with:** `TASKS_TIER_RAIL_MARKET` Wave 5 (I5.*) — land once, check both boards.

---

## Phase 5 — Defense axioms · convoy · covert terraform · espionage · deep rails

**Authority design:** `36_DEFENSE_AXIOMS.md` (species-relative domains
CEL/ATM/COG/CYB/INF/KIN; covert atmospheric tampering; espionage stack).  
**Goal:** Protect the **entity** (not only cargo); make defense mean
different things for mineral / biotic / plasmic (+ clade colour);
optional R3/R4 exits for claims.

### P5-A — KIN vertical (capital in transit)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.0 | [x] Design lock `36_DEFENSE_AXIOMS.md` | P4 land | file exists |
| P5.1 | [x] Escort role **active**: `escort_assign` + cover/risk pure law | P3.3, P5.0 | defense_pins |
| P5.2 | [x] Interdiction residual on +melt hauls (`InterdictionStruck` 133) | P5.1 | pins + haul path |
| P5.3 | [x] DEFEND pane + `defend` CLI: posture stubs + escort/risk | P5.1, P1.1 | desk `d` |

### P5-B — Species-relative posture (readings + in-game reorient)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.11 | [x] Derive domain weights from metabolism × disposition × perception | P5.0, genesis | pure reading |
| P5.12 | [x] Clade colour table (voice + primary residual) from taxon/06 | P5.11 | no new dials |
| P5.13 | [x] DEFEND shows **non-human** posture copy (plasmic/lithic/hive…) | P5.11–P5.12 | CLI/desk |
| P5.13b | [x] In-game doctrine reorient (`PostureReoriented` 144); anatomy sealed | P5.11 | pins |

### P5-C — INF compose (veil / ward / counter-intel) · **NEXT LOADED**

**Authority:** `36` §13. Compose MASK-1 / WARDEN-1a / MANU-1 — no free
stealth points, no chronicle rewrite.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.14 | [ ] Surface MASK-1 / WARDEN-1a on DEFEND + `defend status` | P5.3, veil/ward landed | desk/CLI |
| P5.14b | [ ] INF rank already on posture; show veil/ward flags in suite if any | P5.14, P5.13 | reading |
| P5.15 | [ ] Detection grade from MANU-1 instrument floor (pure) | P5.14, MANU-1 | pin |
| P5.15b | [ ] False-assay / thin-scan refuse path gated by detection grade | P5.15 | pin |
| P5.15c | [ ] Docs + pins green; no schema bump unless state grows | P5.14–P5.15b | ratchet |

### P5-K2 — KIN open-up (species/civ colour, no free constants) · **LOADED**

**Authority:** `36` §12. Base risk formula stays; residual *kind* and
NPC routing re-colour from sealed axes / thermal / taxon / doctrine.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.50 | [x] Design: six KIN openings §12 (no undervived dials) | P5-A, P5-B | `36` §12 |
| P5.51 | [ ] K1 metabolism-keyed interdiction debit (resource match arm) | P5.2, P5.11 | pin SEAL B |
| P5.52 | [ ] K2 NPC `kin_step` disposition-routed (escort / risk / haul) | P5.51, P5.38 pattern | pure NPC |
| P5.53 | [ ] K3 clade colour on escort cover *voice* + optional weight | P5.12, P5.1 | reading |
| P5.54 | [ ] K4 fold orbit/thermal place into risk reading (CALL 35) | P5.1, 35 dynamo | pure |
| P5.55 | [ ] K5 perception fog of corridor (charted/instrument gates) | P5.15, P5.13b | refuse path |
| P5.56 | [ ] K6 CONVOY suite shows doctrine primary + residual kind | P5.35, P5.51 | UI |

### P5-E — COG + CYB

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.20 | [ ] Cognitive integrity reading (wants / practices load) | WANT-1, PRACTICE-1 | |
| P5.21 | [ ] Instrument-guard (anti-spoof) refuse path | MANU-1 | |
| P5.22 | [ ] Mesh-guard (aperture/STAMP clear witness hygiene) | routes, STAMP | |

### P5-M — Market defense (MKT) + optional suites (`36` §10)

**Ruling:** economic warfare is first-class. Suites are **UI over deeds**,
not a second physics. NPC must war without opening a suite.

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.30 | [x] Design: MKT domain + suite architecture (`36` §10) | P4 land, P5.0 | this row |
| P5.31 | [x] MKT rank on `defense_posture` + EXCH integrity flags | P5.30, P5.3 | reading |
| P5.32 | [x] Clear seal: LastClear requires N real clears (Q-S4) | P5.31, P4.2 | pin |
| P5.33 | [x] Circuit breaker deed `MarketSealed` / resume | P5.31 | pins |
| P5.34 | [x] Typed offenses: liquidity raid / clear poison (refuse+residual) | P5.32–P5.33 | pins |
| P5.35 | [x] CONVOY suite shell (dashboard over P5-A risk) | P5.1–P5.3 | optional UI |
| P5.36 | [x] MARKET WAR suite shell (breaker, clears, open instruments) | P5.33–P5.34 | optional UI |
| P5.37 | [ ] CYBER suite shell (instrument + mesh guards) | P5.21–P5.22 | optional UI |
| P5.38 | [x] NPC `market_war_step` (disposition-routed) | P5.34 | pure NPC |
| P5.39 | [ ] Species-relative MKT residual copy (mineral/biotic/plasmic) | P5.11, P5.31 | |

### P5-M2 — Matrix A3 + D8 + D10 (after §11 catalog)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.40 | [x] Design: expansion matrix §11 crosswalk (`36`) | P5.30 | doc |
| P5.41 | [x] Note cascade attack (tag 140) + cascade pressure | P5.34 | pins |
| P5.42 | [x] Collateral haircut ladder (pure floor on `issue_note`) | P5.41 | pins |
| P5.43 | [x] Early `resume_market` audit (clear depth gate) | P5.33 | pins |

### P5-D — ATM defense + covert terraform (land slice)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.16 | [x] Medium envelope reinforce deed (tag 142) | P5.0, thermal | pins |
| P5.17 | [x] CovertTamper deed + pressure scalar (tag 141) | P5.16 | pins |
| P5.18 | [x] TamperDetected at unit threshold (tag 143) | P5.17 | pins |
| P5.19 | [x] Dual-use gate: TERRA-1 overt vs CovertTamper covert | TERRA-1, P5.17 | doc+pin |
| P5.19b | [ ] Species-relative medium residual copy | P5.11, P5.17 | |

### P5-F — Espionage suite

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.23 | [ ] Spy probe / scout under veil | P5.14–P5.15 | |
| P5.24 | [ ] Tech hash exfil (no free mint of node) | P5.23, PLANNER-1 | |
| P5.25 | [ ] Sabotage: false prospect / instrument grade hit | P5.23 | |
| P5.26 | [ ] Disposition-routed NPC espionage_step | P5.23–P5.25 | pure NPC |

### P5-G — Deep rails (original capital exits)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.4 | [ ] R2.3 promotion R1→R2 claims (tier board) | R2.1 landed | |
| P5.5 | [ ] Feature-gated R3 strand adapter for capital packets | P5.4, X7.1 | optional |
| P5.6 | [ ] Feature-gated R4 court/datum path | P5.5 | optional |
| P5.7 | [ ] T9 Anchor exit verbs for instruments/hull claims | P5.5–P5.6, O6.6 | |
| P5.8 | [ ] Lease hulls optional (Q-F6) | P3.3 | |

### P5-H — Close-out

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P5.9 | [ ] Demo: escort convoy + one covert ATM arc + optional R3 | P5.1–P5.3, P5.17 | scripted |
| P5.10 | [ ] Docs Phase 5 land; 33 §12 metric; 36 status → PARTIAL/LAND | P5.9 | |

**Phase 5 done when:** (1) convoy capital is defendable with typed loss;
(2) DEFEND posture is **species-relative** (not human-only war);
(3) covert atmospheric tampering exists as metered dual of ATM defense;
(4) espionage composes veil/ward/instruments; (5) deep rails optional
and never required for R1–R2 fun.

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
