//! NS-1 DOP — dimensional orbs. The colony orb is the first consumer:
//! a self-contained game object whose committed state round-trips its
//! wire codec byte-exact.
//!
//! RULESET v1 (02_COLONY_LOOP §2 — LAW for replay; constants never
//! change silently under an existing world): founding reserves, zone
//! costs, yields, and upkeep live HERE, beside the types they rule.
//!
//! THE NETSTRATUM WIRE TAG REGISTRY (permanent — one tag, one meaning,
//! forever; retired codes are never reused):
//!   1  ClosureEvent::ColonyFounded
//!   2  ClosureEvent::ZoneBuilt
//!   3  ClosureEvent::YieldCollected
//!   4  ClosureEvent::UpkeepSettled
//!   5  ClosureEvent::FamineStruck
//!   6  ClosureEvent::FamineRelieved
//!   7  ClosureEvent::ZoneRazed
//!   10 ClosureEvent::EntityRegistered (N3)
//!   11 ClosureEvent::ScopeGranted (N3)
//!   12 ClosureEvent::ScopeRevoked (N3)
//!   13 ClosureEvent::ColonyCharted (P1)
//!   21 ClosureEvent::EraAnnexed (CONVERGE-1 — universe ledger only)
//!   22 ClosureEvent::OverheatStruck (THERMAL-1)
//!   23 ClosureEvent::OverheatCleared (THERMAL-1)
//!   24 ClosureEvent::StarCommissioned (THERMAL-1)
//!   25 ClosureEvent::PlanetSited (THERMAL-1)
//!   27 ClosureEvent::VentSettled (VENT-1)
//!   29 ClosureEvent::DemurrageSettled (SINK-1)
//!   30 ClosureEvent::ResourceTransferred (TRADE-1)
//!   16 Orb header (orb_id ‖ name)
//!   17 Orb zone entry (tile ‖ kind)
//!   18 Orb banked entry (resource ‖ ratio)
//!   19 Orb famine entry (resource)
//!   26 Orb thermal entry (albedo ‖ orbit_r ‖ overheat u8) — THERMAL-1
//!   28 Orb vent entry (entropy_vented ratio ‖ LE64 last_touch) — VENT-1/SINK-1
//!   31–38 RESERVED for the ideology board (09, BUILD_ORDER §3:
//!         ABSORB-1 → BOND-1 → COVENANT-1 → IDEO-1).
//!   39–40 RESERVED for tech emergence (10: TechDiscovered,
//!         TechLicensed) — next free tag for any OTHER landing is 41.

use crate::ratio::{self, ratio, Ratio};
use crate::tlv;

/// The resources of the first slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Resource {
    Iron = 1,
    Biomass = 2,
    /// D2 (02 §5): smelted from iron; the tier-2 structural currency.
    Alloy = 3,
    /// D2 (02 §5): the ultra-refined tier — dense, trade-worthy.
    Diamond = 4,
    /// THERMAL-1 (05 §1.2): the commodity joule — minted ONLY by
    /// syntropic encasement of excess heat, never by fiat (01 §6.1).
    Joule = 5,
    /// SINK-1 (05 §1.4): bankable heat-absorption capacity — captured
    /// from vault-band headroom by cryo-lattice arrays, never passive.
    Sink = 6,
}

impl Resource {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Iron),
            2 => Some(Self::Biomass),
            3 => Some(Self::Alloy),
            4 => Some(Self::Diamond),
            5 => Some(Self::Joule),
            6 => Some(Self::Sink),
            _ => None,
        }
    }

    pub const ALL: [Resource; 6] = [
        Self::Iron,
        Self::Biomass,
        Self::Alloy,
        Self::Diamond,
        Self::Joule,
        Self::Sink,
    ];
}

