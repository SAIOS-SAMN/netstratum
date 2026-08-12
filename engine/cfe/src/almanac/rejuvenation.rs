//! **THE REGROWTH LAW — the ruled curve** (task #28, `40_REJUVENATION`).
//!
//! Natural resources recover toward a carrying capacity `K` along a
//! logistic S-curve: slow near zero, fast at mid-stock, saturating at
//! `K`. The continuous solution `P = K / (1 + ((K−P0)/P0)·e^{−rΔt})`
//! carries a transcendental `e^{−rΔt}` this engine cannot hold — the
//! same wall that killed proportional demurrage at ~2^1100 and forced
//! the flat law. So the exponential is replaced by a RATIONAL SURROGATE
//! on a grain (the `grained_t_eq4`/`ORBIT_GRAIN` discipline):
//!
//!     s(Δt) = GRAIN / (GRAIN + r·Δt)
//!
//! which is `1` at `Δt=0`, falls monotonically toward `0`, and is a
//! single exact ratio — so the whole curve is ONE rational operation,
//! never an iterated product, and its denominator stays bounded.
//! Substituting and clearing fractions:
//!
//!     P(Δt) = K·P0·(GRAIN + r·Δt)
//!             ─────────────────────────────────────
//!             P0·(GRAIN + r·Δt) + (K − P0)·GRAIN
//!
//! This is the RULED (B) form: the S-shape is preserved (a collapsed
//! world recovers slowly at first, which is what forces the Locust to
//! keep jumping), and the arithmetic stays exact and bounded.

use crate::ratio::{ratio, Ratio};

/// The half-recovery scale — `r·Δt = GRAIN` is where the surrogate
/// reaches `1/2`, so `GRAIN/r` is the time to close half the gap to
/// `K`. Reused from the melt/orbit grid (`ORBIT_GRAIN`): the same
/// discipline that keeps orbit heat bounded keeps regrowth bounded, not
/// a new number.
pub const REGROWTH_GRAIN: i64 = 64;

/// **THE REGROWTH CURVE.** Where a stock of `p0` recovers to after `dt`
/// beats, toward carrying capacity `k` at intrinsic rate `r`. Pure,
/// total, exact-rational, O(1) in `dt`.
///
/// - A stock at or above capacity holds at capacity (no overshoot).
/// - A stock of exactly nothing stays nothing: regrowth needs a living
///   remnant to grow from (the near-zero the collapse law floors at) —
///   a resource extracted to the last unit must be RESEEDED, never
///   spontaneously generated.
pub fn grained_logistic(p0: &Ratio, k: &Ratio, r: &Ratio, dt: u64) -> Ratio {
    let zero = ratio(0, 1);
    // A dead stock with no seed cannot regrow; a full one cannot grow.
    if *p0 <= zero {
        return zero;
    }
    if p0 >= k {
        return k.clone();
    }
    let grain = ratio(REGROWTH_GRAIN, 1);
    // GRAIN + r·Δt — the surrogate's denominator, carrying the elapsed
    // beats. One exact rational; no power is ever taken.
    let span = grain.clone() + r.clone() * ratio(dt as i64, 1);
    let num = k.clone() * p0.clone() * span.clone();
    let den = p0.clone() * span + (k.clone() - p0.clone()) * grain;
    num / den
}
