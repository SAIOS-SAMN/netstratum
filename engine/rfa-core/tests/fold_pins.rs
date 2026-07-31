//! THE FOLD GATE witnesses (U2b, I6, PoWC) — translated from the v0.1
//! ancestor `fold.rs` tests and re-witnessed red-first. The law
//! (SYSTEM_SPEC §4, kernel CHARTER §6, EQUATIONS I6): a fold is lawful
//! IFF `orbit_lock = ∅`. These pins fix the DIRECTION of that law —
//! that a shear (no lock) permits the fold while the identity (a
//! period-1 lock) vetoes it — so an inverted verdict cannot pass.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{f_fold_star, judged_fold, orbit_lock, FoldVerdict, Operator, Delta};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// The anchor: a 3-entity Delta in exact equilibrium (Δ01 = 1,
/// Δ12 = 1/2, Δ02 = 3/2) — the ancestor's `anchor_3x3`.
fn anchor_3() -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![q(1, 1)]);
    d.set_antisym(1, 2, vec![q(1, 2)]);
    d.set_antisym(0, 2, vec![q(3, 2)]);
    d
}

/// A 2-entity anchor (Δ01 = 1) — the ancestor's `anchor_2x2`.
fn anchor_2() -> Delta {
    let mut d = Delta::zero(2, 1);
    d.set_antisym(0, 1, vec![q(1, 1)]);
    d
}

/// A shear operator `I + s·E_{row,col}` and its exact inverse
/// `I − s·E_{row,col}` (unipotent — det 1). The ancestor's `shear_op`.
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

#[test]
fn a_shear_creates_no_orbit_lock() {
    // A shear is unipotent — its conjugation does not carry the anchor's
    // class back onto itself, so no torsion cycle forms (ancestor
    // test_shear_no_orbit_lock_2x2 / _3x3).
    let anchor2 = anchor_2();
    assert_eq!(
        orbit_lock(&anchor2, &anchor2, &[shear(2, 0, 1, q(1, 10))], 10),
        None,
        "a shear must not self-lock the 2×2 anchor"
    );
    let anchor3 = anchor_3();
    assert_eq!(
        orbit_lock(&anchor3, &anchor3, &[shear(3, 0, 1, q(1, 50))], 10),
        None,
        "a shear must not self-lock the 3×3 anchor"
    );
}

#[test]
fn the_shear_fold_is_lawful_gauge() {
    // No orbit lock among the shear operators → the fold is lawful
    // (the difference was gauge). The ancestor's f_fold_star returned
    // identity here (fold permitted); ours reads GaugeFold.
    let anchor = anchor_3();
    let ops = vec![
        shear(3, 0, 1, q(1, 50)),
        shear(3, 1, 2, q(1, 75)),
        shear(3, 0, 2, q(1, 100)),
    ];
    // Fold the anchor against a shear image of itself: still gauge.
    let image = ops[0].apply(&anchor).expect("shear applies");
    assert_eq!(
        f_fold_star(&anchor, &image, &ops, 10),
        FoldVerdict::GaugeFold,
        "a shear difference is gauge — the fold is lawful"
    );
}

#[test]
fn the_identity_orbit_locks_and_vetoes_the_fold() {
    // The identity operator: A·Δ·A⁻¹ = Δ, so the image shares the
    // anchor's RCF class at step 1 — a period-1 torsion cycle. Orbit
    // lock EXISTS, so the fold is VETOED (ancestor
    // test_identity_operator_self_orbit_lock).
    let anchor = anchor_2();
    let id = identity_op(2);
    let witness = orbit_lock(&anchor, &anchor, std::slice::from_ref(&id), 10)
        .expect("the identity creates a period-1 lock");
    assert_eq!(witness.operator_id, 0);
    assert_eq!(witness.step, 1, "the lock is found at the first conjugation");
    assert_eq!(
        f_fold_star(&anchor, &anchor, std::slice::from_ref(&id), 10),
        FoldVerdict::TorsionVeto(witness),
        "a torsion lock forbids the fold — the residue must emit"
    );
}

