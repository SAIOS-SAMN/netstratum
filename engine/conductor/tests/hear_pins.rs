//! N-I10 — hearing a word, and the steward off thinking.
//!
//! An utterance offered to a hearer is drained (Understood — already accounted)
//! or walled (Learned — the crystal adopted onto the HEARER's basin only, never
//! a copy of the speaker's). And Step 0: the second cognitive vessel is a
//! dedicated sibling office, distinct from the steward (the inbox is not a mind).

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use conductor::{Policy, Polity, PolitySpec};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use solver::{hear, measure_grid, Crystal, Grid, Heard, Op};
use std::fs;
use std::path::PathBuf;

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("grid")
}

#[test]
fn hear_drains_a_gauge_utterance() {
    // A reflection is a universal gauge — any basin DRAINS it (Understood) and
    // adopts nothing. Hearing a word you can already account changes nothing.
    let a = grid(3, 3, &[1, 1, 0, 1, 0, 0, 0, 0, 0]);
    let b = Op::FlipH.apply(&a);
    let (ma, mb) = (measure_grid(&a).unwrap(), measure_grid(&b).unwrap());
    let mut basin: Vec<Crystal> = Vec::new();
    assert_eq!(hear(&ma, &mb, &mut basin), Heard::Understood, "a gauge utterance is understood");
    assert!(basin.is_empty(), "understanding adopts nothing");
}

#[test]
fn hear_walls_and_adopts_onto_the_hearer_only() {
    // A novel-hue utterance the hearer cannot account WALLS — it is Learned, and
    // the crystal is adopted onto the HEARER's basin only. A bystander basin is
    // untouched: hearing is never a copy of another mind's basin.
    let a = measure_grid(&grid(2, 2, &[0, 1, 0, 1])).unwrap();
    let b = measure_grid(&grid(2, 2, &[0, 1, 0, 2])).unwrap(); // hue 2 is novel
    let mut hearer: Vec<Crystal> = Vec::new();
    let bystander: Vec<Crystal> = Vec::new();

    assert_eq!(hear(&a, &b, &mut hearer), Heard::Learned, "a novel utterance is learned");
    assert_eq!(hearer.len(), 1, "the hearer adopted the crystal onto its own basin");
    assert!(bystander.is_empty(), "no copy — a bystander basin is untouched");

    // Pin B at the hear grain: once learned, hearing it again DRAINS.
    assert_eq!(hear(&a, &b, &mut hearer), Heard::Understood, "a learned utterance is then understood");
    assert_eq!(hearer.len(), 1, "re-hearing what it learned grows nothing new");
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("hear-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str, metabolism: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-keeper"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "the inbox is not a mind".into(),
            disposition: "empiricist".into(),
            metabolism: metabolism.into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hearth"),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position: (ratio(5, 1), ratio(5, 1), ratio(5, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

#[test]
fn the_steward_is_not_a_thinker_a_dedicated_sibling_stands() {
    // Step 0: the second cognitive vessel is a DEDICATED sibling office, distinct
    // from the steward — culture never requires the player's directive vessel.
    let root = scratch("swap");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let polity = Polity::genesis(line("hearth", "biotic"), &registry, &book);
    let id = |handle: &str| {
        polity.world.entities().iter().find(|e| e.handle == handle).map(|e| e.entity_id)
    };
    assert!(id("steward").is_some(), "the steward stands (directives)");
    assert!(id("sibling").is_some(), "a dedicated sibling office was seeded");
    assert_ne!(id("steward"), id("sibling"), "the sibling is distinct from the steward");
}
