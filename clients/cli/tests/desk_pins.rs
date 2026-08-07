//! Phase-1/2 desk pins (TASKS_TRADE_FLEET_DESK).
//!
//! Desk melt readings call route_melt (pair_melt over D_eff).

use cfe::dop::{Slot, Unit};
use cfe::ratio::ratio;
use cfe::world::{
    alignment_of, d_orbit, effective_distance, pair_melt, route_melt, Alignment, World,
    ORBIT_PHASE_TICKS,
};
use conductor::spatial_l1;

#[test]
fn desk_melt_is_route_melt_over_d_eff() {
    let mut a = World::new();
    a.found("desk-alpha").expect("found");
    let mut b = World::new();
    b.found("desk-beta").expect("found");
    let _ = a.grant_tech_as(cfe::world::SOVEREIGN, cfe::world::FIRST_ORB, 0);
    let _ = b.grant_tech_as(cfe::world::SOVEREIGN, cfe::world::FIRST_ORB, 0);

    let pos_a = (ratio(0, 1), ratio(0, 1), ratio(0, 1));
    let pos_b = (ratio(8, 1), ratio(0, 1), ratio(0, 1));
    let d = spatial_l1(&pos_a, &pos_b);
    let via_route = route_melt(&a, &b, &d);
    let via_pair = pair_melt(&a, &b, &effective_distance(&a, &b, &d));
    assert_eq!(via_route, via_pair, "route_melt ≡ pair_melt(D_eff)");
    assert!(via_route > ratio(0, 1));
    assert!(via_route < ratio(1, 1));
}

#[test]
fn phase_shifts_orbit_and_melt() {
    let mut a = World::new();
    a.found("orb-a").expect("found");
    let mut b = World::new();
    b.found("orb-b").expect("found");
    // Commission stars and site with explicit θ so D_orbit is defined.
    a.commission_star(cfe::thermal::StarClass::G).expect("star");
    b.commission_star(cfe::thermal::StarClass::G).expect("star");
    let albedo = Unit::new(ratio(0, 1)).expect("A");
    let r = cfe::dop::Positive::new(ratio(1, 1)).expect("r");
    a.site_planet(
        cfe::world::FIRST_ORB,
        albedo.clone(),
        r.clone(),
        Some(Slot::from_index(0)),
        None,
    )
    .expect("site a");
    b.site_planet(
        cfe::world::FIRST_ORB,
        albedo,
        r,
        Some(Slot::from_index(30)), // opposite on 60-slot ring
        None,
    )
    .expect("site b");

    let d_chart = ratio(0, 1); // same chart point — pure orbital drama
    let orbit0 = d_orbit(&a, &b);
    let melt0 = route_melt(&a, &b, &d_chart);
    assert!(orbit0 > ratio(0, 1), "sited opposite slots have D_orbit");
    assert_ne!(alignment_of(&a, &b), Alignment::Unplaced);

    // Advance home clock by half a ring of years → θ moves 30 slots.
    let years = ORBIT_PHASE_TICKS * 30;
    for _ in 0..years {
        a.tick();
    }
    let orbit1 = d_orbit(&a, &b);
    let melt1 = route_melt(&a, &b, &d_chart);
    assert_ne!(
        orbit0, orbit1,
        "phase advance must change D_orbit (same chart)"
    );
    assert_ne!(melt0, melt1, "phase advance must change melt");
}

#[test]
fn stamp_mint_and_balance_round_trip() {
    let mut world = World::new();
    world.found("stamp-desk").expect("found");
    let refuse = cfe::world::mint_stamp_from_crystal(
        &mut world,
        cfe::world::FIRST_ORB,
        cfe::dop::Positive::new(ratio(1, 1)).expect("pos"),
    );
    assert!(refuse.is_err(), "no free STAMP without crystal");

    let earned = cfe::dop::NonNegative::new(ratio(4, 1)).expect("nn");
    let work = vec![0u8; 32];
    let _ = world.mint_discovery_as(
        cfe::world::SOVEREIGN,
        cfe::world::FIRST_ORB,
        cfe::dop::Resource::Crystal,
        earned,
        work,
        &kernel::SealedWarrant,
    );
    let stamp0 = world
        .orb()
        .map(|o| o.stamp_amount())
        .unwrap_or_else(|| ratio(0, 1));
    let crystal = world
        .orb()
        .map(|o| o.banked_amount(cfe::dop::Resource::Crystal))
        .unwrap_or_else(|| ratio(0, 1));
    if crystal > ratio(0, 1) {
        let convert = crystal / (ratio(1, 1) + ratio(1, 1));
        let amt = cfe::dop::Positive::new(convert.clone()).expect("pos");
        cfe::world::mint_stamp_from_crystal(&mut world, cfe::world::FIRST_ORB, amt)
            .expect("mint");
        let stamp1 = world
            .orb()
            .map(|o| o.stamp_amount())
            .unwrap_or_else(|| ratio(0, 1));
        assert_eq!(stamp1, stamp0 + convert);
        assert_eq!(cfe::world::replay(world.chronicle()), world.committed());
    }
}