/// Founding banks the bootstrap reserves (02 §2.1): 20 iron, 10 biomass.
pub fn founding_reserves() -> Vec<(Resource, Ratio)> {
    vec![
        (Resource::Iron, ratio(20, 1)),
        (Resource::Biomass, ratio(10, 1)),
    ]
}

/// THERMAL-1 (05 §1.2, placeholder): joules one active encasement
/// shell can capture per tick.
pub fn encasement_throughput() -> Ratio {
    ratio(2, 1)
}

/// THERMAL-1 (05 §1.2, placeholder): the orb's base radiator capacity.
pub fn radiator_base() -> Ratio {
    ratio(2, 1)
}

/// THERMAL-1 (05 §1.2, placeholder): the per-tick leak on ACCRUED
/// joules outside the cryo vault — the D3 spoilage shape.
pub fn joule_leak_per_tick() -> Ratio {
    ratio(1, 8)
}

/// ECON-J (05 §1.2, placeholder): joules one active zone burns per
/// tick for heating on a sub-mesic sited orb.
pub fn heating_per_zone() -> Ratio {
    ratio(1, 4)
}

/// SINK-1 (05 §1.4, placeholder): sink capacity one active cryo-lattice
/// array captures per tick from vault-band headroom.
pub fn sink_throughput() -> Ratio {
    ratio(2, 1)
}

/// SINK-1 (Q2 demurrage, placeholder): the per-elapsed-tick decay rate
/// on un-vaulted BANKED joules/sink, settled by receipt at touch.
pub fn demurrage_per_tick() -> Ratio {
    ratio(1, 32)
}

/// TRADE-1 (09 §1, placeholder): the joule-commensurable weight of a
/// resource for cohesion-book postings — the commensuration table.
/// Weight 1 for every resource in v1; tunes at balance time.
pub fn commensurate_weight(_resource: Resource) -> Ratio {
    ratio(1, 1)
}

/// The zone kinds, with their ruleset-v1 economics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ZoneKind {
    Extractor = 1,
    Habitat = 2,
    /// D2 converter (02 §5.2): its upkeep is its INPUT, its yield its
    /// OUTPUT — conversion is a net flow like every other flow.
    Smelter = 3,
    Synthesizer = 4,
    /// THERMAL-1 (05 §1.2): the syntropic shell — captures excess heat
    /// into joules BEFORE anything vents. Its "yield" is computed by
    /// the heat law at tick time, never by `yield_per_tick`.
    Encasement = 5,
    /// SINK-1 (05 §1.4): the cryo-lattice array — captures vault-band
    /// radiator headroom as bankable sink. Heat-law yield, like the
    /// shell. The infrastructure gate; research/throttle gates deferred.
    CryoArray = 6,
}

