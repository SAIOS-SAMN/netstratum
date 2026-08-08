# 30 — OFFICES AND RAILS: T4–T9 and R1–R4

**Status:** PARTIAL LAND — operator directive 2026-08-07; **Wave-1
`2109bf0`** opened the T-line (stations T4–T9, seat widths, ascend
preconditions, pure-NPC seats to T4/T6). **R-line still DESIGN.** Does
**not** overturn the ruling that **progenitors are the top** (T0 creates
the board). Higher **numbers** on the T-line are **deeper function
bands**, not higher rank. The R-line is a **separate ladder** (settlement
altitude), never a recolor of T.

**Subordinate to:** `00_CHARTER`, `12_UNION`, `01_ECONOMY`, `14_ALMANAC`,
`16_UNIT`, `19_REGISTRY`, `28_VOCABULARY`. Cross-repo: `isthmus` (superhighway),
`datum` (authority bench), `xylarium/strand` (domain tollway).

**Companion landings:** `31_ELEMENT_COMMERCE.md`, `32_STAMP_MARKET.md`,
`33_TRADE_FLEET_AND_DESK.md` (desk · fleets · alignment · local capital),
`TASKS_TIER_RAIL_MARKET.md`, `TASKS_TRADE_FLEET_DESK.md`.

---

## 0. Why two lines

| Line | Answers | Stored as | Derived reading |
|------|---------|-----------|-----------------|
| **T — Office** | *What function does this line hold on the board?* | `station` (and new station codes) | `tier_of(station)` → T0…T9 |
| **R — Rail** | *How far can this claim travel before it dies?* | receipted rail altitude on instruments / routes | R1…R4 |

A line can be T3 (common) and still post R2 claims (aperture-cleared).
A T6 Exchange seat that only settles R1 is a **local** market, not a
highway broker. **Click-to-win is refused:** office without rail is
ceremony; rail without office is unregulated flood. Both must be earned
by deeds.

---

## 1. Law: T0 remains the top

`12_UNION` §2a stands:

> Tier 0 creates the board everyone else moves on.

Therefore:

1. **T4–T9 are not above progenitors.** They extend the table **downward**
   into specialized economic and settlement functions of the union.
2. Energy and coherence still flow **up** toward T0; licensing and gauge
   keys still flow **down** from T0 (council-gated, shield-bounded).
3. T3 remains the **common absence of a specialized office** — the lawful
   biosphere default. Holding T4+ is **optional specialization**, never
   a soft-gate that strands T3 lines.

```
T0  Board-Makers        ← create / fold / license (TOP)
T1  Architect Observers ← corridor / lens
T2  Record Keepers      ← integrity / calendar
T3  Biosphere Common    ← thermodynamic foundation (default)
T4  Caravan             ← route and melt (deeper)
T5  Crucible            ← element refining
T6  Exchange            ← STAMP market-making
T7  Assay               ← element identity courts
T8  Vault               ← custody and collateral
T9  Anchor              ← external settlement seats
T10 Herald              ← diplomacy mesh / information   (+6 balance, 2026-08-08)
T11 Lattice             ← research / knowledge office
T12 Chorus              ← culture / literature
T13 Warden              ← defense / medium envelope
T14 Foundry             ← deep industrial (balances Crucible)
T15 Nexus               ← interstellar / union coordination (BOTTOM)
```

Seat widths: bounded offices **+25** capacity (25 more seats per tier).

---

## 2. T4–T9 — offices named

Stations are **offices of function** (`28_VOCABULARY` §10 spirit:
not free mint-spam). New codes join `STATIONS` only by landing (append-only).

| Tier | Station code | Voice name | Function (one sentence) | Earned by (sketch) |
|------|--------------|------------|-------------------------|---------------------|
| **T4** | `caravan` | **Caravan** | Maintains receipted **trade routes** (ROUTE-1 + element legs) and pays melt upkeep | Chartered lanes, repeated successful clears, unpaid route → lapse |
| **T5** | `crucible` | **Crucible** | Operates **elemental refineries** tuned to registered elements (14 §5) | At least one live refinery + refined-form mints; deepening held |
| **T6** | `exchange` | **Exchange** | Posts two-sided STAMP books (shares, notes, futures, perps) that NPCs clear | Market maker inventory + integrity books above threshold |
| **T7** | `assay` | **Assay** | Certifies element identity and assay fidelity (anti-fake-vein court) | Consilient assay receipts across peers; false assay → slash |
| **T8** | `vault` | **Vault** | Holds collateral and custody for instruments (not owner-equity) | Escrow volume + zero custody breach window |
| **T9** | `anchor` | **Anchor** | Bridges STAMP claims to **R3/R4** rails (strand / datum-isthmus) | Successful cross-rail settlements; NS-7 / isthmus exit proofs |

