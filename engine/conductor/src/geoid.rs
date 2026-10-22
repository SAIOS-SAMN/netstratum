//! THE GEOID (docs/88; operator 2026-10-22: "planetary movement is across a
//! sphere, space traversal is also in a sphere (where sphere is a geoid)";
//! "DAYS_A_YEAR cannot be a constant, it is derived in the same way earth
//! is: the degrees of the rotation in the spin of the planetary mass coupled
//! with the orbit of the planet around its star"; "derive own spin"). Each
//! body answers by its own gravity and density; nothing is squared and no
//! angle passes through a sine.
//!
//! - **Its surface is its throat**, `r = H_b` (the ring law: mass is holding),
//!   where its own hold is the ruled half, `δ_b(H_b) = ½`, so a satellite
//!   skimming it turns at `2·½ = 1` (two rings a bell).
//! - **Its own spin** (`pulse::Body.own`, docs/88) is its turning against its
//!   orbit's frame, on the system's meeting step: born from the SHEAR across
//!   its slot (`shear`: `2·(δ_p(R + d) − δ_p(R − d))`, against its orbit) and
//!   grown by the IMPACTS that build it (`formation::collide_system`), which
//!   bring either sense. The orbit's own turning is met by its star's pull
//!   (that is what an orbit is); only the turning against that frame pushes
//!   the body's matter or turns its star across its sky.
//! - **Its day** is Earth's reading: over one orbit (360° of its star's
//!   direction) its own spin turns it `360°·|ω_own|/ω_orbit` against its
//!   star, so its days a year are `|ω_own|/ω_orbit`, `ω_orbit = 2·δ_p(R)`.
//! - **Its push against its hold:** its own spin's share of the skimming
//!   turn, `s = |ω_own|/1`; only its mantle feels it, the core holds:
//!   `s_κ = s·(1 − κ)`.
//! - **The figure:** at θ degrees from its spin axis the push acts by the
//!   circle's share there, the ruled sine `S(θ) = L(90° − θ) = 1 − |90° −
//!   θ|/90°` (docs/85). The surface stands where the hold less the push is
//!   the pole's: `δ_b(r_θ) = ½·(1 − s_κ·S(θ))`, so
//!   `r_θ = 2H_b/(1 − s_κ·S(θ)) − H_b`. Round where it does not turn, widest
//!   at the equator, shed where `s_κ·S(θ)` reaches one.
use crate::ring::deficit;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A body's figure: its pole (its throat, `H_b`), its orbit's turning about
/// its parent (`2·δ_p(R)`), its own spin (signed: negative against its
/// orbit), and the push share its mantle feels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Figure {
    pub pole: Ratio,
    pub orbit: Ratio,
    pub spin: Ratio,
    pub push: Ratio,
}

/// THE SHEAR a body is born with (docs/88): the matter it gathered from
/// across its reach `d = R·H_b/(H_b + H_p)` (`enfold`) turned about its
/// parent at the parent's deficits there, two rings a bell; in the orbit's
/// frame the inner leads and the outer lags, `2·(δ_p(R + d) − δ_p(R − d))`,
/// against its orbit. None where it holds nothing or stands inside its
/// parent's reach of it.
pub fn shear(r: &Ratio, h_body: &Ratio, h_parent: &Ratio) -> Option<Ratio> {
    if h_body.is_zero() || h_parent.is_zero() || r.is_zero() {
        return None;
    }
    let d = r.clone() * h_body.clone() / (h_body.clone() + h_parent.clone());
    if d.is_zero() || d >= *r {
        return None;
    }
    Some(ratio(2, 1) * (deficit(&(r.clone() + d.clone()), h_parent) - deficit(&(r.clone() - d), h_parent)))
}

