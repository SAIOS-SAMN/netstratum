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
