//! **THE PROVISIONING LAYER — the granary gate** (`43_DISTRIBUTION`,
//! task #45). The granary is a buildable infrastructure zone whose
//! PRESENCE flips the provisioned economy on. It holds no stock of its
//! own (the depot is the orb's banked ration, ruled) and yields nothing —
//! it is the law's unlock, the REGEN-1 precedent. These witnesses pin the
//! gate: it builds like a habitat, produces nothing, and `has_granary`
//! reads it only while it stands active.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A mesic world with biomass banked to build on — enough to raise a
/// granary and still stand a ration.
fn provisioned_world() -> World {
    let mut world = World::new();
    world.found("granary-hold").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites in the mesic band");
    let stamp = world.now();
    world.chronicle_mut().append(
        ClosureEvent::YieldCollected {
            orb_id: FIRST_ORB,
            resource: Resource::Biomass,
            amount: ratio(100, 1),
        },
        stamp,
        vec![SOVEREIGN],
    );
    World::resume(world.chronicle().clone())
}

#[test]
fn the_granary_rides_the_habitat_bill_and_produces_nothing() {
    // Its economics DERIVE from the habitat (a ration house): same cost,
    // same upkeep line; and it yields nothing — infrastructure, not a
    // producer.
    assert_eq!(
        ZoneKind::Granary.cost(),
        ZoneKind::Habitat.cost(),
        "a granary costs the founding ration bill, like the habitat"
    );
    assert_eq!(
        ZoneKind::Granary.yield_per_tick().1,
        ratio(0, 1),
        "a granary produces nothing — it is the provisioning law's unlock"
    );
    assert_eq!(
        ZoneKind::Granary.upkeep_per_tick(),
        ZoneKind::Habitat.upkeep_per_tick(),
        "and is maintained like the ration house it is"
    );
}

#[test]
fn the_granary_wire_code_round_trips_and_is_not_minted() {
    // Code 11, the next compiled code past the ship-yard — a compiled
    // kind, never read as a minted one.
    assert_eq!(ZoneKind::Granary.wire(), 11, "the granary is compiled code 11");
    assert_eq!(ZoneKind::from_wire(11), Some(ZoneKind::Granary), "11 decodes to the granary");
    assert!(ZoneKind::Granary.compiled(), "it is a compiled kind, not minted");
    // The first minted code now floats one higher, above the granary.
    assert!(
        matches!(ZoneKind::from_wire(12), Some(ZoneKind::Minted(12))),
        "minted zone codes float above the compiled table"
    );
}

#[test]
fn has_granary_reads_the_gate_and_replay_matches() {
    let mut world = provisioned_world();
    assert!(!world.has_granary(FIRST_ORB), "a bare world stands no granary");
    world.zone(0, ZoneKind::Granary).expect("builds a granary like any zone");
    assert!(world.has_granary(FIRST_ORB), "the built granary flips the gate on");
    // The gate is derived from committed state — a fresh resume reads it
    // identically, and the chronicle replays byte-exact.
    let resumed = World::resume(world.chronicle().clone());
    assert!(resumed.has_granary(FIRST_ORB), "the gate survives a resume");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
