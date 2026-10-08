//! A chronicle that holds only a TAIL (the rest drained to the annals on
//! disk after a snapshot carried their state) numbers, chains, verifies and
//! syncs exactly as the whole one would.

use cfe::dop::ClosureEvent;
use cfe::persist::{load, sync, sync_from, PersistRefused, SyncCursor};
use cfe::vrp::{ChainRead, Chronicle};

fn founded(c: &mut Chronicle, n: u64) {
    c.append(ClosureEvent::ColonyFounded { orb_id: n, name: format!("orb-{n}") }, n, vec![1]);
}

fn scratch(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("cfe-drain-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.join("chronicle.tlv")
}

#[test]
fn a_drained_chronicle_numbers_and_chains_as_the_whole_one() {
    let (mut whole, mut tail) = (Chronicle::new(), Chronicle::new());
    for n in 1..=6 {
        founded(&mut whole, n);
        founded(&mut tail, n);
    }
    let drained = tail.drain_to(4);
    assert_eq!(drained.len(), 4);
    assert_eq!((tail.base(), tail.receipts().len(), tail.total()), (4, 2, 6));
    assert_eq!(tail.verify_chain(), ChainRead::Intact);
    for n in 7..=9 {
        founded(&mut whole, n);
        founded(&mut tail, n);
    }
    assert_eq!(tail.receipts().last(), whole.receipts().last(), "the same sequence, parent and hash");
    // never drains the last receipt
    assert_eq!(tail.drain_to(100).len(), 4, "all but the last of the 5 held");
    assert_eq!(tail.receipts().len(), 1);
    assert_eq!(tail.receipts().last(), whole.receipts().last());
}

#[test]
fn a_tail_syncs_onto_its_annals_and_refuses_a_file_without_them() {
    let path = scratch("sync");
    let mut c = Chronicle::new();
    for n in 1..=5 {
        founded(&mut c, n);
    }
    sync(&c, &path).unwrap();
    c.drain_to(4);
    for n in 6..=8 {
        founded(&mut c, n);
    }
    assert_eq!(sync(&c, &path).unwrap(), 3, "only the new receipts are written");
    let on_disk = load(&path).unwrap();
    assert_eq!(on_disk.receipts().len(), 8);
    assert_eq!(on_disk.receipts().last(), c.receipts().last());

    let mut cursor = SyncCursor::default();
    founded(&mut c, 9);
    assert_eq!(sync_from(&c, &path, &mut cursor).unwrap(), 1);
    founded(&mut c, 10);
    assert_eq!(sync_from(&c, &path, &mut cursor).unwrap(), 1, "the cursor counts absolute receipts");

    // a file that lacks the drained prefix cannot take the tail
    let bare = scratch("bare");
    assert!(matches!(sync(&c, &bare), Err(PersistRefused::HistoryDiverged { .. })));
}
