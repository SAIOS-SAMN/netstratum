//! **PLANETARY REJUVENATION — the lazy READ, at the world** (task #28).
//!
//! A planet SCAN folds the ruled S-curve over the beats elapsed since
//! the orb's last touch and shows what its biomass has recovered to —
//! writing nothing, minting no receipt, advancing no clock. These
//! witnesses hold that read: capacity comes from the living footprint
//! and the place, a collapsed lush world climbs back, and looking
//! changes nothing.

use cfe::almanac::ThermalRegion;
use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A world sited in the mesic band under a G star, with `habitats`
/// habitats standing — the lush case. Biomass is topped up by receipt
/// first so the habitats can be built (5 each) and a remnant remains
/// banked below K.
fn greenhold(habitats: u16) -> World {
    let mut world = World::new();
    world.found("greenhold").expect("founds");
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
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..habitats {
        world.zone(tile, ZoneKind::Habitat).expect("builds a habitat");
    }
    world
}

#[test]
fn the_place_and_the_footprint_set_the_capacity() {
    let world = greenhold(8);
    assert_eq!(world.orb_region(FIRST_ORB), Some(ThermalRegion::Mesic), "sited in the life band");
    assert_eq!(world.living_zones_of(FIRST_ORB), 8, "eight habitats are the footprint");
    let k = world.biomass_capacity_of(FIRST_ORB);
    assert!(k > ratio(100, 1), "a lush world holds a large K: {k}");
    // A world with no living zones holds no natural biomass ceiling.
    let bare = greenhold(0);
    assert_eq!(bare.biomass_capacity_of(FIRST_ORB), ratio(0, 1), "no footprint, no capacity");
}

#[test]
fn a_scan_shows_biomass_climbing_toward_capacity_over_time() {
    let world = greenhold(8);
    let now = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let k = world.biomass_capacity_of(FIRST_ORB);
    assert!(now < k, "the young colony sits below its lush ceiling: {now} < {k}");

    // At the present beat, the scan reads today's stock unchanged.
    assert_eq!(world.regrown_biomass(FIRST_ORB, world.now()), now, "no time, no growth");

    // As beats pass the scan reads MORE, monotonically, never past K.
    let mut last = now.clone();
    for stamp in [10u64, 100, 1_000, 100_000] {
        let grown = world.regrown_biomass(FIRST_ORB, stamp);
        assert!(grown >= last, "regrowth is monotone in the scan: {grown} >= {last}");
        assert!(grown <= k, "and never past the ceiling: {grown} <= {k}");
        last = grown;
    }
    assert!(last > now, "over a long wait the stock has genuinely climbed");
}

#[test]
fn a_barren_world_never_regrows_and_a_rock_holds_nothing() {
    // A scorched rock (sited hot, no habitable band) carries no capacity
    // and shows no regrowth however long it waits.
    let mut world = World::new();
    world.found("cinder").expect("founds");
    world.commission_star(StarClass::O).expect("commissions hot");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 2), None, None)
        .expect("sites in the furnace");
    world.zone(0, ZoneKind::Habitat).expect("builds");
    assert_eq!(world.biomass_capacity_of(FIRST_ORB), ratio(0, 1), "the furnace bears no life");
    let now = world.orb().expect("stands").banked_amount(Resource::Biomass);
    // Over capacity (any stock over K=0) settles toward zero, never up.
    assert!(world.regrown_biomass(FIRST_ORB, 1_000_000) <= now, "a rock does not bloom");
}

#[test]
fn a_scan_is_not_a_deed_and_changes_nothing() {
    // Looking at a planet writes no receipt and moves no bank — the
    // read-only-audit law. The chronicle and the committed state are
    // untouched by any number of scans.
    let mut world = greenhold(8);
    let before_receipts = world.chronicle().receipts().len();
    let before_bank = world.orb().expect("stands").banked_amount(Resource::Biomass);
    for stamp in [1u64, 100, 10_000, 1_000_000] {
        let _ = world.regrown_biomass(FIRST_ORB, stamp);
    }
    assert_eq!(
        world.chronicle().receipts().len(),
        before_receipts,
        "a scan mints no receipt"
    );
    assert_eq!(
        world.orb().expect("stands").banked_amount(Resource::Biomass),
        before_bank,
        "and moves no bank — the grown value settles only at a real touch"
    );
    // Belt and braces: a fresh append + resume still replays byte-exact,
    // proving nothing was silently mutated.
    let stamp = world.now();
    world.chronicle_mut().append(
        ClosureEvent::YieldCollected {
            orb_id: FIRST_ORB,
            resource: Resource::Biomass,
            amount: ratio(1, 1),
        },
        stamp,
        vec![SOVEREIGN],
    );
    let world = World::resume(world.chronicle().clone());
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
