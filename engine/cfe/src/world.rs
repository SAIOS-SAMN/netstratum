//! The world — the tick loop binding the organs, ruleset v1
//! (02_COLONY_LOOP). Actions that complete CLOSE as receipts; per-tick
//! accrual is workspace (unreceipted, never persisted); the chronicle
//! replays the committed universe from sequence zero (CHARTER SEAL B).
//!
//! The flow model (02 §2.3): each tick, every ACTIVE zone nets
//! yield − upkeep into the accrued workspace (per-resource, may be
//! negative). `collect` settles the workspace against banked. Famine
//! (02 §2.4) strikes at the tick where banked + accrued falls below
//! zero: the stores empty, consumers go dormant, and relief needs the
//! resource back at one-tick upkeep.

use crate::dop::{
    founding_reserves, ClosureEvent, ColonyOrb, Resource, Zone, ZoneKind,
};
use crate::ipsal::{Entity, Scope};
use crate::ratio::{ratio, Ratio};
use crate::vrp::{Attestation, Chronicle, Timekeeper, TickRead, WorldClock};

/// The sovereign's entity id — the human at the terminal, registered
/// at founding.
pub const SOVEREIGN: u64 = 1;

/// A typed action refusal — the world refuses instead of guessing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionRefused {
    NoColony,
    ColonyAlreadyFounded,
    TileAlreadyZoned { tile: u16 },
    NoSuchZone { tile: u16 },
    CannotAfford { resource: Resource, need: Box<Ratio>, have: Box<Ratio> },
    NothingAccrued,
    /// N3 — no entity holds this id.
    NoSuchEntity { entity: u64 },
    /// N3 — the handle is already registered.
    HandleTaken { handle: String },
    /// N3 — the acting entity lacks the scope; containment refused it.
    NotPermitted { entity: u64, scope: Scope },
    /// N3 — grants and revocations apply to agents, never sovereigns.
    NotAnAgent { entity: u64 },
}

/// One tick's read: the clock walk, and any famines that struck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickOutcome {
    pub read: TickRead,
    pub struck: Vec<Resource>,
}

/// The committed projection — exactly what the chronicle grounds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Committed {
    pub orb: Option<ColonyOrb>,
    pub entities: Vec<Entity>,
}

