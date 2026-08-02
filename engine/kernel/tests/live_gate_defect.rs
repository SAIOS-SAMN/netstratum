//! **K4 · THE LIVE GATE IS OPEN, MEASURED.**
//!
//! `19_KERNEL` §7 records the gate as the last step and the task note
//! called it *"caught, but not prevented"*. This file measures what
//! that actually costs on the mint — the most consequential deed in the
//! system — and the answer is worse than the note implied.
//!
//! **These tests assert the DEFECT.** They pass today and MUST FAIL
//! when K4 lands; that is the forcing function. Each carries the
//! assertion that should replace it.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::world::{replay, World, SOVEREIGN};
use kernel::{lawful, Step};

const GARBAGE: &[u8] = b"not a proof at all";
const ABSURD: i64 = 1_000_000;

fn minted_world() -> (World, cfe::world::Committed) {
    let mut world = World::new();
    world.found("gate").expect("founds");
    let before = world.committed();
    let orb = before.orbs[0].orb_id;
    world
        .mint_discovery_as(SOVEREIGN, orb, Resource::Iron, ratio(ABSURD, 1), GARBAGE.to_vec())
        .expect("THE DEFECT: the emitter admits a mint against a garbage payload");
    (world, before)
}

/// DEFECT 1 · THE EMITTER NEVER VERIFIES THE WORK.
///
/// `mint_discovery_as` (`world.rs:2193`) checks scope, sign and orb
/// existence, then closes. It does not call `verify_work`, so any byte
/// string is accepted as a proof of cognition — and the amount is
/// unchecked against `mint_bound`, so the figure is arbitrary.
///
/// **WHEN K4 LANDS:** this must become
/// `assert!(world.mint_discovery_as(..).is_err())`.
#[test]
fn defect_the_emitter_mints_against_a_payload_that_is_not_a_proof() {
    let (_world, _before) = minted_world();
    // Reaching here IS the defect: the call above succeeded.
}

/// DEFECT 2 · THE KERNEL REFUSES IT — AFTERWARDS.
///
/// The judgment is correct and it is too late: the receipt is already
/// in the chronicle, which is the durable artifact. Judgment that runs
/// after the write catches, it does not prevent.
#[test]
fn defect_the_kernel_refuses_a_deed_the_chronicle_already_holds() {
    let (world, before) = minted_world();
    let orb = before.orbs[0].orb_id;

    let step = Step {
        event: ClosureEvent::DiscoveryMinted {
            orb_id: orb,
            resource: Resource::Iron,
            amount: ratio(ABSURD, 1),
            work: GARBAGE.to_vec(),
        },
        actor: SOVEREIGN,
        stamp: 1,
    };
    assert!(lawful(&before, &world.committed(), &step).is_err(), "the kernel does refuse it");

    let written = world
        .chronicle()
        .receipts()
        .iter()
        .any(|r| matches!(r.event, ClosureEvent::DiscoveryMinted { .. }));
    assert!(written, "and the refused deed is ALREADY in the chronicle — caught, not prevented");
}

/// DEFECT 3 · THE VALUE IS REAL ON REPLAY. **This is the severe one.**
///
/// The live projection does not apply the mint, so `committed()` looks
/// untouched and the hole is invisible from the live path. `replay`
/// DOES apply it (`world.rs:4337`), and the chronicle is what survives
/// a restart, a resume, or any other node reading the chain.
///
/// So the million units are not hypothetical: they exist for every
/// reader that replays. Live and replay disagree, which is #30's
/// duplication with a value attached.
#[test]
fn defect_replay_grants_the_million_the_live_path_hides() {
    let (world, before) = minted_world();
    let live = world.committed();
    let replayed = replay(world.chronicle());

    let iron_of = |state: &cfe::world::Committed| {
        state.orbs[0]
            .banked
            .iter()
            .find(|(r, _)| *r == Resource::Iron)
            .map(|(_, held)| held.clone())
            .unwrap_or_else(|| ratio(0, 1))
    };

    let seeded = iron_of(&before);
    assert_eq!(iron_of(&live), seeded, "the LIVE path shows nothing minted");
    assert_eq!(
        iron_of(&replayed),
        seeded + ratio(ABSURD, 1),
        "REPLAY grants the full unverified mint"
    );
    assert_ne!(live, replayed, "live and replay disagree — the chronicle is what persists");
}

/// The same hole with the amount unbounded, to show the figure is not
/// incidental: nothing relates the minted amount to any work at all.
#[test]
fn defect_the_minted_amount_is_related_to_no_work_whatsoever() {
    let mut world = World::new();
    world.found("unbounded").expect("founds");
    let orb = world.committed().orbs[0].orb_id;

    for scale in [1i64, 1_000, 1_000_000_000] {
        assert!(
            world
                .mint_discovery_as(SOVEREIGN, orb, Resource::Iron, ratio(scale, 1), GARBAGE.to_vec())
                .is_ok(),
            "any amount is admitted, including {scale}"
        );
    }
    let replayed = replay(world.chronicle());
    let held = replayed.orbs[0]
        .banked
        .iter()
        .find(|(r, _)| *r == Resource::Iron)
        .map(|(_, h)| h.clone())
        .expect("iron stands");
    assert!(held > ratio(1_000_000_000, 1), "and they accumulate on replay: {held}");
}
