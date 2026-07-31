//! Object-centric perception: connected components and the operators
//! that act on OBJECTS rather than the whole grid.
//!
//! ARC-AGI-2 evaluation tasks are overwhelmingly object-and-relation
//! puzzles; grid-global symmetries cannot even express most answers.
//! This module gives the vocabulary eyes for things: segmentation is
//! exact (4-connectivity over non-background cells), and every
//! operator here remains GENERAL — no task knowledge, no fixture
//! constant, the no-hardcoding law whole.

use crate::grid::Grid;

/// One connected component: its cells (row, col, hue) and bounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    pub cells: Vec<(usize, usize, u8)>,
    pub top: usize,
    pub left: usize,
    pub bottom: usize,
    pub right: usize,
}

impl Object {
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn height(&self) -> usize {
        self.bottom - self.top + 1
    }

    pub fn width(&self) -> usize {
        self.right - self.left + 1
    }
}

/// The background hue: the most common hue in the grid (ties break
/// toward the LOWER hue — deterministic, and 0/black wins its ties,
/// matching ARC's dominant convention without hardcoding it).
pub fn background(g: &Grid) -> u8 {
    let mut counts = [0usize; 10];
    for &cell in &g.cells {
        counts[cell as usize] += 1;
    }
    let mut best = 0u8;
    for hue in 1..10u8 {
        if counts[hue as usize] > counts[best as usize] {
            best = hue;
        }
    }
    best
}

/// Exact segmentation: 4-connected components of non-background
/// cells. `same_hue_only` = components split by hue (each object one
/// color) vs. multicolor blobs — both readings exist in real tasks,
/// so both are lawful lenses.
pub fn segment(g: &Grid, bg: u8, same_hue_only: bool) -> Vec<Object> {
    let mut seen = vec![false; g.cells.len()];
    let mut objects = Vec::new();
    for start in 0..g.cells.len() {
        if seen[start] || g.cells[start] == bg {
            continue;
        }
        // Flood from `start`.
        let mut stack = vec![start];
        let mut cells = Vec::new();
        seen[start] = true;
        while let Some(at) = stack.pop() {
            let (r, c) = (at / g.cols, at % g.cols);
            cells.push((r, c, g.cells[at]));
            let mut push = |nr: usize, nc: usize| {
                let n = nr * g.cols + nc;
                if !seen[n]
                    && g.cells[n] != bg
                    && (!same_hue_only || g.cells[n] == g.cells[at])
                {
                    seen[n] = true;
                    stack.push(n);
                }
            };
            if r > 0 {
                push(r - 1, c);
            }
            if r + 1 < g.rows {
                push(r + 1, c);
            }
            if c > 0 {
                push(r, c - 1);
            }
            if c + 1 < g.cols {
                push(r, c + 1);
            }
        }
        let top = cells.iter().map(|&(r, _, _)| r).min().expect("nonempty");
        let bottom = cells.iter().map(|&(r, _, _)| r).max().expect("nonempty");
        let left = cells.iter().map(|&(_, c, _)| c).min().expect("nonempty");
        let right = cells.iter().map(|&(_, c, _)| c).max().expect("nonempty");
        cells.sort_unstable();
        objects.push(Object {
            cells,
            top,
            left,
            bottom,
            right,
        });
    }
    // Canonical order: by size descending, then top-left position —
    // "largest" and "first" mean the same thing on every machine.
    objects.sort_by(|a, b| {
        b.size()
            .cmp(&a.size())
            .then(a.top.cmp(&b.top))
            .then(a.left.cmp(&b.left))
    });
    objects
}

/// The object-centric operators. Each is a total function on grids
/// (an inapplicable read renders the grid unchanged — the verifier
/// rejects useless words by failing to close, never by crashing).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjOp {
    /// Keep only the largest object (background elsewhere).
    KeepLargest,
    /// Keep only the smallest object.
    KeepSmallest,
    /// Delete the largest object (fill with background).
    DeleteLargest,
    /// Crop the grid to the largest object's bounding box.
    CropLargest,
    /// Crop to the bounding box of ALL non-background content.
    CropContent,
    /// Recolor every object to the hue of the LARGEST object.
    PaintAllLikeLargest,
    /// Recolor each object by its size RANK into the hue ladder the
    /// observation's largest object wears downward (rank r takes hue
    /// of rank 0 where ambiguous — observation-general, no fixed map).
    /// Kept simple: every object takes the majority hue of the grid's
    /// objects.
    PaintAllMajority,
    /// Gravity: every object falls straight down until it rests on
    /// the floor or another object (column-exact).
    GravityDown,
}

