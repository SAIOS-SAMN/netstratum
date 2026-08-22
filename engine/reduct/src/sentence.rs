//! **THE COMPOSE MOVE** (AUTIC-2) — the sentence, the 3rd of the four reduce
//! moves (doc 53). `think` runs DERIVE + REDUCE; this is COMPOSE: a mind SPLICES
//! several of its words into one composed utterance (an `AuticSentence`), and a
//! hearer ASSEMBLES that utterance against its OWN vocabulary — accounting the
//! pieces it recognises and NAMING exactly what it does not (the `DemandUnmet`
//! remainder). Built on the same exact accounting the whole engine reduces with
//! (`drain_exact`); coordinates with the sodalitum/lith shape (shared v0.4
//! lineage) without importing it.
//!
//! A word is a `ChainDelta` (a crystallised class). A SENTENCE is several words
//! spliced by exact entry-sum into one class — a composed meaning, not a list.
//! Hearing is comprehension: subtract the crystals you know; residual to zero is
//! `Assembled` (understood), a leftover is `DemandUnmet` (precisely what you must
//! still learn — mint feedstock).

use crate::congruence::{signed_entry_sum, ChainDelta};
use crate::reduction::{drain_exact, BoundingRecord, Crystal};

/// A composed utterance: several words spoken as one meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuticSentence {
    pub words: Vec<ChainDelta>,
}

impl AuticSentence {
    /// Speak a sentence from words (each a crystallised class).
    pub fn speak(words: Vec<ChainDelta>) -> Self {
        Self { words }
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// SPLICE — the words fused into one composed class by exact signed entry-sum.
    /// The frame endpoints ride from the first word (accounting keys on
    /// `(site,direction)`, never the frame, so the endpoints are carried metadata).
    /// An empty sentence composes to an empty class on a degenerate frame.
    pub fn composed(&self) -> ChainDelta {
        match self.words.first() {
            None => ChainDelta {
                from: crate::peel::CellFrame { rows: 0, cols: 0 },
                to: crate::peel::CellFrame { rows: 0, cols: 0 },
                entries: Vec::new(),
            },
            Some(first) => {
                let terms: Vec<(&ChainDelta, i64)> = self.words.iter().map(|w| (w, 1)).collect();
                ChainDelta {
                    from: first.from,
                    to: first.to,
                    entries: signed_entry_sum(&terms),
                }
            }
        }
    }
}

/// The hearer's verdict on a sentence, read against its own basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyRead {
    /// Every piece of the composed sentence was accounted by known crystals.
    Assembled { accounted: Vec<BoundingRecord> },
    /// A leftover survived — the hearer does not yet hold this structure. The
    /// remainder is exactly what it must learn (mint feedstock).
    DemandUnmet {
        remainder: ChainDelta,
        accounted: Vec<BoundingRecord>,
    },
}

impl AssemblyRead {
    pub fn is_assembled(&self) -> bool {
        matches!(self, AssemblyRead::Assembled { .. })
    }
}

/// **ASSEMBLE** — hear a sentence: splice it, then account it exactly against the
/// hearer's `basis`. Residual to zero is `Assembled`; a surviving remainder is
/// `DemandUnmet`, naming precisely the structure the hearer lacked.
pub fn assemble(sentence: &AuticSentence, basis: &[Crystal]) -> AssemblyRead {
    let (remainder, accounted) = drain_exact(sentence.composed(), basis);
    if remainder.entries.is_empty() {
        AssemblyRead::Assembled { accounted }
    } else {
        AssemblyRead::DemandUnmet { remainder, accounted }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peel::{CellFrame, GaugeDirection, Site};
    use num_bigint::BigInt;

    fn word(site: (i64, i64), dir: u8, coeff: i64) -> ChainDelta {
        ChainDelta {
            from: CellFrame { rows: 1, cols: 1 },
            to: CellFrame { rows: 1, cols: 1 },
            entries: vec![crate::congruence::ChainEntry {
                site: Site { row: site.0, col: site.1 },
                direction: GaugeDirection { index: dir },
                coefficient: BigInt::from(coeff),
            }],
        }
    }

    #[test]
    fn a_sentence_the_hearer_knows_assembles_to_zero() {
        // Two distinct words spliced into a sentence.
        let w1 = word((0, 0), 1, 1);
        let w2 = word((0, 1), 2, 1);
        let sentence = AuticSentence::speak(vec![w1.clone(), w2.clone()]);
        assert_eq!(sentence.len(), 2);
        // A basis holding BOTH words accounts the whole sentence — Assembled.
        let basis = vec![Crystal::seal(w1.clone()), Crystal::seal(w2.clone())];
        assert!(assemble(&sentence, &basis).is_assembled());
    }

    #[test]
    fn an_unknown_word_survives_as_demand_unmet() {
        let w1 = word((0, 0), 1, 1);
        let w2 = word((0, 1), 2, 1);
        let sentence = AuticSentence::speak(vec![w1.clone(), w2.clone()]);
        // A basis holding only the FIRST word leaves the second as the demand.
        let basis = vec![Crystal::seal(w1.clone())];
        match assemble(&sentence, &basis) {
            AssemblyRead::DemandUnmet { remainder, .. } => {
                // The leftover is exactly w2's entry.
                assert_eq!(remainder.entries.len(), 1);
                assert_eq!(remainder.entries[0].direction.index, 2);
            }
            AssemblyRead::Assembled { .. } => panic!("w2 was unknown — must be unmet"),
        }
    }

    #[test]
    fn the_empty_basis_leaves_the_whole_sentence_unmet() {
        let sentence = AuticSentence::speak(vec![word((0, 0), 1, 1)]);
        assert!(!assemble(&sentence, &[]).is_assembled());
    }
}
