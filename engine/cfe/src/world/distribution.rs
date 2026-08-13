//! **THE PROVISIONING LAYER** (`43_DISTRIBUTION`, task #45+).
//!
//! The granary is the infrastructure that turns the #40–42 subsistence
//! ledger into an economy. It holds no stock of its own — the depot IS
//! the orb's banked ration (ruled 2026-08-13) — so this layer stores
//! nothing new and mints no deed: everything it adds (the stratified
//! demand, the leisure tier, the stability reading, the disposition
//! factors) is DERIVED at the collect touch, the `regrown_ration`
//! precedent. This module holds the readings the consumption cycle (L2+)
//! consults; L1 is the gate itself.

use super::*;
use crate::almanac::carrying_capacity;
use crate::dop::{metabolic_leisure, Ration, Resource, ZoneKind};

impl World {
    /// **DOES THIS ORB STAND A GRANARY?** The gate that flips the
    /// provisioned economy on. Where it answers false, the population
    /// runs the #40–42 subsistence model verbatim (harvest→stomach, no
    /// leisure, no stability, byte-identical replay); where true, the
    /// stratified consumption cycle and its disposition factors come into
    /// force. An ACTIVE granary only — one that cannot pay its own upkeep
    /// is dark, and a dark provisioning node provisions nothing.
    pub fn has_granary(&self, orb: u64) -> bool {
        self.granary_footprint(orb) > 0
    }

    /// The count of ACTIVE granaries standing on an orb — the provisioning
    /// footprint that lifts the POPULATION ceiling (§2), distinct from the
    /// ecological footprint (`living_zones_of`) that lifts the ration's.
    pub fn granary_footprint(&self, orb: u64) -> u64 {
        self.orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|zone| zone.kind == ZoneKind::Granary && held.zone_active(zone))
                    .count() as u64
            })
            .unwrap_or(0)
    }

    /// **THE POPULATION CEILING `K_pop`** (§2) — the ecological footprint
    /// PLUS the provisioning footprint, read by the sitting clade at its
    /// place. A granary is provisioning capacity: it lifts how many people
    /// a world can sustain (never the food's own regrowth ceiling, which
    /// stays ecological). With no granary the footprint is exactly
    /// `living_zones_of`, so the ceiling — and every #40–42 world — is
    /// byte-identical.
    pub fn population_ceiling(&self, orb: u64) -> Ratio {
        match self.orb_region(orb) {
            Some(region) => {
                let footprint = self.living_zones_of(orb) + self.granary_footprint(orb);
                carrying_capacity(footprint, self.rejuvenation_clade(), region)
            }
            None => ratio(0, 1),
        }
    }

    /// The per-clade LEISURE good this orb's line savours (§3), or `None`
    /// for a line with no closed ration to pair it with (an element-eater,
    /// whose higher tiers ride #29). Reads the same genesis metabolism the
    /// ration does.
    pub fn leisure_of(&self, orb: u64) -> Option<Resource> {
        match self.ration_of(orb) {
            Ration::Resource(_) => {
                let metabolism =
                    self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
                Some(metabolic_leisure(metabolism))
            }
            Ration::Element(_) => None,
        }
    }

    /// **STABILITY** in `[0,1]` (§4) — a DERIVED contentment reading (no
    /// stored field, no deed, the `regrown_ration` precedent). In the fed
    /// regime it is the LEISURE COVERAGE: the banked leisure good measured
    /// against what the standing population demands over the span, capped
    /// at one. Full leisure → full stability → full growth; none → zero →
    /// the population STALLS (births halt, only mortality runs). A world
    /// with no granary, or no leisure pairing, reads full stability — the
    /// leisure tier simply does not apply to it, so it never stalls on it.
    pub fn stability_of(&self, orb: u64, stamp: u64) -> Ratio {
        let full = ratio(1, 1);
        if !self.has_granary(orb) {
            return full;
        }
        let Some(leisure) = self.leisure_of(orb) else {
            return full;
        };
        let demand = self.leisure_demand(orb, stamp);
        if demand <= ratio(0, 1) {
            return full;
        }
        let banked = self
            .orb_by_id(orb)
            .map(|held| held.banked_amount(leisure))
            .unwrap_or_else(|| ratio(0, 1));
        (banked / demand).min(full)
    }

    /// The leisure the standing population demands over the span since the
    /// last touch — the same per-capita rate the caloric demand uses (R6
    /// half-harvest), now against the leisure good: a people wants as much
    /// recreation as it eats. Derived from the population loop's own rate,
    /// no new dial.
    pub fn leisure_demand(&self, orb: u64, stamp: u64) -> Ratio {
        let p = self.population_of(orb);
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let dt = stamp.saturating_sub(held.last_touch);
        if p == 0 || dt == 0 {
            return ratio(0, 1);
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let pair = ratio(1, 1) + ratio(1, 1);
        let per_capita = crate::dop::metabolic_harvest_of(metabolism, self.metabolic_element()).1 / pair;
        ratio(p as i64, 1) * per_capita * ratio(dt as i64, 1)
    }
}
