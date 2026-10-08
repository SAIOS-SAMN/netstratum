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

fn sent(c: &mut Chronicle, orb: u64, stamp: u64) {
    let amount = cfe::ratio::ratio(1, 1);
    c.append(ClosureEvent::ApertureSent { deal: stamp, from_orb: orb, resource: cfe::dop::Resource::Iron, amount }, stamp, vec![1]);
}

fn assayed(c: &mut Chronicle, orb: u64, stamp: u64) {
    let amount = cfe::ratio::ratio(1, 1);
    c.append(ClosureEvent::AssayStruck { orb_id: orb, element: [7; 32], amount, richness: None }, stamp, vec![1]);
}

#[test]
fn the_annals_keep_what_the_live_world_reads_of_a_drained_prefix() {
    let mut c = Chronicle::new();
    founded(&mut c, 1);
    sent(&mut c, 1, 2);
    assayed(&mut c, 1, 3);
    founded(&mut c, 4);
    sent(&mut c, 1, 5);
    sent(&mut c, 9, 6);
    founded(&mut c, 7);
    let whole: Vec<_> = c.receipts().to_vec();
    c.drain_to(6);
    // history = the drained non-bulk kinds, then everything held
    let history: Vec<_> = c.history().cloned().collect();
    let expect: Vec<_> = whole[..6]
        .iter()
        .filter(|r| !cfe::annals::bulk(r.event()))
        .chain(whole[6..].iter())
        .cloned()
        .collect();
    assert_eq!(history, expect);
    assert_eq!(c.history().rev().next(), whole.last(), "the history walks backward too");
    // every receipt the world sealed is still citable
    for r in &whole {
        assert!(c.sealed(&r.own().0));
    }
    assert!(!c.sealed(&[0; 32]));
    let annals = c.annals();
    assert_eq!(annals.assays(), 1);
    assert_eq!(annals.last_links().get(&1), Some(&5), "the latest aperture stamp per orb");
    assert_eq!(annals.last_links().get(&9), Some(&6));
    assert_eq!(annals.first_digest(), Some(whole[0].own().0));
    // the annals read back whole from their bytes
    let back = cfe::annals::Annals::from_bytes(&annals.to_bytes()).expect("reads");
    assert_eq!(&back, annals);
    let mut longer = annals.to_bytes();
    longer.push(0);
    assert!(cfe::annals::Annals::from_bytes(&longer).is_none(), "trailing bytes refuse");
}

#[test]
fn a_tail_reads_back_from_its_byte_offset() {
    use cfe::persist::{read_tail, TailCursor};
    let path = scratch("tail");
    let mut c = Chronicle::new();
    for n in 1..=6 {
        founded(&mut c, n);
    }
    let mut cursor = SyncCursor::default();
    sync_from(&c, &path, &mut cursor).unwrap();
    c.drain_to(4);
    let held_bytes: u64 = c.receipts().iter().map(|r| r.wire_bytes().len() as u64).sum();
    let mut tail = TailCursor { bytes: cursor.bytes() - held_bytes, receipts: c.base(), head: Some(c.base_parent()) };
    let read = read_tail(&path, &mut tail).unwrap();
    let mut back = Chronicle::from_tail(c.base(), c.base_parent(), read).unwrap();
    back.set_annals(c.annals().clone());
    assert_eq!(back.receipts(), c.receipts());
    assert_eq!(back.total(), c.total());
    founded(&mut back, 7);
    founded(&mut c, 7);
    assert_eq!(back.receipts().last(), c.receipts().last());
    // the resumed tail syncs on from a cursor standing at the file's end
    let mut at = SyncCursor::proved(6, tail.bytes);
    assert_eq!(sync_from(&back, &path, &mut at).unwrap(), 1);
    assert_eq!(load(&path).unwrap().receipts().last(), c.receipts().last());
}
