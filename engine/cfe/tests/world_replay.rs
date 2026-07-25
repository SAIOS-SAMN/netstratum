//! The colony-loop witnesses under ruleset v1: exact net flows (RFA —
//! no drift), builds that SPEND, the workspace/committed boundary, and
//! SEAL B (the chronicle replays the committed universe).

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World};

#[test]
fn nets_accrue_exactly_and_collection_settles_them() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(4, ZoneKind::Extractor).expect("affordable");
    world.zone(9, ZoneKind::Habitat).expect("affordable");
    // Founding banked 20 iron / 10 biomass; builds spent 10 iron + 5
    // biomass: banked now 10 iron, 5 biomass.
    let orb = world.orb().expect("the colony stands");
    assert_eq!(
        orb.banked,
        vec![(Resource::Iron, ratio(10, 1)), (Resource::Biomass, ratio(5, 1))]
    );
    for _ in 0..4 {
        world.tick();
    }
    // 1E+1H nets per tick: iron +3/2 − 1/4 = +5/4; biomass +1 − 1/2 =
    // +1/2. Four ticks: +5 iron, +2 biomass — EXACTLY.
    assert_eq!(
        world.accrued(),
        &[(Resource::Iron, ratio(5, 1)), (Resource::Biomass, ratio(2, 1))]
    );
    let receipts = world.collect().expect("nets settle");
    assert_eq!(receipts, 2, "one receipt per resource");
    let orb = world.orb().expect("the colony stands");
    assert_eq!(
        orb.banked,
        vec![(Resource::Iron, ratio(15, 1)), (Resource::Biomass, ratio(7, 1))]
    );
    assert!(world.accrued().is_empty(), "the workspace closed");
    assert_eq!(world.now(), 4, "the clock walked by quorum, once per tick");
}

#[test]
fn the_chronicle_replays_the_committed_world() {
    // SEAL B under ruleset v1: a session with builds, a negative net
    // settle, and an uncollected tail — replayed from receipts ALONE.
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(4, ZoneKind::Extractor).expect("affordable");
    for _ in 0..3 {
        world.tick();
    }
    // Lone extractor, 3 ticks: iron +9/2 accrued, biomass −3/2 accrued.
    world.collect().expect("nets settle");
    // Settled: banked iron 10 + 9/2 = 29/2; biomass 10 − 3/2 = 17/2.
    world.zone(9, ZoneKind::Habitat).expect("affordable");
    world.tick();
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
        &vec![(Resource::Iron, ratio(29, 2)), (Resource::Biomass, ratio(7, 2))],
        "29/2 iron and 17/2−5 = 7/2 biomass, exact through the wire and back"
    );
}

#[test]
fn actions_refuse_typed() {
    let mut world = World::new();
    assert_eq!(world.zone(1, ZoneKind::Habitat), Err(ActionRefused::NoColony));
    assert_eq!(world.collect(), Err(ActionRefused::NoColony));
    assert_eq!(world.raze(1), Err(ActionRefused::NoColony));
    world.found("meridian").expect("founding closes");
    assert_eq!(world.found("second"), Err(ActionRefused::ColonyAlreadyFounded));
    world.zone(1, ZoneKind::Habitat).expect("affordable");
    assert_eq!(
        world.zone(1, ZoneKind::Extractor),
        Err(ActionRefused::TileAlreadyZoned { tile: 1 })
    );
    assert_eq!(world.raze(7), Err(ActionRefused::NoSuchZone { tile: 7 }));
    assert_eq!(world.collect(), Err(ActionRefused::NothingAccrued));
}
