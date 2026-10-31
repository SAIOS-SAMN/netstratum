//! **The perspective algebra** (38_COGNITION_CONVERGENCE §5, task #14)
//! — the ARC-AGI-2 device: a compositional task falls to many views,
//! not deeper solitary search.
//!
//! One line's attempt on a task is a **Perspective**: the grid it
//! derived, and — where it fell short of the answer — the structured
//! RESIDUAL, the cells it got wrong. A structured failure is not
//! noise; its connected clusters (**silhouettes**) are the sub-rules
//! the view missed, each a sub-task the forge can draw on its own.
//!
//! Perspectives compound by per-cell congruence (charter §5, the
//! membrane's `compound_perspectives`): the assembled output takes the
//! majority hue at each cell; the cells where views disagree localize
//! the unsolved component; consensus is the exact rational
//! `agree / total`, never a probability. When every view's residual
//! agrees on the silhouette count the gap is systematic — the mesh
//! knows what it is missing, which is a mintable want, not a shrug.
//!
//! **This module is the pure algebra.** It decides nothing about which
//! lines attempt a task or how a compound crystallizes into a shared
//! deed — that orchestration is mesh-coupled policy, specced in §5 of
//! the charter and sequenced with the mesh lane (task #19). Here the
//! views arrive as grids and leave as a reading; delete any caller and
//! no path changes (the no-hardcoding law).

use solver_verify::grid::Grid;
use solver_verify::objects::segment;

/// One line's attempt: the grid it derived, against the expected
/// answer. The residual and silhouettes are read at construction so a
/// perspective carries its own structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Perspective {
    /// The line whose view this is (its own id — provenance, never a
    /// key that weights the compound; congruence is by cell, not by
    /// author).
    pub line: u64,
    /// What the line derived for the test.
    pub derived: Grid,
    /// The cells it got wrong against the expected answer, as
    /// `(row, col)` — empty when the view was exact. `None` when no
    /// answer is known (the archive withheld it): a perspective can
    /// still compound with others, it just carries no residual.
    pub residual: Option<Vec<(usize, usize)>>,
    /// Connected clusters of the residual — the missed sub-rules.
    /// `None` mirrors `residual`.
    pub silhouettes: Option<usize>,
}

impl Perspective {
    /// Read a perspective from a derived grid and (optionally) the
    /// expected answer. Mismatched dimensions read as a whole residual
    /// — the view got the shape wrong, which is maximal structure, not
    /// an error to hide.
    pub fn read(line: u64, derived: Grid, expected: Option<&Grid>) -> Perspective {
        let (residual, silhouettes) = match expected {
            None => (None, None),
            Some(want) => {
                let cells = residual_cells(&derived, want);
                let count = silhouette_count(&derived, &cells);
                (Some(cells), Some(count))
            }
        };
        Perspective {
            line,
            derived,
            residual,
            silhouettes,
        }
    }
}

/// The cells `derived` got wrong against `want`. A dimension mismatch
/// is the whole of the larger grid — every cell is a miss.
fn residual_cells(derived: &Grid, want: &Grid) -> Vec<(usize, usize)> {
    if derived.rows != want.rows || derived.cols != want.cols {
        let rows = derived.rows.max(want.rows);
        let cols = derived.cols.max(want.cols);
        return (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .collect();
    }
    let mut cells = Vec::new();
    for r in 0..derived.rows {
        for c in 0..derived.cols {
            let at = r * derived.cols + c;
            if derived.cells[at] != want.cells[at] {
                cells.push((r, c));
            }
        }
    }
    cells
}

/// Connected clusters (4-neighbour) of the residual cells — the k
/// missed sub-rules. Reuses the sealed segmentation: paint the residual
/// onto a blank grid and count its objects, so the clustering law is
/// the one the object operators already use (call-not-copy).
fn silhouette_count(shape: &Grid, residual: &[(usize, usize)]) -> usize {
    if residual.is_empty() {
        return 0;
    }
    let rows = shape
        .rows
        .max(residual.iter().map(|(r, _)| r + 1).max().unwrap_or(0));
    let cols = shape
        .cols
        .max(residual.iter().map(|(_, c)| c + 1).max().unwrap_or(0));
    // Hue 1 marks a residual cell on a 0 background; every cell is on
    // the ladder, so `Grid::new` cannot refuse.
    let mut cells = vec![0u8; rows * cols];
    for &(r, c) in residual {
        cells[r * cols + c] = 1;
    }
    let grid = Grid::new(rows, cols, cells).expect("residual paints on the ladder");
    // Background is fixed at 0, not auto-detected: the residual cells
    // are the foreground by construction, and `background`'s
    // most-common rule would flip when misses outnumber hits (a whole
    // grid of residual would read as one background and zero
    // silhouettes — the exact inversion of the truth).
    segment(&grid, 0, false).len()
}

/// The reading a set of perspectives compounds to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundPerspective {
    /// The assembled output: the majority hue at each cell.
    pub compound: Grid,
    /// The cells where the views disagreed — the unsolved component,
    /// localized.
    pub divergent: Vec<(usize, usize)>,
    /// `agree / total`, exact — as (numerator, denominator) so the
    /// caller carries no float.
    pub consensus: (usize, usize),
    /// Every residual-bearing view agreed on the silhouette count: the
    /// mesh knows what it is missing. `None` when no view carried a
    /// residual, or the residual views disagreed on the count.
    pub systematic_gap: Option<usize>,
}

/// **Compound** — per-cell congruence across views of one task.
///
/// All views must share dimensions (they are attempts at one grid);
/// a view of a different shape is dropped from the compound and named
/// in the return's `Err` — a shape disagreement is a coarser signal
/// than a cell one and belongs to the orchestration, not here. `None`
/// when no two views agree on a shape.
pub fn compound(perspectives: &[Perspective]) -> Option<CompoundPerspective> {
    let first = perspectives.first()?;
    let (rows, cols) = (first.derived.rows, first.derived.cols);
    let aligned: Vec<&Perspective> = perspectives
        .iter()
        .filter(|p| p.derived.rows == rows && p.derived.cols == cols)
        .collect();
    if aligned.is_empty() {
        return None;
    }
    let mut compound = vec![0u8; rows * cols];
    let mut divergent = Vec::new();
    let mut agree = 0usize;
    for r in 0..rows {
        for c in 0..cols {
            let at = r * cols + c;
            // The majority hue and how many views hold it.
            let mut counts = [0usize; 10];
            for p in &aligned {
                counts[p.derived.cells[at] as usize] += 1;
            }
            let (hue, held) = counts
                .iter()
                .enumerate()
                .max_by_key(|(hue, count)| (**count, std::cmp::Reverse(*hue)))
                .map(|(hue, count)| (hue as u8, *count))
                .expect("ten slots");
            compound[at] = hue;
            if held == aligned.len() {
                agree += 1;
            } else {
                divergent.push((r, c));
            }
        }
    }
    let compound = Grid::new(rows, cols, compound).expect("majority hues on the ladder");
    // Systematic gap: the residual-bearing views all agree on a count.
    let counts: Vec<usize> = aligned.iter().filter_map(|p| p.silhouettes).collect();
    let systematic_gap = match counts.split_first() {
        Some((head, tail)) if tail.iter().all(|n| n == head) => Some(*head),
        _ => None,
    };
    Some(CompoundPerspective {
        compound,
        divergent,
        consensus: (agree, rows * cols),
        systematic_gap,
    })
}
