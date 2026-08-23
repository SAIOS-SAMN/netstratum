//! **THE FAILURE STATE** (`50_WIN_CONDITIONS`, operator ruling
//! 2026-08-13): *"there is no win state — there is a failure state —
//! otherwise the game should just continue in perpetuity."* Population
//! settling to zero across every orb closes the chronicle to further
//! scoped action. There is no win-condition pin to pair this with — by
//! the same ruling, none exists.

use cfe::dop::ZoneKind;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

#[test]
fn population_reaching_zero_collapses_the_line() {
    let mut world = World::new();
    world.found("dying-hold").expect("founds");
    world
        .seal_genesis("dying-line", 1, 2, 1)
        .expect("seals");
    assert!(world.committed().collapsed.is_none(), "a fresh founding stands");

    // Seed a population, then extinguish it — settle_population is the
    // one path a touch ever drives it through, live or in a test.
    world.settle_population(FIRST_ORB, 5);
    assert_eq!(world.population_of(FIRST_ORB), 5);
    assert!(world.committed().collapsed.is_none(), "a living line stands");

    world.settle_population(FIRST_ORB, -5);
    assert_eq!(world.population_of(FIRST_ORB), 0);
    assert!(world.committed().collapsed.is_some(), "zero population collapses the line");

    let receipt = world
        .chronicle()
        .receipts()
        .iter()
        .find(|r| matches!(r.event(), cfe::dop::ClosureEvent::CivilizationCollapsed { .. }))
        .expect("a CivilizationCollapsed receipt was chronicled");
    match receipt.event() {
        cfe::dop::ClosureEvent::CivilizationCollapsed { orb_id } => {
            assert_eq!(*orb_id, FIRST_ORB);
        }
        _ => unreachable!(),
    }
}

#[test]
fn a_collapsed_line_refuses_every_further_scoped_action() {
    let mut world = World::new();
    world.found("frozen-hold").expect("founds");
    world.seal_genesis("frozen-line", 1, 2, 1).expect("seals");
    world.settle_population(FIRST_ORB, 5);
    world.settle_population(FIRST_ORB, -5);
    assert!(world.committed().collapsed.is_some());

    match world.zone(0, ZoneKind::Extractor) {
        Err(ActionRefused::CivilizationCollapsed) => {}
        other => panic!("a collapsed line must refuse further building, got {other:?}"),
    }
    match world.collect() {
        Err(ActionRefused::CivilizationCollapsed) => {}
        other => panic!("a collapsed line must refuse further collection, got {other:?}"),
    }
    match world.zone_as(SOVEREIGN, FIRST_ORB, 9, ZoneKind::Habitat) {
        Err(ActionRefused::CivilizationCollapsed) => {}
        other => panic!("collapse refuses uniformly regardless of the acting entity, got {other:?}"),
    }
}

#[test]
fn collapse_never_reduces_population_below_zero_and_never_refires() {
    // The floor at settle_population's own .max(0) means a second
    // "further starvation" delta on an already-collapsed line settles
    // the population reading (still 0) without a second collapse
    // receipt — collapsed only ever goes None -> Some.
    let mut world = World::new();
    world.found("already-gone").expect("founds");
    world.seal_genesis("gone-line", 1, 2, 1).expect("seals");
    world.settle_population(FIRST_ORB, 3);
    world.settle_population(FIRST_ORB, -3);
    let first_collapse = world.committed().collapsed;
    assert!(first_collapse.is_some());

    let collapsed_receipts_before = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event(), cfe::dop::ClosureEvent::CivilizationCollapsed { .. }))
        .count();
    assert_eq!(collapsed_receipts_before, 1);
}

#[test]
fn the_failure_state_replays_byte_exact() {
    let mut world = World::new();
    world.found("replay-hold").expect("founds");
    world.seal_genesis("replay-line", 1, 2, 1).expect("seals");
    world.settle_population(FIRST_ORB, 4);
    world.settle_population(FIRST_ORB, -4);
    assert!(world.committed().collapsed.is_some());
    assert_eq!(replay(world.chronicle()), world.committed(), "failure-state replay MATCHES");
}
