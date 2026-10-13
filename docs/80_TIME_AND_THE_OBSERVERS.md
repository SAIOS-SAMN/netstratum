# 80 — Time, the observers, and TUniversal from the singularity

Operator (2026-10-13): in a deterministic universe time is a coordinate on
the map: to traverse space and time is to produce the value of time as a
coordinate, and the separation between two times is a distance. Observers
must be ordered by the hierarchy so the universe has one unified timeline.
TUniversal is the quantum drive of the universe's core singularity, which
provides the uplink to the hypha, with a dedicated lane for netstratum's
quantum state, aligned on the epicenter of the spherical universe.

Status: scoped and relayed 2026-10-13; not built. It belongs to era 1
(docs/78) with the quantum layer (docs/79).

## Facts (read from the code)

- **TUniversal's point** (mycelium `chronometry/tuniversal/point.rs`, 78.152):
  `origin / h : S : G : T : H : K`.
  - `origin` is a mesh's genesis hash and epoch; "two points of two origins are
    never compared".
  - `h` is the keeper court's head height.
  - `S:G:T:H` is the court's cumulative measured change per lane: S boosts and
    dilations, G hue moves, T null fluxes, H turns.
  - `K` counts the orbits closed.
  - The wire size is fixed at four lanes (`LEN = 32+4+8+4·8+8`).
- **Netstratum reads the fleet's point** (origin 6a86ad4a…/0x93), relayed by its
  seat to `var-g2/tuniversal`. It has no origin and no lane of its own. Its
  own time is the conductor's beat `u`, read beside `h` (the dilation du/dh,
  78.156).
- **No proper time.** Every world steps on one beat; nothing in netstratum
  runs at a rate set by its depth in a well.

## Time is a coordinate

- An EVENT is a point of the map: a place on the S^360 and its time
  coordinate.
- **Proper time** is the length of one observer's worldline: true for that
  observer and for no one else.
- **The interval** between two events is a distance on the map, and every
  observer agrees on it.
- **There is no universal "now".** Two events with no causal path between
  them have no true order.

**Correction to docs/77 item 5.** A clock deep in a well ticks SLOWER than one
far from it, not faster. The ancients at a core live fewer proper ticks per
unit of cosmic time; the outer universe's signals reach them blueshifted and
compressed. That is the narrative's "inundated with temporal waves at a rate
the outer rim cannot imagine" (docs/76). The rim ages faster than the core.

## What is invariant (the unified timeline's foundations)

1. **Causal order.** If one event can influence another (a signal along the
   cones, a hull's passage, a wormhole crossing), every observer agrees on
   which came first.
2. **Cosmic time.** Time measured from the singularity's bounce, the event
   every worldline descends from, along the cones. It is the comoving clock
   of the spherical universe, read from its epicenter.
3. **Proper time.** Each observer's own, and translatable to cosmic time by
   its map coordinates.

## The observers, by tier

| tier | holds | duty |
|---|---|---|
| ANCIENTS | the reference clock | at the cores, the cone points where the bounce's torsion originates; each core's bell keeps cosmic time for its galaxy, and the cores together keep it for the universe |
| PROGENITORS | synchronisation | a wormhole joins two places, so its mouths must agree on cosmic time, or a crossing could arrive before it left (a causal loop); progenitors keep the mouths synchronised to the ancients' clock: chronology protection as a duty |
| STEWARDS | translation | a realm spans depths in the wells, so its members' proper times differ; the steward places each member's events on cosmic time and settles order by causal order first, then cosmic time |
| SOVEREIGNS, CAPTAINS | proper time | they live and decide by their own clocks; reports up carry proper time and map coordinates, and the tier above places them |
| THE HYPHA | TUniversal | the clock between universes, read through final points; the court orders events between universes, never within one |

## TUniversal from the singularity (the proposal relayed)

1. **The universe's own origin.** Netstratum era 1's singularity, the bounce
   at the epicenter of its S^360, is the ORIGIN of its TUniversal points: its
   time starts there, distinct from the fleet's. Era 1's new domain and its
   version-2 origin (docs/78) carry it.
2. **The singularity as quantum drive and uplink.** The core singularity's
   bounce drives the universe's quantum layer (docs/79: its torsion phase is
   the hidden variable) and its link to the hypha, through the core tube.
   What crosses up is read on the singularity's own clock.
3. **A dedicated quantum lane, Q,** beside `S:G:T:H`, measured from the
   epicenter:
   - what it carries: the bounce's torsion phase advance, and the count of
     measurement events written to the annals;
   - the court reads netstratum's quantum state as a coordinate of its time.

**Needed from mycelium, the owner of TUniversal:**
- per-universe origins in the point, read through a universe's core tube;
- a fifth lane (a wire change for every node, and for the μ modes measured in
  lanes);
- the lane's measure and unit.

## In netstratum, once ruled

- **The beat** is the bell's count of turns: units of cosmic time at the core,
  never a global step.
- **Each world has a proper-time rate** from its depth in its core's well
  (slower deep, faster at the rim); its readings are closed forms in its
  proper time, placed on cosmic time when compared.
- **Every annal event records both** its map coordinate (place, cosmic time)
  and the observer's proper time. Order: causal, then cosmic time.
- **Quantum measurement (docs/79) happens at an event:** the bounce's hidden
  phase is read at that point's cosmic time.

## Build order and done-when (the live log, before → after)

Within era 1, after A1/A2, on mycelium's ruling of the proposal.

1. **Proper time.**
   - Done-when: worlds with a proper-time rate from their depth, 0 → all;
     the rim-against-core rate ratio logged.
2. **Events on the map.**
   - Done-when: annal events carrying place, cosmic time and proper time,
     0 → > 0.
3. **The origin and the Q lane.**
   - Done-when: TUniversal points with netstratum's own origin, 0 → > 0; a Q
     lane reading on the court.
   - Witness: the court's point.

## Relay record

- 2026-10-13: proposed to the coordinator (xylarium-kernel), for mycelium's
  ruling on per-universe origins, a fifth lane, and its measure. Awaiting
  their answers.
