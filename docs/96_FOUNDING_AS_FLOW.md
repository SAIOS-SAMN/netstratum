# 96 — The founding as flow: bounce, push, tide, release, the void

Design direction (2026-10-26): a bounce from the core does not throw, it
pushes; the push is the incremental flow of tidal forces between the vacuum
and the continuum, which release the elements into the void, and the void
distributes them. A walk is forward and linear, not multi-axial, and loses
capacity. This document scopes the meanings and aligns them on the topology
for implementation.

Status: steps 1–5 built (2026-10-26); §7 the galaxy enclosure's master
algorithm, the law the code follows. Builds on docs/95 (three spatial planes and the R
loop). Replaces the founding's one-step throw and the board's placement.

## 1. The meanings, derived

Each term in plain words, then what it is in Netstratum's laws.

- **The bounce.** The core's bell: once an echo the core re-pulses at full
  strength in its own units (regeneration, 2026-10-16). It moves nothing by
  itself; it sets this bell's push.
- **The push.** The continuum's share of what the throat holds, `p = ½·u/hold`
  (open frames over closed, `galaxy_push`). It acts in every plane at once
  (the isoclinic pulse). It is an increment per bell, never a displacement.
- **The tide.** The flow between vacuum and continuum at a place: the vacuum
  holds `δ = H/(R + H)`, the continuum's face leaves `½(1 − δ)` unlocked
  (ruled 2026-10-16: the tide works on the unlocked face, never the held).
- **The release.** Each bounce, the push on the unlocked face passes a share
  of what the throat still holds from held to free. Of kind k at its reach,
  the share released at a bounce is `s_k = ½(1 − δ(R_k))` (D1 below). What
  stays held after n bounces is `(1 − s_k)ⁿ`, exact (`compound::after`).
- **Into the void.** The released share leaves the throat on the continuum
  stream and crosses the ring into the vacuum leg by the Möbius jetstream
  (`(δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)`, docs/94 §2): R's sign turns, time's
  sense with it (docs/95 §2).
- **The void.** The vacuum leg: not empty space but the medium of motes
  (direction of 2026-10-25: the ocean of space is made of continuously interlinked motes). At the universe tier it is the ocean
  (`Universe.ocean`); at a galaxy's, its gas on the vacuum stream; at a
  system's, its belts.
- **The void distributes.** Released motes travel about their core, each
  turning at its own rate in all three planes and on the R loop, and each
  lands whole where the pull is strongest within its swing (`ocean::land`;
  at a galaxy, the ring landing). Distribution is landing, never placement.
- **The angle comes from time.** Each bounce releases at the core's phase at
  that bell, in each plane. Successive releases leave at successive phases,
  so the void fills the sphere because the core turns between bounces. No
  draw and no offset: the angular spread is the release's own history.
- **A binding** (direction of 2026-10-27: "clumps" are the bonding of elements, renamed bindings): elements on one
  ring of the disc bound into one body by their holding and their open
  frames together (`λ = 2(m + μ)/R`, m gravity's share, μ the electrons'),
  where that bond, `λ/(1 + λ)`, meets their heat and the tide across them.
  It makes a body, never matter: nuclides are made at the bounce and in the
  stars (docs/97), clusters by chemistry inside a body. A binding that
  ignites is a star; one that does not, its members go on as they were.
  (Formerly "clump"; its strength `λ/(1 + λ)`, formerly "binding", is the
  bond.)
- **Bodies gather where the void's motes coincide.** Where landed motes
  stand together in reach (their bands overlap) and in every plane (the
  product of their in-step shares), they hold if their bond meets their
  agitation and the unlocked face (`flows::chains`, `Chain::holds`): stars in
  a galaxy, worlds in a system. Read for all motes at once, then moved; never
  a walk.

## 2. Aligned on the topology

