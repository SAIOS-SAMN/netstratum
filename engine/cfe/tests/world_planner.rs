//! PLANNER-1 — THE LICENSED LEAP (12_UNION §2, 10 §2.2, tag 40). The
//! planner's issuing power: a technology crosses from one world into
//! another as a two-legged deed (TechLicensed out, TechDiscovered in,
//! one deal), gated by diplomacy — an OPEN council in the granting
//! world — and bounded by the shield: the receiver must know its sun
//! AND have crossed. The solar leap is NEVER licensed: the young meet
//! their sun themselves, or not — that refusal IS the union's oldest
//! law, stated as a deed.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{
    aperture_transfer, license_tech, replay, ActionRefused, Leap, World, FIRST_ORB,
};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    world
}

/// A granting world with an open council (three chartered orbs) and a
/// crossed receiving world.
fn planner_pair() -> (World, World) {
    let mut grantor = sunlit("eridu-forge");
    let b = grantor.chart("midhold", FIRST_ORB).expect("charts");
    let c = grantor.chart("farhold", b).expect("charts");
    grantor.transfer(c, FIRST_ORB, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor.transfer(c, b, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor
        .seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 600, ratio(1, 1))
        .expect("the council stands open");
    let mut receiver = sunlit("kepler-passage");
    aperture_transfer(
        &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 1,
    )
    .expect("both cross");
    (grantor, receiver)
}

/// The refusals, each its own law: the solar leap never licenses; no
/// council, no delegation; a shielded receiver is unreachable; an
/// uncrossed receiver is below the threshold. Nothing moves on any.
#[test]
fn the_gates_hold_in_order() {
    let (mut grantor, mut receiver) = planner_pair();

    assert_eq!(
        license_tech(&mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB, Leap { condition: 0, name: "the solar leap".into(), deal: 7, council: 1 }),
        Err(ActionRefused::SolarLeapNeverLicensed)
    );
    assert_eq!(
        license_tech(&mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB, Leap { condition: 3, name: "radiator doctrine".into(), deal: 7, council: 99 }),
        Err(ActionRefused::NoSuchCouncil { council: 99 })
    );

    let mut shielded = World::new();
    shielded.found("deepmat-veil").expect("founds");
    assert_eq!(
        license_tech(&mut grantor, FIRST_ORB, &mut shielded, FIRST_ORB, Leap { condition: 3, name: "radiator doctrine".into(), deal: 7, council: 1 }),
        Err(ActionRefused::Shielded { source_shielded: false, dest_shielded: true })
    );

    let mut sunlit_only = sunlit("haven");
    assert_eq!(
        license_tech(&mut grantor, FIRST_ORB, &mut sunlit_only, FIRST_ORB, Leap { condition: 3, name: "radiator doctrine".into(), deal: 7, council: 1 }),
        Err(ActionRefused::BelowTheThreshold)
    );
    assert!(sunlit_only.techs().len() == 1, "only its own solar leap — nothing arrived");
}

/// The lawful leap: one deal, two legs, the SAME node derived on both
/// sides; the receiver holds a technology it never crystallized —
/// known or unknown to its people, it is theirs now; a second issue of
/// the same deal refuses idempotent; both chronicles replay whole.
#[test]
fn a_licensed_leap_lands_in_the_far_sky() {
    let (mut grantor, mut receiver) = planner_pair();
    license_tech(&mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB, Leap { condition: 3, name: "radiator doctrine".into(), deal: 7, council: 1 })
        .expect("the leap licenses");

    let issued = grantor.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::TechLicensed { deal, node, .. } => Some((*deal, *node)),
        _ => None,
    });
    let arrived = receiver.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::TechDiscovered { condition: 3, node, .. } => Some(*node),
        _ => None,
    });
    let (deal, node_out) = issued.expect("the issuing leg receipted");
    assert_eq!(deal, 7);
    assert_eq!(Some(node_out), arrived, "one deal, one node, two books");
    assert!(
        receiver.techs().iter().any(|(_, condition, _)| *condition == 3),
        "the receiver holds what it never crystallized"
    );

    // Idempotent: the same deal cannot land twice.
    assert!(license_tech(
        &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
        Leap { condition: 3, name: "radiator doctrine".into(), deal: 7, council: 1 }
    )
    .is_err());

    // Both legs replay in their own world (SEAL B per chronicle).
    let committed = replay(receiver.chronicle());
    assert!(committed.techs.iter().any(|(_, condition, held)| *condition == 3 && *held == node_out));
    let resumed = World::resume(receiver.chronicle().clone());
    assert_eq!(resumed.techs(), receiver.techs());
}

/// Tag 40 round-trips byte-exact (SEAL A/B).
#[test]
fn the_license_round_trips_byte_exact() {
    let event = ClosureEvent::TechLicensed {
        deal: 7,
        from_orb: 1,
        council: 1,
        condition: 3,
        node: [9u8; 32],
        name: "radiator doctrine".into(),
    };
    let bytes = event.wire_bytes();
    assert_eq!(bytes[0], 40, "tag 40 — inscribed at PLANNER-1");
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    let decoded = ClosureEvent::from_wire(tag, value).expect("known tag decodes");
    assert_eq!(decoded, event, "value round trip");
    assert_eq!(decoded.wire_bytes(), bytes, "BYTE round trip");
}
