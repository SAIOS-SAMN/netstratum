//! Exact integer smith normal form — the homological reduction ground.
//!
//! For an integer matrix `A` (rows × cols) this computes the decomposition
//!
//! ```text
//!     D = U · A · V
//! ```
//!
//! where `D` is diagonal with the invariant factors `d₁ | d₂ | … | d_r` on
//! its diagonal (each divides the next), and `U` (rows×rows) and `V`
//! (cols×cols) are **unimodular** integer change-of-basis matrices — their
//! inverses `U⁻¹`, `V⁻¹` are integer too, and carried here whole. Every step
//! is an exact integer row/column operation on `num_bigint::BigInt`: no
//! floats, no thresholding, no coordinate truncation.
//!
//! This is the substrate the cohomology stands on (IAE-2). From the witness
//! matrices the true `Ker(∂)` / `Im(∂)` bases and the torsion coefficients
//! read off exactly (the module doc on [`Smith`] states which columns), so
//! `Hᵏ = Ker(δᵏ)/Im(δᵏ⁻¹)` is the canonical harmonic obstruction class —
//! never a basepoint-gauge representative, never a heuristic count.

use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// A dense integer matrix, row-major.
type Mat = Vec<Vec<BigInt>>;

/// The Smith Normal Form of a matrix together with its full unimodular
/// witnesses. Invariants (verified in the tests):
///
/// * `u · a · v == d` for the original `a`,
/// * `u · u_inv == I`, `v · v_inv == I` (the inverses are exact integers),
/// * `d` is diagonal, entries non-negative, and `dₖ | dₖ₊₁`.
///
/// **Homological read** (`a = ∂ₖ`, the boundary matrix, cols = k-cells,
/// rows = (k−1)-cells):
///
/// * `rank(∂ₖ)` = the number of non-zero diagonal entries `r`.
/// * `Ker(∂ₖ)` is spanned by the **last `cols − r` columns of `v`** (the
///   basis vectors `V` sends to the zero columns of `D`).
/// * `Im(∂ₖ)` is spanned by the **first `r` columns of `u_inv`**, the i-th
///   scaled by `dᵢ` (the invariant factors).
/// * The torsion of `Hₖ₋₁` is exactly the invariant factors `dᵢ > 1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Smith {
    pub rows: usize,
    pub cols: usize,
    pub d: Mat,
    pub u: Mat,
    pub u_inv: Mat,
    pub v: Mat,
    pub v_inv: Mat,
}

impl Smith {
    /// The number of non-zero invariant factors — the rank of `A` over ℚ.
    pub fn rank(&self) -> usize {
        (0..self.rows.min(self.cols))
            .filter(|&t| !self.d[t][t].is_zero())
            .count()
    }

    /// The invariant factors `d₁ | d₂ | … | d_r` (the non-zero diagonal).
    pub fn invariant_factors(&self) -> Vec<BigInt> {
        (0..self.rows.min(self.cols))
            .map(|t| self.d[t][t].clone())
            .filter(|d| !d.is_zero())
            .collect()
    }
}

/// Identity matrix of order `n`.
fn identity(n: usize) -> Mat {
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        BigInt::one()
                    } else {
                        BigInt::zero()
                    }
                })
                .collect()
        })
        .collect()
}

// ── elementary operations, each carried into both the accumulating witness
//    and its inverse. A row op is a left-multiply by an elementary `E`:
//    `A ← E·A`, `U ← E·U`, `U⁻¹ ← U⁻¹·E⁻¹`. A column op is a right-multiply:
//    `A ← A·F`, `V ← V·F`, `V⁻¹ ← F⁻¹·V⁻¹`. The inverse of a row op on `U`
//    is the transposed column op on `U⁻¹` (and symmetrically for columns). ──

/// Swap rows `i`, `j` of `a` and `u`; the inverse swaps columns `i`, `j` of `u_inv`.
fn row_swap(a: &mut Mat, u: &mut Mat, u_inv: &mut Mat, i: usize, j: usize) {
    a.swap(i, j);
    u.swap(i, j);
    for row in u_inv.iter_mut() {
        row.swap(i, j);
    }
}

/// `row_i += q · row_j` on `a` and `u`; the inverse is `col_j -= q · col_i` on `u_inv`.
fn row_addmul(a: &mut Mat, u: &mut Mat, u_inv: &mut Mat, i: usize, j: usize, q: &BigInt) {
    for m in [a, u] {
        let rj = m[j].clone();
        for (x, y) in m[i].iter_mut().zip(rj.iter()) {
            *x += q * y;
        }
    }
    for row in u_inv.iter_mut() {
        let ci = row[i].clone();
        row[j] -= q * &ci;
    }
}

