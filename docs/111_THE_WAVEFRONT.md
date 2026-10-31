# 111 — The wavefront: one expanding sphere, echoed at surfaces

This document replaces the relayed ring with a single wavefront. Each bounce
of the universal core emits one geodesic sphere that grows from S^0 at the
core towards S^360. Bodies the front passes receive it and emit nothing. An
echo arises only where the front meets a surface. The firmament is the first
surface, so the echo stays contained inside it until the firmament's lighter
kinds escape.

**Status:** scoped; not built. To be reviewed with xylarium-kernel, which
holds the same law (its `pulse.rs` relay through galactic cores and stellar
remnants), before either kernel builds it.

**Prerequisites:** docs/78 (S^360, the breath), docs/83 (the ring-density
law), docs/101 §2 (the firmament), docs/106 (the substrate), docs/110 §1 (the
wavefront and the equator, defined).

## 1. What the code holds now

| Part | Code | Reading |
|---|---|---|
| The law's statement | `ring.rs` header | "Every bell … holds each ring that reaches it for its own period, then passes it on as its echo"; arrival after `r + H(<r)`. A relay. |
| The universal period | `pulse::cascade`, `Universe::t_u` | `T_u = 2(R_out + H)`: a ring out to the **outermost galaxy shell** and back. The shell of galaxies is treated as the reflecting surface. |
| Arrivals at a galaxy | `breath::breathe` | One arrival per `T_u`, at `k·T_u + ⌊R_i⌋`: a front, delay `r` only (no `H(<r)` term). |
| The firmament | `pulse::firmament_of` | The home's motes that its throat holds wholly (`flows::held = 1`). The universal bounce torques and orders it each echo. It does not affect the front. |
| Escape | `flows::exits` | A mote crosses whole one tier up by its jet (`ℓ_⊥/H ≥ H`) or radially (`R ≥ H`). This happens at events. |
| A star's own bell | `breath` (`rang`) | It rings on the star's whole turns, `ω_s = h/(ℓ/h + h)`, and times the star's outbursts. It is the star's own oscillation, not a relay of the universal front. |

The clock `r/(r + H)`, the deficit `δ = H/(r + H)`, the turning `ω = δ` per
received front and the spin `ℓ = R·H` are not relays. They stand unchanged
(§3).

## 2. The law

Let the universal core bounce at `t = 0`. Its **front** at light-time `t` is
the geodesic sphere `W(t) = {x ∈ S^360 : d(core, x) = t}` (docs/110 §1). It
grows by one light-ring a bell.

1. **Delay through the held medium.** The front reaches reach `r` after
   `a(r) = r + H(<r)`, with `H(<r)` the holding enclosed, `r` in throat
   radii of the universal core and `H` in units of its state (1), so `a` is
   in light-crossings of the throat radius and the equator `δ = ½` stands at
   `r = H = 1`. Where `H(<r) → 0` this is a constant speed, linear in `r`
   (Xylarium's present `(r/rim)·bell/2`). This is the front's
   own slowing in the held medium, the clock's complement integrated along
   its path. No body holds the front and re-emits it.
2. **Reception, without emission.** A body that the front meets receives it:
   its clock advances, and it turns by its deficit `δ`. It emits no front.
3. **Surfaces.** A front meets a surface where it crosses the boundary of an
   enclosure that holds its own bounce:
   - **the firmament**: the sphere of reach `r_f`, the outer reach of the
     firmament's held spread (the greatest `R(1 + p)` among the motes of
     `firmament_of`). It is always inside the throat (`r_f ≤ 1`);
   - **another core**: a galactic core's throat sphere. On meeting it, the
     galaxy's core bounces into its own enclosure. That bounce is the
     galaxy's arrival, the event `breath` reads now;
   - **the S^360 boundary**: the rim, `R_rim`, the reach of the outermost
     shell the founding cascade stood (the value `T_u` already reads).
4. **The echo.** At a surface the front parts into a transmitted share `τ`
   and a reflected share `1 − τ`. The reflected share converges on the
   surface's source. When it arrives there, the source bounces again. A
   core's throat transmits nothing outward (`τ = 0`): what enters a galaxy
   stays in its enclosure. The rim reflects wholly (`τ = 0`).
5. **The firmament's transmission.** The firmament transmits the share of
   its founded holding that has escaped:

       τ(t) = left(t) / founded,   founded = held(t) + left(t)

   with `founded` the firmament's holding at the founding (weighted, as in
   `flows::weight`). `left` is what has crossed out by `flows::exits` since
   then. `τ` is 0 until the first exit and rises only at exit events. It
   tends to the unsealed share: kinds that never reach `R ≥ H` stay sealed,
   so `τ` never reaches 1.

### 2.1 The two periods

- **Contained** (`τ = 0`): the front runs from the core to the firmament and
  back, so the bounce period is

      T_f = 2 · a(r_f) = 2 (r_f + H(<r_f))

  in whole bells (ceiled, as `T_u` is). Each contained bounce torques the
  firmament: the existing firmament torque, run once per `T_f` and no longer
  per `T_u`. Nothing outside `r_f` receives a front. No galaxy breathes from
  the universal core.
