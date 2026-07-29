//! COUNCIL-1 — THE MULTILATERAL COVENANT (12_UNION §7). Three or more
//! orbs bound alike: each locks the same collateral against
//! delivering ≥ quota to fellow members within the window; the
//! reckoning judges ALL members together at the boundary — the whole
//! of the group over the needs of the individual. A breacher's
//! collateral divides exactly among the faithful; if every member
//! breached there is no aggrieved party — collateral goes home, the
//! breaches still post (the books forget nothing). The bypass
//! readings — warring imperium, diplomacy failed — are DERIVED from
//! receipts, never stored flags.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// found a, chart b from a, chart c from b, then spread iron so every
/// member can stake: a 6 · b 6 · c 8.
fn archipelago() -> (World, u64, u64) {
    let mut world = World::new();
    world.found("commons").expect("founds");
    let b = world.chart("midhold", FIRST_ORB).expect("charts");
    let c = world.chart("farhold", b).expect("charts");
    world.transfer(c, FIRST_ORB, Resource::Iron, ratio(6, 1)).expect("spreads");
    world.transfer(c, b, Resource::Iron, ratio(6, 1)).expect("spreads");
    (world, b, c)
}

fn iron(world: &World, orb: u64) -> cfe::ratio::Ratio {
    world.orb_by_id(orb).expect("orb").banked_amount(Resource::Iron)
}

/// The seal refuses whole: under three members, duplicate members, a
/// poor member — and a refusal moves nothing anywhere.
#[test]
fn a_council_that_starts_unequal_never_starts() {
    let (mut world, b, c) = archipelago();
    assert_eq!(
        world.seal_council(1, vec![FIRST_ORB, b], Resource::Iron, ratio(2, 1), 10, ratio(3, 1)),
        Err(ActionRefused::BadCouncil),
        "two is a covenant, not a council"
    );
    assert_eq!(
        world.seal_council(1, vec![FIRST_ORB, b, b], Resource::Iron, ratio(2, 1), 10, ratio(3, 1)),
        Err(ActionRefused::BadCouncil),
        "a member seats once"
    );
    // A member that cannot stake refuses the WHOLE seal.
    assert_eq!(
        world.seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 10, ratio(7, 1)),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Iron,
            need: Box::new(ratio(7, 1)),
            have: Box::new(ratio(6, 1)),
        })
    );
    assert_eq!(iron(&world, FIRST_ORB), ratio(6, 1), "nothing moved on refusal");
    assert_eq!(iron(&world, c), ratio(8, 1));
}

/// The full arc: seal, two members keep faith, one breaches — the
/// forfeit divides exactly among the faithful, conservation holds
/// across the whole, the books post, the diplomacy-failed reading
/// counts the breach, and replay reproduces it all (SEAL B).
#[test]
fn the_reckoning_judges_all_together() {
    let (mut world, b, c) = archipelago();
    world
        .seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 10, ratio(3, 1))
        .expect("seals");
    // Every lock drawn: 3 from each.
    assert_eq!(iron(&world, FIRST_ORB), ratio(3, 1));
    assert_eq!(iron(&world, b), ratio(3, 1));
    assert_eq!(iron(&world, c), ratio(5, 1));

    // a delivers to b; b delivers to c; c delivers to no one.
    world.transfer(FIRST_ORB, b, Resource::Iron, ratio(2, 1)).expect("a keeps faith");
    world.transfer(b, c, Resource::Iron, ratio(2, 1)).expect("b keeps faith");
    for _ in 0..11 {
        world.tick();
    }

    // The boundary: a and b bank 3 + 3/2 (the breacher's forfeit split
    // exactly two ways); c banks nothing. Conservation across the
    // archipelago: 20 iron, before and after.
    assert_eq!(iron(&world, FIRST_ORB), ratio(11, 2));
    assert_eq!(iron(&world, b), ratio(15, 2));
    assert_eq!(iron(&world, c), ratio(7, 1));
    let total = iron(&world, FIRST_ORB) + iron(&world, b) + iron(&world, c);
    assert_eq!(total, ratio(20, 1), "the council minted nothing");

    // Three reckonings receipted; the books post; the reading counts.
    let closes = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::CouncilClosed { .. }))
        .count();
    assert_eq!(closes, 3, "every member's reckoning is its own receipt");
    let held = world.orb_by_id(c).expect("orb");
    assert_eq!(held.covenants_breached, ratio(3, 1), "the stake posts breached");
    assert_eq!(world.council_breaches(c), 1, "diplomacy failed — once, counted");
    assert_eq!(world.council_breaches(b), 0);
    assert!(!world.is_imperial(c), "one breach is not an imperium");
    assert!(world.councils().is_empty(), "the council left the board");

    // Replay reproduces the banks and the books from receipts alone.
    let committed = replay(world.chronicle());
    let replayed = committed.orbs.iter().find(|o| o.orb_id == c).expect("orb");
    assert_eq!(replayed.banked_amount(Resource::Iron), ratio(7, 1));
    assert_eq!(replayed.covenants_breached, ratio(3, 1));
    assert!(committed.councils.is_empty());
}

/// Total breach is dissolution, not a slaughter: no aggrieved party
/// stands, every collateral goes home — and every breach still posts.
#[test]
fn when_all_breach_the_pact_dissolves() {
    let (mut world, b, c) = archipelago();
    world
        .seal_council(2, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 5, ratio(3, 1))
        .expect("seals");
    for _ in 0..6 {
        world.tick();
    }
    assert_eq!(iron(&world, FIRST_ORB), ratio(6, 1), "collateral home");
    assert_eq!(iron(&world, b), ratio(6, 1));
    assert_eq!(iron(&world, c), ratio(8, 1));
    for orb in [FIRST_ORB, b, c] {
        assert_eq!(
            world.orb_by_id(orb).expect("orb").covenants_breached,
            ratio(3, 1),
            "the books forget nothing"
        );
    }
}

/// Tags 52–53 round-trip byte-exact (SEAL A/B).
#[test]
fn the_council_events_round_trip_byte_exact() {
    for event in [
        ClosureEvent::CouncilSealed {
            council: 9,
            resource: Resource::Joule,
            quota: ratio(5, 2),
            window: 60,
            collateral: ratio(7, 3),
            members: vec![1, 2, 5],
        },
        ClosureEvent::CouncilClosed {
            council: 9,
            member: 5,
            fulfilled: false,
            banked: ratio(0, 1),
        },
    ] {
        let bytes = event.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert!((52..=53).contains(&tag));
        let decoded = ClosureEvent::from_wire(tag, value).expect("known tag decodes");
        assert_eq!(decoded, event, "value round trip");
        assert_eq!(decoded.wire_bytes(), bytes, "BYTE round trip");
    }
}
