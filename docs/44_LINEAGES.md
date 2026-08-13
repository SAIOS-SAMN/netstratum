# 44_LINEAGES — THE CIVILIZATION-DESIGN FRAMEWORK

*Translated from operator directives 2026-08-13 (the named lineages). The
thesis: a civilization is not a preset picked from a short list — it is a
COORDINATE in an open, combinatorial axis space, given depth by four
dimensions, two of which the engine already realizes and two of which are
the build frontier. The flavor the operator writes (evolutionary arcs,
planet-fit, environment-coupled demographics, castes) maps onto engine
mechanics; where it does not yet, this doc names the mechanic owed.*

## 0. Why "three" was never the number

The provisioning layer (L4) keyed on the DISPOSITION axis alone — three
seeds — and so read as three archetypes. A LINE is not its disposition. It
is the cross-product

    disposition × metabolism × perception × class × clade

and every axis is OPEN: a value not in the seed table is MINTED through the
vocabulary/novelty gate (`28_VOCABULARY`), judged like any other. The
seeded space alone is 3 × 3 × 4 × 14 × 20+ — thousands of distinct lines —
before a single mint. The nine named lineages of the first directives each
land on an EXISTING seeded coordinate (witnessed in `lineage_pins.rs`); the
richer civilizations of the third exercise the DEPTH DIMENSIONS below.

## 1. THE COORDINATE (what a line IS)

| axis | seeds | open? | what it fixes |
|---|---|---|---|
| disposition | expansionist · empiricist · epicurean | mint | provisioning appetite, research siphon (§L4) |
| metabolism | mineral · biotic · plasmic · (minted element) | mint | the ration & leisure good (`metabolic_ration`/`_leisure`) |
| perception | planar · volumetric · harmonic · umbral | mint | sensing/cognition coupling (frontier §2d) |
| class | 14 archetypes (biotic-swarm … mirror-form) | mint | LIFESPAN (`lifespan_of`), body-scale (frontier) |
| clade | 20+ legacy + open expansion | mint | HOME BAND (`clade_preferred_region`), habitability |

## 2. THE FOUR DEPTH DIMENSIONS

### 2a. PLANET-FIT — a species matched to its world  ·  **EXISTS**

