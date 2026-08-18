//! N-I·VESSEL — Pin B at VESSEL GRAIN: a person's wall is their own.
//!
//! The ARC basin is per vessel (`vessel_id → Vec<Crystal>`, an ordered INDEX of
//! mouths — never a HashMap, Law II). This witnesses the isolation the keying
//! buys: a vessel that walls a puzzle learns the crystal into ITS OWN basin; the
//! same class drains for that vessel next (Pin B); a SIBLING on the same puzzle
//! walls afresh — it never inherits the wall. Two vessels, two perspectives. The
//! organ is unchanged (`think_task` still takes `&mut Vec<Crystal>`); the grain
//! lives entirely in WHICH basin is drained.

use solver::{think_task, Crystal, Grid, Task, TaskPair};
use std::collections::BTreeMap;

fn grid(cells: &[u8]) -> Grid {
    Grid::new(2, 2, cells.to_vec()).expect("2x2 grid")
}

/// A task whose output carries a hue the input never did (hue 2 — a generation
/// gap, not a gauge): `think` WALLS it and mints one crystal. Two identical
/// pairs give `think` its ≥2 observations (the coincidence gate stays).
fn walling_task() -> Task {
    let inp = grid(&[0, 1, 0, 1]);
    let out = grid(&[0, 1, 0, 2]);
    Task {
        name: "wall".into(),
        train: vec![
            TaskPair { input: inp.clone(), output: out.clone() },
            TaskPair { input: inp.clone(), output: out.clone() },
        ],
        test_in: inp,
        test_expected: None,
    }
}

#[test]
fn pin_b_at_vessel_grain_a_sibling_does_not_inherit_the_wall() {
    let task = walling_task();
    // The BTreeMap is the index of mouths (Law II: ordered, replay-stable walk,
    // never a hash). The organ only ever sees the Vec it is handed.
    let mut basins: BTreeMap<u64, Vec<Crystal>> = BTreeMap::new();
    let (v, w) = (7u64, 9u64); // two vessels — two people

    // V thinks the walling task — it WALLS and learns the crystal into ITS mouth.
    let _ = think_task(&task, basins.entry(v).or_default());
    assert_eq!(basins[&v].len(), 1, "V learned the wall — its own basin grew by one");
    assert!(basins.get(&w).map_or(true, |b| b.is_empty()), "W's basin is untouched");

    // Pin B for V: the SAME class next beat DRAINS against V's own crystal — V's
    // basin grows nothing new (the wall at K is a crystal at K+1, for V).
    let _ = think_task(&task, basins.entry(v).or_default());
    assert_eq!(basins[&v].len(), 1, "V drains its own crystal (Pin B), grows nothing new");

    // A SIBLING W on the SAME unique WALLS AFRESH — it never saw V's crystal.
    let _ = think_task(&task, basins.entry(w).or_default());
    assert_eq!(basins[&w].len(), 1, "W walls afresh — the wall did not cross basins");
    assert_eq!(basins[&v].len(), 1, "and V's basin is still exactly its own");
}

#[test]
fn two_vessels_two_perspectives_on_the_same_puzzle() {
    // The consequence: given the SAME puzzle, a vessel that has already walled it
    // reads it as ACCOUNTED (its basin drains it), while a fresh vessel still
    // walls — the identical input, two verdicts, because the basin is the
    // perspective.
    let task = walling_task();
    let mut basins: BTreeMap<u64, Vec<Crystal>> = BTreeMap::new();
    let (learned, naive) = (1u64, 2u64);

    // Grow the `learned` vessel's basin.
    let _ = think_task(&task, basins.entry(learned).or_default());
    assert_eq!(basins[&learned].len(), 1);

    // The naive vessel, on the identical puzzle, has nothing to drain against.
    let _ = think_task(&task, basins.entry(naive).or_default());
    assert_eq!(basins[&naive].len(), 1, "the naive vessel walls where the learned one drains");
}
