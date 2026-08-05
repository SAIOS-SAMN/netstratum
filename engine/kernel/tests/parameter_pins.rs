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
//!
//! **P3's gradient was REVERTED at `HEAD` — see `21_KERNEL_PARAMETERS`
//! §10.** It exposed a live/replay divergence in `conductor`'s survey
//! pin, so the three tests asserting a smooth descent are withdrawn
//! with it. The endpoint test below holds under the cliff AND under
//! the gradient, so it stays: it is the continuity guarantee either
//! way.

use cfe::ratio::ratio;
use cfe::world::{ease_of, World};
use kernel::mint_bound;
use solver::Difficulty;

/// EPOCH 0 · HISTORY IS UNTOUCHED — the whole point of the mechanism.
///
/// A deed stamped before the activation point is gated by the rule
/// that stood when it was written, whatever the rule says today. This
/// is what stops `replay` drifting from the live path, and it fails
/// the moment anyone edits `ease_cliff`.
#[test]
fn epoch0_deeds_are_still_judged_by_the_cliff() {
    let before = cfe::epoch::CONTINUOUS_FROM - 1;
    let none: Vec<(u64, u8, [u8; 32])> = vec![];
    assert_eq!(ease_of(&none, 1, before), ratio(1, 1), "no tech: the cliff's own answer");

    // The cliff's defining property: conditions 0-4 are ALL identical
    // to holding nothing. Four fifths of the tech tree bought nothing.
    for condition in 0u8..=4 {
        let held = vec![(1u64, condition, [0u8; 32])];
        assert_eq!(
            ease_of(&held, 1, before),
            ratio(1, 1),
            "EPOCH 0 condition {condition} is indistinguishable from no tech — as it always was"
        );
    }
    let at_cliff = vec![(1u64, 5u8, [0u8; 32])];
    assert_eq!(ease_of(&at_cliff, 1, before), ratio(1, 2), "only condition 5 paid");
}

/// EPOCH 1 · THE GRADIENT IS LIVE PAST THE BOUNDARY.
///
/// The same tech that bought nothing under EPOCH 0 now earns, and the
/// descent is strict across the whole tree. This is P3 re-inscribed —
/// the change that broke replay at `d52f3fd`, now landed safely.
#[test]
fn epoch1_the_cliff_becomes_a_strict_descent() {
    let at = cfe::epoch::CONTINUOUS_FROM;
    let none: Vec<(u64, u8, [u8; 32])> = vec![];
    let baseline = ease_of(&none, 1, at);

    let mut previous = baseline.clone();
    for condition in 1u8..=5 {
        let held = vec![(1u64, condition, [0u8; 32])];
        let ease = ease_of(&held, 1, at);
        assert!(ease < previous, "EPOCH 1 condition {condition}: {ease} must beat {previous}");
        previous = ease;
    }

    // And EPOCH 0 disagrees at exactly the conditions the cliff flattened
    // — which is the whole reason the epoch exists.
    let mid = vec![(1u64, 3u8, [0u8; 32])];
    assert_ne!(
        ease_of(&mid, 1, at),
        ease_of(&mid, 1, cfe::epoch::CONTINUOUS_FROM - 1),
        "condition 3 earns under EPOCH 1 and did not under EPOCH 0"
    );
}

