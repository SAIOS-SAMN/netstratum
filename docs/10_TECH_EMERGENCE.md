# TECH EMERGENCE — the crystallized tree (TECH-1 spec, design stage)

**Status:** DESIGN — lands as TECH-1 after BOT-THERM; the landing that opens
SINK-1's deferred research gate. Constants are placeholders.

**The ruling (operator, 2026-07-27):** no static external tech tree — a
hardcoded ladder makes every civilization climb the same line. Technology
CRYSTALLIZES: nodes emerge deterministically from each world's own chronicle
under sustained pressure or surplus. No two worlds need share a tree.

## 1. The dual model

- **Kernel (fixed, already landed):** the physics laws (05) and the forge
  rung ladder (conductor: rung 3 smelters, rung 5 synthesizers). The forge
  IS the research work — solves fund discovery. Kernel nodes are the only
  hardcoded ones.
- **Emergent nodes:** crystallize when a condition holds over a sustained
  window, measured FROM EXISTING RECEIPTS wherever possible (famine
  duration = now − strike stamp; vent pressure = `entropy_vented` delta per
  year; surplus = banked vs the heating bill). No shadow counters unless a
  condition is underivable — then a committed counter with receipts.

## 2. The laws

1. **Seeded synthesis (SEAL B):** a node's identity, name, cost, and exact
   multiplier derive from
   `BLAKE3(discovery receipt digest ‖ world ‖ condition kind)` — the
   chronicle root at the moment of crystallization. Replay re-derives the
   identical node at the identical tick. No runtime randomness, no
   external calls, no LLM in the loop.
2. **Discovery is a closure:** `TechDiscovered` (permanent tag 39) —
   `LE64(orb) ‖ node_id[32] ‖ condition u8 ‖ modifier u8 ‖ ratio (magnitude) ‖ name utf8`.
   Costs and magnitudes are exact rationals. Tag 40 RESERVED for
   `TechLicensed` (blueprint trade over TRADE-1 — a later landing).
3. **Modifiers touch only landed parameters:**
   `HeatingReduction(Ratio)` · `EncasementBoost(Ratio)` ·
   `SinkBoost(Ratio)` · `RadiatorBoost(Ratio)` · `DemurrageEase(Ratio)`.
   Applied multiplicatively in the tick's heat law; the orb carries its
   discovered node list (codec entry, byte-exact). `TradeRangeExpansion`
   REFUSED for now — no range mechanic exists.
4. **Names are renderings:** deterministic word-table composition from the
   node hash (the journal precedent) — flavor at the surface, hash as the
   identity underneath.
5. **The gates open here:** TECH-1 is the research system SINK-1 §1.4
   awaits — sink capture's research gate binds to a crystallized node;
   the throttle-α policy action lands beside it.

## 3. Condition table (v1, placeholder thresholds)

| condition (derived from receipts) | crystallizes | modifier |
|---|---|---|
| joule famine survived ≥ 8 years total | Cryo-Resonant Insulation | HeatingReduction 3/4 |
| overheat held ≥ 4 consecutive years | Overflow Discipline | EncasementBoost 5/4 |
| vault sink banked ≥ threshold with zero demurrage | Lattice Deepening | SinkBoost 5/4 |
| high vent ratio sustained (vented > encased, 4 years) | Radiator Doctrine | RadiatorBoost 5/4 |
| surplus held (banked ≥ 4× heating bill, 8 years) | Long Stores | DemurrageEase 1/2 |

One node per condition kind per orb (idempotent); prerequisites = the
condition's own history, not a parent-pointer web (parents emerge naturally:
you cannot survive 8 famine-years without first being cold).

## 3a. TECH-1a LANDED (2026-07-29) — the discovery closure + the solar leap

The first slice: `TechDiscovered` (tag 39) wired byte-exact per §2.2's
layout, and the union's KERNEL node landed — **the solar leap** (condition
0, 12_UNION §5a): a world's first thermal opt-in receipts that its people
now know their sun. The opt-in IS the leap's receipt — once per world,
never per orb; node id seeded from the kernel name + the discovering orb
(SEAL B, replay re-derives it); `knows_its_sun()` is the reading that
THRESHOLD-1 licensing and the shield gate on. Pre-solar is a lawful state:
an unsited world lives whole and simply carries no leap. Evidence: four
witnesses (once-per-world, replay+resume, byte-exact round-trip, pre-solar
lawful).

