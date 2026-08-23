//! THE APPRAISAL, GATED — the value re-derived at the gate from the
//! receipt's own seed and covering, by the SAME sealed rule (T2's
//! kernel half). Staleness appears in NO arm here, deliberately: it
//! triggers renegotiation and never gates a deed (operator ruling
//! 2026-08-06).

use cfe::dop::{ClosureEvent, Holding, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{admits, Refused, Step};

fn appraised_transition() -> (cfe::world::Committed, cfe::world::Committed, Step) {
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
    let prior = world.committed();
    world.appraise(Holding::Estate).expect("appraises");
    let next = world.committed();
    let receipt = world.chronicle().receipts().last().expect("appraisal").clone();
    (prior, next, Step { event: receipt.event().clone(), actor: SOVEREIGN, stamp: receipt.stamp() })
}

#[test]
fn a_derivable_appraisal_admits() {
    let (prior, next, step) = appraised_transition();
    assert_eq!(admits(&prior, &next, &step), Ok(()), "the gate re-derives and agrees");
}

#[test]
fn a_misvalued_appraisal_refuses() {
    let (prior, next, mut step) = appraised_transition();
    let ClosureEvent::AppraisalStruck { value, .. } = &mut step.event else {
        panic!("appraisal step");
    };
    *value = value.clone() + ratio(1, 1);
    assert!(
        matches!(admits(&prior, &next, &step), Err(Refused::AppraisalMisvalued { .. })),
        "a value quoted one way and receipted another refuses"
    );
}

#[test]
fn a_subject_outside_the_covering_refuses() {
    let (prior, next, mut step) = appraised_transition();
    let ClosureEvent::AppraisalStruck { subject, .. } = &mut step.event else {
        panic!("appraisal step");
    };
    *subject = Holding::Planetary { orb: 999 };
    assert!(
        matches!(
            admits(&prior, &next, &step),
            Err(Refused::AppraisalOutsideSurvey { orb: 999 })
        ),
        "nothing appraises unsurveyed — at the gate too"
    );
}
