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

/// A NUCLIDE'S PLACE ON S^11 (docs/100 §1; xylarium 11.0 (13): "a kind's
/// HARMONIC WEIGHT is its place, never its mass alone"), read off the
/// convergence as its last end arrived: its turning `n` (it holds the
/// degrees `0..n − 1` closed), its open degree `l` (the harmonic its outer
/// ends stand on), and of that degree's `dirs` directions the ends claimed
/// (`lone + bound`) and the fibres paired (`bound`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Place {
    pub turning: u64,
    pub degree: u64,
    pub dirs: u64,
    pub lone: u64,
    pub bound: u64,
}

impl Place {
    /// Its outer degree's claimed share, `(lone + bound)/dirs`.
    pub fn claimed(&self) -> cfe::ratio::Ratio {
        cfe::ratio::ratio(i64::try_from(self.lone + self.bound).unwrap_or(i64::MAX), i64::try_from(self.dirs.max(1)).unwrap_or(i64::MAX))
    }

    /// Its outer degree's paired share, `bound/dirs` (1 at every seal).
    pub fn paired(&self) -> cfe::ratio::Ratio {
        cfe::ratio::ratio(i64::try_from(self.bound).unwrap_or(i64::MAX), i64::try_from(self.dirs.max(1)).unwrap_or(i64::MAX))
    }
}

/// The place of the nuclide of weight `w` (None past the read or at 0).
pub fn place(w: u32) -> Option<Place> {
    places().get(usize::try_from(w).ok()?.checked_sub(1)?).copied()
}

fn places() -> &'static [Place] {
    static PLACES: std::sync::OnceLock<Vec<Place>> = std::sync::OnceLock::new();
    PLACES.get_or_init(|| {
        let mut out = Vec::with_capacity(READ as usize);
        let (mut turning, mut degree) = (1u64, 0u64);
        let (mut lone, mut bound) = (0u64, 0u64);
        for _ in 0..READ {
            let dirs = directions(degree);
            if lone + bound < dirs {
                lone += 1;
            } else {
                lone -= 1;
                bound += 1;
            }
            out.push(Place { turning, degree, dirs, lone, bound });
            if bound == dirs {
                degree += 1;
                lone = 0;
                bound = 0;
                if degree == turning {
                    turning += 1;
                    degree = 0;
                }
            }
        }
        out
    })
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
    // THE MASTER EQUATION (docs/105 §3): a proton binds as a fibre's partner
    // at the held face's rate g, a neutron frees to a lone end at the
    // unlocked face's u; booked both ways.
    let _ = ratio(0, 1);
    let (g, u) = crate::polarity::faces(delta);
    let to_n = whole(Ratio::from_integer(p.clone()) * g);
    let to_p = whole(Ratio::from_integer(n.clone()) * u);
    (p - &to_n + &to_p, n - &to_p + &to_n)
}

/// THE HEAT, READ ON A BODY'S OWN CLOCK (docs/97 §11): a body at deficit δ
/// receiving `nu` rings a bell turns `ν·δ` a bell; the luminal bound (one
/// whole turn a K-step) leaves the excess as agitation, `max(0, ν·δ − 1)`.
pub fn heat(delta: &cfe::ratio::Ratio, nu: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    (nu.clone() * delta.clone() - ratio(1, 1)).max(ratio(0, 1))
}

/// R, THE ELECTRON'S PHASE STATE (docs/97 §3, §13): the nuclide's `Q_w`
/// electrons converge on S^11 as its nucleons do, fibred share
/// `κ_e = 2·Z(Q_w)/Q_w`; at pulse phase `p` (turns) the hemisphere law's
/// response, `R = κ_e·(1 − 2·frac(p))`.
pub fn electron_phase(w: u32, p: &cfe::ratio::Ratio) -> Option<cfe::ratio::Ratio> {
    use cfe::ratio::ratio;
    let kappa_e = coherence(protons(w)?)?;
    let f = p.clone() - p.floor();
    Some(kappa_e * (ratio(1, 1) - ratio(2, 1) * f))
}

