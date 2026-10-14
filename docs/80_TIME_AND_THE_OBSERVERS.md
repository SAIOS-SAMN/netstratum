# 80 — Time, the observers, and TUniversal from the singularity

Operator (2026-10-13): in a deterministic universe time is a coordinate on
the map: to traverse space and time is to produce the value of time as a
coordinate, and the separation between two times is a distance. Observers
must be ordered by the hierarchy so the universe has one unified timeline.
TUniversal is the quantum drive of the universe's core singularity, which
provides the uplink to the hypha, with a dedicated lane for netstratum's
quantum state, aligned on the epicenter of the spherical universe.

Status: scoped 2026-10-13; RULED by mycelium the same day (MU-RECURSION
78.152d, below); not built. It belongs to era 1 (docs/78) with the quantum
layer (docs/79).

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
| ANCIENTS | the reference clock | at the cores, the throats where the bounce's torsion originates (docs/78: the deepest masses the pulse deploys); each core's bell keeps cosmic time for its galaxy, and the cores together keep it for the universe |
| PROGENITORS | synchronisation | a wormhole joins two places, so its mouths must agree on cosmic time, or a crossing could arrive before it left (a causal loop); progenitors keep the mouths synchronised to the ancients' clock: chronology protection as a duty |
| STEWARDS | translation | a realm spans depths in the wells, so its members' proper times differ; the steward places each member's events on cosmic time and settles order by causal order first, then cosmic time |
| SOVEREIGNS, CAPTAINS | proper time | they live and decide by their own clocks; reports up carry proper time and map coordinates, and the tier above places them |
| THE HYPHA | TUniversal | the clock between universes, read through final points; the court orders events between universes, never within one |

## TUniversal from the singularity (the proposal relayed)

1. **The universe's own origin.** Netstratum era 1's singularity, the bounce
   at the epicenter of its S^360, is the ORIGIN of its TUniversal points: its
   time starts there, distinct from the fleet's. The same pulse deploys the
   universe's mass (docs/78, "The bounce"). Era 1's new domain and its
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

## Chronometry inside the universe, built (2026-10-14; one law with xylarium)

`engine/conductor/src/chronometry.rs`, the shared law of xylarium's
11.4d. KEEPERS: the epicenter, then per galactic core its deep core
(innermost system), inner mantle (half-mass radius) and outer rim
(outermost system). Each runs at the wells' rate `r / (r + M(<r))`, nested,
from its delay (its galaxy's distance plus its radius at the ring's light,
the home rim half a bell). At the epicenter's count N it reads local
`rate (N − delay)`, slip, coherence and rung. Every pulse every keeper
appends a hashed entry to `<var>/chrono.jrn` citing its own last entry and
the keeper inside it, its slip against that keeper kept as the RESIDUAL,
never averaged; the epicenter's entry carries the last TUniversal final
point heard (the bridge). Live at beat 1,980: 40 entries (the epicenter and
13 galaxies' three shells), 8 of 13 rims with a residual against their
mantle; point None (the house unlinked while the courts are down).

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

- 2026-10-13: proposed to the coordinator (xylarium-kernel).
- **The coordinator's view (2026-10-13).**
  - AGREED on the frame: it matches xylarium's "time is the bell".
  - (1) A universe's own origin, but kept OUT of TPoint. Its cosmic time runs
    from its bounce, carried in era 1's v2 origin. The fleet's TPoint stays
    single-origin (the operator's one-ledgered-clock ruling). The two meet at
    the core tube: each crossing records the pair (local cosmic t, TUniversal
    point), held by the ancients.
  - (2)/(3) Against a Q lane: it changes `TPoint::LEN` and every reader, its
    content is internal state TUniversal should not order, and μ prices only
    measured discharge. Instead, the torsion phase advance and the
    measurement count go in a field of the universe's LEDGER entries
    (ACT_DOMAIN ledger sub-kind) through its core tube; pricing later would
    be a mode ruling.
  - The hidden variable is netstratum's worldbuilding; keep it deterministic
    and recorded.
- 2026-10-13: the operator's proposal and the coordinator's note relayed
  together to mycelium (mycelium-v0-1-0-84).

## Mycelium's ruling (MU-RECURSION 78.152d, commit 6f23618d, 2026-10-13)

Under mycelium's clock-binding authority. The operator's frame stands as
stated.

- **(a) Origin: crossing pairs, not in the point.** TPoint's origin is the
  mesh's (TU-5), and points of two origins are never compared; an origin in
  the point would leave TUniversal unable to order events between universes,
  the work the frame gives it.
  - A universe's COSMIC TIME is its own: its bounce's count of returns,
    measured from the bounce its version-2 domain origin records.
  - The two meet at the core tube: each crossing records (cosmic time, the
    TPoint it crossed at), held by the ancients.
  - DILATION is read between two crossings, as cosmic advance over the
    point's advance; never stored.
- **(b) No Q lane.** The four lanes are the kinds of a mode (law 5): S
  hyperbolic, G the hue map, T parabolic, H elliptic. A torsion phase is
  elliptic: where it is measured flux it already lands on H. A fifth lane
  would be a lane of no kind. A count of measurement events is a ledger
  coordinate (like K), never a lane. Pricing torsion later is a mode ruling
  on H.
- **(c) Superseded (commit 821841cf): no new statement kind.** Kind 12 is
  already ACT_DOMAIN's LEDGER. Netstratum's LEDGER entry (sub-kind 12) already
  carries the crossing pair: the universe's committed cycle `u` (its cosmic
  time) beside the mesh's final point (`refound::ledger_frame`). The
  torsion's exact coordinates (per hole: canonical generator, rational point
  c and side λ, winding t/ℓ where finite) and the measurement count n will
  ride the LEDGER entry as an OPTIONAL TRAILING FIELD. Entries without it are
  unchanged. The phase advance is never stored; the court re-derives it.
- **Owed:** the trailing field's byte layout, with the build after the peer
  universe's founding.

**What changes here:**
- Item 3 of the proposal (a Q lane) is replaced by the LEDGER entry's
  trailing field.
- Item 1 (an origin in the point) is replaced by crossing pairs.
- Build step 3 becomes "the LEDGER entry's torsion field":
  - done-when: LEDGER entries for era 1's domain sealed with the torsion
    field and n, 0 → > 0, each with its cosmic time u and point; dilation
    read between two;
  - witness: the court's sealed LEDGER entries.
