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
//! - **Rates as versors** (docs/138, #206). The same motion read up to a
//!   scalar is the versor of the rate bivector `B`, whole on every plane it
//!   moves, each plane at its own angle: `V = Σ W_m`, its wedge powers,
//!   `1 + B` on one plane ([`versor`], [`projective`]). The planes and
//!   their angles are read back exactly ([`invariants`], [`plane_of`]). The Cayley motion of `b·e` reads, on a
//!   boost plane, `1 + (2b/(1 + b·b))·e`: one model, two readings.
//!   Versors compose by the geometric product ([`compose_versors`]); a
//!   power is the law composed with itself ([`power`]). Two boosts on
//!   planes sharing the time unit leave a boost times a turning on the
//!   plane between them ([`split`], W2), boost and turning read against an
//!   observer's own time ([`parts`]), never by the base axes, and a path that closes on paper
//!   leaves a residue in the algebra ([`holonomy`], W4).
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
    /// A versor with no scalar part, which no half-rate reads: `1 + β·e`
    /// at `β = ±1` on a boost plane is null (`V·Ṽ = 0`), and a product of
    /// versors can land there.
    Null,
    /// A time that is not a vector squaring to a negative scalar: its
    /// blade masks.
    NotTime { masks: Vec<usize> },
    /// Two planes of a rate at one angle: its parts are not unique there.
    Isoclinic,
    /// An angle that is none of the rate's: no part squares to it.
    NotAngle,
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
                Clifford::put(
                    &mut terms,
                    a ^ b,
                    if swaps.is_multiple_of(2) { c } else { -c },
                );
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

/// `a ∧ b`: the product's blades whose masks are disjoint (exact for the
/// blades of a bivector's wedge powers).
fn outer(a: &Clifford, b: &Clifford) -> Result<Clifford, TurningRefusal> {
    a.agree(b)?;
    let mut terms = BTreeMap::new();
    for (x, cx) in &a.terms {
        for (y, cy) in &b.terms {
            if x & y == 0 {
                let blade = Clifford::blade(a.negatives, *x, cx.clone())
                    .product(&Clifford::blade(a.negatives, *y, cy.clone()))?;
                for (m, c) in blade.terms {
                    Clifford::put(&mut terms, m, c);
                }
            }
        }
    }
    Ok(Clifford {
        negatives: a.negatives,
        terms,
    })
}

/// The rate's wedge powers `W_m = B^{∧m}/m!`, from `W₀ = 1` to the last
/// that is not zero. For `B = Σ B_k`, its parts on orthogonal commuting
/// planes, `W_m` is the sum of the products of `m` distinct parts; the
/// count past `W₀` is the number of planes, `K`.
pub fn wedge_powers(rate: &Clifford) -> Result<Vec<Clifford>, TurningRefusal> {
    let masks = masks_where(rate, |g| g != 2);
    if !masks.is_empty() {
        return Err(TurningRefusal::NotGenerator { masks });
    }
    let mut out = vec![scalar(rate.negatives, Ratio::one())];
    while let Some(last) = out.last() {
        let m = Ratio::from_integer(out.len().into());
        let next = scalar(rate.negatives, Ratio::one() / m).product(&outer(last, rate)?)?;
        if next.terms.is_empty() {
            break;
        }
        out.push(next);
    }
    Ok(out)
}

/// The versor of the rate `B` (docs/138 V1): the rate is the bivector
/// itself, on every plane it moves at once, each plane at its own angle,
/// its coefficient there that plane's half-rate. For `B = Σ B_k` on
/// orthogonal commuting planes, `V = Π(1 + B_k) = Σ W_m`, the wedge
/// powers: exact with no plane read apart. One plane gives `1 + B`.
/// Refused `NotGenerator` where `B` holds a part that is not a bivector,
/// `Null` where `V·Ṽ = Π(1 − B_k·B_k)` vanishes.
pub fn versor(rate: &Clifford) -> Result<Clifford, TurningRefusal> {
    let v = wedge_powers(rate)?
        .iter()
        .try_fold(scalar(rate.negatives, Ratio::zero()), |s, w| s.sum(w))?;
    match v.product(&reversed(&v))?.terms.is_empty() {
        true => Err(TurningRefusal::Null),
        false => Ok(v),
    }
}

