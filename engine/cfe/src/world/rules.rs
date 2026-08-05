//! THE SEALED RULES — every rule read by BOTH the live proposer and
//! the gate.
//!
//! **This module exists because a rule read from two places is the
//! drift that already cost this codebase a landing.** P3 (the ease
//! gradient, reverted) moved a sealed rule's values and the two readers
//! produced different worlds from the same receipts. Nothing named the
//! rule as shared, so nothing caught it until a workspace sweep did.
//!
//! GROUND §3 invariant 3: **shared rules are named as shared.** After
//! this module, "which rules do two paths read?" is answered by reading
//! one file instead of by a bisect.
//!
//! THE BOUNCE (GROUND §4): adding a rule here is a deliberate act. A
//! value that moves in this file moves under BOTH gates at once, so
//! amending anything here requires a workspace sweep and never a
//! single-crate check. The kernel CALLS these, never copies them
//! (CHARTER §0b).

use crate::dop::{ColonyOrb, Resource};
use crate::ratio::{ratio, Ratio};

/// SINK-1 (Q2): the demurrage due on one orb's un-vaulted banked
/// joules/sink at `stamp` — computed PURE; the caller applies and
/// receipts (so a refused action never orphans a settlement).
/// TECH-1b (10 §3): the Long Stores ease — 1/2 once condition 5 has
/// crystallized for this orb, 1 otherwise. Derived from the tech
/// list, never stored. Pub: the kernel CALLS the sealed rule for
/// I5's bound (call-not-copy, KERNEL CHARTER §0b).
pub fn ease_of(techs: &[(u64, u8, [u8; 32])], orb: u64, at: u64) -> Ratio {
    // RULESET EPOCHS (`21_KERNEL_PARAMETERS` §11): the rule is selected
    // by the stamp the deed carries, so a receipt written under EPOCH 0
    // is forever gated by EPOCH 0 and `replay` cannot drift from the
    // live path. Sealed rules are APPEND-ONLY — the cliff below stays
    // in the tree permanently because past receipts still need it.
    match crate::epoch::epoch_at(at) {
        crate::epoch::Epoch::Genesis => ease_cliff(techs, orb),
        crate::epoch::Epoch::Continuous => ease_gradient(techs, orb),
    }
}

/// EPOCH 0 — the landed cliff. **Never edit this function.** Every
/// receipt stamped before `epoch::CONTINUOUS_FROM` is gated by it, and
/// changing it would rewrite history's meaning.
fn ease_cliff(techs: &[(u64, u8, [u8; 32])], orb: u64) -> Ratio {
    match techs.iter().any(|(held, condition, _)| *held == orb && *condition == 5) {
        true => ratio(1, 2),
        false => ratio(1, 1),
    }
}

/// EPOCH 1 — P3, the continuous rational ease gradient.
///
/// The `== 5` cliff made conditions 0 through 4 INDISTINGUISHABLE, so
/// four fifths of the tech tree bought nothing and condition 5 was a
/// strategy target. This descends smoothly across the whole tree.
///
/// Exact over `BigRational` — the algebraic S-curve `s = x/(1+|x|)`,
/// no float, no transcendental, no root.
///
/// **DIRECTION.** `ease` MULTIPLIES decay (`rate · ease · elapsed`), so
/// LOWER ease is the reward. A curve saturating toward 1 at high tech
/// would make advanced technology INCREASE decay; this one descends.
fn ease_gradient(techs: &[(u64, u8, [u8; 32])], orb: u64) -> Ratio {
    let condition = techs
        .iter()
        .filter(|(held, _, _)| *held == orb)
        .map(|(_, condition, _)| *condition)
        .max()
        .unwrap_or(0);

    let x = ratio(condition as i64 - EASE_THRESHOLD, EASE_SCALE);
    let s = x.clone() / (ratio(1, 1) + num_traits::Signed::abs(&x));
    // Floored at 1/2: past the old cliff nothing further is earned, and
    // an ease of zero would remove the sink entirely, leaving I5
    // bounding decay at nothing.
    (ratio(1, 2) - s).clamp(ratio(1, 2), ratio(1, 1))
}