| | universe | galaxy | system |
|---|---|---|---|
| core / throat | the Wellspring | its core | its star's heart |
| bounce | the universal bell | the core's bell | the star's bell |
| push | `½` (regenerated) | `p_g = ½·u/hold` | `p_s = ½·u/hold` |
| tide (unlocked face) | `½(1 − δ_u)` | `½(1 − δ_g)` | `½(1 − δ_s)` |
| release into the void | the cascade's release to the ocean | exits through the ring | outbursts and wind (`spray`) |
| the void | the ocean | its gas on the vacuum leg | its belts |
| distribution | landing in the galaxy pulling hardest | landing from the ring within the disc's span | settling inside the breathing rim |
| gathering | galaxies (the shells' beads) | stars (bindings of the disc) | worlds (`collide_system`) |

One grammar on coupled tiers: what a tier's void distributes is what the tier
below gathers from.

## 3. What is misaligned now (to retire)

- **The throws.** `flows::founding` carries every kind to its reach in one
  step, and `board::place` gathers stars and worlds at bell 0. Both place.
- **One spin.** `Mote.ell_o` is one spin for all three planes, so every orbit
  is one tilted circle (docs/95 built step 1 lean).
- **Six angles.** The founding's offsets `Z/2 + Z·j/3` put every kind in one
  of six phase groups.
- **Two-plane leftovers.** The binding's infall offsets read planes 0 and 1
  (`along(0)`, `along(1)`, breath.rs); the landing's crossing turning is a
  pair `(t_o, t_c)`.
- **The walk** (discarded uncommitted): a forward scan along R.

## 4. Fast forward, in closed form

- **The throat.** Of kind k, held after n bounces `(1 − s_k)ⁿ`, released at
  bounce n `(1 − s_k)ⁿ⁻¹·s_k`: one exact read per kind for any n.
- **A cohort.** Kind k's release at bounce n is one cohort: it leaves at the
  core's phases at bell n and turns by its own rate after; its phase at any
  later bell is `φ + t·rate` (`motion::Turning`), never stepped.
- **Landing.** A cohort lands once, where its swing meets the strongest
  pull, read in closed form at its landing bell.
- **Zoom.** Cohorts that have landed and stand in step are read as one
  (dynamic carriers, 2026-10-24: exact under observation, certified
  sectors when zoomed out). The cost of a read is the cohorts in motion, not
  the bells passed.

## 5. Decisions for the operator

- **D1, the release share.** Proposed `s = ½(1 − δ)`, the unlocked face (the
  tide's face as ruled). Alternative: `p·½(1 − δ)`, the push on that face
  (slower where the push is weak).
- **D2, the galaxy's void.** Proposed: a galaxy's own vacuum leg (its gas)
  distributes its releases, and only what passes its rim reaches the ocean.
  Alternative: every release goes to the ocean first.
- **D3, the founding's end.** Proposed: the founding runs until a throat's
  release in a bounce falls below one nucleus of every kind; the board is
  read there (the count of bounces emerges).

## 6. Plan (one commit each; each step's witness read before the next)

1. **Spin per plane.** `ℓ_k` on each spatial plane plus the R loop's spin,
   on motes and bodies; reach `R = Σ|ℓ_k|/H` (L1); the tape carries them.
   Witness: the spin ledger exact; a single-plane mote reads its old reach.
2. **Two-plane leftovers.** The binding's infall offsets over the three planes
   and R; the landing's crossing turning per plane. Witness: the identities
   exact on the current founding.
3. **The release.** The throat holds its grains; each bounce releases
   `s_k` of each kind at the core's phases on the continuum, through the ring
   (Möbius) onto the vacuum leg. `flows::founding`'s throw and the six-group
   offsets retire. Witness: mass and spin conserved per bounce; held after n
   bounces equals `(1 − s_k)ⁿ` exactly.
4. **The void distributes.** Released motes transit and land by the landing
   law at each tier. Witness: every mote lands whole; landed phases cover
   the sphere (read per plane).
5. **Gathering.** Stars from the galaxy's landed motes by the chains and
   bindings over three planes; worlds by `collide_system`. `board::place`
   retires. Witness: stars per galaxy, worlds per star, compositions, all
   read, none set.
6. **Fast forward.** Cohort reads in closed form; the board read at bounce
   N. Witness: N and 4N counters agree on the closed forms (re-derivation,
   never byte equality).
7. **Read.** The founding through D3's end: counts, worlds per star, every
   identity, cost per echo.

## 7. The galaxy enclosure: the master algorithm (operator 2026-10-26)

Every galaxy reading (its frame, the echo's, the flows', the merge's, its
exits) is one enclosure:

    g      = mass(core.total())                 throat + released + stars
    base   = 1                                  the core's state, set at the
                                                bounce, never drained
    bodies = { (ℓ_x, m_x/g) : each mote x }  ∪  { (ℓ_s, mass(s)/g) : each star s }
    enc    = Enclosed(base, bodies)
    ring   = base                               R = H, δ = ½
    rim    = 2·base                             the loop closes: the image
                                                2H − R reaches the centre

**The base is the core's state alone.** The state is the throat's grains (set
at the bounce, the galaxy's grains then), so the throat's unreleased hold is
never added to it. Counted twice it inflated the ring (to about 1.4 while the
throat held most of its grains): released matter stood inside the ring, the
luminal bound flipped about a thousand motes an echo onto the Möbius leg, and
the disc's rings were too thin to hold (the regression of 2026-10-26).

Each echo, in order:

- **Release.** For each kind k the throat holds, `c = p·w̄/w_k` (the push's
  carry) and `s_k = ½·c/(c + base)` (the tide on the unlocked face, read
  against the core's state); the bounce releases `nucleus(a_k·s_k)`, placed at
  `R = ring + span·c`, turning at `ℓ = R·enc.hold_within(R)`.
- **Luminal.** A mote with `R < ring` takes the Möbius jetstream:
  `(δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)`.
- **Exits.** A mote with `R ≥ rim` crosses into the ocean.
- **Rings.** Band pairs on the strip `ρ = R/ring − 1 ∈ (0, 1]`, images at
  `−ρ`, each stepping by `k(ρ) = (S/G)·(1 + ρ)/(1 − ρ)`, `S = base/whole +`
  the matter within `|ρ'| < ρ`, `G = 1 − S`.
- **Fragments, read in the binding's own enclosure** (the 2026-10-16 ruling,
  "shares read in the body's own enclosure"). A binding of arc w on a ring at R
  (about the galaxy's hold H, carrying m of its holding and μ of its
  magnetism, at heat a) binds by its own deficit
  `λ/(1 + λ)`, `λ = 2(m + μ)/R` (its half-width `w/2` dilated once by its
  share; the arc cancels). Against it: its heat and the galaxy's tide across
  it, the contrast of the galaxy's deficit at its edges, `w/(2(R + H))`. The ring parts into the
  largest bindings that still hold, `w_max = 2(R + H)·(λ/(1 + λ) − a)`,
  `⌊R/w_max⌋` of them, each by phase in the galaxy's disc plane; none where
  the bond does not beat the heat.
  Retired: the crest read against the galaxy's enclosure,
  `x/(R + H + x)` with the galaxy's full shear, which capped every ring's
  bond near 0.25–0.33 against a barrier near 0.48 (measured at bell 462,
  2026-10-26): no ring could hold, and no star formed.

Retired: the throat's unreleased hold in the enclosure's base (built
2026-10-26, `89dcadd`), and the release read against the push's own hold.

## 8. The unlocked face heats; it is never a barrier (direction of 2026-10-27: find the underlying problem, and apply formulas where constants or static variables stood)

The unlocked face `½(1 − δ)` was ruled a heating law (2026-10-16: "the face
the core does not lock heats"). Its heat already reaches the motes and worlds
as agitation: the light a core sends warms each body by the share its face
leaves unlocked at its reach (`breath::shine`, `l·(1 − δ)/whole`). Every hold
test also subtracted it as a barrier, so it was counted twice, and against
the galaxy's frame in the binding's own enclosure (a cross-tier read). It is
also scale-free: near ½ at every ring whatever the binding, so measured at the
home galaxy (2026-10-27) ring bonds 0.03–0.16 stood against about 0.30 of
face and no ring held.

One hold law, every tier: a body holds where its own bond meets its own
heat and the tide across it,

    λ/(1 + λ) ≥ a + tide

- **The crest** (§7): `w_max = 2(R + H)·(λ/(1 + λ) − a)`.
- **A chain** holds where `bind ≥ agitation` (its tide is already in its
  apartness, the edges' contrast `t_edges`).
- **Ignition of a grown body** (`settle_system`): `λ/(1 + λ) ≥ a` and
  `2a < 1` (the ignition share's own bound, `formation::ignition_share`).
- **Ignition of a galaxy's binding:** its agitation is its own, `a_own`; the
  face is no longer added.
- **A stretch's binding** (`formation::holds`): its agitation alone against the
  shear.

Retired: the face as a barrier in every hold test.

### 8.1 The double-pass inversion of the kicks (direction of 2026-10-27: the missing component is the manifold chain's inversion on the star's gravitational lock)

`formation::crossed` reads whether two neighbouring worlds, meeting
`N = a·|Δω|` times over their system's age, are turned across their gap. Their
encounter phase repeats with period q (the continued fraction of
`s = frac(ω_i/Δω)`), and over a period the hemisphere law's responses cancel
but for one residual, `c = in_step(q·ψ₀)` at the pair's phase offset ψ₀ (the
star's own phase common to both cancels, so the offset is read against the
star's lock).

The residual was summed with one sign forever, `⌊N/q⌋·c`, growing with the
age, so every pair eventually joined: 78 worlds standing at bell 704 fell to
5 at 880 (1363 joins; measured 2026-10-27, 2^55 grains).

**The double pass inverts it.** Nothing rests on one leg (`M∘M = id`): a
period on the forward leg is followed by one on the return leg through the
ring's image, where the residual's sign turns. Period k leaves `(−1)^k·c`;
the excursion librates about the lock instead of drifting. The greatest
excursion over the age, in kicks, with `P = ⌊N/q⌋`, `ρ = N mod q`:

    P = 0:  ρ
    P = 1:  max(q − 1, c + ρ)
    P ≥ 2:  c + q − 1

bounded, read once, never stepped: past two periods the age no longer
moves it. A pair joins where `ε·R·kicks ≥ r`, so only neighbours whose own
period's kicks reach their gap join; a resonance holds its worlds apart.

Witness: an old light pair at a 2:1 stands as it stood young; the greatest
excursion is the same at 2q meetings and at any later count.

Open (not changed here): within a period the kicks are counted raw (`ρ`,
`q − 1`), a ceiling; the hemisphere responses also cancel inside the period.

### 8.2 Only forming orbits meet; neighbours exchange echoes, never kicks (ruling of 2026-10-27: pulses and echoes, never kicks; only bodies still forming their orbits collide)

Measured (2^55 grains, NETSTRATUM_DIAG, bells 704–880): 746 joins in one
system, 742 of them at the pair's first meeting, the "kick" `ε·R` exceeding
the gap by a median 4.5·10^4 (to 10^11). The 77 worlds formed at bell 704
stood only because no pair had yet met (`N = 0`); at bell 880 they met once
and 77 joined into one body, which ignited. The kick was the impulse model,
the pull at the gap held over the whole synodic interval `1/|Δω|`: an
import, not Netstratum's grammar. §8.1's libration was sound but never reached.

**Pulses and echoes.** No body strikes another. Each re-pulses at its own
bell; its neighbour receives the echo at its place every ring, and answers
by its deficit read at the gap in the pulsing body's own enclosure,
`m/(r + m)` (m the pulsing body's share, r their gap). Per ring the pair's
turnings differ by `|ω_i − ω_j|` (`ω = δ = H(<R)/(R + H(<R))`, the turning a
ring received). The echo holds its neighbour where it meets that shear:

    max(m_i, m_j)/(r + max(m_i, m_j))  ≥  |ω_i − ω_j|

the crest's own law (bond against tide) one tier down: first degree, no
age, no kick, no ε. A held pair co-moves and joins.

**Only forming orbits meet.** A world whose orbit has closed is solid: since
it last stood it has turned `2ω·r_echo` (its closed-form advance over the
echo's rings), and its orbit is solid where that reaches one whole turn. Two
solid worlds never join, whatever their echoes: they answer each other's
pulses in phase (the double pass, §8.1) and keep their orbits. A pair joins
only where at least one is still forming: the pool's small bodies (each
kind at its reach) and any world not yet once round.

Retired: `formation::crossed` (the kicks over the age, ε, the encounter
period q) and `formation::libration`.
