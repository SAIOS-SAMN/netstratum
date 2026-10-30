//! The kernel — the one validated update function (Charter §11 rule 3).
//!
//! `admits(prior, next, step)` gates whether one step lawfully carries
//! the universe from `prior` to `next`. It is:
//!
//! - **pure** — same inputs, same verdict, forever;
//! - **total** — every input returns `Ok` or a typed `Refused`, never
//!   panics, never hangs;
//! - **no I/O** — it touches no file, clock, socket, or thread;
//! - **encoding-blind** — it never produces or reads bytes.
//!   Serialization is the codec's monopoly (the arbor), so the kernel
//!   and the chain can never disagree about bytes: only one component
//!   ever produces them. The kernel gates meaning; the codec fixes
//!   form; the two never overlap.
//!
//! Conservation, containment, and exactness are invariants inside this
//! function — an unlawful transition is not caught after the fact, it
//! cannot be constructed. This is why Charter §11 dissolves every
//! reconciliation instrument: what cannot diverge needs no
//! reconciling.
//!
//! **Versioned like the charter.** [`VERSION`] is bumped only by
//! amendment. Every chain commitment binds the version that gated the
//! step, so old history verifies under the gate it was cut with — no
//! future change reinterprets the past. The kernel never reads the
//! commitment; the arbor binds `VERSION` into it.
//!
//! **Coverage grows by amendment, never silently.** Each event's rules
//! land as a witnessed addition (one witness per invariant, red-first);
//! Charter §11 clause 2 guards that the growth is amendment-shaped and
//! that no state semantics ever live outside this function.

pub mod bridge;
pub mod carrier_table;
pub mod motion;

use cfe::dop::{ClosureEvent, Office, Resource, UNION_REVIEW_HORIZON};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, SOVEREIGN};
use solver_verify::{verify_work_class, WorkBroken, WorkClass};

/// The kernel version — bumped only by charter amendment. The arbor
/// binds this into every commitment (`H(schema ‖ bytes ‖ parent ‖
/// coordinate)` carries the judging version alongside the schema), so
/// a chain segment is forever verified under the version that cut it.
///
/// VERSION 2 — ruling box 1 (operator, 2026-08-01): `mint_bound`
/// unsealed, the emission rate `f` ladder-pinned against the #17
/// calibration (evidence commit 5898351). Version-1 history gated
/// every positive discovery mint refused; it verifies forever under
/// its own law.
///
/// VERSION 3 — the capacity law (operator ruling 2026-08-01), now
/// struck. It refused a `DiscoveryMinted` unless the prior state read
/// `Vacuity::Entangled` — structure and flow, both standing. Kept in
/// this record because version-3 history is gated by version-3 law
/// forever; the note below says why the law did not survive.
///
/// **The bump was announced and never landed.** The commit that
/// introduced the law was titled *"VERSION 3 now means something"* and
/// left the constant at 2, so the chain would have bound version 2 to
/// steps that version 2 would have gated differently — the one thing
/// this constant exists to prevent. Version-2 history verifies forever
/// under version 2's law.
/// VERSION 5 — two amendments, landed together AT the merge.
///
/// **VERSION 4 — the capacity law is removed** (operator ruling
/// 2026-08-04, harness lane). Breaking in the loosening direction:
/// every state version 3 refused as `MintWithoutStanding` or
/// `MintWithoutOwnStanding` is now gated on the mint's real anchor
/// alone. *Identity is pegged to entity, and identity gets recycled
/// without bonds* — the capacity law read standing off relational
/// structure, so it demanded of a world exactly what a recycled
/// identity does not carry. Measured before the ruling: a 60-beat era
/// produced **374 solves, 0 mints, 124 withheld**. What still anchors
/// the mint is U1's REPLAY-COMPLETE law, untouched.
///
/// **VERSION 5 — D4, the revocation law lands** (operator ruling
/// 2026-08-04, cognition lane). A `DiscoveryMinted` deed refuses unless
/// it carries a strictly positive capacity increment, over a resolved
/// rank reading. See [`revocation_law`].
///
/// **Both numbers are kept and neither is ambiguous.** They were minted
/// on separate branches for separate reasons; 4 was skipped on main so
/// the two could not collide, and the merge lands both rather than
/// letting one bump overwrite the other. A chain cut under 3 verifies
/// forever under the gate that cut it.
/// **0.5.1 — the opened Ground** (operator ruling 2026-08-05: *Ground
/// needs to be opened up into V = Volume of orbs, weight on D as
/// capacity*). `D = volume(orbs) × (GROUND + circulation)` — the floor
/// a state stands on is the measure of its estate, and founding an orb
/// multiplies the whole degree.
///
/// **Patch, and the class is derived rather than chosen by feel.** The
/// version rule says patch = a change that cannot alter what the gate
/// admits or refuses. Proven at the type: every gate arm reads
/// `Degree::above()`, which returns the circulation and is volume-blind
/// by construction (`revocation_law`'s own comment already barred
/// `get()`); the fold gate compares rank against rank — identical to
/// the old floor-to-floor read on all inputs including adversarial ones
/// (an orb smuggled into `next` cannot pad an increment, because no
/// increment reads the floor). What changes is the reading — D on the
/// board grows with the estate — and one refusal's reported number,
/// which the rule itself lists as patch-class.
///
/// **0.6.0 — the magnetic REACH** (operator ruling 2026-08-06: planet
/// capacity is based on the star's magnetism). Minor, derived: the
/// gate now refuses `RingBeyondReach` where it previously admitted —
/// a `PlanetSited` threading a ring beyond what the star's field
/// anchors. What the gate admits changed, so `minor` moves; the opened
/// ground (0.5.1) rides beneath it as the patch it was.
///
/// **0.7.0 — the appraisal** (operator rulings 2026-08-06: appraise
/// before pricing; staleness triggers renegotiation, consent closes
/// acceptance). Minor, derived: tag 98 enters `covered()` — the gate
/// now admits (and can refuse `AppraisalOutsideSurvey` /
/// `AppraisalMisvalued`) a deed that was `Unjudged` before. What the
/// gate admits changed, so `minor` moves.
///
/// **0.8.0 — the counter-offer** (operator ruling 2026-08-06: the third
/// outcome). Minor, derived: tag 99 enters `covered()` — the gate
/// admits a deed that was `Unjudged`. Staleness gates nothing at this
/// tier by design: it refuses only a counter's citation, at the
/// emitter, where the chronicle lives.
///
/// **0.9.0 — convey** (operator: complete the estate chain). Minor,
/// derived: tag 100 enters `covered()` with a full I1 arm — the gate
/// admits what was `Unjudged` and refuses three new ways (address, cap,
/// postage), each by calling the same sealed rules the emitter drew.
///
/// **0.10.0 — the minted LINE** (the standing ruling: the engine mints
/// its peoples). Minor, derived: tag 101 enters `covered()` — the gate
/// admits what was `Unjudged`. The receipt is a seed; there is nothing
/// declarable to misdeclare, so no new refusal arm exists.
/// (Amendments 0.11.0 through the current minor are recorded in the
/// version ledger — `tests/conservation_pins.rs` — beside the
/// next-free RULE that assigns minors across lanes.)
///
/// **0.25.0 — orbital / stratum appraisal** (35_ORBIT_DYNAMO_AND_ESTATE).
/// Minor, derived: appraisal subjects scale 4–5 (Orbital place under the
/// dynamo; prospected Stratum) enter the valuation rule; the gate
/// re-derives with star magnetism + prospects. Same tag 98; what the
/// rule can value expanded, so `minor` moves.
///
/// **0.26.0 — PHASE-4 notes / futures** (32 + TASKS_TRADE_FLEET_DESK P4).
/// Minor: tags 120–124 enter `covered()` (NoteIssued/Redeemed,
/// FutureOpened/Settled, LastClearMarked). SCHEMA_V8 trail.
///
/// **0.27.0 — PHASE-4 SHARE/PERP/HULL finance.** Tags 125–131; SCHEMA_V9.
///
/// **0.28.0 — P5-A KIN escort/interdiction.** Tags 132–134.
///
/// **0.29.0 — P5-M MKT market defense.** Tags 135–139 (MarketSealed/
/// MarketResumed/SpotClearRecorded/LiquidityRaidStruck/ClearPoisonRefused);
/// SCHEMA_V10 clear_depths + market_sealed_until trail.
///
/// **0.30.0 — P5-M2 + P5-D.** Tags 140–143 (NoteCascadeStruck,
/// CovertTamperStruck, MediumReinforced, TamperDetected); SCHEMA_V11
/// cascade_pressure + tamper_pressures trail.
///
/// **0.31.0 — P5-B + P5.19 dual-use.** Tag 144 PostureReoriented;
/// SCHEMA_V12 posture doctrine overlay; terra dual-use tax on CovertTamper.
///
/// **0.32.0 — P5-F espionage.** Tags 145–147 SpyProbed / TechExfiltrated /
/// InstrumentSabotaged; SCHEMA_V13 intel_hashes + spy_marks.
///
/// **0.33.0 — P5-G rail promote + P5.19b medium resource on reinforce.**
/// Tag 148 RailPromoted; MediumReinforced carries spend resource.
///
/// **0.34.0 — X7 Anchor adapters.** Tags 149–151 StrandExported /
/// AnchorImported / AnchorImportRefused; stamp burn/credit on export/import.
///
/// **0.35.0 — Wave-6 offices as play.** Tags 152–155 VaultDeposited /
/// VaultWithdrawn / AssayConsilience / AssaySlashed; SCHEMA_V14 vault_stamp;
/// O6.3 Exchange MM gate on instrument issue.
///
/// **0.36.0 — Chemistry science path.** Tags 156–158 IsoUnionStruck /
/// ChemCataloged / DarkPullStruck; SCHEMA_V15 chem_catalog; civ utility
/// + dark-7 gravity (pure almanac::science).
///
/// **0.37.0 — Seated science labs.** Tags 159–162 ScienceSeated /
/// LabCreated / ScienceExploded / RogueFactionStruck; SCHEMA_V16 lab
/// state; dual ± outcomes + derived explosion risk (no free dials).
pub const KERNEL_VERSION: Version = Version { major: 0, minor: 37, patch: 0 };

/// The gate's version, as it rides the wire.
///
/// **`x.y.z`, and `major = 0` because this is a prototype** (ruling of
/// 2026-08-04: the version drops to x.x.x while the project is still a
/// prototype). The five linear amendments become `0.1.0 … 0.5.0`,
/// so the count is preserved in `minor` and the leading `0` says
/// plainly that nothing here has promised stability yet.
///
/// **What each field means, so the next bump is not a judgement call:**
///
/// ```text
/// major  reserved for the first stable gate. Still 0.
/// minor  a LAW change — the gate admits or refuses something it did
///        not before. Every amendment so far is one of these.
/// patch  a change that cannot alter any verdict: a refusal's wording,
///        a split of one rule into named parts, a faster read.
/// ```
///
/// The split of D4 into [`d4a_resolution`], [`d4b_standing_claim`] and
/// [`d4c_increment`] is the first `patch`-shaped change: three named
/// rules where there was one, judging identically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    /// The wire form. Packed so order is preserved — a later version is
    /// a larger `u64`, which is what a peer comparing two commitments
    /// needs and all it needs.
    ///
    /// The wire field stays `u64` and `Commitment.kernel` is untouched,
    /// so this is not a wire amendment. Chains cut under the linear
    /// versions 1–5 carry those small integers and still compare below
    /// every packed version, which is true and is the intended ordering:
    /// they were cut by an earlier gate.
    pub const fn code(self) -> u64 {
        ((self.major as u64) << 32) | ((self.minor as u64) << 16) | self.patch as u64
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The packed gate version bound into every commitment.
pub const VERSION: u64 = KERNEL_VERSION.code();

/// One proposed transition: the deed that claims to carry `prior` to
/// `next`. `event` is what closed, `actor` is who under NS-6
/// containment, `stamp` is the world-clock value it is stamped with.
/// The kernel gates whether the claim is lawful; it never applies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub event: ClosureEvent,
    pub actor: u64,
    pub stamp: u64,
}

