//! P5-A KIN + P5-B species-relative posture (36_DEFENSE_AXIOMS).
//!
//! Escort activation, typed convoy risk, interdiction residual.
//! Posture: anatomy from genesis metabolism (immutable) + taxon clade
//! colour + reconfigurable doctrine (disposition/perception/primary).

use super::*;
use crate::dop::{HullRole, Resource, ZoneKind};

/// Seven integrity domains (36 §1.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DefenseDomain {
    Cel = 1,
    Atm = 2,
    Cog = 3,
    Cyb = 4,
    Inf = 5,
    Kin = 6,
    Mkt = 7,
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
            DefenseDomain::Mkt => "MKT",
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
            DefenseDomain::Mkt => "books / clears / STAMP",
        }
    }

    pub fn from_wire(code: u8) -> Option<DefenseDomain> {
        match code {
            1 => Some(DefenseDomain::Cel),
            2 => Some(DefenseDomain::Atm),
            3 => Some(DefenseDomain::Cog),
            4 => Some(DefenseDomain::Cyb),
            5 => Some(DefenseDomain::Inf),
            6 => Some(DefenseDomain::Kin),
            7 => Some(DefenseDomain::Mkt),
            _ => None,
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

/// Species-relative defense posture (pure reading; doctrine may reorient).
#[derive(Debug, Clone)]
pub struct DefensePosture {
    pub primary: DefenseDomain,
    pub secondary: DefenseDomain,
    /// Rank 1..=3 per domain order CEL,ATM,COG,CYB,INF,KIN,MKT.
    pub ranks: [u8; 7],
    /// Sealed anatomy (genesis metabolism) — never reoriented.
    pub metabolism: u8,
    /// Active doctrine disposition (genesis or reoriented).
    pub disposition: u8,
    /// Active perception stance (genesis or reoriented).
    pub perception: u8,
    pub class: Option<u8>,
    pub clade: Option<u8>,
    /// True when doctrine differs from sealed genesis axes.
    pub reoriented: bool,
    pub voice: String,
}

impl World {
    /// Pure posture from sealed genesis + taxon + optional doctrine overlay (36 §1.4–1.7).
    pub fn defense_posture(&self) -> DefensePosture {
        let (met, gen_dis, gen_per) = self
            .genesis
            .as_ref()
            .map(|g| (g.metabolism, g.disposition, g.perception))
            .unwrap_or((1, 1, 1));
        let dis = self.posture_disposition.unwrap_or(gen_dis);
        let per = self.posture_perception.unwrap_or(gen_per);
        let reoriented = self.posture_disposition.is_some()
            || self.posture_perception.is_some()
            || self.posture_primary.is_some();
        let (class, clade) = match self.taxon {
            Some((c, l)) => (Some(c), Some(l)),
            None => (None, None),
        };

        // ranks: CEL ATM COG CYB INF KIN MKT — 3 = emphasis
        let (mut primary, mut secondary, mut ranks, met_voice) = match met {
            1 => (
                DefenseDomain::Kin,
                DefenseDomain::Mkt,
                [3u8, 2, 1, 2, 1, 3, 3],
                "mineral lattice",
            ),
            2 => (
                DefenseDomain::Atm,
                DefenseDomain::Mkt,
                [3, 3, 2, 1, 2, 2, 2],
                "biotic organ",
            ),
            3 => (
                DefenseDomain::Atm,
                DefenseDomain::Cog,
                [1, 3, 3, 2, 2, 1, 3],
                "plasmic sheet",
            ),
            _ => (
                DefenseDomain::Kin,
                DefenseDomain::Cel,
                [2, 2, 2, 2, 2, 2, 2],
                "unsealed",
            ),
        };

        // Disposition colour (36 §1.6) — active doctrine.
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
                ranks[6] = ranks[6].saturating_add(1).min(3); // MKT
            }
            _ => {}
        }

        // Perception colour (36 §1.7).
        let per_voice = match per {
            1 => {
                ranks[4] = ranks[4].saturating_add(1).min(3); // INF planar charts
                "planar chart seals"
            }
            2 => {
                ranks[4] = ranks[4].saturating_add(1).min(3);
                ranks[1] = ranks[1].saturating_add(1).min(3); // volumetric medium
                "volumetric depth ward"
            }
            3 => {
                ranks[2] = ranks[2].saturating_add(1).min(3); // harmonic
                "harmonic filter"
            }
            4 => {
                ranks[4] = ranks[4].saturating_add(1).min(3);
                ranks[2] = ranks[2].saturating_add(1).min(3); // umbral
                "umbral occlusion"
            }
            _ => "open perception",
        };

        // Clade colour (36 §1.5) — weights only, no new domain.
        let clade_voice = match clade {
            Some(5) => {
                // tardigrade
                ranks[0] = ranks[0].saturating_add(1).min(3);
                ranks[1] = ranks[1].saturating_add(1).min(3);
                "cryptobiotic endurance"
            }
            Some(4) | Some(14) => {
                // lithic / endolithic
                ranks[0] = ranks[0].saturating_add(1).min(3);
                ranks[2] = ranks[2].saturating_add(1).min(3);
                "lattice memory"
            }
            Some(8) => {
                // hymenopteran
                ranks[2] = ranks[2].saturating_add(1).min(3);
                ranks[4] = ranks[4].saturating_add(1).min(3);
                "hive graph"
            }
            Some(6) => {
                // cetacean
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[2] = ranks[2].saturating_add(1).min(3);
                "song medium"
            }
            Some(12) | Some(13) => {
                // plasmoid / syzygial
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "field sense"
            }
            Some(10) => {
                // fungal
                ranks[0] = ranks[0].saturating_add(1).min(3);
                ranks[4] = ranks[4].saturating_add(1).min(3);
                "mycelial redundancy"
            }
            Some(3) => {
                // cryo-coherent
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[0] = ranks[0].saturating_add(1).min(3);
                "ice shell"
            }
            Some(19) => {
                // draconic
                ranks[2] = ranks[2].saturating_add(1).min(3);
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "coherence apex"
            }
            Some(20) => {
                // metashifting
                ranks[4] = ranks[4].saturating_add(1).min(3);
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "veil-native"
            }
            _ => "standard clade colour",
        };

        // Optional primary override from reorientation.
        if let Some(code) = self.posture_primary {
            if let Some(d) = DefenseDomain::from_wire(code) {
                if d != primary {
                    secondary = primary;
                    primary = d;
                }
            }
        }

        let dis_word = match dis {
            1 => "expansionist perimeter",
            2 => "empiricist proof line",
            3 => "epicurean surplus",
            _ => "doctrine open",
        };
        let voice = format!(
            "{met_voice} — {clade_voice}; {dis_word}; {per_voice}"
        );

        DefensePosture {
            primary,
            secondary,
            ranks,
            metabolism: met,
            disposition: dis,
            perception: per,
            class,
            clade,
            reoriented,
            voice,
        }
    }

    /// Reorient defense doctrine in-game (P5-B). Anatomy (metabolism) stays sealed.
    /// Costs 1 STAMP on the genesis orb. primary 0 clears domain override.
    pub fn reorient_posture(
        &mut self,
        disposition: u8,
        perception: u8,
        primary: u8,
    ) -> Result<(), ActionRefused> {
        self.reorient_posture_as(SOVEREIGN, disposition, perception, primary)
    }

    pub fn reorient_posture_as(
        &mut self,
        actor: u64,
        disposition: u8,
        perception: u8,
        primary: u8,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.genesis.is_none() {
            return Err(ActionRefused::GenesisUnsealed);
        }
        // Axis codes: disposition 1..=3, perception 1..=4 (fold ceilings).
        for (axis, code) in [(1u8, disposition), (3u8, perception)] {
            let folded = crate::vocabulary::Axis::from_code(axis)
                .map(|on| code != 0 && code <= crate::vocabulary::ceiling(self.chronicle(), on))
                .unwrap_or(false);
            if !folded {
                return Err(ActionRefused::UnknownAxisValue { axis, code });
            }
        }
        if primary != 0 && DefenseDomain::from_wire(primary).is_none() {
            return Err(ActionRefused::UnknownAxisValue {
                axis: 0,
                code: primary,
            });
        }
        let orb = FIRST_ORB;
        let fee = ratio(1, 1);
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < fee {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(fee),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-fee);
        self.posture_disposition = Some(disposition);
        self.posture_perception = Some(perception);
        self.posture_primary = if primary == 0 {
            None
        } else {
            Some(primary)
        };
        self.close(
            ClosureEvent::PostureReoriented {
                disposition,
                perception,
                primary,
            },
            actor,
        );
        Ok(())
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
