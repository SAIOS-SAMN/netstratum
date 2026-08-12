//! MARKET-1 — THE EXCHANGE RATE (BUILD_ORDER §3a, tags 49–50). An
//! offer is a receipted intent, never an escrow; the price IS the
//! ratio (no absolute price — guardrail 6); an unmatched offer RESTS;
//! a crossing clears at the RESTING offer's exact price (price-time
//! priority) as two aperture pairs — shield-checked, demurrage-
//! touched, conservation-exact both directions; replay reproduces
//! the whole book.

use cfe::dop::{exchange, positive, unit};

use cfe::dop::Resource;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{clear_trade, replay, ActionRefused, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    // EXPLICIT SHARED PHASE: these pins state melt-zero arithmetic
    // (resting-price fills, exact conservation), and Phase-2 law gives
    // auto-sited worlds distinct orbital phases — D_orbit > 0 even at
    // chart distance 0, which melts fills and demands hull capacity.
    // The pins pin the MARKET law, not the orbit law, so the fixture
    // sits both worlds at one phase; the orbit law keeps its own pins.
    let phase = || Some(cfe::dop::Slot::from_index(0));
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), phase(), phase())
        .expect("sites");
    world
}

/// FLEET-1: a positive-melt haul needs hull capacity, so the
/// melt-positive pins stage a grade-0 hauler on each book — capacity
/// without arithmetic drift (`hull_eta(Hauler, 0) = 0`, so the melt
/// the pin states is the melt that runs).
fn equip_hauler(world: &mut World) {
    world.zone(40, cfe::dop::ZoneKind::ShipYard).expect("yard");
    world
        .salvage_scrap(FIRST_ORB, positive(4, 1))
        .expect("scrap");
    world
        .mint_hull(FIRST_ORB, cfe::dop::HullRole::Hauler, 0)
        .expect("hauler");
}

