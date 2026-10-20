//! THE CHEMISTRY LAYER: a kind of matter is an atom or a cluster, and every
//! reading of it (its weight, its open frames, its charge, its grip) is a
//! reading of its parts on the atom, never a table.
//!
//! Key grammar (shared with xylarium): an atom is `Zddd`; a cluster names its
//! parts heaviest first, a count before each part where it is more than one
//! (`Z008·4Z001` is water on S^11, O + 4H, closed).
//! - its charge is `Σ n·Z` (the 3Z ledger counts `3·charge`, so bonding and
//!   dissociating conserve it exactly);
//! - its open frames are the centre's less the k each hydrogen pairs in a
//!   hydride, and none in a closing cluster;
//! - its hold is `3·charge − open`: the holding of its closed pairs.
use cfe::almanac::atom::{atom, Atom};
use cfe::ratio::{ratio, Ratio};

/// A KIND of matter, by its parts `(count, Z)`: Z descending, each Z once,
/// every count at least one, every Z an atom. Made only by its readers
/// below, so a kind that names no matter cannot be held.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kind {
    parts: Vec<(u32, u32)>,
}

impl Kind {
    /// The atom of `z` (refused for z = 0).
    pub fn atom(z: u32) -> Option<Kind> {
        atom(z)?;
        Some(Kind { parts: vec![(1, z)] })
    }

    /// The cluster of `parts` (counts merged by Z, Z descending), refused
    /// where it holds nothing or names no atom.
    pub fn cluster(parts: &[(u32, u32)]) -> Option<Kind> {
        let mut merged: Vec<(u32, u32)> = Vec::new();
        let mut sorted: Vec<(u32, u32)> = parts.to_vec();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (n, z) in sorted {
            atom(z)?;
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
            let (n, z) = match part.split_once('Z') {
                Some(("", z)) => (1, z.parse().ok()?),
                Some((n, z)) => (n.parse().ok()?, z.parse().ok()?),
                None => return None,
            };
            parts.push((n, z));
        }
        let kind = Kind::cluster(&parts)?;
        (kind.parts == parts).then_some(kind)
    }

    /// Its parts, `(count, Z)`, heaviest first.
    pub fn parts(&self) -> &[(u32, u32)] {
        &self.parts
    }

    /// Its atoms, each part's atom once with its count.
    fn atoms(&self) -> impl Iterator<Item = (u32, Atom)> + '_ {
        self.parts.iter().filter_map(|&(n, z)| atom(z).map(|a| (n, a)))
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self.parts.iter().map(|(n, z)| if *n == 1 { format!("Z{z:03}") } else { format!("{n}Z{z:03}") }).collect();
        write!(f, "{}", names.join("·"))
    }
}

/// Hydrogen, the first kind.
pub fn hydrogen() -> Kind {
    Kind { parts: vec![(1, 1)] }
}

/// A kind's parts, `(count, Z)`, heaviest first.
pub fn components(kind: &Kind) -> &[(u32, u32)] {
    kind.parts()
}

/// The centre's Z and the hydrogens on it, for a plain atom or a hydride
/// (other clusters: None).
pub fn parts(kind: &Kind) -> Option<(u32, u32)> {
    match kind.parts() {
        [(1, z)] => Some((*z, 0)),
        [(1, z), (k, 1)] if *z != 1 => Some((*z, *k)),
        _ => None,
    }
}

/// The hydride of centre `z` with `k` hydrogens (`k = 0`: the atom).
pub fn hydride(z: u32, k: u32) -> Option<Kind> {
    if k == 0 { Kind::atom(z) } else { Kind::cluster(&[(1, z), (k, 1)]) }
}

/// A plain atom's Z (None for a cluster): what may burn, ionize or be cut by
/// its windings.
pub fn atom_z(kind: &Kind) -> Option<u32> {
    match kind.parts() {
        [(1, z)] => Some(*z),
        _ => None,
    }
}

/// Its charge, `Σ n·Z`: the protons of all its parts.
pub fn charge(kind: &Kind) -> u32 {
    kind.parts().iter().map(|(n, z)| n * z).sum()
}

/// Its open frames: an atom's own; a hydride's centre's less the k its
/// hydrogens pair; a closing cluster none (every frame paired).
pub fn unpaired(kind: &Kind) -> u32 {
    match kind.parts() {
        [(1, z)] => atom(*z).map_or(0, |a| a.unpaired),
        [(1, z), (k, 1)] if *z != 1 => atom(*z).map_or(0, |a| a.unpaired).checked_sub(*k).map_or(0, |u| u),
        _ => 0,
    }
}

/// Its hold, `3·charge − open`: the holding of its closed pairs.
pub fn hold(kind: &Kind) -> u32 {
    3 * charge(kind) - unpaired(kind)
}

/// Whether it is closed (every open frame paired): inert.
pub fn closed(kind: &Kind) -> bool {
    unpaired(kind) == 0
}

/// A kind's BOND GRIP, the tightness of its loosest winding in hydrogen's own
/// unit (an atom's outer winding; a cluster's loosest part's).
pub fn tightness(kind: &Kind) -> Ratio {
    kind.atoms().map(|(_, a)| a.tightness).min().map_or_else(|| ratio(0, 1), |t| t)
}

