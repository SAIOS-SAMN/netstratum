# 32 — STAMP: unit of account, books, and self-sustaining markets

**Status:** partial land — operator directive 2026-08-07. **Wave-4
minimal:** orb STAMP (TLV 73), crystal→STAMP mint (111), apertures
(112–113), `stamp_market_step` green. **Phase-4 complete:** note · FUT ·
share · PERP (tags 120–131), SCHEMA_V9, `instrument_step`, desk EXCH,
shipwright `HullFinanced`. Establishes an **in-game currency** that is exact-rational,
receipted, NPC-sustained, and optionally tradeable on **strand / datum /
isthmus** at R3–R4 (`30_OFFICES_AND_RAILS`). Does not replace crystal
(PoUW mint medium) or closed carriers (physical law). Does not bill real
money per tick (`01_ECONOMY` / Charter split).

**Companion play surface:** interactive desk, fleets, alignment-priced
routes, and **localized** instrument books — `33_TRADE_FLEET_AND_DESK.md`
+ `archive/TASKS_TRADE_FLEET_DESK.md` (Phases 1–5). Wave 5 instruments here are
**consumed** by Phase 4 of that board (civ-filtered listings).

**Name:** **STAMP** — a unit of account whose truth is a **sealed
receipt chain**, not a mined commodity fantasy. Plural: stamps.
Symbol in prose: **S̸** (optional; plain `STAMP` in wire).

**Collision law:** Ideology **standing bonds** (09, tags 33–34) keep
the name **bond** in that domain. Market debt instruments are **Notes**.
Do not overload `BondSealed`.

---

## 0. Why STAMP exists

Without a unit of account:

- element trade collapses to barter (double coincidence);
- futures cannot clear;
- NPCs cannot run inventory finance;
- strand/datum have nothing isomorphic to settle.

With STAMP:

- prices are exact rationals `p = STAMP / unit_cargo`;
- instruments compose;
- R-rails know what they carry.

---

## 1. Unit laws

1. **Exact rational only** — numer/denom; no floats (Charter).
2. **Mint is a deed**, never a faucet. Initial sources (§2).
3. **Conservation in transfer** — STAMP moves by receipt; no silent
   inflation outside ruled mints.
4. **Dual with crystal:** crystal remains **PoUW work product**; STAMP
   is **account**. A ruled mint may exchange crystal→STAMP at a
   published schedule (placeholder curve), never free.
5. **Local play first:** R1 STAMP books work with conductor alone.
6. **External trade optional:** R3 strand / R4 court are exits, not
   admission tickets to fun.

---

## 2. Mint and burn (self-sustaining)

### 2.1 Lawful mint sources (v1)

| Source | Deed | Rationale |
|--------|------|-----------|
| **Work mint** | Verified discovery / corpus solve pays STAMP (or crystal then convert) | Ties money to useful work |
| **Collateral mint** | Vault (T8) issues STAMP Notes against locked elements/carriers | Credit, not free lunch |
| **Primary auction** | Exchange (T6) opens auction; winners pay elements/carriers | Price discovery at birth |

### 2.2 Lawful burns / sinks

| Sink | When |
|------|------|
| Route upkeep / melt settlements priced in STAMP | Routes stay costly |
| Instrument settlement fees (exact, small) | Books stay solvent |
| Demurrage on **unbacked** STAMP (optional ruling) | Idle money not free if unbacked |
| Slash on false assay / custody breach | Integrity couples to money |

### 2.3 NPC sustainability

Conductor **`stamp_market_step`** each beat (or year-voice):

1. Market makers (T6 or designated NPC policy) post bid/ask on
   liquid cargoes (ember, crystal, top elements).
2. Inventory constraints: cannot bid what they cannot pay.
3. Clear against resting offers (MARKET-1 shape).
4. If inventory stuck → widen spread or stop quoting (no infinite MM).

**No external player is required for clearing.** Players may compete.

---

## 3. Instruments (exhaustive market surface)

All instruments are **receipts** with: issuer, holder, notional,
collateral, rail (R1–R4), expiry or funding rule, and refuse-typed
lifecycle.

### 3.1 Share (equity)

**Share** — claim on a **production stream** (named: world+zone class,
or refined element output), not on “the civilization soul.”

- Dividend: exact fraction of YieldCollected / AssayStruck in window.
- Transferable on Exchange books.
- Dilution only by receipted new issue.

### 3.2 Note (debt) — not ideology bond

**Note** — promise to pay STAMP (or cargo) at maturity.

- Coupon optional (exact rational schedule).
- Default → collateral seize via Vault; integrity books post breach.
- Distinct wire tags from BondSealed (33–34).

### 3.3 Future

**Future** — obligation to deliver **cargo** (Resource or ElementId) for
STAMP at a fixed price on a fixed stamp.

- Initial margin in Vault.
- Mark-to-model is **refused** if model is float; mark against
  **last clear** (receipted) only.
- Delivery or cash-settle in STAMP at expiry by rule on the contract.

### 3.4 Perp (perpetual)

