//! N-I2 witness — Pin B: a residual that WALLED think at K is a crystal at K+1
//! on the same vessel; the same class then DRAINS. No second vantage, no fold. A
//! gauge does not grow the basis; the adopted word re-forms its source; adoption
//! is idempotent.

use reduct::congruence::re_form_with;
use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use reduct::reduction::{reduce_pair, ReduceVerdict};
use reduct::{adopt, observe, Observed};

fn measurement(rows: i64, cols: i64, fiber: u8, hues: &[u8]) -> FullMeasurement {
    let occupancy = (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .enumerate()
        .map(|(i, (row, col))| (Site { row, col }, GaugeDirection { index: hues[i] }))
        .collect();
    FullMeasurement::try_new(fiber, CellFrame { rows: rows as u32, cols: cols as u32 }, occupancy)
        .expect("whole admits")
}

/// A novel-hue wall: B carries a hue A never expressed (not a global relabel).
fn walled_pair() -> (FullMeasurement, FullMeasurement) {
    (measurement(2, 2, 3, &[0, 1, 0, 1]), measurement(2, 2, 3, &[0, 1, 0, 2]))
}

#[test]
fn pin_b_a_wall_at_k_is_a_crystal_at_k_plus_one() {
    let (a, b) = walled_pair();
    let mut grown: Vec<reduct::Crystal> = Vec::new();

    // K: empty basis — the pair walls, and the wall is minted.
    let crystal = match observe(&a, &b, &grown) {
        Observed::Minted(c) => c,
        other => panic!("a novel hue walls and mints, got {other:?}"),
    };
    adopt(&mut grown, crystal);
    assert_eq!(grown.len(), 1, "the vessel grew by one crystal");

    // K+1: the SAME class, now against the grown basis — it drains.
    assert_eq!(observe(&a, &b, &grown), Observed::Accounted, "the same class now accounts");
    // At the drain level: Congruent with a NON-EMPTY bounding (the crystal
    // accounted the whole residual), sun/fold never consulted.
    match reduce_pair(&a, &b, &grown) {
        ReduceVerdict::Congruent { bounding } => {
            assert_eq!(bounding.len(), 1, "the grown crystal drained the residual whole");
        }
        other => panic!("K+1 drains to Congruent, got {other:?}"),
    }
}

#[test]
fn a_gauge_does_not_grow_the_basis() {
    // all-5s → all-6s is a gauge (CongruentThroughP), never a wall — nothing to
    // adopt, the basis stays empty (the sodalitum false-wall refusal).
    let a = measurement(2, 2, 7, &[5, 5, 5, 5]);
    let b = measurement(2, 2, 7, &[6, 6, 6, 6]);
    let grown: Vec<reduct::Crystal> = Vec::new();
    assert_eq!(observe(&a, &b, &grown), Observed::Accounted, "a gauge accounts, never mints");
    assert!(grown.is_empty(), "a gauge grows nothing");
}

#[test]
fn the_adopted_word_re_forms_its_source() {
    // The seed law on the minted crystal: its class carries A → B, so the class
    // NEGATED carries B → A (re-form the source from the target).
    let (a, b) = walled_pair();
    let crystal = match observe(&a, &b, &[]) {
        Observed::Minted(c) => c,
        other => panic!("expected a mint, got {other:?}"),
    };
    assert_eq!(
        re_form_with(&b, &crystal.class.negated()).expect("the inverse re-forms"),
        a,
        "the word's inverse re-forms A from B (the seed law)",
    );
}

#[test]
fn adoption_is_idempotent() {
    let (a, b) = walled_pair();
    let crystal = match observe(&a, &b, &[]) {
        Observed::Minted(c) => c,
        other => panic!("expected a mint, got {other:?}"),
    };
    let mut grown: Vec<reduct::Crystal> = Vec::new();
    adopt(&mut grown, crystal.clone());
    adopt(&mut grown, crystal);
    assert_eq!(grown.len(), 1, "the same obstruction twice is one crystal");
}
