//! THE FOLD GATE witnesses (U2b, I6, PoWC) — REWRITTEN under
//! DIRECTIVE 01 (operator ruling 2026-08-01, the second polarity
//! ruling). The law:
//!
//! ```text
//! sig(a) ≠ sig(b)               →  TorsionVeto(SignaturesDiffer)
//! sig(a) = sig(b), no witness   →  TorsionVeto(WitnessAbsent)
//! sig(a) = sig(b), bad witness  →  TorsionVeto(WitnessInvalid)
//! sig(a) = sig(b), good witness →  GaugeFold(witness)
//! ```
//!
//! These pins fix the DIRECTION of that law — that a PROVEN genuine
//! difference is never folded away, and that a fold is permitted only
//! against a re-walked conjugator — so an inverted verdict cannot pass.
//! The previous pin set asserted the opposite polarity and is
//! superseded; the reasoning is recorded in `fold.rs`'s header.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{
    f_fold_star, judged_fold, orbit_lock, verify_orbit_witness, Delta, FoldVerdict, Operator,
    OrbitWitness, TorsionCause,
};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// The anchor: a 3-entity Delta in exact equilibrium (Δ01 = 1,
/// Δ12 = 1/2, Δ02 = 3/2) — the ancestor's `anchor_3x3`. A pure
/// coboundary: it reduces to nothing.
fn anchor_3() -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![q(1, 1)]);
    d.set_antisym(1, 2, vec![q(1, 2)]);
    d.set_antisym(0, 2, vec![q(3, 2)]);
    d
}

/// The incoherent 3-entity state (Δ01 = 2, Δ12 = 3, Δ02 = 7): the
/// triangle does not compose, so structure survives reduction.
fn incoherent_3() -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![q(2, 1)]);
    d.set_antisym(1, 2, vec![q(3, 1)]);
    d.set_antisym(0, 2, vec![q(7, 1)]);
    d
}

fn anchor_2() -> Delta {
    let mut d = Delta::zero(2, 1);
    d.set_antisym(0, 1, vec![q(1, 1)]);
    d
}

/// A shear operator `I + s·E_{row,col}` and its exact inverse
/// (unipotent — det 1). It does NOT preserve antisymmetry, so it
/// generates no valid walk.
fn shear(dim: usize, row: usize, col: usize, s: Ratio) -> Operator {
    let mut matrix = vec![vec![q(0, 1); dim]; dim];
    let mut matrix_inv = vec![vec![q(0, 1); dim]; dim];
    for (i, (mrow, irow)) in matrix.iter_mut().zip(matrix_inv.iter_mut()).enumerate() {
        mrow[i] = q(1, 1);
        irow[i] = q(1, 1);
    }
    matrix[row][col] = s.clone();
    matrix_inv[row][col] = -s;
    Operator { id: 1, matrix, matrix_inv, det_sign_positive: true }
}

/// The identity operator — its own inverse, det 1.
fn identity_op(dim: usize) -> Operator {
    let mut matrix = vec![vec![q(0, 1); dim]; dim];
    for (i, row) in matrix.iter_mut().enumerate() {
        row[i] = q(1, 1);
    }
    Operator { id: 0, matrix: matrix.clone(), matrix_inv: matrix, det_sign_positive: true }
}

/// A permutation frame: relabel the entities by `perm` (a conjugation
/// P·Δ·P⁻¹ with P the permutation matrix — its inverse is its
/// transpose). Orthogonal, so it PRESERVES antisymmetry. Use only EVEN
/// permutations: an odd one has det −1 and is not in `GL⁺`.
fn frame(dim: usize, perm: &[usize], id: usize) -> Operator {
    let mut matrix = vec![vec![q(0, 1); dim]; dim];
    let mut matrix_inv = vec![vec![q(0, 1); dim]; dim];
    for (from, &to) in perm.iter().enumerate() {
        matrix[to][from] = q(1, 1);
        matrix_inv[from][to] = q(1, 1);
    }
    Operator { id, matrix, matrix_inv, det_sign_positive: true }
}

/// Add a coboundary (a pure gauge distortion: Δ_ij += p_j − p_i).
fn gauge_shifted(delta: &Delta, potentials: &[Ratio]) -> Delta {
    let dim = delta.dim();
    let fiber = delta.fiber();
    let mut entries = vec![vec![vec![q(0, 1); fiber]; dim]; dim];
    for (i, row) in entries.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            for (l, slot) in cell.iter_mut().enumerate() {
                *slot = delta.entry(i, j, l) + (&potentials[j] - &potentials[i]);
            }
        }
    }
    Delta::from_entries(entries)
}

