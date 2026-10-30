//! RCF — the Rational Canonical Form conjugacy invariant (U2, the
//! PoWC crux). Re-derived from the v0.1 ancestor `rcf.rs`
//! (translate-never-transcribe; consult `/opt/repos`, port nothing
//! blindly).
//!
//! The one question this module answers: are two rational matrices in
//! the same `GL_n(Rat)` conjugacy class? Two matrices `A` and `PAP⁻¹`
//! are the same state seen through different frames — a gauge
//! difference, not a genuine one. The invariant that decides it is the
//! sequence of invariant factors `d₁ | d₂ | … | dₖ` of the
//! characteristic matrix `(xI − A)`, computed by its Smith Normal Form
//! over the polynomial ring `Rat[x]`. Same conjugacy class ⟺ same
//! invariant factors, exactly.
//!
//! This is the angle-free heart of the singularity law (16_UNIT §4a):
//! the RCF quotients out the frame of approach, so the fold gate's
//! verdict cannot depend on which frame a state arrived in.
//!
//! Two translation laws bind the derivation:
//! - **The proportional law** touches nothing here — the RCF is a
//!   CANONICAL form, not an aggregator of difference; no square is
//!   summed as a measure. (The ancestor's `Σ R²` lived only in its
//!   coherence functional, already landed as `Σ |R|`.)
//! - **blake3, never Keccak** — the ancestor hashed with Keccak-256;
//!   the house is blake3-only (N6). The exact hash bytes are
//!   Netstratum's own, re-witnessed red-first; only the conjugacy
//!   invariance property is carried (the NS-3 precedent: the equation
//!   is taken, the vectors re-witnessed).
//!
//! The judgment is exact: `orbit_lock` (the fold gate) compares the
//! [`RcfSignature`] — the whole exact invariant-factor sequence — not
//! the hash. [`rcf_hash`] is the compact blake3 fingerprint, for
//! commitment binding only (a convenience, never a gate — a hash is a
//! lossy summary and the law gates on exact form).

use crate::delta::Delta;
use cfe::ratio::Ratio;
use num_traits::{One, Zero};

// ─── Polynomials over Rat ────────────────────────────────────────────────

/// A polynomial over the exact rationals, `p(x) = Σ coeffs[i]·xⁱ`,
/// stored low-degree-first and trimmed of trailing zeros (so equal
/// polynomials have equal `coeffs`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatPoly {
    coeffs: Vec<Ratio>,
}

impl RatPoly {
    /// The zero polynomial — the empty coefficient list (degree: none).
    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    /// Build from coefficients (low-degree first), trimmed canonical.
    pub fn from_coeffs(coeffs: Vec<Ratio>) -> Self {
        let mut p = Self { coeffs };
        p.trim();
        p
    }

    /// `x − c`, the elementary factor of the characteristic matrix.
    pub fn x_minus(c: &Ratio) -> Self {
        Self::from_coeffs(vec![-c.clone(), Ratio::one()])
    }

    /// The degree, or `None` for the zero polynomial.
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    fn leading(&self) -> Option<&Ratio> {
        self.coeffs.last()
    }

    fn trim(&mut self) {
        while self.coeffs.last().is_some_and(|c| c.is_zero()) {
            self.coeffs.pop();
        }
    }

    /// Scale to monic (leading coefficient 1). A no-op on the zero
    /// polynomial. Monic-ness is what makes the invariant factors a
    /// CANONICAL representative — otherwise `2(x−1)` and `(x−1)` would
    /// read as different factors of the same class.
    fn make_monic(&mut self) {
        if let Some(lc) = self.leading().cloned() {
            if !lc.is_zero() {
                for c in &mut self.coeffs {
                    *c = &*c / &lc;
                }
            }
        }
    }

    fn monic(mut self) -> Self {
        self.make_monic();
        self
    }

    pub fn add(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![Ratio::zero(); len];
        for (i, c) in self.coeffs.iter().enumerate() {
            result[i] = &result[i] + c;
        }
        for (i, c) in other.coeffs.iter().enumerate() {
            result[i] = &result[i] + c;
        }
        Self::from_coeffs(result)
    }

