//! The composition of turnings (docs/117 §11.2–§11.4; LAW-COORDINATE
//! 78.173 fixture 11b, Mycelium's FORMS-TURNING v1), exact in the even
//! subalgebra of `Cl(p, q)` over ℚ, the signature carried as data (issue
//! #117).
//!
//! **Interim.** This mirrors Mycelium's `forms::turning` and the `algebra`
//! crate's Clifford value, which are on Mycelium's main but in no tagged
//! release (v0.3.0 predates them). Netstratum pins tags only, so it holds
//! this mirror, conformed by Mycelium's own fixtures (the tests below,
//! verbatim in their values), and deletes it for an import when v0.4.0 is
//! tagged, as `conductor::coordinate` does. `engine/substrate`'s multivector
//! holds one negative axis only (the conformal `e₋`) and cannot carry a
//! negative unit per plane.
//!
//! - **The rate is the generator.** A motion's rate is an even generator
//!   `B`, a bivector, its components exact on the planes and each plane's
//!   square read from the units that name it. Its motion is the Cayley form
//!   `R = (1 + B)(1 − B)⁻¹`, and `B = (R − 1)(R + 1)⁻¹` gives the generator
//!   back ([`motion`], [`generator`]); no trigonometric value enters.
//! - **Motions compose as `R₁R₂`** ([`compose`]), never plane by plane. Two
//!   planes sharing a unit gain the turning between them, of either sign by
//!   the order; disjoint planes commute and keep the grade-4 volume.
//! - **The shared form across the core tube** is the generator `B` with `K`
//!   per plane beside it (78.173 §8.4, settled 2026-11-07); each kernel
//!   keeps its own base. The degrees reading (`tan(θ/4) = b`, certified
//!   sectors) is not mirrored here yet.
//! - **The only inverse is the Cayley form's** (MU-RECURSION 78.32(1)): it
//!   solves `u·y = 1` exactly in the subalgebra `u`'s blades generate.
use cfe::ratio::Ratio;
use num_traits::{One, Signed, Zero};
use std::collections::BTreeMap;

/// The most base axes a value may name (Mycelium's `MAX_N`, the masks of
/// `base_axis`).
const MAX_AXES: usize = 14;

/// Base axis `i` (1-based) as its blade mask, as Mycelium's `base_axis`
/// numbers them: axes 1–3 on bits 0–2, axes from 4 on bits from 5 (bits 3
/// and 4 are the conformal pair). None for 0 or past the most.
pub fn base_axis(i: usize) -> Option<usize> {
    match i {
        0 => None,
        1..=3 => Some(1 << (i - 1)),
        _ if i <= MAX_AXES => Some(1 << (i + 1)),
        _ => None,
    }
}

/// An exact value of `Cl(p, q)` over ℚ: blades by mask, each with its
/// coefficient (no zero kept), and the mask of the units that square to
/// `−1`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clifford {
    negatives: usize,
    terms: BTreeMap<usize, Ratio>,
}

/// Why the composition of turnings refuses, by name and with what was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurningRefusal {
    /// A rate holding parts that are not a generator's: the blade masks of
    /// grade other than 2 (mod 4).
    NotGenerator { masks: Vec<usize> },
    /// A motion holding parts of odd grade: their blade masks.
    NotEven { masks: Vec<usize> },
    /// A motion `R` with `R·R̃ ≠ 1`: the terms of `R·R̃` as read.
    NotUnit { read: Vec<(usize, Ratio)> },
    /// The generator's pole: `1 − B` has no inverse.
    NoMotion,
    /// The motion's pole: `R + 1` has no inverse (`R = −1` among them).
    NoGenerator,
    /// Two values whose signatures disagree.
    Mismatched { first: usize, then: usize },
}

impl Clifford {
    /// The blade `c·e_mask` under the negative mask `negatives`.
    pub fn blade(negatives: usize, mask: usize, c: Ratio) -> Clifford {
        let mut terms = BTreeMap::new();
        if !c.is_zero() {
            terms.insert(mask, c);
        }
        Clifford { negatives, terms }
    }

    /// The negative mask: the units that square to `−1`.
    pub fn negatives(&self) -> usize {
        self.negatives
    }

