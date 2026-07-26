//! D2 witnesses — the industrial pipeline (02 §5): exact conversion,
//! the starved synthesizer, multi-resource costs, and replay through
//! the tier climb.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World};

/// Play to a running smelter: 2E+2H spend the whole bootstrap; ten
/// ticks and a collect afford the smelter (15 iron + 10 biomass).
fn smelting_colony() -> World {
    let mut world = World::new();
    world.found("forge").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("10 iron");
    world.zone(1, ZoneKind::Extractor).expect("10 iron");
    world.zone(8, ZoneKind::Habitat).expect("5 biomass");
    world.zone(9, ZoneKind::Habitat).expect("5 biomass");
    // 2E+2H nets: iron +3 − 1/2 = +5/2; biomass +2 − 1 = +1.
    for _ in 0..10 {
        world.tick();
    }
    world.collect().expect("25 iron, 10 biomass settle");
    world.zone(16, ZoneKind::Smelter).expect("15 iron + 10 biomass, earned");
    world
}

#[test]
fn the_smelter_converts_exactly() {
    let mut world = smelting_colony();
    // 2E+2H+1S nets/tick: iron +3 − 1/2 − 2 = +1/2; biomass +2 − 1 −
    // 1/2 = +1/2; alloy +1. Four ticks, exactly.
    for _ in 0..4 {
        world.tick();
    }
    assert_eq!(
        world.accrued(),
        &[
            (Resource::Iron, ratio(2, 1)),
            (Resource::Biomass, ratio(2, 1)),
            (Resource::Alloy, ratio(4, 1)),
        ]
    );
    world.collect().expect("the pipeline's first alloy banks");
    let orb = world.orb().expect("stands");
    assert_eq!(orb.banked_amount(Resource::Alloy), ratio(4, 1));
    assert!(orb.famine.is_empty(), "a fed smelter starves nothing");
}

#[test]
fn the_synthesizer_demands_alloy_it_cannot_forge_alone() {
    // 02 §5.2: one synthesizer eats FOUR smelters' output. Built over a
    // single smelter, the alloy store runs dry within a tick — famine
    // strikes ALLOY and the synthesizer goes dormant. The pipeline
    // teaches its own ratios; no special "production stalled" machinery
    // exists (02 §5.3).
    let mut world = smelting_colony();
    // Earn the synthesizer's 20 alloy + 15 biomass with a third
    // habitat and patience.
    for _ in 0..10 {
        world.tick();
    }
    world.collect().expect("settles");
    world.zone(10, ZoneKind::Habitat).expect("5 biomass");
    // 2E+3H+1S nets/tick: iron +3 − 3/4 − 2 = +1/4; biomass +3 − 1 −
    // 1/2 = +3/2; alloy +1.
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("settles");
    let orb = world.orb().expect("stands");
    assert!(orb.banked_amount(Resource::Alloy) >= ratio(20, 1));
    assert!(orb.banked_amount(Resource::Biomass) >= ratio(15, 1));
    world.zone(24, ZoneKind::Synthesizer).expect("earned");
    // Banked alloy after the build is near zero; the synthesizer nets
    // alloy −3/tick against it: famine within ticks, synthesizer
    // dormant, everything else still turning.
    let mut alloy_famine = false;
    for _ in 0..3 {
        let outcome = world.tick();
        if outcome.struck.contains(&(1, Resource::Alloy)) {
            alloy_famine = true;
            break;
        }
    }
    assert!(alloy_famine, "the under-fed synthesizer starves its own input");
    let orb = world.orb().expect("stands");
    let synthesizer = orb.zones.iter().find(|zone| zone.tile == 24).expect("built");
    assert!(!orb.zone_active(synthesizer), "dormant until the alloy recovers");
    let smelter = orb.zones.iter().find(|zone| zone.tile == 16).expect("built");
    assert!(orb.zone_active(smelter), "the smelter keeps forging");
}

#[test]
fn multi_resource_costs_refuse_on_the_exact_shortfall() {
    let mut world = World::new();
    world.found("forge").expect("founding closes");
    // Bootstrap holds 20 iron but the smelter also needs 10 biomass —
    // spend biomass down first and watch the refusal name it.
    world.zone(8, ZoneKind::Habitat).expect("5 biomass");
    world.zone(9, ZoneKind::Habitat).expect("5 biomass");
    assert_eq!(
        world.zone(16, ZoneKind::Smelter),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Biomass,
            need: Box::new(ratio(10, 1)),
            have: Box::new(ratio(0, 1)),
        })
    );
    // A synthesizer before any alloy exists refuses on ALLOY.
    assert_eq!(
        world.zone(17, ZoneKind::Synthesizer),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Alloy,
            need: Box::new(ratio(20, 1)),
            have: Box::new(ratio(0, 1)),
        })
    );
}

#[test]
fn the_tier_climb_replays_from_receipts_alone() {
    let mut world = smelting_colony();
    for _ in 0..6 {
        world.tick();
    }
    world.collect().expect("settles");
    world.raze(1).expect("sacrifice an extractor");
    world.tick();
    world.collect().expect("settles");
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "multi-resource costs, refunds, and conversion all replay"
    );
}