/// A permutation frame: relabel the entities by `perm` (a conjugation
/// P·Δ·P⁻¹ with P the permutation matrix — its inverse is its
/// transpose). The discrete frame group of a Delta over n entities.
fn frame(dim: usize, perm: &[usize]) -> Operator {
    let mut matrix = vec![vec![q(0, 1); dim]; dim];
    let mut matrix_inv = vec![vec![q(0, 1); dim]; dim];
    for (from, &to) in perm.iter().enumerate() {
        matrix[to][from] = q(1, 1);
        matrix_inv[from][to] = q(1, 1);
    }
    Operator { id: 9, matrix, matrix_inv, det_sign_positive: true }
}

/// Add a coboundary (a pure gauge distortion: Δ_ij += p_j − p_i) to a
/// Delta — the "angle" a relabeling of potentials introduces.
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

#[test]
fn the_angle_free_witness() {
    // THE SINGULARITY LAW (16_UNIT §4a) as a test: the composed gate's
    // verdict is INVARIANT to the frame and gauge of approach. The
    // same pair, presented under every permutation frame of its
    // entities AND shifted by a gauge coboundary, yields the identical
    // verdict — fold or veto — because Π_G strips what a relabeling
    // could remove and the RCF signature is conjugation-invariant.
    let a = anchor_3();
    let mut b = Delta::zero(3, 1);
    b.set_antisym(0, 1, vec![q(2, 1)]);
    b.set_antisym(1, 2, vec![q(3, 1)]);
    b.set_antisym(0, 2, vec![q(7, 1)]); // incoherent — genuinely different
    let ops = vec![shear(3, 0, 1, q(1, 50))];
    let base = judged_fold(&a, &b, &ops, 10);

    // Every 3-entity permutation frame: the verdict holds.
    let perms: [[usize; 3]; 6] =
        [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    for perm in perms {
        let f = frame(3, &perm);
        let fa = f.apply(&a).expect("the frame applies");
        let fb = f.apply(&b).expect("the frame applies");
        assert_eq!(
            judged_fold(&fa, &fb, &ops, 10),
            base,
            "the verdict cannot depend on the frame of approach: {perm:?}"
        );
    }

    // A gauge coboundary on either input: stripped by Π_G, the
    // verdict holds.
    let potentials = [q(0, 1), q(5, 1), q(1, 3)];
    let shifted_a = gauge_shifted(&a, &potentials);
    let shifted_b = gauge_shifted(&b, &potentials);
    assert_eq!(judged_fold(&shifted_a, &b, &ops, 10), base);
    assert_eq!(judged_fold(&a, &shifted_b, &ops, 10), base);
    assert_eq!(judged_fold(&shifted_a, &shifted_b, &ops, 10), base);

    // And the self-pair under the identity lock: the verdict is a
    // veto in EVERY frame alike (no angle un-vetoes a torsion).
    let id = identity_op(3);
    let self_base = judged_fold(&a, &a, std::slice::from_ref(&id), 10);
    assert!(matches!(self_base, FoldVerdict::TorsionVeto(_)));
    for perm in perms {
        let f = frame(3, &perm);
        let fa = f.apply(&a).expect("applies");
        assert_eq!(
            judged_fold(&fa, &fa, std::slice::from_ref(&id), 10),
            self_base,
            "the veto is frame-invariant too"
        );
    }
}

#[test]
fn a_broken_operator_generates_no_orbit() {
    // An operator with det ≤ 0 is not in the group: it generates no
    // orbit, so it can never lock — the gate stays total, never panics.
    let anchor = anchor_2();
    let mut bad = identity_op(2);
    bad.det_sign_positive = false;
    assert!(bad.apply(&anchor).is_none(), "a non-group operator does not apply");
    assert_eq!(
        orbit_lock(&anchor, &anchor, &[bad], 10),
        None,
        "no valid orbit → no lock"
    );
}