Every clade has a preferred thermal band (`clade_preferred_region`), and a
world's fitness for it is `planet_fit(clade, region) → Prime · Marginal ·
Hostile`, with `habitability(clade, region)` the life-bearing factor that
tapers with distance from the home band. A lithovore on cold rock, a
plasmoid on a scorching close-orbit world, an abyssal line in a deep
pressured band — each reads its own fit. **This is the "high-gravity super-
earth / ice-crusted ocean / gas giant / volcanic world" matching the
operator names, already in code.** What is NOT yet built: the player
CREATING a fitted world for a founding — that is the progenitor-gated
star/planet creation, `TASKS_ALPHA_BETA` Phase 4 (#34).

### 2b. ENVIRONMENT-COUPLED DEMOGRAPHICS — population tracks the world  ·  **EXISTS**

"Population numbers dynamically swell or contract to match thermal vent
capacity / cooling cycles / jet streams" is exactly the landed law: the
carrying capacity `K_pop = carrying_capacity(footprint, clade, region)` is
the environment's own ceiling, and the logistic population (#40–42) chases
it — growing when the world can bear more, culling when it cannot. A hostile
world holds few; a prime one many; a barren rock none (pinned,
`world_rejuvenation.rs`). **The demographic-tracks-environment mechanic the
operator describes is the population loop already shipped.** Richer, named
couplings (a cooling-cycle that ratchets K down over eras; a jet-stream that
moves the habitable footprint) are per-clade EXTENSIONS on this base, not
new foundations.

### 2c. THE EVOLUTIONARY ARC — a line transforms over the game  ·  **FRONTIER**

"Chitinous labor units → cybernetic aristocrats"; "reef-tribes → planet-
spanning neural webs"; "rock-mats → living archives." A line should not be
static at its founding coordinate — it should ADVANCE through staged
transformations. The engine has the raw material: tech-condition
CRYSTALLIZATION / deepening (a receipted, gated progression) and the
science/discovery path (`38_COGNITION`). What is owed is an EVOLUTION deed:
a lawful, gated re-seal that moves a line along an arc (class/binding shift)
when its conditions are met — progression as a chain event, replay-exact.
**This is the first frontier mechanic to build.**

### 2d. CASTES & ROLE MORPHING — the population differentiates  ·  **FRONTIER**

"Population castes morph physically over generations to match high-precision
industrial roles." Today the population is one undifferentiated count
(`clone_count`). Castes split it into specialized fractions (labor / cognition
/ industry) that a line's disposition and evolutionary stage weight. The
labor system (`world/labor.rs`) is the seed; caste specialization is the
extension. **The second frontier mechanic.**

## 3. THE CATALOG (named lineages → coordinates + what they exercise)

Confidently-seeded coordinates (witnessed distinct in `lineage_pins.rs`):

| lineage | disp · metab · percep · class | leisure | life | exercises |
|---|---|---|---|---|
| Vapor-Lattice | empiricist·plasmic·harmonic·plasma-field | Crystal | long | plasmic (→#29) |
| Basal-Silt | epicurean·mineral·volumetric·abyssal-pressure | Alloy | short | 2a,2b |
| Crystalline-Core | epicurean·mineral·planar·crystal-mind | Alloy | long | 2b, permanence |
| Thermo-Solder | expansionist·plasmic·planar·thermal-banker | Crystal | long | plasmic (→#29) |
| Spore-Matrix | epicurean·biotic·harmonic·biotic-swarm | Diamond | short | 2b, swarm churn |
| Void-Strand | expansionist·biotic·umbral·void-swarm | Diamond | short | 2a low-grav |

Further named lineages and the dimension each foregrounds (coordinates to
finalize as they are built): Cryptobiotic-Silt (dormancy — a 2c arc of
hibernation↔explosive recovery); Abyssal-Wyrm & Glacial-Sovereign (colossal
body ⇒ low headcount, high per-capita — a 2d/class body-scale term);
Scale-Matrix & Vapor-Stalker (cognition/output tied to stellar flux — a 2d
perception×metabolism coupling); Gilded-Mantis & Obsidian-Carapace (2c caste/
morph arcs, slow-growth rationed by cooling — 2b extension); Cephalo-Silt &
Aerosol-Spore & Chrono-Fungus (2b footprint that MOVES with the environment —
vents, jet streams, data-vault density). None require a new FOUNDATION; each
is a coordinate plus one of the four dimensions.

## 4. EXTENSION — the space stays open

A civilization the seed table cannot name is MINTED: a new clade, class,
disposition, or metabolism enters through the vocabulary novelty gate,
carries its own factor bindings over the landed channels, and is judged and
replayed like any compiled value. The catalog is never closed; the framework
is what makes an unbounded roster lawful.

## 5. BUILD SEQUENCE (owed)

1. **Evolutionary arc (2c)** — the EVOLUTION deed: gated staged re-seal, the
   arc as replay-exact chain events. The highest-leverage frontier: it gives
   every line a trajectory, not a fixed point.
2. **Castes (2d)** — split the population into weighted specialized fractions.
3. **Environment-coupling extensions (2b+)** — cooling cycles, moving
   footprints, flux-tied output — per-clade laws on the landed K base.
4. **Plasmic population (#29)** — the energy-capture demographics the two
   plasmic lineages wait on.
5. **Planet-fit CREATION (2a+)** — progenitor-gated fitted-world founding (#34).

## 6. What this composes with

- `planet_fit` / `habitability` / `clade_preferred_region` — dimension 2a/2b.
- `carrying_capacity` / population loop (#40–42) — dimension 2b, landed.
- tech crystallization / `38_COGNITION` — the raw material of 2c.
- `world/labor.rs` — the seed of 2d.
- `vocabulary` novelty gate — §4 extension.
- `43_DISTRIBUTION` — provisioning already differentiates lines by
  metabolism (leisure), class (lifespan), disposition (appetite).