**Seat widths (Q-T1 default, coded Wave-1):** T4–T8 width **16**; T9
Anchor and T2 Sovereign **unbounded**. T0 stays 8 per lineage domain;
T1 16; T3 is absence.

**Ascend path (landed):** `ascend <sigil> to <station>` and conductor
`office_step` call `office_preconditions(station, OfficeEvidence)` —
facts of the chain (element kinds, assays, routes, market clears,
crossed). Failures name the cure. Seat width remains the registry door
(`NoSeatOpen`). One office per line; NPC only climbs from T3 (no seat).

**Wire:** station codes 5–10 on existing `StationSeated` (tag 89); no
new ClosureEvent for seating T4–T9.

---

## 3. R1–R4 — the Rail line (new ladder)

The R-line is **settlement altitude**: how far a **claim** (STAMP balance,
instrument, element shipment) may travel while remaining one truth under
SEAL B.

| Rail | Voice name | Reach | Settlement truth |
|------|------------|-------|------------------|
| **R1** | **Hearth Rail** | Single world chronicle | Replay of that world alone |
| **R2** | **Aperture Rail** | Cross-world, same netstratum mesh | Two-book aperture / treaty; pair conservation |
| **R3** | **Strand Rail** | Xylarium **strand** tollway | Strand wire + lith fold; domain mesh receipts |
| **R4** | **Court Rail** | **Isthmus** carrier + **datum** authority | POW++ multi-axial close; deed-priced space |

```
  world deeds ──R1──► aperture/treaty ──R2──► strand tollway ──R3──► isthmus/datum ──R4
     (local)           (netstratum)            (xylarium)              (superhighway)
```

### 3.1 Laws of rails

1. **Altitude is receipted on the instrument or shipment**, never inferred
   from the holder's T-tier.
2. **A claim cannot climb a rail without a deed** that the lower rail
   can replay. No silent promotion.
3. **Demotion is allowed** (R3→R2) with a typed residual; promotion is
   never free.
4. **R4 is not “more points.”** It is the only altitude at which
   *foreign* kernels can verify without importing netstratum. That is
   why isthmus stays path-dependency-free and datum stays the bench.
5. **NPCs must be able to live entirely on R1–R2** so the game is fun
   offline from strand/datum. R3–R4 are **expansions**, not gates to
   basic play.

### 3.2 Who posts on which rail

| Actor | Default rail | May open |
|-------|--------------|----------|
| T3 common colony | R1 | R2 after crossing |
| T4 Caravan | R2 | R3 with Anchor peer |
| T6 Exchange | R1–R2 books | R3 books if Anchor-linked |
| T9 Anchor | R3–R4 exits | never R0 (undefined) |

---

## 4. Coupling T × R (not click-to-win)

Earned combinations (illustrative; pins land with numbers):

| Want | Must hold | Must prove |
|------|-----------|------------|
| Element trade route | T4 *or* route deed without office | R2 shipment clears |
| Element refinery | T5 *or* tech+zone only (office optional) | R1 refined mint |
| STAMP spot market | T6 recommended | R1 order book + NPC clears |
| STAMP listed on strand | T9 Anchor + T6 book | R3 settlement receipt |
| Cross-kernel claim | T9 | R4 datum close |

**Office without deeds lapses** (same spirit as unpaid ROUTE-1 lanes).
**Deeds without office remain lawful** for T3 — the common is never
stripped of basic economy.

---

## 5. Registry and derivation changes (spec)

1. Extend `STATIONS` append-only:  
   `caravan | crucible | exchange | assay | vault | anchor`.
2. Extend `Tier` enum: `T4`…`T9` with exhaustive `tier_of`.
3. **R is not a station.** Store as:
   - on routes: `rail: u8` (1..=4) on charter receipts;
   - on STAMP instruments: `rail` field on mint/open;
   - reading: `rail_of(claim) -> R1..R4`.
4. Observer: two columns — `STATION/T#` and `RAIL/R#` (max open rail
   this line has settled in the window).

---

## 6. Explicit non-goals

- T4–T9 as a power ladder above T0.
- Paying real-world money per tick (CHARTER / 01_ECONOMY split stands).
- Forcing strand/datum online for local play.
- Renaming ideology **standing bonds** (09) — market debt instruments
  are **Notes** (see 32), not those books.

---

## 7. Open rulings (must not invent silently)

Recorded for `OPEN_QUESTIONS.md`:

- Q-T1: **coded default** — T4–T8 width 16; T9 unbounded (`2109bf0`).
- Q-T2: **coded default** — T5 advantage only; T3 may refine with zone
  (ElementRefinery gate is DEEPENING, not Crucible seat).
- Q-T1 (historical open): seat widths for T4–T9 per domain.
- Q-T2 (historical open): whether T5 Crucible is required to build refineries or only
  confers advantage.
- Q-R1: whether R3 requires a live strand process or may settle to a
  sealed strand receipt file offline.
- Q-R2: mint schedule for STAMP (crystal-only vs multi-collateral).