/// Negate row `i` of `a` and `u`; the inverse negates column `i` of `u_inv`.
fn row_negate(a: &mut Mat, u: &mut Mat, u_inv: &mut Mat, i: usize) {
    for m in [a, u] {
        for x in m[i].iter_mut() {
            *x = -std::mem::take(x);
        }
    }
    for row in u_inv.iter_mut() {
        row[i] = -std::mem::take(&mut row[i]);
    }
}

/// Swap columns `i`, `j` of `a` and `v`; the inverse swaps rows `i`, `j` of `v_inv`.
fn col_swap(a: &mut Mat, v: &mut Mat, v_inv: &mut Mat, i: usize, j: usize) {
    for row in a.iter_mut() {
        row.swap(i, j);
    }
    for row in v.iter_mut() {
        row.swap(i, j);
    }
    v_inv.swap(i, j);
}

/// `col_i += q · col_j` on `a` and `v`; the inverse is `row_j -= q · row_i` on `v_inv`.
fn col_addmul(a: &mut Mat, v: &mut Mat, v_inv: &mut Mat, i: usize, j: usize, q: &BigInt) {
    for m in [a, v] {
        for row in m.iter_mut() {
            let cj = row[j].clone();
            row[i] += q * &cj;
        }
    }
    let ri = v_inv[i].clone();
    for (x, y) in v_inv[j].iter_mut().zip(ri.iter()) {
        *x -= q * y;
    }
}

