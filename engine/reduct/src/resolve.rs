//! IAE-4 — the resolve-solve LOOP, based on locality.
//!
//! "Resolve, don't enumerate." The measured relational field is reduced
//! against the vocabulary (the known local translations) modulo the
//! coboundaries; whatever survives is the **irreducible obstruction** — the
//! `H¹` the civilization has not yet resolved, and the feedstock condensation
//! (IAE-5) mints a new letter for. No candidate is enumerated; the residual
//! is computed exactly by the substrate SNF.
//!
//! **Locality is the source of the signal.** The measurement carries the
//! adjacency coordinate (`encode_grid_objects` coord 3): nonzero only for
//! Manhattan-adjacent cells, and not a coboundary — "spatial structure the
//! difference coordinates cannot represent." The global-difference coordinates
//! (value/row/col) are coboundaries and resolve to nothing on their own; the
//! LOCAL adjacency coordinate is what fails to close and drives the solve.

use crate::mat::{matvec, Mat};
use crate::sheaf::CellSheaf;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use substrate::snf::smith_normal_form;

/// The coordinate count of the locality measurement: value, Δrow, Δcol, and
/// the adjacency boundary.
pub const RELATIONAL_M: usize = 4;

/// The binding of an obstruction onto a geometry — retained whole, never
/// collapsed to a bare residual (doc 51 §6, "bind, don't fold"). A manifold
/// does not disappear when it solves a problem; it wraps it, and this records
/// exactly how:
///
/// * `coefficients` — The overlap: the integer combination of the generators
///   that binds the obstruction (`generators · coefficients + residual` equals
///   the obstruction exactly — nothing is discarded).
/// * `torsion` — The twist: the invariant factors `> 1`. A ℤ-module carries
///   torsion natively (a ℚ-vector space cannot); binding modulo `ℤ/dᵢ` is the
///   flex that lets the geometry map onto an imperfect match.
/// * `residual` — the irreducible remainder that did not bind (the surviving
///   obstruction).
/// * `locked` — `residual == 0`: the geometries overlapped completely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub coefficients: Vec<BigInt>,
    pub torsion: Vec<BigInt>,
    pub residual: Vec<BigInt>,
    pub locked: bool,
}

/// Overlay — bind `obstruction` onto the geometry spanned by the columns of
/// `generators`, retaining the whole binding (not just the residual). The
/// exact reconstruction `generators · coefficients + residual = obstruction`
/// always holds, so nothing collapses: the overlap and its torsion twist are
/// carried, and only the genuinely unbound part survives as `residual`.
pub fn overlay(obstruction: &[BigInt], generators: &Mat) -> Binding {
    let n = generators.first().map(|r| r.len()).unwrap_or(0);
    if generators.is_empty() || n == 0 {
        return Binding {
            coefficients: vec![BigInt::zero(); n],
            torsion: Vec::new(),
            residual: obstruction.to_vec(),
            locked: obstruction.iter().all(|x| x.is_zero()),
        };
    }
    // D = U·G·V. In the U⁻¹ basis, Im(G) = span{ dᵢ · U⁻¹[:,i] }. Decompose the
    // obstruction's coordinates: the torsion coordinates split into a bound
    // quotient (times dᵢ) and a remainder in [0, dᵢ); the free coordinates ride
    // whole into the residual. The generator coefficients are V·q.
    let s = smith_normal_form(generators);
    let coords = matvec(&s.u, obstruction);
    let factors = s.invariant_factors();
    let mut q_ext = vec![BigInt::zero(); n];
    let mut res_coords = coords.clone();
    for (i, d) in factors.iter().enumerate() {
        let mut r = &coords[i] % d;
        if r.is_negative() {
            r += d;
        }
        q_ext[i] = (&coords[i] - &r) / d;
        res_coords[i] = r;
    }
    let residual = matvec(&s.u_inv, &res_coords);
    let coefficients = matvec(&s.v, &q_ext);
    let torsion = factors.into_iter().filter(|d| d > &BigInt::one()).collect();
    let locked = residual.iter().all(|x| x.is_zero());
    Binding { coefficients, torsion, residual, locked }
}

/// The residual alone — the non-collapsing binding's remainder. A thin view
/// over [`overlay`] kept for the proposer's exact verifier (which only needs
/// the yes/no of a complete lock).
pub fn reduce_modulo(c: &[BigInt], generators: &Mat) -> Vec<BigInt> {
    overlay(c, generators).residual
}

/// The outcome of a resolve: the irreducible residual and whether it vanished.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    /// The unexplained part of the field in `C¹` — the irreducible obstruction.
    pub residual: Vec<BigInt>,
    /// True iff the field lies entirely in the resolvable subspace (residual 0).
    pub solved: bool,
}

/// The resolve. Reduce a measured field in `C¹` modulo the resolvable
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

