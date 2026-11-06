// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The dipole coupling (docs/121 §9–§10): every body is a dipole on each
//! plane. Its pole is the sign of its span, closed against open frames. Its
//! orientation is its turning on that plane. Its strength is its open
//! frames. A body answers the field of its range as one: its neighbours'
//! dipoles, each by the ring law's nearness and the hemisphere law's sign.
//! The field moves its rate by the flow law, bounded by the luminal rate.
//! Attraction and repulsion are one law and its sign; nothing is halved,
//! squared or placed.
//!
//! Shear is the degree of modulation at every level (docs/121 §11): phase,
//! quantum and state. `pulled` composes any level's rate, a share of that
//! level's luminal bound; the lane it reads is the level.

use cfe::ratio::{ratio, Ratio};
use num_traits::{Signed, Zero};

/// The L1 circle's cosine (docs/118 F1, size L1; docs/61, the universe's
/// circle the L1 diamond, `8R` a turn): a point at `θ ∈ [0°, 180°]` from its
/// axis projects onto it by `1 − θ/90°`, first degree and exact.
pub fn cosine(theta: &Ratio) -> Ratio {
    ratio(1, 1) - theta.clone() / ratio(90, 1)
}

/// A frame state's angle between its Z pole (0°) and its Q pole (180°),
/// the throat at 90° (docs/117 §10.1): `θ = 180°·Q/(A + Q)`.
pub fn pole_angle(closed: &Ratio, open: &Ratio) -> Ratio {
    let whole = closed.clone() + open.clone();
    if whole.is_zero() {
        ratio(90, 1)
    } else {
        ratio(180, 1) * open.clone() / whole
    }
}

/// A dipole's pole, opened by degrees (operator 2026-11-06): the L1 cosine of
/// its frame angle, `1 − θ/90° = (A − Q)/(A + Q)`, its span over its whole,
/// in `[−1, 1]`; never a bare sign.
pub fn pole(closed: &Ratio, open: &Ratio) -> Ratio {
    cosine(&pole_angle(closed, open))
}

/// Two orientations' alignment on one plane, opened by degrees (operator
/// 2026-11-06): their separation `Δ ∈ [0°, 180°]` on the closed circle (the
/// orientations in turns), read by the L1 cosine, `1 − Δ/90°`: full at 0°,
/// none at 90°, opposed at 180°, every degree between.
pub fn hemisphere(a: &Ratio, b: &Ratio) -> Ratio {
    let d = a.clone() - b.clone();
    let f = d.clone() - d.floor();
    let off = if f > ratio(1, 2) { ratio(1, 1) - f } else { f };
    cosine(&(ratio(360, 1) * off))
}

/// The ring law's nearness of a pair at gap `g` with moments `m`:
/// `M/(|g| + M)`; none where nothing couples.
pub fn nearness(gap: &Ratio, m: &Ratio) -> Ratio {
    let whole = gap.abs() + m.clone();
    if whole.is_zero() {
        ratio(0, 1)
    } else {
        m.clone() / whole
    }
}

/// One neighbour's part in a body's field on one plane: its holding, its
/// pole, its open share, its nearness and its hemisphere sign.
#[derive(Debug, Clone)]
pub struct Neighbour {
    pub holding: Ratio,
    pub pole: Ratio,
    pub open: Ratio,
    pub nearness: Ratio,
    pub side: Ratio,
}

/// The field of a body's range on one plane, as one, against the range's
/// whole holding (docs/121 §9.2): `Σ w_j·σ_j·o_j·n_j·h_j / Σ w_j`. Each term
/// stands in `[−1, 1]`, so the field does.
pub fn field(range: &[Neighbour]) -> Ratio {
    let whole = range.iter().fold(ratio(0, 1), |w, n| w + n.holding.clone());
    if whole.is_zero() {
        return ratio(0, 1);
    }
    range.iter().fold(ratio(0, 1), |f, n| {
        f + n.holding.clone()
            * n.pole.clone()
            * n.side.clone()
            * n.open.clone()
            * n.nearness.clone()
    }) / whole
}

/// The flow law's inverse: `x ⊖ y = x ⊕ (−y)`.
fn minus(x: &Ratio, y: &Ratio) -> Option<Ratio> {
    crate::ring::compose(x, &-y.clone())
}