// ─── THE POLARITY ──────────────────────────────────────────────────────

#[test]
fn the_polarity_is_pinned_against_inversion() {
    // THE LOAD-BEARING PIN (DIRECTIVE 01). Differing invariant factors
    // are an EXACT PROOF that no conjugation relates the two states —
    // the difference is genuine structure. It must VETO, and the
    // residue emits. The prior law folded exactly this case away.
    let coherent = anchor_3();
    let incoherent = incoherent_3();
    let ops = vec![identity_op(3), frame(3, &[1, 2, 0], 9)];

    assert_eq!(
        judged_fold(&coherent, &incoherent, &ops, None),
        FoldVerdict::TorsionVeto(TorsionCause::SignaturesDiffer),
        "POLARITY: a proven genuine difference is NEVER folded away"
    );

    // And the converse half: agreement alone does not fold. A state
    // against ITSELF has identical signatures, but permission needs a
    // witness — and with one, it folds.
    assert_eq!(
        f_fold_star(&coherent, &coherent, &ops, None),
        FoldVerdict::TorsionVeto(TorsionCause::WitnessAbsent),
        "POLARITY: no witness, no fold — the default is veto"
    );
    let identity_witness = OrbitWitness { operator_id: 0, step: 1 };
    assert_eq!(
        f_fold_star(&coherent, &coherent, &ops, Some(&identity_witness)),
        FoldVerdict::GaugeFold(identity_witness),
        "a state IS gauge-identical to itself, and the identity proves it"
    );
}

#[test]
fn the_default_is_veto_not_fold() {
    // Absent proof, structure is preserved. An unproved fold destroys
    // structure irreversibly; an unnecessary veto only emits a residue.
    let a = incoherent_3();
    let ops = vec![identity_op(3)];
    assert_eq!(
        f_fold_star(&a, &a, &ops, None),
        FoldVerdict::TorsionVeto(TorsionCause::WitnessAbsent)
    );
    // An empty operator list cannot prove anything either.
    assert_eq!(
        f_fold_star(&a, &a, &[], None),
        FoldVerdict::TorsionVeto(TorsionCause::WitnessAbsent)
    );
}

// ─── THE WITNESS ───────────────────────────────────────────────────────

#[test]
fn a_valid_conjugator_witness_permits_the_fold() {
    // The prover searches, finds the frame that carries `a` onto `b`,
    // and presents it. The judge re-walks that one path and folds.
    let a = anchor_3();
    let rotate = frame(3, &[1, 2, 0], 9); // even permutation, det +1
    let b = rotate.apply(&a).expect("the frame applies");
    let ops = vec![identity_op(3), rotate];

    let witness = orbit_lock(&a, &b, &ops).expect("the prover finds the conjugator");
    assert_eq!(witness, OrbitWitness { operator_id: 9, step: 1 });
    assert!(verify_orbit_witness(&a, &b, &ops, &witness), "the judge re-walks it");
    assert_eq!(
        f_fold_star(&a, &b, &ops, Some(&witness)),
        FoldVerdict::GaugeFold(witness),
        "a proven frame change IS gauge — the fold is lawful"
    );
}

#[test]
fn the_witness_lands_on_the_tensor_never_on_the_signature() {
    // THE TAUTOLOGY GUARD — the single most important pin here.
    //
    // `a` and `b` are conjugate (same invariant factors) but NOT equal.
    // The identity operator is perfectly valid and preserves the
    // signature exactly, so a signature-comparing verifier would ACCEPT
    // an identity witness for this pair — that was the search fallacy,
    // one level down. Exact tensor equality refuses it, and only the
    // real conjugator passes.
    let a = anchor_3();
    let rotate = frame(3, &[1, 2, 0], 9);
    let b = rotate.apply(&a).expect("applies");
    assert_ne!(a, b, "the pair really is distinct");
    let ops = vec![identity_op(3), rotate];

    let identity_claim = OrbitWitness { operator_id: 0, step: 1 };
    assert!(
        !verify_orbit_witness(&a, &b, &ops, &identity_claim),
        "the identity preserves the signature but does NOT carry a onto b"
    );
    assert_eq!(
        f_fold_star(&a, &b, &ops, Some(&identity_claim)),
        FoldVerdict::TorsionVeto(TorsionCause::WitnessInvalid),
        "a signature-preserving non-conjugator proves nothing"
    );

    let real_claim = OrbitWitness { operator_id: 9, step: 1 };
    assert!(verify_orbit_witness(&a, &b, &ops, &real_claim));
}

