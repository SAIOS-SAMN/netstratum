# CIVILIZATIONS — the substrate world-book (design stage)

**Status:** DESIGN / world-book — the founding roster of playable and
encounterable peoples. No faction mechanic ships before its protocol sections
and test vectors exist (CHARTER §1); tolerances, yields, and rates here are
**placeholder parameters**. This is the `docs/00_CHARTER.md` and `README.md`
"world-book (to come)", opened here.

**The design law (inherited from `05_THERMAL_BALANCE.md` §2):** there is no
universal home and no universal people, just as there is no "Goldilocks" zone.
Each civilization is bound to a *different thermodynamic survival strategy* and
to one substrate equilibrium band (05 §2). A faction is the niche of state,
energy, and entropy that would end the others.

**Register note (binding):** each people is written in **two registers** — a
**myth** (how the world tells its story) and a **physiology** (the hard-SF
substrate). Only two things about each are mechanically real and inscribed:
its **thermal band** and its **economic function** — both tie to the engine.
The physiology is design colour that fixes those two; the myth is flavour.
When a faction lands, its spec fixes band + function; it never becomes a
simulated ecology.

---

### 0.1 The clade tier (ruled 2026-07-30)

Between species and the axes stands the **clade** — the taxon line:
free text on the register card, register-only, never a mechanic and
never the station tier. Biological lines carry their clade (cetacean ·
cephalopod · hymenopteran · amphibian · teleost · avian · cnidarian ·
tardigrade · felid…); substrate lines carry the 12 §5a substrate as
clade (plasmoid · lithic · syzygial · endolithic · fungal ·
cryo-coherent). A divergence branch keeps its line's clade — one line,
one taxon; a confluent hybrid's clade is named at its wave. The tier
ALIGNS lines the axes cannot: Hivelight and the Klethaxu are both
hymenopteran; the register reads the kinship. **Gastropoda stands
open** — no snail line yet walks the board; the register awaits it.

## 1. The roster

Bands from `05_THERMAL_BALANCE.md` §2 (cryogenic 10–100 K · mesic 250–320 K ·
hyper-thermal 600–1500 K) or a cross-band deep regime (mantle, deep rock).

### 1.1 The Glasskeepers (Litho-Chalcogens) — the stone that remembers

- **Myth:** the oldest people, the memory of worlds — vast slow minds set in
  the crystal of mantles, having no bodies, only the lattice they are woven
  through.
- **Physiology:** silicon–sulfur crystalline networks bound in tectonic
  faults and mantle rock; under extreme stress they **vitrify**, freezing
  their structure into a glass phase that preserves stored state across
  geologic time and survives impact or stellar radiation.
- **Band / regime:** mantle / tectonic (cross-band — deep interior state, not
  surface `T_eq⁴`).
- **Function — long-horizon anchoring & deep storage.** The world's cold
  vault: empires carve chronicle segments or high-value contract states into
  the lattice so the record survives even if a sector's surface is lost.
- **Netstratum tie-in:** a durability layer beneath SEAL B — a redundant,
  physically hardened *copy* of a chronicle segment or anchored root (NS-7),
  never an alternate truth. Also the neutral-escrow substrate for treaties
  (see §3 diplomacy).

### 1.2 The Emberkin (Radiotrophs) — those who drink the star

- **Myth:** a people of the fire-orbits who walked into the light no other
  could bear and made a home of it — to the cold peoples, almost a rumour.
- **Physiology:** flux-harvesting lattice-organisms webbing star-facing hulls
  and asteroid faces; render high-energy stellar radiation into metabolic
  energy where visible-light metabolism cannot reach, and draw a hardened
  dormancy shell about themselves in darkness or vacuum until the flux
  returns.
- **Band / regime:** hyper-thermal / high-flux Furnace (05 §2.3, §3.1).
- **Function — Furnace infrastructure & the first refineries.** The galaxy's
  greatest gatherers of stellar fire, thriving where ordinary works burn away.
- **Netstratum tie-in:** the in-fiction source of the Furnace's cheap energy
  (05 §3.1). Harvest-grids around Emberkin holdings render stellar flux into
  exact-rational joules — the energy that flows cold-ward along the thermal trade
  loop (05 §4, 01 §6.1).

### 1.3 The Marrow (Endolith collective) — the slow shapers of worlds

- **Myth:** the patient power inside the rock, spoken of as a tide more than a
  people — a quiet will threaded through the deep stone, moving on the scale
  of ages.
- **Physiology:** a distributed sub-surface collective inhabiting the pore
  spaces of solid rock, signalling by ultra-slow chemical/electronic
  gradients; an eon-scale metabolism (a heartbeat of a century) that drops to
  a zero-energy dormancy under cold or drought and cannot be starved out.
- **Band / regime:** deep rock / moons (cross-band, buffered from surface
  flux).
- **Function — macro-engineering & terraforming.** Across long-horizon loops
  they rewrite a world from within: density, radiator headroom, thermal
  conductivity.
- **Netstratum tie-in:** the only lawful in-fiction way to *move a world
  between substrate bands* — by reshaping a planet's albedo/conductivity terms
  they drive its `T_eq⁴` toward a target band (05 §1.1). The slow counterpart
  to the fast surface-building loop; the power behind "adaptive colonization"
  (05 §4).

### 1.4 The Coherent (Cryo-Superconductors) — the still minds of the deep frost

- **Myth:** a people of pure order in the profound cold at the edge of the
  dark, where nothing moves and so everything can be *known*; a form of flow
  and stillness that holds only because the cold allows it.
- **Physiology:** macro-scale quantum-coherent superfluid (helium-4 /
  Bose–Einstein plasma) stabilized by persistent flux vortices; zero
  resistance and zero viscosity below a critical temperature `T_c`. Warmed to
  `T_eq ≥ T_c` they **dissociate** into inert gas; re-cooled below `T_c` they
  re-condense to the same superfluid matrix with no loss of stored state.
