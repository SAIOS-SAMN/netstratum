//! The validation gates — every proposed state step must pass all
//! three, and a refusal is TYPED (the gate names what failed and by
//! how much; nothing fails silently).

use crate::delta::{coherence_functional, norm_squared, Delta};
use cfe::ratio::Ratio;
use num_traits::Zero;

/// The magnitude bound for one step: the squared norm of the proposed
/// state may not exceed `reference` by more than `epsilon × reference`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepBound {
    /// The reference magnitude (‖Δ‖² of the standing state, or a
    /// temporal mean the caller carries).
    pub reference: Ratio,
    /// The allowed relative excursion (exact rational, e.g. 1/10).
    pub epsilon: Ratio,
}

/// The typed verdict of one validated step. Exactly one arm; a
/// refusal carries its measurement whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateVerdict {
    /// All three gates passed; the coherence readings ride along.
    Accepted {
        coherence_before: Ratio,
        coherence_after: Ratio,
    },
    /// Productivity refused: the step did not decrease the coherence
    /// functional (it must strictly decrease unless already zero).
    Unproductive {
        coherence_before: Ratio,
        coherence_after: Ratio,
    },
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
/// 2. **Bounded magnitude** — `| ‖after‖² − reference |` must not
///    exceed `epsilon × reference`.
/// 3. **Productivity** — the coherence functional must strictly
///    decrease, unless `before` was already fully coherent (C = 0),
///    where holding at zero is lawful.
pub fn validate_step(before: &Delta, after: &Delta, bound: &StepBound) -> GateVerdict {
    let violation = after.conservation_measurement();
    if !violation.is_zero() {
        return GateVerdict::ConservationViolated { violation };
    }

    let magnitude = norm_squared(after);
    let excursion = if magnitude >= bound.reference {
        &magnitude - &bound.reference
    } else {
        &bound.reference - &magnitude
    };
    let allowed = &bound.epsilon * &bound.reference;
    if excursion > allowed {
        return GateVerdict::MagnitudeDrift {
            excursion,
            bound: allowed,
        };
    }

    let coherence_before = coherence_functional(before);
    let coherence_after = coherence_functional(after);
    let productive = coherence_after < coherence_before
        || (coherence_before.is_zero() && coherence_after.is_zero());
    if !productive {
        return GateVerdict::Unproductive {
            coherence_before,
            coherence_after,
        };
    }

    GateVerdict::Accepted {
        coherence_before,
        coherence_after,
    }
}
