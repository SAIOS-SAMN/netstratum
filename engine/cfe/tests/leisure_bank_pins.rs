//! **THE LEISURE BANK** (COG-1, doc 52 §7) — the cognition budget an orb
//! banks from the cascade's surplus above every survival drive, and spends to
//! mint. A stored per-orb reserve, floored at zero, receipted on tag 180 so
//! replay reconstructs it exactly — the same discipline as the sovereign
//! treasury, for the Mind's purse rather than the sovereign's.

use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::world::{replay, World, FIRST_ORB};

#[test]
fn the_leisure_deed_round_trips_on_tag_180() {
    for delta in [ratio(7, 1), ratio(-3, 1), ratio(5, 4), ratio(-5, 4)] {
        let deed = ClosureEvent::LeisureSettled { orb_id: 5, delta: delta.clone() };
        let bytes = deed.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert_eq!(tag, 180, "LeisureSettled is tag 180");
        assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    }
}

#[test]
fn a_fresh_leisure_bank_reads_empty() {
    let world = World::new();
    assert_eq!(
        world.leisure_bank_of(FIRST_ORB),
        ratio(0, 1),
        "a line that has never run a surplus cannot afford a thought"
    );
}

#[test]
fn surplus_accumulates_and_minting_draws_it_down_replay_safe() {
    let mut world = World::new();
    world.found("thinker").expect("founds");
    // Two beats of surplus bank into the cognition budget.
    world.settle_leisure(FIRST_ORB, ratio(6, 1));
    world.settle_leisure(FIRST_ORB, ratio(4, 1));
    assert_eq!(world.leisure_bank_of(FIRST_ORB), ratio(10, 1), "surplus accumulates");
    // A mint spends the budget down.
    world.settle_leisure(FIRST_ORB, ratio(-7, 1));
    assert_eq!(world.leisure_bank_of(FIRST_ORB), ratio(3, 1), "minting draws the budget");
    // The whole history replays byte-exact and survives a resume.
    assert_eq!(replay(world.chronicle()), world.committed(), "leisure replay MATCHES");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.leisure_bank_of(FIRST_ORB), ratio(3, 1), "the budget survives a resume");
}

#[test]
fn a_mint_cannot_overdraw_the_leisure_bank() {
    let mut world = World::new();
    world.found("thinker").expect("founds");
    world.settle_leisure(FIRST_ORB, ratio(3, 1));
    // Spend more than stands — the budget empties, it does not go negative: a
    // mind pays what it has and no more (the metabolic bound on cognition).
    world.settle_leisure(FIRST_ORB, ratio(-10, 1));
    assert_eq!(world.leisure_bank_of(FIRST_ORB), ratio(0, 1), "a thought cannot overdraw the mind");
    assert_eq!(replay(world.chronicle()), world.committed(), "floored replay MATCHES");
}

#[test]
fn the_leisure_bank_survives_a_codec_round_trip() {
    let mut world = World::new();
    world.found("thinker").expect("founds");
    world.settle_leisure(FIRST_ORB, ratio(11, 2));
    let committed = world.committed();
    let bytes = codec::state_bytes(&committed);
    let decoded = codec::from_state_bytes(&bytes).expect("the state decodes");
    assert_eq!(decoded, committed, "the leisure bank round-trips through the codec");
    assert_eq!(decoded.leisure_banks, committed.leisure_banks, "the cognition budget is carried");
}
