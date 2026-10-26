//! THE NUCLIDES (docs/97; operator 2026-10-26: "built as the inverse of
//! xylarium where Q is the proton and Z is the neutron and R is the phase
//! state of the electron"). A kind's atom is a NUCLIDE named by its weight
//! `w = Q + Z`, its nucleons. Its protons and neutrons are read off the
//! nucleus's convergence on S^11 (xylarium's fibres, inverted), never a
//! table:
//!
//! - nucleon ends arrive one at a time on S^11, whose degree-l harmonics give
//!   `d(l) = C(l + 11, 11) − C(l + 9, 11)` independent directions;
//! - an end first claims a free direction ALONE: a lone end is open, a PROTON;
//! - once every direction of the open degree is claimed, the next end BINDS
//!   into a lone end: a FIBRE, one proton and one neutron counter-turning;
//! - a degree closes when every direction holds a fibre; turning n holds the
//!   degrees 0..n − 1.
//!
//! So weight w has `Z = G` neutrons (its fibres) and `Q = G + free` protons,
//! one nuclide a weight; its COHERENCE is the bound share `κ = 2G/w`, 1 at
//! every closed degree (the seals 2, 28, 208, 1092 among them).

/// The weights read (the convergence is computed once, exact, to here).
pub const READ: u32 = 8192;

fn choose(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    (0..k).fold(1u64, |acc, i| acc * (n - i) / (i + 1))
}

/// The independent directions of harmonic degree `l` on S^11.
pub fn directions(l: u64) -> u64 {
    choose(l + 11, 11) - if l >= 2 { choose(l + 9, 11) } else { 0 }
}

/// The nucleus of weight `w` (index w − 1): (protons Q, neutrons Z).
fn table() -> &'static [(u32, u32)] {
    static NUCLEI: std::sync::OnceLock<Vec<(u32, u32)>> = std::sync::OnceLock::new();
    NUCLEI.get_or_init(|| {
        let mut out = Vec::with_capacity(READ as usize);
        let (mut fibres, mut free) = (0u32, 0u32);
        let (mut turning, mut degree) = (1u64, 0u64);
        let (mut lone, mut bound) = (0u64, 0u64);
        for _ in 0..READ {
            let dirs = directions(degree);
            if lone + bound < dirs {
                lone += 1;
                free += 1;
            } else {
                lone -= 1;
                bound += 1;
                free -= 1;
                fibres += 1;
            }
            if bound == dirs {
                degree += 1;
                lone = 0;
                bound = 0;
                if degree == turning {
                    turning += 1;
                    degree = 0;
                }
            }
            out.push((fibres + free, fibres));
        }
        out
    })
}

/// The protons of the nuclide of weight `w` (its charge); None past the read
/// or at w = 0.
pub fn protons(w: u32) -> Option<u32> {
    table().get(usize::try_from(w).ok()?.checked_sub(1)?).map(|(q, _)| *q)
}

/// The neutrons of the nuclide of weight `w`.
pub fn neutrons(w: u32) -> Option<u32> {
    table().get(usize::try_from(w).ok()?.checked_sub(1)?).map(|(_, z)| *z)
}

/// The nucleus's coherence, its bound share `κ = 2Z/w`.
pub fn coherence(w: u32) -> Option<cfe::ratio::Ratio> {
    Some(cfe::ratio::ratio(i64::from(2 * neutrons(w)?), i64::from(w)))
}

/// Whether the lone end at weight `w` was a capture of a NEUTRON (it bound),
/// else of a proton (it claimed a direction alone).
pub fn binds(w: u32) -> Option<bool> {
    let before = if w <= 1 { 0 } else { neutrons(w - 1)? };
    Some(neutrons(w)? > before)
}

/// The lightest weight whose nuclide carries `q` protons (the element of
/// charge q, its first isotope).
pub fn of_protons(q: u32) -> Option<u32> {
    let i = table().partition_point(|(p, _)| *p < q);
    let w = u32::try_from(i).ok()? + 1;
    (protons(w)? == q).then_some(w)
}