pub struct World {
    clock: WorldClock,
    keepers: [Timekeeper; 3],
    chronicle: Chronicle,
    orb: Option<ColonyOrb>,
    entities: Vec<Entity>,
    /// WORKSPACE: net flow since the last collection — no receipt, no
    /// persistence, not part of the committed projection.
    accrued: Vec<(Resource, Ratio)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            clock: WorldClock::new(),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle: Chronicle::new(),
            orb: None,
            entities: Vec::new(),
            accrued: Vec::new(),
        }
    }

    pub fn now(&self) -> u64 {
        self.clock.now()
    }

    pub fn chronicle(&self) -> &Chronicle {
        &self.chronicle
    }

    pub fn orb(&self) -> Option<&ColonyOrb> {
        self.orb.as_ref()
    }

    pub fn accrued(&self) -> &[(Resource, Ratio)] {
        &self.accrued
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn entity(&self, entity: u64) -> Option<&Entity> {
        self.entities.iter().find(|held| held.entity_id == entity)
    }

    /// Found the colony — a closure; founding banks the bootstrap
    /// reserves (02 §2.1) and, first, registers the SOVEREIGN: the
    /// human at the terminal becomes entity 1, and every receipt they
    /// forge carries their generation tag.
    pub fn found(&mut self, name: &str) -> Result<(), ActionRefused> {
        match self.orb {
            Some(_) => Err(ActionRefused::ColonyAlreadyFounded),
            None => {
                if self.entities.is_empty() {
                    self.entities.push(Entity {
                        entity_id: SOVEREIGN,
                        handle: "founder".to_string(),
                        principal: None,
                        grants: Vec::new(),
                    });
                    self.close(
                        ClosureEvent::EntityRegistered {
                            entity_id: SOVEREIGN,
                            principal: 0,
                            handle: "founder".to_string(),
                        },
                        SOVEREIGN,
                    );
                }
                let orb_id = 1;
                let mut orb = ColonyOrb {
                    orb_id,
                    name: name.to_string(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    orb.bank(resource, amount);
                }
                self.orb = Some(orb);
                self.close(
                    ClosureEvent::ColonyFounded {
                        orb_id,
                        name: name.to_string(),
                    },
                    SOVEREIGN,
                );
                Ok(())
            }
        }
    }

    /// Register a scripted agent contained by the sovereign (N3). The
    /// agent starts with NO grants: containment first, capability by
    /// explicit grant.
    pub fn register_agent(&mut self, handle: &str) -> Result<u64, ActionRefused> {
        if self.entities.is_empty() {
            return Err(ActionRefused::NoSuchEntity { entity: SOVEREIGN });
        }
        if self.entities.iter().any(|held| held.handle == handle) {
            return Err(ActionRefused::HandleTaken {
                handle: handle.to_string(),
            });
        }
        let entity_id = self.entities.iter().map(|held| held.entity_id).max().unwrap_or(0) + 1;
        self.entities.push(Entity {
            entity_id,
            handle: handle.to_string(),
            principal: Some(SOVEREIGN),
            grants: Vec::new(),
        });
        self.close(
            ClosureEvent::EntityRegistered {
                entity_id,
                principal: SOVEREIGN,
                handle: handle.to_string(),
            },
            SOVEREIGN,
        );
        Ok(entity_id)
    }

    /// Grant an agent one action scope — a closure by the sovereign.
    pub fn grant(&mut self, agent: u64, scope: Scope) -> Result<(), ActionRefused> {
        let held = self
            .entities
            .iter_mut()
            .find(|held| held.entity_id == agent)
            .ok_or(ActionRefused::NoSuchEntity { entity: agent })?;
        if held.principal.is_none() {
            return Err(ActionRefused::NotAnAgent { entity: agent });
        }
        if !held.grants.contains(&scope) {
            held.grants.push(scope);
            held.grants.sort();
            self.close(ClosureEvent::ScopeGranted { agent, scope }, SOVEREIGN);
        }
        Ok(())
    }

    /// Revoke a grant — revocation is immediate: the agent's next such
    /// action refuses.
    pub fn revoke(&mut self, agent: u64, scope: Scope) -> Result<(), ActionRefused> {
        let held = self
            .entities
            .iter_mut()
            .find(|held| held.entity_id == agent)
            .ok_or(ActionRefused::NoSuchEntity { entity: agent })?;
        if held.principal.is_none() {
            return Err(ActionRefused::NotAnAgent { entity: agent });
        }
        if held.grants.contains(&scope) {
            held.grants.retain(|granted| *granted != scope);
            self.close(ClosureEvent::ScopeRevoked { agent, scope }, SOVEREIGN);
        }
        Ok(())
    }

    /// The containment check (NS-6): sovereigns act by construction;
    /// agents act inside their grants, or the action refuses typed.
    fn permitted(&self, actor: u64, scope: Scope) -> Result<(), ActionRefused> {
        let entity = self
            .entity(actor)
            .ok_or(ActionRefused::NoSuchEntity { entity: actor })?;
        match entity.permits(scope) {
            true => Ok(()),
            false => Err(ActionRefused::NotPermitted { entity: actor, scope }),
        }
    }

    /// Zone a tile as the sovereign.
    pub fn zone(&mut self, tile: u16, kind: ZoneKind) -> Result<(), ActionRefused> {
        self.zone_as(SOVEREIGN, tile, kind)
    }

    /// Zone a tile — the build SPENDS its cost from banked (02 §2.2);
    /// an unaffordable build refuses with the exact shortfall. The
    /// actor must hold Build scope; the receipt carries its tag.
    pub fn zone_as(&mut self, actor: u64, tile: u16, kind: ZoneKind) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let orb = self.orb.as_mut().ok_or(ActionRefused::NoColony)?;
        if orb.zones.iter().any(|zone| zone.tile == tile) {
            return Err(ActionRefused::TileAlreadyZoned { tile });
        }
        for (resource, cost) in kind.cost() {
            let have = orb.banked_amount(resource);
            if have < cost {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(cost),
                    have: Box::new(have),
                });
            }
        }
        for (resource, cost) in kind.cost() {
            orb.bank(resource, -cost);
        }
        orb.zones.push(Zone { tile, kind });
        orb.zones.sort_by_key(|zone| zone.tile);
        let orb_id = orb.orb_id;
        self.close(ClosureEvent::ZoneBuilt { orb_id, tile, kind }, actor);
        Ok(())
    }

    /// Raze as the sovereign.
    pub fn raze(&mut self, tile: u16) -> Result<(), ActionRefused> {
        self.raze_as(SOVEREIGN, tile)
    }

    /// Raze a zone for half its cost back (02 §2.4 — the way out), then
    /// check famine relief (the drain just shrank). Raze scope required.
    pub fn raze_as(&mut self, actor: u64, tile: u16) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Raze)?;
        let orb = self.orb.as_mut().ok_or(ActionRefused::NoColony)?;
        let at = orb
            .zones
            .iter()
            .position(|zone| zone.tile == tile)
            .ok_or(ActionRefused::NoSuchZone { tile })?;
        let zone = orb.zones.remove(at);
        for (resource, refund) in zone.kind.raze_refund() {
            orb.bank(resource, refund);
        }
        let orb_id = orb.orb_id;
        self.close(ClosureEvent::ZoneRazed { orb_id, tile }, actor);
        self.relieve_recovered();
        Ok(())
    }

    /// One simulation tick: active zones net yield − upkeep into the
    /// workspace; famine strikes where banked + accrued falls below
    /// zero; the clock walks by quorum. Famine is a closure — it
    /// chronicles; the per-tick flows do not (NS-3 §2.3).
    pub fn tick(&mut self) -> TickOutcome {
        let mut struck = Vec::new();
        if let Some(orb) = &self.orb {
            let active: Vec<Zone> = orb
                .zones
                .iter()
                .filter(|zone| orb.zone_active(zone))
                .cloned()
                .collect();
            for zone in &active {
                let (resource, amount) = zone.kind.yield_per_tick();
                accrue(&mut self.accrued, resource, amount);
                for (resource, amount) in zone.kind.upkeep_per_tick() {
                    accrue(&mut self.accrued, resource, -amount);
                }
            }
        }
        // The famine check (02 §2.4): trigger where banked + accrued < 0.
        if let Some(orb) = &mut self.orb {
            for resource in Resource::ALL {
                let accrued = ledger_amount(&self.accrued, resource);
                let banked = orb.banked_amount(resource);
                let solvent = banked.clone() + accrued >= ratio(0, 1);
                if !solvent && !orb.famine.contains(&resource) {
                    // The stores empty: banked drawn to zero, the
                    // workspace deficit cleared — there is nothing left
                    // to owe. Consumers go dormant.
                    orb.bank(resource, -banked.clone());
                    self.accrued.retain(|(held, _)| *held != resource);
                    orb.famine.push(resource);
                    orb.famine.sort();
                    let orb_id = orb.orb_id;
                    let event = ClosureEvent::FamineStruck {
                        orb_id,
                        resource,
                        drawn: banked,
                    };
                    let stamp = self.clock.now();
                    self.chronicle.append(event, stamp, vec![]);
                    struck.push(resource);
                }
            }
        }
        let attestations: [Attestation; 3] = [
            self.keepers[0].attest(&self.clock),
            self.keepers[1].attest(&self.clock),
            self.keepers[2].attest(&self.clock),
        ];
        TickOutcome {
            read: self.clock.tick(&attestations),
            struck,
        }
    }

    /// Collect as the sovereign.
    pub fn collect(&mut self) -> Result<usize, ActionRefused> {
        self.collect_as(SOVEREIGN)
    }

    /// Collect — settle the workspace against banked: positive nets
    /// bank (`YieldCollected`), negative nets draw (`UpkeepSettled`).
    /// Then check famine relief (the stores just moved). Collect scope
    /// required; every settle receipt carries the actor's tag.
    pub fn collect_as(&mut self, actor: u64) -> Result<usize, ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let orb = self.orb.as_mut().ok_or(ActionRefused::NoColony)?;
        let accrued = std::mem::take(&mut self.accrued);
        let settleable: Vec<(Resource, Ratio)> = accrued
            .into_iter()
            .filter(|(_, amount)| amount != &ratio(0, 1))
            .collect();
        if settleable.is_empty() {
            return Err(ActionRefused::NothingAccrued);
        }
        let orb_id = orb.orb_id;
        let stamp = self.clock.now();
        let count = settleable.len();
        for (resource, amount) in settleable {
            match amount >= ratio(0, 1) {
                true => {
                    orb.bank(resource, amount.clone());
                    self.chronicle.append(
                        ClosureEvent::YieldCollected { orb_id, resource, amount },
                        stamp,
                        vec![actor],
                    );
                }
                false => {
                    // The tick-time famine check guarantees banked
                    // covers this deficit (else famine already cleared
                    // the workspace).
                    let drawn = -amount;
                    orb.bank(resource, -drawn.clone());
                    self.chronicle.append(
                        ClosureEvent::UpkeepSettled {
                            orb_id,
                            resource,
                            amount: drawn,
                        },
                        stamp,
                        vec![actor],
                    );
                }
            }
        }
        self.relieve_recovered();
        Ok(count)
    }

    /// The committed projection of the live world.
    pub fn committed(&self) -> Committed {
        Committed {
            orb: self.orb.clone(),
            entities: self.entities.clone(),
        }
    }

    /// Famine relief (02 §2.4): a famined resource whose banked stock
    /// covers its dormant zones' ONE-TICK upkeep recovers; the zones
    /// wake. Checked after collect and raze — the two moves that
    /// restock or shrink the drain.
    fn relieve_recovered(&mut self) {
        let Some(orb) = self.orb.as_mut() else { return };
        let famined: Vec<Resource> = orb.famine.clone();
        for resource in famined {
            let one_tick: Ratio = orb
                .zones
                .iter()
                .flat_map(|zone| zone.kind.upkeep_per_tick())
                .filter(|(consumed, _)| *consumed == resource)
                .map(|(_, amount)| amount)
                .fold(ratio(0, 1), |sum, amount| sum + amount);
            if orb.banked_amount(resource) >= one_tick {
                orb.famine.retain(|held| *held != resource);
                let orb_id = orb.orb_id;
                let event = ClosureEvent::FamineRelieved { orb_id, resource };
                let stamp = self.clock.now();
                self.chronicle.append(event, stamp, vec![]);
            }
        }
    }

    /// Seal a closure with its actor's generation tag (NS-6): the
    /// receipt's provenance IS who forged it. World-drive closures
    /// (famine) carry no tag — no entity forged them.
    fn close(&mut self, event: ClosureEvent, actor: u64) {
        let stamp = self.clock.now();
        self.chronicle.append(event, stamp, vec![actor]);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Replay a chronicle into the committed projection it grounds
/// (CHARTER SEAL B). The ruleset constants (costs, reserves, refunds)
/// apply to the receipted events — replay reproduces the committed
/// world exactly, workspace excluded by construction.
pub fn replay(chronicle: &Chronicle) -> Committed {
    let mut orb: Option<ColonyOrb> = None;
    let mut entities: Vec<Entity> = Vec::new();
    for receipt in chronicle.receipts() {
        match &receipt.event {
            ClosureEvent::EntityRegistered { entity_id, principal, handle } => {
                entities.push(Entity {
                    entity_id: *entity_id,
                    handle: handle.clone(),
                    principal: match principal {
                        0 => None,
                        held => Some(*held),
                    },
                    grants: Vec::new(),
                });
            }
            ClosureEvent::ScopeGranted { agent, scope } => {
                if let Some(held) = entities.iter_mut().find(|held| held.entity_id == *agent) {
                    held.grants.push(*scope);
                    held.grants.sort();
                }
            }
            ClosureEvent::ScopeRevoked { agent, scope } => {
                if let Some(held) = entities.iter_mut().find(|held| held.entity_id == *agent) {
                    held.grants.retain(|granted| granted != scope);
                }
            }
            ClosureEvent::ColonyFounded { orb_id, name } => {
                let mut founded = ColonyOrb {
                    orb_id: *orb_id,
                    name: name.clone(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    founded.bank(resource, amount);
                }
                orb = Some(founded);
            }
            ClosureEvent::ZoneBuilt { tile, kind, .. } => {
                if let Some(orb) = orb.as_mut() {
                    for (resource, cost) in kind.cost() {
                        orb.bank(resource, -cost);
                    }
                    orb.zones.push(Zone { tile: *tile, kind: *kind });
                    orb.zones.sort_by_key(|zone| zone.tile);
                }
            }
            ClosureEvent::YieldCollected { resource, amount, .. } => {
                if let Some(orb) = orb.as_mut() {
                    orb.bank(*resource, amount.clone());
                }
            }
            ClosureEvent::UpkeepSettled { resource, amount, .. } => {
                if let Some(orb) = orb.as_mut() {
                    orb.bank(*resource, -amount.clone());
                }
            }
            ClosureEvent::FamineStruck { resource, drawn, .. } => {
                if let Some(orb) = orb.as_mut() {
                    orb.bank(*resource, -drawn.clone());
                    orb.famine.push(*resource);
                    orb.famine.sort();
                }
            }
            ClosureEvent::FamineRelieved { resource, .. } => {
                if let Some(orb) = orb.as_mut() {
                    orb.famine.retain(|held| held != resource);
                }
            }
            ClosureEvent::ZoneRazed { tile, .. } => {
                if let Some(orb) = orb.as_mut() {
                    if let Some(at) = orb.zones.iter().position(|zone| zone.tile == *tile) {
                        let zone = orb.zones.remove(at);
                        for (resource, refund) in zone.kind.raze_refund() {
                            orb.bank(resource, refund);
                        }
                    }
                }
            }
        }
    }
    Committed { orb, entities }
}

fn ledger_amount(ledger: &[(Resource, Ratio)], resource: Resource) -> Ratio {
    ledger
        .iter()
        .find(|(held, _)| *held == resource)
        .map(|(_, amount)| amount.clone())
        .unwrap_or_else(|| ratio(0, 1))
}

fn accrue(ledger: &mut Vec<(Resource, Ratio)>, resource: Resource, amount: Ratio) {
    match ledger.iter_mut().find(|(held, _)| *held == resource) {
        Some((_, held)) => *held += amount,
        None => {
            ledger.push((resource, amount));
            ledger.sort_by_key(|(resource, _)| *resource);
        }
    }
    // A zero net is not a datum: the knife-edge ledger stays canonical.
    ledger.retain(|(_, amount)| amount != &ratio(0, 1));
}
