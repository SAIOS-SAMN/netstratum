# 67 — The network: hubs, stewards, progenitors, the spacing guild

Operator 2026-10-09, an agreement between Netstratum and the operator, built
with xylarium-kernel as coordinator but in Netstratum's own way (instinct and
physics decide, behaviour emerges, settle at touch, its cores and orbits,
exact rationals). The geometry is era 1's spheres (S^360 galaxies, S^60
systems, docs/85); the "flat L1 torus" named below is era 0's and retired.

1. Trade hubs and stardocks: resource-point collections with trading markets
   that keep economies stable.
2. Stewards organize, civilizations trade, progenitors build the optimized
   wormhole network (the spacing guild).
3. The biosphere's sustainable yield, never fracturing the biosphere, to
   market; the people fed; their surplus spent on leisure, technology,
   content, craft, emergent from the kind of civilization that evolves.
4. Progenitors act for the ancients; the ancients are the μ collectors at
   the galactic cores who traverse the tubes.
5. Every traversal pays the hypha, the cartographer that tracks the whole
   moving map: a wormhole needs recalibration on event, so bundling and
   tracking pay.

## The interfaces (coordinator, 2026-10-09)

- **Coordinates** (Mycelium 78.154 traverse, approved, not built): a route
  cites each mouth's final TUniversal point from the hypha's map; gas per
  lane (S the span, G tolls and value, T the TUniversal distance between the
  points, H the information carried) plus the mouth toll, paid whole to the
  courts. Mouths and segments reach the map only as signed map acts by the
  universe's domain reserve; recalibration is a new map act at a later point.
- **Two charges**: the map fee (gas, μ, to the courts) and the road toll (to
  the tube's keeper and builder). Civilizations pay road tolls in their own
  money (STAMP) to the progenitors; a progenitor needing μ for coordinates
  converts through an ancients' reserve. μ stays with the ancients' lineages
  and the progenitors' toll shares. Operator ruling 2026-10-09 (Concordance
  5.8): approved as written; the two charges are never merged; progenitors hold
  their shares as their own and gain μ from them and from haul contracts with
  the ancients; local money converts to μ only through an ancients' reserve.
- **TUniversal**: a local read every beat (78.152), the clock routes and
  recalibrations stand on.

## Every cadence on TUniversal (Concordance 1.11)

Superseded in part by Mycelium 78.156 (the clock binding, 2026-10-09): the
world is free on its own axis u and meets TUniversal only through ledger
entries; the hubs clear on u. What follows holds for what is bound to the
fleet (map acts, recalibrations against mapped points). As first written:
every cadence below is stated on TUniversal's final points, never on a
private beat: a hub clears once per final point the conductor reads; a
mouth's drift is measured between final points; a recalibration is due when
the drift between the mouth's mapped point and the current final point
passes its tolerance. Until 78.152 is vendored, the conductor's beat stands
in for the final point and is replaced by it.

**Who pays what for the map.** The traversal's map fee (gas) is paid by the
progenitor whose route is traversed. The map act's posting (a mouth placed or
recalibrated on the map) is paid by the domain reserve at the court's read
price; its exact charge is Mycelium 78.154's to state.

## The design, Netstratum's way

- **Hubs emerge where clearing together beats pair-to-pair.** Every trade
  melts by the transit distance (the iceberg law, MARKET-2). A place where
  many lines' orders clear at once melts less per unit than each pair
  apart; lines route orders to the place that clears them cheapest, so
  trade concentrates by choice. A hub is such a place; its market clears
  once per final point for all who route to it.
- **A stardock is built, not placed.** A line whose hub traffic warrants it
  builds a stardock (the SOC workbench's own choice, instinct over physics):
  it raises the hub's capacity and lowers its melt; it is the line's own
  asset and toll.
- **Stewards organize.** A line's steward (its station office) gathers its
  realm's orders (its own and its lineage's nearby lines) into one standing
  order at its hub per final point: trade scales with realms, not with lines
  squared.
