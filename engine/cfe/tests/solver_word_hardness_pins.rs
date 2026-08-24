//! ENTITY-ALIGN-1 (tag 189, docs/56 §3) — `SolverWordHardness`, landed
//! schema-only: the wire round-trips exactly, closing one changes NO
//! committed state (identity bookkeeping, not a world event), and
//! `SolverWordMinted`'s own bytes are untouched by this landing — proven
//! by round-tripping it unchanged alongside the new receipt.

use cfe::dop::ClosureEvent;
use cfe::vrp::Chronicle;
use cfe::world::replay;

#[test]
fn solver_word_hardness_round_trips_the_wire() {
    let event = ClosureEvent::SolverWordHardness { orbit: [3u8; 32], hardness: 2 };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(ClosureEvent::from_wire(tag, value), Some(event));
}

#[test]
fn solver_word_minted_bytes_are_untouched_by_the_new_receipt() {
    let event = ClosureEvent::SolverWordMinted {
        orbit: [5u8; 32],
        expansion: vec![1, 2, 3, 4, 5],
        cited: vec![[6u8; 32], [7u8; 32]],
    };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(ClosureEvent::from_wire(tag, value), Some(event));
}

#[test]
fn solver_word_hardness_is_a_replay_no_op() {
    let base = ClosureEvent::EvolutionAdvanced { stage: 1 };

    let mut without = Chronicle::new();
    without.append(base.clone(), 1, vec![]);
    let baseline = replay(&without);

    let mut with = Chronicle::new();
    with.append(base, 1, vec![]);
    with.append(ClosureEvent::SolverWordHardness { orbit: [1u8; 32], hardness: 1 }, 2, vec![]);
    let with_hardness = replay(&with);

    assert_eq!(
        baseline, with_hardness,
        "the hardness receipt changes no committed state — recorded only, at this landing"
    );
}
