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
use super::thermal::ThermalRegion;
use crate::dop::ZoneKind;

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

/// **THE INTRINSIC GROWTH RATE `r`** — how fast the environment
/// recovers. DERIVED, not dialed: it is the natural biomass rate, the
/// `Habitat`'s own per-tick yield (life regrows at the rate life
/// grows). This is the `r` the archetypes are measured against —
/// extract below it and the Custodian holds; above it and the Locust
/// collapses the world.
pub fn intrinsic_rate() -> Ratio {
    ZoneKind::Habitat.yield_per_tick().1
}

/// **HABITABILITY** — how life-bearing a thermal region is, in `[0,1]`.
/// Biomass is workable only in the MESIC band (`workable_at`, phase 2),
/// so habitability peaks there and tapers monotonically with distance
/// FROM mesic: the temperate fringes carry half, the marginal bands a
/// quarter, and the extremes nothing. It is the ordering that is the
/// law (nearer mesic is more habitable); the fractions are the
/// first-cut taper.
pub fn habitability(region: ThermalRegion) -> Ratio {
    match region {
        // The life band.
        ThermalRegion::Mesic => ratio(1, 1),
        // The temperate fringes, one band out either way.
        ThermalRegion::Sublime | ThermalRegion::Torrid => ratio(1, 2),
        // The marginal bands, two out.
        ThermalRegion::Cryogenic | ThermalRegion::HyperThermal => ratio(1, 4),
        // The extremes and above the ceiling: barren.
        ThermalRegion::Quenched | ThermalRegion::Incandescent | ThermalRegion::Beyond => {
            ratio(0, 1)
        }
    }
}

/// **THE CARRYING CAPACITY `K`** — the natural ceiling of a world's
/// biomass, from the three readings the ruling names: the living ZONES
/// (habitats and regenerators — the ecological footprint), and the
/// place (its thermal region, itself the star luminosity ⊗ orbit). A
/// lush mesic world with many habitats carries a large `K`; a barren
/// rock near zero. `living_zones` is the count of `Habitat` +
/// `Regenerator` seats; each anchors one unit of capacity, scaled by
/// the place's [`habitability`].
pub fn carrying_capacity(living_zones: u64, region: ThermalRegion) -> Ratio {
    ratio(living_zones as i64, 1) * habitability(region)
}
