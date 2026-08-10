# TASKS — T4–T9 · R1–R4 · Element commerce · STAMP market

**Status:** IMPLEMENTATION BOARD (operator directive 2026-08-07).  
**Authority docs:** `30_OFFICES_AND_RAILS.md`, `31_ELEMENT_COMMERCE.md`,
`32_STAMP_MARKET.md`.  
**Related program (desk · fleets · alignment · local capital):**  
`33_TRADE_FLEET_AND_DESK.md` + **`TASKS_TRADE_FLEET_DESK.md`** (5 phases).  
That board **consumes** S4.5–S4.6, E3.6, Wave 5 I5.*, O6, R2.3/X7 —
land once, check both boards.  
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
| T1.5 | [x] Observer STATION column shows T4–T9 names | T1.1 | `tools/observer.py` ALL_TIERS + tier_of |
| T1.R | [x] **REFINE-1** + E3.5 (pulled into Wave-1 exit) | ground_step | `ElementRefined` tag 107; pure NPC refine |

---

## Wave 2 — Rails R1–R4 as data  `partial land`

| ID | Task | Depends | Exit |
|----|------|---------|------|
| R2.1 | [x] `Rail` type u8 1..=4 on element routes + wire | D0.5 | tags 110; `Rail::from_wire` |
| R2.2 | [x] Default R1 (Hearth) on local element charters | R2.1 | SCHEMA_V6 replay |
| R2.3 | [x] Promotion deeds R1→R2 (`RailPromoted` 148) | R2.1, E-APT-1 | pins |
| R2.4 | [x] Feature-gated R3 strand adapter (no hard path dep) | R2.1 | `deep-rail-strand` / `stamp-rail-strand` |
| R2.5 | [x] Feature-gated R4 isthmus/datum adapter | R2.1 | `deep-rail-court` / `stamp-rail-court` |
| R2.6 | [x] Observer RAIL column (max settled rail) | R2.1 | P8.2 arc/worlds · state uplink |

---

## Wave 3 — Element commerce  `partial land`

| ID | Task | Depends | Exit |
|----|------|---------|------|
| E3.1 | [x] **E-APT-1** element aperture legs (id in value; conservation) | ground_step live | tags 108–109; element_move |
| E3.2 | [x] **E-ROUTE-1** ElementRouteChartered (tag 110) + rail + SCHEMA_V6 | E3.1, ROUTE-1 | charter + melt via routes table |
| E3.3 | [x] Conductor `element_trade_step` NPC | E3.1 | pure NPC clear in `wave1_era_pins` |
| E3.4 | [x] **REFINE-1** ElementRefinery zone; derive rates from family/tier | 14 §5 | two seeds different chains |
| E3.5 | [x] Conductor build prefers refinery when raw surplus | E3.4 | pin |
| E3.6 | [x] **E-MKT-1** offers carry ElementId cargo | E3.1, MARKET-1 | `Cargo` union; tag 49 wire; clear_trade; `market_step` element intents; `element_market_pins` |
| E3.7 | [x] WANT-1 may target element ids | E3.6 | tags 163–164 · element_wants · market demand intents · SCHEMA_V18 |
| E3.8 | [ ] Observer `routes` + `refineries` views | E3.2, E3.4 | |
| E3.9 | [ ] Treaty allow-lists for element ids | E3.1, TREATY | |

---

## Wave 4 — STAMP unit + spot  `partial land`

| ID | Task | Depends | Exit |
|----|------|---------|------|
| S4.1 | [x] **STAMP-1** balances on orb (tag 73); mint + aperture deeds | D0.5 Q-S1 | tags 111–113; stamp_move; SEAL B |
| S4.2 | [x] Crystal→STAMP identity convert (1:1; no curve dial) | S4.1 | `mint_stamp_from_crystal` |
| S4.3 | [x] **BOOK-S** minimal STAMP spot vs Resource (crystal/iron/biomass) | S4.1 | stamp_market_step legs |
| S4.4 | [x] **MM-1** conductor `stamp_market_step` pure NPC | S4.3 | `pure_npc_stamp_spot_clears` green |
| S4.5 | [x] CLI `stamp` verbs (balance · mint · aperture) | S4.1 | Phase-1 desk land |
| S4.6 | [x] Observer STAMP (tag 73) + lens JSON | S4.1 | worlds table STAMP col |

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
| O6.1 | [x] T4 Caravan: pair_melt suppress by Resonator grade η | E3.2, T1.x | melt × (1 − grade) |
| O6.2 | [x] T5 Crucible: refine spend half (derive 1+1) | E3.4, T1.x | office advantage only |
| O6.3 | [x] T6 Exchange: only seated MM may list Share/Note/Fut/Perp | Wave 5, T1.x | require_exchange_mm on issue |
| O6.4 | [x] T7 Assay: consilience challenge / slash path | E3.1 | tags 154–155 |
| O6.5 | [x] T8 Vault: custody inventory + margin accounts | I5.x | tags 152–153 · SCHEMA_V14 |
| O6.6 | [x] T9 Anchor: only exit R3/R4 (seat required for promote/export) | R2.4–R2.5 | pin |

---

## Wave 7 — Cross-repo settlement

| ID | Task | Depends | Exit | Repo |
|----|------|---------|------|------|
| X7.1 | [x] Strand schema STAMP/Note/Share (`cfe::xrail::strand`) | R2.4, S4.1 | round-trip | netstratum |
| X7.2 | [x] Isthmus opaque frame (`cfe::xrail::isthmus`) | R2.5 | no path dep | netstratum |
| X7.3 | [x] Datum multi-axial verify (`cfe::xrail::datum`) | X7.2 | multi-axial close | netstratum |
| X7.4 | [x] Anchor import refuses bad proofs (tags 149–151) | X7.1–X7.3 | pin | netstratum |
| X7.5 | [x] E2E pin: export/import stamp + court (feature `xrail-test`) | all | `xrail_pins` | netstratum |

