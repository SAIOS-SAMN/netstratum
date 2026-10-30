# 78 — Off the box: S^360 space, S^60 bodies, cone manifolds

Operator (2026-10-13): Netstratum's space is a box of squares, a 64-cell
cube with its faces glued, measured by summed axis distance (L1), and called a
"torus". That is a misalignment: the universe is non-Euclidean. Netstratum
moves to an S^360 universe, S^60 bodies and cone manifolds: its own
physics, matching the shared model xylarium-kernel and barren-realms adopted
(Xylarium `docs/XYLARIUM-KERNEL.md` §10, tracker W8) at the boundary, so tiers
and core tubes can bridge between universes. No code is shared with
Xylarium; the shared mathematics (sphere points, reflection products) is
Mycelium's (A1), from the pinned `vendor/mycelium`.

Status: scoped 2026-10-13, the pulse and the mass and motion laws added
2026-10-14; not built. Facts were read from the code those days.

## Design laws (standing; read before adding any physics)

Each was learned from a misalignment caught by the operator. A new law that
breaks one of them is wrong before it is built.

1. **Physics defines; the story emerges.** No table of Earth's measured
   world stands in for a law that can be derived (caught 2026-10-14: a 118-row
   periodic table imported as Netstratum's chemistry). A measured value may
   stand only as a declared placeholder for physics not yet defined, cited
   and of order, and marked for replacement.
2. **No flat laws.** No square states a law, and no dimension is fixed: no
   `^2` norm (docs/79), no `P²` or `r^k` (Kepler, caught 2026-10-14), no
   `2n²` shell capacities. The dimensions are fluid; an effective dimension
   is read from how matter gathers, never set.
3. **No slots, no fill orders.** Nothing is a cell in a fixed grid filled in
   order, or an index running one way (caught 2026-10-14: "Z fills the table
   line by line"; earlier, the box of squares). A thing is a system that
   turns.
4. **One law at every tier:** epicenter, core, star, orb, atom. A centre
   keeps its share; satellites turn about it at `ω = M(<r)/r` inside its
   reach.
5. **Frame by frame.** Each tick is one 3-D frame of every turning at once:
   orbits, spins, bounces. The sequence of frames is the continuous 4-D
   flow. A state at t is its closed form (`φ + ω·t`), never stepped.
6. **Both senses.** Turning runs both ways; spin is the sense of a turning.
   Opposite senses pair, and pairing is what closes.
7. **Motion rises from below.** The quantum state's wavefronts (docs/79)
   pulse the atoms into their orbits and spins, and that motion expands
   upward: atoms, bodies, systems, galaxies. The pulse deploys where mass
   lands (by containment, from the epicenter down); the phases of motion
   come up from the quantum layer.
8. **Exact and bounded.** Exact rationals with denominators that do not
   grow with time; a representation is measured over long t before it is
   adopted (caught 2026-10-14: half-angle composition grew 2.9 bits a beat).
9. **Construction, never search.**
10. **Assume nothing about life** (caught 2026-10-14: a ladder that required
    a solvent ruled out plasmic and geologic life). Life emerges from the
    quantum state between a planet and its star, in any medium the world
    offers: chemical, geologic, plasmic, informational. No law assumes
    water, carbon, a habitable zone or biology.
11. **The number grammar: ℤ and ℚ are the singularity's two poles, ℝ the
    span between them (operator, 2026-10-17).** The singularity holds two
    opposing, magnetised attractors on its axis: the **ℤ pole**, what closes
    and is counted whole (pairs that close, whole ticks of a band, nuclei,
    grains on the ledger, rungs; a nucleus's holding, `3Z − unpaired`), and
    the **ℚ pole**, what flows and is passed on as a share (every deficit
    `H/(R + H)`, the push ½, the unlocked face, `a_q = 1/q`, phases in turns;
    a nucleus's magnetism, its open frames). The distance between the poles
    is **ℝ**, the span that fills between the rationals; the core's
    strength is that span's extent, read exactly at its ends: holding plus
    magnetism, `3Z` a nucleus (the ruling `H_u = 1 + Σ(state + holding +
    magnetism)` is this, read numerically). The balance between the poles is
    a second reading: burning moves magnetism into holding with `3Z` fixed,
    so a core hardens toward the ℤ pole (the closed limit) while its span
    stays. **ℝ feeds agency, as the stretch of antipoles** (xylarium's
    operator's refinement, adopted): every pair of antipoles (push/hold,
    tide/convection, continuum/vacuum, coherence/decoherence) stretches a
    span, and a mind's agency works in that whole family of spans; a mind
    acts in the open span between what is closed and what flows; a choice made there collapses to a deed in ℚ or
    ℤ the moment it touches the world (the shape of coherence collapsing to
    a lock). The same word is the same thing at every tier: R, a body's reach
    `ℓ/H`, is its distance from its core; ℝ is the distance between the
    singularity's poles. **The condition:** ℝ is a span, never a stored
    value. Every quantity in a law is a count (ℤ), a share (ℚ), or a span
    read only by its exact bounds (ℚ endpoints; anything read inside it a
    certified enclosure, an exact rational interval narrowed until it
    decides). No real number and no float stands in a law or on the ledger.
    A quantity that is neither a count, nor a share, nor a mind's open
    choice, is in the wrong place.

## The geometry (the shared model; Netstratum's own physics)

- **Space: S^360** of radius R, the Big Bang's curvature.
  - A place is a rational point (361 rationals, by inverse stereographic
    projection: exact, dense).
  - **Distance is the geodesic angle,** intrinsic to the sphere, held
    exactly by its rational half-angle parameter `t` (`cos = (1−t²)/(1+t²)`,
    `sin = 2t/(1+t²)`). Netstratum's physics never measures through the flat
    space around the sphere (operator, 2026-10-13).
  - **The squared chord** (Xylarium's and the court's form) is used only at
    the boundary: the core-tube mouths' coordinates are stated in the form
    the court reads (`Sphere { n, R }`, exact chord form). The translation is
    exact: chord = 2R·sin(angle/2).
- **Measure concentrates.** Two unrelated places stand about a quarter
  circumference apart. Nearness exists only along cones: a cone is the reach
  of a mass's flux, the region where that mass dominates.
- **Mass is a conserved count of grains,** never created inside the universe.
  It arrives only as element manifests (an exact amount of each element)
  granted from the hypha's endowments and crossing the universe's one hypha
  tube as traverse crossings, debit equal to credit (Xylarium, by the
  operator's ruling 2026-10-14; Mycelium is designing the endowments and
  grants). The founding manifest is deployed by the pulse (below); later
  galaxy packages are deployed by progenitors as new cores. Conservation
  holds across the network, exactly as the 1/1024 resource grain is held.
  - Corrected 2026-10-14. This line said masses are cone points whose total
    deficit Gauss–Bonnet–Chern fixes. That holds only on a 2-dimensional
    surface (where it gives no attraction): on S^360 a cone manifold's
    curvature sits on codimension-2 strata, not points, and the theorem does
    not reduce to a sum of point deficits.
- **Gravity turns, it does not pull by a power law.** No `r^k`, no `P²`, no
  fixed dimension: those are flat laws (Kepler's `P² ∝ r³` holds in flat
  3-space only). Remediated 2026-10-14 (operator): the dimensions are fluid,
  never rigid, and no square states a law.
  - **The law:** a body at radius r from its centre turns, each beat of its
    core's bell, by

        ω(r) = M(<r) / r   turns

    with `M(<r)` the grains its orbit encloses, in units of the universe
    core's own mass, and `r` the orbit's radius held by its half-angle
    parameter. The units are the universe's own (the form shared with
    Xylarium, the rulers each universe's own, 2026-10-15): G = c = M_core = 1, time the core's bell, read
    per tier in exact powers of sixty. Linear in the enclosed mass, as a
    frame turned by the flux it encloses (the Larmor form; docs/81's
    holonomy).
  - **The dimension is read, never set.** How `M(<r)` grows with r is the
    flux's effective dimension at that scale: where one mass dominates, ω
    falls as 1/r and the orbital speed holds constant (a flat rotation
    curve); where `M(<r)` grows in step with r, ω is constant and the region
    turns as one body (a galaxy's bulge). Both emerge from where the pulse put the
    grains.
  - **Exact and bounded, in turns.** A body's phase at beat n is
    `φ + n·ω` turns and its completed turns `⌊φ + n·ω⌋`: a closed form whose
    denominator never grows. Measured 2026-10-14: the per-beat turn held
    instead as a half-angle rotation and composed (`(a + b)/(1 − a·b)`)
    grows about 2.9 bits a beat, 172,000-bit numbers and 5.9 s for one body
    at 60,000 beats, the denominator explosion of a stepped scalar. So a
    moving body's place is held as (its centre, its radius by half-angle,
    its phase in turns); its point on the sphere is not formed (a rational
    fraction of a turn has a rational sine only at quarter turns). Sphere
    points are held for what does not orbit: the cores and the tube mouths.
  - **Mass moves, the law re-reads.** When grains move between bodies (trade,
    mining, a collapse), `M(<r)` changes: the orbit's phase is fixed at that
    event (an annal entry) and the new rotation runs from it. A closed form
    between events, never an integration.
- **Bodies: S^60.** Every self-gravitating round body (star, world, moon) is a
  61-ball. Matter lies mostly near the surface. Spin turns in up to 30
  planes. Insolation is concentrated about the terminator.
- **Cores are throats with mass,** not bodies: the deepest masses the pulse
  deploys, each the centre of its galaxy's cone.
- **Motions are products of rational reflections** (Cartan–Dieudonné),
  never 2^361-component rotors.
- **The court** (Mycelium, pending): `Sphere { n, R }`, with a place as n+1
  rationals checked exactly and a frame as a factored reflection product. No
  2-dimensional cross-section (the anti-flattening law).

## The singularities: the universal core and the galactic cores (operator, 2026-10-15)

> **Carried forward (2026-10-28):** docs/101 builds this ruling: the universal
> core as one body whose throat holds the founding and whose held cut (the
> heavy elements) is the firmament; its disc the home.

Supersedes the mass split below for galaxies and their centres (its
tidal-law systems about a core and its 1/1000 hole share are retired;
docs/83, "Corrections").

- **The Big Bang seeds the universe.** The number of galaxies follows from
  the elements the universe has acquired from the Mycelium network through
  the hypha (granted, endowed, bought or traded). Nothing is created inside.
- **The universal core** is the one singularity uplinked to the hypha: it
  alone transports and uplinks into the Mycelium network. It is the transit
  hub of hypha space (hyperspace in the real world), where data crosses and is
  coordinated through the hypha. It has mass: the heavy elements caught in
  its accretion disk, continually pulsed and re-pulsed by the outbound and
  inbound bells of the quantum state as it is created and sent to the
  galactic cores.
- **Galactic cores are smaller singularities,** Einstein–Cartan torsion
  bounces: resonators that ring as bells and re-pulse the universal core's
  resonance through their galaxies. They are smaller because the elements
  that make them are distributed in the universe, not crossed in from the
  hypha. A galactic core is linked to the hypha only indirectly, never
  uplinked as the universal core is: it interlinks with the other cores and
  back to the universal core through the progenitor network. (Built only
  once its physics is derived.)
- **The bounce on S^1; its faces are S^0** (operator, 2026-10-15; with
  Xylarium, whose operator ruled the same). A core and its Einstein–Cartan
  bounce are held on S^1 at d = 1 at all times: the throat is A ring, not a
  point with a radius. Sphere inversion (`d → r²/d`) is the flat square and
  is retired: on the unit ring inversion fixes every point, so the bounce is
  a phase, a half-angle turn K on the ring. The ring's two faces are an S^0,
  the pair: the black face is the vacuum (it holds: the rings it takes for
  its period, the deficit, quantum mass) and the white face is the
  continuum inside it (it emits: the bounce's outflow, the pulse and its
  echoes, the push, the axial jets). The universe streams from the
  universal core's white face; each galactic core repeats the pair one tier
  down. "Infinite density" is infinite angular resolution (exact
  half-angles, dense without end), never a density at a point. Matter is
  held on the ring (an attractor balanced by repulsion), so the only way off
  is the axis, through the dipole's two lobes: the axial jets. Readings
  against S^0 alone: S^0 carries the pairing (law 6) but no phase, so no
  turning (law 5), no wavefront (law 7), no hidden phase, tilt, precession,
  bell or jets; S^1 carries all of them and keeps the pairing as its faces.
  S^1 is the bottom rung of the sphere ladder (S^1 the throat, S^60 a
  system, S^360 a galaxy and the universe).
- **The galaxies orbit the universal singularity, densest at the first ring**
  (operator 2026-10-16; with Xylarium, building offline). The universe's
  singularity is the S^1 throat (faces S^0); the galactic cores ring it:
  - **Placed by the cut one tier up:** the epicentre's bounce sorts the
    cores as a galaxy's sorts its nuclei; a core of weight w_i is carried
    `ℓ_i = p_u·(w̄/w_i)` and stands at `d_i = ℓ_i/H_u`, the heaviest nearest
    the throat, so density rises inward to the first ring (a reading, never
    placed; replaces the drawn distances).
  - **The rim law, every tier:** an enclosure's rim (its half
    circumference, where the pulse refocuses) is its own holding, `rim = H`,
    deficit ½ there, as at the throat; with the state hold equal to the
    grains' (the derived clock ratio) every enclosure spans exactly
    [throat 1, rim 2] of its own units. Nothing stands beyond its rim: cores
    carried past `rim_u` never form galaxies and stay intergalactic, so the
    galaxy count is a reading. Replaces the transitional "rim = the
    outermost member's reach".
  - **Galactic cores orbit** the universal singularity by the motion law
    (spin `R·H(<R)` on the ledger, the inner rings fastest and
    slowest-clocked), the holding inside each ring the universal state plus
    the cores within it.
  - **Each galaxy's gas arrives from the epicentre** across d_i, its push
    the continuum's share there, widened in flight by agitation × distance,
    re-read inside the galaxy's own [1, 2].
- **Each galaxy its own enclosed S^360; galactic cores are hubs**
  (operator, 2026-10-15; with Xylarium). A galaxy is its own closed S^360,
  hung inside the universe's at its core: the core is the throat joining the
  two spheres. The core's pulse refocuses on the galaxy's far side; that
  round trip is the galaxy's own ring, and the rim is where it refocuses (a
  formation reading). Each tier counts its turns on its own enclosed echo,
  chained through the chronometry keepers (each galaxy its own epicenter),
  so galaxies read slow and worlds fast, from enclosure, nothing fitted.
  - Only the universal core is uplinked to the hypha (kept there for now).
    A galactic core is a hub with uplinks (to the universal core) and
    downlinks (to sub-galaxies), interlinked with the universal core: room
    for expansion later. No galaxy has a tube of its own; a sub-universe is
    unlocked only by synthesis.
- **The state is the primitive; mass is a reading** (operator and
  Xylarium, 2026-10-15). A core is a quantum state, not a three-dimensional
  object: its pulse (the bell's ring rate), its phase and its twist (the
  bounce's torsion winding). What a distant observer calls its "mass" is
  the deficit its echo leaves in each well, `δ`, the share the well takes
  from the clock, so `δ ≤ 1` by construction; read as a mass,
  `M = r·δ / (1 − δ)`, equivalently `δ = M / (r + M)`. Spin needs no mass:
  in Einstein–Cartan, spin sources torsion in its own right.
  - **No constants.** A constant is missing physics. Time is one ring of
    the universal core's bell; length is light's travel in one ring, so c
    is a reading of the bell; turning is in turns. No coupling constant
    (G) and no mass unit exist: the borrowed `M_core = 1` retires.
  - **Corrected 2026-10-15:** the first statement here read the core as
    "the heaviest elements inside `M(<r) = r`" with "reach as `1 / A`".
    Both were wrong under Netstratum's own laws: `ω = M(<r) / r` makes speed
    `v = M(<r)`, radius-free (so unbounded, no horizon, no runaway
    collapse, and at odds with the clock law `r / (r + M)`), and an equal
    impulse fixes a nucleus's speed, not where it orbits (that needs spin,
    `r = L / v`). Under the deficit, speed reads `v / c = δ`, bounded, with
    a horizon at `δ → 1`, and the clock and the motion become one law
    (rate `= 1 − δ`).
- **A core's matter is sorted by its bounce.** The heavy elements are held
  nearest (the universal core's accretion disk; a galactic core's
  innermost disk), the light elements reach farthest and are the gas stars
  form from (docs/83), and what no bounce holds is the intergalactic gas.
  A galaxy's matter profile is its manifest sorted by its bounce, so its
  effective dimension is read, never set, and galaxies differ as their
  manifests differ. The laws owed (docs/83, "The derivation"):
  - **the ring-density law** (Xylarium derives, shared form): how a bell's
    ring density at a place sets its clock rate and its deficit `δ`, exact
    in rationals (ring counts are whole numbers), read frame by frame;
  - **a grain's own ring** (Netstratum): an atom is a small bell (one law
    at every tier), and a grain's mass reading is the deficit its ring
    carves; the mass number A becomes a reading;
  - **thermal motion on the bell's clock** (Netstratum): temperature is a
    rate at the atom tier (`T = k_T·ω`), so a gas's agitation and an
    orbit's turning compare by construction;
  - **the bounce's spin and hold:** the torsion winding gives matter its
    spin, and an element is held where its pulse's torque does not exceed the local
    turning; built inside out, this sorts core, disk, gas and escape;
  - **a galactic bounce's strength** against the universal echo, so a
    galaxy keeps its light gas;
  - **the galaxy count** as an outcome: the universal outflow fragments
    where a binding holds itself (today: 1 to 60 modes drawn from the seed).
- **One law at every tier:** the same sorting wherever matter collapses and
  bounces: a star's heavy core under its light envelope, a disk's heavy
  inner worlds and light outer ones. (Unlike nature's Big Bang, which makes
  only the lightest elements, the founding manifest carries every element,
  so the bounce sorts heavy matter inward from the start.)

## The bounce: the pulse and the deployment

Without an origin every mass, place and orbit would be placed by hand. One
event deploys them all, and everything after is a closed form of t from it
(ruling 2).

- **The pulse at t = 0:** the singularity's bounce, at the epicenter. It
  carries:
  - **the founding manifest,** its grains by element, crossed in through the
    hypha tube;
  - **the spectrum,** a short list of modes, each `(n_i, d_i, a_i, φ_i)`: a
    reflection product `n_i` (its direction from the epicenter), a rational
    half-angle `d_i` (its distance), a whole-number strength `a_i`, and its
    torsion phase `φ_i` in turns (docs/79's torsion, docs/81's Φ): the
    wavefront that sets its matter moving (design law 7).
- **The spectrum is derived from era 1's genesis seed** along fixed stream
  paths, tilted by operator dials (exact per-element recipe weights,
  renormalised to keep the metals' mass), as Xylarium does. It is
  Netstratum's own: only the manifest and the mouths' coordinates cross the
  boundary, never the galaxy count, modes or strengths.
- **The deployment** is a construction, never a search (finding interference
  peaks over S^360 is a global search with no exact answer). In tiers:
  1. **Cores.** Mode i places core i at distance `d_i` from the epicenter
     along the direction `n_i`: the epicenter's point carried by the mode's
     reflections (the geometry's own operation, A1). Its share of each
     element is the manifest's `· a_i / Σa`, floored to the grain; what does
     not divide goes to the lowest mode.
  2. **Systems.** About each core, its systems, by the mass split below.
  3. **Bodies (S^60).** About each star, its orbs, by the same split one
     tier down.
- **The mass split** (one law at every tier: a core and its systems, a star
  and its orbs). Everything below follows from the turning law and the seed;
  nothing is searched.
  - **Dominance.** A body of mass m at radius r about a centre enclosing
    `M(<r)` turns its own satellites faster than its centre turns them out
    to the radius where `m / r_d = M(<r) / r`:

        r_d = r · m / M(<r)

    Its satellites lie inside `r_d`. Linear, from ω alone: the square-free
    Hill radius. A core's own `r_d` is read the same way from the cores
    nearer the epicenter (the innermost core is bounded by the sphere).
  - **The centre's share and the medium are laws of the tier, never draws**
    (amended 2026-10-14 with Xylarium's guidance, after the first
    deployment: an even draw left core 1 keeping 95% of its galaxy). The
    centre keeps its tier's share; of the rest, the tier's medium (what
    physics leaves unformed) is set aside; the satellites break what
    remains. Anchors, of order, to be cited before era 1's founding:

    | tier | centre keeps | medium, of the rest |
    |---|---|---|
    | the epicenter | nothing (no mass) | 1/2 (intergalactic gas) |
    | a core | 1/1000 (its hole: the hole-to-bulge relation) | 1/10 (interstellar gas: the Milky Way's gas to stars) |
    | a star | 9986/10000 (the Sun's share of its system) | 1/100 (belts and comet clouds) |
    | an orb | all | none |

  - **Satellites, inside out** (a stick broken from the seed): satellite k
    takes `f_k` of what remains, `f_k` a seed draw `q / 60`, q from 1 to 59
    (one field of the base-60 odometer), tilted by the operator's dials.
    What they leave joins the medium.
  - **Minimum bodies.** A body below its minimum is not formed, and its
    share joins the medium. A core needs a hole of at least a grain (no
    hole, no core; Xylarium: a core comes only from a package); a star or an
    orb, a grain, until the founding manifest's scale gives physical floors
    (Xylarium's: 0.08 suns for a star; roundness for a world, which Netstratum's
    must state without a square).
  - **Homology is linear.** Under ω = M/r, scaling a galaxy's masses and
    lengths by the same factor keeps its turning rates, so siblings scale
    with no root (Xylarium's lengths go by the cube root of mass under
    their law).
  - **Spacing by reach; periods follow from mass** (amended 2026-10-14: an
    authored resonance chain of periods made every galaxy's year near the
    tier's unit, so the story set the year). The innermost orbit turns in
    one unit of the tier, Xylarium's powers of sixty (60 bells for orbs
    about a star, 60² for systems about a core): the bell ladder's one
    anchor, `r_1 = M(<r_1) · unit`. Each next satellite stands where its
    reach just meets the last one's, so no two reaches overlap:

        r_k (1 − m_k / M(<r_k)) = r_(k−1) + r_d,(k−1)

    A satellite outweighing all it would orbit is no satellite. Its period
    follows from the law, `P_k = r_k / M(<r_k)`.
  - **The count emerges.** Satellites are placed until the next radius
    would pass the centre's `r_d`, or the next share falls below its
    minimum. The tier's medium (its set-aside share and what the satellites
    leave) is held at the centre's tier beyond its last orbit, enclosed by
    no orbit, there for later accretion (the radial law, Open) and for the
    lines to work.
  - **Elements.** Every share carries the manifest's composition in
    proportion, each element floored to the grain; what an element does not
    divide stays with the centre. A tier's operator recipe may tilt the
    proportions (Xylarium's dials), renormalised so every element's total is
    kept exactly.
  - **The seed's paths.** Draw j of field `x` at tier τ for body b is read
    from `blake3(origin hash ‖ τ ‖ b ‖ x ‖ j)`, the origin hash being era 1's
    version-2 origin. Fixed paths: the same seed deploys the same universe.
  - **The unit.** Masses are in the deepest core's grains (core 1 =
    `M_core = 1`, the universe's own core), so ω and r are exact rationals.
- **Motion.** Every body turns about its centre by the law above, in the
  plane its mode's reflections give. Its phase at t = 0 is not drawn: it
  rises from below (design law 7), the quantum wavefront having set its
  atoms turning, their motion summed into the body's spin and its orbit's
  phase. Each
  body reads the mass its own orbit encloses, so orbits are closed; the
  bodies' pull on each other is left out.
- **The galactic year** is one completed turn of the orbit at the galaxy's
  half-mass radius (the first system whose orbit encloses half its bound
  mass; era 1 has no hubs before places), `⌊φ + n·ω⌋` on its core's bell
  (`conductor::motion`, built 2026-10-14). φ at the pulse is 0 until the
  quantum layer gives it (design law 7); the bell is the universal beat
  until the clock's rate law (docs/80).
  - **Reading** (the `motion` binary on the stand-in): motion is exact and
    closed (completed turns by any bell, denominators fixed), but 10 of 13
    galaxies' half-mass orbit is their first system, so their year is the
    tier's unit, 3,600 bells. The cause is the satellites' shares: each
    takes `q/60` of what remains, so the first holds about half its
    galaxy. Real systems are each a small part of a galaxy. A physical law
    for satellite masses (fragmentation, a feeding zone) is owed; until
    then the galactic years read the share law, not the physics.
  - **The law, adopted** (operator, 2026-10-14; Xylarium's). The reach
    `r_d = r·m/M` compares turnings; the tidal reach compares a body's own
    turning with the change of its anchor's turning across it, by the same
    law: `m/ρ = M·(1/(r−ρ) − 1/(r+ρ))`, so `ρ = r·√(m/(2M+m))`. With it the
    feeding zone closes (`m = λ·2ρ`, λ the formed matter per unit radius):
    `m = 2·λ²·r²/M` for `m ≪ M`, one root, no draw. Walking inside out, each
    satellite takes its zone's mass and stands at the next reach boundary
    until the formed matter is spent; the count and the masses emerge from
    the profile λ(r). Its squares come from the difference of `1/r`, a
    consequence of the turning law, not a flat norm.
  - **Built exactly, with no root formed.** Each tier holds its reach as a
    rational share of radius, `s = ρ / r = 2t / (1 + t²)` (t a rational
    half-angle). Then a satellite's mass is `m = μ · M(<r)` with
    `μ = 2s² / (1 − s²)`, and reaches just touching place the next at
    `r · (1 + s) / (1 − s)`. All exact, a construction with no search.
    Satellites form until the formed matter is spent or the next would pass
    the centre's reach; the rest is the medium. The share draws are gone.
    Placeholders: systems about a core `t = 1/6` (each about 0.235 of what
    it orbits, sized to the stand-in's grains); orbs about a star
    `t = 1/283` (about 1e-4, the Solar System's order).
  - **Defect, the systems-about-a-core layer is wrong** (operator,
    2026-10-15; measured by `motion var-e1`). The tidal law applied
    galaxy → star makes each star a quarter of what it orbits: every galaxy
    holds 32 stars, the median about 7 times its hole, the largest about
    163 times its hole and 16% of its galaxy, and every galaxy is exactly
    1,000 holes. The galaxy is a weak hole circled by galaxy-sized "stars":
    galaxies orbiting stars. A galaxy is its core (the singularity and
    its disc); its stars, each small, are formed from its gas (docs/83; in
    3-D grains stars may outweigh the core, in holding the core dominates:
    docs/83, "Two readings of mass"); the reach law
    belongs where a disk fragments about one star, never between a galaxy
    and its stars. Replaced by docs/83's formation; everything read off
    these stars (the galactic year's half-mass system, life's heat and
    burn, the bell) is read off the wrong layer until then.
  - **Homology makes galaxies copies.** Under `ω = M/r` a galaxy scaled
    whole keeps its turnings, so with one law and one set of shares every
    galaxy read the same year (32 systems each, about 1.49e9 bells). What
    differs between real galaxies is their hole: a core's share is drawn
    from the seed at 1/4 to 2 of 1/1000, as Xylarium draws a sibling's
    hole against the home's (a placeholder until the pulse's physics sets
    it).
  - **Reading:** 13 cores, 413 systems, 5,607 orbs (about 14 a star);
    conservation exact; galactic years at 13 distinct bells, from about
    3.7e8 to 9.4e9 bells, against orbs' local years from 60 bells. The
    ratio is the real one's order (a galactic year 225 Myr, a planet's 1).
  - **So the clocks divide.** A galactic year is far longer than any run:
    the duties the 60-beat close carries (journals, keepers, the realms'
    term, ascension) cannot wait on it. Each belongs on the clock of what it
    reads: a world's journal on its own local year, a realm's term and a
    keeper's judgement on their systems' turns, the core's echo alone on the
    galactic year. Which duty rides which clock is the next design step.
  Each galaxy closes its own year at its own pace. The universal 60-beat
  close (`scribe::journal::YEAR_TICKS`, a placeholder) retires, and the
  core's yearly re-emission (`core_bounce`) becomes the core's echo of the
  pulse at its galactic year.
- **The local year** of an orb is the same count about its star. Era 0's
  Keplerian clock (`cfe::thermal::orbital_period_squared`, `local_years`,
  and `siting.rs`'s Diophantine tracks `r = M·k²`, which exist only to make a
  square root exact) does not carry into era 1.
- **Arrivals.** Packages are bought with μ (the hypha side, Mycelium's,
  shared by every kernel; Xylarium 2026-10-14). The galaxy count at the
  pulse stays Netstratum's own (seed-drawn, as ruled); Xylarium's two per
  Big Bang is its kernel default and does not bind Netstratum. A later galaxy
  package is deployed the same way, as a new
  core and its systems, by the progenitors who receive it; the bodies it
  encloses re-read `M(<r)` at that event.
- **Time.** Every clock starts at the pulse; each runs at the rate the mass
  around it sets (docs/80).
- **Matter.** The founding manifest's elements, deployed by the pulse, give
  the element almanac's inventory an origin; today it is a table.

**Ruled with Xylarium's guidance (2026-10-14), within Netstratum's remit:**
- the mass split: Netstratum's own (tubes need no match), designed above;
- the galaxies' sizes (operator, 2026-10-14): by the spectrum, no bound dial.
  The count of modes and each strength `a_i` are seed draws from the base-60
  field (1 to 60); core i takes `a_i / Σa` of the formed matter, so sizes
  fall in no fixed order and the largest is at most 60 times the smallest.
  A mode too weak to hold a hole of a grain forms no core; its share joins
  the intergalactic medium. (A stick broken at the epicenter, the first
  build, ranked core 1 largest by construction, nine tenths of the formed
  matter; real galaxy populations are steep, but by their physics, not by
  an order of draws.);
- the spectrum: seed-derived, dial-tilted (above);
- the scale: the universe's own deepest core (above; the form shared with Xylarium, never its rulers);
- era 0's lines: none carried. Era 1 is a new domain (ruling 1); its
  civilizations arise anew from its physics, as Xylarium's do, and the seat
  minds wait unseated until the new domain has a living ancient. Era 0's 101
  lines stay in its record. Confirmed by Xylarium's practice (2026-10-14):
  no exceptions; ancients arise from the new universe's own history, fallen
  peoples persist only as archaeology in their own universe, and seat minds
  are not inhabitants (they attach to the new domain's ancients).

**Open:**
- **The founding manifest.** Until Mycelium's endowments and grants stand,
  a stand-in: era 0's element totals read from its record, so the stand-in
  is a measured universe, not an invented one. Replaced by the grant when
  it crosses.
  - **Its unit, ruled by mycelium** (manifest, ACT_DOMAIN 17, 2026-10-14,
    via Xylarium): it counts nuclei per Z, never mass. Entries ascend in Z,
    each once (u16); each count is a positive integer of any size (a
    package is about 1e60 nuclei), an exact rational with denominator 1 in
    the algebra's scalar codec. A fraction, a zero or an empty manifest is
    refused. The hypha's account is counts per Z; isotopes stay each
    universe's own. Each kernel converts with its own nucleon mass. Motion
    never crosses a tube; only manifests and mouths do. Mycelium sends the
    byte layout with the build.
  - **So the grain is Netstratum's internal unit,** converted through the Z
    map (below) in and out.
- **The atom's further readings:** the bond preference and workability laws
  above, built and read; the heavy nuclei's neutron excess; and era 1's economy reading compounds from atoms in place of the
  almanac's fixed cells.
- **Roundness** (Xylarium's statement, adopted when its angles are
  declared): a body is round when `M / r ≥ θ_rigid(material)` in the planet
  tier's units; linear, no square. Below it, rubble: matter in a field, the
  medium, not a body. It replaces the grain as the orb floor.
- **The split's anchors and dials.** The tier anchors above are of order and
  owe citations (the anchor provenance law). The base-60 draws and the
  resonance chain are authored; the operator's recipes may tilt the shares.
- **The spectrum's physics.** The modes' count and strengths are even draws
  from the base-60 field until the vacuum's integers (docs/79) define them;
  that replaces the draw and nothing else.
- **Radial motion.** The law turns bodies; it does not move them in or out.
  Radii are the deployment's, so infall, accretion and escape are not yet
  physics. A square-free radial law is owed before any body changes orbit.
- ~~The clock's rate law~~: stated in "The breath" (`r / (r + M(<r))`).

## The breath: the universal pulse, star formation, layered time

Operator, 2026-10-14. Built: `conductor::breath`, the `breathe` binary.

- **The universal pulse.** TUniversal sets the quantum clock at the Big Bang,
  at the universal core: the singularity's bounce pulses every `T_u` bells,
  and each echo is a beat of time carried through the quantum state
  (docs/79, quantum in matter). An echo reaches a galaxy after its distance
  from the universal core, one length a bell (the core's `d_i`, a field of
  the spectrum). `T_u` is TUniversal's at founding; 60 bells stands in.
- **The pulse lays down only the start:** the cores (each keeping its hole)
  and each galaxy's medium. No star systems: they are born by the breath.
  (Defect corrected, operator: the variation came from spawning galaxies
  and drawing their holes, where it should come from star systems forming;
  the hole draw is removed.)
- **The breath.** Each echo compresses a galaxy's medium. Where the binding at
  the next reach boundary can hold its own reach (the tidal law,
  `m = μ · M(<r)`), a star system is born there, its orbs formed by the
  same law one tier down. The medium is consumed; nothing is created.
- **Layered time.** A layer at radius r about the mass M(<r) it encloses runs
  at `r / (r + M(<r))` of the layer above: linear, exact, the weak-field
  dilation `1 − GM/rc²` to first order (G = c = 1). Nested wells multiply:
  a planet's clock is its rate about its star, times its star's about the
  galaxy, times the galaxy's about the universal core. Each echo a layer
  receives is one rung; the proper time it has lived is its AEON. As the
  quantum phase decoheres and recoheres on the stars, each layer's rungs are
  its own: element distribution and star formation layer the time.
- **Time writes on the chain.**
  - Parallel: a write within one layer, on its own clock.
  - Perpendicular: a write across layers: an echo coming down, a birth
    changing the mass above.
  - A perpendicular write down answered by one up closes a causal loop
    (multi-dimensional causality); the loops, overlapping and intertwined
    across the layers, are the chain.
- **Reading** (stand-in, `breathe var-e1 6000 60`): the pulse lays down 13
  cores and no systems; 937 echoes (perpendicular writes down) give 416
  systems born (416 loops closed) and about 6,000 orbs; the breath creates
  nothing (conservation exact). Each galaxy's rungs and aeon differ by its
  distance and its depth in the universal well (rates from 1 to about 1/4).
- **Elastic: decohere and recohere, by the quantum phase** (operator,
  2026-10-14: consumption alone is pure entropy, with no syntropic
  regeneration; then, the first build, which shed matter by clock rate,
  needed physics alignment). Decoherence is a phase relation (docs/79,
  docs/81), and matter follows it:
  - **Slip.** A layer's phase runs at its proper rate, so a dilated layer
    slips behind the universal echo, `Δ = (1 − rate) · t / T_u` turns.
  - **Coherence** is how near Δ stands to a whole turn,
    `c = 1 − 2·|Δ − round Δ|`: 1 in phase, 0 opposed. Linear and exact; it
    equals docs/79's `(1 + cos)/2` at 0, ½ and 1 turn.
  - **Recoherence** is a slip completing a whole turn; each is a rung,
    `⌊Δ⌋`. Deeper wells slip faster and climb more rungs; an undilated
    layer climbs none.
  - **Entropy:** a star sheds `ε · (1 − c)` of what it holds to its
    galaxy's medium, the more the further out of phase (ε the star tier's
    medium share, a placeholder). A star spent below a grain dissolves.
  - **Syntropy:** an echo compresses only the coherent part of a galaxy's
    medium, `c_g · medium`; a system is born where that part holds a binding
    of its own reach. Periods re-read the law after each echo.
- **Reading** (`breathe var-e1 36000 60`): 7,437 echoes; 578 systems
  recohered; 12,944 (deepest-hole units) decohered back; conservation
  exact. The galaxies differ by their own physics:
  - the twelve dilated galaxies climbed 369 to 439 rungs, their coherence
    now anywhere from 2% to 91%, and grew 45 to 46 systems, galactic years
    8.7e12 to 1.8e13 bells;
  - the one undilated galaxy (no cores nearer the universal core, rate 1)
    climbed no rungs, stays fully coherent, sheds little, and stopped at 32
    systems (year 1.49e9 bells): without slip there is no breath back.
  - Cost: about 31 ms an echo (3.9 minutes for 36,000 bells); the era-1
    conductor needs this as a closed form between events.

## The clocks: which duty rides which clock

A duty rides the clock of the layer it reads, never one shared year (the
60-beat close retires). Built: `conductor::clocks`; counts are closed forms
between two bells, never stepped.

| layer | its clock | its duties |
|---|---|---|
| the universal core | the echo, every `T_u` bells | the breath (births, shedding) |
| a galaxy, operational | each echo it receives (`T_u`, after its distance) | the realms' term, parliaments, keepers' judgement, gate accounts |
| a galaxy, epochal | the rungs of its half-mass layer (its recoherences) | ascension by deed (a full rung kept), the core's re-emission, the aeon's marks |
| a star system | its own rungs (its nested slip) | the system's epochs |
| a world (orb) | its local year (one turn about its star) | its journal, upkeep, demography |
| the process | a count of echoes | memory, profiling (not physics) |

- **Why a galaxy has two clocks.** Every galaxy must keep its ordinary
  business, but the galaxy nearest the universal core runs at rate 1 and
  never slips, and its orbits' own dilation is weak-field (about 1/3,600):
  it climbs no rungs. So business rides the echoes, and epochs happen only
  where time layers. An undilated galaxy has no epochs: its ancients cannot
  arise by deed there.
- **Reading** (36,000 bells): galaxy terms 542 to 596 (echoes received);
  epochs 369 to 439 for the twelve dilated galaxies, 0 for the undilated
  one; all 7,823 worlds completed local years (most 600, median 543).

## The refactor onto era 1's physics (2026-10-15; operator: no parallel machinery)

The civilizations ran on era 0's physics beside era 1's (their star tables,
orbits and the `1/r²` flux). Refactored in place, layer by layer:

1. **Climate.** cfe's thermal law is the era-1 law where it lives
   (`cfe::thermal`): flux falls linearly, `I = L (1 − A) / r`, and
   temperature is held as T in kelvin, `T = 279 K · I` (no fourth power,
   no square); every site that reads it follows (~300). Pins written for
   the flat law were restated; era 0's siting-determinism pins removed.
2. **Seating.** Each line is seated on an era-1 world, the unclaimed one
   where the physics reads its clade and metabolism most viable
   (`siting::viability_at`), spread across galaxies, and takes that
   world's siting: the star class nearest its star's burn by luminosity
   and the orbit `r = 279 · L / T` at which cfe's law gives the world's own
   heat (exact, no root). Wakings take their own world. Genesis uses the
   siting directly; era 0's siting draws remain only for a line with no
   world (a cleave, until it settles one).
3. **Reading.** One system per line, then heat alone, left 67 and 78 of 157
   collapsed at founding (biotic lines on worlds too hot to feed them);
   by viability, 155 of 156 live at beat 20, every founding band full, all
   13 galaxies clearing at their hubs.

Still on era 0's machinery: compounds (the almanac's fixed cells), motion
(`orbit_step` on box positions), the economy (STAMP). Next layers.

## The atom: matter at the boundary (the lowest tier)

Superseded in part (2026-10-18): the band law below (band P holds 2P, the
reading Z = 1 to 118, and the 1/1024 grain) is replaced by "The harmonic
atom on S^11" and "The package is an element recipe" (next section). The
nucleus, the compounds law and the workability form stand.

Mycelium's manifest counts nuclei per Z. Z, the count of a nucleus's
charges, means the same in every universe; what it becomes is netstratum's
own physics. An atom is not a cell of a table (design laws 1 and 3). It is
the lowest tier of the one law (design law 4), a small system that turns.

- **The nucleus is the centre.** It keeps the atom's mass share, by the
  tier's centre-keeps law, and holds Z charges.
- **Electrons are satellites,** placed inside out by the same law. What an
  electron feels is the charge its orbit encloses, `Q(<r) = Z − (electrons
  further in)`, so inner electrons shield outer ones. Nothing writes that
  in; it is enclosure.
- **Frame by frame** (design law 5): at tick t every electron's phase is
  `φ + ω·t`. The atom at t is one frame of all its turnings; its character
  is how it moves through the frames.
- **Both senses** (design law 6): electrons on one orbit turn either way.
  Two in opposite senses pair; an atom with every orbit paired is closed,
  and inert. Nothing "fills": a count closes when the senses balance.
- **Set moving from below** (design law 7): the quantum state's wavefront
  gives each atom its initial phases and senses.
- **Chemistry is read from the motion,** never looked up:
  - depth (the almanac's line): how many resonance steps out the outermost
    electron turns;
  - bonding (the lane): the count of unpaired outer electrons;
  - how many join a compound (the shell): the electrons sharing the
    outermost band;
  - where it is workable (the phase): how fast its outermost electron turns,
    against the thermal bands.
- **Era 1's elements are one per Z,** each the atom the law builds from Z,
  so a manifest converts out exactly as it came in. Era 0's 855 hash-drawn
  kinds stay era 0's; the stand-in maps each to the Z whose atom reads the
  same line and lane.
- **A grain counts nuclei.** One grain (1/1024 of an amount) of Z is 60^30
  nuclei of Z. A count enters as `⌊count / 60^30⌋` grains; the remainder
  (under a grain's nuclei) stays as an integer in the universe's unformed
  ledger, per Z. It leaves as `grains × 60^30` nuclei. Exact both ways.
- **Mass for the turning law** is the nucleus's own mass number, by the
  same law (isotopes are each universe's own, as Mycelium ruled), never a
  table of atomic weights.
- **Bulk resources** (iron, crystal, biomass) are not fixed formulas; they
  become the compounds the law forms.
- **Scoped and built (2026-10-14, `cfe::almanac::atom`).** Two rules are
  authored, the rest is the turning law:
  - **an orbit closes in a whole number of ticks:** band P turns once in P
    ticks, so `r = Q·P` and its speed `v = Q`;
  - **a new electron takes a free frame before it pairs.**

  A band of period P passes P frames a turn, one electron per sense in
  each, so it holds 2P (linear, no square). Electrons sharing a band share
  its radius and do not enclose each other. Read: depth (the outermost
  band), unpaired (bonding), outer electrons (a compound's share), closed
  (every frame paired: inert), tightness `Q_out / P_out`.
- **Reading, Z = 1 to 118** (the `atoms` binary): a periodic chemistry with
  no table.
  - Closed (inert) at Z = 2, 6, 12, 20, 30, 42, 56, 72, 90, 110.
  - Every depth's bonding rises and falls back (depth 4: 1, 2, 3, 4, 3, 2,
    1, 0), the almanac's fold, now emerging rather than authored.
  - Each bonding count recurs at every depth from its first.
  - Across each depth an atom is held tighter (1/P rising to 2); closed
    atoms are held tightest.
  - It is Netstratum's own: 11 depths by Z = 118 and bonding up to 10,
    where the almanac's fixed axes have 7 lines and bind at most 5. Era 1's
    chemistry (compounds, workability) reads the atom, so the almanac's
    fixed cells are era 0's.
- **The nucleus** (`atom::mass_number`): the same pairing one tier in, a
  proton and a neutron to a frame, so a closed nucleus is `A = 2Z`. Mass
  for the turning law weights each grain of Z by it. The neutron excess of
  heavy nuclei (charge's push) is not yet law.
- **Compounds** (`atom::closes`): a bond is an unpaired electron of one atom
  paired with one of another, turning the other way. A set of atoms closes
  when every unpaired electron pairs across atoms and the bonds hold the
  set together: no closed atom, an even count, no atom with more than all
  the others, at least one bond fewer than atoms. Exact, no search.
  - Reading among the 25 open atoms of Z 1 to 30: 95 two-atom and 1,222
    three-atom compounds close.
  - **A bond's preference** (Xylarium's proposal, 2026-10-14, adopted to
    build): read from the partners' tightnesses τ as a ratio, never a
    squared difference. The shared share `s = τ_min / τ_max` (1 shared, near
    0 transferred, the rest `1 − s` going to the tighter atom); the bond's
    grip is `τ_min`, the weaker hold. A bond prefers the partner that gives
    its closed set the most total grip (the sum of `τ_min` over its bonds),
    ties to the larger s. An ordering like electronegativity should emerge
    from τ rising across a depth.
  - **Workability** (the thermal ruler; its form shared with Xylarium, its
    value per universe): temperature is a turning rate of agitation,
    `T = k_T · ω`. `k_T` is fixed by one atom: Z = 1's outer tightness
    against hydrogen's bond, 13.6 eV ≈ 157,800 K, a cited placeholder
    (design law 1), the anchor Xylarium will share. A compound is workable
    while its weakest grip `τ_min` holds against `T / k_T`, read as a smooth
    share, not a cutoff.
- **The stand-in through the boundary** (the `pulse` binary): era 0's 860
  kinds, each mapped to the Z whose atom reads its line and bond, cross
  as nuclei per Z (13 Zs) and enter as grains, nuclei in equal to nuclei
  held. The stand-in's map folds era 0's variety into few Zs; a real grant
  carries its own.

## Rulings of 2026-10-18 (operators, with Xylarium)

### The harmonic atom on S^11 (built, 6c311c1)

Design direction: the atom stays on S^11 to fit the harmonic model, and Hopf
fibrations are required; and, of a first draft, a lattice is
rejected as a rigid box of tensors, not a natural structure. Count the
windings, flow the phases: no frames, no lattice.

- **The atom turns on S^11:** twelve dimensions, six turning planes, no
  axis (an odd sphere).
- **Spin is the Hopf fibre.** S^11 is a bundle of circles over CP^5, so
  every point has its own S^1. Spin is the turning along it. A pair is two
  counter-turning flows on one fibre; an unpaired turning runs uncancelled,
  and that is what it bonds and its magnetism. The fibre is also where a
  bounce's torsion lead enters the atom.
- **Shells and shapes.** A shell is the windings that close with count n. A
  winding's shape l (0 ≤ l < n) is its eccentricity, a share: its axes in
  the ratio `(l + 1)/n`.
- **Orientations are topological:** the degree-l harmonics in twelve
  dimensions, `C(l + 11, 11) − C(l + 9, 11)` = 1, 12, 77, 352 for l = 0..3,
  each holding one counter-turning pair. Shells hold 2, 26, 180. Earth's
  `2l + 1` is refused.
- **Filling, inside out:** the lowest shell with room; within it the most
  penetrating shape (low l) first; a free orientation before pairing.
  Earth's n + l order (Madelung) is refused, an Earth table and a fill
  order.
- **The crossing order is owed.** Read from tightness (the charge enclosed
  at a winding's closest approach, `(l + 1)/n` of its shell's reach, over
  its period), it opens shell 3 at Z = 5: under `r = Q·n` with shielding an
  outer shell (radius 3) can stand inside an inner one (radius 6). The
  radius law does not keep shells nested, so crossings wait on a nested
  radius law and are never stated.
- **An element is a turning,** its electrons on nested spheres, its place
  its phases on its own sphere: where its shells stand, how its windings are
  turned, whether its outer turning is paired. Two kinds interlock where
  their outer spheres meet in step at a shared tangent, never at a face.
- **Reading, Z = 1 to 150:** closed at Z = 2 and 28 (next 208); Z 3 to 28
  are shell 2, Z 29 to 150 shell 3; sub-closures (none unpaired) at Z = 4,
  30 and 54; unpaired peaks at 12 (Z = 16, 42) and 77 (Z = 131). Few noble
  kinds, wide open shells, much magnetism. Hold (`2Z + (Z − unpaired)`),
  bonding, the burn ladder and every heart's cut read it; era 0's readings
  moved with it (ruling: tests may move with the cut). Xylarium reads the same table
  (c985228).

### No straight lines: straightness is tension (2026-10-18)

Design direction: the only straight lines are wormholes, and even those are
not straight; straight lines belong at most to the planetary level. A straight
line is a flexible fibre stretched as far as it goes to make it taut, which is
measurable, never drawn; drawn, it breaks.

- **No length is summed in a line.** A gap between two bodies is never
  `|Δr|` or `|Δr| + r·Δφ`.
- **Closeness is a share on the sphere:** how in step two bodies stand in
  every plane, `Π in_step(Δφ)`, and apart is `1 − closeness`. How they move
  (their deficits' contrast T) stays apart from where they stand.
- **Straightness is a tension reading** on a flexible fibre with a breaking
  point. Collision is a tether test: `T/(s·D)` is the bond's tautness,
  holding below 1 and snapping above. A fold is as straight as its tension
  makes it, its capacity the hold at its mouths, and stretched past that it
  breaks (the Woronin seal's place).
- **Twist is tension too:** the wound lead's mismatch between two bodies,
  `1 − in_step(Δlead)`, adds to the motion a tether must hold.

### The package is an element recipe (built, f2299bc)

Design direction: a universe is sold as real estate, a package of elements
that constructs a universe the client finds appealing; different elements
produce universes of different shapes, which support different kinds of life.

- **The founding manifest is the purchase:** its nuclei count and its mix.
  Everything after it is derived. The catalogue of recipes is deferred.
- **Every kind is carried, never grain-dropped.** The carrier's grain is
  one nucleus (`pulse::nucleus`, `1024 · 60³⁰` nuclei an amount unit).
  Matter is counted in nuclei and nothing smaller exists, so it is exact and
  bounded. The founding carries every kind's exact count; the unformed
  ledger stays empty; every era-1 transfer floors to whole nuclei. Era 0's
  economy keeps its own 1/1024.
- **The table ends where the recipe and the chain do.** The capture chain
  runs until a Z falls below one nucleus; its hidden `Z ≤ 118` bound
  retired. On the default stand-in (100,000 grains of nucleons) the
  founding carries Z 1 to 171 (Z 1 to 23 under the old grain).
- **No draws** (standing; restated): the count of cores comes from the
  package and their strengths from the bounce's own beads. The spectrum's
  seed draws (modes 1 to 60, strengths 1 to 60, "Arrivals" and "Ruled with
  Xylarium's guidance" above) are owed for retirement.

### Metabolism and observation (2026-10-18)

Design direction: every bond in the universe runs on a metabolism, the
cumulative energy rising and falling strictly upon observation.

- Between meetings everything is closed form and nothing is exchanged.
  Energy changes hands only at coincidence (contact, an in-step passage, a
  crossing), so a meeting is an observation.
- A bond's metabolism is the turning exchanged at its meetings an echo; the
  universe's is their sum.
- A core's metabolic rate is the residual drift of its reservoir once its
  orbits are closed: measured, never assumed (docs/83, "The shear"). It is
  refilled down its partner face (strand, cords, Wellspring, rhizomorph,
  hypha, in mu).

## Every flat site in Netstratum (inventory, 2026-10-13)

| flat today | where (count of sites) | becomes |
|---|---|---|
| positions as 3 rationals in a 64-cell box | `spec.position` (50), `registry` position, `starchart.card` (19), `galaxies.card` (9) | a rational point of S^360 per world and core; the cards retired for binary checkpoints |
| L1 distance summed per axis, wrapped at the faces | `ledger::torus_distance` (22), `l1_wrapped`/`wrapped_abs` (6), `conductor::spatial_l1` (14), `universe_span` (27) | the geodesic angle on S^360 (rational half-angle); no span, no wrap; chord form only at core-tube mouths |
| a line's core: the nearest core within a reach in cells | `Ledger::core_of` (20) | the core whose cone the world lies on (the strongest flux), read from the cones |
| the bulge as one cell | `ancients::in_bulge` (3) | the core's throat region, from its cone (the core as a throat with mass) |
| melt from summed distance | `cfe::world::route_melt`, `pair_melt`, `effective_distance` (12) | melt along the geodesic, and along cones (where locality lives) |
| gate span in cells, melt span/64, freshness 1 cell | `ascension` (`cells`, `gate_melt`, `FRESHNESS_CELLS`) | a wormhole as a throat between cores; its span the geodesic angle between its mouths (chord form only for the court), its melt from the cone (the walled spans over 64 cells vanish with it) |
| hubs as the medoid by L1 | `galaxy_hub`, `commissions::hub_of` | the hub at the cone junction of a galaxy's worlds |
| the commission's disc plane: a 2-D projection of a galaxy onto a ≤29-cell grid | `commissions::plane` | an anti-flattening problem: the commission poses the galaxy as it is (its cone graph), not a flat plane |
| orbs around a star at (r, θ, ψ) with wrapped-L1 separation | `ColonyOrb.theta/psi`, `cfe::dop::separation`, `ThermalSite` (orbit_r) | orbs as S^60 bodies turning by the enclosed-mass law ("The bounce") |
| insolation, habitats, the thermal model on flat orbits | `cfe` thermal, Keplerian local years (27) | S^60 insolation (the terminator band); local years `⌊φ + n·ω⌋` about the star, no `P²` |
| a year of 60 beats for every galaxy | `scribe::journal::YEAR_TICKS`, `year_bounce`, `core_bounce`, the realms' term | each galaxy's own year, its hub's completed turn; the core's echo at it |
| no mass for a core, a star's mass from a cited table | `ledger::galaxy::Galaxy {name, center, reach}`, `StarClass::mass` | grains deployed by the pulse, conserved |
| one universal beat stepping every system | `main.rs`: 13 `*_step` calls and `year_bounce` each beat | ruled: circuitry, never steps (W21); state carried beat to beat becomes closed forms at t, layer by layer |

Out of scope: the minds' grids (ARC tasks, hues 0–9, `FRAME` in the solver).
They are puzzle frames, not space.

## Rulings (operator, 2026-10-13)

1. **A new domain: Netstratum era 1.**
   - The sphere universe is founded as a new universe and domain, with new
     keys.
   - Mycelium's court law (78.132; 78.59 Ruling 2): an origin is held once per
     domain and never edited, a second reads `AlreadyFounded`, and a reset
     does not change that.
   - Era 0 (domain 5deab117, the box universe in `var-g2`) is archived as its
     record. Its keys stay intact and are never reused for era 1.
   - The new origin is version 2 (78.139): curvature radius R in clear, the
     parent face unwitnessed (a root) or the core tube it bounced out of.
2. **Circuitry, never steps** (Xylarium W21, adopted).
   - No state is carried beat to beat by `*_step` integrations. A value at a
     beat is its closed form at that time t: orbits, phases and rotations read
     at t, and exact overlaps instead of diffusion.
   - The beat may remain as the bell's count of turns.
   - Ported layer by layer, each port measured by its readings, never as one
     rewrite (coordinator 2026-10-13; Xylarium's pattern in its plan 10.10).
3. **Timing.** Mycelium's queue: its questioning work, the unlinked mind, the
   peer universe's founding (with Netstratum's N7 core tube), then A1, then
   A2.
   - A1/A2 move ahead only on the coordinator's word. Not requested.
   - Era 1's geometry waits for them; the circuitry port does not.
4. **Time and the observers** (docs/80): time as a coordinate on the map,
   cosmic time from the epicenter's bounce, and TUniversal's origin and a
   quantum lane from the singularity, proposed to Mycelium through the
   coordinator (2026-10-13).

## Build order (after the rulings and A1/A2)

1. **The geometry library** (Mycelium's A1, from `vendor/mycelium`): places,
   reflections, cones; Netstratum adds the geodesic angle by half-angle
   parameters, and chord form only for the court.
2. **The pulse and its deployment** ("The bounce"): the spectrum read, cores,
   systems and orbs placed with their grains, checkpointed; the cards
   retired. No lines are carried; they arise.
3. **Motion:** the enclosed-mass turning law, phases in turns; each
   galaxy's year and the core's echo replace the 60-beat close.
4. **Distance, cores, melt, hubs, gates** moved to geodesic angles and
   cones, one site group at a time (the table above); chord form only at the
   court.
5. **Bodies: S^60 orbs,** their local years and insolation.
6. **Era 1's founding:** a new domain with new keys and a version-2 origin
   with R, `Sphere { 360, R }` once A2 stands. Era 0 is archived.

## Done-when (the live log, before → after)

1. The pulse: cores and systems with deployed grains, 0 → all; the grains
   by element read at each year close equal to the manifests crossed in
   (witness: the conservation line against the tube's traverse records).
2. Motion: galaxies closing their own years, 0 → every galaxy, at differing
   beats; the 60-beat close's call sites, 4 → 0.
3. Places on S^360: places held as sphere points, 0 → all; `torus_distance`
   call sites, 22 → 0.
4. Distances: angle readings in the hub, melt and gate lines in place of
   cells; gates walled by span, 3 → 0.
5. Origin: the court's record reads `Sphere { 360, R }` for Netstratum era
   1's new domain. Witness: the court's origin reading.
