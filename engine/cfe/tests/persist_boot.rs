//! P2-PERSIST witnesses — the two-boot proof (the v0.4 daemon's
//! standard): boot 2 reloads the committed core byte-exact and
//! re-earns nothing; the file is literally append-only; a tampered or
//! divergent file refuses typed; the workspace vanishes across the
//! process boundary because nothing receipted it.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{Resource, ZoneKind};
use cfe::persist::{load, sync, PersistRefused};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A per-test scratch path (deterministic name, cleaned before use).
fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir()
        .join(format!("netstratum-persist-{}-{name}", std::process::id()))
        .join("chronicle.tlv");
    let _ = fs::remove_dir_all(path.parent().expect("has parent"));
    path
}

fn a_first_era() -> World {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    world
}

#[test]
fn the_two_boot_witness() {
    let path = scratch("two-boot");
    // BOOT 1: play an era, sync, walk two more ticks WITHOUT collecting.
    let mut boot1 = a_first_era();
    let receipts_1 = boot1.chronicle().receipts().len();
    let appended = sync(boot1.chronicle(), &path).expect("first sync");
    assert_eq!(appended, receipts_1, "the whole era frames");
    boot1.tick();
    boot1.tick();
    assert!(!boot1.accrued().is_empty(), "boot 1 ends with workspace in the field");
    let committed_1 = boot1.committed();
    drop(boot1);

    // BOOT 2: load, resume — byte-exact core, empty workspace, clock at
    // the last PROVEN stamp (4 — the un-receipted ticks 5,6 are gone).
    let loaded = load(&path).expect("the chronicle returns");
    assert_eq!(loaded.verify_chain(), ChainRead::Intact);
    let mut boot2 = World::resume(loaded);
    assert_eq!(boot2.committed(), committed_1, "the committed core reloads exactly");
    assert_eq!(boot2.now(), 4, "the clock resumes at the proven stamp");
    assert!(boot2.accrued().is_empty(), "what had no receipt did not persist");
    assert_eq!(
        boot2.orb().expect("stands").banked_amount(Resource::Iron),
        ratio(15, 1),
        "banked survives; the two uncollected ticks' accrual does not"
    );

    // The second era continues the SAME chain: zone, sync appends ONLY
    // the tail, and boot 3 replays both eras whole.
    boot2.zone(9, ZoneKind::Habitat).expect("era two builds");
    let size_before = fs::metadata(&path).expect("exists").len();
    let appended = sync(boot2.chronicle(), &path).expect("second sync");
    assert_eq!(appended, 1, "only the tail appends");
    assert!(fs::metadata(&path).expect("exists").len() > size_before, "the file only grows");
    let boot3 = World::resume(load(&path).expect("returns again"));
    assert_eq!(boot3.committed(), boot2.committed(), "era two persisted whole");
    assert_eq!(
        replay(boot3.chronicle()),
        boot3.committed(),
        "both eras replay from the one chain"
    );
}

#[test]
fn a_tampered_file_refuses_with_its_break_named() {
    let path = scratch("tamper");
    let world = a_first_era();
    sync(world.chronicle(), &path).expect("syncs");
    let mut bytes = fs::read(&path).expect("reads");
    // Flip one byte mid-file (inside a receipt's content).
    let mid = bytes.len() / 2;
    bytes[mid] ^= 0x01;
    fs::write(&path, &bytes).expect("writes");
    match load(&path) {
        Err(PersistRefused::Broken { .. }) | Err(PersistRefused::Malformed { .. }) => {}
        other => panic!("a tampered chronicle must refuse typed, got {other:?}"),
    }
}

#[test]
fn a_divergent_file_is_never_clobbered() {
    let path = scratch("diverge");
    let world = a_first_era();
    sync(world.chronicle(), &path).expect("syncs the long era");
    // A DIFFERENT, shorter world tries to sync onto the same file.
    let mut other = World::new();
    other.found("impostor").expect("genesis");
    match sync(other.chronicle(), &path) {
        Err(PersistRefused::HistoryDiverged { .. }) => {}
        other => panic!("divergent history must refuse typed, got {other:?}"),
    }
    // The file is untouched: the original era still loads intact.
    let survived = load(&path).expect("survives");
    assert_eq!(survived.receipts().len(), world.chronicle().receipts().len());
}

#[test]
fn receipt_frames_round_trip_and_skip_unknown_tags() {
    let world = a_first_era();
    for receipt in world.chronicle().receipts() {
        let bytes = receipt.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one frame");
        let decoded = cfe::vrp::Receipt::from_wire(tag, value).expect("decodes");
        assert_eq!(&decoded, receipt, "the frame carries the receipt whole");
    }
    // SEAL A at the file layer: a future frame among the receipts is
    // skipped, and the chain still verifies.
    let path = scratch("unknown-tag");
    sync(world.chronicle(), &path).expect("syncs");
    let mut bytes = fs::read(&path).expect("reads");
    cfe::tlv::put(200, b"a-future-frame", &mut bytes);
    fs::write(&path, &bytes).expect("writes");
    let loaded = load(&path).expect("unknown frames skip, never crash");
    assert_eq!(loaded.verify_chain(), ChainRead::Intact);
}

#[test]
fn a_second_era_acts_with_resumed_authority() {
    // Entities persist too: an agent granted in era one still holds
    // exactly its grant in era two — and nothing more.
    let path = scratch("authority");
    let mut boot1 = a_first_era();
    let bot = boot1.register_agent("harvest-bot").expect("registers");
    boot1.grant(bot, cfe::ipsal::Scope::Collect).expect("grants");
    sync(boot1.chronicle(), &path).expect("syncs");
    let mut boot2 = World::resume(load(&path).expect("returns"));
    boot2.tick();
    boot2.collect_as(bot, FIRST_ORB).expect("the grant survived the boundary");
    assert!(
        boot2.zone_as(bot, FIRST_ORB, 1, ZoneKind::Extractor).is_err(),
        "and only the grant survived"
    );
    let _ = SOVEREIGN;
}
