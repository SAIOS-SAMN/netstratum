//! P5-A KIN + defense posture stubs (36_DEFENSE_AXIOMS).
//!
//! Escort activation, typed convoy risk, interdiction residual.
//! Species-relative posture is a pure reading (no free combat HP).

use super::*;
use crate::dop::{HullRole, Resource, ZoneKind};

/// Six integrity domains (36 §1.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DefenseDomain {
    Cel = 1,
    Atm = 2,
    Cog = 3,
    Cyb = 4,
    Inf = 5,
    Kin = 6,
}

impl DefenseDomain {
    pub fn word(self) -> &'static str {
        match self {
            DefenseDomain::Cel => "CEL",
            DefenseDomain::Atm => "ATM",
            DefenseDomain::Cog => "COG",
            DefenseDomain::Cyb => "CYB",
            DefenseDomain::Inf => "INF",
            DefenseDomain::Kin => "KIN",
        }
    }

    pub fn gloss(self) -> &'static str {
        match self {
            DefenseDomain::Cel => "somatic / lattice / sheet",
            DefenseDomain::Atm => "medium envelope",
            DefenseDomain::Cog => "decision / wants",
            DefenseDomain::Cyb => "instrument + mesh",
            DefenseDomain::Inf => "OPSEC / veil",
            DefenseDomain::Kin => "hulls / convoy",
        }
    }
}

/// Risk band for convoy corridors (exact thresholds from 1+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskBand {
    Secure,
    Exposed,
    Contested,
}

impl RiskBand {
    pub fn word(self) -> &'static str {
        match self {
            RiskBand::Secure => "SECURE",
            RiskBand::Exposed => "EXPOSED",
            RiskBand::Contested => "CONTESTED",
        }
    }
}

/// Species-relative defense posture (pure reading).
#[derive(Debug, Clone)]
pub struct DefensePosture {
    pub primary: DefenseDomain,
    pub secondary: DefenseDomain,
    /// Rank 1..=3 per domain order CEL,ATM,COG,CYB,INF,KIN.
    pub ranks: [u8; 6],
    pub metabolism: u8,
    pub disposition: u8,
    pub perception: u8,
    pub voice: &'static str,
}

impl World {
    /// Pure posture from sealed genesis (36 §1.4–1.6). Unsealed → mineral defaults.
    pub fn defense_posture(&self) -> DefensePosture {
        let (met, dis, per) = self
            .genesis
            .as_ref()
            .map(|g| (g.metabolism, g.disposition, g.perception))
            .unwrap_or((1, 1, 1));
        // ranks: CEL ATM COG CYB INF KIN — 3 = emphasis
        let (primary, secondary, ranks, voice) = match met {
            // mineral — kinetic + somatic
            1 => (
                DefenseDomain::Kin,
                DefenseDomain::Cel,
                [3u8, 2, 1, 2, 1, 3],
                "mineral lattice — convoy & shell",
            ),
            // biotic — medium + somatic
            2 => (
                DefenseDomain::Atm,
                DefenseDomain::Cel,
                [3, 3, 2, 1, 2, 2],
                "biotic organ — medium & membrane",
            ),
            // plasmic — medium + cognitive/field
            3 => (
                DefenseDomain::Atm,
                DefenseDomain::Cog,
                [1, 3, 3, 2, 2, 1],
                "plasmic sheet — field cage & pattern",
            ),
            _ => (
                DefenseDomain::Kin,
                DefenseDomain::Cel,
                [2, 2, 2, 2, 2, 2],
                "unsealed — balanced stub",
            ),
        };
        // Disposition colour: expansionist boosts KIN; empiricist CYB/COG; epicurean ATM/CEL.
        let mut ranks = ranks;
        match dis {
            1 => {
                ranks[5] = ranks[5].saturating_add(1).min(3); // KIN
            }
            2 => {
                ranks[2] = ranks[2].saturating_add(1).min(3); // COG
                ranks[3] = ranks[3].saturating_add(1).min(3); // CYB
            }
            3 => {
                ranks[0] = ranks[0].saturating_add(1).min(3); // CEL
                ranks[1] = ranks[1].saturating_add(1).min(3); // ATM
            }
            _ => {}
        }
        let _ = per; // perception colours fog-of-war copy in UI only (P5-B)
        DefensePosture {
            primary,
            secondary,
            ranks,
            metabolism: met,
            disposition: dis,
            perception: per,
            voice,
        }
    }

