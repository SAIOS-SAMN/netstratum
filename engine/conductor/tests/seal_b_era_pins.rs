//! **SEAL B ACROSS THE WHOLE FOUNDED ROSTER** (task #103) — the engine's
//! foundational guarantee is `replay(chronicle) == committed()`: the chain IS
//! the truth. The other era tests assert it on specific constructed worlds; this
//! asserts it on EVERY polity a full era founds, which is what surfaced the
//! science-seat divergence.
//!
//! The bug: the `ScienceSeated` replay arm re-derived the seat's multi-resource
//! bill with a HARDCODED affinity of 1 (Primary), while the live seat billed the
//! clade's real affinity (Secondary/Alien) plus, for Research, the economic-mode
//! multiplier — so every non-Primary line under-charged on replay and its
//! committed food/local stock drifted by the affinity gap. Fixed by reconstructing
//! the same affinity at replay from the sealed taxon this replay already recovers.

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sealb-era-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn every_founded_line_replays_exactly() {
    let root = scratch("all");
    let worlds = run_era(&root.to_string_lossy(), 20);
    assert!(worlds.len() >= 3, "the roster founded");
    for (i, polity) in worlds.iter().enumerate() {
        assert_eq!(
            cfe::world::replay(polity.world.chronicle()),
            polity.world.committed(),
            "polity {i} diverges: replay != committed (the chain is not the truth)"
        );
    }
    eprintln!("SEAL B: all {} founded lines replay exactly", worlds.len());
}
