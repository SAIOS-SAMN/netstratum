//! P5-M / P5-M2 MKT — clear seal, circuit breaker, raid, cascade, haircut (36 §10–§11).

use super::*;
use crate::dop::Resource;

/// Q-S4 / 36 §10: honest clears required before last-clear marks stick.
pub const CLEAR_SEAL_DEPTH: u64 = 8;

/// Breaker duration: one year-voice (compose journal / orbit phase base).
pub const BREAKER_TICKS: u64 = 60;

/// Early resume requires at least half seal depth on crystal (1+1 of 8).
pub const RESUME_AUDIT_DEPTH: u64 = CLEAR_SEAL_DEPTH / 2;

impl World {
    /// Q-S4 depth for a resource.
    pub fn clear_depth_of(&self, resource: Resource) -> u64 {
        self.clear_depths
            .iter()
            .find(|(r, _)| *r == resource)
            .map(|(_, d)| *d)
            .unwrap_or(0)
    }

    /// Note-cascade pressure on an orb (P5-M2).
    pub fn cascade_pressure_of(&self, orb: u64) -> u64 {
        self.cascade_pressures
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, p)| *p)
            .unwrap_or(0)
    }

    /// Circuit breaker engaged?
    pub fn market_sealed(&self) -> bool {
        match self.market_sealed_until {
            Some(t) => self.clock.now() < t,
            None => false,
        }
    }

    pub fn market_resume_at(&self) -> Option<u64> {
        self.market_sealed_until.filter(|&t| self.clock.now() < t)
    }

    /// Integrity flags for EXCH/DEFEND (pure).
    pub fn market_integrity(&self) -> MarketIntegrity {
        let orb = self.orb().map(|o| o.orb_id).unwrap_or(FIRST_ORB);
        MarketIntegrity {
            sealed: self.market_sealed(),
            resume_at: self.market_resume_at(),
            notes_open: self.notes_listable() && !self.market_sealed(),
            futures_open: self.futures_listable() && !self.market_sealed(),
            shares_open: self.shares_listable() && !self.market_sealed(),
            perps_open: self.perps_listable() && !self.market_sealed(),
            crystal_depth: self.clear_depth_of(Resource::Crystal),
            iron_depth: self.clear_depth_of(Resource::Iron),
            seal_need: CLEAR_SEAL_DEPTH,
            cascade_pressure: self.cascade_pressure_of(orb),
        }
    }

    /// Collateral floor for a note face (P5-M2 haircut ladder).
    ///
    /// Base half face; each cascade unit and each open note beyond the
    /// first adds face/8; thin crystal clear (< half seal) adds face/8;
    /// never exceeds face. All steps from 1+1.
    pub fn note_collateral_floor(&self, orb: u64, face: &Ratio) -> Ratio {
        let two = ratio(1, 1) + ratio(1, 1);
        // 1/8 from resonator yield ladder (derived, no free dial).
        let eighth = ZoneKind::Resonator.yield_per_tick().1;
        let half = face.clone() / two;
        let mut extra_steps: u64 = 0;
        extra_steps = extra_steps.saturating_add(self.cascade_pressure_of(orb));
        let open_notes = self
            .notes
            .iter()
            .filter(|n| n.open && n.orb_id == orb)
            .count() as u64;
        if open_notes > 1 {
            extra_steps = extra_steps.saturating_add(open_notes.saturating_sub(1));
        }
        if self.clear_depth_of(Resource::Crystal) < RESUME_AUDIT_DEPTH {
            extra_steps = extra_steps.saturating_add(1);
        }
        let mut floor = half;
        for _ in 0..extra_steps {
            floor += face.clone() * eighth.clone();
        }
        if floor > *face {
            face.clone()
        } else {
            floor
        }
    }

    fn require_market_open(&self) -> Result<(), ActionRefused> {
        if let Some(t) = self.market_resume_at() {
            return Err(ActionRefused::MarketCircuitOpen { resume_at: t });
        }
        Ok(())
    }

    /// Honest spot clear: bump depth and refresh last-clear (defense path).
    pub fn record_spot_clear(
        &mut self,
        orb: u64,
        resource: Resource,
        price: crate::dop::Positive,
    ) -> Result<u64, ActionRefused> {
        self.record_spot_clear_as(SOVEREIGN, orb, resource, price)
    }

    pub fn record_spot_clear_as(
        &mut self,
        actor: u64,
        orb: u64,
        resource: Resource,
        price: crate::dop::Positive,
    ) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        self.require_market_open()?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let p = price.value().clone();
        let depth = self.clear_depth_of(resource).saturating_add(1);
        self.clear_depths.retain(|(r, _)| *r != resource);
        self.clear_depths.push((resource, depth));
        // Once sealed depth met, last clear updates freely on honest clears.
        if depth >= CLEAR_SEAL_DEPTH {
            self.last_clears.retain(|(r, _)| *r != resource);
            self.last_clears.push((resource, p.clone()));
        }
        self.close(
            ClosureEvent::SpotClearRecorded {
                orb_id: orb,
                resource,
                price: p,
                depth,
            },
            actor,
        );
        Ok(depth)
    }

    /// Trip circuit breaker for BREAKER_TICKS (or until explicit resume after).
    pub fn seal_market(&mut self) -> Result<u64, ActionRefused> {
        self.seal_market_as(SOVEREIGN)
    }

    pub fn seal_market_as(&mut self, actor: u64) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let resume = self.clock.now().saturating_add(BREAKER_TICKS);
        self.market_sealed_until = Some(resume);
        self.close(ClosureEvent::MarketSealed { resume_at: resume }, actor);
        Ok(resume)
    }

    /// Lift breaker early only when crystal clear depth meets resume audit.
    /// If already open (clock past resume), no-ops with MarketResumed.
    pub fn resume_market(&mut self) -> Result<(), ActionRefused> {
        self.resume_market_as(SOVEREIGN)
    }

    pub fn resume_market_as(&mut self, actor: u64) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.market_sealed() {
            let depth = self.clear_depth_of(Resource::Crystal);
            if depth < RESUME_AUDIT_DEPTH {
                return Err(ActionRefused::ResumeAuditUnmet {
                    depth,
                    need: RESUME_AUDIT_DEPTH,
                });
            }
        }
        self.market_sealed_until = None;
        self.close(ClosureEvent::MarketResumed {}, actor);
        Ok(())
    }

    /// Economic attack: burn fee STAMP, void up to one resting offer (typed residual).
    pub fn liquidity_raid(&mut self, orb: u64) -> Result<(), ActionRefused> {
        self.liquidity_raid_as(SOVEREIGN, orb)
    }

    pub fn liquidity_raid_as(&mut self, actor: u64, orb: u64) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.market_sealed() {
            return Err(ActionRefused::MarketCircuitOpen {
                resume_at: self.market_resume_at().unwrap_or(0),
            });
        }
        // Fee = 1 STAMP (identity unit).
        let fee = ratio(1, 1);
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < fee {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(fee),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-fee.clone());
        let mut voided: u8 = 0;
        if let Some(i) = self.offers.iter().position(|o| o.orb == orb) {
            self.offers.remove(i);
            voided = 1;
        }
        self.close(
            ClosureEvent::LiquidityRaidStruck {
                orb_id: orb,
                fee,
                offers_voided: voided,
            },
            actor,
        );
        Ok(())
    }

    /// Note cascade (P5-M2 A3): burn 1 STAMP, raise cascade pressure by 1.
    pub fn note_cascade(&mut self, orb: u64) -> Result<u64, ActionRefused> {
        self.note_cascade_as(SOVEREIGN, orb)
    }

    pub fn note_cascade_as(&mut self, actor: u64, orb: u64) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        self.require_market_open()?;
        let fee = ratio(1, 1);
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < fee {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(fee),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-fee.clone());
        let pressure = self.cascade_pressure_of(orb).saturating_add(1);
        self.cascade_pressures.retain(|(o, _)| *o != orb);
        self.cascade_pressures.push((orb, pressure));
        self.close(
            ClosureEvent::NoteCascadeStruck {
                orb_id: orb,
                fee,
                pressure,
            },
            actor,
        );
        Ok(pressure)
    }
}

/// EXCH / suite integrity snapshot.
#[derive(Debug, Clone)]
pub struct MarketIntegrity {
    pub sealed: bool,
    pub resume_at: Option<u64>,
    pub notes_open: bool,
    pub futures_open: bool,
    pub shares_open: bool,
    pub perps_open: bool,
    pub crystal_depth: u64,
    pub iron_depth: u64,
    pub seal_need: u64,
    pub cascade_pressure: u64,
}
