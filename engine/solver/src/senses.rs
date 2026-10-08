//! **THE SENSES** (docs/60 D1) — the channels a word acts in, and the ones a
//! mind perceives. A word's channels are read off its ops; a line's senses
//! are read off its register perception. A sense bounds DERIVATION, never
//! use: a mind withholds a program it could not have perceived, but a word
//! it learned is used whole.

use solver_verify::grid::Op;

/// The channels, as bits.
pub const SIGHT: u8 = 1;
pub const HUE: u8 = 2;
pub const RHYTHM: u8 = 4;
pub const TOUCH: u8 = 8;

/// A set of channels: what a word uses, or what a mind perceives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Senses(pub u8);

impl Senses {
    /// Every channel: the house's mind, and any perception the register
    /// names no profile for.
    pub const ALL: Senses = Senses(SIGHT | HUE | RHYTHM | TOUCH);

    /// THE PROFILE A REGISTER PERCEPTION NAMES (docs/60 §2). Each of the
    /// three planar-family perceptions is deaf to one channel; volumetric
    /// keeps its own seat and reads as every channel here.
    pub fn of_perception(perception: &str) -> Senses {
        match perception {
            "planar" => Senses(SIGHT | HUE | TOUCH),
            "harmonic" => Senses(SIGHT | HUE | RHYTHM),
            "umbral" => Senses(SIGHT | TOUCH | RHYTHM),
            _ => Senses::ALL,
        }
    }

    /// The channel one op acts in.
    pub fn of_op(op: &Op) -> Senses {
        Senses(match op {
            Op::Rotate90 | Op::Rotate180 | Op::Rotate270 | Op::FlipH | Op::FlipV | Op::Transpose => SIGHT,
            Op::Recolor(_) => HUE,
            Op::Tile { .. } => RHYTHM,
            Op::Object(_) => TOUCH,
        })
    }

    /// The channels a word uses: the union over its ops.
    pub fn of_word(word: &[Op]) -> Senses {
        Senses(word.iter().fold(0, |acc, op| acc | Senses::of_op(op).0))
    }

    /// Whether these senses perceive every channel `word` uses.
    pub fn covers(self, word: Senses) -> bool {
        word.0 & !self.0 == 0
    }

    /// How many of `word`'s channels these senses do not perceive.
    pub fn missing(self, word: Senses) -> u32 {
        (word.0 & !self.0).count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_planar_family_perception_is_deaf_to_one_channel() {
        let recolor = Senses::of_word(&[Op::Rotate90, Op::Recolor([0, 2, 1, 3, 4, 5, 6, 7, 8, 9])]);
        assert_eq!(recolor, Senses(SIGHT | HUE));
        assert!(Senses::of_perception("planar").covers(recolor));
        assert!(!Senses::of_perception("umbral").covers(recolor));
        assert_eq!(Senses::of_perception("umbral").missing(recolor), 1);
        for p in ["planar", "harmonic", "umbral"] {
            assert_eq!(Senses::of_perception(p).0.count_ones(), 3, "{p}");
        }
        assert_eq!(Senses::of_perception("volumetric"), Senses::ALL);
    }
}