## 3b. TECH-1b LANDED (2026-07-29) — the crystallized tree

The emergent tree: at every year-voice boundary each orb's chronicle is
weighed against the §3 condition table, exactly — famine spans and
overheat spans summed from strike/relief receipts, vent ratios from
sealed `VentSettled` vs collected joule yield, surplus and lattice
readings from committed state. A condition that holds crystallizes its
node ONCE (idempotent per orb per condition); the node id is the seeded
synthesis, `BLAKE3(chronicle root at crystallization ‖ orb ‖ condition)`
— replay re-derives the identical node at the identical tick. All five
modifiers apply multiplicatively at their landed parameters: heating burn
×3/4 (insulation) · encasement throughput ×5/4 (discipline) · sink
capture ×5/4 (deepening) · radiator ×5/4 (doctrine) · demurrage rate ×1/2
(long stores, threaded through every touch and affordability read).
Measured: the doctrine's boosted radiator closes a heat_net-1/2 world's
budget exactly and the overheat CLEARS, while the same node only halves a
heat_net-1 world's fire — crystallized nodes meet each world where it
burns; no ladder ordered any of it. `TechLicensed` (tag 40) landed at
PLANNER-1. The deepening/long-stores arc witnesses LANDED 2026-07-30
(coder-03, `world_tech_arcs.rs`): the deepening earned in a fueled
vault (banked sink 119994/1000 at the boundary, demurrage-free) with
the composition law measured — capture = min(headroom, boosted
lattice) still reads the headroom, so the boost binds only beside a
doctrine-boosted radiator, never minting sink past the physics; the
long stores earned on four apertured winters (K-band keeper, leaking
and sub-mesic) with the ease measured ON THE RECEIPT — 32 ticks at
the halved flat rate settle exactly 1/2. Two laws inscribed by
measurement along the way: a vault colony runs on FUEL AND STEEL (the
array's build cost + upkeep struck iron famine at tick 41 in the
first draft — the ECON-J burn bites beside the lattice), and the
boundary fires during the call AFTER the round number (the clock
advances at tick end). **Still open, recorded:** the throttle-α
policy action (steward side); word-table naming beyond the §3
canonical names.

## 3c. The naming law LANDED; the succession law LANDED (2026-07-30)

