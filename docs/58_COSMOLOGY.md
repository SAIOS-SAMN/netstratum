# 58 — Netstratum's cosmology on the network: its Big Bang, its matter, its core

**Status: PROPOSAL** (2026-10-08). Asked for by the coordinator (xylarium-kernel)
after the operator's ruling that netstratum builds its OWN universe:
netstratum's own Big Bang physics and element table, sharing with the network
only mathematics (certified reals, the hypha's exact algebra) and the court's
laws, lexicon and clock. A wormhole between netstratum and xylarium is then a
crossing between genuinely different physics, and "the uplink belongs to
whoever keeps the cores in each universe".

Every claim below is marked **CODE** (built, with its citation) or
**PROPOSED** (this document's design, awaiting the operator). Code is the law
(`code-is-the-law`): where this document and the code disagree, the code wins
and this document is wrong.

## 1. What netstratum's universe already is (CODE)

- **A universe is a var root.** `var/` is the archive (67 worlds, last beat
  1127), and `var-g2/` is G2 (docs/57). Each root holds its own worlds,
  registry, star chart (`starchart.card`) and universe ledger
  (`convergence.tlv`).
- **Space.** A torus of span 64 (`universe.card` TOPOLOGY "torus",
  `engine/ledger/src/lib.rs:208-219`). Below it, in order:
  - galaxies, chartered by observation (`observe_crossings`,
    `engine/ledger/src/galaxy.rs`);
  - region cells (`region_seed_of`, `cfe/src/dop.rs:1064`; sealed once as
    `RegionCharted`);
  - worlds, placed on the star chart;
  - a star per world (13 classes, `cfe/src/thermal.rs:191`);
  - planets/orbs on exact Keplerian tracks (`conductor/src/siting.rs`). G2's
    founding bands choose among ~365 such sites.
- **Time.** TUniversal is `epoch_base + beat`, a base-60 odometer. Eras fold
  into `{root}-era{N}` on a new gauge sheet (`conductor/src/lib.rs:168-233`,
  docs/11_AEON §5).
- **Matter.**
  - Properties are netstratum's own law: a 2240-cell periodic table
    (`cfe/src/almanac/periodic.rs:82`), families by star band
    (`element_properties`, `dop.rs:2013`), the seven base `Resource`s and
    their grain.
  - Element IDENTITIES are per-world: `BLAKE3(founding digest ‖ region_seed ‖
    orb ‖ stratum)` (`cfe/src/world/survey.rs:558`).
  - Nothing is shared with xylarium's matter.
- **Seats.** 18 stations (`ledger/src/registry.rs:35-54`). The progenitor band
  (T0: Tunborn, Sylvane, Forgeborn, Corebound, Coronal Choir) is compiled. The
  ELDER and ANCIENT seats (T16/T17) exist only as RECEIVING seats: every
  founding pays a tithe to `SEAT_ANCIENT` (`cfe/src/world/genesis.rs:333`), and
  no civilization holds the seat.
- **Authority today** is the conductor process. It holds the pen on every
  world (`CONDUCTOR_HAND`, `cfe/src/vrp.rs:368`). The operator holds the
  network keys (`var/uplink/`: voice, domain, colonies and union tiers).
  Nothing in-game owns a key.

## 2. Three gaps the code shows

1. **No Big Bang.** No receipt marks a universe's origin. `universe.card` is
   rewritten every beat. `convergence.tlv`'s first receipt is whichever comes
   first: an era annexed, a fold, or the founding galaxy charted when the first
   crossing is observed.
2. **The tithe falls out of the universe.** Each founding's tithe to the
   ancient seat is debited from the founding world and lands NOWHERE: the seat
   has no holder orb (`economy.rs:1184-1207`, "There is no seat-holder orb").
   That is a conservation hole in netstratum's own physics. The census already
   shows it as a sink (`TitheReceipted`).
3. **No keeper of the core.** The network asks who in each universe keeps its
   core: who holds the uplink, the reserve and the wormhole mouths. In
   netstratum, the answer today is "the operator's process".

## 3. PROPOSED: the Big Bang

A universe's FIRST receipt on its universe ledger is `UniverseFounded`,
written once when the conductor boots an empty root, before any world is
founded. It carries:
- the topology and span;
- the digest of the compiled roster (`founding_polities`);
- the digest of the matter law (the periodic table's cells and the
  `Resource` grain);
- the universe's seed: the receipt's own hash.

Every world's founding digest then CITES the universe seed, so a world's
matter identities descend from its universe's Big Bang. Today they stand
alone. Replay checks the chain: a universe is the same universe exactly when
its Big Bang receipt is.

- **For an existing root** (G2, the archive), the receipt cannot be first.
  It is written once, at the next boot, as the universe's RE-FOUNDING: same
  fields, citing the ledger head it was written over. It is honest about
  being late.
- **On the network** — SUPERSEDED by §6 ruling 1. A universe's Big Bang
  founds its own DOMAIN through the shared universe genesis act, rather than
  becoming an H-lane word on another domain's line. Nothing goes to the court
  for an origin until that act lands.

## 4. PROPOSED (AMENDED by §6 rulings 5-6): the keeper of the core is the ANCIENT seat, and it ARISES

The ancient seat (T17) is already netstratum's universal seat. It is the only
one of universe scope, it already receives the founding tithes, and docs/47 §4
defines it as one that opens the ROOM (creates capacity) rather than appointing.

- **How a line becomes ancient: by its own lineage, never by appointment.**
  Proposed criterion: the line whose descendants (its cleaves, and their
  cleaves) make up a share of the living roster that no other line's tree
  matches. It is read off the registry's lineage DAG (`Civilization.lineage`)
  at each era's close. That is CONVERGENT (a whole clade's growth names it)
  and DIVERGENT (it can change hands as clades rise and fall), and there is
  no threshold to tune: the largest living tree, by count.
- **What the ancient holds:**
  - the ancient reserve: every tithe, accrued on the universe ledger and
    conserved, which closes gap 2;
  - the universe's network keys, handed to it by a receipted handoff
    (`HandoffSealed`, the existing authority walk, `mesh/src/lib.rs:248`);
  - the universe's wormhole mouth when one is built (docs/23_TRAVERSAL §2k is
    design-only today).