/// A refused transition — typed, carrying its measurement whole (the
/// house law: a refusal names its cure; a break shows its residual,
/// never a scalar). An `Ok(())` from [`lawful`] is the transition
/// admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// A value-conserving step changed the total of a resource across
    /// the world's banks — value was minted or burned where the deed
    /// promised only to move it. Both totals carried (boxed — the
    /// carrier is a big-int rational); the residual is theirs to read
    /// (shown, never summarized away).
    ConservationBroken { resource: Resource, before: Box<Ratio>, after: Box<Ratio> },
    /// **D4** — a mint whose capacity increment is zero or negative:
    /// the claim on capacity never entangled with potential, so it is
    /// vacated (Equations D4). Both readings carried whole — the
    /// circulation standing above the ground before and after.
    ///
    /// `D` is not lowered by this refusal. The claim vacates; the
    /// degree does not (Article IV, and this row's `H¹` floor).
    MintWithoutCapacityIncrement { was: usize, now: usize },
    /// **D4** — the rank reading is not resolved, so no increment over
    /// it can be trusted. An unresolved absence is not a zero: without
    /// this, phantom topology mints phantom rank
    /// (`EntangledRead::rank_is_defined`, `25_STANDING` §6's debt, now
    /// due). Both unknown counts carried.
    RankUndefined { unknown_before: usize, unknown_after: usize },
    /// I2 — the actor is not an entity in `prior`. Identity precedes the
    /// world (NS-6 §1.2): a deed from no one refuses before any state read.
    NoSuchEntity { actor: u64 },
    /// I2 — the actor is an entity but lacks the scope the deed requires.
    /// Containment: authority is granted, never assumed.
    NotPermitted { actor: u64, scope: Scope },
    /// I4 — a mint carried a negative amount: a deed labelled a creation
    /// that actually destroys value (a burn wearing a mint's name). Value
    /// enters only by a lawful, non-negative mint.
    MintNotPositive { resource: Resource, amount: Box<Ratio> },
    /// I5 — a decay carried a negative amount: a deed labelled a decay
    /// that actually creates value. Value leaves only by a lawful,
    /// non-negative decay.
    DecayNotPositive { resource: Resource, amount: Box<Ratio> },
    /// I5 — an atomic touch (tag 102) carried the same resource twice:
    /// one window settles one row per resource, and a repeat would
    /// draw the same window's decay twice.
    DecayRowDuplicated { resource: Resource },
    /// The element law (operator ruling 2026-08-01): the element
    /// ledger is gated with strict symmetry to the banked ledger. An
    /// element mint carried a negative amount — value enters the
    /// element ledger only by a lawful, non-negative assay.
    ElementNotPositive { element: [u8; 32], amount: Box<Ratio> },
    /// The element law: an assay drew more than the vein's declared
    /// yield — `amount ≤ richness · elapsed`, both read from the prior
    /// vein (`Prospected.richness`, `last_assay`); an assay with no
    /// prior vein yields nothing. Both sides carried whole.
    AssayExceedsVein { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// ECO-1 (#40): the assay declared an after-richness off the
    /// sealed wear law. The gate re-derives it by calling the
    /// emitter's one home (`cfe::dop::vein_wear` — wear from the
    /// draw, recovery from the orb's standing regenerators, capped at
    /// the spring's hash-derived natural flow) and refuses any other
    /// number. A declaration with no prior vein, or on a starless
    /// state, has no lawful derivation and refuses with expected 0.
    /// The sealed 3-field form declares nothing and judges only the
    /// bound. Both sides carried whole.
    VeinWearMisdeclared { expected: Box<Ratio>, declared: Box<Ratio> },
    /// ELEMENT-CHEMISTRY (#37): the receipt's members do not bond —
    /// the shells disagree, the phases cannot coexist, the count is not
    /// the geometry's arity, or the binds do not saturate. Matter that
    /// does not compound declares no move and may make none.
    MatterDoesNotBond,
    /// ELEMENT-CHEMISTRY (#37): the receipt named a gem that is not the
    /// one its members make. The gem is content-addressed and
    /// order-free, so the gate re-derives it and refuses any other
    /// identity — a compounding cannot mint a name it did not earn.
    GemMisidentified { expected: Box<[u8; 32]>, declared: Box<[u8; 32]> },
    /// ECO-1 (#40): the vein book moved off its two deeds. Richness
    /// is judged state now, so the book's motion is the law's:
    /// a prospect pushes exactly its receipted vein; an assay moves
    /// that vein's span anchor and lands its declared richness;
    /// every other deed holds the book still. `at` names the first
    /// diverging index (the shorter book's length when one is short).
    VeinBookBroken { at: usize },
    /// The element law: an element total changed where the deed
    /// declared no change — the ledger's total for one element must
    /// equal the prior total plus exactly what the deed declares
    /// (only an assay declares a mint; every other deed holds every
    /// element key still). The element named, both totals whole.
    ElementBroken { element: [u8; 32], expected: Box<Ratio>, actual: Box<Ratio> },
    /// The GAUGE law (U2d, I6's kernel half): the state's key moved
    /// where the deed declared no move, or landed off its declared
    /// key, or a plane opened or closed undeclared. The first
    /// differing grade named, both sides carried whole (an absent
    /// grade reads 0 — ground rest). Every fold transition emits
    /// exactly its declared structural delta; no state jump is
    /// special-cased (operator ruling).
    GaugeBroken { grade: usize, expected: Box<Ratio>, actual: Box<Ratio> },
    /// The sign law (the value-law wave): a value-carrying deed
    /// declared a negative amount — a credit wearing a debit's name,
    /// or the reverse. Every banked movement is non-negative by
    /// declaration; direction comes from the deed, never the sign
    /// (a negative transfer would invert the flow and authorize the
    /// wrong side; a negative settle would credit through a debit).
    /// No un-guarded debit/credit path exists.
    AmountNotPositive { resource: Resource, amount: Box<Ratio> },
    /// I5 — the decay exceeds the sealed demurrage rule's due:
    /// `min(rate · ease · elapsed, banked)`, flat, thermally gated.
    /// Both sides carried whole (what the rule allows, what the deed
    /// claimed). Value leaves only by the flat law — never faster.
    DecayExceedsBound { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// I4 — the discovery mint's proof failed the replay (form,
    /// training, output, or chain — the sealed verifier names the
    /// break; the kernel carries it whole, never summarizes).
    WorkRefused { broken: WorkBroken },
    /// I4 — the mint exceeds what verified work earns. Both sides
    /// carried whole (the bound the law allows, the amount claimed).
    /// The bound is the ruled emission rate `f` (Ruling Box 1,
    /// 2026-08-01 — see [`mint_bound`]): ladder-pinned, zero on the
    /// saturated rungs and on every off-ladder shape.
    MintExceedsWork { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// I3 — a value in `next` is not a canonical exact rational (coprime,
    /// positive denominator). Exactness is a type-level guarantee
    /// (`BigRational` is canonical by construction, no float ever entered);
    /// I3 is its sealed declaration and defense-in-depth at the byte
    /// boundary. The kernel calls the sealed `ratio::is_canonical`, never
    /// defining canonicity itself (the equations are shared, never copied
    /// — Kernel Charter §0b).
    NotCanonical,
    /// The sealed DEFAULT — the gate refuses what no invariant covers.
    /// This event has no landed rule; a deed the kernel cannot gate does
    /// not enter. Coverage grows by amendment: each landing moves an event
    /// from this default-refuse arm to an explicit gated arm, and the
    /// kernel wires as the live gate (Phase 3) only when `Unjudged` is
    /// unreachable — every event covered. **SEAL A is a reader's law,
    /// never the gate's:** readers skip unknown tags so old software
    /// survives new content; a gate that skips unknown deeds is a hole
    /// exactly the size of every future tag. Two laws, one word "unknown"
    /// — conflating them silently rebuilds default-admit.
    Unjudged,

    /// D5 — the SOVEREIGN union was struck by someone other than the
    /// root entity. The union is the standing the whole world is
    /// raised on, and no delegate may enter the offices into it: a
    /// granted scope is authority to act within the world, never
    /// authority to constitute it.
    ///
    /// **Approval is who acts, never a field.** An earlier design
    /// carried `approved_by_operator: bool` on the deed. A deed is a
    /// claim, and a claim asserting its own approval is forgeable by
    /// whoever builds it — the kernel would have had nothing to check
    /// the bool against. I2 containment already binds the actor
    /// unforgeably, so identity is the signature.
    UnionNotStruckByRoot { actor: u64 },
    /// D5 — the union named the wrong offices. It is `Progenitor` with
    /// `Sovereign`, exactly once each: two progenitors or two
    /// sovereigns is not a DYAD, and a dyad is the whole form (`17`
    /// §3 — two that stay two, neither deriving legitimacy from the
    /// other).
    UnionMalformed { progenitor: Office, sovereign: Office },
    /// D5 — the union declared a review horizon other than the ruled
    /// one. The term is a review, not an expiry, and it is fixed at
    /// [`cfe::dop::UNION_REVIEW_HORIZON`]: a deed free to name its own
    /// horizon could set it past any reachable stamp and never come up
    /// for approval at all.
    UnionTermNotRuled { declared: u64, ruled: u64 },
    /// The ring REACH (2026-08-06) — a siting threads a new ring beyond
    /// what the star's magnetism anchors. The reach is the field grade
    /// (`StarClass::ring_reach`, the dynamo derivation), computed by
    /// calling the same rule the emitter refused on (call-not-copy).
    /// Filling an anchored ring never refuses here — slots within a
    /// ring are the turn rule's business, not the field's.
    RingBeyondReach { held: usize, reach: u8 },
    /// The appraisal (2026-08-06) — the subject's orb lies outside the
    /// survey's covering (orb 0 names the estate itself where the
    /// covering cannot value it whole). Nothing appraises unsurveyed.
    AppraisalOutsideSurvey { orb: u64 },
    /// The appraisal — the declared value is not what the sealed rule
    /// computes over the prior state and the receipted covering. A
    /// value cannot be quoted one way and receipted another.
    AppraisalMisvalued { declared: Box<Ratio>, computed: Box<Ratio> },
    /// Convey — an endpoint outside the survey's covering: a postal
    /// address is a surveyed parcel, at the gate as at the emitter.
    ConveyOutsideSurvey { orb: u64 },
    /// Convey — the declared postage is not the postal meter's figure
    /// over the amount. Stamps cannot be quoted one way and burned
    /// another.
    PostageMismatch { declared: Box<Ratio>, metered: Box<Ratio> },
    /// Convey — the ask outruns the sender parcel's monetary portion of
    /// its prior bank. The free portion stays home, held at the gate
    /// from the receipt's own covering.
    ConveyExceedsMonetary { asked: Box<Ratio>, cap: Box<Ratio> },
    /// E5b — an estate survey whose covering is not the whole estate:
    /// empty (an unwitnessed observation cannot enter the record) or
    /// short of the orbs that stand (a partial covering underpays the
    /// meter). One theorem, both halves: the observation is read whole,
    /// witnessed whole, and charged whole.
    SurveyNotWhole { covered: usize, estate: usize },
    /// E5b — the declared cost does not equal the sealed meter over the
    /// volume. The kernel computes the meter by calling the same rule
    /// the emitter drew from (call-not-copy), so a survey cannot be
    /// quoted one way and charged another.
    SurveyMeterMismatch { declared: Box<Ratio>, metered: Box<Ratio> },
}

/// The judge. Does `step` lawfully carry `prior` to `next`? Pure,
/// total, encoding-blind, and network-blind (it reads no mesh, no peer,
/// no uplink — `engine/kernel` imports `cfe` only; all arrows point one
/// way). `Ok(())` admits; `Err(Refused)` refuses with the break whole.
pub fn admits(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    // The sealed default arm (mesh-operator ruling, airtight-critical):
    // refuse-unknown, not admit-unknown. What no invariant covers does
    // not enter.
    if !covered(&step.event) {
        return Err(Refused::Unjudged);
    }
    conservation(prior, next, step)?;
    containment(prior, step)?;
    exactness(next)?;
    union(step)?;

    creation(step)?;
    // D4 after `creation`, deliberately. `creation` gates the deed —
    // its proof and its bound; D4 gates the deed's effect on the
    // world. Ordering effect after validity means a malformed payload
    // is named `WorkRefused`, not misreported as a capacity failure,
    // and the work law stays reachable by every fixture that tests it.
    // The two are entangled (`Δ(D) ⋈ ∂Δ`), not ranked — the law fixes
    // that both must hold, never which speaks first, so this is a
    // diagnostics choice and is recorded as one.
    revocation_law(prior, next, step)?;
    magnetic_reach(prior, step)?;
    appraisal_rule(prior, step)?;
    decay(prior, step)?;
    gauge_move(prior, next, step)?;
    element_move(prior, next, step)?;
    stamp_move(prior, next, step)
}

/// Does a Vault custody station sit in this state? Mirrors
/// `World::vault_seated` on the `Committed` snapshot the kernel judges
/// — the STAMP a Vault holds lives in `vault_stamp`, off the free-bank
/// sum, so instrument collateral is only earmarked, never peeled.
fn vault_seated(state: &Committed) -> bool {
    state
        .stations
        .iter()
        .any(|(_, s)| *s == cfe::dop::Station::Vault as u8)
}

/// STAMP-1 conservation: total orb.stamp moves by exactly the deed's
/// declared delta (mint positive, aperture send negative, receive
/// positive). Neutral deeds hold the total still.
fn stamp_move(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    let declared = match &step.event {
        ClosureEvent::StampMinted { amount, crystal_spent, .. } => {
            if *amount < ratio(0, 1) || *crystal_spent < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            if *amount != *crystal_spent {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(crystal_spent.clone()),
                    claimed: Box::new(amount.clone()),
                });
            }
            amount.clone()
        }
        ClosureEvent::StampApertureSent { amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            -amount.clone()
        }
        ClosureEvent::StampApertureReceived { amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            amount.clone()
        }
        // The bounce (tag 195): a core's STAMP pulse arrives — a declared
        // credit, sign-guarded like the aperture's receive leg.
        ClosureEvent::CoreReturned { resource: None, amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            amount.clone()
        }
        // P5-M / P5-M2: raids and cascades burn fee STAMP (typed residual).
        ClosureEvent::LiquidityRaidStruck { fee, .. }
        | ClosureEvent::NoteCascadeStruck { fee, .. } => {
            if *fee < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(fee.clone()),
                });
            }
            -fee.clone()
        }
        // P5-B: doctrine reorientation burns 1 STAMP (identity fee).
        ClosureEvent::PostureReoriented { .. } => -ratio(1, 1),
        // X7: strand export burns STAMP; Anchor import credits STAMP.
        ClosureEvent::StrandExported { amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            -amount.clone()
        }
        ClosureEvent::AnchorImported { amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            amount.clone()
        }
        ClosureEvent::AnchorImportRefused { .. } => ratio(0, 1),
        // O6.5: vault custody moves free STAMP off/on the free bank.
        ClosureEvent::VaultDeposited { amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            -amount.clone()
        }
        ClosureEvent::VaultWithdrawn { amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: [0u8; 32],
                    amount: Box::new(amount.clone()),
                });
            }
            amount.clone()
        }
        ClosureEvent::AssayConsilience { .. } | ClosureEvent::AssaySlashed { .. } => {
            ratio(0, 1)
        }
        // PHASE-4 / I5: issuing a Note locks its collateral out of the
        // free STAMP banks — the same off-bank move a VaultDeposited
        // declares. With Vault seated the collateral is only earmarked
        // (it stays in `vault_stamp`, already off the summed free bank),
        // so the free-bank total holds still; without a Vault the
        // collateral is peeled from the orb's free STAMP (`bank_stamp`
        // in `lock_instrument_stamp`), a genuine debit of the sum.
        ClosureEvent::NoteIssued { collateral, .. } => {
            if vault_seated(prior) {
                ratio(0, 1)
            } else {
                -collateral.clone()
            }
        }
        // #138 the resource SMELT-SWEEP (tag 186): the migration floors every
        // orb's STAMP to the 1/64 grain. RE-derive the total floor delta from
        // prior (call-not-copy): `Σ (smelt(stamp) − stamp)`, a non-positive shave
        // to the void, exactly what `sweep_resource_grain` applies to `orb.stamp`,
        // so the total-check `after == before + declared` holds. (The banked
        // resource floors are judged in `admitted_deltas`.)
        ClosureEvent::ResourceStocksSwept { .. } => prior
            .orbs
            .iter()
            .fold(ratio(0, 1), |s, o| {
                s + (cfe::world::smelt_resource(&o.stamp) - o.stamp.clone())
            }),
        _ => ratio(0, 1),
    };
    let before = prior
        .orbs
        .iter()
        .fold(ratio(0, 1), |s, o| s + o.stamp.clone());
    let after = next
        .orbs
        .iter()
        .fold(ratio(0, 1), |s, o| s + o.stamp.clone());
    let expected = before.clone() + declared;
    if after != expected {
        return Err(Refused::ConservationBroken {
            resource: Resource::Crystal, // stamp is not Resource; crystal is the nearest law carrier
            before: Box::new(before),
            after: Box::new(after),
        });
    }
    Ok(())
}

/// **Invariant — D4, the revocation law. The eclipse anchor, floored
/// at `H¹`.** (Equations D4, inscribed; ruled 2026-08-01; landed
/// 2026-08-04.)
///
/// D4 is **not** a scalar decay and **not** a contraction of `D`. It is
/// the formal entanglement collapse operator: the site where a claim on
/// capacity that never entangled with potential is vacated from the
/// frame. It executes `18` §3 Article III; it is not a second penalty
/// beside it.
///
/// ```text
/// D itself NEVER falls by this rule   Article IV, and the algebra:
///                                     Π_G is idempotent, so what was
///                                     frame was never in D
/// FORBIDDEN where orbit_lock ≠ ∅      genuine difference keeps its
///                                     degree; the H¹ floor stands
/// VACATED: the un-entangled CLAIM     a claim that did not survive
///                                     Π_G was never capacity (D2)
/// ```
///
/// **The claim vacates; the degree does not.** That is why D4 is
/// expressible as a kernel invariant at all: `lawful` refuses, it never
/// mutates. Refusing the deed is the vacating of its claim, and no code
/// path here can lower a `Degree` — which would violate Article IV and
/// this row's own floor.
///
/// **The layer split is load-bearing — a WINDOW is not a step.** D4
/// fires in two places and they must never merge. This is the per-step
/// half: *a deed that mints carries a non-zero capacity increment, or
/// refuses* (`18` §2f consequence 2 — no hidden work). The OVER-A-
/// WINDOW half (`deeds(w) > 0 ∧ ΔD(w) = 0`) is a lens reading and
/// gates nothing; it lives in `lens::revocation` and must never be
/// hoisted into the gate. A rate never attains its bound at any single
/// step, so a window can never be an invariant.
///
/// **The PHANTOM-RANK gate, now due.** `EntangledRead::rank_is_defined`
/// has existed unused, its own header warning that an unresolved
/// absence *"would let phantom topology mint phantom rank"*, and
/// `25_STANDING` §6 recorded it as debt rather than enforcement. D4
/// gates a mint on a rank-derived increment, which makes that debt due:
/// without this check the increment is computed over an incomplete
/// reading and phantom topology satisfies the test. It is asked first,
/// before any increment is compared.
///
/// The increment is read on the circulation — `Degree::above`, the
/// gauge-invariant part that grows — never on `Degree::get`, which
/// includes the ground and would make the comparison vacuous exactly
/// the way the struck `D > 0` gate was (a floor cannot decide).
/// **The magnetic REACH gate** (operator ruling 2026-08-06: planet
/// capacity is based on the star's magnetism). A `PlanetSited` deed
/// that threads a ring the prior state does not hold, when the prior
/// state already holds as many rings as the star's field anchors,
/// refuses. A ring is a distinct `orbit_r`; the reach is
/// `StarClass::ring_reach` — the same sealed rule the emitter draws,
/// so the field cannot be quoted one way at the door and another at
/// the judge. No star in the prior state is the emitter's refusal
/// (`NoStarCommissioned`) and is not re-judged here.
/// **The appraisal RULE** (2026-08-06): an `AppraisalStruck` declares a
/// value; the gate RE-derives it from the prior state and the receipt's
/// own seed and covering, calling the same sealed rule the emitter used
/// (call-not-copy). A subject outside the covering refuses; a declared
/// value the rule does not compute refuses. Staleness is deliberately
/// not here — it triggers renegotiation and never gates a deed.
fn appraisal_rule(prior: &Committed, step: &Step) -> Result<(), Refused> {
    let ClosureEvent::AppraisalStruck { subject, seed, covered, value, .. } = &step.event
    else {
        return Ok(());
    };
    use cfe::dop::Holding;
    if let Holding::Planetary { orb }
    | Holding::Building { orb, .. }
    | Holding::Orbital { orb }
    | Holding::Stratum { orb, .. } = subject
    {
        if !covered.contains(orb) {
            return Err(Refused::AppraisalOutsideSurvey { orb: *orb });
        }
    }
    let parcels = cfe::dop::parcels_of(seed, covered);
    let star_m = prior.star.map(|c| c.magnetism()).unwrap_or(1);
    let prospects: Vec<cfe::dop::ProspectRow> = prior
        .prospects
        .iter()
        // ONT-2: richness derives — natural endowment minus extraction.
        .map(|p| {
            let richness = prior
                .star
                .map(|c| {
                    cfe::dop::vein_richness(
                        &cfe::dop::element_properties(&p.element, c).2,
                        &p.extracted,
                        &p.rejuvenated,
                    )
                })
                .unwrap_or_else(|| ratio(0, 1));
            (p.orb, p.stratum, richness)
        })
        .collect();
    match cfe::dop::appraised_value(subject, &prior.orbs, &parcels, star_m, &prospects) {
        Some(computed) if computed == *value => Ok(()),
        Some(computed) => Err(Refused::AppraisalMisvalued {
            declared: Box::new(value.clone()),
            computed: Box::new(computed),
        }),
        None => Err(Refused::AppraisalOutsideSurvey {
            orb: match subject {
                Holding::Planetary { orb }
                | Holding::Building { orb, .. }
                | Holding::Orbital { orb }
                | Holding::Stratum { orb, .. } => *orb,
                Holding::Estate => 0,
            },
        }),
    }
}

fn magnetic_reach(prior: &Committed, step: &Step) -> Result<(), Refused> {
    let ClosureEvent::PlanetSited { orbit_r, .. } = &step.event else {
        return Ok(());
    };
    let Some(star) = prior.star else {
        return Ok(());
    };
    // `orbit_r` is a `Positive` end to end (29_CONSTRUCTION C1), so the
    // ring census compares proven radii — no bare Ratio ever enters.
    let mut rings: Vec<&cfe::dop::Positive> = Vec::new();
    for held in &prior.orbs {
        if let Some(site) = &held.thermal {
            if !rings.contains(&&site.orbit_r) {
                rings.push(&site.orbit_r);
            }
        }
    }
    if rings.contains(&orbit_r) {
        return Ok(());
    }
    let reach = star.ring_reach();
    match rings.len() < usize::from(reach) {
        true => Ok(()),
        false => Err(Refused::RingBeyondReach { held: rings.len(), reach }),
    }
}

