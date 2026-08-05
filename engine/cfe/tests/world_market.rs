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
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1), None, None).expect("sites");
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

    clear_trade(&mut furnace, 7, &mut frost, 9, 100, ratio(0, 1)).expect("the crossing clears");

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

/// MARKET-2 (Samuelson iceberg, ruled 2026-07-31 — CONVERTS the
/// 11 §2.1 side-fee): the manifold gap MELTS the cargo. Aligned,
/// local pairs deliver whole; a clashing or distant pair's receiver
/// banks sent × (1 − φ·D), the void's share derivable from the
/// aperture pair (sent − received); at φ·D ≥ 1 the pair is
/// economically WALLED by decay physics, refused before movement.
#[test]
fn the_gap_melts_the_cargo_and_distance_walls() {
    use cfe::dop::ClosureEvent;
    // Aligned local pair: delivered whole.
    let mut a = sunlit("concord-a");
    let mut b = sunlit("concord-b");
    a.post_offer(FIRST_ORB, Resource::Iron, ratio(2, 1), Resource::Biomass, ratio(1, 1), 1)
        .expect("posts");
    b.post_offer(FIRST_ORB, Resource::Biomass, ratio(1, 1), Resource::Iron, ratio(2, 1), 2)
        .expect("posts");
    let b_iron = b.orb().unwrap().banked_amount(Resource::Iron);
    clear_trade(&mut a, 1, &mut b, 2, 10, ratio(0, 1)).expect("clears");
    assert_eq!(
        b.orb().unwrap().banked_amount(Resource::Iron) - b_iron,
        ratio(2, 1),
        "alignment at zero distance delivers whole"
    );

    // Clashing pair at spatial distance 6: melt = (6 + 1/2)/64 = 13/128.
    let mut c = sunlit("discord-c");
    let mut d = sunlit("discord-d");
    c.shift_gauge(ratio(1, 2));
    c.post_offer(FIRST_ORB, Resource::Iron, ratio(2, 1), Resource::Biomass, ratio(1, 1), 3)
        .expect("posts");
    d.post_offer(FIRST_ORB, Resource::Biomass, ratio(1, 1), Resource::Iron, ratio(2, 1), 4)
        .expect("posts");
    let d_iron = d.orb().unwrap().banked_amount(Resource::Iron);
    clear_trade(&mut c, 3, &mut d, 4, 20, ratio(6, 1)).expect("clears");
    // d receives 2 × (1 − 13/128) = 115/64; the void took 13/64.
    assert_eq!(
        d.orb().unwrap().banked_amount(Resource::Iron) - d_iron,
        ratio(115, 64),
        "the void takes exactly φ·D of the cargo"
    );
    let received = d.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::ApertureReceived { resource: Resource::Iron, amount, .. } => {
            Some(amount.clone())
        }
        _ => None,
    });
    assert_eq!(received, Some(ratio(115, 64)), "the receipt speaks what LANDED");
    // THE CONFORM RULING: the void's share is a first-class receipt
    // on the receiving book — sent = received + void share; every
    // deal sums to zero under ONE rule, and the judge only sums.
    let void_share = d.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::VoidShare { resource: Resource::Iron, amount, .. } => Some(amount.clone()),
        _ => None,
    });
    assert_eq!(void_share, Some(ratio(13, 64)), "the void's share is receipted, never silent");
    assert_eq!(
        ratio(115, 64) + ratio(13, 64),
        ratio(2, 1),
        "sent = received + void share — zero-sum stays universal"
    );

    // Distance 64 walls the pair: melt ≥ 1, refused before movement.
    let mut e = sunlit("far-e");
    let mut f = sunlit("far-f");
    e.post_offer(FIRST_ORB, Resource::Iron, ratio(2, 1), Resource::Biomass, ratio(1, 1), 5)
        .expect("posts");
    f.post_offer(FIRST_ORB, Resource::Biomass, ratio(1, 1), Resource::Iron, ratio(2, 1), 6)
        .expect("posts");
    let iron_before = e.orb().unwrap().banked_amount(Resource::Iron);
    assert!(
        matches!(
            clear_trade(&mut e, 5, &mut f, 6, 30, ratio(64, 1)),
            Err(cfe::world::ActionRefused::EconomicallyWalled { .. })
        ),
        "decay physics wall the void — no rule needed"
    );
    assert_eq!(e.orb().unwrap().banked_amount(Resource::Iron), iron_before, "nothing moved");
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
        clear_trade(&mut a, 1, &mut b, 2, 5, ratio(0, 1)),
        Err(ActionRefused::NoCrossing)
    );
    assert_eq!(a.offers().len(), 1, "the intent still rests");

    // A pre-solar counterpart is unreachable even with crossing prices.
    let mut shielded = World::new();
    shielded.found("darkhold").expect("founds");
    shielded
        .post_offer(FIRST_ORB, Resource::Biomass, ratio(4, 1), Resource::Iron, ratio(2, 1), 3)
        .expect("posts — resting is lawful even under the shield");
    match clear_trade(&mut a, 1, &mut shielded, 3, 6, ratio(0, 1)) {
        Err(ActionRefused::Shielded { .. }) => {}
        other => panic!("the shield must guard the fill, got {other:?}"),
    }
}

