//! ENTITY-ALIGN-1 (docs/56 §3) — `confirmation_hardness` mirrors
//! `solver::consilience`'s already-deployed measure (`orders.dedup().len()`)
//! at vessel scope: the count of DISTINCT knowledge-readings among the
//! confirming vessels, a basin read as a SET (order/duplicate-count
//! ignored).

use conductor::confirmation_hardness;
use reduct::peel::CellFrame;
use reduct::ChainDelta;
use solver::Crystal;
use std::collections::BTreeMap;

fn frame(rows: u32) -> CellFrame {
    CellFrame { rows, cols: 1, layers: 1 }
}

#[test]
fn identical_basins_measure_hardness_one() {
    let mut grown: BTreeMap<u64, Vec<Crystal>> = BTreeMap::new();
    let crystal = Crystal::seal(ChainDelta::empty(frame(1)));
    grown.insert(7, vec![crystal.clone()]);
    grown.insert(9, vec![crystal]);

    assert_eq!(
        confirmation_hardness(&grown, &[7, 9]),
        1,
        "two vessels with the SAME class set bridge no diversity"
    );
}

#[test]
fn distinct_basins_measure_hardness_two() {
    let mut grown: BTreeMap<u64, Vec<Crystal>> = BTreeMap::new();
    grown.insert(7, vec![Crystal::seal(ChainDelta::empty(frame(1)))]);
    grown.insert(9, vec![Crystal::seal(ChainDelta::empty(frame(2)))]);

    assert_eq!(
        confirmation_hardness(&grown, &[7, 9]),
        2,
        "two vessels with genuinely different class sets bridge real diversity"
    );
}

#[test]
fn an_empty_or_unknown_vessel_reads_as_its_own_empty_basin() {
    let grown: BTreeMap<u64, Vec<Crystal>> = BTreeMap::new();
    assert_eq!(
        confirmation_hardness(&grown, &[7, 9]),
        1,
        "two vessels neither of which has grown yet both read as the same empty set"
    );
}
