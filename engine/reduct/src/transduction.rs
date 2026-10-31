//! Transduction — a structure-preserving map between two cellular sheaves.
//!
//! Not a projection onto a shared homogeneous type (that is transmutation,
//! and it collapses whatever does not conform). A transduction is a **chain
//! map** `f_# : C_•(X) → C_•(Y)` that commutes with the differentials,
//!
//! ```text
//!     δ_Y ∘ f_# = f_# ∘ δ_X
//! ```
//!
//! carrying the transfer operator and its adjoint `f^#` (the transpose over
//! ℤ) explicitly. The commutation is a gate, not an assumption: if
//! `f_{k+1}·δ_Xᵏ − δ_Yᵏ·f_k ≠ 0` the operation refuses and returns the
//! non-commuting remainder whole as a typed topological obstruction —
//! `TransductionDefect` — rather than forcing a lossy projection. This is
//! how one carrier's structure (e.g. a Grid topology) transduces into
//! another (a cohomological defect field) without flattening.

use crate::mat::Mat;

/// A verified chain map with its carried adjoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainMap {
    /// `maps[k] = f_k : Cᵏ(X) → Cᵏ(Y)` (shape `dim Cᵏ(Y) × dim Cᵏ(X)`).
    pub maps: Vec<Mat>,
    /// `adjoint[k] = f_kᵀ` (shape `dim Cᵏ(X) × dim Cᵏ(Y)`) — the dual/pullback.
    pub adjoint: Vec<Mat>,
}

/// Why a proposed transduction is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransductionError {
    /// `f_k` is not shaped `dim Cᵏ(Y) × dim Cᵏ(X)`.
    ShapeMismatch {
        grade: usize,
        expected: (usize, usize),
        got: (usize, usize),
    },
    /// `f_{k+1}·δ_Xᵏ − δ_Yᵏ·f_k ≠ 0` — the map does not commute with the
    /// differential. The remainder is the topological obstruction, carried
    /// whole. Never truncated into a lossy projection.
    Defect {
        grade: usize,
        non_commuting_remainder: Mat,
    },
}
