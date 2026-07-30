//! LINEAGE-2 pins (12 §5): the ancestry is a DAG, never a single
//! thread. The registry pointer is an append-only list — a divergence
//! branch carries one parent, a confluent hybrid carries BOTH; the
//! wire form is space-joined slugs in the one `lineage` field, so a
//! single-parent write is byte-identical to LINEAGE-1 and old files
//! read unchanged. resolve_head walks the PRIMARY chain (first parent
//! per hop); resolve_heads returns every root, deterministically.

use mirror::registry::{Civilization, Registry};
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("lineage-pin-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str, lineage: Vec<&str>) -> Civilization {
    Civilization {
        name: sigil.to_uppercase(),
        sigil: sigil.into(),
        species: "witness".into(),
        motto: "we pin the law".into(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
        founder: "conductor".into(),
        lineage: lineage.into_iter().map(String::from).collect(),
        station: None,
    }
}

/// A confluent hybrid carries BOTH parents, round-tripped whole; the
/// single-parent wire form stays byte-identical to LINEAGE-1, and an
/// old single-token file reads unchanged.
#[test]
fn both_parents_round_trip_and_the_old_form_reads_unchanged() {
    let registry = Registry::at(scratch("roundtrip"));
    registry.create(&line("mother", vec![])).expect("creates");
    registry.create(&line("father", vec![])).expect("creates");
    registry
        .create(&line("hybrid", vec!["mother", "father"]))
        .expect("creates the confluent line");
    let held = registry.load("hybrid").expect("loads");
    assert_eq!(held.lineage, vec!["mother".to_string(), "father".to_string()]);

    // The wire form: one field, space-joined, in append order.
    let root = scratch("wire");
    let registry = Registry::at(root.clone());
    registry.create(&line("hybrid", vec!["mother", "father"])).expect("creates");
    let text = fs::read_to_string(root.join("hybrid.json")).expect("reads");
    assert!(text.contains("\"lineage\":\"mother father\""));

    // A LINEAGE-1 file (single token) reads as a one-parent list.
    fs::write(
        root.join("elder-child.json"),
        "{\"name\":\"Elder Child\",\"sigil\":\"elder-child\",\"species\":\"w\",\"motto\":\"m\",\"axes\":{\"disposition\":\"empiricist\",\"metabolism\":\"biotic\",\"perception\":\"planar\"},\"founder\":\"conductor\",\"lineage\":\"mother\"}\n",
    )
    .expect("writes the old form");
    let old = registry.load("elder-child").expect("loads");
    assert_eq!(old.lineage, vec!["mother".to_string()]);
}

/// The primary chain still resolves as LINEAGE-1 did; the DAG read
/// returns EVERY root — both grandparent lines of a confluent hybrid,
/// sigil-sorted, deterministic.
#[test]
fn the_hybrid_resolves_to_both_heads() {
    let registry = Registry::at(scratch("dag"));
    registry.create(&line("deep-root", vec![])).expect("creates");
    registry.create(&line("far-root", vec![])).expect("creates");
    registry.create(&line("mother", vec!["deep-root"])).expect("creates");
    registry.create(&line("father", vec!["far-root"])).expect("creates");
    registry
        .create(&line("hybrid", vec!["mother", "father"]))
        .expect("creates");

    // Primary chain: first parent per hop — mother's line.
    assert_eq!(registry.resolve_head("hybrid").as_deref(), Some("deep-root"));
    // The DAG: both roots, sorted.
    assert_eq!(
        registry.resolve_heads("hybrid"),
        vec!["deep-root".to_string(), "far-root".to_string()]
    );
    // A headless line is its own single head — lawful.
    assert_eq!(registry.resolve_heads("deep-root"), vec!["deep-root".to_string()]);
}

/// The stops are lawful on the DAG exactly as on the chain: a dangling
/// pointer resolves to the deepest recorded ancestor, and a cycle
/// stops deterministically at first revisit.
#[test]
fn dangling_and_cycles_stop_lawfully() {
    let registry = Registry::at(scratch("stops"));
    registry
        .create(&line("orphan", vec!["unrecorded-past"]))
        .expect("creates");
    assert_eq!(registry.resolve_heads("orphan"), vec!["orphan".to_string()]);

    registry.create(&line("ouro", vec!["boros"])).expect("creates");
    registry.create(&line("boros", vec!["ouro"])).expect("creates");
    let heads = registry.resolve_heads("ouro");
    assert_eq!(heads.len(), 1, "one deterministic stop per cycle");
    assert_eq!(registry.resolve_head("ouro").as_deref(), Some("boros"));
}
