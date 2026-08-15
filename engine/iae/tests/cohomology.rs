//! IAE-2 step 3 witnesses — the stratified peel (cohomology by exact SNF).
//!
//! Betti numbers on known complexes (a filled triangle is contractible; a
//! hollow one is a circle), exact ℤ/2 torsion, the Euler-characteristic
//! conservation cross-check `Σ(−1)ᵏ dim Cᵏ = Σ(−1)ᵏ betti_k` (homological
//! exactness), and the peel itself: a coboundary folds to zero, a torsion
//! cochain folds modulo its invariant factor — never truncated.

use iae::sheaf::{CellSheaf, Incidence};
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

fn tri_incidences() -> Vec<Incidence> {
    let inc = |fg, fi, cf, s| Incidence {
        face_grade: fg,
        face_idx: fi,
        coface_idx: cf,
        sign: b(s),
        restriction: vec![vec![b(1)]],
    };
    vec![
        inc(0, 0, 0, -1),
        inc(0, 1, 0, 1),
        inc(0, 0, 1, -1),
        inc(0, 2, 1, 1),
        inc(0, 1, 2, -1),
        inc(0, 2, 2, 1),
    ]
}

/// Euler-characteristic conservation: Σ(−1)ᵏ dim Cᵏ must equal Σ(−1)ᵏ betti_k.
fn assert_euler(x: &CellSheaf) {
    let chi_dim = x.euler_characteristic();
    let chi_betti: i64 = (0..x.grades())
        .map(|k| {
            let bt = x.cohomology(k).betti as i64;
            if k % 2 == 0 {
                bt
            } else {
                -bt
            }
        })
        .sum();
    assert_eq!(chi_dim, chi_betti, "Euler characteristic conserved (dim vs betti)");
}

#[test]
fn a_filled_triangle_is_contractible() {
    let mut inc = tri_incidences();
    inc.push(Incidence {
        face_grade: 1,
        face_idx: 0,
        coface_idx: 0,
        sign: b(1),
        restriction: vec![vec![b(1)]],
    });
    inc.push(Incidence { face_grade: 1, face_idx: 1, coface_idx: 0, sign: b(-1), restriction: vec![vec![b(1)]] });
    inc.push(Incidence { face_grade: 1, face_idx: 2, coface_idx: 0, sign: b(1), restriction: vec![vec![b(1)]] });
    let x = CellSheaf::new(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1]], inc).expect("valid");

    assert_eq!(x.cohomology(0).betti, 1, "H⁰ = ℤ (connected)");
    assert_eq!(x.cohomology(1).betti, 0, "H¹ = 0 (no holes)");
    assert_eq!(x.cohomology(2).betti, 0, "H² = 0");
    assert!(x.cohomology(1).torsion.is_empty());
    assert_euler(&x);
}

#[test]
fn a_hollow_triangle_is_a_circle() {
    // No filling face: H⁰ = ℤ, H¹ = ℤ (one loop).
    let x = CellSheaf::new(vec![vec![1, 1, 1], vec![1, 1, 1]], tri_incidences()).expect("valid");
    assert_eq!(x.cohomology(0).betti, 1);
    assert_eq!(x.cohomology(1).betti, 1, "the loop is the nontrivial H¹");
    assert!(x.cohomology(1).torsion.is_empty());
    assert_euler(&x); // χ = 3 − 3 = 0 = 1 − 1
}

#[test]
fn a_degree_two_map_gives_exact_z_mod_2_torsion() {
    // C⁰ = ℤ →(×2) C¹ = ℤ. H⁰ = 0 (×2 injective), H¹ = ℤ/2.
    let x = CellSheaf::new(
        vec![vec![1], vec![1]],
        vec![Incidence {
            face_grade: 0,
            face_idx: 0,
            coface_idx: 0,
            sign: b(1),
            restriction: vec![vec![b(2)]],
        }],
    )
    .expect("valid");

    assert_eq!(x.cohomology(0).betti, 0);
    let h1 = x.cohomology(1);
    assert_eq!(h1.betti, 0);
    assert_eq!(h1.torsion, vec![b(2)], "exact ℤ/2 torsion — not a truncated zero");
    assert_euler(&x); // χ = 1 − 1 = 0 = betti 0 − 0
}

#[test]
fn the_peel_folds_a_coboundary_to_zero() {
    // On the filled triangle every 1-cocycle is a coboundary (H¹ = 0), so the
    // peel must fold any coboundary to exactly zero.
    let mut inc = tri_incidences();
    inc.push(Incidence { face_grade: 1, face_idx: 0, coface_idx: 0, sign: b(1), restriction: vec![vec![b(1)]] });
    inc.push(Incidence { face_grade: 1, face_idx: 1, coface_idx: 0, sign: b(-1), restriction: vec![vec![b(1)]] });
    inc.push(Incidence { face_grade: 1, face_idx: 2, coface_idx: 0, sign: b(1), restriction: vec![vec![b(1)]] });
    let x = CellSheaf::new(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1]], inc).expect("valid");

    // A coboundary: δ⁰ applied to the 0-cochain e_v0 = (1,0,0) → column 0 of δ⁰.
    let coboundary = iae_matvec(&x.coboundary(0), &[b(1), b(0), b(0)]);
    assert!(x.is_cocycle(1, &coboundary), "a coboundary is a cocycle");
    let reduced = x.reduce_cochain(1, &coboundary);
    assert!(reduced.iter().all(|v| *v == b(0)), "coboundary peels to 0, got {reduced:?}");
}

#[test]
fn the_peel_folds_torsion_modulo_its_invariant_factor() {
    // C⁰ = ℤ →(×2) C¹ = ℤ. Im(δ⁰) = 2ℤ, so reduce mod 2 — exactly.
    let x = CellSheaf::new(
        vec![vec![1], vec![1]],
        vec![Incidence {
            face_grade: 0,
            face_idx: 0,
            coface_idx: 0,
            sign: b(1),
            restriction: vec![vec![b(2)]],
        }],
    )
    .expect("valid");

    assert_eq!(x.reduce_cochain(1, &[b(3)]), vec![b(1)], "3 ≡ 1 (mod 2)");
    assert_eq!(x.reduce_cochain(1, &[b(4)]), vec![b(0)], "4 ≡ 0 (mod 2)");
    assert_eq!(x.reduce_cochain(1, &[b(2)]), vec![b(0)], "2 ∈ Im — folds to 0");
    // Idempotent, and the reduction differs from the input by a coboundary.
    let once = x.reduce_cochain(1, &[b(7)]);
    assert_eq!(x.reduce_cochain(1, &once), once, "peel is idempotent");
    assert_eq!(once, vec![b(1)]);
    assert!((&b(7) - &once[0]) % b(2) == b(0), "input − reduced ∈ Im(δ⁰)");
}

// Local matvec so the test does not depend on the crate-private `mat` module.
fn iae_matvec(a: &[Vec<BigInt>], x: &[BigInt]) -> Vec<BigInt> {
    a.iter()
        .map(|row| row.iter().zip(x).map(|(w, v)| w * v).sum())
        .collect()
}