/// ROUTE-1: a chartered lane bends the iceberg — melt × (1 − η) —
/// and an unpaid lane LAPSES by receipt, raw space returning. The
/// charter is two-legged (one deal, both books) and replays exactly.
#[test]
fn the_lane_bends_the_melt_then_lapses() {
    use cfe::dop::ClosureEvent;
    use cfe::world::charter_route;
    // Both partners pay for infrastructure: the smelting arc banks
    // the charter's alloy (the world_industry staging).
    let alloyed = |name: &str| {
        let mut world = sunlit(name);
        world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones");
        world.zone(1, cfe::dop::ZoneKind::Extractor).expect("zones");
        world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones");
        world.zone(9, cfe::dop::ZoneKind::Habitat).expect("zones");
        for _ in 0..10 {
            world.tick();
        }
        world.collect().expect("collects");
        world.zone(16, cfe::dop::ZoneKind::Smelter).expect("the chain stands");
        for tick in 1..=60 {
            world.tick();
            if tick % 10 == 0 {
                let _ = world.collect();
            }
        }
        world
    };
    let mut a = alloyed("waypoint-a");
    let mut b = alloyed("waypoint-b");
    charter_route(&mut a, FIRST_ORB, &mut b, FIRST_ORB, ratio(1, 2), 40).expect("charters");
    assert!(
        matches!(
            charter_route(&mut a, FIRST_ORB, &mut b, FIRST_ORB, ratio(1, 2), 40),
            Err(cfe::world::ActionRefused::RouteKnown { deal: 40 })
        ),
        "one charter per deal"
    );
    // Distance 16, no gap: raw melt 16/64 = 1/4; the lane halves it
    // to 1/8 — delivered = 2 × 7/8 = 7/4.
    a.post_offer(FIRST_ORB, Resource::Iron, ratio(2, 1), Resource::Biomass, ratio(1, 1), 1)
        .expect("posts");
    b.post_offer(FIRST_ORB, Resource::Biomass, ratio(1, 1), Resource::Iron, ratio(2, 1), 2)
        .expect("posts");
    let b_iron = b.orb().unwrap().banked_amount(Resource::Iron);
    clear_trade(&mut a, 1, &mut b, 2, 50, ratio(16, 1)).expect("clears");
    assert_eq!(
        b.orb().unwrap().banked_amount(Resource::Iron) - b_iron,
        ratio(7, 4),
        "the lane bends the melt exactly"
    );
    // Starve b's lane: repeated maintenance draws drain the joule
    // bank until the lapse lands — the pair dies with either leg.
    let mut lapsed = Vec::new();
    for _ in 0..64 {
        lapsed = b.maintain_routes(FIRST_ORB);
        if !lapsed.is_empty() {
            break;
        }
    }
    assert_eq!(lapsed, vec![40], "the unpaid lane lapses by receipt");
    assert!(b.routes().is_empty(), "raw space takes the path back");
    let receipted = b.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::RouteLapsed { deal: 40 })
    });
    assert!(receipted);
    // Replay MATCHES on both books.
    assert_eq!(cfe::world::replay(a.chronicle()).routes, a.routes());
    assert_eq!(cfe::world::replay(b.chronicle()).routes, b.routes());
}
