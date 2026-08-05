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
    ElevationBarred { settled: bool, coherent: bool, crossed: bool, aligned: bool },
    /// ELEVATE-1 — a climb moves STRICTLY upward in G (the orientation
    /// law: the fiber advances and never retreats).
    BadClimb,
    /// COUNCIL-1 — a council is three or more DISTINCT orbs, a
    /// positive quota and collateral, a real window, a fresh id.
    BadCouncil,
    /// MARKET-1 — an offer trades two DISTINCT resources in positive
    /// amounts under a fresh id.
    BadOffer,
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
    ForwardAlreadySealed,    /// BOOK-1 (19_LENS §16) — a book kind is content-addressed over its
    /// NORMALIZED name, so re-declaring one is not a second book. This
    /// IS the novelty gate: exact, unthresholded, and it names the book
    /// already standing rather than a distance.
    BookAlreadyMinted { name: String },
    /// BOOK-1 — the charter's four are law, not inventory: they cannot
    /// be minted over, and they cannot be archived by anyone.
    CharterBookNotMintable { name: String },
    /// BOOK-1 (§16c) — closing the covenant book must never be a way to
    /// stop being measured by it.
    CharterBookNotArchivable { name: String },
    /// BOOK-1 — a name that normalizes to nothing addresses nothing.
    BookNameEmpty,
    /// BOOK-1 — this orb keeps no such book.
    NoSuchBook,
    /// BOOK-1 (§16c) — an archived book is closed to WRITING. It keeps
    /// every entry it took and stays readable forever.
    BookClosedToWriting { name: String },
    /// BOOK-1 — a book already closed cannot be closed again.
    BookAlreadyArchived { name: String },
    /// BOOK-1 — a book ACCRUES. A negative posting would let an orb
    /// unwrite its own history, which is what the chronicle exists to
    /// prevent.
    BookPostingNegative,
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
    /// LIT-1 — a cited digest is not a receipt this world sealed. The
    /// law verifies the CITATION, never the virtue.
    WitnessNotInChronicle { digest: [u8; 32] },

}

