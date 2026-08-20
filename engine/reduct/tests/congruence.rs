//! N-I1 witness — the congruence read is a signed chain that re-forms its target
//! (the riding inverse). A recolored cell departs its old hue and arrives at the
//! new one; applying that chain to A re-forms B; identical vantages read empty.

use reduct::congruence::{re_form_with, signed_remainder};
use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use num_bigint::BigInt;

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
fn a_recolor_is_a_signed_chain_that_re_forms_the_target() {
    // A carries hue 1 at (0,1); B carries hue 2 there — one cell recolored.
    let a = measurement(2, 2, 3, &[0, 1, 0, 0]);
    let b = measurement(2, 2, 3, &[0, 2, 0, 0]);
    let chain = signed_remainder(&a, &b).expect("same frame reads");
    // −1 at the departed (site, hue), +1 at the arrived — content and support one.
    let departed = chain
        .entries
        .iter()
        .find(|e| e.direction == GaugeDirection { index: 1 })
        .expect("hue 1 departed (0,1)");
    let arrived = chain
        .entries
        .iter()
        .find(|e| e.direction == GaugeDirection { index: 2 })
        .expect("hue 2 arrived (0,1)");
    assert_eq!(departed.site, Site { row: 0, col: 1 });
    assert_eq!(departed.coefficient, BigInt::from(-1));
    assert_eq!(arrived.site, Site { row: 0, col: 1 });
    assert_eq!(arrived.coefficient, BigInt::from(1));
    // THE RIDING INVERSE: applying the chain to A re-forms B exactly.
    assert_eq!(re_form_with(&a, &chain).expect("the chain re-forms"), b, "Δ carries A → B");
}

#[test]
fn identical_vantages_read_the_empty_chain() {
    let a = measurement(2, 2, 2, &[0, 1, 1, 0]);
    let chain = signed_remainder(&a, &a).expect("same frame reads");
    assert!(chain.entries.is_empty(), "no asymmetry is the empty chain, never a fabricated zero");
}
