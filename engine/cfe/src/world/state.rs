//! THE STATE SHAPES — what a world IS, with no logic about what it
//! does.
//!
//! Data only. Split out so a deed module can name a shape without
//! pulling in the whole proposer, and so two lanes adding fields to
//! different shapes do not collide (GROUND §4b -- the parallel-lane
//! vent).

use crate::dop::{ColonyOrb, Resource};
use crate::ipsal::Entity;
use crate::ratio::Ratio;
use crate::vrp::{Chronicle, TickRead, Timekeeper, WorldClock};

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

/// FORWARD-1 (01 §7, 09 §2.3): one vitrified delivery pact — the
/// obligor's `collateral` sits OUT of its bank (the lock gates every
/// spend by absence; the matrix is a lossless vault by construction)
/// until `delivered ≥ quota` releases it home or the passed deadline
/// slashes it to the beneficiary. Committed state; every mutation
/// receipted; replays from receipts alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forward {
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

/// MARKET-1: a resting offer — a receipted intent to trade, no
/// escrow; the price IS want/give (guardrail 6: no absolute price).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offer {
    pub offer: u64,
    pub orb: u64,
    pub give: Resource,
    pub give_amount: Ratio,
    pub want: Resource,
    pub want_amount: Ratio,
    pub posted: u64,
}

/// COUNCIL-1 (12_UNION §7): a multilateral forward — every member
/// bound alike, gated together at the boundary.
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
    /// FORWARD-1: the open vitrified pacts, in sealing order.
    pub forwards: Vec<Forward>,
    /// COUNCIL-1: the open councils, in sealing order.
    pub councils: Vec<Council>,
    /// MARKET-1: the resting offers, in posting order.
    pub offers: Vec<Offer>,
    /// TECH-1a: the discovered nodes — (orb, condition, node id,
    /// RECEIPTED magnitude) in discovery order; condition 0 is the
    /// kernel solar leap. The magnitude is what the `TechDiscovered`
    /// receipt asserted (operator ruling 2026-08-05: receipt
    /// authority) — the tick's boon factors read IT, never a table.
    pub techs: Vec<(u64, u8, [u8; 32], Ratio)>,
    /// WANT-1: the struck cognitive wants — (orb, kind), replay-
    /// derived like famine; kinds per the tag-62 table.
    pub wants: Vec<(u64, u8)>,
    /// THRESHOLD-1: has this world CROSSED — does its chronicle hold
    /// an aperture leg? A world is a star system, so any leg is an
    /// interstellar deed; the first is the crossing (12_UNION §7).
    pub crossed: bool,
    /// GAUGE-N: the world's gauge KEY — a graded vector of wrapped
    /// exact rationals on the torus fiber, grown only by receipt.
    /// Grade 0 is the landed scalar; genesis is [0].
    pub gauge: Vec<Ratio>,
    /// G1 (04 §1/§5): the sealed line — the world-side copy of the
    /// binding receipt; None until the axes seal.
    pub genesis: Option<Genesis>,
    /// D3: the charted terrain seed — None reads all Standard (the
    /// opt-in law; the seed arrives only by the WorldCharted deed).
    pub terrain_seed: Option<[u8; 32]>,
    /// MANU-1: the forged instruments — (orb, tech node, grade) in
    /// forge order; reach law reads the best grade (10 §3f).
    pub instruments: Vec<(u64, [u8; 32], u8)>,
    /// ROUTE-1: this book's live lanes — (deal, eta); a PAIR exists
    /// where both books hold the deal.
    pub routes: Vec<(u64, crate::dop::Unit)>,
    /// E-ROUTE-1: element-specialized lanes — (deal, η, element, rail).
    pub element_routes: Vec<(u64, crate::dop::Unit, [u8; 32], u8)>,
    /// #42: this book's standing PACTS, seal order. A treaty is a PAIR
    /// object and neither world owns it — each carries its own leg, the
    /// aperture's deal-id pattern — so this is what THIS world has
    /// agreed to, and the partner's book carries the mirror.
    /// Replay-rebuilt; expiry is read from the clock, never stored.
    pub treaties: Vec<crate::treaty::Treaty>,
    /// PRACTICE-1: the receipted disciplines, adoption order.
    pub practices: Vec<u8>,
    /// ELEMENT-GRAMMAR-1 (14 §3): the opened veins — the working
    /// registry the assay draws against; replay-rebuilt.
    pub prospects: Vec<Prospected>,
    /// WARDEN-1a: the reverse shield stands (None = not the elder;
    /// Some(false) = warded; Some(true) = lifted).
    pub warded: Option<bool>,
    /// MASK-1a: the veil stands.
    pub veiled: bool,
    /// KERNEL-TAXON (15 §4): the sealed (class, clade), or None —
    /// unclassed is lawful until the seat.
    pub taxon: Option<(u8, u8)>,
    /// THE VERTICAL AXIS (tag 89, 2026-08-01): the seats held, as
    /// (line, station) in seating order. A station GATES AUTHORITY, and
    /// until this landing it lived only in an editable registry file —
    /// so who held an office was outside the constitution entirely
    /// (`KERNEL_RELAY` §1.5). The registry is now a CACHE from the
    /// receipt on, exactly as `GenesisSealed` made it for the axes.
    /// One seat per line: re-seating REPLACES, and the chain carries
    /// the whole history of who sat where.
    pub stations: Vec<(String, u8)>,
    /// FLEET-1: receipted hulls (replay of mint/assign/scrap).
    pub hulls: Vec<crate::dop::Hull>,
    /// PHASE-4: open market Notes (STAMP debt).
    pub notes: Vec<crate::dop::MarketNote>,
    /// PHASE-4: open market Futures.
    pub futures: Vec<crate::dop::MarketFuture>,
    /// PHASE-4: last-clear marks (resource → STAMP price), latest wins.
    pub last_clears: Vec<(crate::dop::Resource, Ratio)>,
    /// SHARE-1 open issues.
    pub shares: Vec<crate::dop::MarketShare>,
    /// PERP-1 open positions.
    pub perps: Vec<crate::dop::MarketPerp>,
    /// P5-M: honest clear depth per resource (for last-clear seal).
    pub clear_depths: Vec<(crate::dop::Resource, u64)>,
    /// P5-M: circuit breaker — if Some(t), books sealed until clock ≥ t.
    pub market_sealed_until: Option<u64>,
    /// P5-M2: note cascade pressure per orb (haircut ladder).
    pub cascade_pressures: Vec<(u64, u64)>,
    /// P5-D: ATM tamper pressure per orb (exact rational ≥ 0).
    pub tamper_pressures: Vec<(u64, Ratio)>,
    /// P5-B: active doctrine disposition (None = use genesis).
    pub posture_disposition: Option<u8>,
    /// P5-B: active perception stance (None = use genesis).
    pub posture_perception: Option<u8>,
    /// P5-B: primary domain override 1..=7 (None = metabolism default).
    pub posture_primary: Option<u8>,
    /// P5-F: exfiltrated tech hashes (intel stash — never free techs mint).
    pub intel_hashes: Vec<[u8; 32]>,
    /// P5-F: probe depth marks per orb (scout residual).
    pub spy_marks: Vec<(u64, u8)>,
    /// O6.5: STAMP held in Vault custody per orb (not free bank).
    pub vault_stamp: Vec<(u64, Ratio)>,
    /// CHEM science: cataloged gem identities (known chemistry).
    pub chem_catalog: Vec<[u8; 32]>,
}

