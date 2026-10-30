//! Derivation — the observation names the operator.
//!
//! The struck enumeration asked "which of these 14+ operators, composed
//! how many deep, happens to fit?" and swept up to 200,000 candidates
//! to find out. This module asks the opposite question: *what actually
//! changed between input and output, and which operator is the one that
//! makes that change?* The answer is read, not searched.
//!
//! Two derivations already existed and were the only non-swept parts of
//! the old solver — the recolor map (read cellwise off the pairs) and
//! the tile ratio (read off the shapes). This extends the principle to
//! every family, so nothing is guessed:
//!
//! ```text
//!     shape ratio is integral        → Tile, its mirrors read off pair 0
//!     output is a sub-box of input   → Crop (largest object, or content)
//!     object count falls             → Keep / Delete
//!     hue variety collapses          → Paint
//!     column mass conserved, moved   → Gravity
//!     dims equal or transposed       → Symmetry
//!     hues map consistently          → Recolor (under any symmetry frame)
//! ```
//!
//! Each is a necessary condition read from the observation. A candidate
//! that survives derivation is not yet a solution — the verifier still
//! re-derives it against every training pair, and the sector door still
//! withdraws whatever fails. Derivation narrows the reach; it never
//! decides.
//!
//! **The no-hardcoding law is untouched.** Nothing here branches on a
//! task's identity; every test is over the observation's own structure.
//! Delete any fixture and no code path changes.

use crate::need::Family;
use solver_verify::grid::{Grid, Op, TileMirror};
use solver_verify::loader::Task;
use solver_verify::objects::{background, segment, ObjOp};

/// One derived candidate: the operator, and the family it belongs to so
/// a [`crate::need::Need`] can order it without naming operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub op: Op,
    pub family: Family,
}

/// The symmetries, as a frame vocabulary (used for derivation prefixes,
/// never swept as a search space).
const SYMMETRIES: [Op; 6] = [
    Op::Rotate90,
    Op::Rotate180,
    Op::Rotate270,
    Op::FlipH,
    Op::FlipV,
    Op::Transpose,
];

/// Symmetries whose shape is compatible with the observation.
///
/// A quarter turn transposes the dimensions; a mirror preserves them.
/// Reading the shapes rules most of them out before a single cell is
/// compared — and where dimensions are square, shape cannot separate
/// them, so all six stay reachable and exactness decides.
fn derive_symmetries(task: &Task) -> Vec<Candidate> {
    let first = &task.train[0];
    let (ir, ic) = (first.input.rows, first.input.cols);
    let (or, oc) = (first.output.rows, first.output.cols);
    let mut found = Vec::new();
    for op in SYMMETRIES {
        let fits = match op {
            Op::Rotate90 | Op::Rotate270 | Op::Transpose => or == ic && oc == ir,
            _ => or == ir && oc == ic,
        };
        if fits {
            found.push(Candidate { op, family: Family::Symmetry });
        }
    }
    found
}

/// The hue map, read cellwise off the pairs under each symmetry frame.
///
/// Carried over from the old `derived_recolors` unchanged in spirit:
/// for each frame, if every pair agrees on where each hue goes, that
/// mapping is the operator. Unobserved hues map to themselves.
fn derive_recolors(task: &Task) -> Vec<Candidate> {
    let prefixes: [Option<Op>; 7] = [
        None,
        Some(Op::Rotate90),
        Some(Op::Rotate180),
        Some(Op::Rotate270),
        Some(Op::FlipH),
        Some(Op::FlipV),
        Some(Op::Transpose),
    ];
    let mut found: Vec<Candidate> = Vec::new();
    'prefix: for prefix in &prefixes {
        let mut map = [255u8; 10];
        for pair in &task.train {
            let seen = match prefix {
                None => pair.input.clone(),
                Some(op) => op.apply(&pair.input),
            };
            if seen.rows != pair.output.rows || seen.cols != pair.output.cols {
                continue 'prefix;
            }
            for (i, &from) in seen.cells.iter().enumerate() {
                let to = pair.output.cells[i];
                match map[from as usize] {
                    255 => map[from as usize] = to,
                    already if already == to => {}
                    _ => continue 'prefix,
                }
            }
        }
        for (hue, slot) in map.iter_mut().enumerate() {
            if *slot == 255 {
                *slot = hue as u8;
            }
        }
        let op = Op::Recolor(map);
        if !found.iter().any(|c| c.op == op) {
            found.push(Candidate { op, family: Family::Recolor });
        }
    }
    found
}