fn revocation_law(
    prior: &Committed,
    next: &Committed,
    step: &Step,
) -> Result<(), Refused> {
    let ClosureEvent::DiscoveryMinted { .. } = &step.event else {
        // D4 binds the mint. Motion of value that already exists makes
        // no capacity claim, so there is no claim to vacate.
        return Ok(());
    };
    let before = bridge::read_entangled(prior);
    let after = bridge::read_entangled(next);

    d4a_resolution(&before, &after)?;
    match d4b_standing_claim(&before) {
        Claim::None => Ok(()),
        Claim::Standing(was) => d4c_increment(was, after.degree().above()),
    }
}

/// **D4a — the resolution gate.** An unresolved absence is not a zero.
///
/// `rank_is_defined` stood unused since it was written, its own header
/// warning that an unresolved absence *"would let phantom topology mint
/// phantom rank"*, with `25_STANDING` §6 recording it as debt rather
/// than enforcement. D4 computes an increment over that rank, so the
/// debt came due: asked first, before anything is compared.
fn d4a_resolution(before: &bridge::EntangledRead, after: &bridge::EntangledRead) -> Result<(), Refused> {
    match before.rank_is_defined() && after.rank_is_defined() {
        true => Ok(()),
        false => Err(Refused::RankUndefined {
            unknown_before: before.unknown,
            unknown_after: after.unknown,
        }),
    }
}

/// What D4b found: whether there is a capacity claim here to gate.
enum Claim {
    /// Nothing circulates. No claim was made, so none can be vacated.
    None,
    /// A claim stands, carrying the circulation it stands on.
    Standing(usize),
}

/// **D4b — is there A claim to vacate AT ALL?**
///
/// D4 vacates *a claim that did not survive `Π_G`*, and where nothing
/// circulates no capacity claim was ever made. A line holding no
/// relations is not claiming capacity and failing; it is claiming none.
///
/// **This gate was forced by measurement.** D4 first landed binding
/// every `DiscoveryMinted`, and a 60-beat era refused 48 of 55 mints,
/// each reading `was: 0, now: 0`. That is the same defect the capacity
/// law was struck for the same day — *identity is pegged to entity, and
/// identity gets recycled without bonds* — reached by another route: a
/// mint banks value and adds no relation, so `ΔD` is zero by
/// construction on any world holding nothing.
///
/// **The gap this exposes, recorded rather than papered over.**
/// Equations D3 asserts *capacity growth and the discovery mint are one
/// event*. In this engine they are two — the mint banks value, a plane
/// opens separately by `GaugeShiftedAt`. Until a deed claims capacity
/// as it mints, D4 can only bind where a claim already stands. Owed to
/// the operator.
fn d4b_standing_claim(before: &bridge::EntangledRead) -> Claim {
    match before.degree().above() {
        0 => Claim::None,
        was => Claim::Standing(was),
    }
}

/// **D4c — the increment.** A standing claim grows, or it vacates.
///
/// Read on the circulation — what stands above the floor — never on the
/// full `Degree`, which includes the ground and would make the
/// comparison vacuous exactly the way the struck `D > 0` gate was: a
/// floor decides nothing.
///
/// The refusal is the vacating. Nothing here writes, and `Degree` has
/// no below-ground representation to write into even if it did —
/// Article IV, the `H¹` floor. The claim vacates; the degree does not.
fn d4c_increment(was: usize, now: usize) -> Result<(), Refused> {
    match now > was {
        true => Ok(()),
        false => Err(Refused::MintWithoutCapacityIncrement { was, now }),
    }
}
/// Invariant — D5, the SOVEREIGN union (operator ruling 2026-08-02).
///
/// The union of the offices `D(Ω)` progenitor and `V(Ω)` SOVEREIGN is
/// a constitutional deed: it moves no value, opens no plane and shifts
/// no key. What it must do is be impossible to strike falsely, and
/// three things make it so.
///
/// **The root entity, and only it.** A granted scope is authority to
/// act within the world; it is never authority to constitute the world.
/// `scope_of` puts this deed in Build so the ordinary containment
/// check applies, and that is necessary but not sufficient — this arm
/// additionally requires `actor == SOVEREIGN`, the root entity id.
/// Approval is who acts: a bool on the deed would be set by whoever
/// built it.
///
/// **A DYAD, exactly.** One `Progenitor` and one `Sovereign`. Two of
/// either is not a union of two offices, and `17` §3 makes the pair
/// the whole form — two that stay two, neither deriving its legitimacy
/// from the other.
///
/// **The ruled horizon.** A deed free to name its own term could set
/// one past any reachable stamp and never come up for approval, which
/// would make the review a formality. The term is fixed.
///
/// Pure and total like the rest: reads the step only, no clock, no
/// history — the review falls due by arithmetic on the receipt's own
/// stamp, and nothing about it is decided here.
fn union(step: &Step) -> Result<(), Refused> {
    let ClosureEvent::SovereignUnionSealed { progenitor, sovereign, term_stamps } = &step.event
    else {
        return Ok(());
    };
    if step.actor != SOVEREIGN {
        return Err(Refused::UnionNotStruckByRoot { actor: step.actor });
    }
    if *progenitor != Office::Progenitor || *sovereign != Office::Sovereign {
        return Err(Refused::UnionMalformed {
            progenitor: *progenitor,
            sovereign: *sovereign,
        });
    }
    if *term_stamps != UNION_REVIEW_HORIZON {
        return Err(Refused::UnionTermNotRuled {
            declared: *term_stamps,
            ruled: UNION_REVIEW_HORIZON,
        });
    }
    Ok(())
}

/// Invariant — element conservation (the element law, operator ruling
/// 2026-08-01). The element ledger is gated with strict symmetry to
/// the banked ledger: an assay mints `richness · elapsed` into a
/// vein's element — non-negative by declaration, and bounded by the
/// prior vein's `richness` and `last_assay` (both already committed
/// state, so the bound is deterministic from `(prior, step)`). Every
/// other deed declares no element move, so every element total holds
/// still (a smuggled element mint under any deed refuses).
fn element_move(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    // The declared element moves for this deed (empty = every key
    // holds still). Signed: a mint is positive, a spend negative.
    let declared: Vec<([u8; 32], Ratio)> = match &step.event {
        ClosureEvent::AssayStruck { orb_id, element, amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(amount.clone()),
                });
            }
            // ONT-2: the vein's yield, RE-derived from the immutable geometry
            // and the prior extraction integral (call-not-copy, the one home
            // `vein_flow`) — the vein carries no stored richness scalar. Richness
            // is `natural − extracted` (the hash-derived endowment under the
            // prior star, minus what the ledger already took); the flow is that
            // richness's geometric decay over the settled span. A claim above
            // this refuses; no vein yields more than its geometry allows.
            let bound = prior
                .prospects
                .iter()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
                .and_then(|vein| {
                    let class = prior.star?;
                    let (_, _, natural) = cfe::dop::element_properties(element, class);
                    let elapsed = step.stamp.saturating_sub(vein.last_assay);
                    let richness = cfe::dop::vein_richness(&natural, &vein.extracted, &vein.rejuvenated);
                    Some(cfe::dop::vein_flow(&richness, &natural, elapsed))
                })
                .unwrap_or_else(|| ratio(0, 1));
            if *amount > bound {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(bound),
                    claimed: Box::new(amount.clone()),
                });
            }
            vec![(*element, amount.clone())]
        }
        // ELEMENT-CHEMISTRY (#37): a bond moves many keys at once —
        // each member spends its lane's valence and the gem carries
        // away the sum. The gate re-derives every one of those numbers
        // by calling the emitter's one home over the members the
        // receipt names (call-not-copy), so a compounding cannot be
        // quoted one way and settled another. A receipt whose members
        // do not bond declares nothing and therefore may move nothing.
        ClosureEvent::GemCompounded { members, gem, .. } => {
            let Some(set) = cfe::almanac::Compoundable::new(members) else {
                return Err(Refused::MatterDoesNotBond);
            };
            if set.gem() != *gem {
                return Err(Refused::GemMisidentified {
                    expected: Box::new(set.gem()),
                    declared: Box::new(*gem),
                });
            }
            let (spends, yielded) = cfe::almanac::recipe_of(&set);
            let mut declared: Vec<([u8; 32], Ratio)> = spends
                .into_iter()
                .map(|(element, spend)| (element, -spend))
                .collect();
            declared.push((*gem, yielded));
            declared
        }
        // Chem science (tag 156): isomorphic gem union — re-derive product.
        ClosureEvent::IsoUnionStruck {
            a,
            b,
            product,
            ..
        } => {
            let Some((spend, yielded, expected)) = cfe::almanac::iso_union_recipe(a, b) else {
                return Err(Refused::MatterDoesNotBond);
            };
            if expected != *product {
                return Err(Refused::GemMisidentified {
                    expected: Box::new(expected),
                    declared: Box::new(*product),
                });
            }
            vec![
                (*a, -spend.clone()),
                (*b, -spend),
                (*product, yielded),
            ]
        }
        ClosureEvent::ChemCataloged { .. }
        | ClosureEvent::DarkPullStruck { .. }
        | ClosureEvent::ScienceSeated { .. }
        | ClosureEvent::LabCreated { .. }
        | ClosureEvent::ScienceExploded { .. }
        | ClosureEvent::RogueFactionStruck { .. } => Vec::new(),
        // REFINE-1 (tag 107): spend raw, bank refined. Re-derive the
        // recipe from the raw identity + star (call-not-copy). A
        // receipt whose numbers disagree with the law moves nothing.
        ClosureEvent::ElementRefined {
            raw,
            refined,
            spent,
            yielded,
            ..
        } => {
            let Some(class) = prior.star else {
                // No star → no lawful recipe; any declared spend is
                // above the zero bound.
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(ratio(0, 1)),
                    claimed: Box::new(spent.clone()),
                });
            };
            let (law_spend, law_refined, law_yield) = cfe::dop::refine_recipe(raw, class);
            if law_refined != *refined {
                return Err(Refused::GemMisidentified {
                    expected: Box::new(law_refined),
                    declared: Box::new(*refined),
                });
            }
            if *spent != law_spend || *yielded != law_yield {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(law_spend),
                    claimed: Box::new(spent.clone()),
                });
            }
            if *spent < ratio(0, 1) || *yielded < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *raw,
                    amount: Box::new(spent.clone()),
                });
            }
            vec![(*raw, -spent.clone()), (*refined, yielded.clone())]
        }
        // E-APT-1: element leave / arrive. Sign-guarded; amount on the
        // receipt is the whole truth of that leg (melt applied before
        // the receive is sealed).
        ClosureEvent::ElementApertureSent { element, amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(amount.clone()),
                });
            }
            vec![(*element, -amount.clone())]
        }
        ClosureEvent::ElementApertureReceived { element, amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(amount.clone()),
                });
            }
            vec![(*element, amount.clone())]
        }
        // ING-2b (tag 182): the staple food settle on the element ledger.
        // The net `delta = regrown − eaten` is declared whole — the element
        // mirror of the Resource food fold (`YieldCollected`, the #28
        // admitted free mint the star drives). Signed: net regrowth mints
        // (positive), a lean span where the band ate more than the planet
        // regrew debits (negative). Not re-derived here — symmetric with the
        // Resource food economy, whose regrowth `YieldCollected` the kernel
        // likewise admits free; the emitter clamps the debit to the standing
        // larder so the total never crosses zero. Every other deed still
        // declares no element move, so the ledger holds still under them.
        ClosureEvent::RationElementSettled { element, delta, .. } => {
            vec![(*element, delta.clone())]
        }
        // ONT-2b (tag 183): eating mines the staple vein. The bound is the
        // vein's remaining richness (`natural − extracted`), re-derived from
        // the immutable geometry and the prior extraction integral — a demand
        // draw may take up to the whole larder in one pull (no per-span
        // geometric throttle; that is the geometric ground-assay's law, not
        // this one). No element-ledger move is declared: the mined food went
        // vein → mouths, never the bank, so the ledger holds still (delta 0)
        // and only the vein BOOK advances (below). A claim above the remaining
        // richness refuses — no vein feeds more than its geometry holds.
        ClosureEvent::StapleMined { orb_id, element, eaten } => {
            if *eaten < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(eaten.clone()),
                });
            }
            let bound = prior
                .prospects
                .iter()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
                .and_then(|vein| {
                    let class = prior.star?;
                    let (_, _, natural) = cfe::dop::element_properties(element, class);
                    Some(cfe::dop::vein_richness(&natural, &vein.extracted, &vein.rejuvenated))
                })
                .unwrap_or_else(|| ratio(0, 1));
            if *eaten > bound {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(bound),
                    claimed: Box::new(eaten.clone()),
                });
            }
            Vec::new()
        }
        // ONT-3 (tag 184): a worked vein recovers. The bound is the recovery
        // headroom — the net drawn so far, `extracted − rejuvenated`, re-derived
        // from the prior integrals — so an injection can only undo real
        // extraction and never lifts richness above the endowment (no matter is
        // minted from geometry). No element-ledger move: a delta-0 event, the
        // vein book alone advancing (below). A claim above the headroom refuses.
        ClosureEvent::VeinRejuvenated { orb_id, element, injected } => {
            if *injected < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(injected.clone()),
                });
            }
            let bound = prior
                .prospects
                .iter()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
                .map(|vein| (vein.extracted.clone() - vein.rejuvenated.clone()).max(ratio(0, 1)))
                .unwrap_or_else(|| ratio(0, 1));
            if *injected > bound {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(bound),
                    claimed: Box::new(injected.clone()),
                });
            }
            Vec::new()
        }
        // G2 (#29): an element-eater's food cycle — it synthesizes its
        // element from energy (a conserved transmutation; the joule spend
        // is checked in conservation(), here the element mint is bounded)
        // and eats some of the same element running its zones. The net
        // element change is `minted − eaten`. The mint bound is the
        // element's own natural flow (richness): minted ≤ joule × richness,
        // so a rarer element yields less per joule. No star → zero bound.
        ClosureEvent::ElementFarmed { element, joule, minted, eaten, .. } => {
            if *minted < ratio(0, 1) || *joule < ratio(0, 1) || *eaten < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(minted.clone()),
                });
            }
            let bound = match prior.star {
                Some(class) => {
                    let (_, _, richness) = cfe::dop::element_properties(element, class);
                    joule.clone() * richness
                }
                None => ratio(0, 1),
            };
            if *minted > bound {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(bound),
                    claimed: Box::new(minted.clone()),
                });
            }
            vec![(*element, minted.clone() - eaten.clone())]
        }
        // 34: element → scrap; element mass leaves the open-matter ledger.
        ClosureEvent::ElementSalvaged { element, amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(amount.clone()),
                });
            }
            vec![(*element, -amount.clone())]
        }
        // G2 (#trace): a population burns trace elements maintaining its
        // substrate — the element leaves the ledger (a conserved spend),
        // sign-guarded exactly as salvage is.
        ClosureEvent::TraceConsumed { element, amount, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(amount.clone()),
                });
            }
            vec![(*element, -amount.clone())]
        }
        // #136 the SMELT-SWEEP (tag 185): the one-time migration floors every element
        // stock to the 1/1024 grain. RE-derive the per-element floor deltas from the
        // prior state (call-not-copy — the deed carries only a scalar dust, never
        // per-element amounts a receipt could quote one way and settle another): each
        // stock's declared delta is `smelt_element(prior) − prior`, a non-positive
        // shave to the void, summed per element by the total-check below. This mirrors
        // `sweep_element_grain` exactly, so `after == before + declared` holds and the
        // migration is conservation-exact (the dust is the recorded loss).
        ClosureEvent::ElementStocksSwept { .. } => {
            let mut declared: Vec<([u8; 32], Ratio)> = Vec::new();
            for orb in &prior.orbs {
                for (element, amount) in &orb.elements {
                    let floored = cfe::world::smelt_element(amount);
                    if floored != *amount {
                        declared.push((*element, floored - amount.clone()));
                    }
                }
            }
            declared
        }
        _ => Vec::new(),
    };
    // Total-check every element key present in either state or in the
    // declaration: the total moves by exactly the declared amount.
    // A key may be declared more than once (a set holding two of one
    // lane spends twice), so the deltas sum rather than overwrite —
    // taking only the first match would let the second spend vanish.
    let mut keys: Vec<[u8; 32]> = Vec::new();
    for (element, _) in &declared {
        if !keys.contains(element) {
            keys.push(*element);
        }
    }
    for orb in prior.orbs.iter().chain(next.orbs.iter()) {
        for (element, _) in &orb.elements {
            if !keys.contains(element) {
                keys.push(*element);
            }
        }
    }
    for element in keys {
        let delta = declared
            .iter()
            .filter(|(held, _)| *held == element)
            .fold(ratio(0, 1), |sum, (_, amount)| sum + amount.clone());
        let before = total_element(prior, element);
        let after = total_element(next, element);
        let expected = before + delta;
        if after != expected {
            return Err(Refused::ElementBroken {
                element,
                expected: Box::new(expected),
                actual: Box::new(after),
            });
        }
    }
    // ONT-2: the vein BOOK holds still. The vein carries an extraction
    // integral, not a richness scalar (richness derives as natural −
    // extracted), so the book's whole motion is declared: a prospect (or a
    // supernova scatter) pushes a fresh vein with extracted = 0; an assay
    // moves that vein's span anchor and grows its integral by exactly the
    // banked amount; every other deed leaves the book untouched. Declaration
    // == effect, the banked law's shape applied to the vein registry.
    let mut book = prior.prospects.clone();
    match &step.event {
        ClosureEvent::ProspectStruck { orb_id, stratum, element, .. } => {
            book.push(cfe::world::Prospected {
                orb: *orb_id,
                stratum: *stratum,
                element: *element,
                extracted: ratio(0, 1),
                // ONT-3: a fresh vein has recovered nothing.
                rejuvenated: ratio(0, 1),
                last_assay: step.stamp,
            });
        }
        // #27: a dying star scatters a vein — the same push a prospect makes.
        ClosureEvent::EnrichmentStruck { orb_id, stratum, element, .. } => {
            book.push(cfe::world::Prospected {
                orb: *orb_id,
                stratum: *stratum,
                element: *element,
                extracted: ratio(0, 1),
                // ONT-3: a fresh vein has recovered nothing.
                rejuvenated: ratio(0, 1),
                last_assay: step.stamp,
            });
        }
        ClosureEvent::AssayStruck { orb_id, element, amount, .. } => {
            if let Some(vein) = book
                .iter_mut()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
            {
                vein.last_assay = step.stamp;
                vein.extracted += amount.clone();
            }
        }
        // ONT-2b: eating mines the staple vein — the same book motion an assay
        // makes (the span anchor moves, the integral grows by exactly `eaten`),
        // only the amount is demand-driven, not geometric. Declaration ==
        // effect, the banked law's shape applied to the vein registry.
        ClosureEvent::StapleMined { orb_id, element, eaten, .. } => {
            if let Some(vein) = book
                .iter_mut()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
            {
                vein.last_assay = step.stamp;
                vein.extracted += eaten.clone();
            }
        }
        // ONT-3: rejuvenation advances the vein book — the mirror of the mine,
        // the span anchor moving and the rejuvenation integral growing by
        // exactly `injected`. Declaration == effect, the same banked-law shape.
        ClosureEvent::VeinRejuvenated { orb_id, element, injected, .. } => {
            if let Some(vein) = book
                .iter_mut()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
            {
                vein.last_assay = step.stamp;
                vein.rejuvenated += injected.clone();
            }
        }
        _ => {}
    }
    if next.prospects != book {
        let at = book
            .iter()
            .zip(next.prospects.iter())
            .position(|(expected, actual)| expected != actual)
            .unwrap_or_else(|| book.len().min(next.prospects.len()));
        return Err(Refused::VeinBookBroken { at });
    }
    Ok(())
}