/// The ruled sine of θ degrees, `S(θ) = 1 − |90° − θ|/90°`, θ reduced to a
/// half turn: 0 at the poles, 1 at the equator.
pub fn sine(theta: &Ratio) -> Ratio {
    let turn = ratio(360, 1);
    let mut t = theta.clone() - (theta.clone() / turn.clone()).floor() * turn.clone();
    if t > ratio(180, 1) {
        t = turn - t;
    }
    let off = t - ratio(90, 1);
    let off = if off < ratio(0, 1) { -off } else { off };
    ratio(1, 1) - off / ratio(90, 1)
}

/// The figure of a body of holding `h_body`, core share `kappa` and own spin
/// `own`, at reach `r` about the holding `h_parent` inside its orbit. None
/// where it or its parent holds nothing.
pub fn figure(r: &Ratio, h_body: &Ratio, h_parent: &Ratio, kappa: &Ratio, own: &Ratio) -> Option<Figure> {
    if h_body.is_zero() || h_parent.is_zero() || r.is_zero() {
        return None;
    }
    let orbit = ratio(2, 1) * deficit(r, h_parent);
    let magnitude = if *own < ratio(0, 1) { -own.clone() } else { own.clone() };
    let push = magnitude * (ratio(1, 1) - kappa.clone());
    Some(Figure { pole: h_body.clone(), orbit, spin: own.clone(), push })
}

impl Figure {
    /// The surface's reach at θ degrees from the spin axis; None where the
    /// push there reaches the hold (the body sheds).
    pub fn radius_at(&self, theta: &Ratio) -> Option<Ratio> {
        let left = ratio(1, 1) - self.push.clone() * sine(theta);
        if left <= ratio(0, 1) {
            return None;
        }
        Some(ratio(2, 1) * self.pole.clone() / left - self.pole.clone())
    }

    /// The equator's reach over the pole's (one where it does not turn).
    pub fn bulge(&self) -> Option<Ratio> {
        self.radius_at(&ratio(90, 1)).map(|eq| eq / self.pole.clone())
    }

    /// ITS DAYS A YEAR, Earth's reading: the turns its own spin makes
    /// against its star's direction in one orbit, `|ω_own|/ω_orbit`. Zero for
    /// a body locked to its star (one face to it, no day).
    pub fn days_a_year(&self) -> Ratio {
        let own = if self.spin < ratio(0, 1) { -self.spin.clone() } else { self.spin.clone() };
        own / self.orbit.clone()
    }
}

/// The share of a world's flux T a turning world receives at θ degrees from
/// its axis, read along a meridian: `2·S(θ)` (the meridian's mean of S is ½,
/// so the meridian holds T): nothing at the poles, twice the mean at the
/// equator.
pub fn flux_share(theta: &Ratio) -> Ratio {
    ratio(2, 1) * sine(theta)
}

