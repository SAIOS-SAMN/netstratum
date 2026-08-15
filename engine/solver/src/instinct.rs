//! **THE EVOLVED INSTINCT** (COG-7 — the Workbench chooser).
//!
//! A civilization's history reshapes the probability space of its cognition.
//! Two layers, and they flow into one another with NO new ledger — the fold is
//! over data the library already holds:
//!
//! - **Basic instinct (the genesis seed).** The species' irreducible prior — a
//!   base weight per generator family, from its disposition. It keeps every
//!   option minimally alive on beat 1 (the brainstem baseline), so a founding
//!   band already knows how to reach before it has minted anything.
//!
//! - **Evolved instinct (stochastic accretion).** Every word the line has
//!   minted was built from generators (operator families) and confirmed against
//!   obstructions (`cited`). The tally of that cited usage, per family, is the
//!   rut the line's own timeline has carved: a line that survived by minting one
//!   kind of geometry reaches first for that kind against the NEXT problem, even
//!   an unrelated one. Its past has rewritten its instinct.
//!
//! ```text
//!   I_total(family) = I_genesis(family) + Σ_{w spells family}  |w.cited|
//! ```
//!
//! Early game the genesis seed is all there is; late game the accreted weight of
//! the minted words drowns it out, and behaviour is dominated by the idiosyncratic
//! structures the line invented to survive its own solar system.
//!
//! **THE LAW, UNBROKEN** (`need.rs`): this only ORDERS/WEIGHTS the reach — a bias
//! on which family the Workbench tries first. The observation still supplies every
//! candidate; instinct never invents an operator the observation cannot support.

use crate::mint::Library;
use crate::need::{family_of, Family};

/// The four generator families, in declaration order — the axes an instinct
/// weights. (Mirrors `selfmap`'s private census order.)
const FAMILIES: [Family; 4] = [Family::Symmetry, Family::Recolor, Family::Tile, Family::Object];

fn index_of(family: Family) -> usize {
    FAMILIES.iter().position(|f| *f == family).unwrap_or(0)
}

/// **THE BASIC INSTINCT** — the genesis prior every founded line carries before
/// it has minted a word: one unit of reach on every family (nothing is
/// unthinkable) plus an extra unit on the families its DISPOSITION favours, so a
/// species leans toward its own nature from beat one. Empiricist minds lean to
/// STRUCTURE (Tile/Object — making what was not there); harmonic/other minds
/// lean to CORRESPONDENCE (Symmetry/Recolor — relating what is). A line whose
/// disposition names neither keeps the flat baseline.
pub fn genesis_instinct(disposition: &str) -> Vec<(Family, u64)> {
    let mut weight = [1u64; 4]; // the brainstem baseline — every option alive
    let favoured: &[Family] = match disposition {
        // Structure-seekers: the empiricist mind makes signal.
        "empiricist" => &[Family::Tile, Family::Object],
        // Spread/relate: the expansionist mind maps a thing onto new ground.
        "expansionist" => &[Family::Symmetry, Family::Recolor],
        _ => &[],
    };
    for f in favoured {
        weight[index_of(*f)] += 1;
    }
    FAMILIES.iter().map(|f| (*f, weight[index_of(*f)])).collect()
}

/// **THE INSTINCT FOLD** — `I_genesis ∪ Σ (word × usage)`. Accrete the cited
/// usage of every minted word onto the genesis seed, per family. A word cited
/// many times (leaned on through crisis after crisis) weights more than one
/// coined once and never used again — usage is `max(1, |cited|)` so even a
/// freshly minted word registers. Exact-integer throughout.
pub fn instinct_weights(library: &Library, genesis: &[(Family, u64)]) -> Vec<(Family, u64)> {
    let mut weight = [0u64; 4];
    for (family, base) in genesis {
        let i = index_of(*family);
        weight[i] = weight[i].saturating_add(*base);
    }
    for word in library.words() {
        // Usage frequency: how battle-tested the word is (its confirmations).
        let usage = (word.cited.len() as u64).max(1);
        for op in &word.expansion {
            let i = index_of(family_of(op));
            weight[i] = weight[i].saturating_add(usage);
        }
    }
    FAMILIES.iter().map(|f| (*f, weight[index_of(*f)])).collect()
}

