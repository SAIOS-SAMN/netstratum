// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The geodesic between two orientations, as a certified reading (docs/117
//! §11; operator 2026-11-05).
//!
//! - **Degrees are the state** (docs/117 §11.1): rotation angles on the
//!   round Clifford turning planes, exact rationals in degrees, the plane
//!   named.
//! - **A body's orientation is one rotor on S³**, built from its three
//!   turnings on `e₂₃`, `e₃₁`, `e₁₂` in cyclic order: the turnings about
//!   axes 1, 2 and 3. Each turning is the half-angle rotor
//!   `cos(θ/2) + sin(θ/2)·B_k`; they are composed in the fixed order `e₂₃`
//!   first, then `e₃₁`, then `e₁₂`, `R = R₁₂·R₃₁·R₂₃` (a vector turns as
//!   `x ↦ R x R̃`, the rightmost factor acting first). The planes' rotors do
//!   not commute, so the order is part of the orientation's meaning.
//! - **The geodesic between two bodies is a certified reading**: exact
//!   rational bounds, never a float, never a stored real (docs/78 law 11,
//!   [`Span`]). It is the angle `Ω` of the relative rotation `R̃_a·R_b`, read
//!   as `cos Ω = 2·cos²(Ω/2) − 1`, `cos(Ω/2) = |⟨R̃_a R_b⟩₀|`. That square is
//!   the reading of a bounded interval, never state.
//!
//! The trigonometric values are spans: π by Machin's formula and cos, sin by
//! their Taylor series, each an alternating series whose partial sums
//! bracket the value, stopped by its remainder bound (the next term under
//! the width asked), never by a fixed count of terms. Every angle is first
//! reduced exactly, mod 360° and by the quarter-turn and octant symmetries,
//! into `[0°, 45°]`. A reading at `depth` is narrower than `60⁻ᵈᵉᵖᵗʰ`; its
//! sexagesimal sector ([`reading`], [`Sector::holding`]) is given when both
//! bounds fall in one sector, and refused otherwise (the caller refines).
//!
//! Pure: no law reads this module yet.

use crate::coordinate::{Sector, BASE, TURN};
use cfe::ratio::{ratio, Ratio, Span};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// `60⁻ᵈᵉᵖᵗʰ`, exactly: the width a reading at `depth` stays under.
fn grain(depth: u32) -> Ratio {
    Ratio::new(BigInt::one(), BigInt::from(BASE).pow(depth))
}

/// The terms an alternating series summed before its remainder bound
/// stopped it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Terms(pub u32);

/// An alternating series `Σ t_k` whose terms shrink in magnitude toward 0:
/// the true sum lies between consecutive partial sums, so it is bracketed by
/// `S_n` and `S_n + t_{n+1}` once `|t_{n+1}| < tol`. `next(t_k, k)` gives
/// `t_{k+1}` from `t_k`. Stopped by that remainder bound only.
fn alternating(first: Ratio, tol: &Ratio, next: impl Fn(&Ratio, u32) -> Ratio) -> (Span, Terms) {
    let mut sum = first.clone();
    let mut term = first;
    let mut k = 0u32;
    loop {
        let t = next(&term, k);
        if t.abs() < *tol {
            return (Span::between(sum.clone(), sum + t), Terms(k + 1));
        }
        sum += &t;
        term = t;
        k += 1;
    }
}

/// `arctan(x)` for `0 ≤ x < 1`: `Σ (−1)ᵏ x²ᵏ⁺¹/(2k+1)`, its terms
/// shrinking, bracketed to under `tol`.
fn arctan(x: &Ratio, tol: &Ratio) -> (Span, Terms) {
    let x2 = x * x;
    // Carry x^(2k+1) beside the term: t_k = (−1)^k p_k/(2k+1).
    let first = x.clone();
    let tol = tol.clone();
    let x0 = x.clone();
    alternating(first, &tol, move |_, k| {
        let p = x0.clone() * pow(&x2, k + 1);
        let sign = if (k + 1) % 2 == 0 { 1 } else { -1 };
        p * ratio(sign, 2 * (k as i64 + 1) + 1)
    })
}