/// G1 (04 §1): a civilization's sealed axes, as the world holds them.
/// The registry caches this; it never grounds it (04 §5 binding 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genesis {
    pub civilization: String,
    pub disposition: u8,
    pub metabolism: u8,
    pub perception: u8,
}

/// ELEMENT-GRAMMAR-1 (14 §3): one opened vein — the prospect's
/// working state. Family, tier, and name live in the receipt (the
/// chronicle is the full register); the working set carries what the
/// assay needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prospected {
    pub orb: u64,
    pub stratum: u8,
    pub element: [u8; 32],
    pub richness: Ratio,
    /// The last assay's stamp (the prospect's own at opening) — the
    /// span anchor, the bond-settle shape.
    pub last_assay: u64,
}

pub struct World {
    pub(super) clock: WorldClock,
    pub(super) keepers: [Timekeeper; 3],
    pub(super) chronicle: Chronicle,
    pub(super) orbs: Vec<ColonyOrb>,
    pub(super) entities: Vec<Entity>,
    /// THERMAL-1: the container's sun (the world IS the star system,
    /// NS-6 §6); ABSENT until commissioned — thermal stays dormant.
    pub(super) star: Option<crate::thermal::StarClass>,
    /// WORKSPACE, per orb: net flow since that orb's last collection —
    /// no receipt, no persistence, not committed.
    pub(super) accrued: Vec<(u64, Resource, Ratio)>,
    /// VENT-1 WORKSPACE, per orb: vented entropy since last settle —
    /// sealed into the cumulative ledger by `VentSettled` at collect,
    /// or carried into a partner's sink by ABSORB-1.
    pub(super) vent_pending: Vec<(u64, Ratio)>,
    /// ABSORB-1: the open reciprocity windows (committed — every
    /// mutation receipted; replays from receipts alone).
    pub(super) debts: Vec<AbsorptionDebt>,
    /// BOND-1: the standing orders (committed, same law).
    pub(super) bonds: Vec<Bond>,
    /// FORWARD-1: the open vitrified pacts (committed, same law).
    pub(super) forwards: Vec<Forward>,
    /// COUNCIL-1: the open councils (committed, same law).
    pub(super) councils: Vec<Council>,
    /// MARKET-1: the resting offers (committed, same law).
    pub(super) offers: Vec<Offer>,
    /// IDEO-1 WORKSPACE, per orb: accrued ideological friction since
    /// the last seal — sealed by `FrictionSettled` at collect.
    pub(super) friction_pending: Vec<(u64, Ratio)>,
    /// TECH-1a: the discovered nodes with their RECEIPTED magnitudes
    /// (committed — replays from receipts alone); condition 0 is the
    /// kernel solar leap.
    pub(super) techs: Vec<(u64, u8, [u8; 32], Ratio)>,
    /// #41 WORKSPACE: the shear cursor — (receipts processed, carried
    /// sheet). A pure cache over receipts: never committed, never
    /// persisted; a resumed world rebuilds it at its first boundary.
    pub(super) shear_cursor: (usize, Vec<(u64, CrystalDeposit)>),
    /// WANT-1: the struck cognitive wants (committed, famine-shaped).
    pub(super) wants: Vec<(u64, u8)>,
    /// THRESHOLD-1: the crossing flag (committed, replay-derived).
    pub(super) crossed: bool,
    /// GAUGE-N: the graded gauge key (committed, replay-derived,
    /// every plane wrapped; len ≥ 1, grade 0 first).
    pub(super) gauge: Vec<Ratio>,
    /// G1: the sealed line, if any (04 §1/§5).
    pub(super) genesis: Option<Genesis>,
    /// D3: the charted terrain seed (replay of the receipt).
    pub(super) terrain_seed: Option<[u8; 32]>,
    /// PRACTICE-1: the receipted disciplines (replay of the receipts).
    pub(super) practices: Vec<u8>,
    /// MANU-1: the forged instruments (replay of the receipts).
    pub(super) instruments: Vec<(u64, [u8; 32], u8)>,
    /// ROUTE-1: the live lanes (replay of the receipts).
    pub(super) routes: Vec<(u64, crate::dop::Unit)>,
    /// E-ROUTE-1: element-specialized lanes (replay of the receipts).
    pub(super) element_routes: Vec<(u64, crate::dop::Unit, [u8; 32], u8)>,
    /// #42: the standing pacts (replay of the receipts).
    pub(super) treaties: Vec<crate::treaty::Treaty>,
    pub(super) prospects: Vec<Prospected>,
    /// WARDEN-1a: the elder's reverse shield (replay of the receipts).
    pub(super) warded: Option<bool>,
    /// MASK-1a: the veil (replay of the receipts).
    pub(super) veiled: bool,
    /// KERNEL-TAXON: the sealed taxon (replay of the receipt).
    pub(super) taxon: Option<(u8, u8)>,
    /// THE VERTICAL AXIS (tag 89): the seats held, (line, station).
    pub(super) stations: Vec<(String, u8)>,
    /// FLEET-1 hulls.
    pub(super) hulls: Vec<crate::dop::Hull>,
    pub(super) notes: Vec<crate::dop::MarketNote>,
    pub(super) futures: Vec<crate::dop::MarketFuture>,
    pub(super) last_clears: Vec<(crate::dop::Resource, Ratio)>,
    pub(super) shares: Vec<crate::dop::MarketShare>,
    pub(super) perps: Vec<crate::dop::MarketPerp>,
    pub(super) clear_depths: Vec<(crate::dop::Resource, u64)>,
    pub(super) market_sealed_until: Option<u64>,
    pub(super) cascade_pressures: Vec<(u64, u64)>,
    pub(super) tamper_pressures: Vec<(u64, Ratio)>,
    pub(super) posture_disposition: Option<u8>,
    pub(super) posture_perception: Option<u8>,
    pub(super) posture_primary: Option<u8>,
    pub(super) intel_hashes: Vec<[u8; 32]>,
    pub(super) spy_marks: Vec<(u64, u8)>,
    /// O6.5: Vault custody STAMP (replay of deposit/withdraw).
    pub(super) vault_stamp: Vec<(u64, Ratio)>,
    /// CHEM science: cataloged gems (replay of ChemCataloged).
    pub(super) chem_catalog: Vec<[u8; 32]>,
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

/// FORWARD-1: a deliberate delivery (transfer or bond settlement)
/// from obligor to beneficiary credits the open pact on that lane —
/// at most one stands per (obligor, beneficiary, resource). Shared
/// verbatim by the live paths and replay.
/// TECH-1b: one orb's crystal deposit — accumulated in a single
/// chronicle sweep (the shear), read at the year boundary.
#[derive(Default, Clone)]
pub(super) struct CrystalDeposit {
    pub(super) famine_total: u64,
    pub(super) famine_open: Option<u64>,
    pub(super) overheat_open: Option<u64>,
    /// Windowed entries (stamp, amount) — pruned at each boundary so
    /// the carried sheet stays bounded (the #41 cursor law).
    pub(super) vented_entries: Vec<(u64, Ratio)>,
    pub(super) encased_entries: Vec<(u64, Ratio)>,
    /// WANT-1: the last cross-world LINK (aperture leg, bond seal or
    /// settle) and whether one ever stood — the resonance reading.
    pub(super) last_link: Option<u64>,
    /// WANT-1: the last discovery and whether one ever landed — the
    /// novelty reading (licensed arrivals count: foreign signal FEEDS).
    pub(super) last_node: Option<u64>,
}

