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

/// **WHY A SHAPE HOLDS OR DOES NOT** — the proof a caller gets back
/// instead of a bare `None` (operator ruling 2026-08-05: *each call
/// should prove that the shape holds or doesn\'t hold, and what the
/// derivation is of*).
///
/// Every arm carries the numbers that decided it, so a refusal teaches
/// the caller the law rather than merely denying them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeVerdict {
    /// The shape stands, with the two divisions that admitted it.
    Holds {
        longitude: ratio::DivisionProof,
        latitude: ratio::DivisionProof,
        cells: usize,
    },
    /// The longitude arity does not divide the full ring.
    LongitudeOffSlot(ratio::DivisionProof),
    /// The latitude arity does not divide the HALVED ring. Note this
    /// refuses arities the longitude admits — 4, 12, 20 and 60 divide
    /// sixty and not thirty — which is the closed sphere\'s own 1:2.
    LatitudeOffSlot(ratio::DivisionProof),
    /// The cell count is not `bands · slots`.
    CellCountWrong { given: usize, wanted: usize },
    /// A hue stands off the ladder.
    HueOffLadder { hue: u8, ladder: u8 },
}

impl ShapeVerdict {
    pub fn holds(&self) -> bool {
        matches!(self, ShapeVerdict::Holds { .. })
    }
    /// The derivation, in words, for a caller or an operator.
    pub fn because(&self) -> String {
        match self {
            ShapeVerdict::Holds { longitude, latitude, cells } => format!(
                "holds: {} cells. longitude — {}. latitude — {}",
                cells,
                longitude.because(),
                latitude.because()
            ),
            ShapeVerdict::LongitudeOffSlot(p) => format!("longitude fails: {}", p.because()),
            ShapeVerdict::LatitudeOffSlot(p) => format!(
                "latitude fails: {} (the polar span is a HALF turn, so a band arity must \
                 divide {} — not {})",
                p.because(),
                BAND_SLOTS,
                TURN_SLOTS
            ),
            ShapeVerdict::CellCountWrong { given, wanted } => {
                format!("cell count fails: {given} given, {wanted} wanted")
            }
            ShapeVerdict::HueOffLadder { hue, ladder } => {
                format!("hue {hue} stands off a ladder of {ladder}")
            }
        }
    }
}

impl Shell {
    /// **EXAMINE A SHAPE, PROVING THE VERDICT.** The law, stated once,
    /// with its reasons attached — `new` is this plus a constructor.
    pub fn examine(bands: usize, slots: usize, cells: usize) -> ShapeVerdict {
        let longitude = ratio::divide_ring(TURN_SLOTS, slots as i64);
        if !longitude.holds() {
            return ShapeVerdict::LongitudeOffSlot(longitude);
        }
        // The polar span is a HALF turn, so a band boundary lands on a
        // slot of the halved ring. Same discreteness law, same reason:
        // a cell centre between teeth is not a place.
        let latitude = ratio::divide_ring(BAND_SLOTS, bands as i64);
        if !latitude.holds() {
            return ShapeVerdict::LatitudeOffSlot(latitude);
        }
        let wanted = bands.saturating_mul(slots);
        if cells != wanted {
            return ShapeVerdict::CellCountWrong { given: cells, wanted };
        }
        ShapeVerdict::Holds { longitude, latitude, cells }
    }

    /// THE ONE DOOR. Every admission carries the proof that admitted it.
    pub fn new(bands: usize, slots: usize, cells: Vec<u8>) -> Option<Shell> {
        Shell::examine(bands, slots, cells.len())
            .holds()
            .then_some(())?;
        if cells.iter().any(|&hue| hue >= HUES) {
            return None;
        }
        Some(Shell { bands, slots, cells })
    }

