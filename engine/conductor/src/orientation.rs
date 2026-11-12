//! H's orientation held as the generator (docs/117 §11.7, #117 step 2), on
//! the state's algebra Cl(5,4) over ℚ (docs/131, operator 2026-11-12): a
//! body's motion is one generator `G` over the nine axes, with its windings
//! `K` per turning plane, never one angle per plane.
//!
//! - **The axes** (docs/131 §1, [`AXES`]): each plane's whole `e₊ₖ` and span
//!   `e₋ₖ`, R's pole axis `e_R`, and the universe's pair `e₊, e₋`; the spans
//!   and `e₋` square to −1 ([`NEGATIVES`]). A pair of axes of one sign is a
//!   turning, of each sign a boost ([`is_turning`]): 16 and 20.
//! - **The built planes** ([`PLANES`]): the three spatial planes `e₊₂e₊₃`,
//!   `e₊₃e₊₁`, `e₊₁e₊₂` (in `flows::SPACE` order), then their R-plane
//!   partners, the dual pairs of §11.6, `e₊₁e_R`, `e₊₂e_R`, `e₊₃e_R`: the
//!   continuum's subspace `e₊₁e₊₂e₊₃e_R`, held as six components. Every other
//!   part of `G` (the vacuum's turnings, the boosts, the universe's planes and
//!   any part of higher grade the composition makes) is held beside them as
//!   one exact value ([`Orientation::wide`]); together they are the one
//!   generator.
//! - **The rate a ring** ([`rate`]): `b = δ/(1 − δ) = H/R`, the hold over
//!   the reach, the one linear-fractional map sending rest, the rim and the
//!   throat (`δ = 0, ½, 1`) to the Cayley rates `0, 1, ∞` (§11.7.3). The
//!   vacuum's inversion `δ ↦ 1 − δ` is the reciprocal `b ↦ 1/b`.
//! - **The advance** ([`Orientation::advance`]): `G ← generator(cayley(G)·
//!   cayley(B))`, whole, by `conductor::turning`'s composition, never plane
//!   by plane; its parts on the planes between are made by the composition
//!   (register K6 retired). Each component is held on the grain. A motion on
//!   the continuum's subspace alone is composed on its isoclinic halves, the
//!   same law in closed form; any other by the Cayley form on Cl(5,4)
//!   ([`Orientation::advance_wide`]).
//! - **One cover, and the windings:** `R` and `−R` turn every vector alike,
//!   so the motion is held as the one whose scalar part is not negative (a
//!   plane alone then stands at `|g| ≤ 1`); a whole turn gives the
//!   components back. A plane's `K` rises by one each time its reading
//!   passes 0 under a forward drive and falls on a pass backward: exact for
//!   a plane turned alone (`K = ⌊θ/360°⌋`, §11.1), a reading of the plane's
//!   component where other planes turn with it (78.173 §8.3: windings stay
//!   on the planes that wound them). A boost has no period and never winds.
//! - **The reading the laws act on** ([`reading`]): `ψ = g/(1 + |g|)` turns,
//!   taken into `[0, 1)`, the L1 reading §8d names, rational, exact at the
//!   hemisphere kinks (`g = 0` and `g = 1` are `0` and `½` a turn), and one
//!   ring from rest gives back the motion law's `δ` exactly. Degrees
//!   (`tan(θ/4) = g`) are rendered on observation only. The isoclinic
//!   readings (form, twist, half axes) are read on the continuum's subspace.
use crate::turning::{base_axis, Clifford, Regime, TurningRefusal};
use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// Each frame plane's whole, `e₊ₖ` (docs/131 §1): bits 0–2.
pub const WHOLE: [usize; 3] = [1, 2, 4];
/// The universe's whole, `e₊` (origin and infinity together): bit 3.
pub const UNIVERSE_WHOLE: usize = 8;
/// The universe's span, `e₋` (origin against infinity): bit 4.
pub const UNIVERSE_SPAN: usize = 16;
/// R, the electron's phase, the pole axis `e_R`: base axis 4, bit 5.
pub const R_AXIS: usize = 32;
/// Each frame plane's span, `e₋ₖ`: base axes 5–7, bits 6–8.
pub const SPAN: [usize; 3] = [64, 128, 256];
/// The nine axes of Cl(5,4), as blade masks.
pub const AXES: [usize; 9] = [1, 2, 4, 8, 16, 32, 64, 128, 256];
/// The axes that square to −1: the spans and the universe's span.
pub const NEGATIVES: usize = UNIVERSE_SPAN | 64 | 128 | 256;

/// The 36 pairs of axes, as grade-2 masks ascending.
pub fn pairs() -> Vec<usize> {
    let mut v: Vec<usize> = AXES
        .iter()
        .enumerate()
        .flat_map(|(i, a)| AXES[i + 1..].iter().map(move |b| a | b))
        .collect();
    v.sort_unstable();
    v
}

/// A pair's character, read from its axes' squares by the product
/// (docs/117 §3): a turning where its unit squares to −1, a boost where to
/// +1. False for anything but a pair.
pub fn is_turning(mask: usize) -> bool {
    Regime::of(&Clifford::blade(NEGATIVES, mask, ratio(1, 1))) == Ok(Regime::Elliptic)
}

/// The pole swap of frame plane `k` (docs/117 §10.1, docs/131 §4): the
/// reflection along its span `e₋ₖ`, exchanging its held and open poles; on a
/// value, every blade holding `e₋ₖ` negated. An involution.
pub fn pole_swap(k: usize, x: &Clifford) -> Clifford {
    let s = SPAN[k];
    x.terms().fold(
        Clifford::blade(x.negatives(), 0, ratio(0, 1)),
        |acc, (m, c)| {
            let c = if m & s != 0 { -c.clone() } else { c.clone() };
            acc.sum(&Clifford::blade(x.negatives(), m, c))
                .expect("one signature")
        },
    )
}

/// A value carried by a motion, `M·x·M̃` (docs/131 §4): exact, the
/// conjugation, no norm read.
pub fn carried(m: &Clifford, x: &Clifford) -> Result<Clifford, TurningRefusal> {
    m.product(x)?.product(&crate::turning::reversed(m))
}

/// The six turning planes: the units each joins (1–3 spatial, 4 = R).
/// Spatial first in `flows::SPACE` order, then each one's R-plane partner.
pub const PLANES: [(usize, usize); 6] = [(2, 3), (3, 1), (1, 2), (1, 4), (2, 4), (3, 4)];

/// The spatial planes' places in [`PLANES`].
pub const SPATIAL: [usize; 3] = [0, 1, 2];

/// Each spatial plane's R-plane partner (its dual pair, §11.6).
pub const PARTNER: [usize; 3] = [3, 4, 5];

/// A plane's blade mask and the sign of `e_i e_j` against its ascending
/// blade.
fn blade(p: usize) -> (usize, Ratio) {
    let (i, j) = PLANES[p];
    let (a, b) = (
        base_axis(i).expect("an axis"),
        base_axis(j).expect("an axis"),
    );
    let sign = if i < j { ratio(1, 1) } else { ratio(-1, 1) };
    (a | b, sign)
}

/// The generator with components `c` on [`PLANES`].
pub fn bivector(c: &[Ratio; 6]) -> Clifford {
    let mut g = Clifford::blade(NEGATIVES, 0, ratio(0, 1));
    for (p, x) in c.iter().enumerate() {
        if x.is_zero() {
            continue;
        }
        let (mask, sign) = blade(p);
        g = g
            .sum(&Clifford::blade(NEGATIVES, mask, sign * x.clone()))
            .expect("one signature");
    }
    g
}

/// A generator's components on [`PLANES`]; refused where it holds a part
/// on no plane.
pub fn components(g: &Clifford) -> Result<[Ratio; 6], TurningRefusal> {
    let mut c: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
    let mut stray = Vec::new();
    for (mask, x) in g.terms() {
        match (0..6).find(|&p| blade(p).0 == mask) {
            Some(p) => c[p] = blade(p).1 * x.clone(),
            None => stray.push(mask),
        }
    }
    if stray.is_empty() {
        Ok(c)
    } else {
        Err(TurningRefusal::NotGenerator { masks: stray })
    }
}

/// The rate a ring at the share `δ`: `b = δ/(1 − δ)`, the hold over the
/// reach. None at the throat (`δ ≥ 1`), which has no generator.
pub fn rate(delta: &Ratio) -> Option<Ratio> {
    let rest = ratio(1, 1) - delta.clone();
    if rest <= ratio(0, 1) {
        return None;
    }
    Some(delta.clone() / rest)
}

/// A component's reading in turns, `[0, 1)`: `g/(1 + |g|)`, taken round.
pub fn reading(g: &Ratio) -> Ratio {
    let psi = g.clone() / (ratio(1, 1) + g.abs());
    psi.clone() - psi.floor()
}

/// The component whose reading is `ψ ∈ [0, 1)`, on the one cover the
/// orientation is held on (`|g| ≤ 1` on a plane alone): `ψ/(1 − ψ)` up to
/// half a turn, `(ψ − 1)/ψ` past it.
pub fn of_reading(psi: &Ratio) -> Ratio {
    let psi = psi.clone() - psi.floor();
    if psi <= ratio(1, 2) {
        psi.clone() / (ratio(1, 1) - psi)
    } else {
        (psi.clone() - ratio(1, 1)) / psi
    }
}

