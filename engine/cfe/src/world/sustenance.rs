//! **The sustenance sources** (SOURCE-1, operator ruling 2026-08-17).
//!
//! ONT-2b built the right behavior (a population eats the wild flow, then draws
//! on the stock its metabolism grows — a Resource bank, or its mineral vein)
//! through the wrong shape: it branched on the carrier type
//! (`match ration { Resource => bank-debit, Element => mine-the-vein }`) in the
//! `available` read and the collect draw — the enumeration/pigeonhole the ONT
//! arc exists to eliminate. This module dissolves that fork.
//!
//! A [`FoodSource`] is a place a line draws sustenance from this span — a wild
//! flow, a banked Resource stock, or a mineral vein — each reporting what it
//! can yield and (at the draw site) how it depletes. The species → source
//! mapping lives once, in [`World::sustenance_sources_with`], derived from the
//! morphodynamic signature (`ration_of` already projects the element staple;
//! the `Resource` arm is the legacy carrier PEEL-1/ONT-1 dissolve). Every
//! consumer — the population's `available` read, the collect draw — iterates the
//! vector uniformly: there is no `match ration` downstream. A flux source for
//! energy-eaters (#29) and multi-source diets slot in here with no new branch.

use super::*;
use crate::dop::{Ration, Resource};

/// One place a line draws food from this span. The generalization that replaces
/// the `Ration::{Resource,Element}` control-flow fork.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FoodSource {
    /// The wild syntropic flow (`syntropic_flow · dt`) — free, gathered first,
    /// never banked or depleted (physics refills it to this floor). Eaten
    /// before any stock, so only demand beyond it draws on a larder.
    Wild(Ratio),
    /// A banked resource stock available this span — the standing bank plus the
    /// tick's harvest (accrued yield + natural regrowth). A draw debits the bank
    /// (`YieldCollected`/`UpkeepSettled`), clamped to what stands. The legacy
    /// carrier the three compiled metabolisms farm (#40–42).
    Bank { resource: Resource, available: Ratio },
    /// A mineral vein — the larder. Its yield is the unextracted richness
    /// (`cultivated_yield` = Σ vein_richness); a draw mines it (`extracted +=
    /// eaten`, `StapleMined`), capped at that richness, the bank bypassed (food
    /// goes vein → mouths). The almanac-element staple (ONT-2b).
    Vein { element: [u8; 32], richness: Ratio },
}

impl FoodSource {
    /// What this source can yield this span — the quantity the population's
    /// `available` read sums over, and the ceiling a draw against it is capped
    /// to.
    pub fn available(&self) -> Ratio {
        match self {
            FoodSource::Wild(amount) => amount.clone(),
            FoodSource::Bank { available, .. } => available.clone(),
            FoodSource::Vein { richness, .. } => richness.clone(),
        }
    }

    /// The wild flow is gathered free and eaten first; every other source is a
    /// larder a draw depletes. This is the one distinction the consumption loop
    /// reads — never the carrier type.
    pub fn is_wild(&self) -> bool {
        matches!(self, FoodSource::Wild(_))
    }
}

impl World {
    /// **Derive A line's sustenance sources** (SOURCE-1) — the ranked places it
    /// eats from this span, and the one point the species → food mapping lives.
    /// Every line gathers the wild flow first (eaten free); then it draws on the
    /// stock its metabolism grows — a Resource bank for the compiled carriers,
    /// or its almanac-element vein (the larder) for an element-eater. Callers
    /// iterate the result uniformly; there is no `match ration` in the
    /// consumption path.
    pub fn sustenance_sources(&self, orb: u64, stamp: u64) -> Vec<FoodSource> {
        self.sustenance_sources_with(orb, stamp, &self.ration_of(orb))
    }

    /// `sustenance_sources` on a PRE-resolved ration (the collect-touch memo,
    /// [`Self::population_touch_delta_with`]).
    pub fn sustenance_sources_with(
        &self,
        orb: u64,
        stamp: u64,
        ration: &Ration,
    ) -> Vec<FoodSource> {
        let Some(held) = self.orb_by_id(orb) else {
            return Vec::new();
        };
        let dt = stamp.saturating_sub(held.last_touch);
        let dtr = ratio(dt as i64, 1);
        let mut sources = Vec::with_capacity(2);
        // The wild flow — every line gathers it, eaten first, never depleted.
        // Pushed unconditionally (even at zero, on a barren world) so the summed
        // `available` matches the pre-SOURCE-1 formula bit-for-bit.
        sources.push(FoodSource::Wild(self.syntropic_flow(orb) * dtr));
        // The cultivated stock, derived from the line's metabolism.
        match ration {
            Ration::Resource(resource) => {
                // The standing bank plus this span's harvest (the tick-accrued
                // yield of this carrier + the natural regrowth/decay settling at
                // this same touch) — the #40–42 belief the band reads.
                let harvested: Ratio = self
                    .accrued_of(orb)
                    .into_iter()
                    .filter(|(res, _)| res == resource)
                    .map(|(_, amount)| amount)
                    .fold(ratio(0, 1), |sum, amount| sum + amount);
                let natural = match self.ration_touch_delta_with(orb, stamp, ration.clone()) {
                    Some((r, signed)) if &r == ration => signed,
                    _ => ratio(0, 1),
                };
                let available = held.ration_amount(ration) + harvested + natural;
                sources.push(FoodSource::Bank { resource: *resource, available });
            }
            Ration::Element(element) => {
                // The vein is the larder — the unextracted richness (no bank, no
                // regrowth until ONT-3): the band grows into the full vein read
                // here, then mines it down by eating.
                sources.push(FoodSource::Vein {
                    element: *element,
                    richness: self.cultivated_yield(orb),
                });
            }
        }
        sources
    }
}
