# 61 — Orbits: the cores pull

Operator 2026-10-08: the cores carried no gravity; align with barren-realms'
singularity work (Einstein-Cartan bounce, the field law) where it applies.
Coordinator rulings relayed the same day: the pitch law
`pitch = core wavelength x (M_hole / M_core)^(1/3)`; turns only by rational
points of the circle, never a float angle.

## The rulings

1. **Lines orbit their core, in closed form, on sheared ellipses in three
   dimensions** (operator, 2026-10-08: "orbits should be sheared in order to
   create ellipses, operating on perpendicular axes in three dimensions").
   A line's place is `centre + M q(s)`, a function of the beat, never
   stepped:
   - `q(s)` is the universe's own circle, an L1 diamond of radius `r` in a
     two-axis parameter plane, at arc `s = s0 + v * t (mod 8r)`, `t` the line's own clock (its proper
     time, from its founding; it persists, so the place is continuous):
     piecewise linear and exact, its denominators bounded for ever (a
     rational rotor applied n times would grow them without bound: the
     scalar-collapse class, refused);
   - `M` is a fixed rational 3x2 map: the plane's two axes are the two
     carrying the line's largest founding offsets, the third is the
     perpendicular; an in-plane SHEAR `h` makes the circle an ellipse and a
     TILT `k` onto the perpendicular axis inclines it in three dimensions.
     `h` = the founding height over the founding radius, `k` the height
     carried by the first axis: derived from where the line was founded,
     never tuned. A line founded in the disc plane orbits round; one
     founded above or below it rides a sheared, inclined ellipse.
   One speed for every radius: the inner orbits turn faster, and the
   shear between neighbours is emergent. Neighbours, speech, the market's
   distances and the bell all read the place at the beat.
2. **The speed is declared, not tuned:** the measured flat rotation speed
   over light, 220 km/s / c ~ 11/15000, in cells a beat (the bell, light,
   is one cell a beat).
3. **Elements sorted by gravity** (barren SCALES): an element spreads from
   the core over a length inversely proportional to its mass; heavy
   elements near the core, light everywhere.
4. **The throat's keeper** — barren-realms' rule as its code has it
   (gateways.hub_contest / hub_keeper), one law across the two kernels:
   among lines with at least one hub-bearer (a people standing inside the
   core's bulge, by the universe's own distance, at its current home), the
   line whose network holds the most systems keeps the throat; ties by size,
   then name. Depth in the bounce wave (1/r^2) picks only the line's rider.
   Recomputed every season; no tenure. Feeds the ancient's wake (cores
   slice 4).
5. **Torsion's pitch** follows the coordinator's law above when the field
   law's pulse reaches netstratum (owed with the field port).

## Task list

| id | slice | status |
|---|---|---|
| G1 | lines orbit their core on sheared, inclined ellipses in 3D, closed form, exact | landed |
| G2 | elements sorted by gravity: deposits by distance to the core | open |
| G3 | the throat's keeper: largest network among hub-bearers; depth picks the rider | landed (docs/68, `ancients.rs`) |
| G4 | the field law's pulse: bell screws from the bounce, closed-form arrivals, the star re-locks its worlds with exact books, anchors carry unlocked shear (spec below, from the coordinator 2026-10-09) | open |

## The field law (G4), as the coordinator built it (2026-10-09, relayed as text)

Exact over Q in Cl(4,1). Lengths in kpc between bells, AU inside a system.
Energy in Earth-mass AU^2 (yr/2pi)^-2, so an orbit's stiffness is M/a^3 (M in
suns, a in AU). Every derived value is put on a 60-bit grid (round down)
before it enters the algebra; past that, everything is exact.

1. **Bell screw.** pitch = lambda_core x (M_hole / M_core)^(1/3), lambda_core
   = c x the core's round trip to the rim (2 r_max / c); period = pitch / c;
   ring k sounds at born + k x period. Axis: the core's is the disc normal,
   spin the hole's Kerr a; a stellar hole's is its progenitor's orbit normal.
   At a receiver d away: r_g = G M / c^2, t = grid(spin x r_g / d), rotor
   R = 1 + t B (B the axis bivector, turning 2 atan(t |axis|)); translation
   along the axis shift = pitch x AU_per_kpc x th / 2pi; pulse V = T(shift) R.
2. **Arrivals.** delay = distance / c; rings(t) = floor((t - born - delay) /
   period), -1 before the first; arrivals in (from, to] = rings(to) -
   rings(from), clamped at 0.
3. **Absorb.** E_in = L x period x sun_year x a_out^2 / (4 d_AU^2). drift =
   V(origin); per world c = V(at), dx = (c - drift) - native; work = 1/2 m M /
   a^3 |dx|^2; need = sum. gravity = min(E_in, need), held = gravity / need;
   rest = E_in - gravity, burned = rest x share (the star's own fusion step's
   mass defect: hydrogen, helium, or 0 for a remnant), light = rest - burned.
   Books: in = gravity + light + burned, exactly.
4. **Anchor.** native = (a,0,0); at starts at native; after an absorb at =
   native + grid((1 - held) dx) (only the kept shear is rounded, so a wholly
   re-locked world sits exactly on its native place; coordinator fix,
   2026-10-09). Source: xylarium-kernel crates/xyl-cosmos/src/field.rs and
   crates/xyl-kernel/src/field.rs (read access authorized). A star that pays for all of it puts its
   worlds back on native.
5. **Drift.** Per ring the star's drift += V(origin); the system moves
   together, no orbit shears from it.

netstratum's adoption: V acts on the anchor point of a line's orbit map M
(docs/61 G1) exactly as in 3-4, dx from the native point on M; the stiffness
radius is the orbit's L1 radius r (stated here as the semi-major measure).