/// One half of the even algebra of `Cl(4, 0)`: the even algebra is the
/// direct sum of two quaternion algebras, its isoclinic halves (docs/117
/// §11.6), cut by the central idempotents `P± = (1 ± I)/2` (`I` the volume,
/// `I² = +1`). On each half the units are `i = e₂₃P±`, `j = e₃₁P±`,
/// `k = e₁₂P±`, with `i² = j² = k² = −1` and `ij = −k` (`e₂₃e₃₁ = −e₁₂`), so
/// the product carries the cross product with a minus sign.
///
/// Held as integer numerators over one positive denominator, so a product
/// is integer products alone (no reduction between them); a generator
/// divides once.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Half {
    n: [BigInt; 4],
    d: BigInt,
}

impl Half {
    fn product(&self, o: &Half) -> Half {
        let (a, b) = (&self.n, &o.n);
        let dot = &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3];
        let cross = [
            &a[2] * &b[3] - &a[3] * &b[2],
            &a[3] * &b[1] - &a[1] * &b[3],
            &a[1] * &b[2] - &a[2] * &b[1],
        ];
        Half {
            n: [
                &a[0] * &b[0] - dot,
                &a[0] * &b[1] + &b[0] * &a[1] - &cross[0],
                &a[0] * &b[2] + &b[0] * &a[2] - &cross[1],
                &a[0] * &b[3] + &b[0] * &a[3] - &cross[2],
            ],
            d: &self.d * &o.d,
        }
    }

    /// The Cayley motion of a pure `u`: `(1 + u)(1 − u)⁻¹ = (1 − |u|² +
    /// 2u)/(1 + |u|²)`, since `(1 − u)(1 + u) = 1 + |u|²` (the algebra's own
    /// product, never a norm read by a law). Over the common denominator `D`
    /// of `u = k/D`: `(D² − Σk², 2Dk)/(D² + Σk²)`.
    fn motion(u: &[Ratio; 3]) -> Half {
        let dd = u.iter().fold(BigInt::one(), |l, x| l.lcm(x.denom()));
        let k: [BigInt; 3] =
            std::array::from_fn(|i| (u[i].clone() * Ratio::from_integer(dd.clone())).to_integer());
        let kk = &k[0] * &k[0] + &k[1] * &k[1] + &k[2] * &k[2];
        let d2 = &dd * &dd;
        let two_d = &dd * BigInt::from(2);
        Half {
            n: [&d2 - &kk, &two_d * &k[0], &two_d * &k[1], &two_d * &k[2]],
            d: d2 + kk,
        }
    }

    /// The generator of a unit `r`: `(r − 1)(r + 1)⁻¹ = r_v/(1 + r₀)`; none
    /// where `r₀ = −1`.
    #[cfg(test)]
    fn generator(&self) -> Option<[Ratio; 3]> {
        let d = &self.d + &self.n[0];
        if d.is_zero() {
            return None;
        }
        Some(std::array::from_fn(|i| {
            Ratio::new(self.n[i + 1].clone(), d.clone())
        }))
    }

    fn negated(&self) -> Half {
        Half {
            n: std::array::from_fn(|k| -self.n[k].clone()),
            d: self.d.clone(),
        }
    }

    /// Its conjugate, `q̄`: the vector part reversed. For a unit, its
    /// inverse, read with no norm.
    fn conjugate(&self) -> Half {
        Half {
            n: [
                self.n[0].clone(),
                -self.n[1].clone(),
                -self.n[2].clone(),
                -self.n[3].clone(),
            ],
            d: self.d.clone(),
        }
    }

    /// A pure `v` carried by this unit, `h·v·h̄` (docs/128 §11.1): exact, a
    /// conjugation, no norm read.
    fn carry(&self, v: &[Ratio; 3]) -> [Ratio; 3] {
        let x = Half::of_ratios(&[ratio(0, 1), v[0].clone(), v[1].clone(), v[2].clone()]);
        let r = self.product(&x).product(&self.conjugate()).ratios();
        [r[1].clone(), r[2].clone(), r[3].clone()]
    }

    /// Its four components, exact: `(scalar, i, j, k)`.
    fn ratios(&self) -> [Ratio; 4] {
        std::array::from_fn(|k| Ratio::new(self.n[k].clone(), self.d.clone()))
    }

    /// From four exact components, over their common denominator.
    fn of_ratios(c: &[Ratio; 4]) -> Half {
        let d = c.iter().fold(BigInt::one(), |l, x| l.lcm(x.denom()));
        Half {
            n: std::array::from_fn(|k| {
                (c[k].clone() * Ratio::from_integer(d.clone())).to_integer()
            }),
            d,
        }
    }
}

/// A generator on [`PLANES`] split into its halves: on `P±` the components
/// `g₂₃ ∓ g₁R`, `g₃₁ ∓ g₂R`, `g₁₂ ∓ g₃R` (`e₁R = −e₂₃I`, `e₂R = −e₃₁I`,
/// `e₃R = −e₁₂I`).
fn halves(g: &[Ratio; 6]) -> ([Ratio; 3], [Ratio; 3]) {
    let plus = std::array::from_fn(|k| g[k].clone() - g[3 + k].clone());
    let minus = std::array::from_fn(|k| g[k].clone() + g[3 + k].clone());
    (plus, minus)
}

/// The generator on [`PLANES`] from its halves (the inverse of [`halves`]).
fn joined(plus: &[Ratio; 3], minus: &[Ratio; 3]) -> [Ratio; 6] {
    let two = ratio(2, 1);
    std::array::from_fn(|p| {
        if p < 3 {
            (plus[p].clone() + minus[p].clone()) / two.clone()
        } else {
            (minus[p - 3].clone() - plus[p - 3].clone()) / two.clone()
        }
    })
}

/// A body's spin split onto the halves (docs/128 §9.3), as [`halves`]
/// splits a generator: `ℓ+_k = ℓ_k − ℓ_kR`, `ℓ−_k = ℓ_k + ℓ_kR`, the spatial
/// spins on `flows::SPACE`'s order and their partners'. A missing component
/// is none.
pub fn spin_halves(spatial: &[Ratio], partner: &[Ratio]) -> ([Ratio; 3], [Ratio; 3]) {
    let at = |v: &[Ratio], k: usize| v.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    let g: [Ratio; 6] = std::array::from_fn(|p| {
        if p < 3 {
            at(spatial, p)
        } else {
            at(partner, p - 3)
        }
    });
    halves(&g)
}

/// A half's size, first degree (design law 2): `Σ_k |u_k|`.
pub fn size(u: &[Ratio; 3]) -> Ratio {
    u.iter().fold(ratio(0, 1), |s, x| s + x.abs())
}

/// One half's motion a ring (docs/128 §9.4): the Cayley vector of the rate
/// `b` along the half's own spin, `u_k = b·ℓ_k/ℓ` (`ℓ` its size). A half
/// with no spin does not move.
pub fn half_motion(spin: &[Ratio; 3], b: &Ratio) -> [Ratio; 3] {
    let l = size(spin);
    if l.is_zero() {
        return std::array::from_fn(|_| ratio(0, 1));
    }
    std::array::from_fn(|k| b.clone() * spin[k].clone() / l.clone())
}

/// A ring's tilt against a reference axis (docs/128 §11.3), read from the
/// product of the axes on the half, `Q = A·ā`: its scalar part `A·a` the axis
/// along the reference, its vector part `A×a` the line of nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tilt {
    /// The inclination share, `|A×a|₁/(|A·a| + |A×a|₁)`: `0` in the
    /// reference's plane, `1` polar.
    pub share: Ratio,
    /// The sense, `sign(A·a)`: `1` prograde, `−1` retrograde, `0` polar.
    pub sense: Ratio,
    /// The line of nodes, `A×a`.
    pub nodes: [Ratio; 3],
}

/// The tilt of an axis `a` against a reference `A` (docs/128 §11.3). None
/// where either has no size.
pub fn tilt(axis: &[Ratio; 3], reference: &[Ratio; 3]) -> Option<Tilt> {
    if size(axis).is_zero() || size(reference).is_zero() {
        return None;
    }
    let pure =
        |v: &[Ratio; 3]| Half::of_ratios(&[ratio(0, 1), v[0].clone(), v[1].clone(), v[2].clone()]);
    let q = pure(reference).product(&pure(axis).conjugate()).ratios();
    let along = q[0].clone();
    let nodes = [q[1].clone(), q[2].clone(), q[3].clone()];
    let across = size(&nodes);
    let share = across.clone() / (along.abs() + across);
    let sense = match along.cmp(&ratio(0, 1)) {
        std::cmp::Ordering::Greater => ratio(1, 1),
        std::cmp::Ordering::Less => ratio(-1, 1),
        std::cmp::Ordering::Equal => ratio(0, 1),
    };
    Some(Tilt {
        share,
        sense,
        nodes,
    })
}

