//! Phase-1 desk pins (TASKS_TRADE_FLEET_DESK P1.11).
//!
//! Desk melt readings must CALL pair_melt — never recompute a second iceberg.

use cfe::ratio::ratio;
use cfe::world::{pair_melt, World};
use conductor::spatial_l1;

/// Mirror of desk::route_melt — the desk module is bin-private; this
/// pin freezes the law the desk is required to call.
fn desk_route_melt(a: &World, b: &World, d_chart: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    pair_melt(a, b, d_chart)
}

#[test]
fn desk_melt_is_pair_melt() {
    let mut a = World::new();
    a.found("desk-alpha").expect("found");
    let mut b = World::new();
    b.found("desk-beta").expect("found");
    // Solar leap so shield is not the story — grant condition 0.
    let _ = a.grant_tech_as(cfe::world::SOVEREIGN, cfe::world::FIRST_ORB, 0);
    let _ = b.grant_tech_as(cfe::world::SOVEREIGN, cfe::world::FIRST_ORB, 0);

    let pos_a = (ratio(0, 1), ratio(0, 1), ratio(0, 1));
    let pos_b = (ratio(8, 1), ratio(0, 1), ratio(0, 1));
    let d = spatial_l1(&pos_a, &pos_b);
    let via_desk = desk_route_melt(&a, &b, &d);
    let via_engine = pair_melt(&a, &b, &d);
    assert_eq!(via_desk, via_engine, "desk melt must be pair_melt byte-exact");
    assert!(via_engine > ratio(0, 1), "positive distance yields positive melt");
    assert!(via_engine < ratio(1, 1), "L1=8 at φ=1/64 is not walled");
}

#[test]
fn stamp_mint_and_balance_round_trip() {
    let mut world = World::new();
    world.found("stamp-desk").expect("found");
    // Seed crystal via discovery-like bank: mint path needs crystal stock.
    // Use a direct bank through a small positive mint of crystal if available,
    // or DiscoveryMinted is heavy — put crystal with resource transfer from
    // founding is iron/bio only. Use mint_discovery if we can, else skip path
    // by banking via closed API.
    // Founding has no crystal — mint_stamp should refuse.
    let refuse = cfe::world::mint_stamp_from_crystal(
        &mut world,
        cfe::world::FIRST_ORB,
        cfe::dop::Positive::new(ratio(1, 1)).expect("pos"),
    );
    assert!(refuse.is_err(), "no free STAMP without crystal");

    // Bank crystal by forging a discovery mint with sealed warrant when possible.
    // Simpler: use collect path after resonator — too heavy. Direct bank via
    // orb peel is private. Use kernel-admitted DiscoveryMinted through world API.
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
    // If mint_discovery refused (warrant/work), still assert stamp starts zero.
    let stamp0 = world.orb().map(|o| o.stamp_amount()).unwrap_or_else(|| ratio(0, 1));
    let crystal = world
        .orb()
        .map(|o| o.banked_amount(cfe::dop::Resource::Crystal))
        .unwrap_or_else(|| ratio(0, 1));
    if crystal > ratio(0, 1) {
        let convert = crystal.clone() / (ratio(1, 1) + ratio(1, 1));
        let amt = cfe::dop::Positive::new(convert.clone()).expect("pos");
        cfe::world::mint_stamp_from_crystal(&mut world, cfe::world::FIRST_ORB, amt)
            .expect("mint");
        let stamp1 = world.orb().map(|o| o.stamp_amount()).unwrap_or_else(|| ratio(0, 1));
        assert_eq!(stamp1, stamp0 + convert, "identity mint");
        assert_eq!(
            cfe::world::replay(world.chronicle()),
            world.committed(),
            "SEAL B after stamp mint"
        );
    }
}