/// The tiling, read off the shape ratio — and its mirrors read off the
/// content, rather than the sixteen combinations being swept.
///
/// The old derivation emitted all 4×4 mirror pairs and let the search
/// sort it out; that was enumeration hiding inside a derivation. Here
/// each odd band is tested against pair 0 directly, so at most one
/// combination is proposed.
fn derive_tiles(task: &Task) -> Vec<Candidate> {
    let first = &task.train[0];
    let (ir, ic) = (first.input.rows, first.input.cols);
    let (or, oc) = (first.output.rows, first.output.cols);
    if ir == 0 || ic == 0 || or % ir != 0 || oc % ic != 0 {
        return Vec::new();
    }
    let (down, across) = (or / ir, oc / ic);
    if down * across <= 1 || down > 6 || across > 6 {
        return Vec::new();
    }
    // Read each band's mirror off the observation instead of trying all.
    let mut found = Vec::new();
    for odd_row in TileMirror::ALL {
        for odd_col in TileMirror::ALL {
            let op = Op::Tile { down, across, odd_row, odd_col };
            if op.apply(&first.input) == first.output {
                found.push(Candidate { op, family: Family::Tile });
                return found; // the observation named exactly one
            }
        }
    }
    // No mirror combination reproduces pair 0: the family does not
    // apply, and proposing it anyway would be a guess.
    found
}

/// Object-centric operators, derived from structural deltas.
///
/// Each arm below is a necessary condition read off pair 0 — object
/// counts, bounding boxes, hue variety, column mass. An operator whose
/// precondition the observation does not show is never proposed.
fn derive_objects(task: &Task) -> Vec<Candidate> {
    let first = &task.train[0];
    let (input, output) = (&first.input, &first.output);
    let bg = background(input);
    let objects = segment(input, bg, false);
    let mut found = Vec::new();
    let push = |op: ObjOp, found: &mut Vec<Candidate>| {
        found.push(Candidate { op: Op::Object(op), family: Family::Object });
    };

    if objects.is_empty() {
        return found;
    }

    let same_dims = input.rows == output.rows && input.cols == output.cols;
    let smaller = output.rows <= input.rows && output.cols <= input.cols && !same_dims;

    // Crops — the output is a sub-box. Compare against the boxes the
    // observation actually contains.
    if smaller {
        let largest = &objects[0];
        if output.rows == largest.height() && output.cols == largest.width() {
            push(ObjOp::CropLargest, &mut found);
        }
        let top = objects.iter().map(|o| o.top).min().unwrap_or(0);
        let left = objects.iter().map(|o| o.left).min().unwrap_or(0);
        let bottom = objects.iter().map(|o| o.bottom).max().unwrap_or(0);
        let right = objects.iter().map(|o| o.right).max().unwrap_or(0);
        if output.rows == bottom - top + 1 && output.cols == right - left + 1 {
            push(ObjOp::CropContent, &mut found);
        }
    }

    if !same_dims {
        return found;
    }

    let out_objects = segment(output, background(output), false);
    let in_non_bg = input.cells.iter().filter(|&&h| h != bg).count();
    let out_non_bg = output.cells.iter().filter(|&&h| h != bg).count();

    // Keep / delete — the object count moved.
    if out_objects.len() < objects.len() {
        if out_objects.len() == 1 {
            push(ObjOp::KeepLargest, &mut found);
            push(ObjOp::KeepSmallest, &mut found);
        }
        if out_objects.len() + 1 == objects.len() {
            push(ObjOp::DeleteLargest, &mut found);
        }
    }

    // Paint — the cells stayed, the hue variety collapsed.
    if in_non_bg == out_non_bg && objects.len() > 1 {
        let in_hues: std::collections::BTreeSet<u8> =
            input.cells.iter().copied().filter(|&h| h != bg).collect();
        let out_hues: std::collections::BTreeSet<u8> =
            output.cells.iter().copied().filter(|&h| h != bg).collect();
        if out_hues.len() < in_hues.len() && out_hues.len() == 1 {
            push(ObjOp::PaintAllLikeLargest, &mut found);
            push(ObjOp::PaintAllMajority, &mut found);
        }
    }

    // Gravity — every column keeps its mass and the content sank.
    if in_non_bg == out_non_bg && in_non_bg > 0 {
        let column_mass_holds = (0..input.cols).all(|c| {
            let before = (0..input.rows).filter(|&r| input.at(r, c) != bg).count();
            let after = (0..output.rows).filter(|&r| output.at(r, c) != bg).count();
            before == after
        });
        let sank = (0..input.cols).any(|c| {
            let lowest = |g: &Grid| (0..g.rows).rfind(|&r| g.at(r, c) != bg);
            match (lowest(input), lowest(output)) {
                (Some(a), Some(b)) => b > a,
                _ => false,
            }
        });
        if column_mass_holds && sank {
            push(ObjOp::GravityDown, &mut found);
        }
    }

    found
}

/// The REACH — every candidate the observation supports, and nothing
/// else. Order here is derivation order; a [`crate::need::Need`] may
/// reorder it, never extend it.
pub fn derive(task: &Task) -> Vec<Candidate> {
    if task.train.is_empty() {
        return Vec::new();
    }
    let mut reach = Vec::new();
    reach.extend(derive_symmetries(task));
    reach.extend(derive_recolors(task));
    reach.extend(derive_tiles(task));
    reach.extend(derive_objects(task));
    reach.dedup_by(|a, b| a.op == b.op);
    reach
}
