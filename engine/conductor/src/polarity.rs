//! Polarity equilibria (docs/105; ruling of 2026-10-28: derivable formulas
//! and master equations for polarity equilibria were missing): the
//! master equation of a two-pole exchange. Two poles A and B; each ring a
//! share α of A passes to B and a share β of B passes back:
//! `A′ = A(1 − α) + B·β`, `B′ = B(1 − β) + A·α`; `A + B` exact. Its
//! equilibrium is the pair share of the two rates, `x* = α/(α + β)` (never a
//! ½: it reads ½ only where the rates are equal), approached in closed form
//! `x_n = x* + (x₀ − x*)·(1 − α − β)ⁿ`. Under inverse gravity (docs/104) what
//! binds goes at the held face `g = 1 − δ` and what frees at the unlocked face
//! `u = δ`, read at the body's own place.
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The share standing on the receiving pole at equilibrium, `α/(α + β)`;
/// None where neither rate moves anything.
pub fn equilibrium(alpha: &Ratio, beta: &Ratio) -> Option<Ratio> {
    let total = alpha.clone() + beta.clone();
    (!total.is_zero()).then(|| alpha.clone() / total)
}

/// The receiving pole's share after `n` rings from `x0`, in closed form:
/// `x* + (x₀ − x*)·(1 − α − β)ⁿ` (compounding over rings).
pub fn after(x0: &Ratio, alpha: &Ratio, beta: &Ratio, n: u32) -> Option<Ratio> {
    let star = equilibrium(alpha, beta)?;
    let keep = ratio(1, 1) - alpha.clone() - beta.clone();
    let pow = (0..n).fold(ratio(1, 1), |acc, _| acc * keep.clone());
    Some(star.clone() + (x0.clone() - star) * pow)
}

/// The faces at a deficit δ under inverse gravity (docs/104 §0): `(g, u)`, the
/// held share `1 − δ` and the unlocked share `δ`. A pair: `g + u = 1`.
pub fn faces(delta: &Ratio) -> (Ratio, Ratio) {
    (ratio(1, 1) - delta.clone(), delta.clone())
}

/// The held share AT the throat (`R = H`): the faces equal, the inversion's
/// fixed point, read there and only there (docs/104 §1.2).
pub fn throat_held() -> Ratio {
    faces(&crate::ring::deficit(&ratio(1, 1), &ratio(1, 1))).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_master_equation_conserves_its_poles_and_stands_at_its_pair_share() {
        let (alpha, beta) = (ratio(1, 5), ratio(1, 3));
        let star = equilibrium(&alpha, &beta).expect("an equilibrium");
        assert_eq!(star, ratio(3, 8));
        // A ring of the exchange leaves the equilibrium standing, the two
        // poles' sum exact.
        let (a, b) = (ratio(1, 1) - star.clone(), star.clone());
        let (a1, b1) = (a.clone() * (ratio(1, 1) - alpha.clone()) + b.clone() * beta.clone(), b.clone() * (ratio(1, 1) - beta.clone()) + a.clone() * alpha.clone());
        assert_eq!(a1.clone() + b1.clone(), ratio(1, 1));
        assert_eq!(b1, star);
        // The closed form agrees with stepping, at N and at 4N rings.
        let step = |x: Ratio| x.clone() * (ratio(1, 1) - beta.clone()) + (ratio(1, 1) - x) * alpha.clone();
        for n in [4u32, 16] {
            let stepped = (0..n).fold(ratio(1, 10), |x, _| step(x));
            assert_eq!(after(&ratio(1, 10), &alpha, &beta, n), Some(stepped));
        }
        // The faces pair; at the throat they meet at the fixed point.
        let (g, u) = faces(&ratio(1, 4));
        assert_eq!(g + u, ratio(1, 1));
        assert_eq!(throat_held(), faces(&crate::ring::deficit(&ratio(1, 1), &ratio(1, 1))).1);
    }
}