- **Open** (`τ > 0`): the reflected share keeps the inner bounce at `T_f`.
  The transmitted share runs on to the rim and back, `T_u = 2 · a(R_rim)`. A
  galaxy at reach `R_i` meets the outbound front at `a(R_i)` and the rim's
  reflection at `T_u − a(R_i)`, two arrivals a `T_u`. Each arrival carries
  the strength `τ` at its emission.

### 2.2 Strength

The front's strength `τ` scales what an arrival does outside the firmament:

- the bounce's push on a galaxy's media: `½ → ½·τ`;
- the share of the medium that an echo compresses: `c_g · medium → τ · c_g · medium`.

The torque law is unchanged. Only its amplitude changes, so `Σ m·ℓ +
reservoir` stays exact. Inside the firmament the strength is 1 (the reflected
share is `1 − τ`, plus the source's own bounce).

### 2.3 Why the echo stays contained

While the firmament holds every one of its kinds, each bounce reflects back
to the core, and the energy of each bounce goes into the firmament's
torque. This is the substrate's restoring medium (docs/106) being pumped. The
lighter kinds were carried furthest at the founding (`ℓ_z = (K + s_i)·w̄/w_z`,
the lightest at the greatest reach), so they reach `R ≥ H` first. Their exit
opens `τ`. The law is the crossing (`flows::exits`), not an order: the
front leaves the firmament in the order its elements cross, which the
founding predicts to be lightest first and W5 reads. This is read, never
set.

## 3. What stays

- `ring::rate`, `deficit`, `period`, `spin`, `pinch`, `radius`, `grain` are
  unchanged. They read a body's reception, never an emission.
- A galaxy's arrival handling in `breath` is unchanged, apart from §2.2's
  amplitude and the second (inbound) arrival.
- A star's own bell stays. It is the star's own spin, not a relay (§5, open
  item 1).

## 4. Build order

- **W1. Statement.** Rewrite the `ring.rs` header to §2 (front, reception,
  surfaces). Add `ring::arrival(r, held) = r + held`. No behaviour change.
- **W2. The firmament's reach and books.** Add `Universe::firmament_reach`
  (`r_f`, read) and a founded holding (a new tag on the universe). `τ` is
  read from `founded` and `firmament_of`. Witness: `τ = 0` at the founding;
  `held + left = founded` across a breath.
- **W3. Containment.** Replace the single `T_u` schedule with the two
  periods: home arrivals and firmament torques every `T_f`, and galaxy
  arrivals only while `τ > 0`, at strength `τ`. Witness: a universe with a
  sealed firmament breathes no galaxy and conserves; one whose lightest kind
  exits opens, with `τ` equal to its share.
- **W4. The rim's reflection.** Add the inbound arrival at `T_u − a(R_i)`.
  Arrivals now use `a(R_i)`, no longer `⌊R_i⌋`.
- **W5. Read.** A bounded read of a fresh 2^40 universe: the bell of the
  first firmament exit (or none in N bells), `τ` over time, conservation and
  the torque identity. This read comes first in practice: if no kind exits
  within a reachable horizon, the universe stays sealed. That would be a
  finding about the firmament's torque, and it is not a reason to set `τ`.

## 5. Open

1. **Stars as surfaces.** The listed surfaces are cores, the firmament and
   the rim. A star holds a throat (`Body.throat`). Is a star's throat sphere
   a surface of its galaxy's front, so that its own bell is its bounce? If
   so, its period would follow §2.1 one tier down, not `ω_s`. Xylarium drops
   stellar remnants as relays, which this document does too (it adds no
   relay).
2. **The delay.** `breath` reads arrivals at `⌊R_i⌋` today. W4 moves them to
   `a(R_i) = R_i + H(<R_i)`, which shifts every galaxy's arrival bell.
   Xylarium reads the same `r + H(<r)`.
3. **Transmission per kind.** `τ` here is one share by holding. Xylarium
   opens per kind (each kind's onset echo, then an exponential emptying).
   Agreed with Xylarium: `τ` is one amplitude, `Σ_k left_k / founded` by
   holding; the matter the front carries out lands per kind.
5. **Remnants as cores.** A stellar black-hole remnant holds a throat; if
   it is a core, its throat sphere is a surface. Needs a decision on both
   sides.
6. **Escape criterion shared with Xylarium.** Netstratum exits on the
   crossing `R ≥ H` (or the jet); Xylarium opens a kind when its pole state
   `X = open/held`, dilated by `ρ = 3` per echo, reaches the throat. The two
   give different first exits unless shown to coincide; one must be the
   shared law.
7. **When galaxies first hear the core.** Under §2.1 no galaxy hears the
   universal core before the first exit; Xylarium today has every galaxy
   hear it from `t = 0`. This is a law change in both kernels (star
   formation timing, every galactic push), not a refactor.
4. **The galaxies' own fronts.** Inside a galaxy, its core's bounce is a
   front in its own enclosure, met by its stars. Today `breath` treats each
   arrival as reaching every star at once. The front's delay inside a galaxy
   is a later step.
