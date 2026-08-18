//! **ONT-3 — VEIN REJUVENATION, the derivation witnessed.** The pure exact-
//! rational recovery law: a worked vein regrows under Keplerian orbital forcing
//! (`vein_forcing_injection`) and richness derives on read as `natural − extracted
//! + rejuvenated` (`rejuvenated_richness`). Distinct from `rejuvenation_pins.rs`,
//! which pins the #28 RATION regrowth S-curve; this pins the ONT-3 element-VEIN
//! recovery. Kept in `tests/` (not the scanned `src` tree) so its fixture `ratio`
//! literals are not counted by the tuned-constant ratchet — they are examples,
//! not dials.

use cfe::almanac::{rejuvenated_richness, vein_forcing_injection, REGROWTH_GRAIN};
use cfe::ratio::{ratio, Ratio};
use num_traits::ToPrimitive;

/// A worked vein: endowment 4, drawn down by 3 (richness now 1), nothing
/// rejuvenated yet.
fn worked() -> (Ratio, Ratio, Ratio) {
    (ratio(4, 1), ratio(3, 1), ratio(0, 1))
}

#[test]
fn injection_caps_at_the_net_drawn() {
    // Many orbits cannot return more than was ever extracted: a vein drawn by 3
    // rejuvenates by at most 3, back to its endowment, never past.
    let (natural, extracted, rejuvenated) = worked();
    let inject = vein_forcing_injection(&natural, &extracted, &rejuvenated, 10_000, 0);
    assert_eq!(inject, ratio(3, 1), "recovery caps at extracted − rejuvenated");
    // And richness with that injection is exactly the endowment, not above.
    let richness = rejuvenated_richness(&natural, &extracted, &inject);
    assert_eq!(richness, natural, "a fully-rejuvenated vein reads its endowment");
}

#[test]
fn more_orbits_recover_more() {
    // Below the cap, the injection is monotone in completed orbits — a tight
    // inner world (more orbits per beat) recovers faster.
    let (natural, extracted, rejuvenated) = worked();
    let few = vein_forcing_injection(&natural, &extracted, &rejuvenated, 1, 0);
    let many = vein_forcing_injection(&natural, &extracted, &rejuvenated, 20, 0);
    assert!(few > ratio(0, 1) && many > few, "recovery grows with orbits: {few} < {many}");
}

#[test]
fn recycling_amplifies_recovery() {
    // β₁ > 0 (a regenerative economy, return-pathways closing the loop) injects
    // strictly more than a barren one over the same orbits — topological cycle
    // injection amplifying the orbital forcing.
    let (natural, extracted, rejuvenated) = worked();
    let barren = vein_forcing_injection(&natural, &extracted, &rejuvenated, 1, 0);
    let recycling = vein_forcing_injection(&natural, &extracted, &rejuvenated, 1, 2);
    assert!(recycling > barren, "recycling recovers faster: {recycling} > {barren}");
    // (1 + cycles) amplification: 2 cycles ⇒ 3× the base quantum.
    assert_eq!(recycling, barren.clone() * ratio(3, 1), "cycles amplify as (1 + β₁)");
}

#[test]
fn a_still_orbit_and_a_whole_vein_return_nothing() {
    let (natural, extracted, rejuvenated) = worked();
    assert_eq!(
        vein_forcing_injection(&natural, &extracted, &rejuvenated, 0, 5),
        ratio(0, 1),
        "no completed orbit, no injection"
    );
    // An unworked vein (nothing extracted) has no headroom to recover into.
    assert_eq!(
        vein_forcing_injection(&natural, &ratio(0, 1), &ratio(0, 1), 100, 3),
        ratio(0, 1),
        "a whole vein rejuvenates nothing"
    );
}

#[test]
fn richness_is_endowment_minus_net_extraction() {
    // The derive-on-read: natural − (extracted − rejuvenated), clamped.
    let natural = ratio(10, 1);
    assert_eq!(rejuvenated_richness(&natural, &ratio(7, 1), &ratio(2, 1)), ratio(5, 1));
    // Worked to nothing between settles: the lower clamp holds at zero.
    assert_eq!(rejuvenated_richness(&natural, &ratio(12, 1), &ratio(0, 1)), ratio(0, 1));
}

#[test]
fn the_integral_stays_denominator_bounded() {
    // THE ANTI-COLLAPSE PROPERTY. Accumulating injection over many settles is a
    // sum of endowment-scaled quanta, never a compounding recurrence, so the
    // denominator stays bounded forever (contrast the stored-scalar detonation
    // ONT-2 killed). Endowment with denom 6 is the worst bounded case.
    let natural = ratio(5, 6);
    let extracted = natural.clone(); // fully drawn, maximal headroom
    let mut rejuvenated = ratio(0, 1);
    let mut max_denom: u64 = 1;
    for _ in 0..500 {
        let inject = vein_forcing_injection(&natural, &extracted, &rejuvenated, 1, 1);
        if inject <= ratio(0, 1) {
            break;
        }
        rejuvenated += inject;
        max_denom = max_denom.max(rejuvenated.denom().to_u64().unwrap_or(u64::MAX));
    }
    // REGROWTH_GRAIN=64, endowment denom 6 ⇒ quanta ride denom ≤ 6·64·(1+β₁);
    // the accumulated sum's denominator never leaves that bound.
    assert!(max_denom <= 6 * REGROWTH_GRAIN as u64 * 2, "denominator bounded: {max_denom}");
}
