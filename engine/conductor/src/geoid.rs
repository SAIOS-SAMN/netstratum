//! THE GEOID (docs/88; operator 2026-10-22: "planetary movement is across a
//! sphere, space traversal is also in a sphere (where sphere is a geoid)").
//! A body's own sphere is the shell of its own enclosure, its reach `d`
//! (`enfold`: `d = R·H_b/(H_b + H_p)`), and its figure is that shell's
//! equipotential under its own turning. Each body answers by its own gravity
//! and density; nothing is squared and no angle passes through a sine.
//!
//! - **Its own spin rises from below** (law 7): the matter a body gathered
//!   across its reach turned about its parent at the parent's deficits
//!   there, `δ_p(R − d)` inside and `δ_p(R + d)` outside (`ring::deficit`:
//!   a place's turning is its deficit). Gathered, the difference is its own
//!   turning, `ω_s = δ_p(R + d) − δ_p(R − d)`: the inner matter leads, so the
//!   SHEAR spins it against its orbit's sense (negative, retrograde; law 6:
//!   both senses).
//! - **Its push against its hold:** a satellite skimming its own reach turns
//!   at `δ_b(d) = H_b/(d + H_b)`; its own turning's share of that is
//!   `s = |ω_s| / δ_b(d)`. Only its mantle feels the push, its core holds:
//!   `s_κ = s·(1 − κ)`.
//! - **The figure:** at θ degrees from its spin axis the push acts by the
//!   circle's share there, the ruled sine `S(θ) = L(90° − θ) = 1 − |90° −
//!   θ|/90°` (docs/85). The surface stands where the hold less the push is
//!   the pole's: `δ_b(r_θ) = δ_b(d)·(1 − s_κ·S(θ))`, so
//!   `r_θ = (d + H_b)/(1 − s_κ·S(θ)) − H_b`. Round where it does not turn,
//!   widest at the equator, shed where `s_κ·S(θ)` reaches one.
use crate::ring::deficit;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A body's figure: its pole reach (its own enclosure), its holding, its
/// own turning (signed: negative against its orbit) and the push share its
/// mantle feels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Figure {
    pub pole: Ratio,
    pub hold: Ratio,
    pub spin: Ratio,
    pub push: Ratio,
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

/// The figure of a body of holding `h_body` and core share `kappa`, at reach
/// `r` about the holding `h_parent` inside its orbit. None where it holds
/// nothing, has no reach of its own, or stands inside its parent's reach of
/// it (`d ≥ R`).
pub fn figure(r: &Ratio, h_body: &Ratio, h_parent: &Ratio, kappa: &Ratio) -> Option<Figure> {
    if h_body.is_zero() || r.is_zero() {
        return None;
    }
    let d = r.clone() * h_body.clone() / (h_body.clone() + h_parent.clone());
    if d.is_zero() || d >= *r {
        return None;
    }
    let spin = deficit(&(r.clone() + d.clone()), h_parent) - deficit(&(r.clone() - d.clone()), h_parent);
    let own = deficit(&d, h_body);
    let s = if spin < ratio(0, 1) { -spin.clone() } else { spin.clone() } / own;
    let push = s * (ratio(1, 1) - kappa.clone());
    Some(Figure { pole: d, hold: h_body.clone(), spin, push })
}

impl Figure {
    /// The surface's reach at θ degrees from the spin axis; None where the
    /// push there reaches the hold (the body sheds).
    pub fn radius_at(&self, theta: &Ratio) -> Option<Ratio> {
        let left = ratio(1, 1) - self.push.clone() * sine(theta);
        if left <= ratio(0, 1) {
            return None;
        }
        Some((self.pole.clone() + self.hold.clone()) / left - self.hold.clone())
    }

    /// The equator's reach over the pole's (one where it does not turn).
    pub fn bulge(&self) -> Option<Ratio> {
        self.radius_at(&ratio(90, 1)).map(|eq| eq / self.pole.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn a_body_bulges_by_its_own_shear_and_its_core_holds_it_round() {
        // A body of holding 1 at reach 100 about 99: its reach d = 1.
        let f = figure(&ratio(100, 1), &ratio(1, 1), &ratio(99, 1), &ratio(0, 1)).expect("a figure");
        assert_eq!(f.pole, ratio(1, 1));
        // The shear: δ_p(101) − δ_p(99) = 99/200 − 99/198 < 0, against its orbit.
        assert_eq!(f.spin, ratio(99, 200) - ratio(1, 2));
        assert!(f.spin < ratio(0, 1));
        // Its push over its own hold δ_b(1) = 1/2: (1/200)/(1/2) = 1/100.
        assert_eq!(f.push, ratio(1, 100));
        // Widest at the equator, round at the poles, exact.
        assert_eq!(f.radius_at(&ratio(0, 1)), Some(ratio(1, 1)));
        assert_eq!(f.radius_at(&ratio(90, 1)), Some(ratio(2, 1) / ratio(99, 100) - ratio(1, 1)));
        assert!(f.bulge().expect("bulge") > ratio(1, 1));
        // A body all core holds round.
        let core = figure(&ratio(100, 1), &ratio(1, 1), &ratio(99, 1), &ratio(1, 1)).expect("a figure");
        assert_eq!(core.bulge(), Some(ratio(1, 1)));
    }
}
