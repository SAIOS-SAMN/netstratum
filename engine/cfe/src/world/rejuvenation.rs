//! **PLANETARY REJUVENATION — the lazy READ** (task #28, `40_REJUVENATION`).
//!
//! A world's natural RATION — biomass for a biotic line, iron for a
//! mineral line, a minted element for a stone-eater (the G2 map,
//! [`crate::dop::metabolic_ration_of`]) — recovers toward a carrying
//! capacity `K` along the ruled S-curve
//! ([`crate::almanac::grained_logistic`]). Nothing here is hardcoded to
//! biomass: what a world regrows, how fast, and toward what ceiling all
//! FOLLOW the sitting line's own metabolism and clade, the same
//! species⊗planet⊗archetype composition every other flow reads
//! (`zone_yield`, `taxon_economy_of`). Deterministic physics get no
//! deed: the curve is recomputed from `(P0, K, r, Δt)` at need, exactly
//! as demurrage's decay is (`World::purse`) — a READ settles nothing
//! and writes nothing, and `Δt` runs from the orb's `last_touch`, the
//! same clock the demurrage organ already carries.
//!
//! This is the read-only half (a planet SCAN). The mutating half — the
//! settle at a value-moving touch, with its receipt digest — banks the
//! grown stock and advances `last_touch`, and lands with the wiring.

use super::*;
use crate::almanac::{carrying_capacity, grained_logistic, intrinsic_rate, region_of, FLATLANDER_CLADE, ThermalRegion};
use crate::dop::{metabolic_ration_of, Ration, ZoneKind};

