//! Resonance as propagation — how a volume moves through a medium
//! (`19_LENS` §5, operator correction 2026-08-02).
//!
//! **Resonance is not a match against a threshold.** It is the dynamic
//! rate and boundary function governing how a volume diverges,
//! converges, degrades and accelerates over time through the medium
//! around it. A threshold test is a photograph of something only
//! meaningful as a film.
//!
//! ```text
//!     PACE        fast / short   vs   long / hard
//!     MOTION      DIVERGING (sources)  vs  CONVERGING (sinks)
//!     MEDIUM      the accompaniment accelerates or decelerates
//!     DECAY       degradation across the separation, over steps
//! ```
//!
//! **Both poles are named** (correction 2026-08-02). Divergence sources;
//! convergence sinks; neither is the other's absence. Convergence is a
//! capability — `05 §1.4`, the dim-star system sells radiator headroom,
//! the rarest resource in a high-flux universe — so a reading that
//! renders it as negative expansion prices a capability as a loss.
//!
//! # The form is discrete, and that is not A compromise
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
//! (`26_ARTICLES` law 5). So every derivative is a difference and every
//! integral is a sum, and the physics arrives whole rather than
//! approximated:
//!
//! ```text
//!     motion         V(k) − V(k−1)       the pole, named, with its size
//!     velocity       motion per step     the step IS the unit
//!     acceleration   the second difference
//! ```
//!
//! **The pole is the content.** A rate collapsed to a magnitude says how
//! fast and throws away which way, which is the one thing a reader
//! needs. Nothing here is collapsed, and the pole lives in the arm
//! rather than in a sign.
//!
//! # What this module does not do
//!
//! It holds no history. Every reading takes the committed points it is
//! given and keeps none of them — a lens with its own timeline is a
//! second timeline (`19_LENS` §1a).

use cfe::dop::Resource;
use cfe::ratio::{ratio, Ratio};
use cfe::world::Committed;
use num_traits::Signed;
use rfa_core::capacity::Degree;

/// **The motion of A volume — both poles named, neither derived from
/// the other** (operator correction 2026-08-02, *invert to
/// convergence*).
///
/// This first read `expansion: Ratio` with a `Direction` taken from its
/// sign, and that was two faults in one field. It named a two-pole
/// reading after one pole — convergence arriving as *negative
/// expansion* — and it carried the same fact twice, since the direction
/// was fully determined by the sign.
///
/// It is the same fault corrected one file away in
/// `rfa_core::capacity`, inverted — there a pole was named for its
/// failure mode, here for its opposite. The census read
/// `Egress { sources, TRAPPED, balanced }` and became
/// `Flux { sources, sinks, balanced }`:
///
/// > *Divergence sources; convergence sinks. Both are poles of one
/// > reading and both are named.*
///
/// **Convergence is a capability, not a deficit.** `05 §1.4`: a dim-star
/// system holds the rarest resource in a high-flux universe — radiator
/// headroom — and sells it. The Frost trades convergence. A reading that
/// renders it as shrinkage prices a capability as a loss.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Motion {
    /// Diverging — the volume sources, pushing outward.
    Diverging { by: Ratio },
    /// Converging — the volume sinks, drawing inward. Named in its own
    /// right, never as an expansion that went negative.
    Converging { by: Ratio },
    /// Neither. **Not "converged"** — a volume that did not move may be
    /// at rest or may be stuck, and this reading refuses to presume.
    /// (Note the collision the correction exposed: "converged" as a
    /// synonym for *finished* is the opposite of converging as a
    /// direction. The still arm is not the convergent one.)
    Still,
}

impl Motion {
    /// How much moved, unsigned — the pole is in the arm, so the
    /// magnitude does not have to carry it.
    pub fn magnitude(&self) -> Ratio {
        match self {
            Motion::Diverging { by } | Motion::Converging { by } => by.clone(),
            Motion::Still => ratio(0, 1),
        }
    }

    /// The signed reading, for arithmetic that genuinely needs one
    /// number: diverging positive, converging negative. **A convenience
    /// at the edge, never the stored form** — storing it is what named
    /// the pole after its opposite.
    pub fn signed(&self) -> Ratio {
        match self {
            Motion::Diverging { by } => by.clone(),
            Motion::Converging { by } => -by.clone(),
            Motion::Still => ratio(0, 1),
        }
    }
}

/// How fast, in what shape of CYCLE.
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

