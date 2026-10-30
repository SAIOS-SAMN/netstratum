//! Composition — what elements make together.
//!
//! The rule is the lanes', not the members': any set drawn from the
//! same lanes compounds alike whatever their lines, so one recipe names
//! a family of gems. That is the periodicity paying out — a
//! civilization that has made one gem knows the recipe for the rest.

use super::periodic::{periodic_of, Periodic, LANES};
use crate::ratio::{ratio, Ratio};

/// Do these elements compound? Three conditions, one per participating
/// axis:
///
///   SHELL  every member shares the geometry, and the count entering
///          equals its `arity`. Above two bodies a compound is
///          irreducibly not A pair — the multi-party case a pairwise
///          projection cannot express.
///   Phase  every member is workable in the same state. Matter that
///          cannot coexist does not bond.
///   Lane   the binds saturate across the whole set:
///          `Σ binds · 2 == arity · (LANES/2 + 1)`.
///
/// **Every arity now saturates.** At `LANES = 8` the figure was
/// `arity · 5 / 2` and odd arities were unreachable in whole binds; the
/// widening to 10 makes it `arity · 3`. See [`super::periodic::LANES`].
pub fn compounds(elements: &[[u8; 32]]) -> bool {
    // Era 1: canonical elements compound by the atom tier's law (a bond is
    // unpaired electrons pairing across atoms; `atom::closes`).
    let zs: Option<Vec<u32>> = elements.iter().map(super::atom::z_of).collect();
    if let Some(zs) = zs.filter(|z| !z.is_empty()) {
        return super::atom::closes(&zs);
    }
    let Some(first) = elements.first().map(periodic_of) else {
        return false;
    };
    if elements.len() != usize::from(first.arity) {
        return false;
    }
    let read: Vec<Periodic> = elements.iter().map(periodic_of).collect();
    if read.iter().any(|p| p.shell != first.shell || p.phase != first.phase) {
        return false;
    }
    let sum: u32 = read.iter().map(|p| u32::from(p.binds)).sum();
    sum * 2 == u32::from(first.arity) * u32::from(LANES / 2 + 1)
}

/// The gem a set makes — content-addressed, so the same set always
/// names the same gem and a "new" gem is new by IDENTITY rather than by
/// anyone's say-so.
///
/// Order-free: members are sorted before hashing, because a compound is
/// not a sequence. **Not spin-blind**: the set's spin parity enters the
/// digest, so identical composition with opposite handedness is a
/// different gem.
pub fn gem_of(elements: &[[u8; 32]]) -> Option<[u8; 32]> {
    if !compounds(elements) {
        return None;
    }
    let mut sorted: Vec<[u8; 32]> = elements.to_vec();
    sorted.sort_unstable();
    let parity: u8 = elements
        .iter()
        .map(|e| periodic_of(e).spin)
        .fold(0u8, |a, b| a ^ b);
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"netstratum.almanac.gem.v2");
    hasher.update(&[parity]);
    for member in &sorted {
        hasher.update(member);
    }
    Some(*hasher.finalize().as_bytes())
}

/// **Knowledge** — the depth and the width that were won: the sum of
/// the members' lines. A deeper compound carries more than a shallow
/// one, and a wider one more than a narrow one at the same depth.
///
/// No compound, no knowledge: `None`, never a zero.
pub fn knowledge_of(elements: &[[u8; 32]]) -> Option<u16> {
    compounds(elements)
        .then(|| elements.iter().map(|e| u16::from(periodic_of(e).line)).sum())
}

/// **A set that compounds, proven at construction** (P1,
/// 29_CONSTRUCTION — the [`super::super::world::survey::ForgeableGrade`]
/// precedent). Whether matter bonds is a property of the set ALONE:
/// the shells agree, the phases agree, and the binds saturate. None of
/// that reads the chain, so it is decidable at a door and a verb
/// holding one of these can never ask again.
///
/// The chain-dependent halves — is this element LOCAL to the star, has
/// the orb the pressure to refine it, is it workable at this
/// temperature — stay facts at the verb forever, because a set that
/// bonds nowhere today bonds tomorrow when the world moves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compoundable(Vec<[u8; 32]>);

impl Compoundable {
    /// The door. `None` where the matter does not bond.
    pub fn new(elements: &[[u8; 32]]) -> Option<Compoundable> {
        compounds(elements).then(|| Compoundable(elements.to_vec()))
    }
    /// The members, in the order they were offered.
    pub fn members(&self) -> &[[u8; 32]] {
        &self.0
    }
    /// The gem this set makes — total on a proven set, so no `Option`
    /// survives the door.
    pub fn gem(&self) -> [u8; 32] {
        gem_of(&self.0).expect("a compoundable set names its gem")
    }
    /// The knowledge it wins — likewise total past the door.
    pub fn knowledge(&self) -> u16 {
        knowledge_of(&self.0).expect("a compoundable set carries its knowledge")
    }
}

/// **The recipe — what a compounding spends and what it makes.** One
/// home: the deed settles it and the kernel re-derives it by calling
/// here (the `vein_wear` discipline).
///
/// A member gives up its lane's valence — `binds`, the quantity that
/// lane brings to any bond — and the gem carries away exactly the sum.
/// So the transformation is mass-exact by construction, and it needs no
/// number of its own: the compounding law already fixes that sum at
/// `arity · (LANES/2 + 1) / 2` for every set that bonds at all.
pub fn recipe_of(set: &Compoundable) -> (Vec<([u8; 32], Ratio)>, Ratio) {
    let spends: Vec<([u8; 32], Ratio)> = set
        .members()
        .iter()
        .map(|element| (*element, ratio(i64::from(periodic_of(element).binds), 1)))
        .collect();
    let yielded = spends
        .iter()
        .fold(ratio(0, 1), |sum, (_, spend)| sum + spend.clone());
    (spends, yielded)
}
