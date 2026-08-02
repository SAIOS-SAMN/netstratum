//! BOND-1 — STANDING BONDS (09_IDEOLOGY §2.2, BUILD_ORDER §3).
//! The standing order: source streams rate/tick to target without
//! per-tick clearing — flow accrues against the source's bank and
//! settles by ONE `BondSettled` receipt at each touch (collect, sever,
//! term expiry), anchored like demurrage. The LOVE books open:
//! `bond_flow` (the source's syntropic book) / `severance_load` (the
//! severer's entropic book). An orderly severance — the bond run to
//! its term — posts NOTHING: leaving is not a sin; dropping a partner
//! mid-winter is. Insolvent settlement breaks the bond as a receipted
//! default.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

fn banked(world: &World, orb: u64, resource: Resource) -> Ratio {
    world
        .orb_by_id(orb)
        .expect("orb exists")
        .banked_amount(resource)
}

/// The plain two-planet economy (no star): mother runs ext+hab for 16
/// ticks, banks 30 iron / 13 biomass, charts the daughter (10i 3b |
/// 20i 10b). Ends at stamp 16.
fn mother_and_daughter() -> (World, u64) {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let daughter = world.chart("outpost", FIRST_ORB).expect("charts");
    (world, daughter)
}

/// A bond streams n ticks and settles the exact accumulated span in
/// ONE receipt at the touch — the demurrage anchor shape.
#[test]
fn a_bond_streams_and_settles_in_one_receipt() {
    let (mut world, daughter) = mother_and_daughter();
    world
        .seal_bond(FIRST_ORB, daughter, Resource::Iron, ratio(1, 2), 0)
        .expect("seals"); // @16, open-ended
    for _ in 0..6 {
        world.tick();
    }
    world.collect().expect("collects the mother"); // @22: the touch

    // owed = 1/2 × 6 = 3, one receipt: mother 10 + 15/2 − 3 = 29/2,
    // daughter 20 + 3 = 23.
    assert_eq!(banked(&world, FIRST_ORB, Resource::Iron), ratio(29, 2));
    assert_eq!(banked(&world, daughter, Resource::Iron), ratio(23, 1));
    let settles: Vec<Ratio> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|r| match &r.event {
            ClosureEvent::BondSettled { amount, .. } => Some(amount.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(settles, vec![ratio(3, 1)], "one receipt seals the whole span");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").bond_flow,
        ratio(3, 1)
    );
    // The bond stands, re-anchored: no double settle.
    assert_eq!(world.committed().bonds.len(), 1);
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the bond");
}

/// Severing mid-winter: the target's next heating bills arrive with
/// the inflow gone — the freeze is measured — and the severance load
/// posts to the cutter's entropic book.
#[test]
fn severing_mid_winter_drops_the_partner() {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let daughter = world.chart("coldstore", FIRST_ORB).expect("charts");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world
        .zone_as(SOVEREIGN, daughter, 0, ZoneKind::CryoArray)
        .expect("zones the array");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None)
        .expect("sites hot");
    world
        .site_planet(daughter, ratio(0, 1), ratio(100, 1), None)
        .expect("sites cold");
    world.tick(); // @16: the furnace mints; the cold orb's joule famine strikes
    world.collect().expect("banks the joules"); // @17
    world
        .transfer(FIRST_ORB, daughter, Resource::Joule, ratio(1, 1))
        .expect("relieves the freeze");
    // The heating lifeline: 1/4 joule per tick, open-ended.
    world
        .seal_bond(FIRST_ORB, daughter, Resource::Joule, ratio(1, 4), 0)
        .expect("seals the lifeline"); // @17
    world.tick();
    world.tick();
    world.collect().expect("collects"); // @19: bond settles 1/2
    assert_eq!(banked(&world, daughter, Resource::Joule), ratio(3, 2));
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").bond_flow,
        ratio(1, 2)
    );

    // The cut, mid-winter: load = rate × notice window = 1/4 × 8 = 2.
    world
        .sever_bond(FIRST_ORB, daughter, Resource::Joule)
        .expect("severs");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").severance_load,
        ratio(2, 1)
    );
    assert!(world.committed().bonds.is_empty());

    // The freeze returns on schedule: banked 3/2 − burn t/4 (with the
    // −1/2 already accrued) runs dry at the 5th further tick.
    let mut struck = false;
    for _ in 0..5 {
        let outcome = world.tick();
        struck |= outcome
            .struck
            .iter()
            .any(|(orb, resource)| *orb == daughter && *resource == Resource::Joule);
    }
    assert!(struck, "the next heating bills arrive with the inflow gone");
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the cut");
}