/// P3's curve centre — the condition at which `ease` equals the EPOCH 0
/// cliff value `1/2`.
const EASE_THRESHOLD: i64 = 5;
/// P3's curve width in condition-steps.
const EASE_SCALE: i64 = 5;

/// THE STANDING SUPPLY — the world's total banked per LEAKING
/// resource, which is what P2's EPOCH 1 rate reads
/// (`21_KERNEL_PARAMETERS` §2).
///
/// Sealed and public for the same reason `demurrage_due` is: the
/// kernel calls it on `prior.orbs` and the proposer on its own, so one
/// computation serves two gates and neither can disagree about what
/// supply stood.
pub fn standing_supply(orbs: &[ColonyOrb]) -> Vec<(Resource, Ratio)> {
    [Resource::Joule, Resource::Sink]
        .into_iter()
        .map(|resource| {
            let total = orbs
                .iter()
                .fold(ratio(0, 1), |sum, held| sum + held.banked_amount(resource));
            (resource, total)
        })
        .collect()
}

/// THE SEALED DEMURRAGE RULE (SINK-1/Q2 + the flat-rate ruling):
/// what an orb owes at `stamp` — `min(rate · ease · elapsed, banked)`
/// per leaking resource, FLAT, never proportional (the 2^1100
/// denominator lesson). Pub: the kernel CALLS this for I5's bound
/// (call-not-copy) — one rule, one computation, two gates.
///
/// **`supply`** is the world's standing total per leaking resource,
/// which P2's EPOCH 1 rate reads. Passed PRECOMPUTED rather than
/// derived inside for two reasons: proposer and gate must reach one
/// rate from one state, and a settle site holding `&mut ColonyOrb`
/// cannot also hand out a shared borrow of the slice that orb lives
/// in. Build it with [`standing_supply`]. Ignored under EPOCH 0.
pub fn demurrage_due(
    orb: &ColonyOrb,
    star: Option<crate::thermal::StarClass>,
    stamp: u64,
    ease: &Ratio,
    supply: &[(Resource, Ratio)],
) -> Vec<(Resource, Ratio)> {
    let mut due = Vec::new();
    if let (Some(class), Some(site)) = (star, &orb.thermal) {
        let t4 = crate::thermal::t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value());
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
                    // TECH-1b: Long Stores eases the rate (×1/2 when
                    // crystallized; 1 otherwise) — still flat, still
                    // bounded denominators.
                    // P2: the RATE now reads standing supply of this
                    // resource across the world. Still FLAT in the
                    // holding — supply moves the rate, never the shape.
                    let standing = supply
                        .iter()
                        .find(|(held, _)| *held == resource)
                        .map(|(_, total)| total.clone())
                        .unwrap_or_else(|| ratio(0, 1));
                    let decay = (crate::dop::demurrage_per_tick(&standing, stamp)
                        * ease.clone()
                        * elapsed.clone())
                    .min(banked);
                    due.push((resource, decay));
                }
            }
        }
    }
    due
}


/// CRISIS — one definition, read by BOTH the proposer and replay.
///
/// **This rule was written out EIGHT times before it had a name** —
/// four in the live deeds and four in replay — with comments in both
/// insisting they must agree ("the move replays with the SAME crisis
/// reads the live path made"; "crisis reads match the live order
/// exactly"). Agreement was enforced by copy-paste, which is the same
/// shape as the double transition definition kernel #30 exists to
/// remove, at one rule's scale.
///
/// It decides whether aid accrues to the syntropic books
/// (`given_in_crisis`, `returned_across`), so a drift between the two
/// paths would silently rewrite a civilization's syntropy ratio.
///
/// THE ABSOLUTE, recorded and not yet fixed: this is a BOOLEAN over a
/// degree the engine already computes. `dop.rs:768` degrades yields by
/// `1/(1+overflow)`, so the intensity exists and is used — while this
/// reading throws it away. A colony 1% over its radiator budget reads
/// identically to one 500% over, and aid to a colony at 99% of its
/// budget counts as nothing at all. Fractionalizing it is task #14;
/// naming it here is what makes that change safe to make ONCE.
pub fn in_crisis(orb: &ColonyOrb) -> bool {
    !orb.famine.is_empty() || orb.overheat
}