/// The rate's invariants: `e_m`, the scalar part of `W_m·W_m`, which is the
/// `m`-th elementary symmetric function of the planes' `μ_k = B_k·B_k`. The
/// angles are the roots of `Σ (−1)^m e_m λ^{K−m}`, held by these
/// coefficients where a root is not rational.
pub fn invariants(rate: &Clifford) -> Result<Vec<Ratio>, TurningRefusal> {
    wedge_powers(rate)?
        .iter()
        .map(|w| {
            Ok(w.product(w)?
                .terms
                .get(&0)
                .cloned()
                .unwrap_or_else(Ratio::zero))
        })
        .collect()
}

/// The rate's part on the plane whose angle is `μ = B_k·B_k`, exact:
///
///     B_k = (Σ_m μ^{K−1−m} W_{2m+1}) · (Σ_m μ^{K−1−m} W_{2m})⁻¹
///
/// since `W_m(B) = W_m(C) + B_k·W_{m−1}(C)`, with `C` the other planes, makes
/// `B_k` times the second sum the first. Refused `Isoclinic` where another
/// plane shares the angle (the second sum has no inverse), `NotAngle` where
/// `μ` is not one of the angles.
pub fn plane_of(rate: &Clifford, mu: &Ratio) -> Result<Clifford, TurningRefusal> {
    let w = wedge_powers(rate)?;
    let k = w.len() - 1;
    let zero = || scalar(rate.negatives, Ratio::zero());
    let (mut odd, mut even) = (zero(), zero());
    for m in 0..k {
        let weight = scalar(
            rate.negatives,
            (0..k - 1 - m).fold(Ratio::one(), |p, _| p * mu.clone()),
        );
        if let Some(x) = w.get(2 * m + 1) {
            odd = odd.sum(&weight.product(x)?)?;
        }
        if let Some(x) = w.get(2 * m) {
            even = even.sum(&weight.product(x)?)?;
        }
    }
    let part = odd.product(&inverse(&even).ok_or(TurningRefusal::Isoclinic)?)?;
    let square = part.product(&part)?;
    let simple = masks_where(&part, |g| g != 2).is_empty()
        && same(&square, &scalar(rate.negatives, mu.clone()))
        && !part.terms.is_empty();
    match simple {
        true => Ok(part),
        false => Err(TurningRefusal::NotAngle),
    }
}

/// `v` read up to a scalar: divided by its scalar part, so each plane's
/// coefficient is its half-rate. Refused `Null` where the scalar part is 0.
pub fn projective(v: &Clifford) -> Result<Clifford, TurningRefusal> {
    let s = v.terms.get(&0).cloned().ok_or(TurningRefusal::Null)?;
    scalar(v.negatives, Ratio::one() / s).product(v)
}

/// Two versors composed, `first·then` read up to a scalar (docs/138 V2):
/// the geometric product, never plane by plane.
pub fn compose_versors(first: &Clifford, then: &Clifford) -> Result<Clifford, TurningRefusal> {
    projective(&first.product(then)?)
}

/// `vⁿ` read up to a scalar: the versor composed with itself `n` times, by
/// squarings (`⌈log₂ n⌉` of them and one product per set bit), exact.
pub fn power(v: &Clifford, n: u64) -> Result<Clifford, TurningRefusal> {
    let (mut out, mut base, mut k) = (scalar(v.negatives, Ratio::one()), projective(v)?, n);
    while k != 0 {
        if k & 1 == 1 {
            out = compose_versors(&out, &base)?;
        }
        k >>= 1;
        if k != 0 {
            base = compose_versors(&base, &base)?;
        }
    }
    Ok(out)
}