    pub fn sub(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![Ratio::zero(); len];
        for (i, c) in self.coeffs.iter().enumerate() {
            result[i] = &result[i] + c;
        }
        for (i, c) in other.coeffs.iter().enumerate() {
            result[i] = &result[i] - c;
        }
        Self::from_coeffs(result)
    }

    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let len = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = vec![Ratio::zero(); len];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in other.coeffs.iter().enumerate() {
                result[i + j] = &result[i + j] + &(a * b);
            }
        }
        Self::from_coeffs(result)
    }

    /// Polynomial division: `(q, r)` with `self = q·divisor + r` and
    /// `deg(r) < deg(divisor)`. Panics only on a zero divisor (a caller
    /// bug — the SNF never divides by the zero polynomial).
    pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        assert!(!divisor.is_zero(), "division by the zero polynomial");
        let d_deg = divisor.degree().unwrap();
        let s_deg = match self.degree() {
            Some(d) => d,
            None => return (Self::zero(), Self::zero()),
        };
        if s_deg < d_deg {
            return (Self::zero(), self.clone());
        }
        let lc_inv = Ratio::one() / divisor.leading().unwrap();
        let mut remainder = self.coeffs.clone();
        let mut quotient = vec![Ratio::zero(); s_deg - d_deg + 1];
        for i in (0..=(s_deg - d_deg)).rev() {
            let coeff = &remainder[i + d_deg] * &lc_inv;
            for j in 0..=d_deg {
                let sub = &coeff * &divisor.coeffs[j];
                remainder[i + j] = &remainder[i + j] - &sub;
            }
            quotient[i] = coeff;
        }
        (Self::from_coeffs(quotient), Self::from_coeffs(remainder))
    }

    /// The monic GCD over `Rat[x]` (Euclid). Zero stays zero.
    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();
        while !b.is_zero() {
            let (_, r) = a.div_rem(&b);
            a = b;
            b = r;
        }
        if a.is_zero() {
            a
        } else {
            a.monic()
        }
    }

    /// Canonical bytes for hashing: degree, then each coefficient's
    /// numerator/denominator as length-prefixed base-10 (the rational
    /// is already coprime with a positive denominator — canonical by
    /// `BigRational`'s construction, so the bytes are canonical too).
    fn canonical_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(self.coeffs.len() as u32).to_be_bytes());
        for c in &self.coeffs {
            let n = c.numer().to_str_radix(10);
            let d = c.denom().to_str_radix(10);
            buf.extend_from_slice(&(n.len() as u32).to_be_bytes());
            buf.extend_from_slice(n.as_bytes());
            buf.extend_from_slice(&(d.len() as u32).to_be_bytes());
            buf.extend_from_slice(d.as_bytes());
        }
    }
}

// ─── The characteristic matrix and its Smith Normal Form over Rat[x] ─────

/// An `n×n` matrix of polynomials, `entries[i][j] ∈ Rat[x]`.
struct PolyMatrix {
    dim: usize,
    entries: Vec<Vec<RatPoly>>,
}

impl PolyMatrix {
    /// The characteristic matrix `xI − A`: `x − aᵢᵢ` on the diagonal,
    /// `−aᵢⱼ` (a constant polynomial) off it.
    fn characteristic(a: &[Vec<Ratio>], dim: usize) -> Self {
        let mut entries = vec![vec![RatPoly::zero(); dim]; dim];
        for (i, row) in entries.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = if i == j {
                    RatPoly::x_minus(&a[i][j])
                } else {
                    RatPoly::from_coeffs(vec![-a[i][j].clone()])
                };
            }
        }
        Self { dim, entries }
    }

    fn get(&self, i: usize, j: usize) -> &RatPoly {
        &self.entries[i][j]
    }

    fn set(&mut self, i: usize, j: usize, val: RatPoly) {
        self.entries[i][j] = val;
    }

    fn swap_rows(&mut self, i: usize, j: usize) {
        self.entries.swap(i, j);
    }

    fn swap_cols(&mut self, i: usize, j: usize) {
        for row in &mut self.entries {
            row.swap(i, j);
        }
    }
}

