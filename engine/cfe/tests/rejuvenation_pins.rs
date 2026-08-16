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

// ── K and r — the derivations that feed the curve, generic over the
// sitting line's own metabolism and clade (not hardcoded to biomass) ──

use cfe::almanac::{carrying_capacity, habitability, intrinsic_rate, ThermalRegion};

/// The bimaniform flatlander — clade 2, biotic (metabolism 1... see
/// `dop::metabolic_ration`), preferred region Mesic. Used to pin the
/// v1-verbatim shape.
const FLATLANDER: u16 = 2;
/// A cryophage clade — preferred region Cryogenic, not Mesic — proving
/// habitability tapers from the CLADE's own band, not a fixed one.
const CRYOPHAGE: u16 = 3;
/// Biotic metabolism code (farms/eats Biomass).
const BIOTIC: u8 = 2;
/// Mineral metabolism code (farms/eats Iron).
const MINERAL: u8 = 1;

#[test]
fn the_rate_is_the_lines_own_ration_rate_not_a_dial() {
    // r is the line's own harvest rate — life regrows at the rate life
    // grows, ore at the rate ore bodies form — the same G2 map every
    // other flow reads.
    assert_eq!(
        intrinsic_rate(BIOTIC, None),
        cfe::dop::metabolic_harvest(BIOTIC).1
    );
    assert_eq!(
        intrinsic_rate(MINERAL, None),
        cfe::dop::metabolic_harvest(MINERAL).1
    );
    assert!(intrinsic_rate(BIOTIC, None) > ratio(0, 1), "a positive recovery rate");
}

#[test]
fn habitability_peaks_at_the_clades_own_band_and_tapers_to_the_extremes() {
    // The ordering IS the law: nearer the CLADE's own preferred band is
    // more habitable, and the far extremes bear nothing — for whatever
    // band that clade prefers, not always Mesic.
    let mesic = habitability(FLATLANDER, ThermalRegion::Mesic);
    let fringe = habitability(FLATLANDER, ThermalRegion::Sublime);
    let marginal = habitability(FLATLANDER, ThermalRegion::Cryogenic);
    assert_eq!(mesic, ratio(1, 1), "the flatlander's own band is fullest");
    assert!(mesic > fringe, "the temperate fringe carries less");
    assert!(fringe > marginal, "the marginal band less still");
    assert_eq!(habitability(FLATLANDER, ThermalRegion::Torrid), fringe, "symmetric about mesic");
    assert_eq!(
        habitability(FLATLANDER, ThermalRegion::HyperThermal),
        marginal,
        "symmetric about mesic"
    );
    for barren in [ThermalRegion::Quenched, ThermalRegion::Incandescent, ThermalRegion::Beyond] {
        assert_eq!(habitability(FLATLANDER, barren), ratio(0, 1), "the extremes are barren");
    }

    // A cryophage's own band is Cryogenic, not Mesic — the SAME shape,
    // recentred on its own preference: Mesic is now the marginal band.
    assert_eq!(
        habitability(CRYOPHAGE, ThermalRegion::Cryogenic),
        ratio(1, 1),
        "the cryophage's own band is fullest, wherever it sits"
    );
    assert_eq!(
        habitability(CRYOPHAGE, ThermalRegion::Mesic),
        habitability(FLATLANDER, ThermalRegion::HyperThermal),
        "the same distance-taper, just recentred"
    );
    assert!(
        habitability(CRYOPHAGE, ThermalRegion::Cryogenic)
            > habitability(FLATLANDER, ThermalRegion::Cryogenic),
        "a cryophage reads the cold band as more habitable than a flatlander does"
    );
}