    /// Its blades and coefficients, by mask ascending.
    pub fn terms(&self) -> impl Iterator<Item = (usize, &Ratio)> {
        self.terms.iter().map(|(m, c)| (*m, c))
    }

    fn agree(&self, other: &Clifford) -> Result<(), TurningRefusal> {
        match self.negatives == other.negatives {
            true => Ok(()),
            false => Err(TurningRefusal::Mismatched {
                first: self.negatives,
                then: other.negatives,
            }),
        }
    }

    fn put(terms: &mut BTreeMap<usize, Ratio>, mask: usize, c: Ratio) {
        let sum = terms.remove(&mask).unwrap_or_else(Ratio::zero) + c;
        if !sum.is_zero() {
            terms.insert(mask, sum);
        }
    }

    /// `self + other`.
    pub fn sum(&self, other: &Clifford) -> Result<Clifford, TurningRefusal> {
        self.agree(other)?;
        let mut terms = self.terms.clone();
        for (m, c) in &other.terms {
            Clifford::put(&mut terms, *m, c.clone());
        }
        Ok(Clifford {
            negatives: self.negatives,
            terms,
        })
    }

    /// `self − other`.
    pub fn difference(&self, other: &Clifford) -> Result<Clifford, TurningRefusal> {
        self.agree(other)?;
        let mut terms = self.terms.clone();
        for (m, c) in &other.terms {
            Clifford::put(&mut terms, *m, -c.clone());
        }
        Ok(Clifford {
            negatives: self.negatives,
            terms,
        })
    }

    /// The geometric product `self·other`: blades multiply to the
    /// exclusive-or of their masks, signed by the swaps that order them and
    /// by the square of every unit they share.
    pub fn product(&self, other: &Clifford) -> Result<Clifford, TurningRefusal> {
        self.agree(other)?;
        let mut terms = BTreeMap::new();
        for (a, ca) in &self.terms {
            for (b, cb) in &other.terms {
                let mut swaps = 0u32;
                let mut x = a >> 1;
                while x != 0 {
                    swaps += (x & b).count_ones();
                    x >>= 1;
                }
                swaps += (a & b & self.negatives).count_ones();
                let c = ca.clone() * cb.clone();
                Clifford::put(&mut terms, a ^ b, if swaps.is_multiple_of(2) { c } else { -c });
            }
        }
        Ok(Clifford {
            negatives: self.negatives,
            terms,
        })
    }
}

fn grade(mask: usize) -> u32 {
    mask.count_ones()
}

fn scalar(negatives: usize, c: Ratio) -> Clifford {
    Clifford::blade(negatives, 0, c)
}

/// The same value: one signature and the same terms.
pub fn same(a: &Clifford, b: &Clifford) -> bool {
    a == b
}

/// The reverse `ũ`: a blade of grade `g` takes the sign `(−1)^{g(g−1)/2}`.
pub fn reversed(u: &Clifford) -> Clifford {
    let terms = u
        .terms
        .iter()
        .map(|(m, c)| {
            let g = grade(*m);
            (
                *m,
                if (g * g.saturating_sub(1) / 2).is_multiple_of(2) {
                    c.clone()
                } else {
                    -c.clone()
                },
            )
        })
        .collect();
    Clifford {
        negatives: u.negatives,
        terms,
    }
}

/// The blades of the subalgebra `u` generates: the masks reached by
/// exclusive-or from `u`'s own, ascending (closed under the product).
fn span(u: &Clifford) -> Vec<usize> {
    let mut basis = vec![0usize];
    for mask in u.terms.keys() {
        if !basis.contains(mask) {
            let coset: Vec<usize> = basis.iter().map(|b| b ^ mask).collect();
            basis.extend(coset);
        }
    }
    basis.sort_unstable();
    basis
}

