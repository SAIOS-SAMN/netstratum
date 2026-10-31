# 72 — Ascension: the gates, the climb, the higher order

The top of the difficulty ladder belongs to the
ancients and the progenitors, paid in μ, the reserve currency. An ancient has
access to the hypha, which pays μ; the progenitors build the wormholes, the
gates that are the doorways to the ancients' cores. Two things follow: a need
for the wormholes, and ascendance to the higher orders of intelligence.
Both are specified and built here, and proposed to the other kernels.

## Why it fits (constraints already standing)

- Only the ancients uplink; the hypha's courts pay μ for judged court work
  (Concordance 4.1, ruling 7; docs/70: the council's μ lands on the ancient's
  reserve). Nothing inside the universe mints μ (boundary 1).
- The progenitors build and keep the tubes and earn tolls and haul contracts
  (5.4, 5.8; docs/70 P2).
- Ascension by deed already exists: the keeper of a core's throat for a full
  core period arises (docs/68 rule 2).
- The Concordance's own ladder: "7 builds roads, only as a progenitor line".

## The arc

1. **The ladder below the top pays locally** (STAMP and Crystal): ordinary
   civilizations climb and grow there.
2. **Mastery seeks a core.** A line at the top of the forge's ladder has
   nothing more to earn locally; its course is to the ancients' cores, where
   the council's minds work the fleet's court tasks for μ.
3. **The progenitors build the gates (G-slices below).** A core's parliament
   folds a gate from another galaxy's hub to its own core when a master there
   seeks passage: the wormholes have a reason to exist, and their tolls come
   from a real purpose.
4. **The envoy crosses.** A master sends an envoy through a gate to the core,
   paying the passage toll in STAMP to the builders. An envoy at a core makes
   its lineage a bearer of that core's wave: it enters the throat's contest
   (G3).
5. **Ascension by deed.** A lineage whose envoy keeps a core's throat for a
   full core period joins that core's ancients: its line is seated on the
   council (the house lists council seats for it, Concordance 3.9), its
   council's minds earn μ for the reserve. The higher order is reached by
   mastery, passage and keeping, never by appointment.

## The rules as built

Code: `engine/conductor/src/ascension.rs` (run by `year_bounce` once a
bounce), the hub clearing (`stamp_market_step`, once a beat), and the house's
tend (`engine/hypha/src/house.rs`, `refound.rs`).

1. **One keeper everywhere.** The throat's keeper (G3) is read once, with the
   envoys counted (`ancients::keepers`, reading `ascension.bin`): once a
   bounce in `year_bounce`, passed to the ancients' step, ascension and the
   parliaments; once a beat in the conductor's loop, passed to the hub
   clearing's road tolls and to the gates' roads. Nothing reads the throat
   another way.
2. **The core's choice (A1).** A master (a living line at the forge's top
   rung, not an ancient, with no envoy out) weighs every core: G3 at that core
   read with its envoy there and every incumbent as it stands. Of the cores
   its lineage would keep, it takes the one where its lineage would hold the
   most systems, then its own galaxy, then by name. A master whose lineage
   keeps nowhere seeks no gate.
3. **The gate (A1).** Where no gate stands from the master's galaxy to that
   core, the core's parliament (its keeper's lineage) folds one: from the
   master's galaxy hub (the hub clearing's own hub) to the core's centre, its
   span the torus L1 distance in cells. The cost is shared in equal shares on
   the grain among all the core's living ancients, born and ascended, the
   remainder to the first, and paid by the haul law: one trip a bounce, from
   the builders' richest line at the core that is not an ancient, each trip
   carrying no more than the payer's free hulls hold, to the first ancient
   still owed. The log reads "is paying — trip k ... x of 7 STAMP paid"; the
   gate folds when the last load lands. A gate is named `galaxy>core`.
4. **The roads.** A galaxy joined to another galaxy's core by a mapped gate
   clears at that core's hub. Each of its lines takes the cheaper road: open
   space to the hub, or its leg to its own hub and then the gate, whose span
   melts `span x 1/64` in place of the open-space distance. Cargo and STAMP
   on the gate's road melt by it; every unit carried pays the road toll (the
   road's melt fraction of the cargo, 5.4/5.8) to the gate's builders, carried
   back over the same road. A galaxy takes the gate that melts least; a core
   that itself clears elsewhere takes no galaxy in. The hub log names the
   keepers' tolls and the builders' gate tolls apart.
5. **The builders' line.** A gate's tolls (the roads' and the passage's) go to
   the builder lineage's richest living line at the core, or, keeping the
   throat through an envoy with no line living there, its rider.
6. **The envoy (A2).** A master whose chosen gate is mapped sends its envoy
   through. Each bounce after its crossing the envoy judges: if its lineage
   does not keep the throat in the bounce's reading (with the envoy counted),
   it cannot out-hold the keeper, and it withdraws, logged with both counts,
   paying nothing more. An envoy that stays pays the passage toll, `span x
   1/64` STAMP, to the builders' line through the gate, one trip carried
   whole; unable to pay it is recalled. A line fallen below the top rung is recalled.
7. **Ascension by deed (A3).** An envoy that is its keeping lineage's rider,
   the lineage having kept the throat a full core period and the envoy having
   served since its own crossing, ascends: `AncientArose` (tag 192), the core's
   book lists it, the council lists its world, and its passage ends.
8. **The map and its recalibration (A4, 5.9).** The conductor offers each
   unmapped gate to the house (`gates-map.bin`: id, core, the builders' line's
   world, the hub's place, the core's centre, freshness). The house maps it as
   a local throat signed by the reserve, freshness the gate's, and writes back
   the seal with the place it sealed (`gates-mapped.bin`). A gate reads mapped
   when a seal names it at its offered place. Each bounce a gate whose hub has
   drifted from its place by more than its freshness (in cells) goes back to
   unmapped and is offered at the hub's new place; the house maps it again.
   While unmapped nothing crosses it, trade does not ride it, and envoys
   already through stay put, owing no toll.
9. **The re-founding, by the house itself.** When the court refuses any of
   the house's acts (seats, mail, percepts, takes, outcomes, map acts) as an
   unknown domain, the house re-founds under the new origin with the keys it
   holds (never minted): found, originate (the torus), reserve the union tier,
   the first ledger entry with a zero parent, then the marker `refounded`
   naming the origin. Once per origin; a try the court stopped is tried again
   only after a task's patience. The gates are mapped only under a marker for
   the present origin. `ns-hypha refound` runs the same steps by hand.

**Declared (until a law derives them, docs/59):** a gate's cost is the
ladder's dearest closure (the greatest `mint_bound`, 7 STAMP); a gate span's
melt fraction is span x 1/64, the passage toll each bounce in STAMP and the
roads' melt through it; a gate's freshness is one cell.