#[test]
fn a_lush_world_carries_more_than_a_barren_one_and_a_rock_carries_nothing() {
    // K rises with the living footprint and with the place's
    // habitability for the SITTING CLADE — a flatlander's own band
    // (Mesic) holds a large K for it.
    // DIO-2: carrying_capacity now reads habitability as the CONTINUOUS flux
    // tent on t_eq4, so feed it a band-CENTRE flux rather than a discrete
    // region. A clade dead-centre in its own band reads η=1; far outside, η=0.
    use cfe::thermal::SubstrateBand;
    let centre = |band: SubstrateBand| -> Ratio {
        let (lo, hi) = band.bounds4();
        (lo + hi) / ratio(2, 1)
    };
    let lush = carrying_capacity(8, FLATLANDER, BIOTIC, &centre(SubstrateBand::Mesic), &ratio(1, 1));
    let cold = carrying_capacity(8, FLATLANDER, BIOTIC, &centre(SubstrateBand::Cryogenic), &ratio(1, 1));
    let rock = carrying_capacity(8, FLATLANDER, BIOTIC, &centre(SubstrateBand::HyperThermal), &ratio(1, 1));
    assert!(lush > cold, "the same zones hold more stock in the flatlander's own band");
    assert_eq!(rock, ratio(0, 1), "a scorched rock carries no ration, whatever it builds");
    // More living zones, more capacity — the footprint scales it.
    assert!(
        carrying_capacity(16, FLATLANDER, BIOTIC, &centre(SubstrateBand::Mesic), &ratio(1, 1)) > lush,
        "a wider ecological footprint holds more"
    );
    // A world with no living zones has no natural ceiling to speak of.
    assert_eq!(carrying_capacity(0, FLATLANDER, BIOTIC, &centre(SubstrateBand::Mesic), &ratio(1, 1)), ratio(0, 1));
    // A cryophage on the SAME cold world reads capacity a flatlander on
    // that world would not — the ceiling follows the clade, not the place.
    let cryophage_cold = carrying_capacity(8, CRYOPHAGE, BIOTIC, &centre(SubstrateBand::Cryogenic), &ratio(1, 1));
    assert!(cryophage_cold > cold, "the cryophage's own band carries more for it");
}

#[test]
fn the_metabolic_phase_boundary_culls_production_live() {
    // ONT-1d: the substrate boundary is LIVE in production. On the SAME cold
    // world, in a clade band that carries capacity for organic life, a PLASMIC
    // metabolism reads ZERO carrying capacity — its plasma/volatile structural
    // phase is absent in the frozen solid, so no body can form, so nothing grows.
    // No prohibition table: the phase feasibility zeroes the ceiling. Biotic
    // organic life on the same world is untouched (feasibility 1).
    let centre = |band: cfe::thermal::SubstrateBand| {
        let (lo, hi) = band.bounds4();
        (lo + hi) / ratio(2, 1)
    };
    const PLASMIC: u8 = 3;
    let cold = centre(cfe::thermal::SubstrateBand::Cryogenic);
    assert!(
        carrying_capacity(8, CRYOPHAGE, BIOTIC, &cold, &ratio(1, 1)) > ratio(0, 1),
        "organic life carries capacity in its own cold band"
    );
    assert_eq!(
        carrying_capacity(8, CRYOPHAGE, PLASMIC, &cold, &ratio(1, 1)),
        ratio(0, 1),
        "a plasmic has no plasma phase in the frozen solid — production culled"
    );
}

#[test]
fn the_derivations_feed_the_curve_and_a_collapsed_lush_world_climbs_back() {
    // The pieces compose: a world in a clade's own band, stripped to an
    // ember, recovers toward its (large) K along the S-curve, slowly at
    // first — the flatlander/biotic pair, verbatim v1.
    let k = {
        let (lo, hi) = cfe::thermal::SubstrateBand::Mesic.bounds4();
        carrying_capacity(10, FLATLANDER, BIOTIC, &((lo + hi) / ratio(2, 1)), &ratio(1, 1))
    };
    let r = intrinsic_rate(BIOTIC, None);
    let ember = ratio(1, 10);
    let early = cfe::almanac::grained_logistic(&ember, &k, &r, 10);
    let late = cfe::almanac::grained_logistic(&ember, &k, &r, 10_000);
    assert!(early > ember && early < k, "it has begun to climb");
    assert!(late > early, "and keeps climbing toward its lush ceiling");
    assert!(late <= k, "never past K");
}
