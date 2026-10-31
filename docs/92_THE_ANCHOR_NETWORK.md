# 92 — The anchor network: the core held to its accretion disc, stars chained by gravity

This document specifies the anchor network: each galactic core is held to
its accretion disc, and stars are chained to it and to each other by
gravity. It addresses four findings:

- The pull strength was computed incorrectly.
- The galactic core's hold extended too far. The bounce holds only to the edge of the accretion disc; the disc offers attachment to the first coordination of stars, which chain down through the order of stars, so that the stars form a network held by gravity.
- One term covered both throats and stellar cores. Stellar cores are reactors, and a distinct link connects core, reactor and planet.
- Anchors weaken with distance, and with torque, twist and spin.

**Status:** scoped, not built. Derived with Xylarium (one form for both
carriers). The reactor's extent fall (§3.3) is built (12574d0).

## 1. The finding that forces it

The pull law, the same at every tier, is the deficit `δ = H/(d + H)`: a
body of hold `h` (its mass share of its parent, 3Z a nucleus since f60beac)
pulls at distance `d` by `h/(d + h)`; its pull falls as one over distance.

A star of hold `h` standing at reach `R` from a core of hold `H` outpulls
the core within `x = h·R/H` of itself (the equal-pull sphere, its inner
crossing the pinch `R·h/(h + H)`). Every grain stands inside its parent's
throat, `R < H`, because the exits remove `R ≥ H` (`flows::exits`). So
`x < h` for every star: a star outpulls its core only within less than its
own hold-length. In the system's own units that is the breathing rim,
`ρ = R_s/(h_s + H) < 1` (41f9c2d), and the heart's own extent fills it:
whatever stands inside `ρ` falls.

Read, both carriers, 2026-10-25:

| | Netstratum (2^55, 340 bells) | Xylarium (seed 7, 8 echoes) |
|---|---|---|
| stars' reaches from the core | 0.0003 – 0.02 (throat at 1) | median 0.31 (min 3e-5, max 2.09) |
| birth discs at or past the rim | 96 – 100 % | disc born 0 once membership held within the gravity reach |
| worlds | 0 | 351 → 18 under the same change (set aside) |

So it is not a reach, a membership rule or the rim's derivation: no body
inside its parent's throat can hold anything beyond its own heart. The
core chains everything in its galaxy; nothing beyond it can be a star's.

## 2. The words (one word was carrying three things)

- **Throat**: a passage between enclosures, the paired face (the universal
  and galactic cores', a remnant's). Its state belongs to the passage and
  holds ("the state holds only where there is a throat"); a galactic core
  is a throat, so its state stays the enclosure's base, 1 (docs/83,
  2026-10-15, stands).
- **Reactor**: a star's heart. It holds what it has gathered and pushes;
  it has no throat (2026-10-17) and no state. What falls into it is read by
  its own extent, never by the throat condition.
- **Anchor**: what connects core → reactor → planet: the tether chain
  (Xylarium 11.3a: double-cone manifolds; stretch, torque, twist). Never a
  pull contest with the core.

## 3. The forms

### 3.1 The accretion disc's edge, `R_d`

The core's bounce holds its accretion disc: what its cut holds. The cut is
already a reading (`flows::read_cut`, ac9c622): each disc-leg mote's held
share against the galaxy's one enclosure. `R_d` is the reach where that
share falls to zero, the outermost disc-leg mote the cut still holds. It
breathes with the cut, echo by echo. (Xylarium's first form, the edge where
`H(<R_d) = 2·state`, does not bite: the state is the founding mass, equal to
the grains, so `H` reaches 2 only at the last mote; both carriers read
`state/grains` at 0.96 – 1.00.)

Inside `R_d`, nothing changes: the core holds by its enclosure, its
reservoir pays every torque, as now.

### 3.2 The anchors and the chain

Beyond `R_d` the core holds only through its first coordination:

