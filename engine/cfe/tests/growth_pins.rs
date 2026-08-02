//! GROWTH-1 — the six growth mechanisms (operator ruling 2026-08-03).
//!
//! *"How do we go from primordialism to significant civilization without
//! leaps and bounds and skipping everything in between?"*
//!
//! Each axis measures something the chain PRODUCES rather than something
//! a line ASSERTS. The pin that matters most is the first: nothing here
//! can be reached at stamp 0.

use cfe::ratio::ratio;
use cfe::world::World;

fn founded() -> (World, u64) {
    let mut world = World::default();
    world.found("home").expect("founds");
    let orb = world.orb().expect("founded").orb_id;
    (world, orb)
}

// ── PRACTICE — TIME ───────────────────────────────────────────────────

/// **THE ANTI-LEAP PIN.** Everything done at one stamp spans nothing,
/// however much of it there is. A span cannot be forged the way a
/// keystroke can.
#[test]
fn practice_cannot_be_reached_at_stamp_zero() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    for _ in 0..50 {
        world
            .post_book(orb, kind, ratio(1, 1), ratio(1, 1))
            .expect("posts");
    }
    let growth = world.growth_of(orb);
    assert_eq!(growth.practice, ratio(0, 1), "fifty postings, no time");
    assert!(!growth.practised());
    assert!(!world.literate(orb), "volume is not practice");
}

/// Practice is the span the writing covers against the age of the line.
#[test]
fn practice_is_span_against_the_age_of_the_line() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    for _ in 0..10 {
        world.tick();
    }
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts later");

    let growth = world.growth_of(orb);
    assert!(growth.practised());
    // Wrote across the whole life so far: span == age.
    assert_eq!(growth.practice, ratio(1, 1));
    assert!(world.literate(orb));
}

/// A line that wrote early and then stopped has its practice DILUTED by
/// the life it went on to have without writing. Growth is not a ratchet.
#[test]
fn practice_dilutes_when_the_writing_stops() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    for _ in 0..5 {
        world.tick();
    }
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    let attentive = world.growth_of(orb).practice;

    for _ in 0..15 {
        world.tick();
    }
    let lapsed = world.growth_of(orb).practice;
    assert!(lapsed < attentive, "the span stood still while the life ran on");
    assert!(lapsed > ratio(0, 1), "it did happen, and still counts");
}

// ── KNOWLEDGE, EXPERTISE — BREADTH against DEPTH ──────────────────────

