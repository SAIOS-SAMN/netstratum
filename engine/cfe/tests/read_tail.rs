//! A reader following a chronicle another process writes: only the new tail
//! is read, a half-written frame waits, a shrunk or rewritten past refuses.

use cfe::dop::ClosureEvent;
use cfe::persist::{read_tail, sync, PersistRefused, TailCursor};
use cfe::vrp::Chronicle;

fn scratch(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("cfe-read-tail-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.join("chronicle.tlv")
}

fn founded(chronicle: &mut Chronicle, n: u64) {
    chronicle.append(
        ClosureEvent::ColonyFounded {
            orb_id: n,
            name: format!("orb-{n}"),
        },
        n,
        vec![1],
    );
}

#[test]
fn the_reader_takes_only_the_new_tail_and_waits_on_a_half_written_frame() {
    let path = scratch("tail");
    let mut chronicle = Chronicle::new();
    for n in 1..=3 {
        founded(&mut chronicle, n);
    }
    sync(&chronicle, &path).unwrap();
    let mut cursor = TailCursor::default();
    assert_eq!(read_tail(&path, &mut cursor).unwrap().len(), 3);
    assert_eq!(
        read_tail(&path, &mut cursor).unwrap().len(),
        0,
        "nothing new, nothing read"
    );

    founded(&mut chronicle, 4);
    sync(&chronicle, &path).unwrap();
    let full = std::fs::read(&path).unwrap();
    // a writer mid-frame: the last frame's bytes are not all there yet
    let mut torn = full.clone();
    founded(&mut chronicle, 5);
    sync(&chronicle, &path).unwrap();
    let whole = std::fs::read(&path).unwrap();
    torn.extend_from_slice(&whole[full.len()..whole.len() - 3]);
    std::fs::write(&path, &torn).unwrap();
    let tail = read_tail(&path, &mut cursor).unwrap();
    assert_eq!(
        tail.len(),
        1,
        "the complete frame is taken, the torn one waits"
    );
    std::fs::write(&path, &whole).unwrap();
    assert_eq!(
        read_tail(&path, &mut cursor).unwrap().len(),
        1,
        "the finished frame is read next time"
    );
    assert_eq!(cursor.receipts, 5);
    assert_eq!(cursor.head, chronicle.receipts().last().map(|r| r.own()));
}

#[test]
fn a_shrunk_or_rewritten_past_refuses() {
    let path = scratch("past");
    let mut chronicle = Chronicle::new();
    for n in 1..=3 {
        founded(&mut chronicle, n);
    }
    sync(&chronicle, &path).unwrap();
    let mut cursor = TailCursor::default();
    read_tail(&path, &mut cursor).unwrap();

    // a different history's frame appended past the cursor: it does not chain
    let mut other = Chronicle::new();
    for n in 10..=14 {
        founded(&mut other, n);
    }
    let foreign = {
        let p = scratch("other");
        sync(&other, &p).unwrap();
        std::fs::read(p).unwrap()
    };
    let (tag, value) = cfe::tlv::records(&foreign).last().unwrap();
    let mut forged = std::fs::read(&path).unwrap();
    cfe::tlv::put(tag, value, &mut forged);
    std::fs::write(&path, &forged).unwrap();
    assert!(matches!(
        read_tail(&path, &mut cursor.clone()),
        Err(PersistRefused::Broken { .. })
    ));

    std::fs::write(&path, b"").unwrap();
    assert!(matches!(
        read_tail(&path, &mut cursor),
        Err(PersistRefused::HistoryDiverged { .. })
    ));
}
