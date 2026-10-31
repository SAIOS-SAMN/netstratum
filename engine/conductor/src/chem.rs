//! The chemistry layer: a kind of matter is an atom or a cluster, and every
//! reading of it (its weight, its open frames, its charge, its grip) is a
//! reading of its parts on the atom, never a table.
//!
//! A kind's atom is A nuclide (docs/97; operator 2026-10-26: "Q is the proton
//! and Z is the neutron and R is the phase state of the electron"), named by
//! its weight `w = Q + Z`, its nucleons: `Wdddd`. Its protons Q and neutrons Z
//! are read off the nucleus's convergence on S^11 (`crate::nuclides`); its
//! electrons, and so its atom and chemistry, follow its protons (`atom(Q)`).
//! A cluster names its parts heaviest first, a count before each part where it
//! is more than one.
//! - its weight is `Σ n·w`, its nucleons: the mass gravity pulls, conserved
//!   through every bond, fusion and peel;
//! - its charge is `Σ n·Q`, its protons;
//! - its open frames (R's domain, the electrons) are the centre's less the k
//!   each hydrogen pairs in a hydride, and none in a closing cluster;
//! - its hold is `weight − open`.
use cfe::almanac::atom::{atom, Atom};
use cfe::ratio::{ratio, Ratio};

/// A KIND of matter, by its parts `(count, w)`: w the nuclide's weight,
/// descending, each once, every count at least one, every nuclide an atom.
/// Made only by its readers below, so a kind that names no matter cannot be
/// held.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kind {
    parts: Vec<(u32, u32)>,
}

impl Kind {
    /// The nuclide of weight `w` (refused for w = 0 or past the read).
    pub fn atom(w: u32) -> Option<Kind> {
        atom_of(w)?;
        Some(Kind {
            parts: vec![(1, w)],
        })
    }

    /// The element of charge `q`: its lightest nuclide.
    pub fn element(q: u32) -> Option<Kind> {
        Kind::atom(crate::nuclides::of_protons(q)?)
    }

    /// The cluster of `parts` (counts merged by Z, Z descending), refused
    /// where it holds nothing or names no atom.
    pub fn cluster(parts: &[(u32, u32)]) -> Option<Kind> {
        let mut merged: Vec<(u32, u32)> = Vec::new();
        let mut sorted: Vec<(u32, u32)> = parts.to_vec();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (n, z) in sorted {
            atom_of(z)?;
            if n == 0 {
                return None;
            }
            match merged.last_mut() {
                Some(last) if last.1 == z => last.0 = last.0.checked_add(n)?,
                _ => merged.push((n, z)),
            }
        }
        (!merged.is_empty()).then_some(Kind { parts: merged })
    }

    /// The kind a key names, refused where the key is not one written by
    /// this grammar (its parts heaviest first, each Z once).
    pub fn parse(key: &str) -> Option<Kind> {
        let mut parts = Vec::new();
        for part in key.split('·') {
            let (n, z) = match part.split_once('W') {
                Some(("", z)) => (1, z.parse().ok()?),
                Some((n, z)) => (n.parse().ok()?, z.parse().ok()?),
                None => return None,
            };
            parts.push((n, z));
        }
        let kind = Kind::cluster(&parts)?;
        (kind.parts == parts).then_some(kind)
    }

    /// Its parts, `(count, w)`, heaviest first.
    pub fn parts(&self) -> &[(u32, u32)] {
        &self.parts
    }

    /// Its atoms, each part's atom once with its count.
    fn atoms(&self) -> impl Iterator<Item = (u32, Atom)> + '_ {
        self.parts
            .iter()
            .filter_map(|&(n, w)| atom_of(w).map(|a| (n, a)))
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self
            .parts
            .iter()
            .map(|(n, w)| {
                if *n == 1 {
                    format!("W{w:04}")
                } else {
                    format!("{n}W{w:04}")
                }
            })
            .collect();
        write!(f, "{}", names.join("·"))
    }
}

