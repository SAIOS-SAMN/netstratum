//! Minimal exact-integer dense matrix ops for the sheaf / transduction layer.
//!
//! Row-major `Vec<Vec<BigInt>>`. A `rows × cols` matrix is a `Vec` of `rows`
//! rows, each of length `cols` — so a `3 × 0` matrix is three empty rows and
//! a `0 × c` matrix is the empty `Vec`. Everything is exact; there is no
//! float, no threshold, no truncation anywhere in this layer.

use num_bigint::BigInt;
use num_traits::Zero;

pub type Mat = Vec<Vec<BigInt>>;

/// A `rows × cols` zero matrix (rows empty when `rows == 0`).
pub fn zeros(rows: usize, cols: usize) -> Mat {
    (0..rows).map(|_| vec![BigInt::zero(); cols]).collect()
}

/// `(rows, cols)`. Cols is read from the first row, so a `0 × c` matrix
/// reports `(0, 0)` — harmless: a zero-row matrix annihilates on the left.
pub fn shape(a: &Mat) -> (usize, usize) {
    (a.len(), a.first().map(|r| r.len()).unwrap_or(0))
}

/// Exact integer product `a · b`. Requires `cols(a) == rows(b)` when both are
/// non-degenerate (debug-asserted); the result is `rows(a) × cols(b)`.
pub fn matmul(a: &Mat, b: &Mat) -> Mat {
    let (ar, ak) = shape(a);
    let (br, bc) = shape(b);
    debug_assert!(ak == br || ar == 0, "matmul: inner dim {ak} != {br}");
    let mut out = zeros(ar, bc);
    for i in 0..ar {
        for l in 0..ak {
            if a[i][l].is_zero() {
                continue;
            }
            for j in 0..bc {
                out[i][j] += &a[i][l] * &b[l][j];
            }
        }
    }
    out
}

/// Exact `a − b` (same shape assumed).
pub fn sub(a: &Mat, b: &Mat) -> Mat {
    a.iter()
        .zip(b.iter())
        .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x - y).collect())
        .collect()
}

/// True iff every entry is zero (vacuously true for an empty matrix).
pub fn is_zero(a: &Mat) -> bool {
    a.iter().all(|row| row.iter().all(|x| x.is_zero()))
}

/// Exact matrix–vector product `a · x` (length `rows(a)`).
pub fn matvec(a: &Mat, x: &[BigInt]) -> Vec<BigInt> {
    let (r, k) = shape(a);
    debug_assert!(
        k == x.len() || r == 0,
        "matvec: inner dim {k} != {}",
        x.len()
    );
    (0..r)
        .map(|i| (0..k).fold(BigInt::zero(), |acc, j| acc + &a[i][j] * &x[j]))
        .collect()
}
