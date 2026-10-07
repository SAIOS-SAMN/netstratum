//! docs/58 §4 — the ancient arises by lineage: the living line whose living
//! descendant tree is STRICTLY the largest; a tie raises no one.

use conductor::ancient_of;
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
    assert_eq!(ancient_of(&reg, &living(&["a", "b", "c", "d", "e"])), Some(("a".to_string(), 3)));
    // c dies: a's tree and d's tree are both 2 — no one is singled out.
    assert_eq!(ancient_of(&reg, &living(&["a", "b", "d", "e"])), None);
    // a dies: a dead line keeps no seat; b's tree (b, c) is the largest.
    assert_eq!(ancient_of(&reg, &living(&["b", "c", "e"])), Some(("b".to_string(), 2)));
    let _ = std::fs::remove_dir_all(&root);
}
