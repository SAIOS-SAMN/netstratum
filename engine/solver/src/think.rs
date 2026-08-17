//! **THE ARC SEAT** (N-I3, doc `53_INTELLIGENCE_PORT` D1/D2) — the solver reads
//! its train grids through the ONE reduce organ. It owns NO search: it peels the
//! observed pairs into measurements, hands them to `iae::think`, and — only when
//! the task GLUES a nameable frame — renders that frame to a program and seals
//! the proof. Every other verdict WITHHOLDS. `search::solve` / `propose_*` /
//! `sweep_vocabulary` are the costume this seat replaces; none is called here.
//!
//! Slice 2 (this landing): the `Grid → FullMeasurement` transducer only.
//! Slice 3 adds `think_task`, the render table, and the certificate.

use iae::peel::{CellFrame, FullMeasurement, GaugeDirection, MeasurementRefusal, Site};
use solver_verify::grid::{Grid, HUES};

/// Transduce a colored grid into a full measurement: every cell is a site
/// carrying its hue as a gauge direction.
///
/// **The fiber is the FIXED ten-hue ladder (`HUES`), NOT the grid's own max.**
/// A uniform fiber is load-bearing: a full measurement carries its fiber rank in
/// its identity, and `canonicalize` preserves it, so `deduce_gauge_move` reads
/// two forms equal ONLY when their ranks match. Were the fiber the per-grid max,
/// an all-5s grid (rank 6) and an all-6s grid (rank 7) would never deduce a
/// gauge move — the 0d3d703e discriminator (a pure hue permutation is
/// CongruentThroughP) would break at the seat. The ARC gauge fiber IS the wire's
/// ten-hue ladder; every grid lives in it.
///
/// Background 0 is a direction like any other. A grid is dense by construction,
/// so it measures whole; `try_new` re-verifies wholeness regardless
/// (whole-or-refused — a grid we cannot measure is never fabricated into one).
pub fn measure_grid(grid: &Grid) -> Result<FullMeasurement, MeasurementRefusal> {
    let occupancy = (0..grid.rows)
        .flat_map(|r| (0..grid.cols).map(move |c| (r, c)))
        .map(|(r, c)| {
            (Site { row: r as i64, col: c as i64 }, GaugeDirection { index: grid.at(r, c) })
        })
        .collect();
    FullMeasurement::try_new(
        HUES,
        CellFrame { rows: grid.rows as u32, cols: grid.cols as u32 },
        occupancy,
    )
}