/// THE ELECTRONS' BOUND SHARE (docs/105 §3, the master equation): they bind
/// at the held face's rate `g` and free at the unlocked face's `u`, so they
/// stand bound at their pair share `g/(g + u) = g`, the held share at the
/// nuclide's own place under inverse gravity. No phase, no halving.
pub fn held(_w: u32, g: &cfe::ratio::Ratio) -> Option<cfe::ratio::Ratio> {
    crate::polarity::equilibrium(g, &(cfe::ratio::ratio(1, 1) - g.clone()))
}

/// A NUCLEUS'S GRIP (docs/97 §11, §13): a fibre is two counter-turning ends
/// sharing one whole turn, the held end's share the held hemisphere, so a
/// nucleus grips by `κ_w·h` at its pulse phase.
pub fn grip(w: u32, p: &cfe::ratio::Ratio) -> Option<cfe::ratio::Ratio> {
    Some(coherence(w)? * held(w, p)?)
}

/// The rings a bell at the source: the double pass (each ring read on both
/// legs of the loop) at rate 1.
pub fn source_clock() -> cfe::ratio::Ratio {
    cfe::ratio::ratio(2, 1)
}

/// THE SEAL WINDOW (docs/97 §11, §13): where deuterium first holds on a
/// clock of `nu` rings a bell, its heat fallen to its grip at the source's
/// own echo (p = 0), `ν·δ − 1 = grip_2`: `δ = (1 + grip)/ν`.
pub fn seal_window(nu: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    // Deuterium crosses at the throat: its grip read at the throat's held
    // share, the fixed point where it holds (docs/105 §4).
    (ratio(1, 1) + grip(2, &crate::polarity::throat_held()).unwrap_or_else(|| ratio(0, 1))) / nu.clone()
}

/// A DYING HEART'S RELEASE (docs/97 §11): each nuclide whose grip at the
/// heart's pulse phase `p` its heat exceeds breaks into its nucleons, standing as hydrogen (the exchange
/// returns their neutrons to the continuum): the fuel of the rapid capture.
/// Nucleons conserved. Returns the nucleons released.
pub fn release(m: &mut crate::pulse::Manifest, a: &cfe::ratio::Ratio, p: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    let Some(h) = crate::chem::Kind::atom(1) else { return ratio(0, 1) };
    let mut freed = ratio(0, 1);
    let broken: Vec<(crate::chem::Kind, cfe::ratio::Ratio)> = m.iter().filter(|(k, _)| crate::chem::atom_w(k).is_some_and(|w| w > 1 && grip(w, p).is_some_and(|g| *a > g))).map(|(k, x)| (k.clone(), x.clone())).collect();
    for (k, x) in broken {
        let w = crate::chem::weight(&k);
        m.remove(&k);
        let n = x * ratio(i64::from(w), 1);
        freed += n.clone();
        *m.entry(h.clone()).or_insert_with(|| ratio(0, 1)) += n;
    }
    freed
}

/// THE EXCHANGE'S FIXED POINT (docs/97 §9): where the pass's two booked
/// directions balance, `p·δ = n·½(1 − δ)`, the neutron share is
/// `2δ/(1 + δ)`; read at the seal window, never set.
pub fn neutron_share(delta: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    // The nucleons' equilibrium (docs/105 §3): binding at g, freeing at u.
    let (g, u) = crate::polarity::faces(delta);
    crate::polarity::equilibrium(&g, &u).unwrap_or_else(|| cfe::ratio::ratio(0, 1))
}

/// A capture's survival at weight `w` at a place of held share `g` (docs/97
/// §10, docs/105 §3): bound by its fibres, or lone and held at the electrons'
/// equilibrium, `κ_w + (1 − κ_w)·g`.
pub fn survival(w: u32, p: &cfe::ratio::Ratio) -> Option<cfe::ratio::Ratio> {
    use cfe::ratio::ratio;
    let k = coherence(w)?;
    Some(k.clone() + (ratio(1, 1) - k) * held(w, p)?)
}