/// A pure `v` turned by the Cayley motion of `u`, `m·v·m̄` with `m = (1 +
/// u)(1 − u)⁻¹` (docs/128 §13): exact, a conjugation by a unit, no norm read.
/// A `v` along `u` is fixed.
pub fn turned(u: &[Ratio; 3], v: &[Ratio; 3]) -> [Ratio; 3] {
    Half::motion(u).carry(v)
}

/// A motion on the two halves.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Motion(Half, Half);

impl Motion {
    fn of(g: &[Ratio; 6]) -> Motion {
        let (p, m) = halves(g);
        Motion(Half::motion(&p), Half::motion(&m))
    }

    fn then(&self, o: &Motion) -> Motion {
        Motion(self.0.product(&o.0), self.1.product(&o.1))
    }

    /// Its scalar part, `(r₊₀ + r₋₀)/2`.
    fn scalar(&self) -> Ratio {
        (Ratio::new(self.0.n[0].clone(), self.0.d.clone())
            + Ratio::new(self.1.n[0].clone(), self.1.d.clone()))
            / ratio(2, 1)
    }

    fn negated(&self) -> Motion {
        Motion(self.0.negated(), self.1.negated())
    }

    /// Its generator on [`PLANES`], each component floored onto the grain
    /// in integers: on the halves `v± = n±/(d± + n±₀)`, a spatial component
    /// `(v₊ + v₋)/2` and its R partner `(v₋ − v₊)/2`, read as `⌊num·60⁶/den⌋`
    /// over `60⁶`, the same value `ring::grain` gives the exact generator.
    fn generator_on_grain(&self) -> Result<[Ratio; 6], TurningRefusal> {
        let dp = &self.0.d + &self.0.n[0];
        let dm = &self.1.d + &self.1.n[0];
        if dp.is_zero() || dm.is_zero() {
            return Err(TurningRefusal::NoGenerator);
        }
        let places = BigInt::from(60).pow(6);
        let den = BigInt::from(2) * &dp * &dm;
        let (den, flip) = if den < BigInt::zero() {
            (-den, true)
        } else {
            (den, false)
        };
        let on_grain = |num: BigInt| {
            let num = if flip { -num } else { num };
            Ratio::new((num * &places).div_floor(&den), places.clone())
        };
        Ok(std::array::from_fn(|p| {
            let k = p % 3 + 1;
            let (a, b) = (&self.0.n[k] * &dm, &self.1.n[k] * &dp);
            if p < 3 {
                on_grain(a + b)
            } else {
                on_grain(b - a)
            }
        }))
    }

    #[cfg(test)]
    fn generator(&self) -> Result<[Ratio; 6], TurningRefusal> {
        match (self.0.generator(), self.1.generator()) {
            (Some(p), Some(m)) => Ok(joined(&p, &m)),
            _ => Err(TurningRefusal::NoGenerator),
        }
    }
}

/// A body's orientation: its generator's components on [`PLANES`], each on
/// the grain, and its windings per plane.
///
/// Its readings in the phase layout are kept beside it, refreshed at every
/// write of `G` (the laws read them inside their pair loops; a reading is a
/// division, so it is taken once per change, never per read). They are a
/// reading of `G`, never saved and never written on their own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Orientation {
    g: [Ratio; 6],
    pub windings: [i64; 6],
    /// The vacuum's factor (docs/131 §2, §9): its generator on the spans'
    /// turnings `e₋₂e₋₃`, `e₋₃e₋₁`, `e₋₁e₋₂` (about axes 1, 2, 3), on the
    /// grain, with their windings.
    vacuum: [Ratio; 3],
    pub vacuum_windings: [i64; 3],
    /// The scale's factor: its rate on the conformal dilator `e₊e₋`, a
    /// boost, composed by the flow law; it never winds.
    scale: Ratio,
    phases: Vec<Ratio>,
}

/// The vacuum's turning about axis k, as a blade mask with the sign of
/// `e₋ₗe₋ₘ` (`{l, m}` after k, cyclic) against its ascending blade.
fn vacuum_blade(k: usize) -> (usize, Ratio) {
    let (l, m) = ((k + 1) % 3, (k + 2) % 3);
    let sign = if l < m { ratio(1, 1) } else { ratio(-1, 1) };
    (SPAN[l] | SPAN[m], sign)
}

/// The vacuum's factor as a quaternion half. On the spans the units multiply
/// with the opposite hand (`(e₋₂e₋₃)(e₋₃e₋₁) = +e₋₁e₋₂`, the inverted face's
/// mirror), so its generator is carried on [`Half`] negated, and read back
/// negated.
fn vacuum_half(u: &[Ratio; 3]) -> Half {
    Half::motion(&std::array::from_fn(|k| -u[k].clone()))
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::of_generator(std::array::from_fn(|_| ratio(0, 1)), [0; 6])
    }
}

impl Orientation {
    /// From a generator's components on [`PLANES`] and its windings.
    pub fn of_generator(g: [Ratio; 6], windings: [i64; 6]) -> Orientation {
        let mut o = Orientation {
            g,
            windings,
            vacuum: std::array::from_fn(|_| ratio(0, 1)),
            vacuum_windings: [0; 3],
            scale: ratio(0, 1),
            phases: Vec::new(),
        };
        o.refresh();
        o
    }

    /// Its generator's components on [`PLANES`].
    pub fn g(&self) -> &[Ratio; 6] {
        &self.g
    }

    fn refresh(&mut self) {
        let mut v = vec![ratio(0, 1); crate::flows::PLANES];
        for (j, &k) in crate::flows::SPACE.iter().enumerate() {
            v[k] = reading(&self.g[SPATIAL[j]]);
        }
        v[crate::flows::LOOP] = reading(&self.loop_mean());
        self.phases = v;
    }

    /// At the readings `psi` (turns) on each plane, unwound: the founding's
    /// form, each plane on its own (no part between planes yet), exact (a
    /// reading `a/q` is the component `a/(q − a)`, its denominator bounded
    /// by the reading's).
    pub fn at(psi: &[Ratio; 6]) -> Orientation {
        Orientation::of_generator(std::array::from_fn(|p| of_reading(&psi[p])), [0; 6])
    }

    /// Each plane's reading in turns.
    pub fn readings(&self) -> [Ratio; 6] {
        std::array::from_fn(|p| reading(&self.g[p]))
    }

    /// One motion of rate `b` (components on [`PLANES`]) composed after the
    /// orientation, whole: `G ← generator(cayley(G)·cayley(B))`, each
    /// component floored onto the grain. A plane driven forward (`b > 0`)
    /// whose reading falls has passed a whole turn (`K + 1`); driven
    /// backward, a reading that rises is a turn back (`K − 1`).
    pub fn advance(&mut self, b: &[Ratio; 6]) -> Result<(), TurningRefusal> {
        self.advance_by(b, 1)
    }

    /// The vacuum's generator on its three turnings (about axes 1, 2, 3).
    pub fn vacuum(&self) -> &[Ratio; 3] {
        &self.vacuum
    }

    /// The scale's rate on the conformal dilator.
    pub fn scale(&self) -> &Ratio {
        &self.scale
    }

    /// The whole motion's generators as exact values on Cl(5,4), one per
    /// factor: the continuum's, the vacuum's and the scale's. The factors
    /// stand on disjoint axes and commute; the motion is their product.
    pub fn factors(&self) -> [Clifford; 3] {
        let vac = (0..3).fold(Clifford::blade(NEGATIVES, 0, ratio(0, 1)), |acc, k| {
            let (m, sign) = vacuum_blade(k);
            acc.sum(&Clifford::blade(
                NEGATIVES,
                m,
                sign * self.vacuum[k].clone(),
            ))
            .expect("one signature")
        });
        [
            bivector(&self.g),
            vac,
            Clifford::blade(
                NEGATIVES,
                UNIVERSE_WHOLE | UNIVERSE_SPAN,
                self.scale.clone(),
            ),
        ]
    }

    /// The vacuum's factor moved by the rate `b` (its components about axes
    /// 1, 2, 3), composed whole on its quaternion half, each component
    /// floored onto the grain on its own sense; windings as
    /// [`Orientation::advance`]'s.
    pub fn advance_vacuum(&mut self, b: &[Ratio; 3]) -> Result<(), TurningRefusal> {
        if b.iter().all(Zero::is_zero) {
            return Ok(());
        }
        let was: [Ratio; 3] = std::array::from_fn(|k| reading(&self.vacuum[k]));
        let r = vacuum_half(&self.vacuum).product(&vacuum_half(b));
        let r = if r.n[0].is_negative() != r.d.is_negative() && !r.n[0].is_zero() {
            r.negated()
        } else {
            r
        };
        let d = &r.d + &r.n[0];
        if d.is_zero() {
            return Err(TurningRefusal::NoGenerator);
        }
        let places = BigInt::from(60).pow(6);
        let (num_sign, den) = if d.is_negative() { (-1, -d) } else { (1, d) };
        self.vacuum = std::array::from_fn(|k| {
            let num = -(&r.n[k + 1]) * BigInt::from(num_sign);
            Ratio::new((num * &places).div_floor(&den), places.clone())
        });
        for k in 0..3 {
            let now = reading(&self.vacuum[k]);
            if b[k] > ratio(0, 1) && now < was[k] {
                self.vacuum_windings[k] += 1;
            } else if b[k] < ratio(0, 1) && now > was[k] {
                self.vacuum_windings[k] -= 1;
            }
        }
        Ok(())
    }

