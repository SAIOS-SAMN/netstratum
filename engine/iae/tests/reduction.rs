//! N-I1 witnesses for the reduce, landed slice by slice.
//!
//! SLICE 1 — the plane group / relabel / observer frame reconstruct exactly.
//! This is where a silent sign lives: a wrong axis or mirror survives every
//! downstream test as a plausible-but-wrong gauge, so it is pinned HERE, by
//! reconstruction, before `reduce_pair` exists.

use iae::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use iae::reduction::{Exchange, Flip, ObserverFrame, PlaneElement, Relabel};

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
