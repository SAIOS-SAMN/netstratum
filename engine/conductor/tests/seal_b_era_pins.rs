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

/// **#136 D3 — THE STANDING NEGATIVE WITNESS: every conserved element ledger stays
/// on the grain.** Every element FILL declares its grain at its fill site — haul and
/// trade (`world/mod.rs`), farm (`economy.rs`), assay (`core.rs`), refine / gem /
/// iso-union (`chemistry.rs`), salvage (`fleet.rs`). After a full era exercises those
/// paths, EVERY element stock on EVERY orb must land on the 1/1024 element grain: its
/// denominator divides 1024, i.e. `smelt_element(amount) == amount` (the floor is
/// idempotent exactly on the grain). A future fill site that forgets to smelt re-opens the
/// denominator explosion that stalled `mass_of` (#136) — and fails THIS witness, at
/// build time, not a live beat. This is what makes doc 54 §6 a law, not an intention.
#[test]
fn every_element_stock_stays_on_the_grain() {
    let root = scratch("grain");
    let worlds = run_era(&root.to_string_lossy(), 20);
    let mut checked = 0u64;
    for (i, polity) in worlds.iter().enumerate() {
        for orb in polity.world.orbs() {
            for (element, amount) in &orb.elements {
                assert_eq!(
                    cfe::world::smelt_element(amount),
                    *amount,
                    "polity {i} orb {} element {:02x?} is off the 1/1024 grain: {amount} \
                     (a fill site forgot to smelt — the #136 denominator explosion)",
                    orb.orb_id,
                    &element[..4],
                );
                checked += 1;
            }
        }
    }
    eprintln!(
        "#136 D3: {checked} element stocks across {} founded lines, all on the 1/1024 grain",
        worlds.len()
    );
}