/// **THE WORKBENCH CHOOSER** — a chronicle-seeded proportional draw over the
/// instinct weights: the family the Mind reaches for first this beat, sampled
/// from the distribution its whole history has shaped. Heavier families are
/// likelier; the same seed always draws the same family (replayable, divergent
/// per line). `None` only when every weight is zero (never, once the genesis
/// seed is present). The seed is the line's latest `chronicle_digest`, so the
/// draw carries the line's trauma and triumph — two lines with the SAME
/// vocabulary facing the SAME deficit still guess differently if their
/// histories differ (the digest is a hash chain of their exact past).
pub fn choose(weights: &[(Family, u64)], seed: [u8; 32]) -> Option<Family> {
    let total: u64 = weights.iter().fold(0u64, |sum, (_, w)| sum.saturating_add(*w));
    if total == 0 {
        return None;
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"instinct");
    let block = *hasher.finalize().as_bytes();
    let roll = u64::from_le_bytes(block[0..8].try_into().expect("32 >= 8")) % total;
    let mut acc = 0u64;
    for (family, w) in weights {
        acc = acc.saturating_add(*w);
        if roll < acc {
            return Some(*family);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mint::MintedWord;
    use solver_verify::grid::Op;

    fn word(ops: Vec<Op>, confirms: usize) -> MintedWord {
        // A distinct orbit per word so the library adopts each.
        let orbit = *blake3::hash(&[ops.len() as u8, confirms as u8]).as_bytes();
        MintedWord { orbit, expansion: ops, cited: vec![[0u8; 32]; confirms.max(1)] }
    }

    fn weight_of(weights: &[(Family, u64)], family: Family) -> u64 {
        weights.iter().find(|(f, _)| *f == family).map(|(_, w)| *w).unwrap_or(0)
    }

    #[test]
    fn the_genesis_seed_leans_to_disposition_but_leaves_nothing_unthinkable() {
        let emp = genesis_instinct("empiricist");
        assert!(weight_of(&emp, Family::Tile) > weight_of(&emp, Family::Symmetry), "empiricist leans to structure");
        assert!(emp.iter().all(|(_, w)| *w >= 1), "nothing is unthinkable at genesis");
        let exp = genesis_instinct("expansionist");
        assert!(weight_of(&exp, Family::Symmetry) > weight_of(&exp, Family::Tile), "expansionist leans to correspondence");
        // An unnamed disposition keeps the flat brainstem baseline.
        assert!(genesis_instinct("stoic").iter().all(|(_, w)| *w == 1), "no lean, flat baseline");
    }

    #[test]
    fn history_carves_the_rut_a_recolor_heavy_past_biases_the_future() {
        let mut library = Library::new();
        // A line that survived by minting RECOLOR geometry, over and over.
        library.adopt(word(vec![Op::Recolor([1u8; 10])], 5));
        library.adopt(word(vec![Op::Recolor([2u8; 10])], 8));
        let w = instinct_weights(&library, &genesis_instinct("stoic"));
        assert!(
            weight_of(&w, Family::Recolor) > weight_of(&w, Family::Symmetry),
            "a recolor-heavy history weights recolor far above the untouched families"
        );
        // The Workbench, seeded from the digest, reaches for the rutted family:
        // recolor holds the overwhelming share of the weight, so across a spread
        // of digests it is chosen far more than any other family.
        let recolor = (0..60u8)
            .filter(|i| choose(&w, *blake3::hash(&[*i]).as_bytes()) == Some(Family::Recolor))
            .count();
        let other = (0..60u8)
            .filter(|i| !matches!(choose(&w, *blake3::hash(&[*i]).as_bytes()), Some(Family::Recolor) | None))
            .count();
        assert!(recolor > other * 3, "the rut steers the guess ({recolor} recolor vs {other} other)");
    }

    #[test]
    fn usage_frequency_weights_a_battle_tested_word_above_a_novelty() {
        // Two words, same family, different confirmation counts.
        let mut leaned_on = Library::new();
        leaned_on.adopt(word(vec![Op::Recolor([1u8; 10])], 10));
        let mut coined_once = Library::new();
        coined_once.adopt(word(vec![Op::Recolor([1u8; 10])], 1));
        let flat = genesis_instinct("stoic");
        assert!(
            weight_of(&instinct_weights(&leaned_on, &flat), Family::Recolor)
                > weight_of(&instinct_weights(&coined_once, &flat), Family::Recolor),
            "a word leaned on through many crises weighs more than one coined once"
        );
    }

    #[test]
    fn late_game_accretion_drowns_the_genesis_seed() {
        // A large minted history swamps the ±1 disposition prior — mature
        // behaviour is the line's inventions, not its birth disposition.
        let mut library = Library::new();
        for k in 0..12u8 {
            library.adopt(MintedWord {
                orbit: *blake3::hash(&[k]).as_bytes(),
                expansion: vec![Op::Recolor([k; 10])],
                cited: vec![[0u8; 32]; 4],
            });
        }
        // Seed as an EMPIRICIST (leans toward Tile/Object structure, not Recolor).
        let w = instinct_weights(&library, &genesis_instinct("empiricist"));
        assert!(
            weight_of(&w, Family::Recolor) > weight_of(&w, Family::Tile),
            "the invented Recolor rut overwhelms the empiricist birth lean toward structure"
        );
    }

    #[test]
    fn the_draw_is_replayable_same_digest_same_guess() {
        let w = instinct_weights(&Library::new(), &genesis_instinct("empiricist"));
        let seed = *blake3::hash(b"a-line-history").as_bytes();
        assert_eq!(choose(&w, seed), choose(&w, seed), "same digest → same guess");
    }
}
