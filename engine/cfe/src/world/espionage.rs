//! P5-F espionage — scout, exfil, sabotage (36 §4). INF+CYB offensive duals.
//!
//! No free tech mint: exfil stores hashes in intel stash only.
//! Detection grade gates ops; veil toll composes Parameter::VeilToll.

use super::*;
use crate::dop::Resource;

fn eighth() -> Ratio {
    ZoneKind::Resonator.yield_per_tick().1
}

impl World {
    /// Pure espionage surface for DEFEND / suite.
    pub fn espionage_integrity(&self) -> EspionageIntegrity {
        EspionageIntegrity {
            intel_count: self.intel_hashes.len() as u64,
            spy_marks: self.spy_marks.len() as u64,
            detection_grade: self.detection_grade(),
            veiled: self.is_veiled(),
            can_probe: self.detection_grade() >= 1,
            can_exfil: self.detection_grade() >= 1 && !self.techs.is_empty(),
            can_sabotage: !self.instruments.is_empty(),
        }
    }

    pub fn intel_hashes(&self) -> &[[u8; 32]] {
        &self.intel_hashes
    }

    pub fn spy_marks(&self) -> &[(u64, u8)] {
        &self.spy_marks
    }

    /// Scout probe: pay crystal (veil toll if veiled, else eighth), need detection ≥ 1.
    /// Depth = detection grade (capped 7). No foreign book required for v1 residual.
    pub fn spy_probe(&mut self, orb: u64) -> Result<u8, ActionRefused> {
        self.spy_probe_as(SOVEREIGN, orb)
    }

    pub fn spy_probe_as(&mut self, actor: u64, orb: u64) -> Result<u8, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        self.require_detection(1)?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let fee = if self.is_veiled() {
            crate::parameter::of(self.chronicle(), crate::parameter::Parameter::VeilToll)
        } else {
            eighth()
        };
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.banked_amount(Resource::Crystal);
        if have < fee {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal,
                need: Box::new(fee.clone()),
                have: Box::new(have),
            });
        }
        held.bank(Resource::Crystal, -fee.clone());
        let depth = self.detection_grade().min(7);
        self.spy_marks.retain(|(o, _)| *o != orb);
        self.spy_marks.push((orb, depth));
        self.close(
            ClosureEvent::SpyProbed {
                orb_id: orb,
                depth,
                fee,
            },
            actor,
        );
        Ok(depth)
    }

    /// Exfil one tech hash into intel stash — never mints into techs.
    /// Requires prior probe mark on orb and detection ≥ 1.
    pub fn tech_exfil(&mut self, orb: u64, condition: u8) -> Result<[u8; 32], ActionRefused> {
        self.tech_exfil_as(SOVEREIGN, orb, condition)
    }

    pub fn tech_exfil_as(
        &mut self,
        actor: u64,
        orb: u64,
        condition: u8,
    ) -> Result<[u8; 32], ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        self.require_detection(1)?;
        if !self.spy_marks.iter().any(|(o, _)| *o == orb) {
            return Err(ActionRefused::SpyProbeRequired { orb });
        }
        let tech_hash = self
            .techs
            .iter()
            .find(|(o, c, _, _)| *o == orb && *c == condition)
            .map(|(_, _, h, _)| *h)
            .ok_or(ActionRefused::ConditionUncrystallized { condition })?;
        if self.intel_hashes.contains(&tech_hash) {
            return Err(ActionRefused::IntelAlreadyHeld);
        }
        let fee = eighth();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.banked_amount(Resource::Crystal);
        if have < fee {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal,
                need: Box::new(fee.clone()),
                have: Box::new(have),
            });
        }
        held.bank(Resource::Crystal, -fee.clone());
        self.intel_hashes.push(tech_hash);
        self.close(
            ClosureEvent::TechExfiltrated {
                condition,
                tech_hash,
                fee,
            },
            actor,
        );
        Ok(tech_hash)
    }

    /// Sabotage: remove one instrument (lowest grade first) — typed residual.
    /// Requires probe mark and at least one instrument.
    pub fn sabotage_instrument(&mut self, orb: u64) -> Result<u8, ActionRefused> {
        self.sabotage_instrument_as(SOVEREIGN, orb)
    }

    pub fn sabotage_instrument_as(
        &mut self,
        actor: u64,
        orb: u64,
    ) -> Result<u8, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        self.require_detection(1)?;
        if !self.spy_marks.iter().any(|(o, _)| *o == orb) {
            return Err(ActionRefused::SpyProbeRequired { orb });
        }
        let at = self
            .instruments
            .iter()
            .enumerate()
            .filter(|(_, (o, _, _))| *o == orb)
            .min_by_key(|(_, (_, _, g))| *g)
            .map(|(i, _)| i)
            .ok_or(ActionRefused::NoInstrumentToSabotage { orb })?;
        let (o, tech_hash, grade) = self.instruments.remove(at);
        let _ = o;
        self.close(
            ClosureEvent::InstrumentSabotaged {
                orb_id: orb,
                grade,
                tech_hash,
            },
            actor,
        );
        Ok(grade)
    }
}

/// Espionage surface snapshot.
#[derive(Debug, Clone)]
pub struct EspionageIntegrity {
    pub intel_count: u64,
    pub spy_marks: u64,
    pub detection_grade: u8,
    pub veiled: bool,
    pub can_probe: bool,
    pub can_exfil: bool,
    pub can_sabotage: bool,
}
