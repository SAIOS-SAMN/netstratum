//! COUNCIL-1 — THE MULTILATERAL FORWARD (12_UNION §7). Three or more
//! orbs bound alike: each locks the same collateral against
//! delivering ≥ quota to fellow members within the window; the
//! reckoning gates ALL members together at the boundary — the whole
//! of the group over the needs of the individual. A breacher's
//! collateral divides exactly among the faithful; if every member
//! breached there is no aggrieved party — collateral goes home, the
//! breaches still post (the books forget nothing). The bypass
//! readings — warring imperium, diplomacy failed — are DERIVED from
//! receipts, never stored flags.

use cfe::dop::{among, positive, window, ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// found a, chart b from a, chart c from b, then spread iron so every
/// member can stake: a 6 · b 6 · c 8.
fn archipelago() -> (World, u64, u64) {
    let mut world = World::new();
    world.found("commons").expect("founds");
    let b = world.chart("midhold", FIRST_ORB).expect("charts");
    let c = world.chart("farhold", b).expect("charts");
    world.transfer(cfe::dop::between(c, FIRST_ORB), Resource::Iron, cfe::dop::positive(6, 1)).expect("spreads");
    world.transfer(cfe::dop::between(c, b), Resource::Iron, cfe::dop::positive(6, 1)).expect("spreads");
    (world, b, c)
}

fn iron(world: &World, orb: u64) -> cfe::ratio::Ratio {
    world.orb_by_id(orb).expect("orb").banked_amount(Resource::Iron)
}

/// The unequal shapes cannot even be constructed — `Among` refuses a
/// short or repeated membership at the door (29_CONSTRUCTION), and a
/// member that cannot stake refuses the WHOLE seal, moving nothing.
#[test]
fn a_council_that_starts_unequal_never_starts() {
    let (mut world, b, c) = archipelago();
    assert!(
        cfe::dop::Among::new(vec![FIRST_ORB, b]).is_none(),
        "two is a forward, not a council"
    );
    assert!(
        cfe::dop::Among::new(vec![FIRST_ORB, b, b]).is_none(),
        "a member seats once"
    );
    // A member that cannot stake refuses the WHOLE seal.
    assert_eq!(
        world.seal_council(1, among(&[FIRST_ORB, b, c]), Resource::Iron, positive(2, 1), window(10), positive(7, 1)),
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
        .seal_council(1, among(&[FIRST_ORB, b, c]), Resource::Iron, positive(2, 1), window(10), positive(3, 1))
        .expect("seals");
    // Every lock drawn: 3 from each.
    assert_eq!(iron(&world, FIRST_ORB), ratio(3, 1));
    assert_eq!(iron(&world, b), ratio(3, 1));
    assert_eq!(iron(&world, c), ratio(5, 1));

    // a delivers to b; b delivers to c; c delivers to no one.
    world.transfer(cfe::dop::between(FIRST_ORB, b), Resource::Iron, cfe::dop::positive(2, 1)).expect("a keeps faith");
    world.transfer(cfe::dop::between(b, c), Resource::Iron, cfe::dop::positive(2, 1)).expect("b keeps faith");
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
        .filter(|r| matches!(r.event(), ClosureEvent::CouncilClosed { .. }))
        .count();
    assert_eq!(closes, 3, "every member's reckoning is its own receipt");
    let held = world.orb_by_id(c).expect("orb");
    assert_eq!(held.forwards_breached, ratio(3, 1), "the stake posts breached");
    assert_eq!(world.council_breaches(c), 1, "diplomacy failed — once, counted");
    assert_eq!(world.council_breaches(b), 0);
    assert!(!world.is_imperial(c), "one breach is not an imperium");
    assert!(world.councils().is_empty(), "the council left the board");

    // Replay reproduces the banks and the books from receipts alone.
    let committed = replay(world.chronicle());
    let replayed = committed.orbs.iter().find(|o| o.orb_id == c).expect("orb");
    assert_eq!(replayed.banked_amount(Resource::Iron), ratio(7, 1));
    assert_eq!(replayed.forwards_breached, ratio(3, 1));
    assert!(committed.councils.is_empty());
}

/// Total breach is dissolution, not a slaughter: no aggrieved party
/// stands, every collateral goes home — and every breach still posts.
#[test]
fn when_all_breach_the_pact_dissolves() {
    let (mut world, b, c) = archipelago();
    world
        .seal_council(2, among(&[FIRST_ORB, b, c]), Resource::Iron, positive(2, 1), window(5), positive(3, 1))
        .expect("seals");
    for _ in 0..6 {
        world.tick();
    }
    assert_eq!(iron(&world, FIRST_ORB), ratio(6, 1), "collateral home");
    assert_eq!(iron(&world, b), ratio(6, 1));
    assert_eq!(iron(&world, c), ratio(8, 1));
    for orb in [FIRST_ORB, b, c] {
        assert_eq!(
            world.orb_by_id(orb).expect("orb").forwards_breached,
            ratio(3, 1),
            "the books forget nothing"
        );
    }
}