/// Smith Normal Form of a polynomial matrix over `Rat[x]`: the diagonal
/// `d₁, …, dₙ` (monic, with `dᵢ | dᵢ₊₁`) to which row/column operations
/// reduce it. Applied to `xI − A`, these are `A`'s invariant factors.
///
/// The reduction, per column: bring the smallest-degree non-zero entry
/// of the trailing submatrix to the pivot, clear its row and column by
/// polynomial division (a non-exact division lowers a degree, so the
/// loop descends and terminates), and, once cleared, ensure the pivot
/// divides every remaining entry (else fold a stubborn row in and
/// repeat). A final GCD pass enforces the divisibility chain.
fn smith_normal_form(mat: &mut PolyMatrix) -> Vec<RatPoly> {
    let n = mat.dim;
    for col in 0..n {
        loop {
            // Smallest-degree non-zero entry in the trailing submatrix.
            let mut best: Option<(usize, usize, usize)> = None;
            for i in col..n {
                for j in col..n {
                    if !mat.get(i, j).is_zero() {
                        let d = mat.get(i, j).degree().unwrap_or(0);
                        if best.is_none_or(|(bd, _, _)| d < bd) {
                            best = Some((d, i, j));
                        }
                    }
                }
            }
            let (_, pi, pj) = match best {
                Some(b) => b,
                None => break, // the whole submatrix is zero
            };
            if pi != col {
                mat.swap_rows(pi, col);
            }
            if pj != col {
                mat.swap_cols(pj, col);
            }

            // Clear the column below the pivot and the row right of it.
            let mut changed = false;
            for i in (col + 1)..n {
                if mat.get(i, col).is_zero() {
                    continue;
                }
                let (q, r) = mat.get(i, col).div_rem(mat.get(col, col));
                if !r.is_zero() {
                    changed = true;
                }
                for j in 0..n {
                    let new_val = mat.get(i, j).sub(&q.mul(mat.get(col, j)));
                    mat.set(i, j, new_val);
                }
            }
            for j in (col + 1)..n {
                if mat.get(col, j).is_zero() {
                    continue;
                }
                let (q, r) = mat.get(col, j).div_rem(mat.get(col, col));
                if !r.is_zero() {
                    changed = true;
                }
                for i in 0..n {
                    let new_val = mat.get(i, j).sub(&q.mul(mat.get(i, col)));
                    mat.set(i, j, new_val);
                }
            }
            if changed {
                continue; // a degree fell — re-pivot
            }

            // Column and row are clear. Does the pivot divide the rest?
            let pivot = mat.get(col, col).clone();
            let mut stubborn = None;
            'scan: for i in col..n {
                for j in col..n {
                    if (i, j) == (col, col) || mat.get(i, j).is_zero() {
                        continue;
                    }
                    let (_, r) = mat.get(i, j).div_rem(&pivot);
                    if !r.is_zero() {
                        stubborn = Some(i);
                        break 'scan;
                    }
                }
            }
            match stubborn {
                Some(i) => {
                    for k in 0..n {
                        let sum = mat.get(col, k).add(mat.get(i, k));
                        mat.set(col, k, sum);
                    }
                    // re-pivot with the folded-in row
                }
                None => break, // pivot divides all — this column is done
            }
        }
    }

    let mut diag: Vec<RatPoly> = (0..n)
        .map(|i| {
            let mut d = mat.get(i, i).clone();
            if !d.is_zero() {
                d.make_monic();
            }
            d
        })
        .collect();

    // Enforce dᵢ | dᵢ₊₁ pairwise (GCD/LCM correction).
    for i in 0..diag.len().saturating_sub(1) {
        if diag[i].is_zero() || diag[i + 1].is_zero() {
            continue;
        }
        let g = diag[i].gcd(&diag[i + 1]);
        let (lcm, _) = diag[i].mul(&diag[i + 1]).div_rem(&g);
        diag[i] = g.monic();
        diag[i + 1] = lcm.monic();
    }
    diag
}

// ─── The conjugacy invariant ───────────────────────────────────────────

