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
    pub clade: Option<u16>,
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
        // Clade colour — imperium words (37); codes stable.
        let clade_voice = match clade {
            Some(5) => {
                // hypobiotid (void-sleeper)
                ranks[0] = ranks[0].saturating_add(1).min(3);
                ranks[1] = ranks[1].saturating_add(1).min(3);
                "hypobiotid endurance"
            }
            Some(4) | Some(14) => {
                // lithophage / endolithid
                ranks[0] = ranks[0].saturating_add(1).min(3);
                ranks[2] = ranks[2].saturating_add(1).min(3);
                "lattice memory"
            }
            Some(8) => {
                // hexasocial
                ranks[2] = ranks[2].saturating_add(1).min(3);
                ranks[4] = ranks[4].saturating_add(1).min(3);
                "comb graph"
            }
            Some(6) => {
                // soniform
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[2] = ranks[2].saturating_add(1).min(3);
                "pressure-song medium"
            }
            Some(7) => {
                // polybrachia
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[4] = ranks[4].saturating_add(1).min(3);
                "arm-cortex volumetric"
            }
            Some(12) | Some(13) => {
                // plasmiform / syzygiform
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "field sense"
            }
            Some(10) => {
                // mycotiform
                ranks[0] = ranks[0].saturating_add(1).min(3);
                ranks[4] = ranks[4].saturating_add(1).min(3);
                "hyphal redundancy"
            }
            Some(3) => {
                // cryocoherens
                ranks[1] = ranks[1].saturating_add(1).min(3);
                ranks[0] = ranks[0].saturating_add(1).min(3);
                "coherent ice shell"
            }
            Some(19) => {
                // draconiform
                ranks[2] = ranks[2].saturating_add(1).min(3);
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "coherence apex"
            }
            Some(20) => {
                // metamorphid
                ranks[4] = ranks[4].saturating_add(1).min(3);
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "veil-native"
            }
            Some(1) => {
                // ailuroform
                ranks[3] = ranks[3].saturating_add(1).min(3);
                "solitary apex colour"
            }
            Some(2) => {
                // bimaniform (flatlander-default planar)
                ranks[4] = ranks[4].saturating_add(1).min(3);
                "planar tool-maker colour"
            }
            _ => "open clade colour",
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
        let voice = format!("{met_voice} — {clade_voice}; {dis_word}; {per_voice}");

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
        self.posture_primary = if primary == 0 { None } else { Some(primary) };
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

    /// K1 — preferred interdiction residual *kind* from sealed metabolism.
    /// Haul still debits the cargo resource; this colours voice + UI (and
    /// future multi-resource residual choice). No free dials.
    pub fn kin_residual_resource(&self) -> Resource {
        match self.defense_posture().metabolism {
            2 => Resource::Biomass, // biotic
            3 => Resource::Crystal, // plasmic spark
            _ => Resource::Iron,    // mineral / unsealed
        }
    }

    /// K1 / K3 residual voice (pure string for defend / convoy).
    pub fn kin_residual_voice(&self) -> &'static str {
        let p = self.defense_posture();
        match (p.metabolism, p.clade) {
            (1, Some(4) | Some(14)) => "lithophage lattice — iron/scrap shear",
            (1, _) => "mineral bulk — iron train interdiction",
            (2, Some(5)) => "hypobiotid barge — biomass under desiccation risk",
            (2, Some(6)) => "soniform convoy — pressure-song cargo path",
            (2, Some(7)) => "polybrachia convoy — multi-arm haul",
            (2, Some(8)) => "hexasocial graph — many small escorts, biomass cargo",
            (2, _) => "biotic organ — biomass / ration cargo loss",
            (3, Some(12) | Some(13)) => "plasmiform sled — crystal/spark at reconnection",
            (3, _) => "plasmic sheet — crystal / STAMP-spark residual",
            _ => "convoy residual — typed cargo loss",
        }
    }

    /// K4 — place stretch from receipted orbit period (call 35 dynamo).
    /// Returns period ticks for genesis orb site, or None if unsited.
    pub fn kin_place_period(&self) -> Option<u64> {
        let site = self.orb()?.thermal.as_ref()?;
        Some(orbit_phase_period(self, site))
    }

    /// Pure KIN integrity snapshot (P5-K2).
    pub fn kin_integrity(&self, deal: u64) -> KinIntegrity {
        let p = self.defense_posture();
        let cover = self.escort_cover(deal);
        let risk = self.convoy_risk(deal);
        let band = self.convoy_risk_band(deal);
        let fog = self.corridor_fog(deal);
        KinIntegrity {
            deal,
            residual: self.kin_residual_resource(),
            residual_voice: self.kin_residual_voice(),
            cover,
            risk,
            band,
            place_period: self.kin_place_period(),
            kin_rank: p.ranks[5],
            primary_is_kin: p.primary == DefenseDomain::Kin,
            doctrine_dis: p.disposition,
            fog,
            fog_clear: fog.is_none(),
        }
    }

    /// K5 — perception fog of corridor: None = clear reading; Some = refuse reason code.
    /// Planar: deal must be a chartered route. Harmonic/umbral: need detection grade ≥ 1.
    /// Volumetric: at least one prospect opened (depth evidence).
    /// Unsealed worlds: no fog (anatomy not present to colour perception).
    pub fn corridor_fog(&self, deal: u64) -> Option<CorridorFog> {
        if deal == 0 {
            return Some(CorridorFog::NoDeal);
        }
        self.genesis.as_ref()?;
        let per = self.defense_posture().perception;
        match per {
            // planar — chart seals: only chartered deals are "seen"
            1 if !self.routes.iter().any(|(d, _)| *d == deal) => Some(CorridorFog::Uncharted),
            // volumetric — need prospect depth evidence
            2 if self.prospects.is_empty() => Some(CorridorFog::NoDepth),
            // harmonic / umbral — instrument grade required for risk mark
            3 | 4 if self.detection_grade() < 1 => Some(CorridorFog::Blind),
            _ => None,
        }
    }

    /// Escort coverage on a deal: sum (grade+1)/8 over escort hulls assigned, capped at 1.
    /// K3: hexasocial (clade 8) escorts count one grade higher (graph redundancy).
    pub fn escort_cover(&self, deal: u64) -> Ratio {
        if deal == 0 {
            return ratio(0, 1);
        }
        let eighth = ZoneKind::Resonator.yield_per_tick().1; // 1/8
        let hive = self.taxon.map(|(_, c)| c) == Some(8u16);
        let mut cover = ratio(0, 1);
        for h in &self.hulls {
            if h.assigned_route != deal {
                continue;
            }
            if HullRole::from_wire(h.role) != Some(HullRole::Escort) {
                continue;
            }
            let mut g_i = i64::from(h.grade.min(7)) + 1;
            if hive {
                g_i = (g_i + 1).min(8); // +1 identity grade, cap grade ladder
            }
            let g = ratio(g_i, 1);
            cover += g * eighth.clone();
        }
        let one = ratio(1, 1);
        if cover > one {
            one
        } else {
            cover
        }
    }

    /// Convoy risk: base eighth × (1 − cover), plus K4 place exposure when
    /// orbit period exceeds the phase base (call orbit_phase_period / 35).
    pub fn convoy_risk(&self, deal: u64) -> Ratio {
        let eighth = ZoneKind::Resonator.yield_per_tick().1;
        let cover = self.escort_cover(deal);
        let bare = eighth.clone() * (ratio(1, 1) - cover.clone());
        // K4: slow/far rings (period > phase base) add half-eighth × (1-cover).
        let place_extra = match self.kin_place_period() {
            Some(p) if p > super::ORBIT_PHASE_TICKS => {
                let two = ratio(1, 1) + ratio(1, 1);
                (eighth / two) * (ratio(1, 1) - cover)
            }
            _ => ratio(0, 1),
        };
        bare + place_extra
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

    pub fn mark_convoy_risk_as(&mut self, actor: u64, deal: u64) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if deal == 0 {
            return Err(ActionRefused::RouteUnknown { deal: 0 });
        }
        // K5 perception fog — refuse path, not dice.
        if let Some(fog) = self.corridor_fog(deal) {
            return Err(fog.into_refused(deal));
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
    /// Resource is the cargo being hauled (conservation of what's on the
    /// hull); K1 residual *kind* is a pure reading for species voice.
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
        let _preferred = self.kin_residual_resource();
        let _ = _preferred; // voice only until multi-resource residual lands
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

/// K5 corridor fog reasons (perception colour).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorridorFog {
    NoDeal,
    Uncharted,
    NoDepth,
    Blind,
}

impl CorridorFog {
    pub fn word(self) -> &'static str {
        match self {
            CorridorFog::NoDeal => "no deal",
            CorridorFog::Uncharted => "uncharted (planar)",
            CorridorFog::NoDepth => "no prospect depth (volumetric)",
            CorridorFog::Blind => "detection blind (harmonic/umbral)",
        }
    }

    fn into_refused(self, deal: u64) -> ActionRefused {
        match self {
            CorridorFog::NoDeal | CorridorFog::Uncharted => ActionRefused::RouteUnknown { deal },
            CorridorFog::NoDepth => ActionRefused::CorridorFogDepth { deal },
            CorridorFog::Blind => ActionRefused::DetectionThin { need: 1, have: 0 },
        }
    }
}

/// P5-K2 KIN integrity snapshot.
#[derive(Debug, Clone)]
pub struct KinIntegrity {
    pub deal: u64,
    pub residual: Resource,
    pub residual_voice: &'static str,
    pub cover: Ratio,
    pub risk: Ratio,
    pub band: RiskBand,
    pub place_period: Option<u64>,
    pub kin_rank: u8,
    pub primary_is_kin: bool,
    pub doctrine_dis: u8,
    pub fog: Option<CorridorFog>,
    pub fog_clear: bool,
}
