//! THE PAIR ENCODER — a grid pair as a relational field (R-D6 step 1).
//!
//! The one piece netstratum did not already hold. `kernel::bridge`
//! projects committed STATE onto `rfa_core::Delta`; this projects an ARC
//! grid PAIR onto the same tensor, so everything downstream —
//! `coboundary_reduce`, `holonomies`, the residual field — applies to a
//! puzzle exactly as it applies to a world.
//!
//! # The coboundary trap, and why the relation is adjacency
//!
//! `kernel/src/bridge.rs` states it: if `Δ_ij = scalar(i) − scalar(j)`
//! then `Δ` is a PURE COBOUNDARY, `coboundary_reduce` sends it to zero,
//! and the class reads empty for every input forever. Node-local scalars
//! carry no structural content.
//!
//! So a per-hue reading — "how many cells of hue `h` changed" — is
//! exactly the wrong projection, however natural it looks. What is
//! irreducibly pairwise in a grid is **directed adjacency**: how often
//! hue `a` sits immediately before hue `b`. That is not derivable from
//! any per-hue property, so it survives the reduction as real structure.
//!
//! # The relation
//!
//! For an ordered hue pair `(a, b)` let `A(a,b)` count the places where
//! `a` sits immediately before `b` — leftward on the row axis, upward on
//! the column axis. The imbalance `A(a,b) − A(b,a)` is the directed
//! content of that pair, and the relation is how the transformation
//! MOVED it:
//!
//! ```text
//! Δ_ab,l = [A_out(a,b) − A_out(b,a)] − [A_in(a,b) − A_in(b,a)]
//! ```
//!
//! Antisymmetric by construction: swapping `a` and `b` negates both
//! brackets. Zero when the transformation left that pair's directional
//! relationship alone — which is the reading we want, since a relation
//! the transformation did not touch is not part of what it is.
//!
//! # Two channels, and why not one
//!
//! Row-adjacency and column-adjacency are separate coordinates. A
//! transpose exchanges them; a horizontal flip negates one and leaves
//! the other; a vertical flip does the reverse. Summed into one channel
//! those become indistinguishable — the scalar collapse `18` §2b bars.
//!
//! # MEASURED 2026-08-03 — this encoding does NOT carry the transformation
//!
//! `examples/class_probe.rs`, 120 forged tasks (each applies ONE secret
//! word to several sampled inputs, so its pairs must agree if the
//! encoding is an invariant of the transformation):
//!
//! ```text
//! pairs of one task agree on class:  9/120  (7 %)
//! class rank reaches 45 — every hue pair carrying its own cycle
//! ```
//!
//! **The encoding reads the GRID, not the map.** `Δ_ab` is built from
//! `A_out − A_in`, which depends on the sampled content, so a rotation
//! applied to two different inputs presents two different classes. And a
//! rank of 45 is `C(10,2)` — the complex saturates, every pair present
//! and independent, which is the density `bridge.rs` warns carries no
//! topology.
//!
//! **The cause is the same collapse in a new place: this aggregates over
//! POSITIONS.** Summing adjacency counts across the grid discards where
//! each cell went, and where each cell went is what identifies the
//! transformation. `saios-arc::pair_encoding` carries
//! `CellCorrespondence { Identity | Permutation | ManyToOne | OneToMany |
//! Free }` per cell for exactly this reason; I skipped the
//! correspondence and went straight to totals.
//!
//! **What survives:** the memory property below, and the probe. What is
//! owed is an encoding over the cell correspondence rather than over
//! aggregate counts — entities as cells or regions, the relation as
//! where each went, so the class is a property of the map.
//!
//! # Memory
//!
//! `dim ≤ 10` (the hues) and `fiber = 2`, both fixed by the ARC grid
//! alphabet rather than by the search. The complex is at most 10×10×2
//! rationals **whatever the grid size and whatever the search depth** —
//! the bounded-by-construction property the reverted image-set lacked.

use cfe::ratio::Ratio;
use rfa_core::{Delta, Support};
use solver_verify::grid::Grid;

/// The hue alphabet — ARC grids carry 0–9 and nothing else.
pub const HUES: usize = 10;

/// Row adjacency: `a` immediately left of `b`.
pub const CHANNEL_ROW: usize = 0;
/// Column adjacency: `a` immediately above `b`.
pub const CHANNEL_COL: usize = 1;
/// The fiber arity of the pair projection.
pub const PAIR_CHANNELS: usize = 2;

