//! KERNEL invariant witness — conservation (CHARTER §11 rule 3, the
//! first invariant, red-first). A deed that only MOVES value may not
//! change the world's total of that value; an honest transfer admits,
//! a minting one refuses with the residual whole. The judge is pure —
//! it reads two states and a claimed step, touches nothing else.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ipsal::Scope;
use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{lawful, Step, Unlawful, VERSION};

/// A world with two orbs and value banked on the first, ready to move.
/// The origin earns before it charters (the daughter's founding
/// reserves are 20 iron + 10 biomass), leaving a surplus to transfer.
fn two_orb_world() -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones the mine");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones the farm");
    for _ in 0..30 {
        world.tick();
    }
    world.collect().expect("banks the yield");
    let _ = world.chart("haven", FIRST_ORB).expect("charts the second orb");
    world
}

#[test]
fn the_sealed_default_refuses_the_unjudged() {
    // The airtight law: a deed with no landed invariant coverage does
    // NOT enter. A founding is not yet covered by the kernel (only I1 /
    // transfers is), so the judge refuses it Unjudged — refuse-unknown,
    // not admit-unknown. Coverage will move it to a judged arm when I2
    // (containment) lands; until then it is refused, never admitted.
    let mut world = World::new();
    world.found("origin").expect("founds");
    let prior = World::new().committed();
    let next = world.committed();
    let step = Step {
        event: ClosureEvent::ColonyFounded { orb_id: FIRST_ORB, name: "origin".into() },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(
        lawful(&prior, &next, &step),
        Err(Unlawful::Unjudged),
        "the sealed kernel refuses what it cannot judge"
    );
}

/// I2 — a transfer (Collect scope) by an agent WITHOUT Collect refuses
/// NotPermitted; the same agent, once granted, is admitted; the sovereign
/// (all scopes by construction) always passes; a non-entity refuses
/// NoSuchEntity before any state is read. The kernel's scope_of(Collect)
/// for ResourceTransferred is ratified here against world.rs.
#[test]
fn containment_gates_the_transfer_by_scope() {
    let mut world = two_orb_world();
    let haven = world.orbs()[1].orb_id;
    let agent = world.register_agent("hand").expect("registers");
    let prior = world.committed();
    world
        .transfer(FIRST_ORB, haven, Resource::Iron, ratio(3, 1))
        .expect("the move closes");
    let next = world.committed();
    let stamp = world.now();
    let step = |actor: u64| Step {
        event: ClosureEvent::ResourceTransferred {
            from_orb: FIRST_ORB,
            to_orb: haven,
            resource: Resource::Iron,
            amount: ratio(3, 1),
        },
        actor,
        stamp,
    };
    // The agent lacks Collect: refused, by scope.
    assert_eq!(
        lawful(&prior, &next, &step(agent)),
        Err(Unlawful::NotPermitted { actor: agent, scope: Scope::Collect }),
        "authority is granted, never assumed"
    );
    // The sovereign holds every scope: admitted.
    assert_eq!(lawful(&prior, &next, &step(SOVEREIGN)), Ok(()));
    // A non-entity: refused before any state read.
    assert_eq!(
        lawful(&prior, &next, &step(999)),
        Err(Unlawful::NoSuchEntity { actor: 999 }),
        "identity precedes the world"
    );
    // Grant the agent Collect: now admitted.
    world.grant(agent, Scope::Collect).expect("grants");
    let granted_prior = world.committed();
    assert_eq!(lawful(&granted_prior, &next, &step(agent)), Ok(()));
}

#[test]
fn exactness_holds_over_the_value_ledgers_and_the_primitive_is_sealed() {
    // I3 passes on a real played state (every banked value is canonical by
    // construction), and the kernel CALLS the sealed primitive — witnessed
    // here so a future non-canonical path (a bad decode) would refuse.
    let world = two_orb_world();
    let prior = world.committed();
    let next = world.committed();
    let step = Step {
        // An authority deed carries no value: exactness is vacuous on it
        // and the deed rides through I3 on the state, not the event.
        event: ClosureEvent::ScopeGranted { agent: 2, scope: Scope::Collect },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    // ScopeGranted is now COVERED (I2 containment + I3 exactness — its full
    // set): no longer Unjudged. The sovereign holds Build, so it admits.
    assert_eq!(lawful(&prior, &next, &step), Ok(()), "the authority deed is judged, not skipped");
    // The sealed canonical primitive: every constructed Ratio is canonical.
    assert!(cfe::ratio::is_canonical(&ratio(1, 2)));
    assert!(cfe::ratio::is_canonical(&ratio(2, 4)), "2/4 constructs AS 1/2 — canonical");
    assert!(cfe::ratio::is_canonical(&ratio(-3, 1)));
}

#[test]
fn the_version_is_pinned() {
    // The commitment binds this; an amendment bumps it. Pinned so a
    // silent change to the judge cannot pass unnoticed.
    assert_eq!(VERSION, 1, "the kernel version is charter-pinned");
}

#[test]
fn an_honest_transfer_is_admitted() {
    let mut world = two_orb_world();
    let haven = world.orbs()[1].orb_id;
    let prior = world.committed();
    world
        .transfer(FIRST_ORB, haven, Resource::Iron, ratio(3, 1))
        .expect("the move closes");
    let next = world.committed();
    let step = Step {
        event: ClosureEvent::ResourceTransferred {
            from_orb: FIRST_ORB,
            to_orb: haven,
            resource: Resource::Iron,
            amount: ratio(3, 1),
        },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(
        lawful(&prior, &next, &step),
        Ok(()),
        "value moved, total unchanged — the judge admits"
    );
}

#[test]
fn a_minting_transfer_refuses_with_the_residual_whole() {
    // The forger writes a real transfer shape but the `next` state
    // MINTS: the destination gains more iron than the source lost.
    // The world's total rises, and the judge refuses by measurement —
    // both totals carried, never a scalar verdict.
    let mut world = two_orb_world();
    let haven = world.orbs()[1].orb_id;
    let prior = world.committed();
    let before = prior
        .orbs
        .iter()
        .map(|orb| orb.banked_amount(Resource::Iron))
        .fold(ratio(0, 1), |sum, held| sum + held);
    // Play the honest move, then corrupt `next` — credit the
    // destination an extra 2 iron the source never paid.
    world
        .transfer(FIRST_ORB, haven, Resource::Iron, ratio(3, 1))
        .expect("the move closes");
    let mut next = world.committed();
    if let Some(orb) = next.orbs.iter_mut().find(|orb| orb.orb_id == haven) {
        orb.bank(Resource::Iron, ratio(2, 1));
    }
    let after = next
        .orbs
        .iter()
        .map(|orb| orb.banked_amount(Resource::Iron))
        .fold(ratio(0, 1), |sum, held| sum + held);
    let step = Step {
        event: ClosureEvent::ResourceTransferred {
            from_orb: FIRST_ORB,
            to_orb: haven,
            resource: Resource::Iron,
            amount: ratio(3, 1),
        },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(
        lawful(&prior, &next, &step),
        Err(Unlawful::ConservationBroken {
            resource: Resource::Iron,
            before: Box::new(before.clone()),
            after: Box::new(after.clone()),
        }),
        "the mint broke conservation — before {before}, after {after}, the residual is theirs to read"
    );
    assert_eq!(after - before, ratio(2, 1), "the residual is exactly the minted 2 iron");
}