fn pow(x: &Ratio, n: u32) -> Ratio {
    let mut r = Ratio::one();
    for _ in 0..n {
        r *= x;
    }
    r
}

/// The lower bound rounded down and the upper rounded up onto the grid of
/// `g`: still bounds, with smaller denominators.
fn outward(s: &Span, g: &Ratio) -> Span {
    let lo = (s.lo() / g).floor() * g;
    let hi = (s.hi() / g).ceil() * g;
    Span::between(lo, hi)
}

/// π bracketed to under `eps`, by Machin's `π = 16·arctan(1/5) −
/// 4·arctan(1/239)`, each arctan to under `eps/64`, then rounded outward
/// onto the grid of `eps/4`: width `< 20·eps/64 + eps/2 < eps`.
fn pi_within(eps: &Ratio) -> (Span, Terms) {
    let tol = eps / ratio(64, 1);
    let (a, ta) = arctan(&ratio(1, 5), &tol);
    let (b, tb) = arctan(&ratio(1, 239), &tol);
    let lo = ratio(16, 1) * a.lo() - ratio(4, 1) * b.hi();
    let hi = ratio(16, 1) * a.hi() - ratio(4, 1) * b.lo();
    let pi = outward(&Span::between(lo, hi), &(eps / ratio(4, 1)));
    (pi, Terms(ta.0 + tb.0))
}

/// Rational bounds of π, `lo ≤ π ≤ hi`, `hi − lo < 60⁻ᵈᵉᵖᵗʰ`.
pub fn pi_bounds(depth: u32) -> (Ratio, Ratio) {
    let (pi, _) = pi_within(&grain(depth));
    (pi.lo().clone(), pi.hi().clone())
}

/// The Taylor series of cos at an exact `0 ≤ x < 1` (`Σ (−1)ᵏ x²ᵏ/(2k)!`,
/// its terms shrinking), bracketed to under `tol`.
fn cos_series(x: &Ratio, tol: &Ratio) -> (Span, Terms) {
    let x2 = x * x;
    alternating(Ratio::one(), tol, move |t, k| {
        let k = k as i64;
        -(t * &x2) / ratio((2 * k + 1) * (2 * k + 2), 1)
    })
}

/// The Taylor series of sin at an exact `0 ≤ x < 1` (`Σ (−1)ᵏ x²ᵏ⁺¹/(2k+1)!`),
/// bracketed to under `tol`.
fn sin_series(x: &Ratio, tol: &Ratio) -> (Span, Terms) {
    let x2 = x * x;
    alternating(x.clone(), tol, move |t, k| {
        let k = k as i64;
        -(t * &x2) / ratio((2 * k + 2) * (2 * k + 3), 1)
    })
}

/// cos φ and sin φ for an exact `φ ∈ [0°, 45°]`, π carried as the span
/// `pi`, each to under `eps` when `pi` is narrower than `eps/2`.
///
/// The radian angle is the span `x = [φ·π_lo/180, φ·π_hi/180]`, inside
/// `[0, π/2)` where cos falls and sin rises, so `cos ∈ [cos_lo(x_hi),
/// cos_hi(x_lo)]` and `sin ∈ [sin_lo(x_lo), sin_hi(x_hi)]`. The width is the
/// two series' brackets (each `< eps/4`) plus the span of x (`≤ φ/180 ·
/// width(π) < eps/8`, |cos′|, |sin′| ≤ 1): under `eps`.
fn octant(phi: &Ratio, pi: &Span, eps: &Ratio) -> ((Span, Terms), (Span, Terms)) {
    let tol = eps / ratio(4, 1);
    let x_lo = phi * pi.lo() / ratio(180, 1);
    let x_hi = phi * pi.hi() / ratio(180, 1);
    debug_assert!(x_hi < Ratio::one());
    let (c_hi, tc_hi) = cos_series(&x_lo, &tol);
    let (c_lo, tc_lo) = cos_series(&x_hi, &tol);
    let (s_lo, ts_lo) = sin_series(&x_lo, &tol);
    let (s_hi, ts_hi) = sin_series(&x_hi, &tol);
    (
        (
            Span::between(c_lo.lo().clone(), c_hi.hi().clone()),
            Terms(tc_hi.0.max(tc_lo.0)),
        ),
        (
            Span::between(s_lo.lo().clone(), s_hi.hi().clone()),
            Terms(ts_lo.0.max(ts_hi.0)),
        ),
    )
}