/// A cluster's PERMANENT DIPOLE (with xylarium, atoms::hold's form): what its
/// symmetry leaves uncancelled, as a share. Its centre is the first atom named
/// (the heaviest), every other atom an arm. The centre keeps LONE PAIRS,
/// `(outer − Σ arms' open)/2`, and they bend it, `bent = lone/(lone + arms)`;
/// unlike arms leave their mixture (twice the share not of the commonest
/// kind, at most 1). The dipole is the asymmetry `1 − τ_min/τ_max` times the
/// larger of the two. A pair of unlike atoms keeps its asymmetry; an atom, or
/// a centre with no lone pairs and like arms, has none. Water on S^11: 2/15.
pub fn dipole(kind: &Kind) -> Ratio {
    let zero = ratio(0, 1);
    let atoms: Vec<Atom> = kind.atoms().flat_map(|(n, a)| std::iter::repeat(a).take(n as usize)).collect();
    let (Some(lo), Some(hi)) = (atoms.iter().map(|a| a.tightness.clone()).min(), atoms.iter().map(|a| a.tightness.clone()).max()) else {
        return zero;
    };
    if hi == zero {
        return zero;
    }
    let asymmetry = ratio(1, 1) - lo / hi;
    let mixture = |of: &[Atom]| {
        let most = of.iter().map(|a| of.iter().filter(|b| b.z == a.z).count()).max().map_or(0, |m| m);
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

/// THE SMALLEST WHOLE CLUSTER two atoms close in (with xylarium, 969536c):
/// atoms with `u_A`, `u_B` open frames pair as `(u_B/g)·A + (u_A/g)·B`,
/// `g = gcd(u_A, u_B)`, every frame paired. None where either has none.
pub fn closing(a: u32, ua: u32, b: u32, ub: u32) -> Option<(u32, u32, Kind)> {
    if ua == 0 || ub == 0 {
        return None;
    }
    fn gcd(x: u32, y: u32) -> u32 { if y == 0 { x } else { gcd(y, x % y) } }
    let g = gcd(ua, ub);
    let (na, nb) = (ub / g, ua / g);
    Some((na, nb, Kind::cluster(&[(na, a), (nb, b)])?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(key: &str) -> Kind {
        Kind::parse(key).expect("a kind")
    }

    #[test]
    fn a_kind_is_made_only_from_atoms_written_heaviest_first() {
        assert_eq!(kind("Z008·4Z001").to_string(), "Z008·4Z001");
        assert_eq!(Kind::cluster(&[(4, 1), (1, 8)]), Some(kind("Z008·4Z001")));
        assert_eq!(Kind::cluster(&[(2, 1), (2, 1)]), Some(kind("4Z001")));
        assert!(Kind::parse("4Z001·Z008").is_none(), "not heaviest first");
        assert!(Kind::parse("Z000").is_none(), "no atom");
        assert!(Kind::parse("resource:Iron").is_none(), "no kind");
        assert!(Kind::atom(0).is_none());
    }

    #[test]
    fn water_is_oxygen_and_four_hydrogens_closed() {
        assert_eq!(parts(&kind("Z008")), Some((8, 0)));
        assert_eq!(parts(&kind("Z008·4Z001")), Some((8, 4)));
        assert_eq!(hydride(8, 4), Some(kind("Z008·4Z001")));
        assert_eq!(atom_z(&kind("Z008·4Z001")), None);
        // On S^11, oxygen opens four, so O + 4H closes.
        assert_eq!(unpaired(&kind("Z008")), 4);
        assert!(closed(&kind("Z008·4Z001")));
        // The 3Z ledger holds across the bond: hold + open = 3(Z + k).
        assert_eq!(hold(&kind("Z008·4Z001")) + unpaired(&kind("Z008·4Z001")), 3 * 12);
        assert_eq!(hold(&kind("Z008")) + 4 * hold(&kind("Z001")) + unpaired(&kind("Z008")) + 4 * unpaired(&kind("Z001")), 3 * 12);
        // The smallest whole cluster: O (4) with H (1) is O·4H; a 2:3 pair.
        assert_eq!(closing(8, 4, 1, 1).map(|c| c.2), Some(kind("Z008·4Z001")));
        assert_eq!(closing(26, 6, 8, 4).map(|c| (c.0, c.1)), Some((2, 3)));
        assert_eq!(charge(&kind("2Z026·3Z008")), 2 * 26 + 3 * 8);
        assert!(closed(&kind("2Z026·3Z008")));
        // Water is bent by oxygen's one lone pair: its dipole 2/15. Salt on
        // S^11 is 7Cl·11Na; an atom has no dipole.
        assert_eq!(dipole(&kind("Z008·4Z001")), ratio(2, 15));
        assert_eq!(closing(11, 7, 17, 11).map(|c| c.2), Some(kind("7Z017·11Z011")));
        assert_eq!(dipole(&kind("Z008")), ratio(0, 1));
    }
}
