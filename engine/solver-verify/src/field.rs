//! **THE FIELD — the n-dimensional carrier** (operator ruling
//! 2026-08-04: *remove `Grid` entirely and upgrade to larger
//! n-dimensional capacity*).
//!
//! `Grid` was rank 2 and `Volume` was rank 3, and they were separate
//! types with separate vocabularies, separate forges, separate
//! certificates and separate wire schemas — the whole apparatus built
//! twice because the rank was baked into the carrier. A `Field` carries
//! its shape as data, so rank 4 costs nothing that rank 2 did not
//! already pay.
//!
//! **THE VOCABULARY COLLAPSES, and that is the point.** Every operator
//! the two carriers had turns out to be an AXIS operation, and once the
//! axis is a parameter the duplicates disappear:
//!
//! ```text
//!   Grid::FlipH        ReverseAxis(1)      mirror the columns
//!   Grid::FlipV        ReverseAxis(0)      mirror the rows
//!   Grid::Transpose    SwapAxes(0, 1)
//!   Grid::Rotate90     SwapAxes(0,1) ∘ ReverseAxis(1)   — a COMPOSITION,
//!                                          never a primitive
//!   Volume::FlipStack  ReverseAxis(0)      on a rank-3 field: the SAME
//!                                          operator as FlipV, one axis out
//!   Volume::CycleLayers RollAxis(0, 1)
//! ```
//!
//! `FlipStack` and `FlipV` were the same operation written twice
//! because one lived in a 2-D type and the other in a 3-D one. That
//! duplication is what "we aren't building a matrix" names.
//!
//! **The hue law is the carrier's, as it was for `Grid`.** A hue past
//! the ladder is not clamped — it is not a field. Two operators index
//! ten-slot tables by hue, and out-of-range there is an out-of-bounds
//! panic inside the gate.

/// The lawful hue ladder: 0–9 inclusive. Wire law, not a preference.
pub const HUES: u8 = 10;

/// An n-dimensional field of hues, row-major over `shape`.
///
/// Rank is `shape.len()`: 2 is a grid, 3 a volume, and nothing in here
/// knows or cares which.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    shape: Vec<usize>,
    cells: Vec<u8>,
}

impl Field {
    /// THE ONE DOOR. A field has at least one axis, every axis has
    /// extent, the cell count is the product of the shape, and every
    /// hue sits on the ladder.
    ///
    /// Rank 0 is refused deliberately: a shapeless field is the
    /// degenerate arm that makes "every cell" vacuously true, the same
    /// vacuity class already struck across this crate.
    pub fn new(shape: Vec<usize>, cells: Vec<u8>) -> Option<Field> {
        if shape.is_empty() || shape.contains(&0) {
            return None;
        }
        let expected = shape.iter().try_fold(1usize, |n, &e| n.checked_mul(e))?;
        if cells.len() != expected {
            return None;
        }
        if cells.iter().any(|&hue| hue >= HUES) {
            return None;
        }
        Some(Field { shape, cells })
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// How many axes this field stands on.
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Row-major offset of a coordinate. `None` if the coordinate is
    /// not in the field — out of range, or the wrong rank entirely.
    pub fn offset(&self, at: &[usize]) -> Option<usize> {
        if at.len() != self.shape.len() {
            return None;
        }
        let mut offset = 0usize;
        for (axis, &coordinate) in at.iter().enumerate() {
            if coordinate >= self.shape[axis] {
                return None;
            }
            offset = offset * self.shape[axis] + coordinate;
        }
        Some(offset)
    }

    /// The hue at a coordinate.
    pub fn at(&self, coordinate: &[usize]) -> Option<u8> {
        self.cells.get(self.offset(coordinate)?).copied()
    }

    /// Walk every coordinate in row-major order.
    fn coordinates(&self) -> impl Iterator<Item = Vec<usize>> + '_ {
        let total = self.cells.len();
        (0..total).map(move |mut offset| {
            let mut at = vec![0usize; self.shape.len()];
            for axis in (0..self.shape.len()).rev() {
                at[axis] = offset % self.shape[axis];
                offset /= self.shape[axis];
            }
            at
        })
    }

