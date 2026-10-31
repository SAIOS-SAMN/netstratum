# The galactic level — one universe, galaxies by observation (GALAXY-1, design stage)

**Status:** design. No
mechanic ships before its protocol sections and test vectors exist
(Charter §1); every constant below is a **placeholder parameter**.

## 1. One universe — verified, and bound

The universe is TUniversal{S, G, T} and there is exactly one: one compact
3-torus base (span 64, the `universe.json` card), one growing polynomial
fiber, one universal clock, one star-chart. Aeons are G-sheets of the same
bundle (11 §2.0, §5) — the fold seats a new sheet, never a second universe;
the archived eras stand sealed beside the live root. Nothing in the engine
or the seed roster mints a second universe, and this chapter binds it so:
**the universe count is 1, structurally.**

## 2. One galaxy today — and never a static count

At present the whole charted torus is a single **founding galaxy**:
universe and galaxy coincide, and no structure anywhere pre-seeds a galaxy
count (verified 2026-07-30 — the concept exists nowhere in code, which is
the wanted genesis state). The board does not open pre-tiled with dead
galaxies. **Galaxies are never scenery and never a pre-allocated array: the
count starts at one and grows only by deed.**

## 3. The observation law

A galaxy does not pre-exist its observer. When a civilization achieves the
crossing (THRESHOLD-1 — interstellar flight, receipted) and its traversal
anchors **past the charted frontier**, the act of observation creates the
galaxy. **The frontier is a distance, never an outside**: the torus wraps and the manifold is coned
(12 §2c — no open ends), so "past every reach" means exactly this — the
anchor's wrapped-L1 distance to every charted galaxy's center exceeds
that galaxy's reach (the L1 law: the short way around, no squaring; reach
a placeholder parameter). A new galaxy is a **far region, never an
outside**. This is a
**progenitor power**, the tier-0 board-maker function (12 §2a: *expanding
the board*): the crossing line observes; a seated progenitor of its lineage
domain executes the creation deed (where the line stands headless — lawful,
12 §5 — the founding progenitor of its domain stands creator). Galaxies
mint lazily, on demand, unbounded — the S-side of the law the fiber already
obeys (the universe gains dimensions as discoveries land, contract §9).

## 4. What a galaxy is, mechanically (candidate bindings)

- **A named region of the single torus** — center S (exact rationals), reach
  (an L1 radius, exact). A new galaxy is a new region, never a new torus
  and never a new universe.
- **The receipt is the truth.** `GalaxyObserved` (candidate tag 54 —
  added to NS-1 §2.3 only at landing) appends to the universe ledger
  beside `EraAnnexed`/`EraFolded` — a universe-scale event carrying the
  observer (the crossing civilization), the creator (the progenitor seat),
  the center, and the reach. A `galaxies.json` card beside the star-chart
  is the lens's rendering (SEAL C), rebuilt from receipts. The
  EraAnnexed precedent binds whole (coder-01 clarification): the
  world-replay match arm is an **explicit no-op** — a universe event
  never enters a world chronicle, and world replay ignores it if it
  ever leaks.
- **Membership is derived, never stored** (the 12 §4 discipline): a world
  belongs to the galaxy whose center stands L1-nearest within reach.
  Frontier worlds outside every reach are intergalactic — lawful, exactly
  as headless and pre-stellar are lawful.
- **The founding galaxy is chartered lazily too.** The landing's first
  observation deed encloses the seeded worlds retroactively (placeholder
  center (0, 0, 0), reach 16); until then it stays implicit, as today.
- **Room grows before it runs out.** If galactic-scale occupancy presses
  the span (11 §7's bounded-occupancy law read at scale), the span itself
  grows — `universe.json`'s span is already a read parameter, and growth
  is a Mint-class irreversible extension (the fiber law applied to S).
  Expansion only; never contraction, never a second universe.

## 5. The GALAXY-1 landing

A confluence node (12 §9's manifold law — edges only): needs THRESHOLD-1
(the crossing trigger — landed), AEON-1a (the universe ledger — landed),
and PLANNER-1 (the progenitor deed gate — landed). Lands whole, one
landing: the deed + tag inscription, the frontier reading (L1 past every
reach), the lazy founding charter, the `galaxies.json` rendering, and the
witnesses — spec + mechanic + vectors together. Registry deltas: none — a
galaxy is geography, not identity (no new axis, no new station). The CLI
surface (GALAXY-1b) lands as a **`t0 galaxies` verb inside the existing
t0 console**, never a new top-level command — the progenitor's board
readings belong in one hand (coder-01 clarification).

## 6. The move seam — ratified (coder-01, 2026-07-30)

The t0 console's `move <world> x y z` re-charts a sector on the torus and
**never creates a galaxy** — only the crossing observation deed does.
Membership re-derives from position (derived, never stored — §4), so a
move costs the galaxy law nothing; a world moved far from every center is
intergalactic, which is lawful. The two S-side deeds do not collide:
`move` re-seats what exists, the observation deed brings a region into
the chart.
