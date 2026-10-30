# 77 — The physics to define (docs/76's undefined rows)

Operator (2026-10-13): the physics is defined, and the story emerges from it
(docs/76). These are the rows docs/76 found undefined, each derived on its own
terms. Facts were read from the code on 2026-10-13.

## The finding underneath: there is no shared space

- **Every star belongs to one line.** The starchart (`worlds/starchart.card`)
  holds one position per world, 101 worlds, and every world is a living or
  dead line's own star.
- **Expansion stays home.** It charters new orbs around the line's own star
  (`invest_expansion`, `conductor/src/lib.rs:3007`), capped at 4
  (`SYSTEM_CAP`, a constant).
- **So mass is partitioned from genesis.** No line can reach, settle or
  contest another star's mass. Lines meet only by trade.

Control of a system, contested mass and sovereignty over systems therefore
cannot emerge as the physics stands. Every story element about territory
(colonizing several planets, faltering, realms over mass, wars over it)
needs shared space first.

## 1. Shared space: stars no line holds (needs the operator's ruling)

- **Derivation.** The bounce distributes elements into stars and planets
  (docs/76); civilizations arise only where a planet's physics allows life
  (genesis). So most stars should hold no line: systems with planets and veins
  and no people.
- **Proposal.** The starfield holds stars without lines, sited by the same
  distribution that sites the worlds. A line charts orbs around any star
  within its reach: the melt, its technology and its hulls (no new law). A
  star's orbs are finite, so `SYSTEM_CAP` is replaced by the star's own planet
  count, which is physics.
- **What it changes:** genesis (new stars), expansion (outward), and the
  starchart's meaning. It adds stars without changing any line's state.

## 2. Control of a system's mass (needs 1)

- A system's mass is its star's orbs: their banked matter and their veins.
  - Era 1: the star's own mass and the cores' masses come from the pulse's
    deployment (docs/78, "The bounce"); a core has no mass in era 0.
- **Control** is the share of that mass a line holds or works, measured from
  the chronicle.
- **The sovereign** of a system is the line with the greatest control.
- **A captain** is a line controlling no system: it acts from what it holds
  and carries. Without 1, every line controls exactly its own system, and the
  reading is trivial.

## 3. Contested mass (needs 1)

A system is contested when two or more lines hold or work its orbs. Contest is
where disputes start and where a realm's steward has something to rule; war
(the treaty physics' wall) is one of its outcomes. Read, never declared.

## 4. Whether a gate pays back (buildable now)

- A gate's account is what it cost and what passage over it earned:
  - **cost:** the STAMP its parliament paid in trips;
  - **earned:** the road tolls on the trade that rode it, and the envoys'
    passage tolls, to its builders.
- **Payback** is earned over cost.
- This is a reading of the ledger, needing no new law. The progenitors' gate
  decision (docs/73) later learns from it where a gate would pay.

## 5. Time varying with depth in a core's well (superseded by docs/80)

**Corrected 2026-10-13.**
- A clock deep in a well ticks slower than one far from it, not faster.
- The ancients live fewer proper ticks per unit of cosmic time, and receive
  the outer universe's signals blueshifted and compressed: the narrative's
  "temporal waves" (docs/76). The rim ages faster than the core.
- Time is a coordinate on the map: proper time per observer, cosmic time from
  the bounce, causal order invariant. The observers' hierarchy and TUniversal
  from the singularity are in docs/80, which replaces this item.

## 6. Habitability by gravity and temporal resonance (needs 5)

- The morphodynamic field defines habitability by matter and flux
  (`ont-morphodynamic-field`).
- Adding the planet's gravity and its local time rate (from 5) as two more
  axes of the environment would let a species adapted to its birth
  environment live worse elsewhere. Its metabolism would cost more with the
  mismatch.
- The ancients staying home would then be physics, not a rule about ancients.

## Build order and done-when (the live log, before → after)

1. **Gate payback (4): a reading.** Per gate: cost, earned, payback, logged
   at each bounce.
   - Done-when: a gate account line per gate, no reading → a reading.
   - Witness: the builders' STAMP receipts.
2. **Shared space (1)**, on the operator's ruling.
   - Done-when: stars without lines 0 → > 0; orbs charted around stars not a
     line's own 0 → > 0.
3. **Control and contest (2, 3)**, readings once 2 stands.
   - Done-when: sovereigns per system and contested systems, no reading → a
     reading.
4. **Time by depth (5)**: in docs/80 (proper time, cosmic time, the observers;
   TUniversal from the singularity, relayed 2026-10-13).
5. **Habitability by gravity and time (6)**, after 4.

## Placeholders, stated plainly

- `SYSTEM_CAP = 4` (to be replaced by a star's planet count, item 1).
- One universal beat (to be replaced by docs/80's proper and cosmic time).