/// The medium — the economic accompaniment a volume propagates through.
///
/// It modulates the velocity: `η > 1` accelerates, `η < 1` damps, `η = 1`
/// is inert. **Derived from the surround, never dialed** — a
/// caller-supplied `η` would make two observers read different paces
/// from one chain, which is not evidence (law 6).
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

    /// **Derived from the surround**: what stands against what is being
    /// shed. A medium that retains more than it loses accelerates; one
    /// leaking faster than it holds damps.
    ///
    /// `η = (retained + shed) / (shed + shed)` is deliberately not used
    /// — that is a normalised score. This is the plain ratio of what
    /// stands to what leaves, with an inert reading when nothing leaves.
    pub fn from_surround(retained: Ratio, shed: Ratio) -> Self {
        match shed == ratio(0, 1) {
            // Nothing is being shed: the medium takes nothing away.
            true => Medium::inert(),
            false => Medium { eta: retained / shed },
        }
    }

    /// **The medium as it actually stands** — derived from committed
    /// state, with nothing supplied by a caller (`19_LENS` §5b: *η is
    /// derived, never dialed*).
    ///
    /// The pairing is the thermal law's own, and it was already in the
    /// tree waiting to be read (`05 §1.2`, encase-first / vent-overflow):
    ///
    /// ```text
    ///     RETAINED   banked Ember + banked Sink — what encasement kept
    ///     SHED       entropy_vented — what overflowed to the void
    /// ```
    ///
    /// A world that keeps more than it loses accelerates what propagates
    /// through it; one leaking faster than it holds damps. Both are
    /// facts about the world rather than arguments about it, so two
    /// observers reading the same chain read the same medium — which is
    /// what law 6 requires and what a passed-in `η` could never give.
    ///
    /// A world that has vented nothing is inert, not infinitely fast:
    /// the surround has taken nothing away yet.
    pub fn of(state: &Committed) -> Self {
        let zero = ratio(0, 1);
        let retained = state.orbs.iter().fold(zero.clone(), |sum, orb| {
            sum + orb.banked_amount(Resource::Ember) + orb.banked_amount(Resource::Sink)
        });
        let shed = state
            .orbs
            .iter()
            .fold(zero, |sum, orb| sum + orb.entropy_vented.clone());
        Medium::from_surround(retained, shed)
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

/// A resonance reading — the propagation of one volume across a span.
///
/// Four quantities carried whole. No verdict, no threshold, no score:
/// a reader learns which way, how fast, how hard the medium pushed, and
/// how much was lost — and draws its own conclusion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resonance {
    /// `V(k) − V(k−1)` — the pole named, carrying its own size. Not a
    /// signed scalar: convergence is its own arm, never a negative.
    pub motion: Motion,
    /// How fast, against the span it took.
    pub pace: Pace,
    /// The motion after the medium has had its effect, signed for
    /// arithmetic. Larger in magnitude where the medium accelerates.
    pub modulated: Ratio,
    /// What was lost across this span — decay consumed, carried by the
    /// separation. Never subtracted from the motion; a reader
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
        // The derivative is the difference — the step is the unit. The
        // pole is decided once, here, and carried in the arm; nothing
        // downstream re-derives it from a sign.
        let change = ratio(now.get() as i64, 1) - ratio(origin.get() as i64, 1);
        let motion = match change.is_positive() {
            true => Motion::Diverging { by: change.clone() },
            false => match change.is_negative() {
                true => Motion::Converging { by: change.abs() },
                false => Motion::Still,
            },
        };

        // Fast is a large change over few steps; long is a small change
        // over many. Compared cross-multiplied so it stays exact and
        // scales with the span rather than against a fixed cut.
        let span = ratio(steps.max(1) as i64, 1);
        let pace = match motion {
            Motion::Still => Pace::Still,
            _ => match motion.magnitude() > span {
                true => Pace::Fast,
                false => Pace::Long,
            },
        };

        Resonance {
            modulated: motion.signed() * medium.eta().clone(),
            motion,
            pace,
            degradation: decay_consumed * distance,
        }
    }

}

/// Alignment — the static depth-indexed match, renamed to what it does.
///
/// **This is not resonance** (`19_LENS` §5). It answers *"do these two
/// read the same at this distance?"*, which is a useful question and a
/// different one. Resonance is [`Resonance`]: a propagation across a
/// span, not a comparison at an instant.
pub use crate::observer::Capture as Alignment;
