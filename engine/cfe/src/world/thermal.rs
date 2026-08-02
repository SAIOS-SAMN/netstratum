//! THERMAL DEEDS — the star, the siting, venting and encasement.
//!
//! THE VENT (20_FOUNDING §4): a mineral or plasmic line founds
//! BANDLESS -- above the mesic ceiling, an order of magnitude below
//! the hyperthermal floor. Coverage is defined by a bucket lookup that
//! returns nothing in that gap. Task #3 opens it.

use super::*;

impl World {
    /// THERMAL-1: the commissioned sun.
    pub fn star(&self) -> Option<crate::thermal::StarClass> {
        self.star
    }
    /// THERMAL-1 — commission the container's sun (once per world): the
    /// StarClass fixes L for every orbit (NS-6 §7).
    pub fn commission_star(
        &mut self,
        class: crate::thermal::StarClass,
    ) -> Result<(), ActionRefused> {
        if self.star.is_some() {
            return Err(ActionRefused::StarAlreadyCommissioned);
        }
        self.star = Some(class);
        self.close(ClosureEvent::StarCommissioned { class: class as u8 }, SOVEREIGN);
        Ok(())
    }
    /// THERMAL-1 — site an orb on its orbit (05 §1.1): albedo A and
    /// radius r enter the master equation; thermal wakes for this orb.
    pub fn site_planet(
        &mut self,
        orb: u64,
        albedo: Ratio,
        orbit_r: Ratio,
    ) -> Result<(), ActionRefused> {
        use crate::dop::ThermalSite;
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        let zero = ratio(0, 1);
        let one = ratio(1, 1);
        if albedo < zero || albedo >= one || orbit_r <= zero {
            return Err(ActionRefused::BadSiting);
        }
        let held = peel(&mut self.orbs, orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.thermal.is_some() {
            return Err(ActionRefused::AlreadySited { orb });
        }
        held.thermal = Some(ThermalSite {
            albedo: albedo.clone(),
            orbit_r: orbit_r.clone(),
            // Historical siting carries no angle — an orb has a radius
            // and no place until one is receipted.
            theta: None,
        });
        // TECH-1a (12_UNION §5a): a world's FIRST thermal opt-in
        // receipts the kernel SOLAR LEAP — its people now know their
        // sun. Once per world, never per orb; the node id is seeded
        // from the kernel name + the discovering orb (SEAL B).
        if !self.techs.iter().any(|(_, condition, _)| *condition == 0) {
            let mut seed = b"tech.kernel.solar-leap".to_vec();
            seed.extend_from_slice(&orb.to_le_bytes());
            let node: [u8; 32] = blake3::hash(&seed).into();
            self.techs.push((orb, 0, node));
            self.close(
                ClosureEvent::TechDiscovered {
                    orb_id: orb,
                    node,
                    condition: 0,
                    modifier: 0,
                    magnitude: ratio(1, 1),
                    name: "the solar leap".into(),
                },
                SOVEREIGN,
            );
        }
        self.close(
            ClosureEvent::PlanetSited { orb_id: orb, albedo, orbit_r, theta: None },
            SOVEREIGN,
        );
        Ok(())
    }
    /// AEON-1a (11 §6): the star MOVES in its region — the class
    /// steps, luminosity recomputes, and every sited orbit's T_eq⁴
    /// shifts at the next tick. Movement, never a rung.
    pub fn age_star(
        &mut self,
        class: crate::thermal::StarClass,
    ) -> Result<(), ActionRefused> {
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        self.star = Some(class);
        self.close(ClosureEvent::StarAged { class: class as u8 }, SOVEREIGN);
        Ok(())
    }
    /// ABSORB-1: this orb's un-sealed vented overflow (workspace) —
    /// what stands absorbable before the void claims it at collect.
    pub fn vent_pending_of(&self, orb: u64) -> Ratio {
        self.vent_pending
            .iter()
            .find(|(held, _)| *held == orb)
            .map(|(_, pending)| pending.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }
}
