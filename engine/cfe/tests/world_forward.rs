//! FORWARD-1 — THE GLASSKEEPER ESCROW (09_IDEOLOGY §2.3, 01 §7,
//! BUILD_ORDER §3). The vitrified delivery pact: the obligor locks
//! collateral (OUT of its bank — the lock gates every spend by
//! absence, and the vitrified matrix is a lossless vault by
//! construction) against an exact-rational condition — deliver ≥
//! quota of a resource to the beneficiary within the window.
//! Deliveries are DELIBERATE deeds (TRADE-1 transfers and BOND-1
//! settlements). Resolution is world-driven at the tick boundary:
//! fulfilled releases (collateral home, `forwards_fulfilled` posts
//! the stake), a passed deadline slashes (collateral to the
//! aggrieved, `forwards_breached` posts) — fulfilment wins the tie.
//! The INTEGRITY books are VALUE-weighted: a thousand toy promises
//! never wash one giant breach. A forward, once sealed, runs to its
//! end — vitrification means no cancel.

use cfe::dop::{collateral_required, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn banked(world: &World, orb: u64, resource: Resource) -> Ratio {
    world
        .orb_by_id(orb)
        .expect("orb exists")
        .banked_amount(resource)
}

/// The plain two-planet economy: source banks 30 iron / 13 biomass at
/// stamp 16, charts the charted (10i 3b | 20i 10b).
fn source_and_charted() -> (World, u64) {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let charted = world.chart("outpost", FIRST_ORB).expect("charts");
    (world, charted)
}

/// A fulfilled forward releases exactly: the collateral comes home
/// and the stake posts the fulfilled book — at the first tick after
/// the quota is met, receipted.
#[test]
fn a_fulfilled_forward_releases_exactly() {
    let (mut world, charted) = source_and_charted();
    // Empty books: the multiplier is 1 — full collateral, the empty
    // book is not a credential.
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(3, 1), 10)
        .expect("seals"); // @16: locks 3 of the source's 10 iron
    assert_eq!(banked(&world, FIRST_ORB, Resource::Iron), ratio(7, 1));

    world
        .transfer(FIRST_ORB, charted, Resource::Iron, ratio(2, 1))
        .expect("delivers two");
    world.tick(); // 2 < 3: nothing resolves
    assert_eq!(world.committed().forwards.len(), 1);

    world
        .transfer(FIRST_ORB, charted, Resource::Iron, ratio(1, 1))
        .expect("delivers the third");
    world.tick(); // quota met → RELEASE at this boundary

    let source = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(source.forwards_fulfilled, ratio(3, 1), "the stake posts");
    assert_eq!(source.forwards_breached, ratio(0, 1));
    // 10 − 3 locked − 2 − 1 delivered + 3 released = 7.
    assert_eq!(banked(&world, FIRST_ORB, Resource::Iron), ratio(7, 1));
    assert_eq!(banked(&world, charted, Resource::Iron), ratio(23, 1));
    assert!(world.committed().forwards.is_empty());
    let released = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::ForwardReleased { .. }))
        .count();
    assert_eq!(released, 1);
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the release");
}

/// A missed deadline slashes to the aggrieved at the exact boundary:
/// the collateral forfeits to the beneficiary and the stake posts the
/// breached book.
#[test]
fn a_missed_deadline_slashes_to_the_aggrieved() {
    let (mut world, charted) = source_and_charted();
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(3, 1), 5)
        .expect("seals"); // @16, deadline at 21
    world
        .transfer(FIRST_ORB, charted, Resource::Iron, ratio(1, 1))
        .expect("delivers one — not enough");
    for _ in 0..6 {
        world.tick(); // the tick opening at now=21 fires the slash
    }
    let source = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(source.forwards_breached, ratio(3, 1), "the stake posts");
    assert_eq!(source.forwards_fulfilled, ratio(0, 1));
    // The aggrieved holds the delivery AND the forfeit: 20 + 1 + 3.
    assert_eq!(banked(&world, charted, Resource::Iron), ratio(24, 1));
    assert!(world.committed().forwards.is_empty());
    let slashed = world
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            ClosureEvent::ForwardSlashed { collateral, .. } => {
                Some((collateral.clone(), r.stamp))
            }
            _ => None,
        })
        .expect("the slash is receipted");
    assert_eq!(slashed, (ratio(3, 1), 21));
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the slash");
}