    /// The scale moved by the rate `u` on the dilator, by the flow law
    /// `s ⊕ u = (s + u)/(1 + s·u)` (LAW-DILATION), on the grain. Refused
    /// where either rate is a pole (`±1`).
    pub fn advance_scale(&mut self, u: &Ratio) -> Result<(), TurningRefusal> {
        let s = crate::turning::composed(Regime::Hyperbolic, &self.scale, u)?;
        self.scale = crate::ring::grain(&s);
        Ok(())
    }

    /// `rings` motions of rate `b` composed after the orientation at once,
    /// `cayley(G)·cayley(B)^rings`, its generator read back once.
    fn advance_by(&mut self, b: &[Ratio; 6], rings: u32) -> Result<(), TurningRefusal> {
        if b.iter().all(Zero::is_zero) || rings == 0 {
            return Ok(());
        }
        let was = self.readings();
        let step = Motion::of(b);
        let mut r = Motion::of(&self.g);
        for _ in 0..rings {
            r = r.then(&step);
        }
        // One cover: `R` and `−R` turn every vector alike, so the motion is
        // held as the one whose scalar part is not negative; a whole turn of
        // the vector (`R ↦ −R`) then gives the components back and is
        // counted in `K`, never carried in `G`.
        let r = if r.scalar() < ratio(0, 1) {
            r.negated()
        } else {
            r
        };
        let g = match r.generator_on_grain() {
            Ok(g) => g,
            // `R + 1` singular on one half: held as `−R`, the same sandwich.
            Err(TurningRefusal::NoGenerator) => r.negated().generator_on_grain()?,
            Err(e) => return Err(e),
        };
        self.g = g;
        self.refresh();
        let now = self.readings();
        for p in 0..6 {
            if b[p] > ratio(0, 1) && now[p] < was[p] {
                self.windings[p] += 1;
            } else if b[p] < ratio(0, 1) && now[p] > was[p] {
                self.windings[p] -= 1;
            }
        }
        Ok(())
    }

    /// One plane turned on its own by the reading `turns ∈ [0, 1)` (a pass
    /// through the centre is `½`, rate 1), composed whole.
    pub fn turn_plane(&mut self, p: usize, turns: &Ratio) -> Result<(), TurningRefusal> {
        let mut b: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
        b[p] = of_reading(turns);
        self.advance(&b)
    }
}

/// Turns split into whole turns (toward zero) and the signed rest, `|rest| < 1`.
fn whole_and_rest(x: &Ratio) -> (i64, Ratio) {
    let whole = x.trunc();
    let n = i64::try_from(whole.to_integer()).unwrap_or(0);
    (n, x.clone() - whole)
}

/// A signed turning of `|x| < 1` turns as its rate: `±rate(|x|)`.
fn signed_rate(x: &Ratio) -> Ratio {
    let r = rate(&x.abs()).unwrap_or_else(|| ratio(0, 1));
    if *x < ratio(0, 1) {
        -r
    } else {
        r
    }
}

impl Orientation {
    /// The readings in the phase layout of `flows` (`flows::SPACE`'s planes
    /// and `flows::LOOP`): each spatial plane's own, and the circulation's,
    /// the reading of its R-planes' mean, first degree (docs/117 §11.7.2;
    /// the laws that still read one circulation, until each reads its pair).
    pub fn phases(&self) -> Vec<Ratio> {
        self.phases.clone()
    }

    /// One reading in the phase layout.
    pub fn phase(&self, k: usize) -> Ratio {
        self.phases.get(k).cloned().unwrap_or_else(|| ratio(0, 1))
    }

    fn loop_mean(&self) -> Ratio {
        PARTNER
            .iter()
            .fold(ratio(0, 1), |s, &p| s + self.g[p].clone())
            / ratio(3, 1)
    }

    /// The circulation's reading on spatial plane `j`'s pair: its R-plane
    /// partner's own (docs/117 §11.6, the dual pair).
    pub fn circulation(&self, j: usize) -> Ratio {
        reading(&self.g[PARTNER[j]])
    }

    /// Hold reading `k` (phase layout) at `v`: a spatial plane's component
    /// on its own branch; the circulation's R-planes scaled together to the
    /// mean whose reading is `v` (each at it where none stands). A spatial
    /// reading is held on the grain, so it reads back exactly and its
    /// generator's denominator stays bounded; the R-planes' scaling is held
    /// on the grain. Exact readings held as they came, and the scaling
    /// chained bell on bell, compounded the denominators (generators of 75
    /// digits at bell 557, found by the ablation probe): the scalar collapse.
    pub fn hold_phase(&mut self, k: usize, v: &Ratio) {
        match crate::flows::SPACE.iter().position(|&s| s == k) {
            Some(j) => {
                // The reading on the grain: it reads back exactly, and its
                // generator's denominator is bounded by the grain's.
                let p = SPATIAL[j];
                let to = of_reading(&crate::ring::grain(v));
                if self.g[p] != to {
                    self.g[p] = to;
                    self.refresh();
                }
            }
            None => {
                let mean = self.loop_mean();
                let to = of_reading(v);
                if to == mean {
                    return;
                }
                for p in PARTNER {
                    self.g[p] = crate::ring::grain(&if mean.is_zero() {
                        to.clone()
                    } else {
                        self.g[p].clone() * to.clone() / mean.clone()
                    });
                }
                self.refresh();
            }
        }
    }

    /// One bell of a body's turning, in the phase layout's turns: each
    /// spatial plane by its own `space[j]` turns, the circulation by `circ`
    /// turns on each R-plane alike (one circulation seen in every pair, its
    /// spin shared by the tilt where the swing reads it). Whole turns wind
    /// the planes they turn on; the rest, under a turn, is the bell's two
    /// rings, each at the rate of half of it held on the grain (the carrier,
    /// as every rate), composed whole: `cayley(G)·cayley(B)²`.
    pub fn bell(&mut self, space: &[Ratio], circ: &[Ratio]) -> Result<(), TurningRefusal> {
        let ring_rate =
            |rest: &Ratio| crate::ring::grain(&signed_rate(&(rest.clone() / ratio(2, 1))));
        let mut b: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
        for j in 0..3 {
            let (n, rest) = whole_and_rest(space.get(j).unwrap_or(&ratio(0, 1)));
            self.windings[SPATIAL[j]] += n;
            b[SPATIAL[j]] = ring_rate(&rest);
        }
        // Each R partner turns with its own plane (docs/128 §8): the pair
        // composed together, never one circulation for all three.
        for j in 0..3 {
            let (n, rest) = whole_and_rest(circ.get(j).unwrap_or(&ratio(0, 1)));
            self.windings[PARTNER[j]] += n;
            b[PARTNER[j]] = ring_rate(&rest);
        }
        self.advance_by(&b, 2)
    }

    /// Its unitary morphodynamic state (docs/128 §10.1), exact: the form
    /// `F = q`, the automorphism (its orientation in space, R held), and the
    /// twist `C = p·q̄`, the isoclinic turning against R, each `(scalar, i,
    /// j, k)`. `p` is the `P+` half of its motion and `q` the `P−` half.
    pub fn form_twist(&self) -> ([Ratio; 4], [Ratio; 4]) {
        let Motion(p, q) = Motion::of(&self.g);
        (q.ratios(), p.product(&q.conjugate()).ratios())
    }

    /// Its twist's generator on the three units (docs/128 §10.4): the Cayley
    /// generator of `C`, `v = C_vec/(1 + C₀)`, isoclinic (one value per dual
    /// pair). None where `C = −1` (the halves opposed, a half turn of R).
    pub fn twist_generator(&self) -> Option<[Ratio; 3]> {
        let Motion(p, q) = Motion::of(&self.g);
        let c = p.product(&q.conjugate());
        let den = &c.d + &c.n[0];
        if den.is_zero() {
            return None;
        }
        Some(std::array::from_fn(|k| {
            Ratio::new(c.n[k + 1].clone(), den.clone())
        }))
    }

    /// Each half's axis in space (docs/128 §11.1): its spin in the body's
    /// own axes carried by its half, `a+ = p·ℓ+·p̄`, `a− = q·ℓ−·q̄`, exact.
    pub fn half_axes(&self, plus: &[Ratio; 3], minus: &[Ratio; 3]) -> ([Ratio; 3], [Ratio; 3]) {
        let Motion(p, q) = Motion::of(&self.g);
        (p.carry(plus), q.carry(minus))
    }

    /// A direction in space carried back into the body's own axes by each
    /// half (the inverse of [`Orientation::half_axes`]): `p̄·a+·p`, `q̄·a−·q`.
    pub fn body_axes(&self, plus: &[Ratio; 3], minus: &[Ratio; 3]) -> ([Ratio; 3], [Ratio; 3]) {
        let Motion(p, q) = Motion::of(&self.g);
        (p.conjugate().carry(plus), q.conjugate().carry(minus))
    }

