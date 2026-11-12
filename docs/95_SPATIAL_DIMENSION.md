# 95 — The spatial dimension: three planes of 3Z/3Q and R

This document derives Netstratum's spatial dimension as three planes of
3Z/3Q with the R coordinate, positive R denoting forward in linear time. It
replaces `flows::PLANES = 2`, which ran the physics on two planes without a
derivation, and the hand-built source of `flows::founding_phases` (a sphere
of radius 4, the point (2,0,0) and the generator ½e₁e₂, typed in and read on
two planes sharing e₁).

**Status:** step 1 built (three spatial planes and the R loop on every
phase; the hand-built source retired).

## 0. Restated in docs/97's nuclide (2026-11-11, #176)

**Operator 2026-11-11:** "we had our documentation aligned on 3Z/3Q with R
offset (electron phase) — we lost that somehow, and it was the nucleon
inverse from xylarium to test inverted physics"; "this needs to get fixed so
that our nucleons are weighted properly".

§1 and §8 below were written before docs/97 (2026-10-28) swapped the letters
to Xylarium's inverse. They read Z as the charge and Q as the unpaired
electrons inside `3Z`. `chem::frames` was built on that reading on
2026-11-06 (64254316), through Mycelium 78.175 F1 (Xylarium's convention, Z
the proton) without the seam's Z ↔ Q translation. The result: the protons
were counted as "3Z", the electrons' unpaired count stood as the Q pole, and
the neutrons were nowhere. In docs/97's terms:

- **The nucleon dipole.** Each nucleon stands in each of the three axes, so a
  nuclide of weight `w = Q + Z` has `3w` dimensions:
  - `3Q`, the protons' open dimensions, the **open pole** (the lone ends, docs/97 §2);
  - `3Z`, the neutrons' held dimensions, the **held pole** (the fibred ends, the glue).

  A cluster sums its parts. This is the whole every reading of the dimensions is a share
  of: the nucleons, weighted as they are.
- **R, the electron's phase: the offset between the poles.** The `Q`
  electrons follow the protons. Their open dimensions `U` (`atom(Q)`, unpaired)
  and their phase `R = κ_e·(1 − 2·frac(p))` (docs/97 §3,
  `nuclides::electron_phase`) are the electrons' domain, "never the
  nucleus's". Charge moves between the poles only with R carrying the change.
  R is its own component beside the dipole, never folded into it.
- **The planes** are `(Z_k, Q_k)`, each axis's neutron and proton dimensions.
  R is the axis that pairs with none (§2).
- **"Frame" is retired** (mycelium #239, adopted for netstratum by the
  operator 2026-11-11): a collapse of volume and dimensions. The coupling's
  two poles are the **held dimensions** (`3Z`) and the **open dimensions**
  (`3Q`); §1 and §8's "frames" read as these.
- **Readings** (`chem::Dimensions`):
  - the dipole's pole is its span over its whole, `(3Z − 3Q)/3w = (Z − Q)/w`,
    in `[−1, 1]`: hydrogen `−1` (all proton), deuterium `0`, a neutron-rich
    kind positive;
  - its open share is the electrons' open dimensions over the whole nucleon
    frames, `U/3w`;
  - read on the vacuum's inverted face, the pole negates and the open share
    is its complement.
- **At every seam** with Xylarium and Mycelium, Z ↔ Q is translated and
  labelled ("Q(proton)", docs/97).

Where §1 and §8 below say `3Z`, `hold`, `u` or `A + Q = 3Z`, read this
section. The push and the open share already divide by the nucleons
(`pulse::push_of`, `flows::open`); the dipole coupling, the magnet and the
throat's base frames are rebuilt on `chem::Dimensions`.

## 1. The axes

A nucleus of charge Z weighs `3Z` (`chem::weight`): its closed frames, the
hold `3Z − u` (the Z pole), and its open frames, `u` (the Q pole). The three
of 3Z is the spatial three: each proton carries a frame on each of three
axes. A frame on axis k is closed or open, so axis k is not a line but a
plane spanned by its closed direction and its open direction:

| plane | base axes | what turning in it is |
|---|---|---|
| 1 | `(Z₁, Q₁)` | frames on axis 1 passing between closed and open |
| 2 | `(Z₂, Q₂)` | the same on axis 2 |
| 3 | `(Z₃, Q₃)` | the same on axis 3 |
| — | `R` | reach: the unpaired axis |

3Q is the quantum domain, the three planes; 3Z lives within it as their
closed axes; weight, mass, is the two together (`hold + u = 3Z`). Space is
three-dimensional because the planes are three: seven base axes, three planes
and one axis that pairs with none.

## 2. R, and the direction of time

In the conformal model the odd last base axis pairs with no base axis; its
flow is carried on the planes it shares with `e₊` and `e₋`, the origin and
infinity (Mycelium `physics::outflow`). An axis joined to origin and
infinity is scale: R is reach.

- **Positive R is forward in linear time.** Outflow from the source (the
  continuum: the founding's throw, a star's release) moves R forward.
