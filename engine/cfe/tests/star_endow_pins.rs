//! STAR-ENDOW-1 / ORBIT-ENDOW-1 — kit scales with the sky and place.
//!
//! Founding banks the bronze packet (20 iron / 10 biomass). Commissioning
//! a sun top-ups the genesis orb to the class kit. Siting adds a band
//! gift. Replay matches.

use cfe::dop::{positive, unit, Resource};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

fn founded(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world
}

#[test]
fn g_star_matches_founding_kit() {
    let mut world = founded("sol-kit");
    assert_eq!(
        world.orb().unwrap().banked_amount(Resource::Iron),
        ratio(20, 1)
    );
    world.commission_star(StarClass::G).expect("sun");
    assert_eq!(
        world.orb().unwrap().banked_amount(Resource::Iron),
        ratio(20, 1),
        "G kit equals founding — no mint"
    );
    assert_eq!(
        world.orb().unwrap().banked_amount(Resource::Biomass),
        ratio(10, 1)
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn hot_star_tops_up_metal() {
    let mut world = founded("hot-kit");
    world.commission_star(StarClass::O).expect("O sun");
    let orb = world.orb().unwrap();
    assert_eq!(orb.banked_amount(Resource::Iron), ratio(36, 1));
    assert_eq!(orb.banked_amount(Resource::Biomass), ratio(10, 1)); // founding 10 > kit 8 — never strip
    assert_eq!(orb.banked_amount(Resource::Crystal), ratio(1, 1));
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn m_star_favors_biomass_and_never_strips_iron() {
    let mut world = founded("cool-kit");
    // Founding already has 20 iron; M target is 16 — top-up does not strip.
    world.commission_star(StarClass::M).expect("M sun");
    let orb = world.orb().unwrap();
    assert_eq!(orb.banked_amount(Resource::Iron), ratio(20, 1));
    assert_eq!(orb.banked_amount(Resource::Biomass), ratio(16, 1));
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn mesic_home_adds_no_orbit_gift() {
    let mut world = founded("sited-kit");
    world.commission_star(StarClass::G).expect("sun");
    // G, A=0, r=1 → mesic — founding already assumes this place.
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    assert_eq!(
        world.orb().unwrap().banked_amount(Resource::Biomass),
        ratio(10, 1),
        "mesic home: no extra biomass mint"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn cryogenic_site_seeds_sink() {
    let mut world = founded("cold-site");
    world.commission_star(StarClass::M).expect("M sun");
    // M, A=0, r=1 → cryogenic (thermal pins).
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    assert!(
        world.orb().unwrap().banked_amount(Resource::Sink) >= ratio(2, 1),
        "cryo orbit gifts sink"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn brown_dwarf_seeds_sink() {
    let mut world = founded("cold-kit");
    world.commission_star(StarClass::BrownDwarf).expect("bd");
    assert_eq!(
        world.orb().unwrap().banked_amount(Resource::Sink),
        ratio(4, 1)
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}
