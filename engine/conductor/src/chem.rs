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

/// A kind's parts: the centre's Z and the hydrogens bonded to it.
pub fn parts(key: &str) -> Option<(u32, u32)> {
    let rest = key.strip_prefix('Z')?;
    match rest.split_once('·') {
        None => Some((rest.parse().ok()?, 0)),
        Some((z, h)) => {
            let k = h.strip_suffix("Z001")?.parse().ok()?;
            Some((z.parse().ok()?, k))
        }
    }
}

/// The key of the hydride of centre `z` with `k` hydrogens (`k = 0`: the atom).
pub fn hydride(z: u32, k: u32) -> String {
    if k == 0 { format!("Z{z:03}") } else { format!("Z{z:03}·{k}Z001") }
}

/// A plain atom's Z (None for a molecule): what may burn, ionize or be cut
/// by its windings.
pub fn atom_z(key: &str) -> Option<u32> {
    parts(key).filter(|(_, k)| *k == 0).map(|(z, _)| z)
}

/// Its charge, `Z + k`: the protons of all its parts.
pub fn charge(key: &str) -> u32 {
    parts(key).map(|(z, k)| z + k).unwrap_or(0)
}

/// Its open frames, `u(Z) − k`.
pub fn unpaired(key: &str) -> u32 {
    parts(key).map(|(z, k)| cfe::almanac::atom::atom(z).map(|a| a.unpaired).unwrap_or(0).saturating_sub(k)).unwrap_or(0)
}

/// Its hold, `3(Z + k) − (u(Z) − k)`: the holding of its closed pairs.
pub fn hold(key: &str) -> u32 {
    match parts(key) {
        Some((z, 0)) => cfe::almanac::atom::hold(z),
        Some((z, k)) => 3 * (z + k) - unpaired(key),
        None => 1,
    }
}

/// Whether it is closed (every open frame paired): inert.
pub fn closed(key: &str) -> bool {
    unpaired(key) == 0
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
    }
}