/// A body's rate on one plane, a share of the luminal bound in `(−1, 1)`,
/// moved by its range's field (docs/121 §10.4): composed by the flow law
/// with the pull `y = −σ·F·o·(x̄ ⊖ x)` toward the range's rate `x̄`
/// (attracting where `σ·F < 0`) or from it (repelling where `σ·F > 0`).
/// None where the composition leaves the bound's domain.
pub fn pulled(
    x: &Ratio,
    range_rate: &Ratio,
    own_pole: &Ratio,
    own_open: &Ratio,
    f: &Ratio,
) -> Option<Ratio> {
    let toward = minus(range_rate, x)?;
    let y = -own_pole.clone() * f.clone() * own_open.clone() * toward;
    crate::ring::compose(x, &y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pole_and_the_alignment_open_by_degrees() {
        // The L1 circle's cosine, 1 - theta/90 degrees, exact.
        assert_eq!(pole_angle(&ratio(2, 1), &ratio(1, 1)), ratio(60, 1));
        assert_eq!(pole(&ratio(2, 1), &ratio(1, 1)), ratio(1, 3), "hydrogen");
        assert_eq!(pole(&ratio(3, 1), &ratio(1, 1)), ratio(1, 2));
        assert_eq!(pole(&ratio(2, 1), &ratio(2, 1)), ratio(0, 1), "the throat");
        assert_eq!(
            pole(&ratio(0, 1), &ratio(1, 1)),
            ratio(-1, 1),
            "a pure Q pole"
        );
        assert_eq!(hemisphere(&ratio(0, 1), &ratio(0, 1)), ratio(1, 1));
        assert_eq!(
            hemisphere(&ratio(0, 1), &ratio(1, 8)),
            ratio(1, 2),
            "45 degrees"
        );
        assert_eq!(hemisphere(&ratio(0, 1), &ratio(1, 4)), ratio(0, 1));
        assert_eq!(hemisphere(&ratio(0, 1), &ratio(1, 2)), ratio(-1, 1));
        assert_eq!(
            hemisphere(&ratio(1, 16), &ratio(15, 16)),
            ratio(1, 2),
            "across 0"
        );
    }

    #[test]
    fn opposed_dipoles_lock_their_rates_and_aligned_ones_push_them_apart() {
        let (x, xbar, o) = (ratio(1, 5), ratio(1, 2), ratio(1, 2));
        // Opposed (σ·F < 0): the rate is drawn toward the range's.
        let opposed = field(&[Neighbour {
            holding: ratio(1, 1),
            pole: ratio(-1, 1),
            open: ratio(1, 2),
            nearness: ratio(2, 3),
            side: ratio(1, 1),
        }]);
        let one = ratio(1, 1);
        let locked = pulled(&x, &xbar, &one, &o, &opposed).expect("in the bound");
        assert_eq!(locked, ratio(23, 91));
        assert!(x < locked && locked < xbar);
        // Aligned (σ·F > 0): pushed away from it.
        let aligned = -opposed.clone();
        let split = pulled(&x, &xbar, &one, &o, &aligned).expect("in the bound");
        assert!(split < x);
        // A body alone in its range is untouched.
        assert_eq!(pulled(&x, &xbar, &one, &o, &field(&[])), Some(x.clone()));
        // At 45 degrees the opposed neighbour pulls by half: 41/181.
        let half = field(&[Neighbour {
            holding: ratio(1, 1),
            pole: ratio(-1, 1),
            open: ratio(1, 2),
            nearness: ratio(2, 3),
            side: ratio(1, 2),
        }]);
        assert_eq!(pulled(&x, &xbar, &one, &o, &half), Some(ratio(41, 181)));
    }

    #[test]
    fn the_flow_law_never_carries_a_rate_past_the_luminal_bound() {
        // The strongest field a range can hold, both signs, against rates
        // near the bound: every result stays within (−1, 1).
        let n = |pole: i64, side: i64| Neighbour {
            holding: ratio(3, 1),
            pole: ratio(pole, 1),
            open: ratio(1, 1),
            nearness: ratio(1, 1),
            side: ratio(side, 1),
        };
        let strongest = [field(&[n(1, 1), n(1, 1)]), field(&[n(-1, 1), n(1, -1)])];
        assert_eq!(strongest, [ratio(1, 1), ratio(-1, 1)]);
        for x in [ratio(9, 10), ratio(-9, 10), ratio(99, 100), ratio(0, 1)] {
            for xbar in [ratio(-99, 100), ratio(99, 100)] {
                for f in &strongest {
                    let y = pulled(&x, &xbar, &ratio(1, 1), &ratio(1, 1), f).expect("in the bound");
                    assert!(
                        y > ratio(-1, 1) && y < ratio(1, 1),
                        "{x} to {xbar} by {f}: {y}"
                    );
                }
            }
        }
    }
}
