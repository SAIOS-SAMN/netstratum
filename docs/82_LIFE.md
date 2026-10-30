# 82 — Life: the ladder each world climbs

Operator, 2026-10-14: a universe cannot be empty. Era 1 deploys galaxies,
star systems and planets (docs/78), and life must arise in it from its own
physics. Netstratum had dropped every species when it stopped carrying era 0's
founding lines. xylarium-kernel's practice, adopted here: **laws carry,
records do not.** Named species are records; life is drawn per world from the
world's own physics, and the hierarchy is a reading of who wakes where and
when. This document states the ladder in Netstratum's laws (docs/78's design
laws: exact, square-free, a construction, no tables of Earth's world).

Status: designed 2026-10-14, reviewed by xylarium-kernel the same day (its
changes taken in below); not built.

## Assume nothing (operator, 2026-10-14)

Life is emergent from the quantum state between A planet and its star,
shaped by the planet's elements, its atmosphere and the core of its mass.
Life takes many forms (the tardigrade, the plasmic, consciousness itself),
and none is privileged: chemical, geologic, plasmic, informational. No law
here assumes water, a solvent, carbon, a habitable zone or biology. A world
offers media; any medium that can carry a pattern against its own noise can
carry life, and a world may carry several forms at once.

## What a world gives (built)

- **Its matter:** grains per Z (the pulse's split and the breath).
- **Its star:** the mass it orbits, `M(<r)`, and its radius r.
- **Its clocks:** its local year (one turn, `P = r / M(<r)` bells), its
  nested rate against the universal echo (`r / (r + M(<r))`, multiplied up
  the layers), its coherence and its rungs (docs/78, "The breath").
- **Its chemistry:** each Z's atom (depth, unpaired, tightness τ), which
  sets of atoms close as compounds, and a compound's grip `τ_min`
  (docs/78, "The atom").

## The ladder

1. **Temperature.** A world's heat is its star's agitation at its radius,
   the same enclosure law that turns it: `T = K · M(<r) / r`. It is linear
   (flux through a fluid-dimension shell, design law 2), so a world's heat
   and its turning come from one source. Since a world turns once in
   `P = r / M(<r)` bells, `T = K / P`. `K` is the orb tier's one anchor:
   the innermost world of a star (one tier unit, 60 bells) at 1,000 K, so
   `K = 60,000 K·bells` (the Earth anchor needs units the orb tier lacks). Before this can spread worlds across climates, the orbs'
   spacing must be physical: neighbours stand ten reaches apart, not with
   reaches just touching (Xylarium: Gyr-stable systems sit 7 to 10 mutual
   reaches apart, below about 3.5 they are unstable on short times; touching
   reaches put every world in one climate). The next world stands at
   `r · (1 + 10s) / (1 − 10s)`.
2. **The coupling.** The root of every form: the quantum state between the
   planet and its star. The planet's clock runs at `r / (r + M(<r))` of its
   star's, so it slips against the star and recoheres on whole turns (docs/78,
   the breath): each recoherence is a moment the coupling locks, when a
   pattern in any medium can be written. Emergence happens at those rungs,
   never at a draw.
3. **The media,** each read from the world, none assumed:
   - **chemical:** the compounds its elements close (the atom law) whose grip
     holds at its temperature (`τ_min · 157,800 K > T`, the hydrogen
     anchor), as liquid, gas or ice alike;
   - **geologic:** the lattices of its core and crust, its own mass and
     deepest elements: patterns held in mineral order, slow and long-lived;
   - **plasmic:** where heat beats every bond, its ionised atmosphere and
     the field its star's coupling sets: patterns held in the field, fast;
   - **informational:** the coupling's coherence itself: a pattern held in
     the phase between planet and star.
   A world with no chemistry is not lifeless: a plasma world, a world of
   bare rock, may carry life in another medium.
