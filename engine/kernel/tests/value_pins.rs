//! THE VALUE LAW WAVE — the value-moving deeds enter coverage under
//! total conservation (operator directive, 2026-07-31). Red-first.
//!
//! Three tiers, one law:
//! - **Tier 1 (single-resource, sign-guarded):** every banked movement
//!   is non-negative BY DECLARATION — direction comes from the DEED,
//!   never the sign. A negative settle is a credit wearing a debit's
//!   name; it refuses `AmountNotPositive` outright. No un-guarded
//!   debit/credit path exists (the guard extends to the already-covered
//!   moves: a negative transfer or aperture leg now refuses too).
//! - **Tier 2 (escrow returns, declaration==effect):**
//!   INVARIANT SHAPE: EscrowTotal + BankedTotal = ConservedState — the
//!   stateful cross-lookup strengthens in a follow-up pass; this pass
//!   holds declaration==effect exactly, with CouncilClosed's resource
//!   read from the PRIOR sealed council (stateful, never the event).
//! - **Tier 3 (multi-resource):** conservation iterates PER RESOURCE
//!   (∀ r ∈ Resources: Δbanked(r) == declared(r)) — founding reserves,
//!   zone costs and refunds are vectors, never forced through a scalar.
//!
//! And the strengthening that binds them all: conservation is now
//! TOTAL — a covered deed with an EMPTY declaration must leave every
//! bank untouched; a smuggled bank change under a neutral deed refuses.

use cfe::dop::{founding_reserves, ClosureEvent, ColonyOrb, Resource, ZoneKind};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, Council, World, FIRST_ORB, SOVEREIGN};
use kernel::{lawful, Step, Unlawful};

const HAND: u64 = 100;

/// A one-orb committed state holding `iron`, sovereign + Collect-hand
/// seated (identity precedes the world).
fn seated(iron: (i64, i64)) -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "assay".into(), ..Default::default() };
    orb.bank(Resource::Iron, ratio(iron.0, iron.1));
    committed.orbs = vec![orb];
    committed.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "sovereign".into(), principal: None, grants: vec![] },
        Entity {
            entity_id: HAND,
            handle: "hand".into(),
            principal: Some(SOVEREIGN),
            grants: vec![Scope::Collect],
        },
    ];
    committed
}

fn step(event: ClosureEvent, actor: u64) -> Step {
    Step { event, actor, stamp: 1 }
}

fn bank_on(state: &Committed, resource: Resource, delta: Ratio) -> Committed {
    let mut next = state.clone();
    next.orbs[0].bank(resource, delta);
    next
}

#[test]
fn tier1_upkeep_settles_exactly_signed() {
    let prior = seated((6, 1));
    let upkeep = |amount: Ratio| {
        ClosureEvent::UpkeepSettled { orb_id: 1, resource: Resource::Iron, amount }
    };
    // The exact declared debit admits.
    let next = bank_on(&prior, Resource::Iron, ratio(-2, 1));
    assert_eq!(lawful(&prior, &next, &step(upkeep(ratio(2, 1)), HAND)), Ok(()));
    // A debit past the declaration refuses with both totals whole.
    let greedy = bank_on(&prior, Resource::Iron, ratio(-3, 1));
    assert!(matches!(
        lawful(&prior, &greedy, &step(upkeep(ratio(2, 1)), HAND)),
        Err(Unlawful::ConservationBroken { resource: Resource::Iron, .. })
    ));
    // THE SIGN LAW: a negative settle is a credit wearing a debit's
    // name — refused outright, before any total is read.
    let inverted = bank_on(&prior, Resource::Iron, ratio(2, 1));
    assert_eq!(
        lawful(&prior, &inverted, &step(upkeep(ratio(-2, 1)), HAND)),
        Err(Unlawful::AmountNotPositive {
            resource: Resource::Iron,
            amount: Box::new(ratio(-2, 1)),
        }),
        "direction comes from the deed, never the sign"
    );
}

#[test]
fn tier1_the_transfer_and_aperture_sign_holes_close() {
    // THE CLOSED HOLE: a negative ResourceTransferred inverted the flow
    // (the "sender" gained) while conservation held at net zero and
    // containment checked the WRONG side's authority. It now refuses.
    let prior = seated((6, 1));
    let inverted_move = ClosureEvent::ResourceTransferred {
        from_orb: 1,
        to_orb: 2,
        resource: Resource::Iron,
        amount: ratio(-3, 1),
    };
    assert_eq!(
        lawful(&prior, &prior.clone(), &step(inverted_move, HAND)),
        Err(Unlawful::AmountNotPositive {
            resource: Resource::Iron,
            amount: Box::new(ratio(-3, 1)),
        }),
        "a negative move authorizes the wrong side — refused"
    );
    // And the aperture legs likewise: a negative send is a receive
    // wearing a send's name.
    let inverted_leg = ClosureEvent::ApertureSent {
        deal: 1,
        from_orb: 1,
        resource: Resource::Iron,
        amount: ratio(-1, 1),
    };
    let credited = bank_on(&prior, Resource::Iron, ratio(1, 1));
    assert_eq!(
        lawful(&prior, &credited, &step(inverted_leg, HAND)),
        Err(Unlawful::AmountNotPositive {
            resource: Resource::Iron,
            amount: Box::new(ratio(-1, 1)),
        })
    );
}

