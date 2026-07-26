//! P1 witnesses — multi-planet worlds: chartering conserves matter,
//! planets fail independently under ONE chronicle, and the whole
//! archipelago replays from receipts alone (binding #1, 04 §5).

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

#[test]
fn chartering_moves_the_reserves_and_mints_nothing() {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    // Earn enough to charter: 1E+1H nets +5/4 iron, +1/2 biomass.
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("settles");
    let source_iron = world.orb().expect("stands").banked_amount(Resource::Iron);
    let source_biomass = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let daughter = world.chart("far-anchor", FIRST_ORB).expect("charters");
    // The reserves MOVED: source lost exactly what the daughter holds.
    let source = world.orb_by_id(FIRST_ORB).expect("stands");
    assert_eq!(source.banked_amount(Resource::Iron), source_iron - ratio(20, 1));
    assert_eq!(source.banked_amount(Resource::Biomass), source_biomass - ratio(10, 1));
    let far = world.orb_by_id(daughter).expect("charted");
    assert_eq!(far.banked_amount(Resource::Iron), ratio(20, 1));
    assert_eq!(far.banked_amount(Resource::Biomass), ratio(10, 1));
    assert_eq!(far.name, "far-anchor");
}

#[test]
fn chartering_refuses_typed() {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    // Bootstrap alone cannot charter (20/10 is exactly the reserves,
    // but zoning first spends them).
    world.zone(0, ZoneKind::Extractor).expect("builds");
    assert!(matches!(
        world.chart("far-anchor", FIRST_ORB),
        Err(ActionRefused::CannotAfford { resource: Resource::Iron, .. })
    ));
    assert_eq!(
        world.chart("anything", 9),
        Err(ActionRefused::NoSuchOrb { orb: 9 })
    );
    assert_eq!(
        world.chart("meridian", FIRST_ORB),
        Err(ActionRefused::PlanetNameTaken { name: "meridian".into() }),
        "planet names are unique within a world"
    );
}

#[test]
fn planets_fail_independently_under_one_chronicle() {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("settles");
    let daughter = world.chart("far-anchor", FIRST_ORB).expect("charters");
    // The daughter runs the famine opening: 3E+1H (biomass net −1/2)
    // while meridian stays balanced at 1E+1H.
    world.zone_as(SOVEREIGN, daughter, 0, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, daughter, 1, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, daughter, 8, ZoneKind::Habitat).expect("builds");
    world.collect_as(SOVEREIGN, FIRST_ORB).ok();
    let mut struck = None;
    for _ in 0..40 {
        let outcome = world.tick();
        if !outcome.struck.is_empty() {
            struck = Some(outcome.struck);
            break;
        }
    }
    // Wait — 2E+1H on the daughter is the knife edge (net 0), never
    // famine; the third extractor above (tiles 0,1 + hab) is 2E+1H.
    // The daughter is SOLVENT: no famine anywhere, both planets turn.
    assert_eq!(struck, None, "the knife edge holds on the daughter too");
    let meridian_accrued = world.accrued_of(FIRST_ORB);
    let daughter_accrued = world.accrued_of(daughter);
    assert!(!meridian_accrued.is_empty() && !daughter_accrued.is_empty());
    assert_ne!(
        meridian_accrued, daughter_accrued,
        "two economies, two workspaces, one clock"
    );
}

#[test]
fn a_daughter_famine_never_touches_the_mother() {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("settles");
    let daughter = world.chart("slag-hollow", FIRST_ORB).expect("charters");
    // Two extractors and NO habitat on the daughter: biomass drains
    // −1/tick against its 10 banked → 10 − t < 0 first at t = 11;
    // famine on schedule, daughter only.
    world.zone_as(SOVEREIGN, daughter, 0, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, daughter, 1, ZoneKind::Extractor).expect("builds");
    let mut struck = Vec::new();
    for _ in 0..11 {
        let outcome = world.tick();
        struck.extend(outcome.struck);
    }
    assert_eq!(struck, vec![(daughter, Resource::Biomass)]);
    assert!(
        world.orb_by_id(FIRST_ORB).expect("stands").famine.is_empty(),
        "meridian never notices slag-hollow's hunger"
    );
    assert!(!world.orb_by_id(daughter).expect("stands").famine.is_empty());
}

#[test]
fn the_archipelago_replays_from_receipts_alone() {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("settles");
    let daughter = world.chart("far-anchor", FIRST_ORB).expect("charters");
    world.zone_as(SOVEREIGN, daughter, 4, ZoneKind::Extractor).expect("builds");
    for _ in 0..3 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, daughter).expect("settles the daughter");
    world.raze_as(SOVEREIGN, daughter, 4).expect("razes");
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "two planets, one chain — the whole archipelago replays"
    );
}