fn neg(s: &Span) -> Span {
    Span::between(-s.hi().clone(), -s.lo().clone())
}

/// cos θ and sin θ as spans, θ in degrees, each narrower than `eps`, with
/// the terms each series summed: θ reduced exactly mod 360° into a quarter
/// `q` and a remainder `r ∈ [0°, 90°)`, then `r > 45°` read through
/// `90° − r`, so every series runs on `[0°, 45°]`.
fn cos_sin_within(theta: &Ratio, pi: &Span, eps: &Ratio) -> ((Span, Terms), (Span, Terms)) {
    let turn = Ratio::from_integer(TURN.into());
    let quarter = ratio(90, 1);
    let t = theta - (theta / &turn).floor() * &turn;
    let q = (&t / &quarter).floor().to_integer();
    let r = t - Ratio::from_integer(q.clone()) * &quarter;
    let (c, s) = match r <= ratio(45, 1) {
        true => octant(&r, pi, eps),
        false => {
            let (c, s) = octant(&(quarter - r), pi, eps);
            (s, c)
        }
    };
    let q: i64 = q.try_into().unwrap_or(0);
    match q {
        1 => ((neg(&s.0), s.1), c),
        2 => ((neg(&c.0), c.1), (neg(&s.0), s.1)),
        3 => (s, (neg(&c.0), c.1)),
        _ => (c, s),
    }
}

/// Certified bounds of cos θ and sin θ, θ in degrees, `((cos_lo, cos_hi),
/// (sin_lo, sin_hi))`, each `lo ≤ true ≤ hi` and narrower than `60⁻ᵈᵉᵖᵗʰ`.
/// Exact at whole quarter turns.
pub fn cos_sin_degrees(theta_deg: &Ratio, depth: u32) -> ((Ratio, Ratio), (Ratio, Ratio)) {
    let ((c, _), (s, _)) = cos_sin_degrees_counted(theta_deg, depth);
    (
        (c.lo().clone(), c.hi().clone()),
        (s.lo().clone(), s.hi().clone()),
    )
}

/// [`cos_sin_degrees`] as spans, with the terms each series summed (for a
/// cos or sin read from the other's series, that series' count).
pub fn cos_sin_degrees_counted(theta_deg: &Ratio, depth: u32) -> ((Span, Terms), (Span, Terms)) {
    let eps = grain(depth);
    let (pi, _) = pi_within(&(&eps / ratio(2, 1)));
    cos_sin_within(theta_deg, &pi, &eps)
}

/// A unit rotor on S³ (the even algebra of three axes), held as four
/// certified spans: the scalar and the parts on `e₂₃`, `e₃₁`, `e₁₂`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rotor {
    pub scalar: Span,
    /// The parts on `e₂₃`, `e₃₁`, `e₁₂`, in that cyclic order.
    pub planes: [Span; 3],
}

fn sub(a: &Span, b: &Span) -> Span {
    a.add(&neg(b))
}

impl Rotor {
    /// The identity, exact.
    pub fn identity() -> Rotor {
        let z = Span::exact(Ratio::zero());
        Rotor {
            scalar: Span::exact(Ratio::one()),
            planes: [z.clone(), z.clone(), z],
        }
    }

    /// The turning by `theta` degrees on plane `k` (0: `e₂₃`, 1: `e₃₁`,
    /// 2: `e₁₂`): `cos(θ/2) + sin(θ/2)·B_k`, each part narrower than `eps`.
    fn turning(k: usize, theta: &Ratio, pi: &Span, eps: &Ratio) -> Rotor {
        let ((c, _), (s, _)) = cos_sin_within(&(theta / ratio(2, 1)), pi, eps);
        let mut r = Rotor::identity();
        r.scalar = c;
        r.planes[k] = s;
        r
    }

