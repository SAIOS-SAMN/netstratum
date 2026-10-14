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
    half-angle `d_i` (its distance), a whole-number strength `a_i`, and a
    rational phase `φ_i` in turns (docs/79's torsion, docs/81's Φ).
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
- **Motion.** Every body turns about its centre by the law above, from its
  mode's phase `φ` at t = 0, in the plane its mode's reflections give. Each
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
- **The split's anchors and dials.** The tier anchors above are of order and
  owe citations (the anchor provenance law). The base-60 draws and the
  resonance chain are authored; the operator's recipes may tilt the shares.
- **The galaxies' sizes.** The epicenter's stick gives galaxies falling
  geometrically (core 1 about nine tenths of the formed matter); xylarium's
  siblings draw their hole within 1/4 to 2 of the home's, so galaxies are
  comparable. Whether ours should be bounded the same way is open.
- **Radial motion.** The law turns bodies; it does not move them in or out.
  Radii are the deployment's, so infall, accretion and escape are not yet
  physics. A square-free radial law is owed before any body changes orbit.
- **The clock's rate law** (docs/80): slower deeper in a well, stated in
  enclosed mass and radius with no square and no root (the flat-space
  `√(1 − 2GM/rc²)` is not used).

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
