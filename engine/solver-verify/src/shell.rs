//! **THE SHELL — a spherical carrier** (operator ruling 2026-08-05:
//! *grid needs to be discarded for spherical references*).
//!
//! A grid has PRIVILEGED AXES. `FlipH` and `FlipV` are different
//! operators only because a rectangle has a distinguished horizontal and
//! vertical; rotate the frame and they exchange. That is a basis, and
//! `26_ARTICLES` Article I — the PURE GEOMETRY MANDATE — voids readings
//! that require one: *no actor shall reduce state mappings to rigid
//! coordinate tensor ranks; static metrics are void ab initio.*
//! `27_EVIDENCE` LAW 1 says the same from the physics side: a quantity
//! that changes when you relabel is a property of the labelling.
//!
//! A sphere has no distinguished axis. Its symmetries act transitively:
//! every point is like every other, which is exactly what "frame-free"
//! means when it is a property of the carrier rather than a promise
//! about the reader.
//!
//! **AND IT IS THE UNIVERSE'S OWN FRAME.** Orbs are sited at `(r, θ, ψ)`
//! with angles as exact rationals in TURNS on a 60-slot ring
//! (`ratio::TURN_SLOTS`). The grid was borrowed from ARC; the shell is
//! native — a solver task and a star system are now addressed the same
//! way, and `separation` already measures wrapped angular distance for
//! both.
//!
//! **WHY TURNS AND NOT RADIANS.** A full turn in radians is `2π`, which
//! no rational closes, so a wrapped angle would need a float and the
//! seam would never meet. In turns the seam is exact. This carrier
//! contains no floats and no trigonometry, and it never resamples: every
//! transform is a PERMUTATION OF CELLS.
//!
//! **WHAT IS REPRESENTABLE, STATED HONESTLY.** The full rotation group
//! of a sphere is `SO(3)`, and a general rotation does NOT permute the
//! cells of a band-and-slot tessellation — it would need interpolation,
//! which needs floats, which this tree does not have. So the vocabulary
//! here is the subgroup that IS exact: rotations about the polar axis,
//! reflection through a meridian, reflection through the equator, and
//! hue permutation. That is a real group, closed and invertible, and
//! naming its boundary is better than pretending to `SO(3)` and
//! rounding.

use crate::field::HUES;
use ratio::TURN_SLOTS;

/// Hues on a spherical shell: `bands` rings of latitude, `slots` cells
/// around each.
///
/// Row-major by band then slot. `slots` must DIVIDE the 60-slot ring, so
/// every cell centre lands on a real orbital slot and a rotation by one
/// cell is a rotation by a whole number of slots — the discreteness that
/// makes collision decidable rather than a tolerance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shell {
    bands: usize,
    slots: usize,
    cells: Vec<u8>,
}

impl Shell {
    /// THE ONE DOOR. At least one band and one slot, the slot count
    /// divides the ring, the cell count is `bands · slots`, every hue on
    /// the ladder.
    pub fn new(bands: usize, slots: usize, cells: Vec<u8>) -> Option<Shell> {
        if bands == 0 || slots == 0 {
            return None;
        }
        if !(TURN_SLOTS as usize).is_multiple_of(slots) {
            return None;
        }
        if cells.len() != bands.checked_mul(slots)? {
            return None;
        }
        if cells.iter().any(|&hue| hue >= HUES) {
            return None;
        }
        Some(Shell { bands, slots, cells })
    }

    pub fn bands(&self) -> usize {
        self.bands
    }
    pub fn slots(&self) -> usize {
        self.slots
    }
    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    /// The hue at a band and slot, wrapping the slot the short way — the
    /// ring closes, so there is no out-of-range longitude.
    pub fn at(&self, band: usize, slot: i64) -> Option<u8> {
        if band >= self.bands {
            return None;
        }
        let slot = slot.rem_euclid(self.slots as i64) as usize;
        self.cells.get(band * self.slots + slot).copied()
    }

    fn build(bands: usize, slots: usize, mut f: impl FnMut(usize, usize) -> u8) -> Shell {
        let mut cells = Vec::with_capacity(bands * slots);
        for band in 0..bands {
            for slot in 0..slots {
                cells.push(f(band, slot));
            }
        }
        Shell { bands, slots, cells }
    }

    /// How many whole ring-slots one cell of this shell spans.
    pub fn slot_width(&self) -> i64 {
        TURN_SLOTS / self.slots as i64
    }
}

/// A transform of the shell. Every one is a PERMUTATION of cells (or a
/// relabelling of hues), so every one is exact, total and invertible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Turn {
    /// Rotate about the polar axis by `k` cells, wrapping. The ring
    /// closes, so this is exact at every `k` including negative ones.
    Spin(i64),
    /// Reflect through a meridian: `slot → −slot`. The handedness flip
    /// a grid spelled as `FlipH`, without needing a horizontal.
    Mirror,
    /// Reflect through the equator: `band → bands−1−band`. What a grid
    /// spelled as `FlipV`, without needing a vertical.
    Invert,
    /// A hue permutation. Frame-independent — it touches no geometry.
    Recolor([u8; HUES as usize]),
}

impl Turn {
    pub fn describe(&self) -> String {
        match self {
            Turn::Spin(k) => format!("spin({k})"),
            Turn::Mirror => "mirror".into(),
            Turn::Invert => "invert".into(),
            Turn::Recolor(map) => {
                let m: Vec<String> = map.iter().map(|h| h.to_string()).collect();
                format!("recolor[{}]", m.join(","))
            }
        }
    }

    /// Apply — exact, total, shape-preserving.
    pub fn apply(&self, shell: &Shell) -> Shell {
        match self {
            Turn::Spin(k) => {
                let k = *k;
                Shell::build(shell.bands, shell.slots, |band, slot| {
                    shell.at(band, slot as i64 - k).unwrap_or(0)
                })
            }
            Turn::Mirror => Shell::build(shell.bands, shell.slots, |band, slot| {
                shell.at(band, -(slot as i64)).unwrap_or(0)
            }),
            Turn::Invert => Shell::build(shell.bands, shell.slots, |band, slot| {
                shell.at(shell.bands - 1 - band, slot as i64).unwrap_or(0)
            }),
            Turn::Recolor(map) => Shell::build(shell.bands, shell.slots, |band, slot| {
                let hue = shell.at(band, slot as i64).unwrap_or(0) as usize;
                map.get(hue).copied().unwrap_or(0)
            }),
        }
    }
}

/// A word of turns.
pub type Ring = Vec<Turn>;

/// Apply a word, left to right.
pub fn run(word: &Ring, input: &Shell) -> Shell {
    word.iter().fold(input.clone(), |held, turn| turn.apply(&held))
}

/// **THE CLOSED VOCABULARY OF A SHELL** — every exact transform,
/// derived from the shell's own tessellation.
///
/// `slots − 1` non-identity spins, one meridian reflection, one
/// equatorial reflection. Recolor is excluded for the same reason it is
/// excluded from the field vocabulary: `10!` permutations is not a
/// fair-depth search.
pub fn vocabulary(shell: &Shell) -> Vec<Turn> {
    let mut out: Vec<Turn> = (1..shell.slots as i64).map(Turn::Spin).collect();
    out.push(Turn::Mirror);
    if shell.bands > 1 {
        out.push(Turn::Invert);
    }
    out
}