/// The RCF signature of a [`Delta`] — its exact conjugacy invariant.
/// Per fiber layer, the invariant factors (monic, `dᵢ | dᵢ₊₁`, only the
/// non-trivial degree-≥1 factors kept). Two Deltas share this signature
/// iff every layer is `GL_n(Rat)`-conjugate — the same state through
/// different frames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcfSignature {
    /// One invariant-factor list per fiber layer, in layer order.
    pub layers: Vec<Vec<RatPoly>>,
}

/// The invariant factors of one rational `n×n` matrix: `(xI − A)` →
/// Smith Normal Form → the monic factors of degree ≥ 1.
pub fn invariant_factors(matrix: &[Vec<Ratio>], dim: usize) -> Vec<RatPoly> {
    let mut char_mat = PolyMatrix::characteristic(matrix, dim);
    smith_normal_form(&mut char_mat)
        .into_iter()
        .filter(|p| p.degree().is_some_and(|d| d >= 1))
        .collect()
}

/// Extract fiber layer `l` of a Delta as a rational `dim×dim` matrix.
fn layer_matrix(delta: &Delta, l: usize) -> Vec<Vec<Ratio>> {
    let dim = delta.dim();
    let mut m = vec![vec![Ratio::zero(); dim]; dim];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = delta.entry(i, j, l).clone();
        }
    }
    m
}

/// The conjugacy invariant — the exact RCF signature of a Delta,
/// computed layer by layer. `rcf_signature(a) == rcf_signature(b)` iff
/// `a` and `b` are the same state up to a frame change (a `GL_n(Rat)`
/// conjugation on every layer). This is what the fold gate compares:
/// exact, never a hash.
pub fn rcf_signature(delta: &Delta) -> RcfSignature {
    let layers = (0..delta.fiber())
        .map(|l| invariant_factors(&layer_matrix(delta, l), delta.dim()))
        .collect();
    RcfSignature { layers }
}

/// The compact blake3 fingerprint of a Delta's RCF signature — the
/// same 32 bytes for conjugate states, for commitment binding only
/// (the arbor may carry it). Never a gate: the fold gate compares the
/// exact [`RcfSignature`], because a hash is a lossy summary and the
/// law gates on exact form. blake3, not the ancestor's Keccak (the
/// house hash; the bytes are Netstratum's own, re-witnessed).
pub fn rcf_hash(delta: &Delta) -> [u8; 32] {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"netstratum-rcf-v1");
    buf.extend_from_slice(&(delta.dim() as u32).to_be_bytes());
    buf.extend_from_slice(&(delta.fiber() as u32).to_be_bytes());
    let signature = rcf_signature(delta);
    for factors in &signature.layers {
        buf.extend_from_slice(&(factors.len() as u32).to_be_bytes());
        for factor in factors {
            factor.canonical_bytes(&mut buf);
        }
    }
    *blake3::hash(&buf).as_bytes()
}

// ─── The skew-bipartite fast path ───────────────────────────────────────

/// `p'` — the formal derivative.
fn derivative(p: &RatPoly) -> RatPoly {
    RatPoly::from_coeffs(
        p.coeffs
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| c * Ratio::from_integer((i as i64).into()))
            .collect(),
    )
}

/// The exact quotient `p / q` (the remainder must be zero — a caller bug
/// otherwise, since every use divides by a known factor).
fn exact_div(p: &RatPoly, q: &RatPoly) -> RatPoly {
    let (quotient, remainder) = p.div_rem(q);
    debug_assert!(remainder.is_zero(), "an exact division left a remainder");
    quotient
}

