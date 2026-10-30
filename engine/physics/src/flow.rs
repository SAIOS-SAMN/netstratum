//! The kinematic flow organ: `Ψ_{k+1} = Ψ_k + 𝒟_T Ψ_k` as an
//! event-driven deterministic step. The organ fires on typed events
//! only — no scheduler, no window, no history buffer.

use crate::axis::{AxisVector, Triple};

/// One event's measured per-axis residual — the shear (twist amount)
/// for the reversible flow events, the seed for a value-creating mint.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Shear {
    pub spatial: AxisVector,
    pub temporal: AxisVector,
    pub gauge: AxisVector,
}

impl Shear {
    pub fn new(spatial: AxisVector, temporal: AxisVector, gauge: AxisVector) -> Self {
        Self {
            spatial,
            temporal,
            gauge,
        }
    }

    /// The exact negation, per axis — the carried inverse.
    pub fn negate(&self) -> Shear {
        Shear {
            spatial: self.spatial.negate(),
            temporal: self.temporal.negate(),
            gauge: self.gauge.negate(),
        }
    }
}

/// The typed events the flow fires on. The event KIND is part of the
/// identity — a step advance and a transfer carrying the same shear
/// are different facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowEvent {
    /// A tick advance carrying its measured shear (reversible).
    StepAdvance { shear: Shear },
    /// A measured transfer event — admission or transit (reversible).
    Transfer { shear: Shear },
    /// **The value-creating mint (irreversible by construction).**
    /// Extends the flow by new planes seeded from a residual the
    /// current frame could not capture — dimensional GROWTH, not a
    /// twist. The additive inverse aligns only with the original
    /// degrees, so a minted plane can never be un-minted.
    Mint { seed: Shear },
}

impl FlowEvent {
    fn residual(&self) -> &Shear {
        match self {
            Self::StepAdvance { shear } | Self::Transfer { shear } => shear,
            Self::Mint { seed } => seed,
        }
    }
}

/// The flow's typed reading — `Still` is a flow that has fired on no
/// event, which is not the same fact as a flow whose motion cancelled
/// to zero content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowReading<'a> {
    Still,
    Flowing { psi: &'a Triple },
}

/// Per-axis flow presence, read off the carrier's own structure —
/// never an argmax, never a magnitude.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisFlow {
    Silent,
    Carries,
}

/// Which directed axes carry flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapacityRead {
    Still,
    PerAxis {
        spatial: AxisFlow,
        temporal: AxisFlow,
        gauge: AxisFlow,
    },
}

/// The kinematic state — the accumulated carrier itself, never a frame
/// queue. `None` = still (no event has ever fired); `Some(Ψ)` = the
/// accumulated flow, whole per axis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowState {
    psi: Option<Triple>,
}

impl FlowState {
    /// Genesis — still. No zero is fabricated for "hasn't fired".
    pub fn genesis() -> Self {
        Self { psi: None }
    }

    /// The evolution step: one typed event, one exact twist per axis.
    /// The first event moves the flow from `Still` to `Flowing` — the
    /// stillness arm is consumed by measurement, never by default.
    pub fn apply(&mut self, event: &FlowEvent) {
        let residual = event.residual();
        let current = self.psi.take().unwrap_or_default();
        self.psi = Some(match event {
            FlowEvent::StepAdvance { .. } | FlowEvent::Transfer { .. } => Triple {
                spatial: current.spatial.add(&residual.spatial),
                temporal: current.temporal.add(&residual.temporal),
                gauge: current.gauge.add(&residual.gauge),
            },
            FlowEvent::Mint { .. } => Triple {
                spatial: current.spatial.extend_planes(&residual.spatial),
                temporal: current.temporal.extend_planes(&residual.temporal),
                gauge: current.gauge.extend_planes(&residual.gauge),
            },
        });
    }

    /// The exact inverse step: twist by the negated residual.
    /// For the reversible flow events `apply ∘ apply_negated =
    /// identity`, byte-exact. For a `Mint` it is not — twisting the
    /// original degrees cannot remove an appended plane, which is
    /// precisely the non-unitarity the step witness measures.
    pub fn apply_negated(&mut self, event: &FlowEvent) {
        let negated = event.residual().negate();
        self.apply(&FlowEvent::Transfer { shear: negated });
    }

    /// The typed flow reading.
    pub fn reading(&self) -> FlowReading<'_> {
        match &self.psi {
            None => FlowReading::Still,
            Some(psi) => FlowReading::Flowing { psi },
        }
    }

    /// The capacity read — presence per directed axis.
    pub fn capacity(&self) -> CapacityRead {
        match &self.psi {
            None => CapacityRead::Still,
            Some(psi) => CapacityRead::PerAxis {
                spatial: axis_flow(&psi.spatial),
                temporal: axis_flow(&psi.temporal),
                gauge: axis_flow(&psi.gauge),
            },
        }
    }

    pub(crate) fn psi(&self) -> Option<&Triple> {
        self.psi.as_ref()
    }
}

fn axis_flow(v: &AxisVector) -> AxisFlow {
    if v.is_zero() {
        AxisFlow::Silent
    } else {
        AxisFlow::Carries
    }
}
