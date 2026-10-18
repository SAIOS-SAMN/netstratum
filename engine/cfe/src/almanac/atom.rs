//! docs/78 "The atom": the lowest tier of the one law. An atom is not a cell
//! of a table; it is a nucleus of Z charges with Z electrons turning about
//! it, and its chemistry is read from that motion.
//!
//! THE HARMONIC ATOM (operators 2026-10-18, with xylarium: "keep atomic at
//! S^11 - so it fits the harmonic model"; "hopf fibrations are absolutely
//! required"). Count the windings, flow the phases: no frames, no lattice.
//! - The atom turns on S^11: twelve dimensions, six turning planes, no axis.
//!   By the Hopf fibration S^11 is a bundle of circles over CP^5, so every
//!   point has its own S^1 fibre, and SPIN IS THE TURNING ALONG IT. A pair is
//!   two counter-turning flows on one fibre; an unpaired turning runs
//!   uncancelled (its magnetism, and what it can bond).
//! - A SHELL is the windings that close with count n; a winding's SHAPE l
//!   (0 ≤ l < n) is its eccentricity, axes in the ratio `(l + 1)/n`.
//! - A shape's ORIENTATIONS are exact and topological: the degree-l
//!   harmonics in twelve dimensions, `C(l + 11, 11) − C(l + 9, 11)` = 1, 12,
//!   77, 352, …, each holding one counter-turning pair. Shells hold 2, 26,
//!   180, …; Earth's `2l + 1` is refused.
//! - Each electron takes the TIGHTEST open winding (the charge it encloses
//!   at its closest approach over its period), on the ring-density radius
//!   law, nested by construction (`build`); a free orientation before it
//!   pairs. Crossings between shells are read, never stated; Earth's n + l
//!   order is refused.
//! - An orbit closes in a whole number of ticks: shell n turns once in n, so
//!   its radius is `r = Q · n` and its speed `v = Q`, Q the charge it
//!   encloses (the nucleus less the shells further in).
//!
//! What is read (never looked up): the depth (outermost shell), the unpaired
//! count (bonding, magnetism), its electrons in the outermost shell, whether
//! it is closed (inert), and how tightly it is held, `Q_out / n_out`.

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

/// The orientations of shape l on S^11: the degree-l harmonics in twelve
/// dimensions, `C(l + 11, 11) − C(l + 9, 11)`, each holding one pair.
pub fn orientations(l: u32) -> u64 {
    fn c(n: u64, k: u64) -> u64 {
        if k > n {
            return 0;
        }
        (0..k).fold(1u64, |acc, i| acc * (n - i) / (i + 1))
    }
    let l = u64::from(l);
    c(l + 11, 11) - if l >= 2 { c(l + 9, 11) } else { 0 }
}

/// A shell's room: twice every shape's orientations, `Σ_{l<n} 2·orient(l)`.
pub fn shell_room(n: u32) -> u64 {
    (0..n).map(|l| 2 * orientations(l)).sum()
}

/// The atom of `z` charges, neutral (z electrons). `None` for z = 0.
pub fn atom(z: u32) -> Option<Atom> {
    if z == 0 {
        return None;
    }
    windings(z).map(|(a, _)| a)
}

/// A WINDING of a built atom: its tightness (the charge it encloses at its
/// closest approach over its period, in hydrogen's own unit: hydrogen's lone
/// electron reads 1), its electrons, and its orientations.
pub type Winding = (Ratio, u64, u64);

/// The atom of `z` with its windings (memoised per Z).
pub fn windings(z: u32) -> Option<(Atom, Vec<Winding>)> {
    if z == 0 {
        return None;
    }
    type Memo = std::collections::BTreeMap<u32, (Atom, Vec<Winding>)>;
    static MEMO: std::sync::OnceLock<std::sync::Mutex<Memo>> = std::sync::OnceLock::new();
    let memo = MEMO.get_or_init(|| std::sync::Mutex::new(Memo::new()));
    if let Some(a) = memo.lock().ok().and_then(|m| m.get(&z).cloned()) {
        return Some(a);
    }
    let a = build(z);
    if let Ok(mut m) = memo.lock() {
        m.insert(z, a.clone());
    }
    Some(a)
}

