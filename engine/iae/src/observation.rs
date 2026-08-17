//! **OBSERVE** (N-I2, doc `53` D2) — one pair, cheap. It owns NO algebra but the
//! route match: the drain (`reduce_pair`) already says gauge / wall / carry, and
//! observe reads that verdict — a GenerationGap wall this vessel could not
//! account MINTS (and the caller adopts onto THIS vessel); everything else is
//! accounted or carried, never minted. Condensation is a wall named into the
//! grown basis, not N guesses. There is no proposer here.

use crate::mint::mint;
use crate::peel::FullMeasurement;
use crate::reduction::{reduce_pair, Crystal, Dir13Route, ReduceVerdict, Remainder};

/// What one observation of a pair did against the vessel's grown basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Observed {
    /// The pair reduced — identical, or through a deduced gauge, or drained by
    /// the basis. Nothing to learn; nothing is minted.
    Accounted,
    /// A GenerationGap wall this vessel could not account — the obstruction is
    /// minted, for the caller to adopt onto this vessel.
    Minted(Crystal),
    /// A remainder on the letters' own axes (a composition gap) or a frame shift
    /// — carried whole, NOT minted (generation feedstock is GenerationGap only).
    Carried,
}

/// Observe a pair against a grown basis: reduce, then route the verdict. The
/// wall mints; the gauge and the drained pair account; the composition/frame
/// remainder carries.
pub fn observe(a: &FullMeasurement, b: &FullMeasurement, grown: &[Crystal]) -> Observed {
    match reduce_pair(a, b, grown) {
        ReduceVerdict::Congruent { .. } | ReduceVerdict::CongruentThroughP { .. } => {
            Observed::Accounted
        }
        ReduceVerdict::Remainder {
            remainder: Remainder::ChainClass { chain },
            route: Dir13Route::GenerationGap,
        } => match mint(chain) {
            Ok(crystal) => Observed::Minted(crystal),
            Err(_) => Observed::Carried,
        },
        ReduceVerdict::Remainder { .. } => Observed::Carried,
    }
}

/// Adopt a crystal onto a vessel — IDEMPOTENT (the same obstruction twice is one
/// crystal; a wall already grown is not re-minted). The ONE home for a vessel's
/// grow: `cfe::ipsal::Entity::adopt` delegates here, and every vessel (a
/// `Vec<Crystal>` test double included) grows through it.
pub fn adopt(grown: &mut Vec<Crystal>, crystal: Crystal) {
    if !grown.contains(&crystal) {
        grown.push(crystal);
    }
}
