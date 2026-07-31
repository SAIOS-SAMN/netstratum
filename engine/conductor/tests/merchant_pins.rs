//! MERCHANT-1 witnesses — the arbitrage pass is STRICTLY ADDITIVE:
//! it bridges only pairs the iceberg has walled, pays its own melt
//! on both legs, profits by the exact rational difference, and every
//! leg lands as an ordinary clear_trade (melt, void share, both
//! books receipted). No wall → the caravan stays home.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::FIRST_ORB;
use conductor::{merchant_step, Egregore, EgregoreSpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("merchant-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn seat(sigil: &str, x: i64) -> EgregoreSpec {
    EgregoreSpec {
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

fn banked(egregore: &Egregore, resource: Resource) -> Ratio {
    egregore.world.orb().unwrap().banked_amount(resource)
}

#[test]
fn the_caravan_bridges_the_walled_pair_at_a_profit() {
    let root = scratch("bridge");
    let registry = Registry::at(root.join("registry"));
    let mirror = Ledger::at(root.join("worlds"));
    // The geometry: A and B sit 70 apart — walled direct (70/64 ≥ 1);
    // M sits midway, 35 to each (melt 35/64 per leg).
    let mut miner = Egregore::genesis(seat("lodefar", 0), &registry, &mirror);
    let mut grower = Egregore::genesis(seat("mossfar", 70), &registry, &mirror);
    let mut caravan = Egregore::genesis(seat("waymid", 35), &registry, &mirror);
    miner.world.zone(0, ZoneKind::Extractor).expect("zones");
    miner.world.zone(1, ZoneKind::Extractor).expect("zones");
    grower.world.zone(0, ZoneKind::Habitat).expect("zones");
    grower.world.zone(1, ZoneKind::Habitat).expect("zones");
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

    // The resting asks: A sells 12 iron for 4 biomass; B sells 12
    // biomass for 4 iron. Direct they are WALLED.
    miner
        .world
        .post_offer(FIRST_ORB, Resource::Iron, ratio(12, 1), Resource::Biomass, ratio(4, 1), 1)
        .expect("posts");
    grower
        .world
        .post_offer(FIRST_ORB, Resource::Biomass, ratio(12, 1), Resource::Iron, ratio(4, 1), 2)
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
    // Leg melts 35/64 each: paid 4 biomass, bought 12·29/64 = 87/16
    // iron; sold 4 iron for 12 biomass arriving 87/16. Net EXACTLY
    // +23/16 of each — the profit is the receipts' arithmetic.
    assert_eq!(
        banked(caravan, Resource::Biomass) - stake_biomass,
        ratio(23, 16),
        "the quote round-trip profits exactly"
    );
    assert_eq!(
        banked(caravan, Resource::Iron) - stake_iron,
        ratio(23, 16),
        "the leftover cargo is inventory, banked"
    );
    // Both legs are ordinary law: the void's share receipted on the
    // receiving books, offers cleared on all three.
    let void_shares = board
        .iter()
        .flat_map(|e| e.world.chronicle().receipts())
        .filter(|r| matches!(r.event, ClosureEvent::VoidShare { .. }))
        .count();
    assert!(void_shares >= 4, "every melted leg receipts the void's share");

    // And with no wall there is no caravan: near worlds clear direct,
    // the merchant stays home.
    let root = scratch("no-wall");
    let registry = Registry::at(root.join("registry"));
    let mirror = Ledger::at(root.join("worlds"));
    let mut near_a = Egregore::genesis(seat("nearlode", 0), &registry, &mirror);
    let mut near_b = Egregore::genesis(seat("nearmoss", 10), &registry, &mirror);
    let near_m = Egregore::genesis(seat("nearway", 5), &registry, &mirror);
    near_a
        .world
        .post_offer(FIRST_ORB, Resource::Iron, ratio(12, 1), Resource::Biomass, ratio(4, 1), 1)
        .expect("posts");
    near_b
        .world
        .post_offer(FIRST_ORB, Resource::Biomass, ratio(12, 1), Resource::Iron, ratio(4, 1), 2)
        .expect("posts");
    let mut near = vec![near_a, near_b, near_m];
    assert!(merchant_step(&mut near, 8).is_empty(), "no wall, no caravan — strictly additive");
}