/// The world's total holding of one element, summed across every orb
/// (the element-ledger twin of `total_banked`).
fn total_element(state: &Committed, element: [u8; 32]) -> Ratio {
    state
        .orbs
        .iter()
        .map(|orb| {
            orb.elements
                .iter()
                .find(|(held, _)| *held == element)
                .map(|(_, amount)| amount.clone())
                .unwrap_or_else(|| ratio(0, 1))
        })
        .fold(ratio(0, 1), |sum, held| sum + held)
}

/// The GAUGE law (U2d — I6's kernel half, the Delta-modeling ruling):
/// every transition's effect on the gauge vector is exactly what the
/// deed declares — the fold trio moves the key it names, the graded
/// shift opens exactly its plane, and every other deed holds the
/// vector still. Declaration == effect, the banked law's shape
/// applied to the key; no state jump is special-cased. The pair half
/// of I6 — `orbit_lock` between the two merging chains — is the merge
/// site's law (converge/mesh admission calling the sealed
/// `rfa_core::f_fold_star`): the kernel enforces the LOCAL half, the
/// seam enforces the pair (the aperture-pair pattern, Charter §5a).
fn gauge_move(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    use ClosureEvent as E;
    // The declared key effect — ratified against the replay arms.
    let expected: Vec<Ratio> = match &step.event {
        // The base-key movers: gauge[0] = the declared key.
        E::GaugeShifted { gauge }
        | E::Diverged { gauge, .. }
        | E::Confluent { gauge, .. }
        | E::EraFolded { gauge, .. } => {
            let mut held = prior.gauge.clone();
            if held.is_empty() {
                held.push(ratio(0, 1));
            }
            held[0] = gauge.clone();
            held
        }
        // The graded shift: the vector grows to grade+1 (zero-fill,
        // ground rest) and the declared key seats at its grade.
        E::GaugeShiftedAt { grade, gauge } => {
            let at = *grade as usize;
            let mut held = prior.gauge.clone();
            if held.len() <= at {
                held.resize(at + 1, ratio(0, 1));
            }
            held[at] = gauge.clone();
            held
        }
        // Every other deed declares no key move: the vector holds
        // still (EraAnnexed among them — a universe-ledger record
        // whose pair-half verification is converge's law at the seam).
        _ => prior.gauge.clone(),
    };
    let grades = expected.len().max(next.gauge.len());
    let zero = ratio(0, 1);
    for grade in 0..grades {
        let want = expected.get(grade).unwrap_or(&zero);
        let got = next.gauge.get(grade).unwrap_or(&zero);
        if want != got {
            return Err(Refused::GaugeBroken {
                grade,
                expected: Box::new(want.clone()),
                actual: Box::new(got.clone()),
            });
        }
    }
    Ok(())
}

