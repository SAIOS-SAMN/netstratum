//! The five pins carried from the v0.3 kinematic organ, re-expressed
//! on the netstratum carrier. Any implementation that holds all five
//! implements the same physics.

use cfe::ratio::ratio;
use num_traits::Zero;
use physics::{
    exhale, inhale, step_witness, Axis, AxisFlow, AxisVector, BreathError, CapacityRead,
    FlowEvent, FlowReading, FlowState, FramePreservation, InverseStanding, ReplayIdentity, Shear,
};

fn shear(s: i64, t: i64, g: i64) -> Shear {
    fn axis(n: i64) -> AxisVector {
        if n == 0 {
            AxisVector::empty()
        } else {
            AxisVector::from_vec(vec![ratio(n, 1)])
        }
    }
    Shear::new(axis(s), axis(t), axis(g))
}

/// PIN 1 — the exact inverse: `apply ∘ apply_negated = identity`,
/// byte-exact, for the reversible flow events.
#[test]
fn apply_then_apply_negated_is_identity() {
    let mut state = FlowState::genesis();
    state.apply(&FlowEvent::StepAdvance {
        shear: shear(2, 3, 5),
    });
    let before = state.clone();
    let event = FlowEvent::Transfer {
        shear: shear(7, 0, 11),
    };
    state.apply(&event);
    state.apply_negated(&event);
    assert_eq!(state, before, "apply ∘ apply_negated = identity, byte-exact");
}

/// PIN 2 — the value-creating mint is IRREVERSIBLE: the appended plane
/// persists through the additive inverse, while the same round trip on
/// a reversible event returns byte-exact. Only the mint creates.
#[test]
fn the_mint_extends_the_frame_irreversibly() {
    let mut state = FlowState::genesis();
    state.apply(&FlowEvent::StepAdvance {
        shear: shear(3, 5, 7),
    });
    let pre_mint = state.clone();

    let mint = FlowEvent::Mint {
        seed: shear(2, 0, 0),
    };
    state.apply(&mint);
    state.apply_negated(&mint);
    assert_ne!(
        state, pre_mint,
        "the additive inverse cannot remove a minted plane"
    );

    let mut control = pre_mint.clone();
    let flow = FlowEvent::Transfer {
        shear: shear(2, 0, 0),
    };
    control.apply(&flow);
    control.apply_negated(&flow);
    assert_eq!(
        control, pre_mint,
        "the reversible flow event returns byte-exact — only the mint creates"
    );
}

/// PIN 3 — event determinism: the same typed event sequence produces a
/// byte-identical flow. No scheduler exists, so no scheduler
/// nondeterminism can.
#[test]
fn the_same_events_produce_the_same_flow() {
    let events = [
        FlowEvent::StepAdvance {
            shear: shear(1, 2, 3),
        },
        FlowEvent::Transfer {
            shear: shear(0, 5, 0),
        },
        FlowEvent::Mint {
            seed: shear(0, 0, 4),
        },
    ];
    let mut a = FlowState::genesis();
    let mut b = FlowState::genesis();
    for e in &events {
        a.apply(e);
        b.apply(e);
    }
    assert_eq!(a, b, "same typed events ⇒ byte-identical flow");
}

/// PIN 4 — typed stillness: a flow that fired on NO event reads
/// `Still`; a flow that moved and returned reads `Flowing` with zero
/// content. Different facts, kept apart by type.
#[test]
fn stillness_is_typed_never_zero_motion() {
    let still = FlowState::genesis();
    assert_eq!(still.reading(), FlowReading::Still);
    assert_eq!(still.capacity(), CapacityRead::Still);

    let mut returned = FlowState::genesis();
    let event = FlowEvent::StepAdvance {
        shear: shear(4, 0, 0),
    };
    returned.apply(&event);
    returned.apply_negated(&event);
    match returned.reading() {
        FlowReading::Flowing { psi } => {
            assert!(psi.spatial.is_zero(), "the motion cancelled exactly")
        }
        FlowReading::Still => panic!("a returned flow is NOT stillness"),
    }
}

/// PIN 5 — the capacity read is presence off the structure: the typed
/// cells name which axes carry; no magnitude enters the read.
#[test]
fn capacity_reads_presence_per_axis() {
    let mut state = FlowState::genesis();
    state.apply(&FlowEvent::StepAdvance {
        shear: shear(9, 0, 1),
    });
    assert_eq!(
        state.capacity(),
        CapacityRead::PerAxis {
            spatial: AxisFlow::Carries,
            temporal: AxisFlow::Silent,
            gauge: AxisFlow::Carries,
        }
    );
}

/// The step witness composite: a reversible event measures
/// Preserved + Verified + ByteIdentical; a mint lawfully measures
/// `Violated` on the minted axis — the irreversibility the witness
/// exists to catch.
#[test]
fn the_step_witness_measures_all_three_verdicts() {
    let mut origin = FlowState::genesis();
    origin.apply(&FlowEvent::StepAdvance {
        shear: shear(1, 1, 1),
    });

    let reversible = step_witness(
        &origin,
        &FlowEvent::Transfer {
            shear: shear(5, 0, 2),
        },
        4,
    );
    assert_eq!(reversible.frame, FramePreservation::Preserved);
    assert_eq!(reversible.inverse, InverseStanding::Verified);
    assert_eq!(reversible.replay, ReplayIdentity::ByteIdentical);
    assert_eq!(reversible.k_step, 4);

    let minted = step_witness(
        &origin,
        &FlowEvent::Mint {
            seed: shear(0, 3, 0),
        },
        5,
    );
    assert_eq!(minted.frame, FramePreservation::Preserved);
    assert_eq!(
        minted.inverse,
        InverseStanding::Violated {
            axis: Axis::Temporal
        },
        "the witness names WHERE the inverse failed"
    );
    assert_eq!(minted.replay, ReplayIdentity::ByteIdentical);
}

/// PIN 6 (the breath law): exhale never discards — the residual holds
/// every out-of-rank plane whole, and exhale→inhale restores the
/// carrier byte-exact. An interior zero plane survives the round trip.
#[test]
fn exhale_inhale_restores_byte_exact() {
    let carrier = AxisVector::from_vec(vec![ratio(3, 2), Zero::zero(), ratio(-7, 4), ratio(1, 8)]);
    let exhaled = exhale(&carrier, 2).expect("narrowing is an exhale");
    assert_eq!(exhaled.residual.components().len(), 2, "nothing discarded");
    assert_eq!(inhale(&exhaled), carrier, "the cycle restores byte-exact");

    // Widening is not an exhale — the refusal is typed.
    assert_eq!(
        exhale(&carrier, 9),
        Err(BreathError::NotAnExhale { from: 4, to: 9 })
    );
}