**Perp** — unending Future on an **index** (e.g. STAMP per unit crystal,
or per named element family basket).

- Funding rate: exact rational function of premium vs spot **last clear**
  (placeholder formula at landing; no continuous time).
- Liquidation: margin < maintenance → Vault auction.
- NPC perps allowed only on **published indices** with ≥N clears in window
  (anti-illiquidity trap).

### 3.5 What is already landed (compose, don’t fork)

| Landed | Role in STAMP world |
|--------|---------------------|
| MARKET-1 offers | Spot barter / STAMP spot |
| FORWARD-1 covenants | Proto-Note / delivery pact (map carefully) |
| BOND-1 standing streams | **Not** market Notes — keep separate |
| MERCHANT-1 caravan | Inventory risk on routes |
| Treaty war/peace | Gates which books may open |

---

## 4. Books and Exchange (T6)

An **Exchange** maintains:

| Book | Contents |
|------|----------|
| Spot | STAMP ↔ cargo resting orders |
| Share | equity issues and secondary |
| Note | primary + secondary debt |
| Future | dated delivery |
| Perp | perpetual indices |

Clearing rules:

- Price-time priority (MARKET-1).
- Exact conservation of STAMP and cargo.
- Failures refuse pure; no partial silent fills unless the contract
  says so on the receipt.

---

## 5. Cross-rail settlement (strand · datum · isthmus)

### 5.1 Mapping

| Rail | Where it settles | What a STAMP claim becomes |
|------|------------------|----------------------------|
| R1 | World chronicle | Local balances |
| R2 | Netstratum mesh / aperture | Two-world conserved books |
| R3 | **strand** | Strand wire instrument / fold-capable claim |
| R4 | **isthmus** frame + **datum** court | PoW++ envelope; multi-axial verify |

### 5.2 Exit flow (R2 → R3 → R4)

```
Exchange marks instrument rail-up
  → Vault locks collateral
  → Anchor (T9) emits exit receipt
  → strand accepts tollway message (path dep OK on strand side)
  → optional isthmus carrier (opaque frame)
  → datum verifies / rewards if claim is POW++ work
```

### 5.3 Import flow (R4 → R1)

Foreign close may **credit** STAMP only through Anchor import deed
with typed proof; no free external mint.

### 5.4 Independence

| Repo | Responsibility |
|------|----------------|
| **netstratum** | STAMP physics, books, NPC market, R1–R2 |
| **strand** | R3 tollway schema, fold safety |
| **isthmus** | R4 carrier (no kernel payload inspect) |
| **datum** | R4 authority, reward, multi-axial close |

Netstratum **must not** path-depend on strand/datum for compile of core
game (`isthmus` client pattern preferred for R4). Optional features:
`stamp-rail-strand` / `deep-rail-strand`, `stamp-rail-court` /
`deep-rail-court`. **X7 land (2026-08-08):** pure adapters live in
`cfe::xrail` (strand packet · isthmus frame · datum multi-axial verify);
Anchor export/import deeds tags 149–151. External xylarium/isthmus/datum
repos remain optional wire peers — not compile deps.

---

## 6. Conductor NPC loop (self-sustaining)

Per beat (sketch):

```
ground_step          # elements
build_step           # refineries, vaults
element_trade_step   # routes
stamp_market_step    # MM quotes + clears
instrument_step      # funding, expiry, liquidations
anchor_step          # optional R3/R4 exits when inventory warrants
```

Failure modes that keep the system honest:

- Empty book → no quotes (not infinite spread fantasy).
- Margin breach → liquidation auction.
- Route lapse → stranded inventory (merchant risk).
- False assay → Note collateral slash.

---

## 7. Player verbs (CLI sketch)

```
stamp balance
stamp offer <amt> STAMP for <amt> <cargo>
share issue | share buy | share sell
note issue | note buy
future open | future settle
perp open | perp close
vault deposit | vault withdraw
anchor exit <instrument> rail <3|4>
anchor import <proof>
```

All refuse with cure named.

---

## 8. Landings

| ID | Name | Delivers |
|----|------|----------|
| **STAMP-1** | Unit + balances | Mint sources, transfer, wire, pins |
| **BOOK-S** | Spot STAMP books | Extend MARKET-1 |
| **NOTE-1** | Notes | Issue, coupon, default, Vault |
| **SHARE-1** | Shares | Issue, dividend from receipts |
| **FUT-1** | Futures | Margin, last-clear mark, delivery |
| **PERP-1** | Perps | Funding, liquidation |
| **MM-1** | NPC market makers | stamp_market_step |
| **XRAIL-1** | Anchor exits | R3 strand adapter feature |
| **XRAIL-2** | Court exits | R4 isthmus/datum adapter feature |

---

## 9. Open rulings

- Q-S1: crystal→STAMP conversion schedule.
- Q-S2: demurrage on unbacked STAMP on/off.
- Q-S3: whether FORWARD-1 migrates into Note or stays parallel.
- Q-S4: minimum clears before a perp index lists.