/// A term bond runs to its end and expires ORDERLY at the exact
/// boundary: the whole term streamed (rate × term), the severance
/// book untouched — leaving is not a sin.
#[test]
fn an_orderly_term_bond_expires_posting_nothing() {
    let (mut world, daughter) = mother_and_daughter();
    world
        .seal_bond(FIRST_ORB, daughter, Resource::Iron, ratio(1, 2), 4)
        .expect("seals for a term"); // @16, expires at 20
    world.tick();
    world.tick();
    world.collect().expect("collects"); // @18: settles 1, re-anchors
    for _ in 0..3 {
        world.tick(); // the tick opening at now=20 fires the expiry
    }
    let held = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(held.bond_flow, ratio(2, 1), "the whole term streamed: 1/2 × 4");
    assert_eq!(held.severance_load, ratio(0, 1), "orderly expiry posts NOTHING");
    assert!(world.committed().bonds.is_empty());
    let severed: Vec<(Ratio, Ratio)> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|r| match &r.event {
            ClosureEvent::BondSettled { severed: true, amount, load, .. } => {
                Some((amount.clone(), load.clone()))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        severed,
        vec![(ratio(1, 1), ratio(0, 1))],
        "the expiry settle carries the final span and zero load"
    );
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the expiry");
}

/// An insolvent settle pays what stands and breaks the bond as a
/// receipted DEFAULT — the load posts: failing the promise is the
/// mid-winter cut.
#[test]
fn an_insolvent_settle_breaks_the_bond_as_default() {
    let (mut world, daughter) = mother_and_daughter();
    // Rate 4/tick against a 10-iron bank and 5/4-iron income: doomed.
    world
        .seal_bond(FIRST_ORB, daughter, Resource::Iron, ratio(4, 1), 0)
        .expect("seals beyond its means"); // @16
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects"); // @20: owed 16, affordable 15
    let held = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(held.bond_flow, ratio(15, 1), "pays every iron it holds");
    assert_eq!(
        held.severance_load,
        ratio(32, 1),
        "the default posts rate × notice = 4 × 8"
    );
    assert_eq!(banked(&world, FIRST_ORB, Resource::Iron), ratio(0, 1));
    assert_eq!(banked(&world, daughter, Resource::Iron), ratio(35, 1));
    assert!(world.committed().bonds.is_empty());
    let defaulted = world
        .chronicle()
        .receipts()
        .iter()
        .any(|r| {
            matches!(
                &r.event,
                ClosureEvent::BondSettled { severed: true, amount, load, .. }
                    if *amount == ratio(15, 1) && *load == ratio(32, 1)
            )
        });
    assert!(defaulted, "the default is one receipt: partial pay + the load");
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the default");
}

/// The typed refusals: one bond per (source, target, resource); shape
/// errors refuse whole; no phantom orbs; severing what never stood.
#[test]
fn bond_refusals_are_typed() {
    let (mut world, daughter) = mother_and_daughter();
    world
        .seal_bond(FIRST_ORB, daughter, Resource::Iron, ratio(1, 2), 0)
        .expect("seals");
    assert_eq!(
        world.seal_bond(FIRST_ORB, daughter, Resource::Iron, ratio(1, 4), 0),
        Err(ActionRefused::BondAlreadySealed)
    );
    assert_eq!(
        world.seal_bond(FIRST_ORB, FIRST_ORB, Resource::Iron, ratio(1, 2), 0),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.seal_bond(FIRST_ORB, daughter, Resource::Iron, ratio(0, 1), 0),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.seal_bond(FIRST_ORB, 99, Resource::Iron, ratio(1, 2), 0),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );
    assert_eq!(
        world.sever_bond(FIRST_ORB, daughter, Resource::Biomass),
        Err(ActionRefused::NoSuchBond)
    );
}

/// Tags 33 and 34 round-trip byte-exact (the VENT-1 witness shape).
#[test]
fn bond_events_round_trip_byte_exact() {
    for (event, expected_tag) in [
        (
            ClosureEvent::BondSealed {
                source_orb: 1,
                target_orb: 2,
                resource: Resource::Iron,
                rate: ratio(1, 2),
                term: 12,
            },
            33,
        ),
        (
            ClosureEvent::BondSettled {
                source_orb: 1,
                target_orb: 2,
                resource: Resource::Joule,
                amount: ratio(3, 2),
                severed: true,
                load: ratio(2, 1),
            },
            34,
        ),
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
        assert_eq!(tag, expected_tag);
        let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
        assert_eq!(decoded, event);
        assert_eq!(decoded.wire_bytes(), bytes);
    }
}