/// `t` read as a time: a vector squaring to a negative scalar, with its
/// inverse. Refused `NotTime` with the masks otherwise.
fn time_of(t: &Clifford) -> Result<Clifford, TurningRefusal> {
    let refused = || TurningRefusal::NotTime {
        masks: t.terms.keys().copied().collect(),
    };
    if t.terms.is_empty() || t.terms.keys().any(|m| grade(*m) != 1) {
        return Err(refused());
    }
    let square = t.product(t)?;
    match square.terms.iter().collect::<Vec<_>>().as_slice() {
        [(0, c)] if c.is_negative() => inverse(t).ok_or_else(refused),
        _ => Err(refused()),
    }
}

/// The rate `B` parted against the time `t`, by the algebra and never by
/// the blades' masks: the turning `½(B + tBt⁻¹)` commutes with `t`, the
/// boost `½(B − tBt⁻¹)` anticommutes with it. Which planes boost and which
/// turn is the observer's, read from its own time.
pub fn parts(rate: &Clifford, t: &Clifford) -> Result<(Clifford, Clifford), TurningRefusal> {
    let inv = time_of(t)?;
    let mirrored = t.product(rate)?.product(&inv)?;
    let half = scalar(rate.negatives, Ratio::one() / Ratio::from_integer(2.into()));
    Ok((
        half.product(&rate.difference(&mirrored)?)?,
        half.product(&rate.sum(&mirrored)?)?,
    ))
}

/// `v` split as a boost times a turning against the time `t`, `V ∝ B·R`
/// (docs/138 W2). `V` carries `t` to `t′ = V·t·Ṽ/(V·Ṽ)`; the boost is the
/// one motion in the plane of `t` and `t′` that does, `B ∝ 1 + t′t⁻¹`; the
/// turning is `R ∝ B̃·V`, which keeps `t`. Orthogonality is read from the
/// observer's time, not from the base axes. Refused `NotTime` where `t` is
/// not a time, `Null` where `t′ = −t`.
pub fn split(v: &Clifford, t: &Clifford) -> Result<(Clifford, Clifford), TurningRefusal> {
    let inv = time_of(t)?;
    let v = projective(v)?;
    let norm = v.product(&reversed(&v))?;
    let carried = v
        .product(t)?
        .product(&reversed(&v))?
        .product(&inverse(&norm).ok_or(TurningRefusal::Null)?)?;
    let boost = projective(&scalar(v.negatives, Ratio::one()).sum(&carried.product(&inv)?)?)?;
    let turning = projective(&reversed(&boost).product(&v)?)?;
    Ok((boost, turning))
}

/// `x` carried by the motion `v`: `V·x·Ṽ/(V·Ṽ)`, exact (docs/138 §3.1). A
/// spin carried by a turning moves between the planes it touches, its angles
/// kept (`invariants`). Refused `Null` where `V·Ṽ` has no inverse.
pub fn carry(v: &Clifford, x: &Clifford) -> Result<Clifford, TurningRefusal> {
    let norm = v.product(&reversed(v))?;
    let held = inverse(&norm).ok_or(TurningRefusal::Null)?;
    v.product(x)?.product(&reversed(v))?.product(&held)
}