impl ZoneKind {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Extractor),
            2 => Some(Self::Habitat),
            3 => Some(Self::Smelter),
            4 => Some(Self::Synthesizer),
            5 => Some(Self::Encasement),
            6 => Some(Self::CryoArray),
            _ => None,
        }
    }

    /// The build cost, spent from banked at build time (02 §2.2, §5.2).
    pub fn cost(&self) -> Vec<(Resource, Ratio)> {
        match self {
            Self::Extractor => vec![(Resource::Iron, ratio(10, 1))],
            Self::Habitat => vec![(Resource::Biomass, ratio(5, 1))],
            Self::Smelter => vec![
                (Resource::Iron, ratio(15, 1)),
                (Resource::Biomass, ratio(10, 1)),
            ],
            Self::Synthesizer => vec![
                (Resource::Alloy, ratio(20, 1)),
                (Resource::Biomass, ratio(15, 1)),
            ],
            Self::Encasement => vec![(Resource::Iron, ratio(8, 1))],
            Self::CryoArray => vec![(Resource::Iron, ratio(10, 1))],
        }
    }

    /// The exact yield rate — a ratio, never a float (RFA). The
    /// encasement shell's joules come from the heat law (05 §1.2), not
    /// from here: its listed rate is zero.
    pub fn yield_per_tick(&self) -> (Resource, Ratio) {
        match self {
            Self::Extractor => (Resource::Iron, ratio(3, 2)),
            Self::Habitat => (Resource::Biomass, ratio(1, 1)),
            Self::Smelter => (Resource::Alloy, ratio(1, 1)),
            Self::Synthesizer => (Resource::Diamond, ratio(1, 4)),
            Self::Encasement => (Resource::Joule, ratio(0, 1)),
            Self::CryoArray => (Resource::Sink, ratio(0, 1)),
        }
    }

    /// The per-tick upkeep drains (02 §2.2, §5.2): rations, life
    /// support, and — for converters — the conversion inputs.
    pub fn upkeep_per_tick(&self) -> Vec<(Resource, Ratio)> {
        match self {
            Self::Extractor => vec![(Resource::Biomass, ratio(1, 2))],
            Self::Habitat => vec![(Resource::Iron, ratio(1, 4))],
            Self::Smelter => vec![
                (Resource::Iron, ratio(2, 1)),
                (Resource::Biomass, ratio(1, 2)),
            ],
            Self::Synthesizer => vec![
                (Resource::Alloy, ratio(4, 1)),
                (Resource::Biomass, ratio(1, 1)),
            ],
            Self::Encasement => vec![(Resource::Iron, ratio(1, 4))],
            Self::CryoArray => vec![(Resource::Iron, ratio(1, 4))],
        }
    }

    /// THERMAL-1 (05 §1.2): each zone's per-tick heat contribution to
    /// the net-heat ledger (placeholder parameters — converters run
    /// hot, the shell itself adds nothing).
    pub fn factory_heat(&self) -> Ratio {
        match self {
            Self::Extractor | Self::Habitat => ratio(1, 4),
            Self::Smelter | Self::Synthesizer => ratio(1, 1),
            Self::Encasement | Self::CryoArray => ratio(0, 1),
        }
    }

    /// Razing refunds half of EACH cost component (02 §2.4).
    pub fn raze_refund(&self) -> Vec<(Resource, Ratio)> {
        self.cost()
            .into_iter()
            .map(|(resource, cost)| (resource, cost / ratio(2, 1)))
            .collect()
    }
}

/// A zoned tile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub tile: u16,
    pub kind: ZoneKind,
}

