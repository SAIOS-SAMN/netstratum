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
//!   8  ClosureEvent::WorldCharted (D3 — the terrain seed, the OPT-IN
//!         deed; 9 stays reserved for D3 milestones)
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
//!   31 ClosureEvent::OverflowAbsorbed (ABSORB-1)
//!   32 ClosureEvent::AbsorptionLapsed (ABSORB-1)
//!   33 ClosureEvent::BondSealed (BOND-1)
//!   34 ClosureEvent::BondSettled (BOND-1 — settlement AND severance:
//!      the severed flag + load ride the one settle receipt)
//!   35 ClosureEvent::CovenantSealed (COVENANT-1)
//!   36 ClosureEvent::CovenantReleased (COVENANT-1)
//!   37 ClosureEvent::CovenantSlashed (COVENANT-1)
//!   38 ClosureEvent::FrictionSettled (IDEO-1)
//!   16 Orb header (orb_id ‖ name)
//!   17 Orb zone entry (tile ‖ kind)
//!   18 Orb banked entry (resource ‖ ratio)
//!   19 Orb famine entry (resource)
//!   26 Orb thermal entry (albedo ‖ orbit_r ‖ overheat u8) — THERMAL-1
//!   28 Orb vent entry (entropy_vented ratio ‖ LE64 last_touch) — VENT-1/SINK-1
//!   41 Orb cohesion entry (the eight books, IDEO-1) — emitted only
//!      when a book is nonzero; a zero-book orb encodes byte-identical
//!      to a pre-ideology one.
//!   39 ClosureEvent::TechDiscovered (TECH-1a — condition 0 the kernel
//!      solar leap; 1–5 the crystallized nodes, the TECH-1b slice)
//!   40 ClosureEvent::TechLicensed (PLANNER-1 — the issuing deed; the
//!      far world receipts TechDiscovered under the same deal; the
//!      solar leap is never licensed).
//!   42 ClosureEvent::GaugeShifted (AEON-1a — the key moves)
//!   43 ClosureEvent::Diverged (AEON-1a — the cleave, a peer branch)
//!   44 ClosureEvent::Confluent (AEON-1a — the recombination)
//!   45 ClosureEvent::StarAged (AEON-1a — the star moves in its region)
//!   46 ClosureEvent::EraFolded (AEON-1a — the fold; base G advances)
//!   47 ClosureEvent::ApertureSent (NS-4 APERTURE — the source leg)
//!   48 ClosureEvent::ApertureReceived (NS-4 APERTURE — the dest leg)
//!   49 ClosureEvent::OfferPosted (MARKET-1 — the resting intent)
//!   50 ClosureEvent::OfferCleared (MARKET-1 — the crossing filled it)
//!   51 RESERVED for the market board's third (BOARD-1 if it needs one)
//!   52 ClosureEvent::CouncilSealed (COUNCIL-1 — the multilateral pact)
//!   53 ClosureEvent::CouncilClosed (COUNCIL-1 — one member's reckoning)
//!   54 ClosureEvent::GalaxyObserved (GALAXY-1 — universe ledger only)
//!   55 ClosureEvent::CrystalDrawn (N4 / NS-2 §1.5 — the scanner's
//!         power draw, the ONE chronicle event of RQP: query, hit, and
//!         refusal never receipt — the lens law)
//!   56–58 NS-2 protocol records (query ‖ hit ‖ refusal — NS-2 §2,
//!         RQP's own framing, never world-chronicle events)
//!   59 ClosureEvent::LineSucceeded (NOVEL-1, 10 §3c — the succession
//!         deed: a successor line adopts a derelict world's chronicle
//!         as its own substrate; the living tree drops to the world's
//!         kernel, the record stands forever)
//!   60 ClosureEvent::GaugeShiftedAt (GAUGE-N, 11 §2.1 — the graded
//!         key: moves or opens plane `grade`; plane 0 is the landed
//!         scalar, tags 42–46 bind to it forever)
//!   61 ClosureEvent::GenesisSealed (G1, 04 §1/§5 — the three axes
//!         sealed as the world-chronicle receipt; the registry becomes
//!         a cache of this receipted truth, never ground truth)
//!   62 ClosureEvent::WantStruck (WANT-1 — the cognitive want: kind 1
//!      epistemological [board-side reading, reserved] · 2 resonance ·
//!      3 novelty)
//!   63 ClosureEvent::WantRelieved (WANT-1 — the mind fed)
//!         — the next free tag outside reservations is 64.
//!   In-value codes opened at N4 (they ride existing orb tags, no new
//!   TLV tag): Resource::Crystal = 7 (banked entry, tag 18);
//!   ZoneKind::Resonator = 7 (zone entry, tag 17).

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
    /// N4 (NS-2 §1.5, 02 §5.1 — the deferral paid): the resonance
    /// crystal, the scanner's power source. Produced by the resonator,
    /// consumed by the lens.
    Crystal = 7,
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
            7 => Some(Self::Crystal),
            _ => None,
        }
    }

    pub const ALL: [Resource; 7] = [
        Self::Iron,
        Self::Biomass,
        Self::Alloy,
        Self::Diamond,
        Self::Joule,
        Self::Sink,
        Self::Crystal,
    ];
}

/// TECH-1b (10 §3): the condition table — the crystallized node's
/// modifier kind and magnitude per condition. MECHANICS only: the
/// NAME derives from the node hash (`tech_node_name` — the 10 §2
/// law-4 word tables), so two lines crystallizing the same condition
/// carry DIFFERENT names. Naming is never stacked on rungs (operator
/// ruling 2026-07-30): a line's technology is its own, seeded from
/// its own chronicle root.
pub fn tech_node_of(condition: u8) -> Option<(u8, Ratio)> {
    match condition {
        1 => Some((1, ratio(3, 4))),
        2 => Some((2, ratio(5, 4))),
        3 => Some((3, ratio(5, 4))),
        4 => Some((4, ratio(5, 4))),
        5 => Some((5, ratio(1, 2))),
        _ => None,
    }
}