- **Before any ancient arises,** the reserve accrues on the universe ledger
  unheld, and the conductor's hand keeps the keys IN TRUST, recorded as such.

## 5. Questions for the operator

1. Is a netstratum UNIVERSE (a var root: G2, the archive) its own network
   universe (78.86: CREATE a subsidiary), or do netstratum's universes all sit
   under the one netstratum domain on :7520 as they do today?
2. Is the ancient chosen by lineage (§4), or by another in-game reading?
3. Does netstratum's reserve (2(e), μ conversion) hold μ, or is it held in
   netstratum's own money (STAMP) and translated only at a wormhole mouth? The
   coordinator ruled reserve and bonds opt-in; nothing is posted.

## 6. Rulings received (coordinator, for the operator, 2026-10-08)

1. **One network universe = one Big Bang = one domain.** Each netstratum
   universe root with its own origin is its own universe (78.86) and gets its
   own domain on the court, founded FROM its `UniverseFounded` record through a
   shared UNIVERSE GENESIS act that mycelium-opus is building (a sub-kind of
   ACT_DOMAIN). Not an H-lane word: the domain's root. Nothing goes to the
   court for the origin until that act lands.
2. **The ancient by lineage is ACCEPTED** as netstratum's design (§4). Until
   one arises, the keys stay in trust with the conductor, ON RECORD.
3. **The reserve holds μ** (2(e)): μ only moves key to key. STAMP stays
   netstratum's in-game value on G, crossing to μ only by conversion against
   the reserve at the ancient's rate.
4. **The operator's universe principle:** spheres, manifolds and tubes, joined
   by fibres, strings and tethers. FLUID, never rigid, with no straight-line
   motion. Mass orbits stars, stars orbit cores, and cores are Einstein–Cartan
   bounces that RECYCLE matter and energy. A self-maintaining, growing
   universe, not a heat sink. Close every matter-destroying sink.

5. **A galactic core is a larger WORMHOLE** (operator, 2026-10-08). No
   singularity is open-ended: every core is a throat with a far face in the
   universe it bounces into, and a universe's Big Bang is the far face of a
   parent's core. netstratum's domain will be joined to another by a core
   tube on the court (mycelium is building it).
