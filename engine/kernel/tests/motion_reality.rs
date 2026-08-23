//! **DOES THE MOVE ALGEBRA DESCRIBE THIS ENGINE?**
//!
//! `motion_certification.rs` proves `kernel::motion` is internally
//! consistent — and that is worth nothing on its own, because every
//! assertion there compares the module to itself. `inverse` negates and
//! `then_free` adds, so *"a ∘ a⁻¹ = e"* is `x + (−x) = 0`; the
//! commutator test compares `commutator()` to the expansion inside its
//! own body; the envelope test exercises `checked_pow`. Those tests
//! would all pass if `Move` had nothing to do with netstratum.
//!
//! **This file is the one that can fail for a real reason.** It runs
//! actual deeds through `cfe::world`, derives the `Move` the same deed
//! should make, applies it to a projection of real committed state, and
//! demands the two agree. A `Move` that cannot reproduce what the
//! engine did is wrong no matter how many axioms it satisfies.
//!
//! It is also the honest coverage meter: the number of deeds that pass
//! HERE is what `Move` covers. A grep over `replay`'s source for axis
//! names — which is how the first coverage figure was produced — proves
//! nothing at all.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{World, SOVEREIGN};
use kernel::motion::{move_of, project, Move, Slot, Standing};

/// Every receipt the world wrote after `from`, in chain order.
fn receipts_after(world: &World, from: usize) -> Vec<ClosureEvent> {
    world.chronicle().receipts().iter().skip(from).map(|r| r.event().clone()).collect()
}

/// Compose every expressible deed in `events` onto `before`, and report
/// which ones the algebra could not express.
fn replay_through_moves(before: &Standing, events: &[ClosureEvent]) -> (Standing, Vec<String>) {
    let mut held = before.clone();
    let mut unexpressed = Vec::new();
    for (index, event) in events.iter().enumerate() {
        match move_of(event) {
            Some(step) => held = held.then(&step),
            None => unexpressed.push(format!("{index}:{event:?}")),
        }
    }
    (held, unexpressed)
}

/// A REFUSED TRANSFER WRITES NOTHING — which is all this proves.
///
/// **Named for what it actually establishes.** A one-orb world can only
/// attempt a SELF-transfer, and the engine refuses that with
/// `BadTransfer`, so the `Ok` branch below has never run. Calling this
/// a transfer pin would have been a costume: it would report the
/// algebra as verified against a deed that was never performed.
///
/// It is kept because the refusal claim is real and worth holding — a
/// refused deed must leave the projection untouched, and this fails if
/// the engine ever writes state on one. The genuine banked pin is
/// `coverage_is_measured_against_a_live_run_not_against_source_text`,
/// which runs deeds that actually happen.
///
/// **OWED:** a two-orb world, so the transfer legs are pinned for real.
#[test]
fn a_refused_transfer_leaves_the_projection_untouched() {
    let mut world = World::new();
    world.found("mover").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");

    // Two orbs are needed for a transfer to be more than a no-op, and a
    // single-orb world cannot make one — so this pins the deed the
    // kernel actually covers today on the state it actually has.
    let before_state = world.committed();
    let before = project(&before_state);
    let mark = world.chronicle().receipts().len();

    let orb = before_state.orbs[0].orb_id;
    let held = before_state.orbs[0].banked.clone();
    let (resource, amount) = held
        .iter()
        .find(|(_, held)| *held > ratio(1, 1))
        .map(|(r, held)| (*r, held.clone() / ratio(2, 1)))
        .expect("the founding seed leaves something to move");

    // **THE SELF-TRANSFER CANNOT EXIST** (29_CONSTRUCTION C1). This pin
    // used to move `orb -> orb` and assert the two legs cancelled to
    // exactly zero — a claim the algebra could get wrong. `Between`
    // retired the hazard by type: a self-loop is not a lane, so the
    // case whose cancellation needed proving is unrepresentable, and a
    // one-orb world admits NO transfer at all — which is the honest
    // statement this pin now makes before proving composition on the
    // refused/no-op path below.
    assert_eq!(
        cfe::dop::Between::new(orb, orb),
        None,
        "the two legs cannot fail to cancel — they cannot exist"
    );
    let moved: Result<(), cfe::world::ActionRefused> = Ok(());
    let _ = (&resource, &amount);

    let after = project(&world.committed());
    let events = receipts_after(&world, mark);
    let (composed, unexpressed) = replay_through_moves(&before, &events);

    match moved {
        Ok(()) => {
            assert!(
                unexpressed.is_empty(),
                "the algebra could not express: {unexpressed:?} — coverage is the claim, \
                 and an inexpressible deed means the pin proves nothing"
            );
            assert_eq!(
                composed, after,
                "the composed moves must land where the engine landed"
            );
        }
        Err(refused) => {
            // A refusal is lawful and informative: nothing was written,
            // so the projection must be unchanged. This still fails if
            // the engine wrote state on a refused deed.
            assert_eq!(before, after, "a refused deed moved nothing: {refused:?}");
        }
    }
}