/// TECH-1b (10 §2, law 4): the node's NAME — a deterministic
/// word-table composition from the node hash. The condition fixes the
/// CONCEPT (what the technology does); the hash draws the VOICE (how
/// this line speaks of it). Flavor at the surface, hash as the
/// identity underneath — replay re-derives the identical name because
/// the receipt carries the identical node.
pub fn tech_node_name(condition: u8, node: &[u8; 32]) -> String {
    let concept = match condition {
        1 => "insulation",
        2 => "discipline",
        3 => "deepening",
        4 => "doctrine",
        5 => "stores",
        // TERRA-1: the gifted crown of the base-7 stack — never
        // crystallized locally, only licensed down from tier 0.
        6 => "terraforming",
        _ => "working",
    };
    const VOICES: [&str; 12] = [
        "resonant", "patient", "vaulted", "keening", "tidal", "ember",
        "hollow", "woven", "still", "spiral", "murmured", "adamant",
    ];
    let voice = VOICES[(node[0] as usize) % VOICES.len()];
    format!("{voice} {concept}")
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

/// ABSORB-1 (09 §2.4, placeholder): the reciprocity window in ticks —
/// a cross-orb draw must be compensated within this span or the
/// remainder lapses to the debtor's extraction book.
pub fn reciprocity_window() -> u64 {
    8
}

/// BOND-1 (09 §2.2, placeholder): the notice window in ticks — a bond
/// cut without it (manual severance, or an insolvent default) posts
/// `rate × this` to the cutter's severance book. A bond run to its
/// term posts nothing.
pub fn bond_notice_window() -> u64 {
    8
}

/// COVENANT-1 (09 §3.1, placeholder curve): collateral scales by the
/// obligor's fidelity books — quota × (breached + 1)/(fulfilled + 1),
/// clamped to [1/4, 4]. The empty book is not a credential (full
/// collateral until history exists); the ceiling is the guardrail-6
/// law made literal: a zero-fidelity orb pays the maximum and is
/// NEVER barred.
pub fn collateral_required(fulfilled: &Ratio, breached: &Ratio, quota: &Ratio) -> Ratio {
    let one = ratio(1, 1);
    let multiplier = (breached.clone() + one.clone()) / (fulfilled.clone() + one);
    let clamped = multiplier.max(ratio(1, 4)).min(ratio(4, 1));
    quota.clone() * clamped
}

/// IDEO-1 (09 §3.2, placeholder): the ideological friction rate φ —
/// `ΔS_ideo = φ × abs(reciprocity_A − reciprocity_B) × volume` per
/// cross-orb transfer. Friction prices DIVERGENCE, not vice: aligned
/// partners (any alignment) pay exactly zero.
pub fn friction_rate() -> Ratio {
    ratio(1, 8)
}

/// N4 (NS-2 §1.5, placeholder): crystals one scan draws from the
/// probing orb's bank — the lens's power, spent on the looking, hit or
/// miss. Judged pure before anything moves.
pub fn scan_draw() -> Ratio {
    ratio(1, 1)
}

/// G1 (04 §1): the axis vocabulary, closed and append-only (the
/// registry lists the words; these are their wire codes). Axis 1
/// disposition (1 expansionist ‖ 2 empiricist ‖ 3 epicurean); axis 2
/// metabolism (1 mineral ‖ 2 biotic ‖ 3 plasmic); axis 3 perception
/// (1 planar ‖ 2 volumetric ‖ 3 harmonic ‖ 4 umbral).
pub fn axis_valid(axis: u8, code: u8) -> bool {
    match axis {
        1 | 2 => (1..=3).contains(&code),
        3 => (1..=4).contains(&code),
        _ => false,
    }
}

/// G2 (04 §1.2): the ration law — the resource filling the FOOD slot
/// in every flow, keyed by the SEALED metabolism (biotic is the
/// ruleset-v1 default). Mineral eats iron; plasmic eats joules — and
/// the fiat law binds (05 §1.2, 01 §6.1): joules enter only by
/// encasement, so plasmic agriculture IS the thermal loop.
pub fn metabolic_ration(metabolism: u8) -> Resource {
    match metabolism {
        1 => Resource::Iron,
        3 => Resource::Joule,
        _ => Resource::Biomass,
    }
}

/// G2: the habitat's harvest under the sealed metabolism — what the
/// line farms. Biotic farms biomass (v1 verbatim); mineral farms
/// iron; plasmic farms NOTHING (rate 0 — its food is captured heat,
/// never a yield line).
pub fn metabolic_harvest(metabolism: u8) -> (Resource, Ratio) {
    match metabolism {
        1 => (Resource::Iron, ratio(1, 1)),
        3 => (Resource::Joule, ratio(0, 1)),
        _ => (Resource::Biomass, ratio(1, 1)),
    }
}

/// G2: one zone's yield under the sealed metabolism — the habitat's
/// biomass line is the FARM slot and reads the line's harvest; every
/// other yield is extraction or conversion and never substitutes.
pub fn zone_yield(kind: ZoneKind, metabolism: u8) -> (Resource, Ratio) {
    match (kind, kind.yield_per_tick()) {
        (ZoneKind::Habitat, (Resource::Biomass, _)) => metabolic_harvest(metabolism),
        (_, held) => held,
    }
}

/// G2: one zone's upkeep under the sealed metabolism — every biomass
/// component is the FOOD slot and reads the line's ration; structural
/// components (iron maintenance, conversion inputs in other
/// resources) never substitute.
pub fn zone_upkeep(kind: ZoneKind, metabolism: u8) -> Vec<(Resource, Ratio)> {
    kind.upkeep_per_tick()
        .into_iter()
        .map(|(resource, amount)| match resource {
            Resource::Biomass => (metabolic_ration(metabolism), amount),
            held => (held, amount),
        })
        .collect()
}

/// G3 (04 §1.3, §4; NS-2 §4, placeholder table): the fraction of
/// structure surviving a cross-frame read — exact rationals, symmetric
/// in v1 (the loss prices the DISTANCE between frames). Same basis 1;
/// the 04 §4 example verbatim (harmonic through a planar eye: 3/4);
/// umbral reads by occlusion and keeps 1/2 of any other frame; basis
/// 0 (unsealed) carries no frame claim and reads whole. The loss is
/// SHOWN, never silent — and never keys a bucket: frames quotient out
/// of the sigil (the H¹ class IS what survives translation).
pub fn translation_survival(ours: u8, theirs: u8) -> Ratio {
    match (ours, theirs) {
        (0, _) | (_, 0) => ratio(1, 1),
        (a, b) if a == b => ratio(1, 1),
        (4, _) | (_, 4) => ratio(1, 2),
        _ => ratio(3, 4),
    }
}

/// G1 (04 §1.1, placeholder weights): the disposition's value
/// weighting over the three v1 counters — territory closures, verified
/// discoveries, consumed yield — exact rationals summing to 1. The
/// physics never differs; only which history counts as winning.
pub fn disposition_weights(disposition: u8) -> Option<[Ratio; 3]> {
    match disposition {
        1 => Some([ratio(3, 5), ratio(1, 5), ratio(1, 5)]),
        2 => Some([ratio(1, 5), ratio(3, 5), ratio(1, 5)]),
        3 => Some([ratio(1, 5), ratio(1, 5), ratio(3, 5)]),
        _ => None,
    }
}

/// IDEO-1 (09 §2.5): the derived axis reading — syntropic /
/// (syntropic + entropic), or None when no deed was ever posted. The
/// ratio DERIVES at read; it is never stored (the syntropy-ratio
/// precedent).
pub fn cohesion_ratio(syntropic: &Ratio, entropic: &Ratio) -> Option<Ratio> {
    let denominator = syntropic.clone() + entropic.clone();
    match denominator > ratio(0, 1) {
        true => Some(syntropic.clone() / denominator),
        false => None,
    }
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
    /// N4 converter (NS-2 §1.5, the 02 §5.2 pattern): refines diamond
    /// into resonance crystals. One synthesizer feeds one resonator on
    /// the knife edge — diamond nets exactly 0.
    Resonator = 7,
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
            7 => Some(Self::Resonator),
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
            Self::Resonator => vec![
                (Resource::Alloy, ratio(10, 1)),
                (Resource::Diamond, ratio(5, 1)),
            ],
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
            Self::Resonator => (Resource::Crystal, ratio(1, 8)),
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
            Self::Resonator => vec![(Resource::Diamond, ratio(1, 4))],
        }
    }

    /// THERMAL-1 (05 §1.2): each zone's per-tick heat contribution to
    /// the net-heat ledger (placeholder parameters — converters run
    /// hot, the shell itself adds nothing).
    pub fn factory_heat(&self) -> Ratio {
        match self {
            Self::Extractor | Self::Habitat => ratio(1, 4),
            Self::Smelter | Self::Synthesizer | Self::Resonator => ratio(1, 1),
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
    /// D3 (02 §2.5) — the star's terrain charted: the seed its tile
    /// classes derive from, pure and forever. The OPT-IN deed (the
    /// THERMAL-1 precedent): a star without this receipt reads all
    /// Standard; the seed is the FOUNDING receipt's digest
    /// (chronicle-seeded, CHARTER §9 — no wall clock, no dice).
    WorldCharted { seed: [u8; 32] },
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
    /// GALAXY-1 (13_GALAXY §3–§4) — the observation deed: a crossing
    /// anchored past the frontier (wrapped-L1 to EVERY charted center
    /// exceeding its reach — a far region, never an outside) CREATES
    /// the galaxy it observes. Lives ONLY in the universe ledger
    /// (`convergence.tlv`), never in a world chronicle; world replay
    /// ignores it.
    GalaxyObserved {
        observer: String,
        creator: String,
        name: String,
        center: (Ratio, Ratio, Ratio),
        reach: Ratio,
    },
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
    /// ABSORB-1 (05 §1.4, 09 §2.4) — un-sealed overflow routed into a
    /// partner's banked sink instead of the void: sink_consumed =
    /// heat_absorbed = vented_after_shed, the absorber's ledger
    /// carrying the shed. Opens a reciprocity window owed by the
    /// source to the absorber.
    OverflowAbsorbed { source_orb: u64, absorber_orb: u64, amount: Ratio },
    /// ABSORB-1 (09 §2.4) — a reciprocity window expired unpaid: the
    /// remainder posts to the debtor's extraction book. World-driven
    /// at the exact stamp (the famine-strike shape).
    AbsorptionLapsed { debtor_orb: u64, creditor_orb: u64, amount: Ratio },
    /// BOND-1 (09 §2.2) — a standing order sealed: source streams
    /// `rate` of `resource` per tick to target; `term` in ticks, 0 =
    /// open-ended. One bond per (source, target, resource).
    BondSealed { source_orb: u64, target_orb: u64, resource: Resource, rate: Ratio, term: u64 },
    /// BOND-1 — a bond's accumulated span settled by ONE receipt at
    /// the touch (collect / sever / term expiry). `severed` ends the
    /// bond; `load` is the severance posting (zero for an orderly
    /// end). Its own tag — a bond settle must stay distinguishable
    /// from a manual transfer, or replay could not re-anchor the bond.
    BondSettled {
        source_orb: u64,
        target_orb: u64,
        resource: Resource,
        amount: Ratio,
        severed: bool,
        load: Ratio,
    },
    /// COVENANT-1 (01 §7, 09 §2.3) — a vitrified delivery pact sealed:
    /// the obligor locks `collateral` (out of its bank, the vault by
    /// construction) against delivering ≥ `quota` of `resource` to the
    /// beneficiary within `window` ticks. The collateral rides the
    /// receipt so replay never re-derives the fidelity curve.
    CovenantSealed {
        obligor_orb: u64,
        beneficiary_orb: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
        collateral: Ratio,
    },
    /// COVENANT-1 — the quota was met: the collateral returns home and
    /// the stake posts `covenants_fulfilled`. World-driven at the tick
    /// boundary (fulfilment wins the deadline tie).
    CovenantReleased {
        obligor_orb: u64,
        beneficiary_orb: u64,
        resource: Resource,
        collateral: Ratio,
    },
    /// COVENANT-1 — the deadline passed unmet: BREACH_AND_SLASH — the
    /// collateral forfeits to the aggrieved and the stake posts
    /// `covenants_breached`.
    CovenantSlashed {
        obligor_orb: u64,
        beneficiary_orb: u64,
        resource: Resource,
        collateral: Ratio,
    },
    /// IDEO-1 (09 §3.2) — accrued ideological friction sealed into the
    /// mover's vented ledger at its collect (the VentSettled shape):
    /// real entropy, the handling cost of divergent boundary policies.
    FrictionSettled { orb_id: u64, amount: Ratio },
    /// WANT-1 (12_UNION §5b) — a COGNITIVE want strikes: a famine of
    /// the mind, derived from the chronicle and the SEALED identity
    /// (G1 — the entity type is the receipted axes, never the registry
    /// cache). Kinds: 1 epistemological (truth-starvation — reading
    /// reserved to the board/mirror side), 2 resonance (isolation — a
    /// harmonic mind starved of links), 3 novelty (stagnation — an
    /// empiricist mind starved of signal; crystallization HALTS while
    /// struck, so relief arrives by licensed leap — the co-op pressure
    /// by design). `reading` is the measured drought, in ticks.
    WantStruck { orb_id: u64, kind: u8, reading: Ratio },
    /// WANT-1 — the want relieved: the mind fed (a new link, a fresh
    /// discovery). World-driven at the year voice, the famine shape.
    WantRelieved { orb_id: u64, kind: u8 },
    /// MARKET-1 (BUILD_ORDER §3a) — an OFFER posts: this orb stands
    /// ready to GIVE `give_amount` of `give` for `want_amount` of
    /// `want`. The price IS the ratio (no absolute price exists —
    /// guardrail 6); the offer RESTS as committed state until a
    /// crossing clears it or it is never met. No escrow: affordability
    /// is judged at the fill, and a stale offer refuses then.
    OfferPosted {
        offer: u64,
        orb_id: u64,
        give: Resource,
        give_amount: Ratio,
        want: Resource,
        want_amount: Ratio,
    },
    /// MARKET-1 — the offer CLEARED: a crossing filled it. The fill's
    /// amounts live in the aperture legs the `deal` links (deal and
    /// deal+1 — the two directions); this receipt closes the resting
    /// intent. Replay removes the offer.
    OfferCleared { offer: u64, deal: u64 },
    /// GAUGE-N (11 §2.1, wired 2026-07-30) — the key at a GRADE: the
    /// fiber is a graded vector of wrapped keys, not one dial, and
    /// this deed moves (or OPENS — a shift at an unopened grade grows
    /// the key, the tonal dimension the growth law promises) one
    /// plane. Grade 0 is the landed scalar; tag 42 stays its own
    /// grade-0 shorthand forever (tags permanent).
    GaugeShiftedAt { grade: u16, gauge: Ratio },
    /// AEON-1a (11 §2.1) — the world's KEY moved: its gauge phase G,
    /// an exact rational wrapped on the torus fiber. Progression,
    /// elevation (12_UNION §6), and crystallized tech all move in G;
    /// replay takes the receipted value verbatim (already wrapped).
    GaugeShifted { gauge: Ratio },
    /// AEON-1a (11 §3) — DIVERGENCE: this line cleaved; a peer branch
    /// (no parent, no child) departs under `branch`, and THIS world's
    /// own key moves to `gauge` (the cleave IS a key change). The
    /// branch founds its own chronicle — annex, never merge.
    Diverged { branch: String, gauge: Ratio },
    /// AEON-1a (11 §3) — CONFLUENCE: this line combined with a bonded
    /// partner line; the key moves to `gauge` (the interference of
    /// both — the midpoint on the torus). Earned via a sustained bond,
    /// never free; the hybrid's own chronicle founds separately.
    Confluent { partner: String, gauge: Ratio },
    /// AEON-1a (11 §6) — the star MOVED in its region (mass ×
    /// composition × age): the class steps, luminosity recomputes,
    /// every bound orbit's T_eq⁴ shifts at the next tick. Movement in
    /// a region, never a rung on a track.
    StarAged { class: u8 },
    /// AEON-1a (11 §5) — the FOLD: the era sealed and rotated into the
    /// archive sequence; the receipt carries the era ordinal and the
    /// NEW base gauge — the next aeon seats on its own G-sheet, a
    /// parallel degree of the torus that cannot recur into this one.
    /// Nothing is destroyed; the fold is archival by construction.
    EraFolded { era: u64, gauge: Ratio },
    /// PLANNER-1 (12_UNION §2, 10 §2.2) — a technology LICENSED out of
    /// this world into another: the planner's issuing deed, the source
    /// leg of a two-world hand-off (the aperture shape — the far world
    /// receipts `TechDiscovered` under the same deal). Bound to an
    /// OPEN council in this world (`council` — delegation settles
    /// inside diplomacy, 12_UNION §7); the solar leap (condition 0) is
    /// NEVER licensed — the young meet their sun themselves.
    TechLicensed {
        deal: u64,
        from_orb: u64,
        council: u64,
        condition: u8,
        node: [u8; 32],
        name: String,
    },
    /// COUNCIL-1 (12_UNION §7) — a COUNCIL seals: a multilateral
    /// covenant among three or more orbs, every member bound alike —
    /// each locks the same collateral against delivering ≥ quota to
    /// fellow members within the window. The group overtakes the
    /// individual: the reckoning judges all members together at the
    /// boundary, never one at a time.
    CouncilSealed {
        council: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
        collateral: Ratio,
        members: Vec<u64>,
    },
    /// COUNCIL-1 — one member's reckoning at the council boundary:
    /// `banked` is exactly what returned to this member's bank (its
    /// own collateral if faithful, plus its share of forfeits; a
    /// breacher amid faithful members banks zero; if EVERY member
    /// breached, each banks its collateral home — no aggrieved party,
    /// the pact dissolves, the books still post). Replay banks the
    /// carried amount verbatim; the integrity stake reads from the
    /// sealed council.
    CouncilClosed { council: u64, member: u64, fulfilled: bool, banked: Ratio },
    /// TECH-1a (10 §2.2) — a technology node enters the chronicle.
    /// `condition` 0 is the KERNEL solar leap (12_UNION §5a): a world's
    /// first thermal opt-in receipts that its people now know their
    /// sun — the opt-in IS the leap's receipt. Conditions 1–5 are the
    /// crystallized emergent nodes (10 §3, the TECH-1b slice); the
    /// node id is the seeded-synthesis hash (SEAL B — replay
    /// re-derives it), `modifier`/`magnitude` the applied parameter
    /// (0 ‖ 1/1 for the kernel leap: knowing is not yet a bonus).
    TechDiscovered {
        orb_id: u64,
        node: [u8; 32],
        condition: u8,
        modifier: u8,
        magnitude: Ratio,
        name: String,
    },
    /// NS-4 APERTURE (BUILD_ORDER §3a) — the SOURCE leg of a cross-world
    /// value hand-off: banked value LEFT this world's orb for another,
    /// bound to `deal` (the id the destination's `ApertureReceived`
    /// carries). Conservation is a property of the PAIR, not of this
    /// chronicle alone — replay of this world sees only the debit.
    ApertureSent { deal: u64, from_orb: u64, resource: Resource, amount: Ratio },
    /// NS-4 APERTURE (BUILD_ORDER §3a) — the DESTINATION leg: banked
    /// value ARRIVED in this world's orb from another, bound to the same
    /// `deal`. Replay of this world sees only the credit.
    ApertureReceived { deal: u64, to_orb: u64, resource: Resource, amount: Ratio },
    /// N4 (NS-2 §1.5) — the scanner's power draw, the ONE chronicle
    /// event of RQP: crystals left the probing orb's bank to hold the
    /// lens open. Deliberately blind to what was asked and what was
    /// seen — query, hit, and refusal never receipt (the lens law).
    CrystalDrawn { orb_id: u64, amount: Ratio },
    /// NOVEL-1 (10 §3c) — the succession deed: a successor line
    /// adopted this world's chronicle as its own substrate (the
    /// derelict door, 07 §3). The living tree drops to the world's
    /// kernel; every discovered node stands in the record forever —
    /// the pre-existing is changed, never replaced.
    LineSucceeded { successor: String },
    /// G1 (04 §1/§5) — the three axes sealed as THE binding copy: from
    /// this receipt on, the registry entry is a cache of receipted
    /// truth and no mechanic reads it where this receipt exists.
    /// Genesis is entry in the bands (04 §6a) — never a tier-0 seat.
    GenesisSealed { civilization: String, disposition: u8, metabolism: u8, perception: u8 },
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
            Self::WorldCharted { seed } => {
                tlv::put(8, seed, &mut out);
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
            Self::GalaxyObserved { observer, creator, name, center, reach } => {
                let mut value = Vec::new();
                for text in [observer, creator, name] {
                    value.extend_from_slice(&(text.len() as u16).to_le_bytes());
                    value.extend_from_slice(text.as_bytes());
                }
                ratio::encode(&center.0, &mut value);
                ratio::encode(&center.1, &mut value);
                ratio::encode(&center.2, &mut value);
                ratio::encode(reach, &mut value);
                tlv::put(54, &value, &mut out);
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
            Self::OverflowAbsorbed { source_orb, absorber_orb, amount } => {
                let mut value = source_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&absorber_orb.to_le_bytes());
                ratio::encode(amount, &mut value);
                tlv::put(31, &value, &mut out);
            }
            Self::AbsorptionLapsed { debtor_orb, creditor_orb, amount } => {
                let mut value = debtor_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&creditor_orb.to_le_bytes());
                ratio::encode(amount, &mut value);
                tlv::put(32, &value, &mut out);
            }
            Self::BondSealed { source_orb, target_orb, resource, rate, term } => {
                let mut value = source_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&target_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(rate, &mut value);
                value.extend_from_slice(&term.to_le_bytes());
                tlv::put(33, &value, &mut out);
            }
            Self::BondSettled { source_orb, target_orb, resource, amount, severed, load } => {
                let mut value = source_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&target_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                value.push(*severed as u8);
                ratio::encode(load, &mut value);
                tlv::put(34, &value, &mut out);
            }
            Self::CovenantSealed {
                obligor_orb,
                beneficiary_orb,
                resource,
                quota,
                window,
                collateral,
            } => {
                let mut value = obligor_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&beneficiary_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(quota, &mut value);
                value.extend_from_slice(&window.to_le_bytes());
                ratio::encode(collateral, &mut value);
                tlv::put(35, &value, &mut out);
            }
            Self::CovenantReleased { obligor_orb, beneficiary_orb, resource, collateral } => {
                tlv::put(
                    36,
                    &pact_layout(*obligor_orb, *beneficiary_orb, *resource, collateral),
                    &mut out,
                );
            }
            Self::CovenantSlashed { obligor_orb, beneficiary_orb, resource, collateral } => {
                tlv::put(
                    37,
                    &pact_layout(*obligor_orb, *beneficiary_orb, *resource, collateral),
                    &mut out,
                );
            }
            Self::FrictionSettled { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(38, &value, &mut out);
            }
            Self::TechLicensed { deal, from_orb, council, condition, node, name } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&from_orb.to_le_bytes());
                value.extend_from_slice(&council.to_le_bytes());
                value.push(*condition);
                value.extend_from_slice(node);
                value.extend_from_slice(name.as_bytes());
                tlv::put(40, &value, &mut out);
            }
            Self::CouncilSealed { council, resource, quota, window, collateral, members } => {
                let mut value = council.to_le_bytes().to_vec();
                value.push(*resource as u8);
                ratio::encode(quota, &mut value);
                value.extend_from_slice(&window.to_le_bytes());
                ratio::encode(collateral, &mut value);
                for member in members {
                    value.extend_from_slice(&member.to_le_bytes());
                }
                tlv::put(52, &value, &mut out);
            }
            Self::CouncilClosed { council, member, fulfilled, banked } => {
                let mut value = council.to_le_bytes().to_vec();
                value.extend_from_slice(&member.to_le_bytes());
                value.push(*fulfilled as u8);
                ratio::encode(banked, &mut value);
                tlv::put(53, &value, &mut out);
            }
            Self::GaugeShifted { gauge } => {
                let mut value = Vec::new();
                ratio::encode(gauge, &mut value);
                tlv::put(42, &value, &mut out);
            }
            Self::GaugeShiftedAt { grade, gauge } => {
                let mut value = grade.to_le_bytes().to_vec();
                ratio::encode(gauge, &mut value);
                tlv::put(60, &value, &mut out);
            }
            Self::WantStruck { orb_id, kind, reading } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*kind);
                ratio::encode(reading, &mut value);
                tlv::put(62, &value, &mut out);
            }
            Self::WantRelieved { orb_id, kind } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*kind);
                tlv::put(63, &value, &mut out);
            }
            Self::OfferPosted { offer, orb_id, give, give_amount, want, want_amount } => {
                let mut value = offer.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.push(*give as u8);
                ratio::encode(give_amount, &mut value);
                value.push(*want as u8);
                ratio::encode(want_amount, &mut value);
                tlv::put(49, &value, &mut out);
            }
            Self::OfferCleared { offer, deal } => {
                let mut value = offer.to_le_bytes().to_vec();
                value.extend_from_slice(&deal.to_le_bytes());
                tlv::put(50, &value, &mut out);
            }
            Self::Diverged { branch, gauge } => {
                let mut value = Vec::new();
                ratio::encode(gauge, &mut value);
                value.extend_from_slice(branch.as_bytes());
                tlv::put(43, &value, &mut out);
            }
            Self::Confluent { partner, gauge } => {
                let mut value = Vec::new();
                ratio::encode(gauge, &mut value);
                value.extend_from_slice(partner.as_bytes());
                tlv::put(44, &value, &mut out);
            }
            Self::StarAged { class } => {
                tlv::put(45, &[*class], &mut out);
            }
            Self::EraFolded { era, gauge } => {
                let mut value = era.to_le_bytes().to_vec();
                ratio::encode(gauge, &mut value);
                tlv::put(46, &value, &mut out);
            }
            Self::TechDiscovered { orb_id, node, condition, modifier, magnitude, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(node);
                value.push(*condition);
                value.push(*modifier);
                ratio::encode(magnitude, &mut value);
                value.extend_from_slice(name.as_bytes());
                tlv::put(39, &value, &mut out);
            }
            Self::ApertureSent { deal, from_orb, resource, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&from_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(47, &value, &mut out);
            }
            Self::ApertureReceived { deal, to_orb, resource, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(48, &value, &mut out);
            }
            Self::CrystalDrawn { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(55, &value, &mut out);
            }
            Self::LineSucceeded { successor } => {
                tlv::put(59, successor.as_bytes(), &mut out);
            }
            Self::GenesisSealed { civilization, disposition, metabolism, perception } => {
                let mut value = vec![*disposition, *metabolism, *perception];
                value.extend_from_slice(civilization.as_bytes());
                tlv::put(61, &value, &mut out);
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
            8 => {
                let seed: [u8; 32] = value.get(..32)?.try_into().ok()?;
                Some(Self::WorldCharted { seed })
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
            54 => {
                let mut at = 0usize;
                let mut take_text = || -> Option<String> {
                    let len =
                        u16::from_le_bytes(value.get(at..at + 2)?.try_into().ok()?) as usize;
                    at += 2;
                    let text = String::from_utf8(value.get(at..at + len)?.to_vec()).ok()?;
                    at += len;
                    Some(text)
                };
                let observer = take_text()?;
                let creator = take_text()?;
                let name = take_text()?;
                let (x, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (y, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (z, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (reach, _) = ratio::decode(value.get(at..)?)?;
                Some(Self::GalaxyObserved { observer, creator, name, center: (x, y, z), reach })
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
            31 => {
                let source_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let absorber_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::OverflowAbsorbed { source_orb, absorber_orb, amount })
            }
            32 => {
                let debtor_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let creditor_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::AbsorptionLapsed { debtor_orb, creditor_orb, amount })
            }
            33 => {
                let source_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let target_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (rate, used) = ratio::decode(value.get(17..)?)?;
                let term =
                    u64::from_le_bytes(value.get(17 + used..17 + used + 8)?.try_into().ok()?);
                Some(Self::BondSealed { source_orb, target_orb, resource, rate, term })
            }
            34 => {
                let source_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let target_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, used) = ratio::decode(value.get(17..)?)?;
                let severed = *value.get(17 + used)? != 0;
                let (load, _) = ratio::decode(value.get(18 + used..)?)?;
                Some(Self::BondSettled { source_orb, target_orb, resource, amount, severed, load })
            }
            35 => {
                let obligor_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let beneficiary_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (quota, used) = ratio::decode(value.get(17..)?)?;
                let window =
                    u64::from_le_bytes(value.get(17 + used..25 + used)?.try_into().ok()?);
                let (collateral, _) = ratio::decode(value.get(25 + used..)?)?;
                Some(Self::CovenantSealed {
                    obligor_orb,
                    beneficiary_orb,
                    resource,
                    quota,
                    window,
                    collateral,
                })
            }
            36 => {
                let (obligor_orb, beneficiary_orb, resource, collateral) = pact_fields(value)?;
                Some(Self::CovenantReleased { obligor_orb, beneficiary_orb, resource, collateral })
            }
            37 => {
                let (obligor_orb, beneficiary_orb, resource, collateral) = pact_fields(value)?;
                Some(Self::CovenantSlashed { obligor_orb, beneficiary_orb, resource, collateral })
            }
            38 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::FrictionSettled { orb_id, amount })
            }
            40 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let council = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let condition = *value.get(24)?;
                let node: [u8; 32] = value.get(25..57)?.try_into().ok()?;
                let name = String::from_utf8(value.get(57..)?.to_vec()).ok()?;
                Some(Self::TechLicensed { deal, from_orb, council, condition, node, name })
            }
            52 => {
                let council = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (quota, used) = ratio::decode(value.get(9..)?)?;
                let mut at = 9 + used;
                let window = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                at += 8;
                let (collateral, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let rest = value.get(at..)?;
                if rest.len() % 8 != 0 {
                    return None;
                }
                let members = rest
                    .chunks_exact(8)
                    .map(|chunk| u64::from_le_bytes(chunk.try_into().expect("8 bytes")))
                    .collect();
                Some(Self::CouncilSealed { council, resource, quota, window, collateral, members })
            }
            53 => {
                let council = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let member = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let fulfilled = *value.get(16)? != 0;
                let (banked, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::CouncilClosed { council, member, fulfilled, banked })
            }
            42 => {
                let (gauge, _) = ratio::decode(value)?;
                Some(Self::GaugeShifted { gauge })
            }
            60 => {
                let grade = u16::from_le_bytes(value.get(..2)?.try_into().ok()?);
                let (gauge, _) = ratio::decode(value.get(2..)?)?;
                Some(Self::GaugeShiftedAt { grade, gauge })
            }
            62 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind = *value.get(8)?;
                let (reading, _) = ratio::decode(value.get(9..)?)?;
                Some(Self::WantStruck { orb_id, kind, reading })
            }
            63 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind = *value.get(8)?;
                Some(Self::WantRelieved { orb_id, kind })
            }
            49 => {
                let offer = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let give = Resource::from_wire(*value.get(16)?)?;
                let (give_amount, used) = ratio::decode(value.get(17..)?)?;
                let at = 17 + used;
                let want = Resource::from_wire(*value.get(at)?)?;
                let (want_amount, _) = ratio::decode(value.get(at + 1..)?)?;
                Some(Self::OfferPosted { offer, orb_id, give, give_amount, want, want_amount })
            }
            50 => {
                let offer = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let deal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                Some(Self::OfferCleared { offer, deal })
            }
            43 => {
                let (gauge, used) = ratio::decode(value)?;
                let branch = String::from_utf8(value.get(used..)?.to_vec()).ok()?;
                Some(Self::Diverged { branch, gauge })
            }
            44 => {
                let (gauge, used) = ratio::decode(value)?;
                let partner = String::from_utf8(value.get(used..)?.to_vec()).ok()?;
                Some(Self::Confluent { partner, gauge })
            }
            45 => Some(Self::StarAged { class: *value.first()? }),
            46 => {
                let era = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (gauge, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::EraFolded { era, gauge })
            }
            39 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let node: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let condition = *value.get(40)?;
                let modifier = *value.get(41)?;
                let (magnitude, used) = ratio::decode(value.get(42..)?)?;
                let name = String::from_utf8(value.get(42 + used..)?.to_vec()).ok()?;
                Some(Self::TechDiscovered { orb_id, node, condition, modifier, magnitude, name })
            }
            47 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::ApertureSent { deal, from_orb, resource, amount })
            }
            48 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::ApertureReceived { deal, to_orb, resource, amount })
            }
            55 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::CrystalDrawn { orb_id, amount })
            }
            59 => {
                let successor = String::from_utf8(value.to_vec()).ok()?;
                Some(Self::LineSucceeded { successor })
            }
            61 => {
                let disposition = *value.first()?;
                let metabolism = *value.get(1)?;
                let perception = *value.get(2)?;
                let civilization = String::from_utf8(value.get(3..)?.to_vec()).ok()?;
                Some(Self::GenesisSealed { civilization, disposition, metabolism, perception })
            }
            _ => None,
        }
    }
}

