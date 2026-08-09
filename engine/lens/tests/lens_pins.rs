//! LENS pins (#37 opening move): the display's committed half
//! re-serves from the chain byte-for-byte — the export is deletable
//! because the lens can always re-derive it. An unborn arbor refuses
//! typed; nothing here reads a ledger file except to prove the
//! equivalence.

use cfe::ratio::ratio;
use cfe::world::World;
use ledger::Ledger;
use mesh::arbor::{Coordinate, Arbor};
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("lens-pin-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn played_world() -> World {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones");
    for _ in 0..8 {
        world.tick();
    }
    world.collect().expect("banks");
    world
}

#[test]
fn the_display_re_serves_from_the_chain_byte_for_byte() {
    let root = scratch("reserve");
    let world = played_world();

    // The display half writes its export (state.tlv only).
    let book = Ledger::at(root.join("worlds"));
    scribe::record(&book, "meridian", "meridian-prime", "test-sigil", &world)
        .expect("the display records");
    let display = fs::read(
        book.world_dir("meridian")
            .join("planets")
            .join("meridian-prime")
            .join("state.tlv"),
    )
    .expect("the export exists");

    // The chain holds the same truth: adopt the committed state.
    let mut arbor = Arbor::open(root.join("spine")).expect("opens");
    let coordinate = Coordinate {
        branch: String::new(),
        s: [ratio(0, 1), ratio(0, 1), ratio(0, 1)],
        g: ratio(0, 1),
        t: world.now(),
    };
    arbor
        .adopt(coordinate, &codec::state_bytes(&world.committed()))
        .expect("adopts");

    // The lens projects the export's bytes FROM the chain.
    let projected = lens::state_tlv(&arbor)
        .expect("the lens reads")
        .expect("the world holds an orb");
    assert_eq!(projected, display, "the export re-derives byte-for-byte — deletable");

    // And the head decodes whole.
    let committed = lens::committed(&arbor).expect("projects");
    assert_eq!(committed, world.committed(), "the head projection is the played state");
}

/// Resume from chronicle.tlv re-derives committed state (no state.json).
#[test]
fn the_rich_rendering_re_serves_from_the_chronicle() {
    let root = scratch("rich");
    let world = played_world();

    let book = Ledger::at(root.join("worlds"));
    scribe::record(&book, "meridian", "meridian-prime", "test-sigil", &world)
        .expect("the display records");
    let tlv_path = book
        .world_dir("meridian")
        .join("planets")
        .join("meridian-prime")
        .join("state.tlv");
    let display = fs::read(&tlv_path).expect("state.tlv exists");
    assert!(!tlv_path.with_extension("json").exists());

    let chronicle_path = root.join("chronicle.tlv");
    cfe::persist::sync(world.chronicle(), &chronicle_path).expect("syncs");
    let resumed = lens::world(&chronicle_path).expect("resumes by replay");
    assert_eq!(resumed.committed(), world.committed());
    // Wire re-derives from resumed orb.
    let projected = resumed.orb().expect("orb").wire_bytes();
    assert_eq!(projected, display, "state.tlv re-derives byte-for-byte");
}

#[test]
fn an_unborn_spine_refuses_typed() {
    let root = scratch("unborn");
    let arbor = Arbor::open(root.join("spine")).expect("opens empty");
    assert!(matches!(lens::committed(&arbor), Err(lens::LensRefused::Unborn)));
}
