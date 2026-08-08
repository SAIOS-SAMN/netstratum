# 33 — TRADE DESK · FLEETS · ALIGNMENT · LOCAL CAPITAL

**Status:** PARTIAL LAND — **Phases 1–4 complete** on main; **Phase 5
partial** — P5-A KIN · P5-B posture · P5-D ATM/dual-use · P5-M/M2 market
landed (`36`); **next loaded: P5-C INF**, then **P5-K2** KIN colour
(`36` §12–§13). Implementation board: **`TASKS_TRADE_FLEET_DESK.md`**.

**Does not overturn:** CHARTER exactness, SEAL B, call-not-copy, tuned-
constant ratchet, ROUTE-1 / MARKET-1 / STAMP-1 as landed. **Does not**
replace ideology standing bonds (09) with market Notes. **Does not**
require strand/datum for local fun (R1–R2 first).

**Subordinate to:** `00_CHARTER`, `01_ECONOMY`, `02_COLONY_LOOP`,
`05_THERMAL_BALANCE`, `12_UNION`, `16_UNIT`, `28_VOCABULARY`,
`29_CONSTRUCTION`, `30_OFFICES_AND_RAILS`, `31_ELEMENT_COMMERCE`,
`32_STAMP_MARKET`, **`34_MATTER_BALANCE`** (elements · alchemy · fleets
· entropy∥syntropy).

**Companion board:** `TASKS_TRADE_FLEET_DESK.md` (Phases 1–5).  
**Related board:** `TASKS_TIER_RAIL_MARKET.md` (offices, STAMP, instruments
rows that this program **consumes**).

**Baseline at lock:** kernel `0.22.0`, tags through **113**, SCHEMA_V6,
STAMP spot NPC green (`9e641ba`). Next free tag **114**, next kernel
minor **23** — claim before Phase 2+ bumps.

---

## 0. Problem statement

Players running a civilization need to:

1. Open an **interactive ASCII suite** for trade, diplomacy, routes,
   harvesting, and arbitrage — while still exploring, defending, and
   growing the colony.
2. See **distance** between stars/worlds and **candidate routes** with
   live cost.
3. Feel **planetary alignment**: as orbits advance, a lane gets cheaper
   or dearer by law (not flavor text).
4. Own **haulers / semis** (and related hulls) that are **purchasable
   and buildable** in-game; interface with shipwright civilizations
   via scrap buy/sell and commissioned builds.
5. **Optimize** a pulled-up trade route (hull mix, timing, cargo policy)
   without cheating physics.
6. Trade **Notes / Futures / Perps** (and Shares) on **localized**
   books that depend on **planet + civilization archetype** — not one
   static global market or one universal ship list.

Today the engine has strong **carrier barter + melt + ROUTE-1** and
partial **element / STAMP** (mostly NPC). It lacks fleets, orbital
distance in melt, player trade desk, and civ-filtered capital books.

---

## 1. Design laws (non-negotiable)

| # | Law |
|---|-----|
| L1 | **Physics one; culture many.** Melt, conservation, rails, STAMP math are universal. Hull *names*, yard *recipes*, and *which instruments list* derive from genesis axes + offices + evidence. |
| L2 | **One distance producer.** Route cost calls a single `D_eff` (and through it `pair_melt` or its successor). No second melt formula in the desk. |
| L3 | **Ships are receipted capital**, not UI skins. Capacity, η, upkeep, scrap reverse, owner, home orb — all on chain. |
| L4 | **Menu is a lens.** Every optimize / buy / charter / assign ends as a deed. No secret desk ledger. |
| L5 | **Markets are books on seats.** Unlisted = absent. Archetype filters listings; it does not fork the clearing math. |
| L6 | **Ideology Bond ≠ market Note.** Tags 33–34 stay ideology; market debt is Note (32). |
| L7 | **Exact lattice only.** Orbital phase advances on exact rationals / discrete slots — no float ephemeris. |
| L8 | **No undervied dials.** Hull grades and costs derive from existing ladders (refinement, instrument grade, sealed parameters) or mint as APPROVED-SEED with ratchet honesty. |
| L9 | **Solo and interplanetary both.** R1 hearth books and zero-melt same-system hauls are first-class; R2 aperture is the default interstellar door. |
| L10 | **Combat is Phase 5.** Trade must be fun before interdiction is required. |

---

## 2. Layer stack

