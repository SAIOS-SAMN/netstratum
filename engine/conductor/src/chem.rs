//! THE CHEMISTRY LAYER (operators 2026-10-18, "build out the chemistry
//! layer"; with xylarium): chemistry extrapolated from our own physics, the
//! meeting law run at the atom tier. A KIND of matter is an atom or a
//! molecule, and every reading of it (its weight, its open frames, its
//! charge) is a reading of its PARTS, never a table.
//!
//! Key grammar (shared with xylarium): an atom is `Zddd`; a HYDRIDE, a centre
//! atom bonded to k hydrogens, is `Zddd·kZ001` (water on S^11 is
//! `Z008·4Z001`, O + 4H, closed). xylarium carries the same as the id
//! `10000·k + Z`.
//! - its charge is `Z + k` (the 3Z ledger counts `3(Z + k)`, so bonding and
//!   dissociating conserve it exactly);
//! - its open frames are the centre's less the k each hydrogen pairs,
//!   `u(Z) − k`;
//! - its hold is `3(Z + k) − (u(Z) − k)`: holding belongs to closed pairs
//!   (the atom tier's `2Z + (Z − u)`, summed over the parts with the bonds
//!   paired).

/// A kind's PARTS: each `(count, Z)`, Z descending. `Zddd` is one atom;
/// `[n]Zddd·[m]Zeee…` a cluster (counts before each part, 1 unwritten).
pub fn components(key: &str) -> Option<Vec<(u32, u32)>> {
    let mut out = Vec::new();
    for part in key.split('·') {
        let (n, z) = match part.split_once('Z') {
            Some(("", z)) => (1, z),
            Some((n, z)) => (n.parse().ok()?, z),
            None => return None,
        };
        out.push((n, z.parse().ok()?));
    }
    (!out.is_empty()).then_some(out)
}

/// The centre's Z and the hydrogens on it, for a plain atom or a hydride
/// (other clusters: None).
pub fn parts(key: &str) -> Option<(u32, u32)> {
    match components(key)?.as_slice() {
        [(1, z)] => Some((*z, 0)),
        [(1, z), (k, 1)] if *z != 1 => Some((*z, *k)),
        _ => None,
    }
}

/// The key of a cluster of parts (counts merged, Z descending).
pub fn cluster(parts: &[(u32, u32)]) -> String {
    let mut by: std::collections::BTreeMap<u32, u32> = std::collections::BTreeMap::new();
    for (n, z) in parts {
        *by.entry(*z).or_insert(0) += n;
    }
    by.into_iter().rev().map(|(z, n)| if n == 1 { format!("Z{z:03}") } else { format!("{n}Z{z:03}") }).collect::<Vec<_>>().join("·")
}

/// The key of the hydride of centre `z` with `k` hydrogens (`k = 0`: the atom).
pub fn hydride(z: u32, k: u32) -> String {
    if k == 0 { format!("Z{z:03}") } else { cluster(&[(1, z), (k, 1)]) }
}

/// A plain atom's Z (None for a molecule): what may burn, ionize or be cut
/// by its windings.
pub fn atom_z(key: &str) -> Option<u32> {
    match components(key)?.as_slice() {
        [(1, z)] => Some(*z),
        _ => None,
    }
}

/// Its charge, `Σ n·Z`: the protons of all its parts.
pub fn charge(key: &str) -> u32 {
    components(key).map(|ps| ps.iter().map(|(n, z)| n * z).sum()).unwrap_or(0)
}

fn atom_open(z: u32) -> u32 {
    cfe::almanac::atom::atom(z).map(|a| a.unpaired).unwrap_or(0)
}

/// Its open frames: an atom's own; a cluster's `Σ n·u − 2·bonds`, where the
/// hydride of k pairs k (`u(Z) − k`) and a two-part cluster closes in its
/// smallest whole (every frame paired, 0).
pub fn unpaired(key: &str) -> u32 {
    match components(key).as_deref() {
        Some([(1, z)]) => atom_open(*z),
        Some([(1, z), (k, 1)]) if *z != 1 => atom_open(*z).saturating_sub(*k),
        Some(_) => 0,
        None => 0,
    }
}

/// Its hold, `3·charge − open`: the holding of its closed pairs.
pub fn hold(key: &str) -> u32 {
    match atom_z(key) {
        Some(z) => cfe::almanac::atom::hold(z),
        None if components(key).is_some() => 3 * charge(key) - unpaired(key),
        None => 1,
    }
}

/// Whether it is closed (every open frame paired): inert.
pub fn closed(key: &str) -> bool {
    unpaired(key) == 0
}

/// A kind's BOND GRIP, the tightness of its loosest winding in hydrogen's
/// own unit (an atom's outer winding; a cluster's loosest part's).
pub fn tightness(key: &str) -> cfe::ratio::Ratio {
    let t = |z: u32| cfe::almanac::atom::atom(z).map(|a| a.tightness).unwrap_or_else(|| cfe::ratio::ratio(0, 1));
    components(key).and_then(|ps| ps.iter().map(|(_, z)| t(*z)).min()).unwrap_or_else(|| cfe::ratio::ratio(0, 1))
}