impl World {
    /// The living ECOLOGICAL FOOTPRINT of an orb — its habitats and
    /// regenerators, the zones that hold and restore life. The count
    /// that scales carrying capacity.
    pub fn living_zones_of(&self, orb: u64) -> u64 {
        self.orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|zone| {
                        matches!(zone.kind, ZoneKind::Habitat | ZoneKind::Regenerator)
                    })
                    .count() as u64
            })
            .unwrap_or(0)
    }

    /// The thermal REGION this orb sits in, or `None` where it has no
    /// place at all (unsited, or no star) — a world with no place bears
    /// no natural stock, so regrowth reads a barren capacity there.
    pub fn orb_region(&self, orb: u64) -> Option<ThermalRegion> {
        self.orb_heat(orb).map(|t4| region_of(&t4))
    }

    /// The sitting CLADE — unsealed worlds default flatlander for play
    /// readability, the same default `taxon_economy_of` reads.
    pub fn rejuvenation_clade(&self) -> u16 {
        self.taxon
            .map(|(_, clade)| clade)
            .unwrap_or(u16::from(FLATLANDER_CLADE))
    }

    /// The RATION this orb's line farms and eats — a closed [`Resource`]
    /// carrier for the three compiled metabolisms, or the line's own
    /// minted element for a G2 element-eater. Reads the same genesis
    /// metabolism and vocabulary fold every other flow does
    /// (`World::metabolic_element`).
    ///
    /// [`Resource`]: crate::dop::Resource
    pub fn ration_of(&self, _orb: u64) -> Ration {
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        metabolic_ration_of(metabolism, self.metabolic_element())
    }

    /// **THE CARRYING CAPACITY of this orb's ration** — the ruled `K`,
    /// from the living footprint and the place (`ZoneKind` ⊗ star ⊗
    /// orbit, the last two folded into the thermal region) AS READ BY
    /// THE SITTING CLADE. An unsited orb, or one on a barren rock (for
    /// this clade), carries none.
    pub fn ration_capacity_of(&self, orb: u64) -> Ratio {
        match self.orb_region(orb) {
            Some(region) => {
                carrying_capacity(self.living_zones_of(orb), self.rejuvenation_clade(), region)
            }
            None => ratio(0, 1),
        }
    }

    /// **THE REGROWN STOCK — a SCAN, never a deed.** What this orb's
    /// banked ration has recovered to as of `stamp`, folding the ruled
    /// S-curve over the beats elapsed since its `last_touch`. Pure and
    /// replay-inert: looking at a planet is not a chain event, so it
    /// settles nothing and advances no clock — the grown value banks for
    /// real only at the next genuine touch (the demurrage precedent,
    /// `World::purse`).
    pub fn regrown_ration(&self, orb: u64, stamp: u64) -> Ratio {
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let ration = self.ration_of(orb);
        let p0 = held.ration_amount(&ration);
        let k = self.ration_capacity_of(orb);
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dt = stamp.saturating_sub(held.last_touch);
        grained_logistic(&p0, &k, &intrinsic_rate(metabolism, self.metabolic_element()), dt)
    }

    /// **THE SETTLE — what a real TOUCH banks.** The stock the natural
    /// ration has GROWN by since the last touch, ready to fold into the
    /// touch's own `YieldCollected` receipt (the `40_REJUVENATION` §2
    /// digest, carried implicitly in the collected amount rather than a
    /// separate deed — "deterministic physics get no deed of their own").
    /// `None` when nothing grew: a stock already at or above `K` (a
    /// well-farmed world), a barren place, or — for now — an
    /// element-eater, whose ration settles with the element economy
    /// (task #29). Only a Resource ration below its ceiling regrows here.
    ///
    /// Because the growth folds into `YieldCollected`, which the kernel
    /// already admits as a `+amount` yield mint and replay banks straight
    /// from the receipt, the whole settle is SEAL-B-safe with no new deed,
    /// no wire change, and no kernel arm.
    pub fn rejuvenation_settle(&self, orb: u64, stamp: u64) -> Option<(Resource, Ratio)> {
        let Ration::Resource(resource) = self.ration_of(orb) else {
            return None;
        };
        let held = self.orb_by_id(orb)?;
        let grown = self.regrown_ration(orb, stamp) - held.banked_amount(resource);
        (grown > ratio(0, 1)).then_some((resource, grown))
    }

    /// **THE QUARANTINE DECAY — what a sealed world LOSES at a touch**
    /// (task #30, ruled 2026-08-12). While a world is quarantined
    /// (bio-pressure past the threshold), its ration does not recover —
    /// it STARVES, decaying at the same intrinsic rate it would else
    /// grow: a derived flat `r·Δt`, the per-beat decay the ruling names,
    /// settled lazily at the touch exactly as the regrowth it replaces.
    /// Capped at the stock (never below zero). `None` when the world
    /// stands free, holds no ration, or is an element-eater (whose stock
    /// rides the element economy, #29).
    ///
    /// Folds into the touch as a resource DEBIT (an `UpkeepSettled`) — the
    /// mirror of the regrowth credit — so it too is SEAL-B-safe with no
    /// new deed, wire, or kernel arm.
    pub fn quarantine_decay(&self, orb: u64, stamp: u64) -> Option<(Resource, Ratio)> {
        if !self.world_quarantined() {
            return None;
        }
        let Ration::Resource(resource) = self.ration_of(orb) else {
            return None;
        };
        let held = self.orb_by_id(orb)?;
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dt = ratio(stamp.saturating_sub(held.last_touch) as i64, 1);
        let decay = (intrinsic_rate(metabolism, self.metabolic_element()) * dt)
            .min(held.banked_amount(resource));
        (decay > ratio(0, 1)).then_some((resource, decay))
    }

    /// The ration delta a collect TOUCH settles: a starving DEBIT while
    /// quarantined, else the recovery CREDIT. One or the other, never
    /// both — a sealed world does not recover.
    pub fn ration_touch_delta(&self, orb: u64, stamp: u64) -> Option<(Resource, Ratio)> {
        if self.world_quarantined() {
            self.quarantine_decay(orb, stamp)
                .map(|(resource, decay)| (resource, -decay))
        } else {
            self.rejuvenation_settle(orb, stamp)
        }
    }
}
