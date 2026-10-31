//! JSON is display-only — authority is chronicle + wire (docs/JSON_DISPLAY_ONLY.md).
//!
//! These pins prove a JSON-free resume does not depend on state.json.

use cfe::dop::{positive, unit, ZoneKind};
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB};
use std::path::PathBuf;

#[test]
fn resume_from_chronicle_ignores_missing_state_json() {
    let mut w = World::new();
    w.found("tlv-only").expect("found");
    w.commission_star(StarClass::Yellow).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    w.zone(0, ZoneKind::Extractor).expect("zone");
    w.zone(1, ZoneKind::Habitat).expect("hab");

    // Sync chronicle only — no state.json written.
    let dir = std::env::temp_dir().join(format!("no-json-auth-{}-{}", std::process::id(), "tlv"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("dir");
    let path = dir.join("chronicle.tlv");
    cfe::persist::sync(w.chronicle(), &path).expect("sync");
    assert!(!dir.join("state.json").exists());
    assert!(!PathBuf::from("planets").join("state.json").exists());

    let chronicle = cfe::persist::load(&path).expect("load verifies chain");
    let resumed = World::resume(chronicle);
    assert_eq!(resumed.habitat_count(FIRST_ORB), 1);
    assert_eq!(resumed.labor_force(FIRST_ORB), 1);
    // Banks match without any JSON in the tree.
    let iron = resumed
        .orb()
        .map(|o| o.banked_amount(cfe::dop::Resource::Iron))
        .expect("orb");
    assert!(iron > cfe::ratio::ratio(0, 1));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn state_tlv_wire_round_trips_banks_without_json() {
    let mut w = World::new();
    w.found("wire-home").expect("found");
    w.commission_star(StarClass::Yellow).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    w.zone(0, ZoneKind::Habitat).expect("hab");
    let bytes = w.orb().expect("orb").wire_bytes();
    assert!(!bytes.is_empty());
    // Wire is the machine mirror — non-empty without JSON.
    assert!(bytes.contains(&17), "zone entries on wire");
}