/// Nuclei of `w` moved to `w + 1` by `share`, each drawing one hydrogen
/// nucleus, never more than either holds (whole nuclei); returns the count
/// moved (in grains).
fn climb(m: &mut crate::pulse::Manifest, w: u32, share: &cfe::ratio::Ratio) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    let (Some(from), Some(to), Some(h)) = (crate::chem::Kind::atom(w), crate::chem::Kind::atom(w + 1), crate::chem::Kind::atom(1)) else { return ratio(0, 1) };
    let here = m.get(&from).cloned().unwrap_or_else(|| ratio(0, 1));
    let fuel = m.get(&h).cloned().unwrap_or_else(|| ratio(0, 1));
    // From hydrogen itself, two nuclei make the one that climbs.
    let fuel_left = if w == 1 { fuel.clone() / ratio(2, 1) } else { fuel.clone() };
    let mv = crate::pulse::nucleus(&(here.clone() * share.clone())).min(here).min(fuel_left);
    if mv <= ratio(0, 1) {
        return ratio(0, 1);
    }
    *m.entry(from).or_insert_with(|| ratio(0, 1)) -= mv.clone();
    *m.entry(h).or_insert_with(|| ratio(0, 1)) -= mv.clone();
    *m.entry(to).or_insert_with(|| ratio(0, 1)) += mv.clone();
    m.retain(|_, a| *a > ratio(0, 1));
    mv
}

/// THE SLOW CAPTURE (docs/97 §10): over `rings` rings of a burning star's
/// clock at coherence `c` and pulse phase `p`, every nuclide its heart holds passes
/// `f_H·c·σ_{w+1}` of itself to `w + 1` each ring (read at the ring's start,
/// heaviest first so no nucleus climbs twice in one ring), f_H hydrogen's
/// share of the heart's nuclei. Nucleons conserved. Returns the nuclei moved.
pub fn capture_slow(heart: &mut crate::pulse::Manifest, rings: u64, c: &cfe::ratio::Ratio, p: &cfe::ratio::Ratio, densest: u32) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    use num_traits::Zero;
    let mut moved = ratio(0, 1);
    for _ in 0..rings {
        let count = heart.values().fold(ratio(0, 1), |s, a| s + a.clone());
        let h = crate::chem::Kind::atom(1).and_then(|k| heart.get(&k).cloned()).unwrap_or_else(|| ratio(0, 1));
        if count.is_zero() || h.is_zero() {
            break;
        }
        let f_h = h / count;
        let mut weights: Vec<u32> = heart.keys().filter_map(crate::chem::atom_w).collect();
        weights.sort_by(|a, b| b.cmp(a));
        // Hydrogen is the fuel the heavier seeds capture, never a seed.
        for w in weights.into_iter().filter(|&w| w >= 2 && w < densest) {
            let Some(s) = survival(w + 1, p) else { continue };
            moved += climb(heart, w, &(f_h.clone() * c.clone() * s));
        }
    }
    moved
}

