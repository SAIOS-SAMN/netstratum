//! THE NON-ABELIAN FIBER — witnesses that the ring wakes up.
//!
//! The claim under test is physical, not decorative: a closed loop of
//! SCALAR connections carries no field by construction, and the same
//! loop with MATRIX connections carries flux. If that holds, the Stasis
//! we measured on rings was an artefact of the fiber rather than a fact
//! about the economy.

use cfe::ratio::ratio;
use rfa_core::fiber::{loop_holonomy, plaquette_curvature, MatrixFiber2x2};

/// A shear — `det = 1`, and it does not commute with its transpose-side
/// partner.
fn shear(k: i64) -> MatrixFiber2x2 {
    MatrixFiber2x2::new(ratio(1, 1), ratio(k, 1), ratio(0, 1), ratio(1, 1))
}

/// The other shear direction. `[shear, lower]` is where the field lives.
fn lower(k: i64) -> MatrixFiber2x2 {
    MatrixFiber2x2::new(ratio(1, 1), ratio(0, 1), ratio(k, 1), ratio(1, 1))
}

/// A scalar connection embedded as a matrix: `s · I`. Scalars commute
/// with everything, which is precisely the problem.
fn scalar(s: i64) -> MatrixFiber2x2 {
    MatrixFiber2x2::new(ratio(s, 1), ratio(0, 1), ratio(0, 1), ratio(s, 1))
}

#[test]
fn a_scalar_fiber_has_no_field_the_bracket_vanishes_identically() {
    // THE DIAGNOSIS. Every scalar connection commutes with every other,
    // so `[A, B] = 0` always and `F = dA + A∧A` loses its second term.
    // A scalar loop is flat by construction — there is nothing for it
    // to hold.
    for (p, q) in [(2i64, 3i64), (-5, 7), (1, 1)] {
        let (a, b) = (scalar(p), scalar(q));
        assert!(a.commutes_with(&b), "scalars always commute");
        assert!(
            plaquette_curvature(&a, &b).is_zero(),
            "and a commuting plaquette carries no curvature"
        );
    }
}

#[test]
fn a_matrix_fiber_carries_field_the_bracket_does_not_vanish() {
    // THE CURE, measured. Two shears in different directions do not
    // commute, so the plaquette holds curvature — the second term of
    // `F` is alive.
    let (a, b) = (shear(1), lower(1));
    assert!(!a.commutes_with(&b), "different directions do not commute");
    let field = plaquette_curvature(&a, &b);
    assert!(!field.is_zero(), "F ≠ 0: the cell carries flux");
    // Exact: [shear(1), lower(1)] = [[1,0],[0,-1]], taxicab magnitude 2.
    // (This pin was written asserting 4 and recorded RED — the
    // arithmetic was the test's error, not the fiber's.)
    assert_eq!(field.l1_norm(), ratio(2, 1), "exact rational curvature, no root taken");
    // AND THE LIE-ALGEBRA PROPERTY: a bracket of SL₂ elements lands in
    // sl₂, which is traceless. The curvature is an algebra element, not
    // a group element — that is what makes it a FIELD rather than
    // another connection.
    assert_eq!(field.trace(), ratio(0, 1), "[A,B] is traceless: it lives in sl₂");
}

#[test]
fn the_scalar_ring_closes_dead_and_the_matrix_ring_does_not() {
    // THE WHOLE ARGUMENT IN ONE PIN. A four-step loop of equal
    // connections:
    //   scalar — the walk multiplies to a multiple of the identity and
    //            carries no direction; nothing was acquired.
    //   matrix — the walk multiplies to Aⁿ ≠ I. The ring HOLDS
    //            something, and it held it without any external charge
    //            bolted onto a vertex.
    let flat = loop_holonomy(&[scalar(1), scalar(1), scalar(1), scalar(1)]);
    assert!(flat.is_identity(), "a scalar ring of unit connections returns nothing");

    let live = loop_holonomy(&[shear(1), shear(1), shear(1), shear(1)]);
    assert!(!live.is_identity(), "the same ring, matrix-valued, does NOT return to rest");
    assert_eq!(live, shear(4), "and what it holds is exact: four shears compose to shear(4)");
}

#[test]
fn order_matters_which_is_what_having_a_field_means() {
    // In an abelian loop the order of traversal is irrelevant — sums
    // commute. Here it is not, and that asymmetry IS the field.
    let forward = loop_holonomy(&[shear(1), lower(1)]);
    let backward = loop_holonomy(&[lower(1), shear(1)]);
    assert_ne!(forward, backward, "the walk depends on the order walked");
    assert_eq!(
        forward.minus(&backward),
        plaquette_curvature(&shear(1), &lower(1)),
        "and the difference between the two walks IS the curvature"
    );
}

#[test]
fn the_connections_are_volume_preserving_sl2() {
    assert!(MatrixFiber2x2::identity().is_sl2());
    assert!(shear(7).is_sl2(), "a shear preserves volume");
    assert!(lower(-3).is_sl2());
    assert_eq!(shear(2).det(), ratio(1, 1), "det exactly one");
    assert!(!scalar(2).is_sl2(), "2·I scales area by four — not SL₂");
}

#[test]
fn congruence_not_conjugation_is_the_gauge_law() {
    // A gauge change acts as `g_i · Δ · g_jᵀ`. With the identity on both
    // sides the relation is untouched; a real congruence moves it.
    let relation = MatrixFiber2x2::new(ratio(2, 1), ratio(1, 1), ratio(1, 1), ratio(1, 1));
    let id = MatrixFiber2x2::identity();
    assert_eq!(
        relation.congruate(&id, &id),
        relation,
        "the trivial gauge changes nothing"
    );
    let moved = relation.congruate(&shear(1), &shear(1));
    assert_ne!(moved, relation, "a real gauge change moves the fiber");
    // Congruence by SL₂ preserves the determinant exactly.
    assert_eq!(moved.det(), relation.det(), "det is a congruence invariant under SL₂");
}

#[test]
fn magnitude_is_l1_and_stays_exact() {
    let f = MatrixFiber2x2::new(ratio(1, 3), ratio(-1, 3), ratio(2, 3), ratio(0, 1));
    assert_eq!(f.l1_norm(), ratio(4, 3), "thirds stay thirds; no square, no root");
    assert!(MatrixFiber2x2::zero().is_zero());
}