/// THE PEOPLE'S LATITUDE (docs/88): where on its world a people's medium
/// takes up the most of its star's flux. A medium of `h` halvings at the
/// world's mean heat carries, at θ, the local heat's agitation: `h_θ =
/// h/(2·S(θ))`, so its uptake there is `2S·(1 − 2^(−⌊h/(2S)⌋))` of T. Within
/// each whole halving the uptake rises with S, so the best stands at a
/// halving's edge or at the equator: of `S = 1` (`⌊a⌋` halvings) and
/// `S = a/⌈a⌉` (`⌈a⌉` halvings), `a = h/2`, the larger, a construction (law
/// 9). Returns its colatitude in degrees (the northern half; the southern is
/// its mirror) and the uptake as a share of T; None where the medium carries
/// nothing anywhere.
pub fn latitude(h: &Ratio) -> Option<(Ratio, Ratio)> {
    let a = h.clone() / ratio(2, 1);
    if a <= ratio(0, 1) {
        return None;
    }
    let taken = |s: &Ratio, n: &num_bigint::BigInt| -> Ratio {
        let n: u32 = n.try_into().unwrap_or(u32::MAX).min(64);
        ratio(2, 1) * s.clone() * (ratio(1, 1) - Ratio::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2u8).pow(n)))
    };
    let at_equator = (ratio(1, 1), taken(&ratio(1, 1), &a.floor().to_integer()));
    let ceil = a.ceil().to_integer();
    let edge_s = a.clone() / Ratio::from_integer(ceil.clone());
    let at_edge = (edge_s.clone(), taken(&edge_s, &ceil));
    let (s, up) = if at_edge.1 > at_equator.1 { at_edge } else { at_equator };
    (up > ratio(0, 1)).then(|| (ratio(90, 1) * s, up))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_people_stands_where_its_medium_takes_up_the_most() {
        // h = 6 (a = 3): the equator keeps 3 halvings, 2·(7/8) = 7/4 of T.
        assert_eq!(latitude(&ratio(6, 1)), Some((ratio(90, 1), ratio(7, 4))));
        // h = 5 (a = 5/2): the equator keeps 2 halvings, 2·(3/4) = 3/2; the
        // edge of the third, S = 5/6, 2·(5/6)·(7/8) = 35/24 < 3/2.
        assert_eq!(latitude(&ratio(5, 1)), Some((ratio(90, 1), ratio(3, 2))));
        // h = 1 (a = 1/2): the hot equator carries nothing; the medium stands
        // at S = 1/2, 45°, with one halving: 2·(1/2)·(1/2) = 1/2.
        assert_eq!(latitude(&ratio(1, 1)), Some((ratio(45, 1), ratio(1, 2))));
        assert_eq!(latitude(&ratio(0, 1)), None);
    }

    #[test]
    fn the_ruled_sine_closes_at_the_poles_and_peaks_at_the_equator() {
        assert_eq!(sine(&ratio(0, 1)), ratio(0, 1));
        assert_eq!(sine(&ratio(90, 1)), ratio(1, 1));
        assert_eq!(sine(&ratio(180, 1)), ratio(0, 1));
        assert_eq!(sine(&ratio(45, 1)), ratio(1, 2));
        assert_eq!(sine(&ratio(135, 1)), ratio(1, 2));
        assert_eq!(sine(&ratio(-45, 1)), ratio(1, 2));
    }

    #[test]
    fn a_body_is_born_with_its_shear_and_its_day_is_its_own_spin_over_its_orbit() {
        // A body of holding 1 at reach 100 about 99: its reach d = 1; the
        // shear 2·(δ_p(101) − δ_p(99)) = 2·(99/200 − 1/2) = −1/100, against it.
        assert_eq!(shear(&ratio(100, 1), &ratio(1, 1), &ratio(99, 1)), Some(ratio(-1, 100)));
        let f = figure(&ratio(100, 1), &ratio(1, 1), &ratio(99, 1), &ratio(0, 1), &ratio(-1, 100)).expect("a figure");
        assert_eq!(f.pole, ratio(1, 1));
        // Its orbit turns at 2·δ_p(100) = 198/199; its days a year
        // (1/100)/(198/199) = 199/19800.
        assert_eq!(f.orbit, ratio(198, 199));
        assert_eq!(f.days_a_year(), ratio(199, 19800));
        // Its push is its own spin's share of the skimming turn (1).
        assert_eq!(f.push, ratio(1, 100));
        assert_eq!(f.radius_at(&ratio(0, 1)), Some(ratio(1, 1)));
        assert!(f.bulge().expect("bulge") > ratio(1, 1));
        // A body all core holds round; a body that does not turn has no day.
        let core = figure(&ratio(100, 1), &ratio(1, 1), &ratio(99, 1), &ratio(1, 1), &ratio(-1, 100)).expect("a figure");
        assert_eq!(core.bulge(), Some(ratio(1, 1)));
        let locked = figure(&ratio(100, 1), &ratio(1, 1), &ratio(99, 1), &ratio(0, 1), &ratio(0, 1)).expect("a figure");
        assert_eq!(locked.days_a_year(), ratio(0, 1));
    }
}