/// Posting refuses typed; an unmatched offer RESTS in committed state
/// and replays.
#[test]
fn an_offer_rests_and_replays() {
    let mut world = sunlit("bazaar");
    assert!(
        cfe::dop::Positive::new(ratio(0, 1)).is_none(),
        "a zero amount cannot even be constructed"
    );
    assert!(
        cfe::dop::Exchange::new(Resource::Iron, Resource::Iron).is_none(),
        "a reflexive trade cannot even be constructed"
    );
    world
        .post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(4, 1), positive(2, 1), 1)
        .expect("posts");
    assert_eq!(
        world.post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(1, 1), positive(1, 1), 1),
        Err(ActionRefused::OfferKnown { offer: 1 }),
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
        .post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(4, 1), positive(2, 1), 7)
        .expect("rests");
    frost.tick();
    // The Frost bids better than asked: 3 biomass for 3 iron.
    frost
        .post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(3, 1), positive(3, 1), 9)
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
    a.post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(2, 1), positive(1, 1), 1)
        .expect("posts");
    b.post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(1, 1), positive(2, 1), 2)
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
    equip_hauler(&mut c);
    equip_hauler(&mut d);
    c.shift_gauge(ratio(1, 2));
    c.post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(2, 1), positive(1, 1), 3)
        .expect("posts");
    d.post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(1, 1), positive(2, 1), 4)
        .expect("posts");
    let d_iron = d.orb().unwrap().banked_amount(Resource::Iron);
    clear_trade(&mut c, 3, &mut d, 4, 20, ratio(6, 1)).expect("clears");
    // THE COMPOSED CROSSING LAW (ratified 2026-08-12 — this pin's
    // "exactly φ·D" predated P5-A and went red the day interdiction
    // landed, unseen behind the fail-fast blindness):
    //   after_melt   = 2 × (1 − 13/128)          = 115/64
    //   interdicted  = after_melt × risk/2, risk = 3/16 uncovered
    //                  (bare eighth + slow-ring half-eighth, no
    //                  escort on the deal)        = 345/2048
    //   delivered    = 115/64 − 345/2048         = 3335/2048
    // Every term is receipted: melt in VoidShare, the residual in
    // InterdictionStruck, and sent = delivered + void, exactly.
    assert_eq!(
        d.orb().unwrap().banked_amount(Resource::Iron) - d_iron,
        ratio(3335, 2048),
        "the void takes φ·D and the uncovered convoy pays its residual"
    );
    let received = d.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::ApertureReceived { resource: Resource::Iron, amount, .. } => {
            Some(amount.clone())
        }
        _ => None,
    });
    assert_eq!(received, Some(ratio(3335, 2048)), "the receipt speaks what LANDED");
    // P5-A: the interdiction residual is receipted on the SENDING
    // book, wearing its risk.
    let interdicted = c.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::InterdictionStruck { resource: Resource::Iron, amount, risk, .. } => {
            Some((amount.clone(), risk.clone()))
        }
        _ => None,
    });
    assert_eq!(
        interdicted,
        Some((ratio(345, 2048), ratio(3, 16))),
        "the convoy's loss is receipted, never silent"
    );
    // THE CONFORM RULING: the void's share is a first-class receipt
    // on the receiving book — sent = received + void share; every
    // deal sums to zero under ONE rule, and the gate only sums.
    let void_share = d.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::VoidShare { resource: Resource::Iron, amount, .. } => Some(amount.clone()),
        _ => None,
    });
    assert_eq!(void_share, Some(ratio(761, 2048)), "the void's share is receipted, never silent");
    assert_eq!(
        ratio(3335, 2048) + ratio(761, 2048),
        ratio(2, 1),
        "sent = received + void share — zero-sum stays universal"
    );

    // Distance 64 walls the pair: melt ≥ 1, refused before movement.
    let mut e = sunlit("far-e");
    let mut f = sunlit("far-f");
    e.post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(2, 1), positive(1, 1), 5)
        .expect("posts");
    f.post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(1, 1), positive(2, 1), 6)
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
    a.post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(2, 1), positive(4, 1), 1)
        .expect("posts");
    // B asks MORE iron per biomass than A offers — the prices never
    // cross (A wants 2 biomass per iron; B gives at most 1/2).
    b.post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(1, 1), positive(2, 1), 2)
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
        .post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(4, 1), positive(2, 1), 3)
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
    // CONSTRUCTED STATE, not weather (realignment 2026-08-12): this
    // pin's charter is the LANE law — the melt bend and the lapse —
    // and it used to stage its alloy by simulating a 60-tick smelting
    // economy, which broke every time any economic factor moved (the
    // taxon yield scales left the farm net-zero and the 34 balance
    // gate refused the hauler's salvage after the venting storm). The
    // fixture now mints its inputs through a test warrant (the
    // instrument_pins pattern) and stages the hauler inside the
    // bootstrap window; the receipts still replay whole.
    let alloyed = |name: &str| {
        let mut world = sunlit(name);
        equip_hauler(&mut world);
        struct TestWarrant;
        impl cfe::world::MintWarrant for TestWarrant {
            fn earns(&self, work: &[u8]) -> Option<cfe::ratio::Ratio> {
                Some(ratio(i64::from(work.first().copied().unwrap_or(1)), 1))
            }
            fn admits(
                &self,
                _prior: &cfe::world::Committed,
                _next: &cfe::world::Committed,
                _event: &ClosureEvent,
                _actor: u64,
                _stamp: u64,
            ) -> bool {
                true
            }
        }
        world
            .mint_discovery_as(
                cfe::world::SOVEREIGN,
                FIRST_ORB,
                Resource::Alloy,
                cfe::dop::NonNegative::new(ratio(8, 1)).expect("nn"),
                vec![8u8; 32],
                &TestWarrant,
            )
            .expect("the warrant banks the charter's alloy");
        world
    };
    let mut a = alloyed("waypoint-a");
    let mut b = alloyed("waypoint-b");
    charter_route(&mut a, FIRST_ORB, &mut b, FIRST_ORB, unit(1, 2), 40).expect("charters");
    assert!(
        matches!(
            charter_route(&mut a, FIRST_ORB, &mut b, FIRST_ORB, unit(1, 2), 40),
            Err(cfe::world::ActionRefused::RouteKnown { deal: 40 })
        ),
        "one charter per deal"
    );
    // Distance 16, no gap: raw melt 16/64 = 1/4; the lane halves it
    // to 1/8 — after_melt = 2 × 7/8 = 7/4. P5-A then takes the
    // uncovered convoy's residual (risk 3/16, half applied):
    // delivered = 7/4 × (1 − 3/32) = 203/128 (ratified 2026-08-12 —
    // the bare 7/4 predated interdiction).
    a.post_offer(FIRST_ORB, exchange(Resource::Iron, Resource::Biomass), positive(2, 1), positive(1, 1), 1)
        .expect("posts");
    b.post_offer(FIRST_ORB, exchange(Resource::Biomass, Resource::Iron), positive(1, 1), positive(2, 1), 2)
        .expect("posts");
    let b_iron = b.orb().unwrap().banked_amount(Resource::Iron);
    clear_trade(&mut a, 1, &mut b, 2, 50, ratio(16, 1)).expect("clears");
    assert_eq!(
        b.orb().unwrap().banked_amount(Resource::Iron) - b_iron,
        ratio(203, 128),
        "the lane bends the melt exactly; the convoy pays its residual"
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
