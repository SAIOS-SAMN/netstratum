//! IAE-2 witnesses — the cellular sheaf and the commutation-gated transduction.
//!
//! The filled 2-simplex proves the coboundary is assembled with faithful
//! incidence signs (δ∘δ = 0); the heterogeneous interval proves stalks of
//! different native dimension compose without padding; a broken restriction
//! is refused typed with the non-zero remainder carried; and the transduction
//! gate admits a commuting chain map while refusing a non-commuting one with
//! its obstruction whole.

use reduct::sheaf::{CellSheaf, Incidence, SheafError};
use reduct::transduction::{transduce, TransductionError};
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

/// A 1×1 restriction carrying the integer `s` — a constant-ℤ stalk morphism.
fn scalar(s: i64) -> Vec<Vec<BigInt>> {
    vec![vec![b(s)]]
}

/// The filled triangle {0,1,2}: 3 vertices, 3 edges (01,02,12), 1 face.
/// Constant ℤ stalks (dim 1). Incidence signs are the simplicial ∂:
/// ∂[a,b] = b − a, ∂[0,1,2] = e12 − e02 + e01.
fn filled_triangle() -> CellSheaf {
    let stalk_dims = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1]];
    let inc = |fg, fi, cf, s| Incidence {
        face_grade: fg,
        face_idx: fi,
        coface_idx: cf,
        sign: b(s),
        restriction: scalar(1),
    };
    let incidences = vec![
        // edges from vertices: e01 = [0,1], e02 = [0,2], e12 = [1,2]
        inc(0, 0, 0, -1),
        inc(0, 1, 0, 1),
        inc(0, 0, 1, -1),
        inc(0, 2, 1, 1),
        inc(0, 1, 2, -1),
        inc(0, 2, 2, 1),
        // face from edges: +e01 − e02 + e12
        inc(1, 0, 0, 1),
        inc(1, 1, 0, -1),
        inc(1, 2, 0, 1),
    ];
    CellSheaf::new(stalk_dims, incidences).expect("the filled triangle is a valid complex")
}

#[test]
fn the_filled_triangle_is_a_complex_delta_squared_is_zero() {
    let x = filled_triangle();
    // Construction already gated on δ∘δ = 0; assert the coboundary shapes too.
    assert_eq!(x.dim_cochain(0), 3);
    assert_eq!(x.dim_cochain(1), 3);
    assert_eq!(x.dim_cochain(2), 1);
    // δ⁰ is 3×3, δ¹ is 1×3, and δ¹·δ⁰ = 0 (verified inside new()).
    assert!(x.verify_complex().is_ok());
}

#[test]
fn heterogeneous_stalks_compose_without_padding() {
    // An interval v0 —e— v1 with DIFFERENT native stalk dimensions:
    // F(v0) = ℤ², F(v1) = ℤ¹, F(e) = ℤ². The restrictions keep each cell's
    // own grading; nothing is zero-padded into a shared m.
    let stalk_dims = vec![vec![2, 1], vec![2]];
    let incidences = vec![
        Incidence {
            face_grade: 0,
            face_idx: 0,
            coface_idx: 0,
            sign: b(-1),
            restriction: vec![vec![b(1), b(0)], vec![b(0), b(1)]], // 2×2
        },
        Incidence {
            face_grade: 0,
            face_idx: 1,
            coface_idx: 0,
            sign: b(1),
            restriction: vec![vec![b(1)], vec![b(2)]], // 2×1
        },
    ];
    let x = CellSheaf::new(stalk_dims, incidences).expect("valid 1-complex");
    // C⁰ = 2 + 1 = 3, C¹ = 2. δ⁰ is 2×3.
    assert_eq!(x.dim_cochain(0), 3);
    assert_eq!(x.dim_cochain(1), 2);
    let d0 = x.coboundary(0);
    assert_eq!(d0.len(), 2);
    assert_eq!(d0[0].len(), 3);
    // Column block for v0 is −I₂; the block for v1 is (1,2)ᵀ.
    assert_eq!(d0[0], vec![b(-1), b(0), b(1)]);
    assert_eq!(d0[1], vec![b(0), b(-1), b(2)]);
}

