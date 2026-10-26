# 96 — The founding as flow: bounce, push, tide, release, the void

Operator 2026-10-26: "each bounce from the core doesnt 'throw' - it pushes,
that push is the incremental 'flow' of tidal forces between the vacuum and the
continuum which release the elements into the void and the void then
distributes them - align on our topology"; "'walk' is forward, and linear -
not multi-axial: where are you losing capacity"; "scope it and derive the
meanings and align on topology for implementation and show plan".

Status: SCOPED, not built. Builds on docs/95 (three spatial planes and the R
loop). Replaces the founding's one-step throw and the board's placement.

## 1. The meanings, derived

Each term in plain words, then what it is in our laws.

- **The bounce.** The core's bell: once an echo the core re-pulses at full
  strength in its own units (regeneration, 2026-10-16). It moves nothing by
  itself; it sets this bell's push.
- **The push.** The continuum's share of what the throat holds, `p = ½·u/hold`
  (open frames over closed, `galaxy_push`). It acts in every plane at once
  (the isoclinic pulse). It is an INCREMENT per bell, never a displacement.
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
  (operator 2026-10-25: "the ocean of space is made up of continuously
  interlinked motes"). At the universe tier it is the ocean
  (`Universe.ocean`); at a galaxy's, its gas on the vacuum stream; at a
  system's, its belts.
- **The void distributes.** Released motes travel about their core, each
  turning at its own rate in all three planes and on the R loop, and each
  LANDS whole where the pull is strongest within its swing (`ocean::land`;
  at a galaxy, the ring landing). Distribution is landing, never placement.
- **The angle comes from time.** Each bounce releases at the core's phase at
  that bell, in each plane. Successive releases leave at successive phases,
  so the void fills the sphere because the core turns between bounces. No
  draw and no offset: the angular spread is the release's own history.
- **Bodies gather where the void's motes coincide.** Where landed motes
  stand together in reach (their bands overlap) and in every plane (the
  product of their in-step shares), they hold if their binding meets their
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
| gathering | galaxies (the shells' beads) | stars (clumps of the disc) | worlds (`collide_system`) |

One grammar on coupled tiers: what a tier's void distributes is what the tier
below gathers from.

## 3. What is misaligned now (to retire)

- **The throws.** `flows::founding` carries every kind to its reach in one
  step, and `board::place` gathers stars and worlds at bell 0. Both place.
- **One spin.** `Mote.ell_o` is one spin for all three planes, so every orbit
  is one tilted circle (docs/95 built step 1 lean).
- **Six angles.** The founding's offsets `Z/2 + Z·j/3` put every kind in one
  of six phase groups.
- **Two-plane leftovers.** The clump's infall offsets read planes 0 and 1
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
2. **Two-plane leftovers.** The clump's infall offsets over the three planes
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
   clumps over three planes; worlds by `collide_system`. `board::place`
   retires. Witness: stars per galaxy, worlds per star, compositions, all
   read, none set.
6. **Fast forward.** Cohort reads in closed form; the board read at bounce
   N. Witness: N and 4N counters agree on the closed forms (re-derivation,
   never byte equality).
7. **Read.** The founding through D3's end: counts, worlds per star, every
   identity, cost per echo.
