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
    /// THERMAL-1 — one sun per world; it is already commissioned.
    StarAlreadyCommissioned,
    /// THERMAL-1 — siting an orbit needs a commissioned sun first.
    NoStarCommissioned,
    /// THERMAL-1 — the orb is already sited; an orbit does not move.
    AlreadySited { orb: u64 },
    /// THERMAL-1 — albedo must satisfy 0 ≤ A < 1, orbit r > 0.
    BadSiting,
    /// TRADE-1 — a transfer moves a POSITIVE amount between two
    /// DISTINCT orbs; anything else refuses whole.
    BadTransfer,
}

/// One tick's read: the clock walk, any famines that struck, and any
/// overheat transitions (per orb).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickOutcome {
    pub read: TickRead,
    pub struck: Vec<(u64, Resource)>,
    /// THERMAL-1: orbs whose overflow went positive this tick.
    pub overheat_struck: Vec<u64>,
    /// THERMAL-1: orbs whose overflow returned to zero this tick.
    pub overheat_cleared: Vec<u64>,
}

/// The committed projection — exactly what the chronicle grounds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Committed {
    pub orbs: Vec<ColonyOrb>,
    pub entities: Vec<Entity>,
    /// THERMAL-1: the commissioned sun, if any.
    pub star: Option<crate::thermal::StarClass>,
}

