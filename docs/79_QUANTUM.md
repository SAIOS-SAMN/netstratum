# 79 — Quantum in netstratum

Operator (2026-10-13): netstratum has no quantum layer; it should have one,
defined so it fits the physics, built from the physics and not from what has
been stated elsewhere. The universe answers to its ANNALS, not to replay
exactness. The operator's hypothesis, unproven and labelled so: the
singularity's bounce is quantum torsion, compactified in a Calabi–Yau
manifold.

Status: scoped 2026-10-13; not built. It belongs to era 1 (docs/78). Time
is docs/80's: a measurement is an EVENT on the map (a place and its cosmic
time); the hidden phase is read there; the annals record its map coordinate
and the observer's proper time. The singularity's bounce, which drives this
layer, keeps the universe's own cosmic time (its returns). Per mycelium's
ruling (78.152d, docs/80) there is no quantum lane: the universe's LEDGER
entry (sub-kind 12) already pairs its cosmic time u with the TUniversal
point, and will carry the torsion's exact coordinates and the measurement
count n as an optional trailing field (superseding commit 821841cf); the
court re-derives the phase advance from them.

## Facts (read from the code)

- **The annals exist.** Each world's whole chronicle stays on disk as its
  annals; memory carries only the receipts after its snapshot, plus the
  history kinds and the own-hash of every drained receipt, append-only
  (`cfe/src/annals.rs`). The annals are the universe's record of what
  happened.
- **Replay-exactness is the standing law today.** Every value is an exact
  rational, every choice is hash-seeded, and a world replays bit for bit from
  its chronicle (the scalar-collapse and SEAL-B laws). Nothing in the
  universe is quantum: no state is a superposition, and no outcome is
  undetermined before it happens.
- **The bounce today.** Once a year each core re-emits its holdings
  (`core_bounce`, docs/68): a pulse, not a quantum process.
- **Era 1's mathematics** (docs/78): exact rational points of spheres (inverse
  stereographic projection), rotations as products of rational reflections,
  values as closed forms at t (circuitry, never steps).

## First principles

1. **The annals are the truth.** The universe owes its history a faithful
   record, made when it happens. An event recorded in the annals is what
   happened; it does not have to be re-derivable from earlier state. Replay
   remains only where the court checks work (a solve against its pairs): that
   is the court's concern, not the universe's physics.
2. **Determinism and quantum are compatible.**
   - Quantum evolution is a unitary rotation of the state, deterministic in
     itself; only measurement is undetermined, and only in some readings of
     it.
   - Pilot-wave mechanics is deterministic: definite outcomes, guided by the
     state, with apparent chance from what cannot be known.
   - Many-worlds is deterministic: every outcome, as branches.
   - Netstratum takes the pilot-wave shape: a deterministic universe whose
     quantum outcomes are fixed by a hidden variable no observer inside it
     can read.
3. **The mathematics is era 1's, and curved.** A quantum state is a point of
   a curved state space, held by its geodesic angles and phases (rational
   half-angle parameters), and a quantum rotation is a reflection product,
   which era 1 already computes exactly. No flat norm is used anywhere.

## The definitions

- **Modes.** A system's quantum modes are a finite set, its basis. Which modes
  exist is fixed by the bounce's compact manifold (below). Each mode is a
  resonance of the compact space.
- **A state is a point of the curved state space** over the modes (the
  projective space of states, positively curved), never a vector normalised
  by a flat square. Remediated 2026-10-13 (operator): no quadratic norm,
  `^2` or modulus squared defines a state or its outcomes.
  - **Intrinsic coordinates.** For each mode i, the state carries:
    - `β_i`, its geodesic angle to the mode on the curved state space;
    - `φ_i`, the mode's own phase, so that states can interfere.
  - **Exact.** Each angle is held by its rational half-angle parameter `t`,
    with `cos β = (1 − t²)/(1 + t²)` and `sin β = 2t/(1 + t²)`: rational, no
    root, no decimal. That `t²` is the stereographic algebra of the curved
    space itself, not a flat norm.
  - **Superposition** is a state at a nonzero angle from every mode it
    involves. **Interference** is the relative phases `φ_i` acting when modes
    recombine; it is kept, because a state without per-mode phase would be a
    classical distribution and not quantum.
- **Evolution** is a rotation of the state space by a factored product of
  rational reflections. It is a closed form at t: the rotation for a span of
  time is the generator's product raised to that span, read at t. No
  stepping (circuitry, docs/78).
