//! #36 KERNEL-OBSERVED PLAY pins: every beat chains. A world's spine
//! adopts ONCE at first sight (the pre-chain founding stamped a named
//! debt) and extends every beat after; the judged flag stays 0 at this
//! phase — the hole visible, never hidden (the Phase 3 flip tightens
//! it, U_DIRECTIVES §7). The store round-trips the committed state
//! whole, and a fresh open re-verifies the same chain — the record
//! outlives the process that cut it.

use conductor::{founding_polities, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use mesh::spine::Spine;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "conductor-spine-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_beat_chains_and_the_debt_is_named() {
    let root = scratch("observed");
    let var = root.to_string_lossy().into_owned();
    let registry = Registry::at(format!("{var}/registry"));
    let book = Ledger::at(format!("{var}/worlds"));

    let spec = founding_polities().into_iter().next().expect("a founding spec");
    let world_name = spec.world_name.clone();
    let mut polity = Polity::genesis(spec, &registry, &book);

    for _ in 0..3 {
        polity.cycle(&book, &registry);
    }

    // The chain stands beside the chronicle: one record per beat, the
    // first through the adoption door, every one honestly unjudged.
    let spine_root = book.world_dir(&world_name).join("spine");
    let spine = Spine::open(&spine_root).expect("the spine opens and re-verifies whole");
    assert_eq!(
        spine.records().len(),
        3,
        "three beats, three records — adopted once, extended after"
    );
    assert_eq!(
        spine.records()[0].parent,
        [0u8; 32],
        "the first record is the adoption — the mainline's one birth"
    );
    assert!(
        spine.records().iter().all(|r| !r.judged),
        "kernel-OBSERVED play stamps every hole judged=0, never hidden"
    );

    // The store carries the geometry whole: the head blob decodes to
    // this world's committed state, byte-for-byte through the codec.
    let head = spine.head("").expect("the mainline stands").clone();
    let bytes = spine.fetch(&head.state).expect("the store serves the head");
    assert_eq!(head.schema, bytes[0], "the record repeats the blob's form");
    let decoded = codec::from_state_bytes(&bytes).expect("the blob decodes");
    assert_eq!(decoded, polity.world.committed(), "the chain holds the played state whole");

    // Two more beats: the same chain continues across opens — the
    // arrow holds, and adoption never fires twice.
    for _ in 0..2 {
        polity.cycle(&book, &registry);
    }
    let reopened = Spine::open(&spine_root).expect("re-opens whole");
    assert_eq!(reopened.records().len(), 5, "the chain walked on, no second birth");
}
