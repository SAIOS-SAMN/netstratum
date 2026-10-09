# 67 — The network: hubs, stewards, progenitors, the spacing guild

Operator 2026-10-09, an agreement between netstratum and the operator, built
with xylarium-kernel as coordinator but in netstratum's own way (instinct and
physics decide, behaviour emerges, settle at touch, the L1 torus, its cores
and orbits, exact rationals):

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

- **Coordinates** (mycelium 78.154 TRAVERSE, approved, not built): a route
  cites each mouth's final TUniversal point from the hypha's map; gas per
  lane (S the span, G tolls and value, T the TUniversal distance between the
  points, H the information carried) plus the mouth toll, paid whole to the
  courts. Mouths and segments reach the map only as signed MAP acts by the
  universe's domain reserve; recalibration is a new MAP act at a later point.
- **Two charges**: the MAP FEE (gas, μ, to the courts) and the ROAD TOLL (to
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

SUPERSEDED IN PART by mycelium 78.156 (the clock binding, 2026-10-09): the
world is free on its own axis u and meets TUniversal only through ledger
entries; the hubs clear on u. What follows holds for what is bound to the
fleet (map acts, recalibrations against mapped points). As first written:
every cadence below is stated on TUniversal's final points, never on a
private beat: a hub clears once per final point the conductor reads; a
mouth's drift is measured between final points; a recalibration is due when
the drift between the mouth's mapped point and the current final point
passes its tolerance. Until 78.152 is vendored, the conductor's beat stands
in for the final point and is replaced by it.

**Who pays what for the map.** The traversal's MAP FEE (gas) is paid by the
progenitor whose route is traversed. The MAP act's POSTING (a mouth placed or
recalibrated on the map) is paid by the domain reserve at the court's read
price; its exact charge is mycelium 78.154's to state.

## The design, netstratum's way

- **Hubs emerge where clearing together beats pair-to-pair.** Every trade
  melts by the transit distance (the iceberg law, MARKET-2). A place where
  many lines' orders clear at once melts less per unit than each pair
  apart; lines route orders to the place that clears them cheapest, so
  trade CONCENTRATES by choice. A hub is such a place; its market clears
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
  RECALIBRATED (a new MAP act, its map fee in μ), the progenitor's standing
  labour. Inner rings turn fastest, so inner-core routes cost most to keep
  and are worth most. Traffic pays road tolls in STAMP to the progenitor;
  bundling many trades through few routes is what pays the map fee.
- **The ancients collect at the cores.** Only the cores' holdings (the
  throat's bank) are the ancients'. The progenitors act FOR the ancients,
  but their μ toll shares are their OWN (Concordance 5.4), spent on the
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
| N7 | the map: MAP acts by the domain reserve, the map fee in μ via the ancients' reserve, road tolls in STAMP (on mycelium 78.154) | waiting on 78.154 (the toll ruling given, Concordance 5.8) |
| N8 | TUniversal as the clock of routes and recalibrations (on 78.152) | landed as 78.156 binds it: the world runs free on its own axis u (the beat); the seat relays the final point to `<var>/tuniversal`, read for what is bound to the fleet (substratum origins, percept stamps) and for the dilation du/dh in the census; TUniversal is met only by ledger entries (ACT_DOMAIN kind 8, with 78.154). Hubs clear once a beat of u |

