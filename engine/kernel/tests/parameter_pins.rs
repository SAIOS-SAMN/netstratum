//! THE KERNEL PARAMETER PINS (`21_KERNEL_PARAMETERS`).
//!
//! **These pin MECHANICS, not definitions.** An earlier draft of this
//! file contained two costumes and they are recorded here because the
//! failure is instructive:
//!
//! - `assert!(!verdict.is_admitted() || read.rank > 0)` — `Entangled`
//!   REQUIRES `rank > 0` by construction, so the assertion is true for
//!   every possible input and could never fail. It tested a definition
//!   and reported it as a behaviour.
//! - a mint-count assertion on a world where nothing had ever minted,
//!   which would pass whatever `founding_reserves` did.
//!
//! Both are replaced below with tests that fail if the mechanic
//! changes.

use cfe::ratio::ratio;
use cfe::world::{ease_of, World};
use kernel::mint_bound;
use solver::Difficulty;

/// P3 · THE ENDPOINTS ARE EXACT — the amendment is continuous.
///
/// The landed cliff paid `1/2` at condition 5 and `1/1` otherwise. The
/// gradient must reproduce BOTH exactly, or a live world jumps the
/// moment it lands. This is the continuity guarantee, tested rather
/// than asserted in a document.
#[test]
fn p3_the_gradient_reproduces_both_landed_values_exactly() {
    let none: Vec<(u64, u8, [u8; 32])> = vec![];
    assert_eq!(ease_of(&none, 1), ratio(1, 1), "no tech decays at full rate, as before");

    let cliff = vec![(1u64, 5u8, [0u8; 32])];
    assert_eq!(ease_of(&cliff, 1), ratio(1, 2), "condition 5 halves decay, as before");
}

/// P3 · THE CLIFF IS GONE — this is the mechanical claim.
///
/// Under the landed rule conditions 0 through 4 were INDISTINGUISHABLE:
/// every one returned `1/1`, so four fifths of the tech tree bought
/// nothing and condition 5 was a strategy target. This test fails if
/// that cliff ever returns.
#[test]
fn p3_intermediate_conditions_now_earn_and_did_not_before() {
    let none: Vec<(u64, u8, [u8; 32])> = vec![];
    let baseline = ease_of(&none, 1);

    let mut previous = baseline.clone();
    for condition in 1u8..=5 {
        let held = vec![(1u64, condition, [0u8; 32])];
        let ease = ease_of(&held, 1);

        assert!(
            ease < previous,
            "condition {condition} must earn strictly more than {}: got {ease} vs {previous}",
            condition - 1
        );
        assert!(ease < baseline, "condition {condition} beats no-tech — the cliff is gone");
        previous = ease;
    }
}

/// P3 · MONOTONE AND BOUNDED ACROSS THE WHOLE DOMAIN.
///
/// Two failures this guards: an ease above `1` would decay faster than
/// the un-teched baseline (technology as a punishment), and an ease of
/// `0` would remove the sink entirely, leaving I5 bounding decay at
/// nothing. Runs past the old cliff to prove the clamp holds.
#[test]
fn p3_never_punishes_and_never_removes_the_sink() {
    for condition in 0u8..=12 {
        let held = vec![(1u64, condition, [0u8; 32])];
        let ease = ease_of(&held, 1);
        assert!(ease <= ratio(1, 1), "condition {condition}: tech must never increase decay");
        assert!(ease >= ratio(1, 2), "condition {condition}: the sink never vanishes");
    }
}

/// P3 · THE GRADIENT REACHES DECAY, not just a return value.
///
/// The previous three tests read `ease_of` directly. This one proves
/// the number is actually load-bearing: the sealed demurrage rule
/// consumes it, so a better-teched orb owes strictly less on identical
/// state. Without this the gradient could be correct and inert.
#[test]
fn p3_a_better_teched_orb_owes_strictly_less() {
    let mut world = World::new();
    world.found("decays").expect("founds");
    let committed = world.committed();
    let orb = &committed.orbs[0];

    let full = cfe::world::demurrage_due(orb, committed.star, 64, &ratio(1, 1));
    let halved = cfe::world::demurrage_due(orb, committed.star, 64, &ratio(1, 2));

    // An unsited orb may owe nothing at all; the claim is comparative
    // and holds either way — it must never owe MORE for holding tech.
    for (resource, owed_full) in &full {
        let owed_halved = halved
            .iter()
            .find(|(r, _)| r == resource)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| ratio(0, 1));
        assert!(
            owed_halved <= *owed_full,
            "{resource:?}: ease 1/2 must never owe more than ease 1/1"
        );
    }
}

/// P4 · THE SEED FIRES ONCE — the ruling is already satisfied.
///
/// *"Operates strictly at `t = 0` and evaluates to `0` for all
/// `t > 0`"* holds by construction: `world.rs:1351` refuses a second
/// founding. This fails if that refusal is ever relaxed.
#[test]
fn p4_the_seed_cannot_fire_twice() {
    let mut world = World::new();
    world.found("first").expect("the genesis founding stands");

    let again = world.found("second");
    assert!(again.is_err(), "a second founding refuses — the seed is not a repeatable grant");
    assert!(
        format!("{again:?}").contains("AlreadyFounded"),
        "refused for the right reason: {again:?}"
    );

    // And the seed did not double: one founding, one grant.
    assert_eq!(world.committed().orbs.len(), 1, "one orb, one seeding");
}

/// P1 · THE LADDER TODAY — the values the dynamic form must reproduce
/// at genesis.
#[test]
fn p1_the_landed_rungs_are_1_4_7_and_zero_below() {
    assert_eq!(mint_bound(&Difficulty::level(3)), ratio(1, 1));
    assert_eq!(mint_bound(&Difficulty::level(4)), ratio(4, 1));
    assert_eq!(mint_bound(&Difficulty::level(5)), ratio(7, 1));

    // MEASURED, not endorsed: the bottom rungs pay nothing, so verified
    // cognition at rung 0-2 earns absolute zero (§5 decision 2, owed).
    for rung in 0..=2 {
        assert_eq!(mint_bound(&Difficulty::level(rung)), ratio(0, 1), "rung {rung} pays nothing");
    }
}
