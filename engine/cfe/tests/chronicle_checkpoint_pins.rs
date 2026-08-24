//! CHRONICLE-COMPACT-1 (tag 188) — `ChronicleCheckpointed`, landed
//! schema-only: the wire round-trips exactly, and closing one changes NO
//! committed state (a pure bookkeeping fact, not a world event). The
//! load-bearing use — a checkpoint-aware `replay` that trusts a verified
//! `folded_hash` and skips straight to the post-checkpoint suffix — is
//! CHRONICLE-COMPACT-3, not yet built.

use cfe::dop::ClosureEvent;
use cfe::vrp::Chronicle;
use cfe::world::replay;

#[test]
fn chronicle_checkpointed_round_trips_the_wire() {
    let event = ClosureEvent::ChronicleCheckpointed { folded_hash: [7u8; 32], receipt_count: 42 };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(ClosureEvent::from_wire(tag, value), Some(event));
}

#[test]
fn chronicle_checkpointed_is_a_replay_no_op() {
    // A harmless real event, so the baseline chronicle isn't empty.
    let base = ClosureEvent::EvolutionAdvanced { stage: 1 };

    let mut without = Chronicle::new();
    without.append(base.clone(), 1, vec![]);
    let baseline = replay(&without);

    let mut with = Chronicle::new();
    with.append(base, 1, vec![]);
    with.append(
        ClosureEvent::ChronicleCheckpointed { folded_hash: [9u8; 32], receipt_count: 1 },
        2,
        vec![],
    );
    let with_checkpoint = replay(&with);

    assert_eq!(
        baseline, with_checkpoint,
        "the checkpoint receipt changes no committed state — schema only, at this landing"
    );
}