/// Knowledge counts accounts that have been WRITTEN IN. Minting alone
/// records nothing.
#[test]
fn knowledge_counts_only_accounts_written_in() {
    let (mut world, orb) = founded();
    let used = world.mint_book(orb, "Kept").expect("mints");
    world.mint_book(orb, "Empty").expect("mints");
    world
        .post_book(orb, used, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    assert_eq!(world.growth_of(orb).knowledge, 1, "an empty book is not knowledge");
}

/// Breadth and depth are orthogonal: one deep account reads expertise 1,
/// and spreading the same weight thins it.
#[test]
fn expertise_is_depth_and_thins_as_breadth_grows() {
    let (mut world, orb) = founded();
    let one = world.mint_book(orb, "First").expect("mints");
    world
        .post_book(orb, one, ratio(4, 1), ratio(0, 1))
        .expect("posts");
    let deep = world.growth_of(orb);
    assert_eq!(deep.knowledge, 1);
    assert_eq!(deep.expertise, ratio(1, 1), "one account holds everything");

    let two = world.mint_book(orb, "Second").expect("mints");
    world
        .post_book(orb, two, ratio(4, 1), ratio(0, 1))
        .expect("posts");
    let broad = world.growth_of(orb);
    assert_eq!(broad.knowledge, 2);
    assert_eq!(broad.expertise, ratio(1, 2), "the same weight, spread");
}

// ── WISDOM — TESTING ──────────────────────────────────────────────────

/// **A LINE THAT ONLY EVER POSTED WINS HAS LEARNED NOTHING.** Wisdom
/// reads the share of accounts carrying BOTH sides — which is why
/// BOOK-1 made the dual structural rather than conventional.
#[test]
fn wisdom_requires_having_been_tested() {
    let (mut world, orb) = founded();
    let unbeaten = world.mint_book(orb, "Unbeaten").expect("mints");
    world
        .post_book(orb, unbeaten, ratio(9, 1), ratio(0, 1))
        .expect("only wins");
    assert_eq!(
        world.growth_of(orb).wisdom,
        ratio(0, 1),
        "a spotless record is not wisdom"
    );

    world
        .post_book(orb, unbeaten, ratio(0, 1), ratio(1, 1))
        .expect("and a loss");
    assert_eq!(world.growth_of(orb).wisdom, ratio(1, 1), "now it has been tested");
}

#[test]
fn wisdom_is_the_share_of_accounts_tested() {
    let (mut world, orb) = founded();
    let tested = world.mint_book(orb, "Tested").expect("mints");
    let spotless = world.mint_book(orb, "Spotless").expect("mints");
    world
        .post_book(orb, tested, ratio(1, 1), ratio(1, 1))
        .expect("posts");
    world
        .post_book(orb, spotless, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    assert_eq!(world.growth_of(orb).wisdom, ratio(1, 2));
}

// ── LOCALITY — SCOPE ──────────────────────────────────────────────────

/// A line with one orb is wholly local: everything it knows, it knows
/// here.
#[test]
fn a_single_orb_line_is_wholly_local() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    world
        .post_book(orb, kind, ratio(3, 1), ratio(1, 1))
        .expect("posts");
    assert_eq!(world.growth_of(orb).locality, ratio(1, 1));
}

/// Locality reads a SHARE of the line, so an orb that has recorded
/// nothing is not local to anything.
#[test]
fn an_orb_that_recorded_nothing_holds_no_locality() {
    let (world, orb) = founded();
    let growth = world.growth_of(orb);
    assert_eq!(growth.locality, ratio(0, 1));
    assert_eq!(growth.knowledge, 0);
    assert!(!growth.substantive());
}

// ── TRIVIALITY — THE GUARD ────────────────────────────────────────────

/// **THE GUARD THAT MAKES THE OTHER FIVE HONEST.** A posting that moved
/// nothing happened and taught nothing; triviality is what says so, and
/// without it every axis is farmable.
#[test]
fn triviality_exposes_a_padded_record() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("a real entry");
    for _ in 0..3 {
        world
            .post_book(orb, kind, ratio(0, 1), ratio(0, 1))
            .expect("padding");
    }
    let growth = world.growth_of(orb);
    assert_eq!(growth.triviality, ratio(3, 4), "three of four moved nothing");
    assert!(growth.substantive(), "something real is still in there");
}

/// A wholly empty record is not substantive however long it is.
#[test]
fn a_wholly_empty_record_is_not_substantive() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    for _ in 0..5 {
        world
            .post_book(orb, kind, ratio(0, 1), ratio(0, 1))
            .expect("padding");
        world.tick();
    }
    let growth = world.growth_of(orb);
    assert_eq!(growth.triviality, ratio(1, 1));
    assert_eq!(growth.knowledge, 0, "nothing was ever written in");
    assert!(!growth.substantive());
    // And padding across time does NOT buy literacy, because the
    // accounts stayed empty.
    assert!(!world.literate(orb), "spanning time with nothing is not practice");
}

// ── THE READING IS WHOLE ──────────────────────────────────────────────

/// Six quantities carried whole, and no aggregate. Collapsing them to
/// one number would be the struck C-gate under a sixth name.
#[test]
fn growth_is_derived_and_reproduces() {
    let (mut world, orb) = founded();
    let kind = world.mint_book(orb, "Ledger").expect("mints");
    world
        .post_book(orb, kind, ratio(2, 1), ratio(1, 1))
        .expect("posts");
    world.tick();
    world
        .post_book(orb, kind, ratio(1, 1), ratio(2, 1))
        .expect("posts");
    assert_eq!(world.growth_of(orb), world.growth_of(orb), "pure");
}
