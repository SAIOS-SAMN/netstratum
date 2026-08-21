//! THE ATLAS — the two manifold laws, proven on canonical fixtures (doc 55).
//!
//! §5 PROVABLE COGNITIVE INCREASE: ΔH is the connecting-homomorphism rank — the
//! classes the glued atlas mints that no chart carries. Two contractible arcs
//! glued on a TWO-point overlap form a circle and mint exactly one such class
//! (the loop) → ΔH = 1; glued on a ONE-point overlap they stay an interval →
//! ΔH = 0. §4 SOUNDNESS: the cocycle condition on transitions.

use num_bigint::BigInt;
use reduct::atlas::{cocycle_holds, delta_h};
use reduct::sheaf::CellSheaf;

fn i(n: i64) -> BigInt {
    BigInt::from(n)
}

/// A single contractible point: one 0-cell, no incidences. H⁰ = ℤ, H¹ = 0.
fn point() -> CellSheaf {
    CellSheaf::new(vec![vec![1]], vec![]).expect("a point is a complex")
}

/// Two disjoint points: two 0-cells. H⁰ = ℤ².
fn two_points() -> CellSheaf {
    CellSheaf::new(vec![vec![1, 1]], vec![]).expect("two points form a complex")
}

#[test]
fn the_circle_mints_one_shared_class() {
    // U, V contractible arcs (a point each); overlap W = the two glued ends.
    let u = point();
    let v = point();
    let w = two_points();
    // Each arc's point restricts to BOTH ends of the overlap.
    let r_u = vec![vec![vec![i(1)], vec![i(1)]]]; // 2×1
    let r_v = vec![vec![vec![i(1)], vec![i(1)]]];
    let dh = delta_h(&u, &v, &w, &r_u, &r_v);
    assert_eq!(dh, 1, "two arcs glued into a circle mint exactly the loop class");
}

#[test]
fn a_single_point_glue_mints_nothing() {
    // Two arcs joined at ONE point stay an interval — contractible, no new class.
    let u = point();
    let v = point();
    let w = point();
    let r_u = vec![vec![vec![i(1)]]]; // 1×1
    let r_v = vec![vec![vec![i(1)]]];
    let dh = delta_h(&u, &v, &w, &r_u, &r_v);
    assert_eq!(dh, 0, "a contractible glue creates no shared class");
}

#[test]
fn the_cocycle_condition_gates_the_bundle() {
    // Consistent transitions on a triple overlap: g_ij·g_jk = g_ik.
    let g_ij = vec![vec![i(1), i(1)], vec![i(0), i(1)]];
    let g_jk = vec![vec![i(1), i(0)], vec![i(2), i(1)]];
    // g_ij·g_jk = [[3,1],[2,1]]
    let g_ik = vec![vec![i(3), i(1)], vec![i(2), i(1)]];
    assert!(cocycle_holds(&g_ij, &g_jk, &g_ik), "a genuine bundle composes");

    // A tampered transition is NOT a bundle — the atlas must refuse it.
    let tampered = vec![vec![i(3), i(1)], vec![i(2), i(9)]];
    assert!(
        !cocycle_holds(&g_ij, &g_jk, &tampered),
        "a broken cocycle is not a bundle and must fail soundness"
    );
}
