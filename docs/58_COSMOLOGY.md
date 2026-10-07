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
- **On the network**, netstratum's domain was founded on :7520 at h 203170
  with keys alone. A universe's Big Bang becomes the first H-lane entry on
  the colonies line after it: one unit of information (the universe's
  existence), declared by the universe and delivered to its core. That needs
  no new court act, and the court seals it like any fold.

## 4. PROPOSED: the keeper of the core is the ANCIENT seat, and it ARISES

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