/// A cluster's PERMANENT DIPOLE (with xylarium, atoms::hold's form; 11.0
/// (19) DISSOLUTION): what its symmetry leaves uncancelled, as a share. Its
/// centre is the first atom named (the heaviest), every other atom an arm.
/// The centre keeps LONE PAIRS, `(outer − Σ arms' open)/2`, and they bend
/// it: `bent = lone/(lone + arms)`. Unlike arms leave their mixture
/// (twice the share not of the commonest kind, at most 1). The dipole is the
/// cluster's asymmetry `1 − τ_min/τ_max` times the larger of the two. A
/// pair of unlike atoms is straight and keeps its asymmetry; an atom, or a
/// centre with no lone pairs and like arms, has none. Water on S^11, O·4H:
/// oxygen's outer six, four bonded, one lone pair: `(2/3)·(1/5) = 2/15`.
pub fn dipole(key: &str) -> cfe::ratio::Ratio {
    use cfe::ratio::ratio;
    let zero = ratio(0, 1);
    let atoms: Vec<cfe::almanac::atom::Atom> = components(key)
        .unwrap_or_default()
        .into_iter()
        .flat_map(|(n, z)| std::iter::repeat(z).take(n as usize))
        .filter_map(cfe::almanac::atom::atom)
        .collect();
    let (Some(lo), Some(hi)) = (atoms.iter().map(|a| a.tightness.clone()).min(), atoms.iter().map(|a| a.tightness.clone()).max()) else {
        return zero;
    };
    if hi == zero {
        return zero;
    }
    let asymmetry = ratio(1, 1) - lo / hi;
    let mixture = |of: &[cfe::almanac::atom::Atom]| {
        let most = of.iter().map(|a| of.iter().filter(|b| b.z == a.z).count()).max().unwrap_or(0);
        let n = of.len().max(1) as i64;
        ratio(2 * (of.len() - most) as i64, n).min(ratio(1, 1))
    };
    match atoms.as_slice() {
        [] | [_] => zero,
        [_, _] => asymmetry,
        [centre, arms @ ..] => {
            let bonded: u32 = arms.iter().map(|a| a.unpaired).sum();
            let lone = i64::from(centre.outer.saturating_sub(bonded) / 2);
            let bent = ratio(lone, lone + arms.len() as i64);
            let unlike = mixture(arms);
            asymmetry * if bent > unlike { bent } else { unlike }
        }
    }
}

/// THE SMALLEST WHOLE CLUSTER two kinds close in (with xylarium, 969536c):
/// atoms with `u_A`, `u_B` open frames pair as `(u_B/g)·A + (u_A/g)·B`,
/// `g = gcd(u_A, u_B)`, every frame paired. None where either has none.
pub fn closing(a: u32, ua: u32, b: u32, ub: u32) -> Option<(u32, u32, String)> {
    if ua == 0 || ub == 0 {
        return None;
    }
    fn gcd(x: u32, y: u32) -> u32 { if y == 0 { x } else { gcd(y, x % y) } }
    let g = gcd(ua, ub);
    let (na, nb) = (ub / g, ua / g);
    Some((na, nb, cluster(&[(na, a), (nb, b)])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_is_oxygen_and_four_hydrogens_closed() {
        assert_eq!(parts("Z008"), Some((8, 0)));
        assert_eq!(parts("Z008·4Z001"), Some((8, 4)));
        assert_eq!(hydride(8, 4), "Z008·4Z001");
        assert_eq!(atom_z("Z008·4Z001"), None);
        // On S^11, oxygen opens four, so O + 4H closes.
        assert_eq!(unpaired("Z008"), 4);
        assert!(closed("Z008·4Z001"));
        // The 3Z ledger holds across the bond: hold + open = 3(Z + k).
        assert_eq!(hold("Z008·4Z001") + unpaired("Z008·4Z001"), 3 * 12);
        assert_eq!(hold("Z008") + 4 * hold("Z001") + unpaired("Z008") + 4 * unpaired("Z001"), 3 * 12);
        // The smallest whole cluster: O (4) with H (1) is O·4H; a 2:3 pair.
        assert_eq!(closing(8, 4, 1, 1).map(|c| c.2), Some("Z008·4Z001".into()));
        assert_eq!(closing(26, 6, 8, 4).map(|c| (c.0, c.1)), Some((2, 3)));
        assert_eq!(charge("2Z026·3Z008"), 2 * 26 + 3 * 8);
        assert!(closed("2Z026·3Z008"));
        // Water is bent by oxygen's one lone pair: its dipole 2/15. Salt on
        // S^11 is 7Cl·11Na; an atom has no dipole.
        assert_eq!(dipole("Z008·4Z001"), cfe::ratio::ratio(2, 15));
        assert_eq!(closing(11, 7, 17, 11).map(|c| c.2), Some("7Z017·11Z011".into()));
        assert_eq!(dipole("Z008"), cfe::ratio::ratio(0, 1));
    }
}
