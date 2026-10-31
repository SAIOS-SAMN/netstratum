//! The fold gate — `F_fold*` and the conjugator witness (U2, I6, PoWC).
//! Re-derived from the v0.1 ancestor `fold.rs` (translate-never-
//! transcribe), then corrected under directive 01 (operator ruling
//! 2026-08-01, the second polarity ruling).
//!
//! The question: may two states be folded into one? A fold collapses a
//! difference. It is lawful only when the difference was a frame change
//! (gauge) — never when it was genuine.
//!
//! # The search fallacy, struck
//!
//! This module previously answered the question by walking a bounded
//! orbit: conjugate `a` by each operator up to `k_max` times and compare
//! [`crate::rcf::rcf_signature`] at every step. **That walk was an
//! identity tautology.** The RCF signature is the invariant-factor
//! sequence of `xI − A`, which is a similarity invariant;
//! [`Operator::apply`] is a similarity (`A·Δ·A⁻¹`). Conjugation
//! therefore cannot move the signature, so every step of the walk
//! compared a quantity that had not changed. `k_max` bounded nothing,
//! and the verdict reduced to "did any operator in the caller's list
//! survive one conjugation" — a property of the argument list, not of
//! the states being gated.
//!
//! `k_max` and the walk are gone. What remains is the exact test the
//! invariant actually supports, at `O(n³ · fiber)`:
//!
//! # The polarity (directive 01, and it is the inverse of the old one)
//!
//! ```text
//! sig(a) ≠ sig(b)  →  TorsionVeto(SignaturesDiffer)
//!                     EXACT PROOF of non-conjugacy. Different invariant
//!                     factors mean no element of GL_n(Rat) — let alone of
//!                     the operator subgroup — carries a onto b. The
//!                     difference is genuine structure. The fold is
//!                     VETOED and the residue EMITS.
//!
//! sig(a) = sig(b)  →  conjugacy is NECESSARY-but-INSUFFICIENT here.
//!                     Equal invariant factors prove conjugacy in the
//!                     FULL GL_n(Rat); the lawful operators are a subgroup
//!                     of it, so equality alone does not prove that a
//!                     LAWFUL frame change relates the two. A presented
//!                     OrbitWitness must be re-walked:
//!                       witness valid    →  GaugeFold(witness)
//!                       witness invalid  →  TorsionVeto(WitnessInvalid)
//!                       no witness       →  TorsionVeto(WitnessAbsent)
//! ```
//!
//! **The default is veto.** Absent proof that the difference was gauge,
//! the difference is treated as genuine and preserved. That is the safe
//! direction: an unproved fold destroys structure irreversibly, while an
//! unnecessary veto only emits a residue.
//!
//! # Why the witness compares tensors, not signatures
//!
//! [`verify_orbit_witness`] ends on `current == *b` — exact entry-wise
//! equality — and not on a signature comparison. A signature comparison
//! there would be the same tautology the search fallacy was: it would
//! pass for any applicable operator whenever `sig(a) = sig(b)`, which is
//! already known by the time the witness is read. Only exact equality
//! proves that this operator, walked this many times, actually carries
//! `a` onto `b`.
//!
//! An operator is a `GL_n(Rat)` conjugation `A·Δ·A⁻¹`. General
//! conjugation need not preserve antisymmetry, so each step is checked
//! by [`Delta::verify_conservation`]: an operator that leaves the
//! antisymmetric surface generates no valid walk (it breaks off), never
//! a panic — the gate is total. Conjugacy is tested by the exact RCF
//! signature ([`crate::rcf::rcf_signature`]), never a hash (the law
//! gates on exact form).

use crate::delta::Delta;
use crate::rcf::rcf_signature;
use cfe::ratio::Ratio;
use num_traits::Zero;

/// A square rational matrix, row-major.
pub type RatMatrix = Vec<Vec<Ratio>>;

