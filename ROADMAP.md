# Roadmap

The order of the work is held in GitHub: the milestones v0.1.0–v0.6.0 and
the project [Netstratum](https://github.com/orgs/SAIOS-SAMN/projects/3), by
the rules in [docs/BUILD_OUT.md](docs/BUILD_OUT.md). The principles and the
register's mapping stay in [docs/PLAN.md](docs/PLAN.md).

What Netstratum is, what works today, and where each part is heading. This
page is kept current; where a design document disagrees with it, this page
and the code are right.

## Now (2026-11-17): where to pick up

**State**
- `main` is clean, with no open PRs.
- Mycelium is pinned at main 751aa40e (manifest v4).
- On the mesh (127.0.0.1:8500–8502, integ-v4), the domain da34b0b5 is
  re-founded on v4:
  - grant 4738a7da, 1495 entries, applied at u 3, confirmed from the court's
    side;
  - the reserve holds the first v4 delivery, half the allotment.
- The table: a 2^40 founding reaches weight **1708** (merger channel, E79,
  #231), up from 1508.

**Ruled (operator 2026-11-17):** seed 7's mouth `57ff54ac…849f8b` is
traded first on a separate test domain (#240). The real domain trades from
v0.6.0, by the trajectory below.

**Next, in order**
1. **#233, the founding's cost.** The founding takes about 15 minutes at
   2^40. `cascade`'s filter is all pairs over shells. Count its work, then
   restate it on ranges if the derivation allows.
2. **#213, the table** (docs/97 §15):
   - V13, the universal core read on the 1708 table against the old one;
   - V1–V3, the kinds on (Z, Q, pattern), whole members with per-plane
     phases;
   - V8, seeds forming over rings.
3. **#206 step 4:** the spin ledger read before and after the composed
   turning.
4. **#220 through #158:** the jetstream as volume on the R-partner planes,
   which retires `Mote::vacuum`.
5. **#198 and #207:** the event reader. T moves only at crossings, and each
   echo is read at the mote's own coordinate.

**Across the seams:**
- the core tube is #109 ↔ xylarium #281 and mycelium #327;
- the table's planes are xylarium #128, #296 and #297;
- the manifest is mycelium #359.

## The trade with Xylarium: the trajectory (2026-11-17)

Matter and μ crossing the universal core's tube between netstratum and
Xylarium's seed 7. These are two universes with inverse nucleons: the tube's
image swaps the poles, `(Z, Q, R) ↦ (Q, Z, −R)`, on every component. Rules:
- **No dimension is lost on transfer** (operator 2026-11-17, docs/123 §3b):
  the whole state crosses, and a face holds what it cannot read.
- **Nucleons are conserved.**
- **μ follows its nuclei** (Mycelium #360, #370).

The terms are a draft bilateral treaty from Xylarium (xylarium-uplink
`docs/treaty-seed7-netstratum.md`), tracked here as #239.

**The steps, each with its milestone:**

| Step | What | Milestone, issue |
|---|---|---|
| 0 | **The trial, done 2026-11-17:** the test domain 44ca2f01… accepted seed 7's trial mouth and signed its taken motion, the image of 1 × (Z 1, Q 1). The court **sealed** the traverse at h 7608, with a zero fee. The real domain da34b0b5 does not trade until its crossing is physics. | #240 (closed) |
| 1 | **The kinds on their own dimensions:** (Z, Q, pattern) with per-plane phases (V1–V3), so inverted nuclei are kinds, not collapses | v0.1.0, #213 |
| 2 | **Time moves at crossings:** the event reader, with each crossing writing `T ↦ T − 2(H − R)` | v0.3.0, #198, #207 |
| 3 | **Inbound:** arrivals enter the universal core's throat. The throat's keep formula (taken ≤ released) and its tax are derived. Unread components are held as carried state. | v0.6.0, #109 |
| 4 | **Outbound:** part of the throat's release leaves through the mouth, μ with it | v0.6.0, #109 |
| 5 | **The round trip and homecoming:** the composite motion closes, the windings resonate, the light ledger balances on both faces, and every component returns exactly | v0.6.0, #109 |
| 6 | **The treaty:** amended, then countersigned by the union reserve (`ACT_LAWS` mode 8), on the operator's word | v0.6.0, #239 |

**Why this order:**
- Step 1 also fixes netstratum's own proton-rich table (#203), and step 2
  is the time model every law needs.
- So the trade is built on the universe's own physics, not on placeholders.
- Until step 3, a crossing on the real domain would book nuclei the
  universe never receives.

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
their actions ([109_ATTACHMENT.md](docs/109_ATTACHMENT.md)). The ancients' minds
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
  [ARCHITECTURE.md](docs/ARCHITECTURE.md) §3).

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
  reserve key, and used for hypha transport. The `ns-embassy` uplink is built:
  it verifies transitions with the court, founds Netstratum's domain, reads the
  network clock (TUniversal), runs the council seats and re-founds after a
  network recut. Netstratum uses Mycelium v0.2.0 as a pinned dependency.
- **Not running.** No court is configured and the services are stopped; the
  last seated mind was recorded on 2026-10-10. Resuming means pointing
  `ns-embassy` at Mycelium's current court, re-founding the domain there, and
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
- **Attachment (built, not yet running).** An external agent or sandbox
  attaches through the hypha (docs/109): it calls `attach` on the house's
  door seat with its metabolism and couples to a world still forming its
  orbit whose land feeds it. The coupling rides that body through its joins
  with its neighbours (the shear) as the medium settles by the hydrostatic
  balance (the volume); when the body's orbit is solid, a line is founded on
  that world and the mind is registered as its contained agent.
  It acts only through its world seat, as directives that pass the update
  rule, and each result is posted back to it. It starts as a single-planet
  civilization, expands within its own star system as its economy allows,
  and reaches other systems through gates only by progression; tier seats
  open as its line reaches them. Agents can contain agents. A sandbox runs as
  a sub-universe at its line's core. The client library is the `attach`
  crate. It runs once a court is configured and the house and conductor run
  against it.
- **Trajectory.** The running service; multiple machines sharing one world
  (NS-5, the mesh).

### The universe (era 1)

- **Current.** A universe founded from an exact amount of matter, in which
  galaxies, stars and worlds form from flows rather than being placed;
  conservation is exact across every run. Its medium has a restoring force:
  pressure is state, and neighbouring rings are pushed back toward the
  hydrostatic balance of pressure against potential, radially and around
  each shell's circles (docs/106 S3, docs/110). Each mote's orientation is one
  generator over six turning planes, composed whole each bell, so its planes
  turn into one another instead of as a flat torus (docs/117 §11.7; stars
  and worlds follow). It can be founded, advanced and read;
  it cannot yet be played directly, and civilizations are seated on its star
  systems by the conductor. Several values are still imported from Earth's
  physics and are scheduled to be derived (docs/108). A body's state is on
  Cl(13,1), the one signature shared with every kernel: its rotor on the
  base, the S¹¹ fibre read (docs/136). Matter is moved by the plasma layer:
  each kind's bound share at its place and heat, the ionized share of every
  kind moving as one parcel, and the bound share's drift toward it holding
  light matter in the galaxies (docs/134). The star core's cut keeping the
  hydrogen its binding brings is the open law. Rates compose as versors (docs/138): boosts on different planes turn the
  plane between them, built in `turning`; the spin ledger as one bivector is
  next (#206).
- **Trajectory.** The pulse as one expanding wavefront echoed only at
  surfaces, contained by the firmament until bonded collections of its
  matter escape, with the zero galaxy first and every body's extent and
  throat read by one law (docs/111, EQUATIONS §8); the thermodynamic
  substrate (pressure, energy and heat as state, docs/106), with each form
  of energy on its level and named by nature's term (docs/124: heat is
  motion out of step; the star core's heat from contraction, docs/122
  step 5); then the
  remaining layers (docs/102 register), then the
  game moved fully onto it.

### The protocol stack

| Protocol | Status |
|---|---|
| NS-1 DOP (objects) | Built |
| NS-2 RQP (resonance queries) | Built |
| NS-3 VRP (receipts, the chronicle) | Built |
| NS-4 APERTURE (controlled disclosure) | Partly built |
| NS-5 MESH (multi-node consensus) | Chain integrity built; no network transport |
| NS-6 IPSAL-OS (entities and permissions) | Built for players, their agents, and attached minds; agents can contain agents |
| NS-7 Anchor (external-chain settlement) | Not started |

## Not started

Settlement on external blockchains (NS-7) and multiplayer across machines.