    /// The orientation whose state is `(F, C)`: `q = F`, `p = C·F`, its
    /// generator held on the grain, unwound.
    pub fn of_form_twist(
        form: &[Ratio; 4],
        twist: &[Ratio; 4],
    ) -> Result<Orientation, TurningRefusal> {
        let q = Half::of_ratios(form);
        let p = Half::of_ratios(twist).product(&q);
        let m = Motion(p, q);
        let m = if m.scalar() < ratio(0, 1) {
            m.negated()
        } else {
            m
        };
        let g = match m.generator_on_grain() {
            Ok(g) => g,
            Err(TurningRefusal::NoGenerator) => m.negated().generator_on_grain()?,
            Err(e) => return Err(e),
        };
        Ok(Orientation::of_generator(g, [0; 6]))
    }

    /// One bell on the group (docs/128 §9.4): each half moved by its own
    /// Cayley vector a ring, `q± ↦ q±·m±·m±`, `m± = (1 + u±)(1 − u±)⁻¹`. The
    /// pair is handed to the composition as the generator joined from the
    /// halves, each component held on the grain (the carrier, as every
    /// rate).
    pub fn bell_halves(
        &mut self,
        plus: &[Ratio; 3],
        minus: &[Ratio; 3],
    ) -> Result<(), TurningRefusal> {
        let b = joined(plus, minus);
        let b: [Ratio; 6] = std::array::from_fn(|p| crate::ring::grain(&b[p]));
        self.advance_by(&b, 2)
    }

    /// The echo's beat (docs/130 §1): an automorphism, both halves turned
    /// alike about the spin `spin` (body axes) by `a` turns, the Cayley motion
    /// of the rate `a/(1 − a)` along its direction, composed once and held on
    /// the grain. The twist `C` is kept (equal halves) and the spin is not
    /// touched. None where `a` is outside the motion law's domain or there is
    /// no spin.
    pub fn beat(&mut self, spin: &[Ratio; 3], a: &Ratio) -> Option<Result<(), TurningRefusal>> {
        if size(spin).is_zero() {
            return None;
        }
        let b = rate(a)?;
        let u = half_motion(spin, &b);
        let g = joined(&u, &u);
        let g: [Ratio; 6] = std::array::from_fn(|p| crate::ring::grain(&g[p]));
        Some(self.advance_by(&g, 1))
    }

    /// Plane `k` (phase layout) through the centre on its own: half a turn
    /// there, `b = 1`, composed whole (docs/120 §8b′).
    pub fn half_turn(&mut self, k: usize) -> Result<(), TurningRefusal> {
        match crate::flows::SPACE.iter().position(|&s| s == k) {
            Some(j) => self.turn_plane(SPATIAL[j], &ratio(1, 2)),
            None => {
                let v = self.phase(k);
                self.hold_phase(k, &crate::flows::frac(&(v + ratio(1, 2))));
                Ok(())
            }
        }
    }

    /// The holding-weighted mean of orientations (a merger, first degree on
    /// the generators); the windings of the heaviest stand.
    pub fn mean(parts: &[(&Orientation, Ratio)]) -> Orientation {
        let whole = parts.iter().fold(ratio(0, 1), |s, (_, w)| s + w.clone());
        let mut out = Orientation::default();
        if whole.is_zero() {
            return out;
        }
        for p in 0..6 {
            let sum = parts
                .iter()
                .fold(ratio(0, 1), |s, (o, w)| s + o.g[p].clone() * w.clone());
            out.g[p] = crate::ring::grain(&(sum / whole.clone()));
        }
        for k in 0..3 {
            let sum = parts
                .iter()
                .fold(ratio(0, 1), |s, (o, w)| s + o.vacuum[k].clone() * w.clone());
            out.vacuum[k] = crate::ring::grain(&(sum / whole.clone()));
        }
        let sum = parts
            .iter()
            .fold(ratio(0, 1), |s, (o, w)| s + o.scale.clone() * w.clone());
        out.scale = crate::ring::grain(&(sum / whole.clone()));
        if let Some((o, _)) = parts.iter().max_by(|a, b| a.1.cmp(&b.1)) {
            out.windings = o.windings;
            out.vacuum_windings = o.vacuum_windings;
        }
        out.refresh();
        out
    }

    /// From readings in the phase layout (a founding, a spray, a body's
    /// phases handed on): each spatial plane at its reading, the
    /// circulation's at its own on every R-plane, unwound, no part between
    /// planes yet.
    pub fn from_phases(phase: &[Ratio]) -> Orientation {
        let at = |k: usize| phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
        let mut psi: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
        for (j, &k) in crate::flows::SPACE.iter().enumerate() {
            psi[SPATIAL[j]] = at(k);
            psi[PARTNER[j]] = at(crate::flows::LOOP);
        }
        Orientation::at(&psi)
    }

    /// Reading `k` (phase layout) turned by `turns` (signed): a spatial
    /// plane by that turning composed whole, the circulation's readings
    /// moved together (a frame's residue handed down, a frame change).
    pub fn shift(&mut self, k: usize, turns: &Ratio) -> Result<(), TurningRefusal> {
        match crate::flows::SPACE.iter().position(|&s| s == k) {
            Some(j) => {
                let (_, rest) = whole_and_rest(turns);
                let mut b: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
                b[SPATIAL[j]] = signed_rate(&rest);
                self.advance(&b)
            }
            None => {
                let v = crate::flows::frac(&(self.phase(k) + turns.clone()));
                self.hold_phase(k, &v);
                Ok(())
            }
        }
    }

    /// The readings' antipode: every spatial reading half a turn on, held
    /// on its own branch (the phase layout's through-the-centre of the
    /// summed spin and the Möbius leg, until each reads its own plane).
    pub fn antipode(&mut self) {
        for &k in &crate::flows::SPACE {
            let v = crate::flows::frac(&(self.phase(k) + ratio(1, 2)));
            self.hold_phase(k, &v);
        }
    }

    /// Encoded for the save record: the six components, then the six
    /// windings, twelve exact values; where it moves off the continuum
    /// (docs/131), then the vacuum's three components and three windings and
    /// the scale, nineteen.
    pub fn values(&self) -> Vec<Ratio> {
        let mut v: Vec<Ratio> = self.g.to_vec();
        v.extend(self.windings.iter().map(|&n| ratio(n, 1)));
        if self.vacuum.iter().all(Zero::is_zero)
            && self.vacuum_windings == [0; 3]
            && self.scale.is_zero()
        {
            return v;
        }
        v.extend(self.vacuum.iter().cloned());
        v.extend(self.vacuum_windings.iter().map(|&n| ratio(n, 1)));
        v.push(self.scale.clone());
        v
    }

