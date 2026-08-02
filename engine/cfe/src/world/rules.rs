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

