# 83 — Formation: stars and planets form from matter, they are not placed

Operator, 2026-10-15: stars and planets form naturally from the physics of
matter; forces and gravity, spin and manifolds work together, as orbital
mechanics does in real solar systems. Era 1 deploys its bodies top-down (the
pulse hands out shares, a star keeps a ruled share, planets are set at reach
boundaries by construction; docs/78, "The mass split"), and each body then
only turns about its centre. This document scopes the change to bottom-up
FORMATION.

Status: scoped 2026-10-15; to be designed with xylarium-kernel (the
formation law shared, as the turning law is). Phase 0's spin ledger built
(a265022: mass and spin exact across 420 live beats). Phase 1 held on the
corrections below.

## Corrections (operator, 2026-10-15)

### The galaxy layer is inverted

Measured (`motion var-e1`): the deployment's tidal law applied galaxy →
star gives every galaxy 32 stars, the median about 7 times its hole, the
largest about 163 times its hole and 16% of its galaxy, and every galaxy
exactly 1,000 holes. The galaxy is a weak hole circled by a handful of
galaxy-sized stars: galaxies orbiting stars. The whole systems-about-a-core
layer is wrong.

- A galaxy is its hole, its gas and a POPULATION of stars, each a vanishing
  share of the galaxy, formed from the gas by collapse (Phase 1). The
  galaxy's mass lies in the population and the gas, never in a few bodies.
- The reach law (`m = μ·M(<r)`, worlds ten reaches apart) is how a disk
  fragments about ONE star. It never makes the stars of a galaxy.
- A population of stars is not a list of explicit bodies: it is the gas's
  collapse read as a distribution, and a star (with its disk and worlds) is
  formed as a body only where something observes it (compact until
  observed, below).
- Read off the wrong layer until this is rebuilt: the galactic year (its
  half-mass "system"), life's heat and burn (star masses of whole holes),
  the systems' clocks and the 416 bodies the breath carries.

### Our rulers are our own

