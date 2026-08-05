//! COMPOSITION — what elements make together.
//!
//! The rule is the LANES', not the members': any set drawn from the
//! same lanes compounds alike whatever their lines, so one recipe names
//! a FAMILY of gems. That is the periodicity paying out — a
//! civilization that has made one gem knows the recipe for the rest.

use super::periodic::{periodic_of, Periodic, LANES};

/// Do these elements COMPOUND? Three conditions, one per participating
/// axis:
///
///   SHELL  every member shares the geometry, and the count entering
///          equals its `arity`. Above two bodies a compound is
///          irreducibly NOT A PAIR — the multi-party case a pairwise
///          projection cannot express.
///   PHASE  every member is workable in the same state. Matter that
///          cannot coexist does not bond.
///   LANE   the binds SATURATE across the whole set:
///          `Σ binds · 2 == arity · (LANES/2 + 1)`.
///
/// **Every arity now saturates.** At `LANES = 8` the figure was
/// `arity · 5 / 2` and odd arities were unreachable in whole binds; the
/// widening to 10 makes it `arity · 3`. See [`super::periodic::LANES`].
pub fn compounds(elements: &[[u8; 32]]) -> bool {
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

/// The gem a set makes — content-addressed, so the SAME set always
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

/// **KNOWLEDGE** — the depth AND the width that were won: the sum of
/// the members' lines. A deeper compound carries more than a shallow
/// one, and a wider one more than a narrow one at the same depth.
///
/// No compound, no knowledge: `None`, never a zero.
pub fn knowledge_of(elements: &[[u8; 32]]) -> Option<u16> {
    compounds(elements)
        .then(|| elements.iter().map(|e| u16::from(periodic_of(e).line)).sum())
}
