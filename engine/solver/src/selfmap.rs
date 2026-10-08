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
use rfa_core::{holonomies, ratio, rcf_hash_skew_bipartite, Delta, Holonomy, Ratio, Support};

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

/// The family block `M` of the projected Delta: `M[f][w]` is word `w`'s
/// draw on family `f` — the same weights `project` writes, rows = families.
fn family_block(library: &Library) -> Vec<Vec<Ratio>> {
    let words = library.words();
    let mut m = vec![vec![ratio(0, 1); words.len()]; FAMILIES.len()];
    for (w, word) in words.iter().enumerate() {
        for op in &word.expansion {
            let family = family_of(op);
            let fidx = FAMILIES.iter().position(|f| *f == family).expect("family is one of four");
            m[fidx][w] = m[fidx][w].clone() + ratio(1, 1);
        }
    }
    m
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
    let frame = sparse_potentials(library);
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

/// THE FOREST POTENTIALS of the family↔word graph, SPARSE: exactly what
/// `forest_potentials(project(library))` reads (roots in index order, each
/// node's neighbours scanned ascending, a potential set as its node is
/// pushed, the frontier a stack), walked on adjacency lists instead of a
/// dense `(4 + words)²` Delta — which every growing library rebuilt, at
/// gigabytes and seconds a time (measured 2026-10-08: the conductor's
/// consilience_step peaking at 3.8G). The graph is bipartite: a family links
/// only words that draw on it, a word only its families.
fn sparse_potentials(library: &Library) -> Vec<Ratio> {
    let words = library.words();
    let dim = FAMILIES.len() + words.len();
    // weight[w][f]: how much word w draws on family f.
    let weights: Vec<[i64; FAMILIES.len()]> = words
        .iter()
        .map(|word| {
            let mut weight = [0i64; FAMILIES.len()];
            for op in &word.expansion {
                let family = family_of(op);
                let fidx = FAMILIES.iter().position(|f| *f == family).expect("family is one of four");
                weight[fidx] += 1;
            }
            weight
        })
        .collect();
    // Neighbours, ascending: a family's words (4 + w), a word's families.
    let neighbours = |node: usize| -> Vec<(usize, Ratio)> {
        if node < FAMILIES.len() {
            weights
                .iter()
                .enumerate()
                .filter(|(_, w)| w[node] > 0)
                .map(|(w, weight)| (FAMILIES.len() + w, ratio(weight[node], 1)))
                .collect()
        } else {
            let weight = &weights[node - FAMILIES.len()];
            (0..FAMILIES.len()).filter(|&f| weight[f] > 0).map(|f| (f, ratio(-weight[f], 1))).collect()
        }
    };
    let mut potential: Vec<Option<Ratio>> = vec![None; dim];
    for root in 0..dim {
        if potential[root].is_some() {
            continue;
        }
        potential[root] = Some(ratio(0, 1));
        let mut frontier = vec![root];
        while let Some(here) = frontier.pop() {
            let base = potential[here].clone().unwrap_or_else(|| ratio(0, 1));
            for (next, entry) in neighbours(here) {
                if next == here || potential[next].is_some() {
                    continue;
                }
                potential[next] = Some(base.clone() + entry);
                frontier.push(next);
            }
        }
    }
    potential.into_iter().map(|p| p.unwrap_or_else(|| ratio(0, 1))).collect()
}

/// **THE FULL MAP** — the curiosity order and
/// the crystallized invariant, in one reading.
///
/// The signature runs `rcf_hash` (invariant factors, `O(dim³)` in the
/// number of concepts) and is heavier than [`knowledge_order`]. It is the
/// INSPECTION and CRYSTALLIZATION reading — for a workbench view, or for
/// the cross-line consilience that mints when two lines' maps carry the
/// same signature — not the per-beat reach.
pub fn knowledge_map(library: &Library) -> SelfMap {
    SelfMap {
        order: knowledge_order(library),
        // The map's only relations run family ↔ word, antisymmetrically, so
        // its RCF signature is read from the 4×W family block exactly
        // (`rcf_hash_skew_bipartite`, byte-identical to `rcf_hash` of the
        // projected Delta — witnessed in rfa-core): k×k algebra, not a
        // dim² Smith form. Measured: 118 s → milliseconds at 467 words.
        signature: rcf_hash_skew_bipartite(FAMILIES.len(), &family_block(library)),
    }
}

/// **THE IRREDUCIBLE TENSIONS** of a line's knowledge: cycles of affinity
/// the coherent frame could not explain away (empty means the knowledge
/// composes). A reading on demand, no longer part of [`knowledge_map`]:
/// it projects the library onto a DENSE `(4 + words)²` Delta of exact
/// rationals — over 16 million entries at 4,000 words — which every line
/// rebuilt at every boot while nothing read it (measured 2026-10-08: the
/// conductor's 3G heap at its first beat, `consilience_step`).
pub fn knowledge_tension(library: &Library) -> Vec<Holonomy> {
    let (delta, support) = project(library);
    holonomies(&delta, &support)
}

/// **CROSS-LINE CONSILIENCE** (38 §5) — the shared knowledge no single
/// line owns.
///
/// A [`SelfMap::signature`] is the crystallized invariant of ONE line's
/// resolved knowledge: two lines carry the same 32 bytes exactly when
/// their knowledge is the same shape up to a frame. When INDEPENDENT
/// lines — different vocabularies, different histories — converge on one
/// signature, that agreement is a MEASUREMENT, not a reconciled copy
/// (the one-truth amendment bars reconciling copies; it does not bar
/// observing that two derivations landed on the same structure — the
/// same law the ARC mint's three-distinct-tasks already runs on).
///
/// This is the KNOWLEDGE row of `38 §1`, realized: mass is the effort
/// share (how many lines reached it), hardness is the diversity bridged
/// (how differently they are built — a truth two UNLIKE lines share is
/// harder-won than one two near-twins do).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consilience {
    /// The shared invariant the converged lines all crystallized.
    pub signature: [u8; 32],
    /// The converged lines, by id, in ascending order.
    pub lines: Vec<u64>,
    /// Effort share — how many independent lines reached this knowledge.
    pub mass: usize,
    /// Diversity bridged — how many DISTINCT curiosity-orders the
    /// converged lines wear. Two lines that reached the same knowledge by
    /// the same route bridge nothing; two that reached it from different
    /// frontiers bridge a gap, and the knowledge is harder for it.
    pub hardness: usize,
}