    /// Build a field of `shape` by reading each coordinate from `f`.
    /// Total by construction: `f` is asked only for coordinates that
    /// exist.
    fn from_shape(shape: Vec<usize>, f: impl Fn(&[usize]) -> u8) -> Field {
        let scratch = Field {
            cells: vec![0u8; shape.iter().product::<usize>()],
            shape,
        };
        let cells: Vec<u8> = scratch.coordinates().map(|at| f(&at)).collect();
        Field { shape: scratch.shape, cells }
    }
}

/// The axis vocabulary — general operators over any rank.
///
/// Every one of these is total: an operator naming an axis the field
/// does not have leaves the field UNCHANGED rather than refusing, the
/// same law the object ops already followed ("an inapplicable read
/// renders the field unchanged — the verifier rejects useless words by
/// failing to close, never by crashing").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    /// Mirror one axis. Rank 2 axis 0 is `FlipV`, axis 1 is `FlipH`;
    /// rank 3 axis 0 is what `Volume` called `FlipStack`.
    Reverse(usize),
    /// Exchange two axes. Rank 2 `(0, 1)` is `Transpose`.
    Swap(usize, usize),
    /// Rotate the coordinates along one axis by `by`, wrapping. Rank 3
    /// `(0, 1)` is what `Volume` called `CycleLayers`.
    Roll(usize, usize),
    /// A hue permutation. Rank-independent — it never touches shape.
    Recolor([u8; HUES as usize]),
}

impl Axis {
    pub fn describe(&self) -> String {
        match self {
            Axis::Reverse(axis) => format!("reverse({axis})"),
            Axis::Swap(a, b) => format!("swap({a},{b})"),
            Axis::Roll(axis, by) => format!("roll({axis},{by})"),
            Axis::Recolor(map) => {
                let m: Vec<String> = map.iter().map(|h| h.to_string()).collect();
                format!("recolor[{}]", m.join(","))
            }
        }
    }

    /// Apply — exact, total, shape-aware.
    pub fn apply(&self, field: &Field) -> Field {
        match self {
            Axis::Reverse(axis) => {
                if *axis >= field.rank() {
                    return field.clone();
                }
                let axis = *axis;
                let shape = field.shape.clone();
                Field::from_shape(shape.clone(), |at| {
                    let mut from = at.to_vec();
                    from[axis] = shape[axis] - 1 - at[axis];
                    field.at(&from).unwrap_or(0)
                })
            }
            Axis::Swap(a, b) => {
                if *a >= field.rank() || *b >= field.rank() {
                    return field.clone();
                }
                let (a, b) = (*a, *b);
                let mut shape = field.shape.clone();
                shape.swap(a, b);
                Field::from_shape(shape, |at| {
                    let mut from = at.to_vec();
                    from.swap(a, b);
                    field.at(&from).unwrap_or(0)
                })
            }
            Axis::Roll(axis, by) => {
                if *axis >= field.rank() {
                    return field.clone();
                }
                let axis = *axis;
                let shape = field.shape.clone();
                let extent = shape[axis];
                let by = by % extent;
                Field::from_shape(shape.clone(), |at| {
                    let mut from = at.to_vec();
                    // Roll FORWARD: the cell at `k` comes from `k - by`,
                    // wrapping. `+ extent` keeps the arithmetic in
                    // `usize` without a signed detour.
                    from[axis] = (at[axis] + extent - by) % extent;
                    field.at(&from).unwrap_or(0)
                })
            }
            Axis::Recolor(map) => Field {
                shape: field.shape.clone(),
                cells: field.cells.iter().map(|&hue| map[hue as usize]).collect(),
            },
        }
    }
}

/// A word over the axis vocabulary, applied left to right.
pub type Word = Vec<Axis>;

/// THE ONE APPLY PATH, at any rank.
pub fn run(word: &Word, input: &Field) -> Field {
    let mut field = input.clone();
    for axis in word {
        field = axis.apply(&field);
    }
    field
}

/// A quarter turn in the plane of two axes, as a COMPOSITION.
///
/// `Rotate90` was a primitive when the carrier was a matrix. It is not
/// primitive — it is a swap and a mirror — and saying so is the
/// difference between a vocabulary that generalizes and one that has to
/// be re-listed for every rank.
pub fn quarter_turn(a: usize, b: usize) -> Word {
    vec![Axis::Swap(a, b), Axis::Reverse(b)]
}
