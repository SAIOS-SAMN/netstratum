//! **THE MEASURED SYSTEM BOUNDS** (P2, operator ruling 2026-08-06:
//! *execute the conductor thresholds as measured system bounds*).
//!
//! The settlement gates DERIVE now — cost × the policy's reserve
//! factor, bootstrap at cost (`build_step`) — because this harness's
//! first measurement proved the dead hand-written thresholds unlawful
//! at bootstrap: a reserve gate on a resource with no income source is
//! unsatisfiable, so seven Steady lines locked at one zone (habitat
//! gate = the whole founding biomass) and every Industry line
//! habitat-spammed into iron famine (extractor gate above the founding
//! endowment). What remains tunable — the reserve factors and city
//! caps — is measured HERE: one deterministic era, the whole board's
//! footprint pinned exactly. A factor change moves these measurements
//! and goes red, so tuning is a re-measurement in review — never a
//! silent nudge.
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

/// Measured 2026-08-06 on the seeded founding roster, 15 beats, under
/// the derived settlement gates.
const MEASURED_ZONE_TOTAL_AT_15: usize = 50;

/// **THE ENERGY-BOOTSTRAP DEBT** — the lines the settlement law cannot
/// reach, pinned so the set can only SHRINK:
///
/// - the five cold-siting lines freeze-lock in joule famine at one
///   CryoArray (the Frost branch's own doc calls the freeze-lock
///   "legitimate drama, not a bug" — recorded here as the measured
///   cost of that ruling);
/// - pleiadian deadlocks in the FURNACE shell-hold: the hold waits for
///   shell + extractor funded together while the line has no iron
///   income — the same unlawful bootstrap shape the settlement gates
///   shed, standing in the thermal branch. Reported; an operator
///   ruling on the Furnace hold owns the fix.
const FROZEN_AT_15: [&str; 5] = ["agarthan", "archon", "draco", "frostwright", "darlok"];
const FURNACE_HELD_AT_15: [&str; 1] = ["pleiadian"];

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
        if FURNACE_HELD_AT_15.contains(&sigil.as_str()) {
            assert!(
                !famished && zones == 1,
                "{sigil}: the Furnace hold moved — re-measure and shrink \
                 the pin ({zones} zones)"
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
