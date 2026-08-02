//! THE SEALED RULES — every rule read by BOTH the live proposer and
//! the judge.
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
//! value that moves in this file moves under BOTH judges at once, so
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
pub fn ease_of(techs: &[(u64, u8, [u8; 32])], orb: u64) -> Ratio {
    match techs.iter().any(|(held, condition, _)| *held == orb && *condition == 5) {
        true => ratio(1, 2),
        false => ratio(1, 1),
    }
}

/// THE SEALED DEMURRAGE RULE (SINK-1/Q2 + the flat-rate ruling):
/// what an orb owes at `stamp` — `min(rate · ease · elapsed, banked)`
/// per leaking resource, FLAT, never proportional (the 2^1100
/// denominator lesson). Pub: the kernel CALLS this for I5's bound
/// (call-not-copy) — one rule, one computation, two judges.
pub fn demurrage_due(
    orb: &ColonyOrb,
    star: Option<crate::thermal::StarClass>,
    stamp: u64,
    ease: &Ratio,
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
                    // TECH-1b: Long Stores eases the rate (×1/2 when
                    // crystallized; 1 otherwise) — still flat, still
                    // bounded denominators.
                    let decay = (crate::dop::demurrage_per_tick() * ease.clone()
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
