//! The Delta tensor and the one cocycle-residual computation.

use crate::cage::Lens;
use cfe::ratio::Ratio;
use num_traits::{Signed, Zero};

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

    /// Build a Delta from raw `entries[i][j][l]` — the ONLY writer that
    /// does NOT enforce antisymmetry. It exists for one lawful use: the
    /// image of a general `GL_n(Rat)` conjugation `A·Δ·A⁻¹` (the fold
    /// gate's orbit), which need not be antisymmetric. The result's
    /// lawfulness is never assumed — [`verify_conservation`] is the
    /// check the orbit applies at every step, breaking any orbit whose
    /// operator left the antisymmetric surface. Panics on a ragged
    /// tensor (every row `dim` wide, every entry `fiber` deep).
    pub fn from_entries(entries: Vec<Vec<Vec<Ratio>>>) -> Self {
        let dim = entries.len();
        let fiber = entries.first().and_then(|r| r.first()).map_or(0, Vec::len);
        for row in &entries {
            assert_eq!(row.len(), dim, "Delta must be square");
            for cell in row {
                assert_eq!(cell.len(), fiber, "fiber arity must be uniform");
            }
        }
        Self { dim, fiber, entries }
    }

    /// THE STRUCTURAL BREAK — the first place antisymmetry or the zero
    /// diagonal fails, named as a POSITION `(i, j, l)`, never as a
    /// magnitude.
    ///
    /// This is the form the engine uses (operator ruling 2026-08-01:
    /// no `==` in deterministic code — accounting, not identity). It
    /// asks a structural question and answers with a structural fact:
    /// WHERE the surface is left. It never forms an aggregate, so there
    /// is no scalar for a later reader to compare, threshold, or
    /// descend, and it short-circuits at the first break rather than
    /// summing the universe to decide a yes/no.
    ///
    /// Contrast [`Self::conservation_measurement`], which collapses the
    /// same question to one number and is a LENS reading only.
    pub fn first_conservation_break(&self) -> Option<(usize, usize, usize)> {
        for i in 0..self.dim {
            for l in 0..self.fiber {
                if !self.entries[i][i][l].is_zero() {
                    return Some((i, i, l));
                }
            }
            for j in (i + 1)..self.dim {
                for l in 0..self.fiber {
                    if !(&self.entries[i][j][l] + &self.entries[j][i][l]).is_zero() {
                        return Some((i, j, l));
                    }
                }
            }
        }
        None
    }

    /// The conservation measurement — a DISTANCE to the invariant
    /// surface: `Σ |Δ_ii| + Σ_{i<j} |Δ_ij + Δ_ji|` (THE PROPORTIONAL
    /// LAW: each violation counts exactly its size, in its own units).
    ///
    /// **LENS READING ONLY.** No engine path decides anything from this
    /// number; it exists so an operator surface can render how far a state
    /// sits from the surface. The engine asks
    /// [`Self::first_conservation_break`] instead, which names a
    /// position rather than a magnitude.
    pub fn conservation_measurement(&self) -> Lens {
        let mut violation = Ratio::zero();
        for i in 0..self.dim {
            for l in 0..self.fiber {
                violation += self.entries[i][i][l].abs();
            }
            for j in (i + 1)..self.dim {
                for l in 0..self.fiber {
                    let sum = &self.entries[i][j][l] + &self.entries[j][i][l];
                    violation += sum.abs();
                }
            }
        }
        Lens::of(violation)
    }

    /// The typed read of the conservation invariant — structural, with
    /// early exit, forming no aggregate.
    pub fn verify_conservation(&self) -> bool {
        self.first_conservation_break().is_none()
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

/// The coherence functional: `C(Δ) = Σ |R_ijk,l|` over all ordered
/// distinct triples. `C(Δ) ≥ 0`, exact rational, and `C(Δ) = 0` iff
/// `Δ_ik = Δ_ij + Δ_jk` for every triple — the cocycle condition.
///
/// THE PROPORTIONAL LAW (operator ruling 2026-07-31): the ancestor's `Σ R²`
/// is BARRED as an aggregator of difference. Squares are the
/// mathematics of minimization — a loss shaped to be differentiated
/// and descended; this lens exists to be READ. L1 keeps the scalar in
/// the SAME UNITS as its payload, weights each residual exactly its
/// size, needs no root ever, and keeps the arithmetic lean (squaring
/// doubles bit-length — the denominator lesson). The zero point — the
/// only place the LAW gates — is unchanged.
pub fn coherence_functional(delta: &Delta) -> Lens {
    let mut c = Ratio::zero();
    for_each_residual(delta, |_i, _j, _k, _l, r| {
        c += r.abs();
    });
    Lens::of(c)
}

/// `‖Δ‖₁` — the sum of entry magnitudes (the L1 mass the bounded-step
/// gate reads; the proportional law — no square, no root, same units as the
/// entries).
pub fn magnitude(delta: &Delta) -> Ratio {
    let mut n = Ratio::zero();
    for i in 0..delta.dim {
        for j in 0..delta.dim {
            for l in 0..delta.fiber {
                n += delta.entry(i, j, l).abs();
            }
        }
    }
    n
}