#[test]
fn a_wrong_shaped_restriction_is_refused_typed() {
    // Declare F(e) = ℤ² but hand a 1×1 restriction: shape mismatch.
    let stalk_dims = vec![vec![1], vec![2]];
    let bad = vec![Incidence {
        face_grade: 0,
        face_idx: 0,
        coface_idx: 0,
        sign: b(1),
        restriction: scalar(1), // 1×1, but must be 2×1
    }];
    match CellSheaf::new(stalk_dims, bad) {
        Err(SheafError::RestrictionShape { expected, got, .. }) => {
            assert_eq!(expected, (2, 1));
            assert_eq!(got, (1, 1));
        }
        other => panic!("expected RestrictionShape refusal, got {other:?}"),
    }
}

#[test]
fn a_non_complex_is_refused_with_the_remainder_carried() {
    // Same skeleton as the triangle but with the face's e02 sign FLIPPED to
    // +1, so ∂∂ ≠ 0 — the restrictions no longer compose to a complex.
    let stalk_dims = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1]];
    let inc = |fg, fi, cf, s| Incidence {
        face_grade: fg,
        face_idx: fi,
        coface_idx: cf,
        sign: b(s),
        restriction: scalar(1),
    };
    let incidences = vec![
        inc(0, 0, 0, -1),
        inc(0, 1, 0, 1),
        inc(0, 0, 1, -1),
        inc(0, 2, 1, 1),
        inc(0, 1, 2, -1),
        inc(0, 2, 2, 1),
        inc(1, 0, 0, 1),
        inc(1, 1, 0, 1), // WRONG: should be −1
        inc(1, 2, 0, 1),
    ];
    match CellSheaf::new(stalk_dims, incidences) {
        Err(SheafError::NotAComplex { grade, remainder }) => {
            assert_eq!(grade, 0);
            assert!(remainder.iter().any(|r| r.iter().any(|x| *x != b(0))));
        }
        other => panic!("expected NotAComplex refusal, got {other:?}"),
    }
}

#[test]
fn the_identity_transduction_commutes() {
    let x = filled_triangle();
    let ident = |n: usize| -> Vec<Vec<BigInt>> {
        (0..n).map(|i| (0..n).map(|j| b((i == j) as i64)).collect()).collect()
    };
    let maps = vec![ident(3), ident(3), ident(1)];
    let f = transduce(&x, &x, maps).expect("identity is a chain map");
    // The adjoint is carried: fᵀ of each grade.
    assert_eq!(f.adjoint.len(), 3);
    assert_eq!(f.maps.len(), 3);
}

#[test]
fn a_uniform_scaling_transduction_commutes() {
    // f_k = 3·I at every grade commutes with δ (scalar commutes with linear).
    let x = filled_triangle();
    let scale = |n: usize, s: i64| -> Vec<Vec<BigInt>> {
        (0..n).map(|i| (0..n).map(|j| b(if i == j { s } else { 0 })).collect()).collect()
    };
    let maps = vec![scale(3, 3), scale(3, 3), scale(1, 3)];
    assert!(transduce(&x, &x, maps).is_ok(), "uniform scaling is a chain map");
}

#[test]
fn a_non_commuting_map_is_refused_with_its_obstruction() {
    // f_0 = I₃ but f_1 = 0: then f_1·δ⁰ − δ⁰·f_0 = −δ⁰ ≠ 0 at grade 0.
    let x = filled_triangle();
    let ident3: Vec<Vec<BigInt>> =
        (0..3).map(|i| (0..3).map(|j| b((i == j) as i64)).collect()).collect();
    let zero3: Vec<Vec<BigInt>> = vec![vec![b(0); 3]; 3];
    let ident1 = vec![vec![b(1)]];
    let maps = vec![ident3, zero3, ident1];
    match transduce(&x, &x, maps) {
        Err(TransductionError::Defect { grade, non_commuting_remainder }) => {
            assert_eq!(grade, 0);
            assert!(non_commuting_remainder.iter().any(|r| r.iter().any(|x| *x != b(0))));
        }
        other => panic!("expected a commutation Defect, got {other:?}"),
    }
}
