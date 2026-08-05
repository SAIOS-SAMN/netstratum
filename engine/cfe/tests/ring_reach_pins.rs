//! THE RING REACH — planet capacity is based on the star's MAGNETISM
//! (operator ruling 2026-08-06). The theorems:
//!
//!   1. the reach IS the field grade — G anchors EIGHT, the home
//!      calibration (Sol's dynamo holds Sol's count);
//!   2. a new ring beyond the reach refuses, refusal-pure;
//!   3. filling an ANCHORED ring stays lawful past the reach — the
//!      field bounds rings, the turn law bounds slots within one;
//!   4. replay MATCHES across the whole arc.

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// A world under the given star with `orbs` colonies chartered and
/// enough banked to keep chartering lawful.
fn system_under(class: StarClass, orbs: usize) -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.commission_star(class).expect("one star per world");
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

#[test]
fn the_reach_is_the_field_grade_and_g_anchors_eight() {
    // The dynamo ordering holds across the table: radiative weakest,
    // fully-convective strongest, the neutron star anchored to the
    // measured pulsar-planet case.
    assert_eq!(StarClass::G.ring_reach(), 8, "Sol's dynamo holds Sol's count");
    assert_eq!(StarClass::NeutronStar.ring_reach(), 3, "PSR B1257+12, the measured case");
    assert!(StarClass::M.ring_reach() > StarClass::G.ring_reach(), "fully convective out-anchors the solar dynamo");
    assert!(StarClass::O.ring_reach() < StarClass::F.ring_reach(), "radiative envelopes anchor least");
    for class in [
        StarClass::O, StarClass::B, StarClass::A, StarClass::F, StarClass::G,
        StarClass::K, StarClass::M, StarClass::BrownDwarf, StarClass::WhiteDwarf,
        StarClass::RedGiant, StarClass::NeutronStar, StarClass::WolfRayet,
        StarClass::LuminalWhiteHole,
    ] {
        assert_eq!(class.ring_reach(), class.magnetism(), "the law IS the derivation: reach = field grade");
        assert!(class.ring_reach() >= 1, "every star anchors at least one ring");
    }
}

#[test]
fn a_new_ring_beyond_the_reach_refuses_pure() {
    // O anchors TWO rings. Thread both, then ask for a third.
    let mut world = system_under(StarClass::O, 3);
    let ids: Vec<u64> = world.orbs().iter().map(|o| o.orb_id).collect();
    world.site_planet(ids[0], ratio(1, 2), ratio(1, 1), None, None).expect("ring one");
    world.site_planet(ids[1], ratio(1, 2), ratio(2, 1), None, None).expect("ring two");
    let receipts_before = world.chronicle().receipts().len();
    match world.site_planet(ids[2], ratio(1, 2), ratio(3, 1), None, None) {
        Err(ActionRefused::RingReachExceeded { held: 2, reach: 2 }) => {}
        other => panic!("the third ring outruns an O star's field, got {other:?}"),
    }
    assert_eq!(
        world.chronicle().receipts().len(),
        receipts_before,
        "the refusal orphaned nothing"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn filling_an_anchored_ring_stays_lawful_past_the_reach() {
    // The field bounds RINGS, never slots: with both O rings held, a
    // third orb still sites ON an anchored ring — different longitude.
    let mut world = system_under(StarClass::O, 3);
    let ids: Vec<u64> = world.orbs().iter().map(|o| o.orb_id).collect();
    world
        .site_planet(ids[0], ratio(1, 2), ratio(1, 1), Some(cfe::dop::Slot::from_index(0)), None)
        .expect("ring one, slot zero");
    world
        .site_planet(ids[1], ratio(1, 2), ratio(2, 1), Some(cfe::dop::Slot::from_index(0)), None)
        .expect("ring two");
    world
        .site_planet(ids[2], ratio(1, 2), ratio(1, 1), Some(cfe::dop::Slot::from_index(30)), None)
        .expect("ring one again, the far slot — slots are the turn law's, not the field's");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
