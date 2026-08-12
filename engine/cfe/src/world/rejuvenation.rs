//! **PLANETARY REJUVENATION — the lazy READ** (task #28, `40_REJUVENATION`).
//!
//! Natural biomass recovers toward a carrying capacity `K` along the
//! ruled S-curve ([`crate::almanac::grained_logistic`]). Deterministic
//! physics get no deed: the curve is recomputed from `(P0, K, r, Δt)`
//! at need, exactly as demurrage's decay is (`World::purse`) — a READ
//! settles nothing and writes nothing, and `Δt` runs from the orb's
//! `last_touch`, the same clock the demurrage organ already carries.
//!
//! This is the read-only half (a planet SCAN). The mutating half — the
//! settle at a value-moving touch, with its receipt digest — banks the
//! grown stock and advances `last_touch`, and lands with the wiring.

use super::*;
use crate::almanac::{carrying_capacity, grained_logistic, intrinsic_rate, region_of, ThermalRegion};
use crate::dop::{Resource, ZoneKind};

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
    /// no natural life, so regrowth reads a barren capacity there.
    pub fn orb_region(&self, orb: u64) -> Option<ThermalRegion> {
        self.orb_heat(orb).map(|t4| region_of(&t4))
    }

    /// **THE CARRYING CAPACITY of this orb's biomass** — the ruled `K`,
    /// from the living footprint and the place (`ZoneKind` ⊗ star ⊗
    /// orbit, the last two folded into the thermal region). An unsited
    /// orb, or one on a barren rock, carries none.
    pub fn biomass_capacity_of(&self, orb: u64) -> Ratio {
        match self.orb_region(orb) {
            Some(region) => carrying_capacity(self.living_zones_of(orb), region),
            None => ratio(0, 1),
        }
    }

    /// **THE REGROWN STOCK — a SCAN, never a deed.** What this orb's
    /// banked biomass has recovered to as of `stamp`, folding the ruled
    /// S-curve over the beats elapsed since its `last_touch`. Pure and
    /// replay-inert: looking at a planet is not a chain event, so it
    /// settles nothing and advances no clock — the grown value banks for
    /// real only at the next genuine touch (the demurrage precedent,
    /// `World::purse`).
    pub fn regrown_biomass(&self, orb: u64, stamp: u64) -> Ratio {
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let p0 = held.banked_amount(Resource::Biomass);
        let k = self.biomass_capacity_of(orb);
        let dt = stamp.saturating_sub(held.last_touch);
        grained_logistic(&p0, &k, &intrinsic_rate(), dt)
    }
}
