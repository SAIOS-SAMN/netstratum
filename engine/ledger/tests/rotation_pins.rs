//! The one-chronicle-per-world-per-era law (Ledger::record): a
//! divergent resident chronicle is a PRIOR era's world — rotated to a
//! closed sibling, never clobbered, never appended onto.

use std::fs;
use std::path::PathBuf;

use cfe::dop::ZoneKind;
use cfe::world::World;
use ledger::Ledger;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rotation-pin-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// Two eras founding the SAME world name: era two's record rotates era
/// one's dir aside and writes a fresh, intact chronicle. Both survive.
#[test]
fn a_recurring_world_name_rotates_the_prior_era_aside() {
    let base = scratch("recur");
    let book = Ledger::at(base.join("worlds"));

    // Era one: found, build, tick — recorded.
    let mut era_one = World::new();
    era_one.found("first-landing").expect("founds");
    era_one.zone(0, ZoneKind::Extractor).expect("zones");
    era_one.tick();
    era_one.collect().expect("collects");
    book.record("amber-veil", "prime", "pin-civ", &era_one).expect("records");
    let lines_one = fs::read_to_string(base.join("worlds/amber-veil/chronicle.jsonl"))
        .expect("era one chronicle")
        .lines()
        .count();

    // Era two: a FRESH world, same name, diverging history.
    let mut era_two = World::new();
    era_two.found("second-landing").expect("founds");
    era_two.zone(8, ZoneKind::Habitat).expect("zones");
    era_two.tick();
    era_two.collect().expect("collects");
    book.record("amber-veil", "prime", "pin-civ", &era_two).expect("records");

    // The fresh file carries ONLY era two, byte-consistent.
    let fresh = fs::read_to_string(base.join("worlds/amber-veil/chronicle.jsonl"))
        .expect("fresh chronicle");
    assert_eq!(fresh.lines().count(), era_two.chronicle().receipts().len());
    assert!(fresh.contains("second-landing"));
    assert!(!fresh.contains("first-landing"));

    // Era one survives whole in a closed sibling — never clobbered.
    let closed: Vec<_> = fs::read_dir(base.join("worlds"))
        .expect("worlds dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("amber-veil--closed-"))
        .collect();
    assert_eq!(closed.len(), 1, "one rotated sibling");
    let kept = fs::read_to_string(
        base.join("worlds").join(&closed[0]).join("chronicle.jsonl"),
    )
    .expect("closed chronicle");
    assert_eq!(kept.lines().count(), lines_one);
    assert!(kept.contains("first-landing"));

    // Same-era re-record still appends incrementally (no rotation).
    era_two.tick();
    era_two.collect().expect("collects again");
    book.record("amber-veil", "prime", "pin-civ", &era_two).expect("records");
    let appended = fs::read_to_string(base.join("worlds/amber-veil/chronicle.jsonl"))
        .expect("appended chronicle");
    assert_eq!(appended.lines().count(), era_two.chronicle().receipts().len());
}
