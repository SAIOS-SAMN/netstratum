# The world-book, chapter 1 — the colony loop

**Status:** design, ruled — this chapter locks the first depth ruleset. It
lands in four increments (D1–D4, boarded in BUILD_ORDER). Numbers here are
the ruleset v1 constants: exact ratios, tuned for tension, changeable only by
a new ruleset version (never silently — replay depends on them).

**The design pillars (fun-first, Charter §2):**
1. Every mechanic creates a decision, or it doesn't land.
2. The player feels scarcity, timing, and place — never the arithmetic.
3. Failure is a crisis with a way out, not a dead end.
4. All quantities are exact ratios; displayed numbers stay crisp (halves,
   quarters — no denominator explosions).

---

## 1. The core loop

```
scarcity ──► zoning SPENDS banked resources     (build order matters)
tension  ──► zones drain upkeep per tick        (balance food vs industry)
place    ──► terrain multiplies yields          (read the map)
synergy  ──► adjacency boosts and penalties     (plan districts)
timing   ──► accrued biomass spoils             (collect under pressure)
```

## 2. Ruleset v1 — the constants

### 2.1 Founding

Founding banks the bootstrap reserves: **20 iron, 10 biomass**.

### 2.2 Zones

| zone | cost (banked, spent at build) | yield /tick | upkeep /tick |
|---|---|---|---|
| Extractor | 10 iron | +3/2 iron | −1/2 biomass (crew rations) |
| Habitat | 5 biomass | +1 biomass | −1/4 iron (life support) |

**Why these numbers:** one extractor + one habitat nets +5/4 iron and +1/2
biomass — a sustainable homestead. Two extractors + one habitat nets biomass
exactly 0 — the knife edge. A third extractor without a second habitat is a
famine clock. The opening (20 iron, 10 biomass) affords 2 extractors + 1
habitat with nothing to spare, or a slower, safer 1+1 — the first real
build-order decision, and an extractor repays its cost in 8 ticks of the
paired net. Industry:food equilibrium sits at 2:1.

### 2.3 Flows and the workspace

Each tick, every active zone nets yield − upkeep into the accrued workspace
(a per-resource net that may be negative). `collect` settles the workspace
against the banked ledger: positive nets bank as `YieldCollected`; negative
nets draw down as `UpkeepSettled`. The workspace stays unreceipted
(the invariant holds: what has no receipt does not persist).

### 2.4 Famine and recovery

- **Trigger (checked at each tick):** if banked + accrued for a resource
  falls below 0, **famine strikes** that resource — a closure
  (`FamineStruck`). Every zone that consumes the exhausted resource goes
  dormant: it neither yields nor drains.
- **Relief:** when the resource returns to ≥ its dormant zones' one-tick
  upkeep (via collect or razing), the zones wake (`FamineRelieved`).
- **The way out:** `raze <tile>` demolishes a zone for **half its build cost
  back** (`ZoneRazed`). A colony spiraling (habitats dormant → biomass
  collapsing) can raze extractors to cut the drain and rebuild — painful,
  survivable, and a real decision about what to sacrifice.

### 2.5 Terrain (D3)

**Landed 2026-07-30 (first slice) — the opt-in adaptation.** The deed
is `survey` (engine `chart_terrain`, tag 8 `WorldCharted`, Build
scope): it charts a star once, and the seed is the founding receipt's
digest (chronicle-seeded, Charter §9 — no wall clock, no dice). Opt-in
per the THERMAL-1 precedent: a star without the receipt reads all
Standard forever, so no landed witness or live world shifts under the
ruleset law. v1 multiplies extraction and agriculture (extractor,
habitat) only; converters and heat-law zones are untouched. `terrain`
renders the map (· ▲ ░, zones lettered). Auto-chart at founding and
conductor adoption are the named follow-up after a fixture audit; tag
9 stays reserved (D3 milestones).

The world is charted from a **world seed** (recorded in the
chronicle — terrain is deterministic and replays exactly). Tile classes and
their yield multipliers:

| class | multiplier | map glyph | frequency (of 64) |
|---|---|---|---|
| Standard | 1/1 | `·` | ~40 |
| Rich | 3/2 | `▲` | ~12 (ore seams and fertile basins) |
| Barren | 1/2 | `░` | ~12 |

A rich-seam extractor yields 9/4 iron/tick; a barren one 3/4. Upkeep does
not scale with terrain — rich tiles are strictly better, barren tiles are a
cost zoned over only when the map forces it. Now tile choice matters and
the map is worth reading before the first build.

### 2.6 Adjacency (D3)

Applied after terrain, from the 4-neighborhood:

- **Extractor beside ≥1 Rich tile:** ×5/4 (the seam runs under the fence).
- **Habitat beside ≥1 Extractor:** ×3/4 (noise and dust — people don't
  thrive next to the pit).

One boost, one penalty — enough to make district planning real (industry
clustered on the seams, housing set apart) without a modifier soup.

### 2.7 Spoilage (D4) — the collect pressure

**Accrued biomass spoils: −1/4 per tick, floored at 0** (linear, so the
numbers stay crisp; iron does not rot). Spoiled yield simply never existed —
it was workspace, it has no receipt, nothing to unwind. A habitat's +1/tick
means the field loses a quarter of a tick's work every tick it waits:
collect early and often, or set a rhythm and live with the loss. The collect
button becomes a timing decision.

### 2.8 Milestones (D4)

A first goal ladder, each a closure (`MilestoneReached`), each pure flavor
pointing at the future boards:
1. **Homestead** — 5 zones active concurrently.
2. **Surplus** — 50 iron banked (the first export stockpile → the economy).
3. **Township** — 10 biomass/tick gross (the colony that could host a
   second entity → N3/N6).