    /// Read back from [`Orientation::values`]; refused unless twelve or
    /// nineteen values with whole windings. Each component is held on the
    /// grain, as `G` is: a record written while `hold_phase` held exact
    /// readings carries them wide.
    pub fn from_values(v: &[Ratio]) -> Option<Orientation> {
        if v.len() != 12 && v.len() != 19 {
            return None;
        }
        let whole = |x: &Ratio| -> Option<i64> {
            x.is_integer()
                .then(|| i64::try_from(x.to_integer()).ok())
                .flatten()
        };
        let mut o = Orientation::default();
        for p in 0..6 {
            o.g[p] = crate::ring::grain(&v[p]);
            if !v[6 + p].is_integer() {
                return None;
            }
            o.windings[p] = i64::try_from(v[6 + p].to_integer()).ok()?;
        }
        if v.len() == 19 {
            for k in 0..3 {
                o.vacuum[k] = crate::ring::grain(&v[12 + k]);
                o.vacuum_windings[k] = whole(&v[15 + k])?;
            }
            o.scale = crate::ring::grain(&v[18]);
        }
        o.refresh();
        Some(o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v3(a: i64, b: i64, c: i64, d: i64) -> [Ratio; 3] {
        [ratio(a, d), ratio(b, d), ratio(c, d)]
    }

    #[test]
    fn a_ring_keeps_its_axis_under_its_own_motion() {
        // docs/128 §11.1: m± commutes with ℓ±, so the axis in space stands.
        let g: [Ratio; 6] = std::array::from_fn(|p| ratio(p as i64 + 1, 11));
        let Motion(p, q) = Motion::of(&g);
        let (plus, minus) = (v3(3, -1, 2, 5), v3(1, 4, -2, 7));
        for (h, spin) in [(p, plus), (q, minus)] {
            let b = ratio(3, 8);
            let m = Half::motion(&half_motion(&spin, &b));
            assert_eq!(h.product(&m).carry(&spin), h.carry(&spin));
        }
    }

    #[test]
    fn equal_halves_give_one_axis() {
        // No partner component: p = q, so equal spins carry to one axis.
        let mut g: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
        g[0] = ratio(1, 3);
        g[1] = ratio(-2, 7);
        g[2] = ratio(1, 5);
        let o = Orientation::of_generator(g, [0; 6]);
        let spin = v3(2, 1, -3, 4);
        let (a, b) = o.half_axes(&spin, &spin);
        assert_eq!(a, b);
        assert_ne!(a, spin, "the orientation carries the axis");
    }

    #[test]
    fn tilt_reads_its_limits_and_is_scale_free() {
        let up = v3(0, 0, 1, 1);
        let t = tilt(&v3(0, 0, 5, 2), &up).expect("aligned");
        assert_eq!((t.share, t.sense), (ratio(0, 1), ratio(1, 1)));
        let t = tilt(&v3(0, 0, -3, 1), &up).expect("reversed");
        assert_eq!((t.share, t.sense), (ratio(0, 1), ratio(-1, 1)));
        let t = tilt(&v3(1, -2, 0, 1), &up).expect("polar");
        assert_eq!((t.share, t.sense), (ratio(1, 1), ratio(0, 1)));
        let (a, r) = (v3(1, 2, 3, 1), v3(-1, 1, 4, 3));
        let s = tilt(&a, &r).expect("tilted");
        let scaled: [Ratio; 3] = std::array::from_fn(|k| a[k].clone() * ratio(7, 2));
        let r2: [Ratio; 3] = std::array::from_fn(|k| r[k].clone() * ratio(5, 1));
        assert_eq!(tilt(&scaled, &r2).expect("scaled").share, s.share);
        assert!(s.share > ratio(0, 1) && s.share < ratio(1, 1));
        assert!(tilt(&v3(0, 0, 0, 1), &up).is_none());
    }

    fn zero() -> [Ratio; 6] {
        std::array::from_fn(|_| ratio(0, 1))
    }

    #[test]
    fn the_rate_sends_rest_the_rim_and_the_throat_to_zero_one_and_none() {
        assert_eq!(rate(&ratio(0, 1)), Some(ratio(0, 1)));
        assert_eq!(rate(&ratio(1, 2)), Some(ratio(1, 1)));
        assert_eq!(rate(&ratio(1, 1)), None);
    }

    #[test]
    fn the_vacuum_inversion_is_the_reciprocal_rate() {
        for d in [ratio(1, 7), ratio(1, 4), ratio(2, 3), ratio(5, 9)] {
            let b = rate(&d).expect("held");
            let inv = rate(&(ratio(1, 1) - d)).expect("held");
            assert_eq!(b * inv, ratio(1, 1));
        }
    }

    #[test]
    fn one_ring_from_rest_reads_back_the_motion_law() {
        // On one plane, one ring at `δ` from rest reads `δ` turns exactly.
        for d in [ratio(1, 5), ratio(1, 2), ratio(3, 4)] {
            let mut o = Orientation::default();
            let mut b = zero();
            b[2] = rate(&d).expect("held");
            o.advance(&b).expect("composed");
            assert_eq!(o.readings()[2], d);
            assert_eq!(o.g[0], ratio(0, 1));
            assert_eq!(o.g[1], ratio(0, 1));
        }
    }

    #[test]
    fn two_rim_rings_are_one_whole_turn_wound() {
        // At the rim a ring is half a turn (`b = 1`); two make a whole turn
        // of the vector, `R = −1`, held as the identity with `K = 1`.
        let mut o = Orientation::default();
        let mut b = zero();
        b[1] = ratio(1, 1);
        o.advance(&b).expect("one ring");
        assert_eq!(o.readings()[1], ratio(1, 2));
        o.advance(&b).expect("two rings");
        assert_eq!(o.g, zero());
        assert_eq!(o.windings[1], 1);
    }

    #[test]
    fn planes_sharing_a_unit_gain_the_plane_between_in_order() {
        // Mycelium 11b-i through the orientation: `½` on `e₊₁e₊₂` then `⅓`
        // on `e₊₂e₊₃` gives `(16 e₁₂ + 12 e₁₃ + 9 e₂₃)/37`; reversed, the part
        // between changes sign. `e₃₁ = −e₁₃`.
        let mut first = zero();
        first[2] = ratio(1, 2);
        let mut then = zero();
        then[0] = ratio(1, 3);
        let mut ab = Orientation::default();
        ab.advance(&first).expect("first");
        ab.advance(&then).expect("then");
        let mut ba = Orientation::default();
        ba.advance(&then).expect("then");
        ba.advance(&first).expect("first");
        let g = |x: i64| crate::ring::grain(&ratio(x, 37));
        assert_eq!(ab.g[2], g(16));
        assert_eq!(ab.g[0], g(9));
        assert_eq!(ab.g[1], g(-12));
        assert_eq!(ba.g[1], g(12));
        assert_eq!(ab.g[3..], zero()[3..]);
    }

    #[test]
    fn disjoint_planes_keep_their_composite_rates() {
        // 11b-ii on `e₊₂e₊₃` and its R partner `e₊₁e₊R` (disjoint): `½` and
        // `⅓` read back as `4/7` and `3/7`, in either order.
        let mut a = zero();
        a[0] = ratio(1, 2);
        let mut c = zero();
        c[3] = ratio(1, 3);
        let mut o = Orientation::default();
        o.advance(&a).expect("a");
        o.advance(&c).expect("c");
        assert_eq!(o.g[0], crate::ring::grain(&ratio(4, 7)));
        assert_eq!(o.g[3], crate::ring::grain(&ratio(3, 7)));
    }

    #[test]
    fn a_lean_is_carried_by_planes_turned_alike() {
        // Founded isoclinic, equal rates keep the spatial planes equal (one
        // turning about the diagonal). A one-grain lean between two planes
        // (§11.5) is carried by that turning, never erased by it: the
        // planes stay apart by a part the composition turns with them.
        let b = {
            let mut b = zero();
            for p in SPATIAL {
                b[p] = ratio(1, 5);
            }
            b
        };
        let mut even = Orientation::default();
        let mut g0: [Ratio; 6] = zero();
        g0[0] = crate::ring::grain_step();
        let mut lean = Orientation::of_generator(g0, [0; 6]);
        for _ in 0..12 {
            even.advance(&b).expect("even");
            lean.advance(&b).expect("lean");
        }
        assert_eq!(even.g[0], even.g[1]);
        assert_eq!(even.g[1], even.g[2]);
        assert!(lean.g[0] != lean.g[1] || lean.g[1] != lean.g[2]);
    }

    #[test]
    fn the_closed_form_is_the_turning_law_value_for_value() {
        // The orientation's own arithmetic against `conductor::turning` (the
        // mirror of Mycelium's FORMS-TURNING v1) on mixed generators over all
        // six planes: the same composite generator, exactly.
        let cases: [[i64; 6]; 4] = [
            [1, 2, 3, 4, 5, 6],
            [-7, 3, 0, 2, -1, 5],
            [9, -4, 11, 0, 0, 3],
            [0, 0, 13, -6, 2, 0],
        ];
        for (n, a) in cases.iter().enumerate() {
            let b = cases[(n + 1) % cases.len()];
            let ga: [Ratio; 6] = std::array::from_fn(|p| ratio(a[p], 17));
            let gb: [Ratio; 6] = std::array::from_fn(|p| ratio(b[p], 23));
            let mirror =
                crate::turning::compose_rates(&bivector(&ga), &bivector(&gb)).expect("composed");
            let own = Motion::of(&ga)
                .then(&Motion::of(&gb))
                .generator()
                .expect("composed");
            assert_eq!(components(&mirror).expect("planes"), own, "case {n}");
            let composed = Motion::of(&ga).then(&Motion::of(&gb));
            let grained: [Ratio; 6] = std::array::from_fn(|p| crate::ring::grain(&own[p]));
            assert_eq!(
                composed.generator_on_grain().expect("on the grain"),
                grained
            );
        }
    }

    #[test]
    fn the_record_holds_the_orientation_and_refuses_a_flat_torus() {
        let mut o = Orientation::default();
        let mut b = zero();
        b[2] = ratio(1, 2);
        b[3] = ratio(1, 3);
        o.advance(&b).expect("composed");
        o.windings[4] = -2;
        assert_eq!(Orientation::from_values(&o.values()), Some(o));
        // Four per-plane phases, the record before §11.7, are refused.
        assert_eq!(Orientation::from_values(&vec![ratio(1, 3); 4]), None);
    }

    #[test]
    fn a_held_reading_is_exact_and_a_shift_composes() {
        let mut o =
            Orientation::from_phases(&[ratio(1, 60), ratio(1, 7), ratio(0, 1), ratio(3, 4)]);
        assert_eq!(o.phase(0), ratio(1, 60));
        assert_eq!(o.phase(crate::flows::LOOP), ratio(1, 7));
        o.hold_phase(0, &ratio(59, 60));
        assert_eq!(o.phase(0), ratio(59, 60));
        // Half a turn on a plane alone, twice, is a whole turn wound, and
        // the orientation is back where it stood (one cover).
        let mut alone = Orientation::default();
        alone.hold_phase(2, &ratio(1, 10));
        let was = alone.g.clone();
        alone.half_turn(2).expect("half");
        // The round half turn of `g = 1/9`: `(1/9 + 1)/(1 − 1/9) = 5/4`, on
        // the one cover `−4/5` (the L1 reading moves 5/9 − 1/10, not ½).
        assert_eq!(alone.g[1], ratio(-4, 5));
        alone.half_turn(2).expect("whole");
        assert_eq!(alone.windings[1], 1);
        assert_eq!(alone.g, was);
    }

    #[test]
    fn the_reading_inverts_on_the_forward_branch() {
        for psi in [ratio(0, 1), ratio(1, 3), ratio(1, 2), ratio(5, 6)] {
            assert_eq!(reading(&of_reading(&psi)), psi);
            assert!(of_reading(&psi).abs() <= ratio(1, 1), "one cover");
        }
        assert_eq!(reading(&ratio(1, 1)), ratio(1, 2));
        assert_eq!(reading(&ratio(-1, 1)), ratio(1, 2));
    }

    #[test]
    fn with_no_partner_spin_the_halves_move_alike_and_r_is_held() {
        // docs/128 §9.4: equal halves rotate the three spatial axes alone.
        let spins = [ratio(1, 4), ratio(1, 2), ratio(1, 4)];
        let (p, m) = spin_halves(&spins, &[]);
        assert_eq!(p, m);
        let b = ratio(1, 5);
        let (up, um) = (half_motion(&p, &b), half_motion(&m, &b));
        assert_eq!(size(&up), b, "a half moves by its one rate");
        let mut o = Orientation::default();
        o.bell_halves(&up, &um).expect("a motion");
        for &q in &PARTNER {
            assert!(o.g()[q].is_zero(), "the partners are held");
        }
        assert!(SPATIAL.iter().all(|&q| !o.g()[q].is_zero()));
    }

    #[test]
    fn partner_spin_parts_the_halves_and_exchanging_them_reverses_r() {
        let spins = [ratio(1, 4), ratio(1, 2), ratio(1, 4)];
        let partner = [ratio(1, 8), ratio(0, 1), ratio(-1, 16)];
        let (p, m) = spin_halves(&spins, &partner);
        assert_ne!(p, m, "the halves part");
        // Reversing every partner exchanges the halves.
        let back: Vec<Ratio> = partner.iter().map(|x| -x.clone()).collect();
        let (p2, m2) = spin_halves(&spins, &back);
        assert_eq!((p2.clone(), m2.clone()), (m.clone(), p.clone()));
        // The law, exact (the grain floors toward −∞, so a mirrored state
        // stands within a grain of the mirror once floored).
        let b = |h: &[Ratio; 3]| half_motion(h, &ratio(1, 6));
        let bell = |p: &[Ratio; 3], m: &[Ratio; 3]| {
            let step = Motion::of(&joined(p, m));
            step.then(&step).generator().expect("a generator")
        };
        let (o, r) = (bell(&b(&p), &b(&m)), bell(&b(&p2), &b(&m2)));
        for k in 0..3 {
            assert_eq!(o[SPATIAL[k]], r[SPATIAL[k]], "spatial kept");
            assert_eq!(o[PARTNER[k]], -r[PARTNER[k]].clone(), "partners reversed");
        }
        assert!(PARTNER.iter().any(|&q| !o[q].is_zero()), "R turns");
    }

    #[test]
    fn the_bell_is_the_groups_product_not_the_charts_sum() {
        // Two rings composed, m·m, against one ring at the doubled rate.
        let u = [ratio(1, 3), ratio(1, 5), ratio(0, 1)];
        let w = [ratio(1, 7), ratio(0, 1), ratio(1, 4)];
        let mut o = Orientation::default();
        o.bell_halves(&u, &w).expect("a motion");
        let joined_b = joined(&u, &w);
        let doubled: [Ratio; 6] =
            std::array::from_fn(|p| crate::ring::grain(&(ratio(2, 1) * joined_b[p].clone())));
        let mut flat = Orientation::default();
        flat.advance_by(&doubled, 1).expect("a motion");
        assert_ne!(o.g(), flat.g(), "composition is not the doubled rate");
    }

    #[test]
    fn a_half_with_no_spin_does_not_move() {
        let none = std::array::from_fn(|_| ratio(0, 1));
        assert_eq!(half_motion(&none, &ratio(3, 1)), none);
    }

    fn unit_from(u: &[Ratio; 3]) -> Half {
        Half::motion(u)
    }

    fn is_one(h: &Half) -> bool {
        h.n[0] == h.d && h.n[1..].iter().all(Zero::is_zero)
    }

    #[test]
    fn the_state_factors_exactly_into_form_and_twist() {
        // docs/128 §10.0: p·x·q̄ = C·(q·x·q̄); (p, q) → (F, C) → (p, q).
        let p = unit_from(&[ratio(1, 3), ratio(-1, 5), ratio(2, 7)]);
        let q = unit_from(&[ratio(1, 4), ratio(1, 6), ratio(0, 1)]);
        let (f, c) = (q.clone(), p.product(&q.conjugate()));
        let back = c.product(&f);
        assert_eq!(back.ratios(), p.ratios(), "p = C·F");
        // The sandwich on a vector, exact.
        let x = Half::of_ratios(&[ratio(2, 3), ratio(1, 2), ratio(-1, 9), ratio(1, 1)]);
        let lhs = p.product(&x).product(&q.conjugate()).ratios();
        let rhs = c.product(&f.product(&x).product(&f.conjugate())).ratios();
        assert_eq!(lhs, rhs);
        // The form fixes R (the real unit) and the twist of equal halves is one.
        let one = Half::of_ratios(&[ratio(1, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1)]);
        assert_eq!(
            f.product(&one).product(&f.conjugate()).ratios(),
            one.ratios()
        );
        assert!(is_one(&q.product(&q.conjugate())));
    }

    #[test]
    fn form_and_twist_are_unitary_with_no_norm_read() {
        let p = unit_from(&[ratio(3, 4), ratio(0, 1), ratio(-5, 2)]);
        let q = unit_from(&[ratio(1, 9), ratio(2, 3), ratio(1, 1)]);
        let c = p.product(&q.conjugate());
        assert!(is_one(&q.product(&q.conjugate())), "F·F̄ = 1");
        assert!(is_one(&c.product(&c.conjugate())), "C·C̄ = 1");
    }

    #[test]
    fn a_bell_moves_the_form_by_its_orbit_and_the_twist_by_the_halves_difference() {
        // docs/128 §10.2: F ↦ F·m−, C ↦ C·F·(m+·m̄−)·F̄; equal halves keep C.
        let p = unit_from(&[ratio(1, 3), ratio(1, 5), ratio(0, 1)]);
        let q = unit_from(&[ratio(-1, 4), ratio(0, 1), ratio(1, 6)]);
        let (mp, mm) = (
            unit_from(&[ratio(1, 11), ratio(0, 1), ratio(1, 13)]),
            unit_from(&[ratio(0, 1), ratio(1, 17), ratio(0, 1)]),
        );
        let (f, c) = (q.clone(), p.product(&q.conjugate()));
        let (p2, q2) = (p.product(&mp), q.product(&mm));
        let c2 = p2.product(&q2.conjugate());
        let rel = mp.product(&mm.conjugate());
        let carried = c.product(&f.product(&rel).product(&f.conjugate()));
        assert_eq!(c2.ratios(), carried.ratios(), "the twist's motion");
        assert_eq!(q2.ratios(), f.product(&mm).ratios(), "the form's orbit");
        // Equal halves: the twist is kept.
        let (p3, q3) = (p.product(&mm), q.product(&mm));
        assert_eq!(p3.product(&q3.conjugate()).ratios(), c.ratios());
    }

    #[test]
    fn exchanging_the_halves_conjugates_the_twist() {
        // The inversion (#241, docs/104 §0): (p, q) ↦ (q, p) takes C ↦ C̄.
        let p = unit_from(&[ratio(2, 5), ratio(-1, 3), ratio(1, 8)]);
        let q = unit_from(&[ratio(1, 7), ratio(1, 2), ratio(0, 1)]);
        let c = p.product(&q.conjugate());
        assert_eq!(q.product(&p.conjugate()).ratios(), c.conjugate().ratios());
    }

    #[test]
    fn an_orientation_reads_its_state_and_is_read_back_from_it() {
        let o = Orientation::of_generator(
            [
                ratio(1, 3),
                ratio(-1, 5),
                ratio(1, 7),
                ratio(1, 9),
                ratio(0, 1),
                ratio(-1, 4),
            ],
            [0; 6],
        );
        let (f, c) = o.form_twist();
        let back = Orientation::of_form_twist(&f, &c).expect("a generator");
        // Exact up to the grain the generator is held on.
        for k in 0..6 {
            let d = (back.g()[k].clone() - o.g()[k].clone()).abs();
            assert!(d <= crate::ring::grain_step(), "plane {k} within a grain");
        }
    }
}

#[cfg(test)]
mod grain_tests {
    use super::*;

    #[test]
    fn a_held_reading_stays_on_the_grain_bell_on_bell() {
        // The scalar collapse the ablation probe found: held readings on the
        // grain and the R-planes' scaling on it keep G's denominators bounded.
        let mut o =
            Orientation::from_phases(&[ratio(1, 7), ratio(2, 11), ratio(3, 13), ratio(5, 17)]);
        for b in 1..40i64 {
            for k in 0..crate::flows::PLANES {
                o.hold_phase(k, &ratio(b * (k as i64 + 3), 101 + b));
            }
        }
        // A spatial generator `a/(N − a)` of a reading on the grain `a/N`, and
        // an R-plane on the grain: every denominator stays within `N = 60⁶`.
        let n = num_bigint::BigInt::from(60).pow(6);
        for g in o.g() {
            assert!(*g.denom() <= n, "bounded: {g}");
        }
        let read = Orientation::from_values(&o.values()).expect("a record");
        for (a, b) in read.g().iter().zip(o.g()) {
            let d = a.clone() - b.clone();
            assert!(
                d.abs() <= crate::ring::grain_step(),
                "read back within a grain"
            );
        }
    }
}

#[cfg(test)]
mod cl54_tests {
    //! The state's algebra, Cl(5,4) over ℚ (docs/131 §8).
    use super::*;

    fn one(mask: usize, c: Ratio) -> Clifford {
        Clifford::blade(NEGATIVES, mask, c)
    }

    fn part(x: &Clifford, mask: usize) -> Ratio {
        x.terms()
            .find(|(m, _)| *m == mask)
            .map(|(_, c)| c.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// Witness 1: each pair's character read from its axes' squares, 16
    /// turnings (10 positive, 6 negative) and 20 boosts.
    #[test]
    fn sixteen_turnings_and_twenty_boosts_by_the_squares() {
        let all = pairs();
        assert_eq!(all.len(), 36);
        let turning: Vec<usize> = all.iter().copied().filter(|&m| is_turning(m)).collect();
        assert_eq!(turning.len(), 16);
        let negative = turning.iter().filter(|&&m| m & NEGATIVES == m).count();
        assert_eq!((turning.len() - negative, negative), (10, 6));
        // The built planes are continuum turnings; each plane's own
        // dilation and the conformal dilator are boosts.
        assert!((0..6).all(|p| is_turning(blade(p).0)));
        assert!((0..3).all(|k| !is_turning(WHOLE[k] | SPAN[k])));
        assert!(!is_turning(UNIVERSE_WHOLE | UNIVERSE_SPAN));
        // The vacuum's orbit about axis 1 is a turning on the spans.
        assert!(is_turning(SPAN[1] | SPAN[2]));
    }

    /// Witness 4: the pole swap is an involution, and it reverses a plane's
    /// span while keeping its whole.
    #[test]
    fn the_pole_swap_is_an_involution_reversing_the_span() {
        let (z, q) = (ratio(3, 1), ratio(1, 1));
        let x = one(WHOLE[0], z.clone() + q.clone())
            .sum(&one(SPAN[0], z.clone() - q.clone()))
            .expect("one signature");
        let swapped = pole_swap(0, &x);
        assert_eq!(part(&swapped, WHOLE[0]), z.clone() + q.clone());
        assert_eq!(part(&swapped, SPAN[0]), q - z);
        assert_eq!(pole_swap(0, &swapped), x);
        let g = one(WHOLE[1] | SPAN[0], ratio(1, 2))
            .sum(&one(SPAN[0] | SPAN[2], ratio(1, 3)))
            .expect("one signature");
        assert_eq!(pole_swap(0, &pole_swap(0, &g)), g);
    }

    /// The general form's components, each on the grain on its blade's
    /// own sense.
    fn on_grain(x: &Clifford) -> Vec<(usize, Ratio)> {
        x.terms().map(|(m, c)| (m, crate::ring::grain(c))).collect()
    }

    /// Witness 2: a motion on the built planes composes in closed form to
    /// the general Cayley form's components on Cl(5,4).
    #[test]
    fn the_built_motion_is_the_general_form_on_its_subspace() {
        let g: [Ratio; 6] = std::array::from_fn(|p| crate::ring::grain(&ratio(p as i64 + 1, 7)));
        let b: [Ratio; 6] = std::array::from_fn(|p| crate::ring::grain(&ratio(1, p as i64 + 5)));
        let mut o = Orientation::of_generator(g.clone(), [0; 6]);
        o.advance(&b).expect("a motion");
        // Held on the one cover, as the orientation holds it.
        let r = crate::turning::compose(
            &crate::turning::motion(&bivector(&g)).expect("a motion"),
            &crate::turning::motion(&bivector(&b)).expect("a motion"),
        )
        .expect("composed");
        let r = if part(&r, 0) < ratio(0, 1) {
            one(0, ratio(0, 1)).difference(&r).expect("one signature")
        } else {
            r
        };
        let general = crate::turning::generator(&r).expect("a generator");
        let general: [Ratio; 6] = std::array::from_fn(|p| {
            let (m, sign) = blade(p);
            crate::ring::grain(&(sign * part(&general, m)))
        });
        assert_eq!(o.g(), &general);
    }

    /// The vacuum's factor in closed form is the general form on the spans,
    /// with the inverted face's hand.
    #[test]
    fn the_vacuum_factor_is_the_general_form_on_the_spans() {
        let v: [Ratio; 3] = [ratio(1, 3), ratio(-1, 5), ratio(2, 7)];
        let b: [Ratio; 3] = [ratio(1, 4), ratio(1, 6), ratio(-1, 9)];
        let mut o = Orientation::default();
        o.advance_vacuum(&v).expect("a motion");
        let start = o.factors()[1].clone();
        assert_eq!(on_grain(&start), on_grain(&o.factors()[1]));
        o.advance_vacuum(&b).expect("a motion");
        let mut step = Orientation::default();
        step.advance_vacuum(&b).expect("a motion");
        let general = crate::turning::compose_rates(&start, &step.factors()[1]).expect("composed");
        let read: [Ratio; 3] = std::array::from_fn(|k| {
            let (m, sign) = vacuum_blade(k);
            crate::ring::grain(&(sign * part(&general, m)))
        });
        assert_eq!(o.vacuum(), &read);
        assert!((0..3).all(|k| is_turning(vacuum_blade(k).0)));
    }

    /// The three factors stand on disjoint axes and commute; the scale
    /// composes by the flow law and never winds; the record reads back.
    #[test]
    fn the_factors_commute_and_the_record_reads_back() {
        let mut o = Orientation::of_generator(
            std::array::from_fn(|p| crate::ring::grain(&ratio(1, p as i64 + 3))),
            [0; 6],
        );
        o.advance_vacuum(&[ratio(1, 5), ratio(0, 1), ratio(1, 7)])
            .expect("a motion");
        o.advance_scale(&ratio(1, 3)).expect("a boost");
        o.advance_scale(&ratio(1, 2)).expect("a boost");
        assert_eq!(o.scale(), &crate::ring::grain(&ratio(5, 7)));
        let f = o.factors();
        let m: Vec<Clifford> = f
            .iter()
            .map(|x| crate::turning::motion(x).expect("a motion"))
            .collect();
        for a in 0..3 {
            for b in 0..3 {
                assert_eq!(
                    m[a].product(&m[b]).expect("product"),
                    m[b].product(&m[a]).expect("product")
                );
            }
        }
        assert_eq!(o.values().len(), 19);
        assert_eq!(Orientation::from_values(&o.values()), Some(o.clone()));
        assert_eq!(o.advance_scale(&ratio(1, 1)), Err(TurningRefusal::NoMotion));
    }

    /// Witness 3: a continuum turning carried by equal dilations of its two
    /// planes gains a part on the vacuum's turning, its size by the reverse
    /// product kept.
    #[test]
    fn spin_passes_to_the_vacuum_by_the_planes_dilation_kept_in_size() {
        let l = one(WHOLE[1] | WHOLE[2], ratio(1, 1));
        let boost = one(WHOLE[1] | SPAN[1], ratio(1, 3))
            .sum(&one(WHOLE[2] | SPAN[2], ratio(1, 3)))
            .expect("one signature");
        let m = crate::turning::motion(&boost).expect("a motion");
        let c = carried(&m, &l).expect("carried");
        assert!(!part(&c, SPAN[1] | SPAN[2]).is_zero(), "a vacuum part");
        assert!(!part(&c, WHOLE[1] | WHOLE[2]).is_zero(), "a continuum part");
        let size = |x: &Clifford| x.product(&crate::turning::reversed(x)).expect("product");
        assert_eq!(size(&c), size(&l));
    }

    /// Witness 5: the conformal dilator composes by the flow law.
    #[test]
    fn the_conformal_dilator_composes_by_the_flow_law() {
        let e = one(UNIVERSE_WHOLE | UNIVERSE_SPAN, ratio(1, 1));
        assert_eq!(Regime::of(&e), Ok(Regime::Hyperbolic));
        let (u1, u2) = (ratio(1, 2), ratio(1, 3));
        let rate = |u: &Ratio| one(UNIVERSE_WHOLE | UNIVERSE_SPAN, u.clone());
        let composed = crate::turning::compose_rates(&rate(&u1), &rate(&u2)).expect("composes");
        let flow = (u1.clone() + u2.clone()) / (ratio(1, 1) + u1 * u2);
        assert_eq!(composed, rate(&flow));
    }
}
