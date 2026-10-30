//! Netstratum physics: event-driven exact kinematics on the RFA carrier.
//!
//! Ported clean-room from the v0.3 kinematic organ (`saios::physics`),
//! re-expressed on the CFE `Ratio` carrier with grounded engineering
//! vocabulary. The laws carried whole:
//!
//! - **No calculus.** All dynamics are finite exact recurrences:
//!   `Ψ_{k+1} = Ψ_k + 𝒟_T Ψ_k`, read per typed event. No scheduler,
//!   no frame queue — the accumulated state is the carrier.
//! - **Every reversible step carries its exact inverse.**
//!   `apply ∘ apply_negated = identity`, byte-exact.
//! - **Value creation is irreversible by construction.** A mint appends
//!   new planes the additive inverse can never reach.
//! - **Stillness is typed.** A flow that never fired is `Still` — a
//!   different fact from a flow whose motion cancelled to zero.

pub mod axis;
pub mod beacon;
pub mod breath;
pub mod flow;
pub mod witness;

pub use axis::{Axis, AxisVector, Triple};
pub use beacon::Beacon;
pub use breath::{exhale, inhale, BreathError, Exhaled};
pub use flow::{AxisFlow, CapacityRead, FlowEvent, FlowReading, FlowState, Shear};
pub use witness::{step_witness, FramePreservation, InverseStanding, ReplayIdentity, StepWitness};