/// The closures of the colony loop — completed transitions, the only
/// things that chronicle (NS-3 §2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureEvent {
    ColonyFounded { orb_id: u64, name: String },
    ZoneBuilt { orb_id: u64, tile: u16, kind: ZoneKind },
    YieldCollected { orb_id: u64, resource: Resource, amount: Ratio },
    /// Collect settled a negative net from banked (02 §2.3).
    UpkeepSettled { orb_id: u64, resource: Resource, amount: Ratio },
    /// banked+accrued fell below 0: the stores emptied (`drawn` = what
    /// banked held at the strike), consumers went dormant (02 §2.4).
    FamineStruck { orb_id: u64, resource: Resource, drawn: Ratio },
    /// The resource recovered to one-tick upkeep; consumers woke.
    FamineRelieved { orb_id: u64, resource: Resource },
    /// A zone demolished for half its cost back.
    ZoneRazed { orb_id: u64, tile: u16 },
    /// N3 — an entity joined this internet: principal 0 = a sovereign
    /// human; nonzero = an agent contained by that principal.
    EntityRegistered { entity_id: u64, principal: u64, handle: String },
    /// N3 — a principal granted an agent one action scope.
    ScopeGranted { agent: u64, scope: crate::ipsal::Scope },
    /// N3 — a grant revoked; the agent's next such action refuses.
    ScopeRevoked { agent: u64, scope: crate::ipsal::Scope },
    /// P1 — a daughter colony charted on a new planet: the founding
    /// reserves MOVE from the source orb's bank (conservation — a
    /// charter mints nothing).
    ColonyCharted { orb_id: u64, source_orb: u64, name: String },
    /// CONVERGE-1 (07 §1) — an archived world adopted onto the one
    /// timeline: its era, name, terminal digest, and epoch offset.
    /// Lives ONLY in the universe ledger (`convergence.tlv`), never in
    /// a world chronicle; world replay ignores it.
    EraAnnexed { era: String, world: String, terminal: [u8; 32], offset: u64 },
    /// THERMAL-1 (05 §1.2) — overflow exceeded encasement: the sector
    /// overheats; yields degrade by 1/(1+overflow) until cleared.
    /// World-driven (no entity forged it), the famine-strike shape.
    OverheatStruck { orb_id: u64, overflow: Ratio },
    /// THERMAL-1 — the overflow returned to zero; yields recover.
    OverheatCleared { orb_id: u64 },
    /// THERMAL-1 (NS-6 §7) — the container's sun commissioned with its
    /// StarClass wire code. Once per world; fixes every orbit's L.
    StarCommissioned { class: u8 },
    /// THERMAL-1 (05 §1.1) — a colony sited on its orbit: albedo A and
    /// orbit radius r, the orb's terms in the master equation.
    PlanetSited { orb_id: u64, albedo: Ratio, orbit_r: Ratio },
    /// VENT-1 (05 §1.4) — accumulated vented entropy settled into the
    /// orb's cumulative ledger at collect (the yield pattern: one
    /// receipt, no per-tick bloat).
    VentSettled { orb_id: u64, amount: Ratio },
    /// SINK-1 (Q2) — receipted demurrage: un-vaulted banked stores
    /// decayed since last touch; settled here, never silently.
    DemurrageSettled { orb_id: u64, resource: Resource, amount: Ratio },
    /// TRADE-1 (09 §2.1) — BANKED value moved orb-to-orb within one
    /// world: the chartering conservation law generalized (nothing
    /// minted, nothing lost). The cohesion-book postings derive from
    /// this receipt and the counterparty's receipted crisis state — a
    /// replay fact, never a judgment call.
    ResourceTransferred { from_orb: u64, to_orb: u64, resource: Resource, amount: Ratio },
}

