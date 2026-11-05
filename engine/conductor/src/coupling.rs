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

use cfe::ratio::{ratio, Ratio};
use num_traits::{Signed, Zero};

/// A dipole's pole: the sign of its span `A − Q`, its closed frames against
/// its open (docs/121 §10.1).
pub fn pole(closed: &Ratio, open: &Ratio) -> i8 {
    match closed.cmp(open) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

/// The hemisphere law on one plane (docs/121 §10.2): two orientations,
/// in turns, aligned (`+1`) where they stand within 90° of each other,
/// opposed (`−1`) past 90°, uncoupled (`0`) at 90°.
pub fn hemisphere(a: &Ratio, b: &Ratio) -> i8 {
    let d = a.clone() - b.clone();
    let f = d.clone() - d.floor();
    let off = if f > ratio(1, 2) { ratio(1, 1) - f } else { f };
    match off.cmp(&ratio(1, 4)) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    }
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
    pub pole: i8,
    pub open: Ratio,
    pub nearness: Ratio,
    pub side: i8,
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
            * Ratio::from_integer((i64::from(n.pole) * i64::from(n.side)).into())
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
    own_pole: i8,
    own_open: &Ratio,
    f: &Ratio,
) -> Option<Ratio> {
    let toward = minus(range_rate, x)?;
    let y =
        -Ratio::from_integer(i64::from(own_pole).into()) * f.clone() * own_open.clone() * toward;
    crate::ring::compose(x, &y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pole_is_the_spans_sign_and_the_hemisphere_law_a_sign() {
        assert_eq!(pole(&ratio(3, 1), &ratio(1, 1)), 1);
        assert_eq!(pole(&ratio(1, 1), &ratio(3, 1)), -1);
        assert_eq!(pole(&ratio(2, 1), &ratio(2, 1)), 0);
        assert_eq!(hemisphere(&ratio(0, 1), &ratio(1, 8)), 1);
        assert_eq!(hemisphere(&ratio(0, 1), &ratio(3, 8)), -1);
        assert_eq!(hemisphere(&ratio(0, 1), &ratio(1, 4)), 0);
        assert_eq!(hemisphere(&ratio(1, 16), &ratio(15, 16)), 1, "across 0");
    }

    #[test]
    fn opposed_dipoles_lock_their_rates_and_aligned_ones_push_them_apart() {
        let (x, xbar, o) = (ratio(1, 5), ratio(1, 2), ratio(1, 2));
        // Opposed (σ·F < 0): the rate is drawn toward the range's.
        let opposed = field(&[Neighbour {
            holding: ratio(1, 1),
            pole: -1,
            open: ratio(1, 2),
            nearness: ratio(2, 3),
            side: 1,
        }]);
        let locked = pulled(&x, &xbar, 1, &o, &opposed).expect("in the bound");
        assert!(x < locked && locked < xbar);
        // Aligned (σ·F > 0): pushed away from it.
        let aligned = -opposed.clone();
        let split = pulled(&x, &xbar, 1, &o, &aligned).expect("in the bound");
        assert!(split < x);
        // A body alone in its range is untouched.
        assert_eq!(pulled(&x, &xbar, 1, &o, &field(&[])), Some(x.clone()));
    }

    #[test]
    fn the_flow_law_never_carries_a_rate_past_the_luminal_bound() {
        // The strongest field a range can hold, both signs, against rates
        // near the bound: every result stays within (−1, 1).
        let n = |pole: i8, side: i8| Neighbour {
            holding: ratio(3, 1),
            pole,
            open: ratio(1, 1),
            nearness: ratio(1, 1),
            side,
        };
        let strongest = [field(&[n(1, 1), n(1, 1)]), field(&[n(-1, 1), n(1, -1)])];
        assert_eq!(strongest, [ratio(1, 1), ratio(-1, 1)]);
        for x in [ratio(9, 10), ratio(-9, 10), ratio(99, 100), ratio(0, 1)] {
            for xbar in [ratio(-99, 100), ratio(99, 100)] {
                for f in &strongest {
                    let y = pulled(&x, &xbar, 1, &ratio(1, 1), f).expect("in the bound");
                    assert!(
                        y > ratio(-1, 1) && y < ratio(1, 1),
                        "{x} to {xbar} by {f}: {y}"
                    );
                }
            }
        }
    }
}