/// YUN'S squarefree decomposition over `Rat[x]` (characteristic zero):
/// monic, pairwise-coprime, squarefree `s_j` with `f = c · ∏ s_jʲ`, listed
/// as `(j, s_j)` for every non-constant `s_j`. Gcds and exact divisions
/// only — nothing is factored.
fn squarefree(f: &RatPoly) -> Vec<(usize, RatPoly)> {
    let mut out = Vec::new();
    if f.degree().is_none_or(|d| d == 0) {
        return out;
    }
    let f = f.clone().monic();
    let df = derivative(&f);
    let a0 = f.gcd(&df);
    let mut b = exact_div(&f, &a0);
    let c = exact_div(&df, &a0);
    let mut d = c.sub(&derivative(&b));
    let mut j = 1;
    while b.degree().is_some_and(|deg| deg > 0) {
        let a = b.gcd(&d);
        let next_b = exact_div(&b, &a);
        let next_c = exact_div(&d, &a);
        if a.degree().is_some_and(|deg| deg > 0) {
            out.push((j, a.monic()));
        }
        d = next_c.sub(&derivative(&next_b));
        b = next_b;
        j += 1;
    }
    out
}

/// `det` of a small square matrix over `Rat[x]`, by the Leibniz sum — for
/// the k×k block of the skew-bipartite path (k is the number of anchor
/// nodes: four for a knowledge map), where k! terms are nothing.
fn small_det(m: &[Vec<RatPoly>]) -> RatPoly {
    let k = m.len();
    let mut perm: Vec<usize> = (0..k).collect();
    let mut total = RatPoly::zero();
    // Heap's algorithm over every permutation, tracking its sign.
    fn sign_of(p: &[usize]) -> bool {
        let mut inversions = 0;
        for i in 0..p.len() {
            for j in (i + 1)..p.len() {
                if p[i] > p[j] {
                    inversions += 1;
                }
            }
        }
        inversions % 2 == 0
    }
    fn walk(n: usize, perm: &mut Vec<usize>, m: &[Vec<RatPoly>], total: &mut RatPoly) {
        if n <= 1 {
            let mut term = RatPoly::from_coeffs(vec![Ratio::one()]);
            for (row, &col) in perm.iter().enumerate() {
                term = term.mul(&m[row][col]);
            }
            *total = if sign_of(perm) { total.add(&term) } else { total.sub(&term) };
            return;
        }
        for i in 0..n {
            walk(n - 1, perm, m, total);
            let swap = if n % 2 == 0 { i } else { 0 };
            perm.swap(swap, n - 1);
        }
    }
    if k == 0 {
        return RatPoly::from_coeffs(vec![Ratio::one()]);
    }
    walk(k, &mut perm, m, &mut total);
    total
}

/// **The skew-bipartite RCF HASH** — [`rcf_hash`] for the one-layer Delta
/// whose only relations run between `k` anchor nodes and `w` leaf nodes,
/// antisymmetrically: `A = [[0, M], [−Mᵀ, 0]]`, `M` the `k×w` block given
/// as `m` (rows = anchors). The same 32 bytes as `rcf_hash` of that Delta,
/// computed from `k×k` algebra instead of a `(k+w)²` Smith normal form over
/// `Rat[x]` (measured: 118 s → milliseconds for a 467-word knowledge map).
///
/// Why it is exact, not an approximation:
///   * `χ_A(x) = x^{w−k} · det(x²I_k + MMᵀ)` — the Schur complement of the
///     `xI_k` block with Sylvester's identity, a `k×k` determinant.
///   * `A` is real skew-symmetric, hence normal, hence diagonalizable over
///     ℂ: every elementary divisor is linear, so the invariant factors are
///     read off the squarefree decomposition of `χ` — `d_t = ∏_{j ≥ t} s_j`
///     (with `x` at its own multiplicity), smallest first, as the Smith form
///     lists them. No factoring over ℚ is needed.
pub fn rcf_hash_skew_bipartite(k: usize, m: &[Vec<Ratio>]) -> [u8; 32] {
    let w = m.first().map_or(0, Vec::len);
    let dim = k + w;
    // G = M·Mᵀ (k×k).
    let g: Vec<Vec<Ratio>> = (0..k)
        .map(|i| (0..k).map(|j| (0..w).fold(Ratio::zero(), |acc, c| acc + &m[i][c] * &m[j][c])).collect())
        .collect();
    // det(x²I + G) over Rat[x].
    let poly: Vec<Vec<RatPoly>> = (0..k)
        .map(|i| {
            (0..k)
                .map(|j| {
                    let lead = if i == j { Ratio::one() } else { Ratio::zero() };
                    RatPoly::from_coeffs(vec![g[i][j].clone(), Ratio::zero(), lead])
                })
                .collect()
        })
        .collect();
    let det = small_det(&poly);
    // χ = x^{w−k} · det: strip every factor of x from det into the x power.
    let mut h = det;
    let mut x_power: i64 = w as i64 - k as i64;
    while h.coeffs.first().is_some_and(|c| c.is_zero()) {
        h = RatPoly::from_coeffs(h.coeffs[1..].to_vec());
        x_power += 1;
    }
    let x_mult = usize::try_from(x_power).expect("χ is a polynomial: the x power is never negative");
    let parts = squarefree(&h);
    let top = parts.iter().map(|(j, _)| *j).max().unwrap_or(0).max(x_mult);
    let x = RatPoly::from_coeffs(vec![Ratio::zero(), Ratio::one()]);
    // d_t for t = top..1 — smallest (fewest components) first.
    let factors: Vec<RatPoly> = (1..=top)
        .rev()
        .map(|t| {
            let mut d = RatPoly::from_coeffs(vec![Ratio::one()]);
            if x_mult >= t {
                d = d.mul(&x);
            }
            for (j, s) in &parts {
                if *j >= t {
                    d = d.mul(s);
                }
            }
            d.monic()
        })
        .collect();
    let mut buf = Vec::new();
    buf.extend_from_slice(b"netstratum-rcf-v1");
    buf.extend_from_slice(&(dim as u32).to_be_bytes());
    buf.extend_from_slice(&1u32.to_be_bytes());
    buf.extend_from_slice(&(factors.len() as u32).to_be_bytes());
    for factor in &factors {
        factor.canonical_bytes(&mut buf);
    }
    *blake3::hash(&buf).as_bytes()
}

