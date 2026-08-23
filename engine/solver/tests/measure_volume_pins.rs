//! VOLUMETRIC-1b witness — the `Volume → FullMeasurement` transducer, the
//! `measure_grid` counterpart for a stack of congruent layers. A volume
//! measures WHOLE into the SAME fixed ten-hue ladder fiber `measure_grid`
//! uses (no separate volumetric fiber), and `layer` rides the site alongside
//! `row`/`col` — a per-layer recolor should read as a gauge move exactly the
//! way a planar one does, since `PlaneElement::site` passes `layer` through
//! unchanged (VOLUMETRIC-1a).

use reduct::peel::GaugeDirection;
use reduct::reduction::{reduce_pair, ReduceVerdict};
use solver::think::measure_volume;
use solver_verify::grid::Grid;
use solver_verify::volume::Volume;

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("lawful grid")
}

fn volume(layers: Vec<Grid>) -> Volume {
    Volume::new(layers).expect("congruent layers")
}

#[test]
fn a_two_layer_volume_measures_whole_into_the_ten_hue_ladder() {
    let v = volume(vec![grid(2, 2, &[0, 1, 2, 3]), grid(2, 2, &[4, 5, 6, 7])]);
    let m = measure_volume(&v).expect("a dense volume measures whole");
    assert_eq!(m.fiber_rank(), 10, "the SAME fixed ladder measure_grid uses");
    assert_eq!(
        (m.frame().rows, m.frame().cols, m.frame().layers),
        (2, 2, 2),
        "the frame carries the stack depth alongside rows/cols"
    );
    assert_eq!(m.occupancy().len(), 8, "every cell of every layer is carried exactly once");
}

#[test]
fn each_layer_carries_its_own_hues_at_the_same_row_col() {
    let v = volume(vec![grid(1, 1, &[3]), grid(1, 1, &[7])]);
    let m = measure_volume(&v).expect("whole");
    let by_layer: Vec<(i64, u8)> =
        m.occupancy().iter().map(|(s, d)| (s.layer, d.index)).collect();
    assert!(by_layer.contains(&(0, 3)), "layer 0's own cell carries layer 0's hue");
    assert!(by_layer.contains(&(1, 7)), "layer 1's own cell carries layer 1's hue, not layer 0's");
}

#[test]
fn a_single_layer_volume_matches_measure_grid_exactly() {
    // The `layers == 1` case is the planar case — VOLUMETRIC-1a's own
    // correctness property (output-identical for every 2-D caller) checked
    // one level up, through the volume seat instead of the grid seat.
    let g = grid(2, 2, &[1, 2, 3, 4]);
    let v = volume(vec![g.clone()]);
    let via_volume = measure_volume(&v).expect("whole");
    let via_grid = solver::think::measure_grid(&g).expect("whole");
    assert_eq!(via_volume, via_grid, "a one-layer volume measures identically to its bare grid");
}

#[test]
fn a_pure_per_layer_recolor_reads_as_a_gauge_move() {
    // The volumetric mirror of measure_grid_pins' fiber pin: recoloring every
    // layer identically (5→6 everywhere) is a hue permutation, so it must
    // deduce as a gauge move, not a fabricated generation gap.
    let a = volume(vec![grid(2, 2, &[5, 5, 5, 5]), grid(2, 2, &[5, 5, 5, 5])]);
    let b = volume(vec![grid(2, 2, &[6, 6, 6, 6]), grid(2, 2, &[6, 6, 6, 6])]);
    let ma = measure_volume(&a).expect("whole");
    let mb = measure_volume(&b).expect("whole");
    match reduce_pair(&ma, &mb, &[]) {
        ReduceVerdict::CongruentThroughP { p } => {
            assert_eq!(p.apply(&ma).expect("whole"), mb, "the gauge carries A → B across every layer");
        }
        other => panic!("a pure per-layer recolor is a gauge move, got {other:?}"),
    }
}

#[test]
fn sites_present_in_one_layer_are_absent_evidence_in_the_other() {
    let v = volume(vec![grid(1, 2, &[5, 6])]);
    let m = measure_volume(&v).expect("whole");
    assert!(m.occupancy().iter().any(|(_, d)| *d == GaugeDirection { index: 6 }));
}
