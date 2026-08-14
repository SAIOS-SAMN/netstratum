//! **THE RESOLVED KNOWLEDGE-MAP** — curiosity read from the STRUCTURE of
//! what a line knows, not from a clock (38 §6, task #15, the algebraic
//! form).
//!
//! [`crate::mint::family_census`] orders the families by a flat COUNT of
//! minted words. This orders them by where each family SITS in the line's
//! resolved knowledge — the same reach question answered by the recursive
//! algebra rather than a tally.
//!
//! The map is a relational `Delta` over CONCEPTS. The four families are
//! anchor nodes; every minted word the line holds is a node too, so the
//! map grows richer exactly as the line learns. A word is joined to each
//! family its expansion draws on, weighted by how many of its letters
//! come from that family. Two words that draw on the same two families
//! close a cycle between them — the map's own topology, formed by the
//! line's vocabulary.
//!
//! [`rfa_core::forest_potentials`] resolves this web into ONE coherent
//! frame — a potential per concept — and the family whose potential sits
//! FARTHEST from the line's knowledge centroid is the most peripheral:
//! the frontier a curious mind reaches for first. What the frame cannot
//! explain — [`rfa_core::holonomies`], cycles of affinity that do not
//! compose — is the line's genuine structural tension, surfaced whole.
//! [`rfa_core::rcf_hash`] of the map is the crystallized invariant: two
//! lines whose knowledge is the same shape up to a frame carry the same
//! signature (the hook the cross-line consilience reads).
//!
//! **Ordering only, as ever** (the reach law): this decides what a
//! starving mind reaches for FIRST; the observation still supplies every
//! candidate, and every family stays reachable.

use crate::mint::Library;
use crate::need::{family_of, Family};
use rfa_core::{forest_potentials, holonomies, ratio, rcf_hash, Delta, Holonomy, Ratio, Support};

/// The families in declaration order — the anchor nodes of the map and
/// the deterministic tie-break of last resort.
const FAMILIES: [Family; 4] = [Family::Symmetry, Family::Recolor, Family::Tile, Family::Object];

/// The fiber is one coordinate: relatedness. The knowledge map carries a
/// single affinity per edge, so every entry is a lone rational.
const AFFINITY: usize = 1;

/// The full resolved reading of a line's knowledge.
#[derive(Debug, Clone)]
pub struct SelfMap {
    /// The families, MOST-PERIPHERAL FIRST — the curiosity order a
    /// novelty-struck mind reaches by.
    pub order: Vec<Family>,
    /// The irreducible tensions: cycles of affinity the coherent frame
    /// could not explain away. Empty means the knowledge composes.
    pub tension: Vec<Holonomy>,
    /// The crystallized invariant of the whole map — the same 32 bytes
    /// for two knowledge-webs that are one shape up to a frame change.
    pub signature: [u8; 32],
}

/// Build the concept `Delta` and its support from a line's library.
///
/// Nodes `0..4` are the families; node `4 + w` is the `w`-th minted word.
/// A word joins each family its expansion draws on, weighted by how many
/// of its letters belong to that family. The support is built from the
/// joins themselves — an edge exists because the word draws on the
/// family, never because the weight came out non-zero.
fn project(library: &Library) -> (Delta, Support) {
    let words = library.words();
    let dim = FAMILIES.len() + words.len();
    let mut delta = Delta::zero(dim, AFFINITY);
    let mut support = Support::new(dim, AFFINITY);
    for (w, word) in words.iter().enumerate() {
        let node = FAMILIES.len() + w;
        let mut weight = [0i64; FAMILIES.len()];
        for op in &word.expansion {
            let family = family_of(op);
            let fidx = FAMILIES.iter().position(|f| *f == family).expect("family is one of four");
            weight[fidx] += 1;
        }
        for (fidx, &w) in weight.iter().enumerate() {
            if w > 0 {
                // family → word, magnitude = the word's draw on that
                // family. `set_antisym` writes the −w twin, keeping the
                // tensor antisymmetric.
                delta.set_antisym(fidx, node, vec![ratio(w, 1)]);
                support.connect(fidx, node, 0);
            }
        }
    }
    (delta, support)
}

/// How many minted words touch each family — the census count, kept as
/// the deterministic tie-break when two families sit equally far from the
/// centroid (the thinner family, fewer words, still reaches first).
fn word_counts(library: &Library) -> [usize; FAMILIES.len()] {
    let mut counts = [0usize; FAMILIES.len()];
    for word in library.words() {
        for (fidx, family) in FAMILIES.iter().enumerate() {
            if word.expansion.iter().any(|op| family_of(op) == *family) {
                counts[fidx] += 1;
            }
        }
    }
    counts
}

