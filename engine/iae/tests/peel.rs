//! N-I1 witness — the peel is whole-or-refused and re-forms its input (the seed
//! law). No judgment, no transform: a `FullMeasurement` exists only whole, and
//! its cover lifts back to it exactly.

use iae::peel::{
    peel, CellFrame, FullMeasurement, GaugeDirection, MeasurementRefusal, Site,
};

/// Every site of a `rows × cols` frame carried once, hue read row-major from
/// `hues` (length rows·cols) — a whole occupancy for the constructor to admit.
fn occupancy(rows: i64, cols: i64, hues: &[u8]) -> Vec<(Site, GaugeDirection)> {
    (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .enumerate()
        .map(|(i, (row, col))| (Site { row, col }, GaugeDirection { index: hues[i] }))
        .collect()
}

#[test]
fn a_whole_measurement_admits_and_its_cover_re_forms_it() {
    // A 2×2 with two hues (background 0, one body of hue 1) — every site once,
    // fiber_rank 2 covers the hues.
    let frame = CellFrame { rows: 2, cols: 2 };
    let measurement =
        FullMeasurement::try_new(2, frame, occupancy(2, 2, &[0, 1, 1, 0])).expect("whole admits");
    // THE SEED LAW: the cover lifts back to the input exactly. A cover that
    // could not re-form its measurement is a projection without its inverse.
    let cover = peel(&measurement);
    assert_eq!(
        cover.re_form_measurement().expect("a cover re-forms whole"),
        measurement,
        "peel → re_form is the identity (the seed law)",
    );
    // The value manifold names the two present directions; absence is structural.
    assert_eq!(cover.value.supports.len(), 2, "both present hues carry a support");
}

#[test]
fn a_partial_or_over_carried_measurement_cannot_exist() {
    let frame = CellFrame { rows: 2, cols: 2 };
    // MISSING a site — (1,1) never carried.
    let missing = vec![
        (Site { row: 0, col: 0 }, GaugeDirection { index: 0 }),
        (Site { row: 0, col: 1 }, GaugeDirection { index: 0 }),
        (Site { row: 1, col: 0 }, GaugeDirection { index: 0 }),
    ];
    assert!(
        matches!(
            FullMeasurement::try_new(2, frame, missing),
            Err(MeasurementRefusal::SiteUncarried { .. })
        ),
        "a measurement missing a site is refused typed",
    );
    // A site carried TWICE — (0,0) doubled, (1,1) dropped.
    let doubled = vec![
        (Site { row: 0, col: 0 }, GaugeDirection { index: 0 }),
        (Site { row: 0, col: 0 }, GaugeDirection { index: 1 }),
        (Site { row: 0, col: 1 }, GaugeDirection { index: 0 }),
        (Site { row: 1, col: 0 }, GaugeDirection { index: 0 }),
    ];
    assert!(
        matches!(
            FullMeasurement::try_new(2, frame, doubled),
            Err(MeasurementRefusal::SiteCarriedTwice { .. })
        ),
        "a site carried twice is refused typed",
    );
    // A direction OUTSIDE the fiber rank — hue 2 with fiber_rank 2.
    assert!(
        matches!(
            FullMeasurement::try_new(2, frame, occupancy(2, 2, &[0, 1, 2, 0])),
            Err(MeasurementRefusal::DirectionOutsideFiber { index: 2, fiber_rank: 2 })
        ),
        "a hue at or above the fiber rank is refused typed",
    );
}
