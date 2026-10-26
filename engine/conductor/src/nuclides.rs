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
}
