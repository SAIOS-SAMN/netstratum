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
//! - **The windings:** a plane's `K` rises by one each time its turning
//!   passes a whole turn of the vector (its reading passes 0 forward) and
//!   falls on a pass backward; where the composite is `R = −1` on a plane
//!   the orientation is held as `−R`'s generator, the same sandwich.
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

/// The component whose reading is `ψ ∈ [0, 1)`: `ψ/(1 − ψ)`, on its
/// forward branch (the founding's, where a phase was given in turns).
pub fn of_reading(psi: &Ratio) -> Ratio {
    let psi = psi.clone() - psi.floor();
    psi.clone() / (ratio(1, 1) - psi)
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
    /// form, each plane on its own (no part between planes yet).
    pub fn at(psi: &[Ratio; 6]) -> Orientation {
        Orientation {
            g: std::array::from_fn(|p| crate::ring::grain(&of_reading(&psi[p]))),
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
        let g = match crate::turning::generator(&r) {
            Ok(g) => g,
            // A whole turn of the vector on a plane: `R = −1` there, held
            // as `−R`, the same sandwich.
            Err(TurningRefusal::NoGenerator) => {
                let minus = Clifford::blade(0, 0, ratio(-1, 1));
                crate::turning::generator(&r.product(&minus)?)?
            }
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
    fn a_lean_separates_planes_turned_alike() {
        // Founded isoclinic, equal rates keep the spatial planes equal; a
        // one-grain lean between two planes (§11.5) makes the turning of
        // equal rates part them, which per-plane sums never did.
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
        let spread = |o: &Orientation| (o.g[0].clone() - o.g[2].clone()).abs();
        assert!(spread(&lean) > crate::ring::grain_step());
    }

    #[test]
    fn the_reading_inverts_on_the_forward_branch() {
        for psi in [ratio(0, 1), ratio(1, 3), ratio(1, 2), ratio(5, 6)] {
            assert_eq!(reading(&of_reading(&psi)), psi);
        }
        assert_eq!(reading(&ratio(1, 1)), ratio(1, 2));
        assert_eq!(reading(&ratio(-1, 1)), ratio(1, 2));
    }
}