pub struct World {
    clock: WorldClock,
    keepers: [Timekeeper; 3],
    chronicle: Chronicle,
    orbs: Vec<ColonyOrb>,
    entities: Vec<Entity>,
    /// THERMAL-1: the container's sun (the world IS the star system,
    /// NS-6 §6); ABSENT until commissioned — thermal stays dormant.
    star: Option<crate::thermal::StarClass>,
    /// WORKSPACE, per orb: net flow since that orb's last collection —
    /// no receipt, no persistence, not committed.
    accrued: Vec<(u64, Resource, Ratio)>,
    /// VENT-1 WORKSPACE, per orb: vented entropy since last settle —
    /// sealed into the cumulative ledger by `VentSettled` at collect.
    vent_pending: Vec<(u64, Ratio)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            clock: WorldClock::new(),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle: Chronicle::new(),
            orbs: Vec::new(),
            entities: Vec::new(),
            star: None,
            accrued: Vec::new(),
            vent_pending: Vec::new(),
        }
    }

    /// P2-PERSIST — resume a world from its chronicle: the committed
    /// projection replays (orbs, entities), the clock reinstates at the
    /// highest stamped value the receipts prove, and the workspace
    /// starts EMPTY — accrued value and un-stamped ticks from the prior
    /// era had no receipts, so they honestly do not persist.
    pub fn resume(chronicle: Chronicle) -> World {
        let committed = replay(&chronicle);
        let stamp = chronicle
            .receipts()
            .iter()
            .map(|receipt| receipt.stamp)
            .max()
            .unwrap_or(0);
        World {
            clock: WorldClock::resume(stamp),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle,
            orbs: committed.orbs,
            entities: committed.entities,
            star: committed.star,
            accrued: Vec::new(),
            vent_pending: Vec::new(),
        }
    }

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
        let held = self
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.thermal.is_some() {
            return Err(ActionRefused::AlreadySited { orb });
        }
        held.thermal = Some(ThermalSite {
            albedo: albedo.clone(),
            orbit_r: orbit_r.clone(),
        });
        self.close(
            ClosureEvent::PlanetSited { orb_id: orb, albedo, orbit_r },
            SOVEREIGN,
        );
        Ok(())
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
    /// zones net their flows into that orb's workspace — through the
    /// heat law where thermal is live (05 §1.2: encase first, only the
    /// overflow vents, degrade by 1/(1+overflow)); famine strikes per
    /// orb; the one clock walks by quorum.
    pub fn tick(&mut self) -> TickOutcome {
        use crate::dop::{encasement_throughput, joule_leak_per_tick, radiator_base};
        use crate::thermal;
        let mut struck = Vec::new();
        let mut overheat_struck = Vec::new();
        let mut overheat_cleared = Vec::new();
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
        let one = ratio(1, 1);
        let zero = ratio(0, 1);
        let stamp_now = self.clock.now();
        for (orb_id, active) in &flows {
            // THERMAL-1: the heat law, live only when the sun is
            // commissioned AND this orb is sited (the opt-in guard).
            let holder = self.orbs.iter().find(|held| held.orb_id == *orb_id);
            let site = holder.and_then(|held| held.thermal.clone());
            let joule_famined = holder
                .map(|held| held.famine.contains(&Resource::Joule))
                .unwrap_or(false);
            let mut efficiency = one.clone();
            let mut t4: Option<Ratio> = None;
            let mut frozen = false;
            if let (Some(class), Some(site)) = (self.star, site) {
                let t_eq4 = thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
                // ECON-J freeze-lock (05 §1.2): a sub-mesic orb in joule
                // famine is frozen whole — no flows until fuel arrives.
                let sub_mesic = t_eq4 < thermal::SubstrateBand::Mesic.bounds4().0;
                frozen = sub_mesic && joule_famined;
                let stellar_load = t_eq4.clone() / thermal::thermal_k();
                let factory: Ratio = active
                    .iter()
                    .fold(zero.clone(), |sum, zone| sum + zone.kind.factory_heat());
                let heat_net = stellar_load + factory - radiator_base();
                // ECON-J (05 §1.2): heating upkeep on a sub-mesic orb —
                // every active zone burns joules against the cold.
                if sub_mesic && !frozen && !active.is_empty() {
                    let burn = crate::dop::heating_per_zone() * ratio(active.len() as i64, 1);
                    accrue(&mut self.accrued, *orb_id, Resource::Joule, -burn);
                }
                // SINK-1 (05 §1.4): vault-band headroom is captured as
                // bankable sink by active cryo-lattice arrays — the
                // infrastructure gate (research/throttle gates deferred).
                if !frozen && heat_net < zero && t_eq4 <= thermal::cryo_vault_max4() {
                    let arrays = active
                        .iter()
                        .filter(|zone| zone.kind == ZoneKind::CryoArray)
                        .count() as i64;
                    if arrays > 0 {
                        let headroom = -heat_net.clone();
                        let captured =
                            headroom.min(crate::dop::sink_throughput() * ratio(arrays, 1));
                        if captured > zero {
                            accrue(&mut self.accrued, *orb_id, Resource::Sink, captured);
                        }
                    }
                }
                if !frozen && heat_net > zero {
                    // The syntropic arm FIRST (05 §1.2): shells encase
                    // up to their throughput; joules accrue as yield.
                    let shells = active
                        .iter()
                        .filter(|zone| zone.kind == ZoneKind::Encasement)
                        .count() as i64;
                    let throughput = encasement_throughput() * ratio(shells, 1);
                    let encased = heat_net.clone().min(throughput);
                    if encased > zero {
                        accrue(&mut self.accrued, *orb_id, Resource::Joule, encased.clone());
                    }
                    // The entropic arm: only the overflow vents.
                    let overflow = heat_net - encased;
                    if overflow > zero {
                        efficiency = one.clone() / (one.clone() + overflow.clone());
                        // VENT-1: the loss accumulates in workspace,
                        // sealed by VentSettled at collect.
                        match self
                            .vent_pending
                            .iter_mut()
                            .find(|(held, _)| held == orb_id)
                        {
                            Some((_, pending)) => *pending += overflow.clone(),
                            None => self.vent_pending.push((*orb_id, overflow.clone())),
                        }
                        let held = self
                            .orbs
                            .iter_mut()
                            .find(|held| held.orb_id == *orb_id)
                            .expect("orb exists");
                        if !held.overheat {
                            held.overheat = true;
                            self.chronicle.append(
                                ClosureEvent::OverheatStruck {
                                    orb_id: *orb_id,
                                    overflow,
                                },
                                stamp_now,
                                vec![],
                            );
                            overheat_struck.push(*orb_id);
                        }
                    }
                }
                t4 = Some(t_eq4);
            }
            if efficiency == one {
                let held = self
                    .orbs
                    .iter_mut()
                    .find(|held| held.orb_id == *orb_id)
                    .expect("orb exists");
                if held.overheat {
                    held.overheat = false;
                    self.chronicle.append(
                        ClosureEvent::OverheatCleared { orb_id: *orb_id },
                        stamp_now,
                        vec![],
                    );
                    overheat_cleared.push(*orb_id);
                }
            }
            if !frozen {
                for zone in active {
                    let (resource, amount) = zone.kind.yield_per_tick();
                    accrue(
                        &mut self.accrued,
                        *orb_id,
                        resource,
                        amount * efficiency.clone(),
                    );
                    for (resource, amount) in zone.kind.upkeep_per_tick() {
                        accrue(&mut self.accrued, *orb_id, resource, -amount);
                    }
                }
            }
            // The cryo law (05 §1.2): accrued joules leak outside the
            // vault (T⁴ > 100⁴), floored at zero; lossless in the cold.
            if let Some(t4) = t4 {
                if t4 > thermal::cryo_vault_max4() {
                    let held_joules = triple_amount(&self.accrued, *orb_id, Resource::Joule);
                    if held_joules > zero {
                        let leak = held_joules.min(joule_leak_per_tick());
                        accrue(&mut self.accrued, *orb_id, Resource::Joule, -leak);
                    }
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
            overheat_struck,
            overheat_cleared,
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
        // SINK-1 (Q2): receipted demurrage at touch — un-vaulted banked
        // joules/sink decay for the elapsed span, settled openly.
        touch(held, self.star, &mut self.chronicle, stamp);
        // VENT-1: seal the pending vented entropy into the ledger.
        let mut vented = ratio(0, 1);
        self.vent_pending.retain(|(held_orb, pending)| match *held_orb == orb_id {
            true => {
                vented += pending.clone();
                false
            }
            false => true,
        });
        if vented > ratio(0, 1) {
            held.entropy_vented += vented.clone();
            self.chronicle.append(
                ClosureEvent::VentSettled { orb_id, amount: vented },
                stamp,
                vec![actor],
            );
        }
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

    /// Transfer banked value between two orbs as the sovereign.
    pub fn transfer(
        &mut self,
        from: u64,
        to: u64,
        resource: Resource,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        self.transfer_as(SOVEREIGN, from, to, resource, amount)
    }

    /// TRADE-1 (09 §2.1) — move BANKED value orb-to-orb within the
    /// world: the chartering conservation law generalized (nothing
    /// minted, nothing lost). Both sides settle due demurrage at the
    /// touch (the SINK-1 law) before value moves; affordability is
    /// judged on the post-decay bank; the compassion books post iff
    /// the counterparty stands in receipted, uncleared crisis at this
    /// stamp — a replay fact, never a judgment call. Collect scope;
    /// relief for the receiver re-checked at the landed law (02 §2.4).
    pub fn transfer_as(
        &mut self,
        actor: u64,
        from: u64,
        to: u64,
        resource: Resource,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if from == to || amount <= ratio(0, 1) {
            return Err(ActionRefused::BadTransfer);
        }
        if !self.orbs.iter().any(|held| held.orb_id == from) {
            return Err(ActionRefused::NoSuchOrb { orb: from });
        }
        if !self.orbs.iter().any(|held| held.orb_id == to) {
            return Err(ActionRefused::NoSuchOrb { orb: to });
        }
        let stamp = self.clock.now();
        let star = self.star;
        // Affordability on the post-decay bank — the books honest at
        // the stamp — computed pure, so a refusal orphans nothing.
        {
            let source = self
                .orbs
                .iter()
                .find(|held| held.orb_id == from)
                .expect("checked above");
            let decay = demurrage_due(source, star, stamp)
                .into_iter()
                .find(|(due, _)| *due == resource)
                .map(|(_, decay)| decay)
                .unwrap_or_else(|| ratio(0, 1));
            let have = source.banked_amount(resource) - decay;
            if have < amount {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(amount),
                    have: Box::new(have),
                });
            }
        }
        // Crisis reads BEFORE the move — the replay-fact order.
        let from_crisis = {
            let held = self.orbs.iter().find(|held| held.orb_id == from).expect("checked");
            !held.famine.is_empty() || held.overheat
        };
        let to_crisis = {
            let held = self.orbs.iter().find(|held| held.orb_id == to).expect("checked");
            !held.famine.is_empty() || held.overheat
        };
        // The touch, both sides (SINK-1), then the move.
        for orb in [from, to] {
            let held = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == orb)
                .expect("checked above");
            touch(held, star, &mut self.chronicle, stamp);
        }
        let value = amount.clone() * crate::dop::commensurate_weight(resource);
        {
            let source = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == from)
                .expect("checked above");
            source.bank(resource, -amount.clone());
            if to_crisis {
                source.given_in_crisis += value.clone();
            }
        }
        {
            let target = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == to)
                .expect("checked above");
            target.bank(resource, amount.clone());
            if from_crisis {
                target.drawn_in_crisis += value;
            }
        }
        self.chronicle.append(
            ClosureEvent::ResourceTransferred { from_orb: from, to_orb: to, resource, amount },
            stamp,
            vec![actor],
        );
        self.relieve_recovered(to);
        Ok(())
    }

    /// The committed projection of the live world.
    pub fn committed(&self) -> Committed {
        Committed {
            orbs: self.orbs.clone(),
            entities: self.entities.clone(),
            star: self.star,
        }
    }

    /// Famine relief for ONE orb (02 §2.4). Joule relief on a cold orb
    /// needs the one-tick HEATING bill covered (ECON-J), not zero.
    fn relieve_recovered(&mut self, orb: u64) {
        let stamp = self.clock.now();
        let star = self.star;
        let Some(held) = self.orbs.iter_mut().find(|held| held.orb_id == orb) else {
            return;
        };
        let heating_need: Ratio = match (star, &held.thermal) {
            (Some(class), Some(site)) => {
                let t4 =
                    crate::thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
                match t4 < crate::thermal::SubstrateBand::Mesic.bounds4().0 {
                    true => {
                        crate::dop::heating_per_zone() * ratio(held.zones.len() as i64, 1)
                    }
                    false => ratio(0, 1),
                }
            }
            _ => ratio(0, 1),
        };
        let famined: Vec<Resource> = held.famine.clone();
        for resource in famined {
            let mut one_tick: Ratio = held
                .zones
                .iter()
                .flat_map(|zone| zone.kind.upkeep_per_tick())
                .filter(|(consumed, _)| *consumed == resource)
                .map(|(_, amount)| amount)
                .fold(ratio(0, 1), |sum, amount| sum + amount);
            if resource == Resource::Joule {
                one_tick += heating_need.clone();
            }
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
    let mut star: Option<crate::thermal::StarClass> = None;
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
                    held.last_touch = receipt.stamp;
                }
            }
            ClosureEvent::UpkeepSettled { orb_id, resource, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.bank(*resource, -amount.clone());
                    // TRADE-1 truth-update: every settle is a touch —
                    // the demurrage anchor replays for upkeep-only
                    // collects too, exactly as the live path set it.
                    held.last_touch = receipt.stamp;
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
            // CONVERGE-1 (07 §1): a universe-ledger event — it never
            // enters a world chronicle, and world replay ignores it.
            ClosureEvent::EraAnnexed { .. } => {}
            // THERMAL-1 replay arms.
            ClosureEvent::StarCommissioned { class } => {
                star = crate::thermal::StarClass::from_wire(*class);
            }
            ClosureEvent::PlanetSited { orb_id, albedo, orbit_r } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.thermal = Some(crate::dop::ThermalSite {
                        albedo: albedo.clone(),
                        orbit_r: orbit_r.clone(),
                    });
                }
            }
            ClosureEvent::OverheatStruck { orb_id, .. } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.overheat = true;
                }
            }
            ClosureEvent::OverheatCleared { orb_id } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.overheat = false;
                }
            }
            // VENT-1: the cumulative ledger replays from settlements.
            ClosureEvent::VentSettled { orb_id, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.entropy_vented += amount.clone();
                    held.last_touch = receipt.stamp;
                }
            }
            // SINK-1: receipted demurrage replays as a bank draw.
            ClosureEvent::DemurrageSettled { orb_id, resource, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                }
            }
            // TRADE-1: the move replays with the SAME crisis reads the
            // live path made — the compassion books are a replay fact.
            ClosureEvent::ResourceTransferred { from_orb, to_orb, resource, amount } => {
                let from_crisis = orbs
                    .iter()
                    .find(|held| held.orb_id == *from_orb)
                    .map(|held| !held.famine.is_empty() || held.overheat)
                    .unwrap_or(false);
                let to_crisis = orbs
                    .iter()
                    .find(|held| held.orb_id == *to_orb)
                    .map(|held| !held.famine.is_empty() || held.overheat)
                    .unwrap_or(false);
                let value = amount.clone() * crate::dop::commensurate_weight(*resource);
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *from_orb) {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                    if to_crisis {
                        held.given_in_crisis += value.clone();
                    }
                }
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *to_orb) {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                    if from_crisis {
                        held.drawn_in_crisis += value;
                    }
                }
            }
        }
    }
    Committed { orbs, entities, star }
}

