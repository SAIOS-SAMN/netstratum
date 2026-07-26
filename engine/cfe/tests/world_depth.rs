//! D1 witnesses — the depth board's proofs (BUILD_ORDER §2): the knife
//! edge, the scheduled famine, the raze recovery, and replay through a
//! full crisis.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World};

/// The aggressive opening: 2 extractors + 1 habitat — every bootstrap
/// resource spent to the floor.
fn aggressive_opening() -> World {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("10 iron");
    world.zone(1, ZoneKind::Extractor).expect("10 iron");
    world.zone(8, ZoneKind::Habitat).expect("5 biomass");
    world
}

#[test]
fn the_knife_edge_nets_biomass_exactly_zero() {
    // 02 §2.2: 2E+1H is the equilibrium — biomass yield +1 vs upkeep
    // 2 × 1/2 = net EXACTLY 0, tick after tick. Iron nets +11/4.
    let mut world = aggressive_opening();
    for _ in 0..8 {
        world.tick();
    }
    assert_eq!(
        world.accrued(),
        &[(Resource::Iron, ratio(22, 1))],
        "iron 8 × 11/4 = 22; the biomass entry nets to zero and is not a datum"
    );
    let orb = world.orb().expect("the colony stands");
    assert!(orb.famine.is_empty(), "the knife edge balances — no famine");
}

#[test]
fn building_spends_and_refuses_beyond_means() {
    let mut world = aggressive_opening();
    // Bootstrap iron 20 − 2×10 = 0: the third extractor refuses with
    // the exact shortfall.
    assert_eq!(
        world.zone(2, ZoneKind::Extractor),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Iron,
            need: Box::new(ratio(10, 1)),
            have: Box::new(ratio(0, 1)),
        })
    );
}

#[test]
fn a_third_extractor_strikes_famine_on_schedule() {
    // Earn the third extractor, then run the famine clock: 3E+1H nets
    // biomass −1/2/tick against 5 banked → banked+accrued < 0 first at
    // tick 11. The strike empties the stores and idles the extractors.
    let mut world = aggressive_opening();
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("11 iron banks");
    world.zone(2, ZoneKind::Extractor).expect("earned");
    let mut struck_at = None;
    for tick in 1..=11u32 {
        let outcome = world.tick();
        if !outcome.struck.is_empty() {
            struck_at = Some((tick, outcome.struck.clone()));
            break;
        }
    }
    assert_eq!(
        struck_at,
        Some((11, vec![(1, Resource::Biomass)])),
        "5 − t/2 < 0 first at t = 11 — the famine clock is exact"
    );
    let orb = world.orb().expect("the colony stands");
    assert_eq!(orb.famine, vec![Resource::Biomass]);
    assert_eq!(orb.banked_amount(Resource::Biomass), ratio(0, 1), "the stores emptied");
    // Dormancy: extractors (biomass consumers) idle — a tick now
    // accrues only the habitat's net (+1 biomass, −1/4 iron).
    let before = world.accrued();
    world.tick();
    let habitat_only: Vec<_> = world
        .accrued()
        .iter()
        .map(|(resource, amount)| {
            let prior = before
                .iter()
                .find(|(held, _)| held == resource)
                .map(|(_, amount)| amount.clone())
                .unwrap_or_else(|| ratio(0, 1));
            (*resource, amount.clone() - prior)
        })
        .collect();
    assert_eq!(
        habitat_only,
        vec![(Resource::Iron, ratio(-1, 4)), (Resource::Biomass, ratio(1, 1))],
        "dormant zones neither yield nor drain"
    );
}

#[test]
fn collect_relieves_a_famine_and_the_colony_recovers() {
    let mut world = aggressive_opening();
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("11 iron banks");
    world.zone(2, ZoneKind::Extractor).expect("earned");
    for _ in 0..11 {
        world.tick();
    }
    // Two habitat-only ticks bank 2 biomass ≥ the extractors' one-tick
    // upkeep (3 × 1/2 = 3/2): collect relieves, the extractors wake.
    world.tick();
    world.tick();
    world.collect().expect("the habitat's surplus settles");
    let orb = world.orb().expect("the colony stands");
    assert!(orb.famine.is_empty(), "famine relieved at the collect");
    assert!(orb.banked_amount(Resource::Biomass) >= ratio(3, 2));
    let before = world.accrued();
    world.tick();
    assert_ne!(world.accrued(), before.as_slice(), "the extractors turn again");
}

#[test]
fn raze_refunds_half_and_cuts_the_drain() {
    let mut world = aggressive_opening();
    let iron_before = world.orb().expect("stands").banked_amount(Resource::Iron);
    world.raze(1).expect("the zone exists");
    let orb = world.orb().expect("stands");
    assert_eq!(
        orb.banked_amount(Resource::Iron),
        iron_before + ratio(5, 1),
        "half the 10-iron build cost returns"
    );
    assert_eq!(orb.zones.len(), 2, "the tile cleared");
    // The drain shrank: 1E+1H is the sustainable homestead again.
    for _ in 0..4 {
        world.tick();
    }
    assert!(world.orb().expect("stands").famine.is_empty());
}

#[test]
fn replay_reproduces_a_full_crisis() {
    // The whole arc — aggressive opening, earned third extractor,
    // famine, raze, relief — replayed from receipts alone (SEAL B).
    let mut world = aggressive_opening();
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("banks");
    world.zone(2, ZoneKind::Extractor).expect("earned");
    for _ in 0..11 {
        world.tick();
    }
    world.raze(2).expect("sacrificed");
    world.tick();
    world.collect().expect("settles");
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "costs, refunds, famine, and relief all replay from the ruleset + receipts"
    );
}
