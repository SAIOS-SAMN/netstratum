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
//! **A SPHERE DOES HAVE A DISTINGUISHED AXIS, and the first draft of
//! this header denied it** (operator correction, 2026-08-05). That was
//! false, and the code below refutes it: `Spin` rotates about the POLAR
//! axis and `Invert` reflects through the EQUATOR. There is no equator
//! without a polar axis — the equator IS the axis's fixed circle, which
//! is where the word comes from.
//!
//! The honest count is ONE distinguished axis, not two. A rectangle
//! privileges a horizontal AND a vertical, and its two mirrors are
//! independent; a sphere privileges one axis, and around it every
//! longitude is alike. That difference is what the carrier buys, and it
//! is pinned as `the_ring_has_no_origin_longitude` — a statement about
//! longitude alone, which is all it ever proved.
//!
//! **AND THE TWO AXES ARE NOT THE SAME KIND.** Longitude closes: it is a
//! full turn and a circle. Latitude does NOT close: pole to pole is a
//! HALF turn with endpoints. That 1:2 ratio is the closed sphere's own
//! shape rather than a convention, and it is why one axis has poles and
//! the other has none.
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
//! **NO 3-D DISTORTION, AND IT IS PROVABLE HERE.** A lat-long
//! tessellation embedded in Euclidean 3-space has cells of unequal AREA
//! — polar cells pinch, equatorial cells spread — and that distortion is
//! the standard objection to spherical grids. It does not arise here,
//! for a reason this tree can state exactly: **the carrier has no
//! metric.** Nothing measures an area, a length or a chord. What is
//! measured is transport around CLOSED LOOPS, which is what `rfa_core`
//! means by a holonomy — *the charge a loop carries that no frame can
//! explain*, exact and rational, no float and no root.
//!
//! Every latitude ring closes in exactly `slots` steps, at EVERY band.
//! The loop structure is band-independent, so there is no quantity that
//! could differ between a polar ring and an equatorial one. The
//! distortion is a property of an EMBEDDING, and the embedding never
//! enters. Pinned as `the_loop_structure_is_band_independent`.
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
use ratio::{ratio, slot_angle, Ratio, TURN_SLOTS};

/// **THE POLAR SPAN IS A HALF TURN.** Longitude closes at a full turn;
/// latitude runs pole to pole and stops. So the bands divide THIRTY
/// where the slots divide sixty — the same ring, halved, which is the
/// closed sphere's own 1:2 rather than a convention.
pub const BAND_SLOTS: i64 = TURN_SLOTS / 2;

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
        // The polar span is a HALF turn, so a band boundary must land on
        // a slot of the halved ring. Same discreteness law, same reason:
        // a cell centre between slots is not a place.
        if !(BAND_SLOTS as usize).is_multiple_of(bands) {
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

    /// The LATITUDE of a band as an exact turn-fraction in `[0, 1/2]` —
    /// `0` at one pole, `1/2` at the other.
    pub fn band_angle(&self, band: usize) -> Option<Ratio> {
        (band < self.bands).then(|| {
            slot_angle(band as i64 * (BAND_SLOTS / self.bands as i64)) * ratio(1, 1)
        })
    }

    /// The LONGITUDE of a slot as an exact turn-fraction in `[0, 1)`.
    pub fn slot_that(&self, slot: usize) -> Option<Ratio> {
        (slot < self.slots).then(|| slot_angle(slot as i64 * self.slot_width()))
    }

    /// **THE HOLONOMY OF A LATITUDE RING** — how many steps transport
    /// takes to close, at this band.
    ///
    /// It is `slots` at EVERY band, which is the whole no-distortion
    /// claim in one function: a polar ring and an equatorial ring are
    /// the same loop. In a Euclidean embedding the polar ring is
    /// physically shorter; the carrier has no embedding and therefore no
    /// shorter.
    pub fn ring_closure(&self, band: usize) -> Option<usize> {
        (band < self.bands).then_some(self.slots)
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
