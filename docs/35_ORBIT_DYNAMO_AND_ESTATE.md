# 35 — Orbit dynamo · estate real estate · exploration economics

**Status:** landing — orbit periods on main (`500be46`); estate ladder
extended for orbital + stratum holdings (this session).  
**Authority:** `05_THERMAL_BALANCE`, `45_ALMANAC` (orbit grain),
`24_FORM` §2j (magnets / field, not collision), `29_CONSTRUCTION` C1
(slot type), `33_TRADE_FLEET_AND_DESK` §3, E5b estate chain (tags
97–100), `ENDOWMENTS`.  
**Does not** invent float ephemeris, undervied dials, or a second melt
formula. `D_eff` remains the one transit distance producer.

---

## 0. Why this file exists

Phase-2 alignment advanced every world's θ as `floor(now / 60)`. That
made **all orbits the same clockwork** — relative phase froze after
siting, and the desk's CHEAP/FAIR/DEAR band was static drama.

The operator ruling (session): *all orbits should not be the same; take
magnetism of cores, offset by polity of the star and class of the star.*

Separately, the estate chain (survey → appraise → convey) valued
**buildings and banked matter** but not **place** (sited rings under a
dynamo) nor **exploration strata** (prospected veins). Exploration and
expansion economics were half-visible.

This document seals both: **differential orbit law** and **real-estate
ladder covering every claimable place the engine already admits**.

---

## 1. Orbit period law (dynamo · core · polity)

### 1.1 Formula (exact, derived)

```text
period_ticks(world, site) = max(1,
    ORBIT_PHASE_TICKS
  × core_magnetism(site)
  × star_polity_offset(world)
  / StarClass::magnetism(star)
)

θ_steps = floor(now / period_ticks)
θ_live  = Slot::from_index(θ_base.index() + θ_steps)   // wrap on TURN_SLOTS
```

| Symbol | Source | Meaning |
|--------|--------|---------|
| `ORBIT_PHASE_TICKS` | 60 | Year-voice / journal ring — already landed |
| `core_magnetism` | grain steps of `orbit_r` on `ORBIT_GRAIN` (64) | Ring depth — farther cores couple differently |
| `star_polity_offset` | sealed genesis `disposition` (1..3), else 1 | Polity character under this star |
| `StarClass::magnetism` | dynamo ladder (O=2 … M=12, …) | Stellar field grade — same table as **ring reach** |

**Stronger main-sequence dynamos (M > G > O) → shorter periods** at
equal ring. **Farther rings → longer periods.** **Epicurean disposition
(3) stretches; expansionist (1) is the tight baseline.**

API: `cfe::world::{core_magnetism, star_polity_offset, orbit_phase_period}`.

### 1.2 What this does to trade

- `d_orbit(a,b)` phases **each** end on its own period.
- Equal wall-clock advance of G and M at the same ring **desynchronizes**
  `D_orbit` (pinned).
- `route_melt` / `alignment_of` inherit the law for free (call-not-copy).
- ψ stays fixed at siting (inclination precession = open Q-O1).

### 1.3 Ring reach (already landed, restated)

```text
rings_anchored ≤ StarClass::ring_reach() == magnetism()
```

A **ring** is a distinct grained `orbit_r`. Slots within a ring are the
turn law (θ, ψ). Magnetism bounds how many rings exist; period law
bounds how those rings *move*.

### 1.4 Non-goals

- Continuous Kepler roots (`T² ∝ a³` needs irrational / barred squares
  as aggregators — period uses grain lattice identity instead)
- Soft float angles
- A second melt formula

---

## 2. Estate real-estate ladder (exploration · expansion)

### 2.1 Holdings (appraisable subjects)

One wire scale byte; orb + tile fill as needed:

| Scale | Holding | What it prices | Exploration role |
|-------|---------|----------------|------------------|
| 1 | **Estate** | Σ planetary × parcel monetary | Whole polity book |
| 2 | **Planetary** | banks + zone sealed costs | Colony capital |
| 3 | **Building** | one zone's sealed cost | Tile industrial real estate |
| 4 | **Orbital** | planetary + **site premium** (must be sited) | **Place under the dynamo** |
| 5 | **Stratum** | prospected vein richness (tile = stratum) | **Exploration claim** |

Site premium (exact):

```text
site_premium =
    (core_magnetism / ORBIT_GRAIN)
  × StarClass::magnetism(star)     // 1 if no star (should not site)
  × commensurate_weight(Iron)      // stakes driven into place
```

Unsited orbs **cannot** appraise as Orbital (`None` — absence is not
zero). Unprospected strata cannot appraise as Stratum.

### 2.2 Estate chain (deeds — already tags 97–100)

```text
EstateSurveyed (97)  →  parcels derive (free/monetary)
AppraisalStruck (98) →  value at any Holding scale
OfferCountered (99)  →  renegotiate on stale appraisal
ParcelConveyed (100) →  monetary portion moves orb→orb
```

**Meter:** iron × volume of orbs surveyed. **Witness:** founding /
chart receipt per orb.

### 2.3 Endowments claimed by this land

| Endowment | Standing | Claim path |
|-----------|----------|------------|
| Differential θ | Law asked for; was lockstep | `orbit_phase_period` |
| Dynamo table dual use | magnetism already priced rings | period + ring reach |
| Sited place as capital | PlanetSited existed; unappraised | Holding::Orbital |
| Prospected stratum capital | ProspectStruck existed; unappraised | Holding::Stratum |
| Desk/period readout | readings orphaned | status / thermal / help |

### 2.4 Open vents (recorded)

| ID | Vent | Status |
|----|------|--------|
| Q-O1 | ψ precession from core dipole | open |
| Q-O2 | Hull / route as Holding scales 6–7 | open (fleet capital already receipted) |
| Q-O3 | Lease of orbital slots (01_ECONOMY § land tenure) | open — convey is inter-orb, not landlord–tenant |
| Q-O4 | Multi-star chart distance × local period | open — D_chart still L1 chart only |
| Q-E1 | Auto NPC survey on expansionist build_step | optional later |

---

## 3. Player surface

| Verb | Deed / reading |
|------|----------------|
| `star` / `site` | commission + site (ring under magnetism) |
| `status` / `thermal` | thermal balance **and** orbit period if sited |
| `estate survey` | EstateSurveyed |
| `estate parcels` | derived free/monetary |
| `appraise estate\|planet\|building\|orbital\|stratum` | AppraisalStruck |
| `convey` | ParcelConveyed (monetary) |
| desk trade | D_orbit under differential periods |

---

## 4. Registers

- Kernel: minor bump if appraisal gate grows subject coverage (0.25.0).
- Tags: no new tag if Holding scales expand inside AppraisalStruck.
- Next free deed tag remains **120** until a new ClosureEvent lands.
- Pins: `desk_pins` orbit periods; `estate_survey_pins` + orbital/stratum appraisal pins.

---

## 5. Explicit non-goals

- Float orbits or continuous funding rates
- Replacing I1 conservation with appraisal numbers
- Free-minting parcels without survey meter
- Combat / interdiction of place (Phase 5)

---

*Companion: `33` §3, `34` matter loop, `ENDOWMENTS`, `TASKS_TRADE_FLEET_DESK`.*
