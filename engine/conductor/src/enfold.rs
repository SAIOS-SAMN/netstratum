//! THE ENFOLD LAW (docs/85 §8; operator 2026-10-20). A body holds each value
//! to its own PLACE, the cap its own gravity holds about it, and the residue
//! finer than that place leaves the body's tier:
//! - within the body's own round (`|r| ≤ a/2` of a turn, a its agitation) it
//!   never located the body more finely than its heat does, and DRAINS as
//!   that heat (with xylarium);
//! - for a body with an enclosure it is a rigid turn of that enclosure's
//!   frame, magnified into the inner tier by the tier map's `1/k`;
//! - for a body with none (a mote) it passes to its matter and below the
//!   atom to the QUANTUM tier, acknowledged and never simulated (the first
//!   ring, operator 2026-10-08, ruled for this law 2026-10-20).
//! Nothing a decision reads is lost, and no stored value grows with the
//! echoes: each is held to a place its own body's state sets.
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A body's PLACE on its tier, as a share of a turn: the sexagesimal place
/// its own CAP reaches. The cap is its own gravity's reach, `α = 180°·d/rim`
/// with `d = R·H_b/(H_b + H_p)` (operator 2026-10-18, gravity's reach); in an
/// enclosure's own units the rim is 2 (the rim law: throat 1, rim 2), so the
/// cap is `d/4` of a turn. The place is the largest `60⁻ᵏ` of a turn within
/// that cap (the sexagesimal ladder): never coarser than the body's own cap,
/// so nothing the body resolves is lost, and its denominator is a power of
/// sixty, never the holds' own primes. None where the body holds nothing or
/// stands at no reach.
pub fn place(reach: &Ratio, h_body: &Ratio, h_parent: &Ratio) -> Option<Ratio> {
    let both = h_body.clone() + h_parent.clone();
    if reach.is_zero() || h_body.is_zero() || both.is_zero() {
        return None;
    }
    let cap = reach.clone() * h_body.clone() / both / ratio(4, 1);
    let mut rung = ratio(1, 1);
    while rung > cap {
        rung /= ratio(60, 1);
    }
    Some(rung)
}

/// A phase held to its place: its turn's fraction to the nearest whole
/// place, and the signed residue, `|residue| ≤ place/2`, so no direction is
/// favoured. The held phase stays within one turn, and its denominator is the
/// place's, whatever the value's was. Where the body has no place of its
/// own, the value is held whole.
pub fn hold(v: &Ratio, place: Option<&Ratio>) -> (Ratio, Ratio) {
    let f = v.clone() - v.floor();
    let Some(p) = place.filter(|p| !p.is_zero()) else { return (f, ratio(0, 1)) };
    let held = (f.clone() / p.clone() + ratio(1, 2)).floor() * p.clone();
    let r = f - held.clone();
    let held = if held >= ratio(1, 1) { held - ratio(1, 1) } else { held };
    (held, r)
}

/// Where a residue goes (§8 E2′, E3′): `Drained` within the body's round,
/// `Frame(r/k)` onto its enclosure's frame, `Acknowledged` at a body without
/// an enclosure (the quantum tier).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Residue {
    Drained,
    Frame(Ratio),
    Acknowledged,
}

/// The fate of residue `r` of a body of agitation `a`, whose enclosure (if it
/// has one) is laid on its tier with dilation `k`.
pub fn residue(r: &Ratio, agitation: &Ratio, k: Option<&Ratio>) -> Residue {
    let mag = if *r < ratio(0, 1) { -r.clone() } else { r.clone() };
    if r.is_zero() || mag <= agitation.clone() / ratio(2, 1) {
        return Residue::Drained;
    }
    match k.filter(|k| !k.is_zero()) {
        Some(k) => Residue::Frame(r.clone() / k.clone()),
        None => Residue::Acknowledged,
    }
}

#[cfg(test)]
mod tests {
    use super::{hold, place, residue, Residue};
    use cfe::ratio::ratio;

    #[test]
    fn a_place_is_the_bodys_own_gravity_cap() {
        // d = R·H_b/(H_b + H_p): reach 2, holds 1 and 3, d = ½, a cap d/4 = 1/8
        // of a turn, so the place is the rung 1/60 within it.
        assert_eq!(place(&ratio(2, 1), &ratio(1, 1), &ratio(3, 1)), Some(ratio(1, 60)));
        assert_eq!(place(&ratio(1, 1), &ratio(1, 10_000), &ratio(1, 1)), Some(ratio(1, 216_000)), "a slight body: a fine rung");
        assert_eq!(place(&ratio(8, 1), &ratio(1, 1), &ratio(1, 1)), Some(ratio(1, 1)), "a cap past a turn holds whole turns");
        assert_eq!(place(&ratio(0, 1), &ratio(1, 1), &ratio(3, 1)), None);
        assert_eq!(place(&ratio(2, 1), &ratio(0, 1), &ratio(3, 1)), None);
    }

    #[test]
    fn a_held_phase_and_its_residue_give_the_value_back() {
        let p = ratio(1, 8);
        for v in [ratio(3, 10), ratio(13, 7), ratio(-1, 9), ratio(1, 16), ratio(63, 64)] {
            let (held, r) = hold(&v, Some(&p));
            assert!(r.clone() * ratio(2, 1) <= p && -(r.clone() * ratio(2, 1)) <= p, "{v}: residue within half a place");
            let whole = v.clone() - held.clone() - r;
            assert_eq!(whole.clone(), whole.floor(), "{v}: held + residue is the value, up to whole turns");
            assert!(held >= ratio(0, 1) && held < ratio(1, 1), "{v}: within one turn");
        }
        assert_eq!(hold(&ratio(5, 4), None), (ratio(1, 4), ratio(0, 1)));
    }

    #[test]
    fn a_residue_drains_within_the_round_and_enfolds_beyond_it() {
        assert_eq!(residue(&ratio(1, 100), &ratio(1, 10), None), Residue::Drained);
        assert_eq!(residue(&ratio(1, 10), &ratio(1, 100), Some(&ratio(1, 4))), Residue::Frame(ratio(2, 5)));
        assert_eq!(residue(&ratio(-1, 10), &ratio(0, 1), None), Residue::Acknowledged);
        assert_eq!(residue(&ratio(0, 1), &ratio(0, 1), None), Residue::Drained);
    }
}
