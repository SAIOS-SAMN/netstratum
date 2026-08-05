//! CONVEY — the postal deed, the estate chain's last link (operator
//! rulings 2026-08-05/06: *estate · parcel · postal · convey · survey*,
//! *parcel breaks open into free/monetary values*, *surveys must be
//! metered*). World-contained by the containment derivation: postal
//! movement between parcels of ONE estate; cross-world rides the
//! aperture pair, never this.
//!
//! The theorems:
//!   T1 address  — a postal address IS a surveyed parcel: both
//!                 endpoints must stand in the standing survey's
//!                 covering, or the conveyance refuses.
//!   T2 monetary — THE BREAK MEANS SOMETHING: only the MONETARY portion
//!                 of the sender's bank is conveyable; the free portion
//!                 stays home, and asking past the cap refuses pure.
//!   T3 postage  — metered like the survey: postage = rate × amount,
//!                 drawn from the sender, burned like upkeep; the gate
//!                 re-derives by calling the same rule.
//!   T4 conserved — the amount arrives whole; only the postage leaves
//!                 the world's total; replay MATCHES.

use cfe::dop::{parcels_of, postal_meter, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn surveyed_estate() -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("mine");
    world.zone(8, ZoneKind::Habitat).expect("dwelling");
    for _ in 0..30 {
        world.tick();
    }
    world.collect().expect("banks");
    world.chart("haven", FIRST_ORB).expect("charts");
    world.survey_estate(FIRST_ORB).expect("surveys");
    world
}

/// T1 — postal addresses are surveyed parcels, at both ends.
#[test]
fn a_postal_address_is_a_surveyed_parcel() {
    let mut unsurveyed = World::new();
    unsurveyed.found("origin").expect("founds");
    let haven = unsurveyed.chart("haven", FIRST_ORB).expect("charts");
    match unsurveyed.convey(FIRST_ORB, haven, Resource::Iron, ratio(1, 2)) {
        Err(ActionRefused::ClaimWithoutWitness { .. }) => {}
        other => panic!("no survey, no postal address, got {other:?}"),
    }

    // Surveyed: both endpoints covered — the conveyance stands.
    let mut world = surveyed_estate();
    let haven = world.orbs()[1].orb_id;
    world.convey(FIRST_ORB, haven, Resource::Iron, ratio(1, 2)).expect("the post delivers");

    // An orb chartered AFTER the survey has no address until resurvey.
    let late = world.chart("annex", FIRST_ORB).expect("charts late");
    match world.convey(FIRST_ORB, late, Resource::Iron, ratio(1, 4)) {
        Err(ActionRefused::ClaimWithoutWitness { .. }) => {}
        other => panic!("an unsurveyed annex is not an address, got {other:?}"),
    }
    world.survey_estate(FIRST_ORB).expect("resurvey — metered again");
    world.convey(FIRST_ORB, late, Resource::Iron, ratio(1, 4)).expect("now addressed");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// T2 — the free portion stays home: conveyable ≤ monetary × banked.
#[test]
fn the_free_portion_stays_home() {
    let mut world = surveyed_estate();
    let haven = world.orbs()[1].orb_id;
    let seed = world
        .chronicle()
        .receipts()
        .iter()
        .rev()
        .find_map(|r| match &r.event {
            ClosureEvent::EstateSurveyed { seed, .. } => Some(*seed),
            _ => None,
        })
        .expect("surveyed");
    let orbs: Vec<u64> = world.committed().orbs.iter().map(|o| o.orb_id).collect();
    let monetary = parcels_of(&seed, &orbs)
        .into_iter()
        .find(|p| p.orb_id == FIRST_ORB)
        .expect("the sender's parcel")
        .monetary;
    let banked = world.orbs()[0].banked_amount(Resource::Iron);
    let cap = monetary.clone() * banked.clone();

    let over = cap.clone() + ratio(1, 100);
    if over <= banked {
        // Only meaningful when the cap actually bites below the bank.
        let receipts = world.chronicle().receipts().len();
        match world.convey(FIRST_ORB, haven, Resource::Iron, over) {
            Err(ActionRefused::ConveyExceedsMonetary { .. }) => {}
            other => panic!("the free portion is not postal, got {other:?}"),
        }
        assert_eq!(world.chronicle().receipts().len(), receipts, "refusal orphaned nothing");
    }
    // Within the monetary portion, the post delivers.
    let lawful = cap * ratio(1, 2);
    if lawful > ratio(0, 1) {
        world.convey(FIRST_ORB, haven, Resource::Iron, lawful).expect("delivers");
    }
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// T3 + T4 — postage metered and burned; the amount arrives whole.
#[test]
fn postage_is_metered_and_the_amount_arrives_whole() {
    let mut world = surveyed_estate();
    let haven = world.orbs()[1].orb_id;
    let (stamp_resource, rate) = postal_meter();
    assert_eq!(stamp_resource, Resource::Iron, "stamps are iron, like survey stakes");

    let amount = ratio(1, 2);
    let postage = rate * amount.clone();
    let from_before = world.orbs()[0].banked_amount(Resource::Iron);
    let to_before = world
        .orbs()
        .iter()
        .find(|o| o.orb_id == haven)
        .expect("haven")
        .banked_amount(Resource::Iron);

    world.convey(FIRST_ORB, haven, Resource::Iron, amount.clone()).expect("delivers");

    assert_eq!(
        world.orbs()[0].banked_amount(Resource::Iron),
        from_before - amount.clone() - postage.clone(),
        "the sender pays the amount AND the stamp"
    );
    assert_eq!(
        world
            .orbs()
            .iter()
            .find(|o| o.orb_id == haven)
            .expect("haven")
            .banked_amount(Resource::Iron),
        to_before + amount.clone(),
        "the amount arrives WHOLE — postage burns, it is never skimmed in transit"
    );

    let receipt = world.chronicle().receipts().last().expect("conveyed");
    let ClosureEvent::ParcelConveyed { postage: declared, .. } = &receipt.event else {
        panic!("the last receipt is the conveyance");
    };
    assert_eq!(declared, &postage, "the receipt declares what the meter drew");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