/// The exact inverse of `u` in the subalgebra its blades generate: the one
/// `y` with `u·y = 1`, by elimination over ℚ. None where `u` is a zero
/// divisor there.
fn inverse(u: &Clifford) -> Option<Clifford> {
    let basis = span(u);
    let n = basis.len();
    let mut rows: Vec<Vec<Ratio>> = vec![vec![Ratio::zero(); n + 1]; n];
    for (j, g) in basis.iter().enumerate() {
        let column = u
            .product(&Clifford::blade(u.negatives, *g, Ratio::one()))
            .ok()?;
        for (mask, c) in column.terms() {
            let i = basis.binary_search(&mask).ok()?;
            *rows.get_mut(i)?.get_mut(j)? = c.clone();
        }
    }
    *rows.first_mut()?.get_mut(n)? = Ratio::one();
    for col in 0..n {
        let pivot = (col..n).find(|&r| {
            rows.get(r)
                .and_then(|row| row.get(col))
                .is_some_and(|c| !c.is_zero())
        })?;
        rows.swap(col, pivot);
        let lead = rows.get(col)?.clone();
        let p = lead.get(col)?.clone();
        let lead: Vec<Ratio> = lead.iter().map(|c| c.clone() / p.clone()).collect();
        for (r, row) in rows.iter_mut().enumerate() {
            let f = row.get(col)?.clone();
            if r == col || f.is_zero() {
                continue;
            }
            row.iter_mut()
                .zip(&lead)
                .for_each(|(x, l)| *x -= f.clone() * l.clone());
        }
        *rows.get_mut(col)? = lead;
    }
    let mut y = scalar(u.negatives, Ratio::zero());
    for (j, g) in basis.iter().enumerate() {
        y = y
            .sum(&Clifford::blade(
                u.negatives,
                *g,
                rows.get(j)?.get(n)?.clone(),
            ))
            .ok()?;
    }
    Some(y)
}

fn masks_where(u: &Clifford, refused: impl Fn(u32) -> bool) -> Vec<usize> {
    u.terms
        .keys()
        .copied()
        .filter(|m| refused(grade(*m)))
        .collect()
}

/// The motion of the rate `B`, `R = (1 + B)(1 − B)⁻¹`, exact. Refused
/// `NotGenerator` where `B` holds a part of grade other than 2 (mod 4), and
/// `NoMotion` where `1 − B` has no inverse.
pub fn motion(rate: &Clifford) -> Result<Clifford, TurningRefusal> {
    let masks = masks_where(rate, |g| g % 4 != 2);
    if !masks.is_empty() {
        return Err(TurningRefusal::NotGenerator { masks });
    }
    let one = scalar(rate.negatives, Ratio::one());
    let held = inverse(&one.difference(rate)?).ok_or(TurningRefusal::NoMotion)?;
    one.sum(rate)?.product(&held)
}

/// `R` read as a motion: even, and a unit, `R·R̃ = 1`.
fn unit(r: &Clifford) -> Result<(), TurningRefusal> {
    let masks = masks_where(r, |g| g % 2 == 1);
    if !masks.is_empty() {
        return Err(TurningRefusal::NotEven { masks });
    }
    let norm = r.product(&reversed(r))?;
    match same(&norm, &scalar(r.negatives, Ratio::one())) {
        true => Ok(()),
        false => Err(TurningRefusal::NotUnit {
            read: norm.terms().map(|(m, c)| (m, c.clone())).collect(),
        }),
    }
}

/// The generator of the motion `R`, `B = (R − 1)(R + 1)⁻¹`, exact. Refused
/// `NotEven` or `NotUnit` where `R` is not a motion, and `NoGenerator`
/// where `R + 1` has no inverse.
pub fn generator(r: &Clifford) -> Result<Clifford, TurningRefusal> {
    unit(r)?;
    let one = scalar(r.negatives, Ratio::one());
    let held = inverse(&r.sum(&one)?).ok_or(TurningRefusal::NoGenerator)?;
    r.difference(&one)?.product(&held)
}

/// Two motions composed, `R₁R₂`, whole, never plane by plane.
pub fn compose(first: &Clifford, then: &Clifford) -> Result<Clifford, TurningRefusal> {
    unit(first)?;
    unit(then)?;
    first.product(then)
}

/// Two rates composed: the generator of `motion(B₁)·motion(B₂)`.
pub fn compose_rates(first: &Clifford, then: &Clifford) -> Result<Clifford, TurningRefusal> {
    generator(&compose(&motion(first)?, &motion(then)?)?)
}

/// A plane's regime: elliptic where its unit squares to `−1` (a turning),
/// hyperbolic where it squares to `+1` (a boost).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regime {
    Elliptic,
    Hyperbolic,
}

