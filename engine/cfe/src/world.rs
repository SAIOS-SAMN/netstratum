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
    /// THRESHOLD-1 (12_UNION §7) — the shield holds: a people that has
    /// not made the solar leap cannot be reached by aperture, in
    /// either direction. The flags name which side stands shielded —
    /// the young are left to make the leap themselves, or not.
    Shielded { source_shielded: bool, dest_shielded: bool },
    /// ELEVATE-1 (12_UNION §6) — the climb refused, every gate's
    /// reading named: settle the books, close the loop, cross the
    /// void. Stagnation is a region, not a sentence — each false here
    /// is escapable.
    ElevationBarred { settled: bool, coherent: bool, crossed: bool },
    /// ELEVATE-1 — a climb moves STRICTLY upward in G (the orientation
    /// law: the fiber advances and never retreats).
    BadClimb,
    /// COUNCIL-1 — a council is three or more DISTINCT orbs, a
    /// positive quota and collateral, a real window, a fresh id.
    BadCouncil,
    /// PLANNER-1 — delegation settles inside diplomacy: the deed must
    /// reference an OPEN council in the granting world.
    NoSuchCouncil { council: u64 },
    /// PLANNER-1 — the solar leap is NEVER licensed: the young meet
    /// their sun themselves, or not (the shield's whole meaning).
    SolarLeapNeverLicensed,
    /// PLANNER-1 — a licensed leap lands only on a union member: the
    /// receiving world must have made its crossing (THRESHOLD-1).
    BelowTheThreshold,
    /// ABSORB-1 — only un-sealed pending overflow absorbs; the refusal
    /// names exactly what stands.
    CannotAbsorb { pending: Box<Ratio> },
    /// BOND-1 — one bond per (source, target, resource); the standing
    /// one must be severed before a new rate is sealed.
    BondAlreadySealed,
    /// BOND-1 — no bond stands on this (source, target, resource).
    NoSuchBond,
    /// COVENANT-1 — one OPEN covenant per (obligor, beneficiary,
    /// resource); a vitrified pact cannot be cancelled, only run out.
    CovenantAlreadySealed,
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

/// ABSORB-1 (09 §2.4): one open reciprocity window — the source of an
/// absorption owes the absorber its value within
/// `dop::reciprocity_window()` ticks of `opened`. Committed state:
/// opened by OverflowAbsorbed, retired oldest-first by TRADE-1
/// payments, lapsed by AbsorptionLapsed at the exact stamp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsorptionDebt {
    pub debtor: u64,
    pub creditor: u64,
    pub remainder: Ratio,
    pub opened: u64,
}

/// BOND-1 (09 §2.2): one standing order — source streams `rate` of
/// `resource` per tick to target, the span settled by ONE BondSettled
/// receipt at each touch (collect / sever / term expiry), anchored
/// like demurrage. Committed state; replays from receipts alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bond {
    pub source: u64,
    pub target: u64,
    pub resource: Resource,
    pub rate: Ratio,
    /// Term in ticks (0 = open-ended); expires ORDERLY at sealed+term.
    pub term: u64,
    pub sealed: u64,
    /// The settle anchor — the stamp the span is owed from.
    pub anchored: u64,
}

/// COVENANT-1 (01 §7, 09 §2.3): one vitrified delivery pact — the
/// obligor's `collateral` sits OUT of its bank (the lock gates every
/// spend by absence; the matrix is a lossless vault by construction)
/// until `delivered ≥ quota` releases it home or the passed deadline
/// slashes it to the beneficiary. Committed state; every mutation
/// receipted; replays from receipts alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Covenant {
    pub obligor: u64,
    pub beneficiary: u64,
    pub resource: Resource,
    pub quota: Ratio,
    /// Deliberate deliveries credited so far (transfers + bond flow).
    pub delivered: Ratio,
    pub collateral: Ratio,
    pub sealed: u64,
    pub window: u64,
}

/// COUNCIL-1 (12_UNION §7): a multilateral covenant — every member
/// bound alike, judged together at the boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Council {
    pub council: u64,
    pub members: Vec<u64>,
    pub resource: Resource,
    pub quota: Ratio,
    /// Per-member deliveries INTO the council (transfers and bond flow
    /// to fellow members), member-order matching `members`.
    pub delivered: Vec<Ratio>,
    /// The collateral each member locked (the same for all — bound
    /// alike).
    pub collateral: Ratio,
    pub sealed: u64,
    pub window: u64,
}

