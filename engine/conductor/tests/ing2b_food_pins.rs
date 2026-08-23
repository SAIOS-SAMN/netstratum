//! **ONT-2b — THE VEIN IS THE LARDER, EATING IS MINING.** A founded era
//! prospects its worlds (`ground_step`), so a line whose species can
//! metabolise one of its planet's own struck veins FLIPS its ration to that
//! almanac ELEMENT (`ration_of` → `Ration::Element`). Under ONT-2b that line
//! no longer eats from a geometrically pre-mined element BANK — the vein IS
//! its larder: `cultivated_yield` = Σ vein_richness lifts `K_pop`, the band
//! grows into the vein, and its hunger MINES it (`StapleMined`,
//! `extracted += eaten`) demand-driven at the collect touch, the ground-assay
//! pre-mine of the staple stilled.
//!
//! Two facts, witnessed here:
//! 1. THE ECONOMY ENGAGES: over a founded era a real fraction of the roster
//!    emits `StapleMined` — eating-is-mining is not a dormant wire but a thing
//!    prospected societies actually run, off the closed 3-`Resource` carriers.
//! 2. IT REPLAYS EXACTLY: every line that mines its staple replays
//!    byte-for-byte (SEAL B holds across the new tag 183 wire, and the
//!    demand-driven `eaten` is lattice-floored so the vein integral never
//!    explodes).

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ont2b-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_staple_food_economy_mines_the_vein_by_eating() {
    let root = scratch("era");
    let worlds = run_era(&root.to_string_lossy(), 20);
    assert!(worlds.len() >= 3, "the roster founded");

    let mut with_element_food = 0usize;
    let mut total_mines = 0usize;
    for polity in &worlds {
        let mines = polity
            .world
            .chronicle()
            .receipts()
            .iter()
            .filter(|r| matches!(r.event(), cfe::dop::ClosureEvent::StapleMined { .. }))
            .count();
        if mines == 0 {
            continue;
        }
        with_element_food += 1;
        total_mines += mines;
        // A line that mines its staple by eating replays exactly — the vein
        // integral reconstructs byte-for-byte from the tag-183 receipts.
        assert_eq!(
            cfe::world::replay(polity.world.chronicle()),
            polity.world.committed(),
            "a staple-mining line replays exactly (SEAL B across tag 183)"
        );
        // THE BUST MECHANISM: a line that mined has DEPLETED its larder — at
        // least one of its veins carries a positive extraction integral (the
        // band ate the vein down). This is what drops `cultivated_yield` →
        // `K_pop` → the plateau/decline (no element regrowth until ONT-3).
        let depleted = polity
            .world
            .committed()
            .prospects
            .iter()
            .any(|vein| vein.extracted > cfe::ratio::ratio(0, 1));
        assert!(
            depleted,
            "a mining line has eaten its vein down — the extraction integral is positive"
        );
    }
    eprintln!(
        "ONT-2b ERA: {with_element_food} of {} lines mine an almanac staple by eating ({total_mines} pulls)",
        worlds.len()
    );
    // The food economy is LIVE and DEMAND-DRIVEN off the closed 3-Resource
    // carriers: prospected societies dig the food element their own world grows,
    // driven by their hunger, geometry only bounding the vein.
    assert!(
        with_element_food >= 1,
        "no line mined its staple vein — the eating-is-mining economy never engaged over the era"
    );
}
