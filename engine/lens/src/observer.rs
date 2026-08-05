//! THE OBSERVER — how deep it may see, and what it reads as resonant
//! (`19_LENS` §4 and §5, tasks 7 and 9).
//!
//! Two properties of whoever is in the seat, and they turn out to be one
//! mechanism seen twice.
//!
//! # CAPACITY BOUNDS DEPTH — lawfully, not for performance
//!
//! The ancestor's rule (`orbital/kinetic.rs`, LAW-13):
//!
//! > *the entity cannot see deeper than its capacity allows, regardless
//! > of pressure — the effective limit is `min(requested, κ)`.*
//!
//! That makes level-of-detail a **property of the observer** rather than
//! a frame budget, so a lens can never be pressured into rendering more
//! than the observer has standing to see. An observable universe renders
//! only what is observed, at only the depth it is observed at.
//!
//! **LAZY, NEVER MEMOISED** (§4). The bound reduces WHAT IS COMPUTED. It
//! caches nothing, because a cache is a second image
//! (`00_CHARTER` §3a) and would arrive dressed as an optimization
//! exactly the way the excised dashboard did.
//!
//! # THE PEEL IS THE TOLERANCE
//!
//! Resonance asks whether two structures are the same. The answer
//! depends on how closely you are looking, and that is the whole of it:
//!
//! ```text
//!     FAR    compare CLASSES          discrete equality; no numeric
//!                                     tolerance exists at this depth
//!     MID    compare CIRCULATION      relative, cross-multiplied
//!     NEAR   compare EXACT ENTRIES    tolerance ZERO — byte-identity
//! ```
//!
//! **To loosen the tolerance IS to step back; to tighten it IS to
//! approach.** So the tolerance is DYNAMIC — as ruled — and it is
//! DERIVED rather than DIALED. A caller-supplied `ε` would make two
//! observers report different captures from one state, which is not
//! evidence (`26_ARTICLES` LAW 6). A depth is a position, and every
//! observer standing in the same place reads the same thing.
//!
//! A fixed absolute tolerance would be worse than a dial: the substrate
//! is scale-blind, so one `ε` means *everything captures* at one
//! magnitude and *nothing captures* at another. The relative form at MID
//! is written cross-multiplied so it stays exact, division-free, and
//! scales with what it compares.

use crate::peel::{peel, Depth, View};
use cfe::ratio::{ratio, Ratio};
use num_traits::Signed;
use rfa_core::{Delta, Support};

/// How deep this observer may see. The ancestor's `κ`.
///
/// Not a quality setting and not a budget — a standing. An observer with
/// a shallow capacity is not being throttled; it genuinely cannot
/// resolve further.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Capacity(usize);

impl Capacity {
    /// The floor: every observer can at least read the class. Nothing
    /// that exists is invisible to everything.
    pub const GROUND: Capacity = Capacity(0);

    /// An observer that can descend `rungs` below the far view.
    pub fn of(rungs: usize) -> Self {
        Capacity(rungs)
    }

    /// The deepest observer — resolves to the exact entries.
    pub fn full() -> Self {
        Capacity(2)
    }

    /// **`min(requested, κ)`.** What this observer actually gets when it
    /// asks for `requested`. A request beyond standing is not refused —
    /// it is answered at the depth the observer can hold.
    pub fn admits(&self, requested: Depth) -> Depth {
        let asked = match requested {
            Depth::Far => 0,
            Depth::Mid => 1,
            Depth::Near => 2,
        };
        match asked.min(self.0) {
            0 => Depth::Far,
            1 => Depth::Mid,
            _ => Depth::Near,
        }
    }

    /// Was the request bounded by standing rather than granted?
    pub fn bounds(&self, requested: Depth) -> bool {
        self.admits(requested) != requested
    }
}

