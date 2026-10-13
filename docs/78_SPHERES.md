# 78 — Off the box: S^360 space, S^60 bodies, cone manifolds

Operator (2026-10-13): netstratum's space is a box of squares, a 64-cell
cube with its faces glued, measured by summed axis distance (L1), and called a
"torus". That is a misalignment: the universe is non-Euclidean. Netstratum
moves to the geometry xylarium-kernel and barren-realms adopted: an S^360
universe, S^60 bodies, cone manifolds (xylarium `docs/XYLARIUM-KERNEL.md` §10,
tracker W8). One geometry across the games lets tiers and core tubes bridge
between universes.

Status: scoped 2026-10-13; not built. Facts were read from the code that day.

## The geometry adopted (from xylarium §10, not reinvented)

- **Space: S^360** of radius R, the Big Bang's curvature.
  - A place is a rational point (361 rationals, by inverse stereographic
    projection: exact, dense).
  - Distance is the exact squared chord; the arc is enclosed only when a
    length is reported.
- **Measure concentrates.** Two unrelated places stand about a quarter
  circumference apart. Nearness exists only along CONES between masses.
- **Masses are cone points; cones carry gravity.** Gravity between two masses
  is flux through the cone joining them: `g = G_k m / (|S^k| r^k)`, with `k`
  set per scale from the bounce's torsion (k = 2 near a mass, toward 1 across
  many pitches).
  - The mass law: the cones' total deficit is fixed (Gauss–Bonnet–Chern), so
    total mass is a topological constant, never created.
- **Bodies: S^60.** Every self-gravitating round body (star, world, moon) is a
  61-ball. Matter lies mostly near the surface. Spin turns in up to 30
  planes. Insolation is concentrated about the terminator.
- **Cores are cone points and throats,** not bodies.
- **Motions are products of rational reflections** (Cartan–Dieudonné),
  never 2^361-component rotors.
- **The court** (mycelium, pending): `Sphere { n, R }`, with a place as n+1
  rationals checked exactly and a frame as a factored reflection product. No
  2-dimensional cross-section (the anti-flattening law).

## Every flat site in netstratum (inventory, 2026-10-13)

| flat today | where (count of sites) | becomes |
|---|---|---|
| positions as 3 rationals in a 64-cell box | `spec.position` (50), `registry` position, `starchart.card` (19), `galaxies.card` (9) | a rational point of S^360 per world and core; the cards retired for binary checkpoints |
| L1 distance summed per axis, wrapped at the faces | `ledger::torus_distance` (22), `l1_wrapped`/`wrapped_abs` (6), `conductor::spatial_l1` (14), `universe_span` (27) | the exact squared chord on S^360; no span, no wrap |
| a line's core: the nearest core within a reach in cells | `Ledger::core_of` (20) | the core whose cone the world lies on (the strongest flux), read from the cones |
| the bulge as one cell | `ancients::in_bulge` (3) | the core's throat region, from its cone (the core as cone point) |
| melt from summed distance | `cfe::world::route_melt`, `pair_melt`, `effective_distance` (12) | melt along the geodesic, and along cones (where locality lives) |
| gate span in cells, melt span/64, freshness 1 cell | `ascension` (`cells`, `gate_melt`, `FRESHNESS_CELLS`) | a wormhole as a throat between cone points; its span the chord, its melt from the cone (the walled spans over 64 cells vanish with it) |
| hubs as the medoid by L1 | `galaxy_hub`, `commissions::hub_of` | the hub at the cone junction of a galaxy's worlds |
| the commission's disc plane: a 2-D projection of a galaxy onto a ≤29-cell grid | `commissions::plane` | an anti-flattening problem: the commission poses the galaxy as it is (its cone graph), not a flat plane |
| orbs around a star at (r, θ, ψ) with wrapped-L1 separation | `ColonyOrb.theta/psi`, `cfe::dop::separation`, `ThermalSite` (orbit_r) | orbs as S^60 bodies on orbits read from cones (Kepler where k = 2, rosettes otherwise) |
| insolation, habitats, the thermal model on flat orbits | `cfe` thermal, Keplerian local years (27) | S^60 insolation (the terminator band), orbits' closed forms |
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

1. **The geometry library** (shared, from mycelium A1): places, chords,
   reflections, cones. Netstratum writes none of its own (as xylarium W10).
2. **The migration of places:** every world and core mapped onto S^360 so
   that today's neighbourhoods are kept as cone relations (each world on its
   core's cone). It is checkpointed, and the cards retired.
3. **Distance, cores, melt, hubs, gates** moved to chords and cones, one
   site group at a time (the table above).
4. **Bodies: S^60 orbs,** orbits and insolation.
5. **Era 1's founding:** a new domain with new keys and a version-2 origin
   with R, `Sphere { 360, R }` once A2 stands. Era 0 is archived.

## Done-when (the live log, before → after)

1. Places on S^360: worlds with a sphere point, 0 → all 101 (and every
   core); `torus_distance` call sites, 22 → 0.
2. Distances: chord readings in the hub, melt and gate lines in place of
   cells; gates walled by span, 3 → 0.
3. Origin: the court's record reads `Sphere { 360, R }` for netstratum era
   1's new domain. Witness: the court's origin reading.
