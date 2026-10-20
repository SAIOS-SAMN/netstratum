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
//!   `δ = H / (r + H)`, so `δ < 1`, approaching 1 as `r → 0`; the clock is
//!   `1 − δ`.
//! - **The turning** (operator, 2026-10-15): each ring a place receives
//!   turns it by its deficit, `ω = δ` turns a received ring, so a turn takes
//!   `P = 1 / δ = 1 + R / H` received rings: linear, no power.
//! - **The spin** (reach × holding, with xylarium): `ℓ = R · H`, a product
//!   of two distinct readings (the rings of reach and the rings held inside
//!   it), never the square of one. Conserved; a system adds `Σ m·ℓ`.
//! - **The balance:** a body of spin ℓ stands at `R = ℓ / H`. Orbits
//!   BREATHE (operator): the law fixes no direction; an orbit draws in when
//!   the holding inside it grows or it loses spin, and moves out when the
//!   holding falls or it gains spin. The ledger decides which. Escape is
//!   read at events (the stretch crossing e → 1), never a share of H.
//!
//! In transition: H is read from the grains' holds in units of the deepest
//! hole (the core's own state hold is not yet on the ledger, docs/83), and a
//! received ring is a conductor bell.
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

/// A turn's length at reach r: `1 / δ = 1 + R / H` received rings; zero
/// where nothing holds (nothing turns it).
pub fn period(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(0, 1);
    }
    ratio(1, 1) + r.clone() / held.clone()
}

/// The specific spin of a body at reach r about the holding H inside it:
/// `ℓ = R · H`.
pub fn spin(r: &Ratio, held: &Ratio) -> Ratio {
    r.clone() * held.clone()
}

/// THE BALANCE: the reach where spin ℓ stands about H, `ℓ / H`; None where
/// nothing holds it or it carries no spin.
pub fn radius(ell: &Ratio, held: &Ratio) -> Option<Ratio> {
    if held.is_zero() || *ell <= ratio(0, 1) {
        return None;
    }
    Some(ell.clone() / held.clone())
}

/// THE SPIN GRAIN (design law 8, exact and bounded): a body's specific spin
/// and its place are held on 60⁻⁶ (six sexagesimal places, the TUniversal
/// coordinate's), floored; what flooring leaves goes back to the medium the
/// spin came from, so the ledger stays exact while no body's numbers grow
/// with the births and echoes that made it.
pub fn grain(x: &Ratio) -> Ratio {
    let places = Ratio::from_integer(num_bigint::BigInt::from(60).pow(6));
    (x.clone() * places.clone()).floor() / places
}

/// The whole count of a non-negative exact value, `⌊x⌋`, where a `u64`
/// carries it; refused otherwise (never a default).
pub fn whole(x: &Ratio) -> Option<u64> {
    if *x < ratio(0, 1) {
        return None;
    }
    x.floor().to_integer().try_into().ok()
}

/// A share's SECTOR on the 60⁶-mark protractor (the TUniversal coordinate's
/// six sexagesimal places, docs/78): which sector holds it, by exact floor.
pub fn sector(share: &Ratio) -> u64 {
    let places = Ratio::from_integer(num_bigint::BigInt::from(60).pow(6));
    (share.clone() * places).floor().to_integer().try_into().unwrap_or(0)
}

/// A BODY'S IDENTITY is its birth reading, never a row number (operator
/// 2026-10-16: "that's spreadsheet logic"): the bell it was born and the
/// sector of its holding share (the deficit `H/(R + H)` at its reach, the
/// coordinate's G lane) where it was born, `born · 2³⁶ + sector`. Unique
/// among its siblings; on an exact tie the next sector is taken.
pub fn birth_key(born: u64, reach: &Ratio, held: &Ratio, siblings: &[u64]) -> u64 {
    let mut key = (born << 36) | sector(&deficit(reach, held));
    while siblings.contains(&key) {
        key += 1;
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_whole_count_is_read_or_refused_never_defaulted() {
        assert_eq!(whole(&ratio(7, 2)), Some(3));
        assert_eq!(whole(&ratio(0, 1)), Some(0));
        assert_eq!(whole(&ratio(-1, 2)), None);
        let past = Ratio::from_integer(num_bigint::BigInt::from(2).pow(70));
        assert_eq!(whole(&past), None);
    }

    #[test]
    fn one_law_for_the_clock_the_motion_and_the_balance() {
        let (r, h) = (ratio(3, 1), ratio(1, 1));
        assert_eq!(rate(&r, &h), ratio(3, 4));
        assert_eq!(deficit(&r, &h), ratio(1, 4));
        assert_eq!(rate(&r, &h) + deficit(&r, &h), ratio(1, 1), "the clock is 1 − δ");
        assert_eq!(period(&r, &h), ratio(4, 1), "one turn in 1/δ received rings");
        // The balance gives back the reach its spin was read at.
        assert_eq!(radius(&spin(&r, &h), &h), Some(r.clone()));
        // Orbits breathe: half the holding, twice the reach; double, half.
        assert_eq!(radius(&spin(&r, &h), &ratio(1, 2)), Some(ratio(6, 1)));
        assert_eq!(radius(&spin(&r, &h), &ratio(2, 1)), Some(ratio(3, 2)));
        assert!(deficit(&ratio(1, 1_000_000), &ratio(1_000_000, 1)) < ratio(1, 1), "bounded below light");
    }
}
