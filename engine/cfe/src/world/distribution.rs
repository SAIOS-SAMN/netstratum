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
    /// PLUS the provisioning footprint PLUS the evolutionary stage, read by
    /// the sitting clade at its place. A granary is provisioning capacity;
    /// an advanced STAGE (`44_LINEAGES` §2c) is earned capacity — an
    /// evolved civilization sustains more people per world. Never the food's
    /// own regrowth ceiling, which stays ecological. With no granary and no
    /// advancement the footprint is exactly `living_zones_of`, so the
    /// ceiling — and every #40–42 world — is byte-identical.
    pub fn population_ceiling(&self, orb: u64) -> Ratio {
        match self.orb_region(orb) {
            Some(region) => {
                let footprint = self.living_zones_of(orb)
                    + self.granary_footprint(orb)
                    + self.evolution_stage() as u64;
                carrying_capacity(footprint, self.rejuvenation_clade(), region)
            }
            None => ratio(0, 1),
        }
    }

    /// **THE EVOLUTIONARY STAGE** (`44_LINEAGES` §2c) — how far this line
    /// has advanced along its arc. 0 at founding; raised only by the gated,
    /// earned `EvolutionAdvanced` deed.
    pub fn evolution_stage(&self) -> u8 {
        self.evolution_stage
    }

    /// **ADVANCE THE LINE ALONG ITS ARC** (`44_LINEAGES` §2c) — the
    /// transformation deed. A line does not sit at its founding coordinate:
    /// once it has CRYSTALLIZED the deepening (condition 3, the earned
    /// technology door the regenerator and refinery already pass), it may
    /// advance a stage, lifting its provisioning capacity. Gated (an
    /// un-deepened line is refused), receipted (replay-exact), and bounded
    /// by `u8`. Build scope. The arc is EARNED, never free.
    pub fn evolve(&mut self) -> Result<u8, ActionRefused> {
        self.evolve_as(SOVEREIGN)
    }

    pub fn evolve_as(&mut self, actor: u64) -> Result<u8, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        // The technology door: the deepening must stand crystallized on the
        // genesis orb — evolution is understood, not stumbled into.
        let deepened = self
            .techs
            .iter()
            .any(|(_, condition, _, _)| *condition == 3);
        if !deepened {
            return Err(ActionRefused::ConditionUncrystallized { condition: 3 });
        }
        let Some(stage) = self.evolution_stage.checked_add(1) else {
            return Err(ActionRefused::ConditionUncrystallized { condition: 3 });
        };
        self.close(ClosureEvent::EvolutionAdvanced { stage }, actor);
        self.evolution_stage = stage;
        Ok(stage)
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
    /// last touch — the base per-capita rate (R6 half-harvest, as the
    /// caloric demand) MODULATED by the disposition (§5): an expansionist
    /// wants none (military rations), an epicurean twice as much (the
    /// luxury appetite that pulls specialised labour), every other line the
    /// base. Derived from the population loop's own rate and the disposition
    /// factor (0 / 1 / 2, all built from identity ratios), no new dial.
    pub fn leisure_demand(&self, orb: u64, stamp: u64) -> Ratio {
        let zero = ratio(0, 1);
        let p = self.population_of(orb);
        let Some(held) = self.orb_by_id(orb) else {
            return zero;
        };
        let dt = stamp.saturating_sub(held.last_touch);
        if p == 0 || dt == 0 {
            return zero;
        }
        let factor = self.provision_leisure_factor();
        if factor <= zero {
            return zero;
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let pair = ratio(1, 1) + ratio(1, 1);
        let per_capita = crate::dop::metabolic_harvest_of(metabolism, self.metabolic_element()).1 / pair;
        ratio(p as i64, 1) * per_capita * ratio(dt as i64, 1) * factor
    }

    /// **THE DISPOSITION'S LEISURE APPETITE** (§5) — how much recreation a
    /// line demands, as a factor on the base per-capita rate. Keyed on the
    /// sealed/postured disposition (`civ_axes`, the same axis the
    /// vocabulary mints): expansionist 0 (lean — no luxury tier, grows on
    /// food alone), epicurean 2 (deep — the granary becomes an economic
    /// engine), every other line 1 (the base). Built from `0`, `1`, and
    /// `1+1` — a factor set, never a dial.
    pub fn provision_leisure_factor(&self) -> Ratio {
        match self.civ_axes().map(|axes| axes.disposition) {
            Some(1) => ratio(0, 1),               // expansionist — military rations
            Some(3) => ratio(1, 1) + ratio(1, 1), // epicurean — luxury appetite
            _ => ratio(1, 1),                     // empiricist / default — the base
        }
    }

    /// **THE EMPIRICIST RESEARCH SIPHON** (§5) — the share of would-be
    /// births an empiricist line spends on KNOWLEDGE instead of mouths.
    /// Half the surplus growth is redirected: the population grows slower,
    /// and the redirected count is the research this society mints from its
    /// provisioning margin (the metric here; the `DiscoveryMinted` deed is
    /// the follow-up). Zero for every other disposition — they spend the
    /// surplus on people. Derived (half = 1/(1+1)), no new dial.
    pub fn research_siphon(&self, births: i64) -> i64 {
        if births <= 0 {
            return 0;
        }
        match self.civ_axes().map(|axes| axes.disposition) {
            Some(2) => {
                let half = ratio(1, 1) / (ratio(1, 1) + ratio(1, 1));
                (ratio(births, 1) * half).floor().to_integer().try_into().unwrap_or(0)
            }
            _ => 0,
        }
    }
}
