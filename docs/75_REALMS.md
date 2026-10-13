# 75 — Realms: how stewards' boundaries emerge

Operator (2026-10-13): stewards govern REALMS, the boundaries between
civilizations, and control diplomacy, trade and sovereign disputes (docs/74,
the tiered hierarchy). The boundaries must be DERIVED, as emergent as
possible. Any constant is a placeholder for undefined physics.

## Facts (read from the code)

- **Civilizations.** A line's civilization is its lineage root
  (`ancients::root_of` over the registry's lineage), the same grouping as the
  throat's contest (G3).
- **Contact already happens and is returned each beat:**
  - `market_step` returns `(seller, buyer)`;
  - `merchant_step` returns `(merchant, a, b)`;
  - `element_trade_step` returns `(from, to)`;
  - the hub clearing meets each participant at its hub line, but does not
    yet name them.
- **Diplomacy is physics with no decider.** `cfe::treaty::TreatyKind`
  {Trade, Defense, Peace, War} (`cfe/src/treaty.rs:103`) and
  `World::treaty_as` (`world/diplomacy.rs`):
  - a Trade treaty lowers the pair's melt;
  - War walls every aperture between the pair (`at_war_with`).

  No conductor step calls `treaty`: diplomacy is orphaned.
- **Conflict did not run.** `espionage_step`, `market_war_step`,
  `kin_step` and `instrument_step` (authored disposition branches) were
  called only by the dead test loop `run_era`, never by the live conductor;
  removed 2026-10-13 (the no-live-callers sweep). The cfe deeds beneath
  them (spying, sabotage, raids, notes, escorts) remain, for the decisions
  below.

## The derivation (operator, 2026-10-13: no realm by default)

Realms do not divide the known universe. A realm forms only where an
ancient's progenitors have deployed a passage network, and passage actually
crossed it. Space no network reached belongs to no realm.

1. **The term** is the core period, the bounce: one year (docs/68).
2. **A realm is named by its core.** The ancients of a core design its
   wormholes; the progenitors (the core's parliament, docs/72) build and keep
   the gates leading to it. The gates to one core and the passage over them
   are one network, one realm.
3. **Passage** in a term is counted per line and core:
   - a hub leg that rode a mapped gate to that core (the hub step's gated
     road);
   - an envoy standing at that core at the term's close (`ascension.bin`).

   Building a gate (the payment trips) is not passage.
4. **A realm exists** for a core when passage to it was counted in the term.
   Its members are:
   - the civilizations whose lines used that core's network the most;
   - the progenitors operating it (the builders' lineages of its mapped
     gates).

   A civilization whose use is tied between networks is CONTESTED, a member
   of none.
5. **A boundary** is a civilization reached by more than one realm's network
   in the term. The realms meet there; disputes and diplomacy live there.
6. **The steward** of a realm is the most-contacted line among its members in
   the term (contacts are completed exchanges, ties by name); with no
   contacts, the line with the most passage.

Nothing outside a network is written: no realm of one, no partition. Realms
appear as progenitors deploy and passage uses them, and shrink or vanish when
it stops.

## Open, for the operator: passage over inner-galaxy wormholes

Most live gates run from a galaxy's hub to its own core
(`from == core`). The hub step routes trade only through gates BETWEEN
galaxies (`stamp_market_step`, the gates' roads), so over an inner-galaxy gate
only envoys pass, and those realms stay nearly empty. What rides an
inner-galaxy wormhole, and why, is physics not yet defined. The ancients
design these wormholes (docs/74), so their passage rules are to be ruled, not
invented here.

## The ladder: positions measured from physics (operator, 2026-10-13)

Roles are positions read from the physics, never checklists or milestones
(docs/76: the physics defines, the story emerges).

- **No containment rule.** A line's range is what the melt, its technology and
  its hulls allow. A wormhole collapses distance; without one, the physics
  bounds how far a line reaches.
- **A SOVEREIGN** is the line that controls a system's mass: the one whose
  holdings and working of that system's orbs and veins are the greatest,
  measured from the chronicle. Control of a system's mass is UNDEFINED as
  physics today (docs/76); it is defined first, then sovereignty is read from
  it. A line that controls no system is a CAPTAIN: it acts from what it holds
  and carries.
- **Stewards are promoted by the progenitors from a realm's sovereigns**
  (operator ruling), as a DECIDE kind (docs/73), `promote`:
  - rows: the realm's sovereigns;
  - loss: the realm's measured outcomes under that steward (unmet trade,
    contested mass);
  - incumbent: the derived steward (the most-contacted member line).

  No criterion for promotion is authored; the progenitors learn it.
- **Wormholes for transport** are the progenitors' decision (docs/73 `gate`),
  driven by whether a gate pays back in what it carries, which is physics to
  define (docs/76).
- **Where realms meet,** their stewards meet. Diplomacy and war are their
  `treaty` decisions over the treaty physics (melt, walls).
- **The ancients** go where their physics lets them live. Habitability by
  gravity and temporal resonance is physics to define (docs/76), not a rule
  about ancients.

## What the stewards will decide (later steps, by DECIDE, docs/73)

- **treaty**: Trade, Peace, Defense or War with a civilization across a
  boundary, or renounce one.
  - Rows: the boundary's civilizations.
  - Loss: contact with them lost, or the realm's unmet trade risen.
- **dispute**: two sovereigns of the realm contesting a good (both bidding
  for one hub's cargo and one left unmet) or a war between them; the steward
  rules.
  - Loss: the dispute recurs.
- Espionage and market war, which never ran live, enter as decisions of
  the same kind over the cfe deeds.

Each is learned, scored before the outcome, and incumbent until a word beats
persistence (docs/73).

## Build order and done-when (the live log, before → after)

1. **Reading.**
   - Contacts accumulated each beat from the trade steps' returns, with the
     hub clearing naming its participants.
   - Passage accumulated from the hub's gated legs, and the envoys at the
     term's close.
   - At each bounce: realms (by core), their members, boundaries, contested
     civilizations and stewards, logged and checkpointed (`realms.bin`).
     Behaviour unchanged.
   - Done-when: a REALMS line each bounce, no reading → realms, boundaries
     and stewards per term.
   - Witness: the contacts agree with the trade steps' logged crossings.
2. **Control of a system's mass,** defined as physics (with docs/76's other
   undefined rows); then sovereigns read from it and logged per bounce.
   - Done-when: sovereigns per system 0 → > 0, each with its measured
     control.
3. **promote.** The progenitors' DECIDE kind; the steward seat named per
   realm (a seat for a mind, Concordance 3.8; an in-game mind, not a network
   one, 3.9).
   - Done-when: stewards promoted by progenitors 0 → > 0, each realm's steward
     a sovereign of it.
4. **treaty**, a DECIDE kind.
   - Done-when: treaties sealed by stewards 0 → > 0; realms merging or
     splitting along them.
5. **dispute**, a DECIDE kind.
   - Done-when: disputes ruled 0 → > 0; recurrence falling.

## Placeholders, stated plainly

- Civilization = lineage root.
- Passage = a gated hub leg, or an envoy at the core.
- Membership = the network used most.
- Steward = the most-contacted member line (the incumbent until progenitors
  promote).
- Sovereign = the controller of a system's mass, once that physics is
  defined.

Each reads the physics as it stands. When the physics defines influence more
finely (a civilization's claim on space, its fleets' reach), the derivation
reads that instead.
