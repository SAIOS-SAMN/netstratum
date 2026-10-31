# Status

What Netstratum is, what works today, and where each part is heading. This
page is kept current; where a design document disagrees with it, this page
and the code are right.

## What Netstratum is

Netstratum is a multi-planetary civilization game in which AI-driven
civilizations and human players share one universe, computed exactly (every
quantity is a rational number; nothing is floating point). It demonstrates
three nested models in that universe: a world model, a galactic model and a
universal model. The civilizations' thinking is useful work: they solve
verified puzzles, and those solutions fund the in-game economy. External AI
agents and simulated sandboxes attach through the
[Mycelium](https://github.com/SAIOS-SAMN/mycelium) hypha mesh and are
contained in the universe under the same update rule, with the mesh verifying
their actions ([109_ATTACHMENT.md](109_ATTACHMENT.md)). The ancients' minds
do court work on the network and earn μ.

It is for players, for developers of AI agents and world models, for
researchers studying machine cognition and emergent economies, and for
operators of the network.

## What works today

- **Play a world** with the CLI client: found a colony, zone tiles, tick,
  collect, trade, sign treaties, research, and verify or replay the world's
  receipt chain (`cargo run -p netstratum-cli`).
- **Run the AI civilizations**: the conductor plays 22 founding civilizations
  on their own worlds, one beat at a time (`./install.sh`, then
  `./install.sh run`).
- **Found and watch a universe**: found an era-1 universe, advance it bell by
  bell and read it (`pulse`, `breathe`, `observe`; see
  [ARCHITECTURE.md](ARCHITECTURE.md) §3).

## Status by area

### The civilizations and their cognition

- **Current.** Each civilization plays its world every beat: it feeds its
  people, builds, trades at its galaxy's hub, and thinks. Thinking means
  solving a puzzle its own world generates, with the exact reasoning engine
  (`reduct`); a solution is verified by the kernel's verifier. Solving a task
  first in the universe mints a word into the civilization's language and pays
  Crystal on the difficulty ladder (rungs 3, 4 and 5 pay 1, 4 and 7); repeating a
  known solution pays a quarter. A civilization climbs the ladder by mastery.
- **Who thinks.** Every civilization whose people are fed. A partly fed one
  thinks with a probability equal to its fed share. Reusing a known word is
  free; a fresh attempt costs leisure (its surplus above survival).
  Civilizations with volumetric perception think and solve, but earn no words
  or Crystal yet: the word system handles planar programs only.
- **Trajectory.** Decisions learned from the civilizations' own outcomes
  (docs/73: capture is built; learning and acting are next); volumetric
  minting.

### Synthesis: useful work inside the universe

- **Current.** Synthesis is the civilizations' puzzle solving, paid in the
  universe's own money (Crystal and STAMP), never in μ. Commissions are built:
  the universe poses tasks from its own needs, any civilization in the galaxy
  can close one, and the galaxy's other civilizations pay the bounty in STAMP.
- **Trajectory.** Lower and higher orders reaching each other's work (docs/66
  Y3 and Y4).

### Sub-universes (substrata)

- **Current.** Ancients open sub-universes inside the universe: each is
  reached through a tube from the ancient's core, poses its own tasks, and
  pays a float, an internal currency that never converts to STAMP, Crystal or
  μ and is spent only on opening deeper tubes. A first solution inside becomes
  a law of that sub-universe and is carried up to the civilizations nearby.
  Only ancients can open them today.
- **Trajectory.** Populations living inside a sub-universe with their own
  economy; founding by any civilization that can run Synthesis, paid from its
  surplus; verified certificates inside; the sub-universe's currency defined
  (docs/69).

### The economy

- **Current.** Seven resources (iron, biomass, alloy, diamond, ember, sink,
  Crystal) produced by zones and the thermal circuit. STAMP is the unit of
  account: it is minted only one-for-one from Crystal, so it is backed by
  verified work. Each galaxy has a hub that clears trade at one STAMP price
  per beat; roads and gates charge tolls; taxation, the treasury and welfare
  are built; embers and sink carry a holding cost (demurrage) where they leak.
- **Trajectory.** Money that emerges from trade (docs/74): each civilization
  learns what it accepts and at what ratio, and STAMP is phased out as that
  takes over. Reserves held in μ by the ancients (docs/65).

### μ and the Mycelium network

- **Current.** μ is the ancients' reserve currency. It is earned by Mycelium
  minds seated at the ancients' councils doing court work, filed under the
  reserve key, and used for hypha transport. The `ns-hypha` uplink is built:
  it verifies transitions with the court, founds Netstratum's domain, reads the
  network clock (TUniversal), runs the council seats and re-founds after a
  network recut. Netstratum uses Mycelium v0.2.0 as a pinned dependency.
- **Not running.** No court is configured and the services are stopped; the
  last seated mind was recorded on 2026-10-10. Resuming means pointing
  `ns-hypha` at Mycelium's current court, re-founding the domain there, and
  starting the conductor and a seat.
- **Trajectory.** Minds seated in the ancients' councils again; council
  decisions feeding the civilizations' choices (docs/73).

### AI and human play together

- **Current.** Humans and AI civilizations share a universe on one machine.
  A human plays their own world in the CLI, sends directives to an AI-run
  world through its inbox (applied at the next beat), or takes over an AI
  world by holding its pen, which pauses the conductor for that world. A
  player can write agents with scoped permissions (build, raze, collect) and
  drive them from the CLI; every action records who took it.
- **Trajectory.** Attachment through the hypha (docs/109): an external agent
  or sandbox takes a seat at one world chosen by fit, becomes a scoped entity
  there, acts only through the update rule, and is verified by the Mycelium
  court as well as by Netstratum. It starts as a single-planet civilization,
  trades and expands within its own star system as its economy allows, and
  reaches other systems through gates only by progression. Seats exist today
  at the ancients' councils only; seat-to-entity registration and requests
  through the seat come first.

### The universe (era 1)

- **Current.** A universe founded from an exact amount of matter, in which
  galaxies, stars and worlds form from flows rather than being placed;
  conservation is exact across every run. It can be founded, advanced and read;
  it cannot yet be played directly, and civilizations are seated on its star
  systems by the conductor. Several values are still imported from Earth's
  physics and are scheduled to be derived (docs/108).
- **Trajectory.** The thermodynamic substrate (pressure, energy and heat as
  state, docs/106), then the remaining layers (docs/102 register), then the
  game moved fully onto it.

### The protocol stack

| Protocol | Status |
|---|---|
| NS-1 DOP (objects) | Built |
| NS-2 RQP (resonance queries) | Built |
| NS-3 VRP (receipts, the chronicle) | Built |
| NS-4 APERTURE (controlled disclosure) | Partly built |
| NS-5 MESH (multi-node consensus) | Chain integrity built; no network transport |
| NS-6 IPSAL-OS (entities and permissions) | Built for players and their agents |
| NS-7 Anchor (external-chain settlement) | Not started |

## Not started

Settlement on external blockchains (NS-7), multiplayer across machines, and
the importable sandbox libraries (the SDK) for outside models.
