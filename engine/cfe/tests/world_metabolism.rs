//! G2 witnesses — METABOLISM TABLES (04 §1.2, §6): ruleset v2 as the
//! ration-substitution law over the shared flows, keyed by the BINDING
//! metabolism (the G1 seal — never identity-only registry data). The
//! axioms measured: an unsealed or biotic world runs ruleset v1
//! byte-identical (constants never change silently); famine strikes
//! the TRUE store; and the joule fiat law holds — plasmic agriculture
//! IS encasement of real heat, never a habitat yield.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

const EXPANSIONIST: u8 = 1;
const MINERAL: u8 = 1;
const BIOTIC: u8 = 2;
const PLASMIC: u8 = 3;
const PLANAR: u8 = 1;

/// One fixed arc: 1E+1H, four ticks — the shared physics every
/// metabolism reads through its own rations.
fn arc(seal: Option<u8>) -> World {
    let mut world = World::new();
    world.found("weighed-hold").expect("founds");
    if let Some(metabolism) = seal {
        world
            .seal_genesis("test-line", EXPANSIONIST, metabolism, PLANAR)
            .expect("seals");
    }
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world
}

#[test]
fn an_unsealed_world_and_a_biotic_seal_run_ruleset_v1_identically() {
    // The no-silent-change axiom: biotic IS the v1 default, and an
    // unsealed world reads v1 — the three accruals match to the byte.
    let unsealed = arc(None);
    let biotic = arc(Some(BIOTIC));
    assert_eq!(unsealed.accrued(), biotic.accrued(), "v1 is the biotic default");
    assert_eq!(
        unsealed.accrued(),
        &[
            (Resource::Iron, ratio(5, 1)),
            (Resource::Biomass, ratio(2, 1)),
        ],
        "1E+1H: iron +5/4/t, biomass +1/2/t — the landed v1 numbers, untouched"
    );
}

#[test]
fn a_mineral_line_eats_iron_and_farms_iron() {
    // The ration substitution: every biomass FOOD component reads
    // iron — the extractor's worker rations drain iron; the habitat
    // farms iron. 1E+1H mineral: iron +3/2 −1/2 (rations) −1/4
    // (habitat upkeep) +1 (habitat harvest) = +7/4/t; biomass never
    // moves.
    let world = arc(Some(MINERAL));
    assert_eq!(
        world.accrued(),
        &[(Resource::Iron, ratio(7, 1))],
        "four ticks at +7/4 — one store, the line's own"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_mesic_band_starves_a_current_sheet() {
    // 04 §1.2: "sites in the Furnace regime, for the mesic band would
    // starve a current sheet." A plasmic line's rations are joules
    // and its habitats farm NOTHING (the fiat law: joules are minted
    // only by encasement) — with no heat captured, the joule store
    // runs dry within a tick and famine strikes the TRUE store.
    let mut world = World::new();
    world.found("cold-sheet").expect("founds");
    world
        .seal_genesis("coronal-line", EXPANSIONIST, PLASMIC, PLANAR)
        .expect("seals");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    let mut struck_joule = false;
    for _ in 0..4 {
        let outcome = world.tick();
        struck_joule = struck_joule
            || outcome.struck.iter().any(|(_, resource)| *resource == Resource::Joule);
    }
    assert!(struck_joule, "no heat, no food — the famine names the true store");
    assert!(
        world
            .orb()
            .expect("stands")
            .famine
            .contains(&Resource::Joule),
        "the strike stands on JOULE, never biomass"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn plasmic_agriculture_is_encasement_of_real_heat() {
    // The vent loop closed: sited hot with encasement, the shell
    // captures overflow into joules (the landed heat law — never
    // fiat) and the captured joules feed the line. The colony that
    // starves cold THRIVES in the Furnace regime.
    let mut world = World::new();
    world.found("coronal-hearth").expect("founds");
    world
        .seal_genesis("coronal-line", EXPANSIONIST, PLASMIC, PLANAR)
        .expect("seals");
    world.commission_star(StarClass::O).expect("commissions hot");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None)
        .expect("sites in the Furnace regime");
    world.zone(0, ZoneKind::Encasement).expect("the shell is the farm");
    world.zone(1, ZoneKind::Encasement).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..8 {
        world.tick();
    }
    let orb = world.orb().expect("stands");
    assert!(
        !orb.famine.contains(&Resource::Joule),
        "captured heat covers the rations — the hot band is HOME"
    );
    let accrued_joules = world
        .accrued()
        .iter()
        .find(|(resource, _)| *resource == Resource::Joule)
        .map(|(_, amount)| amount.clone());
    assert!(
        accrued_joules.is_some_and(|held| held > ratio(0, 1)),
        "the shells net more than the line eats — surplus, not survival"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}
