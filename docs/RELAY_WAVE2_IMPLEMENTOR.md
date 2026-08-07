# RELAY — TO THE WAVE-2 IMPLEMENTOR NODE

> **LANDED Wave-2 (2026-08-07):** E-APT-1 tags 108–109, kernel **0.21.0**,
> `element_trade_step`, harness route clear green.
> **LANDED Wave-3/4 (2026-08-07):** ElementRouteChartered 110, SCHEMA_V6,
> STAMP 111–113 + orb tag 73, kernel **0.22.0**, `stamp_market_step`,
> O6.1/O6.2, harness stamp spot green. Standing law remains authority
> for instruments / R3–R4 / observer STAMP.

You continue the vertical slice after **Wave-1 `2109bf0`**. Read
`RELAY_WAVE1_IMPLEMENTOR.md` for standing law (registers, SEAL B,
ratchet, green-board trap). This brief is **only what changed** and
**what you build next**.

**Working directory:** `/opt/netstratum`.  
**Lane:** netstratum play surface (not datum/isthmus court).  
**Do not** unilaterally edit dirty `tools/observer.py` without
reconciling; T1.5 / R2.6 / E3.8 touch that surface.

---

## 1. WHAT WAVE-1 LEFT YOU

| Item | Value |
|------|--------|
| Kernel | `0.20.0` — next free minor **21** |
| `ClosureEvent` tag | next free **108** |
| Codec schema | still v5 path unless you append state fields |
| ZoneKind compiled | 1–9 (ElementRefinery = 9); minted > 9 |
| Stations | 10 words; wire 1–10; `RING_ARITIES` includes 10 |
| Done-bar harness | `engine/conductor/tests/wave1_era_pins.rs` |
| Green under pure NPC | elements (ground), T4+T6 seats, **refine** |
| Still **ignored** red | element route clear; STAMP spot clear |

**Wave-1 did not land:** R1–R4 wire, element aperture, element routes,
STAMP, office *advantages* (O6), observer T-column.

---

## 2. WHAT YOU ARE BUILDING (Wave-2)

**Slice:** **E-APT-1 → E-ROUTE-1 → `element_trade_step` (E3.3)**  
plus optional **R2.1–R2.2** (rail type + default R1) if it unblocks
without dragging strand.

**Done bar (same harness):** turn
`pure_npc_element_route_clears` **green** — a pure-NPC era charters or
fills an element route and a clear stands, zero player. Keep STAMP
ignored until Wave-4.

**Order:**

1. **Claim registers** — write ledger first:
   - tags for aperture-element legs / route-element cargo (likely
     **108+**; claim in `dop.rs` header before code);
   - kernel minor only if admits gains/refuses new cases (**21**).
2. **Un-ignore or extend harness** for route clear — still red until
   green.
3. **E-APT-1** — element id on aperture value; conservation exact;
   SEAL B; no demurrage on elements by default (31 §1).
4. **E-ROUTE-1** — charter / upkeep / lapse specialized for ElementId
   (ROUTE-1 topology).
5. **E3.3** — conductor `element_trade_step` NPC pure clear.
6. Full board green + clippy + ratchet; **prove EXIT codes** (no
   timeout-as-green).

**Out of Wave-2:** STAMP, perps, R3/R4 adapters, O6 office advantages
(can land after routes exist).

---

## 3. SHARED REGISTERS (as of Wave-1 land)

| Register | Ledger site | Next free |
|----------|-------------|-----------|
| Kernel minor | `engine/kernel/tests/conservation_pins.rs` | **21** |
| ClosureEvent tag | `engine/cfe/src/dop.rs` header | **108** |
| Codec schema | `engine/codec` `SCHEMA_V*` | **v6** if you grow state |
| ZoneKind wire | compiled 1–9; **minted > 9** | — |

Minor vs patch: minor when the gate *admits or refuses* something new.

---

## 4. LAWS THAT STILL BITE (short list)

- **Call-not-copy** for any rate/recipe (Wave-1: `refine_recipe`).
- **Receipt authority** for fold-dependent facts.
- **Additive-trailing** wire only.
- **Ratchet ceiling 112** — no new undervied `ratio(a,b)` dials.
- **Opaque work payloads** for any mint path.
- **Verify-time search prohibited** — complete path on request.
- **Industry vs refine:** refinery must not raid smelter iron gate or
  biomass ration (Wave-1 measured traps).

---

## 5. CODE MAP (start here)

| Concern | Path |
|---------|------|
| Aperture legs | `engine/cfe/src/world/` aperture / economy; `dop` tags |
| Routes | ROUTE-1 charter/lapse; extend for ElementId |
| Conductor cycle | `build → ground → office → refine → tick → collect` — insert
  trade step without breaking ground/refine pins |
| Harness | `engine/conductor/tests/wave1_era_pins.rs` |
| Authority docs | `31_ELEMENT_COMMERCE.md` §2; `TASKS_TIER_RAIL_MARKET.md` E3.* |

---

## 6. VERIFICATION BAR

Same as Wave-1 brief §6: full board finished (not timeout), clippy zero,
ratchet green, `replay == committed` on new arcs, CLI/scripted play.

Known-slow: `conductor` `mint_loop_pins` (~26 min) — do not short-timeout
and call green.

---

## 7. TERRITORY

- **`tools/observer.py` / elements uplink:** other lane may hold dirty
  work — reconcile before T1.5 / E3.8.
- **`#38(b)` conductor corpus:** d3-scope — do not touch unilaterally.
- **Datum/isthmus court, `Act::Anchor`, Υ estates:** separate lane
  (`/opt/datum`, `/opt/isthmus`); Wave-2 stays netstratum R1–R2 play
  unless you only *claim* feature-gated adapters without path deps.

---

## 8. STARTING STATE

`main` @ **`2109bf0`** (Wave-1). Re-measure board yourself before building;
do not inherit a green figure on trust.
