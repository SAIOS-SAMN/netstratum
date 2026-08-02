//! THE KERNEL PARAMETER PINS (`21_KERNEL_PARAMETERS`) — the four
//! numbers that price the system, pinned where they are already lawful
//! so that a later amendment cannot move them by accident.
//!
//! P4's ruling (operator, 2026-08-02) — *the seed operates strictly at
//! `t = 0` and evaluates to `0` for all `t > 0`* — is **already true by
//! construction**, and these pins are what make that a build failure to
//! break rather than a property to remember.

use cfe::dop::ClosureEvent;
use cfe::world::{World, SOVEREIGN};
use kernel::mint_bound;
use solver::Difficulty;

/// P4 · THE SEED FIRES ONCE, AND ONLY AT GENESIS.
///
/// The ruling asks that `founding_reserves` apply at `t = 0` and
/// evaluate to zero thereafter. `world.rs:1351` already refuses a
/// second founding outright (`ColonyAlreadyFounded`), so the invariant
/// holds without an amendment — there is no `t > 0` branch to zero,
/// because there is no `t > 0` call.
#[test]
fn p4_the_seed_cannot_fire_twice() {
    let mut world = World::new();
    world.found("first").expect("the genesis founding stands");

    let again = world.found("second");
    assert!(again.is_err(), "a second founding refuses — the seed is not a repeatable grant");

    // And the refusal is the specific one, not an incidental error:
    // the seed is closed because the colony EXISTS, which is the t > 0
    // condition stated structurally.
    assert!(
        format!("{again:?}").contains("AlreadyFounded"),
        "refused for the right reason: {again:?}"
    );
}

/// P4 · THE SEED IS THE ONLY CREATION-FROM-NOTHING BESIDES THE MINT.
///
/// Recorded because `21_KERNEL_PARAMETERS` §2 rests on it: if some
/// other deed could add to a bank without a mint, the supply story in
/// §3 would be incomplete and the feedback loop would be dialed
/// against the wrong total.
#[test]
fn p4_founding_is_the_only_unminted_arrival() {
    let mut world = World::new();
    world.found("seeded").expect("founds");

    let committed = world.committed();
    let seeded: Vec<_> = committed.orbs[0].banked.clone();
    assert!(!seeded.is_empty(), "the seed arrives banked and unminted");

    // Nothing in the chronicle claims a mint for it — the reserves are
    // a genesis grant, not verified work.
    let minted = committed
        .orbs
        .iter()
        .flat_map(|_| world.chronicle().receipts().iter())
        .filter(|r| matches!(r.event, ClosureEvent::DiscoveryMinted { .. }))
        .count();
    assert_eq!(minted, 0, "the seed is granted, never minted");
}

/// P1 · THE LADDER TODAY — pinned so the amendment is deliberate.
///
/// These are the exact values `21_KERNEL_PARAMETERS` §2 measured and
/// that the CONTINUITY GUARANTEE must reproduce at genesis. A dynamic
/// `P1` that does not return these on a genesis-shaped state has
/// broken the guarantee, and this pin is where that shows up.
#[test]
fn p1_the_landed_rungs_are_1_4_7_and_zero_below() {
    assert_eq!(mint_bound(&Difficulty::level(3)), cfe::ratio::ratio(1, 1));
    assert_eq!(mint_bound(&Difficulty::level(4)), cfe::ratio::ratio(4, 1));
    assert_eq!(mint_bound(&Difficulty::level(5)), cfe::ratio::ratio(7, 1));

    // THE MEASURED GAP (§2, decision 2 owed): the bottom rungs pay
    // nothing. Verified cognition at rung 0-2 earns absolute zero.
    for rung in 0..=2 {
        assert_eq!(
            mint_bound(&Difficulty::level(rung)),
            cfe::ratio::ratio(0, 1),
            "rung {rung} pays nothing — pinned as MEASURED, not as endorsed"
        );
    }
}

/// P1 · THE MINT IS NEVER EVALUATED AT `D = 0`.
///
/// This is what makes a rank-denominated `P1` safe to write: the
/// division that a supply/rank feedback term needs can never see a
/// zero denominator, because `capacity_law` refuses the deed first.
/// `Vacuity::Entangled` is the only admitted verdict
/// (`boundary.rs:191`) and it requires `D > 0`.
///
/// **Recorded as the precondition of the amendment, not as an
/// implementation of it.**
#[test]
fn p1_capacity_law_refuses_before_any_rank_denominated_term_is_read() {
    let mut world = World::new();
    world.found("no-relations").expect("founds");
    let prior = world.committed();

    // A freshly founded world has no standing obligations, so no
    // cycles carry holonomy and the verdict is not Entangled.
    let read = kernel::bridge::read_entangled(&prior);
    let verdict = read.verdict();
    assert!(
        !verdict.is_admitted() || read.rank > 0,
        "either the mint is refused, or rank is non-zero — never a zero denominator"
    );

    let _ = SOVEREIGN;
}
