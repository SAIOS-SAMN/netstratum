//! **SIX CANONICAL LINEAGES** (operator directive 2026-08-13) — that the
//! line space is NOT three archetypes but the CROSS-PRODUCT of the axes
//! (disposition × metabolism × perception × class × clade), each axis
//! open-ended via minting. These witnesses take six richly distinct
//! civilizations the operator named and pin each to a concrete coordinate
//! in the SEEDED axis space, proving they are already representable and
//! that provisioning tells them apart — a different food, a different
//! lifespan, a different luxury appetite — with no new mechanism.
//!
//! The map (disposition · metabolism · perception · class):
//!   Vapor-Lattice    empiricist · plasmic · harmonic  · plasma-field(6)
//!   Basal-Silt       epicurean  · mineral · volumetric· abyssal-pressure(9)
//!   Crystalline-Core epicurean  · mineral · planar    · crystal-mind(10)
//!   Thermo-Solder    expansionist·plasmic · planar    · thermal-banker(12)
//!   Spore-Matrix     epicurean  · biotic  · harmonic  · biotic-swarm(1)
//!   Void-Strand      expansionist·biotic  · umbral    · void-swarm(11)
//!
//! HONEST LIMITS this exposes (reported, not hidden): the two PLASMIC
//! lineages (Vapor-Lattice, Thermo-Solder) have no ration-path population
//! yet — a plasmic line captures heat rather than farming (harvest rate
//! 0), so its people ride the energy economy (#29, pending). Their
//! provisioning READINGS are distinct here; their growth waits on #29.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// Seal a line at a coordinate: `disposition · metabolism · perception`
/// (the civ axes) and a `class` (the taxon archetype → lifespan). A
/// granary stands so the provisioned economy is in force.
fn lineage(disposition: u8, metabolism: u8, perception: u8, class: u8) -> World {
    let mut world = World::new();
    world.found("lineage").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites mesic");
    let stamp = world.now();
    for (resource, amount) in [(Resource::Biomass, 400i64), (Resource::Iron, 100_000)] {
        world.chronicle_mut().append(
            ClosureEvent::YieldCollected { orb_id: FIRST_ORB, resource, amount: ratio(amount, 1) },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..6 {
        world.zone(tile, ZoneKind::Habitat).expect("habitat");
    }
    world.zone(50, ZoneKind::Granary).expect("granary");
    world.seal_genesis("civ", disposition, metabolism, perception).expect("seals genesis");
    world.seal_taxon(class, 1).expect("seals taxon");
    world.settle_population(FIRST_ORB, 10);
    world
}

/// A lineage's PROVISIONING SIGNATURE — the tuple that must differ line
/// to line: the leisure good it savours, whether it is long-lived, and
/// its luxury appetite.
fn signature(world: &World) -> (Option<Resource>, bool, cfe::ratio::Ratio) {
    let long_lived = world.lifespan_of() > cfe::almanac::REGROWTH_GRAIN as u64;
    (world.leisure_of(FIRST_ORB), long_lived, world.provision_leisure_factor())
}

#[test]
fn the_six_lineages_each_hold_a_distinct_provisioning_signature() {
    let vapor = lineage(2, 3, 3, 6);
    let basal = lineage(3, 1, 2, 9);
    let crystalline = lineage(3, 1, 1, 10);
    let thermo = lineage(1, 3, 1, 12);
    let spore = lineage(3, 2, 3, 1);
    let void = lineage(1, 2, 4, 11);

    // Each line's signature is the one its substrate dictates.
    let alloy = Some(Resource::Alloy);
    let diamond = Some(Resource::Diamond);
    let crystal = Some(Resource::Crystal);
    let lean = ratio(0, 1);
    let base = ratio(1, 1);
    let deep = ratio(2, 1);

    assert_eq!(signature(&vapor), (crystal, true, base.clone()), "Vapor-Lattice: plasmic savours crystal, long, empiricist");
    assert_eq!(signature(&basal), (alloy, false, deep.clone()), "Basal-Silt: mineral savours alloy, short, epicurean");
    assert_eq!(signature(&crystalline), (alloy, true, deep.clone()), "Crystalline-Core: mineral, long-lived, epicurean");
    assert_eq!(signature(&thermo), (crystal, true, lean.clone()), "Thermo-Solder: plasmic, long, expansionist-lean");
    assert_eq!(signature(&spore), (diamond, false, deep), "Spore-Matrix: biotic savours diamond, short, epicurean");
    assert_eq!(signature(&void), (diamond, false, lean), "Void-Strand: biotic, short, expansionist-lean");

    // And every one of the six signatures is DISTINCT from the others —
    // the space is not three buckets.
    let sigs = [
        signature(&vapor),
        signature(&basal),
        signature(&crystalline),
        signature(&thermo),
        signature(&spore),
        signature(&void),
    ];
    for i in 0..sigs.len() {
        for j in (i + 1)..sigs.len() {
            assert_ne!(sigs[i], sigs[j], "lineages {i} and {j} must not share a provisioning signature");
        }
    }
}

#[test]
fn the_farming_lineages_grow_and_replay_byte_exact() {
    // The four mineral/biotic lines run the full provisioned population
    // loop; each grows a fed people and replays byte-exact. (The two
    // plasmic lines ride #29 and are asserted separately below.)
    for (disposition, metabolism, perception, class) in
        [(3u8, 1u8, 2u8, 9u8), (3, 1, 1, 10), (3, 2, 3, 1), (1, 2, 4, 11)]
    {
        let mut world = lineage(disposition, metabolism, perception, class);
        for _ in 0..5 {
            for _ in 0..8 {
                world.tick();
            }
            let _ = world.collect();
        }
        assert!(world.population_of(FIRST_ORB) > 0, "a farming lineage endures");
        assert_eq!(replay(world.chronicle()), world.committed(), "lineage replay MATCHES");
    }
}

#[test]
fn a_plasmic_lineage_defers_its_population_to_the_energy_economy() {
    // HONEST GAP: a plasmic capture line (Vapor-Lattice / Thermo-Solder)
    // farms nothing, so the ration-path population loop does not drive it —
    // its people ride #29. Its provisioning readings still stand distinct;
    // the growth is what waits.
    let vapor = lineage(2, 3, 3, 6);
    assert_eq!(
        vapor.population_touch_delta(FIRST_ORB, vapor.now() + 8),
        None,
        "a plasmic line's population rides the energy economy (#29), not the ration path"
    );
    assert_eq!(vapor.leisure_of(FIRST_ORB), Some(Resource::Crystal), "but its leisure reading is distinct");
}