- **Band / regime:** deep Frost, `T_eq⁴ ≤ Threshold_super` (`T_eq → 0`), the
  cold tail below the cryogenic band (05 §2.1, §3.3).
- **Function — high-density proving & the reckoning-engines.** In the perfect
  cold their thought runs without friction; the canonical living reckoners of
  heavy proofs and validations — swift beyond any warm engine, undone by heat.
- **Netstratum tie-in:** the diegetic realization of `05_THERMAL_BALANCE.md`
  §7.5 — frost zones host proving/consensus compute, and the Coherent *are*
  that hardware: the living consensus nodes and ZK hosts (05 §7.3) of the cold
  sectors. Detailed faction spec pending inscription as 05 §8 (see §3 below and
  the boundary rulings). A thermal assault on a Frost sector is therefore an
  attack on the settlement layer itself — bounded by SEAL B (see §3).

---

## 2. Summary matrix

| People | Band / regime | Physiology (hard-SF) | Economic / in-game function |
|---|---|---|---|
| The Wanderers (reference, not a baseline) | Variable / borderlands | Generalist; dormancy through drought | First colonization; masters of no niche |
| Glasskeepers (Litho-Chalcogens) | Mantle / tectonic | Silicon–sulfur crystal; vitrification | Chronicle anchoring, deep storage, neutral escrow |
| Emberkin (Radiotrophs) | High-flux Furnace (hyper-thermal) | Flux-harvesting lattice; dormancy shell | Energy harvesting & primary joules production |
| Marrow (Endolith collective) | Deep rock / moons | Eon-scale sub-rock collective; dormancy | Sub-surface terraforming & `T_eq⁴` band shifting |
| Coherent (Cryo-Superconductors) | Deep Frost (`T_eq⁴ → 0`) | Superfluid / BEC; dissociates above `T_c` | High-speed proving & consensus reckoning-engines |

*(The Wanderers are the world's own generalist myth — "the people who live
everywhere and rule nowhere" — named as the reference point 05 §2 warns
against mistaking for a baseline, never as the measure of the others.)*

---

## 2a. The naming laws (operator rulings, 2026-07-30)

**SIGIL, never slug.** The registry's stable identifier is web jargon
inherited at G0; the ruling retires it in favor of the house's own
word, already waiting in NS-2: the **sigil** — a civilization's
immutable address on the mesh (`sigil:abyssal-vocalis`; NS-2's
sigil-address lookup becomes literal). The rename is a recorded
migration (JSON field + struct field, lens-side), queued like
Crucible→Furnace; surfaces adopt "sigil" immediately.

**Locality-anchored naming.** A civilization's name must carry its
PLACE, its MEDIUM, and its MANIFESTATION — never a vague vibe.
"Deep Chorus" infers sonar and says nothing about where or how; the
law replaces the register:

| founding name | the critique | locality-anchored successor (proposed) |
|---|---|---|
| Deep Chorus | vague sonar-ese | ~~Abyssal Vocalis~~ **Echowright** (operator-ruled 2026-07-30 — echo-maker, the -wright pattern; the sonar sense the operator asked for) |
| Hivelight | generic glow | **Vitreous Hex** — silica comb · quartz lattice · optical pulse computation |
| Slagborn | tells the diet, not the home | ~~Mantle Anhydro~~ **Slagborn stands** (operator-ruled 2026-07-30 — the proposal is declined; the diet-name keeps) |
| Eightfold | counts arms only | **Brine Cortex** — sub-ice ocean · saline convection · distributed arm-borne cognition |
| Mycelial Weft | close, keep | (already locality-anchored: understory · soil · hyphal network) |

The renames are ROSTER data (a wave edit + registry migration), not
code; divergence branch naming (the themed heads/tails) must follow
the same law — the head names the MEDIUM, the tail the MANIFESTATION,
and future word-tables grow toward locality. Applied at the next
roster wave; the old names archive with their chronicles (annex law —
nothing is renamed retroactively in a sealed record).

## 3. Open rulings for the landing that implements this

- **Band-shift is a chronicle mutation, not a free edit.** Marrow
  terraforming that changes a world's `T_eq⁴` emits NS-3 receipts per step and
  draws any uncertain outcome through the chronicle beacon (CHARTER §9); a band
  change is a high-value event and a candidate NS-7 anchor trigger (05 §7.2).
- **A thermal assault on Coherent sectors** couples the physics (05 §1.2
  `Heat_Net`) to the settlement layer's liveness — the rule must keep it a
  *raid, not an unrecoverable consensus kill*: SEAL B guarantees the chronicle
  survives, the reckoning *service* degrades and self-heals when the sector
  re-cools (matching the Coherent's physiology: dissociate above `T_c`,
  re-condense with no state loss below it).
- **Choke points must route around (01 §5 guardrail 6).** Compute/energy
  monopolies (radiator-headroom squeeze, siphon cartels) may pressure but must
  never be *absolute* — a squeezed sector always keeps a lawful, costlier path
  (minimal compute floor, alternate energy route). This is a hard guardrail,
  not flavour.
- **Glasskeeper anchoring & escrow** is redundancy *of* the chronicle, never
  an alternate ground truth — SEAL B stays absolute; the lattice holds copies
  and collateral, and CFE remains the sole arbiter of treaty state.
- **Registers stay separate.** A faction's spec fixes only its band and its
  economic function; physiology and myth are colour and never become simulated
  biology. Every faction lands under the BUILD_ORDER law: spec + mechanic +
  vectors in one landing, red-first, on operator go. This world-book is design
  authority, not a landing.