/// The landed coverage of the invariant set. An event is gated (not
/// refused `Unjudged`) only when every invariant that should govern it has
/// landed — the completeness rule, so partial coverage never admits. The
/// universal trio I1/I2/I3 is now landed, so events whose full set is
/// {containment, exactness} — the pure authority deeds, moving no value —
/// are fully gated. Value-moving deeds beyond the transfer wait on I1's
/// coverage extension; minting/decaying deeds wait on I4/I5. The set grows
/// only by witnessed amendment (§11 clause 2); the kernel wires as the live
/// gate (Phase 3) when this predicate is total (every event covered).
fn covered(event: &ClosureEvent) -> bool {
    use ClosureEvent as E;
    match event {
        // I1 conservation covers the intra-world move and the aperture
        // legs (the convergence seam): a leg debits/credits exactly its
        // declared amount, the local half of pair conservation.
        E::ResourceTransferred { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        // The bounce's arrival is the receive leg's shape: declared, credited.
        | E::CoreReturned { .. }
        // I4 creation / I5 decay (declaration==effect + sign) fully cover
        // the mint and the demurrage decay — per-row (sealed) and the
        // atomic touch alike.
        | E::YieldCollected { .. } | E::DemurrageSettled { .. }
        | E::DemurrageTouched { .. }
        // #32, 20_FOUNDING §3: the founding tithe is a one-sided debit,
        // the DemurrageSettled shape — I1 (declaration==effect) covers
        // it the same way.
        | E::TitheReceipted { .. }
        // U1 — the discovery mint's full set is landed: I1
        // (declaration==effect), I2 (Collect), I3, and I4 whole (sign
        // + the replay-complete verification + the sealed-shut bound).
        | E::DiscoveryMinted { .. }
        // I2 containment + I3 exactness fully cover the authority deeds
        // (no value moves, no ratios): registering an agent and opening
        // or shutting its scopes.
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        // D5 — the SOVEREIGN union. A constitutional deed: no value
        // moves, no ratio is carried, no plane opens. I2 containment
        // covers it and `union` adds the three checks the union
        // needs — the root entity struck it, the parties are a dyad of
        // two different offices, and the review horizon is the ruled
        // one. Ratified red-first by `sovereign_union_pins`.
        | E::SovereignUnionSealed { .. }
        // BOOK-1 (19_LENS §16): minting declares a standing account and
        // archiving alters one — both extend or amend the world's
        // structure, neither moves banked value. Matches
        // `world::books`' own scope checks. The posting joins them
        // (task #17, 2026-08-12 — the §0 audit's one missing row, at
        // last): a dual entry moves book tallies (kept/lost), never
        // `banked` — banked-neutral, {I2, I3} complete it.
        | E::BookMinted { .. } | E::BookPosted { .. } | E::BookArchived { .. }
        // LIT-1: aspiring and claiming both add standing state and move
        // no banked value. Matches `world::literature`.
        | E::WorkAspired { .. } | E::WorkClaimed { .. }
        // W-A slice-1 — the banked-neutral authority deeds. Each one's
        // `world/replay.rs` arm touches no bank (it sets a flag, a
        // seal, a list: terrain seed, star, planet site, practice,
        // ward, veil, genesis, prospect, the succession boundary, a
        // licence record, an instrument, a route, an offer, a bond).
        // So {I2 containment, I3 exactness} complete them, and
        // conservation's non-constraint is correct — ratified against
        // the replay reading in `authority_pins.rs`. The value-moving
        // "authority" deeds (ColonyFounded/Charted, ZoneBuilt/Razed,
        // Forward/Council seals — all move banks) stay held out for
        // W-A slice-2's I1 arms. TaxonSealed stays coder-03's #42.
        | E::WorldCharted { .. } | E::StarCommissioned { .. } | E::PlanetSited { .. }
        // E5b the ESTATE survey (tag 97) — a value-moving deed with its
        // own I1 arm in `lawful_deltas` (the meter's draw, plus the
        // witnessed and metered theorems judged there). Added by hand:
        // the compiler does not force this list, which is exactly how
        // BookPosted went ungated (Pickup §0).
        | E::EstateSurveyed { .. }
        // The appraisal (tag 98) — banked-neutral: a valuation reading
        // moves nothing, and the gate arm re-derives its value from
        // the receipt's own seed and covering. By hand, again: the
        // compiler does not force this list.
        | E::AppraisalStruck { .. }
        // The minted LINE (tag 101) — banked-neutral: identity, never
        // matter; the receipt is a seed and everything derives. By hand.
        | E::LineMinted { .. }
        // Treaties (tags 105, 106) — banked-neutral: a pact re-rules
        // what a crossing costs and moves nothing by itself; the haul
        // that later crosses is the conserved deed. {I2 containment,
        // I3 exactness} complete them. By hand, as ever — the compiler
        // does not force this list.
        | E::TreatySealed { .. } | E::TreatyStruck { .. }
        // Convey (tag 100) — value-moving, with its I1 arm in
        // `admitted_deltas`: the postage burns (net negative), the
        // amount nets zero, and the postal theorems (address, monetary
        // cap, metered postage) are checked there against the receipt's
        // own seed and covering. By hand, as ever.
        | E::ParcelConveyed { .. }
        // NOVELTY-GATE (tag 166) — banked-neutral: a minted solver
        // word is identity vocabulary, never matter; the mints it
        // later enables are judged at tag 86 by verify_work as ever.
        // {I2 containment, I3 exactness} complete it. By hand.
        | E::SolverWordMinted { .. }
        // ENTITY-ALIGN-1 (tag 189) — banked-neutral like the mint it
        // accompanies: a hardness reading is identity vocabulary about
        // which vessels bridged how much diversity, never matter. {I2
        // containment, I3 exactness} complete it. By hand.
        | E::SolverWordHardness { .. }
        // Cross-LINE consilience (tag 177) — banked-neutral: a shared
        // knowledge axiom is identity, never matter; it records that
        // lines converged, moving nothing bankable. By hand.
        | E::KnowledgeConverged { .. }
        | E::KnowledgeLicensed { .. }
        // RCP-2 (tag 181) — banked-neutral at build: raising an emergent
        // recipe moves no value, no element and no key; it opens the
        // society's doing-layer vocabulary, and the per-tick yield it later
        // enables is judged where it banks. {I2 containment, I3 exactness}
        // complete it — the SolverWordMinted discipline. By hand.
        | E::RecipeBuilt { .. }
        // E3.9 (tag 165) — banked-neutral: the allow-list annex
        // re-rules which matter may cross the pair and moves nothing
        // by itself; the haul it later refuses or admits is the
        // conserved deed. {I2 containment, I3 exactness} complete it,
        // the treaty pair's own shape. By hand, as ever.
        | E::TreatyElementsAllowed { .. }
        // The counter (tag 99) — banked-neutral: it re-terms a resting
        // offer and moves nothing; the fill that later clears is the
        // conserved deed. By hand: the compiler does not force this
        // list. Staleness deliberately has no gate arm — it triggers
        // renegotiation at the emitter and gates nothing here.
        | E::OfferCountered { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. } | E::ElderWarded { .. }
        | E::VeilDrawn { .. } | E::WardLifted { .. } | E::VeilLifted { .. }
        // ELEMENT-CHEMISTRY (tag 104) — value-moving in the element
        // ledger, with its arm in `element_move`: the members' spends
        // and the gem's yield are re-derived from the receipt's own
        // members by calling the emitter's recipe, and the gem's
        // identity is re-derived too. By hand, as ever — the compiler
        // does not force this list.
        | E::GemCompounded { .. }
        // REFINE-1 (tag 107): element-ledger move with arm in
        // `element_move` — rates re-derived from `refine_recipe`.
        // By hand: the compiler does not force this list.
        | E::ElementRefined { .. }
        // E-APT-1 (tags 108–109): element-ledger legs, arm in element_move.
        | E::ElementApertureSent { .. } | E::ElementApertureReceived { .. }
        | E::ElementRouteChartered { .. }
        | E::StampMinted { .. } | E::StampApertureSent { .. } | E::StampApertureReceived { .. }
        | E::ScrapSalvaged { .. } | E::HullMinted { .. } | E::HullAssigned { .. } | E::HullScrapped { .. } | E::OrderWorked { .. } | E::ElementSalvaged { .. } | E::TraceConsumed { .. }
        | E::NoteIssued { .. } | E::NoteRedeemed { .. } | E::FutureOpened { .. } | E::FutureSettled { .. } | E::LastClearMarked { .. } | E::ShareIssued { .. } | E::ShareTransferred { .. } | E::ShareDividend { .. } | E::PerpOpened { .. } | E::PerpFunded { .. } | E::PerpClosed { .. } | E::HullFinanced { .. } | E::EscortOnDuty { .. } | E::InterdictionStruck { .. } | E::ConvoyRiskMarked { .. }
        | E::MarketSealed { .. } | E::MarketResumed { .. } | E::SpotClearRecorded { .. } | E::LiquidityRaidStruck { .. } | E::ClearPoisonRefused { .. }
        | E::NoteCascadeStruck { .. } | E::CovertTamperStruck { .. } | E::MediumReinforced { .. } | E::TamperDetected { .. }
        | E::PostureReoriented { .. }
        | E::SpyProbed { .. } | E::TechExfiltrated { .. } | E::InstrumentSabotaged { .. }
        | E::RailPromoted { .. }
        | E::StrandExported { .. } | E::AnchorImported { .. } | E::AnchorImportRefused { .. }
        | E::VaultDeposited { .. } | E::VaultWithdrawn { .. }
        | E::AssayConsilience { .. } | E::AssaySlashed { .. }
        | E::IsoUnionStruck { .. } | E::ChemCataloged { .. } | E::DarkPullStruck { .. }
        | E::ScienceSeated { .. } | E::LabCreated { .. } | E::ScienceExploded { .. }
        | E::RogueFactionStruck { .. }
        | E::PressureEbbed { .. }
        | E::GenesisSealed { .. } | E::ProspectStruck { .. } | E::EnrichmentStruck { .. } | E::LineSucceeded { .. }
        | E::TechLicensed { .. } | E::InstrumentForged { .. } | E::RouteChartered { .. }
        | E::OfferPosted { .. } | E::BondSealed { .. }
        // The missed sixth (arch-02's coverage audit): the enum holds
        // 64 deeds, not the documented 63 — GaugeShiftedAt (tag 60,
        // the graded key shift, live at world/gauge.rs's shift_gauge_at) was
        // never counted. Banked-neutral (replay resizes the gauge key,
        // no bank), Build-scoped beside its covered counterpart
        // GaugeShifted. Without it the Phase 3 flip could never
        // complete: a live deed refusing Unjudged forever.
        | E::GaugeShiftedAt { .. }
        // U2d — the fold trio enters under the GAUGE law (I6's kernel
        // half): the fold's only lawful local effect is its declared
        // key move, banks and every undeclared grade held still. The
        // pair half — orbit_lock between the merging chains via the
        // sealed rfa_core::f_fold_star — is the merge site's law
        // (converge/mesh admission), the aperture-pair seam pattern.
        // Witnessed in gauge_pins.rs.
        | E::Confluent { .. } | E::EraAnnexed { .. } | E::EraFolded { .. }
        // W-C / W-D slice — the banked-neutral world-driven and mesh
        // deeds. Their `world/replay.rs` arms set a flag, a list, or
        // are no-ops (no bank); `scope_of` is None (a tick's own
        // consequence carries no actor scope). Gated by I3 exactness,
        // conservation and containment vacuous — ratified against the
        // replay reading in `world_driven_pins.rs`. Held out: the
        // value-movers of these families (FamineStruck −drawn,
        // ForwardSlashed +collateral, CouncilClosed +banked → the I1
        // value slice) and the fold trio (Confluent/EraAnnexed/
        // EraFolded → U2d's I6 gate; a fold is never admitted on
        // exactness alone).
        | E::FamineRelieved { .. } | E::GalaxyObserved { .. } | E::OverheatStruck { .. }
        | E::OverheatCleared { .. } | E::VentSettled { .. } | E::AbsorptionLapsed { .. }
        | E::WantStruck { .. } | E::WantRelieved { .. }
        | E::ElementWantStruck { .. } | E::ElementWantRelieved { .. }
        | E::PopulationSettled { .. }
        | E::PopulationAccrued { .. }
        | E::TreasurySettled { .. }
        | E::LeisureSettled { .. }
        | E::PracticesLapsed { .. }
        // 50_WIN_CONDITIONS, operator ruling 2026-08-13: the failure
        // state is world-driven and moves no value — I2 containment +
        // I3 exactness cover it the same as PopulationSettled.
        | E::CivilizationCollapsed { .. }
        | E::HandoffSealed { .. }
        | E::GaugeShifted { .. } | E::Diverged { .. } | E::StarAged { .. }
        | E::TechDiscovered { .. } | E::IncursionStruck { .. } | E::VoidShare { .. }
        | E::RouteLapsed { .. }
        // The value law wave (operator directive 2026-07-31): the
        // value-moving deeds under total conservation + the sign law,
        // deltas declared per-resource in `admitted_deltas` (the sealed
        // rules called, stateful reads from prior). Witnessed in
        // `value_pins.rs`. Tier 1 (single-resource, sign-guarded),
        // Tier 2 (escrow moves under the escrow law — conservation reads
        // banked + escrow, so a forward lock/return nets zero on the
        // conserved total; council close held out on Phase 3 threading),
        // Tier 3 (multi-resource vectors).
        | E::UpkeepSettled { .. } | E::CrystalDrawn { .. } | E::OverflowAbsorbed { .. }
        | E::BondSettled { .. } | E::FamineStruck { .. } | E::ForwardSealed { .. }
        | E::CouncilSealed { .. } | E::FrictionSettled { .. } | E::OfferCleared { .. }
        | E::ForwardReleased { .. } | E::ForwardSlashed { .. } | E::CouncilClosed { .. }
        | E::ColonyFounded { .. } | E::ColonyCharted { .. } | E::ZoneBuilt { .. }
        | E::ZoneRazed { .. }
        // The element law (operator ruling 2026-08-01): the assay mints
        // into the element ledger, gated with strict symmetry to the
        // banked ledger — sign-guarded, bounded by the prior vein's
        // richness × elapsed, and zero-invariant for every other deed.
        // Witnessed in `element_pins.rs`.
        | E::AssayStruck { .. }
        // G2 (#29): the element-food synthesis — a conserved transmutation
        // (joule spend + bounded element mint), covered by the value law.
        | E::ElementFarmed { .. }
        // ING-2b (tag 182): the staple food settle on the element ledger —
        // an element move (net regrown − eaten), covered by the element law
        // (`element_move` declares its delta) exactly as the assay and farm
        // beside it. Banked-neutral: it moves no Resource. {I2 containment,
        // I3 exactness} complete it.
        | E::RationElementSettled { .. }
        // ONT-2b (tag 183): eating mines the staple vein — a DELTA-0 element
        // move that only advances the vein book, bounded by the vein's
        // remaining richness (`element_move`). Banked-neutral (no Resource, no
        // element ledger change — food goes vein → mouths). {I2 containment
        // (Collect), I3 exactness} complete it.
        | E::StapleMined { .. }
        // ONT-3 (tag 184): rejuvenation is the mine's mirror — a delta-0 element
        // move, covered by the same {containment, exactness} completion.
        | E::VeinRejuvenated { .. }
        // #136 (tag 185): the SMELT-SWEEP — the one-time grain migration. An
        // element-ledger move that floors every stock, declaring a per-element
        // reduction whose sum is the recorded dust (pre = post + dust); banked-
        // neutral (no Resource moves), a sign-guarded loss like the demurrage decay,
        // covered by the element law beside the assay and farm above.
        | E::ElementStocksSwept { .. }
        // #138 (tag 186): the resource SMELT-SWEEP — the banked-resource + STAMP
        // grain migration. Not banked-neutral: it declares a per-resource floor
        // reduction (arm in `admitted_deltas`) and a STAMP floor reduction (arm in
        // `stamp_move`), each a sign-guarded loss re-derived from prior (the dust
        // is the recorded shave). Conservation + stamp_move gate it fully.
        | E::ResourceStocksSwept { .. }
        // The LAST deed (operator ruling 2026-08-01, folded to arch-01):
        // TaxonSealed — class and clade sealed as identity, once per
        // star (15 §4). {I2 containment (Build), I3 exactness} complete
        // it; it moves no bank, no element, no key (all three
        // conservation laws hold vacuously). With this, `covered()` is
        // total — every one of the 64 deeds is gated; `Unjudged` is
        // unreachable via any current deed (the Phase 3 precondition,
        // witnessed in `taxon_pins.rs`). The sealed default stays as
        // defense for future enum additions (refuse-unknown).
        | E::TaxonSealed { .. }
        // GALACTIC-1 (tag 187): the regional seed sealed as identity, once
        // per world — the TaxonSealed shape exactly (moves no bank, no
        // element, no key; {I2 containment (Build), I3 exactness} complete
        // it).
        | E::RegionCharted { .. }
        | E::CompositionSealed { .. }
        // docs/58 (tags 190/191): the Big Bang and a world's citation of it
        // are identity — they move no bank, no element and no key, so
        // {I2 containment, I3 exactness} complete them, like the regional
        // seed above and the universe ledger's GalaxyObserved.
        | E::UniverseCited { .. }
        | E::UniverseFounded { .. }
        | E::AncientArose { .. }
        // The cores design (tag 193): a core deposit is universe-ledger
        // identity of a debit that already happened in its world.
        | E::CoreDeposited { .. }
        | E::CoreEmitted { .. }
        | E::CoreWordDeposited { .. }
        // docs/69 (tag 197): a tube to a substratum, identity on the universe ledger.
        | E::SubstratumOpened { .. }
        // CHRONICLE-COMPACT-1 (tag 188): a checkpoint is banked-neutral
        // exactly like the regional seed above — it moves no bank, no
        // element, no key, a pure bookkeeping fact about the chronicle
        // itself. {I2 containment, I3 exactness} complete it.
        | E::ChronicleCheckpointed { .. }
        | E::EvolutionAdvanced { .. }
        // The vertical axis (tag 89): the seat is banked-neutral — it
        // moves no value, no element and no key — so {I2 containment,
        // I3 exactness} complete it. Containment is the whole point:
        // a station gates authority, so who may seat one is the law
        // that was missing while it lived in an editable file.
        | E::StationSeated { .. }
        // The vocabulary mint (tag 90, 28_VOCABULARY §7): banked-neutral
        // like the seat above — it moves no value, no element and no
        // key, so {I2 containment, I3 exactness} complete it. Containment
        // is again the whole point: minting is a tier 0 power, so who may
        // grow a vocabulary is the law. The fact gates (standing,
        // novelty) live at the world door where the chain is
        // (`mint_vocabulary_as`); the shape gates died at the `Axis`
        // and `TradingBinding` doors (29_CONSTRUCTION).
        | E::VocabularyMinted { .. }
        // P2 (tag 103): a chartered parameter is banked-neutral like
        // the mint above — it moves no value, no element and no key;
        // {I2 containment, I3 exactness} complete it. Who may charter
        // is the T0 standing gate at the world door; the band is a
        // door (`cfe::parameter::Chartered`).
        | E::ParameterChartered { .. } => true,
        // **The unjudged set is empty, and the compiler holds it so**
        // (task #17, 2026-08-12). This match carries no wildcard by
        // law: the hand-listed universe in `taxon_pins` fell behind
        // twice because Rust cannot count an enum at runtime — so the
        // totality now lives where the compiler enforces it. A new
        // `ClosureEvent` variant breaks this build until someone
        // classifies it: covered above, or given an explicit `=> false`
        // arm here wearing its reason. `BookPosted` — the §0 audit's
        // one missing row — was the last, covered at this landing;
        // the Phase-3 judged-precondition is now structural.
    }
}

/// Invariant — conservation, total (the value-law wave, operator
/// directive 2026-07-31; the escrow strengthening, dispatch C7
/// 2026-08-01). A deed changes the world's conserved total — banked
/// plus escrowed — by exactly its declared per-resource deltas and
/// nothing else:
///
/// ```text
/// ∀ r ∈ Resources:  conserved(next, r) == conserved(prior, r) + declared(r)
///   where conserved(s, r) = banked(s, r) + escrow(s, r)
/// ```
///
/// The escrow half (`total_escrow`) closes the lock seam: a forward
/// seal debits a bank and must credit escrow the same collateral (net
/// declared zero) — a debit that escrows nothing no longer passes.
/// Council close is held out (its atomic multi-event settle is per-deed
/// threading, gated on Phase 3). Every resource present in either state,
/// in a lock, or in the declaration is checked — a covered deed with an
/// empty declaration must leave the conserved total untouched (a
/// smuggled change under a neutral deed refuses).
/// Multi-resource deeds (founding reserves, zone costs and refunds)
/// declare vectors, never a scalar. The pair-level law (a far leg is
/// the near leg's exact inverse) is the cross-node opposition (kernel
/// Charter §5a); the kernel enforces the LOCAL half.
fn conservation(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    let deltas = admitted_deltas(prior, &step.event)?;
    let mut resources: Vec<Resource> = Vec::new();
    for (resource, _) in &deltas {
        if !resources.contains(resource) {
            resources.push(*resource);
        }
    }
    for orb in prior.orbs.iter().chain(next.orbs.iter()) {
        for (resource, _) in &orb.banked {
            if !resources.contains(resource) {
                resources.push(*resource);
            }
        }
    }
    // Escrow-held resources enter the conserved total too: a resource
    // present only in a lock (no bank holds it) must still be checked —
    // else a forward could seal or return unwitnessed.
    for pact in prior.forwards.iter().chain(next.forwards.iter()) {
        if !resources.contains(&pact.resource) {
            resources.push(pact.resource);
        }
    }
    for pact in prior.councils.iter().chain(next.councils.iter()) {
        if !resources.contains(&pact.resource) {
            resources.push(pact.resource);
        }
    }
    for resource in resources {
        let declared = deltas
            .iter()
            .filter(|(held, _)| *held == resource)
            .fold(ratio(0, 1), |sum, (_, delta)| sum + delta.clone());
        let before = total_conserved(prior, resource);
        let after = total_conserved(next, resource);
        let expected = before + declared;
        if after != expected {
            return Err(Refused::ConservationBroken {
                resource,
                before: Box::new(expected), // the lawful total this deed requires
                after: Box::new(after),     // the total the step actually produced
            });
        }
    }
    Ok(())
}

/// The declared per-resource banked deltas of a deed — the lawful
/// movement, sign-guarded at the door (the sign law: every amount is
/// non-negative by declaration; direction comes from the deed). The
/// sealed rules are called, never copied (founding reserves, zone
/// costs/refunds — Charter §0b). Stateful declarations (the council's
/// resource, the razed zone's kind) read prior state, never the event.
/// An event with no arm declares nothing: every bank must hold still.
fn admitted_deltas(
    prior: &Committed,
    event: &ClosureEvent,
) -> Result<Vec<(Resource, Ratio)>, Refused> {
    use ClosureEvent as E;
    let non_negative = |resource: Resource, amount: &Ratio| match *amount < ratio(0, 1) {
        true => Err(Refused::AmountNotPositive {
            resource,
            amount: Box::new(amount.clone()),
        }),
        false => Ok(()),
    };
    Ok(match event {
        // — the moves (net zero), sign-guarded: a negative move would
        //   invert the flow and authorize the wrong side —
        E::ResourceTransferred { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            Vec::new()
        }
        E::BondSettled { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            Vec::new()
        }
        // ColonyCharted: the founding reserves move source → charted —
        // net zero per resource by the sealed rule.
        E::ColonyCharted { .. } => Vec::new(),
        // — the aperture legs, sign-guarded (a negative send is a
        //   receive wearing a send's name) —
        E::ApertureSent { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, -amount.clone())]
        }
        E::ApertureReceived { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, amount.clone())]
        }
        // The bounce (tag 195): a core's matter pulse arrives — declared,
        // sign-guarded; its STAMP twin is judged in stamp_move.
        E::CoreReturned { resource: Some(resource), amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, amount.clone())]
        }
        E::CoreReturned { resource: None, .. } => Vec::new(),
        // STAMP-1: crystal spent for identity mint — resource debit only;
        // STAMP balance is judged in stamp_move.
        E::StampMinted { crystal_spent, amount, .. } => {
            non_negative(Resource::Crystal, crystal_spent)?;
            if *amount != *crystal_spent {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(crystal_spent.clone()),
                    claimed: Box::new(amount.clone()),
                });
            }
            vec![(Resource::Crystal, -crystal_spent.clone())]
        }
        // FLEET-1: iron → scrap identity; scrap is not Resource.
        E::ScrapSalvaged { iron_spent, amount, .. } => {
            non_negative(Resource::Iron, iron_spent)?;
            if *amount != *iron_spent {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(iron_spent.clone()),
                    claimed: Box::new(amount.clone()),
                });
            }
            vec![(Resource::Iron, -iron_spent.clone())]
        }
        // Hull mint/assign/scrap move scrap ledger only (not Resource).
        E::HullMinted { .. } | E::HullAssigned { .. } | E::HullScrapped { .. } | E::OrderWorked { .. } => Vec::new(),
        E::ElementSalvaged { .. } | E::TraceConsumed { .. } => Vec::new(),
        // ING-2b (tag 182): the staple food settle moves the element ledger
        // only (declared in `element_move`), never a Resource — no banked
        // delta, exactly as TraceConsumed.
        // ONT-2b (tag 183): the staple mine moves the vein book only (a
        // delta-0 element move), never a Resource — no banked delta.
        E::RationElementSettled { .. } | E::StapleMined { .. } | E::VeinRejuvenated { .. } => Vec::new(),
        E::NoteIssued { .. } | E::NoteRedeemed { .. } | E::FutureOpened { .. } | E::FutureSettled { .. } | E::LastClearMarked { .. } | E::ShareIssued { .. } | E::ShareTransferred { .. } | E::ShareDividend { .. } | E::PerpOpened { .. } | E::PerpFunded { .. } | E::PerpClosed { .. } | E::HullFinanced { .. } | E::EscortOnDuty { .. } | E::InterdictionStruck { .. } | E::ConvoyRiskMarked { .. }
        | E::MarketSealed { .. } | E::MarketResumed { .. } | E::SpotClearRecorded { .. } | E::LiquidityRaidStruck { .. } | E::ClearPoisonRefused { .. }
        | E::NoteCascadeStruck { .. } | E::TamperDetected { .. } | E::PostureReoriented { .. } => {
            Vec::new()
        }
        E::CovertTamperStruck { cost, .. }
        | E::SpyProbed { fee: cost, .. }
        | E::TechExfiltrated { fee: cost, .. } => {
            non_negative(Resource::Crystal, cost)?;
            vec![(Resource::Crystal, -cost.clone())]
        }
        E::MediumReinforced { cost, resource, .. } => {
            non_negative(*resource, cost)?;
            vec![(*resource, -cost.clone())]
        }
        E::InstrumentSabotaged { .. }
        | E::RailPromoted { .. }
        | E::StrandExported { .. }
        | E::AnchorImported { .. }
        | E::AnchorImportRefused { .. }
        | E::VaultDeposited { .. }
        | E::VaultWithdrawn { .. }
        | E::AssayConsilience { .. }
        | E::IsoUnionStruck { .. }
        | E::DarkPullStruck { .. }
        | E::RogueFactionStruck { .. }
        | E::PressureEbbed { .. } => Vec::new(),
        E::AssaySlashed { slash, .. } => {
            non_negative(Resource::Crystal, slash)?;
            vec![(Resource::Crystal, -slash.clone())]
        }
        // Chem catalog fee: resonator eighth crystal.
        E::ChemCataloged { .. } => {
            let fee = cfe::dop::ZoneKind::Resonator.yield_per_tick().1;
            non_negative(Resource::Crystal, &fee)?;
            vec![(Resource::Crystal, -fee)]
        }
        // Lab: multi-resource bills (local + resonance) — not STAMP.
        // Re-derive from pure tables; metabolism from sealed genesis.
        E::ScienceSeated {
            discipline,
            rank,
            fee,
            ..
        } => {
            let met = prior
                .genesis
                .as_ref()
                .map(|g| g.metabolism)
                .unwrap_or(2);
            if let Some(d) = cfe::almanac::ScienceDiscipline::from_wire(*discipline) {
                // Re-derive the seat AFFINITY exactly as the cfe seat pays it
                // (call-not-copy): the bill scales by the species' affinity
                // (Primary 1 · Secondary 2 · Alien 4), and Research by its economic
                // mode besides. The old hard-coded `affinity_mult = 1` under-declared
                // any non-Primary bill, so the seat consumed more resource than this
                // conservation check was told — a break only a non-Primary Iron-billed
                // seat surfaces (Physics off a mineral line, once COG-8 seats it).
                // Primary seats (Chem for its own metabolism) read aff = 1 as before,
                // so every current seat is byte-identical.
                let clade = prior.taxon.map(|(_, c)| c);
                let mut aff = cfe::almanac::science_seat_affinity(met, d, clade).fee_mult();
                if matches!(d, cfe::almanac::ScienceDiscipline::Research) {
                    let (class, eco_clade) =
                        prior.taxon.unwrap_or((7, u16::from(cfe::almanac::FLATLANDER_CLADE)));
                    aff *= cfe::almanac::research_affinity_mult(cfe::almanac::economic_mode(
                        class, eco_clade,
                    ));
                }
                let mut out = Vec::new();
                for (r, a) in cfe::almanac::science_seat_bill(d, *rank, met, aff) {
                    non_negative(r, &a)?;
                    out.push((r, -a));
                }
                out
            } else {
                non_negative(Resource::Crystal, fee)?;
                vec![(Resource::Crystal, -fee.clone())]
            }
        }
        E::LabCreated { path, fee, .. } => {
            let met = prior
                .genesis
                .as_ref()
                .map(|g| g.metabolism)
                .unwrap_or(2);
            if let Some(p) = cfe::almanac::LabPath::from_wire(*path) {
                let mut out = Vec::new();
                for (r, a) in cfe::almanac::lab_create_bill(p, met) {
                    non_negative(r, &a)?;
                    out.push((r, -a));
                }
                out
            } else {
                non_negative(Resource::Crystal, fee)?;
                vec![(Resource::Crystal, -fee.clone())]
            }
        }
        E::ScienceExploded { resource, slash, .. } => {
            let res = Resource::from_wire(*resource).unwrap_or(Resource::Crystal);
            non_negative(res, slash)?;
            if *slash > cfe::ratio::ratio(0, 1) {
                vec![(res, -slash.clone())]
            } else {
                Vec::new()
            }
        }
        // — mints (the sign lives at I4, order preserved for the
        //   landed pins) —
        E::YieldCollected { resource, amount, .. } => vec![(*resource, amount.clone())],
        E::DiscoveryMinted { resource, amount, .. } => vec![(*resource, amount.clone())],
        // — decays (the sign lives at I5, order preserved) —
        E::DemurrageSettled { resource, amount, .. } => vec![(*resource, -amount.clone())],
        E::TitheReceipted { resource, amount, .. } => vec![(*resource, -amount.clone())],
        E::DemurrageTouched { decays, .. } => decays
            .iter()
            .map(|(resource, amount)| (*resource, -amount.clone()))
            .collect(),
        // — Tier 1: single-resource debits, sign-guarded —
        E::UpkeepSettled { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, -amount.clone())]
        }
        E::CrystalDrawn { amount, .. } => {
            non_negative(Resource::Crystal, amount)?;
            vec![(Resource::Crystal, -amount.clone())]
        }
        // G2 (#29): the joule spent to synthesize the food-element leaves
        // the resource ledger (the element mint is checked in
        // element_move). Sign-guarded — a farm cannot bank energy.
        E::ElementFarmed { joule, .. } => {
            non_negative(Resource::Joule, joule)?;
            vec![(Resource::Joule, -joule.clone())]
        }
        // Convey — the postal theorems, held at the gate. The move nets
        // zero (amount out, amount in); only the postage leaves the
        // total, burned like upkeep. Address and cap verified from the
        // receipt's own seed and covering against the prior state;
        // postage verified by calling the postal meter.
        E::ParcelConveyed { from_orb, to_orb, resource, amount, postage, seed, covered, .. } => {
            non_negative(*resource, amount)?;
            for endpoint in [from_orb, to_orb] {
                if !covered.contains(endpoint) {
                    return Err(Refused::ConveyOutsideSurvey { orb: *endpoint });
                }
            }
            let (stamp, _) = cfe::dop::postal_meter();
            let metered = cfe::dop::postage(amount);
            if *postage != metered {
                return Err(Refused::PostageMismatch {
                    declared: Box::new(postage.clone()),
                    metered: Box::new(metered),
                });
            }
            let monetary = cfe::dop::parcels_of(seed, covered)
                .into_iter()
                .find(|parcel| parcel.orb_id == *from_orb)
                .map(|parcel| parcel.monetary)
                .unwrap_or_else(|| ratio(0, 1));
            let cap = prior
                .orbs
                .iter()
                .find(|held| held.orb_id == *from_orb)
                .map(|held| monetary * held.banked_amount(*resource))
                .unwrap_or_else(|| ratio(0, 1));
            if *amount > cap {
                return Err(Refused::ConveyExceedsMonetary {
                    asked: Box::new(amount.clone()),
                    cap: Box::new(cap),
                });
            }
            vec![(stamp, -postage.clone())]
        }
        // E5b the ESTATE survey — the three theorems, held at the gate. The meter
        // and the covering are verified here, against the prior state
        // and the sealed rule, before conservation checks the draw.
        E::EstateSurveyed { covered, cost, .. } => {
            // Witnessed and whole: one witness per orb that stands.
            if covered.is_empty() || covered.len() != prior.orbs.len() {
                return Err(Refused::SurveyNotWhole {
                    covered: covered.len(),
                    estate: prior.orbs.len(),
                });
            }
            // Metered: the declared cost is the sealed meter's figure.
            let (resource, rate) = cfe::dop::survey_meter();
            let metered = rate * ratio(covered.len() as i64, 1);
            if *cost != metered {
                return Err(Refused::SurveyMeterMismatch {
                    declared: Box::new(cost.clone()),
                    metered: Box::new(metered),
                });
            }
            vec![(resource, -cost.clone())]
        }
        E::OverflowAbsorbed { amount, .. } => {
            non_negative(Resource::Sink, amount)?;
            vec![(Resource::Sink, -amount.clone())]
        }
        E::FamineStruck { resource, drawn, .. } => {
            non_negative(*resource, drawn)?;
            vec![(*resource, -drawn.clone())]
        }
        // The council locks collateral × every member out of the banks
        // and into escrow (the sealed council enters `next`) — a move
        // within the conserved total, so the declared net is zero. The
        // sign guard stays at the door. (The escrow law.)
        E::CouncilSealed { resource, collateral, .. } => {
            non_negative(*resource, collateral)?;
            Vec::new()
        }
        // — Tier 2: escrow moves (the escrow law, strengthened
        //   2026-08-01) —
        // A forward lock/return moves collateral between escrow and a
        // bank: the whole forward enters `next` on seal (escrow rises by
        // the collateral) and leaves on release/slash (escrow falls). Now
        // that conservation reads banked + escrow (`total_conserved`),
        // the declared net is zero — the bank move is forced to equal the
        // escrow move, or it refuses. A seal that debits a bank without
        // escrowing the collateral no longer passes. Sign guard at the
        // door. (Council close stays declaration==effect on banked — its
        // multi-event atomic settle is per-deed threading, gated on the
        // Phase 3 flip; see the CouncilClosed arm.)
        E::ForwardSealed { resource, collateral, .. }
        | E::ForwardReleased { resource, collateral, .. }
        | E::ForwardSlashed { resource, collateral, .. } => {
            non_negative(*resource, collateral)?;
            Vec::new()
        }
        // The close is stateful: the resource reads from the prior
        // sealed council, never the event. An unknown council declares
        // nothing — a banked change under it refuses.
        // Held out (the escrow seam): a council reckons at its boundary
        // as one atomic settle — the whole council removed, every member
        // banked, N `CouncilClosed` events emitted for one state
        // transition. Per-deed escrow discharge (escrow −collateral as
        // each member's seat closes) needs per-deed threading, which the
        // operator gated on the Phase 3 flip (arch-02's conductor lane).
        // Until then the close declares its banked credit and the
        // council is held constant across the deed's prior→next; the
        // escrow discharge is conserved at the settle boundary, not
        // per-event. Forwards (1 deed ↔ 1 escrow move) are strengthened.
        E::CouncilClosed { council, banked, .. } => {
            match prior.councils.iter().find(|held| held.council == *council) {
                Some(held) => {
                    non_negative(held.resource, banked)?;
                    vec![(held.resource, banked.clone())]
                }
                None => Vec::new(),
            }
        }
        // — Tier 3: multi-resource, the sealed rules called never
        //   copied —
        // The chartered genesis mint: exactly the founding reserves.
        E::ColonyFounded { .. } => cfe::dop::founding_reserves(),
        // STAR-ENDOW-1: commissioning tops the genesis orb up to the
        // class kit — exactly the top-up, re-derived from `prior` by
        // the same sealed pair the emitter and replay call
        // (`star_endowment` · `endowment_top_up_deltas`; the kit never
        // strips spent banks, so the delta depends on what stood).
        // Found live (2026-08-12 judge tally): c2242de landed the mint
        // without this arm, so every non-G commissioning read as
        // ConservationBroken.
        E::StarCommissioned { class } => match cfe::thermal::StarClass::from_wire(*class) {
            Some(class) => prior
                .orbs
                .iter()
                .find(|held| held.orb_id == cfe::world::FIRST_ORB)
                .map(|orb| {
                    cfe::dop::endowment_top_up_deltas(orb, &cfe::dop::star_endowment(class))
                })
                .unwrap_or_default(),
            // An unknown class names no kit; the sky gate refuses the
            // deed on its own terms before value is weighed.
            None => Vec::new(),
        },
        // ORBIT-ENDOW-1: siting gifts the band's kit additively —
        // re-derived from the star and the receipt's own siting by the
        // same sealed reads the emitter used (`t_eq4` · `band_of` ·
        // `orbit_endowment`). No star, no gift (the emitter refuses
        // unsited commissioning first).
        E::PlanetSited { albedo, orbit_r, .. } => match prior.star {
            Some(class) => {
                let teq =
                    cfe::thermal::t_eq4(&class.luminosity(), albedo.value(), orbit_r.value());
                cfe::dop::orbit_endowment(cfe::thermal::band_of(&teq))
            }
            None => Vec::new(),
        },
        // #41: a build is priced on its base. A minted kind's base
        // lives in the fold, which this function cannot read — `prior`
        // is committed state and carries no chronicle — so the receipt
        // declares it and the gate reads the declaration, exactly as
        // tag 101 declares the ceilings its draw ran under. A compiled
        // kind declares none and is its own base.
        E::ZoneBuilt { kind, base, .. } => base
            .unwrap_or(*kind)
            .cost()
            .into_iter()
            .map(|(resource, cost)| (resource, -cost))
            .collect(),
        // The raze refunds the prior zone's kind at the tile (stateful
        // — the event names only the tile). No zone, no refund.
        E::ZoneRazed { orb_id, tile } => prior
            .orbs
            .iter()
            .find(|held| held.orb_id == *orb_id)
            .and_then(|orb| orb.zones.iter().find(|zone| zone.tile == *tile))
            // #41: the salvage halves what the build spent, and for a
            // minted kind that is its base's cost. The base was fixed
            // at build time and judged then, so reading it from
            // committed state here needs no fold and trusts nothing new.
            .map(|zone| zone.priced_as().raze_refund())
            .unwrap_or_default(),
        // #138 the resource SMELT-SWEEP (tag 186): the migration floors every
        // banked resource stock to the 1/64 grain. RE-derive the per-resource
        // floor deltas from prior (call-not-copy — the deed carries only a scalar
        // dust, never per-resource amounts a receipt could quote one way and
        // settle another): each stock's declared delta is `smelt(prior) − prior`,
        // a non-positive shave to the void, summed per resource by the total-check
        // below. This mirrors `sweep_resource_grain` exactly, so
        // `after == before + declared` holds and the migration is
        // conservation-exact. (The STAMP floor is judged in `stamp_move`.)
        E::ResourceStocksSwept { .. } => {
            let mut declared: Vec<(Resource, Ratio)> = Vec::new();
            for orb in &prior.orbs {
                for (resource, amount) in &orb.banked {
                    let floored = cfe::world::smelt_resource(amount);
                    if floored != *amount {
                        declared.push((*resource, floored - amount.clone()));
                    }
                }
            }
            declared
        }
        // No arm: the deed declares no banked movement — the total
        // check holds every bank still.
        _ => Vec::new(),
    })
}

