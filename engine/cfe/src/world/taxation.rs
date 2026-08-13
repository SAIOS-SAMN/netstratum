//! **THE CRAFTABLE TAX SYSTEM** (`45_TAXATION`, task #54/#55). A levy is a
//! tax as data — what it draws, when, where it flows (`dop::Levy`). This
//! module holds the readings the levies settle from; the continuity tax
//! (§3) is the first concrete instance, a demurrage on KNOWLEDGE keyed to
//! the lifespan turnover.

use super::*;
use crate::dop::{Levy, Ration, ZoneKind};

impl World {
    /// Does an ACTIVE civic zone of `kind` stand on this orb?
    fn has_civic(&self, orb: u64, kind: ZoneKind) -> bool {
        self.orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .any(|zone| zone.kind == kind && held.zone_active(zone))
            })
            .unwrap_or(false)
    }

    /// **DOES A GOVERNMENT OFFICE STAND?** (`45_TAXATION` §2) — the levy
    /// collection node. Where one stands, crafted levies collect without
    /// slip; the treasury/collection accounting rides this in the next
    /// fiscal layer.
    pub fn has_government_office(&self, orb: u64) -> bool {
        self.has_civic(orb, ZoneKind::GovernmentOffice)
    }

    /// **DOES A WELFARE HUB STAND?** (`45_TAXATION` §2) — the public-service
    /// distribution node that feeds the welfare loop.
    pub fn has_welfare_hub(&self, orb: u64) -> bool {
        self.has_civic(orb, ZoneKind::WelfareHub)
    }

    /// **THE WELFARE SUBSIDY** (`45_TAXATION` §3) — how much a WelfareHub
    /// lifts stability by siphoning the public RESERVE into the population's
    /// pool. It fills the stability GAP (`1 − leisure_coverage`) in
    /// proportion to how well-stocked the reserve ration is against the
    /// caloric demand — a full granary lets welfare cover the whole stall, an
    /// empty one cannot. No hub → 0, so a world without one is byte-identical.
    /// (First-cut: the reserve path; the tax-revenue/Treasury path is the
    /// next fiscal layer.)
    pub fn welfare_subsidy(&self, orb: u64, stamp: u64, leisure_coverage: &Ratio) -> Ratio {
        let zero = ratio(0, 1);
        if !self.has_welfare_hub(orb) {
            return zero;
        }
        let Ration::Resource(ration) = self.ration_of(orb) else {
            return zero;
        };
        let gap = (ratio(1, 1) - leisure_coverage.clone()).max(zero.clone());
        if gap <= zero {
            return zero;
        }
        // How stocked the public reserve is against what the people eat.
        let demand = self.leisure_demand(orb, stamp); // same per-capita span
        if demand <= zero {
            return zero;
        }
        let reserve = self
            .orb_by_id(orb)
            .map(|held| held.banked_amount(ration))
            .unwrap_or_else(|| zero.clone());
        let reserve_coverage = (reserve / demand).min(ratio(1, 1));
        gap * reserve_coverage
    }

    /// **THE CONTINUITY TAX a line owes at each turnover** (`45_TAXATION`
    /// §3 / `42_LIFESPAN §3`) — the fraction of its unconfirmed knowledge
    /// (crystallized boons, adopted practices) that LAPSES when a generation
    /// turns over. Derived from lifespan (first-cut rate, flagged for
    /// tuning): a short-lived line (L = one grain) loses ALL of it each
    /// turnover and must re-earn what the last generation knew; a long-lived
    /// line (L = four grains) loses only a quarter — so the Custodian
    /// ACCUMULATES and the Locust CHURNS. `grain/lifespan`, capped at one;
    /// derived (grain and lifespan are structural, not dialed).
    pub fn continuity_tax(&self) -> Ratio {
        let grain = crate::almanac::REGROWTH_GRAIN as i64;
        let lifespan = self.lifespan_of().max(1) as i64;
        ratio(grain, lifespan).min(ratio(1, 1))
    }

    /// The standing continuity levy — its base/trigger/sink (Knowledge /
    /// Turnover / Void). The `continuity_tax` is its measured rate.
    pub fn continuity_levy(&self) -> Levy {
        Levy::CONTINUITY
    }

    /// The standing estate levy (`45_TAXATION` §4) — Estate / Turnover /
    /// Sovereign. Its rate lands with #56; declared here so the seeded tax
    /// system is legible from one place.
    pub fn estate_levy(&self) -> Levy {
        Levy::ESTATE
    }
}
