//! RESONANCE AS PROPAGATION — how a volume moves through a medium
//! (`19_LENS` §5, operator correction 2026-08-02).
//!
//! **Resonance is not a match against a threshold.** It is the dynamic
//! rate and boundary function governing how an ORIGINATING VOLUME
//! expands, contracts, degrades and accelerates over time through the
//! medium around it. A threshold test is a photograph of something only
//! meaningful as a film.
//!
//! ```text
//!     PACE        fast / short   vs   long / hard
//!     DIRECTION   outward expansion   vs   inward contraction
//!     MEDIUM      the accompaniment accelerates or decelerates
//!     DECAY       degradation from the origin, over steps and distance
//! ```
//!
//! # THE FORM IS DISCRETE, AND THAT IS NOT A COMPROMISE
//!
//! The relayed physics is continuous:
//!
//! ```text
//!     V(t) = V0 · exp( ∫ (expand − contract) dt )
//!     a(t) = d/dt ( velocity · η )
//! ```
//!
//! `exp` is transcendental and `∫` / `d/dt` are continuous. Neither is
//! admissible: exactness is a type-level guarantee here, and **the chain
//! is discrete — one commitment per step, which is `c` itself**
//! (`18_TORSION` LAW 5). So every derivative is a DIFFERENCE and every
//! integral is a SUM, and the physics arrives whole rather than
//! approximated:
//!
//! ```text
//!     expansion      V(k) − V(k−1)          direction preserved
//!     velocity       expansion per step     the step IS the unit
//!     acceleration   the second difference
//! ```
//!
//! **Direction is the content.** A rate collapsed to a magnitude says
//! how fast and throws away whether the volume was growing or dying,
//! which is the one thing a reader needs. Nothing here is collapsed.
//!
//! # WHAT THIS MODULE DOES NOT DO
//!
//! It holds no history. Every reading takes the committed points it is
//! GIVEN and keeps none of them — a lens with its own timeline is a
//! second timeline (`19_LENS` §1a).

use cfe::ratio::{ratio, Ratio};
use num_traits::Signed;
use rfa_core::capacity::Degree;

/// WHICH WAY A VOLUME IS MOVING. The sign of the motion, kept as a
/// fact rather than folded into a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Growing — the volume pushes outward.
    Outward,
    /// Shrinking — the volume is drawn inward.
    Inward,
    /// Neither. **Not "converged"** — a volume that did not move may be
    /// at rest or may be stuck, and this reading refuses to presume.
    Still,
}

/// HOW FAST, IN WHAT SHAPE OF CYCLE.
///
/// Pace is read against the span it took, because the same change over
/// one step and over a hundred are different events entirely: the first
/// is a shock, the second is a drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pace {
    /// Large change over few steps — fast and short.
    Fast,
    /// Small change over many steps — long and hard.
    Long,
    /// Nothing moved over the span.
    Still,
}

/// THE MEDIUM — the economic accompaniment a volume propagates through.
///
/// It modulates the velocity: `η > 1` accelerates, `η < 1` damps, `η = 1`
/// is inert. **Derived from the surround, never dialed** — a
/// caller-supplied `η` would make two observers read different paces
/// from one chain, which is not evidence (LAW 6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Medium {
    /// What the surround retains against what it sheds. Exact.
    eta: Ratio,
}

impl Medium {
    /// The inert medium — neither helps nor hinders.
    pub fn inert() -> Self {
        Medium { eta: ratio(1, 1) }
    }

    /// **DERIVED from the surround**: what stands against what is being
    /// shed. A medium that retains more than it loses accelerates; one
    /// bleeding faster than it holds damps.
    ///
    /// `η = (retained + shed) / (shed + shed)` is deliberately NOT used
    /// — that is a normalised score. This is the plain ratio of what
    /// stands to what leaves, with an inert reading when nothing leaves.
    pub fn from_surround(retained: Ratio, shed: Ratio) -> Self {
        match shed == ratio(0, 1) {
            // Nothing is being shed: the medium takes nothing away.
            true => Medium::inert(),
            false => Medium { eta: retained / shed },
        }
    }

    /// Does this medium speed the motion, damp it, or leave it alone?
    pub fn accelerates(&self) -> bool {
        self.eta > ratio(1, 1)
    }

    pub fn decelerates(&self) -> bool {
        self.eta < ratio(1, 1)
    }

    pub fn eta(&self) -> &Ratio {
        &self.eta
    }
}

/// A RESONANCE READING — the propagation of one volume across a span.
///
/// Four quantities carried whole. No verdict, no threshold, no score:
/// a reader learns which way, how fast, how hard the medium pushed, and
/// how much was lost — and draws its own conclusion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resonance {
    /// `V(k) − V(k−1)`, signed. The whole motion, direction intact.
    pub expansion: Ratio,
    /// Which way.
    pub direction: Direction,
    /// How fast, against the span it took.
    pub pace: Pace,
    /// The expansion after the medium has had its effect: `expansion · η`.
    /// Larger than the bare expansion where the medium accelerates.
    pub modulated: Ratio,
    /// What the origin lost over this span — decay consumed, carried
    /// away by distance. Never subtracted from the expansion; a reader
    /// that wants the net can take it, and one that wants both has both.
    pub degradation: Ratio,
}

impl Resonance {
    /// Read the propagation of a volume from `origin` to `now` across
    /// `steps` chain commitments, through `medium`.
    ///
    /// `decay_consumed` is what the origin shed over the span and
    /// `distance` is how far the reading is taken from it (wrapped L1
    /// elsewhere in the tree); their product is the degradation field.
    ///
    /// Pure and total. Takes the two committed points it is given and
    /// keeps neither.
    pub fn across(
        origin: Degree,
        now: Degree,
        steps: u64,
        medium: &Medium,
        decay_consumed: Ratio,
        distance: Ratio,
    ) -> Self {
        // The derivative IS the difference — the step is the unit.
        let expansion = ratio(now.get() as i64, 1) - ratio(origin.get() as i64, 1);

        let direction = match expansion.is_positive() {
            true => Direction::Outward,
            false => match expansion.is_negative() {
                true => Direction::Inward,
                false => Direction::Still,
            },
        };

        // FAST is a large change over few steps; LONG is a small change
        // over many. Compared cross-multiplied so it stays exact and
        // scales with the span rather than against a fixed cut.
        let magnitude = expansion.abs();
        let span = ratio(steps.max(1) as i64, 1);
        let pace = match direction {
            Direction::Still => Pace::Still,
            _ => match magnitude.clone() > span {
                true => Pace::Fast,
                false => Pace::Long,
            },
        };

        Resonance {
            modulated: expansion.clone() * medium.eta().clone(),
            expansion,
            direction,
            pace,
            degradation: decay_consumed * distance,
        }
    }

    /// Did the medium change what the motion amounted to?
    pub fn medium_mattered(&self) -> bool {
        self.modulated != self.expansion
    }
}

/// ALIGNMENT — the static depth-indexed match, renamed to what it does.
///
/// **This is not resonance** (`19_LENS` §5). It answers *"do these two
/// read the same at this distance?"*, which is a useful question and a
/// different one. Resonance is [`Resonance`]: a propagation across a
/// span, not a comparison at an instant.
pub use crate::observer::Capture as Alignment;
