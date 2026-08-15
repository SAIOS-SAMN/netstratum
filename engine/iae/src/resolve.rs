//! IAE-4 — THE RESOLVE-SOLVE LOOP, based on locality.
//!
//! "Resolve, don't enumerate." The measured relational field is reduced
//! against the vocabulary (the known local translations) modulo the
//! coboundaries; whatever survives is the **irreducible obstruction** — the
//! `H¹` the civilization has not yet resolved, and the feedstock condensation
//! (IAE-5) mints a new letter for. No candidate is enumerated; the residual
//! is computed exactly by the substrate SNF.
//!
//! **Locality is the source of the signal.** The measurement carries the
//! adjacency coordinate (`encode_grid_objects` coord 3): nonzero ONLY for
//! Manhattan-adjacent cells, and NOT a coboundary — "spatial structure the
//! difference coordinates cannot represent." The global-difference coordinates
//! (value/row/col) are coboundaries and resolve to nothing on their own; the
//! LOCAL adjacency coordinate is what fails to close and drives the solve.

use crate::bridge::{relational_complex, GridObs};
use crate::mat::{matvec, Mat};
use crate::sheaf::CellSheaf;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use substrate::snf::smith_normal_form;

/// The coordinate count of the locality measurement: value, Δrow, Δcol, and
/// the adjacency boundary.
pub const RELATIONAL_M: usize = 4;

/// Reduce `c ∈ ℤ^d` to its canonical representative modulo the sublattice
/// spanned by the COLUMNS of `generators` (`d × g`). The exact ℤ-module
/// quotient via SNF — the general "resolve, don't enumerate" primitive.
/// `Im(G) = span{ dᵢ · U⁻¹[:,i] }`, so a coordinate on a torsion generator is
/// taken modulo its invariant factor and the free coordinates are kept whole.
pub fn reduce_modulo(c: &[BigInt], generators: &Mat) -> Vec<BigInt> {
    if generators.is_empty() {
        return c.to_vec();
    }
    let s = smith_normal_form(generators);
    let mut coords = matvec(&s.u, c);
    for (i, d) in s.invariant_factors().iter().enumerate() {
        let mut r = &coords[i] % d;
        if r.is_negative() {
            r += d;
        }
        coords[i] = r;
    }
    matvec(&s.u_inv, &coords)
}

/// The outcome of a resolve: the irreducible residual and whether it vanished.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    /// The unexplained part of the field in `C¹` — the irreducible obstruction.
    pub residual: Vec<BigInt>,
    /// True iff the field lies entirely in the resolvable subspace (residual 0).
    pub solved: bool,
}

/// THE RESOLVE. Reduce a measured field in `C¹` modulo the resolvable
/// subspace: the coboundaries `Im(δ⁰)` (any global potential is free) together
/// with the vocabulary's local translations. The residual is the irreducible
/// obstruction — zero iff the field is fully explained by known structure.
pub fn resolve(complex: &CellSheaf, field: &[BigInt], vocabulary: &[Vec<BigInt>]) -> Resolution {
    let dim_c1 = complex.dim_cochain(1);
    let d0 = complex.coboundary(0); // dim C¹ × dim C⁰
    let dim_c0 = complex.dim_cochain(0);

    // Generator columns: every column of δ⁰, then each vocabulary translation.
    let mut gens: Mat = vec![Vec::with_capacity(dim_c0 + vocabulary.len()); dim_c1];
    for j in 0..dim_c0 {
        for (i, row) in gens.iter_mut().enumerate() {
            row.push(d0[i][j].clone());
        }
    }
    for v in vocabulary {
        for (i, row) in gens.iter_mut().enumerate() {
            row.push(v[i].clone());
        }
    }

    let residual = reduce_modulo(field, &gens);
    let solved = residual.iter().all(|x| x.is_zero());
    Resolution { residual, solved }
}

/// THE LOCALITY MEASUREMENT (`encode_grid_objects`): the relational field on
/// the grid cells, `m = 4` coordinates per pair — value difference, Δrow,
/// Δcol, and the ADJACENCY boundary (nonzero only for Manhattan-adjacent
/// cells). Coords 0–2 are coboundaries; coord 3 is the local, non-coboundary
/// signal. Returns the relational complex and the field as a `C¹` cochain
/// (edges lexicographic, matching [`relational_complex`]).
pub fn measure_relational(grid: &GridObs) -> (CellSheaf, Vec<BigInt>) {
    let n = grid.rows * grid.cols;
    let complex = relational_complex(n, RELATIONAL_M);
    let mut field = Vec::with_capacity(n * (n.saturating_sub(1)) / 2 * RELATIONAL_M);
    for i in 0..n {
        let (ri, ci) = ((i / grid.cols) as i64, (i % grid.cols) as i64);
        for j in (i + 1)..n {
            let (rj, cj) = ((j / grid.cols) as i64, (j % grid.cols) as i64);
            let vdiff = grid.cells[j] - grid.cells[i];
            let manhattan = (ri - rj).abs() + (ci - cj).abs();
            let adj = if manhattan == 1 { vdiff } else { 0 };
            field.push(BigInt::from(vdiff)); // 0: value difference (coboundary)
            field.push(BigInt::from(rj - ri)); // 1: Δrow (coboundary)
            field.push(BigInt::from(cj - ci)); // 2: Δcol (coboundary)
            field.push(BigInt::from(adj)); // 3: adjacency boundary (LOCAL, not a coboundary)
        }
    }
    (complex, field)
}
