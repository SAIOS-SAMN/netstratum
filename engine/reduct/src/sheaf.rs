//! The cellular sheaf — the stratified carrier (IAE-2, step 2).
//!
//! A base cell complex whose cells carry **heterogeneous stalks**: each cell
//! `σ` holds its own free ℤ-module `F(σ) = ℤ^{dim σ}`, its native grading —
//! no global `m`, so an entity living in a 2-manifold and one in a 5-manifold
//! keep their own dimension and are never zero-padded into a shared type
//! (that padding is the dimensional collapse this exists to refuse).
//!
//! For an incidence `σ ◁ τ` (a `k`-cell `σ` a face of a `(k+1)`-cell `τ`) the
//! **restriction map** `ρ_{σ,τ}: F(σ) → F(τ)` is an exact integer morphism,
//! and the coboundary is assembled straight from the signed incidence
//! restrictions — never a basepoint gauge, never a flat coordinate loop:
//!
//! ```text
//!     δᵏ(c)(τ) = Σ_{σ ◁ τ} [τ:σ] · ρ_{σ,τ}( c(σ) )
//! ```
//!
//! The stratified peel walks the filtration `X₀ ⊂ X₁ ⊂ …` by incidence —
//! whole geometric strata — rather than iterating Cartesian coordinate axes.
//! `verify_complex` gates the sheaf on `δᵏ⁺¹ ∘ δᵏ = 0` (the restrictions
//! compose consistently around every 2-cell); a violation is refused typed
//! with the non-zero remainder carried whole.

use crate::mat::{is_zero, matmul, shape, zeros, Mat};
use num_bigint::BigInt;

/// One incidence `σ ◁ τ` with its orientation sign `[τ:σ]` and restriction
/// `ρ_{σ,τ}` (shape `dim F(τ) × dim F(σ)`). `τ` sits at grade `face_grade + 1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Incidence {
    pub face_grade: usize,
    pub face_idx: usize,
    pub coface_idx: usize,
    pub sign: BigInt,
    pub restriction: Mat,
}

/// A cellular sheaf: per-grade stalk dimensions and the incidence restrictions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellSheaf {
    /// `stalk_dims[k][i]` = `dim F(σ)` for the `i`-th `k`-cell. Heterogeneous.
    stalk_dims: Vec<Vec<usize>>,
    incidences: Vec<Incidence>,
}

/// Why a proposed sheaf is not a valid complex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SheafError {
    /// A restriction's shape does not match `dim F(τ) × dim F(σ)`.
    RestrictionShape {
        face_grade: usize,
        face_idx: usize,
        coface_idx: usize,
        expected: (usize, usize),
        got: (usize, usize),
    },
    /// An incidence references a cell outside the declared grades.
    CellOutOfRange { grade: usize, idx: usize },
    /// `δᵏ⁺¹ ∘ δᵏ ≠ 0` — the restrictions do not compose to a complex. The
    /// non-zero remainder is carried whole (a topological obstruction, not a
    /// bool). This is exactness enforced at construction.
    NotAComplex { grade: usize, remainder: Mat },
}

impl CellSheaf {
    /// Build and gate a sheaf: validate every restriction shape, then verify
    /// `δ ∘ δ = 0`. Refuses typed on the first violation.
    pub fn new(stalk_dims: Vec<Vec<usize>>, incidences: Vec<Incidence>) -> Result<Self, SheafError> {
        let sheaf = CellSheaf { stalk_dims, incidences };
        for inc in &sheaf.incidences {
            let fg = inc.face_grade;
            let face_dim = sheaf
                .stalk_dim(fg, inc.face_idx)
                .ok_or(SheafError::CellOutOfRange { grade: fg, idx: inc.face_idx })?;
            let coface_dim = sheaf
                .stalk_dim(fg + 1, inc.coface_idx)
                .ok_or(SheafError::CellOutOfRange { grade: fg + 1, idx: inc.coface_idx })?;
            let got = shape(&inc.restriction);
            if got != (coface_dim, face_dim) {
                return Err(SheafError::RestrictionShape {
                    face_grade: fg,
                    face_idx: inc.face_idx,
                    coface_idx: inc.coface_idx,
                    expected: (coface_dim, face_dim),
                    got,
                });
            }
        }
        sheaf.verify_complex()?;
        Ok(sheaf)
    }

    /// Number of grades the complex spans.
    pub fn grades(&self) -> usize {
        self.stalk_dims.len()
    }

    /// `dim F(σ)` for the `idx`-th `grade`-cell, or `None` if out of range.
    pub fn stalk_dim(&self, grade: usize, idx: usize) -> Option<usize> {
        self.stalk_dims.get(grade).and_then(|g| g.get(idx)).copied()
    }

    /// `dim Cᵏ = Σ_σ dim F(σ)` over the `k`-cells (0 outside the grades).
    pub fn dim_cochain(&self, k: usize) -> usize {
        self.stalk_dims.get(k).map(|g| g.iter().sum()).unwrap_or(0)
    }

    /// Offset of the `idx`-th `k`-cell's stalk block inside `Cᵏ`.
    fn offset(&self, k: usize, idx: usize) -> usize {
        self.stalk_dims.get(k).map(|g| g[..idx].iter().sum()).unwrap_or(0)
    }

    /// The coboundary `δᵏ : Cᵏ → Cᵏ⁺¹` as an exact integer matrix
    /// (`dim Cᵏ⁺¹ × dim Cᵏ`), assembled from the signed incidence restrictions.
    pub fn coboundary(&self, k: usize) -> Mat {
        let rows = self.dim_cochain(k + 1);
        let cols = self.dim_cochain(k);
        let mut m = zeros(rows, cols);
        for inc in &self.incidences {
            if inc.face_grade != k {
                continue;
            }
            let row0 = self.offset(k + 1, inc.coface_idx);
            let col0 = self.offset(k, inc.face_idx);
            for (r, rrow) in inc.restriction.iter().enumerate() {
                for (c, val) in rrow.iter().enumerate() {
                    m[row0 + r][col0 + c] += &inc.sign * val;
                }
            }
        }
        m
    }

    /// Verify `δᵏ⁺¹ ∘ δᵏ = 0` for every grade — the complex axiom.
    pub fn verify_complex(&self) -> Result<(), SheafError> {
        for k in 0..self.grades().saturating_sub(1) {
            let d_k = self.coboundary(k);
            let d_k1 = self.coboundary(k + 1);
            let comp = matmul(&d_k1, &d_k);
            if !is_zero(&comp) {
                return Err(SheafError::NotAComplex { grade: k, remainder: comp });
            }
        }
        Ok(())
    }
}