    /// Escort coverage on a deal: sum (grade+1)/8 over escort hulls assigned, capped at 1.
    pub fn escort_cover(&self, deal: u64) -> Ratio {
        if deal == 0 {
            return ratio(0, 1);
        }
        let eighth = ZoneKind::Resonator.yield_per_tick().1; // 1/8
        let mut cover = ratio(0, 1);
        for h in &self.hulls {
            if h.assigned_route != deal {
                continue;
            }
            if HullRole::from_wire(h.role) != Some(HullRole::Escort) {
                continue;
            }
            let g = ratio(i64::from(h.grade.min(7)) + 1, 1);
            cover += g * eighth.clone();
        }
        let one = ratio(1, 1);
        if cover > one {
            one
        } else {
            cover
        }
    }

    /// Convoy risk in [0,1): base eighth without cover; reduced by escort_cover.
    /// `base = 1/8`; `risk = base * (1 - cover)`.
    pub fn convoy_risk(&self, deal: u64) -> Ratio {
        let eighth = ZoneKind::Resonator.yield_per_tick().1;
        let cover = self.escort_cover(deal);
        eighth * (ratio(1, 1) - cover)
    }

    pub fn convoy_risk_band(&self, deal: u64) -> RiskBand {
        let r = self.convoy_risk(deal);
        let two = ratio(1, 1) + ratio(1, 1);
        let four = two.clone() + two.clone();
        let quarter = ratio(1, 1) / four;
        let eighth = ZoneKind::Resonator.yield_per_tick().1;
        if r <= eighth / two {
            RiskBand::Secure
        } else if r < quarter {
            RiskBand::Exposed
        } else {
            RiskBand::Contested
        }
    }

    /// Assign an **Escort** hull to a corridor deal (KIN activation).
    pub fn escort_assign(&mut self, hull_id: u64, deal: u64) -> Result<(), ActionRefused> {
        self.escort_assign_as(SOVEREIGN, hull_id, deal)
    }

    pub fn escort_assign_as(
        &mut self,
        actor: u64,
        hull_id: u64,
        deal: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let hull = self
            .hulls
            .iter()
            .find(|h| h.hull_id == hull_id)
            .ok_or(ActionRefused::NoSuchHull { hull_id })?;
        let role = HullRole::from_wire(hull.role).unwrap_or(HullRole::Hauler);
        if role != HullRole::Escort {
            return Err(ActionRefused::EscortRoleRequired { hull_id });
        }
        let grade = hull.grade;
        self.assign_hull_as(actor, hull_id, deal)?;
        self.close(
            ClosureEvent::EscortOnDuty {
                hull_id,
                deal,
                grade,
            },
            actor,
        );
        Ok(())
    }

    /// Receipt a convoy risk reading for a deal (optional; pure derive also OK).
    pub fn mark_convoy_risk(&mut self, deal: u64) -> Result<Ratio, ActionRefused> {
        self.mark_convoy_risk_as(SOVEREIGN, deal)
    }

    pub fn mark_convoy_risk_as(
        &mut self,
        actor: u64,
        deal: u64,
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if deal == 0 {
            return Err(ActionRefused::RouteUnknown { deal: 0 });
        }
        let risk = self.convoy_risk(deal);
        let cover = self.escort_cover(deal);
        self.close(
            ClosureEvent::ConvoyRiskMarked {
                deal,
                risk: risk.clone(),
                escort_cover: cover,
            },
            actor,
        );
        Ok(risk)
    }

    /// Apply interdiction residual on a haul: lose `amount * risk / 2` cargo
    /// (typed receipt). Zero risk ⇒ Ok no-op.
    pub fn strike_interdiction(
        &mut self,
        actor: u64,
        deal: u64,
        orb: u64,
        resource: Resource,
        cargo: &Ratio,
    ) -> Result<Ratio, ActionRefused> {
        let risk = self.convoy_risk(deal);
        if risk == ratio(0, 1) || *cargo == ratio(0, 1) {
            return Ok(ratio(0, 1));
        }
        let two = ratio(1, 1) + ratio(1, 1);
        let lost = cargo.clone() * risk.clone() / two;
        if lost == ratio(0, 1) {
            return Ok(ratio(0, 1));
        }
        // Debit is applied by caller on the haul path; this only receipts.
        self.close(
            ClosureEvent::InterdictionStruck {
                deal,
                orb_id: orb,
                resource,
                amount: lost.clone(),
                risk,
            },
            actor,
        );
        Ok(lost)
    }
}

