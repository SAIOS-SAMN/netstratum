//! **THE MEASURED SYSTEM BOUNDS** (P2, operator ruling 2026-08-06:
//! *execute the conductor thresholds as measured system bounds*).
//!
//! Re-measured 2026-08-07 after GROUND STEP (prospect/assay + regenerator
//! ecology) and the sovereign founding band (Kemet · Baktun · Dogon).
//! The autonomous loop opens veins and may place Regenerators when veins
//! wear, so the settlement footprint moved. The pin records the new
//! measurement deliberately — not a silent nudge.
//!
//! The industry half of the bounds (smelter/synthesizer gates behind
//! research rungs) is measured where it always was:
//! `research_unlocks_industry` pins the 45-beat outcome; this file
//! deliberately stays inside the cheap 15-beat window the settlement
//! thresholds govern.

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "measured-bounds-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// Measured 2026-08-07 on the founding roster (incl. sovereign band),
/// 15 beats, under derived settlement gates + ground_step.
/// Prior pin was 50 (2026-08-06, pre-ground, no T2 seats).
const MEASURED_ZONE_TOTAL_AT_15: usize = 52;

/// **THE ENERGY-BOOTSTRAP DEBT** — the lines the settlement law cannot
/// reach, pinned so the set can only SHRINK:
///
/// - cold-siting lines freeze-lock in joule famine at one CryoArray
///   (the Frost branch's own doc calls the freeze-lock "legitimate
///   drama, not a bug");
/// - Furnace / thermal holds and new sovereign seats that famish inside
///   the window (pleiadian, baktun, dogon) — recorded so a cure shrinks
///   this list rather than silently green-washing the debt.
const FROZEN_AT_15: [&str; 8] = [
    "agarthan",
    "archon",
    "draco",
    "frostwright",
    "darlok",
    "pleiadian",
    "baktun",
    "dogon",
];

/// **THE SETTLEMENT BOUNDS, MEASURED.** Fifteen beats over the whole
/// founding roster; per line, the city its gates actually built.
#[test]
fn the_settlement_thresholds_measure_and_pin() {
    let root = scratch("settlement");
    let worlds = run_era(&root.to_string_lossy(), 15);
    assert!(worlds.len() >= 3, "the roster founded");

    let mut census: Vec<(String, usize)> = Vec::new();
    for polity in &worlds {
        let sigil = polity.spec.civilization.sigil.clone();
        let orb = polity.world.orb().expect("every polity holds its orb");
        let zones = orb.zones.len();
        let famished = !orb.famine.is_empty();
        census.push((sigil.clone(), zones));

        if FROZEN_AT_15.contains(&sigil.as_str()) {
            assert!(
                famished,
                "{sigil}: recorded frozen, measured fed — the energy debt \
                 moved; re-measure and shrink the pin"
            );
            continue;
        }
        // THE BOOTSTRAP LAW, held: every line the energy debt does not
        // claim builds past its founding inside the window…
        assert!(
            zones >= 2 && !famished,
            "{sigil}: stranded at {zones} zones (famine: {famished}) — \
             a gate stopped satisfying the bootstrap law"
        );
        // …and none runs past the largest city cap.
        assert!(zones <= 20, "{sigil}: {zones} zones exceeds every cap");
    }

    // The temperaments DISTINGUISH — the reserve factors buy
    // divergence or they buy nothing.
    let counts: Vec<usize> = census.iter().map(|(_, zones)| *zones).collect();
    assert!(
        counts.windows(2).any(|pair| pair[0] != pair[1]),
        "every temperament built the same city: {census:?}"
    );

    // THE MEASUREMENT, PINNED WHOLE: the era's zone total — the
    // reserve factors' aggregate footprint at beat 15, deterministic,
    // exact. A factor change lands here first and answers for its
    // whole-board effect in review.
    let total: usize = counts.iter().sum();
    assert_eq!(
        total, MEASURED_ZONE_TOTAL_AT_15,
        "the settlement footprint moved — a bound changed; re-measure \
         deliberately and move this pin with it: {census:?}"
    );
}
