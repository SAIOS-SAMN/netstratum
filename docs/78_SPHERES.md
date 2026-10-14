# 78 — Off the box: S^360 space, S^60 bodies, cone manifolds

Operator (2026-10-13): netstratum's space is a box of squares, a 64-cell
cube with its faces glued, measured by summed axis distance (L1), and called a
"torus". That is a misalignment: the universe is non-Euclidean. Netstratum
moves to an S^360 universe, S^60 bodies and cone manifolds: its OWN
physics, matching the shared MODEL xylarium-kernel and barren-realms adopted
(xylarium `docs/XYLARIUM-KERNEL.md` §10, tracker W8) at the boundary, so tiers
and core tubes can bridge between universes. No code is shared with
xylarium; the shared mathematics (sphere points, reflection products) is
mycelium's (A1), from the pinned `vendor/mycelium`.

Status: scoped 2026-10-13, the pulse and the mass and motion laws added
2026-10-14; not built. Facts were read from the code those days.

## Design laws (standing; read before adding any physics)

Each was learned from a misalignment caught by the operator. A new law that
breaks one of them is wrong before it is built.

1. **Physics defines; the story emerges.** No table of Earth's measured
   world stands in for a law we can derive (caught 2026-10-14: a 118-row
   periodic table imported as netstratum's chemistry). A measured value may
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
   kick the atoms into their orbits and spins, and that motion expands
   upward: atoms, bodies, systems, galaxies. The pulse deploys WHERE mass
   lands (by containment, from the epicenter down); the phases of motion
   come UP from the quantum layer.
8. **Exact and bounded.** Exact rationals with denominators that do not
   grow with time; a representation is measured over long t before it is
   adopted (caught 2026-10-14: half-angle composition grew 2.9 bits a beat).
9. **Construction, never search.**

## The geometry (the shared model; netstratum's own physics)

- **Space: S^360** of radius R, the Big Bang's curvature.
  - A place is a rational point (361 rationals, by inverse stereographic
    projection: exact, dense).
  - **Distance is the GEODESIC ANGLE,** intrinsic to the sphere, held
    exactly by its rational half-angle parameter `t` (`cos = (1−t²)/(1+t²)`,
    `sin = 2t/(1+t²)`). Netstratum's physics never measures through the flat
    space around the sphere (operator, 2026-10-13).
  - **The squared chord** (xylarium's and the court's form) is used ONLY at
    the boundary: the core-tube mouths' coordinates are stated in the form
    the court reads (`Sphere { n, R }`, exact chord form). The translation is
    exact: chord = 2R·sin(angle/2).
- **Measure concentrates.** Two unrelated places stand about a quarter
  circumference apart. Nearness exists only along CONES: a cone is the reach
  of a mass's flux, the region where that mass dominates.
- **Mass is a conserved count of grains,** never created inside the universe.
  It ARRIVES only as element MANIFESTS (an exact amount of each element)
  granted from the hypha's endowments and crossing the universe's one hypha
  tube as TRAVERSE crossings, debit equal to credit (xylarium, by the
  operator's ruling 2026-10-14; mycelium is designing the endowments and
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
    parameter. The units are xylarium's anchor, so two universes compare by
    exact ratios at a tube: G = c = M_core = 1, time the core's bell, read
    per tier in exact powers of sixty. Linear in the enclosed mass, as a
    frame turned by the flux it encloses (the Larmor form; docs/81's
    holonomy).
  - **The dimension is read, never set.** How `M(<r)` grows with r IS the
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
- **The court** (mycelium, pending): `Sphere { n, R }`, with a place as n+1
  rationals checked exactly and a frame as a factored reflection product. No
  2-dimensional cross-section (the anti-flattening law).

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
  renormalised to keep the metals' mass), as xylarium does. It is
  netstratum's own: only the manifest and the mouths' coordinates cross the
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
    (amended 2026-10-14 with xylarium's guidance, after the first
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
    hole, no core; xylarium: a core comes only from a package); a star or an
    orb, a grain, until the founding manifest's scale gives physical floors
    (xylarium's: 0.08 suns for a star; roundness for a world, which ours
    must state without a square).
  - **Homology is linear.** Under ω = M/r, scaling a galaxy's masses and
    lengths by the same factor keeps its turning rates, so siblings scale
    with no root (xylarium's lengths go by the cube root of mass under
    their law).
  - **Radii from periods, not dials.** Satellite k's period is
    `P_k = P_(k−1) · (q_k + 1) / q_k` beats of its centre's bell (a
    resonance chain, `q_k` a seed draw from 1 to 60), the first `P_1` one
    unit of the tier in xylarium's powers of sixty: 60 bells for orbs about
    a star, 60² for systems about a core.
    Its radius follows from the law: ω = 1/P_k turns a bell, so

        r_k = M(<r_k) · P_k

    with `M(<r_k)` the centre plus the satellites already placed inside.
    Periods and enclosed mass both grow outward, so radii do too.
  - **The count emerges.** Satellites are placed until the next radius
    would pass the centre's `r_d`, or the next share falls below its
    minimum. The tier's MEDIUM (its set-aside share and what the satellites
    leave) is held at the centre's tier beyond its last orbit, enclosed by
    no orbit, there for later accretion (the radial law, Open) and for the
    lines to work.
  - **Elements.** Every share carries the manifest's composition in
    proportion, each element floored to the grain; what an element does not
    divide stays with the centre. A tier's operator recipe may tilt the
    proportions (xylarium's dials), renormalised so every element's total is
    kept exactly.
  - **The seed's paths.** Draw j of field `x` at tier τ for body b is read
    from `blake3(origin hash ‖ τ ‖ b ‖ x ‖ j)`, the origin hash being era 1's
    version-2 origin. Fixed paths: the same seed deploys the same universe.
  - **The unit.** Masses are in the deepest core's grains (core 1 =
    `M_core = 1`, xylarium's anchor), so ω and r are exact rationals.
- **Motion.** Every body turns about its centre by the law above, in the
  plane its mode's reflections give. Its phase at t = 0 is not drawn: it
  rises from below (design law 7), the quantum wavefront having set its
  atoms turning, their motion summed into the body's spin and its orbit's
  phase. Each
  body reads the mass its own orbit encloses, so orbits are closed; the
  bodies' pull on each other is left out.
- **The galactic year** is one completed turn of the galaxy's hub about its
  core, `⌊φ + n·ω⌋` on its core's bell.
  Each galaxy closes its own year at its own pace. The universal 60-beat
  close (`scribe::journal::YEAR_TICKS`, a placeholder) retires, and the
  core's yearly re-emission (`core_bounce`) becomes the core's echo of the
  pulse at its galactic year.
- **The local year** of an orb is the same count about its star. Era 0's
  Keplerian clock (`cfe::thermal::orbital_period_squared`, `local_years`,
  and `siting.rs`'s Diophantine tracks `r = M·k²`, which exist only to make a
  square root exact) does not carry into era 1.
- **Arrivals.** A later galaxy package is deployed the same way, as a new
  core and its systems, by the progenitors who receive it; the bodies it
  encloses re-read `M(<r)` at that event.
- **Time.** Every clock starts at the pulse; each runs at the rate the mass
  around it sets (docs/80).
- **Matter.** The founding manifest's elements, deployed by the pulse, give
  the element almanac's inventory an origin; today it is a table.

**Ruled with xylarium's guidance (2026-10-14), within netstratum's remit:**
- the mass split: netstratum's own (tubes need no match), designed above;
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
- the scale: xylarium's anchor (above);
- era 0's lines: none carried. Era 1 is a new domain (ruling 1); its
  civilizations arise anew from its physics, as xylarium's do, and the seat
  minds wait unseated until the new domain has a living ancient. Era 0's 101
  lines stay in its record.

**Open:**
- **The founding manifest.** Until mycelium's endowments and grants stand,
  a stand-in: era 0's element totals read from its record, so the stand-in
  is a measured universe, not an invented one. Replaced by the grant when
  it crosses.
  - **Its unit, ruled by mycelium** (MANIFEST, ACT_DOMAIN 17, 2026-10-14,
    via xylarium): it COUNTS NUCLEI per Z, never mass. Entries ascend in Z,
    each once (u16); each count is a positive integer of any size (a
    package is about 1e60 nuclei), an exact rational with denominator 1 in
    the algebra's scalar codec. A fraction, a zero or an empty manifest is
    refused. The hypha's account is counts per Z; isotopes stay each
    universe's own. Each kernel converts with its own nucleon mass. Motion
    never crosses a tube; only manifests and mouths do. Mycelium sends the
    byte layout with the build.
  - **So the grain is netstratum's internal unit,** converted through the Z
    map (below) in and out.
- **The atom's further readings:** the nucleus's mass number (the same law,
  one tier in), the tightness-to-workability bands, and compounds read from
  atoms in place of the almanac's fixed cells.
- **Roundness** (xylarium's statement, adopted when its angles are
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
- **The clock's rate law** (docs/80): slower deeper in a well, stated in
  enclosed mass and radius with no square and no root (the flat-space
  `√(1 − 2GM/rc²)` is not used).

## The atom: matter at the boundary (the lowest tier)

Mycelium's manifest counts nuclei per Z. Z, the count of a nucleus's
charges, means the same in every universe; what it becomes is netstratum's
own physics. An atom is NOT a cell of a table (design laws 1 and 3). It is
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
  same law (isotopes are each universe's own, as mycelium ruled), never a
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
  - It is netstratum's own: 11 depths by Z = 118 and bonding up to 10,
    where the almanac's fixed axes have 7 lines and bind at most 5. Era 1's
    chemistry (compounds, workability) reads the atom, so the almanac's
    fixed cells are era 0's.
- **The stand-in through the boundary** (the `pulse` binary): era 0's 860
  kinds, each mapped to the Z whose atom reads its line and binding, cross
  as nuclei per Z (13 Zs) and enter as grains, nuclei in equal to nuclei
  held. The stand-in's map folds era 0's variety into few Zs; a real grant
  carries its own.

## Every flat site in netstratum (inventory, 2026-10-13)

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

1. **A new domain: netstratum era 1.**
   - The sphere universe is founded as a NEW universe and domain, with new
     keys.
   - Mycelium's court law (78.132; 78.59 Ruling 2): an origin is held once per
     domain and never edited, a second reads `AlreadyFounded`, and a reset
     does not change that.
   - Era 0 (domain 5deab117, the box universe in `var-g2`) is archived as its
     record. Its keys stay intact and are never reused for era 1.
   - The new origin is version 2 (78.139): curvature radius R in clear, the
     parent face unwitnessed (a root) or the core tube it bounced out of.
2. **Circuitry, never steps** (xylarium W21, adopted).
   - No state is carried beat to beat by `*_step` integrations. A value at a
     beat is its closed form at that time t: orbits, phases and rotations read
     at t, and exact overlaps instead of diffusion.
   - The beat may remain as the bell's count of turns.
   - Ported layer by layer, each port measured by its readings, never as one
     rewrite (coordinator 2026-10-13; xylarium's pattern in its plan 10.10).
3. **Timing.** Mycelium's queue: its questioning work, the unlinked mind, the
   peer universe's founding (with netstratum's N7 core tube), then A1, then
   A2.
   - A1/A2 move ahead only on the coordinator's word. Not requested.
   - Era 1's geometry waits for them; the circuitry port does not.
4. **Time and the observers** (docs/80): time as a coordinate on the map,
   cosmic time from the epicenter's bounce, and TUniversal's origin and a
   quantum lane from the singularity, proposed to mycelium through the
   coordinator (2026-10-13).

## Build order (after the rulings and A1/A2)

1. **The geometry library** (mycelium's A1, from `vendor/mycelium`): places,
   reflections, cones; netstratum adds the geodesic angle by half-angle
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
   (witness: the conservation line against the tube's TRAVERSE records).
2. Motion: galaxies closing their own years, 0 → every galaxy, at differing
   beats; the 60-beat close's call sites, 4 → 0.
3. Places on S^360: places held as sphere points, 0 → all; `torus_distance`
   call sites, 22 → 0.
4. Distances: angle readings in the hub, melt and gate lines in place of
   cells; gates walled by span, 3 → 0.
5. Origin: the court's record reads `Sphere { 360, R }` for netstratum era
   1's new domain. Witness: the court's origin reading.
