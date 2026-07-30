//! MARKET-1 — THE EXCHANGE RATE (BUILD_ORDER §3a, tags 49–50). An
//! offer is a receipted intent, never an escrow; the price IS the
//! ratio (no absolute price — guardrail 6); an unmatched offer RESTS;
//! a crossing clears at the RESTING offer's exact price (price-time
//! priority) as two aperture pairs — shield-checked, demurrage-
//! touched, conservation-exact both directions; replay reproduces
//! the whole book.

use cfe::dop::Resource;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{clear_trade, replay, ActionRefused, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    world
}

/// Posting refuses typed; an unmatched offer RESTS in committed state
/// and replays.
#[test]
fn an_offer_rests_and_replays() {
    let mut world = sunlit("bazaar");
    assert_eq!(
        world.post_offer(FIRST_ORB, Resource::Iron, ratio(0, 1), Resource::Biomass, ratio(1, 1), 1),
        Err(ActionRefused::BadOffer),
        "positive amounts only"
    );
    assert_eq!(
        world.post_offer(FIRST_ORB, Resource::Iron, ratio(1, 1), Resource::Iron, ratio(1, 1), 1),
        Err(ActionRefused::BadOffer),
        "two DISTINCT resources"
    );
    world
        .post_offer(FIRST_ORB, Resource::Iron, ratio(4, 1), Resource::Biomass, ratio(2, 1), 1)
        .expect("posts");
    assert_eq!(
        world.post_offer(FIRST_ORB, Resource::Iron, ratio(1, 1), Resource::Biomass, ratio(1, 1), 1),
        Err(ActionRefused::BadOffer),
        "a fresh id per intent"
    );
    assert_eq!(world.offers().len(), 1, "the unmatched offer RESTS");
    assert_eq!(replay(world.chronicle()).offers.len(), 1, "the book replays");
}

/// The crossing clears at the RESTING price exactly; conservation
/// holds across both resources and both worlds; the intents close;
/// replay reproduces the fill.
#[test]
fn a_crossing_clears_at_the_resting_price() {
    let mut furnace = sunlit("emberside");
    let mut frost = sunlit("rimeside");
    // The Furnace rests first: 4 iron for 2 biomass (price 1/2 per).
    furnace
        .post_offer(FIRST_ORB, Resource::Iron, ratio(4, 1), Resource::Biomass, ratio(2, 1), 7)
        .expect("rests");
    frost.tick();
    // The Frost bids better than asked: 3 biomass for 3 iron.
    frost
        .post_offer(FIRST_ORB, Resource::Biomass, ratio(3, 1), Resource::Iron, ratio(3, 1), 9)
        .expect("posts");

    let iron_before = furnace.orb().unwrap().banked_amount(Resource::Iron)
        + frost.orb().unwrap().banked_amount(Resource::Iron);
    let biomass_before = furnace.orb().unwrap().banked_amount(Resource::Biomass)
        + frost.orb().unwrap().banked_amount(Resource::Biomass);

    clear_trade(&mut furnace, 7, &mut frost, 9, 100).expect("the crossing clears");

    // Fill: x = min(4, 3) = 3 iron; y at the RESTING price (1/2
    // biomass per iron) = 3/2 biomass — the earlier offer ruled, the
    // late bidder keeps its surplus.
    assert_eq!(
        frost.orb().unwrap().banked_amount(Resource::Iron),
        ratio(23, 1),
        "20 founding + 3 filled"
    );
    assert_eq!(
        furnace.orb().unwrap().banked_amount(Resource::Biomass),
        ratio(23, 2),
        "10 founding + 3/2 at the resting price"
    );
    // Conservation across the pair, both resources.
    let iron_after = furnace.orb().unwrap().banked_amount(Resource::Iron)
        + frost.orb().unwrap().banked_amount(Resource::Iron);
    let biomass_after = furnace.orb().unwrap().banked_amount(Resource::Biomass)
        + frost.orb().unwrap().banked_amount(Resource::Biomass);
    assert_eq!(iron_after, iron_before, "the market minted no iron");
    assert_eq!(biomass_after, biomass_before, "the market minted no biomass");

    // Both intents closed; both books replay.
    assert!(furnace.offers().is_empty() && frost.offers().is_empty());
    assert!(replay(furnace.chronicle()).offers.is_empty());
    let replayed = replay(frost.chronicle());
    assert_eq!(
        replayed.orbs[0].banked_amount(Resource::Iron),
        ratio(23, 1),
        "the fill replays from receipts alone"
    );
}

/// No crossing: wrong resources, or prices that never meet — refused
/// typed, nothing moves. The shield still guards the fill.
#[test]
fn no_crossing_no_movement() {
    let mut a = sunlit("askance");
    let mut b = sunlit("beneath");
    a.post_offer(FIRST_ORB, Resource::Iron, ratio(2, 1), Resource::Biomass, ratio(4, 1), 1)
        .expect("posts");
    // B asks MORE iron per biomass than A offers — the prices never
    // cross (A wants 2 biomass per iron; B gives at most 1/2).
    b.post_offer(FIRST_ORB, Resource::Biomass, ratio(1, 1), Resource::Iron, ratio(2, 1), 2)
        .expect("posts");
    assert_eq!(
        clear_trade(&mut a, 1, &mut b, 2, 5),
        Err(ActionRefused::NoCrossing)
    );
    assert_eq!(a.offers().len(), 1, "the intent still rests");

    // A pre-solar counterpart is unreachable even with crossing prices.
    let mut shielded = World::new();
    shielded.found("darkhold").expect("founds");
    shielded
        .post_offer(FIRST_ORB, Resource::Biomass, ratio(4, 1), Resource::Iron, ratio(2, 1), 3)
        .expect("posts — resting is lawful even under the shield");
    match clear_trade(&mut a, 1, &mut shielded, 3, 6) {
        Err(ActionRefused::Shielded { .. }) => {}
        other => panic!("the shield must guard the fill, got {other:?}"),
    }
}