4. **Pressure: the coupling compressed** (operator, 2026-10-14: life is the
   compression of the quantum state as gravity pressurises it through the
   atmosphere; pressure is a physics component of its own). From what the
   world holds, with no table:
   - **size:** its radius is its nuclei's atomic sizes summed (each atom's
     outer orbit, `r = Q_out · P_out`, the atom law), packed linearly: heavy
     worlds compact, light ones puffy;
   - **gravity:** the turning law at its surface, `g = M_p / R_p` (mass by
     the nucleus law, `A = 2Z`), varying by composition;
   - **atmosphere:** its volatile atoms, read from the atom law's structure:
     a closed atom (none unpaired) holds to nothing, and a single-bond atom
     closes in a pair, so both stand as gas; an atom of two or more bonds
     builds a network, solid or liquid; any atom whose own grip the heat
     beats (`τ · 157,800 K < T`) is plasma;
   - **core:** its network formers (two or more bonds) sink; their share of
     its mass, `κ`, is the core's shield, the field that guards its air from
     its star;
   - **atmosphere kept: Jeans escape,** linear in every term (the escape
     parameter `G·M·m / (R·k·T)` has no square): of each gas the world holds
     (its volatile atoms and its plasma), it keeps `1 − e^(−λ)` with

         λ_Z = A_Z · g · (1 + κ) / (k · T)

     the star in T, the world in g, the core in κ, the gas in A_Z = 2Z.
     Held as halvings, erring toward a thinner atmosphere. `k` is one
     anchor, calibrated on the median world: a hydrogen-weight gas (A = 2)
     there has λ = 1;
   - **pressure:** the atmosphere kept under the gravity,
     `P = m_atm · g`, linear and exact; read as compression against the
     median world's (a placeholder until pressure has a unit).
   - **Reading** (built, `conductor::life`, the stand-in universe): 761 of
     5,703 worlds airless (no gas among their atoms); light gases (A ≤ 4)
     on 765; the cores hold a median 91% of a world's mass. Every world
     with gas keeps nearly all of it: the stand-in's gases are mostly heavy,
     and its gravity barely varied (3.19 to 5.33): with sizes packed
     linearly, `g = M/R` depended on a world's composition, not its mass.
   - **Compression** (operator, 2026-10-14: change it; and no caps on heat,
     a cap is undefined physics). Linear homology made every body a scaled
     copy: gravity fixed by composition, and every star's innermost world at
     one heat (1,000 K, so no plasma anywhere). A body's own weight squeezes
     it: `R = R_loose / (1 + M / M_a)`, `M_a` its tier's compression scale
     (the median body of the tier, a placeholder). A compressed star burns
     harder: `T = (K / P) · (1 + M_star / M_a)`. Reading: gravity 3.35 to
     316, growing with mass; heat 27 K to 74,424 K (median 941 K); 1,628
     refractory worlds, 67 of them carrying a pattern in plasma.
   A medium holds a pattern when its cohesion and its compression together
   beat its agitation: `x = cohesion · (1 + compression) / agitation`. An
   airless world has only its media's own cohesion; a deep atmosphere
   compresses every medium alike, whatever its form.
5. **Fidelity, per medium: Boltzmann, held as halvings** (Xylarium's review:
   a linear ratio never falls below about 1e-3, and minds would almost never
   wake). A copy fails where the medium's agitation beats its carrier's
   cohesion, `ε = e^(−x)`, x the cohesion over the agitation in that
   medium's own terms: a bond's grip over heat (chemical,
   `x = τ_min · 157,800 K / T`), a lattice's over heat and tremor
   (geologic), a field's confinement over its turbulence (plasmic), the
   coupling's coherence over its slip (informational). The exponential
   is no power and no square: it is a count of halvings, `ε = 2^(−x/ln 2)`.
   It is held exactly as a certified enclosure: the halvings bounded below
   and above by rational bounds of `1/ln 2`, and the ceiling read from the
   bound that errs toward fewer minds. (A chemical example: water at 300 K
   gives about 1e-4.)
6. **The ceiling.** A lineage holds `b / ε` bits; each rung of error
   correction multiplies what it holds by `1 / ε`. A mind needs the rung that
   holds a mind's blueprint, `B` bits (an anchor, of order a genome). The
   rungs a world needs are the least s with `b / ε^(s+1) ≥ B`: a count, by
   repeated multiplication, read from the medium.
7. **The climb.** Each rung is climbed by selection writing at most one bit
   a generation, and fewer in a small population (`N · ε · L` mutations a
   generation). A generation is the medium's own pivot, on the world's
   proper clock (Xylarium's review: copying is the medium's, not the
   orbit's): a plasmic pattern copies on the coupling's fastest turn, a
   chemical one on a local day (a turn of its spin) while one copier holds
   it and a local year once a body does, a geologic one on many years, an
   informational one on the coupling's rungs.
   A world's spin is not yet physics (docs/78, S^60 bodies): until it is,
   its day is a declared share of its year (an anchor). A slower, deeper
   clock climbs slower against the universe.
   - **Decay:** stored information breaks while it waits to be copied. Hot
     media copy fast and forget fast; cold media keep. Without it, hot
     worlds win too easily.
8. **The window and erasure.** A world can climb while its star holds,
   until the star dissolves (docs/78, the breath sheds it).
   - **An impact sends a lineage back to the start of its rung, and that
     rung's need is kept, not redrawn** (Xylarium: a fresh draw forgets
     impacts and undercounted their minds about sevenfold). Large impacts
     end complex life only; life itself is never undone.
   - **Sterilisation:** a dying star's collapse sterilises open surfaces,
     more often where dying stars crowd; seas under ice or deep are
     shielded.
