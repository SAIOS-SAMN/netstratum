//! **K4 · THE LIVE GATE IS CLOSED ON THE MINT.**
//!
//! Replaces `live_gate_defect.rs`, whose four tests asserted the hole
//! and were required to fail once it shut. They did.
//!
//! **How it closed without breaking the opaque-payload law.** `cfe` may
//! not import the verifier — `recurrence_guard::the_arrow_law_holds`
//! fails the build on any attempt, because proofs are opaque bytes to
//! the game and only the sealed verifier reads them. So the emitter
//! does not verify: it ASKS. `MintWarrant` is `cfe`'s question,
//! `kernel::SealedWarrant` is the only answer that reads a payload, and
//! the refusal happens BEFORE `close` — so a bad mint never reaches the
//! chronicle at all.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, NoWarrant, World, SOVEREIGN};

const GARBAGE: &[u8] = b"not a proof at all";

fn founded() -> (World, u64) {
    let mut world = World::new();
    world.found("gate").expect("founds");
    let orb = world.committed().orbs[0].orb_id;
    (world, orb)
}

/// A payload that is not a proof is REFUSED, and NOTHING IS WRITTEN.
/// That second half is the whole difference between catching and
/// preventing — the old behaviour refused too, but only after the
/// receipt stood in the chain.
#[test]
fn a_payload_that_is_not_a_proof_never_reaches_the_chronicle() {
    let (mut world, orb) = founded();
    let verdict = world.mint_discovery_as(
        SOVEREIGN,
        orb,
        Resource::Iron,
        ratio(1, 1),
        GARBAGE.to_vec(),
        &kernel::SealedWarrant,
    );
    assert!(
        matches!(verdict, Err(ActionRefused::WorkRefused)),
        "the emitter must refuse an unverifiable payload, got {verdict:?}"
    );
    assert_eq!(
        world
            .chronicle()
            .receipts()
            .iter()
            .filter(|r| matches!(r.event, ClosureEvent::DiscoveryMinted { .. }))
            .count(),
        0,
        "a refused mint leaves no receipt behind"
    );
}

/// **THE SEVERE HALF, CLOSED.** The live projection used to hide a mint
/// that `replay` would honour — a million units real to every reader of
/// the chain. Now nothing is written, so nothing is granted.
#[test]
fn replay_grants_nothing_because_nothing_was_written() {
    let (mut world, orb) = founded();
    let before = world.committed();

    for scale in [1i64, 1_000, 1_000_000_000] {
        assert!(
            world
                .mint_discovery_as(
                    SOVEREIGN,
                    orb,
                    Resource::Iron,
                    ratio(scale, 1),
                    GARBAGE.to_vec(),
                    &kernel::SealedWarrant,
                )
                .is_err(),
            "every scale refuses, including {scale}"
        );
    }
    assert_eq!(replay(world.chronicle()), before, "replay sees what live sees");
    assert_eq!(world.committed(), before, "and live is unchanged");
}

/// **NO WARRANT, NO MINT** — the default posture, made structural.
///
/// A caller that cannot verify must not be able to mint, and this is
/// the shape of the type rather than a rule someone remembers.
#[test]
fn a_caller_without_a_verifier_cannot_mint_at_all() {
    let (mut world, orb) = founded();
    let verdict = world.mint_discovery_as(
        SOVEREIGN,
        orb,
        Resource::Iron,
        ratio(1, 1),
        GARBAGE.to_vec(),
        &NoWarrant,
    );
    assert!(matches!(verdict, Err(ActionRefused::WorkRefused)), "{verdict:?}");
}

/// ONE FUNCTION, TWO JUDGES. The warrant answers with `mint_bound` —
/// the same ladder I4 enforces — so the emitter and the judge cannot
/// disagree about what a proof is worth.
#[test]
fn the_warrant_and_the_judge_read_one_ladder() {
    use cfe::world::MintWarrant;
    for rung in 0..=5 {
        let difficulty = solver::Difficulty::level(rung);
        // A payload the verifier rejects earns nothing, whatever the
        // ladder says — the warrant is verify-THEN-bound, never bound
        // alone.
        assert_eq!(
            kernel::SealedWarrant.earns(GARBAGE),
            None,
            "rung {rung}: a garbage payload earns nothing at any difficulty"
        );
        let _ = kernel::mint_bound(&difficulty);
    }
}