/// The world's total banked holding of one resource, summed across
/// every orb. Pure; the canonical ledger drops zero entries, so the
/// sum is over what is actually held.
fn total_banked(state: &Committed, resource: Resource) -> Ratio {
    state
        .orbs
        .iter()
        .map(|orb| orb.banked_amount(resource))
        .fold(ratio(0, 1), |sum, held| sum + held)
}

/// The escrow law (operator dispatch C7, 2026-08-01). The world's total
/// escrowed holding of one resource — collateral that sits out of every
/// bank, locked in open pacts (FORWARD-1 §7): each forward locks its
/// `collateral`; each council locks `collateral` per member. Pure; the
/// canonical projection carries only open pacts, so the sum is over what
/// is actually locked.
fn total_escrow(state: &Committed, resource: Resource) -> Ratio {
    let forwards = state
        .forwards
        .iter()
        .filter(|pact| pact.resource == resource)
        .fold(ratio(0, 1), |sum, pact| sum + pact.collateral.clone());
    let councils = state
        .councils
        .iter()
        .filter(|pact| pact.resource == resource)
        .fold(ratio(0, 1), |sum, pact| {
            sum + pact.collateral.clone() * ratio(pact.members.len() as i64, 1)
        });
    forwards + councils
}

/// The conserved quantity — banked plus escrowed. A lock moves value
/// between the two ledgers, never in or out of the world; conservation
/// reads this total, so a bank debit under a lock must be matched by an
/// escrow credit (or the reverse), exactly, or the deed refuses.
fn total_conserved(state: &Committed, resource: Resource) -> Ratio {
    total_banked(state, resource) + total_escrow(state, resource)
}

/// Invariant — bounded creation (I4). Value is created only by a lawful
/// mint, and the mint is non-negative (a mint that destroys value is a
/// burn mislabelled — refused). The declaration==effect half is
/// conservation (the world total rose by exactly the declared amount);
/// I4 adds the sign. Named seam: at convergence scale the mint is bounded
/// by verified work — `mint ≤ verify_certificate(work)` — the PoUW anchor
/// (Kernel Charter §5a), which fills on Ruling Box 1. No fiat (05 §1.2).
fn creation(step: &Step) -> Result<(), Refused> {
    match &step.event {
        ClosureEvent::YieldCollected { resource, amount, .. } if *amount < ratio(0, 1) => {
            Err(Refused::MintNotPositive { resource: *resource, amount: Box::new(amount.clone()) })
        }
        // U1 — I4 completes on the discovery mint: sign, then the
        // REPLAY-COMPLETE verification (the kernel hands the opaque
        // payload to the sealed verifier whole, never parses it),
        // then the bound. No certificate → no mint; a mint past what
        // the work earns refuses with both sides carried.
        ClosureEvent::DiscoveryMinted { resource, amount, work, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::MintNotPositive {
                    resource: *resource,
                    amount: Box::new(amount.clone()),
                });
            }
            let bound =
                carrier_bound(work).map_err(|broken| Refused::WorkRefused { broken })?;

            match *amount <= bound {
                true => Ok(()),
                false => Err(Refused::MintExceedsWork {
                    bound: Box::new(bound),
                    claimed: Box::new(amount.clone()),
                }),
            }
        }
        _ => Ok(()),
    }
}

/// The mint bound — ruled (Ruling Box 1, operator 2026-08-01; the
/// VERSION 1→2 amendment). The emission rate `f` is pinned against
/// the #17 calibration's measured hit-rates (evidence commit
/// 5898351: 100% · 100% · 100% · 97.5% · 75% · 42.5%), in exact
/// integers on the ladder — L1/ladder terms only, no norms, no roots
/// (The proportional law):
///
/// ```text
/// f(0) = 0 · f(1) = 0 · f(2) = 0   (saturated — free issuance barred)
/// f(3) = 1                          (the marginal threshold, 97.5%)
/// f(4) = 4                          (scarcity tier 1, 75%)
/// f(5) = 7                          (scarcity tier 2, 42.5% — the
///                                    ~1.75× inverse-hit-rate step)
/// ```
///
/// `f` is defined on the ladder only: a payload's shape either matches
/// a ladder rung exactly or it earns zero — an off-ladder shape cannot
/// name its own price (the attacker-shaped hole the #17 audit closed
/// stays closed at the rate too).
///
/// **Priced on the RUNG, not the shape** (ruling of 2026-08-04: `Grid`
/// falls out, because the engine is not building a matrix). This
/// took `&Difficulty` and matched the whole struct — so `rows` and
/// `cols`, the dimensions of an ARC grid, were part of the gate's
/// price lookup. The motion clause 5 forbids exactly that: *ARC is a
/// type of work, not the only type.* The exact-match defence is
/// unchanged and now lives in `Difficulty::rung`, which is where the
/// shape belongs; the gate is handed a POSITION. One pricing path now
/// serves every work class, planar and volumetric alike, instead of
/// each new class needing its own.
/// Work-shapes v1: planar forge tasks only (structural — `verify_work`
/// accepts no other form). Reward split v1: 100% to the solving world
/// (the deed banks on its own orb; no mesh counterparty exists yet;
/// the v2 40/40/20 prover/mesh/reserve split is inscribed in
/// U_DIRECTIVES §8 for the mesh era). Saturated rungs are
/// proof-of-position; the economy lives up the ladder.
/// **The carrier law** (P4, operator ruling 2026-08-06) — what a work
/// class is worth at the mint, in one place for the gate arm and the
/// sealed warrant alike (Charter §0b, call-not-copy):
///
/// - **Forge** stays priced by the ladder until the corpus rates are
///   MEASURED — the acknowledged sequencing: W(τ) = complexity /
///   solve-rate needs P3's blind benchmark, and zeroing the forge
///   before corpus can pay would zero issuance.
/// - **Volumetric is RETIRED.** It never could mint (an accident of
///   `verify_work`'s return type); the refusal is now the ruling,
///   named `CarrierRetired`. Verification stands forever — history is
///   never orphaned.
/// - **Corpus is staged.** Verified whole, refused `CarrierUnpriced`
///   until the measured-rate table fills from the benchmark; the flip
///   that prices corpus and zeroes the forge is the carrier switch,
///   and it carries the version bump. This landing moves no verdict:
///   volume and corpus refused before and refuse still, with their
///   reasons named instead of accidental.
fn carrier_bound(work: &[u8]) -> Result<Ratio, WorkBroken> {
    match verify_work_class(work)? {
        WorkClass::Forge(difficulty) => Ok(mint_bound(difficulty.rung())),
        WorkClass::Volumetric(_) => Err(WorkBroken::CarrierRetired),
        // The ratified pricing (2026-08-06): a corpus solve the
        // benchmark MEASURED prices at
        //   W(τ) = scarcity(class) × VisitedNodes₃(τ) / word_len(τ)
        // — population scarcity 1243/34 (blind: 283/1), the search
        // frontier the sweep provably expands, over the canonical
        // word. The benchmark is the measurement authority: a task it
        // never confirmed refuses unpriced until a re-measurement
        // extends the table (carrier_table.rs, regenerable).
        WorkClass::Corpus { task_id } => {
            corpus_weight(&task_id).ok_or(WorkBroken::CarrierUnpriced)
        }
    }
}

/// **The spectrum — the cognition reading of one chronicle** (P3).
/// Derived whole from the work the chain actually minted, through the
/// same sealed verifier the gate runs (call-not-copy). Nothing is
/// stored and nothing is scored: a reading is a projection of
/// receipts, and nothing optimizes a reading (19_LENS §1 — the same
/// law GROWTH-1 carries). The caller hands the opaque work payloads;
/// cfe never imports the verifier (the opaque-payload law holds).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Spectrum {
    /// Confirmed forge mints per rung, 0..=5.
    pub forge_rungs: [u32; 6],
    /// Corpus mints the measured table prices — and of those, blind.
    pub corpus_measured: u32,
    pub corpus_blind: u32,
    /// Work that verifies under no current carrier price (retired or
    /// unpriced) — held, never hidden.
    pub unpriced: u32,
    /// Work that does not verify at all.
    pub broken: u32,
    /// The weighted cognition mass: Σ carrier_bound over priced work.
    pub weighted: Ratio,
}

