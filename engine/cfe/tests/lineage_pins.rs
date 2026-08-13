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

// ── THE EVOLUTIONARY ARC (§2c) — a line transforms over the game ──

/// Grant the deepening (condition 3) on the genesis orb — the earned
/// technology door evolution passes, the same the regenerator/refinery use.
fn crystallize_deepening(world: &mut World) {
    world.grant_tech_as(SOVEREIGN, FIRST_ORB, 3).expect("grants the deepening");
}

#[test]
fn evolution_is_gated_earned_and_replay_exact() {
    let mut world = lineage(3, 1, 1, 10); // Crystalline-Core
    assert_eq!(world.evolution_stage(), 0, "a line founds at stage 0");

    // UNGATED: an un-deepened line cannot evolve — the arc is earned.
    assert!(world.evolve().is_err(), "evolution refuses without the crystallized deepening");
    assert_eq!(world.evolution_stage(), 0, "a refused evolution advances nothing");

    // Crystallize the technology door, then the arc advances.
    let mut world = World::resume(world.chronicle().clone());
    crystallize_deepening(&mut world);
    let mut world = World::resume(world.chronicle().clone());
    assert_eq!(world.evolve().expect("deepened — evolution proceeds"), 1, "the first advance reaches stage 1");
    assert_eq!(world.evolve().expect("advances again"), 2, "and stage 2");
    assert_eq!(world.evolution_stage(), 2, "the stage stands where the arc left it");
    assert_eq!(replay(world.chronicle()), world.committed(), "evolution replay MATCHES");
}

#[test]
fn an_evolved_line_sustains_a_larger_population() {
    // Each stage is earned CAPACITY: an advanced civilization provisions
    // more people per world. The ceiling rises with the arc.
    let mut base = lineage(3, 1, 1, 10);
    let k0 = base.population_capacity(FIRST_ORB);
    crystallize_deepening(&mut base);
    let mut evolved = World::resume(base.chronicle().clone());
    evolved.evolve().expect("advances");
    evolved.evolve().expect("advances");
    let k2 = evolved.population_capacity(FIRST_ORB);
    assert!(k2 > k0, "two stages of evolution lift the population ceiling: {k2} > {k0}");
}

#[test]
fn the_evolution_deed_round_trips_on_the_wire() {
    for stage in [1u8, 7, 42, 255] {
        let deed = ClosureEvent::EvolutionAdvanced { stage };
        let bytes = deed.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert_eq!(tag, 169, "EvolutionAdvanced is tag 169");
        assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    }
}

// ── CASTES (§2d) — the population differentiates by identity ──

#[test]
fn castes_partition_the_population_by_disposition() {
    use cfe::dop::Caste;
    // Population 10 splits cleanly on the banded weights: the sitting
    // disposition takes 6 of its own caste, the other two take 2 each.
    let cases = [
        (1u8, Caste::Labour),    // expansionist → mostly Labour
        (2, Caste::Cognition),   // empiricist   → mostly Cognition
        (3, Caste::Artisan),     // epicurean    → mostly Artisan
    ];
    for (disposition, majority) in cases {
        let world = lineage(disposition, 1, 1, 10);
        let split = world.caste_split(FIRST_ORB);
        assert_eq!(split.iter().sum::<u64>(), world.population_of(FIRST_ORB), "the caste split conserves every head");
        assert_eq!(world.caste_of(FIRST_ORB, majority), 6, "disposition {disposition} is mostly its own caste");
        for caste in Caste::ALL {
            if caste != majority {
                assert_eq!(world.caste_of(FIRST_ORB, caste), 2, "the minority castes take an equal fifth each");
            }
        }
    }
}

#[test]
fn the_same_headcount_morphs_to_each_identity() {
    // The SAME ten people, three civilizations — three different societies.
    let expansionist = lineage(1, 1, 1, 10).caste_split(FIRST_ORB);
    let empiricist = lineage(2, 1, 1, 10).caste_split(FIRST_ORB);
    let epicurean = lineage(3, 1, 1, 10).caste_split(FIRST_ORB);
    assert_eq!(expansionist, [6, 2, 2], "the expansionist is a workforce");
    assert_eq!(empiricist, [2, 6, 2], "the empiricist is a think-tank");
    assert_eq!(epicurean, [2, 2, 6], "the epicurean is an atelier");
    assert!(
        expansionist != empiricist && empiricist != epicurean && expansionist != epicurean,
        "one headcount, three distinct societies"
    );
}

#[test]
fn an_undifferentiated_line_is_all_labour() {
    // A people with no sealed disposition has not specialized — every hand
    // is a working hand.
    let mut world = World::new();
    world.found("unsealed").expect("founds");
    world.settle_population(FIRST_ORB, 12);
    assert_eq!(
        world.caste_split(FIRST_ORB),
        [12, 0, 0],
        "no identity, no castes — all Labour"
    );
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
