//! The grid carrier and the transformation vocabulary — exact,
//! discrete, invertible-or-typed. A transformation either produces a
//! grid or the composition is unrepresentable; nothing rounds, nothing
//! interpolates, nothing guesses.

/// A colored grid: row-major cells, hues 0–9. The ARC state carrier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<u8>,
}

/// The lawful hue ladder: 0–9 inclusive. Wire law, not a preference —
/// two operators in the sealed math index ten-slot tables by hue
/// (`Op::Recolor`'s `[u8; 10]`, `objects::background`'s `[usize; 10]`),
/// so a hue past the ladder is an out-of-bounds panic inside the gate.
pub const HUES: u8 = 10;

impl Grid {
    /// The one door. Dimensions must be non-degenerate, the cell count
    /// must match, and every hue must sit on the ladder.
    ///
    /// The hue check was added 2026-08-04: the type documented "hues
    /// 0–9" and never enforced it, leaving the invariant to whichever
    /// caller remembered — the loader did, the wire did not. Under
    /// work-shapes v1 no wire-borne grid ever reached an operator, so
    /// nothing had gone wrong yet; the corpus registry (v2) ends that
    /// by carrying task content in the payload. Refuse never guess: a
    /// hue past the ladder is not clamped, it is not a grid.
    pub fn new(rows: usize, cols: usize, cells: Vec<u8>) -> Option<Self> {
        if rows == 0 || cols == 0 || cells.len() != rows * cols {
            return None;
        }
        if cells.iter().any(|&hue| hue >= HUES) {
            return None;
        }
        Some(Self { rows, cols, cells })
    }

    pub fn at(&self, r: usize, c: usize) -> u8 {
        self.cells[r * self.cols + c]
    }

    /// Refill this grid in place, reusing its existing allocation.
    ///
    /// A grid at live difficulty is 3×3 to 4×4 — nine to sixteen bytes.
    /// Allocating a fresh `Vec` for each is almost pure allocator
    /// overhead: the payload is smaller than the bookkeeping. Any caller
    /// that produces many grids in a loop should hold one buffer and
    /// refill it, which is what this exists for.
    ///
    /// `clear` keeps the capacity, so the second and later calls on the
    /// same buffer allocate nothing.
    pub fn fill_from(&mut self, rows: usize, cols: usize, f: impl Fn(usize, usize) -> u8) {
        self.cells.clear();
        self.cells.reserve(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                self.cells.push(f(r, c));
            }
        }
        self.rows = rows;
        self.cols = cols;
    }
}

/// The transformation vocabulary — general operators only (the
/// no-hardcoding law: nothing in here knows any task).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    /// Quarter turn clockwise.
    Rotate90,
    Rotate180,
    Rotate270,
    /// Mirror left↔right.
    FlipH,
    /// Mirror top↔bottom.
    FlipV,
    /// Reflect across the main diagonal.
    Transpose,
    /// A hue permutation/mapping: `map[old] = new`.
    Recolor([u8; 10]),
    /// An object-centric operator (segmentation-based — see
    /// `crate::objects`).
    Object(crate::objects::ObjOp),
    /// Tile the grid `down × across`; odd tile rows/columns may carry
    /// a mirror (the symmetry-quilt family).
    Tile {
        down: usize,
        across: usize,
        /// Mirror applied inside tiles on odd tile rows.
        odd_row: TileMirror,
        /// Mirror applied inside tiles on odd tile columns.
        odd_col: TileMirror,
    },
    /// A translator (docs/62 L1): every cell carried by `(dr, dc)`; a cell
    /// the motion vacates takes the grid's background. A generator of the
    /// algebra, not a named transform: its arguments are the motion.
    Shift {
        dr: i8,
        dc: i8,
    },
    /// A dilator (docs/62 L1): each cell becomes an `up x up` block, or each
    /// `down x down` block is read as its top-left cell. One of `up`, `down`
    /// is 1.
    Dilate {
        up: u8,
        down: u8,
    },
    /// A projector (docs/62 L7): the bodies a predicate picks, recoloured
    /// or removed. Learned from the pairs, never a named object op.
    Project(crate::project::ObjectRule),
}

/// The in-tile mirror an odd row/column band carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileMirror {
    None,
    /// Reverse row order (vertical mirror).
    Rows,
    /// Reverse column order (horizontal mirror).
    Cols,
    /// Both (a 180° turn).
    Both,
}

impl TileMirror {
    pub const ALL: [TileMirror; 4] = [
        TileMirror::None,
        TileMirror::Rows,
        TileMirror::Cols,
        TileMirror::Both,
    ];

    fn apply(self, rr: &mut usize, cc: &mut usize, rows: usize, cols: usize) {
        if matches!(self, TileMirror::Rows | TileMirror::Both) {
            *rr = rows - 1 - *rr;
        }
        if matches!(self, TileMirror::Cols | TileMirror::Both) {
            *cc = cols - 1 - *cc;
        }
    }

    fn label(self) -> &'static str {
        match self {
            TileMirror::None => "none",
            TileMirror::Rows => "rows",
            TileMirror::Cols => "cols",
            TileMirror::Both => "both",
        }
    }
}

