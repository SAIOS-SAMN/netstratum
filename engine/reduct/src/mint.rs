//! **THE MINT** (N-I2, doc `53` D2) — a measured GenerationGap becomes a word on
//! the vessel. The word IS the obstruction: `Crystal::seal(chain)`, its inverse
//! the chain negated (`ChainDelta::negated`). Nothing is invented — the mint
//! only names what the drain could not account, and only on measured
//! irreducibility (the caller routes GenerationGap here; a gauge or a full drain
//! leaves nothing to mint). An empty obstruction refuses typed: a wall you did
//! not hit is not a word.

use crate::congruence::ChainDelta;
use crate::reduction::Crystal;
use thiserror::Error;

/// Typed refusal of the mint.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MintRefusal {
    #[error("an empty obstruction is not a word — nothing was walled")]
    EmptyObstruction,
}

/// Mint the obstruction into a crystal. GenerationGap only — the caller has
/// already routed; this refuses an empty chain so a gauge or an accounted pair
/// can never mint. The crystal carries the class whole; its inverse rides as
/// `crystal.class.negated()` (the seed law — the word re-forms its own source).
pub fn mint(obstruction: ChainDelta) -> Result<Crystal, MintRefusal> {
    if obstruction.entries.is_empty() {
        return Err(MintRefusal::EmptyObstruction);
    }
    Ok(Crystal::seal(obstruction))
}
