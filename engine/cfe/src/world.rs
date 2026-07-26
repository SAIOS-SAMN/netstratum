//! The world — THE CONTAINER of the topology (04 §5): multiple colony
//! orbs (planets' settlements) under ONE clock, ONE chronicle, ONE
//! receipt chain (binding #1 — the world is the replay unit, the mesh
//! unit at N6, the anchor root at N7). The P1 landing paid the
//! registered naming debt by making the struct MEAN the container.
//!
//! The flow model (02 §2.3) runs PER ORB: each tick, every active zone
//! nets yield − upkeep into its orb's accrued workspace; famine strikes
//! per orb; `collect` settles one orb. Chartering (P1) founds a
//! daughter colony by MOVING the founding reserves from a source
//! colony's bank — conservation holds; no colony after the first gets
//! free matter.

use crate::dop::{
    founding_reserves, ClosureEvent, ColonyOrb, Resource, Zone, ZoneKind,
};
use crate::ipsal::{Entity, Scope};
use crate::ratio::{ratio, Ratio};
use crate::vrp::{Attestation, Chronicle, Timekeeper, TickRead, WorldClock};

/// The sovereign's entity id — the human at the terminal, registered
/// at founding.
pub const SOVEREIGN: u64 = 1;

/// The genesis orb id — the first colony.
pub const FIRST_ORB: u64 = 1;

/// A typed action refusal — the world refuses instead of guessing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionRefused {
    NoColony,
    ColonyAlreadyFounded,
    TileAlreadyZoned { tile: u16 },
    NoSuchZone { tile: u16 },
    CannotAfford { resource: Resource, need: Box<Ratio>, have: Box<Ratio> },
    NothingAccrued,
    NoSuchEntity { entity: u64 },
    HandleTaken { handle: String },
    NotPermitted { entity: u64, scope: Scope },
    NotAnAgent { entity: u64 },
    /// P1 — no orb holds this id.
    NoSuchOrb { orb: u64 },
    /// P1 — a planet name is unique within its world.
    PlanetNameTaken { name: String },
}

/// One tick's read: the clock walk, and any famines that struck
/// (per orb).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickOutcome {
    pub read: TickRead,
    pub struck: Vec<(u64, Resource)>,
}

/// The committed projection — exactly what the chronicle grounds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Committed {
    pub orbs: Vec<ColonyOrb>,
    pub entities: Vec<Entity>,
}

pub struct World {
    clock: WorldClock,
    keepers: [Timekeeper; 3],
    chronicle: Chronicle,
    orbs: Vec<ColonyOrb>,
    entities: Vec<Entity>,
    /// WORKSPACE, per orb: net flow since that orb's last collection —
    /// no receipt, no persistence, not committed.
    accrued: Vec<(u64, Resource, Ratio)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            clock: WorldClock::new(),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle: Chronicle::new(),
            orbs: Vec::new(),
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

    /// The genesis orb (convenience for the single-planet reads).
    pub fn orb(&self) -> Option<&ColonyOrb> {
        self.orbs.first()
    }

    pub fn orbs(&self) -> &[ColonyOrb] {
        &self.orbs
    }

    pub fn orb_by_id(&self, orb: u64) -> Option<&ColonyOrb> {
        self.orbs.iter().find(|held| held.orb_id == orb)
    }

    /// This orb's accrued workspace.
    pub fn accrued_of(&self, orb: u64) -> Vec<(Resource, Ratio)> {
        self.accrued
            .iter()
            .filter(|(held, _, _)| *held == orb)
            .map(|(_, resource, amount)| (*resource, amount.clone()))
            .collect()
    }

