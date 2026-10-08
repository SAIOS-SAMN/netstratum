//! docs/58 §4 (amended 2026-10-08) — the largest living lineage: the living
//! line whose living descendant tree is STRICTLY the largest (the progenitor
//! candidate; lineage does not crown an ancient); a tie names no one.

use conductor::largest_lineage;
use ledger::registry::{Civilization, Registry};
use std::collections::BTreeSet;

fn card(sigil: &str, lineage: &[&str]) -> Civilization {
    Civilization {
        name: sigil.to_string(),
        sigil: sigil.to_string(),
        species: "test".into(),
        clade: String::new(),
        motto: String::new(),
        disposition: "expansionist".into(),
        metabolism: "mineral".into(),
        perception: "planar".into(),
        founder: "test".into(),
        lineage: lineage.iter().map(|s| s.to_string()).collect(),
        station: None,
    }
}

fn living(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|s| s.to_string()).collect()
}

#[test]
fn the_largest_living_tree_raises_its_root_and_a_tie_raises_no_one() {
    let root = std::env::temp_dir().join(format!("conductor-ancient-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let reg = Registry::at(&root);
    // a → b → c, and d → e: a's tree is 3, d's is 2.
    for c in [card("a", &[]), card("b", &["a"]), card("c", &["b"]), card("d", &[]), card("e", &["d"])] {
        reg.create(&c).unwrap();
    }
    assert_eq!(largest_lineage(&reg, &living(&["a", "b", "c", "d", "e"])), Some(("a".to_string(), 3)));
    // c dies: a's tree and d's tree are both 2 — no one is singled out.
    assert_eq!(largest_lineage(&reg, &living(&["a", "b", "d", "e"])), None);
    // a dies: a dead line keeps no seat; b's tree (b, c) is the largest.
    assert_eq!(largest_lineage(&reg, &living(&["b", "c", "e"])), Some(("b".to_string(), 2)));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_world_named_by_a_compiled_seat_or_a_seat_card_is_taken() {
    let root = std::env::temp_dir().join(format!("conductor-world-taken-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let reg = Registry::at(&root);
    // Forgeborn's compiled world.
    assert!(conductor::world_name_taken(&reg, "forge-reach"));
    assert!(!conductor::world_name_taken(&reg, "nowhere-yet-founded"));
    reg.create(&card("cleaved", &["a"])).unwrap();
    reg.write_founded_at(
        "cleaved",
        &ledger::registry::Seat {
            world: "nowhere-yet-founded".into(),
            planet: "p".into(),
            colony: "c".into(),
            position: (cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1)),
            policy: "steady".into(),
        },
    )
    .unwrap();
    assert!(conductor::world_name_taken(&reg, "nowhere-yet-founded"), "a seat card holds its world");
    let _ = std::fs::remove_dir_all(&root);
}
