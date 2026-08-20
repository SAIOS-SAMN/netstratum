//! The forge pins: determinism (a task is a reading of history), digest
//! separation (a different chronicle forges a different task), and the forged
//! name's provenance. (The old `forged_tasks_close_at_fair_depth` pin was tied
//! to the retired enumerative `solve` — the "all forged tasks close at word_len
//! depth" assumption is a search-budget claim, not a forge invariant, and the
//! ARC seat's reach is pinned in `think_task_pins` / `tile_seat_pins` instead.)

use solver::{forge_task, Difficulty};

/// Same digest + same difficulty → byte-identical task, on every
/// machine, forever (CHARTER §9 at the forge).
#[test]
fn the_forge_is_a_reading_of_history() {
    for level in 1..=5 {
        let difficulty = Difficulty::level(level);
        let a = forge_task([11u8; 32], difficulty);
        let b = forge_task([11u8; 32], difficulty);
        assert_eq!(a, b, "level {level} forges deterministically");
    }
}

/// A different digest forges a different task; a different difficulty
/// forges a different task from the same digest.
#[test]
fn digests_and_difficulties_separate() {
    let base = forge_task([11u8; 32], Difficulty::level(2));
    let other_digest = forge_task([12u8; 32], Difficulty::level(2));
    let other_level = forge_task([11u8; 32], Difficulty::level(3));
    assert_ne!(base, other_digest);
    assert_ne!(base, other_level);
}

/// The forged name carries provenance: digest prefix + tier + word
/// length — an observer reads the difficulty off the certificate file.
#[test]
fn the_forged_name_carries_provenance() {
    let task = forge_task([0xAB; 32], Difficulty::level(4));
    assert!(task.name.starts_with("forge_abababababab_t3w2"), "{}", task.name);
}
