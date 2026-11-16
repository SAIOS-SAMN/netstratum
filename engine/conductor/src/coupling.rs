// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The dipole coupling (docs/121 §9–§10): every body is a dipole on each
//! plane. Its pole and strength are LAW-COUPLING v2's (`flows::dipole`,
//! docs/129 §1: the push `1 − 2π` and the bond ends `U/3w`). Its
//! orientation is its turning on that plane. A body answers the field of its range as one: its neighbours'
//! dipoles, each by the ring law's nearness and the hemisphere law's sign.
//! The field moves its rate by the flow law, bounded by the luminal rate.
//! Attraction and repulsion are one law and its sign; nothing is halved,
//! squared or placed.
//!
//! Shear is the degree of modulation at every level (docs/121 §11): phase,
//! quantum and state. `pulled` composes any level's rate, a share of that
//! level's luminal bound; the lane it reads is the level.
//!
//! The spatial planes' rates are composed whole (docs/138 §5, #199):
//! `compose_whole` takes the rates and the pulls as one boost each, gives the
//! boost's part on each plane as the new rate, and reads the turning between
//! the planes (Thomas–Wigner) projectively.

use cfe::ratio::{ratio, Ratio};
use num_traits::{Signed, Zero};

/// The L1 circle's cosine (docs/118 F1, size L1; docs/61, the universe's
/// circle the L1 diamond, `8R` a turn): a point at `θ ∈ [0°, 180°]` from its
/// axis projects onto it by `1 − θ/90°`, first degree and exact.
pub fn cosine(theta: &Ratio) -> Ratio {
    ratio(1, 1) - theta.clone() / ratio(90, 1)
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
/// stands in `[−1, 1]`, so the field does. Each neighbour's term enters on
/// the grain (the carrier, never the law): its nearness carries its pair's
/// own denominator, and a sum over a range of thousands would compound them
/// without bound (the scalar collapse; register A28).
pub fn field(range: &[Neighbour]) -> Ratio {
    let whole = range.iter().fold(ratio(0, 1), |w, n| w + n.holding.clone());
    if whole.is_zero() {
        return ratio(0, 1);
    }
    range.iter().fold(ratio(0, 1), |f, n| {
        f + crate::ring::grain(
            &(n.holding.clone()
                * n.pole.clone()
                * n.side.clone()
                * n.open.clone()
                * n.nearness.clone()),
        )
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
    crate::ring::compose(x, &pull(x, range_rate, own_pole, own_open, f)?)
}

/// The pull on one plane, `y = −σ·F·o·(x̄ ⊖ x)`, before it is composed.
pub fn pull(
    x: &Ratio,
    range_rate: &Ratio,
    own_pole: &Ratio,
    own_open: &Ratio,
    f: &Ratio,
) -> Option<Ratio> {
    let toward = minus(range_rate, x)?;
    Some(-own_pole.clone() * f.clone() * own_open.clone() * toward)
}

/// The algebra of the rates (docs/138 §5): the three spatial axes and a time
/// unit `e₀`, squaring to `−1`. The rate on spatial plane `k` (in
/// `orientation::PLANES` order) is a boost along the axis dual to it,
/// `e₀·n_k`.
fn time_unit() -> usize {
    crate::turning::base_axis(4).expect("an axis")
}

fn boost_plane(k: usize) -> crate::turning::Clifford {
    use crate::turning::{base_axis, Clifford};
    let e0 = time_unit();
    Clifford::blade(e0, e0, ratio(1, 1))
        .product(&Clifford::blade(
            e0,
            base_axis(k + 1).expect("an axis"),
            ratio(1, 1),
        ))
        .expect("one signature")
}

fn coefficient(v: &crate::turning::Clifford, plane: &crate::turning::Clifford) -> Ratio {
    let Some((mask, unit)) = plane.terms().next() else {
        return ratio(0, 1);
    };
    v.terms()
        .find(|(m, _)| *m == mask)
        .map(|(_, c)| c.clone() / unit.clone())
        .unwrap_or_else(|| ratio(0, 1))
}

/// The rates `x` and the pulls `y` on the three spatial planes composed
/// whole, never plane by plane (docs/138 §5): `V′ = V(Y)·V(X)` with
/// `V(X) = 1 + Σ x_k·e₀n_k`, split against the mote's time `e₀`. The boost's
/// part on each plane is the new rate, and on one plane alone it is the flow
/// law `x ⊕ y`. The turning `1 + C` between the planes (Thomas–Wigner) is
/// read as its parts `c_p` on the spatial planes, in `orientation::PLANES`
/// order, with its norm `R·R̃ = 1 + |c|`-composed. None where the composition
/// leaves the luminal bound (a null versor).
pub fn compose_whole(x: &[Ratio], y: &[Ratio]) -> Option<(Vec<Ratio>, [Ratio; 3], Ratio)> {
    use crate::turning::{compose_versors, reversed, split, versor, Clifford};
    let e0 = time_unit();
    let rate = |v: &[Ratio]| {
        (0..3).try_fold(Clifford::blade(e0, 0, ratio(0, 1)), |s, k| {
            let c = v.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
            s.sum(&Clifford::blade(e0, 0, c).product(&boost_plane(k))?)
        })
    };
    let v = compose_versors(&versor(&rate(y).ok()?).ok()?, &versor(&rate(x).ok()?).ok()?).ok()?;
    let (boost, turning) = split(&v, &Clifford::blade(e0, e0, ratio(1, 1))).ok()?;
    let rates = (0..3)
        .map(|k| coefficient(&boost, &boost_plane(k)))
        .collect();
    let spatial: [Ratio; 3] = std::array::from_fn(|p| {
        let (i, j) = crate::orientation::PLANES[p];
        let plane = Clifford::blade(
            e0,
            crate::turning::base_axis(i).expect("an axis"),
            ratio(1, 1),
        )
        .product(&Clifford::blade(
            e0,
            crate::turning::base_axis(j).expect("an axis"),
            ratio(1, 1),
        ))
        .expect("one signature");
        coefficient(&turning, &plane)
    });
    let norm = turning
        .product(&reversed(&turning))
        .ok()?
        .terms()
        .find(|(m, _)| *m == 0)
        .map(|(_, c)| c.clone())?;
    Some((rates, spatial, norm))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// On one plane the whole composition is the flow law and turns nothing;
    /// on two it is docs/138's W2, `V(½ on n₂)·V(⅓ on n₁)`, the turning on
    /// the plane between them.
    #[test]
    fn rates_compose_whole_and_turn_the_plane_between() {
        let z = ratio(0, 1);
        let (rates, c, norm) = compose_whole(
            &[ratio(1, 3), z.clone(), z.clone()],
            &[ratio(1, 2), z.clone(), z.clone()],
        )
        .expect("in the bound");
        assert_eq!(rates, vec![ratio(5, 7), z.clone(), z.clone()]);
        assert!(c.iter().all(Zero::is_zero) && norm == ratio(1, 1));
        let (rates, c, norm) = compose_whole(
            &[ratio(1, 3), z.clone(), z.clone()],
            &[z.clone(), ratio(1, 2), z.clone()],
        )
        .expect("in the bound");
        // The pull composed after the rate, `V(Y)·V(X)`: the later motion on
        // the left (docs/138 V2).
        assert_eq!(rates, vec![ratio(9, 37), ratio(20, 37), z.clone()]);
        assert_eq!((c[0].clone(), c[1].clone()), (z.clone(), z.clone()));
        assert_eq!(c[2].abs(), ratio(1, 6), "the turning's half-rate is x₁·y₂");
        assert_eq!(norm, ratio(37, 36), "R·R̃ = 1 + c·c");
    }

    #[test]
    fn the_alignment_opens_by_degrees() {
        // The L1 circle's cosine, 1 - theta/90 degrees, exact.
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