#[cfg(test)]
mod skew_bipartite_tests {
    use super::*;
    use crate::delta::Delta;

    fn delta_of(k: usize, m: &[Vec<Ratio>]) -> Delta {
        let w = m.first().map_or(0, Vec::len);
        let mut d = Delta::zero(k + w, 1);
        for (i, row) in m.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                if !v.is_zero() {
                    d.set_antisym(i, k + c, vec![v.clone()]);
                }
            }
        }
        d
    }

    /// A tiny deterministic generator, so the differential sweep is
    /// reproducible without a dependency.
    fn lcg(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *seed >> 33
    }

    #[test]
    fn the_fast_path_is_byte_identical_to_the_smith_form() {
        let mut seed = 7u64;
        for case in 0..120 {
            let k = 1 + (lcg(&mut seed) % 4) as usize;
            let w = (lcg(&mut seed) % 7) as usize;
            let m: Vec<Vec<Ratio>> = (0..k)
                .map(|_| {
                    (0..w)
                        .map(|_| {
                            // Mostly small integers and zeros, with repeats, so
                            // repeated eigenvalues (multiplicities) occur.
                            let v = (lcg(&mut seed) % 4) as i64;
                            Ratio::from_integer(v.into())
                        })
                        .collect()
                })
                .collect();
            let slow = rcf_hash(&delta_of(k, &m));
            let fast = rcf_hash_skew_bipartite(k, &m);
            assert_eq!(fast, slow, "case {case}: k={k} w={w} m={m:?}");
        }
    }

    #[test]
    fn duplicated_leaves_and_the_empty_map_agree_too() {
        let one = Ratio::from_integer(1.into());
        let two = Ratio::from_integer(2.into());
        let m = vec![vec![one.clone(), one.clone(), one.clone()], vec![two.clone(), two.clone(), two.clone()], vec![one.clone(), Ratio::zero(), one.clone()], vec![Ratio::zero(); 3]];
        assert_eq!(rcf_hash_skew_bipartite(4, &m), rcf_hash(&delta_of(4, &m)));
        let empty: Vec<Vec<Ratio>> = vec![Vec::new(); 4];
        assert_eq!(rcf_hash_skew_bipartite(4, &empty), rcf_hash(&Delta::zero(4, 1)));
    }
}