/// SINK-1 (Q2): the demurrage due on one orb's un-vaulted banked
/// joules/sink at `stamp` — computed PURE; the caller applies and
/// receipts (so a refused action never orphans a settlement).
fn demurrage_due(
    orb: &ColonyOrb,
    star: Option<crate::thermal::StarClass>,
    stamp: u64,
) -> Vec<(Resource, Ratio)> {
    let mut due = Vec::new();
    if let (Some(class), Some(site)) = (star, &orb.thermal) {
        let t4 = crate::thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
        if crate::thermal::leaks(&t4) && stamp > orb.last_touch {
            let elapsed = ratio((stamp - orb.last_touch) as i64, 1);
            for resource in [Resource::Joule, Resource::Sink] {
                let banked = orb.banked_amount(resource);
                if banked > ratio(0, 1) {
                    let decay = (banked.clone()
                        * crate::dop::demurrage_per_tick()
                        * elapsed.clone())
                    .min(banked);
                    due.push((resource, decay));
                }
            }
        }
    }
    due
}

/// The touch law (SINK-1, extended by TRADE-1): settle an orb's due
/// demurrage by receipt and anchor `last_touch` — decay or not, a
/// touch anchors.
fn touch(
    orb: &mut ColonyOrb,
    star: Option<crate::thermal::StarClass>,
    chronicle: &mut Chronicle,
    stamp: u64,
) {
    for (resource, decay) in demurrage_due(orb, star, stamp) {
        orb.bank(resource, -decay.clone());
        chronicle.append(
            ClosureEvent::DemurrageSettled { orb_id: orb.orb_id, resource, amount: decay },
            stamp,
            vec![],
        );
    }
    orb.last_touch = stamp;
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
