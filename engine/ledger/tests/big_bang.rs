//! docs/58 §3 — a universe's Big Bang: first on an empty ledger, a late
//! re-founding on a used one, written once; both events round-trip the wire.

use cfe::dop::ClosureEvent;
use ledger::Ledger;

fn scratch(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("ledger-bigbang-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[test]
fn the_big_bang_is_first_and_written_once() {
    let root = scratch("first");
    let book = Ledger::at(&root);
    assert_eq!(book.universe_seed(), None);
    let (seed, fresh) = book.found_universe([1; 32], [2; 32], 0).unwrap();
    assert!(fresh);
    assert_eq!(book.universe_seed(), Some(seed));
    let (again, fresh) = book.found_universe([9; 32], [9; 32], 5).unwrap();
    assert!(!fresh, "a universe is founded once");
    assert_eq!(again, seed);
    let ledger = cfe::persist::load(&root.join("convergence.tlv")).unwrap();
    assert_eq!(ledger.receipts().len(), 1);
    assert!(matches!(ledger.receipts()[0].event(), ClosureEvent::UniverseFounded { over: None, roster, .. } if *roster == [1; 32]));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_used_ledger_is_refounded_late_and_says_so() {
    let root = scratch("late");
    let book = Ledger::at(&root);
    let zero = cfe::ratio::ratio(0, 1);
    book.observe_galaxy("someone", "someone", (&zero, &zero, &zero), 3).unwrap();
    let head = cfe::persist::load(&root.join("convergence.tlv")).unwrap().receipts().last().unwrap().own().0;
    book.found_universe([1; 32], [2; 32], 4).unwrap();
    let ledger = cfe::persist::load(&root.join("convergence.tlv")).unwrap();
    let last = ledger.receipts().last().unwrap();
    assert!(matches!(last.event(), ClosureEvent::UniverseFounded { over: Some(h), .. } if *h == head));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn both_events_round_trip_the_wire() {
    use cfe::tlv;
    for event in [
        ClosureEvent::UniverseFounded { topology: "torus".into(), span: cfe::ratio::ratio(64, 1), roster: [3; 32], matter: [4; 32], over: None },
        ClosureEvent::UniverseFounded { topology: "torus".into(), span: cfe::ratio::ratio(64, 1), roster: [3; 32], matter: [4; 32], over: Some([5; 32]) },
        ClosureEvent::UniverseCited { universe: [6; 32] },
        ClosureEvent::AncientArose { sigil: "tallykeep".into(), tree: 5, living: 41 },
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = tlv::records(&bytes).next().unwrap();
        assert_eq!(ClosureEvent::from_wire(tag, value), Some(event));
    }
}
