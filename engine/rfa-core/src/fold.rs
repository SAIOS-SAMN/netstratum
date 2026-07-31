//! THE FOLD GATE — `F_fold*` and the orbit-lock (U2, I6, PoWC).
//! Re-derived from the v0.1 ancestor `fold.rs` (translate-never-
//! transcribe).
//!
//! The question: may two states be FOLDED into one? A fold collapses a
//! difference. It is lawful ONLY when the difference was a frame change
//! (gauge) — never when it was genuine. `orbit_lock(a, b)` decides:
//! is there a mesh operator whose repeated conjugation carries `a`'s
//! conjugacy class onto `b`'s within `K_max` steps? If so, the two are
//! the same state seen through a symmetry — the fold is lawful. If not,
//! the difference is TORSION, genuine; the collapse is forbidden and
//! the residue must EMIT as a new plane (the singularity's bounce,
//! 16_UNIT §4a — emission, never annihilation).
//!
//! An operator is a `GL_n(Q)` conjugation `A·Δ·A⁻¹`. General
//! conjugation need not preserve antisymmetry, so each orbit step is
//! checked by [`Delta::verify_conservation`]: an operator that leaves
//! the antisymmetric surface simply does not generate a valid orbit
//! (it breaks off), never a panic — the gate is total. Conjugacy is
//! tested by the EXACT RCF signature ([`crate::rcf::rcf_signature`]),
//! never a hash (the law judges on exact form).

use crate::delta::Delta;
use crate::rcf::rcf_signature;
use cfe::ratio::Ratio;
use num_traits::Zero;

/// A square rational matrix, row-major.
pub type QMatrix = Vec<Vec<Ratio>>;

/// A mesh operator — a `GL_n⁺(Q)` conjugation. Carries its inverse
/// (the caller supplies both; the pair defines the conjugation
/// `A·Δ·A⁻¹`) and the orientation flag `det(A) > 0`. A non-positive
/// determinant is not in the group: [`Operator::apply`] refuses it,
/// so it can generate no orbit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    pub id: usize,
    pub matrix: QMatrix,
    pub matrix_inv: QMatrix,
    pub det_sign_positive: bool,
}

impl Operator {
    /// Apply `N(Δ) = A · Δ · A⁻¹`, entry-wise per fiber layer. `None`
    /// if the operator is not in the group (`det ≤ 0`) — it generates
    /// no orbit. The result may not be antisymmetric; the caller
    /// checks conservation. Total: no panic on a well-formed operator
    /// of the Delta's dimension.
    pub fn apply(&self, delta: &Delta) -> Option<Delta> {
        if !self.det_sign_positive {
            return None;
        }
        let n = delta.dim();
        let m = delta.fiber();
        if self.matrix.len() != n || self.matrix_inv.len() != n {
            return None;
        }

        // intermediate = A · Δ : intermediate[i][j][l] = Σ_k A_ik Δ_kj,l
        let mut intermediate = vec![vec![vec![Ratio::zero(); m]; n]; n];
        for (i, inter_row) in intermediate.iter_mut().enumerate() {
            for (k, a_ik) in self.matrix[i].iter().enumerate() {
                if a_ik.is_zero() {
                    continue;
                }
                for (j, inter_cell) in inter_row.iter_mut().enumerate() {
                    for (l, slot) in inter_cell.iter_mut().enumerate() {
                        *slot += a_ik * delta.entry(k, j, l);
                    }
                }
            }
        }

        // result = intermediate · A⁻¹ : result[i][j][l] = Σ_k inter[i][k][l] (A⁻¹)_kj
        let mut result = vec![vec![vec![Ratio::zero(); m]; n]; n];
        for (i, res_row) in result.iter_mut().enumerate() {
            for (k, inter_ik) in intermediate[i].iter().enumerate() {
                for (j, res_cell) in res_row.iter_mut().enumerate() {
                    let ainv_kj = &self.matrix_inv[k][j];
                    if ainv_kj.is_zero() {
                        continue;
                    }
                    for (l, slot) in res_cell.iter_mut().enumerate() {
                        *slot += &inter_ik[l] * ainv_kj;
                    }
                }
            }
        }
        Some(Delta::from_entries(result))
    }
}

