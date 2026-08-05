//! THE KERNEL — the one validated update function (CHARTER §11 rule 3).
//!
//! `admits(prior, next, step)` judges whether ONE step lawfully carries
//! the universe from `prior` to `next`. It is:
//!
//! - **pure** — same inputs, same verdict, forever;
//! - **total** — every input returns `Ok` or a typed `Refused`, never
//!   panics, never hangs;
//! - **no I/O** — it touches no file, clock, socket, or thread;
//! - **encoding-blind** — it never produces or reads bytes.
//!   Serialization is the codec's monopoly (the arbor), so the kernel
//!   and the chain can never disagree about bytes: only one component
//!   ever produces them. The kernel judges MEANING; the codec fixes
//!   FORM; the two never overlap.
//!
//! Conservation, containment, and exactness are INVARIANTS inside this
//! function — an unlawful transition is not caught after the fact, it
//! cannot be constructed. This is why CHARTER §11 dissolves every
//! reconciliation instrument: you do not reconcile what cannot
//! diverge.
//!
//! **Versioned like the charter.** [`VERSION`] is bumped only by
//! amendment. Every chain commitment binds the version that judged the
//! step, so old history verifies under the judge it was cut with — no
//! future change reinterprets the past. The kernel never reads the
//! commitment; the arbor binds `VERSION` into it.
//!
//! **Coverage grows by amendment, never silently.** Each event's rules
//! land as a witnessed addition (one witness per invariant, red-first);
//! CHARTER §11 clause 2 guards that the growth is amendment-shaped and
//! that no state semantics ever live outside this function.

pub mod bridge;
pub mod motion;

use cfe::dop::{ClosureEvent, Office, Resource, UNION_REVIEW_HORIZON};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, SOVEREIGN};
use solver_verify::{verify_work, WorkBroken};

/// The kernel version — bumped ONLY by charter amendment. The arbor
/// binds this into every commitment (`H(schema ‖ bytes ‖ parent ‖
/// coordinate)` carries the judging version alongside the schema), so
/// a chain segment is forever verified under the version that cut it.
///
/// VERSION 2 — RULING BOX 1 (operator, 2026-08-01): `mint_bound`
/// unsealed, the emission rate `f` ladder-pinned against the #17
/// calibration (evidence commit 5898351). Version-1 history judged
/// every positive discovery mint refused; it verifies forever under
/// its own law.
///
/// VERSION 3 — THE CAPACITY LAW (operator ruling 2026-08-01), now
/// STRUCK. It refused a `DiscoveryMinted` unless the prior state read
/// `Vacuity::Entangled` — structure AND flow, both standing. Kept in
/// this record because version-3 history is judged by version-3 law
/// forever; the note below says why the law did not survive.
///
/// **The bump was announced and never landed.** The commit that
/// introduced the law was titled *"VERSION 3 now means something"* and
/// left the constant at 2, so the chain would have bound version 2 to
/// steps that version 2 would have judged differently — the one thing
/// this constant exists to prevent. Version-2 history verifies forever
/// under version 2's law.
/// VERSION 5 — TWO AMENDMENTS, LANDED TOGETHER AT THE MERGE.
///
/// **VERSION 4 — THE CAPACITY LAW IS REMOVED** (operator ruling
/// 2026-08-04, harness lane). BREAKING in the LOOSENING direction:
/// every state version 3 refused as `MintWithoutStanding` or
/// `MintWithoutOwnStanding` is now judged on the mint's real anchor
/// alone. *Identity is pegged to entity, and identity gets recycled
/// without bonds* — the capacity law read STANDING off relational
/// structure, so it demanded of a world exactly what a recycled
/// identity does not carry. Measured before the ruling: a 60-beat era
/// produced **374 solves, 0 mints, 124 withheld**. What still anchors
/// the mint is U1's REPLAY-COMPLETE LAW, untouched.
///
/// **VERSION 5 — D4, THE REVOCATION LAW LANDS** (operator ruling
/// 2026-08-04, cognition lane). A `DiscoveryMinted` deed refuses unless
/// it carries a strictly positive capacity increment, over a RESOLVED
/// rank reading. See [`revocation_law`].
///
/// **Both numbers are kept and neither is ambiguous.** They were minted
/// on separate branches for separate reasons; 4 was skipped on main so
/// the two could not collide, and the merge lands both rather than
/// letting one bump overwrite the other. A chain cut under 3 verifies
/// forever under the judge that cut it.
pub const KERNEL_VERSION: Version = Version { major: 0, minor: 5, patch: 0 };