/// The collateral curve reads the derived fidelity books only — high
/// fidelity earns thin collateral, the floor and the guardrail-6
/// ceiling clamp both ends, and a zero-fidelity orb pays the maximum
/// but is NEVER barred.
#[test]
fn the_collateral_curve_reads_the_books() {
    // The pure curve (placeholder parameters, clamped [1/4, 4]).
    let quota = ratio(4, 1);
    assert_eq!(
        collateral_required(&ratio(0, 1), &ratio(0, 1), &quota),
        ratio(4, 1),
        "empty books pay full collateral"
    );
    assert_eq!(
        collateral_required(&ratio(3, 1), &ratio(0, 1), &quota),
        ratio(1, 1),
        "a fulfilled history thins the lock: (0+1)/(3+1) = 1/4"
    );
    assert_eq!(
        collateral_required(&ratio(100, 1), &ratio(0, 1), &quota),
        ratio(1, 1),
        "the floor holds at 1/4 × quota"
    );
    assert_eq!(
        collateral_required(&ratio(0, 1), &ratio(100, 1), &quota),
        ratio(16, 1),
        "the ceiling holds at 4 × quota — costly, never barred"
    );

    // And the engine reads it: fulfil a 3-iron forward, then the next
    // seal locks quota × 1/4.
    let (mut world, charted) = source_and_charted();
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(3, 1), 10)
        .expect("seals");
    world
        .transfer(FIRST_ORB, charted, Resource::Iron, ratio(3, 1))
        .expect("delivers whole");
    world.tick(); // releases: fulfilled book = 3
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(4, 1), 10)
        .expect("seals again");
    let locked = world
        .chronicle()
        .receipts()
        .iter()
        .rev()
        .find_map(|r| match &r.event {
            ClosureEvent::ForwardSealed { collateral, .. } => Some(collateral.clone()),
            _ => None,
        })
        .expect("the seal is receipted");
    assert_eq!(locked, ratio(1, 1), "quota 4 × the earned 1/4 multiplier");
}

/// Locked collateral is OUT of the bank: every spend gate sees its
/// absence — the vitrified lock needs no second mechanism.
#[test]
fn locked_collateral_cannot_be_spent() {
    let (mut world, charted) = source_and_charted();
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(3, 1), 10)
        .expect("seals"); // source: 10 − 3 locked = 7
    assert_eq!(
        world.zone(16, ZoneKind::Extractor),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Iron,
            need: Box::new(ratio(10, 1)),
            have: Box::new(ratio(7, 1)),
        })
    );
    assert_eq!(
        world.transfer(FIRST_ORB, charted, Resource::Iron, ratio(8, 1)),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Iron,
            need: Box::new(ratio(8, 1)),
            have: Box::new(ratio(7, 1)),
        })
    );
}

/// BOND-1 composes: a standing bond's settlements count toward the
/// forward's quota — constancy fulfils the promise.
#[test]
fn bond_flow_counts_toward_the_quota() {
    let (mut world, charted) = source_and_charted();
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(3, 1), 12)
        .expect("seals"); // @16
    world
        .seal_bond(FIRST_ORB, charted, Resource::Iron, ratio(1, 2), 0)
        .expect("seals the bond");
    for _ in 0..6 {
        world.tick();
    }
    world.collect().expect("collects"); // @22: the bond settles 3
    world.tick(); // quota met by the stream → RELEASE
    let source = world.orb_by_id(FIRST_ORB).expect("orb");
    assert_eq!(source.forwards_fulfilled, ratio(3, 1));
    assert_eq!(source.bond_flow, ratio(3, 1), "the stream did the fulfilling");
    assert!(world.committed().forwards.is_empty());
    assert_eq!(world.committed().bonds.len(), 1, "the bond outlives the pact");
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES across the axes");
}

/// The typed refusals: one open forward per (obligor, beneficiary,
/// resource); shape errors refuse whole; the collateral shortfall is
/// named exactly.
#[test]
fn forward_refusals_are_typed() {
    let (mut world, charted) = source_and_charted();
    world
        .seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(2, 1), 5)
        .expect("seals");
    assert_eq!(
        world.seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(1, 1), 5),
        Err(ActionRefused::ForwardAlreadySealed)
    );
    assert_eq!(
        world.seal_forward(FIRST_ORB, FIRST_ORB, Resource::Iron, ratio(1, 1), 5),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.seal_forward(FIRST_ORB, charted, Resource::Iron, ratio(0, 1), 5),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.seal_forward(FIRST_ORB, charted, Resource::Biomass, ratio(1, 1), 0),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.seal_forward(FIRST_ORB, 99, Resource::Iron, ratio(1, 1), 5),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );
    // Fresh books on biomass: quota 20 wants 20 collateral; 3 stands.
    assert_eq!(
        world.seal_forward(FIRST_ORB, charted, Resource::Biomass, ratio(20, 1), 5),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Biomass,
            need: Box::new(ratio(20, 1)),
            have: Box::new(ratio(3, 1)),
        })
    );
}

/// Tags 35–37 round-trip byte-exact (the VENT-1 witness shape).
#[test]
fn forward_events_round_trip_byte_exact() {
    for (event, expected_tag) in [
        (
            ClosureEvent::ForwardSealed {
                obligor_orb: 1,
                beneficiary_orb: 2,
                resource: Resource::Iron,
                quota: ratio(3, 1),
                window: 10,
                collateral: ratio(3, 4),
            },
            35,
        ),
        (
            ClosureEvent::ForwardReleased {
                obligor_orb: 1,
                beneficiary_orb: 2,
                resource: Resource::Iron,
                collateral: ratio(3, 4),
            },
            36,
        ),
        (
            ClosureEvent::ForwardSlashed {
                obligor_orb: 1,
                beneficiary_orb: 2,
                resource: Resource::Joule,
                collateral: ratio(5, 2),
            },
            37,
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