impl ClosureEvent {
    /// The normative TLV event encoding (NS-3 §2.2): one record, tag =
    /// the event's permanent code, value = the event's field layout.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Self::ColonyFounded { orb_id, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(name.as_bytes());
                tlv::put(1, &value, &mut out);
            }
            Self::ZoneBuilt { orb_id, tile, kind } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&tile.to_le_bytes());
                value.push(*kind as u8);
                tlv::put(2, &value, &mut out);
            }
            Self::YieldCollected { orb_id, resource, amount } => {
                tlv::put(3, &amount_layout(*orb_id, *resource, amount), &mut out);
            }
            Self::UpkeepSettled { orb_id, resource, amount } => {
                tlv::put(4, &amount_layout(*orb_id, *resource, amount), &mut out);
            }
            Self::FamineStruck { orb_id, resource, drawn } => {
                tlv::put(5, &amount_layout(*orb_id, *resource, drawn), &mut out);
            }
            Self::FamineRelieved { orb_id, resource } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*resource as u8);
                tlv::put(6, &value, &mut out);
            }
            Self::ZoneRazed { orb_id, tile } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&tile.to_le_bytes());
                tlv::put(7, &value, &mut out);
            }
            Self::EntityRegistered { entity_id, principal, handle } => {
                let mut value = entity_id.to_le_bytes().to_vec();
                value.extend_from_slice(&principal.to_le_bytes());
                value.extend_from_slice(handle.as_bytes());
                tlv::put(10, &value, &mut out);
            }
            Self::ScopeGranted { agent, scope } => {
                let mut value = agent.to_le_bytes().to_vec();
                value.push(*scope as u8);
                tlv::put(11, &value, &mut out);
            }
            Self::ScopeRevoked { agent, scope } => {
                let mut value = agent.to_le_bytes().to_vec();
                value.push(*scope as u8);
                tlv::put(12, &value, &mut out);
            }
            Self::ColonyCharted { orb_id, source_orb, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&source_orb.to_le_bytes());
                value.extend_from_slice(name.as_bytes());
                tlv::put(13, &value, &mut out);
            }
            Self::EraAnnexed { era, world, terminal, offset } => {
                let mut value = offset.to_le_bytes().to_vec();
                value.extend_from_slice(terminal);
                value.extend_from_slice(&(era.len() as u16).to_le_bytes());
                value.extend_from_slice(era.as_bytes());
                value.extend_from_slice(world.as_bytes());
                tlv::put(21, &value, &mut out);
            }
            Self::OverheatStruck { orb_id, overflow } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(overflow, &mut value);
                tlv::put(22, &value, &mut out);
            }
            Self::OverheatCleared { orb_id } => {
                tlv::put(23, &orb_id.to_le_bytes(), &mut out);
            }
            Self::StarCommissioned { class } => {
                tlv::put(24, &[*class], &mut out);
            }
            Self::PlanetSited { orb_id, albedo, orbit_r } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(albedo, &mut value);
                ratio::encode(orbit_r, &mut value);
                tlv::put(25, &value, &mut out);
            }
            Self::VentSettled { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(27, &value, &mut out);
            }
            Self::DemurrageSettled { orb_id, resource, amount } => {
                tlv::put(29, &amount_layout(*orb_id, *resource, amount), &mut out);
            }
            Self::ResourceTransferred { from_orb, to_orb, resource, amount } => {
                let mut value = from_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(30, &value, &mut out);
            }
        }
        out
    }

    /// Decode one event from its wire record; unknown tags are not an
    /// event (the caller skips them — SEAL A).
    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        match tag {
            1 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let name = String::from_utf8(value.get(8..)?.to_vec()).ok()?;
                Some(Self::ColonyFounded { orb_id, name })
            }
            2 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let tile = u16::from_le_bytes(value.get(8..10)?.try_into().ok()?);
                let kind = ZoneKind::from_wire(*value.get(10)?)?;
                Some(Self::ZoneBuilt { orb_id, tile, kind })
            }
            3 => {
                let (orb_id, resource, amount) = amount_fields(value)?;
                Some(Self::YieldCollected { orb_id, resource, amount })
            }
            4 => {
                let (orb_id, resource, amount) = amount_fields(value)?;
                Some(Self::UpkeepSettled { orb_id, resource, amount })
            }
            5 => {
                let (orb_id, resource, drawn) = amount_fields(value)?;
                Some(Self::FamineStruck { orb_id, resource, drawn })
            }
            6 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                Some(Self::FamineRelieved { orb_id, resource })
            }
            7 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let tile = u16::from_le_bytes(value.get(8..10)?.try_into().ok()?);
                Some(Self::ZoneRazed { orb_id, tile })
            }
            10 => {
                let entity_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let principal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let handle = String::from_utf8(value.get(16..)?.to_vec()).ok()?;
                Some(Self::EntityRegistered { entity_id, principal, handle })
            }
            11 => {
                let agent = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let scope = crate::ipsal::Scope::from_wire(*value.get(8)?)?;
                Some(Self::ScopeGranted { agent, scope })
            }
            12 => {
                let agent = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let scope = crate::ipsal::Scope::from_wire(*value.get(8)?)?;
                Some(Self::ScopeRevoked { agent, scope })
            }
            13 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let source_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let name = String::from_utf8(value.get(16..)?.to_vec()).ok()?;
                Some(Self::ColonyCharted { orb_id, source_orb, name })
            }
            21 => {
                let offset = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let terminal: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let era_len = u16::from_le_bytes(value.get(40..42)?.try_into().ok()?) as usize;
                let era = String::from_utf8(value.get(42..42 + era_len)?.to_vec()).ok()?;
                let world = String::from_utf8(value.get(42 + era_len..)?.to_vec()).ok()?;
                Some(Self::EraAnnexed { era, world, terminal, offset })
            }
            22 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (overflow, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::OverheatStruck { orb_id, overflow })
            }
            23 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                Some(Self::OverheatCleared { orb_id })
            }
            24 => Some(Self::StarCommissioned { class: *value.first()? }),
            25 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (albedo, used) = ratio::decode(value.get(8..)?)?;
                let (orbit_r, _) = ratio::decode(value.get(8 + used..)?)?;
                Some(Self::PlanetSited { orb_id, albedo, orbit_r })
            }
            27 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::VentSettled { orb_id, amount })
            }
            29 => {
                let (orb_id, resource, amount) = amount_fields(value)?;
                Some(Self::DemurrageSettled { orb_id, resource, amount })
            }
            30 => {
                let from_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::ResourceTransferred { from_orb, to_orb, resource, amount })
            }
            _ => None,
        }
    }
}