/// The shared `LE64 ‖ LE64 ‖ resource u8 ‖ ratio` pact layout (36–37).
fn pact_layout(obligor: u64, beneficiary: u64, resource: Resource, collateral: &Ratio) -> Vec<u8> {
    let mut value = obligor.to_le_bytes().to_vec();
    value.extend_from_slice(&beneficiary.to_le_bytes());
    value.push(resource as u8);
    ratio::encode(collateral, &mut value);
    value
}

fn pact_fields(value: &[u8]) -> Option<(u64, u64, Resource, Ratio)> {
    let obligor = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
    let beneficiary = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
    let resource = Resource::from_wire(*value.get(16)?)?;
    let (collateral, _) = ratio::decode(value.get(17..)?)?;
    Some((obligor, beneficiary, resource, collateral))
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
    /// ABSORB-1 (09 §2.4): the morality books — cumulative value
    /// returned against cross-orb draws inside the reciprocity window,
    /// and the lapsed remainders. Same deferral as the compassion
    /// books: committed, replay-rebuilt, wire at IDEO-1.
    pub returned_across: Ratio,
    /// The entropic book of the pair (see `returned_across`).
    pub extracted_across: Ratio,
    /// BOND-1 (09 §2.2): the love books — cumulative value streamed
    /// over this orb's standing bonds (as source), and the severance
    /// loads posted when it cut a bond without notice (or defaulted).
    /// Same deferral: committed, replay-rebuilt, wire at IDEO-1.
    pub bond_flow: Ratio,
    /// The entropic book of the pair (see `bond_flow`).
    pub severance_load: Ratio,
    /// COVENANT-1 (09 §2.3): the integrity books — VALUE-weighted
    /// stakes of covenants released and slashed (a thousand toy
    /// promises never wash one giant breach). Same deferral:
    /// committed, replay-rebuilt, wire at IDEO-1.
    pub covenants_fulfilled: Ratio,
    /// The entropic book of the pair (see `covenants_fulfilled`).
    pub covenants_breached: Ratio,
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
        // IDEO-1: the cohesion entry — the eight books in fixed order,
        // emitted only when a book is nonzero (one state, one
        // encoding; a zero-book orb stays byte-identical to a
        // pre-ideology one).
        let books = self.cohesion_books();
        if books.iter().any(|book| **book != ratio(0, 1)) {
            let mut value = Vec::new();
            for book in books {
                ratio::encode(book, &mut value);
            }
            tlv::put(41, &value, &mut out);
        }
        out
    }

    /// IDEO-1: the eight cohesion books in their canonical wire order.
    fn cohesion_books(&self) -> [&Ratio; 8] {
        [
            &self.given_in_crisis,
            &self.drawn_in_crisis,
            &self.bond_flow,
            &self.severance_load,
            &self.covenants_fulfilled,
            &self.covenants_breached,
            &self.returned_across,
            &self.extracted_across,
        ]
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
                41 => {
                    let mut at = 0usize;
                    let mut books = Vec::with_capacity(8);
                    for _ in 0..8 {
                        let (book, used) = ratio::decode(value.get(at..)?)?;
                        books.push(book);
                        at += used;
                    }
                    let mut drain = books.into_iter();
                    orb.given_in_crisis = drain.next()?;
                    orb.drawn_in_crisis = drain.next()?;
                    orb.bond_flow = drain.next()?;
                    orb.severance_load = drain.next()?;
                    orb.covenants_fulfilled = drain.next()?;
                    orb.covenants_breached = drain.next()?;
                    orb.returned_across = drain.next()?;
                    orb.extracted_across = drain.next()?;
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

/// D3 (02 §2.5): the tile classes and their yield multipliers. Upkeep
/// never scales with terrain — rich tiles are strictly better, barren
/// ones a cost you zone over only when the map forces you.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileClass {
    Standard,
    Rich,
    Barren,
}

impl TileClass {
    pub fn multiplier(&self) -> Ratio {
        match self {
            Self::Standard => ratio(1, 1),
            Self::Rich => ratio(3, 2),
            Self::Barren => ratio(1, 2),
        }
    }

    /// The map glyph (02 §2.5): · standard, ▲ rich, ░ barren.
    pub fn glyph(&self) -> char {
        match self {
            Self::Standard => '·',
            Self::Rich => '▲',
            Self::Barren => '░',
        }
    }
}

/// The pure derivation: BLAKE3(seed ‖ LE64 orb ‖ LE16 tile), first
/// byte mod 64 → ~40 Standard, ~12 Rich, ~12 Barren (02 §2.5 — the
/// frequencies are approximate by construction, the derivation exact).
pub fn tile_class(seed: &[u8; 32], orb_id: u64, tile: u16) -> TileClass {
    let mut hasher = blake3::Hasher::new();
    hasher.update(seed);
    hasher.update(&orb_id.to_le_bytes());
    hasher.update(&tile.to_le_bytes());
    match hasher.finalize().as_bytes()[0] % 64 {
        0..=39 => TileClass::Standard,
        40..=51 => TileClass::Rich,
        _ => TileClass::Barren,
    }
}

