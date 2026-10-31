# 110 — The shell: rings defined on the manifold

This document defines the objects the code has called "rings" in manifold
terms, and extends the hydrostatic balance (S3, docs/106 §2) from a radial law
between rings to a field law on each shell's fibre. A ring read as one band
of reach, averaged over every phase, is a radial bin: it carries no state
along the phase circles, the planes or the R loop, so nothing in the medium
can vary around an orbit, and the higher dimensions cannot couple back into
it.

**Status:** built (§4); `flows::phase_beads`, `flows::fluid`.

**Prerequisites:** docs/78 (S^360, S^60, the design laws), docs/95 (the
three planes and the R loop), docs/96 §8.2 (forming and solid orbits),
docs/106 (the substrate; S3).

## 1. Definitions

Let a body's enclosure be the sphere S^n it defines (a galaxy's S^360, a
star system's S^60), with the deficit `δ = H/(R + H)` a function on it, and
let each place carry a phase on each spatial plane and on the R loop.

| Term | Definition | Formerly |
|---|---|---|
| **Wavefront** | The geodesic sphere of light-time `t` about a source in its enclosure, `{x : d(source, x) = t}`, a hypersurface of dimension n − 1 that grows one light-ring a bell. An arrival is a wavefront meeting a body's orbit. | the pulse's ring (`ring.rs`) |
| **Equator** | The fixed hypersurface of the inversion `δ ↦ 1 − δ`, where `δ = ½` (`R = H`): the meeting of the two faces. | "the ring" (the throat's) |
| **Shell** | The neighbourhood of a level set of δ on one face (a band of reach), fibred over that band by the phase torus, one circle per spatial plane, and the R loop: the motes of one face whose bands `R(1 ± p)` overlap, connected by their bands (the band `band_pairs` spreads each mote's matter over). Its state is a field on the fibre: holding, spin, heat and pressure per cell. | the fluid's ring (`flows::fluid`), which held one kind and in practice one mote |
| **Cell** | A bead of a shell along one plane's circle: the members between two gaps wider than the shell's mean gap on that circle. A cell owns the arc from its first member to the next cell's first member. | — |
| **Orbit torus** | A body's motion is a linear flow on its invariant torus: on plane k, `φ_k(n) = φ_k(0) + n·t_k/P`, with `t_k` its tilt share on that plane and `P = 1 + R/H`. A shell is the union of the orbit tori whose reach lies in its band. | the orbit (`motion.rs`) |

"Ring" remains the name of the light-ring, the unit of distance a wavefront
crosses in one bell (`ring.rs`), and of the ring-density law.

## 2. The cells, read from the shell itself

The cells of a shell on plane k are found from its members' phases on that
plane, never from a fixed number of sectors (design law 1: no tables, no fill
orders):

1. Order the members by phase `φ_k ∈ [0, 1)`.
2. Read the circular gaps between consecutive members, the last wrapping
   through 1; they sum to 1, and their mean is `1/n` for `n` members.
3. A gap wider than the mean parts two cells; the members between two such
   gaps are one cell. A shell whose members are spread evenly has no gap
   wider than the mean and is one cell.
4. A cell's arc is the sum of the gaps from its first member to the next
   cell's first member, so the arcs of a shell's cells on a plane sum to 1.

## 3. The balance on the fibre

Each cell's pressure is per unit of arc and of the shell's width `w`, the
span of its members' bands (from the innermost `R(1 − p)` to the outermost
`R(1 + p)`):

    P_cell = (Σ m·a + M_cell) / (w · arc) + P_h

with `m` the members' holding, `a` their heat, `M_cell` their open frames in
step with the shell's field phase, and `P_h` the hydrostatic column its
members bear (holding-weighted). Its potential is its members' mean,
`Φ = 1 − δ̄`, since a shell spans a band of reach.

Between two neighbouring cells on a plane's circle, the restoring share is
S3's:

    s = P_a/(P_a + P_b) − Φ_a/(Φ_a + Φ_b)

Cells at the same depth stand at balance where their pressures are equal;
cells at different depths, where their pressure shares match their potential
shares. Off balance, the over-pressured cell gives
turning to the other by the exchange law, weighted by the pair's tilt share
on that plane, `t̄_k` (holding-weighted, the shares summing to 1 over the
planes):

    u = |s| · |ℓ̄_a − ℓ̄_b| · t̄_k / (m_a + m_b)

the giver's members each losing `u·m_taker`, the taker's each gaining
`u·m_giver`, so `Σ m·ℓ` is unchanged. Weighted by `t̄_k`, the three planes
together move at most what one exchange would. The radial balance between
consecutive shells is S3 as built (docs/106 §2).

## 4. Built

- `flows::phase_beads`: a shell's cells on one plane, with their arcs.
- `flows::fluid`: the radial balance (S3) between consecutive rings, then
  the shells formed from overlapping bands and the balance around each
  plane's circle of each shell, cell pair by cell pair. The spin moved around
  the circles and the number of shell planes that parted are reported
  separately in `breathe` (CIRCULATION).
- Read over 300 bells of a fresh 2^40 universe: 12 shell planes parted into
  cells, and the balance around their circles moved about 6.6 times the spin
  the radial balance moved; the breath created nothing, the manifest
  unchanged, the torque identity exact; the run took 23% longer. Before the
  shells were formed from overlapping bands, every ring held one mote and no
  circle could part.
- Witnesses:
  - `an_even_shell_is_one_cell_and_a_clumped_one_parts_at_its_wide_gaps`;
  - `a_shell_at_balance_around_its_circle_moves_nothing_and_an_over_pressured_cell_gives_turning`
    (the exchange keeps `Σ m·ℓ` exact).

## 5. Open

- The R loop's own balance (cells along the loop), which needs the loop's
  coupling to time (docs/95 §2).
- Cells across planes: a mote belongs to one cell on each plane; the
  product cells (one per point of the torus) are not formed.
- Reading the shells' fibres in `observe`.