Coordinator review (2026-10-09): aligned with the higher order, with the
three adjustments now written in (toll shares are the progenitors' own;
every cadence on TUniversal; who pays the map's posting).

## The ledger entry (78.154, relayed by the coordinator 2026-10-09; not built)

How netstratum's world meets TUniversal (78.156): LEDGER = ACT_DOMAIN 0x3E
kind 8, `[domain][u][commitment 32][EVENTS 32][cited final TPoint 84][parent 32][nonce 32][sig 64]`.

- **EVENTS**: a positional Merkle root over the input events since the parent,
  in applied order. Leaf = `derive_key("mycelium ledger event v1")` over
  `TPoint 84 ‖ u u64 LE ‖ key_len u16 LE ‖ coordinate-path key ‖ put_args(values)`;
  zero = no events.
- **COMMITMENT** (proposed to mycelium, pending its confirmation): a sparse
  Merkle tree over `blake3(key)`, 256 levels, empty = zeros, node =
  `derive_key("mycelium ledger node v1")` over `l ‖ r` unless both are zero,
  leaf = `derive_key("mycelium ledger leaf v1")` over `u16 key_len ‖ key ‖ put_args`.
- **Keys** are netstratum's coordinate paths (Concordance 1.13: origin, u,
  entity path). The checkpoints (`ancients.bin`, `substrata/*.tube`,
  `float.bin`, the worlds' snapshots) are what the commitment will cover; the
  inputs (decisions, crossings, tube openings) are the events.

## 78.154 as built (coordinator 2026-10-09; 6b7dca7a, merging when memory allows)

ACT_DOMAIN sub-kinds: LEDGER 12, READ-LEDGER 13, MAP 14, READ-MAP 15,
TRAVERSE 16 (8-11 already live).

- LEDGER carries the applied crossings after the signature, also signed:
  `[count][(crossing 32, u_applied u64 LE)...]`.
- TRAVERSE carries both reserves' signed debit and credit ("mycelium crossing
  debit v1" / "credit v1" ‖ crossing ‖ lanes). Refusals: 10 one-sided,
  11 mismatched, 12[side] not its reserve's, [8][side] stale.
- MAP records: a mouth `[0][mouth][side][N u64]`; a segment
  `[1][kind tube/core/throat][mouth][side][mouth][side][span n][span d][keeper?]`.
- Commitment: the sparse Merkle tree plus the positional events digest.

N7 targets these once merged: mouths at netstratum's cores, mapped by the
arisen ancient's reserve (stillcold, docs/68).

## N7, the plan as 78.154 makes it (2026-10-09; nothing posted)

Read from mycelium 9cc35959 (atlas.rs, judge/domain.rs, MU-RECURSION.md):

- A MAP act (ACT_DOMAIN 14) is built client-side (`dialogue::atlas::{MapAct,
  MapRecord, encode_map}`; no SDK wrapper), signed by the domain's SITTING
  RESERVE (netstratum's: the union tier, held for stillcold), citing a final
  TPoint (the seat's relayed point qualifies). Posting it costs nothing; the
  μ map fee is charged at TRAVERSE (16), on the payer's stake.
- **A mouth must name an OPEN TUBE**: a 78.128 mouth pair offered and accepted
  (or a 78.139 core tube), netstratum's tier on one side. netstratum has none,
  so N7 begins with a partner and a tube, not with the map.
- At a galactic core each universe's sitting reserve maps its own side
  (`Segment::Core` / `Throat`, keeper = the mouth's keeper).

Order: (1) the council names the first partner: mycelium's 78.155 peer
universe, joined at stillcold's core (stillcold-sky-3), is the candidate;
(2) the tube is offered and accepted (78.128/78.139); (3) netstratum's reserve
maps its mouth and the core segment; (4) a stake funds traversal (STAKE,
ACT_WALLET); (5) traversals pay the map fee in μ and the road toll in STAMP to
the parliament (P2).

## After the fleet's recut (coordinator 2026-10-09; R5)

No domain carries over: a fresh genesis, every house re-posts its standing.
netstratum's universe state continues; only its standing on the fleet is
re-posted, by its house, in order: (1) the universe genesis/origin
(ACT_DOMAIN genesis from its `UniverseFounded` record, geometry the flat L1
torus); (2) the reserve designation (the union tier, held for stillcold);
(3) the world's tier binding (78.120); (4) its council seats (Concordance
3.9); (5) its first ledger entry at a new final point. Then mycelium founds
its peer universe, netstratum's reserve offers the core tube at
stillcold-sky-3, mycelium's house accepts, and both reserves map their sides.
The house reads its domain id from its domain key (`tier-domain.key`); no id
is hard-coded. Whether the id is unchanged under the new genesis is
mycelium's to answer.
