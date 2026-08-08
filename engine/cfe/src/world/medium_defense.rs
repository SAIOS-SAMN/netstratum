//! P5-D / P5.19b ATM — CovertTamper + MediumEnvelope + species residual voice.

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

    /// P5.19b — reinforce spend resource by sealed metabolism (no free dial).
    /// Mineral: iron shell · biotic: biomass membrane · plasmic: crystal field.
    pub fn medium_reinforce_resource(&self) -> Resource {
        match self.defense_posture().metabolism {
            2 => Resource::Biomass,
            3 => Resource::Crystal,
            _ => Resource::Iron,
        }
    }

    /// P5.19b — species-relative medium residual voice (36 §1.4 / §3.3).
    pub fn medium_residual_voice(&self) -> &'static str {
        let p = self.defense_posture();
        match (p.metabolism, p.clade) {
            (1, Some(4) | Some(14)) => "lithic pore fluid / dust radiator under stress",
            (1, _) => "mineral medium — dust, albedo, coolant envelope",
            (2, Some(5)) => "cryptobiotic shell — desiccation as strategy until limit",
            (2, Some(6)) => "cetacean song medium — density wrongness is attack",
            (2, _) => "biotic medium — gas mix, humidity, Habitat envelope",
            (3, Some(12) | Some(13)) => "plasmic bottle — field shear and density quench",
            (3, _) => "plasmic weather — magnetic envelope, radiative strip",
            _ => "medium envelope under pressure",
        }
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
            dual_use: self.can_terraform(),
            reinforce_resource: self.medium_reinforce_resource(),
            residual_voice: self.medium_residual_voice(),
            metabolism: self.defense_posture().metabolism,
        }
    }

    /// Dual-use ATM reading (P5.19): TERRA crown held ⇒ covert path taxed.
    pub fn dual_use_atm(&self) -> DualUseAtm {
        DualUseAtm {
            can_overt_terraform: self.can_terraform(),
            can_covert_tamper: true,
            dual_use_tax: self.can_terraform(),
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
        // P5.19 dual-use: never rewrites PlanetSited / thermal seat.
        // Holding TERRA crown doubles crystal cost (1+1) — overt path is terraform.
        let base = eighth();
        let cost = if self.can_terraform() {
            base.clone() + base.clone()
        } else {
            base.clone()
        };
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        // Dual-use law: crystal debit only — thermal / PlanetSited never written here.
        let have = held.banked_amount(Resource::Crystal);
        if have < cost {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal,
                need: Box::new(cost.clone()),
                have: Box::new(have),
            });
        }
        held.bank(Resource::Crystal, -cost.clone());
        // Pressure rises by one eighth always (meter, not cost).
        let mut pressure = self.tamper_pressure_of(orb) + base;
        // Cap at 2 (two units) so reinforce remains meaningful — 1+1+1+1.
        let cap = ratio(1, 1) + ratio(1, 1);
        if pressure > cap {
            pressure = cap;
        }
        self.set_tamper_pressure(orb, pressure.clone());
        self.close(
            ClosureEvent::CovertTamperStruck {
                orb_id: orb,
                cost,
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
        // P5.19b: spend species medium resource (iron / biomass / crystal).
        let resource = self.medium_reinforce_resource();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.banked_amount(resource);
        if have < step {
            return Err(ActionRefused::CannotAfford {
                resource,
                need: Box::new(step.clone()),
                have: Box::new(have),
            });
        }
        held.bank(resource, -step.clone());
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
                resource,
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
    /// True when TERRA crown is held — covert path is dual-use taxed.
    pub dual_use: bool,
    /// P5.19b reinforce debit resource.
    pub reinforce_resource: Resource,
    /// Species-relative residual voice.
    pub residual_voice: &'static str,
    pub metabolism: u8,
}

/// P5.19 dual-use ATM gate reading.
#[derive(Debug, Clone)]
pub struct DualUseAtm {
    pub can_overt_terraform: bool,
    pub can_covert_tamper: bool,
    pub dual_use_tax: bool,
}
