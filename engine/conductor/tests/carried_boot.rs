//! The memory bound (slice 2): a world carried at a snapshot drains what the
//! snapshot holds to its annals, and boots back reading ONLY the chronicle
//! tail the snapshot names — the same state, the same history, the same
//! position, and a sync that continues the file without re-reading it.

use cfe::dop::ZoneKind;
use cfe::persist::{load, sync_from, SyncCursor};
use cfe::world::World;
use conductor::snapshot::{self, Resumed};

fn grown() -> World {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(4, ZoneKind::Extractor).expect("affordable");
    for _ in 0..3 {
        world.tick();
    }
    world.collect().expect("nets settle");
    world
}

#[test]
fn a_carried_world_boots_from_its_tail_alone() {
    let dir = std::env::temp_dir().join(format!("conductor-carried-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let carrier = dir.join("chronicle.tlv");

    let mut world = grown();
    let mut cursor = SyncCursor::default();
    sync_from(world.chronicle(), &carrier, &mut cursor).unwrap();
    let total = world.chronicle().total();
    world.drain_chronicle_to(total - 1);
    assert_eq!(world.chronicle().receipts().len(), 1, "only the head stays held");
    snapshot::write(&dir, &world, cursor.bytes()).unwrap();

    // The world plays on past its snapshot; the tail reaches the file.
    world.zone(9, ZoneKind::Habitat).expect("affordable");
    sync_from(world.chronicle(), &carrier, &mut cursor).unwrap();

    let (back, resumed, mut sync) = snapshot::boot(&dir).expect("boots");
    assert_eq!(resumed, Resumed::Carried { tail: 1 });
    assert_eq!(back.chronicle().base(), total - 1, "the drained prefix stays on disk");
    assert_eq!(back.chronicle().total(), world.chronicle().total());
    assert_eq!(back.chronicle().receipts(), world.chronicle().receipts());
    assert!(back.chronicle().history().eq(world.chronicle().history()), "the same history");
    assert_eq!(back.committed(), world.committed(), "the same carried state");
    assert_eq!(sync.receipts() as u64, world.chronicle().total());

    // Synced on from the boot's cursor, the file continues whole.
    let mut back = back;
    back.zone(14, ZoneKind::Extractor).expect("affordable");
    assert_eq!(sync_from(back.chronicle(), &carrier, &mut sync).unwrap(), 1);
    let on_disk = load(&carrier).unwrap();
    assert_eq!(on_disk.total(), back.chronicle().total());
    assert_eq!(on_disk.receipts().last(), back.chronicle().receipts().last());
}

#[test]
fn the_digests_append_and_are_never_rewritten() {
    let dir = std::env::temp_dir().join(format!("conductor-digests-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let carrier = dir.join("chronicle.tlv");
    let digests = dir.join("annals.dig");

    let mut world = grown();
    let mut cursor = SyncCursor::default();
    sync_from(world.chronicle(), &carrier, &mut cursor).unwrap();
    world.drain_chronicle_to(world.chronicle().total() - 1);
    snapshot::write(&dir, &world, cursor.bytes()).unwrap();
    let first = std::fs::read(&digests).unwrap();
    assert_eq!(first.len(), world.chronicle().annals().digests().len() * 32);

    // A second carry appends only the newly drained hashes.
    world.zone(9, ZoneKind::Habitat).expect("affordable");
    world.zone(14, ZoneKind::Extractor).expect("affordable");
    sync_from(world.chronicle(), &carrier, &mut cursor).unwrap();
    world.drain_chronicle_to(world.chronicle().total() - 1);
    snapshot::write(&dir, &world, cursor.bytes()).unwrap();
    let second = std::fs::read(&digests).unwrap();
    assert_eq!(&second[..first.len()], &first[..], "nothing already written moves");
    assert_eq!(second.len(), world.chronicle().annals().digests().len() * 32);

    // A file left longer than its snapshot (a crash before the snapshot
    // landed) still boots: the snapshot takes only its own count.
    let mut longer = second.clone();
    longer.extend_from_slice(&[9u8; 32]);
    std::fs::write(&digests, &longer).unwrap();
    let (back, _, _) = snapshot::boot(&dir).expect("boots");
    assert_eq!(back.chronicle().annals(), world.chronicle().annals());
    assert!(back.chronicle().sealed(&world.chronicle().annals().digests()[0]));
    // and the next carry cuts it back before appending
    snapshot::write(&dir, &world, cursor.bytes()).unwrap();
    assert_eq!(std::fs::read(&digests).unwrap(), second);
}