    /// The proof that admitted THIS shell — available after the fact, so
    /// a reader never has to re-derive why it stands.
    pub fn shape_proof(&self) -> ShapeVerdict {
        Shell::examine(self.bands, self.slots, self.cells.len())
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

// ─────────────────────────────────────────────────────────────────────
// THE ENTITY FRAME — base-60 orbit tied to base-11 stack
// ─────────────────────────────────────────────────────────────────────
//
// Operator ruling 2026-08-05: *tie spin to orbit and you'll get
// individual frames on base-60; extrapolate down to base-11 per entity
// in population on partition and hold structure.*
//
// Both bases were already landed law, and neither was invented here:
//
//   base-60  THE ORBITAL RING (`ratio::TURN_SLOTS`) — the SHARED frame.
//            Every orb is sited at a slot `k/60`, and the ring is the
//            one thing the whole population holds in common.
//   base-11  THE CONDITION STACK (`cfe::world::survey::LAST_CONDITION`,
//            operator ruling 2026-08-02, *uplift to base-11*) — the
//            PER-ENTITY frame. Eleven nodes: the solar leap, the five
//            crystallized conditions, the gifted crown, and four
//            chartered works.
//
// **WHY THE TIE IS EXACT: gcd(11, 60) = 1.** Sixty factors as 2²·3·5 and
// eleven is prime, so the two bases share no factor. By the Chinese
// Remainder Theorem the pair `(orbit mod 60, node mod 11)` maps to a
// single index mod 660 — and the map is a BIJECTION, so every entity in
// the population gets a frame no other entity holds. That is what
// "individual frames" means, and it is a theorem rather than a policy.
//
// **AND IT HOLDS STRUCTURE.** CRT is not merely a relabelling: it is a
// ring isomorphism, so the index of a sum is the sum of the indices.
// Advancing an orbit by one slot and advancing a stack by one node
// commute, and composing two frames composes their spins. A map that
// individuated the population without preserving addition would be a
// scramble; this one is a frame.

/// **THE STACK, DERIVED FROM ITS PARTS** (operator ruling 2026-08-05:
/// *bring these down to the theorem level*).
///
/// Base-11 was written as `11` in two crates. It is not a number: it is
/// a SUM, and the sum is the sky's own structure —
///
/// ```text
///     1  the solar leap
///     5  the crystallized conditions
///     1  the gifted crown          → 7, the BASE tier
///     4  the chartered works       → 11, the UPLIFT
/// ```
///
/// Written as the sum, a reader can see WHERE eleven comes from and a
/// change to any part carries through. Written as `11`, the day a sixth
/// condition lands someone edits one crate and not the other.
pub const SOLAR_LEAP: i64 = 1;
pub const CRYSTALLIZED_CONDITIONS: i64 = 5;
pub const GIFTED_CROWN: i64 = 1;
/// The BASE tier — what `base_seven_complete` waits for.
pub const BASE_NODES: i64 = SOLAR_LEAP + CRYSTALLIZED_CONDITIONS + GIFTED_CROWN;
/// The UPLIFT tier — one node per chartered work.
pub const CHARTERED_WORKS: i64 = 4;
/// **DERIVED: `7 + 4 = 11`.** Never typed.
pub const STACK_NODES: i64 = BASE_NODES + CHARTERED_WORKS;

/// The composite period: every distinct frame, once.
pub const FRAME_PERIOD: i64 = TURN_SLOTS * STACK_NODES;

/// **THE FRAME OF AN ENTITY** — its orbital slot tied to its stack node.
///
/// The CRT index: the unique `x` in `0..660` with `x ≡ orbit (mod 60)`
/// and `x ≡ node (mod 11)`.
///
/// The coefficients are exact and derived, not tuned: `121 = 11²` is
/// `≡ 1 (mod 60)` and `≡ 0 (mod 11)`; `540 = 60·9` is `≡ 0 (mod 60)` and
/// `≡ 1 (mod 11)`. Integer arithmetic throughout — no float, and nothing
/// to round.
pub fn frame_index(orbit_slot: i64, node: i64) -> i64 {
    (121 * orbit_slot + 540 * node).rem_euclid(FRAME_PERIOD)
}

/// **THE PROOF THE FRAME MAP RESTS ON** — returned by the call rather
/// than asserted in a comment.
///
/// The whole construction is valid iff the two bases are coprime. This
/// reports the gcd that decides it, the period that follows, and the two
/// CRT coefficients — each of which is checkable by the reader in one
/// division.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameProof {
    pub ring: i64,
    pub stack: i64,
    pub common: i64,
    pub period: i64,
    /// `≡ 1 (mod ring)`, `≡ 0 (mod stack)`.
    pub ring_carrier: i64,
    /// `≡ 0 (mod ring)`, `≡ 1 (mod stack)`.
    pub stack_carrier: i64,
}

impl FrameProof {
    /// Does the shape hold? Only if the bases share no factor.
    pub fn holds(&self) -> bool {
        self.common == 1
            && self.period == self.ring * self.stack
            && self.ring_carrier.rem_euclid(self.ring) == 1
            && self.ring_carrier.rem_euclid(self.stack) == 0
            && self.stack_carrier.rem_euclid(self.ring) == 0
            && self.stack_carrier.rem_euclid(self.stack) == 1
    }
    pub fn because(&self) -> String {
        match self.holds() {
            true => format!(
                "gcd({}, {}) = 1, so the frames are individual and the period is {}. \
                 Carriers {} (=1 mod {}, =0 mod {}) and {} (=0 mod {}, =1 mod {})",
                self.ring, self.stack, self.period,
                self.ring_carrier, self.ring, self.stack,
                self.stack_carrier, self.ring, self.stack
            ),
            false => format!(
                "gcd({}, {}) = {} — the bases SHARE a factor, so the map collapses: \
                 {} pairs would fold onto {} indices and entities would collide",
                self.ring, self.stack, self.common,
                self.ring * self.stack, self.period / self.common.max(1)
            ),
        }
    }
}

/// Prove the frame map, from the bases themselves.
pub fn frame_proof() -> FrameProof {
    FrameProof {
        ring: TURN_SLOTS,
        stack: STACK_NODES,
        common: ratio::gcd(TURN_SLOTS, STACK_NODES),
        period: FRAME_PERIOD,
        ring_carrier: 121,
        stack_carrier: 540,
    }
}

/// The orbital slot and stack node a frame index came from — the
/// inverse, and the reason the map is a bijection rather than a hash.
pub fn frame_parts(index: i64) -> (i64, i64) {
    let index = index.rem_euclid(FRAME_PERIOD);
    (index.rem_euclid(TURN_SLOTS), index.rem_euclid(STACK_NODES))
}

impl Shell {
    /// **TIE SPIN TO ORBIT.** The rotation this entity's frame presents.
    ///
    /// The frame index runs on the composite 660-ring; a shell of
    /// `slots` cells reads it at its own arity. An orb at a different
    /// orbital slot, or at a different node of its stack, sees the same
    /// shell turned differently — which is what it means for a frame to
    /// be the entity's own rather than the observer's.
    pub fn frame_turn(&self, orbit_slot: i64, node: i64) -> Turn {
        Turn::Spin(frame_index(orbit_slot, node).rem_euclid(self.slots as i64))
    }

    /// The shell as this entity sees it.
    pub fn in_frame(&self, orbit_slot: i64, node: i64) -> Shell {
        self.frame_turn(orbit_slot, node).apply(self)
    }
}
