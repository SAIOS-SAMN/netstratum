//! KERNEL invariant witness — conservation (CHARTER §11 rule 3, the
//! first invariant, red-first). A deed that only MOVES value may not
//! change the world's total of that value; an honest transfer admits,
//! a minting one refuses with the residual whole. The gate is pure —
//! it reads two states and a claimed step, touches nothing else.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ipsal::Scope;
use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{admits, Step, Refused, KERNEL_VERSION, Version, VERSION};

/// A world with two orbs and value banked on the first, ready to move.
/// The origin earns before it charters (the charted's founding
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
    // final holdout is gated (admits, prior == next, neutral). The
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
        admits(&state, &state, &step),
        Ok(()),
        "the final holdout is judged — coverage is total"
    );
}

/// I2 — a transfer (Collect scope) by an agent WITHOUT Collect refuses
/// NotPermitted; the same agent, once granted, is admitted; the sovereign
/// (all scopes by construction) always passes; a non-entity refuses
/// NoSuchEntity before any state is read. The kernel's scope_of(Collect)
/// for ResourceTransferred is ratified here against `cfe::world`.
#[test]
fn containment_gates_the_transfer_by_scope() {
    let mut world = two_orb_world();
    let haven = world.orbs()[1].orb_id;
    let agent = world.register_agent("hand").expect("registers");
    let prior = world.committed();
    world
        .transfer(cfe::dop::between(FIRST_ORB, haven), Resource::Iron, cfe::dop::positive(3, 1))
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
        admits(&prior, &next, &step(agent)),
        Err(Refused::NotPermitted { actor: agent, scope: Scope::Collect }),
        "authority is granted, never assumed"
    );
    // The sovereign holds every scope: admitted.
    assert_eq!(admits(&prior, &next, &step(SOVEREIGN)), Ok(()));
    // A non-entity: refused before any state read.
    assert_eq!(
        admits(&prior, &next, &step(999)),
        Err(Refused::NoSuchEntity { actor: 999 }),
        "identity precedes the world"
    );
    // Grant the agent Collect: now admitted.
    world.grant(agent, Scope::Collect).expect("grants");
    let granted_prior = world.committed();
    assert_eq!(admits(&granted_prior, &next, &step(agent)), Ok(()));
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
    assert_eq!(admits(&prior, &next, &step), Ok(()), "the authority deed is judged, not skipped");
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
    // The kernel check is unit-level: it gates prior/next against the
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
        admits(&prior, &next, &sent(ratio(3, 1))),
        Ok(()),
        "a leg debits exactly what it declares"
    );
    // A leg that CLAIMS to send 1 but the state debited 3: the world lost
    // more than the deed declared — phantom burn at the seam, refused,
    // lawful total (prior − 1) vs actual (prior − 3) carried whole.
    assert_eq!(
        admits(&prior, &next, &sent(ratio(1, 1))),
        Err(Refused::ConservationBroken {
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
    assert_eq!(admits(&prior, &minted, &yield_step(ratio(4, 1))), Ok(()), "a mint of exactly 4 admits");
    // A "mint" of -4 (a burn wearing a mint's name): the state HONESTLY
    // fell by 4 (so declaration==effect passes), then the sign refuses.
    let mut burn_as_mint = world.committed();
    burn_as_mint.orbs[0].bank(Resource::Iron, ratio(-4, 1));
    assert_eq!(
        admits(&prior, &burn_as_mint, &yield_step(ratio(-4, 1))),
        Err(Refused::MintNotPositive { resource: Resource::Iron, amount: Box::new(ratio(-4, 1)) }),
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
        admits(&prior, &decayed, &demurrage_step(ratio(2, 1))),
        Err(Refused::DecayExceedsBound {
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
        admits(&prior, &mint_as_decay, &demurrage_step(ratio(-2, 1))),
        Err(Refused::DecayNotPositive { resource: Resource::Iron, amount: Box::new(ratio(-2, 1)) }),
        "value leaves only by a non-negative decay"
    );
}

#[test]
fn the_version_is_pinned() {
    // The commitment binds this; an amendment bumps it. Pinned so a
    // silent change to the gate cannot pass unnoticed.
    // VERSION 2 — THE RULING BOX 1 AMENDMENT (operator, 2026-08-01):
    // mint_bound unsealed, f ladder-pinned 0·0·0·1·4·7 against the
    // #17 calibration (5898351). Version-1 chains verify forever
    // under the sealed-shut gate they were cut with.
    // VERSION 3 — THE CAPACITY LAW AMENDMENT (operator, 2026-08-01):
    // a DiscoveryMinted deed refuses unless the prior state reads
    // Entangled. BREAKING — version 2 admitted mints on Void, Stasis
    // and Unframed worlds. The introducing commit announced the bump
    // in its title and left the constant at 2; landed here.
    // VERSION 5 — TWO AMENDMENTS LANDED TOGETHER AT THE MERGE.
    // 4: the capacity law REMOVED (harness lane) — breaking in the
    //    loosening direction; a recycled identity carries no bonds, so
    //    a standing test read off relational structure refused the
    //    lawful recycled line by construction.
    // 5: D4, the revocation law (cognition lane) — a DiscoveryMinted
    //    deed refuses without a strictly positive capacity increment,
    //    over a RESOLVED rank reading.
    // Minted on separate branches for separate reasons; 4 was skipped
    // on main so the two could not collide, and both are kept.
    // x.y.z, major 0 — a PROTOTYPE gate (operator ruling 2026-08-04).
    // The five linear amendments become 0.1.0 .. 0.5.0, so the count
    // survives in `minor` and the leading 0 says nothing has promised
    // stability yet.
    //   minor  a LAW change — the gate admits or refuses what it did
    //          not before. Every amendment so far is one of these.
    //   patch  a change that cannot alter a verdict.
    // 0.5.0 carries BOTH of the day's amendments: the capacity law
    // removed (harness), and D4 the revocation law (cognition).
    // 0.5.1 — THE OPENED GROUND (2026-08-05): D = volume × (GROUND +
    //   circulation). PATCH, derived: every judge arm reads above(),
    //   which is volume-blind; the fold guard compares ranks. No
    //   verdict moved. (And this pin CAUGHT the bump landing without a
    //   test run — it sat red for a day because the bumping commit
    //   only built. The pin did its job; the process did not.)
    // 0.6.0 — THE MAGNETIC REACH (2026-08-06): a PlanetSited threading
    //   a ring beyond the star's field grade refuses RingBeyondReach.
    //   MINOR: the judge refuses what it admitted before.
    // 0.7.0 — THE APPRAISAL (2026-08-06): tag 98 covered; the gate
    // admits what was Unjudged and refuses AppraisalOutsideSurvey /
    // AppraisalMisvalued. Staleness deliberately gates NOTHING — it
    // triggers renegotiation; consent closes acceptance.
    // 0.8.0 — THE COUNTER-OFFER (2026-08-06): tag 99 covered — the
    //   renegotiate arm; the gate admits what was Unjudged. Staleness
    //   refuses only a counter's citation, at the emitter, where the
    //   chronicle lives.
    // 0.9.0 — CONVEY (2026-08-06): tag 100 covered with a full I1 arm —
    //   the postal deed; address, monetary cap and postage all verified
    //   at the gate by the emitter's own sealed rules. The estate chain
    //   is whole: survey → appraise → counter → convey.
    // 0.10.0 — THE MINTED LINE (2026-08-06): tag 101 covered — the
    //   engine mints its peoples; the receipt is a seed, everything
    //   derives, nothing declarable to misdeclare.
    // 0.11.0 — THE ATOMIC TOUCH (2026-08-06): tag 102 covered — the
    //   demurrage seam closed. One touch, one receipt: the judge reads
    //   the whole settlement against the pre-touch anchor, and refuses
    //   DecayRowDuplicated where a repeat would draw one window twice.
    //   MINOR: the gate admits the dual-resource settlement the
    //   per-row form structurally refused, and new touches emit the
    //   new tag. Old per-row chains verify forever under the sealed
    //   arm they were cut with.
    // 0.12.0 — FOLD-AWARE LINE SYNTHESIS (2026-08-06): tag 101's value
    //   widens — the receipt DECLARES the (class, clade) ceilings its
    //   draw ran under (the fold's at the mint stamp), so minted
    //   classes are drawable and every reader re-derives the exact
    //   draw from the receipt alone. MINOR by the schema discipline
    //   (the event's wire value grew); no verdict moves — the deed is
    //   banked-neutral and the old 32-byte form decodes sealed
    //   (ceilings: None -> the compiled derivation, forever).
    // 0.13.0 — THE CHARTERED PARAMETERS (2026-08-06): tag 103 covered
    //   — a balance placeholder chartered by a T0 line moves the live
    //   rule for its world; the band is a door (fourfold of the seed,
    //   anchored), the fold reads seed ∪ charters, the tick obeys.
    //   MINOR: a new tag enters the registry (banked-neutral,
    //   {I2, I3} complete it — the gate admits what was Unjudged).
    // ── THE NEXT-FREE RULE (adopted 2026-08-06, the tag registry's
    // discipline applied to versions): the version constant is a
    // SHARED REGISTER — two lanes bumping concurrently collide by
    // construction (measured: both lanes minted 0.13.0 independently;
    // resolved as a sequence, the collision named in both ledgers).
    // CLAIM THE NEXT MINOR HERE BEFORE BUMPING ANYWHERE.
    //   0.15.0 — claimed and landed by the d3-scope composition
    //            (the genesis exception, 54f85ea, their ledger).
    //   0.16.0 — claimed and landed here (ECO-1, this ledger).
    //   0.17.0 — claimed and landed here (ELEMENT-CHEMISTRY, below).
    //   0.18.0 — claimed and landed here (ZONE VOCABULARY, below).
    //   0.19.0 — claimed and landed here (TREATIES, below).
    //   — the next free minor is 20.
    // 0.14.0 — THE CORPUS PRICES (2026-08-06): the ratified carrier
    //   pricing lands with its measured table — a benchmark-confirmed
    //   corpus solve mints at W = scarcity × VisitedNodes₃/word_len
    //   (1243/34 standard, 283/1 blind). MINOR: the gate ADMITS what
    //   refused CarrierUnpriced. The forge stays priced until a live
    //   corpus submission path exists; ITS zeroing is the next
    //   carrier amendment.
    // 0.16.0 — THE ECOLOGICAL LOOP (2026-08-06, #40): tag 71 widens —
    //   the assay's receipt carries the vein's after-richness, and the
    //   deed settles wear (scaled by the draw against the spring's
    //   hash-derived natural flow) and recovery (the Regenerator's
    //   second arm, at its own working rate, capped at natural) in ONE
    //   receipt. The gate re-derives the declaration by CALLING the
    //   emitter's one home (`cfe::dop::vein_wear`) and refuses
    //   VeinWearMisdeclared off it; the vein book itself becomes
    //   judged state (VeinBookBroken — a prospect pushes, an assay
    //   moves its vein, everything else holds the book still). MINOR:
    //   the gate refuses what it admitted (a book left unmoved by an
    //   assay) and admits the widened form. The sealed 3-field form
    //   declares nothing, judges only the bound, and holds richness
    //   still forever.
    // 0.17.0 — ELEMENT CHEMISTRY (2026-08-06, #37): tag 104 covered —
    //   matter BONDS. The almanac's periodic table, complete and pinned
    //   but read by nothing, gets its first consumer: a set of banked
    //   elements whose lanes saturate compounds into a gem, each member
    //   spending its lane's valence and the gem carrying away exactly
    //   the sum (mass-exact by the saturation identity, so the recipe
    //   needs no number of its own). The gate re-derives the recipe AND
    //   the gem's identity from the receipt's members by calling the
    //   emitter's own law, and refuses MatterDoesNotBond /
    //   GemMisidentified off it. The element declaration widens from
    //   ONE move to MANY — a deed may now move several element keys at
    //   once, and repeated keys SUM rather than overwrite. MINOR: the
    //   gate admits what was Unjudged.
    // 0.18.0 — ZONE VOCABULARY (2026-08-07, #41): a world MINTS its own
    //   building kinds, and the gate learns to price one. `ZoneBuilt`
    //   widens by the additive pattern to declare the compiled BASE a
    //   build was priced against, because a minted kind's base lives in
    //   the fold and `admits` is handed committed state with no
    //   chronicle in it — the tag-101 precedent, where the receipt
    //   declares what the gate cannot re-derive. The raze reads the
    //   same base from committed `Zone` state, where it was fixed at
    //   build time and judged then, so salvage cannot outrun the build.
    //   MINOR: the gate prices a build it previously priced at the
    //   compiled table alone. Sealed chains carry no base, read as the
    //   kind's own, and judge exactly as they always did.
    // 0.19.0 — TREATIES (2026-08-07, #42): tags 105 and 106 covered —
    //   the political spectrum as one co-signed pairwise charter.
    //   Banked-neutral: a pact re-rules what a CROSSING costs and moves
    //   nothing by itself, so {I2 containment, I3 exactness} complete
    //   both deeds and the haul that later crosses is the conserved
    //   one. MINOR: the gate admits what was Unjudged.
    // 0.20.0 — OFFICES T4–T9 (2026-08-07, Wave-1): six economic
    //   stations (caravan·crucible·exchange·assay·vault·anchor) enter
    //   the closed office vocabulary and the tier ladder. StationSeated
    //   already covered; MINOR because serving-seat and tier gates now
    //   admit six offices they previously refused as unknown.
    // 0.21.0 — E-APT-1 (2026-08-07, Wave-2): ElementApertureSent /
    //   ElementApertureReceived (tags 108–109) enter covered() and
    //   element_move.
    // 0.22.0 — STAMP + E-ROUTE (2026-08-07): StampMinted / StampAperture*
    //   and ElementRouteChartered enter covered(); stamp conservation
    //   and crystal convert identity. Next free minor 23.
    //   — the next free minor is 23.
    assert_eq!(KERNEL_VERSION.to_string(), "0.22.0", "the gate is version-pinned");
    assert_eq!(VERSION, KERNEL_VERSION.code(), "the wire carries the packed form");
    // ORDER IS PRESERVED: a chain cut under the old linear versions
    // compares BELOW every packed one, which is true — an earlier gate
    // cut it.
    // The packed form orders above the linear era (1..5) — what a
    // peer comparing two commitments needs. Asserted against the
    // last linear version rather than a literal, so it stays a claim
    // about ORDER and cannot become constant-true.
    assert!(
        KERNEL_VERSION.code() > Version { major: 0, minor: 0, patch: 5 }.code(),
        "a packed version orders above the linear era it succeeded"
    );
}

#[test]
fn an_honest_transfer_is_admitted() {
    let mut world = two_orb_world();
    let haven = world.orbs()[1].orb_id;
    let prior = world.committed();
    world
        .transfer(cfe::dop::between(FIRST_ORB, haven), Resource::Iron, cfe::dop::positive(3, 1))
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
        admits(&prior, &next, &step),
        Ok(()),
        "value moved, total unchanged — the judge admits"
    );
}

#[test]
fn a_minting_transfer_refuses_with_the_residual_whole() {
    // The forger writes a real transfer shape but the `next` state
    // MINTS: the destination gains more iron than the source lost.
    // The world's total rises, and the gate refuses by measurement —
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
        .transfer(cfe::dop::between(FIRST_ORB, haven), Resource::Iron, cfe::dop::positive(3, 1))
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
        admits(&prior, &next, &step),
        Err(Refused::ConservationBroken {
            resource: Resource::Iron,
            before: Box::new(before.clone()),
            after: Box::new(after.clone()),
        }),
        "the mint broke conservation — before {before}, after {after}, the residual is theirs to read"
    );
    assert_eq!(after - before, ratio(2, 1), "the residual is exactly the minted 2 iron");
}
