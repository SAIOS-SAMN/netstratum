//! docs/83 "The derivation": THE RING-DENSITY LAW (xylarium 11.0, shared
//! form). Every bell (a core, a star, a grain) holds each ring that reaches
//! it for its own period, then passes it on as its echo. A ring crosses
//! empty space to a place at r (in light-rings) in r and is held by every
//! bell inside r for their periods together, `H(<r)`: it arrives after
//! `r + H(<r)`. MASS IS HOLDING (`M = H`); no G, no c, no mass unit.
//!
//! - **The clock:** a place's rate is the rings it receives,
//!   `r / (r + H)`.
//! - **The deficit:** the share of the turn the holding withholds,
//!   `δ = H / (r + H)`, so `δ < 1`: speed `v = δ` (a share of light) is
//!   bounded, with a horizon as `r → 0`, and the clock is `1 − δ`.
//! - **The turning:** `ω = δ / r` turns a ring, so a turn takes
//!   `P = r / δ = r · (r + H) / H`.
//! - **The balance** (cold, `v_th = 0`): a body of specific spin
//!   `ℓ = r · v = r · δ(r)` stands where `ℓ · (r + H) = r · H`,
//!   `r = ℓ · H / (H − ℓ)`; at `ℓ ≥ H` no radius holds it: it ESCAPES.
//!
//! In transition (as xylarium's): H is read from the grains' holds in units
//! of the deepest hole, and a ring is a conductor bell, until the bounce
//! rebuilds the deployment (docs/83 Phase 1). The clock (`rate`) is live;
//! the motion (`period`, `radius`) is HELD: measured 2026-10-15, its spin
//! `ℓ = r·δ` saturates at H, so on the pulse's orbits (r ≈ 3600·H) a mass
//! loss above 1/3600 unbinds an orbit, and a 6,000-bell breath escaped 1,894
//! systems and bore none (docs/83, "The spin under the ring law").
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A place's clock: `r / (r + H)`; 1 where nothing holds.
pub fn rate(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(1, 1);
    }
    r.clone() / (r.clone() + held.clone())
}

/// The deficit: `H / (r + H)`, the speed of a turning at r as a share of
/// light.
pub fn deficit(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(0, 1);
    }
    held.clone() / (r.clone() + held.clone())
}

/// A turn's length at r: `r / δ = r · (r + H) / H`; zero where nothing
/// holds (nothing turns it).
pub fn period(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(0, 1);
    }
    r.clone() * (r.clone() + held.clone()) / held.clone()
}

/// The specific spin of a circular turning at r: `ℓ = r · δ(r)`.
pub fn spin(r: &Ratio, held: &Ratio) -> Ratio {
    r.clone() * deficit(r, held)
}

/// THE BALANCE: the radius where spin ℓ turns about H, `ℓ · H / (H − ℓ)`;
/// None where it escapes (`ℓ ≥ H`) or nothing holds it.
pub fn radius(ell: &Ratio, held: &Ratio) -> Option<Ratio> {
    if held.is_zero() || *ell <= ratio(0, 1) || ell >= held {
        return None;
    }
    Some(ell.clone() * held.clone() / (held.clone() - ell.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_law_for_the_clock_the_motion_and_the_balance() {
        let (r, h) = (ratio(3, 1), ratio(1, 1));
        assert_eq!(rate(&r, &h), ratio(3, 4));
        assert_eq!(deficit(&r, &h), ratio(1, 4));
        assert_eq!(rate(&r, &h) + deficit(&r, &h), ratio(1, 1), "the clock is 1 − δ");
        assert_eq!(period(&r, &h), ratio(12, 1));
        // The balance gives back the radius its spin was read at.
        assert_eq!(radius(&spin(&r, &h), &h), Some(r));
        assert_eq!(radius(&ratio(1, 1), &h), None, "ℓ ≥ H escapes");
        assert!(deficit(&ratio(1, 1_000_000), &ratio(1_000_000, 1)) < ratio(1, 1), "bounded below light");
    }
}
