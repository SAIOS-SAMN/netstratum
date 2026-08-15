//! IAE-4 witnesses — the resolve-solve loop, based on locality.
//!
//! The locality measurement carries the adjacency coordinate whose non-closure
//! IS the obstruction (the difference coordinates are coboundaries and close);
//! the resolve reduces a field modulo the coboundaries and the vocabulary,
//! leaving the irreducible residual; and adding the explaining local
//! translation to the vocabulary drives the residual to zero (solved).

use iae::bridge::{obstruction, GridObs};
use iae::resolve::{measure_relational, reduce_modulo, resolve};
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

#[test]
fn reduce_modulo_is_the_exact_quotient() {
    // Reduce ℤ modulo the sublattice spanned by (2): 5 ≡ 1, 4 ≡ 0.
    let g = vec![vec![b(2)]]; // 1×1 generator column (2)
    assert_eq!(reduce_modulo(&[b(5)], &g), vec![b(1)]);
    assert_eq!(reduce_modulo(&[b(4)], &g), vec![b(0)]);
    // Empty generator set leaves the vector whole.
    let empty: Vec<Vec<BigInt>> = Vec::new();
    assert_eq!(reduce_modulo(&[b(7)], &empty), vec![b(7)]);
}

#[test]
fn the_adjacency_coordinate_carries_the_obstruction() {
    // A row of three cells [1, 2, 4]. Adjacencies: 0-1 and 1-2 (Manhattan 1),
    // but 0-2 is distance 2 (not adjacent). On the triangle (0,1,2):
    //   coord 0 (value): 2 − 3 + 1 = 0        (coboundary — closes)
    //   coord 2 (Δcol) : 1 − 2 + 1 = 0        (coboundary — closes)
    //   coord 3 (adj)  : 2 − 0 + 1 = 3 ≠ 0    (LOCAL — does not close)
    let g = GridObs::new(1, 3, vec![1, 2, 4]);
    let (complex, field) = measure_relational(&g);
    let obs = obstruction(&complex, &field);
    // One triangle × 4 coords: only the adjacency coordinate is non-zero.
    assert_eq!(obs, vec![b(0), b(0), b(0), b(3)], "only locality fails to close");
}

#[test]
fn a_pure_coboundary_field_resolves_to_nothing() {
    // Take the same field but strip the adjacency coordinate: every remaining
    // coordinate is a global difference (a coboundary), so with no vocabulary
    // the resolve explains all of it.
    let g = GridObs::new(1, 3, vec![1, 2, 4]);
    let (complex, mut field) = measure_relational(&g);
    // zero out coord 3 of every edge (indices 3, 7, 11).
    for e in 0..3 {
        field[e * 4 + 3] = b(0);
    }
    let r = resolve(&complex, &field, &[]);
    assert!(r.solved, "a pure coboundary field resolves with empty vocabulary");
}

#[test]
fn the_local_signal_is_unresolved_until_its_translation_is_known() {
    let g = GridObs::new(1, 3, vec![1, 2, 4]);
    let (complex, field) = measure_relational(&g);

    // With no vocabulary the adjacency signal is irreducible.
    let bare = resolve(&complex, &field, &[]);
    assert!(!bare.solved, "the local adjacency structure is not a coboundary");
    assert!(bare.residual.iter().any(|x| *x != b(0)), "a real residual remains");

    // The explaining translation: the field's adjacency component alone
    // (coord 3 of each edge). field − this ∈ Im(δ⁰), so adding it resolves all.
    let mut adj_only = vec![b(0); field.len()];
    for e in 0..3 {
        adj_only[e * 4 + 3] = field[e * 4 + 3].clone();
    }
    let solved = resolve(&complex, &field, &[adj_only.clone()]);
    assert!(solved.solved, "knowing the local translation resolves the obstruction");

    // Idempotent: resolving the residual again changes nothing.
    let again = resolve(&complex, &bare.residual, &[]);
    assert_eq!(again.residual, bare.residual, "resolve is idempotent");
}