---

## Chemistry science (alchemy bench as play)

| ID | Task | Depends | Exit |
|----|------|---------|------|
| C1.1 | [x] Civ survival utility from genesis axes | #37 compound | pure `almanac::science` |
| C1.2 | [x] Dark-7 gravity wells (green…pride) | C1.1, WANT-1 | `dark_pulls` + residual 158 |
| C1.3 | [x] Science-ranked bench | C1.1–C1.2 | `bench_science` CLI |
| C1.4 | [x] Isomorphic union (tags 156) | #37 | mass-exact gem fuse |
| C1.5 | [x] Chem catalog creation (tag 157) | C1.1 | crystal fee · novelty relief |
| C1.6 | [x] NPC `chemistry_step` disposition-routed | C1.3 | pure NPC · cycle after refine |
| C1.7 | [x] Stellar capacity game (star lines as project) | locality | `stellar_capacity_board` · `bench star capacity` |
| H1.1 | [x] BENCH hub (planetary · build · star · chem · econ) | C1.x | CLI `bench` |
| H1.2 | [x] TRADE hub (books · fleet · rails · vault · aperture) | H1.1 | CLI `trade` |
| H1.3 | [x] Desk panes deep-link into hub domains | H1.1–H1.2 | desk HUB strip · `h`/`hc` · `docs/HUBS.md` |
| H1.4 | [x] Back-outs on every nested view (`bench`/`back`/`..`) | H1.1 | never stuck |
| H1.5 | [x] Bench research · intel · link · doctrine · estate | H1.1 | map missing games |
| H1.6 | [x] Deep boards: research_board · intel_board · link_board · doctrine | H1.5 | engine pure |
| H1.7 | [x] Deep deeds from hub: forge · probe · exfil · sabotage · estate | H1.6 | mut bench |
| H1.8 | [x] Root INTEL peer to BENCH/TRADE (knowledge activity, not craft) | H1.7 | `intel` |
| H1.9 | [x] Full domain wire: offices·medium·fleet·assay·culture·books·opsec·stamp·element·diplo local/stellar | H1.8 | `docs/HUBS.md` |
| H1.10 | [x] Dual law: activity root S ⇒ bench S (diplomatic · intel · trade craft twins) | H1.9 | pin |
| H1.11 | [x] Preferred name `diplomatic` (diplo alias) + science benches (chem·bio·phys·math·research) | H1.10 | pin |
| C1.8 | [x] Seated science labs: pay unlock · dual ± · explosion · synth/clone/skin | H1.11 | tags 159–162 · SCHEMA_V16 · kernel 0.37 |
| C1.9 | [x] Taxon economy: extractive·regenerative·balanced + flatlander baseline + planet fit | C1.8 | `taxon_economy` pins · regen tech soften |
| C1.10 | [x] +6 class · +6 office tiers · **u16 open clade** (25×13=345) · seat +25 | C1.9 | SCHEMA_V17 · ceilings 13/345 |

---

## Wave 8 — Exhaustive play / anti click-to-win

| ID | Task | Depends | Exit |
|----|------|---------|------|
| P8.1 | [x] Shared AP budget across build / prospect / research / quote | year-voice | `ap_budget` · opportunity cost pin |
| P8.2 | [x] Trajectory + hunger + rail on observer | R2.6, research | state uplink · `observer arc` · worlds TRAJ/HUNGER/RAIL |
| P8.3 | [x] Labor/population derived from habitats | 02 | `labor_force` · habitats + clones · polity labor |
| P8.4 | [x] Closed-ruins history view | observer | `observer ruins` |
| P8.5 | [x] Balance pass: AP capacity ruled table (slice) | P8.1 | `AP_BASE` · `AP_HABITAT_MULT` · `AP_EXCHANGE_BONUS` |
| POL.1 | [x] Polity root vent open (CLI scaffold · genesis/taxon/mint/ap) | H1.x | `polity` hub · docs/HUBS.md |
| POL.2 | [x] Dual-law `bench polity` craft twin | POL.1 | offices · doctrine · labor · practice |
| POL.3 | [x] Conductor identity_step (habitat labor parity) | POL.1 | pure NPC after chemistry |

---

## Suggested first vertical slice (minimum fun)

```
[x] T1.1–T1.4 stations T4–T9 + NPC seats     ← Wave-1 2109bf0
[x] E3.4–E3.5 REFINE-1 + NPC build           ← Wave-1 2109bf0
[x] E3.1 + E3.3 element aperture + NPC trade ← Wave-2 4686399
[x] E3.2 ElementRouteChartered + SCHEMA_V6   ← Wave-3 this land
[x] S4.1–S4.4 STAMP + spot + NPC MM          ← Wave-4 this land
[x] R2.1–R2.2 Rail type + default R1         ← this land
[x] O6.1–O6.2 Caravan/Crucible advantages    ← this land
[x] R2.3–R2.5 + X7.1–X7.5 adapters           ← landed (feature-gated R3/R4)
[ ] S4.5–S4.6 CLI + observer STAMP           ← open
```

**Wave-2 land:** tags 108–109, kernel 0.21.0, `element_trade_step`,
harness `pure_npc_element_route_clears` green.

**Wave-3/4 land:** tags 110–113, orb STAMP TLV 73, kernel **0.22.0**,
SCHEMA_V6 element_routes, `stamp_market_step`, harness
`pure_npc_stamp_spot_clears` green. Ratchet stays ≤112.

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
