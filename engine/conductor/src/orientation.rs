//! H's orientation held as the generator (docs/117 §11.7, #117 step 2):
//! a body's turning state is one generator `G` over the four continuum
//! axes `e₊₁, e₊₂, e₊₃, e₊R`, so six elliptic planes, with its windings `K`
//! per plane, never one angle per plane.
//!
//! - **The planes** ([`PLANES`]): the three spatial planes `e₊₂e₊₃`,
//!   `e₊₃e₊₁`, `e₊₁e₊₂` (in `flows::SPACE` order), then their R-plane
//!   partners, the dual pairs of §11.6, `e₊₁e₊R`, `e₊₂e₊R`, `e₊₃e₊R`.
//! - **The rate a ring** ([`rate`]): `b = δ/(1 − δ) = H/R`, the hold over
//!   the reach, the one linear-fractional map sending rest, the rim and the
//!   throat (`δ = 0, ½, 1`) to the Cayley rates `0, 1, ∞` (§11.7.3). The
//!   vacuum's inversion `δ ↦ 1 − δ` is the reciprocal `b ↦ 1/b`.
//! - **The advance** ([`Orientation::advance`]): `G ← generator(cayley(G)·
//!   cayley(B))`, whole, by `conductor::turning`'s composition, never plane
//!   by plane; its parts on the planes between are made by the composition
//!   (register K6 retired). Each component is held on the grain; any
//!   bivector is a generator, so the grain never breaks the motion.
//! - **One cover, and the windings:** `R` and `−R` turn every vector alike,
//!   so the motion is held as the one whose scalar part is not negative (a
//!   plane alone then stands at `|g| ≤ 1`); a whole turn gives the
//!   components back. A plane's `K` rises by one each time its reading
//!   passes 0 under a forward drive and falls on a pass backward: exact for
//!   a plane turned alone (`K = ⌊θ/360°⌋`, §11.1), a reading of the plane's
//!   component where other planes turn with it (78.173 §8.3: windings stay
//!   on the planes that wound them).
//! - **The reading the laws act on** ([`reading`]): `ψ = g/(1 + |g|)` turns,
//!   taken into `[0, 1)`, the L1 reading §8d names, rational, exact at the
//!   hemisphere kinks (`g = 0` and `g = 1` are `0` and `½` a turn), and one
//!   ring from rest gives back the motion law's `δ` exactly. Degrees
//!   (`tan(θ/4) = g`) are rendered on observation only.
use crate::turning::{base_axis, Clifford, TurningRefusal};
use cfe::ratio::{ratio, Ratio};
use num_traits::{Signed, Zero};

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
    let mut g = Clifford::blade(0, 0, ratio(0, 1));
    for (p, x) in c.iter().enumerate() {
        if x.is_zero() {
            continue;
        }
        let (mask, sign) = blade(p);
        g = g
            .sum(&Clifford::blade(0, mask, sign * x.clone()))
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

/// A body's orientation: its generator's components on [`PLANES`], each on
/// the grain, and its windings per plane.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Orientation {
    pub g: [Ratio; 6],
    pub windings: [i64; 6],
}

impl Orientation {
    /// At the readings `psi` (turns) on each plane, unwound: the founding's
    /// form, each plane on its own (no part between planes yet), exact (a
    /// reading `a/q` is the component `a/(q − a)`, its denominator bounded
    /// by the reading's).
    pub fn at(psi: &[Ratio; 6]) -> Orientation {
        Orientation {
            g: std::array::from_fn(|p| of_reading(&psi[p])),
            windings: [0; 6],
        }
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
        if b.iter().all(Zero::is_zero) {
            return Ok(());
        }
        let was = self.readings();
        let r = crate::turning::motion(&bivector(&self.g))?
            .product(&crate::turning::motion(&bivector(b))?)?;
        // One cover: `R` and `−R` turn every vector alike, so the motion is
        // held as the one whose scalar part is not negative; a whole turn of
        // the vector (`R ↦ −R`) then gives the components back and is
        // counted in `K`, never carried in `G`.
        let minus = Clifford::blade(0, 0, ratio(-1, 1));
        let scalar = r
            .terms()
            .find(|(m, _)| *m == 0)
            .map(|(_, c)| c.clone())
            .unwrap_or_else(|| ratio(0, 1));
        let r = if scalar < ratio(0, 1) {
            r.product(&minus)?
        } else {
            r
        };
        let g = match crate::turning::generator(&r) {
            Ok(g) => g,
            // `R + 1` singular on one plane: held as `−R`, the same sandwich.
            Err(TurningRefusal::NoGenerator) => crate::turning::generator(&r.product(&minus)?)?,
            Err(e) => return Err(e),
        };
        let g = components(&g)?;
        self.g = std::array::from_fn(|p| crate::ring::grain(&g[p]));
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
        let mut v = vec![ratio(0, 1); crate::flows::PLANES];
        for (j, &k) in crate::flows::SPACE.iter().enumerate() {
            v[k] = reading(&self.g[SPATIAL[j]]);
        }
        v[crate::flows::LOOP] = reading(&self.loop_mean());
        v
    }