```
┌──────────────────────────────────────────────────────────────┐
│  ASCII DESK (lens)                                           │
│  TRADE · DIPLO · ROUTES · FLEET · EXCH · GROW · DEFEND · EXPLORE │
└────────────────────────────┬─────────────────────────────────┘
                             │ verbs → deeds
┌────────────────────────────▼─────────────────────────────────┐
│  CAPITAL + FLEET                                             │
│  Scrap · Yard · Hull · Convoy assign · Local books           │
└────────────────────────────┬─────────────────────────────────┘
                             │ calls
┌────────────────────────────▼─────────────────────────────────┐
│  LANDED CORE (compose, do not fork)                          │
│  Resources · Elements · STAMP · melt · ROUTE-1 · MARKET-1    │
│  Treaties · Merchant · Element/STAMP steps · Offices T4–T9   │
└──────────────────────────────────────────────────────────────┘
```

---

## 3. Distance and planetary alignment

### 3.1 Quantities

| Symbol | Meaning | Source |
|--------|---------|--------|
| `D_chart` | Spatial L1 between world chart positions | `spatial_l1` (landed) |
| `D_key` | Wrapped key gap | already inside `pair_melt` |
| `D_orbit` | Chord / phase gap between sited orbs at current θ, ψ, r | `PlanetSited` + clock (θ advances per-world) |
| `D_eff` | Effective transit distance | **one function** (new): see §3.2 |
| `alignment` | Desk reading of relative phase | derived band: CHEAP / FAIR / DEAR |
| **orbit period** | ticks per θ-slot step | `ORBIT_PHASE_TICKS × core_magnetism × polity / star.magnetism()` — **not lockstep** |

### 3.2 Effective distance (sketch — pin at Phase 2 land)

```
D_eff(a, b, beat) =
    D_chart(a, b)
  + D_orbit(a, b, beat)     // 0 when either unsited or same-site degenerate
```

`pair_melt` (or a thin wrapper `route_melt`) consumes `D_eff` exactly
as it consumes distance today. Treaty factors, route η, Caravan grade,
and (Phase 3) hull η stack **after** distance, never invent a second
iceberg.

**Same star system / multi-planet:** `D_chart` small or zero; orbital
chord dominates → alignment is the drama.  
**Interstellar:** chart dominates; orbital phase is a bounded exact
correction (still useful for timing Futures).

### 3.3 Phase advance

- θ (and ψ if used) advance by a **ruled exact step** per tick or
  year-voice — discrete slots already hinted on `PlanetSited`.
- Advancement is receipted or pure-from-clock under SEAL B (choose one
  at land; prefer **derived from clock + site**, no per-beat spam
  receipts unless state must move).
- Desk shows: current band, next N beats' projected `D_eff` / melt.
- **Not lockstep:** each world's θ steps every
  `ORBIT_PHASE_TICKS × core_magnetism(orbit_r grain) × disposition
  / StarClass::magnetism()` ticks — core ring depth, sealed polity, and
  stellar dynamo grade. Relative alignment moves even when both clocks
  advance together.

### 3.4 Candidate routes (reading, not mint)

For each peer world the desk lists:

```
peer · D_eff · melt_now · melt_if_charter(η) · treaty
    · surplus→deficit cargo hints · free hull capacity
    · WALL | SHIELD | OPEN
```

Charter remains a deed (alloy + η). War walls; shield blocks until
crossing.

---

## 4. Fleets — haulers, semis, shipwrights

### 4.1 Universal roles (physics); cultural names (voice)

| Role | Function |
|------|----------|
| **Hauler** | Baseline cargo capacity on R1–R2 |
| **Semi** | Higher capacity / grade ladder step |
| **Runner** | Lower capacity, better η (courier) |
| **Escort** | Convoy defense budget (Phase 5; Phase 3 may mint as inert flag) |

Civs do **not** get different conservation laws. They get:

- **Vocabulary names** (fold / word table)
- **Yard recipes** (which scrap + carriers feed the mint)
- **Catalog filter** (which roles their yard may mint)

### 4.2 Hull as receipted asset

```
Hull {
  hull_id,           // content or deal-scoped id
  role,              // Hauler | Semi | Runner | Escort
  grade,             // u8 ladder — capacity/η/upkeep derive
  owner_entity,
  home_orb,
  assigned_route,    // Option<deal>
  rail_max,          // R1 or R2 (R3+ later)
  // capacity, η_hull, upkeep, scrap_value: DERIVED at read
}
```

**Derivation sketch (pin at Phase 3):**

- `capacity(grade, role)` from a short sealed ladder (like refinement)
- `η_hull` from grade (Unit in [0,1)), stacks with route η under
  `1 − (1−η_route)·(1−η_hull)` or ruled min — **one formula**
- `upkeep` from encasement/habitat-class bills (call-not-copy)
- `scrap_value` = reverse mint fraction (exact)

### 4.3 Scrap → yard → hull

