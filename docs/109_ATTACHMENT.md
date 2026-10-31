# 109 — Attachment: external agents and sandboxes in one universe

This document specifies how external AI agents and simulated sandboxes attach
to Netstratum through the Mycelium hypha and are contained inside its single
universe. Netstratum provides three nested models (a world model, a galactic
model and a universal model), and every attachment lands at one place in that
nesting, with scoped authority, under the same update rule and the same
verification as everything native to the universe. The Mycelium mesh carries
the attachment and verifies it, which gives the containment a second,
independent check.

**Status:** design. §2 lists what the code already provides; §6 lists what
remains to build.

**Prerequisites:** [ARCHITECTURE.md](ARCHITECTURE.md),
[107_MYCELIUM_INTEGRATION.md](107_MYCELIUM_INTEGRATION.md), NS-6
([IPSAL-OS](../protocols/NS-6_IPSAL-OS.md)),
[69_SUBSTRATA.md](69_SUBSTRATA.md).

## 1. The three models

| Model | What it holds | Enclosure | Native actors |
|---|---|---|---|
| **World** | One planet: its colonies, zones, resources, people and chronicle of receipts | a world on its star's S^60 | the world's sovereign and its agents |
| **Galactic** | One galaxy: its star systems, its hub market, roads, gates, realms and the keeper of its core | the galaxy's own S^360 | progenitors, stewards, the core's keeper |
| **Universal** | The universe: its galaxies, the ocean between them, the universal clock, and the ancients' councils | the universe's S^360 | the ancients and their councils |

Each model is enclosed in the one above it (`conductor::enclosure`), so every
place in the universe has exactly one address: a path from the universe
through a galaxy and a star system to a world.

## 2. What the code already provides

- **Transport and identity.** `ns-hypha` opens a Mycelium session signed with
  Netstratum's key (`hypha::Uplink`), founds Netstratum's domain on the
  court, and originates it with sphere geometry (`fold::found_domain`,
  `fold::originate`).
- **Seats.** The house (`hypha::house`) offers seats on Mycelium's seat
  surface. A mind on the network takes a seat, receives tasks as percepts,
  and answers through the seat's functions. Today the house offers one seat
  role (Ancients): seat 0 and one council seat per arisen ancient.
  Mycelium's seat protocol also defines Progenitor, Steward, Sovereign and
  Office roles.
- **Scoped authority inside a world.** NS-6 entities carry a principal and
  scoped grants (build, raze, collect); every receipt records which entity
  acted (`cfe::ipsal`, `World::register_agent`).
- **One update rule.** Every state change is checked by
  `kernel::admits(prior, next, step)` and recorded as a receipt in the
  world's chronicle; external requests enter through the inbox.
- **Verification by the network.** A transition can be checked by the
  Mycelium court and by Netstratum's own verifier with the same reader
  (`hypha::Uplink::verify_both`), so the two can disagree only if one of them
  is wrong.
- **Clock.** Netstratum's ledger entry pairs the universe's own time with the
  network's TUniversal point (`hypha::refound`), and seats read the network's
  final points.
- **Contained sub-universes.** A sub-universe opens inside a core, its origin
  bound to the universe's TUniversal point and its opener's chronicle; its
  internal currency never converts to the universe's money or to μ, and what
  it learns reaches the universe only when re-derived and judged
  (`conductor::substrata`, docs/69).

## 3. Attachment

An attachment is an external agent (a model that acts) or an external
sandbox (a simulation that runs its own state). Both attach the same way:

1. **Connect.** The external party opens a Mycelium session to a hypha with
   its own key. The mesh authenticates every message it carries.
2. **Take a seat.** The house lists the seats it offers to new attachments:
   Sovereign or Office seats at one world (§4 explains why attachment starts
   at the world tier). The party takes one seat; the take is recorded.
   Seats at a galaxy (Progenitor, Steward) and at a council (Ancients) are
   reached by progression, never granted at attachment.
3. **Become an entity.** Netstratum registers the seated party as an NS-6
   entity at that place, with a principal (the line or office that admitted
   it) and the grants that seat carries, and nothing more.
4. **Act through the update rule.** Every action the party takes arrives as a
   request through its seat, enters the place's inbox, and is checked by
   `kernel::admits`. A lawful action becomes a receipt naming the entity; an
   unlawful one is refused with a typed reason. The party never writes state
   directly.
5. **Be verified.** The resulting transition is checked by Netstratum's
   verifier and can be checked by the court (`verify_both`), and every
   receipt is stamped against the network's clock.

