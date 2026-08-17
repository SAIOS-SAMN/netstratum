//! **ING-2b — THE STAPLE FOOD ECONOMY IS LIVE.** A founded era prospects
//! its worlds (`ground_step`), so a line whose species can metabolise one
//! of its planet's own struck veins FLIPS its ration to that almanac
//! ELEMENT (`ration_of` → `Ration::Element`) and settles food on the
//! element ledger — the 3-`Resource` food enum dissolved at the mechanism.
//!
//! Two facts, witnessed here:
//! 1. THE ECONOMY ENGAGES: over a founded era a real fraction of the roster
//!    emits `RationElementSettled` — the element food loop is not a dormant
//!    wire but a thing prospected societies actually run.
//! 2. IT REPLAYS EXACTLY: every line that settles element food replays
//!    byte-for-byte (SEAL B holds across the new tag 182 wire).

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ing2b-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_staple_food_economy_runs_on_the_element_ledger() {
    let root = scratch("era");
    let worlds = run_era(&root.to_string_lossy(), 20);
    assert!(worlds.len() >= 3, "the roster founded");

    let mut with_element_food = 0usize;
    let mut total_settles = 0usize;
    for polity in &worlds {
        let settles = polity
            .world
            .chronicle()
            .receipts()
            .iter()
            .filter(|r| {
                matches!(r.event, cfe::dop::ClosureEvent::RationElementSettled { .. })
            })
            .count();
        if settles == 0 {
            continue;
        }
        with_element_food += 1;
        total_settles += settles;
        // A line that eats an almanac staple replays exactly — the element
        // food settle reconstructs byte-for-byte from the tag-182 receipt.
        assert_eq!(
            cfe::world::replay(polity.world.chronicle()),
            polity.world.committed(),
            "an element-food line replays exactly (SEAL B across tag 182)"
        );
    }
    eprintln!(
        "ING-2b ERA: {with_element_food} of {} lines eat an almanac staple ({total_settles} settles)",
        worlds.len()
    );
    // The food economy is LIVE off the closed 3-Resource carriers: prospected
    // societies eat the food element their own world grows.
    assert!(
        with_element_food >= 1,
        "no line settled element food — the staple economy never engaged over the era"
    );
}