/// WARDEN-1b — the depth a claimer must reach to lift an elder's
/// reverse shield.
///
/// **A placeholder, named and single-sourced.** It was the bare
/// literal `5` inside `lift_ward`, where nothing could find it and
/// nothing could tune it. The value is unchanged; what changed is that
/// it is now one rule with one name, and §7's binding applies to it
/// visibly: it becomes a READING when it has an operand — the star's
/// own difficulty, or the elder's capacity — rather than staying a
/// constant nobody can see.
pub fn ward_gate_depth() -> u32 {
    5
}

/// How far a claimer stands toward lifting a ward, in `[0, 1]`.
///
/// **The shield itself stays binary and that is correct** — a barrier
/// is sealed or it is open, and a partially sealed shield would leak
/// value through a gate nobody solved. What was wrong is that the work
/// toward it was INVISIBLE: a claimer at depth 4 of 5 had done four
/// fifths of the work and could read nothing back, which is the
/// strategy-target defect of 20_FOUNDING §6a-bis applied to the ward.
///
/// This measures the approach without weakening the barrier. It gates
/// nothing today; it is the operand a partial-credit mechanic would
/// need, and the reading a lens can show so a line knows where it
/// stands.
pub fn ward_progress(depth: u32) -> Ratio {
    let gate = ward_gate_depth();
    match depth >= gate {
        true => ratio(1, 1),
        false => ratio(i64::from(depth), i64::from(gate.max(1))),
    }
}

/// **THE MINT WARRANT (K4).** What a payload EARNS, answered by
/// somebody who may read it.
///
/// `cfe` may not import the verifier — the opaque-payload law, enforced
/// by `kernel/tests/recurrence_guard.rs::the_arrow_law_holds`. Proofs
/// are opaque bytes to the game and only the sealed verifier reads
/// them; a proposer that could interpret proof structure would break
/// the seal.
///
/// So the emitter does not verify: it ASKS. This trait is the callback,
/// and it keeps the arrow intact — `cfe` defines the question, whoever
/// sits above it answers.
///
/// `None` means the payload is not a proof. The emitter refuses BEFORE
/// closing, so a bad mint never reaches the chronicle.
pub trait MintWarrant {
    /// What this payload earns, or `None` if it is not a proof.
    fn earns(&self, work: &[u8]) -> Option<Ratio>;

    /// **THE ESCROW CHECK — would the gate ADMIT this deed?**
    ///
    /// `earns` answers what a payload is WORTH. It cannot answer
    /// whether the deed will stand, because most of the law is a
    /// property of the TRANSITION — D4 reads the capacity increment
    /// across `prior → next`, and conservation reads both totals. A
    /// warrant that only prices the work leaves every transition-shaped
    /// invariant to be discovered afterwards, by which point the
    /// chronicle already holds the deed.
    ///
    /// That is exactly what was measured: a 60-beat era put mints
    /// carrying `MintWithoutCapacityIncrement` into the record, and the
    /// gate refused them where nobody could take them back.
    ///
    /// So the emitter ESCROWS: it builds the state the deed WOULD
    /// produce, asks here whether that transition stands, and settles
    /// only on yes. `cfe` still parses nothing and imports no gate —
    /// it hands over two committed states and the event, and whoever
    /// can gate, gates.
    ///
    /// Default `false`: a warrant that cannot answer must not be able
    /// to settle. The same posture `NoWarrant` takes on `earns`, for
    /// the same reason — the restrictive answer is the shape of the
    /// type, not a rule someone remembers.
    fn admits(
        &self,
        _prior: &crate::world::Committed,
        _next: &crate::world::Committed,
        _event: &crate::dop::ClosureEvent,
        _actor: u64,
        _stamp: u64,
    ) -> bool {
        false
    }
}

/// The warrant that admits nothing — for callers with no verifier.
///
/// Not a convenience: it is the DEFAULT POSTURE. A caller that cannot
/// verify must not be able to mint, and this makes that the shape of
/// the type rather than a rule someone remembers.
pub struct NoWarrant;

impl MintWarrant for NoWarrant {
    fn earns(&self, _work: &[u8]) -> Option<Ratio> {
        None
    }
}