/// IONIZATION BY HEAT (operators 2026-10-18: the missing layer, "let there
/// be light"; with xylarium): in a hot place (a star's heart) every winding
/// held less tightly than the heat, both in hydrogen's own unit, is
/// STRIPPED (at or below), its electrons gone. Returns the open (unpaired)
/// count of what remains. Hydrogen (tightness 1) is stripped at heat 1;
/// heavy kinds keep their deep windings. Heat 0: the whole atom.
pub fn open_at(z: u32, heat: &Ratio) -> u32 {
    let Some((a, ws)) = windings(z) else { return 0 };
    if *heat <= ratio(0, 1) {
        return a.unpaired;
    }
    // Stripped AT OR BELOW the heat (operator, 2026-10-18): a heart that
    // rings strips what is held as loosely as hydrogen's lone electron.
    let left: u64 = ws.iter().filter(|(t, _, _)| *t > *heat).map(|(_, v, o)| if *v <= *o { *v } else { 2 * o - v }).sum();
    u32::try_from(left).unwrap_or(u32::MAX)
}

/// THE FILL (operators 2026-10-18, with xylarium; the ring-density radius
/// law, ruled): a ring reaching shell n is held by every bell inside it, the
/// nucleus and every inner electron, so `H(<r) = Z` and each shell's reach
/// adds `n·Z`: `R_n = Z·n(n + 1)/2`, nested by construction. A winding of
/// shape l comes closest at `(l + 1)/n` of its shell's reach,
/// `Z·(n + 1)(l + 1)/2`, and there it encloses the nucleus less every
/// electron of another shell standing inside, `m(m + 1) < (n + 1)(l + 1)`.
/// Its TIGHTNESS is that charge over its period n. Each electron takes the
/// tightest open winding (a free orientation before it pairs); crossings
/// between shells are READ, never stated (Earth's n + l refused). On an
/// exact tie the inner shell, then the lower shape, goes first (the
/// carrier's choice).
fn build(z: u32) -> (Atom, Vec<Winding>) {
    let mut occ: std::collections::BTreeMap<(u32, u32), u64> = std::collections::BTreeMap::new();
    for _ in 0..z {
        let nmax = occ.keys().map(|(n, _)| *n).max().unwrap_or(0) + 1;
        let mut best: Option<(Ratio, u32, u32)> = None;
        for n in 1..=nmax {
            for l in 0..n {
                if occ.get(&(n, l)).copied().unwrap_or(0) >= 2 * orientations(l) {
                    continue;
                }
                let reach = u64::from(n + 1) * u64::from(l + 1);
                let inside: u64 = occ.iter().filter(|((m, _), _)| *m != n && u64::from(*m) * u64::from(m + 1) < reach).map(|(_, v)| *v).sum();
                let t = ratio(i64::from(z) - inside as i64, i64::from(n));
                let better = match &best {
                    None => true,
                    Some((bt, bn, bl)) => t > *bt || (t == *bt && (n, l) < (*bn, *bl)),
                };
                if better {
                    best = Some((t, n, l));
                }
            }
        }
        let (_, n, l) = best.expect("a shell always has room");
        *occ.entry((n, l)).or_insert(0) += 1;
    }
    let depth = occ.keys().map(|(n, _)| *n).max().unwrap_or(1);
    let outer: u64 = occ.iter().filter(|((n, _), _)| *n == depth).map(|(_, v)| *v).sum();
    let inner: u64 = occ.iter().filter(|((n, _), _)| *n < depth).map(|(_, v)| *v).sum();
    let unpaired: u64 = occ.iter().map(|((_, l), v)| { let o = orientations(*l); if *v <= o { *v } else { 2 * o - v } }).sum();
    let closed = occ.iter().all(|((_, l), v)| *v == 2 * orientations(*l)) && (0..depth).all(|l| occ.get(&(depth, l)).copied().unwrap_or(0) == 2 * orientations(l));
    let to32 = |x: u64| u32::try_from(x).unwrap_or(u32::MAX);
    let enclosed = to32(u64::from(z) - inner);
    // Each occupied winding's tightness in the finished atom, as the fill
    // reads it: the charge enclosed at its closest approach over its period.
    let ws: Vec<Winding> = occ
        .iter()
        .map(|((n, l), v)| {
            let reach = u64::from(n + 1) * u64::from(l + 1);
            let inside: u64 = occ.iter().filter(|((m, _), _)| m != n && u64::from(*m) * u64::from(m + 1) < reach).map(|(_, v)| *v).sum();
            (ratio(i64::from(z) - inside as i64, i64::from(*n)), *v, orientations(*l))
        })
        .collect();
    (Atom { z, depth, outer: to32(outer), unpaired: to32(unpaired), closed, enclosed, tightness: ratio(i64::from(enclosed), i64::from(depth)) }, ws)
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
/// A GRAIN'S OWN HOLD (docs/83 "The derivation", netstratum's share of the
/// ring-density law; operator 2026-10-15). HOLDING BELONGS TO CLOSED PAIRS:
/// every paired member (a nucleon, an electron in a paired frame) holds a
/// passing ring; an unpaired electron is open, free to bond, and holds
/// nothing. So bonding and weight are two sides of pairing, and an atom's
/// mass reading is its hold, `2Z + (Z − unpaired)`: the mass number is this
/// reading. It is a relative WEIGHT, never an atomic beat: on the core's
/// clock it is normalised by the derived clock ratio (all the universe's
/// grains together hold exactly one core ring), so a grain's hold in core
/// rings is its weight over the manifest's total weight, the gravitational
/// hold, linear and additive.
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
    fn the_harmonic_shells_close_at_two_and_twenty_eight() {
        // Orientations of the degree-l harmonics on S^11.
        assert_eq!((0..4).map(orientations).collect::<Vec<_>>(), vec![1, 12, 77, 352]);
        assert_eq!((1..=3).map(shell_room).collect::<Vec<_>>(), vec![2, 26, 180]);
        assert!(atom(2).unwrap().closed);
        assert!(atom(28).unwrap().closed);
        assert!(!atom(27).unwrap().closed);
        assert!(atom(208).unwrap().closed);
        // Shell 2: its round shape pairs (Z 3, 4), then 12 free orientations
        // of the next before pairing (Z 5..16 rising, 17..28 falling).
        let unpaired: Vec<u32> = (3..=7).map(|z| atom(z).unwrap().unpaired).collect();
        assert_eq!(unpaired, vec![1, 0, 1, 2, 3]);
        assert_eq!(atom(16).unwrap().unpaired, 12);
        assert_eq!(atom(28).unwrap().unpaired, 0);
        assert_eq!(atom(29).unwrap().depth, 3);
    }

    #[test]
    fn heat_strips_the_loosest_windings_first() {
        // Hydrogen's lone electron reads 1: whole below, stripped above.
        assert_eq!(open_at(1, &ratio(0, 1)), 1);
        assert_eq!(open_at(1, &ratio(1, 2)), 1);
        assert_eq!(open_at(1, &ratio(1, 1)), 0);
        // A heavy kind keeps its deep windings at a heat that strips hydrogen.
        let z = 18;
        assert!(open_at(z, &ratio(2, 1)) <= atom(z).unwrap().unpaired);
        assert_eq!(open_at(z, &ratio(1000, 1)), 0);
    }

    #[test]
    fn holding_belongs_to_closed_pairs() {
        // Hydrogen: 2 nucleons hold, its lone electron is open.
        assert_eq!(hold(1), 2);
        // Helium closes its shell: 4 nucleons and 2 paired electrons.
        assert_eq!(hold(2), 6);
        // Z = 4: 8 nucleons; shell 2's round shape is paired, so all hold.
        assert_eq!(hold(4), 8 + 4);
    }

    #[test]
    fn compounds_close_by_pairing_across_atoms() {
        assert!(closes(&[1, 1]), "two single bonders pair");
        assert!(closes(&[6, 1, 1]), "a double bonder with two singles");
        assert!(!closes(&[6, 1]), "one bond left open");
        assert!(!closes(&[2, 1]), "a closed atom bonds nothing");
        assert!(!closes(&[1]), "no partner");
    }
}
