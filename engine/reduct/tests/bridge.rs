//! IAE-3 witnesses — the grid → obstruction bridge.
//!
//! Color entities are extracted exactly and deterministically; the relational
//! 2-skeleton is a valid complex; a consistent grid measures to a coboundary
//! whose obstruction vanishes (δ¹∘δ⁰ = 0 — the transduction property); and a
//! directly-measured non-closing triple carries a non-zero torsion obstruction
//! whole, which the peel folds to its harmonic class.

use reduct::bridge::{color_entities, entity_potential, measure, obstruction, relational_complex, GridObs};
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

#[test]
fn color_entities_are_exact_and_deterministic() {
    // A 2×2 grid: colors 1 and 2, background 0.
    //   1 2
    //   2 0
    let g = GridObs::new(2, 2, vec![1, 2, 2, 0]);
    let e = color_entities(&g, 0);
    // Two entities, sorted by color: color 1 (count 1 at (0,0)), color 2
    // (count 2 at (0,1) and (1,0)).
    assert_eq!(e.len(), 2);
    assert_eq!(e[0], vec![b(1), b(1), b(0), b(0)], "color 1: (value,count,Σrow,Σcol)");
    assert_eq!(e[1], vec![b(2), b(2), b(1), b(1)], "color 2: Σrow=0+1, Σcol=1+0");
    // Re-running is byte-identical (no hashing nondeterminism).
    assert_eq!(color_entities(&g, 0), e);
}

#[test]
fn the_relational_complex_is_a_valid_2_skeleton() {
    // n = 3, m = 1 is the filled triangle: constructing it already gated δ∘δ=0.
    let d = relational_complex(3, 1);
    assert_eq!(d.dim_cochain(0), 3); // 3 vertices
    assert_eq!(d.dim_cochain(1), 3); // 3 edges
    assert_eq!(d.dim_cochain(2), 1); // 1 triangle
    assert!(d.verify_complex().is_ok());
    // With coordinate module ℤ² the dims scale by m.
    let d2 = relational_complex(4, 2);
    assert_eq!(d2.dim_cochain(0), 4 * 2); // 4 vertices × ℤ²
    assert_eq!(d2.dim_cochain(1), 6 * 2); // C(4,2)=6 edges × ℤ²
    assert_eq!(d2.dim_cochain(2), 4 * 2); // C(4,3)=4 triangles × ℤ²
}

#[test]
fn a_consistent_grid_has_zero_obstruction() {
    // Any entity potential measured through δ⁰ is a coboundary, so its triple
    // residual δ¹Δ = δ¹δ⁰f = 0. This is the transduction property: carrying
    // the potential through the coboundary loses nothing to close.
    let g = GridObs::new(2, 3, vec![1, 2, 3, 3, 2, 1]);
    let entities = color_entities(&g, 0); // 3 colors → 3 entities, m = 4
    let n = entities.len();
    let m = entities[0].len();
    let complex = relational_complex(n, m);
    let field = measure(&complex, &entities);
    let obs = obstruction(&complex, &field);
    assert!(obs.iter().all(|x| *x == b(0)), "δ¹δ⁰f = 0, got {obs:?}");
}

#[test]
fn a_non_closing_triple_carries_a_torsion_obstruction() {
    // A field measured DIRECTLY (not as δ⁰ of a potential): three pairwise
    // relationships that do not compose. Edges in lex order are (0,1),(0,2),
    // (1,2); on the triangle δ¹c = +c(0,1) − c(0,2) + c(1,2).
    let complex = relational_complex(3, 1);
    // c(0,1)=1, c(0,2)=0, c(1,2)=1 → residual = 1 − 0 + 1 = 2 (A→B→C→A ≠ 0).
    let c = vec![b(1), b(0), b(1)];
    let obs = obstruction(&complex, &c);
    assert_eq!(obs, vec![b(2)], "the triple fails to close — the torsion signal");

    // On the full 2-skeleton H¹ = 0, so a genuine cocycle folds to nothing;
    // this non-cocycle's obstruction is the raw residual above. A coboundary,
    // by contrast, peels to zero.
    let coboundary = measure(&complex, &[vec![b(0)], vec![b(4)], vec![b(9)]]);
    assert!(obstruction(&complex, &coboundary).iter().all(|x| *x == b(0)));
    assert!(complex.reduce_cochain(1, &coboundary).iter().all(|x| *x == b(0)), "a coboundary peels to 0");
}

#[test]
fn the_potential_layout_matches_the_measurement() {
    // entity_potential flattens in entity order; measure = δ⁰·potential.
    let entities = vec![vec![b(2), b(5)], vec![b(7), b(1)]]; // n=2, m=2
    assert_eq!(entity_potential(&entities), vec![b(2), b(5), b(7), b(1)]);
    let complex = relational_complex(2, 2);
    // One edge (0,1); Δ = f_1 − f_0 = (7−2, 1−5) = (5, −4).
    assert_eq!(measure(&complex, &entities), vec![b(5), b(-4)]);
}
