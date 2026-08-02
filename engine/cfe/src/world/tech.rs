//! TECH AND PRACTICE — conditions, instruments, adopted disciplines.
//!
//! THE CLIFF (task #4/#5): `condition == 5` and `hardships < 8` gate
//! outcomes here, so everything short of the line reads as nothing.
//! P3 tried to remove the first and was reverted -- see `super::rules`.

use super::*;

impl World {
    /// TECH-1a: the discovered nodes — (orb, condition, node id).
    pub fn techs(&self) -> &[(u64, u8, [u8; 32])] {
        &self.techs
    }
    /// #40 (the petition's executor): GRANT a technology node under
    /// steward containment — the inbox form of the licensed arrival
    /// (the two-book licensing pair is recorded debt until NS-5
    /// arbitrates a true grantor). The crown law holds HERE: condition
    /// 6 lands only on a complete base-7 stack; the solar leap is
    /// never granted; one node per condition per orb.
    pub fn grant_tech_as(
        &mut self,
        actor: u64,
        orb: u64,
        condition: u8,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if condition == 0 {
            return Err(ActionRefused::SolarLeapNeverLicensed);
        }
        if !(1..=6).contains(&condition) {
            return Err(ActionRefused::BadTransfer);
        }
        if condition == 6 && !self.base_seven_complete() {
            return Err(ActionRefused::BaseSevenUnfinished);
        }
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        if self.techs.iter().any(|(o, c, _)| *o == orb && *c == condition) {
            return Err(ActionRefused::BadTransfer);
        }
        let mut seed = b"tech.granted".to_vec();
        seed.extend_from_slice(&orb.to_le_bytes());
        seed.push(condition);
        if let Some(receipt) = self.chronicle.receipts().last() {
            seed.extend_from_slice(&receipt.own.0);
        }
        let node: [u8; 32] = blake3::hash(&seed).into();
        let name = format!("gifted {}", crate::dop::tech_node_name(condition, &node));
        self.techs.push((orb, condition, node));
        let stamp = self.clock.now();
        let (modifier, magnitude) =
            crate::dop::tech_node_of(condition).unwrap_or((condition, ratio(1, 1)));
        self.chronicle.append(
            ClosureEvent::TechDiscovered {
                orb_id: orb,
                node,
                condition,
                modifier,
                magnitude,
                name,
            },
            stamp,
            vec![actor],
        );
        Ok(())
    }
    /// MANU-1 (10 §3f): FORGE AN INSTRUMENT — manufacturing overlays
    /// technology + understanding + industry. Grade IS the
    /// crystallized condition; the converter chain must stand (a
    /// smelter for grades 1–2, smelter + synthesizer for 3+); the
    /// frame draws alloy = grade, and grade 3+ optics draw one
    /// diamond (v1 placeholders at the condition table's cadence).
    /// The scan-reach law reads the best grade held: reach = base +
    /// grade — never a global constant again.
    pub fn forge_instrument(&mut self, orb: u64, condition: u8) -> Result<(), ActionRefused> {
        if condition == 0 || condition > 6 {
            return Err(ActionRefused::InstrumentGradeUnknown { condition });
        }
        let tech_hash = self
            .techs
            .iter()
            .find(|(held, held_condition, _)| *held == orb && *held_condition == condition)
            .map(|(_, _, node)| *node)
            .ok_or(ActionRefused::ConditionUncrystallized { condition })?;
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let chain = match condition {
            1 | 2 => vec![ZoneKind::Smelter],
            _ => vec![ZoneKind::Smelter, ZoneKind::Synthesizer],
        };
        for needs in chain {
            if !held.zones.iter().any(|zone| zone.kind == needs) {
                return Err(ActionRefused::ChainIncomplete { needs });
            }
        }
        let mut costs = vec![(Resource::Alloy, ratio(condition as i64, 1))];
        if condition >= 3 {
            costs.push((Resource::Diamond, ratio(1, 1)));
        }
        for (resource, need) in &costs {
            let have = held.banked_amount(*resource);
            if have < *need {
                return Err(ActionRefused::CannotAfford {
                    resource: *resource,
                    need: Box::new(need.clone()),
                    have: Box::new(have),
                });
            }
        }
        for (resource, need) in costs {
            held.bank(resource, -need);
        }
        self.instruments.push((orb, tech_hash, condition));
        self.close(
            ClosureEvent::InstrumentForged { orb_id: orb, tech_hash, grade: condition },
            SOVEREIGN,
        );
        Ok(())
    }
    /// MANU-1: the held instruments — (orb, tech node, grade).
    pub fn instruments(&self) -> &[(u64, [u8; 32], u8)] {
        &self.instruments
    }
    /// TECH-1b (10 §3): does a condition HOLD, read from the deposit
    /// sheet + committed state? Thresholds stay the table's
    /// placeholder parameters.
    pub(super) fn condition_holds(&self, orb: u64, condition: u8, now: u64, sheet: &CrystalDeposit) -> bool {
        let Some(held) = peek(&self.orbs, orb) else {
            return false;
        };
        match condition {
            1 => {
                let open = sheet
                    .famine_open
                    .map(|struck| now.saturating_sub(struck))
                    .unwrap_or(0);
                sheet.famine_total + open >= 480
            }
            2 => sheet
                .overheat_open
                .is_some_and(|struck| now.saturating_sub(struck) >= 240),
            3 => {
                held.banked_amount(Resource::Sink) >= ratio(100, 1)
                    && demurrage_due(held, self.star, now, &ease_of(&self.techs, orb, now), &standing_supply(&self.orbs))
                        .iter()
                        .all(|(_, due)| *due == ratio(0, 1))
            }
            4 => {
                let floor = now.saturating_sub(self.deposit_window());
                let vented: Ratio = sheet
                    .vented_entries
                    .iter()
                    .filter(|(stamp, _)| *stamp >= floor)
                    .map(|(_, amount)| amount.clone())
                    .sum();
                let encased: Ratio = sheet
                    .encased_entries
                    .iter()
                    .filter(|(stamp, _)| *stamp >= floor)
                    .map(|(_, amount)| amount.clone())
                    .sum();
                vented > ratio(0, 1) && vented > encased
            }
            5 => {
                let active = held
                    .zones
                    .iter()
                    .filter(|zone| held.zone_active(zone))
                    .count() as i64;
                if active == 0 {
                    return false;
                }
                let bill_year =
                    crate::dop::heating_per_zone() * ratio(active, 1) * ratio(60, 1);
                let cold = match (self.star, &held.thermal) {
                    (Some(class), Some(site)) => {
                        let t4 = crate::thermal::t_eq4(
                            &class.luminosity(),
                            &site.albedo,
                            &site.orbit_r,
                        );
                        t4 < crate::thermal::SubstrateBand::Mesic.bounds4().0
                    }
                    _ => false,
                };
                cold && held.banked_amount(Resource::Joule) >= ratio(4, 1) * bill_year
            }
            _ => false,
        }
    }
    /// PRACTICE-1 — adopt a discipline: one benefit, one friction,
    /// bound by receipt (the derived creed never binds; the practice
    /// does). Build scope. The Athanor (5) takes only the tempered —
    /// eight hardship receipts stand in the chronicle or the door
    /// refuses.
    pub fn adopt_practice(&mut self, practice: u8) -> Result<(), ActionRefused> {
        self.adopt_practice_as(SOVEREIGN, practice)
    }
    pub fn adopt_practice_as(&mut self, actor: u64, practice: u8) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !(1..=5).contains(&practice) {
            return Err(ActionRefused::PracticeUnknown { practice });
        }
        if self.practices.contains(&practice) {
            return Err(ActionRefused::PracticeAlreadyAdopted { practice });
        }
        if practice == 5 {
            let hardships = self
                .chronicle
                .receipts()
                .iter()
                .filter(|receipt| {
                    matches!(
                        receipt.event,
                        ClosureEvent::FamineStruck { .. }
                            | ClosureEvent::OverheatStruck { .. }
                            | ClosureEvent::VentSettled { .. }
                            | ClosureEvent::DemurrageSettled { .. }
                    )
                })
                .count();
            if hardships < 8 {
                return Err(ActionRefused::AthanorUntempered { hardships });
            }
        }
        self.close(ClosureEvent::PracticeAdopted { practice }, actor);
        self.practices.push(practice);
        Ok(())
    }
    /// PRACTICE-1 — release is a REDISCOVERY (operator-ruled): lawful
    /// only after new doctrine landed since the adoption. A lawful
    /// release emits the RE-EMERGENCE node (TechDiscovered, condition
    /// 7) seeded from the chronicle root and the practice — divergent
    /// and novel per line per moment; its window-ease boon applies
    /// while alignment holds (the tick reads it).
    pub fn release_practice(&mut self, practice: u8) -> Result<(), ActionRefused> {
        self.release_practice_as(SOVEREIGN, practice)
    }
    pub fn release_practice_as(&mut self, actor: u64, practice: u8) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.practices.contains(&practice) {
            return Err(ActionRefused::PracticeNotHeld { practice });
        }
        let adopted_at = self
            .chronicle
            .receipts()
            .iter()
            .rposition(|receipt| {
                matches!(receipt.event, ClosureEvent::PracticeAdopted { practice: held } if held == practice)
            })
            .expect("the adoption receipted");
        let rediscovered = self.chronicle.receipts()[adopted_at..]
            .iter()
            .any(|receipt| matches!(receipt.event, ClosureEvent::TechDiscovered { .. }));
        if !rediscovered {
            return Err(ActionRefused::ReleaseUnearned { practice });
        }
        self.close(ClosureEvent::PracticeReleased { practice }, actor);
        if let Some(at) = self.practices.iter().position(|held| *held == practice) {
            self.practices.remove(at);
        }
        // The re-emergence: a novel node from the current state.
        let root = self
            .chronicle
            .receipts()
            .last()
            .map(|receipt| receipt.own.0)
            .expect("the release receipted");
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&root);
        seed[0] ^= practice;
        let name = crate::dop::tech_node_name(7, &seed);
        self.close(
            ClosureEvent::TechDiscovered {
                orb_id: FIRST_ORB,
                node: seed,
                condition: 7,
                modifier: 7,
                magnitude: ratio(3, 4),
                name,
            },
            actor,
        );
        self.techs.push((FIRST_ORB, 7, seed));
        Ok(())
    }
}
