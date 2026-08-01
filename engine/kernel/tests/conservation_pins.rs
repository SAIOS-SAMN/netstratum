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
fn the_last_deed_covered_exhausts_the_sealed_default() {
    // THE TRANSITION (recorded): this pin rode the last-uncovered deed
    // through four coverage waves — ColonyFounded (value law),
    // EraAnnexed (gauge law), AssayStruck (element law), TaxonSealed
    // (the last). TaxonSealed now COVERS, so `Unjudged` is unreachable
    // via any current deed. The pin transforms: it witnesses that the
    // final holdout is judged (admits, prior == next, neutral). The
    // completeness of all 64 lives in `taxon_pins.rs`; the sealed
    // default stays in the kernel as refuse-unknown for future
    // enum additions.
    let world = {
        let mut world = World::new();
        world.found("origin").expect("founds");
        world
    };
    let state = world.committed();
    let step = Step {
        event: ClosureEvent::TaxonSealed { class: 1, clade: 1 },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(
        lawful(&state, &state, &step),
        Ok(()),
        "the final holdout is judged — coverage is total"
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

/// I1 the aperture seam — the convergence merge. A source leg debits the
/// world's total by EXACTLY its declared amount (value crossed out); a leg
/// that debits more than it declared refuses. This is the LOCAL half of
/// pair conservation; the cross-node half (far leg = near leg's inverse)
/// is the mesh opposition.
#[test]
fn the_aperture_leg_conserves_its_exact_declared_amount() {
    // The kernel check is unit-level: it judges prior/next against the
    // leg's declared amount, needing no THRESHOLD-1 crossing machinery.
    // `next` = the source world after a 3-iron debit (value crossed out).
    let world = two_orb_world();
    let prior = world.committed();
    let before = prior
        .orbs
        .iter()
        .map(|orb| orb.banked_amount(Resource::Iron))
        .fold(ratio(0, 1), |sum, held| sum + held);
    let mut next = world.committed();
    next.orbs[0].bank(Resource::Iron, ratio(-3, 1));
    // The honest source leg: the world's iron fell by exactly 3.
    let sent = |amount| Step {
        event: ClosureEvent::ApertureSent {
            deal: 7,
            from_orb: FIRST_ORB,
            resource: Resource::Iron,
            amount,
        },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(
        lawful(&prior, &next, &sent(ratio(3, 1))),
        Ok(()),
        "a leg debits exactly what it declares"
    );
    // A leg that CLAIMS to send 1 but the state debited 3: the world lost
    // more than the deed declared — phantom burn at the seam, refused,
    // lawful total (prior − 1) vs actual (prior − 3) carried whole.
    assert_eq!(
        lawful(&prior, &next, &sent(ratio(1, 1))),
        Err(Unlawful::ConservationBroken {
            resource: Resource::Iron,
            before: Box::new(before.clone() - ratio(1, 1)),
            after: Box::new(before - ratio(3, 1)),
        }),
        "the leg moved more than it declared — the seam refuses by measurement"
    );
}

/// I4/I5 the value law — a mint raises the world total by exactly its
/// declared amount (and is non-negative); a decay lowers it by exactly its
/// declared amount (and is non-negative). A "mint" that destroys value, or
/// a "decay" that creates it, is a mislabelled deed and refuses by sign.
#[test]
fn creation_and_decay_bound_the_value_law_by_declaration_and_sign() {
    let world = two_orb_world();
    let prior = world.committed();
    // I4 mint: the world's iron rises by exactly 4 — a lawful creation.
    let mut minted = world.committed();
    minted.orbs[0].bank(Resource::Iron, ratio(4, 1));
    let yield_step = |amount| Step {
        event: ClosureEvent::YieldCollected {
            orb_id: FIRST_ORB,
            resource: Resource::Iron,
            amount,
        },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(lawful(&prior, &minted, &yield_step(ratio(4, 1))), Ok(()), "a mint of exactly 4 admits");
    // A "mint" of -4 (a burn wearing a mint's name): the state HONESTLY
    // fell by 4 (so declaration==effect passes), then the sign refuses.
    let mut burn_as_mint = world.committed();
    burn_as_mint.orbs[0].bank(Resource::Iron, ratio(-4, 1));
    assert_eq!(
        lawful(&prior, &burn_as_mint, &yield_step(ratio(-4, 1))),
        Err(Unlawful::MintNotPositive { resource: Resource::Iron, amount: Box::new(ratio(-4, 1)) }),
        "value enters only by a non-negative mint"
    );
    // I5 decay — WITNESS TRUTH-UPDATE (U1f): this pin was written in
    // the sign-only era, when "the world's iron falls by exactly 2"
    // admitted. The bound has landed: this world has NO star, so the
    // sealed demurrage rule owes NOTHING — the same decay now refuses
    // with the bound carried whole. Value leaves only by the flat
    // law; an un-owed decay is theft wearing a decay's name.
    let mut decayed = world.committed();
    decayed.orbs[0].bank(Resource::Iron, ratio(-2, 1));
    let demurrage_step = |amount| Step {
        event: ClosureEvent::DemurrageSettled {
            orb_id: FIRST_ORB,
            resource: Resource::Iron,
            amount,
        },
        actor: SOVEREIGN,
        stamp: world.now(),
    };
    assert_eq!(
        lawful(&prior, &decayed, &demurrage_step(ratio(2, 1))),
        Err(Unlawful::DecayExceedsBound {
            bound: Box::new(ratio(0, 1)),
            claimed: Box::new(ratio(2, 1)),
        }),
        "the rule owes nothing here — the decay refuses, both sides whole"
    );
    // A "decay" of -2 (creating value): the state HONESTLY rose by 2 (a
    // decay of -2 is a delta of +2), so declaration==effect passes, then
    // the sign refuses.
    let mut mint_as_decay = world.committed();
    mint_as_decay.orbs[0].bank(Resource::Iron, ratio(2, 1));
    assert_eq!(
        lawful(&prior, &mint_as_decay, &demurrage_step(ratio(-2, 1))),
        Err(Unlawful::DecayNotPositive { resource: Resource::Iron, amount: Box::new(ratio(-2, 1)) }),
        "value leaves only by a non-negative decay"
    );
}

#[test]
fn the_version_is_pinned() {
    // The commitment binds this; an amendment bumps it. Pinned so a
    // silent change to the judge cannot pass unnoticed.
    // VERSION 2 — THE RULING BOX 1 AMENDMENT (operator, 2026-08-01):
    // mint_bound unsealed, f ladder-pinned 0·0·0·1·4·7 against the
    // #17 calibration (5898351). Version-1 chains verify forever
    // under the sealed-shut judge they were cut with.
    assert_eq!(VERSION, 2, "the kernel version is charter-pinned");
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
