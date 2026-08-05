//! THE COUNTER-OFFER — the third outcome (operator rulings 2026-08-06:
//! *survey/appraise → refuse/accept/renegotiate*, and *staleness
//! triggers renegotiation* — the counter is what CONSUMES the staleness
//! reading the appraisal landed).
//!
//! The protocol's three outcomes, as machinery:
//!   accept       the existing clear — fills read the COMMITTED terms,
//!                so a countered price is honored by construction;
//!   refuse       typed, pure, at every door;
//!   renegotiate  THIS deed — a receipted counter amending the resting
//!                offer's terms, keeping the negotiation standing (the
//!                resting-book law), evidenced by an appraisal when one
//!                is cited — and a STALE citation refuses: the market
//!                moved, re-appraise first. Consent needs no such gate.

use cfe::dop::{ClosureEvent, Holding, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn market_world() -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("mine");
    world.zone(8, ZoneKind::Habitat).expect("dwelling");
    for _ in 0..30 {
        world.tick();
    }
    world.collect().expect("banks");
    world
        .post_offer(FIRST_ORB, Resource::Iron, ratio(4, 1), Resource::Biomass, ratio(2, 1), 7)
        .expect("a resting intent");
    world
}

/// The counter AMENDS the resting terms in committed state — which is
/// exactly how acceptance honors it: fills read the committed offer.
#[test]
fn a_counter_amends_the_resting_terms() {
    let mut world = market_world();
    world
        .counter_offer(7, ratio(3, 1), None)
        .expect("the counter stands");
    let held = world.offers().iter().find(|o| o.offer == 7).expect("still resting");
    assert_eq!(held.want_amount, ratio(3, 1), "the ask moved to the countered figure");
    assert_eq!(held.give_amount, ratio(4, 1), "the give side stands as posted");

    let receipt = world.chronicle().receipts().last().expect("countered");
    let ClosureEvent::OfferCountered { offer: 7, want_amount, appraisal } = &receipt.event
    else {
        panic!("the last receipt is the counter");
    };
    assert_eq!(want_amount, &ratio(3, 1));
    assert!(appraisal.is_none(), "an uncited counter is lawful — evidence is optional");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// THE RULING, ENCODED: a counter citing a STALE appraisal refuses —
/// the market moved, so the evidence must be re-taken. A FRESH citation
/// stands. And the refusal is pure.
#[test]
fn a_stale_cited_counter_refuses_and_a_fresh_one_stands() {
    let mut world = market_world();
    world.survey_estate(FIRST_ORB).expect("surveys");
    world.appraise(Holding::Estate).expect("appraises");
    let stale = world.chronicle().receipts().last().expect("appraisal").own.0;
    world.tick();
    world.collect().expect("the world moves — the appraisal dates");
    assert_eq!(world.appraisal_stale(&stale), Some(true), "measurably stale");

    let receipts = world.chronicle().receipts().len();
    match world.counter_offer(7, ratio(5, 2), Some(stale)) {
        Err(ActionRefused::StaleAppraisalCited { .. }) => {}
        other => panic!("stale evidence must refuse, got {other:?}"),
    }
    assert_eq!(world.chronicle().receipts().len(), receipts, "refusal orphaned nothing");

    // Re-appraise — fresh evidence — and the same counter stands.
    world.appraise(Holding::Estate).expect("re-appraises");
    let fresh = world.chronicle().receipts().last().expect("appraisal").own.0;
    world
        .counter_offer(7, ratio(5, 2), Some(fresh))
        .expect("the renegotiation proceeds on fresh evidence");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// A counter on nothing refuses; a re-counter is lawful — negotiation
/// is rounds, and each round is a receipt.
#[test]
fn counters_need_a_resting_offer_and_rounds_are_receipts() {
    let mut world = market_world();
    assert!(
        matches!(world.counter_offer(99, ratio(1, 1), None), Err(ActionRefused::BadOffer)),
        "no such resting offer"
    );
    world.counter_offer(7, ratio(3, 1), None).expect("round one");
    world.counter_offer(7, ratio(7, 2), None).expect("round two — the poster counters back");
    let held = world.offers().iter().find(|o| o.offer == 7).expect("resting");
    assert_eq!(held.want_amount, ratio(7, 2), "the latest round holds the terms");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