/// The atom of A nuclide: the electrons its protons hold (`atom(Q)`).
pub fn atom_of(w: u32) -> Option<Atom> {
    atom(crate::nuclides::protons(w)?)
}

/// The protons of a nuclide of weight `w` (0 where it names none).
pub fn protons(w: u32) -> u32 {
    crate::nuclides::protons(w).unwrap_or(0)
}

/// Hydrogen, the first kind.
pub fn hydrogen() -> Kind {
    Kind {
        parts: vec![(1, 1)],
    }
}

/// A kind's parts, `(count, w)`, heaviest first.
pub fn components(kind: &Kind) -> &[(u32, u32)] {
    kind.parts()
}

/// The centre's weight and the hydrogens on it, for a plain atom or a hydride
/// (other clusters: None).
pub fn parts(kind: &Kind) -> Option<(u32, u32)> {
    match kind.parts() {
        [(1, z)] => Some((*z, 0)),
        [(1, z), (k, 1)] if *z != 1 => Some((*z, *k)),
        _ => None,
    }
}

/// A plain atom's weight (None for a cluster): the nuclide that may burn,
/// ionize or be cut by its windings.
pub fn atom_w(kind: &Kind) -> Option<u32> {
    match kind.parts() {
        [(1, z)] => Some(*z),
        _ => None,
    }
}

/// Its charge, `Σ n·Q`: the protons of all its parts.
pub fn charge(kind: &Kind) -> u32 {
    kind.parts().iter().map(|(n, w)| n * protons(*w)).sum()
}

/// Its open frames: an atom's own; a hydride's centre's less the k its
/// hydrogens pair; a closing cluster none (every frame paired).
pub fn unpaired(kind: &Kind) -> u32 {
    match kind.parts() {
        [(1, z)] => atom_of(*z).map_or(0, |a| a.unpaired),
        [(1, z), (k, 1)] if *z != 1 => atom_of(*z)
            .map_or(0, |a| a.unpaired)
            .checked_sub(*k)
            .map_or(0, |u| u),
        _ => 0,
    }
}

/// Its hold, `weight − open`: the held share of it (chemistry reads its open
/// frames against it).
pub fn hold(kind: &Kind) -> u32 {
    weight(kind).saturating_sub(unpaired(kind))
}

/// Its weight as mass (docs/97): its nucleons, `Σ n·w`, the one weight that
/// gravity pulls and inertia answers by. Conserved through every bond,
/// fusion and peel.
pub fn weight(kind: &Kind) -> u32 {
    kind.parts().iter().map(|(n, w)| n * w).sum()
}

/// Whether it is closed (every open frame paired): inert.
pub fn closed(kind: &Kind) -> bool {
    unpaired(kind) == 0
}

/// A kind's bond grip, the tightness of its loosest winding in hydrogen's own
/// unit (an atom's outer winding; a cluster's loosest part's).
pub fn tightness(kind: &Kind) -> Ratio {
    kind.atoms()
        .map(|(_, a)| a.tightness)
        .min()
        .map_or_else(|| ratio(0, 1), |t| t)
}