pub const OBJ_OPS: [ObjOp; 8] = [
    ObjOp::KeepLargest,
    ObjOp::KeepSmallest,
    ObjOp::DeleteLargest,
    ObjOp::CropLargest,
    ObjOp::CropContent,
    ObjOp::PaintAllLikeLargest,
    ObjOp::PaintAllMajority,
    ObjOp::GravityDown,
];

impl ObjOp {
    pub fn describe(&self) -> &'static str {
        match self {
            ObjOp::KeepLargest => "keep_largest",
            ObjOp::KeepSmallest => "keep_smallest",
            ObjOp::DeleteLargest => "delete_largest",
            ObjOp::CropLargest => "crop_largest",
            ObjOp::CropContent => "crop_content",
            ObjOp::PaintAllLikeLargest => "paint_all_like_largest",
            ObjOp::PaintAllMajority => "paint_all_majority",
            ObjOp::GravityDown => "gravity_down",
        }
    }

    pub fn apply(&self, g: &Grid) -> Grid {
        let bg = background(g);
        let objects = segment(g, bg, false);
        if objects.is_empty() {
            return g.clone();
        }
        let blank = |rows: usize, cols: usize| Grid {
            rows,
            cols,
            cells: vec![bg; rows * cols],
        };
        let stamp = |grid: &mut Grid, object: &Object, hue: Option<u8>| {
            for &(r, c, h) in &object.cells {
                grid.cells[r * grid.cols + c] = hue.unwrap_or(h);
            }
        };
        match self {
            ObjOp::KeepLargest => {
                let mut out = blank(g.rows, g.cols);
                stamp(&mut out, &objects[0], None);
                out
            }
            ObjOp::KeepSmallest => {
                let mut out = blank(g.rows, g.cols);
                stamp(&mut out, objects.last().expect("nonempty"), None);
                out
            }
            ObjOp::DeleteLargest => {
                let mut out = g.clone();
                stamp(&mut out, &objects[0], Some(bg));
                out
            }
            ObjOp::CropLargest => crop(g, &objects[0]),
            ObjOp::CropContent => {
                let top = objects.iter().map(|o| o.top).min().expect("nonempty");
                let left = objects.iter().map(|o| o.left).min().expect("nonempty");
                let bottom = objects.iter().map(|o| o.bottom).max().expect("nonempty");
                let right = objects.iter().map(|o| o.right).max().expect("nonempty");
                let bounds = Object {
                    cells: Vec::new(),
                    top,
                    left,
                    bottom,
                    right,
                };
                crop(g, &bounds)
            }
            ObjOp::PaintAllLikeLargest => {
                let hue = majority_hue(&objects[0]);
                let mut out = g.clone();
                for object in &objects {
                    stamp(&mut out, object, Some(hue));
                }
                out
            }
            ObjOp::PaintAllMajority => {
                let mut counts = [0usize; 10];
                for object in &objects {
                    counts[majority_hue(object) as usize] += 1;
                }
                let hue = (0..10u8)
                    .max_by_key(|&h| (counts[h as usize], u8::MAX - h))
                    .expect("ten hues");
                let mut out = g.clone();
                for object in &objects {
                    stamp(&mut out, object, Some(hue));
                }
                out
            }
            ObjOp::GravityDown => {
                let mut out = blank(g.rows, g.cols);
                // Column-exact fall: per column, stack non-background
                // cells to the floor preserving their order.
                for c in 0..g.cols {
                    let column: Vec<u8> = (0..g.rows)
                        .map(|r| g.at(r, c))
                        .filter(|&h| h != bg)
                        .collect();
                    for (i, &hue) in column.iter().rev().enumerate() {
                        let r = g.rows - 1 - i;
                        out.cells[r * g.cols + c] = hue;
                    }
                }
                out
            }
        }
    }
}

fn crop(g: &Grid, bounds: &Object) -> Grid {
    let rows = bounds.bottom - bounds.top + 1;
    let cols = bounds.right - bounds.left + 1;
    let mut cells = Vec::with_capacity(rows * cols);
    for r in bounds.top..=bounds.bottom {
        for c in bounds.left..=bounds.right {
            cells.push(g.at(r, c));
        }
    }
    Grid { rows, cols, cells }
}

fn majority_hue(object: &Object) -> u8 {
    let mut counts = [0usize; 10];
    for &(_, _, h) in &object.cells {
        counts[h as usize] += 1;
    }
    (0..10u8)
        .max_by_key(|&h| (counts[h as usize], u8::MAX - h))
        .expect("ten hues")
}