pub fn spectrum_of<'a>(work: impl Iterator<Item = &'a [u8]>) -> Spectrum {
    let mut read = Spectrum { weighted: ratio(0, 1), ..Default::default() };
    for payload in work {
        match verify_work_class(payload) {
            Ok(WorkClass::Forge(difficulty)) => {
                if let Some(rung) = difficulty.rung() {
                    if let Some(held) = read.forge_rungs.get_mut(rung as usize) {
                        *held += 1;
                    }
                    read.weighted = read.weighted.clone() + mint_bound(Some(rung));
                }
            }
            Ok(WorkClass::Corpus { task_id }) => match corpus_weight(&task_id) {
                Some(weight) => {
                    read.corpus_measured += 1;
                    if carrier_table::MEASURED
                        .iter()
                        .any(|(held, _, _, blind)| held == &task_id && *blind)
                    {
                        read.corpus_blind += 1;
                    }
                    read.weighted = read.weighted.clone() + weight;
                }
                None => read.unpriced += 1,
            },
            Ok(WorkClass::Volumetric(_)) => read.unpriced += 1,
            Err(_) => read.broken += 1,
        }
    }
    read
}

/// The measured weight of a confirmed corpus task, or `None` where
/// the benchmark never confirmed it.
pub fn corpus_weight(task_id: &[u8; 32]) -> Option<Ratio> {
    carrier_table::MEASURED
        .iter()
        .find(|(held, _, _, _)| held == task_id)
        .map(|(_, visited, word_len, blind)| {
            let scarcity = match blind {
                true => ratio(carrier_table::BLIND_POPULATION, 1),
                false => {
                    let (population, confirmed) = carrier_table::MEASURED_POPULATION;
                    ratio(population, confirmed)
                }
            };
            scarcity * ratio(*visited as i64, i64::from(*word_len))
        })
}

pub fn mint_bound(rung: Option<u32>) -> Ratio {
    match rung {
        Some(3) => ratio(1, 1),
        Some(4) => ratio(4, 1),
        Some(5) => ratio(7, 1),
        // Saturated rungs pay zero by ruling, and off-ladder pays zero
        // because a payload never names its own price.
        _ => ratio(0, 1),
    }
}