#[test]
fn tier1_famine_draws_exactly_without_a_hand() {
    // World-driven (scope None): admits regardless of actor, but the
    // draw is bounded by its declaration and its sign.
    let prior = seated((6, 1));
    let famine = |drawn: Ratio| {
        ClosureEvent::FamineStruck { orb_id: 1, resource: Resource::Iron, drawn }
    };
    let next = bank_on(&prior, Resource::Iron, ratio(-1, 1));
    assert_eq!(lawful(&prior, &next, &step(famine(ratio(1, 1)), 999)), Ok(()));
    let overdraw = bank_on(&prior, Resource::Iron, ratio(-2, 1));
    assert!(matches!(
        lawful(&prior, &overdraw, &step(famine(ratio(1, 1)), 999)),
        Err(Unlawful::ConservationBroken { resource: Resource::Iron, .. })
    ));
    assert!(matches!(
        lawful(&prior, &prior.clone(), &step(famine(ratio(-1, 1)), 999)),
        Err(Unlawful::AmountNotPositive { .. })
    ));
}

#[test]
fn tier2_covenants_lock_and_return_declared() {
    // INVARIANT SHAPE: EscrowTotal + BankedTotal = ConservedState —
    // declaration==effect this pass (the seal debits exactly the
    // collateral; the release/slash credits exactly the collateral);
    // the stateful escrow cross-lookup strengthens in a follow-up.
    let prior = seated((6, 1));
    let sealed = bank_on(&prior, Resource::Iron, ratio(-2, 1));
    let seal = ClosureEvent::CovenantSealed {
        obligor_orb: 1,
        beneficiary_orb: 2,
        resource: Resource::Iron,
        quota: ratio(4, 1),
        window: 8,
        collateral: ratio(2, 1),
    };
    assert_eq!(lawful(&prior, &sealed, &step(seal, SOVEREIGN)), Ok(()));

    let released = bank_on(&prior, Resource::Iron, ratio(2, 1));
    let release = ClosureEvent::CovenantReleased {
        obligor_orb: 1,
        beneficiary_orb: 2,
        resource: Resource::Iron,
        collateral: ratio(2, 1),
    };
    assert_eq!(lawful(&prior, &released, &step(release, HAND)), Ok(()));
    // The slash credits the aggrieved the same declared amount
    // (world-driven, no hand).
    let slashed = bank_on(&prior, Resource::Iron, ratio(2, 1));
    let slash = ClosureEvent::CovenantSlashed {
        obligor_orb: 1,
        beneficiary_orb: 2,
        resource: Resource::Iron,
        collateral: ratio(2, 1),
    };
    assert_eq!(lawful(&prior, &slashed, &step(slash, 999)), Ok(()));
    // A release past its declaration refuses.
    let greedy = bank_on(&prior, Resource::Iron, ratio(3, 1));
    let release = ClosureEvent::CovenantReleased {
        obligor_orb: 1,
        beneficiary_orb: 2,
        resource: Resource::Iron,
        collateral: ratio(2, 1),
    };
    assert!(matches!(
        lawful(&prior, &greedy, &step(release, HAND)),
        Err(Unlawful::ConservationBroken { .. })
    ));
}

#[test]
fn tier2_council_seals_by_member_count_and_closes_from_prior() {
    // The seal locks collateral × EVERY member (multi-orb,
    // single-resource): 3 members × 2 iron = 6 iron leaves the banks.
    let prior = seated((9, 1));
    let seal = ClosureEvent::CouncilSealed {
        council: 7,
        resource: Resource::Iron,
        quota: ratio(4, 1),
        window: 8,
        collateral: ratio(2, 1),
        members: vec![1, 2, 3],
    };
    let sealed = bank_on(&prior, Resource::Iron, ratio(-6, 1));
    assert_eq!(lawful(&prior, &sealed, &step(seal.clone(), SOVEREIGN)), Ok(()));
    // Locking only one member's share under a three-member seal refuses.
    let partial = bank_on(&prior, Resource::Iron, ratio(-2, 1));
    assert!(matches!(
        lawful(&prior, &partial, &step(seal, SOVEREIGN)),
        Err(Unlawful::ConservationBroken { .. })
    ));

    // The close is STATEFUL: the resource is read from the PRIOR sealed
    // council, never from the event.
    let mut with_council = seated((3, 1));
    with_council.councils = vec![Council {
        council: 7,
        members: vec![1, 2],
        resource: Resource::Iron,
        quota: ratio(4, 1),
        delivered: vec![ratio(0, 1), ratio(0, 1)],
        collateral: ratio(2, 1),
        sealed: 0,
        window: 8,
    }];
    let close = ClosureEvent::CouncilClosed {
        council: 7,
        member: 1,
        fulfilled: true,
        banked: ratio(2, 1),
    };
    let closed = bank_on(&with_council, Resource::Iron, ratio(2, 1));
    assert_eq!(lawful(&with_council, &closed, &step(close, 999)), Ok(()));
    // An UNKNOWN council declares nothing: any banked change refuses.
    let ghost_close = ClosureEvent::CouncilClosed {
        council: 99,
        member: 1,
        fulfilled: true,
        banked: ratio(2, 1),
    };
    let smuggled = bank_on(&with_council, Resource::Iron, ratio(2, 1));
    assert!(matches!(
        lawful(&with_council, &smuggled, &step(ghost_close, 999)),
        Err(Unlawful::ConservationBroken { .. })
    ));
}

