# Thermal balance — the equilibrium baseline (design stage)

**Status:** design — no thermal mechanic ships before its protocol sections
and test vectors exist (Charter §1). All thresholds, constants, and yields in
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

## 1. The equilibrium baseline

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

### 1.1 The exact-rational ruling (RFA semantics — binding on the CFE)

The fourth root and the physical constants π, σ are irrational; the CFE
computes in exact rationals and refuses rounding drift (Charter §8). The
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

**Carrying π exactly — the symbolic-π law.** Circular and spherical geometry
(orbital perimeters, the `4πr²` flux shell, a `πR²` collector cross-section)
need not approximate π at all. The CFE carries π as a symbolic transcendental
adjoined to the rationals: a state value is a formal pair `(a, b)` of exact
rationals denoting `a + bπ`, with addition componentwise and multiplication as
exact-rational polynomials in π (the symbolic-π extension). Consequences:

- **No approximation in `K`.** The `K = 1/(16πσ)` above need not be a *rational
  approximation* — its π part is carried exactly in the symbolic-π extension;
  only σ (a chosen physical constant) is a rational placeholder.
- **Geometry stays pristine.** No boxy Manhattan/cube boundaries, no spatial
  aliasing — continuous circles and spheres, exact.
- **π cancels in interaction ratios.** When a continuous emitter (flux
  `∝ 1/(4πr²)`) meets a continuous collector (area `πR²`), the π factors
  cancel and the delivered quantity lands back in pure exact rationals — e.g.
  absorbed power `= M c² R² / (4 r² Δt)`, π-free. Emitter↔collector state
  transitions settle to exact rationals with no residue.
- **The root is a separate matter.** A fourth root is not expressible in the
  symbolic-π extension either, so the `T_eq⁴` / fourth-power discipline above
  still stands. The symbolic-π extension removes the π approximation, not the
  root.

This is an engine-wide arithmetic law (all circular/spherical geometry), not a
thermal-only rule; it is stated here because the thermal equation is its first
consumer.

### 1.2 The net-heat ledger — entropic arm and syntropic arm

Where local industry adds heat on top of stellar flux:

```
    Heat_Net = Stellar Radiation + Factory Heat − Radiator Capacity
```

All terms are exact rationals per tick; the ledger accrues like yield (no
float, no drift).

**The linear (entropic-only) reading is refused (operator ruling
2026-07-27).** A design in which every excess joule vents to the void
discards the residual — against the house law that the residual is
structured information, measured and kept. The complete loop has two arms:

