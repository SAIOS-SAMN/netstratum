//! N-I3 Slice 2 witness — the `Grid → FullMeasurement` transducer. A grid
//! measures WHOLE into the fixed ten-hue ladder fiber (never the per-grid max),
//! and that uniform fiber is exactly what lets a pure hue permutation read as a
//! gauge move (CongruentThroughP) rather than false content difference.

use iae::peel::GaugeDirection;
use iae::reduction::{reduce_pair, ReduceVerdict};
use solver::think::measure_grid;
use solver_verify::grid::Grid;

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("lawful grid")
}

#[test]
fn a_grid_measures_whole_into_the_ten_hue_ladder() {
    let m = measure_grid(&grid(2, 2, &[0, 1, 2, 3])).expect("a dense grid measures whole");
    assert_eq!(m.fiber_rank(), 10, "the ARC fiber is the fixed ten-hue ladder, not the grid's max");
    assert_eq!((m.frame().rows, m.frame().cols), (2, 2), "the frame is the grid's shape");
    assert_eq!(m.occupancy().len(), 4, "every cell is carried exactly once");
    // Row-major carriage: (0,0)→0, (0,1)→1, (1,0)→2, (1,1)→3.
    let hues: Vec<u8> = m.occupancy().iter().map(|(_, d)| d.index).collect();
    assert_eq!(hues, vec![0, 1, 2, 3], "each cell carries its hue as a gauge direction");
}

#[test]
fn a_sparse_palette_still_lives_in_the_full_ladder() {
    // Hues {5, 6} — the per-grid max would give fiber 6/7 and split the ladder;
    // the transducer pins fiber 10 so the two grids are comparable at all.
    let a = measure_grid(&grid(1, 2, &[5, 6])).expect("whole");
    assert_eq!(a.fiber_rank(), 10, "a sparse palette is still measured in the full ladder");
    // Direction 5 is present, direction 9 is absent — absence is structural.
    assert!(a.occupancy().iter().any(|(_, d)| *d == GaugeDirection { index: 6 }));
}

#[test]
fn a_pure_recolor_reads_as_a_gauge_move() {
    // THE FIBER PIN: all-5s → all-6s is a hue permutation, a gauge move. This
    // reads CongruentThroughP ONLY because both grids measure into the SAME
    // fiber rank (10). A per-grid-max fiber would make the forms unequal and
    // fabricate a generation gap — the sodalitum false wall, at the seat.
    let a = measure_grid(&grid(2, 2, &[5, 5, 5, 5])).expect("whole");
    let b = measure_grid(&grid(2, 2, &[6, 6, 6, 6])).expect("whole");
    match reduce_pair(&a, &b, &[]) {
        ReduceVerdict::CongruentThroughP { p } => {
            assert_eq!(p.apply(&a).expect("whole"), b, "the gauge carries A → B");
            assert_eq!(p.inverse().apply(&b).expect("whole"), a, "and reconstructs B → A");
        }
        other => panic!("a pure recolor is a gauge move, got {other:?}"),
    }
}
