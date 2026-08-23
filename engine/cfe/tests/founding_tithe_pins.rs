//! `#32`, `20_FOUNDING` §3 — the founding tithe. A fresh founding pays
//! a receipted tithe, denominated in the line's OWN metabolic ration
//! (never a flat iron/biomass amount), to the seat scope covering the
//! founding. FALLBACK siting is the only landed acquisition path today,
//! so every founding raises a new world and tithes to the ANCIENT seat
//! (PURCHASE/COMMISSION, the elder path, are deferred).

use cfe::dop::{ClosureEvent, Resource, SEAT_ANCIENT, SEAT_ELDER};
use cfe::world::{replay, World, FIRST_ORB};

const EXPANSIONIST: u8 = 1;
const MINERAL: u8 = 1;
const BIOTIC: u8 = 2;
const PLASMIC: u8 = 3;
const PLANAR: u8 = 1;

#[test]
fn founding_pays_a_tithe_in_the_lines_own_ration() {
    // MINERAL tithes iron, not the flat founding-reserves amount.
    let mut world = World::new();
    world.found("iron-hold").expect("founds");
    let before = world.orbs()[0].banked_amount(Resource::Iron);
    world
        .seal_genesis("mineral-line", EXPANSIONIST, MINERAL, PLANAR)
        .expect("seals");
    let after = world.orbs()[0].banked_amount(Resource::Iron);
    assert!(after < before, "the tithe debits iron for a mineral line");

    let receipt = world
        .chronicle()
        .receipts()
        .iter()
        .find(|r| matches!(r.event(), ClosureEvent::TitheReceipted { .. }))
        .expect("a TitheReceipted receipt was chronicled");
    match receipt.event() {
        ClosureEvent::TitheReceipted { orb_id, to_seat, resource, .. } => {
            assert_eq!(*orb_id, FIRST_ORB);
            assert_eq!(*to_seat, SEAT_ANCIENT, "fallback siting always tithes the ancient today");
            assert_eq!(*resource, Resource::Iron, "a mineral line tithes iron, not a flat amount");
        }
        _ => unreachable!(),
    }
}

#[test]
fn a_biotic_line_tithes_biomass_not_iron() {
    let mut world = World::new();
    world.found("biotic-hold").expect("founds");
    world
        .seal_genesis("biotic-line", EXPANSIONIST, BIOTIC, PLANAR)
        .expect("seals");
    let receipt = world
        .chronicle()
        .receipts()
        .iter()
        .find(|r| matches!(r.event(), ClosureEvent::TitheReceipted { .. }))
        .expect("chronicled");
    match receipt.event() {
        ClosureEvent::TitheReceipted { resource, .. } => {
            assert_eq!(*resource, Resource::Biomass, "a biotic line tithes its own ration, biomass");
        }
        _ => unreachable!(),
    }
}

#[test]
fn a_plasmic_line_tithes_zero_until_29_lands() {
    // metabolic_harvest(PLASMIC).1 is rate 0 today (plasmic agriculture
    // is #29, still open) — the tithe is byte-identically a no-op for
    // this metabolism, not a special case in the founding code.
    let mut world = World::new();
    world.found("plasmic-hold").expect("founds");
    let before = world.orbs()[0].banked_amount(Resource::Joule);
    world
        .seal_genesis("plasmic-line", EXPANSIONIST, PLASMIC, PLANAR)
        .expect("seals");
    let after = world.orbs()[0].banked_amount(Resource::Joule);
    assert_eq!(before, after, "plasmic's rate-0 harvest makes the tithe a no-op debit until #29");
}

#[test]
fn the_founding_tithe_replays_byte_exact() {
    let mut world = World::new();
    world.found("replay-hold").expect("founds");
    world
        .seal_genesis("replay-line", EXPANSIONIST, MINERAL, PLANAR)
        .expect("seals");
    assert_eq!(replay(world.chronicle()), world.committed(), "founding-tithe replay MATCHES");
}

#[test]
fn the_ancient_treasury_folds_the_tithe() {
    let mut world = World::new();
    world.found("treasury-hold").expect("founds");
    world
        .seal_genesis("treasury-line", EXPANSIONIST, MINERAL, PLANAR)
        .expect("seals");
    let ancient = world.seat_treasury(SEAT_ANCIENT);
    assert_eq!(
        ancient,
        vec![(Resource::Iron, cfe::ratio::ratio(1, 1))],
        "the ancient's treasury folds the one iron tithe this founding paid"
    );
    assert!(
        world.seat_treasury(SEAT_ELDER).is_empty(),
        "no founding has tithed the elder yet — fallback siting always pays the ancient"
    );
}
