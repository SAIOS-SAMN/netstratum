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
    let k = world.ration_capacity_of(FIRST_ORB);
    assert!(k > ratio(100, 1), "a lush world holds a large K: {k}");
    // A world with no living zones holds no natural biomass ceiling.
    let bare = greenhold(0);
    assert_eq!(bare.ration_capacity_of(FIRST_ORB), ratio(0, 1), "no footprint, no capacity");
}

#[test]
fn a_scan_shows_biomass_climbing_toward_capacity_over_time() {
    let world = greenhold(8);
    let now = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let k = world.ration_capacity_of(FIRST_ORB);
    assert!(now < k, "the young colony sits below its lush ceiling: {now} < {k}");

    // At the present beat, the scan reads today's stock unchanged.
    assert_eq!(world.regrown_ration(FIRST_ORB, world.now()), now, "no time, no growth");

    // As beats pass the scan reads MORE, monotonically, never past K.
    let mut last = now.clone();
    for stamp in [10u64, 100, 1_000, 100_000] {
        let grown = world.regrown_ration(FIRST_ORB, stamp);
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
    assert_eq!(world.ration_capacity_of(FIRST_ORB), ratio(0, 1), "the furnace bears no life");
    let now = world.orb().expect("stands").banked_amount(Resource::Biomass);
    // Over capacity (any stock over K=0) settles toward zero, never up.
    assert!(world.regrown_ration(FIRST_ORB, 1_000_000) <= now, "a rock does not bloom");
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
        let _ = world.regrown_ration(FIRST_ORB, stamp);
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

// ── the SETTLE: regrowth banks at a real collect touch ──

#[test]
fn a_collect_banks_the_regrown_ration_folded_into_its_yield() {
    // A depleted mesic world, left to run: at the collect touch its
    // natural biomass has climbed, and that growth banks WITH the farmed
    // yield — one receipt, no separate deed.
    let mut world = greenhold(8);
    for _ in 0..50 {
        world.tick();
    }
    let before = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let farmed: cfe::ratio::Ratio = world
        .accrued()
        .iter()
        .filter(|(resource, _)| *resource == Resource::Biomass)
        .map(|(_, amount)| amount.clone())
        .fold(ratio(0, 1), |sum, amount| sum + amount);
    let grown = world.regrown_ration(FIRST_ORB, world.now()) - before.clone();
    assert!(grown > ratio(0, 1), "a depleted mesic world has regrowth to settle: {grown}");

    world.collect().expect("collects");
    let after = world.orb().expect("stands").banked_amount(Resource::Biomass);
    assert_eq!(
        after,
        before + farmed + grown,
        "the collect banks farmed yield AND the regrown stock, in one touch"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_world_at_capacity_regrows_nothing_at_the_touch() {
    // Rejuvenation only aids RECOVERY: a stock already at or above its
    // ceiling settles no growth (farming may still push it higher, but
    // the natural law adds nothing above K).
    let world = greenhold(8);
    let k = world.ration_capacity_of(FIRST_ORB);
    assert!(k > ratio(0, 1), "a lush world has a ceiling");
    // Nothing has elapsed and the stock sits below K, but the SETTLE at
    // t=last_touch is zero regardless (no time, no growth).
    assert_eq!(world.rejuvenation_settle(FIRST_ORB, world.now()), None, "no time, no settle");
    // And an over-capacity stock never settles growth, however long it waits.
    // (Read the curve directly: a stock at K stays at K.)
    assert_eq!(
        cfe::almanac::grained_logistic(&k, &k, &cfe::almanac::intrinsic_rate(2, None), 10_000),
        k,
        "at capacity, the curve holds — the settle would bank nothing"
    );
}

// ── QUARANTINE (task #30): a sealed world starves instead of recovering ──

/// A lush mesic greenhold whose bio-pressure has been driven past the
/// quarantine threshold (cascade strikes by receipt), so it is sealed.
fn quarantined_greenhold(habitats: u16) -> World {
    let mut world = World::new();
    world.found("sealed-hold").expect("founds");
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
    for _ in 0..World::QUARANTINE_PRESSURE {
        world.chronicle_mut().append(
            ClosureEvent::ScienceExploded {
                orb_id: FIRST_ORB,
                path: 0,
                kind: cfe::almanac::ExplosionKind::BioChemCross.as_wire(),
                resource: Resource::Biomass as u8,
                slash: ratio(0, 1),
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..habitats {
        world.zone(tile, ZoneKind::Habitat).expect("builds a habitat");
    }
    world
}

#[test]
fn a_quarantined_world_starves_its_ration_at_the_touch() {
    let mut world = quarantined_greenhold(8);
    assert!(world.world_quarantined(), "the world is sealed by bio-pressure");
    for _ in 0..50 {
        world.tick();
    }
    let before = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let farmed: cfe::ratio::Ratio = world
        .accrued()
        .iter()
        .filter(|(resource, _)| *resource == Resource::Biomass)
        .map(|(_, amount)| amount.clone())
        .fold(ratio(0, 1), |sum, amount| sum + amount);
    // A sealed world DECAYS where a free one would regrow.
    let (resource, decay) = world
        .quarantine_decay(FIRST_ORB, world.now())
        .expect("a sealed world starves");
    assert_eq!(resource, Resource::Biomass);
    assert!(decay > ratio(0, 1), "the ration is decaying: {decay}");
    // The TOUCH settles a starving DEBIT, not a recovery credit — the
    // quarantine branch overrides regrowth for a sealed world.
    assert_eq!(
        world.ration_touch_delta(FIRST_ORB, world.now()),
        Some((Resource::Biomass, -decay.clone())),
        "a sealed world's touch is a debit, never a credit"
    );

    world.collect().expect("collects — quarantine seals TRADE, not internal yield");
    let after = world.orb().expect("stands").banked_amount(Resource::Biomass);
    assert_eq!(
        after,
        before + farmed - decay,
        "the sealed world banks farmed yield MINUS the starving decay"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
