//! THE THERMAL BOOK, BOTH ARMS — the missing-leg sweep, 2026-08-02.
//!
//! `entropy_vented` sits on the orb: committed, replayed, public, and
//! CUMULATIVE FOREVER. Nothing on the orb recorded what was CAPTURED.
//! The capture arm lived only on the crystal sheet, windowed, pruned at
//! each boundary and `pub(super)` — so the permanent public book
//! recorded loss and forgot gain.
//!
//! `heat_encased_of` is that arm, derived from receipts already written.

use cfe::dop::{positive, unit};

use cfe::dop::ZoneKind;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A HOT colony with an ENCASEMENT shell — the capture zone. `Encasement`
/// is what yields `Resource::Joule`, and a joule yield collected is
/// exactly what the crystal sheet counts as ENCASED, so this is the
/// fixture in which the capture arm has anything at all to count.
fn furnace() -> World {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, unit(0, 1), positive(1, 2), None, None).expect("sites hot");
    world
}

#[test]
fn a_world_that_has_captured_nothing_reads_the_ground_on_both_arms() {
    // THE FLOOR, and it is a reading rather than a refusal. A fresh
    // colony has neither shed nor kept; both arms answer, and they
    // answer zero — never `None`, never a panic, never "unknown".
    let world = furnace();
    let (encased, vented) = world.thermal_books_of(FIRST_ORB);
    assert_eq!(encased, ratio(0, 1), "nothing captured yet");
    assert_eq!(vented, ratio(0, 1), "and nothing shed yet");

    // An orb the world never seated answers the ground too.
    let (e, v) = world.thermal_books_of(9_999);
    assert_eq!((e, v), (ratio(0, 1), ratio(0, 1)), "an unseated orb has no book");
}

#[test]
fn the_capture_arm_accumulates_and_is_never_pruned() {
    // THE WHOLE POINT. The sheet's `encased_entries` is WINDOWED and
    // pruned at each boundary, so a long-lived orb loses its early
    // capture while `entropy_vented` keeps every joule it ever shed.
    // This arm is read from the chronicle, which is append-only, so it
    // reaches back as far as the loss arm does.
    let mut world = furnace();
    for _ in 0..40 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, FIRST_ORB).expect("collects");
    let first = world.heat_encased_of(FIRST_ORB);
    assert!(first > ratio(0, 1), "a sunlit colony captured joules: {first}");

    // Tick well past a year boundary — the window the sheet keeps would
    // have moved on by now — and collect again.
    for _ in 0..80 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, FIRST_ORB).expect("collects again");
    let later = world.heat_encased_of(FIRST_ORB);
    assert!(
        later > first,
        "the capture arm must GROW across a boundary, not reset: {first} -> {later}"
    );
}

#[test]
fn the_two_arms_are_returned_as_a_pair_and_never_netted() {
    // `18` §2b: a directional quantity is never collapsed to a scalar
    // where the system computes. An orb that captured and shed the same
    // amount is NOT the same orb as one that did neither, and only the
    // pair can tell them apart — a net figure is the one-armed reading
    // wearing two arms' clothes.
    let mut world = furnace();
    for _ in 0..40 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, FIRST_ORB).expect("collects");

    let (encased, vented) = world.thermal_books_of(FIRST_ORB);
    assert_eq!(encased, world.heat_encased_of(FIRST_ORB), "the pair carries the live arm");
    assert_eq!(
        vented,
        world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented,
        "and the landed one, untouched"
    );
    // Both are magnitudes: a book, not a score.
    assert!(encased >= ratio(0, 1) && vented >= ratio(0, 1));
}

#[test]
fn the_landed_loss_arm_is_not_disturbed() {
    // The cure for a missing arm is to ADD the arm, never to touch the
    // one that stands. `entropy_vented` is bound by 05 §1.4's absorption
    // invariant, and `world_absorb` proves that binding by failing the
    // moment anything else is summed into it.
    let mut world = furnace();
    for _ in 0..40 {
        world.tick();
    }
    let before = world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented.clone();
    let _ = world.heat_encased_of(FIRST_ORB);
    let _ = world.thermal_books_of(FIRST_ORB);
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented,
        before,
        "reading the new arm moves nothing — both are derived reads"
    );
}

#[test]
fn the_two_books_are_symmetric_and_both_stored() {
    // THE ASYMMETRY CLOSED (operator ruling 2026-08-03). `entropy_vented`
    // has always been a stored orb field — cumulative, replayed, public.
    // The capture arm lived in a pruned window on the crystal sheet and,
    // later, in a derived reader. An orb could be asked what it had lost
    // forever and never what it had KEPT.
    //
    // `syntropy_contained` is now its exact inverse: same shape, same
    // lifetime, same book.
    let mut world = furnace();
    for _ in 0..40 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, FIRST_ORB).expect("collects");

    let orb = world.orb_by_id(FIRST_ORB).expect("orb");
    assert!(
        orb.syntropy_contained > ratio(0, 1),
        "a shelled furnace contained joules: {}",
        orb.syntropy_contained
    );

    // THE STORED BOOK EQUALS THE DERIVED READING. Storing something
    // derivable is only honest if drift is a test failure rather than a
    // silent divergence, so the two are pinned against each other.
    assert_eq!(
        orb.syntropy_contained,
        world.heat_encased_of(FIRST_ORB),
        "the stored book and the derived reading must never disagree"
    );
}

#[test]
fn the_syntropic_book_replays_exactly() {
    // A FIELD IS ONLY REAL IF REPLAY REBUILDS IT. The live path and
    // `replay` book the contained joule from the same rule; if they
    // ever differ, two nodes holding the same chain hold different
    // states, which is the one thing the chain exists to prevent.
    let mut world = furnace();
    for _ in 0..40 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, FIRST_ORB).expect("collects");
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the syntropic book");
    assert_eq!(
        rebuilt.orbs[0].syntropy_contained,
        world.orb_by_id(FIRST_ORB).expect("orb").syntropy_contained,
        "and the book itself rebuilt, not merely the rest of the state"
    );
}

#[test]
fn an_orb_that_contained_nothing_encodes_nothing() {
    // ONE STATE, ONE ENCODING. The entry is emitted only when nonzero,
    // so a world that has contained nothing stays byte-identical to a
    // pre-syntropy one and no chain is re-priced by the field's arrival.
    let mut bare = World::new();
    bare.found("meridian").expect("founds");
    let orb = bare.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(orb.syntropy_contained, ratio(0, 1), "nothing contained yet");

    let rebuilt = replay(bare.chronicle());
    assert_eq!(rebuilt, bare.committed(), "and it round-trips as zero");
}
