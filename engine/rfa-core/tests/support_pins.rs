//! SUPPORT-AWARE REDUCTION — the sparse complex carries topology the
//! dense one cannot (operator ruling 2026-08-01).
//!
//! `coboundary_reduce` assumes every pair is an edge and anchors at
//! vertex 0. On a complete relation every cocycle is a coboundary, so
//! it annihilates everything and `D ≡ 0`. Worse, treating an ABSENT
//! relation as a ZERO relation invents triangles that do not exist and
//! reports their phantom residuals as real strain. These pins measure
//! both failures and the fix.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{coherence_functional, holonomies, Delta, Support, Lens};

/// |x| for an exact rational — the holonomy's sign is a basis choice
/// (see the ring pin); its size and its vanishing are not.
fn magnitude(value: &Ratio) -> Ratio {
    match *value < ratio(0, 1) {
        true => -value.clone(),
        false => value.clone(),
    }
}

/// A ring: 0—1—2—3—0, each edge carrying `1`, and NO chords. Four
/// edges, four vertices, one component: exactly one independent cycle.
fn ring() -> (Delta, Support) {
    let mut delta = Delta::zero(4, 1);
    let mut support = Support::new(4, 1);
    for (i, j) in [(0, 1), (1, 2), (2, 3)] {
        delta.set_antisym(i, j, vec![ratio(1, 1)]);
        support.connect(i, j, 0);
    }
    // The closing edge, written 3→0 so the loop sum is non-zero.
    delta.set_antisym(0, 3, vec![ratio(-1, 1)]);
    support.connect(0, 3, 0);
    (delta, support)
}

#[test]
fn an_unchorded_cycle_carries_holonomy_the_dense_reduction_cannot_see() {
    // THE PAYOFF. A ring with no chords has no filled triangle, so it
    // is a cocycle — and it is NOT a coboundary, because walking the
    // loop does not return to where it started. That charge is `∮ ≠ 0`,
    // the holonomic pole's topological charge, and it is capacity.
    let (delta, support) = ring();
    let found = holonomies(&delta, &support);

    assert_eq!(found.len(), 1, "four edges, four vertices, one component: ONE independent cycle");
    assert_ne!(found[0].value, ratio(0, 1), "the loop carries charge no frame can explain");
    // The SIGN depends on which edge the forest left out and how it is
    // oriented — both are basis choices. The MAGNITUDE and the
    // vanishing are invariant, and only those two carry law: `D` counts
    // non-zero holonomies, so it never sees the sign.
    assert_eq!(magnitude(&found[0].value), ratio(4, 1), "exact: the walk around the ring");
}

#[test]
fn the_dense_reading_invents_strain_from_absent_edges() {
    // THE FALSE GAUGE COLLAPSE, measured. The dense reading cannot tell
    // "0 and 2 do not relate" from "0 and 2 relate by zero", so it
    // checks triangle (0,1,2) as though it were filled and reports a
    // residual that describes nothing in the world. That phantom strain
    // is what made D undefined instead of positive.
    let (delta, _) = ring();
    assert_ne!(
        coherence_functional(&delta),
        Lens::of(ratio(0, 1)),
        "dense: phantom residuals from triangles that are not there"
    );
    // Sparse: no triangle is filled, so there is no cocycle condition
    // to fail — the ring IS a cocycle, and its charge is real.
    let (delta, support) = ring();
    assert_eq!(holonomies(&delta, &support).len(), 1);
}

#[test]
fn a_tree_is_pure_frame_and_carries_nothing() {
    // A spanning tree has no cycles at all, so every edge is explained
    // by the potentials it induces. Frame, entirely.
    let mut delta = Delta::zero(4, 1);
    let mut support = Support::new(4, 1);
    for (i, j, v) in [(0, 1, 3), (1, 2, 5), (1, 3, 7)] {
        delta.set_antisym(i, j, vec![ratio(v, 1)]);
        support.connect(i, j, 0);
    }
    assert!(holonomies(&delta, &support).is_empty(), "a tree carries no holonomy");
}

#[test]
fn a_cycle_that_closes_consistently_carries_nothing() {
    // The ring again, but the closing edge AGREES with the walk: the
    // loop returns to its origin, so the charge is zero. A cycle is not
    // automatically structure — only a cycle that fails to close is.
    let mut delta = Delta::zero(4, 1);
    let mut support = Support::new(4, 1);
    for (i, j) in [(0, 1), (1, 2), (2, 3)] {
        delta.set_antisym(i, j, vec![ratio(1, 1)]);
        support.connect(i, j, 0);
    }
    delta.set_antisym(0, 3, vec![ratio(3, 1)]); // p3 − p0 = 3: consistent
    support.connect(0, 3, 0);
    assert!(holonomies(&delta, &support).is_empty(), "a closing loop carries no charge");
}

#[test]
fn components_reduce_independently_and_absent_pairs_are_never_walked() {
    // Two disjoint pairs. Each is its own component with its own root,
    // and nothing crosses the gap — an absent relation is not an edge,
    // so the forest never walks it.
    let mut delta = Delta::zero(4, 1);
    let mut support = Support::new(4, 1);
    for (i, j) in [(0, 1), (2, 3)] {
        delta.set_antisym(i, j, vec![ratio(9, 1)]);
        support.connect(i, j, 0);
    }
    assert!(holonomies(&delta, &support).is_empty(), "two trees, no cycles, no charge");
    assert!(!support.has(0, 2, 0), "the gap is absent, not zero");
}

#[test]
fn the_support_is_symmetric_and_refuses_the_diagonal() {
    let mut support = Support::new(3, 2);
    support.connect(0, 1, 1);
    assert!(support.has(0, 1, 1) && support.has(1, 0, 1), "a relation runs both ways");
    assert!(!support.has(0, 1, 0), "each coordinate carries its own edges");
    support.connect(2, 2, 0);
    assert!(!support.has(2, 2, 0), "the diagonal is never an edge");
}

#[test]
fn holonomy_is_exact_rational_never_rounded() {
    // The charge is exact: thirds stay thirds.
    let mut delta = Delta::zero(3, 1);
    let mut support = Support::new(3, 1);
    for (i, j) in [(0, 1), (1, 2)] {
        delta.set_antisym(i, j, vec![ratio(1, 3)]);
        support.connect(i, j, 0);
    }
    delta.set_antisym(0, 2, vec![ratio(1, 1)]);
    support.connect(0, 2, 0);
    let found = holonomies(&delta, &support);
    assert_eq!(found.len(), 1);
    let expected: Ratio = ratio(1, 1) - ratio(2, 3);
    assert_eq!(magnitude(&found[0].value), expected, "1 − 2/3 = 1/3, exactly");
}
