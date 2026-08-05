//! THE PLACEMENT DRAW — an unangled siting is not placeless; the world
//! draws its place (2026-08-06, closing the collision-blind hole the
//! ring-reach landing recorded).
//!
//! The placement law's own point is DECIDABILITY: "an angle must sit ON
//! a slot, and no two orbs may share a place… collision DECIDABLE
//! rather than a tolerance." A `theta: None` siting opted out of the
//! whole law — any number of unangled orbs stacked on one ring,
//! undetected. Under this law `None` means *the world places me*: the
//! longitude draws from the chronicle's terminal digest (CHARTER §9 —
//! chronicle-seeded, no wall clock), walks forward past occupied slots
//! deterministically, and the RECEIPT CARRIES THE RESOLVED ANGLE so
//! replay applies, never re-draws.

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

fn system(orbs: usize) -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.commission_star(StarClass::G).expect("star");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("mine");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("farm");
    for _ in 0..120 {
        world.tick();
    }
    world.collect().expect("banks");
    for k in 1..orbs {
        world.chart(&format!("orb{k}"), FIRST_ORB).expect("charts");
    }
    world
}

/// THE RED THIS CLOSES: two unangled sitings on ONE ring previously
/// both landed with `theta: None` — stacked, collision-blind. Now both
/// resolve to slots, and to DIFFERENT slots.
#[test]
fn two_unangled_sitings_on_one_ring_resolve_to_distinct_slots() {
    let mut world = system(3);
    let ids: Vec<u64> = world.orbs().iter().map(|o| o.orb_id).collect();
    world.site_planet(ids[0], ratio(1, 2), ratio(1, 1), None, None).expect("first");
    world.site_planet(ids[1], ratio(1, 2), ratio(1, 1), None, None).expect("second, same ring");

    let sited: Vec<Option<cfe::ratio::Ratio>> = world
        .orbs()
        .iter()
        .filter(|o| ids[..2].contains(&o.orb_id))
        .map(|o| o.thermal.as_ref().expect("sited").theta.clone())
        .collect();
    assert!(
        sited.iter().all(|t| t.is_some()),
        "an unangled siting RESOLVES to a slot — placeless is not a place: {sited:?}"
    );
    assert!(
        cfe::dop::on_slot(sited[0].as_ref().expect("resolved")),
        "and the drawn angle sits ON the lattice"
    );
    assert_ne!(sited[0], sited[1], "two orbs, one ring, two places — collision decided");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// The draw is CHRONICLE-SEEDED: two worlds with identical chronicles
/// draw identical slots — no wall clock, no dice.
#[test]
fn the_draw_is_deterministic_across_identical_chronicles() {
    let make = || {
        let mut world = system(2);
        let id = world.orbs()[0].orb_id;
        world.site_planet(id, ratio(1, 2), ratio(1, 1), None, None).expect("sites");
        world
            .orbs()
            .iter()
            .find(|o| o.orb_id == id)
            .and_then(|o| o.thermal.as_ref())
            .and_then(|s| s.theta.clone())
    };
    assert_eq!(make(), make(), "same chronicle, same draw");
}

/// A GIVEN angle is untouched — the draw serves only the unangled, and
/// the drawn walk respects what the given have already taken.
#[test]
fn a_given_angle_is_never_overridden_and_the_walk_respects_it() {
    let mut world = system(3);
    let ids: Vec<u64> = world.orbs().iter().map(|o| o.orb_id).collect();
    world
        .site_planet(ids[0], ratio(1, 2), ratio(1, 1), Some(ratio(1, 2)), None)
        .expect("given: the far slot");
    world.site_planet(ids[1], ratio(1, 2), ratio(1, 1), None, None).expect("drawn");
    let given = world.orbs().iter().find(|o| o.orb_id == ids[0]).unwrap();
    let drawn = world.orbs().iter().find(|o| o.orb_id == ids[1]).unwrap();
    assert_eq!(
        given.thermal.as_ref().unwrap().theta,
        Some(ratio(1, 2)),
        "the given angle stands exactly as asked"
    );
    assert_ne!(
        drawn.thermal.as_ref().unwrap().theta,
        Some(ratio(1, 2)),
        "and the draw walked past the taken place"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
