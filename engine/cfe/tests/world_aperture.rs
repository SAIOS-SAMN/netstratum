//! NS-4 APERTURE — THE CROSS-WORLD VALUE CARRIER (BUILD_ORDER §3a).
//! Banked value crosses from an orb in ONE world to an orb in ANOTHER,
//! conservation-exact across the pair, receipted in BOTH chronicles as
//! a two-legged deed sharing one deal id: `ApertureSent` debits the
//! source, `ApertureReceived` credits the destination. No single
//! chronicle proves conservation — the deal id links the legs (double
//! entry across two books). Each leg replays in its own world (SEAL B
//! across the pair). The cross-empire cohesion books DO NOT post here:
//! cross-world reading composes at N5/N6 (09 board law).

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{aperture_transfer, replay, ActionRefused, World, FIRST_ORB};

fn banked(world: &World, orb: u64, resource: Resource) -> Ratio {
    world.orb_by_id(orb).expect("orb exists").banked_amount(resource)
}

fn count<F: Fn(&ClosureEvent) -> bool>(world: &World, pred: F) -> usize {
    world.chronicle().receipts().iter().filter(|r| pred(&r.event)).count()
}

/// Conservation across the pair: value leaves the source world and
/// arrives in the destination, exact; one leg receipted in each
/// chronicle, none crossed; the legs share the deal id.
#[test]
fn an_aperture_conserves_across_two_worlds() {
    let mut alpha = World::new();
    alpha.found("meridian").expect("founds alpha");
    let mut beta = World::new();
    beta.found("haven").expect("founds beta");

    // Founding reserves give each orb 20 iron. Cross 5 alpha → beta.
    aperture_transfer(
        &mut alpha,
        FIRST_ORB,
        &mut beta,
        FIRST_ORB,
        Resource::Iron,
        ratio(5, 1),
        7,
    )
    .expect("the aperture opens");

    assert_eq!(banked(&alpha, FIRST_ORB, Resource::Iron), ratio(15, 1), "source debited");
    assert_eq!(banked(&beta, FIRST_ORB, Resource::Iron), ratio(25, 1), "dest credited");
    // Conservation across the PAIR: what left alpha entered beta, exact.
    let pair = banked(&alpha, FIRST_ORB, Resource::Iron)
        + banked(&beta, FIRST_ORB, Resource::Iron);
    assert_eq!(pair, ratio(40, 1), "the pair minted nothing");

    // One leg in each chronicle, none crossed.
    assert_eq!(count(&alpha, |e| matches!(e, ClosureEvent::ApertureSent { .. })), 1);
    assert_eq!(count(&alpha, |e| matches!(e, ClosureEvent::ApertureReceived { .. })), 0);
    assert_eq!(count(&beta, |e| matches!(e, ClosureEvent::ApertureReceived { .. })), 1);
    assert_eq!(count(&beta, |e| matches!(e, ClosureEvent::ApertureSent { .. })), 0);

    // The legs share the deal id.
    let sent_deal = alpha
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            ClosureEvent::ApertureSent { deal, .. } => Some(*deal),
            _ => None,
        })
        .expect("a sent leg");
    let recv_deal = beta
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            ClosureEvent::ApertureReceived { deal, .. } => Some(*deal),
            _ => None,
        })
        .expect("a received leg");
    assert_eq!(sent_deal, recv_deal, "the deal links the legs");
    assert_eq!(sent_deal, 7);
}

/// Typed refusals, each naming its cure — and no orphan leg on refusal.
#[test]
fn an_aperture_refuses_typed() {
    let mut alpha = World::new();
    alpha.found("meridian").expect("founds");
    let mut beta = World::new();
    beta.found("haven").expect("founds");

    // Cannot afford — the exact shortfall named.
    assert_eq!(
        aperture_transfer(
            &mut alpha,
            FIRST_ORB,
            &mut beta,
            FIRST_ORB,
            Resource::Iron,
            ratio(100, 1),
            1
        ),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Iron,
            need: Box::new(ratio(100, 1)),
            have: Box::new(ratio(20, 1)),
        })
    );
    // Non-positive amount.
    assert_eq!(
        aperture_transfer(
            &mut alpha,
            FIRST_ORB,
            &mut beta,
            FIRST_ORB,
            Resource::Iron,
            ratio(0, 1),
            1
        ),
        Err(ActionRefused::BadTransfer)
    );
    // No such source / destination orb.
    assert_eq!(
        aperture_transfer(&mut alpha, 99, &mut beta, FIRST_ORB, Resource::Iron, ratio(1, 1), 1),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );
    assert_eq!(
        aperture_transfer(&mut alpha, FIRST_ORB, &mut beta, 99, Resource::Iron, ratio(1, 1), 1),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );
    // No leg written on any refusal — a refusal orphans nothing.
    assert_eq!(count(&alpha, |e| matches!(e, ClosureEvent::ApertureSent { .. })), 0);
    assert_eq!(count(&beta, |e| matches!(e, ClosureEvent::ApertureReceived { .. })), 0);
}

/// SEAL B across the pair: each leg replays in its OWN world — the
/// source projects the debit, the destination the credit; the resumed
/// banks match the live banks exactly.
#[test]
fn both_legs_replay_in_their_own_world() {
    let mut alpha = World::new();
    alpha.found("meridian").expect("founds");
    let mut beta = World::new();
    beta.found("haven").expect("founds");
    aperture_transfer(
        &mut alpha,
        FIRST_ORB,
        &mut beta,
        FIRST_ORB,
        Resource::Iron,
        ratio(8, 1),
        3,
    )
    .expect("the aperture opens");

    let alpha_live = banked(&alpha, FIRST_ORB, Resource::Iron);
    let beta_live = banked(&beta, FIRST_ORB, Resource::Iron);
    assert_eq!(alpha_live, ratio(12, 1));
    assert_eq!(beta_live, ratio(28, 1));

    let alpha_replay = replay(alpha.chronicle());
    let beta_replay = replay(beta.chronicle());
    let alpha_orb =
        alpha_replay.orbs.iter().find(|o| o.orb_id == FIRST_ORB).expect("replayed orb");
    let beta_orb =
        beta_replay.orbs.iter().find(|o| o.orb_id == FIRST_ORB).expect("replayed orb");
    assert_eq!(alpha_orb.banked_amount(Resource::Iron), alpha_live, "source leg replays");
    assert_eq!(beta_orb.banked_amount(Resource::Iron), beta_live, "dest leg replays");
}

/// The two legs round-trip their wire records byte-exact (SEAL A).
#[test]
fn the_aperture_legs_round_trip_byte_exact() {
    for event in [
        ClosureEvent::ApertureSent {
            deal: 7,
            from_orb: 1,
            resource: Resource::Joule,
            amount: ratio(5, 2),
        },
        ClosureEvent::ApertureReceived {
            deal: 7,
            to_orb: 2,
            resource: Resource::Joule,
            amount: ratio(5, 2),
        },
    ] {
        let bytes = event.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        let decoded = ClosureEvent::from_wire(tag, value).expect("known tag decodes");
        assert_eq!(decoded, event, "value round trip");
        assert_eq!(decoded.wire_bytes(), bytes, "BYTE round trip");
    }
}