/// The shared `LE64(orb_id) ‖ resource u8 ‖ ratio` layout (tags 3–5).
fn amount_layout(orb_id: u64, resource: Resource, amount: &Ratio) -> Vec<u8> {
    let mut value = orb_id.to_le_bytes().to_vec();
    value.push(resource as u8);
    ratio::encode(amount, &mut value);
    value
}

fn amount_fields(value: &[u8]) -> Option<(u64, Resource, Ratio)> {
    let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
    let resource = Resource::from_wire(*value.get(8)?)?;
    let (amount, _) = ratio::decode(value.get(9..)?)?;
    Some((orb_id, resource, amount))
}

/// THERMAL-1 (05 §1.1): a colony's terms in the master equation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThermalSite {
    /// Albedo A, 0 ≤ A < 1.
    pub albedo: Ratio,
    /// Orbit radius r > 0, in the star's units.
    pub orbit_r: Ratio,
}

/// The colony orb — COMMITTED state only. Unreceipted accrual is
/// workspace (it lives on the world, not the orb) and never encodes:
/// what has no receipt does not persist.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColonyOrb {
    pub orb_id: u64,
    pub name: String,
    /// Zones, tile-ascending (canonical order).
    pub zones: Vec<Zone>,
    /// Banked yields, resource-ascending (canonical order); zero
    /// entries are dropped (one state, one encoding).
    pub banked: Vec<(Resource, Ratio)>,
    /// Resources under famine, ascending — zones consuming these are
    /// DORMANT (no yield, no drain).
    pub famine: Vec<Resource>,
    /// THERMAL-1: the orbit siting; ABSENT until `PlanetSited` — an
    /// unsited orb is thermally dormant (the opt-in guard).
    pub thermal: Option<ThermalSite>,
    /// THERMAL-1: currently overheating (overflow > 0)?
    pub overheat: bool,
    /// VENT-1: cumulative vented entropy, settled by receipt.
    pub entropy_vented: Ratio,
    /// SINK-1: the stamp of the last settle (demurrage anchor).
    pub last_touch: u64,
    /// TRADE-1 (09 §2.1): the compassion books — cumulative
    /// joule-commensurable value given TO counterparties standing in
    /// receipted crisis, and drawn FROM them. Committed state rebuilt
    /// from receipts alone on replay; wire encoding DEFERRED to
    /// IDEO-1 (the reserved orb cohesion entry, tag 38 — recorded on
    /// the board).
    pub given_in_crisis: Ratio,
    /// The entropic book of the pair (see `given_in_crisis`).
    pub drawn_in_crisis: Ratio,
}