    /// One reading in the phase layout.
    pub fn phase(&self, k: usize) -> Ratio {
        match crate::flows::SPACE.iter().position(|&s| s == k) {
            Some(j) => reading(&self.g[SPATIAL[j]]),
            None => reading(&self.loop_mean()),
        }
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
    /// mean whose reading is `v` (each at it where none stands). Exact: a
    /// reading held to a place has a bounded denominator, and every
    /// composition floors onto the grain again.
    pub fn hold_phase(&mut self, k: usize, v: &Ratio) {
        match crate::flows::SPACE.iter().position(|&s| s == k) {
            Some(j) => {
                let p = SPATIAL[j];
                if reading(&self.g[p]) != *v {
                    self.g[p] = of_reading(v);
                }
            }
            None => {
                let mean = self.loop_mean();
                let to = of_reading(v);
                if to == mean {
                    return;
                }
                for p in PARTNER {
                    self.g[p] = if mean.is_zero() {
                        to.clone()
                    } else {
                        self.g[p].clone() * to.clone() / mean.clone()
                    };
                }
            }
        }
    }

    /// One ring of a body's turning, in the phase layout's turns: each
    /// spatial plane by its own `space[j]` turns, the circulation by `circ`
    /// turns on each R-plane alike (one circulation seen in every pair, its
    /// spin shared by the tilt where the swing reads it). Whole turns wind
    /// the planes they turn on; the rest composes whole.
    pub fn ring(&mut self, space: &[Ratio], circ: &Ratio) -> Result<(), TurningRefusal> {
        let mut b: [Ratio; 6] = std::array::from_fn(|_| ratio(0, 1));
        for j in 0..3 {
            let (n, rest) = whole_and_rest(space.get(j).unwrap_or(&ratio(0, 1)));
            self.windings[SPATIAL[j]] += n;
            b[SPATIAL[j]] = signed_rate(&rest);
        }
        let (n, rest) = whole_and_rest(circ);
        let c = signed_rate(&rest);
        for p in PARTNER {
            self.windings[p] += n;
            b[p] = c.clone();
        }
        self.advance(&b)
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
        if let Some((o, _)) = parts.iter().max_by(|a, b| a.1.cmp(&b.1)) {
            out.windings = o.windings;
        }
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
    /// windings, twelve exact values.
    pub fn values(&self) -> Vec<Ratio> {
        let mut v: Vec<Ratio> = self.g.to_vec();
        v.extend(self.windings.iter().map(|&n| ratio(n, 1)));
        v
    }

    /// Read back from [`Orientation::values`]; refused unless twelve
    /// values with whole windings.
    pub fn from_values(v: &[Ratio]) -> Option<Orientation> {
        if v.len() != 12 {
            return None;
        }
        let mut o = Orientation::default();
        for p in 0..6 {
            o.g[p] = v[p].clone();
            if !v[6 + p].is_integer() {
                return None;
            }
            o.windings[p] = i64::try_from(v[6 + p].to_integer()).ok()?;
        }
        Some(o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut lean = Orientation::default();
        lean.g[0] = crate::ring::grain_step();
        for _ in 0..12 {
            even.advance(&b).expect("even");
            lean.advance(&b).expect("lean");
        }
        assert_eq!(even.g[0], even.g[1]);
        assert_eq!(even.g[1], even.g[2]);
        assert!(lean.g[0] != lean.g[1] || lean.g[1] != lean.g[2]);
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
}
