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

// ── K and r — the derivations that feed the curve ──

use cfe::almanac::{carrying_capacity, habitability, intrinsic_rate, ThermalRegion};

#[test]
fn the_rate_is_the_natural_biomass_rate_not_a_dial() {
    // r is the Habitat's own yield — life regrows at the rate life grows.
    assert_eq!(intrinsic_rate(), cfe::dop::ZoneKind::Habitat.yield_per_tick().1);
    assert!(intrinsic_rate() > ratio(0, 1), "a positive recovery rate");
}

#[test]
fn habitability_peaks_at_mesic_and_tapers_to_the_extremes() {
    // The ordering IS the law: nearer the life band is more habitable,
    // and the extremes bear nothing.
    let mesic = habitability(ThermalRegion::Mesic);
    let fringe = habitability(ThermalRegion::Sublime);
    let marginal = habitability(ThermalRegion::Cryogenic);
    assert_eq!(mesic, ratio(1, 1), "the life band is fullest");
    assert!(mesic > fringe, "the temperate fringe carries less");
    assert!(fringe > marginal, "the marginal band less still");
    assert_eq!(habitability(ThermalRegion::Torrid), fringe, "symmetric about mesic");
    assert_eq!(habitability(ThermalRegion::HyperThermal), marginal, "symmetric about mesic");
    for barren in [ThermalRegion::Quenched, ThermalRegion::Incandescent, ThermalRegion::Beyond] {
        assert_eq!(habitability(barren), ratio(0, 1), "the extremes are barren");
    }
}

#[test]
fn a_lush_world_carries_more_than_a_barren_one_and_a_rock_carries_nothing() {
    // K rises with the living footprint and with the place's
    // habitability — a mesic world with many habitats holds a large K.
    let lush = carrying_capacity(8, ThermalRegion::Mesic);
    let cold = carrying_capacity(8, ThermalRegion::Cryogenic);
    let rock = carrying_capacity(8, ThermalRegion::Incandescent);
    assert!(lush > cold, "the same zones hold more life in the mesic band");
    assert_eq!(rock, ratio(0, 1), "a scorched rock carries no biomass, whatever it builds");
    // More living zones, more capacity — the footprint scales it.
    assert!(
        carrying_capacity(16, ThermalRegion::Mesic) > lush,
        "a wider ecological footprint holds more"
    );
    // A world with no living zones has no natural biomass ceiling to speak of.
    assert_eq!(carrying_capacity(0, ThermalRegion::Mesic), ratio(0, 1));
}

#[test]
fn the_derivations_feed_the_curve_and_a_collapsed_lush_world_climbs_back() {
    // The pieces compose: a mesic world stripped to an ember recovers
    // toward its (large) K along the S-curve, slowly at first.
    let k = carrying_capacity(10, ThermalRegion::Mesic);
    let r = intrinsic_rate();
    let ember = ratio(1, 10);
    let early = cfe::almanac::grained_logistic(&ember, &k, &r, 10);
    let late = cfe::almanac::grained_logistic(&ember, &k, &r, 10_000);
    assert!(early > ember && early < k, "it has begun to climb");
    assert!(late > early, "and keeps climbing toward its lush ceiling");
    assert!(late <= k, "never past K");
}