impl Op {
    /// A stable, human-readable name — hashed into the certificate.
    pub fn describe(&self) -> String {
        match self {
            Op::Rotate90 => "rotate90".into(),
            Op::Rotate180 => "rotate180".into(),
            Op::Rotate270 => "rotate270".into(),
            Op::FlipH => "flip_h".into(),
            Op::FlipV => "flip_v".into(),
            Op::Transpose => "transpose".into(),
            Op::Recolor(map) => {
                let m: Vec<String> = map.iter().map(|h| h.to_string()).collect();
                format!("recolor[{}]", m.join(","))
            }
            Op::Object(op) => op.describe().into(),
            Op::Tile {
                down,
                across,
                odd_row,
                odd_col,
            } => format!(
                "tile{down}x{across}(odd_row={},odd_col={})",
                odd_row.label(),
                odd_col.label()
            ),
            Op::Shift { dr, dc } => format!("shift({dr},{dc})"),
            Op::Dilate { up, down } => format!("dilate({up}/{down})"),
            Op::Project(rule) => format!("project({:?}->{:?})", rule.select, rule.act),
        }
    }

    /// Apply — exact by construction.
    ///
    /// Allocates a fresh grid. A caller applying ops in a tight loop
    /// should use [`Op::apply_into`] with one reused buffer instead;
    /// this is the same computation, called not copied.
    pub fn apply(&self, g: &Grid) -> Grid {
        let mut out = Grid {
            rows: 0,
            cols: 0,
            cells: Vec::new(),
        };
        self.apply_into(g, &mut out);
        out
    }

    /// Apply, writing into a caller-held buffer that keeps its
    /// allocation between calls.
    ///
    /// Identical arithmetic to [`Op::apply`] — this is its body, and
    /// `apply` is the wrapper. At live grid sizes the allocation costs
    /// more than the transformation, so a search testing tens of
    /// thousands of candidates wants this form.
    pub fn apply_into(&self, g: &Grid, out: &mut Grid) {
        match self {
            Op::Recolor(map) => {
                out.cells.clear();
                out.cells.reserve(g.cells.len());
                out.cells.extend(g.cells.iter().map(|&h| map[h as usize]));
                out.rows = g.rows;
                out.cols = g.cols;
            }
            // The object operators segment the grid and build their own
            // result; they are the costly arm and there are only eight,
            // so they keep the allocating path.
            Op::Object(op) => *out = op.apply(g),
            Op::Rotate90 => out.fill_from(g.cols, g.rows, |r, c| g.at(g.rows - 1 - c, r)),
            Op::Rotate180 => {
                out.fill_from(g.rows, g.cols, |r, c| g.at(g.rows - 1 - r, g.cols - 1 - c))
            }
            Op::Rotate270 => out.fill_from(g.cols, g.rows, |r, c| g.at(c, g.cols - 1 - r)),
            Op::FlipH => out.fill_from(g.rows, g.cols, |r, c| g.at(r, g.cols - 1 - c)),
            Op::FlipV => out.fill_from(g.rows, g.cols, |r, c| g.at(g.rows - 1 - r, c)),
            Op::Transpose => out.fill_from(g.cols, g.rows, |r, c| g.at(c, r)),
            Op::Tile {
                down,
                across,
                odd_row,
                odd_col,
            } => out.fill_from(g.rows * down, g.cols * across, |r, c| {
                let (tile_r, mut rr) = (r / g.rows, r % g.rows);
                let (tile_c, mut cc) = (c / g.cols, c % g.cols);
                if tile_r % 2 == 1 {
                    odd_row.apply(&mut rr, &mut cc, g.rows, g.cols);
                }
                if tile_c % 2 == 1 {
                    odd_col.apply(&mut rr, &mut cc, g.rows, g.cols);
                }
                g.at(rr, cc)
            }),
            Op::Shift { dr, dc } => {
                let bg = crate::objects::background(g);
                let (dr, dc) = (i64::from(*dr), i64::from(*dc));
                let (rows, cols) = (g.rows as i64, g.cols as i64);
                out.fill_from(g.rows, g.cols, |r, c| {
                    let (sr, sc) = (r as i64 - dr, c as i64 - dc);
                    if sr >= 0 && sr < rows && sc >= 0 && sc < cols {
                        g.at(sr as usize, sc as usize)
                    } else {
                        bg
                    }
                })
            }
            Op::Project(rule) => *out = crate::project::apply(rule, g),
            Op::Dilate { up, down } => {
                let (up, down) = (usize::from((*up).max(1)), usize::from((*down).max(1)));
                out.fill_from(g.rows * up / down, g.cols * up / down, |r, c| {
                    g.at(r * down / up, c * down / up)
                })
            }
        }
    }
}

/// A program: an ordered word over the vocabulary, applied left to
/// right. The solution object — small, exact, replayable.
pub type Program = Vec<Op>;

pub fn run(program: &Program, input: &Grid) -> Grid {
    let mut g = input.clone();
    for op in program {
        g = op.apply(&g);
    }
    g
}