- the disc is one anchor, standing at `R_d`, its hold `H_d =` the hold
  inside its edge (the state and the disc's matter);
- each star system standing beyond `R_d` is an anchor, its hold its whole
  system's mass share `h_s`, standing at its reach `R_s`;
- every body beyond `R_d` (a mote, a star) has one parent: its nearest
  inner anchor by reach (radial, as every gap is now; the shared plane
  enters when gaps go on the sphere, the same gate as the tide's
  re-derivation). The first stars anchor to the disc; later ones to the
  nearest inner star.

So the network is a tree: core → disc → first stars → theirs.

### 3.3 What falls into a newborn heart (built, 12574d0)

A member falls where its place stands within the reactor's extent,
`λ·R_max`: `λ` the fallen share of the binding, `R_max` the outermost kind the
dark cut of what has fallen holds (`heart_cut` at turning 0, heat 0, push
½), order-free to the fixed point. Read: 56 % of each binding falls (the
throat test took ~88 %).

### 3.4 The pull from an anchor, and the grip

A body at reach `r` anchored to an anchor at `R_p` of hold `H_p` feels

    δ_p = H_p / ((r − R_p) + H_p)

measured across its gap from the anchor, never from the core. The anchor's
grip on it weakens by distance, torque, twist and spin (the opening
requirements), each a reading the law already has, each entering once:

    g = δ_p · |k| · in_step(ψ_c − ψ_c,p) · d/(d + turned)

- distance (the stretch): `δ_p`, the anchor's deficit across the gap;
- torque: `|k|`, the coherent share of the anchor's torque the body takes
  in step (the passage mean of the hemisphere response); the rest is heat;
- twist: the body's in-step with its anchor across the orthogonal plane; a
  body a quarter turn twisted is held half;
- spin: the gyroscope's give, `d/(d + turned)`, `d = 1 − δ_p`, `turned` the
  body's own spin's turns this echo (the return the star already rides its
  galaxy by, breath.rs).

Down the chain the grip passes as the product of its links: a far star
feels the core through every link between them, each weakening by its own
four readings (a product of distinct readings, never one reading squared).

`g` replaces the bare share in the anchor's sphere (`k = g·h/H_p`, the
tether), in the system rim against the anchor (`ρ = (r − R_p)/(h + H_p)`
read through `g`), and in the torque it passes on (§3.5).

### 3.5 The torque from an anchor, and who pays

A body beyond `R_d` takes its anchor's push, never the core's: the disc
anchor's push is the core's (`p`, the bounce), a star anchor's its reactor's
(`½·magnetism/held` of its kept), times the link's grip `g`.