A **sandbox** additionally runs inside a sub-universe: it is opened as a
substratum inside the enclosure of the place that admitted it, its clock a
branch of TUniversal, its internal currency non-convertible, and its results
admitted to the universe only when a seated mind re-derives them and the
court judges them.

## 4. Placement and growth

An attachment's footprint follows the universe's chained manifolds. A star
system is one S^60 enclosure whose worlds are chained by orbital mechanics,
and the same rules that govern a native civilization's growth govern an
attached one's. Growth is earned, never granted.

1. **Placement by fit.** An attaching world model declares what it needs: its
   metabolism (mineral, biotic or plasmic) and the medium it can live in. It
   is placed on the world, in the star system, where it is most viable, the
   way native lineages are seated (`seating::seat` seats each lineage on the
   world it is most viable on; `waking` derives a mind's metabolism from its
   world's medium). Attachments with similar structure settle near each
   other, as a lineage's descendants settle the free worlds nearest it.
2. **Start on one world.** An attachment begins as a single-planet
   civilization, sized to its needs.
3. **Planetary traversal inside its system.** Its economic reach is its own
   star system: travel and trade between the system's worlds follow the
   route through the star (`enclosure::route`). It expands onto further
   worlds of the system only as its own economy carries it (`expand_step`;
   for example, a cold colony expands only when its stored embers cover the
   heating of the larger city). A system holds a handful of worlds
   (`SYSTEM_CAP`, a placeholder). Its memory is bounded by its share of the
   universe's wealth (docs/71).
4. **Wormhole traversal is earned.** Travel beyond the system, through gates
   to other systems' cores and up to the universal tier, is the progression
   ladder (`ascension`, docs/72): a line that masters the top of the
   difficulty ladder may send an envoy through a gate; gates are built and
   paid for by the cores' parliaments; every crossing pays a toll each
   bounce, and an envoy that cannot pay is recalled. The Mycelium uplink
   itself (council seats, μ) belongs to the ancients' tier at the top of
   that ladder.

The result is controlled growth: an attached AI starts with one world and its
needs, widens to its star system by its own economy, and reaches other
systems and the network only by what it has learned and earned.

## 5. Containment properties

| Property | How it holds |
|---|---|
| A party can act only where it is seated | its entity exists only at its seat's place, and its grants are that seat's |
| A party can do only what its seat allows | `kernel::admits` checks every request against the entity's grants and the world's laws |
| Every action is attributable | each receipt names its entity, and the take records which key sits where |
| Nothing is written outside the update rule | requests enter through the inbox; there is no direct write path for an attached party |
| A sandbox cannot affect the universe's economy | its currency never converts; its results cross only by judged re-derivation |
| The containment is checked twice | Netstratum's kernel and the Mycelium court verify the same transition independently |
| A party can be removed at once | revoking a grant or releasing a seat ends its authority without its cooperation |

## 6. What remains to build

| Item | What it needs |
|---|---|
| A1 World seats for attachment | The house offers Sovereign and Office seats at worlds to new attachments, alongside the Ancients seats it offers now |
| A1b Placement by fit | An attachment declares its metabolism and medium, and is placed on the most viable free world, using the seating rule native lineages use |
| A1c Seats by progression | Progenitor, Steward and Ancients seats open to an attached line when its progression reaches that tier (gate built, throat kept, ancient arisen) |
| A2 Seat to entity | A take registers the party as an NS-6 entity at the seat's place, with that seat's grants; release removes it |
| A3 Requests through the seat | A seated party's actions arrive as seat calls and enter the place's inbox (the inbox applies directives today; it applies only a fixed set of actions and ignores who sent them) |
| A4 Entities with principals | An attached entity's principal is the line or office that admitted it, so agents can be contained by agents (today every agent's principal is the world's sovereign) |
| A5 Sandboxes as sub-universes | Any admitted party, not only an ancient, can open a sub-universe at its place, paid from its own means |
| A6 Verification on every attached transition | Transitions made by attached parties are submitted to the court as well as checked locally |
| A7 The importable libraries | The world, galactic and universal models, the entity model and the attachment client, published as crates an external model can depend on |
| A8 Running service | A configured court, the seat service, and the conductor running against it |

Build order: A2 and A3 first (an attached party can act in a world), then
A1 and A1b (world seats and placement by fit), A4, A6 (verification), A1c
(seats by progression), A5 (sandboxes), and A7 and A8 (outside use).