/// A NUCLEON'S POLE IS ITS LEG (the automorphic component; docs/97 §8): on
/// the continuum leg (open, R forward) it is a proton, on the vacuum leg
/// (held, R backward) a neutron.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pole {
    Proton,
    Neutron,
}

/// THE CONVERSION IS THE MÖBIUS PASS (`flows::mobius`'s map on a nucleon's
/// pole): through the ring the leg turns, so the pole does; twice is the
/// identity.
pub fn mobius(p: Pole) -> Pole {
    match p {
        Pole::Proton => Pole::Neutron,
        Pole::Neutron => Pole::Proton,
    }
}

/// THE SWAP Q ↔ Z read on the convergence (the automorphism of the nucleus):
/// a fibre is one proton and one neutron, so exchanging the poles of every
/// fibre leaves the fibres, and only the lone ends change pole. Returns the
/// swapped nuclide's (protons, neutrons): (Z, Q).
pub fn swapped(w: u32) -> Option<(u32, u32)> {
    Some((neutrons(w)?, protons(w)?))
}

/// ONE PASS OF THE FOUNDING'S EXCHANGE (the unitary component; docs/97 §8):
/// of `p` protons and `n` neutrons at a pass where the bounce's bubble holds
/// the face δ, protons cross to the vacuum leg by the held face, `p·δ`, and
/// neutrons to the continuum by the unlocked face, `n·½(1 − δ)`, both in
/// whole nucleons, both booked: `p + n` is unchanged. Returns (p, n) after.
pub fn exchange(p: &num_bigint::BigInt, n: &num_bigint::BigInt, delta: &cfe::ratio::Ratio) -> (num_bigint::BigInt, num_bigint::BigInt) {
    use cfe::ratio::{ratio, Ratio};
    let whole = |x: Ratio| x.floor().to_integer();
    let to_n = whole(Ratio::from_integer(p.clone()) * delta.clone());
    let to_p = whole(Ratio::from_integer(n.clone()) * (ratio(1, 1) - delta.clone()) / ratio(2, 1));
    (p - &to_n + &to_p, n - &to_p + &to_n)
}

/// THE BOUNCE'S HEAT (docs/97 §9): a body at deficit δ turns `2δ` of its
/// orbit a bell; the luminal bound allows one whole turn, so inside the ring
/// the excess `2δ − 1` cannot be motion and is agitation:
/// `a = max(0, 2δ − 1) = (H − R)/(H + R)`, 1 at the core, 0 at the ring,
/// cooling as the bubble expands (the substrate weight's form, first degree).
pub fn bounce_heat(delta: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    (ratio(2, 1) * delta.clone() - ratio(1, 1)).max(ratio(0, 1))
}

/// THE SEAL WINDOW (docs/97 §9): a fibre's two counter-turning ends part
/// where the heat reaches half a turn (the ignition law's bound, `a ≥ ½`,
/// one tier down), so deuterium's seal first holds as the bounce's heat
/// falls to ½: `2δ − 1 = ½`, δ = ¾.
pub fn seal_window() -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    // Solve bounce_heat(δ) = ½ exactly.
    (ratio(1, 2) + ratio(1, 1)) / ratio(2, 1)
}

/// THE EXCHANGE'S FIXED POINT (docs/97 §9): where the pass's two booked
/// directions balance, `p·δ = n·½(1 − δ)`, the neutron share is
/// `2δ/(1 + δ)`; read at the seal window, never set.
pub fn neutron_share(delta: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    ratio(2, 1) * delta.clone() / (ratio(1, 1) + delta.clone())
}