/// A mesh operator — a `GL_n⁺(Rat)` conjugation. Carries its inverse
/// (the caller supplies both; the pair defines the conjugation
/// `A·Δ·A⁻¹`) and the orientation flag `det(A) > 0`. A non-positive
/// determinant is not in the group: [`Operator::apply`] refuses it,
/// so it can generate no walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    pub id: usize,
    pub matrix: RatMatrix,
    pub matrix_inv: RatMatrix,
    pub det_sign_positive: bool,
}

impl Operator {
    /// Apply `N(Δ) = A · Δ · A⁻¹`, entry-wise per fiber layer. `None`
    /// if the operator is not in the group (`det ≤ 0`). The result may
    /// not be antisymmetric; the caller checks conservation. Total: no
    /// panic on a well-formed operator of the Delta's dimension.
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

/// The witness of a claimed gauge relation: which operator, and how
/// many conjugations by it carry `a` exactly onto `b`.
///
/// The witness is the prover's output and the judge's input. It proves
/// the difference was a lawful frame change — the branch that permits
/// the collapse. Under directive 01 this is the only branch that needs
/// a proof: the veto branch is established by the signature comparison
/// alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrbitWitness {
    pub operator_id: usize,
    pub step: usize,
}

/// Why a fold was vetoed — the refusal carries its cause whole (the
/// house law: a refusal names what failed, never a bare bool).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorsionCause {
    /// Exact proof of genuine difference: the invariant factors differ,
    /// so no conjugation of any kind relates the two states. This is
    /// the branch that emits — the residue becomes a new plane.
    SignaturesDiffer,
    /// The states are conjugate in `GL_n(Rat)`, but no witness was
    /// presented, so no lawful frame change has been shown to relate
    /// them. Conjugacy unproven; the default is veto.
    WitnessAbsent,
    /// A witness was presented and failed its re-walk: the named
    /// operator does not carry `a` onto `b` in the claimed number of
    /// steps.
    WitnessInvalid,
}

/// The verdict of the fold gate for a pair `(a, b)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldReading {
    /// The difference was a proven lawful frame change: equal invariant
    /// factors and a re-walked conjugator witness. The two states are
    /// one seen through a symmetry, and the fold is lawful. Carries the
    /// witness that permitted it.
    GaugeFold(OrbitWitness),
    /// The fold is forbidden. Either the difference is provably genuine
    /// (`SignaturesDiffer` — the residue must emit as a new plane, the
    /// bounce of 16_UNIT §4a), or gauge was not proven and the default
    /// holds. Carries the cause.
    TorsionVeto(TorsionCause),
}

impl FoldReading {}

/// The prover's search — find an operator that carries `a` exactly onto
/// `b` in one conjugation.
///
/// No `k_max` and no iterated walk (directive 01: the walk was a
/// signature tautology). The operator set is a group presentation: a
/// prover wanting a deeper conjugator composes generators itself and
/// presents the composite as one [`Operator`], which is exactly the
/// element the gate then re-walks. Multi-step witnesses remain
/// verifiable — [`verify_orbit_witness`] re-walks any `step` — but the
/// built-in search is single-step by construction.
///
/// Altitude: this is prover-side work and belongs nowhere inside a
/// gate. The gate calls [`verify_orbit_witness`] (O(step)) or, for
/// the veto branch, [`rcf_signature`] directly (O(n³·fiber)); it never
/// sweeps an operator space. (The `solver`/`solver-verify` split is the
/// house precedent: search is free and non-constitutional, replay is
/// sealed.)
pub fn orbit_lock(a: &Delta, b: &Delta, operators: &[Operator]) -> Option<OrbitWitness> {
    for op in operators {
        let Some(image) = op.apply(a) else {
            continue;
        };
        // Only antisymmetric images are lawful states.
        if !image.verify_conservation() {
            continue;
        }
        if image == *b {
            return Some(OrbitWitness {
                operator_id: op.id,
                step: 1,
            });
        }
    }
    None
}