/// The committed projection — exactly what the chronicle grounds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Committed {
    pub orbs: Vec<ColonyOrb>,
    pub entities: Vec<Entity>,
    /// THERMAL-1: the commissioned sun, if any.
    pub star: Option<crate::thermal::StarClass>,
    /// ABSORB-1: the open reciprocity windows, in opening order.
    pub debts: Vec<AbsorptionDebt>,
    /// BOND-1: the standing orders, in sealing order.
    pub bonds: Vec<Bond>,
    /// COVENANT-1: the open vitrified pacts, in sealing order.
    pub covenants: Vec<Covenant>,
    /// COUNCIL-1: the open councils, in sealing order.
    pub councils: Vec<Council>,
    /// TECH-1a: the discovered nodes — (orb, condition, node id) in
    /// discovery order; condition 0 is the kernel solar leap.
    pub techs: Vec<(u64, u8, [u8; 32])>,
    /// THRESHOLD-1: has this world CROSSED — does its chronicle hold
    /// an aperture leg? A world is a star system, so any leg is an
    /// interstellar deed; the first is the crossing (12_UNION §7).
    pub crossed: bool,
    /// AEON-1a: the world's gauge key G — exact rational, wrapped on
    /// the torus fiber [0, 1). Zero at genesis; moves only by receipt.
    pub gauge: Ratio,
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
    /// sealed into the cumulative ledger by `VentSettled` at collect,
    /// or carried into a partner's sink by ABSORB-1.
    vent_pending: Vec<(u64, Ratio)>,
    /// ABSORB-1: the open reciprocity windows (committed — every
    /// mutation receipted; replays from receipts alone).
    debts: Vec<AbsorptionDebt>,
    /// BOND-1: the standing orders (committed, same law).
    bonds: Vec<Bond>,
    /// COVENANT-1: the open vitrified pacts (committed, same law).
    covenants: Vec<Covenant>,
    /// COUNCIL-1: the open councils (committed, same law).
    councils: Vec<Council>,
    /// IDEO-1 WORKSPACE, per orb: accrued ideological friction since
    /// the last seal — sealed by `FrictionSettled` at collect.
    friction_pending: Vec<(u64, Ratio)>,
    /// TECH-1a: the discovered nodes (committed — replays from
    /// receipts alone); condition 0 is the kernel solar leap.
    techs: Vec<(u64, u8, [u8; 32])>,
    /// THRESHOLD-1: the crossing flag (committed, replay-derived).
    crossed: bool,
    /// AEON-1a: the gauge key (committed, replay-derived, wrapped).
    gauge: Ratio,
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
            debts: Vec::new(),
            bonds: Vec::new(),
            covenants: Vec::new(),
            councils: Vec::new(),
            friction_pending: Vec::new(),
            techs: Vec::new(),
            crossed: false,
            gauge: ratio(0, 1),
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
            debts: committed.debts,
            bonds: committed.bonds,
            covenants: committed.covenants,
            councils: committed.councils,
            friction_pending: Vec::new(),
            techs: committed.techs,
            crossed: committed.crossed,
            gauge: committed.gauge,
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
            ClosureEvent::PlanetSited { orb_id: orb, albedo, orbit_r },
            SOVEREIGN,
        );
        Ok(())
    }

    /// TECH-1a: has this world's people made the SOLAR LEAP — do they
    /// know the powers of their sun? Pre-solar is a lawful state
    /// (12_UNION §5a); the reading gates licensing and the shield,
    /// never the world's own life.
    pub fn knows_its_sun(&self) -> bool {
        self.techs.iter().any(|(_, condition, _)| *condition == 0)
    }

    /// TECH-1a: the discovered nodes — (orb, condition, node id).
    pub fn techs(&self) -> &[(u64, u8, [u8; 32])] {
        &self.techs
    }

    /// THRESHOLD-1: has this world made the CROSSING — its first
    /// receipted aperture deed? Council eligibility reads here.
    pub fn has_crossed(&self) -> bool {
        self.crossed
    }

    /// THRESHOLD-1: a shielded world is one whose people have not made
    /// the solar leap — the Watchers' charge, reachable by no aperture.
    pub fn is_shielded(&self) -> bool {
        !self.knows_its_sun()
    }

    /// AEON-1a: the world's gauge key G — wrapped exact rational on
    /// the torus fiber. Zero at genesis ("G moves with the fiber
    /// landings"); every move is a receipt.
    pub fn gauge(&self) -> &Ratio {
        &self.gauge
    }

    /// AEON-1a (11 §2.1): move the key. The value wraps onto [0, 1) —
    /// the fiber is a circle, not a line; the receipted value is the
    /// WRAPPED one, so replay never re-derives the wrap.
    pub fn shift_gauge(&mut self, gauge: Ratio) {
        let wrapped = wrap_gauge(gauge);
        self.gauge = wrapped.clone();
        self.close(ClosureEvent::GaugeShifted { gauge: wrapped }, SOVEREIGN);
    }

    /// AEON-1a (11 §3): DIVERGENCE — this line cleaves; a peer branch
    /// departs under `branch`, and this world's own key moves by
    /// `delta` (the branch takes the opposite phase in ITS chronicle —
    /// annex, never merge; the caller founds it).
    pub fn diverge(&mut self, branch: &str, delta: Ratio) {
        let gauge = wrap_gauge(self.gauge.clone() + delta);
        self.gauge = gauge.clone();
        self.close(
            ClosureEvent::Diverged { branch: branch.into(), gauge },
            SOVEREIGN,
        );
    }

    /// AEON-1a (11 §3): CONFLUENCE — this line combines with a bonded
    /// partner; the key moves to the interference of both (the caller
    /// computes the torus midpoint from both chronicles and passes it;
    /// the receipt carries the wrapped result verbatim).
    pub fn conflue(&mut self, partner: &str, midpoint: Ratio) {
        let gauge = wrap_gauge(midpoint);
        self.gauge = gauge.clone();
        self.close(
            ClosureEvent::Confluent { partner: partner.into(), gauge },
            SOVEREIGN,
        );
    }

    /// COUNCIL-1 (12_UNION §7) — seal a COUNCIL: a multilateral
    /// covenant among three or more distinct orbs, every member bound
    /// alike. Each locks the SAME collateral (a bank withdrawal, the
    /// vitrified-vault law) against delivering ≥ quota to fellow
    /// members within the window; the reckoning judges all together at
    /// the boundary — the whole of the group over the needs of the
    /// individual. Any member unable to lock refuses the WHOLE seal:
    /// a council that starts unequal never starts.
    pub fn seal_council(
        &mut self,
        council: u64,
        members: Vec<u64>,
        resource: Resource,
        quota: Ratio,
        window: u64,
        collateral: Ratio,
    ) -> Result<(), ActionRefused> {
        let mut distinct = members.clone();
        distinct.sort_unstable();
        distinct.dedup();
        if distinct.len() < 3
            || distinct.len() != members.len()
            || quota <= ratio(0, 1)
            || collateral <= ratio(0, 1)
            || window == 0
            || self.councils.iter().any(|held| held.council == council)
        {
            return Err(ActionRefused::BadCouncil);
        }
        for member in &members {
            if !self.orbs.iter().any(|held| held.orb_id == *member) {
                return Err(ActionRefused::NoSuchOrb { orb: *member });
            }
        }
        // Every member must afford its lock BEFORE any lock happens —
        // a refusal moves nothing (the aperture discipline).
        for member in &members {
            let held = self
                .orbs
                .iter()
                .find(|held| held.orb_id == *member)
                .expect("checked");
            let have = held.banked_amount(resource);
            if have < collateral {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(collateral.clone()),
                    have: Box::new(have),
                });
            }
        }
        for member in &members {
            let held = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == *member)
                .expect("checked");
            held.bank(resource, -collateral.clone());
        }
        let stamp = self.clock.now();
        self.councils.push(Council {
            council,
            members: members.clone(),
            resource,
            quota: quota.clone(),
            delivered: vec![ratio(0, 1); members.len()],
            collateral: collateral.clone(),
            sealed: stamp,
            window,
        });
        self.close(
            ClosureEvent::CouncilSealed { council, resource, quota, window, collateral, members },
            SOVEREIGN,
        );
        Ok(())
    }

    /// COUNCIL-1: the open councils.
    pub fn councils(&self) -> &[Council] {
        &self.councils
    }

    /// COUNCIL-1 (12_UNION §7) — the WARRING IMPERIUM reading, derived
    /// from the landed books and never stored: an orb whose extraction
    /// runs past double its reciprocity while its breaches outweigh
    /// its fulfilments has stepped outside the council's protection by
    /// its own receipted conduct. Thresholds are placeholder
    /// parameters (the 06 discipline).
    pub fn is_imperial(&self, orb: u64) -> bool {
        let Some(held) = self.orbs.iter().find(|held| held.orb_id == orb) else {
            return false;
        };
        held.extracted_across > ratio(0, 1)
            && held.extracted_across > ratio(2, 1) * held.returned_across.clone()
            && held.covenants_breached > held.covenants_fulfilled
    }

    /// COUNCIL-1 (12_UNION §7) — the DIPLOMACY-FAILED reading: how
    /// many council reckonings this orb has breached, straight from
    /// the chronicle. The slash is the receipt that unbinds; until it
    /// exists, the council holds.
    pub fn council_breaches(&self, orb: u64) -> usize {
        self.chronicle
            .receipts()
            .iter()
            .filter(|receipt| {
                matches!(
                    &receipt.event,
                    ClosureEvent::CouncilClosed { member, fulfilled: false, .. }
                        if *member == orb
                )
            })
            .count()
    }

    /// ELEVATE-1 (12_UNION §6) — THE THREE-GATE GAUGE CLIMB, the
    /// discrete-gauge-morphodynamics ascent (Q3 resolved). An earned,
    /// receipted rise in G, executed as `GaugeShifted`, gated on three
    /// exact readings of this world's own chronicle:
    ///   S — thermodynamic settlement: no famine, no overheat, vent
    ///       workspace sealed, demurrage current on every orb;
    ///   T — loop coherence: the colony closed a loop within the last
    ///       year voice (60 ticks — 11 §1; placeholder constant);
    ///   G — the crossing: this world entered the union (THRESHOLD-1).
    /// The climb is monotone in G (the orientation law — time's arrow,
    /// not a queue): `delta` must be strictly positive. A refusal
    /// names every gate's reading — the cure, not a verdict.
    pub fn elevate(&mut self, delta: Ratio) -> Result<(), ActionRefused> {
        if delta <= ratio(0, 1) {
            return Err(ActionRefused::BadClimb);
        }
        let stamp = self.clock.now();
        let settled = self.orbs.iter().all(|held| {
            held.famine.is_empty()
                && !held.overheat
                && demurrage_due(held, self.star, stamp)
                    .iter()
                    .all(|(_, due)| *due == ratio(0, 1))
        }) && self
            .vent_pending
            .iter()
            .all(|(_, pending)| *pending == ratio(0, 1));
        let year_floor = stamp.saturating_sub(60);
        let coherent = self.chronicle.receipts().iter().any(|receipt| {
            receipt.stamp >= year_floor
                && matches!(receipt.event, ClosureEvent::YieldCollected { .. })
        });
        let crossed = self.crossed;
        if !(settled && coherent && crossed) {
            return Err(ActionRefused::ElevationBarred { settled, coherent, crossed });
        }
        let gauge = wrap_gauge(self.gauge.clone() + delta);
        self.gauge = gauge.clone();
        self.close(ClosureEvent::GaugeShifted { gauge }, SOVEREIGN);
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

    /// AEON-1a (11 §5): the FOLD — seal this world's part in the
    /// closing era and seat it on the new aeon's G-sheet (`base` is
    /// the advanced base gauge). Archival by construction: the receipt
    /// is the seal; rotation of the var root is the conductor's deed.
    pub fn seal_fold(&mut self, era: u64, base: Ratio) {
        let gauge = wrap_gauge(base);
        self.gauge = gauge.clone();
        self.close(ClosureEvent::EraFolded { era, gauge }, SOVEREIGN);
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

    /// ABSORB-1: this orb's un-sealed vented overflow (workspace) —
    /// what stands absorbable before the void claims it at collect.
    pub fn vent_pending_of(&self, orb: u64) -> Ratio {
        self.vent_pending
            .iter()
            .find(|(held, _)| *held == orb)
            .map(|(_, pending)| pending.clone())
            .unwrap_or_else(|| ratio(0, 1))
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
        // ABSORB-1 (09 §2.4): reciprocity windows expire at their
        // exact stamp — the unpaid remainder posts to the debtor's
        // extraction book, receipted (world-driven, the famine shape).
        {
            let now = self.clock.now();
            let window = crate::dop::reciprocity_window();
            let mut lapsed = Vec::new();
            self.debts.retain(|debt| match now >= debt.opened + window {
                true => {
                    lapsed.push(debt.clone());
                    false
                }
                false => true,
            });
            for debt in lapsed {
                if let Some(held) =
                    self.orbs.iter_mut().find(|held| held.orb_id == debt.debtor)
                {
                    held.extracted_across += debt.remainder.clone();
                }
                self.chronicle.append(
                    ClosureEvent::AbsorptionLapsed {
                        debtor_orb: debt.debtor,
                        creditor_orb: debt.creditor,
                        amount: debt.remainder,
                    },
                    now,
                    vec![],
                );
            }
            // BOND-1: term bonds expire ORDERLY at their exact
            // boundary — the final span settles, nothing posts.
            let expired: Vec<(u64, u64, Resource)> = self
                .bonds
                .iter()
                .filter(|bond| bond.term > 0 && now >= bond.sealed + bond.term)
                .map(|bond| (bond.source, bond.target, bond.resource))
                .collect();
            for (source, target, resource) in expired {
                self.settle_bond(source, target, resource, now, vec![], Some(ratio(0, 1)));
            }
            // COVENANT-1: the vitrified pacts resolve at the boundary —
            // fulfilment releases, the passed deadline slashes;
            // fulfilment wins the tie. World-driven, the famine shape.
            let due: Vec<(u64, u64, Resource, bool)> = self
                .covenants
                .iter()
                .filter_map(|pact| {
                    if pact.delivered >= pact.quota {
                        Some((pact.obligor, pact.beneficiary, pact.resource, true))
                    } else if now >= pact.sealed + pact.window {
                        Some((pact.obligor, pact.beneficiary, pact.resource, false))
                    } else {
                        None
                    }
                })
                .collect();
            for (obligor, beneficiary, resource, fulfilled) in due {
                let Some(at) = self.covenants.iter().position(|pact| {
                    pact.obligor == obligor
                        && pact.beneficiary == beneficiary
                        && pact.resource == resource
                }) else {
                    continue;
                };
                let pact = self.covenants.remove(at);
                let stake =
                    pact.collateral.clone() * crate::dop::commensurate_weight(resource);
                let recipient = match fulfilled {
                    true => obligor,
                    false => beneficiary,
                };
                if let Some(held) =
                    self.orbs.iter_mut().find(|held| held.orb_id == recipient)
                {
                    held.bank(resource, pact.collateral.clone());
                }
                if let Some(held) =
                    self.orbs.iter_mut().find(|held| held.orb_id == obligor)
                {
                    match fulfilled {
                        true => held.covenants_fulfilled += stake,
                        false => held.covenants_breached += stake,
                    }
                }
                let event = match fulfilled {
                    true => ClosureEvent::CovenantReleased {
                        obligor_orb: obligor,
                        beneficiary_orb: beneficiary,
                        resource,
                        collateral: pact.collateral,
                    },
                    false => ClosureEvent::CovenantSlashed {
                        obligor_orb: obligor,
                        beneficiary_orb: beneficiary,
                        resource,
                        collateral: pact.collateral,
                    },
                };
                self.chronicle.append(event, now, vec![]);
                self.relieve_recovered(recipient);
            }
            // COUNCIL-1: councils reckon at the BOUNDARY only — all
            // members judged together (the group over the individual;
            // no early release, no early slash). A breacher's
            // collateral divides exactly among the faithful; if every
            // member breached there is no aggrieved party — each
            // collateral goes home, the breaches still post.
            let boundary: Vec<u64> = self
                .councils
                .iter()
                .filter(|pact| now >= pact.sealed + pact.window)
                .map(|pact| pact.council)
                .collect();
            for id in boundary {
                let at = self
                    .councils
                    .iter()
                    .position(|pact| pact.council == id)
                    .expect("due above");
                let pact = self.councils.remove(at);
                let verdicts: Vec<bool> =
                    pact.delivered.iter().map(|given| *given >= pact.quota).collect();
                let faithful = verdicts.iter().filter(|kept| **kept).count() as i64;
                let breachers = pact.members.len() as i64 - faithful;
                let share = match faithful {
                    0 => ratio(0, 1),
                    _ => pact.collateral.clone() * ratio(breachers, 1) / ratio(faithful, 1),
                };
                let weight = crate::dop::commensurate_weight(pact.resource);
                for (seat, member) in pact.members.iter().enumerate() {
                    let fulfilled = verdicts[seat];
                    let banked = match (fulfilled, faithful) {
                        (true, _) => pact.collateral.clone() + share.clone(),
                        (false, 0) => pact.collateral.clone(),
                        (false, _) => ratio(0, 1),
                    };
                    let stake = pact.collateral.clone() * weight.clone();
                    if let Some(held) =
                        self.orbs.iter_mut().find(|held| held.orb_id == *member)
                    {
                        if banked > ratio(0, 1) {
                            held.bank(pact.resource, banked.clone());
                        }
                        match fulfilled {
                            true => held.covenants_fulfilled += stake,
                            false => held.covenants_breached += stake,
                        }
                    }
                    self.chronicle.append(
                        ClosureEvent::CouncilClosed {
                            council: id,
                            member: *member,
                            fulfilled,
                            banked,
                        },
                        now,
                        vec![],
                    );
                    if fulfilled {
                        self.relieve_recovered(*member);
                    }
                }
            }
        }
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
        // IDEO-1: the accrued ideological friction seals into the same
        // vented ledger — real entropy, its own receipt.
        let mut friction = ratio(0, 1);
        self.friction_pending.retain(|(held_orb, pending)| match *held_orb == orb_id {
            true => {
                friction += pending.clone();
                false
            }
            false => true,
        });
        if friction > ratio(0, 1) {
            held.entropy_vented += friction.clone();
            self.chronicle.append(
                ClosureEvent::FrictionSettled { orb_id, amount: friction },
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
        // BOND-1: the standing orders touching this orb settle their
        // accumulated spans — one receipt each (settle-at-touch).
        let touching: Vec<(u64, u64, Resource)> = self
            .bonds
            .iter()
            .filter(|bond| bond.source == orb || bond.target == orb)
            .map(|bond| (bond.source, bond.target, bond.resource))
            .collect();
        for (source, target, resource) in touching {
            self.settle_bond(source, target, resource, stamp, vec![actor], None);
        }
        self.relieve_recovered(orb);
        Ok(count)
    }

    /// BOND-1: the standing orders.
    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
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
        // IDEO-1 (09 §3.2): friction prices DIVERGENCE, not vice — the
        // handling overhead of unlike measured boundary policies, read
        // BEFORE the move, accrued as workspace against the mover and
        // sealed at its collect. Bonds and absorptions are exempt by
        // design: the standing loop is the friction-free channel.
        let divergence = {
            let reading = |orb: u64| {
                let held =
                    self.orbs.iter().find(|held| held.orb_id == orb).expect("checked");
                reciprocity_reading(held)
            };
            let from_reading = reading(from);
            let to_reading = reading(to);
            match from_reading >= to_reading {
                true => from_reading - to_reading,
                false => to_reading - from_reading,
            }
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
                target.drawn_in_crisis += value.clone();
            }
        }
        // ABSORB-1: the payment compensates open reciprocity windows
        // owed from→to, oldest first.
        settle_debts(&mut self.debts, &mut self.orbs, from, to, value.clone());
        // COVENANT-1: the delivery credits the open pact on this lane.
        credit_covenants(&mut self.covenants, from, to, resource, &amount);
        credit_councils(&mut self.councils, from, to, resource, &amount);
        let friction = crate::dop::friction_rate() * divergence * value;
        if friction > ratio(0, 1) {
            match self
                .friction_pending
                .iter_mut()
                .find(|(held, _)| *held == from)
            {
                Some((_, pending)) => *pending += friction,
                None => self.friction_pending.push((from, friction)),
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

    /// Seal a bond as the sovereign.
    pub fn seal_bond(
        &mut self,
        source: u64,
        target: u64,
        resource: Resource,
        rate: Ratio,
        term: u64,
    ) -> Result<(), ActionRefused> {
        self.seal_bond_as(SOVEREIGN, source, target, resource, rate, term)
    }

    /// BOND-1 (09 §2.2) — seal a standing order: source streams `rate`
    /// of `resource` per tick to target, non-transactional, settled by
    /// ONE receipt at each touch. One bond per (source, target,
    /// resource); `term` 0 is open-ended, otherwise the bond expires
    /// ORDERLY at sealed+term posting nothing. Collect scope.
    pub fn seal_bond_as(
        &mut self,
        actor: u64,
        source: u64,
        target: u64,
        resource: Resource,
        rate: Ratio,
        term: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if source == target || rate <= ratio(0, 1) {
            return Err(ActionRefused::BadTransfer);
        }
        if !self.orbs.iter().any(|held| held.orb_id == source) {
            return Err(ActionRefused::NoSuchOrb { orb: source });
        }
        if !self.orbs.iter().any(|held| held.orb_id == target) {
            return Err(ActionRefused::NoSuchOrb { orb: target });
        }
        if self.bonds.iter().any(|bond| {
            bond.source == source && bond.target == target && bond.resource == resource
        }) {
            return Err(ActionRefused::BondAlreadySealed);
        }
        let stamp = self.clock.now();
        self.bonds.push(Bond {
            source,
            target,
            resource,
            rate: rate.clone(),
            term,
            sealed: stamp,
            anchored: stamp,
        });
        self.chronicle.append(
            ClosureEvent::BondSealed {
                source_orb: source,
                target_orb: target,
                resource,
                rate,
                term,
            },
            stamp,
            vec![actor],
        );
        Ok(())
    }

    /// Sever a bond as the sovereign.
    pub fn sever_bond(
        &mut self,
        source: u64,
        target: u64,
        resource: Resource,
    ) -> Result<(), ActionRefused> {
        self.sever_bond_as(SOVEREIGN, source, target, resource)
    }

    /// BOND-1 — cut a standing bond (v1 ruling: the SOURCE side cuts;
    /// the load posts to it — the giver stopped giving without the
    /// notice). The final span settles in the same receipt; the
    /// severance load is `rate × bond_notice_window()`. Collect scope.
    pub fn sever_bond_as(
        &mut self,
        actor: u64,
        source: u64,
        target: u64,
        resource: Resource,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let Some(bond) = self.bonds.iter().find(|bond| {
            bond.source == source && bond.target == target && bond.resource == resource
        }) else {
            return Err(ActionRefused::NoSuchBond);
        };
        let load = bond.rate.clone() * ratio(crate::dop::bond_notice_window() as i64, 1);
        let stamp = self.clock.now();
        self.settle_bond(source, target, resource, stamp, vec![actor], Some(load));
        Ok(())
    }

    /// BOND-1 — settle one bond's accumulated span up to `stamp`
    /// (capped at the term boundary) by ONE BondSettled receipt.
    /// `ending: Some(load)` ends the bond deliberately (sever: rate ×
    /// notice; term expiry: zero). Insolvency — the span exceeding the
    /// source's post-decay bank — pays what stands and ends the bond
    /// as a receipted DEFAULT with the notice load. Bond flow posts
    /// the love book only (the crisis books belong to deliberate
    /// deeds); the payment still compensates open reciprocity windows.
    fn settle_bond(
        &mut self,
        source: u64,
        target: u64,
        resource: Resource,
        stamp: u64,
        provenance: Vec<u64>,
        ending: Option<Ratio>,
    ) {
        let Some(at) = self.bonds.iter().position(|bond| {
            bond.source == source && bond.target == target && bond.resource == resource
        }) else {
            return;
        };
        let (rate, term, sealed, anchored) = {
            let bond = &self.bonds[at];
            (bond.rate.clone(), bond.term, bond.sealed, bond.anchored)
        };
        let boundary = match term {
            0 => stamp,
            span => stamp.min(sealed + span),
        };
        let owed = rate.clone() * ratio(boundary.saturating_sub(anchored) as i64, 1);
        let star = self.star;
        let affordable = {
            let held = self
                .orbs
                .iter()
                .find(|held| held.orb_id == source)
                .expect("bond endpoints exist");
            let decay = demurrage_due(held, star, stamp)
                .into_iter()
                .find(|(due, _)| *due == resource)
                .map(|(_, decay)| decay)
                .unwrap_or_else(|| ratio(0, 1));
            held.banked_amount(resource) - decay
        };
        let pay = owed.clone().min(affordable);
        let insolvent = pay < owed;
        let ended = ending.is_some() || insolvent;
        // The caller's ruling wins (expiry: zero — the term was
        // served); an undirected insolvency is the mid-winter default.
        let load = match (&ending, insolvent) {
            (Some(load), _) => load.clone(),
            (None, true) => rate * ratio(crate::dop::bond_notice_window() as i64, 1),
            (None, false) => ratio(0, 1),
        };
        if pay == ratio(0, 1) && !ended {
            return; // nothing streamed, nothing ends — no receipt
        }
        let value = pay.clone() * crate::dop::commensurate_weight(resource);
        if pay > ratio(0, 1) {
            for orb in [source, target] {
                let held = self
                    .orbs
                    .iter_mut()
                    .find(|held| held.orb_id == orb)
                    .expect("bond endpoints exist");
                touch(held, star, &mut self.chronicle, stamp);
            }
            {
                let held = self
                    .orbs
                    .iter_mut()
                    .find(|held| held.orb_id == source)
                    .expect("bond endpoints exist");
                held.bank(resource, -pay.clone());
                held.bond_flow += value.clone();
            }
            {
                let held = self
                    .orbs
                    .iter_mut()
                    .find(|held| held.orb_id == target)
                    .expect("bond endpoints exist");
                held.bank(resource, pay.clone());
            }
            settle_debts(&mut self.debts, &mut self.orbs, source, target, value);
            // COVENANT-1: the stream credits the open pact on this lane.
            credit_covenants(&mut self.covenants, source, target, resource, &pay);
            credit_councils(&mut self.councils, source, target, resource, &pay);
        }
        if ended {
            self.bonds.remove(at);
            if load > ratio(0, 1) {
                let held = self
                    .orbs
                    .iter_mut()
                    .find(|held| held.orb_id == source)
                    .expect("bond endpoints exist");
                held.severance_load += load.clone();
            }
        } else {
            self.bonds[at].anchored = boundary;
        }
        self.chronicle.append(
            ClosureEvent::BondSettled {
                source_orb: source,
                target_orb: target,
                resource,
                amount: pay.clone(),
                severed: ended,
                load,
            },
            stamp,
            provenance,
        );
        if pay > ratio(0, 1) {
            self.relieve_recovered(target);
        }
    }

    /// Seal a covenant as the sovereign.
    pub fn seal_covenant(
        &mut self,
        obligor: u64,
        beneficiary: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
    ) -> Result<(), ActionRefused> {
        self.seal_covenant_as(SOVEREIGN, obligor, beneficiary, resource, quota, window)
    }

    /// COVENANT-1 (01 §7, 09 §2.3) — seal a vitrified delivery pact:
    /// the obligor locks collateral (the fidelity curve prices it —
    /// `dop::collateral_required`, the guardrail-6 ceiling holding)
    /// against delivering ≥ quota within the window. The lock is a
    /// bank withdrawal: every spend gate sees its absence, and the
    /// vitrified matrix vaults it losslessly by construction. Once
    /// sealed, the pact runs to release or slash — no cancel. One
    /// open pact per (obligor, beneficiary, resource). Collect scope.
    pub fn seal_covenant_as(
        &mut self,
        actor: u64,
        obligor: u64,
        beneficiary: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if obligor == beneficiary || quota <= ratio(0, 1) || window == 0 {
            return Err(ActionRefused::BadTransfer);
        }
        if !self.orbs.iter().any(|held| held.orb_id == obligor) {
            return Err(ActionRefused::NoSuchOrb { orb: obligor });
        }
        if !self.orbs.iter().any(|held| held.orb_id == beneficiary) {
            return Err(ActionRefused::NoSuchOrb { orb: beneficiary });
        }
        if self.covenants.iter().any(|held| {
            held.obligor == obligor
                && held.beneficiary == beneficiary
                && held.resource == resource
        }) {
            return Err(ActionRefused::CovenantAlreadySealed);
        }
        let stamp = self.clock.now();
        let collateral = {
            let held = self
                .orbs
                .iter()
                .find(|held| held.orb_id == obligor)
                .expect("checked above");
            let required = crate::dop::collateral_required(
                &held.covenants_fulfilled,
                &held.covenants_breached,
                &quota,
            );
            let have = held.banked_amount(resource);
            if have < required {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(required),
                    have: Box::new(have),
                });
            }
            required
        };
        {
            let held = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == obligor)
                .expect("checked above");
            held.bank(resource, -collateral.clone());
        }
        self.covenants.push(Covenant {
            obligor,
            beneficiary,
            resource,
            quota: quota.clone(),
            delivered: ratio(0, 1),
            collateral: collateral.clone(),
            sealed: stamp,
            window,
        });
        self.chronicle.append(
            ClosureEvent::CovenantSealed {
                obligor_orb: obligor,
                beneficiary_orb: beneficiary,
                resource,
                quota,
                window,
                collateral,
            },
            stamp,
            vec![actor],
        );
        Ok(())
    }

    /// COVENANT-1: the open vitrified pacts.
    pub fn covenants(&self) -> &[Covenant] {
        &self.covenants
    }

    /// Absorb as the sovereign.
    pub fn absorb(
        &mut self,
        source: u64,
        absorber: u64,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        self.absorb_as(SOVEREIGN, source, absorber, amount)
    }

    /// ABSORB-1 (05 §1.4, 09 §2.4) — route the source's un-sealed
    /// overflow into the absorber's banked sink instead of the void:
    /// `sink_consumed = heat_absorbed = vented_after_shed`, the
    /// absorber's ledger carrying the shed, the source's staying
    /// clean. Emptying the pending clears the strike (relief, the
    /// famine shape — the imbalance may strike anew next tick). The
    /// draw opens a reciprocity window owed source → absorber; TRADE-1
    /// payments retire it oldest-first; the unpaid remainder lapses to
    /// the extraction book. The absorber settles due demurrage at this
    /// touch; affordability judged on the post-decay sink, computed
    /// pure — a refusal orphans nothing. Collect scope.
    pub fn absorb_as(
        &mut self,
        actor: u64,
        source: u64,
        absorber: u64,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if source == absorber || amount <= ratio(0, 1) {
            return Err(ActionRefused::BadTransfer);
        }
        if !self.orbs.iter().any(|held| held.orb_id == source) {
            return Err(ActionRefused::NoSuchOrb { orb: source });
        }
        if !self.orbs.iter().any(|held| held.orb_id == absorber) {
            return Err(ActionRefused::NoSuchOrb { orb: absorber });
        }
        let stamp = self.clock.now();
        let star = self.star;
        let pending = self.vent_pending_of(source);
        if amount > pending {
            return Err(ActionRefused::CannotAbsorb { pending: Box::new(pending) });
        }
        {
            let held = self
                .orbs
                .iter()
                .find(|held| held.orb_id == absorber)
                .expect("checked above");
            let decay = demurrage_due(held, star, stamp)
                .into_iter()
                .find(|(due, _)| *due == Resource::Sink)
                .map(|(_, decay)| decay)
                .unwrap_or_else(|| ratio(0, 1));
            let have = held.banked_amount(Resource::Sink) - decay;
            if have < amount {
                return Err(ActionRefused::CannotAfford {
                    resource: Resource::Sink,
                    need: Box::new(amount),
                    have: Box::new(have),
                });
            }
        }
        // Crisis reads BEFORE the move — the replay-fact order.
        let source_crisis = {
            let held = self.orbs.iter().find(|held| held.orb_id == source).expect("checked");
            !held.famine.is_empty() || held.overheat
        };
        let absorber_crisis = {
            let held = self.orbs.iter().find(|held| held.orb_id == absorber).expect("checked");
            !held.famine.is_empty() || held.overheat
        };
        let value = amount.clone() * crate::dop::commensurate_weight(Resource::Sink);
        {
            let held = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == absorber)
                .expect("checked above");
            touch(held, star, &mut self.chronicle, stamp);
            held.bank(Resource::Sink, -amount.clone());
            held.entropy_vented += amount.clone();
            if source_crisis {
                held.given_in_crisis += value.clone();
            }
        }
        if let Some((_, held)) = self
            .vent_pending
            .iter_mut()
            .find(|(held, _)| *held == source)
        {
            *held -= amount.clone();
        }
        self.vent_pending.retain(|(_, pending)| pending != &ratio(0, 1));
        if absorber_crisis {
            let held = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == source)
                .expect("checked above");
            held.drawn_in_crisis += value;
        }
        self.debts.push(AbsorptionDebt {
            debtor: source,
            creditor: absorber,
            remainder: amount.clone(),
            opened: stamp,
        });
        self.chronicle.append(
            ClosureEvent::OverflowAbsorbed { source_orb: source, absorber_orb: absorber, amount },
            stamp,
            vec![actor],
        );
        // Emptying the pending relieves the strike, receipted.
        if self.vent_pending_of(source) == ratio(0, 1) {
            let held = self
                .orbs
                .iter_mut()
                .find(|held| held.orb_id == source)
                .expect("checked above");
            if held.overheat {
                held.overheat = false;
                self.chronicle.append(
                    ClosureEvent::OverheatCleared { orb_id: source },
                    stamp,
                    vec![],
                );
            }
        }
        Ok(())
    }

    /// The committed projection of the live world.
    pub fn committed(&self) -> Committed {
        Committed {
            orbs: self.orbs.clone(),
            entities: self.entities.clone(),
            star: self.star,
            debts: self.debts.clone(),
            bonds: self.bonds.clone(),
            covenants: self.covenants.clone(),
            councils: self.councils.clone(),
            techs: self.techs.clone(),
            crossed: self.crossed,
            gauge: self.gauge.clone(),
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

/// NS-4 APERTURE (BUILD_ORDER §3a) — THE CROSS-WORLD VALUE CARRIER.
/// Banked value crosses from an orb in ONE world to an orb in ANOTHER,
/// conservation-exact across the pair, receipted in BOTH chronicles as
/// a two-legged deed sharing one `deal` id: `ApertureSent` debits the
/// source, `ApertureReceived` credits the destination. Unlike TRADE-1
/// (one chronicle proves conservation), no single chronicle proves it
/// here — the deal id links the legs (double entry across two books).
/// Both orbs settle due demurrage at the touch (the SINK-1 law) before
/// value moves; affordability is judged on the source's post-decay
/// bank, computed pure, so a refusal orphans nothing in either world.
/// The cross-empire cohesion books (compassion/morality) DO NOT post
/// here: cross-world reading composes at N5/N6 (09 board law) — an
/// aperture carries value, never judgment.
pub fn aperture_transfer(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    resource: Resource,
    amount: Ratio,
    deal: u64,
) -> Result<(), ActionRefused> {
    if amount <= ratio(0, 1) {
        return Err(ActionRefused::BadTransfer);
    }
    // THRESHOLD-1 (12_UNION §7): the shield — an aperture touches only
    // worlds whose peoples have made the solar leap (TECH-1a). A world
    // is a star system, so every aperture IS interstellar; the first
    // lawful one is a world's CROSSING into the union.
    if !src.knows_its_sun() || !dst.knows_its_sun() {
        return Err(ActionRefused::Shielded {
            source_shielded: !src.knows_its_sun(),
            dest_shielded: !dst.knows_its_sun(),
        });
    }
    if !src.orbs.iter().any(|held| held.orb_id == from) {
        return Err(ActionRefused::NoSuchOrb { orb: from });
    }
    if !dst.orbs.iter().any(|held| held.orb_id == to) {
        return Err(ActionRefused::NoSuchOrb { orb: to });
    }
    let src_stamp = src.clock.now();
    let dst_stamp = dst.clock.now();
    // Affordability on the source's post-decay bank — honest at the
    // stamp, computed pure, so a refusal moves nothing in either world.
    {
        let source = src.orbs.iter().find(|held| held.orb_id == from).expect("checked");
        let decay = demurrage_due(source, src.star, src_stamp)
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
    // The touch (SINK-1) then the debit, in the source world.
    {
        let source = src.orbs.iter_mut().find(|held| held.orb_id == from).expect("checked");
        touch(source, src.star, &mut src.chronicle, src_stamp);
        source.bank(resource, -amount.clone());
    }
    src.chronicle.append(
        ClosureEvent::ApertureSent { deal, from_orb: from, resource, amount: amount.clone() },
        src_stamp,
        vec![SOVEREIGN],
    );
    // The touch then the credit, in the destination world.
    {
        let target = dst.orbs.iter_mut().find(|held| held.orb_id == to).expect("checked");
        touch(target, dst.star, &mut dst.chronicle, dst_stamp);
        target.bank(resource, amount.clone());
    }
    dst.chronicle.append(
        ClosureEvent::ApertureReceived { deal, to_orb: to, resource, amount },
        dst_stamp,
        vec![SOVEREIGN],
    );
    dst.relieve_recovered(to);
    // THRESHOLD-1: the deed IS the crossing, for both worlds.
    src.crossed = true;
    dst.crossed = true;
    Ok(())
}

/// PLANNER-1: what a licensed leap carries — the condition and name
/// of the technology, the deal id binding both legs, and the open
/// council the delegation settles inside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leap {
    pub condition: u8,
    pub name: String,
    pub deal: u64,
    pub council: u64,
}

/// PLANNER-1 (12_UNION §2) — LICENSE A TECHNOLOGY LEAP into another
/// world: the planner's issuing power, receipted in BOTH chronicles
/// under one deal (the aperture shape). The deed is council-gated —
/// it must reference an OPEN council in the granting world (delegation
/// settles inside diplomacy) — and shield-bounded: the receiving world
/// must know its sun AND have crossed (a licensed leap lands only on a
/// union member; below the threshold the young are left to their sky).
/// The solar leap (condition 0) is NEVER licensed. The leap may be
/// known or UNKNOWN to the receiver — the name and node arrive with
/// the deed; a node the receiver already holds refuses idempotent.
pub fn license_tech(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    leap: Leap,
) -> Result<(), ActionRefused> {
    let Leap { condition, name, deal, council } = leap;
    if condition == 0 {
        return Err(ActionRefused::SolarLeapNeverLicensed);
    }
    if !src.councils.iter().any(|pact| pact.council == council) {
        return Err(ActionRefused::NoSuchCouncil { council });
    }
    if dst.is_shielded() {
        return Err(ActionRefused::Shielded {
            source_shielded: src.is_shielded(),
            dest_shielded: true,
        });
    }
    if !dst.crossed {
        return Err(ActionRefused::BelowTheThreshold);
    }
    if !src.orbs.iter().any(|held| held.orb_id == from) {
        return Err(ActionRefused::NoSuchOrb { orb: from });
    }
    if !dst.orbs.iter().any(|held| held.orb_id == to) {
        return Err(ActionRefused::NoSuchOrb { orb: to });
    }
    // The node id is seeded from the licensing deed (SEAL B): the
    // granting chronicle's own digest never enters — the deal and
    // condition suffice, so both legs derive the SAME node.
    let mut seed = b"tech.licensed".to_vec();
    seed.extend_from_slice(&deal.to_le_bytes());
    seed.push(condition);
    let node: [u8; 32] = blake3::hash(&seed).into();
    if dst.techs.iter().any(|(_, _, held)| *held == node) {
        return Err(ActionRefused::BadTransfer);
    }
    let src_stamp = src.clock.now();
    let dst_stamp = dst.clock.now();
    src.chronicle.append(
        ClosureEvent::TechLicensed {
            deal,
            from_orb: from,
            council,
            condition,
            node,
            name: name.clone(),
        },
        src_stamp,
        vec![SOVEREIGN],
    );
    dst.techs.push((to, condition, node));
    dst.chronicle.append(
        ClosureEvent::TechDiscovered {
            orb_id: to,
            node,
            condition,
            modifier: 0,
            magnitude: ratio(1, 1),
            name,
        },
        dst_stamp,
        vec![SOVEREIGN],
    );
    Ok(())
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Replay a chronicle into the committed projection it grounds
/// (CHARTER SEAL B) — every orb, every entity, from receipts alone.
/// AEON-1a: wrap a gauge value onto the torus fiber [0, 1) — exact
/// rational modulo 1, no floats, no drift.
pub fn wrap_gauge(gauge: Ratio) -> Ratio {
    gauge.clone() - gauge.floor()
}

pub fn replay(chronicle: &Chronicle) -> Committed {
    let mut orbs: Vec<ColonyOrb> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();
    let mut star: Option<crate::thermal::StarClass> = None;
    let mut debts: Vec<AbsorptionDebt> = Vec::new();
    let mut bonds: Vec<Bond> = Vec::new();
    let mut covenants: Vec<Covenant> = Vec::new();
    let mut councils: Vec<Council> = Vec::new();
    let mut techs: Vec<(u64, u8, [u8; 32])> = Vec::new();
    let mut crossed = false;
    let mut gauge = ratio(0, 1);
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
                        held.drawn_in_crisis += value.clone();
                    }
                }
                settle_debts(&mut debts, &mut orbs, *from_orb, *to_orb, value);
                credit_covenants(&mut covenants, *from_orb, *to_orb, *resource, amount);
                credit_councils(&mut councils, *from_orb, *to_orb, *resource, amount);
            }
            // NS-4 APERTURE: the source leg replays as a bank draw —
            // value left THIS world for another; last_touch anchors at
            // the deal stamp (the touch settled demurrage separately).
            ClosureEvent::ApertureSent { from_orb, resource, amount, .. } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *from_orb) {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                }
                crossed = true;
            }
            // NS-4 APERTURE: the destination leg replays as a bank
            // credit — value arrived from another world.
            ClosureEvent::ApertureReceived { to_orb, resource, amount, .. } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *to_orb) {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                }
                crossed = true;
            }
            // ABSORB-1: the three-way move replays — the absorber's
            // ledger carries the shed; the window opens; crisis reads
            // mirror the live order exactly.
            ClosureEvent::OverflowAbsorbed { source_orb, absorber_orb, amount } => {
                let source_crisis = orbs
                    .iter()
                    .find(|held| held.orb_id == *source_orb)
                    .map(|held| !held.famine.is_empty() || held.overheat)
                    .unwrap_or(false);
                let absorber_crisis = orbs
                    .iter()
                    .find(|held| held.orb_id == *absorber_orb)
                    .map(|held| !held.famine.is_empty() || held.overheat)
                    .unwrap_or(false);
                let value =
                    amount.clone() * crate::dop::commensurate_weight(Resource::Sink);
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *absorber_orb)
                {
                    held.bank(Resource::Sink, -amount.clone());
                    held.entropy_vented += amount.clone();
                    held.last_touch = receipt.stamp;
                    if source_crisis {
                        held.given_in_crisis += value.clone();
                    }
                }
                if absorber_crisis {
                    if let Some(held) =
                        orbs.iter_mut().find(|held| held.orb_id == *source_orb)
                    {
                        held.drawn_in_crisis += value;
                    }
                }
                debts.push(AbsorptionDebt {
                    debtor: *source_orb,
                    creditor: *absorber_orb,
                    remainder: amount.clone(),
                    opened: receipt.stamp,
                });
            }
            // ABSORB-1: the lapse replays — extraction posts, the
            // oldest matching window drops (live emission order).
            ClosureEvent::AbsorptionLapsed { debtor_orb, creditor_orb, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *debtor_orb) {
                    held.extracted_across += amount.clone();
                }
                if let Some(at) = debts.iter().position(|debt| {
                    debt.debtor == *debtor_orb
                        && debt.creditor == *creditor_orb
                        && debt.remainder == *amount
                }) {
                    debts.remove(at);
                }
            }
            // BOND-1: the standing order replays whole.
            ClosureEvent::BondSealed { source_orb, target_orb, resource, rate, term } => {
                bonds.push(Bond {
                    source: *source_orb,
                    target: *target_orb,
                    resource: *resource,
                    rate: rate.clone(),
                    term: *term,
                    sealed: receipt.stamp,
                    anchored: receipt.stamp,
                });
            }
            // BOND-1: one settle receipt moves the span, posts the
            // love book, compensates open windows, and — severed —
            // ends the bond with its load. Non-ending settles re-
            // anchor at the stamp (live boundary == stamp, proven at
            // the landing).
            ClosureEvent::BondSettled { source_orb, target_orb, resource, amount, severed, load } => {
                let value = amount.clone() * crate::dop::commensurate_weight(*resource);
                if *amount > ratio(0, 1) {
                    if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *source_orb)
                    {
                        held.bank(*resource, -amount.clone());
                        held.last_touch = receipt.stamp;
                        held.bond_flow += value.clone();
                    }
                    if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *target_orb)
                    {
                        held.bank(*resource, amount.clone());
                        held.last_touch = receipt.stamp;
                    }
                    settle_debts(&mut debts, &mut orbs, *source_orb, *target_orb, value);
                    credit_covenants(&mut covenants, *source_orb, *target_orb, *resource, amount);
                    credit_councils(&mut councils, *source_orb, *target_orb, *resource, amount);
                }
                match severed {
                    true => {
                        if let Some(at) = bonds.iter().position(|bond| {
                            bond.source == *source_orb
                                && bond.target == *target_orb
                                && bond.resource == *resource
                        }) {
                            bonds.remove(at);
                        }
                        if *load > ratio(0, 1) {
                            if let Some(held) =
                                orbs.iter_mut().find(|held| held.orb_id == *source_orb)
                            {
                                held.severance_load += load.clone();
                            }
                        }
                    }
                    false => {
                        if let Some(bond) = bonds.iter_mut().find(|bond| {
                            bond.source == *source_orb
                                && bond.target == *target_orb
                                && bond.resource == *resource
                        }) {
                            bond.anchored = receipt.stamp;
                        }
                    }
                }
            }
            // COVENANT-1: the pact replays whole — the seal receipt
            // carries the priced collateral (the curve never re-runs).
            ClosureEvent::CovenantSealed {
                obligor_orb,
                beneficiary_orb,
                resource,
                quota,
                window,
                collateral,
            } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *obligor_orb) {
                    held.bank(*resource, -collateral.clone());
                }
                covenants.push(Covenant {
                    obligor: *obligor_orb,
                    beneficiary: *beneficiary_orb,
                    resource: *resource,
                    quota: quota.clone(),
                    delivered: ratio(0, 1),
                    collateral: collateral.clone(),
                    sealed: receipt.stamp,
                    window: *window,
                });
            }
            ClosureEvent::CovenantReleased {
                obligor_orb, beneficiary_orb, resource, collateral,
            } => {
                let stake = collateral.clone() * crate::dop::commensurate_weight(*resource);
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *obligor_orb) {
                    held.bank(*resource, collateral.clone());
                    held.covenants_fulfilled += stake;
                }
                if let Some(at) = covenants.iter().position(|pact| {
                    pact.obligor == *obligor_orb
                        && pact.beneficiary == *beneficiary_orb
                        && pact.resource == *resource
                }) {
                    covenants.remove(at);
                }
            }
            ClosureEvent::CovenantSlashed {
                obligor_orb, beneficiary_orb, resource, collateral,
            } => {
                let stake = collateral.clone() * crate::dop::commensurate_weight(*resource);
                if let Some(held) =
                    orbs.iter_mut().find(|held| held.orb_id == *beneficiary_orb)
                {
                    held.bank(*resource, collateral.clone());
                }
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *obligor_orb) {
                    held.covenants_breached += stake;
                }
                if let Some(at) = covenants.iter().position(|pact| {
                    pact.obligor == *obligor_orb
                        && pact.beneficiary == *beneficiary_orb
                        && pact.resource == *resource
                }) {
                    covenants.remove(at);
                }
            }
            // IDEO-1: sealed friction replays into the vented ledger.
            ClosureEvent::FrictionSettled { orb_id, amount } => {
                if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                    held.entropy_vented += amount.clone();
                    held.last_touch = receipt.stamp;
                }
            }
            // TECH-1a: a discovered node replays into the tech list —
            // the solar leap (condition 0) among them.
            ClosureEvent::TechDiscovered { orb_id, node, condition, .. } => {
                techs.push((*orb_id, *condition, *node));
            }
            // COUNCIL-1: the seal replays whole — every member's lock
            // withdraws; deliveries re-credit through the transfer and
            // bond arms above.
            ClosureEvent::CouncilSealed {
                council, resource, quota, window, collateral, members,
            } => {
                for member in members {
                    if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *member) {
                        held.bank(*resource, -collateral.clone());
                    }
                }
                councils.push(Council {
                    council: *council,
                    members: members.clone(),
                    resource: *resource,
                    quota: quota.clone(),
                    delivered: vec![ratio(0, 1); members.len()],
                    collateral: collateral.clone(),
                    sealed: receipt.stamp,
                    window: *window,
                });
            }
            // COUNCIL-1: each member's reckoning banks exactly what
            // the receipt carries; the integrity stake reads from the
            // sealed council; the council leaves state when its last
            // member has closed.
            ClosureEvent::CouncilClosed { council, member, fulfilled, banked } => {
                if let Some(at) = councils.iter().position(|pact| pact.council == *council) {
                    let stake = councils[at].collateral.clone()
                        * crate::dop::commensurate_weight(councils[at].resource);
                    let resource = councils[at].resource;
                    if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *member) {
                        if *banked > ratio(0, 1) {
                            held.bank(resource, banked.clone());
                        }
                        match fulfilled {
                            true => held.covenants_fulfilled += stake,
                            false => held.covenants_breached += stake,
                        }
                    }
                    if let Some(seat) =
                        councils[at].members.iter().position(|held| held == member)
                    {
                        councils[at].members.remove(seat);
                        councils[at].delivered.remove(seat);
                    }
                    if councils[at].members.is_empty() {
                        councils.remove(at);
                    }
                }
            }
            // PLANNER-1: the issuing deed is a pure record src-side —
            // the far world's TechDiscovered leg carries the arrival.
            ClosureEvent::TechLicensed { .. } => {}
            // AEON-1a: every key move replays verbatim — the receipt
            // carries the wrapped value, the last one stands.
            ClosureEvent::GaugeShifted { gauge: key }
            | ClosureEvent::Diverged { gauge: key, .. }
            | ClosureEvent::Confluent { gauge: key, .. }
            | ClosureEvent::EraFolded { gauge: key, .. } => {
                gauge = key.clone();
            }
            // AEON-1a: the star moved in its region.
            ClosureEvent::StarAged { class } => {
                star = crate::thermal::StarClass::from_wire(*class);
            }
        }
    }
    Committed { orbs, entities, star, debts, bonds, covenants, councils, techs, crossed, gauge }
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
                    // FLAT-rate decay (the leak/spoilage shape), never
                    // proportional: banked × (31/32)^n explodes the
                    // canonical denominator without bound (measured
                    // live: ~2^1100 after 220 beats). Flat keeps every
                    // denominator bounded by the rate's.
                    let decay = (crate::dop::demurrage_per_tick() * elapsed.clone())
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

