//! WARDEN-1a + MASK-1a pins (10 §3e, 14 §4): the elder's reverse
//! shield refuses every aperture until the depth gate opens; the veil
//! clouds renders and pays its toll at every touch — an unpayable
//! toll lifts it by receipt, never silently. The chronicle never lies
//! under either.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{aperture_transfer, replay, ActionRefused, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1))
        .expect("sites");
    world
}

/// The reverse shield: both directions refuse while the ward stands;
/// the depth gate opens it at the ceiling; the deeds run in order.
#[test]
fn the_ward_refuses_both_ways_until_the_depth_gate_opens() {
    let mut elder = sunlit("the-elder");
    let mut visitor = sunlit("the-visitor");
    elder.ward_elder().expect("wards");
    assert!(elder.ward_holds());
    assert!(matches!(elder.ward_elder(), Err(ActionRefused::WardOutOfOrder)));

    // No aperture reaches a warded star — either direction.
    assert!(matches!(
        aperture_transfer(&mut visitor, FIRST_ORB, &mut elder, FIRST_ORB, Resource::Iron, ratio(1, 1), 7),
        Err(ActionRefused::TheWardHolds)
    ));
    assert!(matches!(
        aperture_transfer(&mut elder, FIRST_ORB, &mut visitor, FIRST_ORB, Resource::Iron, ratio(1, 1), 8),
        Err(ActionRefused::TheWardHolds)
    ));

    // The depth gate: below the ceiling refuses with both numbers.
    assert!(matches!(
        elder.lift_ward("the-visitor", 3),
        Err(ActionRefused::DepthBelowCeiling { depth: 3, ceiling: 5 })
    ));
    elder.lift_ward("the-visitor", 5).expect("the gate opens");
    assert!(!elder.ward_holds());
    assert!(matches!(elder.lift_ward("again", 9), Err(ActionRefused::WardOutOfOrder)));

    // The elder trades from here on.
    aperture_transfer(&mut visitor, FIRST_ORB, &mut elder, FIRST_ORB, Resource::Iron, ratio(1, 1), 9)
        .expect("the aperture opens");
    let lifted = elder.chronicle().receipts().iter().any(|receipt| {
        matches!(&receipt.event, ClosureEvent::WardLifted { claimer } if claimer == "the-visitor")
    });
    assert!(lifted, "the gate names its opener");
    assert_eq!(replay(elder.chronicle()), elder.committed(), "replay MATCHES");
    let resumed = World::resume(elder.chronicle().clone());
    assert_eq!(resumed.committed(), elder.committed(), "resume carries the ward");
}

/// The veil starves honestly: with no joules banked, the first touch
/// lifts it by receipt — never silently.
#[test]
fn an_unpayable_veil_lifts_by_receipt() {
    let mut world = World::new();
    world.found("the-masked").expect("founds");
    world.draw_veil().expect("draws");
    assert!(world.is_veiled());
    assert!(matches!(world.draw_veil(), Err(ActionRefused::VeilOutOfOrder)));

    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.tick();
    world.collect().expect("collects");
    assert!(!world.is_veiled(), "the toll went unpaid — the veil lifted");
    let starved = world.chronicle().receipts().iter().any(|receipt| {
        matches!(receipt.event, ClosureEvent::VeilLifted { starved: true })
    });
    assert!(starved, "the lift is receipted STARVED");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// A funded veil holds: the toll settles as joule upkeep at the
/// touch, receipted, and the veil stands.
#[test]
fn a_funded_veil_pays_its_toll_and_stands() {
    // The Furnace seat mints joules through the shell (BOT-THERM).
    let mut world = World::new();
    world.found("the-funded").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(1, 2), ratio(1, 2))
        .expect("sites hot");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    // Receipt-anchored, never tick-counted: mint until the bank shows.
    let mut minted = ratio(0, 1);
    for _ in 0..12 {
        world.tick();
        let _ = world.collect();
        minted = world.orb().expect("orb").banked_amount(Resource::Joule);
        if minted > ratio(0, 1) {
            break;
        }
    }
    assert!(minted > ratio(0, 1), "the shell minted");

    world.draw_veil().expect("draws");
    world.tick();
    world.collect().expect("collects under the veil");
    assert!(world.is_veiled(), "the funded veil stands");
    let tolled = world.chronicle().receipts().iter().any(|receipt| {
        matches!(&receipt.event,
            ClosureEvent::UpkeepSettled { resource: Resource::Joule, amount, .. }
                if *amount == ratio(1, 4))
    });
    assert!(tolled, "the toll settled as receipted joule upkeep");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries the veil");
}