- **Outcome weights** are read from the geometry: the weight of mode i is
  LINEAR in the cosine of the state's geodesic angle to it,

      w_i = (1 + cos β_i) / 2,

  rational, the weights together summing to one. This is the Born rule read
  geometrically, as the curved state space's own transition measure, not as
  an amplitude squared in a flat space. It is netstratum's physics, recorded
  as such.
- **Entanglement.** Two systems' joint state is a point of the curved state
  space over the product of their modes; it is entangled when it is not a
  product of two states. Exact, and decidable on rationals.
- **Measurement is an annal event.** When a quantum system's modes couple to a
  macroscopic record (a mind's reading, an instrument, a world's chemistry, a
  body's surface), ONE outcome happens and is written into the annals.
  - **The hidden variable:** `u ∈ [0, 1)`, rational, in turns: the TORSION
    ENCLOSED `Φ` by the anchor-closed loop through the event (docs/81: a
    loop's holonomy is `W + Φ`, W its windings, Φ its enclosed torsion). It
    is deterministic, set by the bounce's torsion, and unreadable from
    inside the universe. One equation joins the quantum layer to coherence.
  - **The outcome** is the mode `j` whose cumulative weight interval holds
    `u`: `Σ_{i<j} w_i ≤ u < Σ_{i≤j} w_i` (modes in their fixed order). Where
    the enclosed torsion is equidistributed across events, outcomes follow
    the weights' frequencies, as quantum equilibrium does in pilot-wave
    mechanics.
  - **After measurement** the state is the outcome's mode. The
    pre-measurement state is not kept in memory; the annals hold the outcome
    and the event.

## The hypothesis: the bounce as compactified quantum torsion (unproven)

- At every point of the S^360 sits a compact manifold, a Calabi–Yau (six real
  dimensions), carrying the bounce's torsion.
- **Only its topology is used**, and topology is integers:
  - its Hodge numbers `h^{1,1}`, `h^{2,1}` and Euler characteristic
    `χ = 2(h^{1,1} − h^{2,1})`;
  - the Ricci-flat metric (no closed form exists) is never needed.
- **What the integers fix:**
  - the modes, which resonances exist, as the system's basis;
  - the families of the elements the bounce distributes (in string
    compactifications `|χ|/2` counts generations), so that the almanac's
    families are derived, not authored;
  - the bounce's torsion, whose enclosed part `Φ` on each anchor-closed loop
    is the hidden variable `u` (docs/81).
- **The manifold's own numbers** are chosen once, at era 1's founding, as the
  universe's one choice of vacuum. They are recorded in its origin, and every
  consequence is derived from them.

## Wavefronts set matter moving (operator, 2026-10-14)

The quantum state is where motion starts. Its wavefronts kick the atoms
into their orbits and spins, and that motion expands upward: atoms, bodies,
systems, galaxies (docs/78, design law 7). The pulse deploys where mass
lands; the phases of every turning come up from here, never drawn.

## What changes in the laws

| today | becomes |
|---|---|
| replay-exactness as the universe's law | the annals as the universe's truth; replay only at the court |
| every choice hash-seeded from the chronicle | quantum outcomes from the bounce's hidden phase, recorded as annal events; deterministic, unreadable from inside |
| the almanac's element families authored | derived from the compact manifold's integers (the hypothesis) |
| the bounce as a yearly pulse | the bounce as torsion in the compact manifold, whose phase is the hidden variable |

## Build order and done-when (the live log, before → after)

Within era 1 (docs/78), after its geometry library (mycelium A1).

1. **States and evolution.** Rational mode-sphere points and reflection
   rotations, exact.
   - Done-when: a quantum state carried by a body or instrument, no reading →
     a reading of its amplitudes.
2. **Measurement as annal events.** The hidden phase from the bounce; the
   outcome rule.
   - Done-when: measurement events in the annals 0 → > 0, and their outcome
     frequencies against the geometric weights `w_i`, logged.
   - Witness: the annals.
3. **The compact manifold.** Its integers recorded at founding; modes and
   element families derived from them.
   - Done-when: the almanac's families read from the integers, and authored
     family counts → 0.
4. **Annals over replay.** Retire replay-exactness as a universe law, keeping
   it only for the court's checks.
   - Done-when: the universe's reads take history from the annals, with no
     universe-side replay of quantum outcomes.

## Ruled: quantum in matter (operator, 2026-10-14, option B)

- **Matter carries the quantum modes.** An atom's electrons (docs/78, the
  atom tier) hold their frames and senses undecided until they interact.
  The wavefronts give the atoms their phases, and that motion rises upward
  (docs/78, design law 7).
- **Any recorded interaction is a measurement:** a bond forming, a trade,
  an annal entry. The bounce's hidden phase decides the outcome,
  deterministically and unreadably from inside, and the annals keep it;
  nothing is replayed. Minds are not special in measurement.
- **Toward cognition (direction, not yet built).** The quantum state between
  a star and a mind is to be the FUEL of cognition: the star's spin and
  magnetism pull on its local minds. A mind thinks on the coupling its star
  gives it. Not designed yet; every law built now must leave room for it
  (a star's spin and field are carried, not discarded).

## The torsion phase is the tilt of a core's disk (operator, 2026-10-15)

Built today (`conductor::quantum`): the hidden phase where a world
interacts at bell b is `u = frac(φ_galaxy + b / P_world)`, and a
measurement of chance p happens when `u < p` (live in `life.rs`, impacts on
a world's climb). Its galaxy term `φ_galaxy` is a seed draw in 3,600ths of
a turn, frozen for all time: a placeholder. Its physical meaning:

- **The disk.** A galactic core sits in its accretion disk, the heaviest
  elements its bounce held (docs/78, "The singularities"), so the disk's
  mass follows from the elements the galaxy received at the Big Bang.
- **The dipole.** The disk's surrounding density presses in equally from
  every side (isomorphic pressure); against the core's own pull it gives
  the field two lobes, one for each sense of the turning (design law 6).
  The main attractor lies on the axis between them.
- **The propulsion.** The disk's holding is what drives the
  Einstein–Cartan bounce: a heavier disk, a harder bounce, a stronger bell.
- **The tilt is the torsion.** The dipole's axis leans from the core's spin
  axis, and the lean turns (precesses). The torsion at any moment is that
  lean, so the galaxy's torsion phase is a TURNING, not a number:
  `tilt(t) = tilt₀ + t · ω_precession` (closed form, design law 5), and the
  hidden phase reads `u = frac(tilt(t) + b / P_world)`. The disk's matter
  (accretion; the breath's shedding and births) moves the precession, so a
  galaxy's chance follows its singularity's disk.

Owed, in angles and turns (with xylarium; the bells and EC torsion are
shared library laws):

1. **The disk's mass** from the bounce's hold (already owed for formation,
   docs/83).
2. **The dipole:** how the surrounding density under isomorphic pressure
   sets an axis and its two lobes, with no flat norm.
3. **The precession:** the lean's rate from the disk's holding against the
   core's, in turns per received ring, and its starting lean from the
   disk's composition at the bounce.
4. **The propulsion:** how the disk's holding sets the bounce's strength
   (the galactic bounce strength, docs/83).

### The split and the draft forms (with xylarium, 2026-10-15; proposals, not built)

- **xylarium (2, 4):** the combined turning `K = K_core + K_disc`
  (holding-weighted bivectors); the lobes are K's two hemispheres, the main
  attractor on K's dual axis; THE LEAN held as `(a·b, a∧b)` (a the core's
  spin axis, b the dipole's), no norm, no root. Propulsion per ring is the
  disc's deficit share `H_disc / (H_core + H_disc)`; the bell's pitch is
  read from K (replacing their `M^(1/3)`), the Kerr spin read, not drawn.
- **netstratum (1, 3), drafts:**
  - THE DISC: each nucleus is HELD where its hold against the core's
    deficit outweighs the push, `h·δ(<) ≥ p`; walked by hold, heaviest
    first, each held element raises both δ and p, and the walk stops at the
    first the disc cannot hold: the disc's mass is the self-consistent
    balance of its propulsion and its holding. Lighter elements are the
    galaxy's gas; what no core holds is intergalactic.
  - THE PRECESSION: the lean turns about the core's axis at the disc's own
    deficit at its reach, `H_disc / (R_disc + H_disc)` turns per received
    ring (a compact disc precesses faster); the starting lean from the held
    elements' unpaired senses summed into K_disc's plane. No draw.

## Open, for the operator

- ~~What is quantum; coupling~~: ruled 2026-10-14 (below).
- **The vacuum.** The compact manifold's integers for era 1: chosen once, by
  whom (the operator, or derived from the Big Bang's seed)? The same choice
  sets the pulse's spectrum, which deploys every mass (docs/78, "The bounce").