/// **THE CURIOSITY ORDER** — the families ordered most-peripheral first,
/// read off the resolved knowledge frame. This is the live-path reading:
/// it runs the resolve ([`rfa_core::forest_potentials`], `O(dim²)`) and
/// nothing heavier, so a mind may consult it every beat it reaches.
///
/// A family's PERIPHERALITY is the distance of its resolved potential
/// from the line's knowledge centroid (the mean potential over every
/// concept). A weakly-developed family — few words, loosely joined —
/// drifts to the edge of the frame; the well-developed cluster pulls the
/// centroid toward itself. So farthest-from-centroid is thinnest-frontier,
/// the same reach `family_census` names, measured by the resolve instead
/// of a tally. Exact rational; ties break to the fewer-word family, then
/// to declaration order.
pub fn knowledge_order(library: &Library) -> Vec<Family> {
    let (delta, support) = project(library);
    let potentials = forest_potentials(&delta, &support);
    let frame = &potentials[0];
    let dim = frame.len();
    // The knowledge centroid — the mean potential over every concept.
    let sum = frame.iter().fold(ratio(0, 1), |acc, p| acc + p);
    let centroid = sum / ratio(dim as i64, 1);
    // |p − centroid|, by comparison — the solver names no num_traits.
    let abs = |r: Ratio| -> Ratio {
        if r < ratio(0, 1) {
            -r
        } else {
            r
        }
    };
    let distance = |fidx: usize| -> Ratio { abs(frame[fidx].clone() - centroid.clone()) };

    let counts = word_counts(library);
    let mut order: Vec<usize> = (0..FAMILIES.len()).collect();
    order.sort_by(|&a, &b| {
        distance(b)
            .cmp(&distance(a)) // most peripheral first
            .then(counts[a].cmp(&counts[b])) // then thinner (fewer words) first
            .then(a.cmp(&b)) // then declaration order — deterministic
    });
    order.into_iter().map(|i| FAMILIES[i]).collect()
}

/// **THE FULL MAP** — the curiosity order, the irreducible tensions, and
/// the crystallized invariant, in one reading.
///
/// The signature runs `rcf_hash` (invariant factors, `O(dim³)` in the
/// number of concepts) and is heavier than [`knowledge_order`]. It is the
/// INSPECTION and CRYSTALLIZATION reading — for a workbench view, or for
/// the cross-line consilience that mints when two lines' maps carry the
/// same signature — not the per-beat reach.
pub fn knowledge_map(library: &Library) -> SelfMap {
    let (delta, support) = project(library);
    SelfMap {
        order: knowledge_order(library),
        tension: holonomies(&delta, &support),
        signature: rcf_hash(&delta),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mint::MintedWord;
    use solver_verify::grid::{Op, TileMirror};

    fn word(expansion: Vec<Op>) -> MintedWord {
        MintedWord { orbit: [0u8; 32], expansion, cited: Vec::new() }
    }

    fn recolor(map: [u8; 10]) -> Op {
        Op::Recolor(map)
    }

    fn tile() -> Op {
        Op::Tile { down: 2, across: 2, odd_row: TileMirror::None, odd_col: TileMirror::None }
    }

    /// An empty library resolves to the bare declaration order — the
    /// young-line fallback, byte-identical to no census.
    #[test]
    fn an_empty_library_reaches_in_declaration_order() {
        let order = knowledge_order(&Library::new());
        assert_eq!(order, FAMILIES.to_vec());
    }

    /// The family with NO words is the frontier: a mind whose whole
    /// vocabulary draws on Symmetry and Recolor reaches first for the
    /// families it has never worked — Tile and Object sit farthest from
    /// the centroid its known cluster defines.
    #[test]
    fn the_unworked_families_are_the_frontier() {
        let mut library = Library::new();
        // Two words, all letters drawn from Symmetry and Recolor.
        let mut lib_word = |exp: Vec<Op>| library.adopt(word(exp));
        lib_word(vec![Op::Rotate90, recolor([1, 0, 2, 3, 4, 5, 6, 7, 8, 9])]);
        lib_word(vec![Op::FlipH, recolor([0, 2, 1, 3, 4, 5, 6, 7, 8, 9])]);

        let order = knowledge_order(&library);
        // Tile and Object — untouched — come before Symmetry and Recolor.
        let tile_rank = order.iter().position(|f| *f == Family::Tile).unwrap();
        let object_rank = order.iter().position(|f| *f == Family::Object).unwrap();
        let sym_rank = order.iter().position(|f| *f == Family::Symmetry).unwrap();
        let recolor_rank = order.iter().position(|f| *f == Family::Recolor).unwrap();
        assert!(tile_rank < sym_rank, "the unworked Tile reaches before the worked Symmetry");
        assert!(object_rank < recolor_rank, "the unworked Object reaches before Recolor");
    }

    /// Deterministic: the same library resolves to the same order, every
    /// reading. Curiosity is a function of structure, nothing else.
    #[test]
    fn the_order_is_the_same_on_every_reading() {
        let mut library = Library::new();
        library.adopt(word(vec![Op::Rotate90, tile()]));
        library.adopt(word(vec![recolor([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), Op::Transpose]));
        assert_eq!(knowledge_order(&library), knowledge_order(&library));
    }

    /// Two words drawing on the same two families close a cycle whose
    /// affinities need not compose — the map carries real topology, and
    /// the crystallized signature is a stable 32 bytes.
    #[test]
    fn a_shared_pair_of_families_forms_a_map_with_a_signature() {
        let mut library = Library::new();
        // Both words draw on Symmetry AND Tile, but with different
        // weights — the two family→word edges disagree on the affinity,
        // so the four-cycle Symmetry→w0→Tile→w1→Symmetry carries charge.
        library.adopt(word(vec![Op::Rotate90, Op::Rotate180, tile()]));
        library.adopt(word(vec![Op::FlipH, tile()]));
        let map = knowledge_map(&library);
        assert_eq!(map.signature, knowledge_map(&library).signature, "the invariant is stable");
        assert_eq!(map.order.len(), FAMILIES.len(), "every family is placed");
    }
}