/// ABSORB-1 (09 §2.4): a payment from debtor to creditor compensates
/// open reciprocity windows OLDEST FIRST; each paid portion posts the
/// debtor's returned book. Shared verbatim by the live path and replay.
fn settle_debts(
    debts: &mut Vec<AbsorptionDebt>,
    orbs: &mut [ColonyOrb],
    from: u64,
    to: u64,
    value: Ratio,
) {
    let zero = ratio(0, 1);
    let mut left = value;
    for debt in debts.iter_mut() {
        if left == zero {
            break;
        }
        if debt.debtor != from || debt.creditor != to {
            continue;
        }
        let pay = left.clone().min(debt.remainder.clone());
        if pay > zero {
            debt.remainder -= pay.clone();
            left -= pay.clone();
            if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == from) {
                held.returned_across += pay;
            }
        }
    }
    debts.retain(|debt| debt.remainder != ratio(0, 1));
}

/// IDEO-1 (09 §3.2, placeholder ruling): the measured boundary policy
/// for the friction law — returned/(returned+extracted); EMPTY books
/// read as 1 (a fresh orb trades frictionless until measured).
fn reciprocity_reading(orb: &ColonyOrb) -> Ratio {
    let denominator = orb.returned_across.clone() + orb.extracted_across.clone();
    match denominator > ratio(0, 1) {
        true => orb.returned_across.clone() / denominator,
        false => ratio(1, 1),
    }
}

/// COVENANT-1: a deliberate delivery (transfer or bond settlement)
/// from obligor to beneficiary credits the open pact on that lane —
/// at most one stands per (obligor, beneficiary, resource). Shared
/// verbatim by the live paths and replay.
fn credit_covenants(
    covenants: &mut [Covenant],
    from: u64,
    to: u64,
    resource: Resource,
    amount: &Ratio,
) {
    for pact in covenants.iter_mut() {
        if pact.obligor == from && pact.beneficiary == to && pact.resource == resource {
            pact.delivered += amount.clone();
        }
    }
}

/// COUNCIL-1: a delivery from one member to a FELLOW member credits
/// the giver's council quota (transfers and bond flow alike — the
/// covenant precedent, widened to the group).
fn credit_councils(
    councils: &mut [Council],
    from: u64,
    to: u64,
    resource: Resource,
    amount: &Ratio,
) {
    for pact in councils.iter_mut() {
        if pact.resource == resource && pact.members.contains(&to) {
            if let Some(at) = pact.members.iter().position(|member| *member == from) {
                pact.delivered[at] += amount.clone();
            }
        }
    }
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