impl ColonyOrb {
    /// The canonical orb codec: header, zones tile-ascending, banked
    /// resource-ascending, famines ascending.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut header = self.orb_id.to_le_bytes().to_vec();
        header.extend_from_slice(self.name.as_bytes());
        tlv::put(16, &header, &mut out);
        for zone in &self.zones {
            let mut value = zone.tile.to_le_bytes().to_vec();
            value.push(zone.kind as u8);
            tlv::put(17, &value, &mut out);
        }
        for (resource, amount) in &self.banked {
            let mut value = vec![*resource as u8];
            ratio::encode(amount, &mut value);
            tlv::put(18, &value, &mut out);
        }
        for resource in &self.famine {
            tlv::put(19, &[*resource as u8], &mut out);
        }
        if let Some(site) = &self.thermal {
            let mut value = Vec::new();
            ratio::encode(&site.albedo, &mut value);
            ratio::encode(&site.orbit_r, &mut value);
            value.push(self.overheat as u8);
            tlv::put(26, &value, &mut out);
        }
        if self.entropy_vented != ratio(0, 1) || self.last_touch != 0 {
            let mut value = Vec::new();
            ratio::encode(&self.entropy_vented, &mut value);
            value.extend_from_slice(&self.last_touch.to_le_bytes());
            tlv::put(28, &value, &mut out);
        }
        out
    }

    /// Decode an orb; unknown tags are SKIPPED (SEAL A).
    pub fn from_wire(bytes: &[u8]) -> Option<Self> {
        let mut orb = ColonyOrb::default();
        let mut seen_header = false;
        for (tag, value) in tlv::records(bytes) {
            match tag {
                16 => {
                    orb.orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                    orb.name = String::from_utf8(value.get(8..)?.to_vec()).ok()?;
                    seen_header = true;
                }
                17 => {
                    let tile = u16::from_le_bytes(value.get(..2)?.try_into().ok()?);
                    let kind = ZoneKind::from_wire(*value.get(2)?)?;
                    orb.zones.push(Zone { tile, kind });
                }
                18 => {
                    let resource = Resource::from_wire(*value.first()?)?;
                    let (amount, _) = ratio::decode(value.get(1..)?)?;
                    orb.banked.push((resource, amount));
                }
                19 => {
                    orb.famine.push(Resource::from_wire(*value.first()?)?);
                }
                26 => {
                    let (albedo, used) = ratio::decode(value)?;
                    let (orbit_r, used_r) = ratio::decode(value.get(used..)?)?;
                    orb.thermal = Some(ThermalSite { albedo, orbit_r });
                    orb.overheat = *value.get(used + used_r)? != 0;
                }
                28 => {
                    let (vented, used) = ratio::decode(value)?;
                    orb.entropy_vented = vented;
                    orb.last_touch =
                        u64::from_le_bytes(value.get(used..used + 8)?.try_into().ok()?);
                }
                _ => {}
            }
        }
        seen_header.then_some(orb)
    }

    /// The banked amount of a resource (zero when absent).
    pub fn banked_amount(&self, resource: Resource) -> Ratio {
        self.banked
            .iter()
            .find(|(held, _)| *held == resource)
            .map(|(_, amount)| amount.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// Add (or, negative, draw) an amount in the canonical ledger; zero
    /// entries drop so equal states share one encoding.
    pub fn bank(&mut self, resource: Resource, amount: Ratio) {
        match self.banked.iter_mut().find(|(held, _)| *held == resource) {
            Some((_, held)) => *held += amount,
            None => {
                self.banked.push((resource, amount));
                self.banked.sort_by_key(|(resource, _)| *resource);
            }
        }
        self.banked.retain(|(_, amount)| amount != &ratio(0, 1));
    }

    /// Is a zone active? Dormant while ANY of its consumed resources
    /// is famined.
    pub fn zone_active(&self, zone: &Zone) -> bool {
        zone.kind
            .upkeep_per_tick()
            .iter()
            .all(|(consumed, _)| !self.famine.contains(consumed))
    }
}
