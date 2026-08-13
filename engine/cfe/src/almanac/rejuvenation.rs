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
use super::taxon_economy::clade_preferred_region;
use super::thermal::ThermalRegion;
use crate::dop::metabolic_harvest_of;

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
/// recovers. DERIVED, not dialed: it is the line's OWN ration rate, the
/// same G2 metabolic-harvest map every other flow reads
/// ([`metabolic_harvest_of`]) — a biotic line regrows biomass at the
/// rate biotic life grows, a mineral line regrows iron at the rate ore
/// bodies form, a stone-eater regrows its own minted element at the
/// biotic rate. This is the `r` the archetypes are measured against —
/// extract below it and the Custodian holds; above it and the Locust
/// collapses the world. Plasmic farms no yield line (rate 0): captured
/// heat has no stock to regrow.
pub fn intrinsic_rate(metabolism: u8, element: Option<[u8; 32]>) -> Ratio {
    metabolic_harvest_of(metabolism, element).1
}

/// **HABITABILITY** — how life-bearing a thermal region is FOR A GIVEN
/// CLADE, in `[0,1]`. Every clade has its own workable band
/// ([`clade_preferred_region`] — Mesic for the bimaniform flatlander,
/// Cryogenic for a cryophage, HyperThermal for a magmatic line, and so
/// on), and habitability peaks there and tapers monotonically with
/// thermal-band DISTANCE from that preference: one band out is half,
/// two bands out a quarter, three or more barren. It is the ordering
/// that is the law (nearer a clade's own preference is more habitable);
/// the fractions are the first-cut taper. `ThermalRegion`'s declared
/// order IS the band ladder ([`ThermalRegion`] derives `Ord`), so the
/// distance is just the ladder-index gap.
pub fn habitability(clade: u16, region: ThermalRegion) -> Ratio {
    let pref = clade_preferred_region(clade);
    let distance = (region as i32 - pref as i32).unsigned_abs();
    match distance {
        0 => ratio(1, 1),
        1 => ratio(1, 2),
        2 => ratio(1, 4),
        _ => ratio(0, 1),
    }
}

/// **THE CARRYING CAPACITY `K`** — the natural ceiling of a world's
/// ration stock, from the three readings the ruling names: the living
/// ZONES (habitats and regenerators — the ecological footprint), and
/// the place (its thermal region, itself the star luminosity ⊗ orbit)
/// AS READ BY THE SITTING CLADE. A lush world in a clade's own band
/// with many habitats carries a large `K`; a barren rock near zero.
/// `living_zones` is the count of `Habitat` + `Regenerator` seats; each
/// sustains a RECOVERY-WINDOW's worth of stock — `REGROWTH_GRAIN`
/// units, a living zone's unit output over the half-recovery span —
/// scaled by the place's [`habitability`] for that clade, so a lush
/// world's `K` stands well above any young colony's founding stock and
/// a scorched rock holds nothing.
pub fn carrying_capacity(living_zones: u64, clade: u16, region: ThermalRegion) -> Ratio {
    ratio(living_zones as i64 * REGROWTH_GRAIN, 1) * habitability(clade, region)
}
