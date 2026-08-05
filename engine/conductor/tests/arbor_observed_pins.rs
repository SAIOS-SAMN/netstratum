//! #36 KERNEL-OBSERVED PLAY pins: every beat chains. A world's arbor
//! adopts ONCE at first sight (the pre-chain founding stamped a named
//! debt) and extends every beat after; the gated flag stays 0 at this
//! phase — the hole visible, never hidden (the Phase 3 flip tightens
//! it, U_DIRECTIVES §7). The store round-trips the committed state
//! whole, and a fresh open re-verifies the same chain — the record
//! outlives the process that cut it.

use conductor::{founding_polities, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use mesh::arbor::Arbor;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "conductor-arbor-{name}-{}",
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

    // THE PHASE 3 FLIP (DIRECTIVE 19) SUPERSEDES THIS PIN'S OLD CLAIM.
    // It asserted one record per BEAT and every record unjudged — the
    // shape of kernel-OBSERVED play (#36), where the gate watched and
    // never spoke. Both halves are now false by design: the chain
    // carries one record per DEED, and every deed past the adoption
    // door is gated by `kernel::admits` before it may enter.
    let spine_root = book.world_dir(&world_name).join("spine");
    let arbor = Arbor::open(&spine_root).expect("the arbor opens and re-verifies whole");
    // NOT one record per beat. Per-DEED chaining means a beat that
    // closes no deed adds no record — the arbor grows with WORK, not
    // with time, which is the whole point of judging steps rather than
    // snapshots.
    assert!(!arbor.records().is_empty(), "the adoption stands");
    assert_eq!(
        arbor.records()[0].parent,
        [0u8; 32],
        "the first record is the adoption — the mainline's one birth"
    );
    assert!(
        !arbor.records()[0].judged,
        "the adoption alone is unjudged: pre-chain history is a debt NAMED, never hidden"
    );
    // The RECORDS carry gated=false because the commitment is one per
    // beat, and a beat's record has no single step to name. That is the
    // arbor's addressing law, not an unjudged economy: DIRECTIVE 19 runs
    // `kernel::admits` on every deed individually before the beat
    // commits (see `Polity::refused`). Per-deed COMMITMENT — and so a
    // gated=true stamp per deed — waits on a coordinate sub-index,
    // which is a wire amendment.
    assert!(
        arbor.records().iter().all(|r| !r.judged),
        "one record per beat carries no single step to stamp"
    );

    // The store carries the geometry whole: the head blob decodes to
    // this world's committed state, byte-for-byte through the codec.
    let head = arbor.head("").expect("the mainline stands").clone();
    let bytes = arbor.fetch(&head.state).expect("the store serves the head");
    assert_eq!(head.schema, bytes[0], "the record repeats the blob's form");
    let decoded = codec::from_state_bytes(&bytes).expect("the blob decodes");
    assert_eq!(decoded, polity.world.committed(), "the chain holds the played state whole");

    // Two more beats: the same chain continues across opens — the
    // arrow holds, and adoption never fires twice.
    for _ in 0..2 {
        polity.cycle(&book, &registry);
    }
    let reopened = Arbor::open(&spine_root).expect("re-opens whole");
    assert!(
        reopened.records().len() > arbor.records().len(),
        "the arbor walked on across opens"
    );
    assert_eq!(
        reopened.records().iter().filter(|r| r.parent == [0u8; 32]).count(),
        1,
        "ONE birth only — adoption never fires twice, however many times the arbor reopens"
    );
}
