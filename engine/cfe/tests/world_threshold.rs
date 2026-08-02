//! THRESHOLD-1 — THE CROSSING AND THE SHIELD (12_UNION §7). A world
//! is a star system, so every aperture deed IS interstellar; a world's
//! first receipted aperture is its CROSSING into the union. Below the
//! solar leap the SHIELD holds: no aperture reaches a people that has
//! not met its sun, in either direction — the young are left to make
//! the leap themselves, or not. The shield is a typed refusal naming
//! which side stands shielded; a refusal moves nothing anywhere.

use cfe::dop::Resource;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{aperture_transfer, replay, ActionRefused, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1), None).expect("sites");
    world
}

fn iron(world: &World) -> cfe::ratio::Ratio {
    world
        .orb_by_id(FIRST_ORB)
        .expect("orb")
        .banked_amount(Resource::Iron)
}

/// The shield holds in both directions, names the shielded side, and
/// moves nothing on refusal.
#[test]
fn the_shield_refuses_both_directions_and_moves_nothing() {
    let mut sunward = sunlit("sunward");
    let mut deepmat = World::new();
    deepmat.found("deepmat").expect("founds");
    assert!(deepmat.is_shielded(), "a pre-solar world stands shielded");
    assert!(!sunward.is_shielded());

    // Toward the shielded: refused, the shielded side named.
    assert_eq!(
        aperture_transfer(
            &mut sunward, FIRST_ORB, &mut deepmat, FIRST_ORB,
            Resource::Iron, ratio(5, 1), 1,
        ),
        Err(ActionRefused::Shielded { source_shielded: false, dest_shielded: true })
    );
    // From the shielded: equally refused — the shield is not a cage
    // with a one-way door; it is no door at all.
    assert_eq!(
        aperture_transfer(
            &mut deepmat, FIRST_ORB, &mut sunward, FIRST_ORB,
            Resource::Iron, ratio(5, 1), 2,
        ),
        Err(ActionRefused::Shielded { source_shielded: true, dest_shielded: false })
    );
    // Nothing moved anywhere; neither world crossed.
    assert_eq!(iron(&sunward), ratio(20, 1));
    assert_eq!(iron(&deepmat), ratio(20, 1));
    assert!(!sunward.has_crossed());
    assert!(!deepmat.has_crossed());
    assert!(sunward.chronicle().receipts().iter().all(|r| {
        !matches!(
            r.event,
            cfe::dop::ClosureEvent::ApertureSent { .. }
                | cfe::dop::ClosureEvent::ApertureReceived { .. }
        )
    }));
}

/// The crossing: the first lawful aperture crosses BOTH worlds into
/// the union, the reading survives replay and resume, and the leap
/// lifts the shield — the whole arc, receipted.
#[test]
fn the_first_aperture_is_the_crossing() {
    let mut alpha = sunlit("alpha-centauri");
    let mut haven = World::new();
    haven.found("haven").expect("founds");

    // Shielded first; then haven makes its solar leap...
    assert!(haven.is_shielded());
    haven.commission_star(StarClass::G).expect("commissions");
    haven.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1), None).expect("sites");
    assert!(!haven.is_shielded(), "the leap lifts the shield");
    assert!(!haven.has_crossed(), "knowing the sun is not yet crossing the void");

    // ...and the first aperture is the crossing, for both.
    aperture_transfer(
        &mut alpha, FIRST_ORB, &mut haven, FIRST_ORB,
        Resource::Iron, ratio(5, 1), 9,
    )
    .expect("the aperture opens");
    assert!(alpha.has_crossed());
    assert!(haven.has_crossed());

    // The crossing is a replay fact and survives resume (SEAL B).
    assert!(replay(alpha.chronicle()).crossed);
    assert!(replay(haven.chronicle()).crossed);
    let resumed = World::resume(haven.chronicle().clone());
    assert!(resumed.has_crossed());
    assert!(!resumed.is_shielded());
}