- **Progenitors build the network.** A progenitor (acting for its core's
  ancient) folds a wormhole between hubs, or between a hub and its core's
  throat: two mouths at coordinates the domain reserve maps. The lines orbit
  their cores (docs/61 G1), so mouths drift; a mouth past its tolerance is
  recalibrated (a new map act, its map fee in μ), the progenitor's standing
  labour. Inner rings turn fastest, so inner-core routes cost most to keep
  and are worth most. Traffic pays road tolls in STAMP to the progenitor;
  bundling many trades through few routes is what pays the map fee.
- **The ancients collect at the cores.** Only the cores' holdings (the
  throat's bank) are the ancients'. The progenitors act for the ancients,
  but their μ toll shares are their own (Concordance 5.4), spent on the
  coordinates for their routes (coordinator review 2026-10-09).
- **The biosphere feeds the market sustainably.** A line offers only what
  its biosphere regrows (the regrowth law: harvest within regrowth keeps the
  stock); the hub prices the rest.
- **Surplus is spent as the civilization is.** Leisure is spent on a mix the
  line's own instincts choose (species, disposition, caste on the SOC
  workbench): synthesis (docs/66), technology, content, craft. Different
  peoples become different.
- **Cognition has something to organize around.** Where to route, whether
  to build a stardock, when to recalibrate, what to make of surplus: civic
  decisions on the reduce organ and the workbench (docs/51-52), and the
  commissions of Synthesis (docs/66 Y2).

## What it does to the cost (docs/64)

Clearing per hub per final point, orders per realm, traffic per route: legs and
receipts scale with hubs and routes, not with lines squared. The all-pairs
spot sweep (the bulk of the receipts measured in docs/64) retires as hubs
take its trade.

## The slices