/// Compute the exact integer Smith Normal Form of `a` with full witnesses.
///
/// `a` is `rows × cols` (a ragged input is padded conceptually by treating a
/// short row as trailing zeros is not done — every row must have `cols`
/// entries; an empty matrix is the 0×0 / r×0 / 0×c degenerate, handled).
pub fn smith_normal_form(a: &[Vec<BigInt>]) -> Smith {
    let rows = a.len();
    let cols = a.first().map(|r| r.len()).unwrap_or(0);
    debug_assert!(a.iter().all(|r| r.len() == cols), "SNF: ragged matrix");

    let mut d: Mat = a.to_vec();
    let mut u = identity(rows);
    let mut u_inv = identity(rows);
    let mut v = identity(cols);
    let mut v_inv = identity(cols);

    let limit = rows.min(cols);
    for t in 0..limit {
        // Find any non-zero pivot in the trailing submatrix and bring it home.
        let pivot = (t..rows)
            .flat_map(|i| (t..cols).map(move |j| (i, j)))
            .find(|&(i, j)| !d[i][j].is_zero());
        let Some((pi, pj)) = pivot else {
            break; // the rest is identically zero — SNF complete.
        };
        if pi != t {
            row_swap(&mut d, &mut u, &mut u_inv, t, pi);
        }
        if pj != t {
            col_swap(&mut d, &mut v, &mut v_inv, t, pj);
        }

        // Euclid on the pivot cross until row t and column t are clean, then
        // enforce divisibility of the trailing block by the pivot.
        loop {
            // Clear column t below the diagonal.
            for i in (t + 1)..rows {
                if !d[i][t].is_zero() {
                    let q = &d[i][t] / &d[t][t];
                    if !q.is_zero() {
                        row_addmul(&mut d, &mut u, &mut u_inv, i, t, &(-q));
                    }
                }
            }
            if let Some(i) = (t + 1..rows).find(|&i| !d[i][t].is_zero()) {
                // A non-zero remainder smaller than the pivot: swap it in, redo.
                row_swap(&mut d, &mut u, &mut u_inv, t, i);
                continue;
            }
            // Clear row t right of the diagonal.
            for j in (t + 1)..cols {
                if !d[t][j].is_zero() {
                    let q = &d[t][j] / &d[t][t];
                    if !q.is_zero() {
                        col_addmul(&mut d, &mut v, &mut v_inv, j, t, &(-q));
                    }
                }
            }
            if let Some(j) = (t + 1..cols).find(|&j| !d[t][j].is_zero()) {
                col_swap(&mut d, &mut v, &mut v_inv, t, j);
                continue;
            }
            // Row t and column t are clean. Enforce dₜ | (every trailing entry);
            // a violation is folded up into row t and the cross re-cleared.
            let mut violated = None;
            'scan: for i in (t + 1)..rows {
                for j in (t + 1)..cols {
                    if !(&d[i][j] % &d[t][t]).is_zero() {
                        violated = Some(i);
                        break 'scan;
                    }
                }
            }
            match violated {
                Some(i) => {
                    row_addmul(&mut d, &mut u, &mut u_inv, t, i, &BigInt::one());
                    continue;
                }
                None => break,
            }
        }

        // Canonical sign: the invariant factor is non-negative.
        if d[t][t].is_negative() {
            row_negate(&mut d, &mut u, &mut u_inv, t);
        }
    }

    Smith {
        rows,
        cols,
        d,
        u,
        u_inv,
        v,
        v_inv,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(rows: &[&[i64]]) -> Mat {
        rows.iter()
            .map(|r| r.iter().map(|&x| BigInt::from(x)).collect())
            .collect()
    }

    fn matmul(a: &Mat, b: &Mat) -> Mat {
        let (n, k, p) = (a.len(), b.len(), b.first().map(|r| r.len()).unwrap_or(0));
        let mut out = vec![vec![BigInt::zero(); p]; n];
        for i in 0..n {
            for l in 0..k {
                if a[i][l].is_zero() {
                    continue;
                }
                for j in 0..p {
                    out[i][j] += &a[i][l] * &b[l][j];
                }
            }
        }
        out
    }

    /// The full contract: U·A·V = D, U·U⁻¹ = I, V·V⁻¹ = I, D diagonal with
    /// dₖ | dₖ₊₁ and non-negative. Checked for every input the suite throws.
    fn assert_smith(a: &Mat) -> Smith {
        let s = smith_normal_form(a);
        assert_eq!(matmul(&matmul(&s.u, a), &s.v), s.d, "U·A·V = D");
        assert_eq!(matmul(&s.u, &s.u_inv), identity(s.rows), "U·U⁻¹ = I");
        assert_eq!(matmul(&s.u_inv, &s.u), identity(s.rows), "U⁻¹·U = I");
        assert_eq!(matmul(&s.v, &s.v_inv), identity(s.cols), "V·V⁻¹ = I");
        assert_eq!(matmul(&s.v_inv, &s.v), identity(s.cols), "V⁻¹·V = I");
        // D diagonal, non-negative, divisibility chain.
        for i in 0..s.rows {
            for j in 0..s.cols {
                if i != j {
                    assert!(s.d[i][j].is_zero(), "D off-diagonal zero at ({i},{j})");
                } else {
                    assert!(!s.d[i][j].is_negative(), "invariant factor non-negative");
                }
            }
        }
        let f = s.invariant_factors();
        for w in f.windows(2) {
            assert!((&w[1] % &w[0]).is_zero(), "dₖ | dₖ₊₁: {} | {}", w[0], w[1]);
        }
        s
    }

    #[test]
    fn identity_is_its_own_smith() {
        let s = assert_smith(&identity(3));
        assert_eq!(s.rank(), 3);
        assert_eq!(s.invariant_factors(), vec![BigInt::one(); 3]);
    }

    #[test]
    fn a_full_rank_3x3_has_invariant_factors_2_2_156() {
        // det = 624; gcd of all entries = 2 ⇒ d₁ = 2. The Smith form is
        // diag(2, 2, 156): 2·2·156 = 624, and 2 | 2 | 156. `assert_smith`
        // independently proves this is the Smith form (U·A·V = D, unimodular
        // witnesses, divisibility chain — which uniquely pin the SNF).
        let a = m(&[&[2, 4, 4], &[-6, 6, 12], &[10, 4, 16]]);
        let s = assert_smith(&a);
        assert_eq!(
            s.invariant_factors(),
            vec![BigInt::from(2), BigInt::from(2), BigInt::from(156)]
        );
    }

    #[test]
    fn a_non_square_wide_matrix_reduces_with_rectangular_witnesses() {
        let a = m(&[&[2, 4, 6, 8], &[1, 3, 5, 7]]);
        let s = assert_smith(&a);
        assert_eq!(s.u.len(), 2);
        assert_eq!(s.v.len(), 4);
        assert_eq!(s.rank(), 2);
    }

    #[test]
    fn a_tall_matrix_with_torsion_keeps_the_factor() {
        // ∂ with a torsion class: invariant factor 2 survives (ℤ/2 torsion).
        let a = m(&[&[2, 0], &[0, 1], &[0, 0]]);
        let s = assert_smith(&a);
        assert_eq!(s.invariant_factors(), vec![BigInt::one(), BigInt::from(2)]);
    }
}