- **The Möbius jetstream's sense flip is R's sign.** `M: (δ, ψ, s) ↦ (1 − δ,
  ψ + ½, −s)` (docs/94 §2): `s` is the sign of R's motion. The vacuum leg
  runs R backward; crossing the ring flips it; `M∘M = id` returns it.
- **The luminal bound is R's rate bound.** `2δ ≤ 1` (docs/94 §2): nothing
  moves R by more than the ring a K-step.

## 3. The source's three inputs, from Netstratum's own universe

The law (`mycelium::physics::outflow`, n = 7 read off the coordinates):
`K = L − σ∧(σ·L)/σ²`, `v = P·K`, `N = P∧v`, and on each base plane its
outflow `N` and its direction `v` in the plane.

- **σ, the equipotential figure: the ring.** The sphere at the centre of
  radius the ring, `R = H` (δ = ½, 1 in the enclosure's own units), where
  the luminal bound stands and through which the throat inverts.
- **P, the pivot: the body's point on the ring.** Its plane components are
  its own frames: on plane k, `(z_k, q_k)` its closed and open frames on
  axis k, its R the ring. Held exactly as a rational point by inverse
  stereographic projection (docs/78); the projection's squares are the
  sphere's representation, never a law.
- **L, the lift: the bounce.** On each plane its push, `p_k = ½·q_k/z_k`
  (the push law, `½·magnetism/held`, read per axis), on `e_Zk ∧ e_Qk`; and on
  R the forward push, `p` on `e_R ∧ e₊` (forward, positive). K is the part
  keeping the ring.

Each plane's outflow is the share of the throw that plane carries; its
direction `(v_Z, v_Q)` is where in the plane the matter leaves.

## 4. What a mote carries

Its place: a phase on each of the three planes, `ψ_k` in turns, and its R.

- **Spin per plane**, `ℓ_k`, sets that phase's rate (one turn takes
  `1 + R/H` rings, `ring::period`), signed.
- **Reach** is the L1 sum of its plane spins over the hold, `R = Σ|ℓ_k|/H`;
  its time sense the sign of its R motion.
- **A phase from a direction**, by the L1 angle on the plane's circle: the
  quadrant plus `|v_Q|/(|v_Z| + |v_Q|)` of a quarter turn. Exact, first
  degree, no root, no trigonometry.
- **Separation of two places**: the L1 sum of their phase gaps on each plane
  (each the shorter way round) with their R gap in the enclosure's
  units. Meetings, capture, the ocean's landing and the highways read it;
  two bodies at one reach on opposite sides are apart.

The existing laws carry over plane by plane, first degree: the torque's
hemisphere sign and `1 − 2|ψ|` on every plane; the held cut and the in-step
coherence as products over the planes (their present form for two).

## 5. What retires

- `flows::PLANES` and `flows::founding_phases`: planes and phases are the
  source's reading.
- `ell_o` / `ell_orth` as two named spins: they become planes of `ℓ_k`; the
  "orbit plane" is whichever plane carries the most spin, a reading.
- The jet on the circulation plane's phase leg `[½, 1)`: outbound and
  return are R's sign.

## 6. Open decisions

1. **How a kind's frames part among the three axes.** Proposed: evenly,
   `z_k = (3Z − u)/3`, `q_k = u/3` (exact rationals; an atom names no axis,
   so the source is isotropic and every difference between planes rises from
   the lift and the motion, never a fill order).
2. **The forward push on R**: proposed the same push share `p` as the
   planes' mean, so the throw's forward motion and its turning come from one
   law.

## 7. Build order (one commit each, every identity re-read)

1. The coordinates: per-plane state (`ψ_k`, `ℓ_k`) and R with its sign on
   `Mote` and `Body`; the tape carries them.
2. The founding through `outflow_planes` at n = 7 (σ, P, L above).
3. The torque, reach, cut and coherence on the three planes.
4. Separation along the sphere for meetings, capture, ocean landing.
5. The board on the new state (stars and worlds each in their own planes).
6. Read: identities exact, worlds placed and kept, cost per echo measured.

## 8. Z and Q together

Q and Z must work together on the same axis, or R diverges; the forward
trajectory is realigned on that bond.

- Q is Z's partner on the same plane, never a separate axis: a bond turns a
  frame from the Q pole to the Z pole within its plane.
- Every law reads the two as shares of their sum, the whole weight
  `A + Q = 3Z`, never one pole over the other. R is their span, `A − Q`
  (the number grammar: ℝ the span between the ℤ and ℚ poles),
  bounded by the whole.
- Built: a body's push is `½·Q/(A + Q)` (`pulse::push_of`), at most ½, the
  luminal bound itself (it was `½·Q/A`, past ½ wherever the open outweighed
  the held, Z 31 among Netstratum's own atoms); a kind's open share is `Q/(A + Q)`
  (`flows::open`, it was `Q/A`, past one for the same kinds).
- Withdrawn: the rest reach `H·A/Q` (infinite at Q = 0) and the spacing
  `(2A + Q)/(2A − Q)` (a pole at Q = 2A), both sent to Xylarium.