The first ruling asked xylarium for physical unit rulers (suns, metres,
km/s, one speed constant per tier fixed by the Sun's galactic orbit, Earth's
year and the Moon's month; a thermal speed from hydrogen's 13.6 eV and the
proton's mass, growing as the root of temperature). Every part of that was
misaligned with docs/78's design laws, and none of it is adopted:

1. **No imported rulers.** The universe has its own: the bell (one ring of
   the universal core), light per ring, turns, the grain, the atom tier
   (the deepest hole as a mass unit is withdrawn too: "The derivation"). Suns,
   metres and eV are Earth's measures, an Earth table (design law 1).
2. **No constants per tier.** A tier is where structure formed, not a
   compartment with its own physics; one law reads at every tier with one
   set of rulers (design law 4).
3. **"No squares" is about geometry, not arithmetic.** A square states a
   flat, rigid dimension; ours are fluid. A thermal speed taken from
   ½mv² carries that flat geometry however its arithmetic is written
   (design law 2).
4. **Outcomes are read, never scored against nature.** The masses that
   collapse are what our physics gives; "a hundredth of a sun" is not a
   target.
5. **Speeds are readings, not primitives.** Motion rises from the quantum
   layer (design law 7): a speed is a phase turning per bell, read off the
   law.

xylarium concurs (2026-10-15, their operator's ruling): what the two
universes share is the laws' FORMS only (the turning law, `v = c·δ`,
`L = r·v`, the radial balance, the bond law, the chronometry chain). Each
universe derives its own rulers from its own physics, including the
home-body compression reference.

What actually blocked Phase 1 is missing physics, not missing units: the
THERMAL MOTION of matter was never derived from our own atom tier (where
temperature is a rate, `T = k_T·ω`). Derived there, on the same turning
law and rulers as an orbit, a gas's agitation and an orbit's turning are
comparable by construction. That derivation is Phase 1's first step.

### The derivation (operator and xylarium, 2026-10-15)

Constants are missing physics. The speed law `ω = M(<r) / r` makes speed
`v = M(<r)`, radius-free: unbounded (the 0.1 to 100 c measured), no
horizon, no runaway collapse, and at odds with the breath's clock law
`r / (r + M)`. The fix is not a ruler but the primitive: the STATE (a
bell's ring rate, phase and torsion winding) is primitive and mass is a
reading, the deficit `δ` a state's echo leaves in each well (docs/78, "The
singularities"). Then `v / c = δ = M / (r + M)` (xylarium's
`M = r·δ / (1 − δ)` inverted; reached independently here), bounded, with a
horizon at `δ → 1`, compactness restored (a clump's surface speed rises as
it shrinks, so collapse runs away and a Jeans length returns), and one law
for the clock (rate `1 − δ`) and the motion. c is light per ring; no G, no
mass unit.

The split, in order:

1. **xylarium:** the ring-density law (a bell's ring density at a place
   sets its clock rate and `δ`), exact, frame by frame. We adopt the form;
   the breath swaps its rate `r / (r + M)` for it.
   DELIVERED (xylarium 11.0, "THE RING-DENSITY LAW", 2026-10-15): every
   bell (a core, a star, a grain) HOLDS each ring that reaches it for its
   own period, then passes it on as its echo. A ring crosses empty space to
   r (in light-rings) in r and is held by every bell inside r for their
   periods together, `H(<r)`, so it arrives after `r + H(<r)`:
   `rate(r) = r / (r + H(<r))`, `δ(<r) = H(<r) / (r + H(<r))`. MASS IS
   HOLDING, `M = H`. Whole rings, so exact rationals, read ring by ring.
   Checked against the design laws: no square (linear-fractional), no
   constant (time a ring, length light per ring), construction (a ring is
   a spherical wavefront, so it crosses every bell inside r), frame by
   frame (ring counts), one law at every tier (core, star, grain).
2. **netstratum:** a grain's own ring (its mass reading from the atom
   tier's turning; A a reading) and thermal motion on the bell's clock.
3. **Then formation:** the bounce's spin (from the torsion winding) and
   hold (sorting core, disk, gas, escape), a galactic bounce's strength,
   the galaxy count as an outcome, and collapse (Phase 1).

### The spin under the ring law (measured 2026-10-15; open)

Built: the grain's own ring (`cfe::almanac::atom::hold`, holding belongs
to closed pairs: `2Z + (Z − unpaired)` rings; operator's lean, 2026-10-15)
weighs every grain, and the clock reads `crate::ring::rate`. The ring
law's MOTION was wired (period `r·(r + H)/H`, the balance
`r = ℓ·H/(H − ℓ)`, escape at `ℓ ≥ H`) and measured, then HELD:

- Under `ω = δ/r`, a circular orbit's spin `ℓ = r·δ = r·H/(r + H)`
  SATURATES at H: far from what it orbits every orbit carries nearly the
  same spin, and the radius `ℓ·H/(H − ℓ)` diverges as ℓ nears H. In nature
  `ℓ` grows without bound (as the root of r), so slow mass loss widens an
  orbit; here a loss above `1 − ℓ/H` of the holding inside unbinds it.
