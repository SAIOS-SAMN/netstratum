//! **ONT-4 — THE LEISURE MANIFOLD, the accounting witnessed.** Leisure is banked
//! per drive-ladder stratum, not collapsed to a scalar: the budget is the volume
//! (sum over strata), each stratum is a bounded floored-flow integral, and the
//! geometric `bank × d` recurrence that detonated (~beat 117) is replaced by FLAT
//! floored flows — so no denominator can explode. These pins live in `tests/`
//! (not the scanned `src` tree), so their fixture `ratio` literals are not counted
//! by the tuned-constant ratchet.

use cfe::ratio::{ratio, Ratio};
use conductor::civic::{settle_leisure_manifold, Drive, LeisureManifold};
use num_traits::ToPrimitive;

/// The flow grain the witnesses settle against (a bounded fixture, not the wire's
/// value — the algebra takes it as an argument).
const GRAIN: i64 = 64;

fn surplus(cognitive: Ratio, expansive: Ratio) -> Vec<(Drive, Ratio)> {
    vec![(Drive::Cognitive, cognitive), (Drive::Expansive, expansive)]
}

#[test]
fn the_budget_is_the_volume_the_sum_over_strata() {
    let m = settle_leisure_manifold(
        &LeisureManifold::default(),
        &surplus(ratio(3, 1), ratio(2, 1)),
        &[],
        &ratio(0, 1),
        GRAIN,
    );
    assert_eq!(m.of(Drive::Cognitive), ratio(3, 1));
    assert_eq!(m.of(Drive::Expansive), ratio(2, 1));
    assert_eq!(m.budget(), ratio(5, 1), "the budget is the sum over strata");
}

#[test]
fn dimensional_volume_is_preserved_not_collapsed() {
    // Two manifolds with the SAME budget but DIFFERENT distributions are
    // DISTINCT — the volume is kept, the manifold is not a 1-D shadow.
    let a = settle_leisure_manifold(
        &LeisureManifold::default(),
        &surplus(ratio(4, 1), ratio(1, 1)),
        &[],
        &ratio(0, 1),
        GRAIN,
    );
    let b = settle_leisure_manifold(
        &LeisureManifold::default(),
        &surplus(ratio(1, 1), ratio(4, 1)),
        &[],
        &ratio(0, 1),
        GRAIN,
    );
    assert_eq!(a.budget(), b.budget(), "same volume");
    assert_ne!(a, b, "but distinct manifolds — the strata are not collapsed");
}

#[test]
fn a_spend_draws_the_named_stratum_alone() {
    let seeded = settle_leisure_manifold(
        &LeisureManifold::default(),
        &surplus(ratio(5, 1), ratio(5, 1)),
        &[],
        &ratio(0, 1),
        GRAIN,
    );
    // Spend on cognition — only the Cognitive stratum falls; Expansive stands.
    let after = settle_leisure_manifold(
        &seeded,
        &[],
        &[(Drive::Cognitive, ratio(2, 1))],
        &ratio(0, 1),
        GRAIN,
    );
    assert_eq!(after.of(Drive::Cognitive), ratio(3, 1), "the mint drew its own stratum");
    assert_eq!(after.of(Drive::Expansive), ratio(5, 1), "the other stratum is untouched");
}

#[test]
fn flat_decay_is_bounded_not_geometric() {
    // The decay is a FIXED per-beat drain, not bank × d: a small stratum sheds
    // the same absolute amount a large one does, and floors at zero.
    let seeded = settle_leisure_manifold(
        &LeisureManifold::default(),
        &surplus(ratio(1, 2), ratio(100, 1)),
        &[],
        &ratio(0, 1),
        GRAIN,
    );
    let after = settle_leisure_manifold(&seeded, &[], &[], &ratio(1, 1), GRAIN);
    assert_eq!(after.of(Drive::Cognitive), ratio(0, 1), "a small stratum drains to zero (floored)");
    assert_eq!(after.of(Drive::Expansive), ratio(99, 1), "a large one sheds the same fixed amount");
}

#[test]
fn the_strata_stay_denominator_bounded_over_many_settles() {
    // THE ANTI-COLLAPSE PROPERTY. Flow the manifold for a long horizon with
    // odd-denominator surplus and a flat decay; because every flow is floored to
    // 1/GRAIN and NO stratum multiplies itself, the denominator never leaves the
    // grain — the ~beat-117 detonation is impossible by construction.
    let mut m = LeisureManifold::default();
    let mut max_denom: u64 = 1;
    for i in 0..1_000i64 {
        // A surplus whose raw denominator would compound without the floor.
        let s = surplus(ratio(i % 7 + 1, 3), ratio(i % 5 + 1, 6));
        m = settle_leisure_manifold(&m, &s, &[(Drive::Cognitive, ratio(1, 5))], &ratio(1, 8), GRAIN);
        for (_, bal) in &m.strata {
            max_denom = max_denom.max(bal.denom().to_u64().unwrap_or(u64::MAX));
        }
    }
    assert!(max_denom <= GRAIN as u64, "every stratum stays a 1/GRAIN multiple: {max_denom}");
}
