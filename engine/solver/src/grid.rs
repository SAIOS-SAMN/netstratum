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

impl Grid {
    pub fn new(rows: usize, cols: usize, cells: Vec<u8>) -> Option<Self> {
        if rows == 0 || cols == 0 || cells.len() != rows * cols {
            return None;
        }
        Some(Self { rows, cols, cells })
    }

    pub fn at(&self, r: usize, c: usize) -> u8 {
        self.cells[r * self.cols + c]
    }

    fn from_fn(rows: usize, cols: usize, f: impl Fn(usize, usize) -> u8) -> Grid {
        let mut cells = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                cells.push(f(r, c));
            }
        }
        Grid { rows, cols, cells }
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
    /// Tile the grid `down × across`; odd tile rows/columns may carry
    /// a mirror (the symmetry-quilt family).
    Tile {
        down: usize,
        across: usize,
        /// Mirror applied inside tiles on odd tile ROWS.
        odd_row: TileMirror,
        /// Mirror applied inside tiles on odd tile COLUMNS.
        odd_col: TileMirror,
    },
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
        }
    }

    /// Apply — exact by construction.
    pub fn apply(&self, g: &Grid) -> Grid {
        match self {
            Op::Rotate90 => {
                Grid::from_fn(g.cols, g.rows, |r, c| g.at(g.rows - 1 - c, r))
            }
            Op::Rotate180 => Grid::from_fn(g.rows, g.cols, |r, c| {
                g.at(g.rows - 1 - r, g.cols - 1 - c)
            }),
            Op::Rotate270 => {
                Grid::from_fn(g.cols, g.rows, |r, c| g.at(c, g.cols - 1 - r))
            }
            Op::FlipH => Grid::from_fn(g.rows, g.cols, |r, c| g.at(r, g.cols - 1 - c)),
            Op::FlipV => Grid::from_fn(g.rows, g.cols, |r, c| g.at(g.rows - 1 - r, c)),
            Op::Transpose => Grid::from_fn(g.cols, g.rows, |r, c| g.at(c, r)),
            Op::Recolor(map) => Grid {
                rows: g.rows,
                cols: g.cols,
                cells: g.cells.iter().map(|&h| map[h as usize]).collect(),
            },
            Op::Tile {
                down,
                across,
                odd_row,
                odd_col,
            } => Grid::from_fn(g.rows * down, g.cols * across, |r, c| {
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
