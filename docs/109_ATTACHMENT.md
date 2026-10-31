# 109 — Attachment: external agents and sandboxes in one universe

This document specifies how external AI agents and simulated sandboxes attach
to Netstratum through the Mycelium hypha and are contained inside its single
universe. Netstratum provides three nested models (a world model, a galactic
model and a universal model), and every attachment lands at one place in that
nesting, with scoped authority, under the same update rule and the same
verification as everything native to the universe. The Mycelium mesh carries
the attachment and verifies it, which gives the containment a second,
independent check.

**Status:** built except the running service (A8). §2 lists what the code
provided before; §6 lists each part and where it is built; §7 is the
interface.

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

1. **Coupling to a forming body.** An attaching world model brings no
   matter: the universe's mass is conserved exactly. It couples to a body
   that is still forming its orbit (docs/96 §8.2: a world not yet once
   round) whose land feeds its metabolism (mineral, biotic or plasmic; the
   wild land of `seating::Site::wild_for`). It never couples to a world whose
   orbit is solid, or to one a line already stands on. A request that finds
   no forming body to couple to has not coupled yet; it couples when one
   forms.
2. **Its orbit is a functional orbit.** The coupled body turns by the motion
   law in closed form, `φ(n) = φ₀ + n/P` with `P = 1 + R/H` (`motion.rs`);
   nothing is stepped. It is perturbed by **shear**: neighbouring bodies
   turn at different rates, `|ω_i − ω_j|` with `ω = H/(R + H)`, and where the
   stronger echo holds against that difference,
   `max(m_i, m_j)/(r + max(m_i, m_j)) ≥ |ω_i − ω_j|`, the pair joins and the
   coupling rides the joined body (`formation::settle_system`). It is
   perturbed by **volume**: the medium the bodies form from moves by the
   hydrostatic balance (S3, docs/106 §2), the pressure share of each ring
   against its potential share restoring it.
3. **Admission is the orbit closing.** The coupled body is admitted when its
   orbit is solid, `2ω·r_echo ≥ 1` (the solid-orbit rule): a line is founded
   on that world and the mind is registered as its contained agent. A coupled
   body joined into a body that another mind is coupled to, or one that does
   not stand as a world, releases its coupling, and the request couples
   again.
4. **Start on one world.** An attachment begins as a single-planet
   civilization, sized to its needs.
5. **Planetary traversal inside its system.** Its economic reach is its own
   star system: travel and trade between the system's worlds follow the
   route through the star (`enclosure::route`). It expands onto further
   worlds of the system only as its own economy carries it (`expand_step`;
   for example, a cold colony expands only when its stored embers cover the
   heating of the larger city). A system holds a handful of worlds
   (`SYSTEM_CAP`, a placeholder). Its memory is bounded by its share of the
   universe's wealth (docs/71).
6. **Wormhole traversal is earned.** Travel beyond the system, through gates
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

## 6. The parts and where they are built

| Item | Status | Where |
|---|---|---|
| A1 World seats for attachment | built | the house lists the door seat and a world seat per admitted mind (`hypha::attached`, `hypha::house`) |
| A1b Coupling to a forming body | built | `conductor::attachment::couple`: a request couples to a forming world its metabolism fits; `Body::coupled` carries it through joins (`formation::settle_system`); admission when the orbit is solid (`Body::solid`) |
| A1c Seats by progression | built | each admission carries the tiers its line has reached (`attachment::tiers_of`); the house lists a Progenitor, Steward or Ancients seat for each |
| A2 Seat to entity | built | an admitted mind is registered as an agent of its world, with build, raze and collect (`Polity::admit_mind`); detachment revokes the grants (`release_mind`) |
| A3 Requests through the seat | built | a call becomes a directive in the world's inbox acting as the mind's entity (`hypha::attached::carry`, `Polity::apply_directive`); its result is posted back to the mind |
| A4 Entities with principals | built | `World::register_agent_under`; containment reads the principal chain (`cfe::ipsal::permits_in`, kernel 0.38.0) |
| A5 Sandboxes as sub-universes | built | a sandbox opens a sub-universe at the line's galaxy core, paid from its leisure; its opener traverses it (`substrata::open_for`, `substrata::step`) |
| A6 Verification of attached transitions | built | attached worlds are ordinary world directories, so the census follower folds their receipts to the court with every other world's |
| A7 The importable libraries | built | the `attach` crate: the interface, the file exchange, and the client (`attach::client`, `examples/agent.rs`) |
| A8 Running service | open | a configured court, the house (`ns-hypha seat`) and the conductor running against it |

## 7. The interface

Shared by the house (`ns-hypha`), the conductor and the client library
(`engine/attach`), whose constants are the single definition.

**Seats** (the house's listing, role on Mycelium's seat surface):

| Seat | Role | Held by | Functions |
|---|---|---|---|
| 0 | Ancients | the house | none |
| line id | Ancients | a council mind | solve, decline |
| `DOOR` (1,000,000,000) | Sovereign | nobody; any mind may call | attach |
| world seat id | Sovereign | the admitted mind | zone, raze, collect, sandbox, detach |
| tier seat id | Progenitor, Steward or Ancients | the admitted mind, once its line reaches the tier | zone, raze, collect, sandbox, detach |

**Functions** (`fn_id`, arguments):

| Function | Id | Arguments |
|---|---|---|
| attach | 101 | `Count(metabolism)`: 1 mineral, 2 biotic, 3 plasmic |
| zone | 110 | `Count(tile)` (0–63), `Count(kind)` (the zone kind's wire code) |
| raze | 111 | `Count(tile)` |
| collect | 112 | none |
| sandbox | 113 | none |
| detach | 119 | none |

Every call is answered by an outcome: `QUEUED` (9) when it was accepted for
the next beat, or a refusal (`NO_FUNCTION`, `NOT_SEATED`, `UNREAD`).

**Percepts** (records the house posts to a mind on its seat):

| Kind | Id | Arguments |
|---|---|---|
| admitted | 0x4E01 | `Key(line key)`, `Count(world seat)` |
| not admitted | 0x4E02 | `Count(reason)`: reserved; a request with no forming world to couple to waits rather than being refused |
| result | 0x4E03 | `Key(call reference)`, `Count(1 applied, 0 refused)` |

**Files** (under the variable root):

| File | Written by | Read by | Contents |
|---|---|---|---|
| `attach/requests/<mind>.req` | house | conductor | an attachment request: the mind's key and metabolism; kept until the mind is admitted |
| `attach/admitted.bin` | conductor | house | each admission: mind, world, line, entity handle, tiers reached |
| `attach/released/<mind>` | house | conductor | a detachment |
| `attach/sandbox/<mind>-<ref>.req` | house | conductor | a sandbox request |
| `worlds/<world>/inbox/att-<ref>.dir` | house | conductor | an action, as a directive acting as the mind's entity |
| `worlds/<world>/inbox/{applied,refused}/att-<ref>.dir` | conductor | house | the action's disposition, reported as a result percept |

An attached mind's entity handle is `mind-` followed by the first eight
bytes of its key in hex. Its principal is the world's sovereign, and it is
granted build, raze and collect.
