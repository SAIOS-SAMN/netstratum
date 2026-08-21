//! THE ATLAS — the two manifold laws, proven on canonical fixtures (doc 55).
//!
//! §5 PROVABLE COGNITIVE INCREASE: ΔH is the connecting-homomorphism rank — the
//! classes the glued atlas mints that no chart carries. Two contractible arcs
//! glued on a TWO-point overlap form a circle and mint exactly one such class
//! (the loop) → ΔH = 1; glued on a ONE-point overlap they stay an interval →
//! ΔH = 0. §4 SOUNDNESS: the cocycle condition on transitions.

use num_bigint::BigInt;
use reduct::atlas::{atlas_betti, cocycle_holds, curvature, delta_h, holonomy, transports_flatly};
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
fn the_mayer_vietoris_spectrum_matches_ground_truth() {
    // Build the circle DIRECTLY as one complex: one 0-cell + one 1-cell, no
    // incidence (a loop's head/tail cancel), so H⁰ = ℤ, H¹ = ℤ → betti [1,1].
    let circle_direct = CellSheaf::new(vec![vec![1], vec![1]], vec![]).expect("S¹ is a complex");
    let ground_truth: Vec<usize> = (0..circle_direct.grades())
        .map(|k| circle_direct.cohomology(k).betti)
        .collect();
    assert_eq!(ground_truth, vec![1, 1], "the direct circle is H⁰=H¹=ℤ");

    // The atlas assembled by Mayer–Vietoris from two arcs glued on the two-point
    // overlap must reproduce that exact spectrum — the ground-truth soundness
    // witness (S4: its alternating sum is χ(U)+χ(V)−χ(W)).
    let u = point();
    let v = point();
    let w = two_points();
    let r_u = vec![vec![vec![i(1)], vec![i(1)]]];
    let r_v = vec![vec![vec![i(1)], vec![i(1)]]];
    let mv = atlas_betti(&u, &v, &w, &r_u, &r_v);
    assert_eq!(mv, vec![1, 1], "MV reconstructs the circle's cohomology exactly");

    // Euler additivity (S4): χ(X) = χ(U)+χ(V)−χ(W).
    let chi = |b: &[usize]| -> i64 {
        b.iter().enumerate().map(|(k, &x)| if k % 2 == 0 { x as i64 } else { -(x as i64) }).sum()
    };
    let chi_x = chi(&mv);
    let chi_u = u.euler_characteristic();
    let chi_v = v.euler_characteristic();
    let chi_w = w.euler_characteristic();
    assert_eq!(chi_x, chi_u + chi_v - chi_w, "Euler additivity of the cover holds");
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

#[test]
fn the_connection_holonomy_measures_curvature() {
    let shear = vec![vec![i(1), i(1)], vec![i(0), i(1)]];
    let unshear = vec![vec![i(1), i(-1)], vec![i(0), i(1)]];
    let id2 = vec![vec![i(1), i(0)], vec![i(0), i(1)]];

    // A consistent loop (g · g⁻¹ · I = I) is FLAT — a class transports intact
    // across the charts (fluid intelligence: knowledge propagates geometrically).
    assert!(
        transports_flatly(&[shear.clone(), unshear, id2.clone()], 2),
        "a flat loop transports the class whole"
    );

    // A curved loop (g · g · I = g² ≠ I) has non-trivial HOLONOMY — the class hits
    // an obstruction it cannot transport past (the reduce organ's torsion, the
    // 'unglueable structural cohort').
    let curved = holonomy(&[shear.clone(), shear.clone(), id2.clone()]);
    assert_eq!(curved, vec![vec![i(1), i(2)], vec![i(0), i(1)]], "holonomy = g²");
    assert!(
        !transports_flatly(&[shear.clone(), shear.clone(), id2.clone()], 2),
        "a curved loop obstructs transport — the class does not survive convergence"
    );
    // The curvature is exactly holonomy − I (the measurable obstruction).
    assert_eq!(
        curvature(&[shear.clone(), shear, id2], 2),
        vec![vec![i(0), i(2)], vec![i(0), i(0)]],
        "curvature = holonomy − I is the obstruction, non-zero iff not flat"
    );
}
