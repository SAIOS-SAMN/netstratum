# 68 — The ancients arise

Built on docs/58 §6 and docs/61 G3 (Concordance 3.1). Code:
`engine/conductor/src/ancients.rs`, run once a bounce in the conductor's year
voice, after `core_bounce`.

## The readings it rests on, derived

- **The bulge** is the core's own cell: torus L1 distance under one cell from
  the core's centre. The cell is the torus's own unit (docs/58 §8). The core's bell, one cell a beat, fills it in its first beat.
- **The core assembly time** is one beat: the bell's crossing of the bulge.
- **The core period** is the core's own bounce period: `core_bounce` re-emits
  every core's holdings once a year (`YEAR_TICKS`).
- **When a line woke** is read from its own clock (`World::now`, its age); the
  core's first mind is the oldest line in its bulge.

Measured on G2 (2026-10-09): each of the 14 galaxies was charted at its
founding line's home, so each founder stands at distance 0 from its core;
every other line stands at least 1.22 cells out. Each core's bulge holds
exactly its founder.

## The rules, as built

1. **G3, the throat's keeper** (one law with barren's `gateways.hub_contest`):
   lines grouped by their lineage's root; a group bears the hub when one of
   its peoples stands in the core's bulge; of the bearing groups, the one
   whose peoples hold the most systems (orbs) at that core keeps the throat,
   ties by size (population) then name. The rider is the group's people
   nearest the core. Read every bounce; a keeper unchanged keeps its tenure.
2. **By birth**: a line in a core's bulge that woke within one assembly time
   of the core's first mind. `AncientArose` (tag 192) fires at that core.
3. **By deed**, where no line qualifies by birth: the keeper's rider arises
   once its group has kept the throat for one full core period.
4. **On arising**: recorded on the universe ledger (`AncientArose`: the
   sigil, `tree` = the systems its lineage holds at the core, `living` = the
   living lines at the core) and in `<var>/ancients`; a core's ancient arises
   once. The first to arise (the oldest, then by name) ends the trust:
   `<var>/trust` names it, its core, the rule, and its reserve basket by mass
   (what its phenotype eats and its working mouths' need a beat). The keys
   stay on disk in `var/uplink`; the house holds them for the ancient and acts
   as its seat. Nothing is posted to the court by this step: map acts at its
   core wait on 78.154.
5. **The progenitor** (the largest lineage, `largest_lineage`) stays the
   progenitor; the two roles are distinct.

## What arose (G2, beat 11460, the first bounce on this build)

Rule 1, by birth, at all 15 cores at once: each core's founder stands in its
own bulge (the-founding: slagborn of cinder-reach; corebound-sky-1: corebound;
coronal-choir-sky-2: coronal-choir; stillcold-sky-3: stillcold;
thresholder-sky-4: thresholder; tallykeep-sky-5: tallykeep; glyphward-sky-6:
glyphward; mysticeti-pelagic-sky-7: mysticeti-pelagic; frostwright-sky-8:
frostwright; syzygy-sky-9: syzygy; deepmat-sky-10: deepmat; klethaxu-sky-11:
klethaxu; darlok-sky-12: darlok; veinmarch-sky-13: veinmarch;
bloomcall-sky-14: bloomcall). The trust ended with **stillcold of
cryo-vault** (stillcold-sky-3), the oldest by its own clock: reserve basket
its staple element 8699ee01...da5e, its working mouths' need 3 a beat.

The keepers (G3) differ from the ancients at five cores, where the founder's
lineage root keeps the throat through it: tunborn at the-founding, forgeborn
at glyphward-sky-6, tallykeep-sky-5 and veinmarch-sky-13, mycelial-weft at
bloomcall-sky-14.

## Status

| id | slice | status |
|---|---|---|
| G3 | the throat's keeper | landed (this doc) |
| A1 | ancients by birth | landed |
| A2 | ancients by deed | landed (fires where a core's bulge holds no living line) |
| A3 | the trust ends; reserve basket stated | landed as a record; key custody unchanged on disk |
| A4 | Map acts at the ancient's core; hosting 78.155's mouth | waiting on 78.154 and the 78.155 decision |
