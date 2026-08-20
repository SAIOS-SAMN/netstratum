//! THE STRATIFIED PEEL — cohomology by exact SNF (IAE-2, step 3).
//!
//! The reduction the whole architecture exists to get right. `Hᵏ` is the
//! genuine quotient `Ker(δᵏ) / Im(δᵏ⁻¹)`, read off the substrate's exact
//! Smith Normal Form — never the legacy `coboundary_reduce`, which subtracted
//! the basepoint-0 coboundary over a uniform-`m` loop (a gauge fix, not a
//! quotient).
//!
//! * **Betti** (the free rank) `= dim Ker δᵏ − rank δᵏ⁻¹`.
//! * **Torsion** `=` the invariant factors `> 1` of `δᵏ⁻¹` (exact `ℤ/dᵢ`).
//! * **The peel** `reduce_cochain` folds a cochain to its canonical
//!   representative in `Cᵏ / Im(δᵏ⁻¹)`: in the basis given by the columns of
//!   `U⁻¹` (where `Im(δᵏ⁻¹) = span{ dᵢ · U⁻¹[:,i] }`), each torsion coordinate
//!   is taken **modulo its invariant factor** and each free coordinate is
//!   kept whole. Exact ℤ-module quotient — no coordinate truncation, and the
//!   surviving harmonic part IS the irreducible obstruction (`H¹`).

use crate::mat::matvec;
use crate::sheaf::CellSheaf;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use substrate::snf::smith_normal_form;

/// The cohomology of one grade: free rank and exact torsion coefficients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cohomology {
    pub grade: usize,
    /// Free rank of `Hᵏ` (the k-th Betti number).
    pub betti: usize,
    /// Torsion coefficients `dᵢ > 1` — `Hᵏ ≅ ℤ^betti ⊕ ⨁ᵢ ℤ/dᵢ`.
    pub torsion: Vec<BigInt>,
}

impl CellSheaf {
    /// `Hᵏ = Ker(δᵏ) / Im(δᵏ⁻¹)` via exact SNF. Free rank and torsion both
    /// exact; no thresholding, no heuristic count.
    pub fn cohomology(&self, k: usize) -> Cohomology {
        let s_k = smith_normal_form(&self.coboundary(k));
        let rank_k = s_k.rank();
        let ker_dim = self.dim_cochain(k) - rank_k;

        let (rank_prev, torsion) = if k == 0 {
            (0, Vec::new())
        } else {
            let s_prev = smith_normal_form(&self.coboundary(k - 1));
            let torsion = s_prev
                .invariant_factors()
                .into_iter()
                .filter(|d| d > &BigInt::one())
                .collect();
            (s_prev.rank(), torsion)
        };

        Cohomology { grade: k, betti: ker_dim - rank_prev, torsion }
    }

    /// Is `c ∈ Cᵏ` a cocycle (`δᵏ c = 0`)?
    pub fn is_cocycle(&self, k: usize, c: &[BigInt]) -> bool {
        matvec(&self.coboundary(k), c).iter().all(|x| x.is_zero())
    }

    /// THE PEEL. Reduce `c ∈ Cᵏ` to its canonical representative in
    /// `Cᵏ / Im(δᵏ⁻¹)` — the exact ℤ-module quotient. A `c` that lies in
    /// `Im(δᵏ⁻¹)` (a coboundary) reduces to zero; a cocycle reduces to its
    /// harmonic `Hᵏ` representative (the irreducible obstruction).
    pub fn reduce_cochain(&self, k: usize, c: &[BigInt]) -> Vec<BigInt> {
        if k == 0 {
            return c.to_vec(); // Im(δ⁻¹) = 0 — nothing to quotient by.
        }
        // δᵏ⁻¹ : Cᵏ⁻¹ → Cᵏ. With D = U·A·V, A = U⁻¹·D·V⁻¹, so
        // Im(A) = span{ dᵢ · U⁻¹[:,i] }. Coordinates of c in the U⁻¹ basis
        // are U·c; reduce coord i mod dᵢ for the torsion coords, keep the rest.
        let s = smith_normal_form(&self.coboundary(k - 1));
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

    /// The Euler characteristic `Σ (−1)ᵏ dim Cᵏ` — an integer-conservation
    /// invariant: it must equal `Σ (−1)ᵏ betti_k` (homological exactness,
    /// torsion-blind), a cross-check the tests assert on every complex.
    pub fn euler_characteristic(&self) -> i64 {
        (0..self.grades())
            .map(|k| {
                let dim = self.dim_cochain(k) as i64;
                if k % 2 == 0 {
                    dim
                } else {
                    -dim
                }
            })
            .sum()
    }
}