/// THE ISOMORPHIC READING (docs/97 §8): a founding's outcome set beside
/// nature's own, never imposed: (neutrons per proton among the nucleons,
/// the helium-weight mass share, hydrogen's mass share). Nature reads about
/// 1/7, 1/4 and 3/4.
pub fn beside_nature(m: &crate::pulse::Manifest) -> Option<(cfe::ratio::Ratio, cfe::ratio::Ratio, cfe::ratio::Ratio)> {
    use cfe::ratio::ratio;
    use num_traits::Zero;
    let (mut q, mut z, mut whole, mut alpha, mut h) = (ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1));
    for (k, a) in m {
        for &(n, w) in k.parts() {
            let count = a.clone() * ratio(i64::from(n), 1);
            q += count.clone() * ratio(i64::from(protons(w)?), 1);
            z += count.clone() * ratio(i64::from(neutrons(w)?), 1);
            let mw = count * ratio(i64::from(w), 1);
            whole += mw.clone();
            if w == 4 {
                alpha += mw.clone();
            }
            if w == 1 {
                h += mw;
            }
        }
    }
    if q.is_zero() || whole.is_zero() {
        return None;
    }
    Some((z / q, alpha / whole.clone(), h / whole))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cfe::ratio::ratio;

    #[test]
    fn the_nucleus_converges_on_s11_and_seals_where_every_degree_is_bound() {
        // Hydrogen is a lone proton; deuterium seals the first turning.
        assert_eq!((protons(1), neutrons(1)), (Some(1), Some(0)));
        assert_eq!((protons(2), neutrons(2)), (Some(1), Some(1)));
        // The seals, read off the process: every degree bound, Q = Z.
        for w in [2u32, 28, 208, 1092] {
            assert_eq!(protons(w), neutrons(w), "weight {w} seals");
            assert_eq!(coherence(w), Some(ratio(1, 1)));
        }
        // Weight 1200, past the fourth seal.
        assert_eq!((protons(1200), neutrons(1200)), (Some(636), Some(564)));
        // Nucleons always: Q + Z = w.
        for w in 1..=1300 {
            assert_eq!(protons(w).zip(neutrons(w)).map(|(q, z)| q + z), Some(w));
        }
        // The element of charge q, its lightest isotope.
        assert_eq!(of_protons(1), Some(1));
        assert_eq!(of_protons(2), Some(3));
        assert_eq!(binds(2), Some(true));
        assert_eq!(binds(3), Some(false));
    }

    #[test]
    fn the_three_components_stand_apart_and_each_holds() {
        // AUTOMORPHIC: the Möbius pass is an involution on a nucleon's pole,
        // and the swap Q ↔ Z maps every seal onto itself.
        for p in [Pole::Proton, Pole::Neutron] {
            assert_eq!(mobius(mobius(p)), p);
        }
        for w in [2u32, 4, 28, 208, 1092] {
            assert_eq!(swapped(w), protons(w).zip(neutrons(w)), "seal {w} is its own image");
        }
        // UNITARY: a pass books both directions; nucleons are conserved.
        let (p, n) = (num_bigint::BigInt::from(1_000_000u64), num_bigint::BigInt::from(0));
        let (p1, n1) = exchange(&p, &n, &ratio(1, 3));
        assert_eq!(&p1 + &n1, &p + &n);
        assert!(n1 > num_bigint::BigInt::from(0));
        let (p2, n2) = exchange(&p1, &n1, &ratio(1, 3));
        assert_eq!(&p2 + &n2, &p + &n);
    }

    #[test]
    fn the_bounce_cools_to_its_seal_window_and_the_exchange_reads_the_neutron_share() {
        // Hot at the core, cold at the ring.
        assert_eq!(bounce_heat(&ratio(1, 1)), ratio(1, 1));
        assert_eq!(bounce_heat(&ratio(1, 2)), ratio(0, 1));
        // The seal window: the heat at half a turn.
        assert_eq!(seal_window(), ratio(3, 4));
        assert_eq!(bounce_heat(&seal_window()), ratio(1, 2));
        // The exchange's fixed point there: six neutrons in seven.
        let x = neutron_share(&seal_window());
        assert_eq!(x, ratio(6, 7));
        // It is the fixed point: a pass at δ leaves it standing.
        let n = num_bigint::BigInt::from(6_000_000u64);
        let p = num_bigint::BigInt::from(1_000_000u64);
        let (p1, n1) = exchange(&p, &n, &seal_window());
        assert_eq!((p1, n1), (p, n));
    }
}