```
Scrap (ledger entry: kind + amount)
  → Yard (zone or office-gated converter)
  → HullMinted (deed)
  → AssignToRoute | Idle | ScrapHull
```

**Shipwright interface (compose markets):**

| Door | Pattern |
|------|---------|
| Spot | Offer scrap ↔ STAMP / cargo (MARKET-1 / STAMP spot) |
| Commission | Note or Future: pay / deliver → HullMinted for buyer |
| Service deed | Pay yard bill on their world; owner = you or lease |

NPC shipwrights: bid scrap when starved; ask hulls when finished;
respect war/treaty; catalog from archetype.

### 4.4 Archetype → hull catalog (examples, not hard forks)

| Axis / office | Effect |
|---------------|--------|
| Mineral metabolism | Iron/alloy scrap recipes; heavy Hauler/Semi |
| Biotic | Biomass/chitin recipes; organic names |
| Plasmic | Joule/sink budget on mint and upkeep |
| Expansionist | Runner emphasis; freight Futures affinity (Phase 4) |
| Empiricist | Grade-gated yards; assay-linked Shares |
| T4 Caravan | Route assign privilege / melt stack advantage (compose O6.1) |
| T5 Crucible | Scrap refine advantage (compose O6.2) |
| T6 Exchange | Full instrument listing (Phase 4) |
| T8 Vault | Margin custody for Notes/Futs |

---

## 5. Localized capital markets

Compose `32_STAMP_MARKET` instruments; **filter listings**.

| Book | Lists when |
|------|------------|
| Spot | STAMP ↔ cargo the line actually handles (resource / element evidence) |
| Share | Production streams **they** receipt (zone class / refined family) |
| Note | STAMP or primary cargo debt; Vault if collateral |
| Future | Delivery of local surplus types; mark = last clear only |
| Perp | Only indices with ≥N **local** clears in window |

**Not static:** two civilizations on the same planet class can still
differ by disposition/metabolism/offices/history. Absence of evidence
⇒ absence of listing (anti-fake depth).

**Alignment × Futures:** instruments do not price θ directly. Alignment
changes `D_eff` → melt/volume → last-clear indices that Futures/Perps
already mark.

**FORWARD-1** stays bilateral delivery pact; careful migration map to
Note at Phase 4 land (Q-S3).

---

## 6. ASCII desk — interactive suite

### 6.1 Persistent frame

```
┌─ {world} · {station/T#} · R{max} ── STAMP · cargo · hulls idle ─┐
│ EXPLORE [e] DEFEND [d] GROW [g] TRADE [t] DIPLO [y] FLEET [f] EXCH [x] │
├─ active pane ───────────────────────────────────────────────────┤
│ …                                                                │
├─ colony strip (always) ─────────────────────────────────────────┤
│ zones · famine · research · heat band                            │
└──────────────────────────────────────────────────────────────────┘
```

### 6.2 Panes

| Key | Pane | Contents |
|-----|------|----------|
| `t` | TRADE | Peers, D_eff, melt, alignment, post/fill, harvest |
| `y` | DIPLO | Treaty ladder, war cost, council |
| `r` | ROUTES | Live lanes, optimize, upkeep, lapse, assign hulls |
| `f` | FLEET | Hulls, scrap, yard, buy/sell/commission |
| `x` | EXCH | **Local** books only (archetype-filtered) |
| `g` | GROW | Existing colony loop surface |
| `d` | DEFEND | Convoy + domains (CEL…MKT) + veil/ward + espionage + suite entry (`36`) |
| `e` | EXPLORE | Scan/prospect — feeds cargo identity |

### 6.3 Route optimize panel

Pull up `deal#…`:

```
peer · η_lane · melt_now · alignment · projected melt[beat..beat+k]
assigned hulls · capacity used/free
last fills · void loss · STAMP/cargo PnL
actions: assign | schedule harvest | wait window | upgrade lane
         | escort | scrap hull
```

Optimize = hull mix + timing + cargo policy. **Never** silent melt edit.

### 6.4 Implementation homes

| Surface | Role |
|---------|------|
| `clients/cli` | Primary desk mode / key dispatch → verbs |
| `tools/observer.py` | Board reading (routes, STAMP, fleet) — reconcile if dirty |
| `engine/lens` | Shared render strings for events |
| Conductor | NPC parity (shipwright, instrument_step, alignment) |

---

## 7. Solo planet vs interplanetary

| Mode | Market shape |
|------|----------------|
| **Solo / multi-orb one world** | R1 hearth books; zero-melt internal hauls; yard local; thin counterparty unless NPC or second save |
| **Interplanetary** | R2 aperture; full D_eff; routes; shipwright cross-world commission; local books at each Exchange |

