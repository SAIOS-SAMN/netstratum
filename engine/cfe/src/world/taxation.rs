//! **THE CRAFTABLE TAX SYSTEM** (`49_TAXATION`, task #54/#55). A levy is a
//! tax as data — what it draws, when, where it flows (`dop::Levy`). This
//! module holds the readings the levies settle from; the continuity tax
//! (§3) is the first concrete instance, a demurrage on KNOWLEDGE keyed to
//! the lifespan turnover.

use super::*;
use crate::dop::{Levy, ZoneKind};

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

    /// **DOES A GOVERNMENT OFFICE STAND?** (`49_TAXATION` §2) — the levy
    /// collection node. Where one stands, crafted levies collect without
    /// slip; the treasury/collection accounting rides this in the next
    /// fiscal layer.
    pub fn has_government_office(&self, orb: u64) -> bool {
        self.has_civic(orb, ZoneKind::GovernmentOffice)
    }

    /// **DOES A WELFARE HUB STAND?** (`49_TAXATION` §2) — the public-service
    /// distribution node that feeds the welfare loop.
    pub fn has_welfare_hub(&self, orb: u64) -> bool {
        self.has_civic(orb, ZoneKind::WelfareHub)
    }

    /// **THE WELFARE BUDGET NEEDED** (`49_TAXATION` §5) — the fiscal cost
    /// to fully close the stability stall this cycle: the gap
    /// (`1 − leisure_coverage`) times the population's leisure demand.
    /// This is what welfare WOULD spend to erase the stall entirely; the
    /// treasury may or may not hold that much. Zero when there is no hub,
    /// no stall, or no demand.
    fn welfare_budget_needed(&self, orb: u64, stamp: u64) -> Ratio {
        let zero = ratio(0, 1);
        if !self.has_welfare_hub(orb) {
            return zero;
        }
        let Some(leisure_coverage) = self.leisure_coverage_of(orb, stamp) else {
            return zero;
        };
        let gap = (ratio(1, 1) - leisure_coverage).max(zero.clone());
        if gap <= zero {
            return zero;
        }
        let demand = self.leisure_demand(orb, stamp);
        if demand <= zero {
            return zero;
        }
        gap * demand
    }

    /// **THE WELFARE SPEND at a touch** (`49_TAXATION` §5) — what the
    /// WelfareHub actually draws from the sovereign TREASURY this cycle:
    /// the budget it needs to close the stall, capped by what the
    /// treasury holds. An empty treasury spends nothing; a flush one
    /// spends only what the stall costs. This is the fiscal OUTFLOW the
    /// collect touch settles (a negative `TreasurySettled`).
    pub fn welfare_spend(&self, orb: u64, stamp: u64) -> Ratio {
        self.treasury_of(orb).min(self.welfare_budget_needed(orb, stamp))
    }

    /// **THE WELFARE SUBSIDY** (`49_TAXATION` §5) — how much a WelfareHub
    /// lifts stability by spending the sovereign TREASURY into the
    /// population's pool. It fills the stability GAP in proportion to how
    /// far the treasury covers the fiscal cost of the stall — a flush
    /// treasury buys the whole gap, an empty one buys nothing. The tax
    /// loop closes here: levies fund the treasury (`levy_collected_at_touch`),
    /// the treasury funds welfare, welfare lifts stability. No hub or an
    /// empty treasury → 0, so a world without the loop is byte-identical.
    pub fn welfare_subsidy(&self, orb: u64, stamp: u64, leisure_coverage: &Ratio) -> Ratio {
        let zero = ratio(0, 1);
        let gap = (ratio(1, 1) - leisure_coverage.clone()).max(zero.clone());
        if gap <= zero {
            return zero;
        }
        let budget_needed = self.welfare_budget_needed(orb, stamp);
        if budget_needed <= zero {
            return zero;
        }
        // What fraction of the stall the treasury can actually fund.
        let treasury_coverage = (self.welfare_spend(orb, stamp) / budget_needed).min(ratio(1, 1));
        gap * treasury_coverage
    }

    /// **THE CONTINUITY TAX a line owes at each turnover** (`49_TAXATION`
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

    /// **THE SOVEREIGN TREASURY BALANCE** (`49_TAXATION` §5) — the
    /// abstract fiscal reserve standing on this orb: everything crafted
    /// levies have collected, net of everything welfare has spent. A
    /// world that never collected a levy reads zero. Folded from the
    /// `TreasurySettled` receipts (floored at zero, so it never goes
    /// negative).
    pub fn treasury_of(&self, orb: u64) -> Ratio {
        self.treasury_balances
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, bal)| bal.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// **SETTLE THE TREASURY** (`49_TAXATION` §5, tag 171) — apply a NET
    /// fiscal change to the sovereign treasury (levies collected in,
    /// welfare spent out), floored at zero, and receipt it so replay
    /// applies the same delta. A zero delta emits nothing, so a world
    /// that neither taxes nor spends is byte-identical.
    pub fn settle_treasury(&mut self, orb: u64, delta: Ratio) {
        let zero = ratio(0, 1);
        if delta == zero {
            return;
        }
        let next = (self.treasury_of(orb) + delta.clone()).max(zero.clone());
        self.treasury_balances.retain(|(o, _)| *o != orb);
        if next > zero {
            self.treasury_balances.push((orb, next));
        }
        let stamp = self.clock.now();
        self.chronicle.append(
            crate::dop::ClosureEvent::TreasurySettled { orb_id: orb, delta },
            stamp,
            vec![],
        );
    }

    /// **GOVERNANCE REACH** (`49_TAXATION` §5) — how far the sovereign's
    /// fiscal apparatus extends over its own economy: the share of the
    /// orb's ACTIVE zones that are GovernmentOffices. A line with no
    /// office reaches nothing (0 → byte-identical); one whose civic
    /// build keeps pace with its economy reaches further, up to the
    /// whole of it. Derived from structural state (the office/zone
    /// ratio), never a dialed rate — the moving-footprint precedent. The
    /// levy collects proportionally, so the "slip" (evasion) is exactly
    /// what governance does not reach.
    pub fn governance_reach(&self, orb: u64) -> Ratio {
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let active = held.zones.iter().filter(|zone| held.zone_active(zone));
        let (offices, total) = active.fold((0i64, 0i64), |(offices, total), zone| {
            let is_office = zone.kind == ZoneKind::GovernmentOffice;
            (offices + is_office as i64, total + 1)
        });
        if total == 0 {
            return ratio(0, 1);
        }
        ratio(offices, total)
    }

    /// **THE LEVY COLLECTED AT A TOUCH** (`49_TAXATION` §5) — the fiscal
    /// revenue a Touch-triggered levy draws into the treasury: the
    /// governance reach applied to the `taxable` throughput (the net
    /// yield banked this touch, the Trade base). The treasury is an
    /// abstract fiscal reserve — the yield still banks in full; this
    /// meters a governance shadow of it, so nothing is destroyed and
    /// conservation is untouched. No office → zero reach → zero revenue,
    /// so a world without a GovernmentOffice collects exactly as before.
    pub fn levy_collected_at_touch(&self, orb: u64, taxable: &Ratio) -> Ratio {
        let zero = ratio(0, 1);
        if *taxable <= zero {
            return zero;
        }
        self.governance_reach(orb) * taxable.clone()
    }

    /// **THE CONTINUITY LAPSE at a turnover** (`49_TAXATION` §3) — how
    /// many of the line's adopted practices lapse to the void for the
    /// `boundaries` generation turnovers crossed this span. Per boundary
    /// `floor(continuity_tax × standing)` lapse, applied in turn: a
    /// short-lived Locust (rate 1) forgets ALL of them at the first
    /// turnover and must re-adopt; a long-lived Custodian (rate 1/4) keeps
    /// most, its law compounding. Zero unless a lifespan class is sealed
    /// AND practices stand — a line with neither is byte-identical to the
    /// pre-taxation era. The lapse is LIFO (the newest, least-confirmed
    /// knowledge first); the `PracticesLapsed` receipt carries the count.
    pub fn continuity_lapse_at_touch(&self, boundaries: u64) -> u8 {
        if boundaries == 0 || self.taxon().is_none() {
            return 0;
        }
        let start = self.practices.len() as i64;
        if start == 0 {
            return 0;
        }
        let rate = self.continuity_tax();
        let mut remaining = start;
        for _ in 0..boundaries {
            let lapse: i64 = (rate.clone() * ratio(remaining, 1))
                .floor()
                .to_integer()
                .try_into()
                .unwrap_or(0);
            if lapse <= 0 {
                break; // a long-lived line keeps its last few — settled
            }
            remaining -= lapse;
            if remaining <= 0 {
                remaining = 0;
                break;
            }
        }
        (start - remaining) as u8
    }

    /// **LAPSE THE PRACTICES** (`49_TAXATION` §3, tag 172) — drop `count`
    /// practices from the tail (LIFO) and receipt it so replay drops the
    /// same. A zero count emits nothing, so a line that keeps all its
    /// knowledge is byte-identical.
    pub fn lapse_practices(&mut self, count: u8) {
        if count == 0 {
            return;
        }
        let keep = self.practices.len().saturating_sub(count as usize);
        self.practices.truncate(keep);
        let stamp = self.clock.now();
        self.chronicle.append(
            crate::dop::ClosureEvent::PracticesLapsed { count },
            stamp,
            vec![],
        );
    }

    /// The standing continuity levy — its base/trigger/sink (Knowledge /
    /// Turnover / Void). `continuity_tax` is its rate,
    /// `continuity_lapse_at_touch` its settle.
    pub fn continuity_levy(&self) -> Levy {
        Levy::CONTINUITY
    }

    /// **THE ESTATE TAX RATE** (`49_TAXATION` §4) — the share of a
    /// generation's estate taxed to the sovereign at each turnover. It
    /// shares the lifespan-derived TURNOVER rate with the continuity tax
    /// (`grain/lifespan`): a short-lived line turns over often and is
    /// taxed near its whole per-capita estate each time; a long-lived one
    /// rarely, and only a quarter. One clock, one rate — the two levies
    /// that fire at Turnover read it alike.
    pub fn estate_tax(&self) -> Ratio {
        self.continuity_tax()
    }

    /// **THE ESTATE TAX DUE at a turnover** (`49_TAXATION` §4) — what the
    /// estate levy collects into the treasury for the generation
    /// boundaries crossed this span. The estate is population-proportional
    /// (ruled 2026-08-13): `population × per_capita_ration`, the
    /// generation's worth. Collection needs a GovernmentOffice — no office
    /// → nothing collected, so a world without one is byte-identical.
    /// `boundaries` is how many full generations turned over this span
    /// (`now/L − last_touch/L`), so a touch that spans several settles
    /// them all at once.
    pub fn estate_due_at_touch(&self, orb: u64, boundaries: u64) -> Ratio {
        let zero = ratio(0, 1);
        if boundaries == 0 || !self.has_government_office(orb) {
            return zero;
        }
        let population = self.population_of(orb);
        if population == 0 {
            return zero;
        }
        let estate = ratio(population as i64, 1) * self.per_capita_ration();
        self.estate_tax() * estate * ratio(boundaries as i64, 1)
    }

    /// The standing estate levy (`49_TAXATION` §4) — Estate / Turnover /
    /// Sovereign. `estate_tax` is its rate, `estate_due_at_touch` its
    /// settle; declared here so the seeded tax system is legible from one
    /// place.
    pub fn estate_levy(&self) -> Levy {
        Levy::ESTATE
    }
}
