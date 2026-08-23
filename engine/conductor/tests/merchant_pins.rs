//! MERCHANT-1 witnesses — the arbitrage pass is STRICTLY ADDITIVE:
//! it bridges only pairs the iceberg has walled, pays its own melt
//! on both legs, profits by the exact rational difference, and every
//! leg lands as an ordinary clear_trade (melt, void share, both
//! books receipted). No wall → the caravan stays home.

use cfe::dop::{exchange, positive, ClosureEvent, HullRole, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::FIRST_ORB;
use conductor::{merchant_step, Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("merchant-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn seat(sigil: &str, x: i64) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: sigil.to_string(),
            sigil: sigil.to_string(),
            species: "caravan witness".into(),
            clade: String::new(),
            motto: "the walled trade flows".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hold"),
        planet: format!("{sigil}-p"),
        colony: format!("{sigil}-c"),
        policy: Policy::Steady,
        position: (ratio(x, 1), ratio(0, 1), ratio(0, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(1, 4)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

fn banked(polity: &Polity, resource: Resource) -> Ratio {
    polity.world.orb().unwrap().banked_amount(resource)
}

/// FLEET-1: a positive-melt haul needs hull capacity (`world/mod.rs`
/// haul_cargo). Every caravan leg is a positive-melt haul on BOTH sides,
/// so each trading world carries a fleet. Grade-0 Haulers are the clean
/// unit: `hull_eta(Hauler, 0) = 0`, so the melt the pin states is the
/// melt that runs, and `free_capacity` SUMS them — `haulers` grade-0
/// hulls give exactly `4·haulers` capacity with zero arithmetic drift.
fn equip_haulers(polity: &mut Polity, haulers: usize) {
    polity.world.zone(40, ZoneKind::ShipYard).expect("yard");
    for _ in 0..haulers {
        polity
            .world
            .salvage_scrap(FIRST_ORB, positive(4, 1))
            .expect("scrap salvaged from iron");
        polity
            .world
            .mint_hull(FIRST_ORB, HullRole::Hauler, 0)
            .expect("grade-0 hauler");
    }
}

#[test]
fn the_caravan_bridges_the_walled_pair_at_a_profit() {
    let root = scratch("bridge");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    // The geometry: A and B sit 70 apart — walled direct (70/64 ≥ 1);
    // M sits midway, 35 to each (melt 35/64 per leg).
    let mut miner = Polity::genesis(seat("lodefar", 0), &registry, &book);
    let mut grower = Polity::genesis(seat("mossfar", 70), &registry, &book);
    let mut caravan = Polity::genesis(seat("waymid", 35), &registry, &book);
    miner.world.zone(0, ZoneKind::Extractor).expect("zones");
    miner.world.zone(1, ZoneKind::Extractor).expect("zones");
    // A Habitat stages the biomass the ShipYard build costs — the miner
    // still sells iron; this only lets it afford a yard for its fleet.
    miner.world.zone(2, ZoneKind::Habitat).expect("zones");
    grower.world.zone(0, ZoneKind::Habitat).expect("zones");
    grower.world.zone(1, ZoneKind::Habitat).expect("zones");
    // The grower hauls iron home on the return leg, so it needs iron to
    // salvage into a hull — an Extractor stages it alongside the biomass.
    grower.world.zone(2, ZoneKind::Extractor).expect("zones");
    caravan.world.zone(0, ZoneKind::Extractor).expect("zones");
    caravan.world.zone(8, ZoneKind::Habitat).expect("zones");
    for world in [&mut miner.world, &mut grower.world, &mut caravan.world] {
        for tick in 1..=60 {
            world.tick();
            if tick % 10 == 0 {
                let _ = world.collect();
            }
        }
    }
    assert!(banked(&miner, Resource::Iron) >= ratio(12, 1), "iron staged");
    assert!(banked(&grower, Resource::Biomass) >= ratio(12, 1), "biomass staged");
    assert!(banked(&caravan, Resource::Biomass) >= ratio(4, 1), "the caravan's stake");

    // FLEET-1: each world carries a fleet to haul across the positive-melt
    // legs. A resting seller ships its whole 12-unit offer, so three
    // grade-0 haulers (capacity 4·3 = 12) cover the largest leg; η stays 0
    // so the profit pin below is unmoved. Equipped BEFORE the stake is
    // captured, so the scrap it spends never enters the caravan's measured
    // round-trip.
    equip_haulers(&mut miner, 3);
    equip_haulers(&mut grower, 3);
    equip_haulers(&mut caravan, 3);

    // The resting asks: A sells 12 iron for 4 biomass; B sells 12
    // biomass for 4 iron. Direct they are WALLED.
    miner
        .world
        .post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(12, 1), positive(4, 1), 1)
        .expect("posts");
    grower
        .world
        .post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(12, 1), positive(4, 1), 2)
        .expect("posts");

    let stake_biomass = banked(&caravan, Resource::Biomass);
    let stake_iron = banked(&caravan, Resource::Iron);
    let mut board = vec![miner, grower, caravan];
    let crossings = merchant_step(&mut board, 7);
    assert_eq!(
        crossings,
        vec![("waymid-hold".into(), "lodefar-hold".into(), "mossfar-hold".into())],
        "the caravan bridges the walled pair"
    );
    let caravan = &board[2];
    // The profit is the receipts' exact arithmetic. The two legs do NOT
    // melt equally: `route_melt` folds each pair's orbital phase (D_orbit)
    // into the chart distance, so A→M and M→B carry different melts even
    // though both span 35 of chart — and the biomass and iron round-trips
    // profit by different exact rationals. Both are POSITIVE: the caravan
    // clears the walled pair at a gain, which is the law under test. (The
    // earlier symmetric 23/16 pin assumed pure 35/64 chart melt, before
    // the orbital-phase term entered the distance.)
    //
    // #138 THE RESOURCE GRAIN — the aperture delivery now floors to the 1/1024
    // resource grain (`smelt_resource`), so each melted leg lands on the grain
    // and the round-trip profits at ON-GRAIN rationals. GALACTIC-1's regional
    // gradient re-seated the melt route's terms again after #138 landed (its
    // own 55/64 pin went stale the same way #138's landing re-seated the pin
    // before it, per the law-witness-over-exact-pins ruling: an exact pin
    // tracks a derivation, not an invariant, and re-seats when the derivation
    // does) — this pin is the CURRENT on-grain measurement. Both legs still
    // strictly POSITIVE — the law is unmoved; only the exact value re-seats.
    assert_eq!(
        banked(caravan, Resource::Biomass) - stake_biomass,
        ratio(469, 512),
        "the biomass round-trip profits exactly, on the grain"
    );
    assert_eq!(
        banked(caravan, Resource::Iron) - stake_iron,
        ratio(459, 512),
        "the iron leftover is inventory, banked on the grain"
    );
    // Both legs are ordinary law: the void's share receipted on the
    // receiving books, offers cleared on all three.
    let void_shares = board
        .iter()
        .flat_map(|e| e.world.chronicle().receipts())
        .filter(|r| matches!(r.event(), ClosureEvent::VoidShare { .. }))
        .count();
    assert!(void_shares >= 4, "every melted leg receipts the void's share");

    // And with no wall there is no caravan: near worlds clear direct,
    // the merchant stays home.
    let root = scratch("no-wall");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut near_a = Polity::genesis(seat("nearlode", 0), &registry, &book);
    let mut near_b = Polity::genesis(seat("nearmoss", 10), &registry, &book);
    let near_m = Polity::genesis(seat("nearway", 5), &registry, &book);
    near_a
        .world
        .post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(12, 1), positive(4, 1), 1)
        .expect("posts");
    near_b
        .world
        .post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(12, 1), positive(4, 1), 2)
        .expect("posts");
    let mut near = vec![near_a, near_b, near_m];
    assert!(merchant_step(&mut near, 8).is_empty(), "no wall, no caravan — strictly additive");
}