The payment, in the frame both share: the anchor's orbit pays, `W·Δℓ_p =
−Σ m·Δℓ`, all in galaxy units (the anchor's whole system mass `W`, the
anchored bodies' `m`), the grain's remainder to the galaxy's medium spin,
identity `anchor`; the disc anchor's payments are the core reservoir's, as
now. (Paying from a star's own spin would cross from the galaxy frame into
the system's by the share twice, a square.)

The passage and the invariant (4fe71e0: `I = ℓ − (p/2)·f(1 − f)` telescopes
because the push is read over the very arc the phase advances). About an
anchor the body's phase must advance about the anchor: by its own turning
about the anchor, `2δ_p`, carried with the anchor's own advance about its
parent. The push is then read over the relative arc `2δ_p` from the relative
phase `ψ − ψ_p`, so `I` telescopes about the anchor. (Correction to the
first statement of this form, 2026-10-25: the relative arc is `2δ_p`, the
turning about the anchor, not `2(δ_m − δ_p)`; only the former telescopes.)
Witness first: the invariant read about an anchor over N bells, exact, before
the live torque uses it.

### 3.6 Exits

A mote leaves the galaxy only past the outermost anchor's tether, beyond
every anchor's sphere (the disc's included, read against what stands beyond
it), never at `R ≥ H`. A mote inside no anchor's sphere and beyond the disc
is past every tether: it exits to the ocean.

### 3.7 The planets: the second movement (the lock share)

The star's path about its core and the planets' path about the star are
two movements; a planet's orbit is its tug plus the
star's spin. The pull of the core on the star is built (1133c7c). The lock
share is derived, not built: `c = δ(r)`, the held face (the unlocked face
`½(1 − δ)` is what the tide heats), as shared inertia:

    ℓ_s = S·unit / (kept + Σ m_i·δ_i),   ℓ_p = ℓ_tug + δ(r)·ℓ_s

`S` the star's extensive spin (0f8173e), the reading only partitioning it, so
the books close by construction; a star with locked worlds spins slower.
The planets' phases read against the star's own bell, never its galaxy
orbit: their plane is the star's spin plane, independent of the galaxy's
disc. It waits on §3.1–3.5 (inside a throat it could act only within the
heart).

## 4. Scope in the code

| Piece | Where | What changes |
|---|---|---|
| `R_d` | `flows.rs` (new `disc_edge`, beside `read_cut`) | a reading from the cut, each echo |
| anchors | `breath.rs`, the flows pass (after the arrivals, before the torque) | the disc and the stars beyond `R_d`, sorted by reach; each body's parent |
| the grip | `flows.rs` (new `grip`) | `δ_p · |k| · in_step · d/(d + turned)` |
| the torque | `flows::torque` / `torque_one` | an anchored body's hold, reach, deficit, push and passage read from its anchor; payments returned per anchor |
| payment | `breath.rs` | the disc's to the core reservoir; a star's from its orbit, `W·Δℓ_p = −Σ m·Δℓ`, identity `anchor` |
| shear, chains, bindings | `flows::shear`, `chains` | beyond `R_d`, neighbours read about their anchor |
| capture (settling) | `breath.rs` capture is settling | the sphere from each body's anchor, `k = g·h/H_p` |
| the system's rim | `breath::rim_of` | from the star's anchor, through `g` |
| exits | `flows::exits` | past the outermost anchor's tether |
| the lock share | `formation::collide_system`, `pulse::star_spin` | §3.7, after the rest |

Witnesses, in order: the invariant about an anchor (§3.5); `anchor`
identity exact per payment; exits only past every tether; the network a
tree (one parent each, no cycles); the books (3Z, every identity) exact.

## 5. Build order

1. `R_d`, the anchors and each body's parent, as readings only (printed by
   breathe: `R_d`, how many stars stand beyond it, the tree's depth).
2. The pull and the grip from the anchor: capture, the rim, chains, exits.
3. The torque from the anchor and its payment, after the invariant's witness.
4. The lock share (§3.7).

Each step its own read against the last, on the same founding.

## 6. Open

- Where Netstratum's stars stand against `R_d`: read (§7): most form inside the
  accretion disc as §3.1 reads it. The edge's reading (the held share's
  zero, or its half) decides how much of the galaxy the network governs.
- The disc anchor's own sphere: read against what stands beyond it (§3.6),
  `k = H_d/(H_d + matter beyond)`; to be confirmed by both operators.
- The grip's twist uses the orthogonal phase only; a third plane would add
  its own in-step.

## 7. Probe (2026-10-25)

`R_d` (the outermost disc-leg mote the cut holds) against the motes' extent
and the stars' reaches, 2^55, mass-as-3Z founding, bells 158–220, in the
galaxy's reach (the throat at 1):

| Galaxy | `R_d` | motes out to | stars |
|---|---|---|---|
| 19 | 0.32 – 0.41 | 0.38 – 0.40 | 0.015 – 0.31, one at 0.38: all inside `R_d` but one |
| 18 | 0.32 – 0.35 | 0.41 – 0.43 | 0.31 inside; 0.325 and 0.42 just beyond |
| 0 (home) | 0.56 – 0.71 (every mote) | 0.69 – 0.73 | none |

The core's cut holds 80 – 100 % of its galaxy's extent: the accretion disc is
nearly the whole galaxy, and most stars form inside it. Read by §3.1 as it
stands, the network reaches only a thin outer ring and the odd star just past
the edge. So before the build (§5) either:

- the accretion disc's edge is narrower than "what the cut holds" (the cut's
  held share is a spread `R(1 ± p)` against the hold, so nearly every
  disc-leg mote keeps some share; the edge may instead be where the held
  share falls below its free share, ½, the throat's own half, read from the
  cut); or
- where bindings collapse (§1, §6) is the first problem: stars form in the
  dense inner disc, where the core holds them, whatever the network does
  beyond it.

Both are readings to take before the network is built.