- **The syntropic arm — encasement (first):** excess heat is captured into
  ordered store before anything vents. Each tick,
  `encased = min(Heat_Net, encasement throughput)` accrues as **joules** —
  the commodity money of `01_ECONOMY.md` §6.1, whose supply law ("governed
  strictly by sector thermal balance") this mechanic makes literal.
  Encasement is provided by encasement shells (a zone kind), the harvest
  architecture of §3.1 at colony scale.
- **The entropic arm — vent (only the overflow):**
  `overflow = Heat_Net − encased`. Only when `overflow > 0` does the sector
  overheat: every yield that tick degrades by the exact factor
  `1 / (1 + overflow)`, and the transition chronicles (OverheatStruck on
  entering, OverheatCleared on leaving — the famine strike/relief shape).
  The void is the failure mode, never the design.

**The cryo law — the Frost is the bank:** accrued (uncollected) joules
*leak* `1/8` per tick, floored at zero (the D3 spoilage precedent) — except
where `T_eq⁴ ≤ 100⁴ K⁴` (the cryogenic band and colder), where the store
holds **lossless**: the coherence-vault physics of the Coherent
(`06` §1.4). Banked (collected, receipted) joules are canonical everywhere.
Consequence: the Furnace **mints**, the Frost **banks**, and the thermal
trade loop (§4) closes into a circuit — mint → encase → ship cold-ward →
vault — with entropy-to-void the only true loss. This two-armed closure is
the **syntropic encasement**; cryo is its pole, not the cold end of a line.

### 1.3 Extreme stellar sources — the Luminal White Hole

The equilibrium baseline's `L` term admits extreme sources. A **Luminal White
Hole** is the design's ultimate stellar engine: the time-reverse of a black
hole (the Schwarzschild metric under `t → −t`, or outflowing
Eddington–Finkelstein coordinates), an un-enterable spherical boundary at the
gravitational radius `r_g = 2GM/c²` that continuously ejects energy outward at
`c`.

- **Impermeable boundary:** for `r ≤ r_g` all radial null/timelike geodesics
  point strictly outward — nothing enters. In-game: an impenetrable spatial
  barrier emitting maximum photon density; sectors build orbital collection
  rings *around* it, never on it.
- **Outward flux:** at a collector radius `r > r_g`,
  `Φ(r) = E_ejection / (4πr²)`, with `E = M c²` over ejection interval `Δt`.
  The π sits in the shell area and, per §1.1, is carried exactly and cancels
  against a `πR²` collector — delivered power `P = M c² R² / (4 r² Δt)`, an
  exact rational.
- **Role:** a hyper-dense energy anchor at the extreme of the Furnace regime
  (§3.1) — it pushes the equilibrium radius far outward, makes the surrounding
  sectors the richest joule sources in the trade loop (§4), and its
  gravitational/thermal push forces high-efficiency ring architecture around
  it. `M`, `Δt`, `r_g` are placeholder parameters that tune at balance time.

*(A design energy-source, not an astrophysical claim about real white-hole
stability.)*

### 1.4 The sink dual — exporting cold, and the earned syntropic state

**(Landed 2026-07-27 as VENT-1/ECON-J/SINK-1 — except the research and
throttle gates, which await the tech-tree landing; the infrastructure gate
is live.)**

**Exporting cold is exporting absorption capacity.** A dim-star system
(`T_eq⁴ → 0`) holds the rarest resource in a high-flux universe: radiator
headroom. It does not ship a negative substance — it sells **Banked Sink**
(`S_cool`), the capacity to absorb heat, measured in the same joule-
commensurable units as the energy side. Two export mechanisms:

- **Physical:** cryogenic coolant cycles out; entropic (heated) fluid
  returns; the cold system sheds the imported heat to the void through its
  deep-space arrays and re-arms the coolant.
- **Protocol:** radiator-headroom allocations — the thermal entropy rights
  already inscribed (01 §6.2) — leased to Furnace sectors.

**The symmetric ledger (the VENT-1 sheet, both sides):**

| surplus side (heat) | deficit side (cold) |
|---|---|
| banked joules (captured syntropy) | banked sink (captured headroom) |
| vented entropy (accumulated loss) | absorbed heat (imported waste) |

**Conservation invariants (exact, witnessed at landing):** hot side:
`encased + vented = total excess` (§1.2). Cold side: `heat_absorbed ≤
banked_sink`, and `sink_consumed = heat_absorbed = vented_after_shed` —
absorbing a neighbour's waste consumes sink and, once shed to the void,
settles into the absorber's `entropy_vented` ledger. No fuzzy net: each
side balances in its own books.

**The earned-state law (operator ruling, 2026-07-27):** neither side is an
environmental passive. Raw flux and raw void are worthless until a sector
passes three gates, symmetric for energy and sink:

1. **Research gate** — the capture capability unlocks (energy path and sink
   path are separate unlocks). *Dependency: the research/milestone system —
   the D3 milestone ladder is the current carrier; a full tech tree is its
   own landing.* Ungated sectors lose 100% of excess to entropy.
2. **Infrastructure gate** — the hard cap: structures (encasement shells /
   cryo-lattice arrays) set max throughput per tick, exactly.
3. **Throttle gate** — a policy coefficient `α ∈ [0,1]`, an exact rational
   set per sector: `captured = α × raw excess`. The uncaptured `(1−α)`
   accumulates into the vented ledger and settles as `VentSettled` at
   collect — never un-receipted.

**Symmetric decay (the §1.2 leak law, both sides):** accrued stores leak
outside their native band — joules leak when warm, sink leaks when
irradiated without insulation. Banked stores are canonical and never leak
(landed law); vaulting either side in a Glasskeeper escrow node (01 §7)
holds it lossless long-term.

**The trade equilibrium:** the price ratio `S_cool / J` is the natural
exchange rate of the Furnace↔Frost axis — energy cheap where sink is
priceless, sink abundant where energy is scarce (ECON-J heating upkeep).
Cold systems import joules to live; in exchange they absorb the thermal
death of the Furnace. Placeholder pricing; tunes at balance.

---

## 2. Terminology ruling: no "Goldilocks"

The term "Goldilocks Zone" is **refused vocabulary** (in the spirit of
Charter §8). It is anthropocentric chauvinism: it assumes carbon-based,
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
- **Optimal placement:** inner-system furnace zones near dense Class O/B
  stars or white dwarfs.
- **Friction:** high structural stress, but zero heating overhead for
  high-activation-energy chemistry and nuclear work.

---

## 3. The three sector zones

Per star (density/size fixing `L`), the sector's coordinate grid breaks into
three operational regions. Note these are zones of the *grid*; which zone is
"good" depends entirely on the occupying substrate (§2).

### 3.1 The Furnace Zone (extreme heat)

- **Physics:** `T_eq⁴` above the thermal melting threshold — the star
  radiates massive photon energy and the vacuum cannot dissipate it.
- **Pro:** hyper-cheap solar energy (Dyson arrays at 300% yield —
  placeholder).
- **Con:** heavy thermal maintenance — continuous energy and coolant spend on
  radiator arrays and heat sinks, governed by the `Heat_Net` ledger (§1.2).

### 3.2 The Equilibrium Band (per-substrate)

- **Physics:** `T_eq⁴` inside the occupying substrate's band.
- **Pro:** near-zero heating/cooling overhead for that substrate; for the
  mesic band, populations, bio-domes, and education centers flourish.
- **Con:** prime real estate — minimal-maintenance zones become the most
  contested territory for trade hubs, civic cities, population centers.

### 3.3 The Absolute Frost Zone (extreme cold)

- **Physics:** `T_eq⁴ < 100⁴ K⁴` — far orbits, or low-density red-dwarf
  sectors.
- **Pro:** maximum radiator headroom — foundries, quantum processors, and
  high-energy weapons run at full overdrive without overheating.
- **Con:** zero ambient solar power; habitats and transit conduits burn
  thermal heating fuel to prevent embrittlement and atmosphere freeze-out.

---

## 4. How thermal balance drives trade and logistics

The single balance loop natively creates the supply-and-demand chain without
artificial mechanics:

```
   FURNACE ZONE (HEAVY HEAT)                    FROST ZONE (EXTREME COLD)
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
   │ • Inner Furnace Planet:     │                    │ • Inner Furnace Planet:     │
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
  chronicle beacon under Charter §9 with a domain tag (e.g.
  `"thermal.flare"`).
- Candidate protocol home: the sector/orb physics belongs with NS-1 DOP orb
  attributes (star `L`, node `A`, `r`) and NS-3 receipts for every thermal
  ledger mutation; the trade loop rides the existing economy design
  (`01_ECONOMY.md`).

---

## 7. The crypto ruling (thermal value at the settlement boundary)

**The boundary is the charter's, restated:** game logic never touches
contract bytecode; chains settle, never simulate (Charter §7). Crypto cannot
enter the equation — it settles the *value* the equation creates. Three
layers, world to chain:

### 7.1 Energy is the in-world currency — chronicle-native, never a token

`T_eq⁴` gives every sector an exact, replayable marginal cost of energy:
near-free in the Furnace, expensive in the Frost. Energy (joules, exact
rationals in the chronicle) is therefore the commodity money of the world —
its supply set by physics, not by a mint. The hot/cold price differential is
the monetary system: coolant flowing hot-ward and energy cold-ward is a
currency exchange between thermal regimes. This layer lives entirely in the
CFE — free, high-frequency, chain-free, no wallet at onboarding
(Charter §3.5).

**The hard line:** no in-game unit is ever a per-unit crypto asset. Energy
stays chronicle-native and free to move; anything tighter re-creates Dark
Forest's every-tick-on-chain mistake.

### 7.2 Thermal scarcity defines the NS-7 high-value triggers

The thermal equation supplies NS-7 §1's (unwritten) high-value trigger
conditions with physical definitions:

- **Rare-world minting** (NS-7's named first consumer): rarity is a world
  whose `T_eq⁴` sits inside a substrate's equilibrium band — a mesic-band
  world near a dense star, or a frost moon with radiator headroom adjacent
  to a furnace energy source, is provably scarce by the equation. That deed
  anchors as an ERC-721.
- **Thermal entropy rights** (§4): the transferable right to dump heat into
  a sector's radiator headroom or draw from its flux — an ERC-1155 class,
  settled through the adapter when it changes hands between empires.
- **Macro-trades:** a long-lived coolant/energy route contract between a
  furnace and a frost sector is the macro-trade NS-7 anticipates.

### 7.3 The adapter maps value, never physics (SEAL C)

On-chain token metadata never carries game rules, but it may carry the exact
`T_eq⁴` rationals and band-membership facts from the anchored chronicle
root, so an external buyer verifies what they own (which band, which
substrate it suits) from the root alone. The chain sees a checkpoint
containing thermal facts; it never computes them. If settlement ever needs
ZK proofs of band membership, "`T_eq⁴` ∈ `[250⁴, 320⁴]`" is a cheap
rational-comparison circuit — the no-float law (Charter §3.2) pays off
directly at the crypto boundary.

**Succinct settlement proofs (the buildable core).** The goal at the NS-7
boundary is to prove a fact about the anchored chronicle root — a band
membership, a delivered-quota inequality `A/B ≥ C/D`, a closed energy loop —
*without replaying every tick* that produced it. The lawful tool is a real
succinct proof system over a finite field: a stark or a multilinear /
sum-check prover (Lasso, Jolt, HyperPlonk) whose statements are exact-rational
inequalities. This is a genuine win precisely because the state is already
exact rational (Charter §3.2) — the arithmetization is clean and the
verifier stays constant-size.

Two guardrails keep this honest, learned from a rejected "topological /
holographic" proposal (see `docs/OPEN_QUESTIONS.md` Q1):

- **A proof, not a boundary check.** Matching neighbouring sectors on their
  shared boundary gives *consistency*, not integrity — a node can compute a
  false interior while keeping its edges clean. Settlement integrity requires
  an actual succinct proof of the interior transition, never boundary-stitch
  agreement alone.
- **Structure is not history.** A homological / connectivity invariant can
  prove "a route exists" or "a loop is closed", but it cannot prove "≥ N
  joules were delivered by tick T". Quota and delivery facts settle by exact
  rational inequality inside the proof, not by a topological invariant.

The chain still never computes physics (§7.3); it verifies one succinct proof
derived from the chronicle root and acts on the verdict.

### 7.4 The chain ruling: existing chain, never a new one

No new chain is spun up for settlement — inscribed here as a ruling, per
Charter §7's definition of WWW3 as *existing* EVM/UTXO chains:

- **The mesh is the sovereign ledger.** The NS-3 chronicle is append-only
  ground truth (SEAL B); NS-5 proof-of-convergence is the consensus. A
  custom L1 would duplicate that machinery while adding validator
  bootstrapping, security-budget, and liquidity-from-zero problems the
  dual-anchoring design exists to avoid.
- **Settlement rents what only existing chains have:** ownership that
  survives any single node, and real market depth for deeds and
  macro-trades. A fresh chain provides neither — anchoring to it is a
  checkpoint into an empty room.
- **SEAL C keeps the choice cheap:** adapters retarget; game state never
  moves. At landing N7 the concrete pick is a cheap, stable EVM chain — an
  established L2 fits the cadence (periodic single-root posts wanting
  negligible gas and high uptime, not L1 prestige) — verified first against
  a local testnet (NS-7 §3).

**The mesh is the world; an existing chain is the notary.**

### 7.5 The closed loop (world-book note)

Frost zones are canonically where high-density compute lives (§2.1) — so the
in-fiction home of proving and consensus infrastructure is the cold sectors.
The world's physics explains its own settlement layer.