#[test]
fn tier3_the_genesis_mint_is_chartered_exactly() {
    // ColonyFounded mints EXACTLY the founding reserves — the sealed
    // rule called, never copied. Identity precedes the colony: the
    // sovereign is registered in prior (the N3 law), the orb is not.
    let mut prior = World::new().committed();
    prior.entities = vec![Entity {
        entity_id: SOVEREIGN,
        handle: "sovereign".into(),
        principal: None,
        grants: vec![],
    }];
    let mut founded = prior.clone();
    let mut orb = ColonyOrb { orb_id: FIRST_ORB, name: "origin".into(), ..Default::default() };
    for (resource, amount) in founding_reserves() {
        orb.bank(resource, amount);
    }
    founded.orbs = vec![orb];
    let deed = ClosureEvent::ColonyFounded { orb_id: FIRST_ORB, name: "origin".into() };
    assert_eq!(lawful(&prior, &founded, &step(deed.clone(), SOVEREIGN)), Ok(()));
    // A founding that mints one iron beyond the charter refuses.
    let mut greedy = founded.clone();
    greedy.orbs[0].bank(Resource::Iron, ratio(1, 1));
    assert!(matches!(
        lawful(&prior, &greedy, &step(deed, SOVEREIGN)),
        Err(Unlawful::ConservationBroken { resource: Resource::Iron, .. })
    ));
}

#[test]
fn tier3_zones_pay_their_cost_and_refund_from_prior() {
    // Ratified against the LIVE world path: the real zone() closure is
    // the next state; the kernel re-judges it. Multi-resource: the
    // cost vector is per-resource, never a scalar.
    let mut world = World::new();
    world.found("origin").expect("founds");
    let prior = world.committed();
    world.zone(0, ZoneKind::Extractor).expect("zones");
    let next = world.committed();
    let build = ClosureEvent::ZoneBuilt { orb_id: FIRST_ORB, tile: 0, kind: ZoneKind::Extractor };
    assert_eq!(
        lawful(&prior, &next, &step(build.clone(), SOVEREIGN)),
        Ok(()),
        "the live zone closure re-judges lawful"
    );
    // Underpaying the cost refuses per-resource.
    assert!(matches!(
        lawful(&prior, &prior.clone(), &step(build, SOVEREIGN)),
        Err(Unlawful::ConservationBroken { .. })
    ));

    // The raze refunds FROM THE PRIOR ZONE (stateful — the event names
    // only the tile; the kind is prior state's fact).
    let razed_prior = world.committed();
    world.raze(0).expect("razes");
    let razed_next = world.committed();
    let raze = ClosureEvent::ZoneRazed { orb_id: FIRST_ORB, tile: 0 };
    assert_eq!(
        lawful(&razed_prior, &razed_next, &step(raze.clone(), SOVEREIGN)),
        Ok(()),
        "the live raze closure re-judges lawful"
    );
    // Razing an empty tile declares nothing: a refund from nowhere
    // refuses.
    let mut ghost_refund = razed_next.clone();
    ghost_refund.orbs[0].bank(Resource::Iron, ratio(5, 1));
    assert!(matches!(
        lawful(&razed_next, &ghost_refund, &step(raze, SOVEREIGN)),
        Err(Unlawful::ConservationBroken { .. })
    ));
}

#[test]
fn the_total_check_stops_smuggling_under_neutral_deeds() {
    // THE STRENGTHENING: conservation is now TOTAL — a covered deed
    // with an empty declaration must leave EVERY bank untouched. A
    // bank tweak smuggled under a neutral deed refuses.
    let prior = seated((6, 1));
    let smuggled = bank_on(&prior, Resource::Iron, ratio(1, 1));
    let neutral = ClosureEvent::OverheatCleared { orb_id: 1 };
    assert!(matches!(
        lawful(&prior, &smuggled, &step(neutral, 999)),
        Err(Unlawful::ConservationBroken { resource: Resource::Iron, .. })
    ));
}

#[test]
fn assay_stays_out_for_the_element_ledger() {
    // AssayStruck mints ELEMENT value (a separate ledger the banked
    // totals never see). Admitting it on banked-neutrality would be an
    // unguarded element mint — it stays Unjudged until the element
    // ledger's accounting lands (the almanac's law, a named follow-up).
    let prior = seated((6, 1));
    let assay = ClosureEvent::AssayStruck { orb_id: 1, element: [3u8; 32], amount: ratio(1, 1) };
    assert_eq!(
        lawful(&prior, &prior.clone(), &step(assay, HAND)),
        Err(Unlawful::Unjudged),
        "an element mint is not admitted by banked-neutrality"
    );
}