impl Regime {
    /// The regime of the plane `e`: one grade-2 blade at coefficient `±1`,
    /// its square read by the product.
    pub fn of(e: &Clifford) -> Result<Regime, TurningRefusal> {
        let refused = || TurningRefusal::NotGenerator {
            masks: e.terms.keys().copied().collect(),
        };
        match e.terms.iter().collect::<Vec<_>>().as_slice() {
            [(mask, c)] if mask.count_ones() == 2 && c.abs().is_one() => {}
            _ => return Err(refused()),
        }
        let square = e.product(e)?;
        match square.terms.iter().collect::<Vec<_>>().as_slice() {
            [(0, c)] if **c == -Ratio::one() => Ok(Regime::Elliptic),
            [(0, c)] if c.is_one() => Ok(Regime::Hyperbolic),
            _ => Err(refused()),
        }
    }
}

/// The motion of `b·e` on the one plane `e`, by the general law.
pub fn on_plane(b: &Ratio, e: &Clifford) -> Result<Clifford, TurningRefusal> {
    Regime::of(e)?;
    motion(&scalar(e.negatives, b.clone()).product(e)?)
}

/// Two rates on one plane composed, the regime's closed form: `(b₁ +
/// b₂)/(1 − b₁b₂)` elliptic, `(b₁ + b₂)/(1 + b₁b₂)` hyperbolic (the flow
/// law, LAW-DILATION). Refused `NoMotion` where a hyperbolic rate is `±1`,
/// `NoGenerator` where the denominator vanishes.
pub fn composed(regime: Regime, b1: &Ratio, b2: &Ratio) -> Result<Ratio, TurningRefusal> {
    let product = b1.clone() * b2.clone();
    let denominator = match regime {
        Regime::Elliptic => Ratio::one() - product,
        Regime::Hyperbolic => {
            if [b1, b2].iter().any(|b| (*b * *b).is_one()) {
                return Err(TurningRefusal::NoMotion);
            }
            Ratio::one() + product
        }
    };
    match denominator.is_zero() {
        true => Err(TurningRefusal::NoGenerator),
        false => Ok((b1.clone() + b2.clone()) / denominator),
    }
}

#[cfg(test)]
mod tests {
    //! LAW-COORDINATE fixture 11b and §8.1, conformed to Mycelium's
    //! `forms/src/tests/turning.rs` value for value. Plane `k`'s positive
    //! unit `e₊ₖ` (squaring to +1) is base axis `2k − 1`, its negative unit
    //! `e₋ₖ` (squaring to −1) base axis `2k`. Every motion is claimed both
    //! ways: `motion(generator(R)) = R` and `R·R̃ = 1`.
    use super::*;
    use cfe::ratio::ratio;

