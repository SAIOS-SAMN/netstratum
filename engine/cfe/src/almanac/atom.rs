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
}