/// THE ORBIT LOCK (`O_ab ≠ ∅`). Is there a mesh operator whose orbit
/// carries `a`'s conjugacy class onto `b`'s within `k_max` conjugations
/// — a torsion cycle? Each step conjugates and is kept only while it
/// stays on the antisymmetric surface (an operator that breaks
/// conservation generates no valid orbit and is abandoned). Conjugacy
/// is the EXACT RCF signature. Returns the operator id and step at
/// which the lock was found (the witness), or `None` — no lock, the
/// difference is genuine.
pub fn orbit_lock(
    a: &Delta,
    b: &Delta,
    operators: &[Operator],
    k_max: usize,
) -> Option<OrbitWitness> {
    let sig_a = rcf_signature(a);
    let sig_b = rcf_signature(b);
    for op in operators {
        let mut current = a.clone();
        for step in 0..k_max {
            current = match op.apply(&current) {
                Some(next) => next,
                None => break,
            };
            // Only antisymmetric images are valid orbit elements.
            if !current.verify_conservation() {
                break;
            }
            let sig_current = rcf_signature(&current);
            if sig_current == sig_b {
                return Some(OrbitWitness { operator_id: op.id, step: step + 1 });
            }
            // Returned to the start's class: the orbit is exhausted
            // under this operator (a closed cycle back to `a`).
            if step > 0 && sig_current == sig_a {
                break;
            }
        }
    }
    None
}

/// The witness of a found orbit lock: which operator, and after how
/// many conjugations `a`'s class reached `b`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrbitWitness {
    pub operator_id: usize,
    pub step: usize,
}

/// The verdict of the fold gate for a pair `(a, b)`. The law
/// (SYSTEM_SPEC §4, kernel CHARTER §6, EQUATIONS I6): a fold is lawful
/// IFF `orbit_lock = ∅`. A torsion cycle is a rigid, information-bearing
/// lock — its presence FORBIDS the collapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldVerdict {
    /// No orbit lock (`O_ab = ∅`): the difference was gauge — the fold
    /// is LAWFUL, the two states are one seen through a symmetry.
    GaugeFold,
    /// A torsion orbit locks the classes (`O_ab ≠ ∅`): the difference
    /// is genuine. The collapse is FORBIDDEN; the residue must emit as
    /// a new plane (the bounce — 16_UNIT §4a). Carries the lock witness.
    TorsionVeto(OrbitWitness),
}

/// `F_fold*` — the gate. A fold of `a` into `b` is lawful IFF no orbit
/// lock connects their classes (the difference was gauge). An orbit
/// lock is the veto: the difference is a genuine torsion the collapse
/// would destroy, so it is preserved and emitted, never compressed.
pub fn f_fold_star(
    a: &Delta,
    b: &Delta,
    operators: &[Operator],
    k_max: usize,
) -> FoldVerdict {
    match orbit_lock(a, b, operators, k_max) {
        Some(witness) => FoldVerdict::TorsionVeto(witness),
        None => FoldVerdict::GaugeFold,
    }
}

/// THE COMPOSED GATE (U2e — the singularity pipeline, 16_UNIT §4a):
/// both states pass through `Π_G` (`coboundary_reduce` — what a
/// relabeling could remove, is removed) BEFORE the gate judges. This
/// composition is what makes the verdict ANGLE-FREE: a frame change
/// (conjugation) leaves the RCF signature identical, and a gauge
/// shift (an added coboundary) is stripped by the reduction — so the
/// verdict cannot depend on the frame or gauge of approach. Witnessed
/// as the angle-free pin in `fold_pins.rs`.
pub fn judged_fold(
    a: &Delta,
    b: &Delta,
    operators: &[Operator],
    k_max: usize,
) -> FoldVerdict {
    f_fold_star(
        &crate::reduce::coboundary_reduce(a),
        &crate::reduce::coboundary_reduce(b),
        operators,
        k_max,
    )
}