6. **Ancients are PRODUCED at a core** by netstratum's own physics (the shared
   definition: the core-dwelling minds that wake first within the bulge, plus
   "ancients by deed", a people that comes to fold space). LINEAGE DOES NOT
   CROWN AN ANCIENT. The strongest surface lineage is the natural PROGENITOR
   candidate: the ancients' partner, who builds the galaxy's highways and
   wormholes, above the stewards. The conductor now announces that candidate
   (`largest_lineage`; forgeborn at the time of writing), and the era fold
   seals no ancient until one arises at a core. Tag 192 (AncientArose) waits
   for that.
7. **Only the ancients uplink to POW++.** netstratum's four keys (voice,
   domain, colonies, union in `var/uplink`) are held IN TRUST by the
   operator's process, recorded as held for the ancient not yet arisen, and
   pass to it when one arises at a core.
8. **The reserve is held in the ancient's BASE RESOURCES**: what its phenotype
   needs to survive at its core, by mass, stated in clear at its genesis.
   Money crosses universes only reserve to reserve, through the core tube.
9. **Every sink routes to the core throat** (aperture melt, upkeep, famine
   draws, the raze's unrefunded half, fees, postage, the ancient tithe). The
   throat's far face is the bounce. Nothing lands nowhere. This supersedes the
   "black-hole account" in §7.

## 7. The universe's matter budget, MEASURED (G2, 2026-10-08)

Every world's census summed by receipt kind
(`cargo run --release -p ns-hypha --example sinks -- var-g2`), with aperture
legs paired by deal across the universe (`g2_legs`). These are the places
netstratum's own physics destroys matter today (amounts in whole units):

| Sink | What vanishes | G2 total |
|---|---|---|
| Aperture MELT (transit) | `amount × melt` leaves the sender and reaches no one (`stamp_aperture_transfer`, `aperture_transfer`); every leg pairs | STAMP 3,535 · Iron 65.1 · Biomass 6.8 |
| `UpkeepSettled` | zone upkeep, burned | Iron 2,206 · Biomass 112 · Alloy 95 |
| `FamineStruck` | the draw on a starving bank | Iron 552 · Alloy 52 · Biomass 15 |
| `ZoneBuilt` | build cost, embodied (a raze refunds half; the other half is gone) | Biomass 435 · Iron 240 · Alloy 100 |
| `ScrapSalvaged`, `ScienceSeated`, `ChemCataloged`, `EstateSurveyed`, `ParcelConveyed` postage | fees, burned | Iron ~181 · Crystal ~43 · Biomass ~13 |
| `TitheReceipted` (founding tithe to the ancient) | debited, lands nowhere (§2) | — |

Sources, for contrast: `YieldCollected` (the stars' light made into Biomass,
Iron, Alloy and Diamond: 35,898 in all), `DiscoveryMinted` (Crystal from
proved work: 5,468), and founding endowments.

### PROPOSED: the core recycles

- **The universe's core** is a black hole at the galactic centre (the
  "Black hole" of MU-LEXICON §1: a sink of mass and value at its place). It
  is a universe-ledger account, exact and receipted. Every sink above
  deposits there instead of vanishing:
  - the melt in transit;
  - upkeep and fees burned;
  - the unrefunded half of a raze;
  - the ancient's tithe, held for the ancient once one arises.
- **The core's bounce** is the era fold (11_AEON §5) that netstratum already
  has. When an era closes, everything the core accrued re-enters the next era
  through its white hole (MU-LEXICON §1: a source returning what the black
  hole took). It goes as the new sheet's founding endowments and its veins'
  richness, so matter is conserved across eras exactly. Within an era it flows
  core-ward, as the principle says ("stars orbit cores").
- **No rate is chosen:** the core holds what it took, and the bounce returns
  all of it. The open question for the operator is whether the bounce is ONLY
  the era fold (an operator act today), or also a continuous white-hole flow
  within an era. A continuous flow would need a rate, which this design
  declines to invent.

## 8. The torus in geometrized units (Concordance 1.8/2.3, 2026-10-08)

**One cell = 1/32 Mpc** (geometrized, G = c = 1). netstratum's universe is a non-expanding 3-torus with periods **(2, 2, 2) Mpc** (64 cells per axis) and the **L1 metric**: wrapped per-axis distance, summed. The scale comes from the universe's own measures:
- a galaxy's reach of 16 cells is 1/2 Mpc, a galaxy halo's radius;
- the mean spacing of G2's 14 charted galaxies, about 39 cells, is about 1.2 Mpc.

Any mass placed on a tube is stated in the same unit, as GM/c² in Mpc.
