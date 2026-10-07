//! The seat card: where a line not compiled in was founded, recorded once
//! and read back whole, so a restart resumes it instead of founding it
//! again under a derived name.

use ledger::registry::{Registry, Seat};
use ratio::ratio;

fn scratch(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("ledger-seat-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn seat() -> Seat {
    Seat {
        world: "bloom-call".into(),
        planet: "callfall".into(),
        colony: "bloomcall-landing".into(),
        position: (ratio(7, 2), ratio(-3, 1), ratio(1, 2)),
        policy: "industry".into(),
    }
}

#[test]
fn a_seat_reads_back_whole_and_is_written_once() {
    let root = scratch("once");
    let reg = Registry::at(&root);
    assert_eq!(reg.founded_at("bloomcall"), None, "an unseated line reads unrecorded");
    reg.write_founded_at("bloomcall", &seat()).unwrap();
    assert_eq!(reg.founded_at("bloomcall"), Some(seat()));
    let mut moved = seat();
    moved.world = "elsewhere".into();
    assert!(reg.write_founded_at("bloomcall", &moved).is_err(), "a seat is never moved");
    assert_eq!(reg.founded_at("bloomcall"), Some(seat()));
    assert!(reg.list().is_empty(), "a seat card is not a civilization");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_seat_missing_a_field_reads_unrecorded_never_partial() {
    let root = scratch("partial");
    let reg = Registry::at(&root);
    reg.write_founded_at("cinder", &seat()).unwrap();
    let path = root.join("cinder.seat");
    let bytes = std::fs::read(&path).unwrap();
    // drop the trailing record (the last coordinate)
    let mut cut = Vec::new();
    let mut last = 0;
    for (tag, value) in ledger::card::records(&bytes) {
        last = tag;
        let _ = value;
    }
    for (tag, value) in ledger::card::records(&bytes) {
        if tag != last {
            ledger::card::put(tag, value, &mut cut);
        }
    }
    std::fs::write(&path, cut).unwrap();
    assert_eq!(reg.founded_at("cinder"), None);
    let _ = std::fs::remove_dir_all(&root);
}
