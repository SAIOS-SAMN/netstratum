//! TRADE-1 — THE TRANSFER SUBSTRATE (09_IDEOLOGY, BUILD_ORDER §3).
//! Orb-to-orb transfer of BANKED resources within one world: the
//! chartering conservation precedent generalized; demurrage settles
//! both sides at the touch (SINK-1 law); the COMPASSION books open —
//! `given_in_crisis` / `drawn_in_crisis`, posting iff the counterparty
//! stands in a receipted, uncleared crisis at the deed's stamp. Every
//! posting is a replay fact, never a judgment call.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn banked(world: &World, orb: u64, resource: Resource) -> Ratio {
    world
        .orb_by_id(orb)
        .expect("orb exists")
        .banked_amount(resource)
}

/// Conservation and the typed refusals: a transfer moves exactly what
/// leaves — nothing minted, nothing lost — and refuses with the cure
/// named. A healthy↔healthy transfer posts NOTHING to the books.
#[test]
fn a_transfer_conserves_and_refuses_typed() {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    let daughter = world.chart("outpost", FIRST_ORB).expect("charts");

    // The daughter holds the moved founding reserves; send 5 iron back.
    world
        .transfer(daughter, FIRST_ORB, Resource::Iron, ratio(5, 1))
        .expect("transfers");
    assert_eq!(banked(&world, daughter, Resource::Iron), ratio(15, 1));
    assert_eq!(banked(&world, FIRST_ORB, Resource::Iron), ratio(5, 1));
    let total = banked(&world, daughter, Resource::Iron)
        + banked(&world, FIRST_ORB, Resource::Iron);
    assert_eq!(total, ratio(20, 1), "conservation: the world minted nothing");

    // One receipt sealed the move.
    let transfers = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::ResourceTransferred { .. }))
        .count();
    assert_eq!(transfers, 1);

    // Refusals, typed with the exact shortfall.
    assert_eq!(
        world.transfer(daughter, FIRST_ORB, Resource::Iron, ratio(100, 1)),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Iron,
            need: Box::new(ratio(100, 1)),
            have: Box::new(ratio(15, 1)),
        })
    );
    assert_eq!(
        world.transfer(daughter, daughter, Resource::Iron, ratio(1, 1)),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.transfer(daughter, FIRST_ORB, Resource::Iron, ratio(0, 1)),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.transfer(daughter, FIRST_ORB, Resource::Iron, ratio(-1, 1)),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.transfer(daughter, 99, Resource::Iron, ratio(1, 1)),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );

    // Neither orb stands in crisis: the compassion books stay empty.
    for orb in [FIRST_ORB, daughter] {
        let held = world.orb_by_id(orb).expect("orb");
        assert_eq!(held.given_in_crisis, ratio(0, 1));
        assert_eq!(held.drawn_in_crisis, ratio(0, 1));
    }
}

/// The crisis fixture: a mother economy runs 16 clean ticks, charts a
/// daughter, and the daughter's lone extractor starves its biomass at
/// exactly tick 21 (10 − t/2 < 0). No star — no thermal arm at all.
fn starved_daughter() -> (World, u64) {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects"); // mother banks 30 iron, 13 biomass
    let daughter = world.chart("outpost", FIRST_ORB).expect("charts");
    world
        .zone_as(cfe::world::SOVEREIGN, daughter, 0, ZoneKind::Extractor)
        .expect("zones the daughter");
    let mut struck = false;
    for _ in 0..21 {
        let outcome = world.tick();
        struck |= outcome
            .struck
            .iter()
            .any(|(orb, resource)| *orb == daughter && *resource == Resource::Biomass);
    }
    assert!(struck, "the daughter's biomass famine strikes on schedule");
    (world, daughter)
}

/// Compassion is the famine-relief mechanic made inter-orb: a gift
/// below the one-tick bill relieves nothing; reaching the bill
/// relieves at the landed law (02 §2.4) — and every crisis posting is
/// exact.
#[test]
fn a_transfer_into_famine_relieves_at_the_law() {
    let (mut world, daughter) = starved_daughter();

    // A gift of 1/4 biomass: below the extractor's 1/2 one-tick
    // upkeep — the famine holds.
    world
        .transfer(FIRST_ORB, daughter, Resource::Biomass, ratio(1, 4))
        .expect("gives");
    assert!(world
        .orb_by_id(daughter)
        .expect("orb")
        .famine
        .contains(&Resource::Biomass));

    // The second 1/4 reaches the bill: relief fires, receipted once.
    world
        .transfer(FIRST_ORB, daughter, Resource::Biomass, ratio(1, 4))
        .expect("gives again");
    assert!(!world
        .orb_by_id(daughter)
        .expect("orb")
        .famine
        .contains(&Resource::Biomass));
    let relieved = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| {
            matches!(
                r.event,
                ClosureEvent::FamineRelieved { orb_id, resource: Resource::Biomass }
                    if orb_id == daughter
            )
        })
        .count();
    assert_eq!(relieved, 1);

    // Both gifts landed while the daughter stood struck: the mother's
    // syntropic book holds exactly 1/2 (weight 1 in v1).
    let mother = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(mother.given_in_crisis, ratio(1, 2));
    assert_eq!(mother.drawn_in_crisis, ratio(0, 1));
}

