//! docs/78 "The atom": the lowest tier of the one law. An atom is not a cell
//! of a table; it is a nucleus of Z charges with Z electrons turning about
//! it, and its chemistry is read from that motion.
//!
//! The law, frame by frame (docs/78 design laws 4–6):
//! - An electron turns at `ω = Q(<r) / r`, Q the charge its orbit encloses:
//!   the nucleus less the electrons in bands further in. Electrons sharing a
//!   band share its radius, so none encloses another.
//! - **An orbit closes in a whole number of ticks** (authored): band P turns
//!   once in P ticks, so its radius is `r = Q · P` and its speed `v = Q`.
//! - **A band of period P passes P frames a turn;** each frame holds one
//!   electron in each sense, so the band holds 2P. Linear in P, no square.
//! - **A new electron takes a free frame before it pairs** (authored): it
//!   joins a frame already turned the other way only when every frame of
//!   the band is taken. Opposite senses on one frame pair; a band with every
//!   frame paired is closed.
//! - Electrons are placed inside out, each in the lowest band with room.
//!
//! What is read (never looked up): the depth (outermost band), the unpaired
//! count in it (bonding), its electrons (the share of a compound), whether it
//! is closed (inert), and how tightly it is held, `Q_out / P_out` (its speed
//! over its period).

use crate::ratio::{ratio, Ratio};

/// An atom's reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom {
    pub z: u32,
    /// The outermost band's period: its depth.
    pub depth: u32,
    /// Electrons in the outermost band.
    pub outer: u32,
    /// Of those, unpaired: what it can bond.
    pub unpaired: u32,
    /// Every frame of the outermost band paired.
    pub closed: bool,
    /// The charge the outermost band encloses.
    pub enclosed: u32,
    /// How tightly the outermost band is held: `Q_out / P_out`.
    pub tightness: Ratio,
}

/// The atom of `z` charges, neutral (z electrons). `None` for z = 0.
pub fn atom(z: u32) -> Option<Atom> {
    if z == 0 {
        return None;
    }
    // Band P holds 2P; place inside out.
    let (mut left, mut depth, mut inner) = (z, 0u32, 0u32);
    let mut outer = 0;
    while left > 0 {
        depth += 1;
        let room = 2 * depth;
        outer = left.min(room);
        left -= outer;
        if left > 0 {
            inner += outer;
        }
    }
    // Free frames first, then pairs: n ≤ P are all unpaired; past P each
    // new electron pairs one.
    let unpaired = if outer <= depth { outer } else { 2 * depth - outer };
    let enclosed = z - inner;
    Some(Atom {
        z,
        depth,
        outer,
        unpaired,
        closed: outer == 2 * depth,
        enclosed,
        tightness: ratio(i64::from(enclosed), i64::from(depth)),
    })
}

/// A CANONICAL ELEMENT (the refactor onto era 1, 2026-10-15): era 1's matter
/// is one element per Z. Its id carries the mark `Zatm` at bytes 8..12 and
/// Z at 12..14 (little-endian); bytes 1 and 2 stay a hash of Z (the
/// element's family and tier, `dop::element_properties`).
const MARK: [u8; 4] = *b"Zatm";

pub fn canonical(z: u32) -> [u8; 32] {
    let mut id = *blake3::Hasher::new_derive_key("netstratum element Z v1").update(&z.to_le_bytes()).finalize().as_bytes();
    id[8..12].copy_from_slice(&MARK);
    id[12..14].copy_from_slice(&(z as u16).to_le_bytes());
    id
}

/// The Z of a canonical element, or None for an era-0 hash-drawn kind.
pub fn z_of(id: &[u8; 32]) -> Option<u32> {
    (id[8..12] == MARK).then(|| u32::from(u16::from_le_bytes([id[12], id[13]])))
}

/// THE NUCLEUS (the same law one tier in): its Z protons and its neutrons
/// pair, one of each kind to a frame, and pairing is what closes (design law
/// 6), so a closed nucleus holds as many neutrons as protons, 2Z nucleons.
/// The neutron excess of heavy nuclei, which charge's push would bring, is
/// not yet law (docs/78, Open).
///
/// A GRAIN'S OWN RING (docs/83 "The derivation", netstratum's share of the
/// ring-density law; operator 2026-10-15): an atom is a small bell, its tick
/// a ring received. HOLDING BELONGS TO CLOSED PAIRS: every paired member (a
/// nucleon, an electron in a paired frame) holds each passing ring one tick;
/// an unpaired electron is open, free to bond, and holds nothing. So bonding
/// and weight are two sides of pairing, and an atom's mass reading is its
/// hold: `2Z + (Z − unpaired)` rings. The mass number is this reading.
pub fn hold(z: u32) -> u32 {
    atom(z).map(|a| 2 * z + (z - a.unpaired)).unwrap_or(0)
}

/// A COMPOUND (design law 6 across atoms): a bond is an unpaired electron of
/// one atom paired with one of another, turning the other way. A set of
/// atoms (by Z, with repeats) closes when every unpaired electron in it
/// pairs with one in ANOTHER atom of the set and the set holds together:
/// - a closed atom (none unpaired) bonds nothing;
/// - the unpaired count is even (every bond takes two);
/// - no atom has more unpaired than all the others together (its partners
///   must be elsewhere);
/// - the bonds can join every atom (at least one fewer than the atoms).
/// Exact conditions for a connected bond graph with those counts; nothing
/// is searched.
pub fn closes(zs: &[u32]) -> bool {
    let Some(counts) = zs.iter().map(|&z| atom(z).map(|a| a.unpaired)).collect::<Option<Vec<u32>>>() else {
        return false;
    };
    if zs.len() < 2 || counts.iter().any(|&u| u == 0) {
        return false;
    }
    let total: u32 = counts.iter().sum();
    let most = counts.iter().copied().max().unwrap_or(0);
    total % 2 == 0 && most <= total - most && total / 2 + 1 >= zs.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bands_close_at_two_p() {
        // Band 1 holds 2, band 2 holds 4: closed at z = 2 and z = 6.
        assert!(atom(2).unwrap().closed);
        assert!(atom(6).unwrap().closed);
        assert!(!atom(5).unwrap().closed);
        // In band 2: 1, 2 unpaired rising, then pairing down.
        let unpaired: Vec<u32> = (3..=6).map(|z| atom(z).unwrap().unpaired).collect();
        assert_eq!(unpaired, vec![1, 2, 1, 0]);
    }

    #[test]
    fn holding_belongs_to_closed_pairs() {
        // Hydrogen: 2 nucleons hold, its lone electron is open.
        assert_eq!(hold(1), 2);
        // Helium closes its band: 4 nucleons and 2 paired electrons.
        assert_eq!(hold(2), 6);
        // Z = 4: 8 nucleons; band 1's pair holds, band 2's two are open.
        assert_eq!(hold(4), 8 + 2);
    }

    #[test]
    fn compounds_close_by_pairing_across_atoms() {
        assert!(closes(&[1, 1]), "two single bonders pair");
        assert!(closes(&[4, 1, 1]), "a double bonder with two singles");
        assert!(!closes(&[4, 1]), "one bond left open");
        assert!(!closes(&[2, 1]), "a closed atom bonds nothing");
        assert!(!closes(&[1]), "no partner");
    }
}
