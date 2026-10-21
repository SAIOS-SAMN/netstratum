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

/// THE CROSS (operator 2026-10-20: "descent can not be done without all of
/// the other pivotry components: ascent, trajectory, drag, velocity,
/// condense, drain, puncture, cauterize"; ruled for netstratum 2026-10-21).
/// Every residue's fate is recorded by its plane (its TRAJECTORY: the plane,
/// and its sense in the signed sum) for each echo (its VELOCITY: the record is
/// per echo). DRAG is the dynamics' own hold (δ at the body's reach), never a
/// fate here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fate {
    /// Handed into the body's own enclosure (its members' places).
    Descended,
    /// Read back up: a decision read the full value (a condensation's
    /// members, a held value with its residue).
    Ascended,
    /// Within the body's own round: its heat already holds it.
    Drained,
    /// Past a body's round, its WEIGHT taken by the lever rule: the share
    /// `|r|/p` of its holding stands on the neighbouring mark, so the
    /// holding's centre is the body's true place (its moment `m·r` kept).
    Weighted,
    /// Below one nucleus of weight, to the quantum tier: acknowledged, never
    /// simulated.
    Acknowledged,
    /// Joined in a meeting: bodies meeting into one, their full phases
    /// meaned by holding on the circle.
    Condensed,
    /// Through a throat (a body entering the other face).
    Punctured,
    /// Leaving a throat on the continuum, sealed.
    Sealed,
    /// A body with no place of its own (no reach or no holding): sealed by
    /// heat at the coordinate's mark, its residue kept as a SCAR.
    Cauterized,
}

/// One plane's record for one fate: how many residues, and their exact
/// signed sum (the sense of the trajectory).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tally {
    pub count: u64,
    pub sum: Ratio,
}

/// The cross's record over an echo: by plane, by fate.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cross {
    pub by: std::collections::BTreeMap<(usize, Fate), Tally>,
}

impl Cross {
    /// Record residue `r` in `plane` with its fate.
    pub fn note(&mut self, plane: usize, fate: Fate, r: &Ratio) {
        let t = self.by.entry((plane, fate)).or_default();
        t.count += 1;
        t.sum += r.clone();
    }

    /// Fold another record into this one.
    pub fn join(&mut self, other: &Cross) {
        for (k, t) in &other.by {
            let mine = self.by.entry(*k).or_default();
            mine.count += t.count;
            mine.sum += t.sum.clone();
        }
    }

    /// How many residues met each fate, every plane together.
    pub fn counts(&self) -> std::collections::BTreeMap<Fate, u64> {
        let mut out = std::collections::BTreeMap::new();
        for ((_, f), t) in &self.by {
            *out.entry(*f).or_insert(0) += t.count;
        }
        out
    }
}

/// The coordinate's mark, `60⁻⁶` of a turn (the TUniversal coordinate's six
/// sexagesimal places): where a body with no place of its own is sealed.
pub fn mark() -> Ratio {
    Ratio::new(1.into(), num_bigint::BigInt::from(60).pow(6))
}

/// THE LEVER (docs/85 §9): a body held to place p whose true position lies r
/// past its mark keeps its holding on marks without loss only by standing
/// on the two marks that bracket it: `1 − |r|/p` on its mark and `|r|/p` on
/// the next mark in r's sense. Then the holding's centre is the true place,
/// `(1 − f)·held + f·(held ± p) = held + r`, its moment `m·r = W·p` kept, so
/// the residue's WEIGHT is `W = m·|r|/p`. Returns (the neighbouring mark, the
/// share f).
pub fn lever(held: &Ratio, r: &Ratio, place: &Ratio) -> (Ratio, Ratio) {
    let sense = if *r < ratio(0, 1) { -place.clone() } else { place.clone() };
    let n = held.clone() + sense;
    let mag = if *r < ratio(0, 1) { -r.clone() } else { r.clone() };
    (n.clone() - n.floor(), mag / place.clone())
}