/// Detect the cross-line consilences among a roster of resolved maps.
///
/// Each `(line_id, map)` is one line's own reading. Lines are grouped by
/// their exact signature; a group of TWO OR MORE independent lines is a
/// consilience. A lone line is no agreement and yields nothing. The
/// zero-map signature (an empty library) is skipped — a line that knows
/// nothing has reached no shared truth by having nothing in common.
/// Deterministic: groups are returned in ascending signature order, each
/// naming its lines in ascending id order.
pub fn consilience(maps: &[(u64, &SelfMap)]) -> Vec<Consilience> {
    let sensed: Vec<(u64, &SelfMap, crate::senses::Senses)> =
        maps.iter().map(|&(line, map)| (line, map, crate::senses::Senses::ALL)).collect();
    consilience_sensed(&sensed)
}

/// [`consilience`] where each line also carries its SENSES (docs/60 D5): a
/// route is a curiosity order AND a way of perceiving, so a truth lines of
/// different perceptions reach is harder than one alike lines reach.
pub fn consilience_sensed(maps: &[(u64, &SelfMap, crate::senses::Senses)]) -> Vec<Consilience> {
    let empty = knowledge_map(&Library::new()).signature;
    let mut groups: std::collections::BTreeMap<[u8; 32], Vec<(u64, &SelfMap, crate::senses::Senses)>> =
        std::collections::BTreeMap::new();
    for &(line, map, senses) in maps {
        if map.signature == empty {
            continue; // nothing known is nothing shared
        }
        groups.entry(map.signature).or_default().push((line, map, senses));
    }
    let mut found = Vec::new();
    for (signature, mut members) in groups {
        if members.len() < 2 {
            continue; // one line is not a consilience
        }
        members.sort_by_key(|(line, _, _)| *line);
        let lines: Vec<u64> = members.iter().map(|(line, _, _)| *line).collect();
        // Diversity bridged: the count of DISTINCT routes among the
        // converged lines, a route being a curiosity order and the senses
        // that walked it. Same knowledge from the same frontier, seen the
        // same way, bridges nothing; the more distinct the routes, the
        // harder-won.
        let mut orders: Vec<(&Vec<Family>, u8)> = members.iter().map(|(_, m, senses)| (&m.order, senses.0)).collect();
        orders.sort();
        orders.dedup();
        found.push(Consilience {
            signature,
            mass: lines.len(),
            hardness: orders.len(),
            lines,
        });
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use rfa_core::forest_potentials;

    #[test]
    fn the_sparse_forest_reads_exactly_what_the_dense_one_did() {
        let mut lib = Library::new();
        for (i, expansion) in [
            vec![Op::Rotate90, Op::FlipH],
            vec![Op::Recolor([0, 2, 1, 3, 4, 5, 6, 7, 8, 9]), Op::Transpose],
            vec![Op::Tile { down: 2, across: 1, odd_row: TileMirror::None, odd_col: TileMirror::None }, Op::Rotate180, Op::Rotate180],
            vec![Op::FlipV, Op::Recolor([1, 0, 2, 3, 4, 5, 6, 7, 8, 9])],
        ]
        .into_iter()
        .enumerate()
        {
            let mut w = word(expansion);
            w.orbit = [i as u8 + 1; 32];
            lib.adopt(w);
            let (delta, support) = project(&lib);
            assert_eq!(sparse_potentials(&lib), forest_potentials(&delta, &support)[0], "after {} words", i + 1);
        }
        assert!(sparse_potentials(&Library::new()).len() == FAMILIES.len());
    }
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

    /// A knowledge shape is frame-invariant: a line whose library spells
    /// the SAME structure with re-labelled symmetry letters crystallizes
    /// the SAME signature — the ground the cross-line consilience stands
    /// on.
    #[test]
    fn the_same_shape_crystallizes_the_same_signature() {
        let mut a = Library::new();
        a.adopt(word(vec![Op::Rotate90, tile()]));
        let mut b = Library::new();
        // A DIFFERENT symmetry letter, same family draw and structure.
        b.adopt(word(vec![Op::FlipH, tile()]));
        assert_eq!(
            knowledge_map(&a).signature,
            knowledge_map(&b).signature,
            "same shape up to a frame → same crystallized knowledge"
        );
    }

    /// Two INDEPENDENT lines that crystallize the same signature are a
    /// consilience; a lone line is not; an empty line shares nothing.
    #[test]
    fn independent_lines_that_converge_are_a_consilience() {
        let mut a = Library::new();
        a.adopt(word(vec![Op::Rotate90, tile()]));
        let mut b = Library::new();
        b.adopt(word(vec![Op::FlipH, tile()])); // same shape, different letter
        let mut lone = Library::new();
        // A STRUCTURALLY distinct shape: one word drawing on a SINGLE
        // family (a pendant), not a two-family bridge — a different
        // invariant, so a different crystallized knowledge.
        lone.adopt(word(vec![Op::Rotate90, Op::Rotate180]));
        let empty = Library::new();

        let ma = knowledge_map(&a);
        let mb = knowledge_map(&b);
        let ml = knowledge_map(&lone);
        let me = knowledge_map(&empty);
        let found = consilience(&[(1, &ma), (2, &mb), (3, &ml), (7, &me)]);

        assert_eq!(found.len(), 1, "exactly one shared truth — lines 1 and 2");
        let c = &found[0];
        assert_eq!(c.lines, vec![1, 2], "the two converged lines, in id order");
        assert_eq!(c.mass, 2, "effort share is the count that reached it");
        assert_eq!(c.signature, ma.signature, "keyed by the shared invariant");
    }

    /// An empty roster and a roster of all-distinct lines both yield no
    /// consilience — agreement is the measurement, and there is none.
    #[test]
    fn distinct_lines_share_nothing() {
        let mut a = Library::new();
        a.adopt(word(vec![Op::Rotate90, tile()]));
        let mut b = Library::new();
        b.adopt(word(vec![recolor([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), Op::Transpose, tile()]));
        let ma = knowledge_map(&a);
        let mb = knowledge_map(&b);
        assert!(consilience(&[(1, &ma), (2, &mb)]).is_empty(), "no two share a shape");
        assert!(consilience(&[]).is_empty(), "an empty roster converges nothing");
    }
}

#[cfg(test)]
mod signature_witness {
    use super::*;

    #[test]
    fn the_fast_signature_is_the_projected_deltas_rcf_hash() {
        // A library built from real families, with repeats and mixed words.
        use solver_verify::grid::{Op, TileMirror};
        let mut map = [0u8; 10];
        map[1] = 2;
        let tile = Op::Tile { down: 2, across: 1, odd_row: TileMirror::Rows, odd_col: TileMirror::None };
        let ops = vec![Op::Rotate90, Op::Recolor(map), Op::FlipH, tile.clone(), Op::Transpose, Op::Recolor(map), Op::Rotate180, tile, Op::FlipV, Op::Rotate270, Op::Recolor(map), Op::FlipH, Op::Rotate90];
        let mut library = Library::new();
        for (n, window) in ops.windows(2).enumerate().take(12) {
            let mut orbit = [0u8; 32];
            orbit[0] = n as u8;
            library.adopt(crate::mint::MintedWord { orbit, expansion: window.to_vec(), cited: Vec::new() });
        }
        let (delta, _) = project(&library);
        assert_eq!(knowledge_map(&library).signature, rfa_core::rcf_hash(&delta));
    }
}
