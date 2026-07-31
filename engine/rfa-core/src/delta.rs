//! The Delta tensor and the one cocycle-residual computation.

use cfe::ratio::Ratio;
use num_traits::Zero;

/// An antisymmetric relational tensor over exact rational.
///
/// `dim` entities; each ordered pair `(i, j)` carries a vector of
/// `fiber` exact-rational coordinates. Invariants (enforced by the
/// constructors and [`Delta::set_antisym`]):
///
/// - antisymmetry: `Δ_ij = −Δ_ji` for all `i, j`
/// - zero diagonal: `Δ_ii = 0`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta {
    dim: usize,
    fiber: usize,
    /// `entries[i][j]` = the the exact rationals^fiber coordinates of the (i, j) relation.
    entries: Vec<Vec<Vec<Ratio>>>,
}

impl Delta {
    /// The zero tensor — every relation unmeasured (all coordinates 0).
    pub fn zero(dim: usize, fiber: usize) -> Self {
        Self {
            dim,
            fiber,
            entries: vec![vec![vec![Ratio::zero(); fiber]; dim]; dim],
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn fiber(&self) -> usize {
        self.fiber
    }

    /// Set the (i, j) relation and its antisymmetric twin in one
    /// stroke — the only lawful writer (`i == j` refuses: the diagonal
    /// is identically zero).
    ///
    /// # Panics
    /// If `i == j`, or an index/coordinate length is out of bounds.
    pub fn set_antisym(&mut self, i: usize, j: usize, coords: Vec<Ratio>) {
        assert_ne!(i, j, "the diagonal is identically zero — Δ_ii = 0");
        assert_eq!(coords.len(), self.fiber, "coordinate arity mismatch");
        self.entries[j][i] = coords.iter().map(|c| -c).collect();
        self.entries[i][j] = coords;
    }

    /// One coordinate of one relation.
    pub fn entry(&self, i: usize, j: usize, l: usize) -> &Ratio {
        &self.entries[i][j][l]
    }

    /// The conservation measurement — a DISTANCE to the invariant
    /// surface, not a boolean: `Σ Δ_ii² + Σ_{i<j} (Δ_ij + Δ_ji)²`.
    /// Zero iff antisymmetry and the zero diagonal hold exactly.
    pub fn conservation_measurement(&self) -> Ratio {
        let mut violation = Ratio::zero();
        for i in 0..self.dim {
            for l in 0..self.fiber {
                let d = &self.entries[i][i][l];
                violation += d * d;
            }
            for j in (i + 1)..self.dim {
                for l in 0..self.fiber {
                    let sum = &self.entries[i][j][l] + &self.entries[j][i][l];
                    violation += &sum * &sum;
                }
            }
        }
        violation
    }

    /// The typed read of the conservation invariant.
    pub fn verify_conservation(&self) -> bool {
        self.conservation_measurement().is_zero()
    }
}

/// The ONE computation of the cocycle condition. For every ordered
/// distinct triple `(i, j, k)` and fiber coordinate `l`:
///
/// `R_ijk,l = Δ_ik,l − Δ_ij,l − Δ_jk,l`
///
/// Non-zero residuals are handed to `f(i, j, k, l, residual)`; zero
/// residuals are skipped (sparse evaluation). Every downstream
/// measurement composes this iterator — the algebra has one cocycle
/// condition, the code has one computation of it.
pub fn for_each_residual<F>(delta: &Delta, mut f: F)
where
    F: FnMut(usize, usize, usize, usize, &Ratio),
{
    let n = delta.dim;
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                for l in 0..delta.fiber {
                    let residual =
                        delta.entry(i, k, l) - delta.entry(i, j, l) - delta.entry(j, k, l);
                    if !residual.is_zero() {
                        f(i, j, k, l, &residual);
                    }
                }
            }
        }
    }
}

/// The coherence functional: `C(Δ) = Σ R_ijk,l²` over all ordered
/// distinct triples. `C(Δ) ≥ 0`, exact rational, and `C(Δ) = 0` iff
/// `Δ_ik = Δ_ij + Δ_jk` for every triple — the cocycle condition.
pub fn coherence_functional(delta: &Delta) -> Ratio {
    let mut c = Ratio::zero();
    for_each_residual(delta, |_i, _j, _k, _l, r| {
        c += r * r;
    });
    c
}

/// `‖Δ‖²` — the sum of squared entries (the magnitude the bounded-step
/// gate reads).
pub fn norm_squared(delta: &Delta) -> Ratio {
    let mut n = Ratio::zero();
    for i in 0..delta.dim {
        for j in 0..delta.dim {
            for l in 0..delta.fiber {
                let e = delta.entry(i, j, l);
                n += e * e;
            }
        }
    }
    n
}