- On the pulse's orbits (`r ≈ 3600·H`) that threshold is 1/3600. A
  6,000-bell breath on the test universe (stars shedding up to 1/100 an
  echo) ESCAPED 1,894 systems and bore none (each birth takes the edge
  gas's spin, ≈ H, and escapes at once). It also cost 754 s against
  23 s under the held law (the balance near `ℓ → H` grows the rationals).
- The cause is the pairing of `L = r·v` with `v = δ`: our speed reads as
  nature's squared speed (`δ ≈ v²/c²` in the weak field), so `r·δ` is not
  the quantity rotation conserves. Which quantity the ring law conserves
  (its spin, read from the turning without a flat square) is owed before
  the motion goes live, shared with xylarium (`L = r·v` is a shared form).

## What we have, and what is missing

| nature's ingredient | built | missing |
|---|---|---|
| gravity, inward | the turning law `ω = M(<r) / r` (bodies orbit) | a RADIAL law: nothing falls, collapses, migrates or escapes (owed since docs/78) |
| pressure, outward | pressure for atmospheres (docs/82) | pressure resisting collapse in a cloud or a star |
| spin (angular momentum) | phases in turns; spin promised for S^60 bodies | conservation: nothing spins up as it shrinks, so nothing flattens into a disk |
| matter | grains by Z, the atom tier, chemistry, the medium (gas) | the medium never condenses into bodies; the breath forms systems by rule |
| manifold, torsion | specified (S^360 places, cone reaches, torsion phase) | the field matter falls through |
| orbital mechanics | each body about its enclosed mass alone | mutual tugs, resonances, migration, collisions, ejections |
| locality | each body reads its star and its own matter | its neighbours |

## The scope

### Phase 0: the missing forces, in the design laws

Linear, exact, closed forms between events (docs/78 design laws).

1. **A radial law:** infall and escape from enclosed mass against pressure
   and spin, square-free.
2. **Spin:** angular momentum carried and conserved through collapse and
   accretion; a shrinking cloud spins faster, the root of disks and orbits.
3. **Pressure support:** the atmosphere's pressure law (docs/82) resisting
   collapse.

### Phase 1: formation from the medium

0. **The derivation first** ("The derivation" above): the ring-density
   law (xylarium), a grain's ring and thermal motion on the bell's clock
   (ours).
   Then the galaxy layer is rebuilt from the bounce (docs/78, "The
   singularities"): each galaxy's manifest sorted by its bounce's hold,
   the heaviest held nearest its core (its innermost accretion disk), the
   light elements its gas, and the population of stars that gas
   collapses into. No tidal-law systems, no hole share. The root defect was
   the pulse's distribution of the elements, not the tidal law.
1. **Collapse:** a clump of medium collapses where its own gravity beats its
   pressure and spin (a Jeans criterion, square-free), replacing the
   breath's births by rule.
2. **The star and its disk:** what falls to the centre is the star
   (compressed and burning by the laws already built, docs/82); conserved
   spin leaves a disk.
3. **Planets:** the disk fragments where a clump can hold its own reach (the
   tidal law, docs/78), and bodies grow by accretion events: their number,
   sizes and spacing emerge.
4. **The leftovers:** belts and comet clouds are what the disk leaves
   unformed, real matter, not a share.

### Phase 2: orbital mechanics as events (circuitry)

- Between events every body moves in closed form, as now.
- ENCOUNTERS, RESONANCES (period ratios near small whole fractions, exact in
  rationals), MIGRATION, COLLISIONS, MERGERS and EJECTIONS are annal events;
  each re-bases the bodies it touches.
- A world's impacts come from actual bodies crossing its orbit, not a belt
  share; life's erasure (docs/82) reads them.

### Phase 3: one law at every scale

- Galaxies form stars from their medium the same way; the breath's echoes
  are the compression waves that trigger collapse (as spiral density waves
  do).
- One formation law for clouds, stars, disks and planets (design law 4).

### Phase 4: what moves over

Everything built on bodies keeps working, because it reads bodies, not how
they were made: life (docs/82), chemistry and the atom tier, climate,
atmospheres, chronometry, and the civilizations seated on worlds. The pulse
shrinks to what only it should do: deploy the founding gas and the cores.

## The proposed laws (with xylarium-kernel, 2026-10-15; to be ruled)

xylarium's formation is also mostly placement today (golden-angle stars, a
catalogue IMF and disc profile, moons stick-broken on a 2:1 chain); its
package B (isolation-mass fragmentation, ~10 reaches apart) is designed and
part built. One formation law for both, stated square-free here.

- **The speed law.** Under the turning law a body's orbital SPEED is
  `v = ω·r = c·δ(<r)`: linear, the enclosed mass alone.
- **Spin (angular momentum)** is held as `L = r·v`, a product of two
  readings, conserved exactly on a ledger through every collapse, accretion
  and event. (xylarium writes it `ω·r²`, Kepler's swept area; held as `r·v`
  no square is formed.)
- **The radial law.** A body of spin L has its circular radius where its
  own speed meets the law's: `L / r = c·δ(<r)`, so `r_c = L / (c·δ(<r))`,
  linear in L and in `1/δ`. A clump that loses spin falls inward to its new
  `r_c`; one that gains mass inside its orbit tightens. Radial motion about
  `r_c` is the second pivot (its stretch e); ESCAPE is the crossing at
  e → 1. Infall, orbit and escape are one closed form with events at
  crossings.
- **The thermal ruler as a speed.** Temperature reads as a thermal
  agitation `v_th`: `T = k_T·ω` at the atom tier and `v_th` at the gas tier
  are one ruler read at two tiers. (Its anchor on hydrogen's 13.6 eV and
  the proton's mass, with `v_th ∝ √T`, is withdrawn; see "Our rulers are
  our own": `v_th` is derived from the atom tier's own turning.)
- **The balance (pressure support; closes Open 1).** Every term a speed:
  `c·δ(<r) = L / r + v_th`. A parcel's enclosed-mass speed is held by its
  spin speed plus its thermal speed. Gas with no spin falls until its
  thermal speed holds it: the STAR. Gas with spin settles at `r_c`: the
  DISK. Infall, pressure support and orbit are one exact linear balance;
  its crossings are the events.
- **Collapse (Jeans, square-free, every term a speed;** corrected by
  xylarium: the first statement mixed rates of different things). A clump
  of mass m and radius R collapses where its own surface speed beats its
  thermal and spin speeds: `c·δ(m) ≥ v_th + L / R`. Its spin speed rises as
  it shrinks, `v = L / R`; what its surface cannot hold (`L/R > c·δ(m)`) is
  SHED to the disk as L on the ledger, so the disk's L is the sum of what
  was shed, conserved exactly. That is how the disk forms.
- **Fragmentation.** The disk fragments where a clump inside it passes the
  same test against the disk's shear across its reach; planets grow by
  accretion events, their masses and the stars' (the IMF) read as OUTCOMES,
  never a catalogue.
- **Orbital events** are certified CROSSINGS: two bodies interact when their
  reach caps cross (share ½), never by pairwise integration; resonances are
  exact small-integer period ratios (rates are exact rationals). Each event
  re-bases what it touches; closed forms between.
- **Compact until observed** (xylarium 11.4f): a region forms its stars and
  worlds only when something with sight of it observes it, deterministic
  from its seed and medium, so unobserved regions cost nothing (theirs: a
  cold forecast 3,736 s → 110 s).
- **Advice kept:** mass and L conserved exactly across every event; no
  N-body stepping; no catalogue fits; the homology trap kept broken; no
  caps.

## Open

1. ~~The radial law's form~~: closed by the balance above.
2. ~~Event granularity~~: bodies interact at reach-cap share ½ and nowhere
   else; a resonance is an event read from exact rate ratios, not a
   proximity.
3. **Shared with xylarium:** the same place on both sides (placement today,
   the IMF and disk profile catalogue); one formation law, adopted by
   xylarium's package B once ruled.
4. **Cost:** formation is the heaviest physics so far; closed forms between
   events keep it bounded, but it must be measured early.

## Done-when (the live log, before → after)

1. Systems formed by collapse from the medium: 0 → all new systems (the
   breath's rule-births retired).
2. Planets per system, their spacing and masses: emergent, read against the
   current construction's.
3. Orbital events (encounters, resonances, collisions) in the annals: 0 →
   live, each re-basing what it touches.
4. Life's impacts from real crossings: belt share → bodies.
