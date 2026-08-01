//! REGRESSION PINS for the two faults DIRECTIVE 01 and DIRECTIVE 05
//! corrected. Each test states a property that must never come back.
//!
//! The audit that produced these found the fold gate deciding nothing:
//! `orbit_lock` walked an orbit comparing `rcf_signature` at each step,
//! but the signature is a SIMILARITY invariant and conjugation is a
//! similarity, so the compared quantity never moved. The verdict
//! reduced to "did any operator in the caller's list survive one
//! conjugation", and the polarity was inverted on top of that.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{
    coboundary_reduce, coboundary_reduce_over, f_fold_star, judged_fold, rcf_signature, Delta,
    FoldVerdict, Operator, OrbitWitness, Support, TorsionCause,
};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

fn identity_op(dim: usize) -> Operator {
    let mut matrix = vec![vec![q(0, 1); dim]; dim];
    for (i, row) in matrix.iter_mut().enumerate() {
        row[i] = q(1, 1);
    }
    Operator { id: 0, matrix: matrix.clone(), matrix_inv: matrix, det_sign_positive: true }
}

fn frame(dim: usize, perm: &[usize], id: usize) -> Operator {
    let mut matrix = vec![vec![q(0, 1); dim]; dim];
    let mut matrix_inv = vec![vec![q(0, 1); dim]; dim];
    for (from, &to) in perm.iter().enumerate() {
        matrix[to][from] = q(1, 1);
        matrix_inv[from][to] = q(1, 1);
    }
    Operator { id, matrix, matrix_inv, det_sign_positive: true }
}

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

fn triangle(d01: Ratio, d12: Ratio, d02: Ratio) -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![d01]);
    d.set_antisym(1, 2, vec![d12]);
    d.set_antisym(0, 2, vec![d02]);
    d
}

// ─── WHY THERE IS NO WALK ──────────────────────────────────────────────

/// THE MATHEMATICAL FACT the search fallacy ignored. Conjugation cannot
/// change the RCF signature — similar matrices share their invariant
/// factors. Any gate that walks an orbit comparing signatures is
/// comparing a constant, and this test is why `orbit_lock` takes no
/// `k_max` and performs no walk.
#[test]
fn conjugation_never_changes_the_signature() {
    let a = triangle(q(2, 1), q(3, 1), q(7, 1));
    let p = frame(3, &[1, 2, 0], 9);
    let before = rcf_signature(&a);

    let mut current = a.clone();
    for step in 1..=6 {
        current = p.apply(&current).expect("the permutation frame applies");
        assert!(current.verify_conservation(), "a frame preserves antisymmetry");
        assert_eq!(
            rcf_signature(&current),
            before,
            "step {step}: the signature is a similarity invariant and cannot move"
        );
    }
}

// ─── THE VERDICT IS A FUNCTION OF THE STATES ───────────────────────────

/// REGRESSION. The old gate returned OPPOSITE verdicts for the same
/// pair depending only on which operators the caller passed — with the
/// identity in the list a state "differed genuinely from itself", with
/// a shear it did not. The verdict must now depend on the states.
#[test]
fn the_operator_list_no_longer_decides_the_verdict() {
    let a = triangle(q(1, 1), q(1, 2), q(3, 2));
    let lists: [Vec<Operator>; 4] = [
        vec![],
        vec![identity_op(3)],
        vec![shear(3, 0, 1, q(1, 50))],
        vec![identity_op(3), frame(3, &[1, 2, 0], 9), shear(3, 0, 1, q(1, 50))],
    ];
    for (n, ops) in lists.iter().enumerate() {
        assert_eq!(
            f_fold_star(&a, &a, ops, None),
            FoldVerdict::TorsionVeto(TorsionCause::WitnessAbsent),
            "list {n}: the same pair must reach the same verdict"
        );
    }
}

/// REGRESSION. An empty operator list once permitted EVERY fold,
/// however different the states. It now proves nothing, and a proven
/// difference still vetoes.
#[test]
fn an_empty_operator_list_permits_nothing() {
    let coherent = triangle(q(1, 1), q(1, 2), q(3, 2));
    let incoherent = triangle(q(2, 1), q(3, 1), q(7, 1));
    assert_eq!(
        judged_fold(&coherent, &incoherent, &[], None),
        FoldVerdict::TorsionVeto(TorsionCause::SignaturesDiffer)
    );
    assert_eq!(
        judged_fold(&incoherent, &coherent, &[], None),
        FoldVerdict::TorsionVeto(TorsionCause::SignaturesDiffer)
    );
}

// ─── THE POLARITY ──────────────────────────────────────────────────────