#[test]
fn a_forged_witness_refuses() {
    // Every unproved claim refuses, and each refusal is typed.
    let a = anchor_3();
    let rotate = frame(3, &[1, 2, 0], 9);
    let b = rotate.apply(&a).expect("applies");
    let ops = vec![rotate];

    for forged in [
        OrbitWitness { operator_id: 77, step: 1 }, // no such operator
        OrbitWitness { operator_id: 9, step: 0 },  // asserts nothing
        OrbitWitness { operator_id: 9, step: 2 },  // overshoots the class
    ] {
        assert!(!verify_orbit_witness(&a, &b, &ops, &forged), "forged: {forged:?}");
        assert_eq!(
            f_fold_star(&a, &b, &ops, Some(&forged)),
            FoldVerdict::TorsionVeto(TorsionCause::WitnessInvalid)
        );
    }
}

#[test]
fn an_operator_that_leaves_the_antisymmetric_surface_proves_nothing() {
    // A shear breaks conservation at the first conjugation, so it
    // generates no valid walk and can witness nothing.
    let a = anchor_2();
    let shear_ops = [shear(2, 0, 1, q(1, 10))];
    assert_eq!(orbit_lock(&a, &a, &shear_ops), None, "the shear carries a nowhere lawful");
    for step in 1..6 {
        assert!(!verify_orbit_witness(
            &a,
            &a,
            &shear_ops,
            &OrbitWitness { operator_id: 1, step }
        ));
    }
}

#[test]
fn a_broken_operator_generates_no_orbit() {
    // An operator with det ≤ 0 is not in the group: it applies to
    // nothing, so it can never witness — the gate stays total.
    let a = anchor_2();
    let mut bad = identity_op(2);
    bad.det_sign_positive = false;
    assert!(bad.apply(&a).is_none(), "a non-group operator does not apply");
    assert_eq!(orbit_lock(&a, &a, &[bad.clone()]), None);
    assert!(!verify_orbit_witness(&a, &a, &[bad], &OrbitWitness { operator_id: 0, step: 1 }));
}

// ─── ANGLE-FREEDOM ─────────────────────────────────────────────────────

#[test]
fn the_angle_free_witness() {
    // THE SINGULARITY LAW (16_UNIT §4a) as a test: the composed gate's
    // verdict is INVARIANT to the frame and gauge of approach. Π_G
    // strips what a relabeling could remove, and the RCF signature is
    // conjugation-invariant, so the verdict cannot depend on how the
    // pair arrived.
    let a = anchor_3();
    let b = incoherent_3();
    let ops = vec![identity_op(3), frame(3, &[1, 2, 0], 9)];
    let base = judged_fold(&a, &b, &ops, None);
    assert_eq!(base, FoldVerdict::TorsionVeto(TorsionCause::SignaturesDiffer));

    // Every EVEN 3-entity permutation frame: the verdict holds.
    for (n, perm) in [[0, 1, 2], [1, 2, 0], [2, 0, 1]].into_iter().enumerate() {
        let f = frame(3, &perm, 20 + n);
        let fa = f.apply(&a).expect("the frame applies");
        let fb = f.apply(&b).expect("the frame applies");
        assert_eq!(
            judged_fold(&fa, &fb, &ops, None),
            base,
            "the verdict cannot depend on the frame of approach: {perm:?}"
        );
    }

    // A gauge coboundary on either input: stripped by Π_G, verdict holds.
    let potentials = [q(0, 1), q(5, 1), q(1, 3)];
    let shifted_a = gauge_shifted(&a, &potentials);
    let shifted_b = gauge_shifted(&b, &potentials);
    assert_eq!(judged_fold(&shifted_a, &b, &ops, None), base);
    assert_eq!(judged_fold(&a, &shifted_b, &ops, None), base);
    assert_eq!(judged_fold(&shifted_a, &shifted_b, &ops, None), base);
}

#[test]
fn a_pure_gauge_pair_agrees_on_signature() {
    // Two states differing only by a coboundary reduce to the same
    // representative, so `judged_fold` reaches the witness branch
    // rather than the exact-proof branch. Gauge is never mistaken for
    // structure.
    let base = incoherent_3();
    let shifted = gauge_shifted(&base, &[q(0, 1), q(5, 1), q(1, 3)]);
    let ops = vec![identity_op(3)];
    assert_eq!(
        judged_fold(&base, &shifted, &ops, None),
        FoldVerdict::TorsionVeto(TorsionCause::WitnessAbsent),
        "signatures agree (it is only gauge) — what is missing is the proof, not the sameness"
    );
}
