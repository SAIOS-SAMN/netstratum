# THERMAL BALANCE — the master equation (design stage)

**Status:** DESIGN — no thermal mechanic ships before its protocol sections
and test vectors exist (CHARTER §1). All thresholds, constants, and yields in
this document are **placeholder parameters**, not inscribed law — they tune at
balance time. Inscribed here so the founding physics ruling is never lost.

**The founding insight (operator, 2026-07-26):** the single most important
unifying variable for the engine is the balance between heat and cold from the
star, driven by star density/size. Vacuum is an insulator: too close to a
dense star and a node cooks; too far and it freezes out its atmosphere and
locks down its infrastructure. Tying star luminosity directly to the heat/cold
balance makes every sector a thermodynamic puzzle where energy generation,
life support, and manufacturing are all bound by the same underlying equation.

---

## 1. The master equation

Every node (planet, orbital ring, station) at distance `r` from a star of
luminosity `L` has a baseline thermal state given by Stefan–Boltzmann
radiation balance:

```
            ⎛  L (1 − A)  ⎞ 1/4
    T_eq =  ⎜ ─────────── ⎟
            ⎝  16 π σ r²  ⎠
```

- `L` — stellar luminosity, a function of star density/size (mass class)
- `A` — the node's albedo (reflectivity), 0 ≤ A < 1
- `σ` — the Stefan–Boltzmann constant
- `r` — orbital distance from the star

```
                         THERMAL BALANCE SCALE

  EXTREME COLD                                               EXTREME HEAT
  [ Deep Freeze ] <─────── [ EQUILIBRIUM BAND ] ───────> [ Thermal Meltdown ]

  • Cryo-Brittle Metals       • 100% Efficiency          • Radiator Overload
  • High Heating Cost         • Minimal Life Support     • Shield Degradation
  • Atmosphere Freezes        • Optimal Human Capital    • Structural Distortion
```

### 1.1 The exact-ℚ ruling (RFA semantics — binding on the CFE)

The fourth root and the physical constants π, σ are irrational; the CFE
computes in exact rationals and refuses rounding drift (CHARTER §8). The
lawful formulation therefore never takes the root:

- **Work in flux space.** The canonical engine quantity is
  `T_eq⁴ = L(1−A) / (16πσr²)` — with `L`, `A`, `r` rational and the combined
  constant `K = 1/(16πσ)` inscribed as a single rational placeholder
  parameter, `T_eq⁴` is exact. `T_eq` itself is display-only.
- **Compare in fourth powers.** Every band threshold is stored as `T⁴`
  (e.g. the mesic band `[250 K, 320 K]` is inscribed as
  `[250⁴, 320⁴] = [3906250000, 10485760000]` in K⁴). Band membership is an
  exact rational comparison; no root is ever evaluated.
- Kelvin readouts shown to players are a client-side rendering; the chronicle
  and all receipts carry only the exact `T⁴` rationals.

### 1.2 The net-heat ledger

Where local industry adds heat on top of stellar flux:

```
    Heat_Net = Stellar Radiation + Factory Heat − Radiator Capacity
```

If `Heat_Net > 0`: manufacturing efficiency drops, structural integrity
decays, and habitats suffer catastrophic venting. All three terms are exact
rationals per tick; the ledger accrues like yield (no float, no drift).

---

## 2. Terminology ruling: no "Goldilocks"

The term "Goldilocks Zone" is **refused vocabulary** (in the spirit of
CHARTER §8). It is anthropocentric chauvinism: it assumes carbon-based,
liquid-water life at ≈300 K is the universal baseline, when there is no
baseline for lifeforms in the universe by population or density. In a world
where species, computational substrates, and industrial megastructures
operate across radically different thermal envelopes, there is no universal
paradise — only the alignment between a substrate's thermal requirements and
the stellar flux hitting its hull.

The lawful concept is the **Substrate-Specific Equilibrium Band**: every
entity — organic population, quantum compute cluster, high-energy foundry —
has its own optimal operating band `T_opt` and tolerance window `ΔT`
(inscribed, per §1.1, as fourth-power bounds).

```
                       SUBSTRATE THERMAL ENVELOPES

  CRYOGENIC BAND           MESIC BAND (Earth-type)        HYPER-THERMAL BAND
  [ 10 K – 100 K ]            [ 250 K – 320 K ]            [ 600 K – 1500 K ]
         │                            │                            │
  • Quantum Processors         • Carbon-Based Organics      • High-Flux Foundries
  • Superconductors            • Hydro-Biospheres           • Plasma Collectors
  • High-Density AI            • Human Capital              • Metal Synthesis
```

### 2.1 The cryogenic substrate (10 K – 100 K)

- **Occupants:** quantum compute cores, automated agent harnesses,
  superconducting power grids, liquid-gas processors.
- **Optimal placement:** deep frost zones, far from high-luminosity stars.
- **Friction:** near a dense star, thermal noise collapses qubit coherence
  and tanks processing output; warm-space operation demands heavy
  refrigeration work.

### 2.2 The mesic substrate (250 K – 320 K)

- **Occupants:** carbon-based biomass, traditional habitats, agricultural
  domes.
- **Optimal placement:** medium-luminosity orbits (what old Earth texts
  naively called "Goldilocks").
- **Friction:** highly sensitive to fluctuation — a ±30 K shift causes
  ecological collapse or massive life-support overhead.

### 2.3 The hyper-thermal substrate (600 K – 1500 K)

- **Occupants:** exotic silicon/lithic lifeforms, plasma refineries, Dyson
  energy harvesters, heavy alloy smelters.