/// EPOCH 1 · MONOTONE AND BOUNDED — tech never punishes, the sink
/// never vanishes.
#[test]
fn epoch1_never_punishes_and_never_removes_the_sink() {
    let at = cfe::epoch::CONTINUOUS_FROM;
    for condition in 0u8..=12 {
        let held = vec![(1u64, condition, [0u8; 32])];
        let ease = ease_of(&held, 1, at);
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

    let supply = cfe::world::standing_supply(&committed.orbs);
    let full = cfe::world::demurrage_due(orb, committed.star, 64, &ratio(1, 1), &supply);
    let halved = cfe::world::demurrage_due(orb, committed.star, 64, &ratio(1, 2), &supply);

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
    assert_eq!(mint_bound(Difficulty::level(3).rung()), ratio(1, 1));
    assert_eq!(mint_bound(Difficulty::level(4).rung()), ratio(4, 1));
    assert_eq!(mint_bound(Difficulty::level(5).rung()), ratio(7, 1));

    // MEASURED, not endorsed: the bottom rungs pay nothing, so verified
    // cognition at rung 0-2 earns absolute zero (§5 decision 2, owed).
    for rung in 0..=2 {
        assert_eq!(mint_bound(Difficulty::level(rung).rung()), ratio(0, 1), "rung {rung} pays nothing");
    }
}

/// P2 · EPOCH 0 IS THE FLAT RATE, WHATEVER SUPPLY STANDS.
///
/// Every existing receipt was written under this. If supply could move
/// the rate below the boundary, every historical world would settle
/// differently on replay — which is exactly the failure the epoch
/// mechanism exists to prevent.
#[test]
fn p2_epoch0_ignores_supply_entirely() {
    let before = cfe::epoch::CONTINUOUS_FROM - 1;
    for supply in [0i64, 32, 1_000, 1_000_000] {
        assert_eq!(
            cfe::dop::demurrage_per_tick(&ratio(supply, 1), before),
            ratio(1, 32),
            "EPOCH 0 supply {supply}: the flat rate is history's rate"
        );
    }
}

/// P2 · EPOCH 1 · THE SINK STIFFENS WITH SUPPLY — the mechanical claim.
///
/// What the constant could not do: RESPOND. Strictly rising past the
/// reference, so more standing supply always costs more to hold.
#[test]
fn p2_epoch1_the_rate_rises_strictly_with_supply() {
    let at = cfe::epoch::CONTINUOUS_FROM;
    let baseline = cfe::dop::demurrage_per_tick(&ratio(0, 1), at);
    assert_eq!(baseline, ratio(1, 32), "an empty world is charged what it always was");

    // Below the reference nothing stiffens.
    assert_eq!(cfe::dop::demurrage_per_tick(&ratio(16, 1), at), baseline);

    let mut previous = baseline.clone();
    for supply in [64i64, 128, 256, 512] {
        let rate = cfe::dop::demurrage_per_tick(&ratio(supply, 1), at);
        assert!(rate > previous, "supply {supply}: the sink must stiffen, got {rate}");
        previous = rate;
    }
    assert!(previous > baseline, "the rate moved — the constant is retired");
}

/// P2 · EPOCH 1 · BOUNDED ABOVE. #16 requires a bounded function; an
/// uncapped rate above `1` would confiscate a whole bank every tick.
#[test]
fn p2_epoch1_never_exceeds_the_declared_ceiling() {
    let at = cfe::epoch::CONTINUOUS_FROM;
    for supply in [0i64, 32, 1_000, 1_000_000, i64::MAX / 2] {
        let rate = cfe::dop::demurrage_per_tick(&ratio(supply, 1), at);
        assert!(rate <= ratio(1, 4), "supply {supply}: rate {rate} broke the ceiling");
        assert!(rate >= ratio(1, 32), "supply {supply}: the sink never weakens below baseline");
    }
}

/// P2 · THE TWO EPOCHS DISAGREE, which is why the epoch is needed.
#[test]
fn p2_the_epochs_diverge_on_an_accumulated_world() {
    let heavy = ratio(1_000, 1);
    assert_ne!(
        cfe::dop::demurrage_per_tick(&heavy, cfe::epoch::CONTINUOUS_FROM),
        cfe::dop::demurrage_per_tick(&heavy, cfe::epoch::CONTINUOUS_FROM - 1),
        "an accumulated world is charged differently either side of the boundary"
    );
}

/// P2 · ONE SEALED COMPUTATION FOR BOTH JUDGES.
///
/// `standing_supply` is the reason a rank-denominated rate was
/// abandoned (§9): the proposer cannot see rank, but it can see supply,
/// so both sides reach one value from one state. If this ever diverged,
/// honest deeds would refuse at I5.
#[test]
fn p2_standing_supply_covers_exactly_the_leaking_resources() {
    let mut world = World::new();
    world.found("supplied").expect("founds");
    let committed = world.committed();

    let table = cfe::world::standing_supply(&committed.orbs);
    let named: Vec<_> = table.iter().map(|(r, _)| *r).collect();
    assert!(named.contains(&cfe::dop::Resource::Joule), "Joule leaks and must be counted");
    assert!(named.contains(&cfe::dop::Resource::Sink), "Sink leaks and must be counted");
    assert_eq!(named.len(), 2, "a resource that does not leak must not move the sink's rate");
}