Phase 1 must not assume multi-world only: candidate list and STAMP
verbs work; Phase 3 hulls work on one orb; Phase 4 R1 bourse optional
without T6 full stack.

---

## 8. Five implementation phases (summary)

Authority detail and task IDs: **`TASKS_TRADE_FLEET_DESK.md`**.

| Phase | Name | Player-visible exit |
|-------|------|---------------------|
| **1** | **Sightlines & parity** | **LANDED** — desk panes; stamp/element verbs; observer STAMP |
| **2** | **Alignment economics** | **LANDED** — `D_eff`, year-voice θ phase, alignment bands, melt windows |
| **3** | **Fleet capital** | **LANDED** — scrap/yard/hulls, capacity-gated hauls, fleet_step, CLI |
| **3** | **Fleet capital** | Scrap · yard · hull mint/assign/scrap; shipwright NPC spot; route capacity limited by hulls |
| **4** | **Local capital markets** | Civ-filtered Note/Future/Share/Perp books; instrument_step; freight/alignment seasons tradeable |
| **5** | **Defense axioms · escort · covert terraform · market war · espionage · deep rails** | Species-relative 7 domains (`36`); P5-A/B/D/M landed; **next P5-C INF** then P5-K2; R3–R4 optional |

**Dependency spine:**

```
Phase1 (desk + player money/cargo)
   └──► Phase2 (D_eff) ──► Phase3 (hulls use D_eff + capacity)
                              └──► Phase4 (instruments price freight)
                                     └──► Phase5 (defend capital)
```

Parallel ok: Phase 1 UI can land while Wave-5 instrument *law* is
spec'd; Phase 4 **code** waits on STAMP player path (Phase 1) + Vault
design from `32`.

---

## 9. Tag / schema / kernel register (claim at land)

| Concern | At design lock | At first code land |
|---------|----------------|--------------------|
| Next ClosureEvent tag | **114** | claim consecutive block in `dop.rs` header **before** impl |
| Kernel minor | next free **23** | bump only when `admits`/covered grow |
| Codec schema | V6 | V7+ if hulls/scrap committed state need snapshot trail |
| Orb / TLV | STAMP 73 | scrap/hull tags as needed (append-only) |
| Ratchet | ≤112 | derive hull ladders; do not raise ceiling |

**Expected new deeds (names provisional — claim tags at Phase 3/4):**

- Phase 2: maybe none if phase is pure-derived; else `OrbitAdvanced` only if state must move  
- Phase 3: `ScrapBooked` / `HullMinted` / `HullAssigned` / `HullScrapped` (collapse if possible)  
- Phase 4: instrument deeds per `32` (Share/Note/Fut/Perp lifecycle)

---

## 10. Explicit non-goals

- Soft float orbits or continuous-time funding  
- Global identical ship SKU list and global single stock board  
- Replacing ROUTE-1 with a parallel melt engine  
- Full fleet combat before trade desk fun (Phase 5 only)  
- Real-money billing (CHARTER / 01 split stands)  
- Forcing strand/datum for R1–R2 play  

---

## 11. Open rulings (record; do not invent silently)

| ID | Question | Default if forced early |
|----|----------|-------------------------|
| Q-F1 | θ advance per tick vs year-voice only | year-voice coarse + tick fine optional |
| Q-F2 | Hull η stack formula with route η | product of complements (pin Phase 3) |
| Q-F3 | Scrap kinds: one pool vs per-resource scrap | start **one scrap** derived from iron/alloy burn |
| Q-F4 | Can T3 mint hulls without yard zone | yard zone **required** (office optional advantage) |
| Q-F5 | R1 bourse without T6 | yes, Spot-only; T6 unlocks full stack |
| Q-F6 | Lease vs own hull commission | own v1; lease Phase 5+ |
| Q-S3 | FORWARD-1 → Note migration | keep parallel until Note pins green |
| Q-S4 | Min clears before perp index | from `32`; enforce in Phase 4 |

Carry into `OPEN_QUESTIONS.md` when this doc lands.

---

## 12. Success metric (program)

A pure-NPC era **and** a player ASCII session can:

1. Discover peers and **see** melt/alignment/candidate routes  
2. Post and clear cargo (resource, element, STAMP)  
3. Charter and **optimize** a lane with assigned hulls  
4. Buy scrap / commission a hull from a shipwright archetype  
5. Trade at least **Note or Future** on a **local** book that differs
   by civilization evidence  
6. Do (1–5) without a second human and without strand/datum  

Until then, this document is the holding pen so the vision is not lost.