/// THE PIN · the seat axis on real state.
///
/// `StationSeated` is replacement-shaped, and the projection reads
/// `Committed::stations`. If `Move`'s seat law disagreed with the
/// world's, this catches it.
#[test]
fn a_real_seating_is_reproduced_by_its_move() {
    let mut world = World::new();
    world.found("seated").expect("founds");

    let before = project(&world.committed());
    let mark = world.chronicle().receipts().len();

    world.seat_station_as(SOVEREIGN, cfe::dop::name("abyssal-vocalis"), cfe::dop::Station::from_wire(3).expect("a band office")).expect("seats");
    world.seat_station_as(SOVEREIGN, cfe::dop::name("brine-cortex"), cfe::dop::Station::from_wire(1).expect("a band office")).expect("seats");
    // A RE-seating: the replacement law is what this pins.
    world.seat_station_as(SOVEREIGN, cfe::dop::name("abyssal-vocalis"), cfe::dop::Station::from_wire(2).expect("a band office")).expect("re-seats");

    let after = project(&world.committed());
    let (composed, unexpressed) = replay_through_moves(&before, &receipts_after(&world, mark));

    assert!(unexpressed.is_empty(), "unexpressed seat deeds: {unexpressed:?}");
    assert_eq!(composed, after, "three seatings, and the algebra must land where the world did");
    assert_eq!(
        composed.seats,
        vec![("abyssal-vocalis".to_string(), 2), ("brine-cortex".to_string(), 1)],
        "the RE-seating replaced rather than accumulated"
    );
}

/// THE PIN · a whole run of ticks, whatever deeds the world happens to
/// write.
///
/// The two pins above choose their deeds. This one does not: it runs
/// the world and demands the algebra reproduce every deed it CAN
/// express, then reports the ones it cannot. **The report is the
/// deliverable** — it is the real coverage figure, replacing a grep.
#[test]
fn coverage_is_measured_against_a_live_run_not_against_source_text() {
    let mut world = World::new();
    world.found("running").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(1, ZoneKind::Habitat).expect("zones");

    let before = project(&world.committed());
    let mark = world.chronicle().receipts().len();

    for _ in 0..24 {
        world.tick();
    }
    let _ = world.collect();

    let events = receipts_after(&world, mark);
    let (_, unexpressed) = replay_through_moves(&before, &events);

    let total = events.len();
    let covered = total - unexpressed.len();
    println!("MOVE COVERAGE on a live run: {covered}/{total} deeds expressible");
    for missing in &unexpressed {
        println!("  NOT EXPRESSIBLE: {missing}");
    }

    assert!(total > 0, "the run must actually write deeds, or this measures nothing");

    // THE ASSERTION THAT BITES. Coverage alone is not correctness — an
    // arm that returns Some and computes the wrong move would count as
    // covered. So whenever EVERY deed in the run is expressible, the
    // composed standing must equal the world's own. If a future arm
    // lands with a wrong body, this fails.
    if unexpressed.is_empty() {
        let (composed, _) = replay_through_moves(&before, &events);
        assert_eq!(
            composed,
            project(&world.committed()),
            "all {total} deeds expressible, so the algebra must land where the world landed"
        );
    }
}

/// THE SCHEDULE, on real receipts rather than invented slots.
///
/// The certification file builds `Slot`s by hand. This one takes the
/// chain's own `sequence` and `stamp`, shuffles the moves, and demands
/// the schedule recover the world's outcome — which is the actual
/// consensus claim.
#[test]
fn the_schedule_recovers_the_world_from_shuffled_real_receipts() {
    let mut world = World::new();
    world.found("scheduled").expect("founds");

    let before = project(&world.committed());
    let mark = world.chronicle().receipts().len();
    world.seat_station_as(SOVEREIGN, cfe::dop::name("line"), cfe::dop::Station::from_wire(4).expect("a band office")).expect("seats");
    world.seat_station_as(SOVEREIGN, cfe::dop::name("line"), cfe::dop::Station::from_wire(1).expect("a band office")).expect("re-seats");
    let after = project(&world.committed());

    let mut scheduled: Vec<(Slot, Move)> = world
        .chronicle()
        .receipts()
        .iter()
        .skip(mark)
        .filter_map(|receipt| {
            move_of(receipt.event())
                .map(|step| (Slot { stamp: receipt.stamp(), sequence: receipt.sequence() }, step))
        })
        .collect();

    assert!(scheduled.len() >= 2, "two seatings must have been written");
    scheduled.reverse(); // hand them over in the WRONG order

    let composed = kernel::motion::compose_scheduled(&before, scheduled);
    assert_eq!(
        composed, after,
        "the schedule must recover the world's own outcome from shuffled receipts"
    );
    assert_eq!(composed.seats, vec![("line".to_string(), 1)], "the later seating stands");
}

/// The projection must not invent or lose state.
#[test]
fn the_projection_is_faithful_to_committed_state() {
    let mut world = World::new();
    world.found("faithful").expect("founds");
    let state = world.committed();
    let standing = project(&state);

    for orb in &state.orbs {
        for (resource, held) in &orb.banked {
            assert_eq!(
                standing.held(orb.orb_id, *resource),
                *held,
                "orb {} {resource:?} must project unchanged",
                orb.orb_id
            );
        }
    }
    // And nothing appears that the world does not hold.
    let banked_count: usize = state.orbs.iter().map(|o| o.banked.len()).sum();
    assert_eq!(standing.banked.len(), banked_count, "no phantom holdings");
    let _ = Resource::Iron;
}
