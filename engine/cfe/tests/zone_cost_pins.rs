//! **ZONE BUILD COST — the FOOD-slot substitution, extended to `cost()`**
//! (the `zone_upkeep`/`zone_yield` precedent). A Habitat's build price
//! is the founding ration every metabolism can meet once (`31 §3`):
//! mineral pays iron, biotic pays biomass, structural lines (an
//! Extractor's ore intake) never substitute.

use cfe::dop::{Resource, ZoneKind};
use cfe::world::{replay, World};

const EXPANSIONIST: u8 = 1;
const MINERAL: u8 = 1;
const BIOTIC: u8 = 2;
const PLASMIC: u8 = 3;
const PLANAR: u8 = 1;

#[test]
fn a_plasmic_line_still_builds_its_habitat_in_biomass_not_joule() {
    // Plasmic's ration is Joule at rate 0 (agriculture IS encasement,
    // never a habitat yield) and Joules never enter the founding kit
    // — a plain resource swap would deadlock the FIRST habitat every
    // plasmic line ever tries to build. Gated on the harvest rate, the
    // cost stays at the compiled default so founding is never blocked.
    let mut world = World::new();
    world.found("plasma-hearth").expect("founds");
    world
        .seal_genesis("plasmic-line", EXPANSIONIST, PLASMIC, PLANAR)
        .expect("seals");
    let biomass_before = world.orbs()[0].banked_amount(Resource::Biomass);
    world.zone(8, ZoneKind::Habitat).expect("builds — never CannotAfford on joules it can't hold yet");
    let biomass_after = world.orbs()[0].banked_amount(Resource::Biomass);
    assert!(biomass_after < biomass_before, "a plasmic line's habitat still costs the compiled default, biomass");
}

#[test]
fn a_mineral_line_builds_its_habitat_in_iron_not_biomass() {
    let mut world = World::new();
    world.found("iron-hearth").expect("founds");
    world
        .seal_genesis("iron-line", EXPANSIONIST, MINERAL, PLANAR)
        .expect("seals");
    let iron_before = world.orbs()[0].banked_amount(Resource::Iron);
    let biomass_before = world.orbs()[0].banked_amount(Resource::Biomass);
    world.zone(8, ZoneKind::Habitat).expect("builds");
    let iron_after = world.orbs()[0].banked_amount(Resource::Iron);
    let biomass_after = world.orbs()[0].banked_amount(Resource::Biomass);
    assert!(iron_after < iron_before, "a mineral line's habitat costs its own ration, iron");
    assert_eq!(biomass_after, biomass_before, "biomass is untouched — mineral never spends it");
}

#[test]
fn a_biotic_line_still_builds_its_habitat_in_biomass() {
    // The default/unsealed metabolism (2) — byte-identical to before
    // this change, since substituting Biomass -> metabolic_ration(2)
    // is a no-op.
    let mut world = World::new();
    world.found("green-hearth").expect("founds");
    world
        .seal_genesis("biotic-line", EXPANSIONIST, BIOTIC, PLANAR)
        .expect("seals");
    let before = world.orbs()[0].banked_amount(Resource::Biomass);
    world.zone(8, ZoneKind::Habitat).expect("builds");
    let after = world.orbs()[0].banked_amount(Resource::Biomass);
    assert!(after < before, "a biotic line's habitat still costs biomass");
}

#[test]
fn an_extractors_iron_intake_never_substitutes() {
    // Extractor is structural (ore mining), not the FOOD slot — its
    // cost is always Iron, for every metabolism, exactly like its
    // upkeep and yield already were.
    let mut world = World::new();
    world.found("smelter-hearth").expect("founds");
    world
        .seal_genesis("biotic-line", EXPANSIONIST, BIOTIC, PLANAR)
        .expect("seals");
    let iron_before = world.orbs()[0].banked_amount(Resource::Iron);
    world.zone(0, ZoneKind::Extractor).expect("builds");
    let iron_after = world.orbs()[0].banked_amount(Resource::Iron);
    assert!(iron_after < iron_before, "an extractor always costs iron, biotic line or not");
}

#[test]
fn the_metabolism_aware_build_cost_replays_byte_exact() {
    let mut world = World::new();
    world.found("replay-hearth").expect("founds");
    world
        .seal_genesis("iron-line", EXPANSIONIST, MINERAL, PLANAR)
        .expect("seals");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    assert_eq!(replay(world.chronicle()), world.committed(), "zone-cost replay MATCHES");
}
