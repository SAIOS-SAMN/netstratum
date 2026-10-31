//! The step witness: three measured verdicts over one evolution step.
//!
//! Every check derives per axis through typed arms — `Violated { axis }`
//! names where. An unaskable check reads `Unmeasured`: "could not ask"
//! is never "verified".

use crate::axis::{Axis, AxisVector};
use crate::flow::{FlowEvent, FlowState};

/// The dimensional frame survived the step (measured: the round trip
/// still reads as a three-axis flow).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramePreservation {
    Preserved,
    FrameLost,
}

/// The step's exact inverse undoes it, per axis. `Violated` names the
/// first axis that failed to return; `Unmeasured` means the frame was
/// lost and the question could not be asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InverseStanding {
    Verified,
    Violated { axis: Axis },
    Unmeasured,
}

/// Replaying the same typed event yields a byte-identical state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayIdentity {
    ByteIdentical,
    Diverged,
}

/// The composite witness — every member measured, never assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepWitness {
    pub frame: FramePreservation,
    pub inverse: InverseStanding,
    pub replay: ReplayIdentity,
    /// The tick this step was witnessed at (chronometric metadata).
    pub k_step: u64,
}

/// One axis's return verdict: from a flowing origin the content must
/// return byte-exact; from a still origin the lawful return is zero
/// content (a returned flow is not stillness).
fn axis_returned(origin: Option<&AxisVector>, after: &AxisVector) -> bool {
    match origin {
        Some(before) => after == before,
        None => after.is_zero(),
    }
}

/// Measure one evolution step: apply the event, apply its negation,
/// and read the three verdicts. A `Mint` lawfully measures
/// `Violated` — the appended plane persists through the inverse; that
/// irreversibility is the witness's teeth, not a defect.
pub fn step_witness(origin: &FlowState, event: &FlowEvent, k_step: u64) -> StepWitness {
    // The round trip.
    let mut round = origin.clone();
    round.apply(event);
    round.apply_negated(event);

    let frame = match round.psi() {
        Some(_) => FramePreservation::Preserved,
        None => FramePreservation::FrameLost,
    };

    let inverse = match round.psi() {
        None => InverseStanding::Unmeasured,
        Some(after) => {
            let origin_psi = origin.psi();
            let failed = [Axis::Spatial, Axis::Temporal, Axis::Gauge]
                .into_iter()
                .find(|&axis| !axis_returned(origin_psi.map(|p| p.axis(axis)), after.axis(axis)));
            match failed {
                None => InverseStanding::Verified,
                Some(axis) => InverseStanding::Violated { axis },
            }
        }
    };

    // Replay: the same typed event from the same origin, twice.
    let mut a = origin.clone();
    let mut b = origin.clone();
    a.apply(event);
    b.apply(event);
    let replay = if a == b {
        ReplayIdentity::ByteIdentical
    } else {
        ReplayIdentity::Diverged
    };

    StepWitness {
        frame,
        inverse,
        replay,
        k_step,
    }
}
