//! The first-slice witnesses: exact accrual (RFA — no drift), the
//! workspace/committed boundary (what has no receipt does not persist),
//! and SEAL B (the chronicle replays the committed universe).

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World};

#[test]
fn yields_accrue_exactly_and_collection_banks_them() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(4, ZoneKind::Extractor).expect("zoning closes");
    world.zone(9, ZoneKind::Habitat).expect("zoning closes");
    for _ in 0..4 {
        world.tick();
    }
    // 4 ticks: extractor 4 × 3/2 = 6 iron EXACTLY; habitat 4 × 1/4 = 1
    // biomass EXACTLY — ratios, not drift.
    assert_eq!(
        world.accrued(),
        &[(Resource::Iron, ratio(6, 1)), (Resource::Biomass, ratio(1, 1))]
    );
    let receipts = world.collect().expect("accrual closes");
    assert_eq!(receipts, 2, "one receipt per resource");
    let orb = world.orb().expect("the colony stands");
    assert_eq!(
        orb.banked,
        vec![(Resource::Iron, ratio(6, 1)), (Resource::Biomass, ratio(1, 1))]
    );
    assert!(world.accrued().is_empty(), "the workspace closed");
    assert_eq!(world.now(), 4, "the clock walked by quorum, once per tick");
}

#[test]
fn the_chronicle_replays_the_committed_world() {
    // SEAL B: play a session, then rebuild the committed projection
    // from the receipts ALONE and compare.
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(4, ZoneKind::Extractor).expect("zoning closes");
    for _ in 0..3 {
        world.tick();
    }
    world.collect().expect("accrual closes");
    world.zone(9, ZoneKind::Habitat).expect("zoning closes");
    world.tick();
    // NOTE: the last tick's accrual is UNCOLLECTED — workspace. It must
    // NOT appear in the replayed committed state.
    assert!(!world.accrued().is_empty(), "workspace holds the uncollected tail");
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    let replayed = replay(world.chronicle());
    assert_eq!(
        replayed,
        world.committed(),
        "the chronicle grounds the committed world, and ONLY the committed world"
    );
    let banked = &replayed.orb.expect("replay refounds the colony").banked;
    assert_eq!(
        banked,
        &vec![(Resource::Iron, ratio(9, 2))],
        "3 ticks × 3/2 = 9/2 iron, exact through the wire and back"
    );
}

#[test]
fn actions_refuse_typed() {
    let mut world = World::new();
    assert_eq!(world.zone(1, ZoneKind::Habitat), Err(ActionRefused::NoColony));
    assert_eq!(world.collect(), Err(ActionRefused::NoColony));
    world.found("meridian").expect("founding closes");
    assert_eq!(world.found("second"), Err(ActionRefused::ColonyAlreadyFounded));
    world.zone(1, ZoneKind::Habitat).expect("zoning closes");
    assert_eq!(
        world.zone(1, ZoneKind::Extractor),
        Err(ActionRefused::TileAlreadyZoned { tile: 1 })
    );
    assert_eq!(world.collect(), Err(ActionRefused::NothingAccrued));
}
