//! The one-chronicle-per-world-per-era law (scribe::record): a
//! divergent resident chronicle is a PRIOR era's world — rotated to a
//! closed counterpart, never clobbered, never appended onto.

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
    scribe::record(&book, "amber-veil", "prime", "pin-civ", &era_one).expect("records");
    // Authority carrier for divergence detection.
    cfe::persist::sync(
        era_one.chronicle(),
        &base.join("worlds/amber-veil/chronicle.tlv"),
    )
    .expect("sync era one");
    let len_one = era_one.chronicle().receipts().len();

    // Era two: a FRESH world, same name, diverging history.
    let mut era_two = World::new();
    era_two.found("second-landing").expect("founds");
    era_two.zone(8, ZoneKind::Habitat).expect("zones");
    era_two.tick();
    era_two.collect().expect("collects");
    scribe::record(&book, "amber-veil", "prime", "pin-civ", &era_two).expect("records");
    cfe::persist::sync(
        era_two.chronicle(),
        &base.join("worlds/amber-veil/chronicle.tlv"),
    )
    .expect("sync era two");

    // Fresh live dir carries era two only.
    let live = cfe::persist::load(&base.join("worlds/amber-veil/chronicle.tlv")).expect("load");
    assert_eq!(live.receipts().len(), era_two.chronicle().receipts().len());

    // Era one survives whole in a closed counterpart — never clobbered.
    let closed: Vec<_> = fs::read_dir(base.join("worlds"))
        .expect("worlds dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("amber-veil--closed-"))
        .collect();
    assert_eq!(closed.len(), 1, "one rotated counterpart");
    let kept = cfe::persist::load(
        &base.join("worlds").join(&closed[0]).join("chronicle.tlv"),
    )
    .expect("closed chronicle tlv");
    assert_eq!(kept.receipts().len(), len_one);

    // Same-era re-record does not rotate again.
    era_two.tick();
    era_two.collect().expect("collects again");
    scribe::record(&book, "amber-veil", "prime", "pin-civ", &era_two).expect("records");
    cfe::persist::sync(
        era_two.chronicle(),
        &base.join("worlds/amber-veil/chronicle.tlv"),
    )
    .expect("sync");
    let closed2: Vec<_> = fs::read_dir(base.join("worlds"))
        .expect("worlds")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("amber-veil--closed-"))
        .collect();
    assert_eq!(closed2.len(), 1, "no second rotation on same era");
}