/// The judge's version, as it rides the wire.
///
/// **`x.y.z`, and `major = 0` because this is a prototype** (operator
/// ruling 2026-08-04: *knock version down to x.x.x — we are still
/// prototyping*). The five linear amendments become `0.1.0 … 0.5.0`,
/// so the count is preserved in `minor` and the leading `0` says
/// plainly that nothing here has promised stability yet.
///
/// **What each field means, so the next bump is not a judgement call:**
///
/// ```text
/// major  reserved for the first stable judge. Still 0.
/// minor  a LAW change — the judge admits or refuses something it did
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
    /// The wire form. Packed so ORDER IS PRESERVED — a later version is
    /// a larger `u64`, which is what a peer comparing two commitments
    /// needs and all it needs.
    ///
    /// The wire field stays `u64` and `Commitment.kernel` is untouched,
    /// so this is not a wire amendment. Chains cut under the linear
    /// versions 1–5 carry those small integers and still compare BELOW
    /// every packed version, which is true and is the ordering we want:
    /// they were cut by an earlier judge.
    pub const fn code(self) -> u64 {
        ((self.major as u64) << 32) | ((self.minor as u64) << 16) | self.patch as u64
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The packed judge version bound into every commitment.
pub const VERSION: u64 = KERNEL_VERSION.code();

/// One proposed transition: the deed that CLAIMS to carry `prior` to
/// `next`. `event` is what closed, `actor` is who under NS-6
/// containment, `stamp` is the world-clock value it is stamped with.
/// The kernel judges whether the claim is lawful; it never applies it.
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
    /// promised only to MOVE it. Both totals carried (boxed — the
    /// carrier is a big-int rational); the residual is theirs to read
    /// (shown, never summarized away).
    ConservationBroken { resource: Resource, before: Box<Ratio>, after: Box<Ratio> },
    /// **D4** — a mint whose capacity increment is zero or negative:
    /// the claim on capacity never entangled with potential, so it is
    /// VACATED (EQUATIONS D4). Both readings carried whole — the
    /// circulation standing above the ground before and after.
    ///
    /// `D` is NOT lowered by this refusal. The claim vacates; the
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
    /// that actually DESTROYS value (a burn wearing a mint's name). Value
    /// enters only by a lawful, non-negative mint.
    MintNotPositive { resource: Resource, amount: Box<Ratio> },
    /// I5 — a decay carried a negative amount: a deed labelled a decay
    /// that actually CREATES value. Value leaves only by a lawful,
    /// non-negative decay.
    DecayNotPositive { resource: Resource, amount: Box<Ratio> },
    /// THE ELEMENT LAW (operator ruling 2026-08-01): the element
    /// ledger is judged with strict symmetry to the banked ledger. An
    /// element mint carried a negative amount — value enters the
    /// element ledger only by a lawful, non-negative assay.
    ElementNotPositive { element: [u8; 32], amount: Box<Ratio> },
    /// THE ELEMENT LAW: an assay drew more than the vein's declared
    /// yield — `amount ≤ richness · elapsed`, both read from the PRIOR
    /// vein (`Prospected.richness`, `last_assay`); an assay with no
    /// prior vein yields nothing. Both sides carried whole.
    AssayExceedsVein { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// THE ELEMENT LAW: an element total changed where the deed
    /// declared no change — the ledger's total for one element must
    /// equal the prior total plus exactly what the deed declares
    /// (only an assay declares a mint; every other deed holds every
    /// element key still). The element named, both totals whole.
    ElementBroken { element: [u8; 32], expected: Box<Ratio>, actual: Box<Ratio> },
    /// THE GAUGE LAW (U2d, I6's kernel half): the state's key moved
    /// where the deed declared no move, or landed off its declared
    /// key, or a plane opened or closed undeclared. The first
    /// differing grade named, both sides carried whole (an absent
    /// grade reads 0 — ground rest). Every fold transition emits
    /// exactly its declared structural delta; no state jump is
    /// special-cased (operator ruling).
    GaugeBroken { grade: usize, expected: Box<Ratio>, actual: Box<Ratio> },
    /// THE SIGN LAW (the value-law wave): a value-carrying deed
    /// declared a NEGATIVE amount — a credit wearing a debit's name,
    /// or the reverse. Every banked movement is non-negative by
    /// declaration; DIRECTION comes from the deed, never the sign
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
    /// The bound is the RULED emission rate `f` (Ruling Box 1,
    /// 2026-08-01 — see [`mint_bound`]): ladder-pinned, zero on the
    /// saturated rungs and on every off-ladder shape.
    MintExceedsWork { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// I3 — a value in `next` is not a canonical exact rational (coprime,
    /// positive denominator). Exactness is a TYPE-LEVEL guarantee
    /// (`BigRational` is canonical by construction, no float ever entered);
    /// I3 is its sealed declaration and defense-in-depth at the byte
    /// boundary. The kernel CALLS the sealed `ratio::is_canonical`, never
    /// defining canonicity itself (the equations are shared, never copied
    /// — KERNEL CHARTER §0b).
    NotCanonical,
    /// THE SEALED DEFAULT — the judge refuses what no invariant covers.
    /// This event has no landed rule; a deed the kernel cannot judge does
    /// NOT enter. Coverage grows by amendment: each landing moves an event
    /// from this default-refuse arm to an explicit judged arm, and the
    /// kernel wires as the live gate (Phase 3) only when `Unjudged` is
    /// unreachable — every event covered. **SEAL A is a READER's law,
    /// never the judge's:** readers skip unknown TAGS so old software
    /// survives new content; a judge that skips unknown DEEDS is a hole
    /// exactly the size of every future tag. Two laws, one word "unknown"
    /// — conflating them silently rebuilds default-admit.
    Unjudged,

    /// D5 — THE SOVEREIGN UNION was struck by someone other than the
    /// ROOT ENTITY. The union is the standing the whole world is
    /// raised on, and no delegate may enter the offices into it: a
    /// granted scope is authority to act WITHIN the world, never
    /// authority to constitute it.
    ///
    /// **Approval is WHO ACTS, never a field.** An earlier design
    /// carried `approved_by_operator: bool` on the deed. A deed is a
    /// CLAIM, and a claim asserting its own approval is forgeable by
    /// whoever builds it — the kernel would have had nothing to check
    /// the bool against. I2 containment already binds the actor
    /// unforgeably, so identity IS the signature.
    UnionNotStruckByRoot { actor: u64 },
    /// D5 — the union named the wrong offices. It is `Progenitor` with
    /// `Sovereign`, exactly once each: two progenitors or two
    /// sovereigns is not a DYAD, and a dyad is the whole form (`17`
    /// §3 — two that STAY TWO, neither deriving legitimacy from the
    /// other).
    UnionMalformed { progenitor: Office, sovereign: Office },
    /// D5 — the union declared a review horizon other than the ruled
    /// one. The term is a REVIEW, not an expiry, and it is fixed at
    /// [`cfe::dop::UNION_REVIEW_HORIZON`]: a deed free to name its own
    /// horizon could set it past any reachable stamp and never come up
    /// for approval at all.
    UnionTermNotRuled { declared: u64, ruled: u64 },
}

/// THE JUDGE. Does `step` lawfully carry `prior` to `next`? Pure,
/// total, encoding-blind, AND network-blind (it reads no mesh, no peer,
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
    // D4 AFTER `creation`, deliberately. `creation` judges the DEED —
    // its proof and its bound; D4 judges the deed's EFFECT on the
    // world. Ordering effect after validity means a malformed payload
    // is named `WorkRefused`, not misreported as a capacity failure,
    // and the work law stays reachable by every fixture that tests it.
    // The two are entangled (`Δ(D) ⋈ ∂Δ`), not ranked — the law fixes
    // that both must hold, never which speaks first, so this is a
    // diagnostics choice and is recorded as one.
    revocation_law(prior, next, step)?;
    decay(prior, step)?;
    gauge_move(prior, next, step)?;
    element_move(prior, next, step)
}

/// **INVARIANT — D4, THE REVOCATION LAW. The eclipse anchor, floored
/// at `H¹`.** (EQUATIONS D4, INSCRIBED; RULED 2026-08-01; LANDED
/// 2026-08-04.)
///
/// D4 is **not** a scalar decay and **not** a contraction of `D`. It is
/// the FORMAL ENTANGLEMENT COLLAPSE OPERATOR: the site where a claim on
/// capacity that never entangled with potential is VACATED from the
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
/// **THE CLAIM VACATES; THE DEGREE DOES NOT.** That is why D4 is
/// expressible as a kernel invariant at all: `lawful` REFUSES, it never
/// mutates. Refusing the deed IS the vacating of its claim, and no code
/// path here can lower a `Degree` — which would violate Article IV and
/// this row's own floor.
///
/// **THE LAYER SPLIT IS LOAD-BEARING — a WINDOW is not a STEP.** D4
/// fires in two places and they must never merge. This is the PER-STEP
/// half: *a deed that mints carries a non-zero capacity increment, or
/// refuses* (`18` §2f consequence 2 — no hidden work). The OVER-A-
/// WINDOW half (`deeds(w) > 0 ∧ ΔD(w) = 0`) is a LENS reading and
/// gates nothing; it lives in `lens::revocation` and must never be
/// hoisted into the judge. A rate never attains its bound at any single
/// step, so a window can never be an invariant.
///
/// **THE PHANTOM-RANK GATE, now due.** `EntangledRead::rank_is_defined`
/// has existed unused, its own header warning that an unresolved
/// absence *"would let phantom topology mint phantom rank"*, and
/// `25_STANDING` §6 recorded it as DEBT rather than enforcement. D4
/// gates a mint on a rank-derived increment, which makes that debt due:
/// without this check the increment is computed over an incomplete
/// reading and phantom topology satisfies the test. It is asked FIRST,
/// before any increment is compared.
///
/// The increment is read on the CIRCULATION — `Degree::above`, the
/// gauge-invariant part that grows — never on `Degree::get`, which
/// includes the ground and would make the comparison vacuous exactly
/// the way the struck `D > 0` gate was (a floor cannot decide).
fn revocation_law(
    prior: &Committed,
    next: &Committed,
    step: &Step,
) -> Result<(), Refused> {
    let ClosureEvent::DiscoveryMinted { .. } = &step.event else {
        // D4 binds the MINT. Motion of value that already exists makes
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

/// **D4a — THE RESOLUTION GATE.** An unresolved absence is not a zero.
///
/// `rank_is_defined` stood unused since it was written, its own header
/// warning that an unresolved absence *"would let phantom topology mint
/// phantom rank"*, with `25_STANDING` §6 recording it as DEBT rather
/// than enforcement. D4 computes an increment OVER that rank, so the
/// debt came due: asked FIRST, before anything is compared.
fn d4a_resolution(before: &bridge::EntangledRead, after: &bridge::EntangledRead) -> Result<(), Refused> {
    match before.rank_is_defined() && after.rank_is_defined() {
        true => Ok(()),
        false => Err(Refused::RankUndefined {
            unknown_before: before.unknown,
            unknown_after: after.unknown,
        }),
    }
}

/// What D4b found: whether there is a capacity claim here to judge.
enum Claim {
    /// Nothing circulates. No claim was made, so none can be vacated.
    None,
    /// A claim stands, carrying the circulation it stands on.
    Standing(usize),
}

/// **D4b — IS THERE A CLAIM TO VACATE AT ALL?**
///
/// D4 vacates *a claim that did not survive `Π_G`*, and where NOTHING
/// circulates no capacity claim was ever made. A line holding no
/// relations is not claiming capacity and failing; it is claiming none.
///
/// **This gate was forced by measurement.** D4 first landed binding
/// every `DiscoveryMinted`, and a 60-beat era refused 48 of 55 mints,
/// each reading `was: 0, now: 0`. That is the SAME defect the capacity
/// law was struck for the same day — *identity is pegged to entity, and
/// identity gets recycled without bonds* — reached by another route: a
/// mint banks value and adds no relation, so `ΔD` is zero by
/// construction on any world holding nothing.
///
/// **THE GAP THIS EXPOSES, recorded rather than papered over.**
/// EQUATIONS D3 asserts *capacity growth and the discovery mint are ONE
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

/// **D4c — THE INCREMENT.** A standing claim grows, or it vacates.
///
/// Read on the CIRCULATION — what stands above the floor — never on the
/// full `Degree`, which includes the ground and would make the
/// comparison vacuous exactly the way the struck `D > 0` gate was: a
/// floor decides nothing.
///
/// The refusal IS the vacating. Nothing here writes, and `Degree` has
/// no below-ground representation to write into even if it did —
/// Article IV, the `H¹` floor. The claim vacates; the degree does not.
fn d4c_increment(was: usize, now: usize) -> Result<(), Refused> {
    match now > was {
        true => Ok(()),
        false => Err(Refused::MintWithoutCapacityIncrement { was, now }),
    }
}
/// INVARIANT — D5, THE SOVEREIGN UNION (operator ruling 2026-08-02).
///
/// The union of the offices `D(Ω)` PROGENITOR and `V(Ω)` SOVEREIGN is
/// a CONSTITUTIONAL deed: it moves no value, opens no plane and shifts
/// no key. What it must do is be impossible to strike falsely, and
/// three things make it so.
///
/// **THE ROOT ENTITY, and only it.** A granted scope is authority to
/// act WITHIN the world; it is never authority to constitute the world.
/// `scope_of` puts this deed in Build so the ordinary containment
/// check applies, and that is NECESSARY BUT NOT SUFFICIENT — this arm
/// additionally requires `actor == SOVEREIGN`, the root entity id.
/// Approval is WHO ACTS: a bool on the deed would be set by whoever
/// built it.
///
/// **A DYAD, exactly.** One `Progenitor` and one `Sovereign`. Two of
/// either is not a union of two offices, and `17` §3 makes the pair
/// the whole form — two that STAY TWO, neither deriving its legitimacy
/// from the other.
///
/// **THE RULED HORIZON.** A deed free to name its own term could set
/// one past any reachable stamp and never come up for approval, which
/// would make the review a formality. The term is fixed.
///
/// Pure and total like the rest: reads the step only, no clock, no
/// history — the review FALLS DUE by arithmetic on the receipt's own
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

/// INVARIANT — element conservation (THE ELEMENT LAW, operator ruling
/// 2026-08-01). The element ledger is judged with strict symmetry to
/// the banked ledger: an assay mints `richness · elapsed` into a
/// vein's element — non-negative by declaration, and bounded by the
/// PRIOR vein's `richness` and `last_assay` (both already committed
/// state, so the bound is deterministic from `(prior, step)`). Every
/// other deed declares no element move, so every element total holds
/// still (a smuggled element mint under any deed refuses).
fn element_move(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    // The declared element mint for this deed (None = holds still).
    let declared: Option<([u8; 32], Ratio)> = match &step.event {
        ClosureEvent::AssayStruck { orb_id, element, amount } => {
            if *amount < ratio(0, 1) {
                return Err(Refused::ElementNotPositive {
                    element: *element,
                    amount: Box::new(amount.clone()),
                });
            }
            // The vein's yield, read from PRIOR state: richness ×
            // elapsed since its last assay; no vein yields nothing.
            let bound = prior
                .prospects
                .iter()
                .find(|vein| vein.orb == *orb_id && vein.element == *element)
                .map(|vein| match step.stamp > vein.last_assay {
                    true => vein.richness.clone() * ratio((step.stamp - vein.last_assay) as i64, 1),
                    false => ratio(0, 1),
                })
                .unwrap_or_else(|| ratio(0, 1));
            if *amount > bound {
                return Err(Refused::AssayExceedsVein {
                    bound: Box::new(bound),
                    claimed: Box::new(amount.clone()),
                });
            }
            Some((*element, amount.clone()))
        }
        _ => None,
    };
    // Total-check every element key present in either state or in the
    // declaration: the total moves by exactly the declared amount.
    let mut keys: Vec<[u8; 32]> = Vec::new();
    if let Some((element, _)) = &declared {
        keys.push(*element);
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
            .as_ref()
            .filter(|(held, _)| *held == element)
            .map(|(_, amount)| amount.clone())
            .unwrap_or_else(|| ratio(0, 1));
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

/// THE GAUGE LAW (U2d — I6's kernel half, the Delta-modeling ruling):
/// every transition's effect on the gauge vector is EXACTLY what the
/// deed declares — the fold trio moves the key it names, the graded
/// shift opens exactly its plane, and every other deed holds the
/// vector still. Declaration == effect, the banked law's shape
/// applied to the key; no state jump is special-cased. The PAIR half
/// of I6 — `orbit_lock` between the two MERGING chains — is the merge
/// site's law (converge/mesh admission calling the sealed
/// `rfa_core::f_fold_star`): the kernel enforces the LOCAL half, the
/// seam enforces the pair (the aperture-pair pattern, CHARTER §5a).
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
        // Every other deed declares NO key move: the vector holds
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

/// The landed coverage of the invariant set. An event is judged (not
/// refused `Unjudged`) only when EVERY invariant that should govern it has
/// landed — the completeness rule, so partial coverage never admits. The
/// universal trio I1/I2/I3 is now landed, so events whose full set is
/// {containment, exactness} — the pure authority deeds, moving no value —
/// are fully judged. Value-moving deeds beyond the transfer wait on I1's
/// coverage extension; minting/decaying deeds wait on I4/I5. The set grows
/// only by witnessed amendment (§11 clause 2); the kernel wires as the live
/// gate (Phase 3) when this predicate is total (every event covered).
fn covered(event: &ClosureEvent) -> bool {
    use ClosureEvent as E;
    matches!(
        event,
        // I1 conservation covers the intra-world move AND the aperture
        // legs (the convergence seam): a leg debits/credits exactly its
        // declared amount, the local half of pair conservation.
        E::ResourceTransferred { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        // I4 creation / I5 decay (declaration==effect + sign) fully cover
        // the mint and the demurrage decay.
        | E::YieldCollected { .. } | E::DemurrageSettled { .. }
        // U1 — the discovery mint's FULL set is landed: I1
        // (declaration==effect), I2 (Collect), I3, and I4 whole (sign
        // + the replay-complete verification + the sealed-shut bound).
        | E::DiscoveryMinted { .. }
        // I2 containment + I3 exactness fully cover the authority deeds
        // (no value moves, no ratios): registering an agent and opening
        // or shutting its scopes.
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        // D5 — THE SOVEREIGN UNION. A CONSTITUTIONAL deed: no value
        // moves, no ratio is carried, no plane opens. I2 containment
        // covers it and `union` adds the three checks the union
        // needs — the ROOT entity struck it, the parties are a dyad of
        // two different offices, and the review horizon is the ruled
        // one. Ratified red-first by `sovereign_union_pins`.
        | E::SovereignUnionSealed { .. }
        // BOOK-1 (19_LENS §16): minting DECLARES a standing account and
        // archiving ALTERS one — both extend or amend the world's
        // structure, neither moves banked value. Matches
        // `world::books`' own scope checks.
        | E::BookMinted { .. } | E::BookArchived { .. }
        // LIT-1: aspiring and claiming both add standing state and move
        // no banked value. Matches `world::literature`.
        | E::WorkAspired { .. } | E::WorkClaimed { .. }
        // W-A slice-1 — the BANKED-NEUTRAL authority deeds. Each one's
        // `world.rs::replay` arm touches NO bank (it sets a flag, a
        // seal, a list: terrain seed, star, planet site, practice,
        // ward, veil, genesis, prospect, the succession boundary, a
        // licence record, an instrument, a route, an offer, a bond).
        // So {I2 containment, I3 exactness} complete them, and
        // conservation's non-constraint is CORRECT — ratified against
        // the replay reading in `authority_pins.rs`. The value-moving
        // "authority" deeds (ColonyFounded/Charted, ZoneBuilt/Razed,
        // Forward/Council seals — all move banks) stay held out for
        // W-A slice-2's I1 arms. TaxonSealed stays coder-03's #42.
        | E::WorldCharted { .. } | E::StarCommissioned { .. } | E::PlanetSited { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. } | E::ElderWarded { .. }
        | E::VeilDrawn { .. } | E::WardLifted { .. } | E::VeilLifted { .. }
        | E::GenesisSealed { .. } | E::ProspectStruck { .. } | E::LineSucceeded { .. }
        | E::TechLicensed { .. } | E::InstrumentForged { .. } | E::RouteChartered { .. }
        | E::OfferPosted { .. } | E::BondSealed { .. }
        // THE MISSED SIXTH (arch-02's coverage audit): the enum holds
        // 64 deeds, not the documented 63 — GaugeShiftedAt (tag 60,
        // the graded key shift, LIVE at world.rs's shift_gauge_at) was
        // never counted. Banked-neutral (replay resizes the gauge key,
        // no bank), Build-scoped beside its covered counterpart
        // GaugeShifted. Without it the Phase 3 flip could never
        // complete: a live deed refusing Unjudged forever.
        | E::GaugeShiftedAt { .. }
        // U2d — THE FOLD TRIO enters under THE GAUGE LAW (I6's kernel
        // half): the fold's only lawful local effect is its declared
        // key move, banks and every undeclared grade held still. The
        // pair half — orbit_lock between the merging chains via the
        // sealed rfa_core::f_fold_star — is the merge site's law
        // (converge/mesh admission), the aperture-pair seam pattern.
        // Witnessed in gauge_pins.rs.
        | E::Confluent { .. } | E::EraAnnexed { .. } | E::EraFolded { .. }
        // W-C / W-D slice — the BANKED-NEUTRAL world-driven and mesh
        // deeds. Their `world.rs::replay` arms set a flag, a list, or
        // are no-ops (no bank); `scope_of` is None (a tick's own
        // consequence carries no actor scope). Judged by I3 exactness,
        // conservation and containment vacuous — ratified against the
        // replay reading in `world_driven_pins.rs`. HELD OUT: the
        // value-movers of these families (FamineStruck −drawn,
        // ForwardSlashed +collateral, CouncilClosed +banked → the I1
        // value slice) and the FOLD TRIO (Confluent/EraAnnexed/
        // EraFolded → U2d's I6 gate; a fold is never admitted on
        // exactness alone).
        | E::FamineRelieved { .. } | E::GalaxyObserved { .. } | E::OverheatStruck { .. }
        | E::OverheatCleared { .. } | E::VentSettled { .. } | E::AbsorptionLapsed { .. }
        | E::WantStruck { .. } | E::WantRelieved { .. } | E::HandoffSealed { .. }
        | E::GaugeShifted { .. } | E::Diverged { .. } | E::StarAged { .. }
        | E::TechDiscovered { .. } | E::IncursionStruck { .. } | E::VoidShare { .. }
        | E::RouteLapsed { .. }
        // THE VALUE LAW WAVE (operator directive 2026-07-31): the
        // value-moving deeds under TOTAL conservation + THE SIGN LAW,
        // deltas declared per-resource in `lawful_deltas` (the sealed
        // rules called, stateful reads from prior). Witnessed in
        // `value_pins.rs`. Tier 1 (single-resource, sign-guarded),
        // Tier 2 (escrow moves under THE ESCROW LAW — conservation reads
        // banked + escrow, so a forward lock/return nets ZERO on the
        // conserved total; council CLOSE held out on Phase 3 threading),
        // Tier 3 (multi-resource vectors).
        | E::UpkeepSettled { .. } | E::CrystalDrawn { .. } | E::OverflowAbsorbed { .. }
        | E::BondSettled { .. } | E::FamineStruck { .. } | E::ForwardSealed { .. }
        | E::CouncilSealed { .. } | E::FrictionSettled { .. } | E::OfferCleared { .. }
        | E::ForwardReleased { .. } | E::ForwardSlashed { .. } | E::CouncilClosed { .. }
        | E::ColonyFounded { .. } | E::ColonyCharted { .. } | E::ZoneBuilt { .. }
        | E::ZoneRazed { .. }
        // THE ELEMENT LAW (operator ruling 2026-08-01): the assay mints
        // into the element ledger, judged with strict symmetry to the
        // banked ledger — sign-guarded, bounded by the prior vein's
        // richness × elapsed, and zero-invariant for every other deed.
        // Witnessed in `element_pins.rs`.
        | E::AssayStruck { .. }
        // THE LAST DEED (operator ruling 2026-08-01, folded to arch-01):
        // TaxonSealed — class and clade sealed as identity, once per
        // star (15 §4). {I2 containment (Build), I3 exactness} complete
        // it; it moves no bank, no element, no key (all three
        // conservation laws hold vacuously). With this, `covered()` is
        // TOTAL — every one of the 64 deeds is judged; `Unjudged` is
        // unreachable via any current deed (the Phase 3 precondition,
        // witnessed in `taxon_pins.rs`). The sealed default stays as
        // defense for FUTURE enum additions (refuse-unknown).
        | E::TaxonSealed { .. }
        // THE VERTICAL AXIS (tag 89): the seat is banked-neutral — it
        // moves no value, no element and no key — so {I2 containment,
        // I3 exactness} complete it. Containment is the whole point:
        // a station GATES AUTHORITY, so WHO may seat one is the law
        // that was missing while it lived in an editable file.
        | E::StationSeated { .. }
    )
}

/// INVARIANT — conservation, TOTAL (the value-law wave, operator
/// directive 2026-07-31; the escrow strengthening, dispatch C7
/// 2026-08-01). A deed changes the world's CONSERVED total — banked
/// PLUS escrowed — by exactly its DECLARED per-resource deltas and
/// nothing else:
///
/// ```text
/// ∀ r ∈ Resources:  conserved(next, r) == conserved(prior, r) + declared(r)
///   where conserved(s, r) = banked(s, r) + escrow(s, r)
/// ```
///
/// The escrow half (`total_escrow`) closes the lock seam: a forward
/// seal debits a bank and MUST credit escrow the same collateral (net
/// declared ZERO) — a debit that escrows nothing no longer passes.
/// Council CLOSE is held out (its atomic multi-event settle is per-deed
/// threading, gated on Phase 3). Every resource present in either state,
/// in a lock, or in the declaration is checked — a covered deed with an
/// EMPTY declaration must leave the conserved total untouched (a
/// smuggled change under a neutral deed refuses).
/// Multi-resource deeds (founding reserves, zone costs and refunds)
/// declare vectors, never a scalar. The PAIR-level law (a far leg is
/// the near leg's exact inverse) is the cross-node opposition (KERNEL
/// CHARTER §5a); the kernel enforces the LOCAL half.
fn conservation(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Refused> {
    let deltas = lawful_deltas(prior, &step.event)?;
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
    // present ONLY in a lock (no bank holds it) must still be checked —
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

/// The declared per-resource banked deltas of a deed — the LAWFUL
/// movement, sign-guarded at the door (THE SIGN LAW: every amount is
/// non-negative by declaration; direction comes from the DEED). The
/// sealed rules are CALLED, never copied (founding reserves, zone
/// costs/refunds — CHARTER §0b). Stateful declarations (the council's
/// resource, the razed zone's kind) read PRIOR state, never the event.
/// An event with no arm declares nothing: every bank must hold still.
fn lawful_deltas(
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
        // ColonyCharted: the founding reserves MOVE source → charted —
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
        // — mints (the sign lives at I4, order preserved for the
        //   landed pins) —
        E::YieldCollected { resource, amount, .. } => vec![(*resource, amount.clone())],
        E::DiscoveryMinted { resource, amount, .. } => vec![(*resource, amount.clone())],
        // — decays (the sign lives at I5, order preserved) —
        E::DemurrageSettled { resource, amount, .. } => vec![(*resource, -amount.clone())],
        // — Tier 1: single-resource debits, sign-guarded —
        E::UpkeepSettled { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, -amount.clone())]
        }
        E::CrystalDrawn { amount, .. } => {
            non_negative(Resource::Crystal, amount)?;
            vec![(Resource::Crystal, -amount.clone())]
        }
        E::OverflowAbsorbed { amount, .. } => {
            non_negative(Resource::Sink, amount)?;
            vec![(Resource::Sink, -amount.clone())]
        }
        E::FamineStruck { resource, drawn, .. } => {
            non_negative(*resource, drawn)?;
            vec![(*resource, -drawn.clone())]
        }
        // The council locks collateral × EVERY member OUT of the banks
        // and INTO escrow (the sealed council enters `next`) — a move
        // WITHIN the conserved total, so the declared NET is ZERO. The
        // sign guard stays at the door. (THE ESCROW LAW.)
        E::CouncilSealed { resource, collateral, .. } => {
            non_negative(*resource, collateral)?;
            Vec::new()
        }
        // — Tier 2: escrow moves (THE ESCROW LAW, strengthened
        //   2026-08-01) —
        // A forward lock/return moves collateral BETWEEN escrow and a
        // bank: the whole forward enters `next` on seal (escrow rises by
        // the collateral) and leaves on release/slash (escrow falls). Now
        // that conservation reads banked + escrow (`total_conserved`),
        // the declared NET is ZERO — the bank move is FORCED to equal the
        // escrow move, or it refuses. A seal that debits a bank without
        // escrowing the collateral no longer passes. Sign guard at the
        // door. (Council CLOSE stays declaration==effect on banked — its
        // multi-event atomic settle is per-deed threading, gated on the
        // Phase 3 flip; see the CouncilClosed arm.)
        E::ForwardSealed { resource, collateral, .. }
        | E::ForwardReleased { resource, collateral, .. }
        | E::ForwardSlashed { resource, collateral, .. } => {
            non_negative(*resource, collateral)?;
            Vec::new()
        }
        // The close is STATEFUL: the resource reads from the PRIOR
        // sealed council, never the event. An unknown council declares
        // nothing — a banked change under it refuses.
        // HELD OUT (the escrow seam): a council reckons at its boundary
        // as ONE atomic settle — the whole council removed, every member
        // banked, N `CouncilClosed` events emitted for ONE state
        // transition. Per-deed escrow discharge (escrow −collateral as
        // each member's seat closes) needs per-deed threading, which the
        // operator gated on the Phase 3 flip (arch-02's conductor lane).
        // Until then the close declares its banked credit and the
        // council is held constant across the deed's prior→next; the
        // escrow discharge is conserved at the settle boundary, not
        // per-event. Forwards (1 deed ↔ 1 escrow move) ARE strengthened.
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
        // The chartered genesis mint: EXACTLY the founding reserves.
        E::ColonyFounded { .. } => cfe::dop::founding_reserves(),
        E::ZoneBuilt { kind, .. } => {
            kind.cost().into_iter().map(|(resource, cost)| (resource, -cost)).collect()
        }
        // The raze refunds the PRIOR zone's kind at the tile (stateful
        // — the event names only the tile). No zone, no refund.
        E::ZoneRazed { orb_id, tile } => prior
            .orbs
            .iter()
            .find(|held| held.orb_id == *orb_id)
            .and_then(|orb| orb.zones.iter().find(|zone| zone.tile == *tile))
            .map(|zone| zone.kind.raze_refund())
            .unwrap_or_default(),
        // No arm: the deed declares NO banked movement — the total
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

/// THE ESCROW LAW (operator dispatch C7, 2026-08-01). The world's total
/// ESCROWED holding of one resource — collateral that sits OUT of every
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

/// The CONSERVED quantity — banked PLUS escrowed. A lock moves value
/// between the two ledgers, never in or out of the world; conservation
/// reads this total, so a bank debit under a lock must be matched by an
/// escrow credit (or the reverse), exactly, or the deed refuses.
fn total_conserved(state: &Committed, resource: Resource) -> Ratio {
    total_banked(state, resource) + total_escrow(state, resource)
}

/// INVARIANT — bounded creation (I4). Value is created ONLY by a lawful
/// mint, and the mint is non-negative (a mint that destroys value is a
/// burn mislabelled — refused). The declaration==effect half is
/// conservation (the world total rose by exactly the declared amount);
/// I4 adds the sign. NAMED SEAM: at convergence scale the mint is bounded
/// by verified work — `mint ≤ verify_certificate(work)` — the PoUW anchor
/// (KERNEL CHARTER §5a), which fills on Ruling Box 1. No fiat (05 §1.2).
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
            let difficulty =
                verify_work(work).map_err(|broken| Refused::WorkRefused { broken })?;
            let bound = mint_bound(difficulty.rung());

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

/// THE MINT BOUND — RULED (Ruling Box 1, operator 2026-08-01; the
/// VERSION 1→2 amendment). The emission rate `f` is pinned against
/// the #17 calibration's measured hit-rates (evidence commit
/// 5898351: 100% · 100% · 100% · 97.5% · 75% · 42.5%), in exact
/// integers on the LADDER — L1/ladder terms only, no norms, no roots
/// (THE PROPORTIONAL LAW):
///
/// ```text
/// f(0) = 0 · f(1) = 0 · f(2) = 0   (saturated — free issuance barred)
/// f(3) = 1                          (the marginal threshold, 97.5%)
/// f(4) = 4                          (scarcity tier 1, 75%)
/// f(5) = 7                          (scarcity tier 2, 42.5% — the
///                                    ~1.75× inverse-hit-rate step)
/// ```
///
/// `f` is defined ON THE LADDER ONLY: a payload's shape either matches
/// a ladder rung exactly or it earns ZERO — an off-ladder shape cannot
/// name its own price (the attacker-shaped hole the #17 audit closed
/// stays closed at the rate too).
///
/// **PRICED ON THE RUNG, NOT THE SHAPE** (operator ruling 2026-08-04:
/// *`Grid` should fall out, because we aren't building a matrix*). This
/// took `&Difficulty` and matched the whole struct — so `rows` and
/// `cols`, the dimensions of an ARC grid, were part of the judge's
/// price lookup. THE MOTION clause 5 forbids exactly that: *ARC is a
/// TYPE of work, not the only type.* The exact-match defence is
/// unchanged and now lives in `Difficulty::rung`, which is where the
/// shape belongs; the judge is handed a POSITION. One pricing path now
/// serves every work class, planar and volumetric alike, instead of
/// each new class needing its own.
/// Work-shapes v1: planar forge tasks only (structural — `verify_work`
/// accepts no other form). Reward split v1: 100% to the solving world
/// (the deed banks on its own orb; no mesh counterparty exists yet;
/// the v2 40/40/20 prover/mesh/reserve split is INSCRIBED in
/// U_DIRECTIVES §8 for the mesh era). Saturated rungs are
/// proof-of-position; the economy lives up the ladder.
pub fn mint_bound(rung: Option<u32>) -> Ratio {
    match rung {
        Some(3) => ratio(1, 1),
        Some(4) => ratio(4, 1),
        Some(5) => ratio(7, 1),
        // Saturated rungs pay zero by ruling, and OFF-LADDER pays zero
        // because a payload never names its own price.
        _ => ratio(0, 1),
    }
}

/// INVARIANT — bounded decay (I5), COMPLETE (U1f). Value is destroyed
/// non-negative decay (a decay that creates value is refused). The
/// declaration==effect half is conservation (the world total fell by
/// exactly the declared amount); I5 adds the sign. NAMED SEAM: the decay
/// is FLAT and denominator-bounded, `decay ≤ rate · ease · elapsed`
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
            // I5's bound (U1f): THE SEALED DEMURRAGE RULE, called
            // never copied (CHARTER §0b) — one computation, two
            // judges. `elapsed` reads the prior orb's `last_touch`,
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
                        // RULESET EPOCHS (`21_KERNEL_PARAMETERS` §11): the judge
                        // selects the rule by the stamp the DEED carries,
                        // not by the rule standing today. A receipt
                        // written under EPOCH 0 is judged by EPOCH 0
                        // forever, which is what stops `replay` drifting
                        // from the live path when a sealed rule moves.
                        &cfe::world::ease_of(&prior.techs, *orb_id, step.stamp),
                        // P2: the judge reads standing supply from the
                        // SAME sealed function the proposer used, on
                        // `prior`'s own orbs — one computation, two
                        // judges (CHARTER §0b). A rate the kernel
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
        _ => Ok(()),
    }
}

/// INVARIANT — exactness (I3). Every value in `next` is a canonical exact
/// rational. Exactness is primarily a TYPE-LEVEL guarantee — `Ratio` is
/// `BigRational`, canonical by construction, and no float exists anywhere
/// in the codebase — so this passes for every well-typed state; I3 is the
/// sealed DECLARATION of that law plus defense-in-depth over the value
/// ledgers (the banked resources and elements most likely to carry a
/// decoded value). It CALLS the sealed `ratio::is_canonical` (KERNEL
/// CHARTER §0b: the equation is shared, never reimplemented in the judge).
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

/// INVARIANT — containment (I2). An actor-deed's actor must be an entity
/// in `prior` that holds the scope the deed requires. Identity precedes
/// the world: a deed from no one refuses before any state is read. A
/// sovereign holds every scope by construction; an agent holds exactly
/// its granted scopes. World-driven events (consequences the tick
/// computes, no actor) carry no scope requirement (`scope_of` = None).
fn containment(prior: &Committed, step: &Step) -> Result<(), Refused> {
    let Some(scope) = scope_of(&step.event) else {
        return Ok(());
    };
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

/// **THE LAYER OF A DEED — how far a rule reaches** (`17_CAPACITY` §3d).
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
    /// Real and lawful, and NEVER a precondition for anyone else's
    /// standing.
    Civilizational,
}

/// **THE LAYER LAW — EXHAUSTIVE BY DESIGN**, in the shape of
/// [`scope_of`]: adding a `ClosureEvent` variant forces a LAYER
/// decision here at compile time. There is no wildcard arm and there
/// must never be one, because a wildcard is exactly the inference this
/// function exists to abolish.
///
/// **`None` means NOT YET RULED — it does not mean "no layer".** A
/// false assignment is worse than an absent one, so a genuinely
/// contested deed answers `None` and a gate consulting this must REFUSE
/// on it rather than assume a reach — the discipline `scope_of` uses
/// for a deed with no actor scope and `covered()` for an unjudged
/// event.
///
/// **No deed answers `None` today** (2026-08-02): the three that were
/// contested were all settled by reading their projections. The
/// `Option` REMAINS in the signature deliberately — the next deed whose
/// reach is genuinely undecided needs an honest home that does not
/// require changing this signature under pressure, which is exactly
/// when a default would get invented.
///
/// The rows are keyed on WHAT THE DEED CARRIES, never on the document
/// it is described in — a carrier is not a mechanism (`17` §3d.1).
pub fn layer_of(event: &ClosureEvent) -> Option<Layer> {
    use ClosureEvent as E;
    match event {
        // ── CIVILIZATIONAL — keyed on a LINE's own identity ──────────
        E::GenesisSealed { .. }        // civilization + the three axes
        | E::TaxonSealed { .. }        // class, clade
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

        // ── UNIVERSAL — the universe, the mesh, or any actor ─────────
        E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        | E::EraAnnexed { .. } | E::EraFolded { .. } | E::GalaxyObserved { .. }
        | E::HandoffSealed { .. }
        | E::ResourceTransferred { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        | E::DiscoveryMinted { .. } | E::SovereignUnionSealed { .. }
        // Obligation instruments: ordinary finance, open to any line,
        // each feeding a D channel. The forward RULING binds the
        // ceremony, never the remainder (`17` §3d.1a).
        | E::BondSealed { .. } | E::BondSettled { .. }
        | E::ForwardSealed { .. } | E::ForwardReleased { .. } | E::ForwardSlashed { .. }
        | E::OverflowAbsorbed { .. } | E::AbsorptionLapsed { .. }
        | E::CouncilSealed { .. } | E::CouncilClosed { .. }
        | E::OfferPosted { .. } | E::OfferCleared { .. }
        | E::RouteChartered { .. } | E::RouteLapsed { .. }
        | E::VoidShare { .. } => Some(Layer::Universal),

        // ── WORLD — keyed on an ORB inside one committed state ───────
        E::ColonyFounded { .. } | E::ColonyCharted { .. } | E::WorldCharted { .. }
        | E::ZoneBuilt { .. } | E::ZoneRazed { .. }
        | E::YieldCollected { .. } | E::DemurrageSettled { .. } | E::UpkeepSettled { .. }
        | E::VentSettled { .. }
        | E::StarCommissioned { .. } | E::StarAged { .. } | E::PlanetSited { .. }
        | E::OverheatStruck { .. } | E::OverheatCleared { .. }
        | E::FamineStruck { .. } | E::FamineRelieved { .. }
        | E::GaugeShifted { .. } | E::GaugeShiftedAt { .. }
        | E::ProspectStruck { .. } | E::AssayStruck { .. } | E::CrystalDrawn { .. }
        | E::InstrumentForged { .. }
        | E::TechDiscovered { .. } | E::TechLicensed { .. } => Some(Layer::World),

        // ── RULED 2026-08-02 by reading the projection ───────────────
        // FrictionSettled was contested on the guess that it computed
        // from 09's ideology axes. IT DOES NOT. The chain is:
        //   friction   = friction_rate() * divergence * value
        //   divergence = |reciprocity_reading(from) - reciprocity_reading(to)|
        //   reciprocity_reading(orb)
        //              = returned_across / (returned_across + extracted_across)
        // Two of the orb's OWN measured behavioural books — the same
        // pair the missing-leg sweep found already two-armed. No creed,
        // no genesis axis, no class or clade appears anywhere in it.
        // `09` CALLS it "ideological friction"; the projection prices
        // UNLIKE CONDUCT, and any line can be reciprocal or not.
        // Second instance of the §3d.1 trap: a document title is not a
        // scope, exactly as a carrier is not a mechanism.
        E::FrictionSettled { .. } => Some(Layer::Universal),

        // ── RULED 2026-08-02 by reading the projection ───────────────
        // WantStruck/WantRelieved: the whole block is gated on a SEALED
        // genesis — "no seal, no entity type, no want" — and each kind
        // on a specific axis: resonance needs perception == 3
        // (harmonic), novelty needs disposition == 2 (empiricist). A
        // line with different axes NEVER EXPERIENCES THE MECHANIC. That
        // is civilizational by the exact criterion: a people's own, and
        // never a precondition for anyone else's standing.
        E::WantStruck { .. } | E::WantRelieved { .. } => Some(Layer::Civilizational),

        // IncursionStruck LOOKED like it spanned all three layers,
        // carrying era, world AND line. It does not, and the puzzle
        // dissolves on a sharper criterion: A LAYER IS WHAT A DEED
        // BINDS, NOT WHAT IT MENTIONS. Replay does nothing with it
        // (`world/replay.rs`: `IncursionStruck { .. } => {}`), no live
        // path consumes it, and it is universe-ledger only — "the
        // archive never edited, the living never raided". The world and
        // line names are the SUBJECT of the record, not keys binding
        // those entities, exactly as `EraAnnexed` carries a world and
        // binds none.
        E::IncursionStruck { .. } => Some(Layer::Universal),
    }
}

/// The sealed event → scope law (a CFE transition rule extracted into the
/// kernel). EXHAUSTIVE by design: adding a `ClosureEvent` variant forces a
/// scope decision here at compile time — the amendment discipline made
/// structural. `None` = a world-driven consequence with no actor scope, or
/// a mesh/universe-ledger deed governed by its own law (never a hole: an
/// uncovered event refuses `Unjudged` before this is consulted). Each arm
/// is RATIFIED by a red-first witness as its event enters `covered()`
/// against `world.rs`; today only `ResourceTransferred` (Collect) is
/// covered and witnessed. The rest are the drafted law, unenforced until
/// their coverage lands.
fn scope_of(event: &ClosureEvent) -> Option<Scope> {
    use ClosureEvent as E;
    match event {
        // Build — create or extend the world, or open authority.
        E::ColonyFounded { .. } | E::ZoneBuilt { .. } | E::WorldCharted { .. }
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        | E::ColonyCharted { .. } | E::StarCommissioned { .. } | E::PlanetSited { .. }
        | E::BondSealed { .. } | E::ForwardSealed { .. } | E::CouncilSealed { .. }
        | E::GaugeShiftedAt { .. } | E::TechLicensed { .. } | E::GenesisSealed { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. } | E::ProspectStruck { .. }
        | E::ElderWarded { .. } | E::VeilDrawn { .. } | E::InstrumentForged { .. }
        | E::RouteChartered { .. } | E::OfferPosted { .. } | E::LineSucceeded { .. }
        // KERNEL-TAXON: the drafted Build-class arm coder-03 named for
        // their #42 — unenforced until their covered() entry + witness.
        | E::TaxonSealed { .. }
        | E::StationSeated { .. }
        // D5 — THE SOVEREIGN UNION. Build-class: it opens the standing
        // the whole world is raised on. The scope is necessary and NOT
        // sufficient — `union` additionally requires the ROOT
        // entity, because no delegate may strike the union.
        | E::SovereignUnionSealed { .. }        // BOOK-1 (19_LENS §16): minting DECLARES a standing account and
        // archiving ALTERS one — both extend or amend the world's
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
        | E::YieldCollected { .. } | E::UpkeepSettled { .. } | E::ResourceTransferred { .. }
        | E::OverflowAbsorbed { .. } | E::BondSettled { .. } | E::ForwardReleased { .. }
        | E::FrictionSettled { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        | E::CrystalDrawn { .. } | E::AssayStruck { .. } | E::OfferCleared { .. }
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
        | E::GalaxyObserved { .. } | E::OverheatStruck { .. } | E::OverheatCleared { .. }
        | E::VentSettled { .. } | E::DemurrageSettled { .. } | E::AbsorptionLapsed { .. }
        | E::ForwardSlashed { .. } | E::WantStruck { .. } | E::WantRelieved { .. }
        | E::HandoffSealed { .. } | E::GaugeShifted { .. } | E::Diverged { .. }
        | E::Confluent { .. } | E::StarAged { .. } | E::EraFolded { .. }
        | E::CouncilClosed { .. } | E::TechDiscovered { .. } | E::IncursionStruck { .. }
        | E::VoidShare { .. } | E::RouteLapsed { .. } => None,
    }
}

/// **THE SEALED WARRANT (K4).** The kernel's answer to `cfe`'s
/// question: what does this payload earn?
///
/// This is the one organ that may read a work payload, and it does the
/// same two things the judge does — `verify_work` then `mint_bound` —
/// so proposer and judge cannot disagree about what a proof is worth
/// (CHARTER §0b, call-not-copy).
///
/// It closes the live gate WITHOUT breaking the opaque-payload law:
/// `cfe` still never imports the verifier. It asks; this answers.
pub struct SealedWarrant;

impl cfe::world::MintWarrant for SealedWarrant {
    fn earns(&self, work: &[u8]) -> Option<Ratio> {
        let difficulty = verify_work(work).ok()?;
        Some(mint_bound(difficulty.rung()))
    }

    /// THE ESCROW CHECK, answered by the judge itself — `admits` is the
    /// same function the door runs, not a copy of its reasoning
    /// (CHARTER §0b, call-not-copy). So a deed the emitter settles is
    /// one the judge has already accepted on the identical inputs, and
    /// proposer/judge drift on the mint path is not caught late — it is
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
