//! THE REFUSAL VOCABULARY — the world refuses instead of guessing.
//!
//! One enum, no logic. Split out so the deed modules can name a
//! refusal without importing the world's state, and so adding a
//! refusal does not touch a file anyone else is editing.

use crate::dop::{Resource, ZoneKind};
use crate::ipsal::Scope;
use crate::ratio::Ratio;

/// A typed action refusal — the world refuses instead of guessing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionRefused {
    /// K4 THE LIVE GATE: the payload is not a proof. The emitter asks a
    /// `MintWarrant` what the work earns and refuses BEFORE closing, so
    /// an unverifiable payload never reaches the chronicle. Previously
    /// the kernel refused it afterwards, by which point the receipt
    /// stood and `replay` granted the value.
    WorkRefused,
    /// K4: the amount exceeds what the verified work earns. Bounded at
    /// the emitter, both sides carried.
    MintExceedsWork { bound: Box<Ratio>, claimed: Box<Ratio> },

    /// The angle is not ON a slot. A turn divides into sixty
    /// (`dop::TURN_SLOTS`); an orbit sits at a slot or it does not sit.
    /// Off-slot placement makes collision a matter of tolerance rather
    /// TWO ORBS CANNOT SHARE A PLACE. Same radius, same slot — the
    /// tension this law exists to resolve.
    OrbitOccupied { orb: u64, slot: i64 },

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
    /// NOVEL-1 (10 §3c, 07 §3 triage) — succession is the derelict
    /// door, never a reset button: it refuses while any orb still
    /// stands unfamished (v1 cascade law: EVERY orb famished).
    LineStillLiving { orbs: usize, famished: usize },
    /// G1 — one line per world: the axes seal once.
    GenesisAlreadySealed,
    /// G1 — the axis vocabulary is closed (04 §1, 12 §5a): an unknown
    /// code refuses at the door, naming the axis it offended.
    UnknownAxisValue { axis: u8, code: u8 },
    /// ELEMENT-GRAMMAR-1 — one stratum, one reading: what the seed
    /// held is already read.
    StratumProspected { orb: u64, stratum: u8 },
    /// ELEMENT-GRAMMAR-1 — no prospect on this orb carries that
    /// identity; the assay reads only opened veins.
    NoSuchElement { element: [u8; 32] },
    /// THERMAL-1 — one sun per world; it is already commissioned.
    StarAlreadyCommissioned,
    /// D3 — one terrain charter per star; the seed is forever.
    TerrainAlreadyCharted,
    /// ROUTE-1 — this deal id already charters a lane on this book.
    RouteKnown { deal: u64 },
    /// MARKET-2 (Samuelson iceberg, ruled 2026-07-31) — the melt
    /// fraction reached 1: this pair is economically walled off by
    /// decay physics, not by rule.
    EconomicallyWalled { melt: Box<Ratio> },
    /// MANU-1 — an instrument grade is a crystallized condition, 1–6.
    InstrumentGradeUnknown { condition: u8 },
    /// MANU-1 — the line has not crystallized this condition here.
    ConditionUncrystallized { condition: u8 },
    /// MANU-1 — the converter chain does not stand on this orb.
    ChainIncomplete { needs: ZoneKind },
    /// PRACTICE-1 — outside the practice vocabulary (1–5).
    PracticeUnknown { practice: u8 },
    /// PRACTICE-1 — one adoption per discipline per star.
    PracticeAlreadyAdopted { practice: u8 },
    /// PRACTICE-1 — the line does not hold this discipline.
    PracticeNotHeld { practice: u8 },
    /// PRACTICE-1 — release is a REDISCOVERY: no new doctrine has
    /// landed since the adoption; the lesson is unearned.
    ReleaseUnearned { practice: u8 },
    /// PRACTICE-1 — the Overdrive takes only the tempered: fewer than
    /// eight hardship receipts stand in this chronicle.
    OverdriveUntempered { hardships: usize },
    /// KERNEL-TAXON — a code outside the ratified vocabularies
    /// (15 §2–§3): the vocabulary grows by amendment, never by deed.
    TaxonUnknown { class: u8, clade: u8 },
    /// KERNEL-TAXON — one seal per star; identity is forever.
    TaxonAlreadySealed,
    /// **THE COVENANT IS A T3 INSTRUMENT** (operator ruling 2026-08-05:
    /// *drop covenant to T3 and below*). A line holding a chartered
    /// OFFICE does not covenant — it decrees, delegates, or petitions
    /// through the council. The forward obligation is how the COMMON
    /// band binds: parties with no chartered authority over each other
    /// reach agreement by posting collateral against a deadline, which
    /// is the whole reason collateral exists in it.
    ///
    /// The refusal names the seat that bars it, so the way down is
    /// legible: release the office and the instrument opens.
    CovenantAboveTheCommon { line: String, station: u8 },
    /// 28_VOCABULARY §5 — minting is a TIER 0 power (`12_UNION` ruling
    /// 4: progenitors *create the board everyone else plays on*). The
    /// line named holds no progenitor seat in this chronicle.
    MintWithoutStanding { line: String },
    /// 28_VOCABULARY §7 — no such vocabulary. Note there is no axis for
    /// STATIONS: minting an office would let a T0 seat manufacture the
    /// seat that gates it, so offices change by amendment alone (§10).
    VocabularyAxisUnknown { axis: u8 },
    /// 28_VOCABULARY §4a — the binding does not TRADE: it must carry at
    /// least one benefit and at least one friction, every factor inside
    /// the bounds, every channel one the engine already computes. A
    /// strictly-dominant entry is unmintable by construction.
    BindingUnlawful,
    /// 28_VOCABULARY §6 — the mechanism already exists under another
    /// name. Word and mark differences are not novelty; a partition
    /// whose members compute identically distinguishes nothing.
    MintNotNovel { like: u8 },
    /// 28_VOCABULARY §6 — the mark or the word is already taken on this
    /// axis. A mark that names two things defines neither.
    MintNameTaken { word: String },
    /// WARDEN-1a — the elder's reverse shield: no aperture reaches a
    /// warded star; solve the gate.
    TheWardHolds,
    /// WARDEN-1a — only one elder deed per star, in order: ward once,
    /// lift once, lift only what is warded.
    WardOutOfOrder,
    /// WARDEN-1a — the depth gate: the claimer's recorded depth stands
    /// below the ceiling.
    DepthBelowCeiling { depth: u32, ceiling: u32 },
    /// MASK-1a — one veil per star; lift before drawing anew.
    VeilOutOfOrder,
    /// THERMAL-1 — siting an orbit needs a commissioned sun first.
    NoStarCommissioned,
    /// THERMAL-1 — the orb is already sited; an orbit does not move.
    AlreadySited { orb: u64 },
    /// The destination already holds this tech node — a FACT that wore
    /// the dead BadTransfer's word for want of its own.
    NodeAlreadyHeld { condition: u8 },
    /// Licensing asked for a node outside the crystallized tier and the
    /// crown — derived from `node_kind`, never a magic range.
    NodeUnlicensable { condition: u8 },
    /// THRESHOLD-1 (12_UNION §7) — the shield holds: a people that has
    /// not made the solar leap cannot be reached by aperture, in
    /// either direction. The flags name which side stands shielded —
    /// the young are left to make the leap themselves, or not.
    Shielded { source_shielded: bool, dest_shielded: bool },
    /// ELEVATE-1 (12_UNION §6) — the climb refused, every gate's
    /// reading named: settle the books, close the loop, cross the
    /// void. Stagnation is a region, not a sentence — each false here
    /// is escapable.
    ElevationBarred { settled: bool, coherent: bool, crossed: bool, aligned: bool },
    /// COUNCIL-1 — this id already seals a council on this book. The
    /// SHAPE half of the dead `BadCouncil` (short or repeated members,
    /// non-positive quota or collateral, a zero window) is
    /// unrepresentable now — `Among`, `Positive` and `Window` proved
    /// it at the doors; the fresh-id FACT keeps its own name.
    CouncilKnown { council: u64 },
    /// MARKET-1 — this id already rests on the board. The dead
    /// `BadOffer`'s shape half (reflexive trade, non-positive amounts)
    /// died at the `Exchange` and `Positive` doors.
    OfferKnown { offer: u64 },
    /// MARKET-1 — no offer with this id rests on this book.
    NoSuchOffer { offer: u64 },
    /// MARKET-1 — no crossing: the two offers do not meet (wrong
    /// resources, or the prices never cross).
    NoCrossing,
    /// GAUGE-N — a new plane is EARNED: opening grade k needs k
    /// receipted mints (crystallized/granted nodes, divergences,
    /// confluences). The count names how far the fiber has grown.
    PlaneUnearned { grade: u16, mints: u64 },
    /// PLANNER-1 — delegation settles inside diplomacy: the deed must
    /// reference an OPEN council in the granting world.
    NoSuchCouncil { council: u64 },
    /// PLANNER-1 — the solar leap is NEVER licensed: the young meet
    /// their sun themselves, or not (the shield's whole meaning).
    SolarLeapNeverLicensed,
    /// PLANNER-1 — a licensed leap lands only on a union member: the
    /// receiving world must have made its crossing (THRESHOLD-1).
    BelowTheThreshold,
    /// TERRA-1 — terraforming is the gifted crown: it licenses only
    /// onto a COMPLETE base-7 stack (the solar leap + all five
    /// crystallized conditions, earned by the line itself).
    BaseSevenUnfinished,
    /// TERRA-1 — the orbit moves only under the crown technology.
    TerraformUnearned,
    /// ABSORB-1 — only un-sealed pending overflow absorbs; the refusal
    /// names exactly what stands.
    CannotAbsorb { pending: Box<Ratio> },
    /// BOND-1 — one bond per (source, target, resource); the standing
    /// one must be severed before a new rate is sealed.
    BondAlreadySealed,
    /// BOND-1 — no bond stands on this (source, target, resource).
    NoSuchBond,
    /// FORWARD-1 — one OPEN forward per (obligor, beneficiary,
    /// resource); a vitrified pact cannot be cancelled, only run out.
    ForwardAlreadySealed,
    /// BOOK-1 (19_LENS §16) — a book kind is content-addressed over its
    /// NORMALIZED name, so re-declaring one is not a second book. This
    /// IS the novelty gate: exact, unthresholded, and it names the book
    /// already standing rather than a distance. (The empty normal form
    /// and the charter-mint collision died at the `BookName` door; the
    /// negative posting died at the `NonNegative` doors —
    /// 29_CONSTRUCTION.)
    BookAlreadyMinted { name: String },
    /// BOOK-1 (§16c) — closing the covenant book must never be a way to
    /// stop being measured by it. Archiving is addressed by KIND, not
    /// by name, so this stays a fact of the charter table at the verb.
    CharterBookNotArchivable { name: String },
    /// BOOK-1 — this orb keeps no such book.
    NoSuchBook,
    /// BOOK-1 (§16c) — an archived book is closed to WRITING. It keeps
    /// every entry it took and stays readable forever.
    BookClosedToWriting { name: String },
    /// BOOK-1 — a book already closed cannot be closed again.
    BookAlreadyArchived { name: String },
    /// LIT-1 — the works are CHARTERED. A line invents accounts
    /// (BOOK-1); it does not invent the literature it reaches for.
    NoSuchWork { name: String },
    /// LIT-1 — literacy is DERIVED: a line that has not kept its own
    /// books has not learned to write, and has no business reaching
    /// for the literature.
    NotLiterate,
    /// LIT-1 — one reach per work.
    WorkAlreadyAspired { name: String },
    /// LIT-1 — a work is reached for before it is claimed; skipping
    /// that would let a line claim what it never set out to do.
    WorkNotAspired { name: String },
    /// LIT-1 — a claim stands once. Re-citing would let evidence be
    /// swapped after the fact.
    WorkAlreadyClaimed { name: String },
    /// LIT-1 — a claim without receipts is a boast. The chronicle is
    /// what separates the two.
    ClaimWithoutWitness { name: String },
    /// THE RING REACH (2026-08-06) — this star's field anchors no more
    /// rings. A ring is a distinct orbit radius; the reach derives from
    /// the star's MAGNETISM (`StarClass::ring_reach`). Slots within an
    /// anchored ring stay open — the refusal is for a NEW ring beyond
    /// the field, never for filling one the field already holds.
    RingReachExceeded { held: usize, reach: u8 },
    /// CONVEY (2026-08-06) — the ask outruns the sender's MONETARY
    /// portion. The free portion of a parcel is held without
    /// denomination and stays home; only the monetary break travels
    /// the post.
    ConveyExceedsMonetary { asked: Box<Ratio>, monetary: Box<Ratio> },
    /// THE COUNTER (2026-08-06) — the cited appraisal is STALE: the
    /// world moved past its stamp, so the evidence must be re-taken
    /// before the renegotiation proceeds. Staleness gates ONLY this —
    /// a counter's citation — never acceptance, which is consent.
    StaleAppraisalCited { appraisal: [u8; 32] },
    /// LIT-1 — a cited digest is not a receipt this world sealed. The
    /// law verifies the CITATION, never the virtue.
    WitnessNotInChronicle { digest: [u8; 32] },

}

