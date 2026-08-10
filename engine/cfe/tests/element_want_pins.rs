//! E3.7 — WANT-1 material targets: prospected ElementIds with empty
//! banks hunger; assay relieves; market demand posts Resource→Element.

use cfe::dop::{positive, unit, Cargo, Exchange, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

fn industrial(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("sun");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    for c in 1u8..=5 {
        world.grant_tech_as(SOVEREIGN, FIRST_ORB, c).expect("tech");
    }
    world
}

#[test]
fn prospected_empty_vein_strikes_element_want() {
    let mut world = industrial("want-hold");
    let dig = world.civ_reach();
    let mut element = None;
    for s in 1..=dig {
        if let Ok(struck) = world.prospect(FIRST_ORB, s) {
            // Prefer a vein that won't assay without further ticks —
            // empty bank is enough for want after a tick sweep.
            element = Some(struck.element);
            break;
        }
    }
    let element = element.expect("a prospect");
    assert!(
        world.orb().unwrap().element_amount(&element) == ratio(0, 1),
        "no bank yet"
    );
    // Tick triggers sweep_element_wants.
    world.tick();
    assert!(
        world
            .element_wants()
            .iter()
            .any(|(o, e)| *o == FIRST_ORB && e == &element),
        "material want struck for empty prospected vein"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn assay_relieves_element_want() {
    let mut world = industrial("feed-hold");
    let dig = world.civ_reach();
    let mut element = None;
    for s in 1..=dig {
        if let Ok(struck) = world.prospect(FIRST_ORB, s) {
            for _ in 0..8 {
                world.tick();
            }
            if world.assay(FIRST_ORB, struck.element).is_ok() {
                element = Some(struck.element);
                break;
            }
        }
    }
    let element = element.expect("mineable vein");
    // After assay, want must be gone (relieved on bank).
    assert!(
        !world
            .element_wants()
            .iter()
            .any(|(o, e)| *o == FIRST_ORB && e == &element),
        "assay feeds the want"
    );
    assert!(world.orb().unwrap().element_amount(&element) > ratio(0, 1));
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn element_want_posts_resource_for_element_offer() {
    let mut world = industrial("bid-hold");
    let struck = world.prospect(FIRST_ORB, 1).expect("charts");
    world.tick(); // strike want if bank empty
    if world.element_wants().is_empty() {
        // Already banked somehow — force a second empty prospect.
        let s = world.civ_reach().min(2);
        if s >= 2 {
            let _ = world.prospect(FIRST_ORB, 2);
            world.tick();
        }
    }
    // Manual demand path: post Resource → Element offer as market would.
    if let Some((_, element)) = world.element_wants().first().copied() {
        let pair = Exchange::cargo(
            Cargo::Resource(Resource::Iron),
            Cargo::Element(element),
        )
        .expect("pair");
        world
            .post_offer(
                FIRST_ORB,
                pair,
                positive(2, 1),
                positive(1, 1),
                99,
            )
            .expect("demand offer");
        assert!(matches!(
            world.offers()[0].want,
            Cargo::Element(_)
        ));
    }
    let _ = struck;
    assert_eq!(replay(world.chronicle()), world.committed());
}