    /// The turning by `theta` degrees on plane `k` (0: `e₂₃`, 1: `e₃₁`,
    /// 2: `e₁₂`), each part narrower than `60⁻ᵈᵉᵖᵗʰ`.
    pub fn plane(k: usize, theta: &Ratio, depth: u32) -> Rotor {
        let eps = grain(depth);
        let (pi, _) = pi_within(&(&eps / ratio(2, 1)));
        Rotor::turning(k, theta, &pi, &eps)
    }

    /// The reverse `R̃` (the inverse of a unit rotor): the plane parts
    /// negated.
    pub fn reverse(&self) -> Rotor {
        Rotor {
            scalar: self.scalar.clone(),
            planes: [
                neg(&self.planes[0]),
                neg(&self.planes[1]),
                neg(&self.planes[2]),
            ],
        }
    }

    /// The geometric product `self · o`, by exact span arithmetic (each
    /// product the least and greatest of its endpoint products). With
    /// `B₁ = e₂₃, B₂ = e₃₁, B₃ = e₁₂`: `B_k² = −1` and `B₁B₂ = −B₃`,
    /// `B₂B₃ = −B₁`, `B₃B₁ = −B₂`; it does not commute.
    pub fn mul(&self, o: &Rotor) -> Rotor {
        let (a0, a) = (&self.scalar, &self.planes);
        let (b0, b) = (&o.scalar, &o.planes);
        let scalar = sub(
            &a0.mul(b0),
            &a[0].mul(&b[0]).add(&a[1].mul(&b[1])).add(&a[2].mul(&b[2])),
        );
        let part = |i: usize, j: usize, k: usize| {
            // B_i: a0 b_i + a_i b0 − a_j b_k + a_k b_j.
            a0.mul(&b[i])
                .add(&a[i].mul(b0))
                .add(&sub(&a[k].mul(&b[j]), &a[j].mul(&b[k])))
        };
        Rotor {
            scalar,
            planes: [part(0, 1, 2), part(1, 2, 0), part(2, 0, 1)],
        }
    }
}

/// The rotor of three plane turnings (degrees on `e₂₃`, `e₃₁`, `e₁₂`),
/// composed in the fixed order: `e₂₃` first, then `e₃₁`, then `e₁₂`,
/// `R = R₁₂·R₃₁·R₂₃`. The order matters: plane rotors do not commute.
/// Each half-angle part is narrower than `60⁻ᵈᵉᵖᵗʰ`; the composed parts
/// carry the products' widening.
pub fn rotor_of(turns: [Ratio; 3], depth: u32) -> Rotor {
    let eps = grain(depth);
    let (pi, _) = pi_within(&(&eps / ratio(2, 1)));
    rotor_within(&turns, &pi, &eps)
}

fn rotor_within(turns: &[Ratio; 3], pi: &Span, eps: &Ratio) -> Rotor {
    turns
        .iter()
        .enumerate()
        .fold(Rotor::identity(), |r, (k, th)| {
            Rotor::turning(k, th, pi, eps).mul(&r)
        })
}

/// `cos Ω` of the relative rotation `R̃_a·R_b` between two rotors, as a span:
/// `cos(Ω/2) = |⟨R̃_a R_b⟩₀|`, `cos Ω = 2·cos²(Ω/2) − 1`, the square the
/// reading of the bounded span, met with `[−1, 1]` where the value lies.
pub fn between(a: &Rotor, b: &Rotor) -> Span {
    let w = a.reverse().mul(b).scalar;
    let half = match (w.lo().is_negative(), w.hi().is_negative()) {
        (false, _) => w,
        (true, true) => neg(&w),
        (true, false) => Span::between(Ratio::zero(), w.lo().abs().max(w.hi().clone())),
    };
    let sq = Span::between(half.lo() * half.lo(), half.hi() * half.hi());
    let c = Span::between(
        ratio(2, 1) * sq.lo() - Ratio::one(),
        ratio(2, 1) * sq.hi() - Ratio::one(),
    );
    c.meet(&Span::between(ratio(-1, 1), Ratio::one()))
        .unwrap_or(c)
}