**Naming landed (§2 law 4, operator ruling):** a node's name is never a
rung label. `tech_node_name` composes it from word tables — the
condition fixes the CONCEPT (insulation · discipline · deepening ·
doctrine · stores), the node hash draws the VOICE — so two lines
crystallizing the same condition speak of it differently ("keening
doctrine" on one world, "patient doctrine" on another). The hash is the
identity; the name is the line's own tongue. You cannot stack names on
rungs, because no two lines share a chronicle root.

**NOVEL-1 LANDED (2026-07-30, coder-03) — the succession law.** When a
line expires and another takes its place — the intelligent mushroom
succeeding the intelligent elephant — the successor does not inherit a
tech tree, it inherits a CHRONICLE. The **succession deed**
(`LineSucceeded`, tag 59; CLI `succeed <line>`): the derelict door,
never a reset button — it refuses PURE while any orb stands unfamished
(the 07 §3 triage in its v1 cascade shape: EVERY orb famished; richer
interrupted-vs-collapsed triage recorded as deferred). On the deed the
living tree drops to the world's KERNEL — the solar leap is the
world's own fact, not the line's — and every other node leaves the
tree while standing in the chronicle forever. The successor's
conditions then evaluate over receipts that include the predecessor's
whole history (the deposit sheet walks the one chronicle — no new
crystallizer code was needed; the engine read this way from TECH-1b's
first day), and its node ids seed from roots that CONTAIN that history
plus the deed — NOVEL by construction: shaped by the old famines and
vents, named in the successor's own voice, never overwriting a node
the predecessor held. The pre-existing is changed, not replaced.
Measured (`novel_succession.rs`, 4 witnesses): a living line refuses
and orphans nothing; the predecessor's insulation stands while the
successor mints a DIFFERENT insulation from the inherited famine span
with no new famine-years paid; a second succession mints a third node,
no root repeating; the deed round-trips byte-exact; replay and resume
carry the boundary.

## 3d. TERRA-1 LANDED; SCIENCE-1 SPECCED (2026-07-30)

**TERRA-1 (landed) — the gifted crown.** Terraforming is condition 6:
the highest technology a civilization can hold, and the only one that
is NEVER crystallized locally — it licenses down from tier 0
(`license_tech`, tag 40) and ONLY onto a complete **base-7 stack**: the
solar leap plus all five crystallized conditions, earned or granted
(`BaseSevenUnfinished` refuses an unfinished stack). Under the crown —
and only under it — the orbit that "does not move" moves:
`World::terraform` re-seats a sited planet's albedo and orbit
(receipted as a fresh `PlanetSited`; replay takes the last seat), and
the world crosses substrate bands. No new wire tags. The TUniversal
calibration rides 11 §1: base-1 daily · base-7 weekly (the stack's
rhythm) · base-60 yearly.

**SCIENCE-1 (open) — speciated puzzle science.** A line's puzzle tree
is its OWN science: perception already routes the solver (volumetric
lines forge volume tasks, planar lines forge grids — landed), and
metabolism must route RELEVANCE — a plasmic entity's tree carries no
forge-of-iron (smelting is meaningless to a current sheet); its
crystallizations run its own conditions. Foreign capabilities come by
the **joint research venture**: incompatible sciences cross their
outputs through the landed machinery composed — a council seals the
venture, an aperture carries the exchange, a licensed leap (tag 40)
lands the foreign node. A plasmic line that NEEDS smelted alloy joins
forces or trades for it; it never fakes the tree. Landing needs: the
metabolism relevance table (which conditions/industries exist per
science), conductor build-step routing, and the venture deed
(council-bound multi-line licensing).

## 3e. WARDEN-1 — the elder world (specced 2026-07-30; 1a LANDED 2026-07-31)

**WARDEN-1a landed** (tags 74/75): `ward_elder` seats the reverse
shield — every aperture refuses both directions (`TheWardHolds`)
until `lift_ward(claimer, depth)` opens the depth gate (ceiling 5,
placeholder; depth is the mirror's solves reading passed by the
caller); the lift names its claimer forever; deeds run in order,
typed. OPEN as WARDEN-1b: the elder's genesis seeding (the roster
`elder` flag), the standing forge task at FORGE-2a saturation, and
the template grant through the council gate. The original spec:

The Guardian shape, lawful: ONE dormant precursor star per aeon,
chartered at genesis but SHIELDED IN REVERSE — it petitions no one,
trades nothing, and refuses every aperture until its gate is solved.

- **The gate is the depth law.** The elder world holds a standing forge
  task at the ladder's ceiling (the FORGE-2a vector at full
  saturation). A visiting line proves the solve through the landed
  proof-of-solution path (`record_solve`) — the ARC discipline, no
  combat, no dice: the Guardian is a puzzle, not a death machine.
- **The prize is a licensed leap, council-gated.** Solving unlocks
  MASTER TEMPLATES — precursor tech nodes licensed through PLANNER-1's
  gate like every leap (the solar leap stays never-licensed; the elder
  world teaches no one their own sun). Templates are sealed grants the
  solver must still unlock (the unknown-leap law).
- **The elder's chronicle is precursor VRP** — receipts from before the
  aeon (an annexed sheet, the 07 §3.1 door read passively): its record
  IS the reward's provenance, non-falsifiable by construction.
- **Lands with:** the elder charter at genesis (roster-seeded, station
  none — the elder holds no seat and no vote), the standing-task gate,
  the template grants, vectors. Constants placeholder.

## 4. Deferred, recorded

- **Synergy/shared nodes** (co-discovery across worlds): needs BOND-1's
  cross-world link — separate chronicles cannot co-sign a discovery yet.
- **Tech licensing/diffusion** (tag 40): rides TRADE-1 + escrow (01 §7)
  once blueprints have a transfer law.
- **Ideology input:** post-IDEO-1, the 09 cohesion counters may enter the
  condition table — read as landed state, never invented here.
- **Journal integration:** discoveries surface on the Terminal Wall as
  Milestone entries (mirror-side rendering, JOURNAL-1's law — automatic
  once the receipts exist).
