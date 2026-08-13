//! **PROVISION — THE FOOD-IDENTITY LAW** (`41_PROVISION` §1, task #38).
//!
//! A civilization eats what its SPECIES is, never "food" in the
//! abstract. The sealed metabolism is the identity and the ration
//! follows it — so the plasmoid eats captured energy, the lithovore
//! eats ore, and no mechanic may re-hardcode a single food. These
//! witnesses pin that identity TOTAL over the sealed metabolisms, so a
//! regression that reintroduced a biomass assumption goes red here.

use cfe::dop::{
    metabolic_harvest_of, metabolic_ration_of, Ration, Resource,
};
use cfe::ratio::ratio;

/// An element id (a stone-eater's minted ration).
fn stone() -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = 1;
    id[4] = 2;
    id
}

#[test]
fn the_ration_is_the_species_not_a_single_food() {
    // Each compiled metabolism eats a DISTINCT closed carrier — the
    // food is decided by what the line IS.
    assert_eq!(
        metabolic_ration_of(1, None),
        Ration::Resource(Resource::Iron),
        "a mineral line (lithovore) eats ore"
    );
    assert_eq!(
        metabolic_ration_of(2, None),
        Ration::Resource(Resource::Biomass),
        "a biotic line eats a field"
    );
    assert_eq!(
        metabolic_ration_of(3, None),
        Ration::Resource(Resource::Joule),
        "a plasmic line (plasmoid) eats captured ENERGY, never grass"
    );
    // A minted element-eater eats its OWN element, no closed resource.
    assert_eq!(
        metabolic_ration_of(4, Some(stone())),
        Ration::Element(stone()),
        "a stone-eater eats its own minted stone"
    );
}

#[test]
fn the_plasmoid_farms_nothing_it_captures_heat() {
    // The fiat law (04 §1.2 / 05 §1.2): plasmic agriculture IS
    // encasement of real heat — a plasmoid grows no yield line.
    assert_eq!(
        metabolic_harvest_of(3, None).1,
        ratio(0, 1),
        "the plasmoid's harvest rate is zero — it captures, never farms"
    );
    // Every other species farms its own ration at a positive rate.
    assert!(metabolic_harvest_of(1, None).1 > ratio(0, 1), "mineral farms ore");
    assert!(metabolic_harvest_of(2, None).1 > ratio(0, 1), "biotic farms biomass");
    assert!(
        metabolic_harvest_of(4, Some(stone())).1 > ratio(0, 1),
        "the stone-eater farms its stone"
    );
}

#[test]
fn the_ration_and_its_harvest_name_the_same_carrier() {
    // Identity is coherent: a species farms exactly what it eats.
    for (metabolism, element) in [(1u8, None), (2, None), (3, None), (4, Some(stone()))] {
        let eats = metabolic_ration_of(metabolism, element);
        let (farms, _rate) = metabolic_harvest_of(metabolism, element);
        assert_eq!(eats, farms, "a line farms exactly the ration it eats (metabolism {metabolism})");
    }
}

// ── PopDyn 2 (#39): the population as a mutable, settle-safe quantity ──

use cfe::dop::ClosureEvent;
use cfe::world::{replay, World, FIRST_ORB};

#[test]
fn the_population_deed_round_trips_both_directions() {
    for delta in [7i64, -3, 1, -1, 0] {
        let deed = ClosureEvent::PopulationSettled { orb_id: 5, delta };
        let bytes = deed.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert_eq!(tag, 167, "PopulationSettled is tag 167");
        assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    }
}

#[test]
fn settle_population_grows_and_culls_the_elastic_population_replay_safe() {
    let mut world = World::new();
    world.found("hold").expect("founds");
    assert_eq!(world.population_of(FIRST_ORB), 0, "a fresh world has no clone population");
    world.settle_population(FIRST_ORB, 10);
    assert_eq!(world.population_of(FIRST_ORB), 10, "births grow it");
    world.settle_population(FIRST_ORB, -4);
    assert_eq!(world.population_of(FIRST_ORB), 6, "deaths cull it");
    // Floored at zero — a cull never takes it negative.
    world.settle_population(FIRST_ORB, -100);
    assert_eq!(world.population_of(FIRST_ORB), 0, "a line can empty but never go negative");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn lifespan_is_the_grain_for_short_lived_and_four_grains_for_long() {
    let grain = cfe::almanac::REGROWTH_GRAIN as u64;
    let mut swarm = World::new();
    swarm.found("swarm").expect("founds");
    swarm.seal_taxon(1, 1).expect("biotic-swarm");
    assert_eq!(swarm.lifespan_of(), grain, "a biotic swarm lives one grain");

    let mut stone = World::new();
    stone.found("stone").expect("founds");
    stone.seal_taxon(2, 2).expect("geodic-lithovore");
    assert_eq!(stone.lifespan_of(), grain * 4, "a lithovore lives four grains");
    assert!(stone.lifespan_of() > swarm.lifespan_of(), "the stone-being outlives the swarm");
}