/// THE RAPID CAPTURE (docs/97 §10): the matter a star returns whole, at its
/// pulse phase `p`, runs the chain at once on its seeds (every kind heavier than hydrogen, which is the
/// fuel), each weight in turn (lightest first) passing `σ_{w+1}` of what
/// stands there upward, until its hydrogen is spent. Nucleons conserved.
/// Returns the heaviest weight reached.
pub fn capture_rapid(m: &mut crate::pulse::Manifest, p: &cfe::ratio::Ratio, densest: u32) -> u32 {
    use cfe::ratio::ratio;
    let mut top = m.keys().filter_map(crate::chem::atom_w).max().unwrap_or(0);
    // Hydrogen is the fuel; the seeds are the heavier kinds.
    let mut w = 2u32;
    // The universe's densest element (the founding's own, docs/97 §14)
    // stands; nothing climbs past it.
    while w < densest {
        let h = crate::chem::Kind::atom(1).and_then(|k| m.get(&k).cloned()).unwrap_or_else(|| ratio(0, 1));
        if h <= ratio(0, 1) {
            break;
        }
        if let Some(s) = survival(w + 1, p) {
            if climb(m, w, &s) > ratio(0, 1) {
                top = top.max(w + 1);
            }
        }
        if w >= top {
            break;
        }
        w += 1;
    }
    top
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
    fn a_nuclides_place_is_read_off_the_convergence() {
        // Every seal pairs its outer degree whole; its protons and neutrons
        // follow the place (Q = Z + lone, Z the fibres closed and open).
        for w in [2u32, 28, 208, 1092] {
            let p = place(w).expect("a place");
            assert_eq!(p.paired(), ratio(1, 1), "seal {w}");
            assert_eq!(p.claimed(), ratio(1, 1));
        }
        // Hydrogen: one lone end on the first degree of the first turning.
        let h = place(1).expect("hydrogen");
        assert_eq!((h.turning, h.degree, h.lone, h.bound), (1, 0, 1, 0));
        // The lone ends of the open degree are the protons past the fibres.
        for w in [3u32, 100, 1200, 1612] {
            let p = place(w).expect("a place");
            assert_eq!(protons(w).zip(neutrons(w)).map(|(q, z)| q - z), Some(u32::try_from(p.lone).expect("small")), "weight {w}");
            assert!(p.claimed() <= ratio(1, 1));
        }
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
        let nu = source_clock();
        // Hot at the core, cold at the ring, on the source's own clock.
        assert_eq!(heat(&ratio(1, 1), &nu), ratio(1, 1));
        assert_eq!(heat(&ratio(1, 2), &nu), ratio(0, 1));
        // Deuterium crosses at the throat: its grip at the throat's held
        // share (the fixed point where it holds); the window is where its
        // heat falls to it.
        let g0 = crate::polarity::throat_held();
        let g2 = grip(2, &g0).expect("deuterium");
        assert_eq!(heat(&seal_window(&nu), &nu), g2);
        assert_eq!(seal_window(&nu), (ratio(1, 1) + g2) / nu.clone());
        // A slower clock (a nested body) opens its window deeper.
        assert!(seal_window(&ratio(3, 2)) > seal_window(&nu));
        // Unsealed heavy nuclei grip less than the seals.
        assert!(grip(1200, &g0) < grip(1092, &g0));
        // The nucleons' equilibrium there (docs/105): the master equation's
        // pair share, and a pass at δ leaves it standing.
        let x = neutron_share(&seal_window(&nu));
        assert_eq!(x, crate::polarity::faces(&seal_window(&nu)).0);
        // Counts whole at the share: n : p = x : (1 − x).
        let k = num_bigint::BigInt::from(1_000_000u64);
        let n = x.numer().clone() * k.clone();
        let p = (x.denom().clone() - x.numer().clone()) * k;
        let (p1, n1) = exchange(&p, &n, &seal_window(&nu));
        assert_eq!((p1, n1), (p, n));
    }

    #[test]
    fn the_stars_captures_climb_and_conserve_nucleons_by_the_held_share() {
        let k = |w: u32| crate::chem::Kind::atom(w).expect("a nuclide");
        let nucleons = |m: &crate::pulse::Manifest| m.iter().fold(ratio(0, 1), |s, (kk, a)| s + a.clone() * ratio(i64::from(crate::chem::weight(kk)), 1));
        let (g_hi, g_lo, g_throat) = (ratio(9, 10), ratio(1, 10), crate::polarity::throat_held());
        let mut m: crate::pulse::Manifest = [(k(1), ratio(1_000_000, 1)), (k(4), ratio(1, 1))].into();
        let before = nucleons(&m);
        // A dying heart's heat breaks what it out-grips into nucleons.
        let mut spent: crate::pulse::Manifest = [(k(7), ratio(10, 1)), (k(28), ratio(10, 1))].into();
        let was = nucleons(&spent);
        // Where the held share is high, a gentle heat below both grips
        // breaks nothing.
        let mut gentle = spent.clone();
        let low = grip(7, &g_hi).expect("a grip").min(grip(28, &g_hi).expect("a grip")) / ratio(2, 1);
        assert_eq!(release(&mut gentle, &low, &g_hi), ratio(0, 1));
        // Where it is low, between the grips the unsealed breaks, the seal
        // holds.
        let between = (grip(7, &g_lo).expect("a grip") + grip(28, &g_lo).expect("a grip")) / ratio(2, 1);
        let freed = release(&mut spent, &between, &g_lo);
        assert_eq!(nucleons(&spent), was);
        assert!(freed > ratio(0, 1) && spent.contains_key(&k(28)), "the unsealed break, the seal holds");
        // Held high, one burst climbs to the densest element.
        let mut held_hi = m.clone();
        assert_eq!(capture_rapid(&mut held_hi, &g_hi, 1200), 1200);
        assert_eq!(nucleons(&held_hi), before, "nucleons conserved");
        // At the throat's held share the climb passes the third seal, and the
        // next generation climbs higher.
        let top = capture_rapid(&mut m, &g_throat, 1200);
        assert_eq!(nucleons(&m), before, "nucleons conserved");
        assert!(top > 208, "one burst passes the third seal: reached {top}");
        *m.entry(k(1)).or_insert_with(|| ratio(0, 1)) += ratio(1_000_000, 1);
        let again = capture_rapid(&mut m, &g_throat, 1200);
        assert!(again > top, "the next generation climbs higher: {top} then {again}");
        // A burning heart captures slowly and conserves too.
        let mut heart: crate::pulse::Manifest = [(k(1), ratio(1000, 1)), (k(4), ratio(100, 1))].into();
        let before = nucleons(&heart);
        let moved = capture_slow(&mut heart, 10, &ratio(1, 1), &g_throat, 1200);
        assert!(moved > ratio(0, 1));
        assert_eq!(nucleons(&heart), before);
        assert!(heart.keys().filter_map(crate::chem::atom_w).max() > Some(4));
    }

    #[test]
    fn twist_is_tension_in_the_fibre_a_wound_seal_parts_where_heat_and_torsion_pass_its_grip() {
        let k = |w: u32| crate::chem::Kind::atom(w).expect("a nuclide");
        let g = crate::polarity::throat_held();
        let nucleons = |m: &crate::pulse::Manifest| m.iter().fold(ratio(0, 1), |s, (kk, a)| s + a.clone() * ratio(i64::from(crate::chem::weight(kk)), 1));
        let grip28 = grip(28, &g).expect("a grip");
        // Unwound at its grip it holds (it parts only past it).
        let mut m: crate::pulse::Manifest = [(k(28), ratio(10, 1))].into();
        assert_eq!(release(&mut m, &grip28, &g), ratio(0, 1));
        // Wound by τ the same heat passes its grip: it parts to hydrogen.
        let was = nucleons(&m);
        assert!(release(&mut m, &(grip28.clone() + ratio(1, 4)), &g) > ratio(0, 1));
        assert_eq!(nucleons(&m), was);
        assert!(!m.contains_key(&k(28)));
    }

    #[test]
    fn the_electrons_stand_bound_at_the_held_share_and_survival_rises_with_it() {
        // R stays a reading of the electrons' phase.
        let w = 1200;
        let ke = coherence(protons(w).expect("protons")).expect("κ_e");
        assert_eq!(electron_phase(w, &ratio(0, 1)), Some(ke));
        assert_eq!(electron_phase(2, &ratio(1, 3)), Some(ratio(0, 1)));
        // Their bound share is the master equation's equilibrium at the
        // place: the held share g, no phase, no halving (docs/105).
        for g in [ratio(1, 10), ratio(3, 5), ratio(9, 10)] {
            assert_eq!(held(w, &g), Some(g.clone()));
        }
        // The seals keep every capture at any place; an unsealed nuclide
        // survives more where it is held more.
        assert_eq!(survival(1092, &ratio(1, 10)), Some(ratio(1, 1)));
        assert!(survival(w, &ratio(9, 10)) > survival(w, &ratio(1, 10)));
    }
}
