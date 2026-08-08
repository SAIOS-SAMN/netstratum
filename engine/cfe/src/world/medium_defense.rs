//! P5-D ATM — CovertTamper pressure + MediumEnvelope reinforce (36 §3, §11).

use super::*;
use crate::dop::Resource;

/// Unit threshold: pressure ≥ 1 triggers TamperDetected (1+1 identity).
pub fn tamper_threshold() -> Ratio {
    ratio(1, 1)
}

/// Per-deed pressure step and cost — eighth from resonator yield (derived).
fn eighth() -> Ratio {
    ZoneKind::Resonator.yield_per_tick().1
}

impl World {
    /// ATM tamper pressure on an orb (pure reading).
    pub fn tamper_pressure_of(&self, orb: u64) -> Ratio {
        self.tamper_pressures
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// Medium envelope reading for DEFEND / suite.
    pub fn medium_integrity(&self, orb: u64) -> MediumIntegrity {
        let pressure = self.tamper_pressure_of(orb);
        let thr = tamper_threshold();
        MediumIntegrity {
            orb,
            pressure: pressure.clone(),
            threshold: thr.clone(),
            compromised: pressure >= thr,
        }
    }

    fn set_tamper_pressure(&mut self, orb: u64, pressure: Ratio) {
        self.tamper_pressures.retain(|(o, _)| *o != orb);
        if pressure > ratio(0, 1) {
            self.tamper_pressures.push((orb, pressure));
        }
    }

    /// Covert atmospheric tamper: spend crystal eighth, raise pressure by eighth.
    /// At unit threshold, also receipts TamperDetected.
    pub fn covert_tamper(&mut self, orb: u64) -> Result<Ratio, ActionRefused> {
        self.covert_tamper_as(SOVEREIGN, orb)
    }

    pub fn covert_tamper_as(&mut self, actor: u64, orb: u64) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let step = eighth();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.banked_amount(Resource::Crystal);
        if have < step {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal,
                need: Box::new(step.clone()),
                have: Box::new(have),
            });
        }
        held.bank(Resource::Crystal, -step.clone());
        let mut pressure = self.tamper_pressure_of(orb) + step.clone();
        // Cap at 2 (two units) so reinforce remains meaningful — 1+1+1+1.
        let cap = ratio(1, 1) + ratio(1, 1);
        if pressure > cap {
            pressure = cap;
        }
        self.set_tamper_pressure(orb, pressure.clone());
        self.close(
            ClosureEvent::CovertTamperStruck {
                orb_id: orb,
                cost: step,
                pressure: pressure.clone(),
            },
            actor,
        );
        if pressure >= tamper_threshold() {
            self.close(
                ClosureEvent::TamperDetected {
                    orb_id: orb,
                    pressure: pressure.clone(),
                },
                actor,
            );
        }
        Ok(pressure)
    }

    /// Reinforce medium envelope: spend iron eighth, lower pressure by eighth.
    pub fn medium_reinforce(&mut self, orb: u64) -> Result<Ratio, ActionRefused> {
        self.medium_reinforce_as(SOVEREIGN, orb)
    }

    pub fn medium_reinforce_as(
        &mut self,
        actor: u64,
        orb: u64,
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let step = eighth();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.banked_amount(Resource::Iron);
        if have < step {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Iron,
                need: Box::new(step.clone()),
                have: Box::new(have),
            });
        }
        held.bank(Resource::Iron, -step.clone());
        let cur = self.tamper_pressure_of(orb);
        let pressure = if cur > step {
            cur - step.clone()
        } else {
            ratio(0, 1)
        };
        self.set_tamper_pressure(orb, pressure.clone());
        self.close(
            ClosureEvent::MediumReinforced {
                orb_id: orb,
                cost: step,
                pressure: pressure.clone(),
            },
            actor,
        );
        Ok(pressure)
    }
}

/// ATM integrity snapshot (pure).
#[derive(Debug, Clone)]
pub struct MediumIntegrity {
    pub orb: u64,
    pub pressure: Ratio,
    pub threshold: Ratio,
    pub compromised: bool,
}