/// The witness verifier — the judge's half of the split (U1a's pattern
/// applied to the fold gate: the prover searches, the gate replays).
///
/// Given a presented [`OrbitWitness`], re-walk the one path it claims:
/// conjugate `step` times by the named operator and require the result
/// to equal `b` exactly. **No search, no operator sweep, no `k_max`** —
/// the cost is O(`step`) conjugations plus one id lookup, and it is the
/// same exact arithmetic on every node, so two gates always agree.
///
/// **The landing test is tensor equality, never a signature compare.**
/// A signature compare here would re-introduce the search fallacy: it
/// would accept any applicable operator whenever `sig(a) = sig(b)`,
/// which the caller already established before reading the witness.
/// Only `current == *b` proves this operator carries `a` onto `b`.
///
/// Returns `false` on every unproved claim: an unknown operator id, a
/// zero-step claim (which asserts nothing), an operator that leaves the
/// antisymmetric surface, or a walk that does not land exactly on `b`.
pub fn verify_orbit_witness(
    a: &Delta,
    b: &Delta,
    operators: &[Operator],
    witness: &OrbitWitness,
) -> bool {
    // A zero-step claim asserts nothing: `a` is trivially itself.
    if witness.step == 0 {
        return false;
    }
    let Some(operator) = operators.iter().find(|op| op.id == witness.operator_id) else {
        return false;
    };
    let mut current = a.clone();
    for _ in 0..witness.step {
        current = match operator.apply(&current) {
            Some(next) => next,
            None => return false,
        };
        // Only antisymmetric images are lawful states — the same law
        // the search applies, applied to the claimed path.
        if !current.verify_conservation() {
            return false;
        }
    }
    current == *b
}

/// `F_fold*` — The gate (directive 01 polarity).
///
/// A fold of `a` into `b` is lawful IFF their invariant factors agree
/// and a presented witness re-walks to `b` exactly. Signature
/// disagreement is an exact proof of genuine difference and vetoes
/// outright; signature agreement without a valid witness vetoes by
/// default.
///
/// `witness` is `None` when no prover has offered one. That is not an
/// error — it is the ordinary case, and it vetoes.
pub fn f_fold_star(
    a: &Delta,
    b: &Delta,
    operators: &[Operator],
    witness: Option<&OrbitWitness>,
) -> FoldReading {
    // O(n³ · fiber), exact, coordinate-free: the invariant-factor
    // sequences over Rat[x]. Different factors = provably non-conjugate.
    if rcf_signature(a) != rcf_signature(b) {
        return FoldReading::TorsionVeto(TorsionCause::SignaturesDiffer);
    }
    match witness {
        None => FoldReading::TorsionVeto(TorsionCause::WitnessAbsent),
        Some(claimed) => match verify_orbit_witness(a, b, operators, claimed) {
            true => FoldReading::GaugeFold(*claimed),
            false => FoldReading::TorsionVeto(TorsionCause::WitnessInvalid),
        },
    }
}

/// The composed gate (U2e — the singularity pipeline, 16_UNIT §4a):
/// both states pass through `Π_G` ([`crate::reduce::coboundary_reduce`]
/// — what a relabeling could remove, is removed) before the gate
/// gates. This composition is what makes the verdict angle-free: a
/// frame change leaves the RCF signature identical, and a gauge shift
/// is stripped by the reduction — so the verdict cannot depend on the
/// frame or gauge of approach.
///
/// The witness is re-walked against the reduced tensors, so a prover
/// searches on `Π_G(a)` and `Π_G(b)`, not on the raw pair.
pub fn gated_fold(
    a: &Delta,
    b: &Delta,
    operators: &[Operator],
    witness: Option<&OrbitWitness>,
) -> FoldReading {
    f_fold_star(
        &crate::reduce::coboundary_reduce(a),
        &crate::reduce::coboundary_reduce(b),
        operators,
        witness,
    )
}