/// The residue of the path `first → then → first⁻¹ → then⁻¹`,
/// `then⁻¹·first⁻¹·then·first` read up to a scalar (docs/138 W4), each
/// inverse the reverse (`V·Ṽ` is a scalar). It is `1` only where the two
/// commute; otherwise the path does not close in the algebra.
pub fn holonomy(first: &Clifford, then: &Clifford) -> Result<Clifford, TurningRefusal> {
    compose_versors(
        &compose_versors(&reversed(then), &reversed(first))?,
        &compose_versors(then, first)?,
    )
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

#[cfg(test)]
mod versor_tests {
    //! docs/138's fixtures, exact in `Cl(2,1)` on `e₀, e₁, e₂`: `e₀` (base
    //! axis 1) squares to `−1`, `e₁` and `e₂` to `+1`. The boost planes are
    //! `e₀₁` and `e₀₂`, the turning plane between them `e₁₂`.
    use super::*;
    use cfe::ratio::ratio;

    const SIG: usize = 1;
    const E01: usize = 0b011;
    const E02: usize = 0b101;
    const E12: usize = 0b110;

    fn q(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    fn plane(mask: usize) -> Clifford {
        Clifford::blade(SIG, mask, q(1, 1))
    }

    fn value(parts: &[(usize, Ratio)]) -> Clifford {
        parts
            .iter()
            .fold(Clifford::blade(SIG, 0, q(0, 1)), |s, (m, c)| {
                s.sum(&Clifford::blade(SIG, *m, c.clone()))
                    .expect("one signature")
            })
    }

    fn boost(beta: Ratio, mask: usize) -> Clifford {
        versor(&Clifford::blade(SIG, mask, beta)).expect("a versor")
    }

    fn time() -> Clifford {
        Clifford::blade(SIG, 0b001, q(1, 1))
    }

    fn commutes(a: &Clifford, b: &Clifford) -> bool {
        a.product(b).expect("one signature") == b.product(a).expect("one signature")
    }

    fn pow(x: &Ratio, n: u64) -> Ratio {
        (0..n).fold(q(1, 1), |p, _| p * x.clone())
    }

    /// W1: on one plane the versors compose by the flow law, `⅓` then `½`
    /// is `5/7`; n of them is `((1 + β)ⁿ − (1 − β)ⁿ)/((1 + β)ⁿ + (1 − β)ⁿ)`;
    /// and the Cayley motion of `b` reads the half-rate `2b/(1 + b·b)`.
    #[test]
    fn parallel_boosts_compose_by_the_flow_law() {
        let one = compose_versors(&boost(q(1, 3), E01), &boost(q(1, 2), E01)).expect("composes");
        assert_eq!(one, value(&[(0, q(1, 1)), (E01, q(5, 7))]));
        let beta = q(1, 3);
        for n in [7u64, 28] {
            let (up, down) = (
                pow(&(q(1, 1) + beta.clone()), n),
                pow(&(q(1, 1) - beta.clone()), n),
            );
            let flow = (up.clone() - down.clone()) / (up + down);
            assert_eq!(
                power(&boost(beta.clone(), E01), n).expect("a power"),
                value(&[(0, q(1, 1)), (E01, flow)])
            );
        }
        for b in [q(1, 2), q(-2, 7), q(3, 5)] {
            let cayley = on_plane(&b, &plane(E01)).expect("a motion");
            let read = q(2, 1) * b.clone() / (q(1, 1) + b.clone() * b);
            assert_eq!(projective(&cayley).expect("a versor"), boost(read, E01));
        }
    }

    /// W2: `(1 + ⅓e₀₁)(1 + ½e₀₂) = 1 + ⅓e₀₁ + ½e₀₂ + ⅙e₁₂`, a boost
    /// `1 + (15/37)e₀₁ + (16/37)e₀₂` times the turning `1 + ⅙e₁₂`, the
    /// turning's half-rate `β₁β₂`; and the product reads the same through
    /// the unit Cayley motions.
    #[test]
    fn non_parallel_boosts_turn_the_plane_between_them() {
        let (v1, v2) = (boost(q(1, 3), E01), boost(q(1, 2), E02));
        let v = compose_versors(&v1, &v2).expect("composes");
        assert_eq!(
            v,
            value(&[(0, q(1, 1)), (E01, q(1, 3)), (E02, q(1, 2)), (E12, q(1, 6))])
        );
        let (b, r) = split(&v, &time()).expect("splits");
        assert_eq!(
            b,
            value(&[(0, q(1, 1)), (E01, q(15, 37)), (E02, q(16, 37))])
        );
        assert_eq!(r, value(&[(0, q(1, 1)), (E12, q(1, 6))]));
        assert_eq!(compose_versors(&b, &r).expect("composes"), v, "V ∝ B·R");
        // One rate on both planes at once is another motion: no turning.
        let both = Clifford::blade(SIG, E01, q(1, 3))
            .sum(&Clifford::blade(SIG, E02, q(1, 2)))
            .expect("one signature");
        let (b0, r0) = split(&versor(&both).expect("a versor"), &time()).expect("splits");
        assert_eq!(
            (b0, r0),
            (versor(&both).expect("a versor"), value(&[(0, q(1, 1))]))
        );
        // The per-plane law books the boost planes alone and drops e₁₂.
        let (m1, m2) = (
            on_plane(&q(1, 5), &plane(E01)).expect("a motion"),
            on_plane(&q(1, 4), &plane(E02)).expect("a motion"),
        );
        assert_eq!(
            projective(&compose(&m1, &m2).expect("composes")).expect("a versor"),
            compose_versors(
                &projective(&m1).expect("a versor"),
                &projective(&m2).expect("a versor")
            )
            .expect("composes")
        );
    }

    /// W4: the path `V₁ → V₂ → V₁⁻¹ → V₂⁻¹` leaves `1 − (3/11)e₀₁ +
    /// (2/11)e₀₂ − (6/11)e₁₂`, a boost and a turning; on one plane it
    /// closes.
    #[test]
    fn a_path_closed_on_paper_leaves_a_boost_and_a_turning() {
        let (v1, v2) = (boost(q(1, 3), E01), boost(q(1, 2), E02));
        let residue = holonomy(&v1, &v2).expect("composes");
        assert_eq!(
            residue,
            value(&[
                (0, q(1, 1)),
                (E01, q(-3, 11)),
                (E02, q(2, 11)),
                (E12, q(-6, 11))
            ])
        );
        let (b, r) = split(&residue, &time()).expect("splits");
        assert_eq!(compose_versors(&b, &r).expect("composes"), residue);
        assert!(b.terms().all(|(m, _)| m != E12) && r.terms().any(|(m, _)| m == E12));
        let closed = holonomy(&v1, &boost(q(1, 2), E01)).expect("composes");
        assert_eq!(closed, value(&[(0, q(1, 1))]), "parallel boosts commute");
    }

    /// A power at N and at 4N: `V^{4N} = (V^N)⁴`, and both split into a
    /// boost and a turning on `e₁₂`.
    #[test]
    fn a_power_at_n_and_4n_rederives() {
        let v = compose_versors(&boost(q(1, 3), E01), &boost(q(1, 2), E02)).expect("composes");
        let n = 7u64;
        let far = power(&v, 4 * n).expect("a power");
        assert_eq!(
            far,
            power(&power(&v, n).expect("a power"), 4).expect("a power")
        );
        let stepped = (0..4 * n).try_fold(value(&[(0, q(1, 1))]), |p, _| compose_versors(&p, &v));
        assert_eq!(stepped.expect("composes"), far);
        let (b, r) = split(&far, &time()).expect("splits");
        assert_eq!(compose_versors(&b, &r).expect("composes"), far);
    }

    /// Orthogonality is the observer's: against a moving observer's time
    /// `t = W·e₀·W̃` the same `V` splits into another boost and turning,
    /// each mixing blades the base axes would part; the boost carries `t`
    /// to `V·t·Ṽ`, the turning keeps `t`, and `V ∝ B·R` either way. The
    /// rate's parts commute and anticommute with `t`.
    #[test]
    fn orthogonality_is_read_from_the_observers_time() {
        let v = compose_versors(&boost(q(1, 3), E01), &boost(q(1, 2), E02)).expect("composes");
        let w = boost(q(1, 4), E01);
        let moving = w
            .product(&time())
            .and_then(|x| x.product(&reversed(&w)))
            .expect("one signature");
        assert_eq!(moving, value(&[(0b001, q(17, 16)), (0b010, q(1, 2))]));
        let (b, r) = split(&v, &moving).expect("splits");
        assert_eq!(
            b,
            value(&[
                (0, q(1, 1)),
                (E01, q(2399, 8149)),
                (E02, q(4624, 8149)),
                (E12, q(2176, 8149))
            ])
        );
        assert_eq!(
            r,
            value(&[(0, q(1, 1)), (E02, q(-28, 675)), (E12, q(-119, 1350))])
        );
        assert_eq!(compose_versors(&b, &r).expect("composes"), v);
        assert!(
            commutes(&r, &moving),
            "the turning keeps the observer's time"
        );
        let carried = |x: &Clifford| {
            projective(
                &x.product(&moving)
                    .and_then(|y| y.product(&reversed(x)))
                    .expect("one signature")
                    .product(&moving)
                    .expect("one signature"),
            )
            .expect("a versor")
        };
        assert_eq!(carried(&b), carried(&v), "the boost carries t where V does");
        let rate = value(&[(E01, q(1, 3)), (E02, q(1, 2)), (E12, q(1, 6))]);
        for t in [time(), moving] {
            let (boosting, turning) = parts(&rate, &t).expect("parts");
            assert_eq!(boosting.sum(&turning).expect("one signature"), rate);
            assert!(commutes(&turning, &t));
            let anti = boosting
                .product(&t)
                .and_then(|x| x.sum(&t.product(&boosting)?))
                .expect("one signature");
            assert_eq!(anti, value(&[]), "the boost anticommutes with t");
        }
        let (boosting, turning) = parts(&rate, &time()).expect("parts");
        assert_eq!(boosting, value(&[(E01, q(1, 3)), (E02, q(1, 2))]));
        assert_eq!(turning, value(&[(E12, q(1, 6))]));
    }

    #[test]
    fn versors_refuse_by_name() {
        let rate = |m: usize, c: Ratio| Clifford::blade(SIG, m, c);
        assert_eq!(versor(&rate(E01, q(1, 1))), Err(TurningRefusal::Null));
        assert!(
            versor(&rate(E12, q(1, 1))).is_ok(),
            "a turning is never null"
        );
        assert!(matches!(
            versor(&rate(0b111, q(1, 2))),
            Err(TurningRefusal::NotGenerator { .. })
        ));
        assert_eq!(projective(&plane(E01)), Err(TurningRefusal::Null));
        assert!(matches!(
            split(&boost(q(1, 3), E01), &plane(0b010)),
            Err(TurningRefusal::NotTime { .. })
        ));
        assert!(matches!(
            split(&boost(q(1, 3), E01), &plane(E01)),
            Err(TurningRefusal::NotTime { .. })
        ));
    }
}

#[cfg(test)]
mod angle_tests {
    //! A rate on several planes at once, each at its own angle, in `Cl(6,0)`
    //! and in `Cl(3,1)`. The planes are turned off the base axes by a
    //! motion so no mask names them.
    use super::*;
    use cfe::ratio::ratio;

    fn q(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    fn e(sig: usize, axes: &[usize], c: Ratio) -> Clifford {
        axes.iter().fold(Clifford::blade(sig, 0, c), |w, i| {
            w.product(&Clifford::blade(
                sig,
                base_axis(*i).expect("an axis"),
                Ratio::one(),
            ))
            .expect("one signature")
        })
    }

    fn one(sig: usize) -> Clifford {
        Clifford::blade(sig, 0, Ratio::one())
    }

    fn total(parts: &[Clifford]) -> Clifford {
        parts
            .iter()
            .skip(1)
            .fold(parts[0].clone(), |s, p| s.sum(p).expect("one signature"))
    }

    /// Three planes at `½`, `⅓`, `⅕`, turned off the axes by
    /// `(1 + ½e₂₃)(1 + ⅓e₄₅)(1 + ¼e₁₆)`.
    fn three() -> Vec<Clifford> {
        let t = [
            e(0, &[2, 3], q(1, 2)),
            e(0, &[4, 5], q(1, 3)),
            e(0, &[1, 6], q(1, 4)),
        ]
        .iter()
        .try_fold(one(0), |w, b| w.product(&one(0).sum(b)?))
        .expect("a motion");
        let inv = inverse(&t).expect("a motion");
        [
            e(0, &[1, 2], q(1, 2)),
            e(0, &[3, 4], q(1, 3)),
            e(0, &[5, 6], q(1, 5)),
        ]
        .iter()
        .map(|b| {
            t.product(b)
                .and_then(|x| x.product(&inv))
                .expect("one signature")
        })
        .collect()
    }

    #[test]
    fn the_versor_holds_every_plane_at_its_own_angle() {
        let parts = three();
        let rate = total(&parts);
        let v = versor(&rate).expect("a versor");
        for order in [[0, 1, 2], [2, 0, 1], [1, 2, 0]] {
            let product = order
                .iter()
                .try_fold(one(0), |w, k| w.product(&one(0).sum(&parts[*k])?))
                .expect("one signature");
            assert_eq!(product, v, "V = Π(1 + B_k), in any order");
        }
        assert!(
            v.terms().any(|(m, _)| m.count_ones() == 6),
            "the volume of all three"
        );
        assert_eq!(
            wedge_powers(&rate).expect("powers").len(),
            4,
            "three planes"
        );
    }

    #[test]
    fn the_angles_and_planes_read_back_exactly() {
        let parts = three();
        let rate = total(&parts);
        let mu = [q(-1, 4), q(-1, 9), q(-1, 25)];
        let e_m = invariants(&rate).expect("invariants");
        assert_eq!(
            e_m,
            vec![
                q(1, 1),
                mu.iter().cloned().fold(q(0, 1), |s, x| s + x),
                mu[0].clone() * mu[1].clone()
                    + mu[0].clone() * mu[2].clone()
                    + mu[1].clone() * mu[2].clone(),
                mu[0].clone() * mu[1].clone() * mu[2].clone(),
            ]
        );
        for (k, m) in mu.iter().enumerate() {
            assert_eq!(plane_of(&rate, m).expect("a plane"), parts[k]);
        }
        assert_eq!(plane_of(&rate, &q(-1, 7)), Err(TurningRefusal::NotAngle));
    }

    /// A boost and a turning on orthogonal planes of `Cl(3,1)` (`e₀`
    /// negative): `⅓e₀₁ + ½e₂₃` keeps the volume `⅙e₀₁₂₃`, and reads back
    /// with angles `1/9` (a boost) and `−1/4` (a turning).
    #[test]
    fn a_boost_and_a_turning_at_once() {
        let sig = base_axis(1).expect("an axis");
        let (b, r) = (e(sig, &[1, 2], q(1, 3)), e(sig, &[3, 4], q(1, 2)));
        let rate = b.sum(&r).expect("one signature");
        let v = versor(&rate).expect("a versor");
        assert_eq!(
            v,
            total(&[
                one(sig),
                b.clone(),
                r.clone(),
                e(sig, &[1, 2, 3, 4], q(1, 6))
            ])
        );
        assert_eq!(plane_of(&rate, &q(1, 9)).expect("a plane"), b);
        assert_eq!(plane_of(&rate, &q(-1, 4)).expect("a plane"), r);
    }

    #[test]
    fn isoclinic_planes_refuse() {
        let rate = e(0, &[1, 2], q(1, 2))
            .sum(&e(0, &[3, 4], q(1, 2)))
            .expect("one signature");
        assert_eq!(plane_of(&rate, &q(-1, 4)), Err(TurningRefusal::Isoclinic));
        assert!(versor(&rate).is_ok());
    }

    /// A spin carried by a turning moves between the planes the turning
    /// touches and keeps its angles; carried by a motion on its own plane, it
    /// stays as it was.
    #[test]
    fn a_carried_spin_keeps_its_angles() {
        let parts = three();
        let spin = total(&parts);
        let turn = one(0).sum(&e(0, &[2, 5], q(2, 3))).expect("one signature");
        let moved = carry(&turn, &spin).expect("carried");
        assert_ne!(moved, spin);
        assert!(
            moved.terms().all(|(m, _)| m.count_ones() == 2),
            "still a bivector"
        );
        assert_eq!(invariants(&moved), invariants(&spin));
        let own = e(0, &[1, 2], q(1, 2));
        let along = one(0).sum(&e(0, &[1, 2], q(3, 7))).expect("one signature");
        assert_eq!(carry(&along, &own).expect("carried"), own);
    }
}