/// The posting rule is deterministic — a replay fact: drawing FROM a
/// struck orb posts the taker's entropic book; giving TO a struck orb
/// posts the giver's syntropic book; after relief, posts stop; and the
/// whole ledger replays from receipts alone.
#[test]
fn crisis_posting_is_a_replay_fact() {
    let (mut world, daughter) = starved_daughter();

    // Draw 2 banked iron OUT of the struck daughter: distress
    // extraction — the mother's entropic book posts exactly 2.
    world
        .transfer(daughter, FIRST_ORB, Resource::Iron, ratio(2, 1))
        .expect("draws");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").drawn_in_crisis,
        ratio(2, 1)
    );

    // Relieve the daughter with one 1/2 biomass gift.
    world
        .transfer(FIRST_ORB, daughter, Resource::Biomass, ratio(1, 2))
        .expect("gives");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").given_in_crisis,
        ratio(1, 2)
    );

    // Healthy↔healthy now: no book moves.
    world
        .transfer(FIRST_ORB, daughter, Resource::Iron, ratio(1, 1))
        .expect("transfers");
    let mother = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(mother.given_in_crisis, ratio(1, 2));
    assert_eq!(mother.drawn_in_crisis, ratio(2, 1));
    let held = world.orb_by_id(daughter).expect("orb");
    assert_eq!(held.given_in_crisis, ratio(0, 1));
    assert_eq!(held.drawn_in_crisis, ratio(0, 1));

    // SEAL B: banks, famine state, and the cohesion books replay.
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the books");
}

/// The SINK-1 touch law extends to transfers: BOTH sides settle their
/// due demurrage at the transfer's stamp, receipted, before value
/// moves — and a transfer at the same stamp as the last touch pays
/// nothing (the anchor is exact).
#[test]
fn demurrage_settles_both_sides_at_the_transfer_touch() {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects"); // 30 iron, 13 biomass
    let daughter = world.chart("coldstore", FIRST_ORB).expect("charts");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2))
        .expect("sites the furnace");
    world
        .site_planet(daughter, ratio(0, 1), ratio(1, 2))
        .expect("sites the store");

    // One furnace tick mints 15/8 joules (the THERMAL-1 arithmetic).
    world.tick();
    world.collect().expect("banks the joules");
    assert_eq!(banked(&world, FIRST_ORB, Resource::Joule), ratio(15, 8));

    // Transfer at the same stamp as the collect: elapsed 0 — no decay.
    world
        .transfer(FIRST_ORB, daughter, Resource::Joule, ratio(1, 1))
        .expect("transfers");
    let demurrage = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::DemurrageSettled { .. }))
        .count();
    assert_eq!(demurrage, 0, "a same-stamp touch pays nothing");

    // Four ticks pass; the second transfer touches BOTH warm orbs:
    // mother decays 7/8 × 1/32 × 4 = 7/64, daughter 1 × 1/32 × 4 = 1/8.
    for _ in 0..4 {
        world.tick();
    }
    world
        .transfer(FIRST_ORB, daughter, Resource::Joule, ratio(1, 8))
        .expect("transfers again");
    let mut settled: Vec<(u64, Ratio)> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|r| match &r.event {
            ClosureEvent::DemurrageSettled { orb_id, resource: Resource::Joule, amount } => {
                Some((*orb_id, amount.clone()))
            }
            _ => None,
        })
        .collect();
    settled.sort_by_key(|(orb, _)| *orb);
    assert_eq!(
        settled,
        vec![(FIRST_ORB, ratio(7, 64)), (daughter, ratio(1, 8))],
        "both sides settle their exact decay at the one touch"
    );
    // Mother: 7/8 − 7/64 − 1/8 = 41/64. Daughter: 1 − 1/8 + 1/8 = 1.
    assert_eq!(banked(&world, FIRST_ORB, Resource::Joule), ratio(41, 64));
    assert_eq!(banked(&world, daughter, Resource::Joule), ratio(1, 1));

    // The furnace stands overheat-STRUCK: both draws left a crisis orb,
    // so the daughter's entropic book holds exactly 1 + 1/8.
    assert_eq!(
        world.orb_by_id(daughter).expect("orb").drawn_in_crisis,
        ratio(9, 8)
    );

    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the touches");
}

/// Truth-update to the landed touch law: an upkeep-ONLY collect (a
/// cold shell burning joules it does not have) must anchor
/// `last_touch` identically live and on replay — the demurrage anchor
/// is law for every settle shape, not only positive yields.
#[test]
fn an_upkeep_only_collect_anchors_last_touch_on_replay() {
    let mut world = World::new();
    world.found("anchor").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(100, 1))
        .expect("sites cold");
    world.zone(0, ZoneKind::Encasement).expect("zones the shell");
    world.tick(); // heating burns joules the orb lacks: joule famine
    world.collect().expect("settles the iron upkeep alone");
    let rebuilt = replay(world.chronicle());
    assert_eq!(
        rebuilt, world.committed(),
        "the touch anchor replays for an upkeep-only settle"
    );
}

/// Tag 30 round-trips byte-exact (the VENT-1 witness shape).
#[test]
fn the_transfer_event_round_trips_byte_exact() {
    let event = ClosureEvent::ResourceTransferred {
        from_orb: 1,
        to_orb: 2,
        resource: Resource::Iron,
        amount: ratio(5, 1),
    };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 30);
    let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
    assert_eq!(decoded, event);
    assert_eq!(decoded.wire_bytes(), bytes);
}
