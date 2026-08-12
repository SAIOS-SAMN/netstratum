//! **THE REGROWTH CURVE (task #28, ruled B — rational sigmoid on a grain).**
//!
//! The witnesses hold the ruled shape: a stock recovers toward capacity
//! along a logistic S-curve — slow near zero, fastest at mid-stock,
//! saturating at K and never past it — and the exact-rational surrogate
//! keeps every denominator bounded, unlike the transcendental it stands
//! in for.

use cfe::almanac::{grained_logistic, REGROWTH_GRAIN};
use cfe::ratio::{ratio, Ratio};

/// A middling rate for the fixtures — half a grain of gap closes in one
/// grain of time.
fn r() -> Ratio {
    ratio(1, 1)
}

#[test]
fn at_zero_elapsed_the_stock_is_unchanged() {
    let k = ratio(1000, 1);
    for p0 in [ratio(1, 1), ratio(250, 1), ratio(500, 1), ratio(999, 1)] {
        assert_eq!(grained_logistic(&p0, &k, &r(), 0), p0, "Δt = 0 moves nothing");
    }
}

#[test]
fn growth_saturates_at_capacity_and_never_overshoots() {
    let k = ratio(1000, 1);
    let p0 = ratio(10, 1);
    let mut last = p0.clone();
    // Over a very long span the stock approaches K monotonically...
    for dt in [1u64, 10, 100, 1_000, 100_000, 10_000_000] {
        let p = grained_logistic(&p0, &k, &r(), dt);
        assert!(p >= last, "regrowth is monotone in time");
        assert!(p <= k, "and never exceeds carrying capacity: {p} vs {k}");
        last = p;
    }
    // ...and a stock already at or above K holds at K.
    assert_eq!(grained_logistic(&k, &k, &r(), 100), k, "at capacity, it holds");
    assert_eq!(
        grained_logistic(&ratio(2000, 1), &k, &r(), 100),
        k,
        "over capacity settles to K, never above"
    );
}

#[test]
fn a_dead_stock_with_no_seed_stays_dead() {
    // Regrowth needs a living remnant — a resource extracted to exactly
    // nothing must be RESEEDED, never spontaneously generated.
    let k = ratio(1000, 1);
    assert_eq!(grained_logistic(&ratio(0, 1), &k, &r(), 1_000_000), ratio(0, 1));
    // But a near-zero remnant DOES climb back (slowly) — the Locust's
    // "dead" world is only mostly dead.
    let ember = grained_logistic(&ratio(1, 1), &k, &r(), 1_000_000);
    assert!(ember > ratio(1, 1), "a living ember recovers: {ember}");
}

#[test]
fn the_curve_is_an_s_the_middle_grows_fastest() {
    // THE LOGISTIC SIGNATURE: over one fixed window, the increment is
    // largest from mid-stock (~K/2) and smaller from near-zero and from
    // near-capacity — dP ∝ P·(K−P). This is what makes collapse
    // expensive to recover from and what a flat law would lose.
    let k = ratio(1000, 1);
    let dt = REGROWTH_GRAIN as u64; // one half-recovery window
    let gain = |p0: Ratio| grained_logistic(&p0, &k, &r(), dt) - p0;

    let from_low = gain(ratio(10, 1)); // near zero
    let from_mid = gain(ratio(500, 1)); // K/2
    let from_high = gain(ratio(990, 1)); // near K

    assert!(from_mid > from_low, "mid-stock grows faster than a near-dead one: {from_mid} vs {from_low}");
    assert!(from_mid > from_high, "and faster than a near-full one: {from_mid} vs {from_high}");
    assert!(from_low > ratio(0, 1), "the ember still climbs");
    assert!(from_high > ratio(0, 1), "and the near-full still creeps");
}

#[test]
fn the_denominator_cannot_run_away_however_long_the_wait() {
    // THE WHOLE POINT OF THE GRAIN. The transcendental it replaces would
    // blow the denominator without bound; this is ONE rational, so its
    // figure stays holdable no matter how many beats elapse.
    let k = ratio(1000, 1);
    let p0 = ratio(7, 1);
    let mut widest = 0usize;
    for dt in [1u64, 100, 10_000, 1_000_000, 100_000_000, 1_000_000_000] {
        let p = grained_logistic(&p0, &k, &r(), dt);
        widest = widest.max(p.numer().to_string().len() + p.denom().to_string().len());
    }
    assert!(widest < 40, "the figure stays holdable across any wait: {widest} digits");
}
