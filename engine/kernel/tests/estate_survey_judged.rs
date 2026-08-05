//! THE ESTATE SURVEY, JUDGED — the theorems' kernel half (E5b).
//!
//! The emitter's proofs live in `cfe/tests/estate_survey_pins.rs`; here
//! the JUDGE holds them: a lawful survey admits, a partial covering
//! refuses (unwitnessed observations cannot enter), a misquoted meter
//! refuses (the kernel calls the SAME sealed rule the emitter drew
//! from), and the draw conserves.

use cfe::dop::{survey_meter, ClosureEvent};
use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{admits, Refused, Step};

fn surveyed_transition() -> (cfe::world::Committed, cfe::world::Committed, Step) {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("mine");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("farm");
    for _ in 0..30 {
        world.tick();
    }
    world.collect().expect("banks");
    let _ = world.chart("haven", FIRST_ORB).expect("charts");
    let prior = world.committed();
    world.survey_estate(FIRST_ORB).expect("surveys");
    let next = world.committed();
    let receipt = world.chronicle().receipts().last().expect("the survey").clone();
    let step = Step { event: receipt.event, actor: SOVEREIGN, stamp: receipt.stamp };
    (prior, next, step)
}

#[test]
fn a_lawful_survey_is_admitted() {
    let (prior, next, step) = surveyed_transition();
    assert_eq!(admits(&prior, &next, &step), Ok(()), "the deed the emitter escrowed admits");
}

#[test]
fn a_partial_covering_refuses_the_observation() {
    // THEOREM (witnessed, whole): drop one orb from the covering — the
    // observation no longer stands over the estate it claims.
    let (prior, next, mut step) = surveyed_transition();
    let ClosureEvent::EstateSurveyed { covered, .. } = &mut step.event else {
        panic!("survey step");
    };
    covered.pop();
    match admits(&prior, &next, &step) {
        Err(Refused::SurveyNotWhole { covered: 1, estate: 2 }) => {}
        other => panic!("a partial covering must refuse SurveyNotWhole, got {other:?}"),
    }
    // And the empty covering — an unwitnessed observation — refuses too.
    let ClosureEvent::EstateSurveyed { covered, .. } = &mut step.event else {
        panic!("survey step");
    };
    covered.clear();
    assert!(
        matches!(admits(&prior, &next, &step), Err(Refused::SurveyNotWhole { covered: 0, .. })),
        "an unwitnessed observation cannot enter the record"
    );
}

#[test]
fn a_misquoted_meter_refuses() {
    // THEOREM (metered): the judge computes the meter by CALLING the
    // sealed rule — a declared cost above or below it refuses, so the
    // survey cannot be quoted one way and charged another.
    let (prior, next, mut step) = surveyed_transition();
    let ClosureEvent::EstateSurveyed { cost, .. } = &mut step.event else {
        panic!("survey step");
    };
    *cost = cost.clone() + ratio(1, 1);
    assert!(
        matches!(admits(&prior, &next, &step), Err(Refused::SurveyMeterMismatch { .. })),
        "a misquoted meter must refuse"
    );
    let (_, rate) = survey_meter();
    assert_eq!(rate * ratio(2, 1), ratio(1, 4), "and the ruled figure is the dial's: 2 × 1/8");
}

#[test]
fn the_draw_conserves_or_refuses() {
    // I1 on the meter's debit: a `next` that keeps the iron while the
    // deed declares the draw refuses ConservationBroken.
    let (prior, _, step) = surveyed_transition();
    assert!(
        matches!(admits(&prior, &prior, &step), Err(Refused::ConservationBroken { .. })),
        "declared draw with no debit is minting by omission"
    );
}