/// A cluster's permanent dipole (with Xylarium, atoms::hold's form): what its
/// symmetry leaves uncancelled, as a share. Its centre is the first atom named
/// (the heaviest), every other atom an arm. The centre keeps lone pairs,
/// `(outer − Σ arms' open)/2`, and they bend it, `bent = lone/(lone + arms)`;
/// unlike arms leave their mixture (twice the share not of the commonest
/// kind, at most 1). The dipole is the asymmetry `1 − τ_min/τ_max` times the
/// larger of the two. A pair of unlike atoms keeps its asymmetry; an atom, or
/// a centre with no lone pairs and like arms, has none. Water on S^11: 2/15.
pub fn dipole(kind: &Kind) -> Ratio {
    let zero = ratio(0, 1);
    let atoms: Vec<Atom> = kind
        .atoms()
        .flat_map(|(n, a)| std::iter::repeat(a).take(n as usize))
        .collect();
    let (Some(lo), Some(hi)) = (
        atoms.iter().map(|a| a.tightness.clone()).min(),
        atoms.iter().map(|a| a.tightness.clone()).max(),
    ) else {
        return zero;
    };
    if hi == zero {
        return zero;
    }
    let asymmetry = ratio(1, 1) - lo / hi;
    let mixture = |of: &[Atom]| {
        let most = of
            .iter()
            .map(|a| of.iter().filter(|b| b.z == a.z).count())
            .max()
            .map_or(0, |m| m);
        let n = i64::try_from(of.len().max(1)).map_or(1, |n| n);
        let unlike = i64::try_from(of.len() - most).map_or(0, |u| u);
        ratio(2 * unlike, n).min(ratio(1, 1))
    };
    match atoms.as_slice() {
        [] | [_] => zero,
        [_, _] => asymmetry,
        [centre, arms @ ..] => {
            let bonded: u32 = arms.iter().map(|a| a.unpaired).sum();
            let lone = i64::from(centre.outer.checked_sub(bonded).map_or(0, |l| l) / 2);
            let n_arms = i64::try_from(arms.len()).map_or(0, |a| a);
            let bent = ratio(lone, lone + n_arms);
            let unlike = mixture(arms);
            asymmetry * if bent > unlike { bent } else { unlike }
        }
    }
}

/// The smallest whole cluster two atoms close in (with Xylarium, 969536c):
/// nuclides `a`, `b` (weights) with `u_A`, `u_B` open frames pair as
/// `(u_B/g)·A + (u_A/g)·B`, `g = gcd(u_A, u_B)`, every frame paired. None
/// where either has none.
pub fn closing(a: u32, ua: u32, b: u32, ub: u32) -> Option<(u32, u32, Kind)> {
    if ua == 0 || ub == 0 {
        return None;
    }
    fn gcd(x: u32, y: u32) -> u32 {
        if y == 0 {
            x
        } else {
            gcd(y, x % y)
        }
    }
    let g = gcd(ua, ub);
    let (na, nb) = (ub / g, ua / g);
    Some((na, nb, Kind::cluster(&[(na, a), (nb, b)])?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn el(q: u32) -> u32 {
        crate::nuclides::of_protons(q).expect("an element")
    }

    #[test]
    fn a_kind_is_made_only_from_nuclides_written_heaviest_first() {
        let (o, h) = (el(8), el(1));
        let water = Kind::cluster(&[(4, h), (1, o)]).expect("water");
        assert_eq!(Kind::parse(&water.to_string()), Some(water.clone()));
        assert_eq!(
            Kind::cluster(&[(2, 1), (2, 1)]).map(|k| k.to_string()),
            Some("4W0001".into())
        );
        assert!(Kind::parse("4W0001·W0009").is_none(), "not heaviest first");
        assert!(Kind::parse("W0000").is_none(), "no nuclide");
        assert!(Kind::parse("resource:Iron").is_none(), "no kind");
        assert!(Kind::atom(0).is_none());
        // Its weight is its nucleons, its charge its protons.
        assert_eq!(weight(&water), o + 4);
        assert_eq!(charge(&water), 8 + 4);
    }

    #[test]
    fn water_is_oxygen_and_four_hydrogens_closed() {
        let (o, h) = (el(8), el(1));
        let ox = Kind::atom(o).expect("oxygen");
        let water = Kind::cluster(&[(1, o), (4, h)]).expect("water");
        assert_eq!(parts(&ox), Some((o, 0)));
        assert_eq!(parts(&water), Some((o, 4)));
        assert_eq!(atom_w(&water), None);
        // On S^11, oxygen opens four, so O + 4H closes.
        assert_eq!(unpaired(&ox), 4);
        assert!(closed(&water));
        // Its hold and open frames make its weight.
        assert_eq!(hold(&water) + unpaired(&water), weight(&water));
        assert_eq!(closing(o, 4, h, 1).map(|c| c.2), Some(water.clone()));
        // Water is bent by oxygen's one lone pair: its dipole 2/15.
        assert_eq!(dipole(&water), ratio(2, 15));
        assert_eq!(dipole(&ox), ratio(0, 1));
    }
}
