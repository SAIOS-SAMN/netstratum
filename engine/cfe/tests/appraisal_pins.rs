//! THE APPRAISAL — value derives from the survey, at every scale
//! (operator rulings 2026-08-06: *you have to APPRAISE the estate
//! before you can put a price tag on it — planetary → estate →
//! building → dwelling — survey/appraise → refuse/accept/renegotiate*,
//! and *staleness triggers renegotiation* — never blocks acceptance).
//!
//! The four theorems:
//!   T1 custody   — nothing appraises unsurveyed; the receipt cites
//!                  the survey it stands on.
//!   T2 derived   — value computes from surveyed content by ONE sealed
//!                  rule; stamped, so staleness is decidable.
//!   T3 one law   — one deed, every scale; the rule dispatches on the
//!                  subject as `zone_yield` dispatches on kind.
//!   T4 the trigger — STALENESS TRIGGERS RENEGOTIATION, NEVER BLOCKS
//!                  ACCEPTANCE: the reading is decidable and consent
//!                  closes deals; the counter-offer deed consumes it.

use cfe::dop::{appraised_value, commensurate_weight, Holding, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn surveyed_pair() -> World {
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

/// T1 — an unsurveyed estate cannot be appraised, and the refusal is
/// the SAME law the survey's witnesses use: a claim stands on receipts.
#[test]
fn appraising_an_unsurveyed_estate_refuses_pure() {
    let mut world = World::new();
    world.found("bare").expect("founds");
    let receipts = world.chronicle().receipts().len();
    match world.appraise(Holding::Estate) {
        Err(ActionRefused::ClaimWithoutWitness { .. }) => {}
        other => panic!("no survey, no appraisal, got {other:?}"),
    }
    assert_eq!(world.chronicle().receipts().len(), receipts, "refusal orphaned nothing");
}

/// T2 + T3 — one rule, three scales, and the compositions hold exactly:
/// a building is its sealed cost; a planet is its banks plus its
/// buildings; the estate is its planets DENOMINATED BY THE PARCELS.
#[test]
fn one_rule_every_scale_and_the_parcels_denominate() {
    let mut world = surveyed_pair();

    let dwelling = world
        .appraise(Holding::Building { orb: FIRST_ORB, tile: 8 })
        .expect("the dwelling appraises");
    let habitat_cost: Ratio = ZoneKind::Habitat
        .cost()
        .into_iter()
        .fold(ratio(0, 1), |sum, (resource, amount)| {
            sum + amount * commensurate_weight(resource)
        });
    assert_eq!(dwelling, habitat_cost, "a building IS its sealed cost, weighted");

    let planet = world.appraise(Holding::Planetary { orb: FIRST_ORB }).expect("appraises");
    assert!(planet > dwelling, "a planet carries its buildings and its banks");

    let estate = world.appraise(Holding::Estate).expect("appraises");
    let parcels = world.parcels().expect("the survey's parcels");
    let expected = parcels.iter().fold(ratio(0, 1), |sum, parcel| {
        let planetary = world
            .appraise_value(&Holding::Planetary { orb: parcel.orb_id })
            .expect("covered orb values");
        sum + planetary * parcel.monetary.clone()
    });
    assert_eq!(estate, expected, "the estate is its planets × the parcels' monetary break");

    // And the receipt cites the survey it stands on (T1's custody).
    let receipt = world.chronicle().receipts().last().expect("appraisal");
    let cfe::dop::ClosureEvent::AppraisalStruck { survey, value, .. } = &receipt.event else {
        panic!("the last receipt is the appraisal");
    };
    let survey_digest = world
        .chronicle()
        .receipts()
        .iter()
        .rev()
        .find(|r| matches!(r.event, cfe::dop::ClosureEvent::EstateSurveyed { .. }))
        .map(|r| r.own.0)
        .expect("the survey stands");
    assert_eq!(survey, &survey_digest, "the appraisal cites its survey");
    assert_eq!(value, &estate, "and declares the value the rule computed");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// T4 — the staleness reading: decidable, and it is the RENEGOTIATION
/// trigger, never an acceptance block. The reading exists so a counter
/// can demand a fresh appraisal; consent needs no such gate.
#[test]
fn staleness_is_decidable_and_triggers_renegotiation_never_blocks() {
    let mut world = surveyed_pair();
    world.appraise(Holding::Estate).expect("appraises");
    let appraisal = world.chronicle().receipts().last().expect("receipt").own.0;
    assert_eq!(
        world.appraisal_stale(&appraisal),
        Some(false),
        "fresh at its own stamp"
    );
    world.tick();
    world.collect().expect("the world moved");
    assert_eq!(
        world.appraisal_stale(&appraisal),
        Some(true),
        "the world moved past the stamp — the market moved, renegotiate"
    );
    assert_eq!(world.appraisal_stale(&[9u8; 32]), None, "no such appraisal is not a staleness");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// T2's pure half — the sealed rule is callable without a deed, total
/// over lawful subjects, `None` beyond them.
#[test]
fn the_rule_is_pure_and_none_off_the_estate() {
    let world = surveyed_pair();
    assert!(world.appraise_value(&Holding::Building { orb: FIRST_ORB, tile: 0 }).is_some());
    assert!(
        world.appraise_value(&Holding::Building { orb: FIRST_ORB, tile: 63 }).is_none(),
        "no zone, no value — absence is not zero"
    );
    assert!(world.appraise_value(&Holding::Planetary { orb: 999 }).is_none());
    let _ = appraised_value; // the dop rule is the one public truth
}