    /// The genesis orb's workspace (single-planet convenience).
    pub fn accrued(&self) -> Vec<(Resource, Ratio)> {
        self.accrued_of(FIRST_ORB)
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn entity(&self, entity: u64) -> Option<&Entity> {
        self.entities.iter().find(|held| held.entity_id == entity)
    }

    /// Found the GENESIS colony — banks the bootstrap reserves (02
    /// §2.1); registers the sovereign first. Later colonies arrive by
    /// `chart` — founding is once per world.
    pub fn found(&mut self, name: &str) -> Result<(), ActionRefused> {
        match self.orbs.is_empty() {
            false => Err(ActionRefused::ColonyAlreadyFounded),
            true => {
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
                let mut orb = ColonyOrb {
                    orb_id: FIRST_ORB,
                    name: name.to_string(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    orb.bank(resource, amount);
                }
                self.orbs.push(orb);
                self.close(
                    ClosureEvent::ColonyFounded {
                        orb_id: FIRST_ORB,
                        name: name.to_string(),
                    },
                    SOVEREIGN,
                );
                Ok(())
            }
        }
    }

    /// Chart a daughter colony as the sovereign.
    pub fn chart(&mut self, name: &str, source: u64) -> Result<u64, ActionRefused> {
        self.chart_as(SOVEREIGN, name, source)
    }

    /// P1 — chart a daughter colony on a new planet: the founding
    /// reserves MOVE from the source colony's bank to the daughter's.
    /// Conservation holds — chartering mints nothing. Build scope.
    pub fn chart_as(&mut self, actor: u64, name: &str, source: u64) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if self.orbs.iter().any(|held| held.name == name) {
            return Err(ActionRefused::PlanetNameTaken { name: name.to_string() });
        }
        let source_orb = self
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == source)
            .ok_or(ActionRefused::NoSuchOrb { orb: source })?;
        for (resource, amount) in founding_reserves() {
            let have = source_orb.banked_amount(resource);
            if have < amount {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(amount),
                    have: Box::new(have),
                });
            }
        }
        for (resource, amount) in founding_reserves() {
            source_orb.bank(resource, -amount);
        }
        let orb_id = self.orbs.iter().map(|held| held.orb_id).max().unwrap_or(0) + 1;
        let mut daughter = ColonyOrb {
            orb_id,
            name: name.to_string(),
            ..ColonyOrb::default()
        };
        for (resource, amount) in founding_reserves() {
            daughter.bank(resource, amount);
        }
        self.orbs.push(daughter);
        self.close(
            ClosureEvent::ColonyCharted {
                orb_id,
                source_orb: source,
                name: name.to_string(),
            },
            actor,
        );
        Ok(orb_id)
    }

    /// Register a scripted agent contained by the sovereign (N3).
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

    /// Revoke a grant — immediate; the agent's next such action refuses.
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

    fn permitted(&self, actor: u64, scope: Scope) -> Result<(), ActionRefused> {
        let entity = self
            .entity(actor)
            .ok_or(ActionRefused::NoSuchEntity { entity: actor })?;
        match entity.permits(scope) {
            true => Ok(()),
            false => Err(ActionRefused::NotPermitted { entity: actor, scope }),
        }
    }

    /// Zone on the genesis orb as the sovereign.
    pub fn zone(&mut self, tile: u16, kind: ZoneKind) -> Result<(), ActionRefused> {
        self.zone_as(SOVEREIGN, FIRST_ORB, tile, kind)
    }

    /// Zone a tile of one orb — the build SPENDS from that orb's bank
    /// (02 §2.2); Build scope; the receipt carries the actor's tag.
    pub fn zone_as(
        &mut self,
        actor: u64,
        orb: u64,
        tile: u16,
        kind: ZoneKind,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        let held = self
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.zones.iter().any(|zone| zone.tile == tile) {
            return Err(ActionRefused::TileAlreadyZoned { tile });
        }
        for (resource, cost) in kind.cost() {
            let have = held.banked_amount(resource);
            if have < cost {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(cost),
                    have: Box::new(have),
                });
            }
        }
        for (resource, cost) in kind.cost() {
            held.bank(resource, -cost);
        }
        held.zones.push(Zone { tile, kind });
        held.zones.sort_by_key(|zone| zone.tile);
        let orb_id = held.orb_id;
        self.close(ClosureEvent::ZoneBuilt { orb_id, tile, kind }, actor);
        Ok(())
    }

    /// Raze on the genesis orb as the sovereign.
    pub fn raze(&mut self, tile: u16) -> Result<(), ActionRefused> {
        self.raze_as(SOVEREIGN, FIRST_ORB, tile)
    }

    /// Raze a zone of one orb for half its cost back (02 §2.4); Raze
    /// scope; famine relief re-checked for that orb.
    pub fn raze_as(&mut self, actor: u64, orb: u64, tile: u16) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Raze)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        let held = self
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        let at = held
            .zones
            .iter()
            .position(|zone| zone.tile == tile)
            .ok_or(ActionRefused::NoSuchZone { tile })?;
        let zone = held.zones.remove(at);
        for (resource, refund) in zone.kind.raze_refund() {
            held.bank(resource, refund);
        }
        let orb_id = held.orb_id;
        self.close(ClosureEvent::ZoneRazed { orb_id, tile }, actor);
        self.relieve_recovered(orb);
        Ok(())
    }

    /// One simulation tick for the WHOLE world: every orb's active
    /// zones net their flows into that orb's workspace; famine strikes
    /// per orb; the one clock walks by quorum.
    pub fn tick(&mut self) -> TickOutcome {
        let mut struck = Vec::new();
        let flows: Vec<(u64, Vec<Zone>)> = self
            .orbs
            .iter()
            .map(|orb| {
                (
                    orb.orb_id,
                    orb.zones
                        .iter()
                        .filter(|zone| orb.zone_active(zone))
                        .cloned()
                        .collect(),
                )
            })
            .collect();
        for (orb_id, active) in &flows {
            for zone in active {
                let (resource, amount) = zone.kind.yield_per_tick();
                accrue(&mut self.accrued, *orb_id, resource, amount);
                for (resource, amount) in zone.kind.upkeep_per_tick() {
                    accrue(&mut self.accrued, *orb_id, resource, -amount);
                }
            }
        }
        // The famine check, per orb (02 §2.4).
        let stamp = self.clock.now();
        for orb in &mut self.orbs {
            for resource in Resource::ALL {
                let accrued = triple_amount(&self.accrued, orb.orb_id, resource);
                let banked = orb.banked_amount(resource);
                let solvent = banked.clone() + accrued >= ratio(0, 1);
                if !solvent && !orb.famine.contains(&resource) {
                    orb.bank(resource, -banked.clone());
                    self.accrued
                        .retain(|(held, res, _)| !(*held == orb.orb_id && *res == resource));
                    orb.famine.push(resource);
                    orb.famine.sort();
                    self.chronicle.append(
                        ClosureEvent::FamineStruck {
                            orb_id: orb.orb_id,
                            resource,
                            drawn: banked,
                        },
                        stamp,
                        vec![],
                    );
                    struck.push((orb.orb_id, resource));
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

    /// Collect the genesis orb as the sovereign.
    pub fn collect(&mut self) -> Result<usize, ActionRefused> {
        self.collect_as(SOVEREIGN, FIRST_ORB)
    }

    /// Collect ONE orb — settle its workspace against its bank;
    /// Collect scope; relief re-checked for that orb.
    pub fn collect_as(&mut self, actor: u64, orb: u64) -> Result<usize, ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        let held = self
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        let mut settleable: Vec<(Resource, Ratio)> = Vec::new();
        self.accrued.retain(|(owner, resource, amount)| {
            match *owner == orb && amount != &ratio(0, 1) {
                true => {
                    settleable.push((*resource, amount.clone()));
                    false
                }
                false => *owner != orb,
            }
        });
        if settleable.is_empty() {
            return Err(ActionRefused::NothingAccrued);
        }
        let orb_id = held.orb_id;
        let stamp = self.clock.now();
        let count = settleable.len();
        for (resource, amount) in settleable {
            match amount >= ratio(0, 1) {
                true => {
                    held.bank(resource, amount.clone());
                    self.chronicle.append(
                        ClosureEvent::YieldCollected { orb_id, resource, amount },
                        stamp,
                        vec![actor],
                    );
                }
                false => {
                    let drawn = -amount;
                    held.bank(resource, -drawn.clone());
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
        self.relieve_recovered(orb);
        Ok(count)
    }

    /// The committed projection of the live world.
    pub fn committed(&self) -> Committed {
        Committed {
            orbs: self.orbs.clone(),
            entities: self.entities.clone(),
        }
    }

    /// Famine relief for ONE orb (02 §2.4).
    fn relieve_recovered(&mut self, orb: u64) {
        let stamp = self.clock.now();
        let Some(held) = self.orbs.iter_mut().find(|held| held.orb_id == orb) else {
            return;
        };
        let famined: Vec<Resource> = held.famine.clone();
        for resource in famined {
            let one_tick: Ratio = held
                .zones
                .iter()
                .flat_map(|zone| zone.kind.upkeep_per_tick())
                .filter(|(consumed, _)| *consumed == resource)
                .map(|(_, amount)| amount)
                .fold(ratio(0, 1), |sum, amount| sum + amount);
            if held.banked_amount(resource) >= one_tick {
                held.famine.retain(|kept| *kept != resource);
                self.chronicle.append(
                    ClosureEvent::FamineRelieved {
                        orb_id: held.orb_id,
                        resource,
                    },
                    stamp,
                    vec![],
                );
            }
        }
    }

    /// Seal a closure with its actor's generation tag (NS-6).
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
/// (CHARTER SEAL B) — every orb, every entity, from receipts alone.
pub fn replay(chronicle: &Chronicle) -> Committed {
    let mut orbs: Vec<ColonyOrb> = Vec::new();
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
                orbs.push(founded);
            }
            ClosureEvent::ColonyCharted { orb_id, source_orb, name } => {
                if let Some(source) = orbs.iter_mut().find(|held| held.orb_id == *source_orb) {
                    for (resource, amount) in founding_reserves() {
                        source.bank(resource, -amount);
                    }
                }
                let mut daughter = ColonyOrb {
                    orb_id: *orb_id,
                    name: name.clone(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    daughter.bank(resource, amount);
                }
                orbs.push(daughter);
            }
            ClosureEvent::ZoneBuilt { orb_id, tile, kind } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    for (resource, cost) in kind.cost() {
                        held.bank(resource, -cost);
                    }
                    held.zones.push(Zone { tile: *tile, kind: *kind });
                    held.zones.sort_by_key(|zone| zone.tile);
                }
            }
            ClosureEvent::YieldCollected { orb_id, resource, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.bank(*resource, amount.clone());
                }
            }
            ClosureEvent::UpkeepSettled { orb_id, resource, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.bank(*resource, -amount.clone());
                }
            }
            ClosureEvent::FamineStruck { orb_id, resource, drawn } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.bank(*resource, -drawn.clone());
                    held.famine.push(*resource);
                    held.famine.sort();
                }
            }
            ClosureEvent::FamineRelieved { orb_id, resource } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.famine.retain(|kept| kept != resource);
                }
            }
            ClosureEvent::ZoneRazed { orb_id, tile } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    if let Some(at) = held.zones.iter().position(|zone| zone.tile == *tile) {
                        let zone = held.zones.remove(at);
                        for (resource, refund) in zone.kind.raze_refund() {
                            held.bank(resource, refund);
                        }
                    }
                }
            }
        }
    }
    Committed { orbs, entities }
}

fn triple_amount(ledger: &[(u64, Resource, Ratio)], orb: u64, resource: Resource) -> Ratio {
    ledger
        .iter()
        .find(|(held, res, _)| *held == orb && *res == resource)
        .map(|(_, _, amount)| amount.clone())
        .unwrap_or_else(|| ratio(0, 1))
}

fn accrue(ledger: &mut Vec<(u64, Resource, Ratio)>, orb: u64, resource: Resource, amount: Ratio) {
    match ledger
        .iter_mut()
        .find(|(held, res, _)| *held == orb && *res == resource)
    {
        Some((_, _, held)) => *held += amount,
        None => {
            ledger.push((orb, resource, amount));
            ledger.sort_by_key(|(orb, resource, _)| (*orb, *resource));
        }
    }
    ledger.retain(|(_, _, amount)| amount != &ratio(0, 1));
}