/// Directed adjacency counts for one grid: `[channel][a][b]`.
fn adjacency(g: &Grid) -> [[[i64; HUES]; HUES]; PAIR_CHANNELS] {
    let mut counts = [[[0i64; HUES]; HUES]; PAIR_CHANNELS];
    for r in 0..g.rows {
        for c in 0..g.cols {
            let here = g.at(r, c) as usize;
            if here >= HUES {
                continue;
            }
            if c + 1 < g.cols {
                let right = g.at(r, c + 1) as usize;
                if right < HUES {
                    counts[CHANNEL_ROW][here][right] += 1;
                }
            }
            if r + 1 < g.rows {
                let below = g.at(r + 1, c) as usize;
                if below < HUES {
                    counts[CHANNEL_COL][here][below] += 1;
                }
            }
        }
    }
    counts
}

/// Which hues the pair actually speaks. A hue absent from both grids is
/// ABSENT, never a zero row — the sparsity law (`bridge.rs`: a dense
/// tensor cannot tell "these do not relate" from "these relate by zero",
/// and that distinction is what carries topology).
fn hues_present(input: &Grid, output: &Grid) -> [bool; HUES] {
    let mut seen = [false; HUES];
    for g in [input, output] {
        for &cell in &g.cells {
            if (cell as usize) < HUES {
                seen[cell as usize] = true;
            }
        }
    }
    seen
}

/// **Project one grid pair onto the relational tensor.**
///
/// Returns the field and its support. Feed both to
/// `rfa_core::holonomies` for the H¹ class, or to `coboundary_reduce`
/// for the irreducible core.
pub fn project_pair(input: &Grid, output: &Grid) -> (Delta, Support) {
    let (before, after) = (adjacency(input), adjacency(output));
    let present = hues_present(input, output);

    let mut delta = Delta::zero(HUES, PAIR_CHANNELS);
    let mut support = Support::new(HUES, PAIR_CHANNELS);

    for a in 0..HUES {
        for b in (a + 1)..HUES {
            if !present[a] || !present[b] {
                continue;
            }
            let mut coords = Vec::with_capacity(PAIR_CHANNELS);
            let mut carries = false;
            for channel in 0..PAIR_CHANNELS {
                let moved = (after[channel][a][b] - after[channel][b][a])
                    - (before[channel][a][b] - before[channel][b][a]);
                if moved != 0 {
                    carries = true;
                }
                coords.push(Ratio::from_integer(moved.into()));
            }
            // An edge stands where the pair is OBSERVED and the
            // transformation moved something between them. A pair the
            // transformation left alone is not part of what it is.
            if carries {
                delta.set_antisym(a, b, coords);
                for channel in 0..PAIR_CHANNELS {
                    support.connect(a, b, channel);
                }
            }
        }
    }
    (delta, support)
}

/// **The cohomology fingerprint of one pair** — the H¹ class, rendered
/// canonically so two pairs can be compared without comparing grids.
///
/// This is what `saios-arc::refinement` carries on an
/// `OperatorHypothesis` in place of a program: the invariant the
/// transformation IS, rather than one spelling of how to perform it.
///
/// Sorted by `(channel, a, b)` before hashing so the reading is
/// order-free and byte-identical on every machine.
pub fn fingerprint(input: &Grid, output: &Grid) -> [u8; 32] {
    let (delta, support) = project_pair(input, output);
    let mut cycles = rfa_core::holonomies(&delta, &support);
    cycles.sort_by_key(|held| (held.l, held.i, held.j));
    let mut hasher = blake3::Hasher::new();
    for held in &cycles {
        hasher.update(&(held.i as u64).to_le_bytes());
        hasher.update(&(held.j as u64).to_le_bytes());
        hasher.update(&(held.l as u64).to_le_bytes());
        hasher.update(held.value.numer().to_string().as_bytes());
        hasher.update(b"/");
        hasher.update(held.value.denom().to_string().as_bytes());
        hasher.update(b";");
    }
    *hasher.finalize().as_bytes()
}

/// The rank of the pair's class — how many independent cycles the
/// transformation left that no relabelling of hues explains.
pub fn class_rank(input: &Grid, output: &Grid) -> usize {
    let (delta, support) = project_pair(input, output);
    rfa_core::holonomies(&delta, &support).len()
}