/// A LEAF's phase through the cross: held to its place; its residue drains
/// within its round; past it, its weight is taken by the lever (`lever`,
/// returned for the body to split its holding on the two marks); with no
/// place of its own it is cauterized at the coordinate's mark, the residue a
/// scar that enters the body's own heat: the round that would hold it,
/// `2·|r|` (§2: a round of agitation a holds half a turn's share a/2 about
/// the body). Returns the held phase, the scar's heat, and the lever.
pub fn leaf(v: &Ratio, place: Option<&Ratio>, agitation: &Ratio, plane: usize, cross: &mut Cross) -> (Ratio, Ratio, Option<(Ratio, Ratio)>) {
    let (held, r, cauterized) = match place.filter(|p| !p.is_zero()) {
        Some(p) => {
            let (h, r) = hold(v, Some(p));
            (h, r, false)
        }
        None => {
            let (h, r) = hold(v, Some(&mark()));
            (h, r, true)
        }
    };
    if r.is_zero() {
        return (held, ratio(0, 1), None);
    }
    if cauterized {
        cross.note(plane, Fate::Cauterized, &r);
        let mag = if r < ratio(0, 1) { -r.clone() } else { r.clone() };
        return (held, ratio(2, 1) * mag, None);
    }
    match (residue(&r, agitation, None), place) {
        (Residue::Drained, _) | (_, None) => {
            cross.note(plane, Fate::Drained, &r);
            (held, ratio(0, 1), None)
        }
        (_, Some(p)) => {
            cross.note(plane, Fate::Weighted, &r);
            let l = lever(&held, &r, p);
            (held, ratio(0, 1), Some(l))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{hold, lever, leaf, place, residue, Cross, Fate, Residue};
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

    #[test]
    fn the_lever_keeps_the_holdings_centre_on_the_true_place() {
        let p = ratio(1, 60);
        for v in [ratio(31, 1800), ratio(59, 3600), ratio(1, 7)] {
            let (held, r) = hold(&v, Some(&p));
            let (n, f) = lever(&held, &r, &p);
            // The centre of (1 − f) on its mark and f on the next, against the mark.
            let off = |x: &cfe::ratio::Ratio| { let d = x.clone() - held.clone(); let d = d.clone() - d.round(); d };
            assert_eq!((ratio(1, 1) - f.clone()) * ratio(0, 1) + f * off(&n), r, "{v}: the moment m·r is kept");
        }
    }

    #[test]
    fn every_residue_meets_a_recorded_fate() {
        let mut cross = Cross::default();
        // Held to a place of 1/60: 31/1800 lies 1/1800 past a mark.
        let (held, heat, lever) = leaf(&ratio(31, 1800), Some(&ratio(1, 60)), &ratio(0, 1), 0, &mut cross);
        assert_eq!(heat, ratio(0, 1));
        assert_eq!(held, ratio(1, 60));
        assert_eq!(cross.by.get(&(0, Fate::Weighted)).map(|t| (t.count, t.sum.clone())), Some((1, ratio(1, 1800))));
        // Past its round its weight stands on the next mark: f = (1/1800)/(1/60) = 1/30.
        assert_eq!(lever, Some((ratio(2, 60), ratio(1, 30))));
        // Within its round it drains.
        let _ = leaf(&ratio(31, 1800), Some(&ratio(1, 60)), &ratio(1, 100), 1, &mut cross);
        assert_eq!(cross.by.get(&(1, Fate::Drained)).map(|t| t.count), Some(1));
        // With no place of its own it is cauterized at the coordinate's mark.
        let (_, scar, _) = leaf(&ratio(1, 7), None, &ratio(0, 1), 0, &mut cross);
        assert_eq!(cross.counts().get(&Fate::Cauterized), Some(&1));
        assert!(scar > ratio(0, 1), "a cauterized residue enters the body's heat");
    }
}
