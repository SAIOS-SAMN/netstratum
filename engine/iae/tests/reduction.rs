//! N-I1 witnesses for the reduce, landed slice by slice.
//!
//! SLICE 1 — the plane group / relabel / observer frame reconstruct exactly.
//! This is where a silent sign lives: a wrong axis or mirror survives every
//! downstream test as a plausible-but-wrong gauge, so it is pinned HERE, by
//! reconstruction, before `reduce_pair` exists.

use iae::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use iae::reduction::{
    canonicalize, deduce_gauge_move, Exchange, Flip, GaugeDeduction, ObserverFrame, PlaneElement,
    Relabel,
};

fn measurement(rows: i64, cols: i64, fiber: u8, hues: &[u8]) -> FullMeasurement {
    let occupancy = (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .enumerate()
        .map(|(i, (row, col))| (Site { row, col }, GaugeDirection { index: hues[i] }))
        .collect();
    FullMeasurement::try_new(fiber, CellFrame { rows: rows as u32, cols: cols as u32 }, occupancy)
        .expect("whole admits")
}

#[test]
fn a_quarter_turn_and_its_inverse_return_the_source() {
    // rot90 is ONE of the eight plane elements, read off its components (Swap
    // the axes, mirror the column) — never an Op::Rotate90 special case.
    let rot90 = PlaneElement { exchange: Exchange::Swap, row: Flip::Keep, col: Flip::Mirror };
    // Distinct hues so the round-trip actually moves sites, not just a symmetric
    // blur that any wrong sign would also fix.
    let m = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let turned = ObserverFrame { plane: rot90, relabel: Relabel::new(Vec::new()) }
        .apply(&m)
        .expect("a plane move stays whole");
    let back = ObserverFrame { plane: rot90.inverse(), relabel: Relabel::new(Vec::new()) }
        .apply(&turned)
        .expect("the inverse move stays whole");
    assert_eq!(back, m, "p⁻¹ ∘ p reconstructs the source (occupancy is content)");
    // The plane group law itself: p ∘ p⁻¹ is the identity element.
    assert_eq!(rot90.compose(&rot90.inverse()), PlaneElement::IDENTITY, "p ∘ p⁻¹ = e");
}

#[test]
fn a_mirror_is_an_involution() {
    // FlipH — mirror the columns — composed with itself is the identity.
    let flip_h = PlaneElement { exchange: Exchange::Keep, row: Flip::Keep, col: Flip::Mirror };
    assert_eq!(flip_h.compose(&flip_h), PlaneElement::IDENTITY, "FlipH ∘ FlipH = e");
}

#[test]
fn a_relabel_transposition_is_an_involution() {
    // The permutation (1 2) — swap two hues — twice is the identity, carried as
    // the empty (no moved points) relabel.
    let swap12 = Relabel::new(vec![(1, 2), (2, 1)]);
    assert_eq!(swap12.compose(&swap12), Relabel::new(Vec::new()), "(1 2) ∘ (1 2) = e");
    assert_eq!(swap12.inverse(), swap12, "a transposition is its own inverse");
    // Total on unmoved points; fixed points are not data.
    assert_eq!(swap12.image(GaugeDirection { index: 3 }), GaugeDirection { index: 3 });
}

// ── SLICE 2 — canonicalize + deduce_gauge_move (the DERIVE seat) ──────────
// Equal canonical forms are a GAUGE MOVE with a bijective rule that
// reconstructs BOTH ways; distinct content is ContentDiffers. The all-5s vs
// all-6s row is the sodalitum discriminator: a no-op canonicalize reads it as a
// generation gap; a real orbit-min reads the relabel AS the rule.

fn deduce(a: &FullMeasurement, b: &FullMeasurement) -> GaugeDeduction {
    deduce_gauge_move(&canonicalize(a), &canonicalize(b))
}

/// A gauge rule must carry its inverse: p(A)=B AND p⁻¹(B)=A (the reconstruction
/// pin — a non-bijective rule fails the second leg).
fn reconstructs(rule: &ObserverFrame, a: &FullMeasurement, b: &FullMeasurement) {
    assert_eq!(rule.apply(a).expect("rule stays whole"), *b, "p(A) = B");
    assert_eq!(rule.inverse().apply(b).expect("inverse stays whole"), *a, "p⁻¹(B) = A");
}

#[test]
fn equal_content_is_a_gauge_move_that_reconstructs() {
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    match deduce(&a, &a) {
        GaugeDeduction::GaugeMove { rule } => reconstructs(&rule, &a, &a),
        other => panic!("A = A is a gauge move, got {other:?}"),
    }
}

#[test]
fn a_rotation_is_deduced_as_the_gauge_and_reconstructs() {
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let rot90 = PlaneElement { exchange: Exchange::Swap, row: Flip::Keep, col: Flip::Mirror };
    let b = ObserverFrame { plane: rot90, relabel: Relabel::new(Vec::new()) }
        .apply(&a)
        .expect("a plane move stays whole");
    match deduce(&a, &b) {
        GaugeDeduction::GaugeMove { rule } => reconstructs(&rule, &a, &b),
        other => panic!("a rotation is a gauge move, got {other:?}"),
    }
}

#[test]
fn a_pure_hue_permutation_is_the_gauge_not_a_wall() {
    // 0d3d703e: all fives vs all sixes — the SAME structure through a relabel.
    // The rule must be the bijection 5↔6, whose inverse reconstructs; a
    // no-op canonicalize would mis-read this as ContentDiffers → GenerationGap.
    let a = measurement(2, 2, 7, &[5, 5, 5, 5]);
    let b = measurement(2, 2, 7, &[6, 6, 6, 6]);
    match deduce(&a, &b) {
        GaugeDeduction::GaugeMove { rule } => reconstructs(&rule, &a, &b),
        other => panic!("a pure hue permutation is a gauge, not a wall, got {other:?}"),
    }
}

#[test]
fn a_frame_shape_change_is_content_difference() {
    let a = measurement(2, 2, 1, &[0, 0, 0, 0]);
    let b = measurement(3, 3, 1, &[0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert!(
        matches!(deduce(&a, &b), GaugeDeduction::ContentDiffers),
        "a different frame shape is content difference, never a fabricated gauge",
    );
}