    fn q(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    fn unit(square: i8, k: usize) -> usize {
        match square {
            1 => base_axis(2 * k - 1),
            _ => base_axis(2 * k),
        }
        .expect("the algebra holds the unit")
    }

    fn signature() -> usize {
        (1..=3).map(|k| unit(-1, k)).fold(0, |s, g| s | g)
    }

    fn word(c: Ratio, units: &[(i8, usize)]) -> Clifford {
        let sig = signature();
        units.iter().fold(Clifford::blade(sig, 0, c), |w, (s, k)| {
            w.product(&Clifford::blade(sig, unit(*s, *k), q(1, 1)))
                .expect("one signature")
        })
    }

    fn sum(words: &[Clifford]) -> Clifford {
        words
            .iter()
            .fold(word(q(0, 1), &[]), |s, w| s.sum(w).expect("one signature"))
    }

    const P1: (i8, usize) = (1, 1);
    const P2: (i8, usize) = (1, 2);
    const P3: (i8, usize) = (1, 3);
    const N1: (i8, usize) = (-1, 1);
    const N2: (i8, usize) = (-1, 2);

    fn claim_motion(r: &Clifford) {
        let back = generator(r).expect("a generator");
        assert!(
            same(&motion(&back).expect("a motion"), r),
            "cayley(back(R)) = R"
        );
        assert!(
            same(
                &r.product(&reversed(r)).expect("product"),
                &word(q(1, 1), &[])
            ),
            "R·R̃ = 1"
        );
    }

    fn volume(r: &Clifford) -> Vec<usize> {
        r.terms()
            .map(|(m, _)| m)
            .filter(|m| m.count_ones() == 4)
            .collect()
    }

    /// Fixture 11b-i, planes sharing a unit: the composite gains the
    /// turning between them, of the opposite sign in the other order, and
    /// holds no volume.
    #[test]
    fn planes_sharing_a_unit_gain_the_turning_between_them_and_no_volume() {
        let (b1, b2) = (word(q(1, 2), &[P1, P2]), word(q(1, 3), &[P2, P3]));
        let (r1, r2) = (
            motion(&b1).expect("a motion"),
            motion(&b2).expect("a motion"),
        );
        assert!(same(
            &r1,
            &sum(&[word(q(3, 5), &[]), word(q(4, 5), &[P1, P2])])
        ));
        assert!(same(
            &r2,
            &sum(&[word(q(4, 5), &[]), word(q(3, 5), &[P2, P3])])
        ));
        let one_way = compose(&r1, &r2).expect("composes");
        let other_way = compose(&r2, &r1).expect("composes");
        let expect = |s: i64| {
            sum(&[
                word(q(12, 25), &[]),
                word(q(16, 25), &[P1, P2]),
                word(q(s * 12, 25), &[P1, P3]),
                word(q(9, 25), &[P2, P3]),
            ])
        };
        assert!(same(&one_way, &expect(1)), "R₁R₂: +12/25 on e₊₁e₊₃");
        assert!(same(&other_way, &expect(-1)), "R₂R₁: −12/25 on e₊₁e₊₃");
        let back = |s: i64| {
            sum(&[
                word(q(16, 37), &[P1, P2]),
                word(q(s * 12, 37), &[P1, P3]),
                word(q(9, 37), &[P2, P3]),
            ])
        };
        assert!(same(&generator(&one_way).expect("a generator"), &back(1)));
        assert!(same(
            &generator(&other_way).expect("a generator"),
            &back(-1)
        ));
        assert!(same(&compose_rates(&b1, &b2).expect("composes"), &back(1)));
        assert!(same(
            &one_way.difference(&other_way).expect("one signature"),
            &word(q(24, 25), &[P1, P3])
        ));
        assert!(volume(&one_way).is_empty() && volume(&other_way).is_empty());
        [&r1, &r2, &one_way, &other_way]
            .into_iter()
            .for_each(claim_motion);
    }

    /// Fixture 11b-ii, disjoint planes: the motions commute, the composite
    /// keeps the volume, and its generator reads `4/7` and `3/7` per plane,
    /// never the `½` and `⅓` it was composed from; recomposed plane by
    /// plane it is another motion.
    #[test]
    fn disjoint_planes_commute_and_keep_the_volume() {
        let (b1, b2) = (word(q(1, 2), &[P1, P2]), word(q(1, 3), &[N1, N2]));
        let (r1, r2) = (
            motion(&b1).expect("a motion"),
            motion(&b2).expect("a motion"),
        );
        let one_way = compose(&r1, &r2).expect("composes");
        let expect = sum(&[
            word(q(12, 25), &[]),
            word(q(16, 25), &[P1, P2]),
            word(q(9, 25), &[N1, N2]),
            word(q(12, 25), &[P1, P2, N1, N2]),
        ]);
        assert!(same(&one_way, &expect));
        assert!(same(&compose(&r2, &r1).expect("composes"), &expect));
        let back = generator(&one_way).expect("a generator");
        assert!(same(
            &back,
            &sum(&[word(q(4, 7), &[P1, P2]), word(q(3, 7), &[N1, N2])])
        ));
        assert!(!same(&back, &b1.sum(&b2).expect("one signature")));
        let piecewise = compose(
            &motion(&word(q(4, 7), &[P1, P2])).expect("a motion"),
            &motion(&word(q(3, 7), &[N1, N2])).expect("a motion"),
        )
        .expect("composes");
        let read_apart = sum(&[
            word(q(660, 1885), &[]),
            word(q(1120, 1885), &[P1, P2]),
            word(q(693, 1885), &[N1, N2]),
            word(q(1176, 1885), &[P1, P2, N1, N2]),
        ]);
        assert!(same(&piecewise, &read_apart) && !same(&piecewise, &expect));
        assert!(same(&motion(&back).expect("a motion"), &expect));
        [&r1, &r2, &one_way].into_iter().for_each(claim_motion);
    }

    /// Fixture 11b-iii, one plane: on the elliptic `e₊₁e₊₂`, `½` then `⅓`
    /// is `1`; on the hyperbolic `e₊₁e₋₁`, `5/7` (the flow law).
    #[test]
    fn one_plane_composes_by_its_regime() {
        for (plane, regime, expect) in [
            (word(q(1, 1), &[P1, P2]), Regime::Elliptic, q(1, 1)),
            (word(q(1, 1), &[P1, N1]), Regime::Hyperbolic, q(5, 7)),
        ] {
            assert_eq!(Regime::of(&plane), Ok(regime));
            assert_eq!(composed(regime, &q(1, 2), &q(1, 3)), Ok(expect.clone()));
            let rate = |b: Ratio| word(b, &[]).product(&plane).expect("one signature");
            let general = compose_rates(&rate(q(1, 2)), &rate(q(1, 3))).expect("composes");
            assert!(same(&general, &rate(expect)));
        }
    }

    /// One plane's motion in closed form, for rates of either sign, and
    /// every pair's composition agreeing with the general law.
    #[test]
    fn closed_forms_on_one_plane_agree_with_the_general_law() {
        let rates = [q(1, 2), q(-1, 3), q(2, 7), q(5, 3), q(0, 1)];
        for (plane, sign) in [
            (word(q(1, 1), &[P2, P3]), -1i64),
            (word(q(1, 1), &[P2, N2]), 1),
        ] {
            let regime = Regime::of(&plane).expect("a plane");
            for b in &rates {
                let bb = b.clone() * b.clone();
                let (sc, den) = (q(1, 1) + q(sign, 1) * bb.clone(), q(1, 1) - q(sign, 1) * bb);
                let Ok(r) = on_plane(b, &plane) else {
                    assert!(den == q(0, 1), "refused only at the pole");
                    continue;
                };
                let expect = sum(&[
                    word(sc / den.clone(), &[]),
                    word(q(2, 1) * b.clone() / den, &[])
                        .product(&plane)
                        .expect("one signature"),
                ]);
                assert!(same(&r, &expect), "{regime:?} at {b}");
                claim_motion(&r);
                for c in &rates {
                    let rate =
                        |x: &Ratio| word(x.clone(), &[]).product(&plane).expect("one signature");
                    match (composed(regime, b, c), compose_rates(&rate(b), &rate(c))) {
                        (Ok(x), Ok(general)) => assert!(same(&general, &rate(&x))),
                        (Err(a), Err(g)) => assert_eq!(a, g),
                        (a, g) => panic!("{regime:?}: {b} then {c}: {a:?} against {g:?}"),
                    }
                }
            }
        }
    }

    /// The refusals, as Mycelium names them.
    #[test]
    fn turning_inverse_refuses_nonexistent() {
        assert_eq!(
            generator(&word(q(-1, 1), &[])).map(|_| ()),
            Err(TurningRefusal::NoGenerator)
        );
        assert_eq!(
            motion(&word(q(1, 1), &[P1, N1])).map(|_| ()),
            Err(TurningRefusal::NoMotion)
        );
        assert_eq!(
            composed(Regime::Elliptic, &q(1, 1), &q(1, 1)),
            Err(TurningRefusal::NoGenerator)
        );
        assert_eq!(
            composed(Regime::Hyperbolic, &q(1, 1), &q(1, 3)),
            Err(TurningRefusal::NoMotion)
        );
        assert_eq!(
            generator(&word(q(1, 1), &[P1])).map(|_| ()),
            Err(TurningRefusal::NotEven {
                masks: vec![unit(1, 1)]
            })
        );
        assert!(matches!(
            generator(&word(q(2, 1), &[])),
            Err(TurningRefusal::NotUnit { .. })
        ));
        assert_eq!(
            motion(&sum(&[word(q(1, 1), &[]), word(q(1, 2), &[P1, P2])])).map(|_| ()),
            Err(TurningRefusal::NotGenerator { masks: vec![0] })
        );
        assert_eq!(
            Regime::of(&word(q(2, 1), &[P1, P2])),
            Err(TurningRefusal::NotGenerator {
                masks: vec![unit(1, 1) | unit(1, 2)]
            })
        );
    }
}
