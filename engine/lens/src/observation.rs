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

use crate::books::Regard;
use crate::luminal::{emerge, Aperture, Luminal, Phase};
use crate::observer::Capacity;
use crate::peel::{Depth, View};
use crate::propagation::{Medium, Resonance};
use crate::standing::Standing;
use cfe::dop::ColonyOrb;
use cfe::ratio::Ratio;
use rfa_core::capacity::{flux, Degree, Flux};
use rfa_core::{Delta, Support};

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
    /// Borrowed. The instrument derives from it; it is never copied.
    observer: Option<&'a ColonyOrb>,
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
            observer: None,
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

    /// **STAND AS A PARTICULAR OBSERVER.** Holds a BORROW, never a
    /// derived instrument — the regard is ground at read time from that
    /// observer's own committed books (`19_LENS` §16).
    ///
    /// Storing the ground instrument here would have been a reading
    /// retained on the position, which §1a forbids for the same reason
    /// it forbids every cache: two things that can disagree.
    pub fn seen_by(mut self, orb: &'a ColonyOrb) -> Self {
        self.observer = Some(orb);
        self
    }

    /// Set what this observer can hold.
    pub fn holding(mut self, capacity: Capacity) -> Self {
        self.capacity = capacity;
        self
    }

    /// The instrument this position looks through — the ELEVEN fixed
    /// harmonic channels of the substrate. A position parameter, read
    /// back plainly like [`Self::granted`]; it is not a reading.
    ///
    /// Books never reach it (`19_LENS` §16a): naming an observer widens
    /// its [`Self::regard`] and leaves this untouched.
    pub fn instrument(&self) -> &Aperture {
        &self.aperture
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

    /// ARM — the standing. Which row this state occupies.
    pub fn standing(&self) -> Standing {
        Standing::of(self.delta, self.support)
    }

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
            View::Near(_) => Degree::above_ground(self.standing().circulation),
        }
    }

    /// ARM — the regard. This observer's sensitivity across the OPEN
    /// set of books it keeps, derived from committed state on demand.
    ///
    /// An observer that has not been named keeps no books, and the
    /// empty regard is the correct reading of that — not a zeroed one.
    pub fn regard(&self) -> Regard {
        match self.observer {
            Some(orb) => Regard::of_orb(orb),
            None => Regard::flat(),
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