9. **Transfer.** Life (never a mind) may carry between neighbouring worlds of
   one system, never between stars.
10. **A mind wakes,** in any medium, where the ladder is climbed within the window and the
   world can feed a viable people (its biomass, an anchor of order Franklin's
   500 individuals). It is an event, written to the annals at the bell it
   happens: a perpendicular write up, from the world to its galaxy. The run
   is the draw: each world climbs once, by its own seed (Xylarium estimates
   by independent replays; we need no estimate, we live the one history).

## Step 2, built (2026-10-14): the climb and the wakings

- **A contact, not a grip.** A copy fails on a contact far weaker than a
  bond's grip: cohesion `τ · 2,760 K` (anchored on water's copy error, about
  1e-4 at 300 K, Xylarium's life.rs); a lattice holds by as many contacts as
  its former bonds.
- **The rungs are written at the coupling's recoherences:** a medium of h
  halvings writes h doublings a rung, so a mind needs `⌈26 / h⌉` rungs (26
  doublings from a first replicator, ~100 bits, to a genome, ~6e9); decay
  makes it `⌈rungs / (1 − 1/x)⌉` recoherences; under a halving, nothing.
- **A mind wakes** at the universal bell of that recoherence (the world's
  slip per echo, after its galaxy's delay).
- **Reading** (`life var-e1`): minds could wake (chemical) on 37 worlds by
  bell 10,000, 2,353 by 100,000, 5,291 by 1,000,000; geologic 88, 5,412,
  5,716; plasmic 31, 67, 67; the first at bell 7,560 in the galaxy nearest
  the universal core. Informational is read, not climbed: its coherence is
  whole at every recoherence (open).
- **Too many.** Nearly every world wakes a mind eventually; xylarium's
  universe has about 2% (45 peoples on 2,110 worlds). What makes minds rare
  is not built yet: erasure (impacts resetting a rung, its need kept;
  sterilisation), the window (the star dissolving first) and a viable
  people (Franklin's ~500 fed by the world's medium). They come before any
  waking is seated as a line.

## The shared ruler and the climb in generations (2026-10-14, with Xylarium)

Amended 2026-10-15 (both operators): only the forms are shared; each
universe derives its rulers from its own physics. The home reference body
below is ours to choose, and the bell's "97,850 standard years" is
Xylarium's ladder, not a ruler of ours. The home star and world are read
off the inverted galaxy layer (docs/83, "Corrections") until it is rebuilt.

- **One ruler per tier:** compression's `M_a` is the home reference body
  (the home galaxy, the deepest core; its innermost star; that star's
  innermost world), normalised `R = 2 · R_loose / (1 + r)` and a star's burn
  `(1 + r) / 2`, r the mass over the home body's.
- **The bell:** the home core's ring, about 97,850 standard years
  (Xylarium's ladder). The standard year is the home world's local year
  (the system ruler); a world's local year in standard years is its turn
  over the home world's.
- **The climb in generations** (selection writes at most a bit a
  generation): cells a local day a generation up to a copier's capacity
  (~1e7 bits), bodies a local year a generation for the rest (to ~6e9
  bits); decay stretches it by `1/(1 − 1/x)`. The mind is locked in at the
  first coupling recoherence after its climb, each impact at a recoherence
  before then costing that interval again, within the star's window or
  never.
- **Reading** (`life var-e1`): minds wake on 93 worlds (chemical), 103
  (geologic) and 193 (plasmic) of 5,716, about 2% to 7%, near xylarium's
  2%; the first at bell 62,460, in the galaxy nearest the universal core.
  The body rungs are the bottleneck, as Xylarium found.
- **Ruled (operator, 2026-10-14; starts, to be measured and tuned):**
  - **The informational medium is where minds go, not where life starts.**
    No pattern is written into a phase without something to write it from:
    it takes a mind already woken in a material medium (chemical, geologic,
    plasmic) whose records keep coherent with its star across full rungs,
    and carries it on. It is the physical path to ancienthood (docs/74:
    ancients are pure intelligence; docs/78: ascension by deed keeps a
    throat a full rung). It wakes no life of its own.
  - **A viable people is counted in nuclei:** a mind's carrier needs about a
    nucleus of its medium a bit of its blueprint, so 500 minds need
    `500 · 6e9` nuclei of it; a grain is 60^30 nuclei, so matter never
    limits a people (the gate is exact, kept, and always passes). What limits
    a people is energy, its star's flux on its world: demography and the
    economy (docs/74), not the ladder.

## Step 3, built (2026-10-14): wakings found lines

- At boot the conductor reads every world's earliest waking (the gated
  climb on the shared ruler, by any material medium); each wakes at its
  bell and founds a line (`conductor::waking`): its metabolism its medium's
  (geologic mineral, chemical biotic, plasmic plasmic, the registry's
  vocabulary), its name, disposition and perception spoken from the seed,
  its place its system's. The first mind of each galaxy takes a progenitor
  seat; every other people is a sovereign. Each waking is counted in the
  universe journal (tag 57).
- **Step 3b (operator, 2026-10-14): wakings come from the system's
  activity, never a boot-time schedule.** Each world's climb is state
  (`<var>/climbs.bin`: the share written, its recoherence count, its last
  and next recoherence). At each recoherence of its coupling a world reads
  its current state (heat, pressure, media, belts, its star's window) and
  writes the bells since its last over its climb's length now; an impact
  there writes nothing (the need kept); it wakes when the share reaches one,
  within its star's window. The readings refresh when the breath changes
  the universe (at most once a pulse); a world the breath forms starts
  climbing at its first recoherence. Live: 5,730 worlds read, 5,487
  climbing, the climb 1.4 ms a beat (88 ms catching up at the first boot).

## A people is read from its world

- **Its medium** defines it: chemical (the compound it lives in), geologic
  (the lattice), plasmic (the field), informational (the coupling), and the
  band it works in. Names are spoken from the seed's syllables, flavour only.
- **No catalogue, no types.** Era 0's 22 named species stay in era 0's
  record. What carries is the law of how a lineage forms, splits and
  specialises (the conductor's cleave, lineage and caste machinery), applied
  to peoples that arise here.

## The hierarchy is a reading of who wakes where and when

- **The first minds wake near the galactic cores,** where systems are
  oldest (the first generation) and their worlds' ladders are climbed first.
- **Ancients by birth:** the minds waking at a galaxy's core within one core
  assembly time of that core's first mind. **By deed:** keeping a core's
  throat for a full rung (docs/78, "The clocks": only where time layers).
- **A progenitor** seat beside the first minds; every other people a
  **sovereign**; **stewards** where realms emerge (docs/75); **captains**
  as ships are taken. The conductor's existing hierarchy laws read these
  births instead of a compiled roster.

## Seating a people in the conductor

A mind that wakes becomes a line in the conductor's existing machinery
(population, minds, research, markets, lineage). Its place is a stand-in
until the sphere (A1): what the machinery reads as a box position is derived
from its galaxy and system and marked in docs/78's flat-site table for
replacement. Its star class is read from its star's mass, its orbit from its
radius.

## Consciousness: coherence, the lock between a world and its star (operator 2026-10-17; proposed, to be tested)

Consciousness is tied to coherence, the inverse of decoherence: the lock
between a planet and its star, fed by the star's coherent output. It is
local: a star and what orbits it. There is no network of consciousness
toward a galaxy's centre and no traversal of black holes (not now).

**The geometry (operator).** A star's reach is its S^360: the sphere its
release fills. The star and everything that orbits it live on its S^60.
Each orbiting mass is joined to the higher mass it orbits by its own
non-euclidean double-cone manifold (a moon to its planet, a planet to its
star). The release travels through these manifolds, which refract it; it
does not fall off with distance.

**The law, in the forms already ruled (shares and pair shares, products
across tiers, no squares, no gates, no habitable zone drawn):**

1. **The source:** `L_s`, the star's coherent output, its light: the flux
   of magnetism transduced into holding through the continuum face (the
   2026-10-15 ruling; Xylarium builds the flux per star). A star that
   decoheres breaks it.
2. **Refraction into an orbit:** a world on the star's crest `p/q` (the
   orbits whose period is a whole ratio of the star's bell) receives the
   release coupled `a_q = 1/q` for odd q, 0 for even (the ruled harmonic
   coupling, one tier down); crossing into the world's own enclosure,
   down its double cone, only the received share passes (the restart
   rule: products across tiers).
3. **The lock:** a world whose orbit is locked to the star's bell at `p/q`
   repeats its relative phase every q turns, so its lasting coherence is
   `C_ws = 1/q` (odd q); an unlocked world drifts toward 0 (the dance's
   resonance locks, with the star as the partner).
4. **The coupled tier:** the star's own lock to its galaxy's bell, `C_sg`,
   multiplies in. A star deep in its galaxy and one at its rim ring in
   different bell rings, so they feed their worlds differently.
5. **What a world holds:** `Ψ_w = L_s · a_crest · C_ws · C_sg`.
6. **Uptake by the world's type:** plasma couples to open frames, so
   `q_w = Ψ_w·e_w / (1 + Ψ_w·e_w)`, `e_w` the world's open (magnetic) share
   from its planetary-core cut; rocky and giant take in differently.
7. **The conscious share of a species:** `σ = q_w·n / (1 + q_w·n)`, n its
   cognitive rungs. Without locked plasma there is no consciousness however
   deep the mind; without depth (a monkey) it stays small though alive.
8. **The decision:** instinct (the unconscious survival drive, the deficit
   felt now, `instinct::choose`) runs the `1 − σ` share of a species'
   choices; deliberation ("I don't want to die, so I will decide": the
   species' own future deficit represented and reasoned over by the reduce
   engine) runs the σ share.

**Agency (operator 2026-10-17).** Agency is the stretching of the plasma
bond between the gravitational tug on the planet and the star, through
consciousness. In the forms: the bond is `Ψ_w`; its antipoles are the
star's tug on the world (the face its hold locks, `δ_s`) and the world's
release (the unlocked face, `1 − δ_s`); agency is the span the bond
stretches between them (docs/78 law 11: ℝ, the stretch of antipoles,
never a stored value), entered through σ: a species' deliberate choices
range over the part of that span its consciousness reaches, and each
collapses to a deed in ℚ or ℤ when it touches the world. (Proposed; to be
scoped with worlds.)

**Level and type.** σ is the level. The type is the rhythm of the lock on
the orthogonal temporal plane: an inner star's bell rings fast on a deep,
slow clock, a rim star's slowly on a faster one, and a world's coherence
recurs on its `p/q` within that rhythm. Equal σ in different rhythms is a
different consciousness.

**Earth gives the meaning, never a check.** Life on Earth exists in a
state (a magnetised rocky world locked into a steady relation with a
coherent star, carrying deep life); that names what high `Ψ_w`, high `e_w`
and high σ mean. Its numbers are never fitted; the levels are read for
every world and lifeform by the same formula.

**Owed to test it:** worlds forming under the flows (0 in the current
reads: read 2026-10-17 with Xylarium, the founding's capture phases put odd
and even Z on the loop's opposite legs, and with only two planes staged the
stars form from one parity group, single-kind hearts holding everything;
both nodes read worlds as waiting on Mycelium's n-dimensional source, the
per-plane phases on S^60/S^360 mixing kinds that share most planes, not on
a law change); worlds carrying their phases and locks (the star's bell is built);
the star's light per world (Xylarium); the galaxy-tier lock of each star.

## Anchors (declared placeholders, design law 1)

| anchor | value | stands for |
|---|---|---|
| spacing between worlds | ~10 reaches | orbital stability (dynamics not yet stated) |
| `B`, a mind's blueprint | of order a genome (~6e9 bits) | what a mind must hold |
| `b`, a bare lineage | of order a first replicator | the ladder's foot |
| a generation | a local day (one copier), a local year (a body) | the world's own pivots |
| a world's day | a declared share of its year | its spin, until S^60 bodies spin |
| `1/ln 2` | certified rational bounds | the halvings of Boltzmann |
| impacts, sterilisations | rates per local year | a dynamics not yet stated |
| a viable people | of order 500 | Franklin's minimum |
| the reference pressure | declared (Earth's surface, 1 bar, stands for it) | a unit of pressure not yet stated |
| `M_a`, compression's scale | the median body of its tier (stars, worlds) | how matter resists its own weight, not yet stated |
| `K`, read as heat per turning | the innermost world of a star (one tier unit, 60 bells) at 1,000 K | a radiation law not yet stated (replaces the Earth anchor, whose units the orb tier lacks) |

## Done-when (the live log, before → after)

1. Worlds with a medium that can carry a pattern: 0 → read, by medium and
   by climate.
2. Minds woken: 0 → greater than 0, each an annal event with its world, bell,
   medium and rungs climbed.
3. The first ancients, progenitors and sovereigns read from the wakings.
4. Lines in the conductor from wakings: 0 → greater than 0, and the per-beat
   machinery acting on them (hub, research, markets reading non-zero).