- **Optimal placement:** inner-system crucible zones near dense Class O/B
  stars or white dwarfs.
- **Friction:** high structural stress, but zero heating overhead for
  high-activation-energy chemistry and nuclear work.

---

## 3. The three sector zones

Per star (density/size fixing `L`), the sector's coordinate grid breaks into
three operational regions. Note these are zones of the *grid*; which zone is
"good" depends entirely on the occupying substrate (§2).

### 3.1 The Crucible Zone (extreme heat)

- **Physics:** `T_eq⁴` above the thermal melting threshold — the star
  radiates massive photon energy and the vacuum cannot dissipate it.
- **PRO:** hyper-cheap solar energy (Dyson arrays at 300% yield —
  placeholder).
- **CON:** heavy thermal maintenance — continuous energy and coolant spend on
  radiator arrays and heat sinks, governed by the `Heat_Net` ledger (§1.2).

### 3.2 The Equilibrium Band (per-substrate)

- **Physics:** `T_eq⁴` inside the occupying substrate's band.
- **PRO:** near-zero heating/cooling overhead for that substrate; for the
  mesic band, populations, bio-domes, and education centers flourish.
- **CON:** prime real estate — minimal-maintenance zones become the most
  contested territory for trade hubs, civic cities, population centers.

### 3.3 The Absolute Frost Zone (extreme cold)

- **Physics:** `T_eq⁴ < 100⁴ K⁴` — far orbits, or low-density red-dwarf
  sectors.
- **PRO:** maximum radiator headroom — foundries, quantum processors, and
  high-energy weapons run at full overdrive without overheating.
- **CON:** zero ambient solar power; habitats and transit conduits burn
  thermal heating fuel to prevent embrittlement and atmosphere freeze-out.

---

## 4. How thermal balance drives trade and logistics

The single balance loop natively creates the supply-and-demand chain without
artificial mechanics:

```
   CRUCIBLE ZONE (HEAVY HEAT)                    FROST ZONE (EXTREME COLD)
   ┌───────────────────────────┐                 ┌───────────────────────────┐
   │ • Excess Energy & Plasma  │                 │ • High Radiator Headroom  │
   │ • Desperate for Coolants  │                 │ • Desperate for Energy    │
   └─────────────┬─────────────┘                 └─────────────┬─────────────┘
                 │                                             │
                 └───────────────> TRADE ROUTE <───────────────┘
                                   (Coolant / Energy Exchange)
```

- Hot sectors produce energy, but need coolants / radiator alloys.
- Cold sectors produce superconductors and heavy processing, but need
  energy / thermal fuel.
- **The player's role:** build the transit conduits and automated cargo
  fleets shuttling coolant hot-ward and energy/fuel cold-ward.

Because planetary value is relative to the occupying substrate, the loop also
produces non-zero-sum diplomacy:

```
   CRYOGENIC EMPIRE                                    MESIC / CARBON EMPIRE
   (Supercomputing AI)                                 (Human Population)
   ┌──────────────────────────────┐                    ┌──────────────────────────────┐
   │ • Outer Frozen Moon:         │                    │ • Outer Frozen Moon:         │
   │   PARADISE (0 Coolant Cost)  │                    │   WASTE (High Heat Cost)     │
   │ • Inner Crucible Planet:     │                    │ • Inner Crucible Planet:     │
   │   HELL (Thermal Noise)       │                    │   UNHABITABLE (Meltdowns)    │
   └──────────────┬───────────────┘                    └──────────────┬───────────────┘
                  │                                                   │
                  └───────────────> DIPLOMATIC TRADE <────────────────┘
                                  (Resource & Territory Swaps)
```

- **Non-zero-sum real estate:** an outer frozen moon worthless to a carbon
  civilization is prime zero-coolant territory for an AI compute hub.
- **Interspecies trade:** empires trade thermal entropy rights and
  substrate-suited territorial sectors, not just raw commodities.
- **Adaptive colonization:** materials/shielding research shifts a faction's
  thermal envelope, turning hostile "dead space" into industrial territory.

---

## 5. Mechanics summary (placeholder parameters)

| Variable | High star density/luminosity | Low star density/luminosity |
|---|---|---|
| Equilibrium radius (per band) | pushed far outward into the sector | pulled tight, close to the star surface |
| Energy yield | massive solar output | low solar (requires fusion/nuclear) |
| Primary resource need | coolants, radiators, thermal shields | heating fuel, energy cells, insulation |
| Industrial advantage | rapid solar harvesting | supercomputing, heavy weapons cooling |

Centering the math on thermal equilibrium gives every star type a distinct
physical personality, turns spatial placement into a high-stakes engineering
puzzle, and creates organic trade routes between thermal opposites.

---

## 6. Binding notes for the landing that implements this

- The mechanic lands under the BUILD_ORDER law: spec sections + test vectors
  in the same landing, red-first, on explicit operator go. This document is
  design authority, not a landing.
- `T_eq⁴`, `Heat_Net`, and band thresholds are exact rationals in the CFE
  (§1.1); any stochastic thermal event (flares, venting) draws through the
  chronicle beacon under CHARTER §9 with a domain tag (e.g.
  `"thermal.flare"`).
- Candidate protocol home: the sector/orb physics belongs with NS-1 DOP orb
  attributes (star `L`, node `A`, `r`) and NS-3 receipts for every thermal
  ledger mutation; the trade loop rides the existing economy design
  (`01_ECONOMY.md`).
