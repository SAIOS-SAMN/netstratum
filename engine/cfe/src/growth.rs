//! GROWTH-1 — the six growth mechanisms (operator ruling 2026-08-03).
//!
//! *"How do we go from primordialism to significant civilization without
//! leaps and bounds and skipping everything in between?"*
//!
//! The answer this module encodes: **a rung must measure something the
//! chain PRODUCES, never something a line ASSERTS.** `26_ARTICLES`
//! ARTICLE IV clause 2 rules that this manifold is one commitment per
//! step, so a stamp IS elapsed history. A claim can be typed; a span
//! cannot be forged.
//!
//! ```text
//!     THE TEST: can a line reach this rung at stamp 0?
//!               If yes, it is a leap.
//! ```
//!
//! # THE SIX AXES, and they are orthogonal
//!
//! ```text
//!     PRACTICE     TIME       did the doing span real chain time?
//!     KNOWLEDGE    BREADTH    how many distinct accounts are kept?
//!     EXPERTISE    DEPTH      how concentrated is the deepest one?
//!     WISDOM       TESTING    how much of it has been tested?
//!     LOCALITY     SCOPE      how much of the line's record sits here?
//!     TRIVIALITY   EMPTINESS  how much of it moved nothing?
//! ```
//!
//! **WISDOM is the one worth explaining.** It reads the share of an
//! orb's accounts carrying BOTH sides — kept AND lost. A line that has
//! only ever posted wins has recorded a great deal and learned nothing;
//! the dual ledger is what makes that measurable, and it is why BOOK-1
//! made both sides structural rather than conventional.
//!
//! **TRIVIALITY is the guard that makes the other five honest.** Without
//! it, every axis is farmable: a thousand empty postings would read as
//! practice, breadth and depth alike. It reports how much of an apparent
//! record moved nothing at all, and a reader that ignores it is reading
//! a padded book.
//!
//! # NO THRESHOLDS LIVE HERE
//!
//! Every field is an exact rational or a count, derived and reported
//! whole. Nothing in this module compares against a number, so there is
//! nothing here to dial (operator ruling 2026-08-03: constants and
//! statics give way to deterministic throughput driven by dynamism).
//!
//! Where a gate needs one of these it compares against ZERO — the
//! distinction between *happened* and *did not*, which is not a dial.
//! `World::literate` is the first: a line reads and writes when its
//! postings SPAN time, not when one posting exists.

use crate::ratio::{ratio, Ratio};

/// **THE GROWTH READING of one orb.** Derived from the chronicle and the
/// books, held by nobody, recomputed whenever asked.
///
/// Six quantities carried whole. No verdict, no score, no aggregate: a
/// reader learns how long, how wide, how deep, how tested, how local
/// and how empty — and draws its own conclusion. Collapsing them to one
/// number is the struck C-gate wearing a sixth name.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Growth {
    /// TIME — the span the postings cover, against the age of the line.
    /// Zero when everything happened at one stamp, which is exactly the
    /// leap this axis exists to expose.
    pub practice: Ratio,
    /// BREADTH — how many distinct accounts have been posted to. A
    /// COUNT, because breadth has no natural ceiling to divide by.
    pub knowledge: usize,
    /// DEPTH — the deepest account's share of all this orb's book
    /// weight. One account carrying everything reads 1.
    pub expertise: Ratio,
    /// TESTING — the share of posted accounts carrying BOTH sides. A
    /// line that only ever posted wins has learned nothing.
    pub wisdom: Ratio,
    /// SCOPE — the share of the whole LINE's book weight that sits on
    /// this orb. A single-orb line reads 1: wholly local.
    pub locality: Ratio,
    /// EMPTINESS — the share of postings that moved nothing at all.
    /// The guard that keeps the other five from being farmed.
    pub triviality: Ratio,
}

impl Growth {
    /// Has this line practised at all — did its writing span any chain
    /// time? Compared against ZERO, which is the difference between
    /// happened and did not, never a dial.
    pub fn practised(&self) -> bool {
        self.practice > ratio(0, 1)
    }

    /// Has anything been recorded that moved something? A record that is
    /// wholly trivial is an empty record however long it is.
    pub fn substantive(&self) -> bool {
        self.knowledge > 0 && self.triviality < ratio(1, 1)
    }
}

/// A safe exact share. `part / whole`, and zero when the whole is zero —
/// a share of nothing is nothing, never a division.
pub(crate) fn share(part: Ratio, whole: Ratio) -> Ratio {
    match whole == ratio(0, 1) {
        true => ratio(0, 1),
        false => part / whole,
    }
}
