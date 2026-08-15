//! TRANSDUCTION — a structure-preserving map between two cellular sheaves.
//!
//! Not a projection onto a shared homogeneous type (that is transmutation,
//! and it collapses whatever does not conform). A transduction is a **chain
//! map** `f_# : C_•(X) → C_•(Y)` that commutes with the differentials,
//!
//! ```text
//!     δ_Y ∘ f_# = f_# ∘ δ_X
//! ```
//!
//! carrying the transfer operator AND its adjoint `f^#` (the transpose over
//! ℤ) explicitly. The commutation is a GATE, not an assumption: if
//! `f_{k+1}·δ_Xᵏ − δ_Yᵏ·f_k ≠ 0` the operation refuses and returns the
//! non-commuting remainder whole as a typed topological obstruction —
//! `TransductionDefect` — rather than forcing a lossy projection. This is
//! how one carrier's structure (e.g. a Grid topology) transduces into
//! another (a cohomological defect field) without flattening.

use crate::mat::{is_zero, matmul, shape, sub, transpose, Mat};
use crate::sheaf::CellSheaf;

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
    ShapeMismatch { grade: usize, expected: (usize, usize), got: (usize, usize) },
    /// `f_{k+1}·δ_Xᵏ − δ_Yᵏ·f_k ≠ 0` — the map does not commute with the
    /// differential. The remainder is the topological obstruction, carried
    /// whole. NEVER truncated into a lossy projection.
    Defect { grade: usize, non_commuting_remainder: Mat },
}

/// Construct a transduction `f_# : X → Y` from the per-grade transfer matrices,
/// gated on the commutation law. On success the adjoint is carried alongside.
pub fn transduce(x: &CellSheaf, y: &CellSheaf, maps: Vec<Mat>) -> Result<ChainMap, TransductionError> {
    // Shape gate: every f_k must send Cᵏ(X) into Cᵏ(Y).
    for (k, f) in maps.iter().enumerate() {
        let expected = (y.dim_cochain(k), x.dim_cochain(k));
        let got = shape(f);
        // A genuinely empty block (either dim 0) reports (0,0); accept it when
        // the expected dimension is likewise degenerate.
        if got != expected && !(expected.0 == 0 || expected.1 == 0) {
            return Err(TransductionError::ShapeMismatch { grade: k, expected, got });
        }
    }
    // Commutation gate: δ_Y ∘ f = f ∘ δ_X at every grade where both sides land.
    for k in 0..maps.len().saturating_sub(1) {
        let f_k = &maps[k];
        let f_k1 = &maps[k + 1];
        let lhs = matmul(f_k1, &x.coboundary(k)); // f_{k+1} · δ_Xᵏ
        let rhs = matmul(&y.coboundary(k), f_k); // δ_Yᵏ · f_k
        if shape(&lhs) == shape(&rhs) {
            let remainder = sub(&lhs, &rhs);
            if !is_zero(&remainder) {
                return Err(TransductionError::Defect { grade: k, non_commuting_remainder: remainder });
            }
        } else if !is_zero(&lhs) || !is_zero(&rhs) {
            // Shapes disagree AND a side is non-trivial → a real obstruction.
            let remainder = if is_zero(&rhs) { lhs } else { rhs };
            return Err(TransductionError::Defect { grade: k, non_commuting_remainder: remainder });
        }
    }
    let adjoint = maps.iter().map(transpose).collect();
    Ok(ChainMap { maps, adjoint })
}
