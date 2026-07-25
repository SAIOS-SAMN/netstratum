# THE WORLD-BOOK, CHAPTER 1 — THE COLONY LOOP

**Status:** DESIGN, ruled — this chapter locks the first depth ruleset. It
lands in three increments (D1–D3, boarded in BUILD_ORDER). Numbers here are
the ruleset v1 constants: exact ratios, tuned for tension, changeable only by
a new ruleset version (never silently — replay depends on them).

**The design pillars (fun-first, CHARTER §2):**
1. Every mechanic creates a DECISION, or it doesn't land.
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
biomass — a sustainable homestead. TWO extractors + one habitat nets biomass
exactly 0 — the knife edge. A third extractor without a second habitat is a
famine clock. The opening (20 iron, 10 biomass) affords 2 extractors + 1
habitat with nothing to spare, or a slower, safer 1+1 — the first real
build-order decision, and an extractor repays its cost in 8 ticks of the
paired net. Industry:food equilibrium sits at 2:1.

### 2.3 Flows and the workspace

Each tick, every ACTIVE zone nets yield − upkeep into the accrued workspace
(a per-resource net that MAY be negative). `collect` settles the workspace
against the banked ledger: positive nets bank as `YieldCollected`; negative
nets draw down as `UpkeepSettled`. The workspace stays unreceipted
(the invariant holds: what has no receipt does not persist).

### 2.4 Famine and recovery

- **Trigger (checked at each tick):** if banked + accrued for a resource
  falls below 0, **famine strikes** that resource — a closure
  (`FamineStruck`). Every zone that CONSUMES the exhausted resource goes
  DORMANT: it neither yields nor drains.
- **Relief:** when the resource returns to ≥ its dormant zones' one-tick
  upkeep (via collect or razing), the zones wake (`FamineRelieved`).
- **The way out:** `raze <tile>` demolishes a zone for **half its build cost
  back** (`ZoneRazed`). A colony spiraling (habitats dormant → biomass
  collapsing) can raze extractors to cut the drain and rebuild — painful,
  survivable, and a real decision about what to sacrifice.

### 2.5 Terrain (D2)

The world is charted at founding from a **world seed** (recorded in the
chronicle — terrain is deterministic and replays exactly). Tile classes and
their yield multipliers:

| class | multiplier | map glyph | frequency (of 64) |
|---|---|---|---|
| Standard | 1/1 | `·` | ~40 |
| Rich | 3/2 | `▲` | ~12 (ore seams and fertile basins) |
| Barren | 1/2 | `░` | ~12 |

A rich-seam extractor yields 9/4 iron/tick; a barren one 3/4. Upkeep does
NOT scale with terrain — rich tiles are strictly better, barren tiles are a
cost you zone over only when the map forces you. Now tile choice matters and
the map is worth reading before the first build.

### 2.6 Adjacency (D2)

Applied after terrain, from the 4-neighborhood:

- **Extractor beside ≥1 Rich tile:** ×5/4 (the seam runs under the fence).
- **Habitat beside ≥1 Extractor:** ×3/4 (noise and dust — people don't
  thrive next to the pit).

One boost, one penalty — enough to make district planning real (industry
clustered on the seams, housing set apart) without a modifier soup.

### 2.7 Spoilage (D3) — the collect pressure

**Accrued biomass spoils: −1/4 per tick, floored at 0** (linear, so the
numbers stay crisp; iron does not rot). Spoiled yield simply never existed —
it was workspace, it has no receipt, nothing to unwind. A habitat's +1/tick
means the field loses a quarter of a tick's work every tick it waits:
collect early and often, or set a rhythm and live with the loss. The collect
button becomes a timing decision.

### 2.8 Milestones (D3)

A first goal ladder, each a closure (`MilestoneReached`), each pure flavor
pointing at the future boards:
1. **Homestead** — 5 zones active concurrently.
2. **Surplus** — 50 iron banked (the first export stockpile → the economy).
3. **Township** — 10 biomass/tick gross (the colony that could host a
   second entity → N3/N6).

## 3. Protocol additions (the LAW XVII ledger)

New permanent wire tags (NS-1 §2.3 registry; layouts land with D1/D2):

| tag | event | fired when |
|---|---|---|
| 4 | UpkeepSettled | collect settles a negative net from banked |
| 5 | FamineStruck | a resource's banked+accrued falls below 0 |
| 6 | FamineRelieved | a dormant resource recovers |
| 7 | ZoneRazed | a zone is demolished for half cost |
| 8 | WorldCharted | founding records the terrain seed (D2) |
| 9 | MilestoneReached | a milestone closes (D3) |

**Replay rule:** costs, upkeep, terrain generation, and multipliers are
RULESET CONSTANTS, versioned with the engine — replay applies them to the
receipted events (build/raze/settle/seed) and reproduces the committed world
exactly. A future ruleset change requires a chronicled ruleset-version event
before it applies; constants never change silently under an existing world.

## 4. The board increments

| # | Landing | Contains | The decision it creates |
|---|---|---|---|
| **D1** | COSTS, UPKEEP, FAMINE | build costs spent from banked; per-tick net flows; famine/dormancy/relief; `raze`; tags 4–7 | build order under scarcity; food-vs-industry balance; crisis triage |
| **D2** | TERRAIN + ADJACENCY | the seeded chart (tag 8); class multipliers; adjacency pair; map renders terrain | reading the map; district planning |
| **D3** | SPOILAGE + MILESTONES | biomass decay in the workspace; the goal ladder (tag 9) | collect timing; something to play toward |

Sequencing: D1 first (scarcity is the foundation every other tension leans
on); D2 second (place only matters once builds cost something); D3 last
(pressure and goals polish a loop that already exists). Each lands red-first
with witnesses on the knife-edge cases: the 2E+1H zero-net equilibrium, the
famine trigger at exactly −1/‌n, the raze refund, terrain determinism from
the seed, spoilage flooring at 0.
