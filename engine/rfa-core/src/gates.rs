//! The validation gates — every proposed state step must pass all
//! three, and a refusal is TYPED (the gate names what failed and by
//! how much; nothing fails silently).

use crate::delta::{magnitude, Delta};
use cfe::ratio::Ratio;
use num_traits::Zero;

/// The magnitude bound for one step: the L1 mass of the proposed
/// state may not exceed `reference` by more than `epsilon × reference`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepBound {
    /// The reference magnitude (‖Δ‖₁ of the standing state, or a
    /// temporal mean the caller carries).
    pub reference: Ratio,
    /// The allowed relative excursion (exact rational, e.g. 1/10).
    pub epsilon: Ratio,
}

/// The typed verdict of one validated step. Exactly one arm; a
/// refusal carries its measurement whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateVerdict {
    /// Both gates passed.
    Accepted,
    /// Conservation refused: antisymmetry / zero-diagonal violated,
    /// with the distance to the invariant surface.
    ConservationViolated { violation: Ratio },
    /// Bounded-magnitude refused: the excursion beyond the reference
    /// exceeded the allowed band.
    MagnitudeDrift { excursion: Ratio, bound: Ratio },
}

/// Validate one proposed step `before → after`.
///
/// Gate order (first refusal wins, most structural first):
/// 1. **Conservation** — `after` must sit exactly on the invariant
///    surface (antisymmetric, zero diagonal).
/// 2. **Bounded magnitude** — `| ‖after‖₁ − reference |` must not
///    exceed `epsilon × reference`. A TWO-SIDED bound, never a
///    direction: it constrains how far a step may move, never which way.
///
/// **THE PRODUCTIVITY GATE IS STRUCK** (operator ruling 2026-08-01). It
/// read:
///
/// ```text
/// STRUCK:  productive = C(after) < C(before) || both zero
///          → else GateVerdict::Unproductive
/// ```
///
/// It refused any step that did not strictly lower `C`. The fault is
/// NOT that `C` is a scalar walking one way — a scalar IS linear and
/// uni-directional, and that is fine FOR `C`. The fault is that this
/// gate **bound the whole LINEAGE to `C`'s single axis**: it admitted
/// only motion along one coordinate in one direction, and so foreclosed
/// every other branch an n-dimensional lineage can open. That is the
/// uni-directional linear lane the foundation ruling struck, and it was
/// here in executable form while the prose around it was being
/// corrected.
///
/// Nothing replaces it. A step that does not lower `C` is not
/// unproductive — under THE BIFURCATION LAW (`17_CAPACITY` §11) its
/// non-composition is a ROOT, and capacity opens by receipt. `C` keeps
/// its existence as an inert reading and loses every gate it held: no
/// verdict of this module consults it, and `Accepted` no longer carries
/// it, because a verdict that ships a scalar invites the next reader to
/// gate on it.
pub fn validate_step(before: &Delta, after: &Delta, bound: &StepBound) -> GateVerdict {
    // `before` is retained in the signature: the two-sided magnitude
    // bound is a per-step law and callers hold the pair. Nothing about
    // the prior state gates the step's DIRECTION any more.
    let _ = before;
    let violation = after.conservation_measurement();
    if !violation.is_zero() {
        return GateVerdict::ConservationViolated { violation };
    }

    let mass = magnitude(after);
    let excursion = if mass >= bound.reference {
        &mass - &bound.reference
    } else {
        &bound.reference - &mass
    };
    let allowed = &bound.epsilon * &bound.reference;
    if excursion > allowed {
        return GateVerdict::MagnitudeDrift {
            excursion,
            bound: allowed,
        };
    }

    GateVerdict::Accepted
}