## 3. Protocol additions (the law xvii ledger)

New permanent wire tags (NS-1 §2.3 registry; layouts land with their increments):

| tag | event | fired when |
|---|---|---|
| 4 | UpkeepSettled | collect settles a negative net from banked |
| 5 | FamineStruck | a resource's banked+accrued falls below 0 |
| 6 | FamineRelieved | a dormant resource recovers |
| 7 | ZoneRazed | a zone is demolished for half cost |
| 8 | WorldCharted | the terrain seed, the opt-in deed (D3 — landed 2026-07-30; the seed is the founding receipt's digest) |
| 9 | MilestoneReached | a milestone closes (D4) — **not landed, corrected 2026-08-13 (doc audit): `ClosureEvent` has no such variant, 0 hits in `engine/`; spoilage (§2.7) is likewise unbuilt — `demurrage_due` exists but applies only to Ember/Sink, never biomass** |

**Replay rule:** costs, upkeep, terrain generation, and multipliers are
ruleset constants, versioned with the engine — replay applies them to the
receipted events (build/raze/settle/seed) and reproduces the committed world
exactly. A future ruleset change requires a chronicled ruleset-version event
before it applies; constants never change silently under an existing world.

## 4. The board increments

| # | Landing | Contains | The decision it creates |
|---|---|---|---|
| **D1** | Costs, upkeep, famine | build costs spent from banked; per-tick net flows; famine/dormancy/relief; `raze`; tags 4–7 | build order under scarcity; food-vs-industry balance; crisis triage |
| **D2** | The industrial pipeline (§5) | alloy + diamond materials; smelter + synthesizer converter zones; multi-resource costs and upkeep | raw expansion vs refining; feeding the chain; when to climb the tier |
| **D3** | Terrain + adjacency | **first slice landed 2026-07-30**: the seeded chart (tag 8, opt-in); class multipliers exact (9/4 · 3/4 witnessed); adjacency pair after terrain (×5/4 seam-fence, ×3/4 noise — witnessed); `survey` + `terrain` verbs. Open: auto-chart at founding + conductor adoption (fixture audit first); D3 milestones (tag 9) | reading the map; district planning |
| **D4** | Spoilage + milestones | biomass decay in the workspace; the goal ladder (tag 9) | collect timing; something to play toward |

Sequencing: D1 first (scarcity is the foundation every other tension leans
on); D2 second (the pipeline gives terrain something to matter for — rich
seams feed smelters); D3 third; D4 last (pressure and goals polish a loop
that already exists). Each lands red-first with witnesses on the knife-edge
cases: the 2E+1H zero-net equilibrium, the famine trigger at exactly −1/n,
the raze refund, the smelter's exact conversion, the starved synthesizer,
terrain determinism from the seed, spoilage flooring at 0.

## 5. The industrial pipeline (D2 addendum, ruled 2026-07-25)

The mid-game: raw ore is bulky and cheap; refined matter is dense and
valuable. Mine → smelt → synthesize is the progression arc from outpost to
industrial metropolis, and the dense outputs are what will be worth carrying
along hyperlanes (the economy board) and finding with the scanner (N4).

### 5.1 The materials

| material | wire code | tier | earned by |
|---|---|---|---|
| ferrite (iron) | 1 | raw | extractors |
| biomass | 2 | raw | habitats |
| refined alloy | 3 | smelted | smelters |
| synthetic diamond | 4 | ultra-refined | synthesizers |

**Rulings against the first draft:** the draft's *resonance crystals* are
deferred to the N4 scanner landing, where the name earns its meaning
(crystals as the scanner's power source); its *regolith slag / carboniferous
matrix* intermediate materials are dropped (they appeared once each and fed
nothing — every material must be produced by one zone and consumed by
another, or it's inventory clutter).

### 5.2 The converter zones

A converter is a zone whose upkeep is its input and whose yield is its
output — conversion is a per-tick net flow in the workspace, exactly like
every other flow (02 §2.3):

| zone | cost (spent at build) | inputs /tick | output /tick |
|---|---|---|---|
| Smelter | 15 iron + 10 biomass | −2 iron, −1/2 biomass (fuel) | +1 alloy |
| Synthesizer | 20 alloy + 15 biomass | −4 alloy, −1 biomass | +1/4 diamond |

Raze refunds remain half of each cost component.

**The demand curve these ratios force:** one smelter consumes more iron than
one extractor makes (2 vs 3/2) — smelting demands a mining base. One
synthesizer consumes four smelters' output. The full one-synthesizer colony
(≈6E + 4S + 1Syn + 6H) lands biomass on the knife edge again at a
seventeen-zone build — the early-game tension recurs at industrial scale.

### 5.3 No new event vocabulary (the ruling that matters)

The draft proposed `ResourceSmelted` / `DiamondSynthesized` /
`ProductionDormant` tags. **Refused — the existing vocabulary already
expresses the pipeline:**

- Conversion is a workspace flow; `collect` settles it as the existing
  `YieldCollected` (alloy, diamonds) and `UpkeepSettled` (consumed inputs).
  Conversion rates are ruleset constants; the receipts fully ground replay.
- Input starvation is famine: a smelter outrunning its ore supply drives
  iron's banked+accrued below zero → `FamineStruck{iron}` → iron consumers
  dormant. `ProductionDormant` would be a second name for a mechanism that
  already exists. A synthesizer built without four smelters behind it
  starves within a tick — the pipeline teaches its own ratios.

New wire content of D2 is therefore only: resource codes 3–4 and zone-kind
codes 3–4 inside existing tag layouts. Additive, byte-compatible, no new
tags spent.