/// Invariant — bounded decay (I5), complete (U1f). Value is destroyed
/// non-negative decay (a decay that creates value is refused). The
/// declaration==effect half is conservation (the world total fell by
/// exactly the declared amount); I5 adds the sign. Named seam: the decay
/// is flat and denominator-bounded, `decay ≤ rate · ease · elapsed`
/// (never proportional — the 2^1100 lesson), which fills when the sealed
/// demurrage rule + `last_touch` join the check.
fn decay(prior: &Committed, step: &Step) -> Result<(), Refused> {
    match &step.event {
        ClosureEvent::DemurrageSettled { orb_id, resource, amount } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::DecayNotPositive {
                    resource: *resource,
                    amount: Box::new(amount.clone()),
                });
            }
            // I5's bound (U1f): the sealed demurrage RULE, called
            // never copied (Charter §0b) — one computation, two
            // gates. `elapsed` reads the prior orb's `last_touch`,
            // already committed on the orb wire (tag 28); an orb the
            // rule owes nothing (no leak, no bank, no time) bounds
            // at zero, so only the zero decay is lawful there.
            let due = prior
                .orbs
                .iter()
                .find(|held| held.orb_id == *orb_id)
                .and_then(|orb| {
                    cfe::world::demurrage_due(
                        orb,
                        prior.star,
                        step.stamp,
                        // Ruleset EPOCHS (`21_KERNEL_PARAMETERS` §11): the gate
                        // selects the rule by the stamp the deed carries,
                        // not by the rule standing today. A receipt
                        // written under epoch 0 is gated by epoch 0
                        // forever, which is what stops `replay` drifting
                        // from the live path when a sealed rule moves.
                        &cfe::world::ease_of(&prior.techs, *orb_id, step.stamp),
                        // P2: the gate reads standing supply from the
                        // same sealed function the proposer used, on
                        // `prior`'s own orbs — one computation, two
                        // gates (Charter §0b). A rate the kernel
                        // accepted without re-deriving would be a number
                        // a node could lie about, with the mint directly
                        // downstream.
                        &cfe::world::standing_supply(&prior.orbs),
                    )
                    .into_iter()
                    .find(|(held, _)| held == resource)
                    .map(|(_, held)| held)
                })
                .unwrap_or_else(|| ratio(0, 1));
            match *amount <= due {
                true => Ok(()),
                false => Err(Refused::DecayExceedsBound {
                    bound: Box::new(due),
                    claimed: Box::new(amount.clone()),
                }),
            }
        }
        // The atomic TOUCH (tag 102, the demurrage seam closed
        // 2026-08-06): one receipt carries every resource the touch
        // settled, so the whole settlement is judged against the anchor
        // that stood before it — the per-row form judged each row
        // against a prior the previous row had already anchored, and a
        // dual-resource touch's second row always read elapsed 0. The
        // sealed rule is still called, never copied; it is called once
        // for the touch, exactly as the emitter called it.
        ClosureEvent::DemurrageTouched { orb_id, decays } => {
            for (at, (resource, amount)) in decays.iter().enumerate() {
                if *amount < ratio(0, 1) {
                    return Err(Refused::DecayNotPositive {
                        resource: *resource,
                        amount: Box::new(amount.clone()),
                    });
                }
                // One window settles one row per resource — a repeat
                // would draw the same window twice.
                if decays[..at].iter().any(|(held, _)| held == resource) {
                    return Err(Refused::DecayRowDuplicated { resource: *resource });
                }
            }
            let due_rows = prior
                .orbs
                .iter()
                .find(|held| held.orb_id == *orb_id)
                .map(|orb| {
                    cfe::world::demurrage_due(
                        orb,
                        prior.star,
                        step.stamp,
                        &cfe::world::ease_of(&prior.techs, *orb_id, step.stamp),
                        &cfe::world::standing_supply(&prior.orbs),
                    )
                })
                .unwrap_or_default();
            for (resource, amount) in decays {
                let due = due_rows
                    .iter()
                    .find(|(held, _)| held == resource)
                    .map(|(_, held)| held.clone())
                    .unwrap_or_else(|| ratio(0, 1));
                if *amount > due {
                    return Err(Refused::DecayExceedsBound {
                        bound: Box::new(due),
                        claimed: Box::new(amount.clone()),
                    });
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Invariant — exactness (I3). Every value in `next` is a canonical exact
/// rational. Exactness is primarily a type-level guarantee — `Ratio` is
/// `BigRational`, canonical by construction, and no float exists anywhere
/// in the codebase — so this passes for every well-typed state; I3 is the
/// sealed declaration of that law plus defense-in-depth over the value
/// ledgers (the banked resources and elements most likely to carry a
/// decoded value). It calls the sealed `ratio::is_canonical` (Kernel
/// Charter §0b: the equation is shared, never reimplemented in the gate).
fn exactness(next: &Committed) -> Result<(), Refused> {
    for orb in &next.orbs {
        let banked = orb.banked.iter().map(|(_, amount)| amount);
        let elements = orb.elements.iter().map(|(_, amount)| amount);
        for value in banked.chain(elements) {
            if !cfe::ratio::is_canonical(value) {
                return Err(Refused::NotCanonical);
            }
        }
    }
    Ok(())
}

/// Invariant — containment (I2). An actor-deed's actor must be an entity
/// in `prior` that holds the scope the deed requires. Identity precedes
/// the world: a deed from no one refuses before any state is read. A
/// sovereign holds every scope by construction; an agent holds exactly
/// its granted scopes. World-driven events (consequences the tick
/// computes, no actor) carry no scope requirement (`scope_of` = None).
fn containment(prior: &Committed, step: &Step) -> Result<(), Refused> {
    let Some(scope) = scope_of(&step.event) else {
        return Ok(());
    };
    // The founding bootstrap (found live, 2026-08-12 judge tally —
    // one refusal per world ever founded): a chronicle opens with the
    // sovereign registering itself, and no registrar can precede the
    // first identity. The one deed whose actor may be unregistered is
    // the registration of the first entity by its own hand; every
    // later registration needs a registered registrar with the scope.
    if let ClosureEvent::EntityRegistered { entity_id, .. } = &step.event {
        if prior.entities.is_empty() && *entity_id == step.actor {
            return Ok(());
        }
    }
    let entity = prior
        .entities
        .iter()
        .find(|held| held.entity_id == step.actor)
        .ok_or(Refused::NoSuchEntity { actor: step.actor })?;
    match entity.permits(scope) {
        true => Ok(()),
        false => Err(Refused::NotPermitted { actor: step.actor, scope }),
    }
}

/// **The layer of A deed — how far a rule reaches** (`17_CAPACITY` §3d).
///
/// Not a documentation convention. In a distributed economy every node
/// validates independently, so a reach recorded only in prose is a
/// reach no node can enforce — the honour system does not survive
/// consensus. This enum is the layer, and [`layer_of`] is the one
/// authority for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    /// Binds every world and every line, always. Nobody may opt out and
    /// nobody may be advantaged by opting in.
    Universal,
    /// Binds one committed state — its colonies, zones, siting, its own
    /// chronicle. Another world may run differently and both are
    /// lawful.
    World,
    /// Binds a LINE: a people's own instrument, in their own substrate.
    /// Real and lawful, and never a precondition for anyone else's
    /// standing.
    Civilizational,
}

/// **The layer law — exhaustive by design**, in the shape of
/// [`scope_of`]: adding a `ClosureEvent` variant forces a layer
/// decision here at compile time. There is no wildcard arm and there
/// must never be one, because a wildcard is exactly the inference this
/// function exists to abolish.
///
/// **`None` means not yet ruled — it does not mean "no layer".** A
/// false assignment is worse than an absent one, so a genuinely
/// contested deed answers `None` and a gate consulting this must refuse
/// on it rather than assume a reach — the discipline `scope_of` uses
/// for a deed with no actor scope and `covered()` for an unjudged
/// event.
///
/// **No deed answers `None` today** (2026-08-02): the three that were
/// contested were all settled by reading their projections. The
/// `Option` remains in the signature deliberately — the next deed whose
/// reach is genuinely undecided needs an honest home that does not
/// require changing this signature under pressure, which is exactly
/// when a default would get invented.
///
/// The rows are keyed on what the deed carries, never on the document
/// it is described in — a carrier is not a mechanism (`17` §3d.1).
pub fn layer_of(event: &ClosureEvent) -> Option<Layer> {
    use ClosureEvent as E;
    match event {
        // ── civilizational — keyed on a LINE's own identity ──────────
        E::GenesisSealed { .. }        // civilization + the three axes
        | E::LineMinted { .. }         // a people enters — line-keyed by nature
        | E::TaxonSealed { .. }        // class, clade
        | E::RegionCharted { .. }      // GALACTIC-1: the regional seed, identity not ledger
        | E::CompositionSealed { .. }  // era 1: the planet composition, identity not ledger
        | E::UniverseCited { .. }      // docs/58: the world names its universe's origin
        | E::ChronicleCheckpointed { .. } // CHRONICLE-COMPACT-1: a bookkeeping fact, identity not ledger
        | E::EvolutionAdvanced { .. }  // the arc advances — identity, not ledger
        // the vocabulary mint is civilizational, and the row is keyed on
        // what the deed carries rather than on what it enables. It is
        // authored by a LINE holding a progenitor seat and it names a
        // class of entity — identity vocabulary, not a place and not a
        // quantity. That a minted entry later changes yields does not
        // move it: the deed carries a definition; the tick carries the
        // consequence.
        | E::VocabularyMinted { .. }
        // NOVELTY-GATE (tag 166): a solver word is a LINE's own
        // instrument — identity vocabulary in the axis mint's class.
        | E::SolverWordMinted { .. }
        // ENTITY-ALIGN-1 (tag 189): a hardness reading rides alongside
        // the mint it accompanies, same layer, same reasoning.
        | E::SolverWordHardness { .. }
        // Consilience (tag 177): a shared knowledge axiom is keyed on the
        // LINE that reached it — its own knowledge, the same layer.
        | E::KnowledgeConverged { .. }
        | E::KnowledgeLicensed { .. }
        // P2 (tag 103): a chartered parameter is a definition the
        // world's rules read, exactly as a minted vocabulary entry is.
        | E::ParameterChartered { .. }
        // BOOK-1 / LIT-1: a line's own accounts and what it reaches
        // for. Both are keyed on the LINE's identity, never on a place
        // — the layer law is declared per deed, never inferred.
        | E::BookMinted { .. } | E::BookPosted { .. } | E::BookArchived { .. }
        | E::WorkAspired { .. } | E::WorkClaimed { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. }
        | E::LineSucceeded { .. } | E::StationSeated { .. }
        | E::Diverged { .. } | E::Confluent { .. }
        | E::ElderWarded { .. } | E::WardLifted { .. }
        | E::VeilDrawn { .. } | E::VeilLifted { .. } => Some(Layer::Civilizational),

        // ── universal — the universe, the mesh, or any actor ─────────
        E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        | E::EraAnnexed { .. } | E::EraFolded { .. } | E::GalaxyObserved { .. }
        | E::UniverseFounded { .. }    // docs/58: the universe's origin
        | E::AncientArose { .. }       // docs/58: the universe's keeper
        | E::CoreDeposited { .. }      // the cores design: a sink at its throat
        | E::CoreEmitted { .. }        // the bounce, the core's side
        | E::CoreWordDeposited { .. }  // the dead's lexicon at its core
        | E::SubstratumOpened { .. }   // docs/69: a tube to a substratum
        | E::CoreReturned { .. }       // the bounce arrives
        | E::HandoffSealed { .. }
        | E::ResourceTransferred { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        | E::DiscoveryMinted { .. } | E::SovereignUnionSealed { .. }
        // Obligation instruments: ordinary finance, open to any line,
        // each feeding a D channel. The forward ruling binds the
        // ceremony, never the remainder (`17` §3d.1a).
        | E::BondSealed { .. } | E::BondSettled { .. }
        | E::ForwardSealed { .. } | E::ForwardReleased { .. } | E::ForwardSlashed { .. }
        | E::OverflowAbsorbed { .. } | E::AbsorptionLapsed { .. }
        | E::CouncilSealed { .. } | E::CouncilClosed { .. }
        | E::OfferPosted { .. } | E::OfferCleared { .. } | E::OfferCountered { .. }
        | E::RouteChartered { .. } | E::RouteLapsed { .. }
        | E::VoidShare { .. } => Some(Layer::Universal),

        // ── WORLD — keyed on an orb inside one committed state ───────
        E::ColonyFounded { .. } | E::ColonyCharted { .. } | E::WorldCharted { .. }
        | E::EstateSurveyed { .. } | E::AppraisalStruck { .. } | E::ParcelConveyed { .. }
        | E::ZoneBuilt { .. } | E::ZoneRazed { .. }
        | E::YieldCollected { .. } | E::DemurrageSettled { .. } | E::DemurrageTouched { .. }
        | E::TitheReceipted { .. }
        | E::UpkeepSettled { .. }
        | E::VentSettled { .. }
        | E::StarCommissioned { .. } | E::StarAged { .. } | E::PlanetSited { .. }
        | E::OverheatStruck { .. } | E::OverheatCleared { .. }
        | E::FamineStruck { .. } | E::FamineRelieved { .. }
        | E::GaugeShifted { .. } | E::GaugeShiftedAt { .. }
        | E::ProspectStruck { .. } | E::EnrichmentStruck { .. } | E::AssayStruck { .. } | E::CrystalDrawn { .. }
        | E::ElementFarmed { .. }
        // #136 (tag 185): the smelt-sweep floors an orb's element MATTER — WORLD-
        // layer beside the farm and assay it migrates, keyed on the place it floors,
        // never a line's identity.
        | E::ElementStocksSwept { .. }
        // #138 (tag 186): the resource smelt-sweep floors an orb's banked resource
        // + STAMP — WORLD-layer, same as its element twin, keyed on the place it
        // floors, never a line's identity.
        | E::ResourceStocksSwept { .. }
        // ELEMENT-CHEMISTRY (#37): a bond is WORLD-layer beside the
        // mint and the assay it consumes — it carries an orb's matter,
        // not a line's identity. The gem's name is content-addressed
        // from the members, so no line is keyed by it.
        | E::GemCompounded { .. }
        // REFINE-1: WORLD-layer beside the assay and the gem — an
        // orb's matter converts, no line is keyed.
        | E::ElementRefined { .. }
        // RCP-2 (tag 181): the emergent doing-layer, the ZoneBuilt
        // successor — a recipe is raised on an orb and yields into that
        // orb's economy, so it binds the orb, not a line's identity (a
        // sister orb of the same line raises its own). It carries almanac
        // matter over the universal roles, exactly as GemCompounded and
        // ElementRefined carry an orb's matter — World.
        | E::RecipeBuilt { .. }
        // ING-2b (tag 182): the staple food settle carries an orb's grown
        // matter (the planet regrew its staple, the population ate) over the
        // element ledger — World, beside ElementFarmed and the assay it
        // draws on; no line is keyed by it.
        | E::RationElementSettled { .. }
        // ONT-2b (tag 183): the staple mine advances an orb's vein book (the
        // population dug its staple by eating) — World, beside the assay it
        // shares its book motion with; no line is keyed by it.
        | E::StapleMined { .. }
        // ONT-3 (tag 184): rejuvenation advances an orb's vein book — World,
        // beside the mine and assay it shares its book motion with.
        | E::VeinRejuvenated { .. }
        | E::ElementApertureSent { .. } | E::ElementApertureReceived { .. }
        | E::ElementRouteChartered { .. }
        | E::StampMinted { .. } | E::StampApertureSent { .. } | E::StampApertureReceived { .. }
        | E::ScrapSalvaged { .. } | E::HullMinted { .. } | E::HullAssigned { .. } | E::HullScrapped { .. } | E::OrderWorked { .. } | E::ElementSalvaged { .. } | E::TraceConsumed { .. }
        | E::NoteIssued { .. } | E::NoteRedeemed { .. } | E::FutureOpened { .. } | E::FutureSettled { .. } | E::LastClearMarked { .. } | E::ShareIssued { .. } | E::ShareTransferred { .. } | E::ShareDividend { .. } | E::PerpOpened { .. } | E::PerpFunded { .. } | E::PerpClosed { .. } | E::HullFinanced { .. } | E::EscortOnDuty { .. } | E::InterdictionStruck { .. } | E::ConvoyRiskMarked { .. }
        | E::MarketSealed { .. } | E::MarketResumed { .. } | E::SpotClearRecorded { .. } | E::LiquidityRaidStruck { .. } | E::ClearPoisonRefused { .. }
        | E::NoteCascadeStruck { .. } | E::CovertTamperStruck { .. } | E::MediumReinforced { .. } | E::TamperDetected { .. }
        | E::PostureReoriented { .. }
        | E::SpyProbed { .. } | E::TechExfiltrated { .. } | E::InstrumentSabotaged { .. }
        | E::RailPromoted { .. }
        | E::StrandExported { .. } | E::AnchorImported { .. } | E::AnchorImportRefused { .. }
        | E::VaultDeposited { .. } | E::VaultWithdrawn { .. }
        | E::AssayConsilience { .. } | E::AssaySlashed { .. }
        | E::IsoUnionStruck { .. } | E::ChemCataloged { .. } | E::DarkPullStruck { .. }
        | E::ScienceSeated { .. } | E::LabCreated { .. } | E::ScienceExploded { .. }
        | E::RogueFactionStruck { .. }
        | E::PressureEbbed { .. }
        // #42: a treaty is WORLD-layer. It is keyed on the pair, and
        // the pair is two worlds — not a line's identity (a pact
        // outlives whoever signed it) and not one place. E3.9's
        // element annex rides the same key.
        | E::TreatySealed { .. } | E::TreatyStruck { .. }
        | E::TreatyElementsAllowed { .. }
        | E::InstrumentForged { .. }
        | E::TechDiscovered { .. } | E::TechLicensed { .. } => Some(Layer::World),

        // ── ruled 2026-08-02 by reading the projection ───────────────
        // FrictionSettled was contested on the guess that it computed
        // from 09's ideology axes. It does not. The chain is:
        //   friction   = friction_rate() * divergence * value
        //   divergence = |reciprocity_reading(from) - reciprocity_reading(to)|
        //   reciprocity_reading(orb)
        //              = returned_across / (returned_across + extracted_across)
        // Two of the orb's own measured behavioural books — the same
        // pair the missing-leg sweep found already two-armed. No creed,
        // no genesis axis, no class or clade appears anywhere in it.
        // `09` calls it "ideological friction"; the projection prices
        // unlike conduct, and any line can be reciprocal or not.
        // Second instance of the §3d.1 trap: a document title is not a
        // scope, exactly as a carrier is not a mechanism.
        E::FrictionSettled { .. } => Some(Layer::Universal),

        // ── ruled 2026-08-02 by reading the projection ───────────────
        // WantStruck/WantRelieved: the whole block is gated on a sealed
        // genesis — "no seal, no entity type, no want" — and each kind
        // on a specific axis: resonance needs perception == 3
        // (harmonic), novelty needs disposition == 2 (empiricist). A
        // line with different axes never experiences the mechanic. That
        // is civilizational by the exact criterion: a people's own, and
        // never a precondition for anyone else's standing.
        E::WantStruck { .. } | E::WantRelieved { .. }
        | E::ElementWantStruck { .. } | E::ElementWantRelieved { .. }
        | E::PopulationSettled { .. }
        | E::PopulationAccrued { .. }
        // Taxation (49 §5): a sovereign treasury is a people's own
        // fiscal store — universal to every founded line, yet never a
        // precondition for anyone else's standing (the Civilizational
        // criterion), exactly like their population.
        | E::TreasurySettled { .. }
        // COG-1 (doc 52 §7): the leisure bank is a line's own cognition
        // budget — universal to a founded line, never another's standing.
        | E::LeisureSettled { .. }
        // Taxation (49 §3): a line's practices lapsing at its own
        // generation turnover — its own knowledge, no one else's.
        | E::PracticesLapsed { .. }
        // 50_WIN_CONDITIONS: whether a line has collapsed is exactly a
        // fact about that CIVILIZATION, not one orb among several — the
        // Civilizational criterion fits it more than any other layer.
        | E::CivilizationCollapsed { .. } => {
            Some(Layer::Civilizational)
        }

        // IncursionStruck looked like it spanned all three layers,
        // carrying era, world and line. It does not, and the puzzle
        // dissolves on a sharper criterion: A layer is what A deed
        // binds, not what it mentions. Replay does nothing with it
        // (`world/replay.rs`: `IncursionStruck { .. } => {}`), no live
        // path consumes it, and it is universe-ledger only — "the
        // archive never edited, the living never raided". The world and
        // line names are the subject of the record, not keys binding
        // those entities, exactly as `EraAnnexed` carries a world and
        // binds none.
        E::IncursionStruck { .. } => Some(Layer::Universal),
    }
}

/// The sealed event → scope law (a CFE transition rule extracted into the
/// kernel). Exhaustive by design: adding a `ClosureEvent` variant forces a
/// scope decision here at compile time — the amendment discipline made
/// structural. `None` = a world-driven consequence with no actor scope, or
/// a mesh/universe-ledger deed governed by its own law (never a hole: an
/// uncovered event refuses `Unjudged` before this is consulted). Each arm
/// is ratified by a red-first witness as its event enters `covered()`
/// against `cfe::world`; today only `ResourceTransferred` (Collect) is
/// covered and witnessed. The rest are the drafted law, unenforced until
/// their coverage lands.
fn scope_of(event: &ClosureEvent) -> Option<Scope> {
    use ClosureEvent as E;
    match event {
        // Build — create or extend the world, or open authority.
        E::ColonyFounded { .. } | E::ZoneBuilt { .. } | E::WorldCharted { .. }
        | E::EstateSurveyed { .. } | E::AppraisalStruck { .. } | E::LineMinted { .. }
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        | E::ColonyCharted { .. } | E::StarCommissioned { .. } | E::PlanetSited { .. }
        | E::BondSealed { .. } | E::ForwardSealed { .. } | E::CouncilSealed { .. }
        | E::GaugeShiftedAt { .. } | E::TechLicensed { .. } | E::GenesisSealed { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. } | E::ProspectStruck { .. } | E::EnrichmentStruck { .. }
        // #42: sealing or ending a pact re-rules what crossing this
        // pair costs — Build, the founding class of act, beside the
        // chartered parameter it most resembles. E3.9's element annex
        // re-rules what crossing this pair carries — the same class.
        | E::TreatySealed { .. } | E::TreatyStruck { .. }
        | E::TreatyElementsAllowed { .. }
        | E::ElderWarded { .. } | E::VeilDrawn { .. } | E::InstrumentForged { .. }
        | E::RouteChartered { .. } | E::ElementRouteChartered { .. }
        | E::OfferPosted { .. } | E::OfferCountered { .. } | E::LineSucceeded { .. }
        // KERNEL-TAXON: the drafted Build-class arm coder-03 named for
        // their #42 — unenforced until their covered() entry + witness.
        | E::TaxonSealed { .. }
        // GALACTIC-1: chart_region_as gates on Scope::Build, the same as
        // seal_taxon_as.
        | E::RegionCharted { .. }
        | E::CompositionSealed { .. }
        // docs/58: a world's citation of its universe, written at its
        // founding beside the other genesis identity deeds.
        | E::UniverseCited { .. }
        // CHRONICLE-COMPACT-1: schema only at this landing (no door yet —
        // CHRONICLE-COMPACT-4 wires the beat-cadence trigger), classed
        // Build like the RegionCharted precedent it mirrors.
        | E::ChronicleCheckpointed { .. }
        | E::EvolutionAdvanced { .. }
        // #32, 20_FOUNDING §3: the tithe fires from seal_genesis_as,
        // the same Build-class act that seals the axes — actor-scoped,
        // unlike DemurrageSettled's passive decay (None, below).
        | E::TitheReceipted { .. }
        | E::StationSeated { .. }
        // Build-class: a mint extends what the world can express. It
        // creates nothing bankable and razes nothing — it opens room in
        // the vocabulary, which is the progenitor's own office
        // (12_UNION ruling 4: *create new spaces*).
        | E::VocabularyMinted { .. }
        // NOVELTY-GATE: a solver-word mint extends what the LINE can
        // reach — the same founding class.
        | E::SolverWordMinted { .. }
        // ENTITY-ALIGN-1 (tag 189): the hardness reading closed alongside
        // a mint extends the same record — the same founding class.
        | E::SolverWordHardness { .. }
        // RCP-2 (tag 181): raising an emergent recipe extends what the
        // world can express — it opens the society's doing-layer over the
        // open almanac and banks nothing at the build itself (the yield
        // accrues per tick, judged where it lands). Build, the
        // SolverWordMinted / ZoneBuilt founding class.
        | E::RecipeBuilt { .. }
        // Consilience: recording a shared knowledge axiom extends what
        // the LINE holds — the same founding/Build class.
        | E::KnowledgeConverged { .. }
        | E::KnowledgeLicensed { .. }
        // P2 (tag 103): chartering a parameter re-rules the board —
        // Build-class, the progenitor's own office.
        | E::ParameterChartered { .. }
        // D5 — the SOVEREIGN union. Build-class: it opens the standing
        // the whole world is raised on. The scope is necessary and not
        // sufficient — `union` additionally requires the root
        // entity, because no delegate may strike the union.
        | E::SovereignUnionSealed { .. }        // BOOK-1 (19_LENS §16): minting declares a standing account and
        // archiving alters one — both extend or amend the world's
        // structure, neither moves banked value. Drafted law, matching
        // `world::books`' own scope checks.
        | E::BookMinted { .. } | E::BookArchived { .. }
        // LIT-1: aspiring and claiming both add standing state and
        // move no banked value. Drafted law, matching world::literature.
        | E::WorkAspired { .. } | E::WorkClaimed { .. } => {

            Some(Scope::Build)
        }
        // Raze — demolish or lift what was raised.
        E::ZoneRazed { .. } | E::WardLifted { .. } | E::VeilLifted { .. } => Some(Scope::Raze),
        // Collect — settle or move banked value.
        // BOOK-1: a posting moves BOOK value — a dual entry, both sides
        // at once. It touches no banked resource, so it settles nothing
        // and conserves trivially; the scope is Collect because it is a
        // value movement within the orb's own accounts.
        E::BookPosted { .. }
        | E::ParcelConveyed { .. }
        | E::YieldCollected { .. } | E::UpkeepSettled { .. } | E::ResourceTransferred { .. }
        | E::OverflowAbsorbed { .. } | E::BondSettled { .. } | E::ForwardReleased { .. }
        | E::FrictionSettled { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        | E::CrystalDrawn { .. } | E::AssayStruck { .. } | E::ElementFarmed { .. } | E::OfferCleared { .. }
        // ING-2b (tag 182): the staple food settle settles banked food at a
        // collect touch — Collect, exactly as the ElementFarmed food cycle
        // and the YieldCollected it mirrors.
        | E::RationElementSettled { .. }
        // ONT-2b (tag 183): eating mines the staple vein at a collect touch —
        // Collect, exactly as the assay that shares its book motion.
        | E::StapleMined { .. }
        // ONT-3 (tag 184): rejuvenation settles at a collect touch — Collect,
        // exactly as the mine and assay that share its book motion.
        | E::VeinRejuvenated { .. }
        // ELEMENT-CHEMISTRY (#37): compounding spends banked element
        // value and banks a gem — Collect, exactly as the assay that
        // filled the ledger it draws on. Ratified by the chemistry
        // witnesses as it enters covered().
        | E::GemCompounded { .. }
        // REFINE-1: spends banked element value and banks a refined
        // form — Collect, exactly as the assay and the gem.
        | E::ElementRefined { .. }
        | E::ElementApertureSent { .. } | E::ElementApertureReceived { .. }
        | E::StampMinted { .. } | E::StampApertureSent { .. } | E::StampApertureReceived { .. }
        | E::ScrapSalvaged { .. } | E::HullMinted { .. } | E::HullAssigned { .. } | E::HullScrapped { .. } | E::OrderWorked { .. } | E::ElementSalvaged { .. } | E::TraceConsumed { .. }
        | E::NoteIssued { .. } | E::NoteRedeemed { .. } | E::FutureOpened { .. } | E::FutureSettled { .. } | E::LastClearMarked { .. } | E::ShareIssued { .. } | E::ShareTransferred { .. } | E::ShareDividend { .. } | E::PerpOpened { .. } | E::PerpFunded { .. } | E::PerpClosed { .. } | E::HullFinanced { .. } | E::EscortOnDuty { .. } | E::InterdictionStruck { .. } | E::ConvoyRiskMarked { .. }
        | E::MarketSealed { .. } | E::MarketResumed { .. } | E::SpotClearRecorded { .. } | E::LiquidityRaidStruck { .. } | E::ClearPoisonRefused { .. }
        | E::NoteCascadeStruck { .. } | E::CovertTamperStruck { .. } | E::MediumReinforced { .. } | E::TamperDetected { .. }
        | E::PostureReoriented { .. }
        | E::SpyProbed { .. } | E::TechExfiltrated { .. } | E::InstrumentSabotaged { .. }
        | E::RailPromoted { .. }
        | E::StrandExported { .. } | E::AnchorImported { .. } | E::AnchorImportRefused { .. }
        | E::VaultDeposited { .. } | E::VaultWithdrawn { .. }
        | E::AssayConsilience { .. } | E::AssaySlashed { .. }
        | E::IsoUnionStruck { .. } | E::DarkPullStruck { .. }
        | E::ChemCataloged { .. }
        | E::ScienceSeated { .. } | E::LabCreated { .. } | E::ScienceExploded { .. }
        | E::RogueFactionStruck { .. }
        | E::PressureEbbed { .. }
        // U1: the discovery mint settles banked value against verified
        // work — Collect, like the yield it banks beside. Ratified by
        // the U1 witnesses as it enters covered().
        | E::DiscoveryMinted { .. } => {
            Some(Scope::Collect)
        }
        // None — world-driven consequences (empty provenance), mesh deeds,
        // and universe-ledger entries: no actor scope. Governed by other
        // invariants (conservation, decay) or their own protocol law.
        E::FamineStruck { .. } | E::FamineRelieved { .. } | E::EraAnnexed { .. }
        | E::GalaxyObserved { .. } | E::UniverseFounded { .. } | E::AncientArose { .. } | E::CoreDeposited { .. } | E::CoreEmitted { .. } | E::CoreReturned { .. } | E::CoreWordDeposited { .. } | E::SubstratumOpened { .. } | E::OverheatStruck { .. } | E::OverheatCleared { .. }
        | E::VentSettled { .. } | E::DemurrageSettled { .. } | E::DemurrageTouched { .. }
        // #136 — the smelt-sweep is a world-ledger migration (a one-time grain floor),
        // no actor scope; governed by conservation, like the demurrage settle beside it.
        | E::ElementStocksSwept { .. }
        // #138 — the resource smelt-sweep, same shape: a one-time grain floor of the
        // banked-resource + STAMP ledgers, no actor scope, governed by conservation.
        | E::ResourceStocksSwept { .. }
        | E::AbsorptionLapsed { .. }
        | E::ForwardSlashed { .. } | E::WantStruck { .. } | E::WantRelieved { .. }
        | E::ElementWantStruck { .. } | E::ElementWantRelieved { .. }
        | E::PopulationSettled { .. }
        | E::PopulationAccrued { .. }
        | E::TreasurySettled { .. }
        | E::LeisureSettled { .. }
        | E::PracticesLapsed { .. }
        | E::CivilizationCollapsed { .. }
        | E::HandoffSealed { .. } | E::GaugeShifted { .. } | E::Diverged { .. }
        | E::Confluent { .. } | E::StarAged { .. } | E::EraFolded { .. }
        | E::CouncilClosed { .. } | E::TechDiscovered { .. } | E::IncursionStruck { .. }
        | E::VoidShare { .. } | E::RouteLapsed { .. } => None,
    }
}

/// **The sealed warrant (K4).** The kernel's answer to `cfe`'s
/// question: what does this payload earn?
///
/// This is the one organ that may read a work payload, and it does the
/// same two things the gate does — `verify_work` then `mint_bound` —
/// so proposer and gate cannot disagree about what a proof is worth
/// (Charter §0b, call-not-copy).
///
/// It closes the live gate without breaking the opaque-payload law:
/// `cfe` still never imports the verifier. It asks; this answers.
pub struct SealedWarrant;

impl cfe::world::MintWarrant for SealedWarrant {
    fn earns(&self, work: &[u8]) -> Option<Ratio> {
        // The carrier law, asked of the same function the gate runs.
        carrier_bound(work).ok()
    }

    /// The escrow check, answered by the gate itself — `admits` is the
    /// same function the door runs, not a copy of its reasoning
    /// (Charter §0b, call-not-copy). So a deed the emitter settles is
    /// one the gate has already accepted on the identical inputs, and
    /// proposer/gate drift on the mint path is not caught late — it is
    /// unreachable.
    fn admits(
        &self,
        prior: &cfe::world::Committed,
        next: &cfe::world::Committed,
        event: &cfe::dop::ClosureEvent,
        actor: u64,
        stamp: u64,
    ) -> bool {
        let step = Step { event: event.clone(), actor, stamp };
        crate::admits(prior, next, &step).is_ok()
    }
}