/// Certified bounds of `cos Ω`, `Ω` the angle between the orientations `a`
/// and `b` (each three plane turnings in degrees, [`rotor_of`]), narrower
/// than `60⁻ᵈᵉᵖᵗʰ`: the rotors are read deeper, one sexagesimal place at a
/// time, until the reading's width is under it (stopped by the width, never
/// by a fixed depth).
pub fn geodesic(a: [Ratio; 3], b: [Ratio; 3], depth: u32) -> (Ratio, Ratio) {
    let want = grain(depth);
    let mut d = depth + 1;
    loop {
        let eps = grain(d);
        let (pi, _) = pi_within(&(&eps / ratio(2, 1)));
        let c = between(&rotor_within(&a, &pi, &eps), &rotor_within(&b, &pi, &eps));
        if c.extent() < want {
            return (c.lo().clone(), c.hi().clone());
        }
        d += 1;
    }
}

/// The sexagesimal reading of a certified span at `depth`: when both bounds
/// fall in one `60⁻ᵈᵉᵖᵗʰ` sector, that sector's lower bound, exact; None
/// when the span straddles a sector boundary (the caller refines).
pub fn reading(bounds: &(Ratio, Ratio), depth: u32) -> Option<Ratio> {
    let lo = Sector::holding(&bounds.0, depth);
    let hi = Sector::holding(&bounds.1, depth);
    (lo == hi).then(|| lo.lo())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deg(n: i64) -> Ratio {
        ratio(n, 1)
    }

    fn holds(b: &(Ratio, Ratio), x: &Ratio) -> bool {
        b.0 <= *x && *x <= b.1
    }

    fn narrow(b: &(Ratio, Ratio), depth: u32) -> bool {
        b.0 <= b.1 && &b.1 - &b.0 < grain(depth)
    }

    #[test]
    fn pi_is_bracketed_under_the_grain() {
        let (lo, hi) = pi_bounds(6);
        assert!(ratio(314159, 100000) < lo);
        assert!(lo <= hi);
        assert!(hi < ratio(314160, 100000));
        assert!(&hi - &lo < grain(6));
        let (pi, t) = pi_within(&grain(6));
        assert!(pi.extent() < grain(6));
        assert!(t.0 > 0);
    }

    #[test]
    fn quarter_turns_are_exact() {
        let z = (Ratio::zero(), Ratio::zero());
        let one = (Ratio::one(), Ratio::one());
        let m1 = (ratio(-1, 1), ratio(-1, 1));
        for (th, c, s) in [
            (0, &one, &z),
            (90, &z, &one),
            (180, &m1, &z),
            (270, &z, &m1),
            (360, &one, &z),
            (-90, &z, &m1),
            (450, &z, &one),
        ] {
            let (cc, ss) = cos_sin_degrees(&deg(th), 6);
            assert_eq!(&cc, c, "cos {th}");
            assert_eq!(&ss, s, "sin {th}");
        }
    }

    #[test]
    fn known_values_are_held() {
        let half = ratio(1, 2);
        let (c60, _) = cos_sin_degrees(&deg(60), 6);
        let (_, s30) = cos_sin_degrees(&deg(30), 6);
        assert!(holds(&c60, &half) && narrow(&c60, 6));
        assert!(holds(&s30, &half) && narrow(&s30, 6));
        let (c120, s210) = (
            cos_sin_degrees(&deg(120), 6).0,
            cos_sin_degrees(&deg(210), 6).1,
        );
        assert!(holds(&c120, &-half.clone()) && holds(&s210, &-half));
    }

    #[test]
    fn cos_squared_plus_sin_squared_brackets_one() {
        for th in [
            ratio(1, 3),
            deg(17),
            deg(45),
            ratio(1271, 7),
            deg(-200),
            ratio(9001, 13),
        ] {
            let (c, s) = cos_sin_degrees(&th, 6);
            let (c, s) = (Span::between(c.0, c.1), Span::between(s.0, s.1));
            let sum = c.mul(&c).add(&s.mul(&s));
            assert!(sum.contains(&Ratio::one()), "θ = {th}");
        }
    }

    /// A fixed linear congruence: the angles are reproducible.
    fn angles(n: usize) -> Vec<Ratio> {
        let mut x: u64 = 0x2545_f491_4f6c_dd1d;
        (0..n)
            .map(|_| {
                x = x
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let num = ((x >> 20) % 2_000_000) as i64 - 1_000_000;
                let den = ((x >> 50) % 997) as i64 + 1;
                ratio(num, den)
            })
            .collect()
    }

    #[test]
    fn sin_and_cos_of_the_complement_overlap() {
        for th in angles(40) {
            let (c, s) = cos_sin_degrees(&th, 6);
            assert!(narrow(&c, 6) && narrow(&s, 6), "θ = {th}");
            let (cc, _) = cos_sin_degrees(&(deg(90) - &th), 6);
            let (s, cc) = (Span::between(s.0, s.1), Span::between(cc.0, cc.1));
            assert!(s.meet(&cc).is_some(), "θ = {th}");
        }
    }

    #[test]
    fn terms_are_stopped_by_the_remainder() {
        let mut seen = Vec::new();
        for th in [deg(0), deg(1), deg(10), deg(30), deg(45), deg(60), deg(89)] {
            let ((c, tc), _) = cos_sin_degrees_counted(&th, 6);
            assert!(c.extent() < grain(6));
            seen.push((th.to_string(), tc.0));
        }
        eprintln!("cos terms at depth 6: {seen:?}");
        let (_, tp) = pi_within(&(grain(6) / ratio(2, 1)));
        eprintln!("pi terms at depth 6: {}", tp.0);
        // More of the octant needs more terms; none needs many.
        assert!(seen.iter().all(|(_, t)| *t <= 12));
        assert!(seen[4].1 >= seen[1].1);
    }

    #[test]
    fn identical_orientations_are_at_no_angle() {
        let a = [deg(10), ratio(-37, 3), deg(200)];
        let g = geodesic(a.clone(), a, 4);
        assert!(holds(&g, &Ratio::one()) && narrow(&g, 4));
    }

    #[test]
    fn one_plane_apart_reads_that_angle() {
        let half = ratio(1, 2);
        let zero = Ratio::zero();
        for k in 0..3 {
            let a = [deg(10), deg(20), deg(30)];
            let mut b = a.clone();
            b[k] += deg(60);
            let g = geodesic(a.clone(), b, 4);
            assert!(holds(&g, &half) && narrow(&g, 4), "plane {k}");
            let mut b = a.clone();
            b[k] += deg(90);
            let g = geodesic(a, b, 4);
            assert!(holds(&g, &zero) && narrow(&g, 4), "plane {k}");
        }
    }

    #[test]
    fn the_geodesic_is_symmetric() {
        let a = [deg(15), ratio(7, 2), deg(-80)];
        let b = [deg(100), deg(33), ratio(1, 9)];
        assert_eq!(geodesic(a.clone(), b.clone(), 3), geodesic(b, a, 3));
    }

    #[test]
    fn plane_turnings_do_not_commute() {
        let one_then_two = Rotor::plane(1, &deg(90), 5).mul(&Rotor::plane(0, &deg(90), 5));
        let two_then_one = Rotor::plane(0, &deg(90), 5).mul(&Rotor::plane(1, &deg(90), 5));
        let c = between(&one_then_two, &two_then_one);
        // Not at no angle: cos Ω stands wholly below 1 (Ω = 120°, cos −½).
        assert!(*c.hi() < Ratio::one());
        assert!(c.contains(&ratio(-1, 2)));
        // rotor_of's fixed order is the first.
        let r = rotor_of([deg(90), deg(90), deg(0)], 5);
        assert!(between(&r, &one_then_two).contains(&Ratio::one()));
    }

    #[test]
    fn a_reading_is_given_inside_one_sector_and_refused_across() {
        let (lo, hi) = pi_bounds(6);
        let r = reading(&(lo.clone(), hi.clone()), 3);
        let s = Sector::holding(&lo, 3);
        assert_eq!(r, Some(s.lo()));
        assert!(s.lo() <= lo && hi < s.hi());
        let edge = ratio(1, 60);
        let across = (&edge - ratio(1, 7200), &edge + ratio(1, 7200));
        assert_eq!(reading(&across, 1), None);
        assert_eq!(reading(&across, 0), Some(Ratio::zero()));
    }
}