/// **RENDER ON DEMAND.** Read at the depth requested, bounded by what
/// the observer can hold.
///
/// The bound is applied BEFORE any work is done, so a request for the
/// near view from a far-sighted observer computes the far view and
/// nothing else. That is the laziness: unobserved depths cost zero
/// operations, and nothing is retained between calls.
pub fn observe(
    delta: &Delta,
    support: &Support,
    requested: Depth,
    capacity: &Capacity,
) -> View {
    peel(delta, support, capacity.admits(requested))
}

/// What a resonance reading found. Three arms, each carrying its
/// measurement whole — the reader introspects rather than being handed
/// a verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capture {
    /// The two structures resonate at this depth.
    Captured {
        /// The depth the reading was taken at — the tolerance, named.
        at: Depth,
    },
    /// They do not, and the reading says by how much.
    Mismatch {
        at: Depth,
        /// The separation found, carried whole.
        separation: Ratio,
    },
    /// Neither structure carries anything to compare at this depth.
    /// Not a mismatch — an absence of subject.
    Silent { at: Depth },
}

/// **RESONANCE.** Do these two structures read as one, at this depth?
///
/// The depth IS the tolerance (see the module header). Nothing is
/// dialed, nothing is configured, and two observers standing in the same
/// place always agree.
pub fn resonance(
    a: &Delta,
    b: &Delta,
    support: &Support,
    requested: Depth,
    capacity: &Capacity,
) -> Capture {
    let at = capacity.admits(requested);
    match (peel(a, support, at), peel(b, support, at)) {
        // FAR — compare CLASSES. Two structures either stand at the same
        // elevation or they do not; there is no "nearly the same class".
        (View::Far(left), View::Far(right)) => match left == right {
            true => Capture::Captured { at },
            false => Capture::Mismatch {
                at,
                separation: ratio(
                    (left.get() as i64 - right.get() as i64).abs(),
                    1,
                ),
            },
        },

        // MID — compare CIRCULATION, relatively. The comparison is
        // cross-multiplied so it never divides and scales with what it
        // compares: a separation of 1 between charges of 2 and 3 is a
        // different fact from a separation of 1 between 2000 and 2001.
        (View::Mid(left), View::Mid(right)) => {
            let carried = |v: &[rfa_core::Holonomy]| {
                v.iter().fold(ratio(0, 1), |sum, h| sum + h.value.abs())
            };
            let (l, r) = (carried(&left), carried(&right));
            let scale = l.clone() + r.clone();
            let separation = (l - r).abs();
            match scale == ratio(0, 1) {
                // Nothing circulates on either side — no subject.
                true => Capture::Silent { at },
                // |l − r| · den ≤ num · (|l| + |r|), with the relative
                // width fixed by the depth rather than by a caller.
                false => match separation.clone() * ratio(MID_DEN, 1)
                    <= scale * ratio(MID_NUM, 1)
                {
                    true => Capture::Captured { at },
                    false => Capture::Mismatch { at, separation },
                },
            }
        }

        // NEAR — compare EXACT ENTRIES. Tolerance zero: the strictest
        // reading there is, and the one an approaching observer earns.
        (View::Near(left), View::Near(right)) => match left == right {
            true => Capture::Captured { at },
            false => {
                let sum = |v: &[crate::peel::Entry]| {
                    v.iter().fold(ratio(0, 1), |s, e| s + e.value.abs())
                };
                Capture::Mismatch { at, separation: (sum(&left) - sum(&right)).abs() }
            }
        },

        // Unreachable: both sides are peeled at the same depth.
        _ => Capture::Silent { at },
    }
}

/// The relative width at MID depth, as an exact rational `1/16`.
///
/// **This is a property of the DEPTH, not a dial.** It is fixed here
/// rather than passed in, because a caller-supplied width would make two
/// observers at the same distance disagree — which LAW 6 refuses. An
/// observer who wants a different tolerance changes DEPTH, which is a
/// position anyone can occupy and anyone can check.
const MID_NUM: i64 = 1;
const MID_DEN: i64 = 16;
