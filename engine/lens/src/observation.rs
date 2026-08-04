//! THE OBSERVATION — a POSITION, from which every arm is reachable
//! (`19_LENS` §15, operator correction 2026-08-02).
//!
//! **Reading is not uni-directional.** The lane's first shape was one
//! arrow — chain, through an arm, to a display — and each arm was wired
//! separately toward the same linear end. That is a journal: digital
//! paper, written once, read front to back. A functioning system does
//! not read that way. **A reading may be entered from ANY ARM**, and the
//! arms are related rather than queued.
//!
//! So an observation is not a record produced one way. It is a POSITION:
//! a depth, a phase, an aperture and a standing, held together — and
//! from that one position every arm derives on demand:
//!
//! ```text
//!                            ┌──────────────┐
//!                    ┌───────┤  STANDING    ├───────┐
//!                    │       └──────────────┘       │
//!            ┌───────┴──────┐              ┌─────────┴────┐
//!       ─────┤   THE PEEL   ├──────────────┤    FLUX      ├─────
//!            └───────┬──────┘   OBSERVATION└─────────┬────┘
//!                    │        (depth · phase ·       │
//!            ┌───────┴──────┐  aperture · capacity)  │
//!            │   LUMINAL    ├────────────────────────┘
//!            └──────────────┘
//!
//!        no arm is the entrance; every arm is reachable from any other
//! ```
//!
//! # WHY THIS IS NOT A RECORD
//!
//! An `Observation` holds **borrowed** references to the relation and
//! its observation record, plus four scalars naming where the observer
//! stands. It holds **no reading**. Every arm is computed when asked and
//! discarded after — so the type cannot become a second image
//! (`19_LENS` §1a) and cannot become a cache (§4), because there is
//! nowhere in it to put one.
//!
//! That is also what makes the multi-directionality free rather than
//! expensive: asking for one arm computes exactly one arm. Standing at a
//! position costs nothing until a question is asked of it.
//!
//! # WHAT IT STILL MAY NOT DO
//!
//! Compose all it likes — it never gates. An observation answers
//! questions and decides nothing, and a caller that branches on one has
//! made a reading load-bearing.

use crate::luminal::{emerge, Aperture, Luminal, Phase};
use crate::observer::Capacity;
use crate::peel::{Depth, View};
use crate::propagation::{Medium, Resonance};
use cfe::ratio::Ratio;
use rfa_core::capacity::{flux, Degree, Flux};
use rfa_core::{holonomies, Delta, Support};

/// WHERE AN OBSERVER STANDS, and everything reachable from there.
///
/// Borrowed, never owning. Constructing one is free; every arm below
/// derives when asked.
#[derive(Debug, Clone)]
pub struct Observation<'a> {
    delta: &'a Delta,
    support: &'a Support,
    depth: Depth,
    phase: Phase,
    aperture: Aperture,
    capacity: Capacity,
}

impl<'a> Observation<'a> {
    /// Stand somewhere. The default position: as deep as standing
    /// allows, head-on, through an untuned instrument.
    pub fn at(delta: &'a Delta, support: &'a Support) -> Self {
        Observation {
            delta,
            support,
            depth: Depth::Far,
            phase: Phase::direct(),
            aperture: Aperture::flat(),
            capacity: Capacity::full(),
        }
    }

    /// Move closer or further. The depth is still bounded by standing —
    /// a position may be requested and is answered at what the observer
    /// can hold.
    pub fn depth(mut self, depth: Depth) -> Self {
        self.depth = depth;
        self
    }

    /// Turn the angle.
    pub fn phase(mut self, phase: Phase) -> Self {
        self.phase = phase;
        self
    }

    /// Change instrument.
    pub fn through(mut self, aperture: Aperture) -> Self {
        self.aperture = aperture;
        self
    }

    /// Set what this observer can hold.
    pub fn holding(mut self, capacity: Capacity) -> Self {
        self.capacity = capacity;
        self
    }

    /// The depth actually granted — `min(requested, κ)`.
    pub fn granted(&self) -> Depth {
        self.capacity.admits(self.depth)
    }

    // ── THE ARMS. Each derives on demand; none is privileged. ─────────

    /// ARM — the peel. What the relation is made of, at this depth.
    pub fn view(&self) -> View {
        crate::observer::observe(self.delta, self.support, self.depth, &self.capacity)
    }

    // THE STANDING ARM IS GONE (operator ruling 2026-08-04): it
    // rendered which of four rows a state occupied, and both the rows
    // and the mint gate they fed are struck. What it uniquely offered —
    // the circulation count — is still reachable through the peel arm
    // below, so no reading was lost with it.

    /// ARM — the flux census. What sources, what sinks, what balances.
    /// **Divergence sources; convergence sinks** — both poles named.
    pub fn flux(&self) -> Flux {
        flux(self.delta)
    }

    /// ARM — the luminal. What emerges at this phase through this
    /// aperture. Reads THROUGH the peel arm, which is the composition
    /// that was there from the start.
    pub fn luminal(&self) -> Luminal {
        emerge(self.delta, &self.view(), &self.phase, &self.aperture)
    }

    /// ARM — the elevation. How high this state stands.
    ///
    /// Reachable from the peel arm, and also directly: an observer that
    /// wants only the degree does not pay for the entries.
    pub fn degree(&self) -> Degree {
        match self.view() {
            View::Far(degree) => degree,
            View::Mid(charges) => Degree::above_ground(charges.len()),
            // The circulation count, read directly rather than through
            // the struck standing arm — the same number it computed.
            View::Near(_) => {
                Degree::above_ground(holonomies(self.delta, self.support).len())
            }
        }
    }

    /// ARM — propagation. How this volume moved from a prior position,
    /// through a medium, across a span.
    ///
    /// Takes the PRIOR observation rather than a remembered one: the
    /// caller holds both committed points, and this holds neither
    /// (§1a — no private history).
    pub fn propagation_from(
        &self,
        prior: &Observation<'_>,
        steps: u64,
        medium: &Medium,
        decay_consumed: Ratio,
        separation: Ratio,
    ) -> Resonance {
        Resonance::across(
            prior.degree(),
            self.degree(),
            steps,
            medium,
            decay_consumed,
            separation,
        )
    }
}