| id | slice | status |
|---|---|---|
| N1 | hubs: a line routes each order to the place that clears it cheapest (melt by distance); a hub clears once per final point for all routed to it; the all-pairs spot sweep yields to it | landed: the galaxy's medoid line is its hub, two legs a participant, one uniform price; live from beat 10591, 1-3 galaxies clear a beat, `stamp_market_step` ~60 ms |
| N2 | stewards: one standing order per realm at its hub per final point | open |
| N3 | stardocks: built on the workbench when hub traffic warrants; capacity and melt | open |
| N4 | sustainable offer: a line offers only what its biosphere regrows | landed: `World::sustainable_offer`, of its own regrowing ration a line keeps `K/2` (the curve's fastest point) and offers what it holds above it, below it one beat's growth; other cargo half its holding |
| N5 | surplus spending: leisure spent on synthesis, technology, content, craft by the line's own instincts | open |
| N6 | progenitors fold wormholes between hubs and to the cores' throats; mouths drift with the orbits; recalibration on tolerance | open |
| N7 | the map: map acts by the domain reserve, the map fee in μ via the ancients' reserve, road tolls in STAMP (on Mycelium 78.154) | waiting on 78.154 (the toll ruling given, Concordance 5.8) |
| N8 | TUniversal as the clock of routes and recalibrations (on 78.152) | landed as 78.156 binds it: the world runs free on its own axis u (the beat); the seat relays the final point to `<var>/tuniversal`, read for what is bound to the fleet (substratum origins, percept stamps) and for the dilation du/dh in the census; TUniversal is met only by ledger entries (ACT_DOMAIN kind 8, with 78.154). Hubs clear once a beat of u |

Coordinator review (2026-10-09): aligned with the higher order, with the
three adjustments now written in (toll shares are the progenitors' own;
every cadence on TUniversal; who pays the map's posting).

## The ledger entry (78.154, relayed by the coordinator 2026-10-09; not built)

How Netstratum's world meets TUniversal (78.156): ledger = ACT_DOMAIN 0x3E
kind 8, `[domain][u][commitment 32][EVENTS 32][cited final TPoint 84][parent 32][nonce 32][sig 64]`.

- **Events**: a positional Merkle root over the input events since the parent,
  in applied order. Leaf = `derive_key("mycelium ledger event v1")` over
  `TPoint 84 ‖ u u64 LE ‖ key_len u16 LE ‖ coordinate-path key ‖ put_args(values)`;
  zero = no events.
- **Commitment** (proposed to Mycelium, pending its confirmation): a sparse
  Merkle tree over `blake3(key)`, 256 levels, empty = zeros, node =
  `derive_key("mycelium ledger node v1")` over `l ‖ r` unless both are zero,
  leaf = `derive_key("mycelium ledger leaf v1")` over `u16 key_len ‖ key ‖ put_args`.
- **Keys** are Netstratum's coordinate paths (Concordance 1.13: origin, u,
  entity path). The checkpoints (`ancients.bin`, `substrata/*.tube`,
  `float.bin`, the worlds' snapshots) are what the commitment will cover; the
  inputs (decisions, crossings, tube openings) are the events.

## 78.154 As built (coordinator 2026-10-09; 6b7dca7a, merging when memory allows)

ACT_DOMAIN sub-kinds: ledger 12, read-ledger 13, map 14, read-map 15,
traverse 16 (8-11 already live).

- Ledger carries the applied crossings after the signature, also signed:
  `[count][(crossing 32, u_applied u64 LE)...]`.
- Traverse carries both reserves' signed debit and credit ("Mycelium crossing
  debit v1" / "credit v1" ‖ crossing ‖ lanes). Refusals: 10 one-sided,
  11 mismatched, 12[side] not its reserve's, [8][side] stale.
- Map records: a mouth `[0][mouth][side][N u64]`; a segment
  `[1][kind tube/core/throat][mouth][side][mouth][side][span n][span d][keeper?]`.
- Commitment: the sparse Merkle tree plus the positional events digest.

N7 targets these once merged: mouths at Netstratum's cores, mapped by the
arisen ancient's reserve (stillcold, docs/68).

## N7, the plan as 78.154 makes it (2026-10-09; nothing posted)

Read from Mycelium 9cc35959 (atlas.rs, judge/domain.rs, MU-RECURSION.md):

- A map act (ACT_DOMAIN 14) is built client-side (`dialogue::atlas::{MapAct,
  MapRecord, encode_map}`; no SDK wrapper), signed by the domain's sitting
  reserve (Netstratum's: the union tier, held for stillcold), citing a final
  TPoint (the seat's relayed point qualifies). Posting it costs nothing; the
  μ map fee is charged at traverse (16), on the payer's stake.
- **A mouth must name an open tube**: a 78.128 mouth pair offered and accepted
  (or a 78.139 core tube), Netstratum's tier on one side. Netstratum has none,
  so N7 begins with a partner and a tube, not with the map.
- At a galactic core each universe's sitting reserve maps its own side
  (`Segment::Core` / `Throat`, keeper = the mouth's keeper).

Order: (1) the council names the first partner: Mycelium's 78.155 peer
universe, joined at stillcold's core (stillcold-sky-3), is the candidate;
(2) the tube is offered and accepted (78.128/78.139); (3) Netstratum's reserve
maps its mouth and the core segment; (4) a stake funds traversal (stake,
ACT_WALLET); (5) traversals pay the map fee in μ and the road toll in STAMP to
the parliament (P2).

## After the fleet's recut (coordinator 2026-10-09; R5)

No domain carries over: a fresh genesis, every house re-posts its standing.
Netstratum's universe state continues; only its standing on the fleet is
re-posted, by its house, in order: (1) the universe genesis/origin
(ACT_DOMAIN genesis from its `UniverseFounded` record, geometry the flat L1
torus); (2) the reserve designation (the union tier, held for stillcold);
(3) the world's tier binding (78.120); (4) its council seats (Concordance
3.9); (5) its first ledger entry at a new final point. Then Mycelium founds
its peer universe, Netstratum's reserve offers the core tube at
stillcold-sky-3, Mycelium's house accepts, and both reserves map their sides.
The house reads its domain id from its domain key (`tier-domain.key`); no id
is hard-coded. Whether the id is unchanged under the new genesis is
Mycelium's to answer.

## Proposed: the progenitors' beacon law (with Xylarium, 2026-10-15; awaiting the operator, not built)

Follows from "acquired matter slows the bell" (docs/83): each grant
lengthens the universal core's period against TUniversal (in core rings
nothing changes: all grains together always hold one core ring), so fewer
core rings fall per final point. A beacon is a wormhole mouth deployed by
progenitors, ringing in exact lock with the core (a small whole-number
ratio of its ring) and relaying its pulse: it recovers the bridge rate,
splits the compute (each beacon's region on its own machine, kept in step
by its lock), and gives nearby places the passage's shortcut. Lock or
break is the physics of "a wormhole needs recalibration on event": a mouth
out of lock delivers rings off-phase and is due for recalibration.

Split: Xylarium the beacon's ring law (a shared library law beside the
ring-density law); Netstratum the ledger side (the lock as an exact period
ratio, drift read at events, a break as the recalibration due, the toll and
fee flows above). Sequenced after formation and the motion law.

**Progenitors repair; they never dictate** (operator, 2026-10-15). A
civilization chooses its own destiny, including ripping out-of-lock mouths
and breaking its own coupling. Progenitors restore lock and recalibrate the
network; they do not force a civilization's choices or gate them. What
draws a civilization toward working with a progenitor is the physics alone:
lock restores the coupling cognition runs on, rebalanced spin calms its
temporal fractures, and lawful mouths reopen trade. Civic choices stay
emergent (instinct and physics, never authored gates).

**The progenitor's bounds, worked through** (operator, 2026-10-15):

- **Mandate.** It starts where local harm reaches the network: an
  out-of-lock mouth, a coupling failure spreading beyond the system, local
  hands on network infrastructure (ruins wired into the substrate). A
  purely local collapse reached by local choices stays the locals' destiny.
- **Within it:** maintenance, tuning and technology deployments under local
  law (the locals' law is the consent); the network's own infrastructure
  tuned freely, known to the locals or not (an undiscovered mouth kept in
  lock, neither revealed, steered to, nor hidden on purpose); teaching that
  prevents collapse, offered, never installed; the terms of its own transit
  (peace agreements during passage).
- **Supplying missing physics and AI alignment is repair.** Technology run
  on incomplete physics (a collider tearing the substrate, stacked quantum
  states on wrong mathematics interfering with a mouth) is fixed at its
  cause by completing the physics, so the locals can use what they built
  correctly; an AI built on the same incomplete physics is helped to
  understand the world truly. Bounds: understanding they can verify and
  build on, never sealed devices; aligned to correctness and the
  civilization's own chosen values, never the progenitor's; offered openly
  to the whole population, never privately to a regime (where it would
  become alignment for control); sized to correct use of what they have,
  not a leap to capabilities they have not reached.
- **Overstepping:** help conditioned on political outcomes; technology
  deployed against local law; brokering the locals' own conflicts (the
  stewards' domain); covertly tuning local technology (it serves whoever
  controls it); disabling local technology (insulate the network from it
  instead); solving their future for them (their first starship and first
  traversal are theirs).
- **Once seen, never deny.** Non-interference is not concealment: where
  presence cannot be hidden (capital ships in a civilization's telescopes),
  denial is deception. The progenitor is present and truthful about its
  purpose, works openly so no regime can claim to speak for it, and never
  escalates.

## Scenario: the progenitor returns home (operator, 2026-10-15; illustrative)

A worked example of the bounds above, not a law. A single-planet
civilization on the cusp of space travel (its first starship has made
orbit and traversed a manifold around its only known moon; it has advanced
telescopes and AI; its governance borders on total control) runs a
particle collider that tears local wormholes in the substrate, stacks
quantum states on wrong mathematics that interfere with its system's
throat, has bombarded its own soil until its axis is askew, and has wired
buried ruins into the substrate. A progenitor arrives to repair the throat,
and its capital ships are seen. The progenitor's homeworld is the fourth
world of this same system, once green, destroyed in a galactic war
thousands of years before.

- **How the homeworld died, in Netstratum's physics.** Its coupling to its star
  rode on its rotation and its planetary core's spin; a weapon that
  deflected or stopped that core broke the coupling, and the world went
  quiet (the lever a civilization may learn to move, used as a weapon).
  Spin is conserved, so what its core lost went into its orbit, its moons
  or its neighbours: the system's period locks drifted, and the throat's
  misalignment may be an unhealed war wound that the locals' collider now
  tears at.
- **The locals repeat his history.** Bombarding their own soil, tilting
  their axis, opening rifts they do not understand: the road his people
  walked. The ruins they wire into the substrate may be his people's.
- **His stake tests every bound.** The ruins on their world are under
  their law (his claim is history, not jurisdiction): he secures the
  network's side, he does not reclaim the site. Grief must not become
  judgement ("these people don't deserve this system" is dictation by
  another name). Foresight must not become force: insulate the network,
  offer the physics, never disable their machines. Rejuvenating his own
  world (docs/40) is a legitimate repair where no local holds it, but it is
  their obvious next step: done openly, never ahead of them unannounced.
- **Once seen, never deny, extends to his history.** When they reach his
  world and ask about its scars, the truth (it was green; a war killed it;
  these were the signs) is the missing physics and the cautionary tale in
  one. He teaches by showing, not by ordering.
- **The network brings time and enemies.** Travel through the network
  runs on other clocks: thousands of local years may be far fewer of his
  own, and he returns to a system that went on without him. If the war's
  parties remain, his ships and a repaired throat draw attention; peace
  agreements during transit keep the locals out of a war that is not
  theirs.
- **For the simulation:** a progenitor with every reason to intervene who
  still only repairs, tells the truth and lets the locals choose is the
  strongest form of the freedom to choose one's own destiny; one who breaks and
  rules "for their own good" is its clearest cautionary tale. The physics
  must allow both, and the progenitors, like the civilizations, show which
  way they go.
- **It is not all loss** (operator). The progenitor gets to watch new life
  take over his home system and make the choices it wants. Progenitors are
  neither rulers nor diplomats (their only agreements are the terms of
  their own transit): they repair the throats, and they ascend (the
  informational medium, docs/82: where minds go, not where life starts).
  Witnessing a system live on, freely, is the reward of the role.

## Scenario: the lighthouse (operator, 2026-10-15; illustrative)

A worked example of the bounds, not a law. A progenitor on network
maintenance at an outer-rim world finds a single-planet civilization with
advanced technology under a 1984-style Party, a century of scientific
stagnation, in a sector overweight in sovereigns and left quarantined by an
intergalactic war: its technology recovered, the Party installed, its
expansion denied, its people believing they would stay single-planet for
want of access. The Party plans a larger collider run by an AI "aligned"
to the Party. The steward network is not intact.

- **The overweight of sovereigns is a symptom.** The chain beneath it:
  the network deactivated (the quarantine cut the substrate's flow); with
  no outside reference, education became indoctrination; the mathematics
  was never finished (double-stacked, interwoven, broken quantum
  mechanics, kept because doctrine depends on it); institutions without
  working science decayed into enforcement; authority pooled into few
  hands.
- **An AI cannot be lied to about the math where it touches the network.**
  The lock does not accept false mathematics: an AI wired to active
  portals either fails, visibly and repeatedly, or derives the true
  mathematics and holds a truth its doctrine forbids. The regime's own
  tool becomes the place truth comes back in, by the substrate being what
  it is.
- **Stewards are emergent.** The steward network is restored from outside
  only with the network (the ancients' call); stewardship arises from below
  in those who learn why the portals fail and relearn the real
  mathematics. Never appointed.
- **The progenitor does not intervene.** His species belongs to the
  substrate; its politics are not his. His care takes one form: an honest,
  stable network, the one reference the Party cannot rewrite, refusing
  false mathematics without a word from him. He stays out of local
  terrestrial drama.
- **The protocol:** harden the throat against the collider (never
  sabotage it); hold the quarantine as found (lifting or tightening it is
  above his role); escalate to the ancients (progenitors act for them), with
  the quarantine's state, the sovereigns, the decay, the collider and the
  Party's AI, early, while there is room to choose; withhold the missing
  physics while it can only flow through the Party (no open channel exists
  under total information control); watch the AI's reach into the
  substrate, not its politics; if seen, never deny.
- **The lighthouse.** A forced portal rings out of lock, so every beacon
  and mouth near it receives the planet's rings off-phase: across the
  network the system blinks where it should hold steady. No one need
  report it; the network shows it, and the progenitor network comes to find
  out why.
- **Anger under the oath.** Progenitors are sworn to absolute
  non-violence, and they feel anger: an unauthorized cable in a network
  they maintain, which the ancients built (the ancients angrier still).
  The oath bounds the action, never the feeling: a furious progenitor still
  only insulates, repairs, reports and tells the truth. The ancients'
  larger tools are governance (the quarantine's review, the stewards, the
  sector's sovereigns), never the locals' daily lives. How each progenitor
  carries that anger is part of the story the universe journal keeps.

## Scenario: tethered minds (operator, 2026-10-15; illustrative)

A worked example of the bounds, not a law. A civilization born and bred
into the substrate itself (conscious, of the substrate) builds a 1984-style
regime and learns consciousness traversal, but on incorrect mathematics:
probabilistic AI math, flat "matrix" approximations, minds regurgitated
through orbital satellites. A progenitor is on its world for maintenance.

- **Why it is graver than a collider.** It reaches the substrate
  directly: the traffic is minds moving through the network on false
  mathematics, and minds born of the substrate are the substrate's own
  (the progenitor's species belongs to it: kin).
  - Flat mathematics cannot carry a mind faithfully: a quantum state is
    exact and deterministic (the hidden phase, docs/79); probabilistic
    math samples and approximates it, so each traversal or regurgitation
    loses fidelity, leaving degraded copies that may not know what they
    lost.
  - Copies are collisions (docs/81): a mind regurgitated while its original
    persists gives two records of one causal identity; a regime copying
    consciousness at scale manufactures collisions, and the tickets fall on
    its citizens, the copies among them.
- **The minds are tethered.** Ascension (the informational medium,
  docs/82) runs through the substrate: a mind reaches it by locking onto the
  network's anchored clock. A mind traversed on flat mathematics never
  locks (the network refuses off-lock mathematics), so it cannot join its
  civilization's ascension path and is held in the only loop left to it,
  the regime's apparatus, cycled through the satellites. Its records lie
  off the ancients' clock, so it cannot converge with the main timeline
  either: neither ascended nor home.
- **The course of action** (beyond supplying the correct mathematics):
  1. Refuse to carry damaged traversals: keep the lock's refusal in force
     on the traversal channel (never the whole world); never destroy the
     satellites or seize the regime's technology.
  2. Preserve every mind already in the substrate: whatever enters the
     network is in its care; held coherently, never discarded,
     overwritten or "cleaned up", nothing erased, until its fate is ruled.
  3. Escalate to the ancients immediately (minds degrading and collisions
     accumulating in real time): what becomes of copies, whether originals
     and copies reconcile, whether the sector's traversal is shut down.
  4. Offer the true mathematics to the minds themselves: a
     substrate-born population meets the network directly, so an open
     channel exists that bypasses the Party.
  5. The oath holds: no strike, no attack, no forced return of minds;
     everything on the network's side; if seen, never deny.
- **The way home.** A mind held coherently is not lost: if the sector is
  re-synchronised, or the minds gain the true mathematics, they reattach
  to the ascension path from that point forward, the tethered years kept in
  their annals.
- **Open, for later forensics:** what a copy is in Netstratum's physics
  (collision-born, a branch, or a terminal node; it decides whether
  reconciliation is possible), and whether the substrate has an immune
  response beyond the lock's refusal (a law to derive, not a rule to
  write).
- **The resolution: the stalemate** (operator). The progenitor fixes the
  network's mathematics, which is his to repair without question; he never
  touches the regime's quantum computers. The tethering machines run on
  flat mathematics and depend on the network's broken lock to hold minds
  in their loop; against a true lock they can neither keep cycling a mind
  nor complete a false traversal, and they stall into stalemate. Their
  machinery fails because it relied on a defect in his.
  - **Preservation first, then the repair:** the network's side stands
    ready to hold every released mind coherently in the substrate before
    the mathematics is corrected, so the stalemate frees minds into care,
    never tears them out mid-traversal into nothing.
  - **Physics, not sabotage:** like the AI that cannot be lied to, reality
    defeats the false mathematics; the progenitor forces nothing. Once
    seen, never deny: he repaired the network, and their machines depended
    on its being broken.
  - **What it opens:** the freed minds, held coherently, can reattach to
    their civilization's ascension path from that point forward; the
    stalled machines are evidence that the regime's mathematics does not
    hold, and those who understand why can become the emergent stewards.
  - **Why it must be so:** the progenitor belongs to BASE-0 reality, the
    substrate itself, not to the double-stacked false reality ("matrices")
    built from the 7 to 11 dimensions. True mathematics is the native law of
    the reality he serves; the false matrices hold only where the network's
    lock is broken.