/// REGRESSION — THE UNSOUND DIRECTION, closed. `coherent` composes (a
/// pure coboundary, reducing to zero); `incoherent` does not. Their
/// reduced signatures differ, which is exact proof the difference is
/// NOT gauge. The old gate read that as `GaugeFold` and permitted the
/// collapse, destroying the structure I6 exists to preserve.
#[test]
fn a_proven_genuine_difference_is_vetoed_never_collapsed() {
    let coherent = triangle(q(1, 1), q(1, 2), q(3, 2));
    let incoherent = triangle(q(2, 1), q(3, 1), q(7, 1));

    assert_ne!(
        rcf_signature(&coboundary_reduce(&coherent)),
        rcf_signature(&coboundary_reduce(&incoherent)),
        "different signatures = provably NOT related by any frame change"
    );

    let ops = vec![identity_op(3), frame(3, &[1, 2, 0], 9)];
    // No witness can rescue it: the exact proof short-circuits first.
    for witness in [
        None,
        Some(OrbitWitness { operator_id: 0, step: 1 }),
        Some(OrbitWitness { operator_id: 9, step: 1 }),
    ] {
        assert_eq!(
            judged_fold(&coherent, &incoherent, &ops, witness.as_ref()),
            FoldVerdict::TorsionVeto(TorsionCause::SignaturesDiffer),
            "genuine difference vetoes regardless of any presented witness"
        );
    }
}

/// REGRESSION — THE MIRROR ERROR, closed. Two states that reduce to the
/// same representative are the textbook GAUGE case. The old gate vetoed
/// them and routed them to the minting branch.
#[test]
fn a_gauge_pair_is_never_routed_to_the_minting_branch() {
    let one = triangle(q(2, 1), q(3, 1), q(7, 1));
    let scaled = triangle(q(20, 1), q(30, 1), q(70, 1));
    assert_eq!(coboundary_reduce(&one), coboundary_reduce(&scaled));

    let ops = vec![identity_op(3)];
    // Same class, so the verdict is about PROOF, not about structure —
    // and with the identity presented it folds rather than mints.
    let witness = OrbitWitness { operator_id: 0, step: 1 };
    assert_eq!(
        judged_fold(&one, &scaled, &ops, Some(&witness)),
        FoldVerdict::GaugeFold(witness)
    );
}

// ─── ACTIVE SUPPORT MASKING (DIRECTIVE 05) ─────────────────────────────

/// REGRESSION. A PATH of relations `0—1—2` with no `0—2` observation is
/// a tree: `E − V + C = 2 − 3 + 1 = 0`, so it carries no `H¹` at all.
/// The dense reduction read the absent `(0,2)` entry as a real zero and
/// manufactured a non-zero survivor at `(1,2)` — capacity 1 out of a
/// tree. Support masking reads it correctly.
#[test]
fn a_tree_of_relations_carries_no_capacity() {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![q(2, 1)]);
    d.set_antisym(1, 2, vec![q(3, 1)]);
    // (0,2) is NOT observed.

    let mut support = Support::none(3, 1);
    support.mark(0, 1, 0);
    support.mark(1, 2, 0);

    let reduced = coboundary_reduce_over(&d, &support);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(
                *reduced.entry(i, j, 0),
                ratio(0, 1),
                "({i},{j}): a spanning tree is pure frame — nothing may survive"
            );
        }
    }
}

/// The complement: close the tree into a cycle that does NOT compose,
/// and exactly one independent entry survives — the cycle rank.
#[test]
fn a_closed_cycle_that_does_not_compose_survives_reduction() {
    let d = triangle(q(2, 1), q(3, 1), q(9, 1));
    let mut support = Support::none(3, 1);
    support.mark(0, 1, 0);
    support.mark(1, 2, 0);
    support.mark(0, 2, 0);

    let reduced = coboundary_reduce_over(&d, &support);
    let survivors = (0..3)
        .flat_map(|i| ((i + 1)..3).map(move |j| (i, j)))
        .filter(|(i, j)| *reduced.entry(*i, *j, 0) != ratio(0, 1))
        .count();
    assert_eq!(survivors, 1, "E − V + C = 3 − 3 + 1 = 1 independent cycle");
}

/// An observed relation whose VALUE is zero is still structure, and an
/// explicit support is the only way to say so. This is why the kernel
/// bridge builds the mask from the obligation lists rather than letting
/// `Support::of` infer it from non-zero entries.
#[test]
fn an_observed_zero_is_not_an_absent_relation() {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![q(2, 1)]);
    d.set_antisym(1, 2, vec![q(3, 1)]);
    d.set_antisym(0, 2, vec![q(0, 1)]); // observed, and it nets to zero

    // Inferred support misses the zero-valued observation: it sees a
    // tree, and reduces to nothing.
    assert_eq!(coboundary_reduce(&d), Delta::zero(3, 1));

    // The explicit mask keeps it, and the cycle's defect survives.
    let mut support = Support::none(3, 1);
    support.mark(0, 1, 0);
    support.mark(1, 2, 0);
    support.mark(0, 2, 0);
    assert_ne!(
        coboundary_reduce_over(&d, &support),
        Delta::zero(3, 1),
        "0 = 2 + 3 is false: the observed cycle does not compose"
    );
}
