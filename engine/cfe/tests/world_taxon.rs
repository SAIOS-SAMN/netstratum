//! KERNEL-TAXON pins (15 §4): the seal is once-forever, in-vocabulary
//! or refused, moves no value, and rides replay, resume, and the
//! snapshot codec across both schema versions. Coder-03's coverage
//! ({containment, exactness}) seals at their amendment; these pins
//! hold the deed's own law.

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::world::{replay, ActionRefused, World};

#[test]
fn the_seal_is_once_and_in_vocabulary() {
    let mut world = World::new();
    world.found("taxon-hold").expect("founds");

    // Outside the ratified vocabularies: refused with both codes named.
    assert!(matches!(
        world.seal_taxon(0, 5),
        Err(ActionRefused::TaxonUnknown { class: 0, clade: 5 })
    ));
    assert!(matches!(
        world.seal_taxon(8, 5),
        Err(ActionRefused::TaxonUnknown { class: 8, clade: 5 })
    ));
    assert!(matches!(
        world.seal_taxon(2, 19),
        Err(ActionRefused::TaxonUnknown { class: 2, clade: 19 })
    ));

    // The lawful seal: geodic-lithovore (2), tardigrade (5).
    world.seal_taxon(2, 5).expect("seals");
    assert_eq!(world.taxon(), Some((2, 5)));
    assert!(matches!(
        world.seal_taxon(2, 5),
        Err(ActionRefused::TaxonAlreadySealed)
    ));

    // One receipt; no value moved anywhere.
    let seals = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|receipt| matches!(receipt.event, ClosureEvent::TaxonSealed { .. }))
        .count();
    assert_eq!(seals, 1, "one seal per star, forever");

    // SEAL B whole: replay and resume carry the taxon.
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries it");
}

/// The codec seam: a sealed taxon rides the v2 snapshot byte-exact,
/// and a v1 snapshot (no taxon field) still reads whole — every old
/// commitment replays.
#[test]
fn the_taxon_rides_the_snapshot_and_v1_still_reads() {
    let mut world = World::new();
    world.found("codec-hold").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.seal_taxon(5, 15).expect("seals");
    let committed = world.committed();

    let bytes = codec::state_bytes(&committed);
    let back = codec::from_state_bytes(&bytes).expect("v2 reads whole");
    assert_eq!(back, committed, "the taxon rides the snapshot");
    assert_eq!(back.taxon, Some((5, 15)));

    // A v1 stream is the same bytes with the old schema byte and no
    // trailing taxon field: it must still read, taxon lawfully None.
    let unsealed = {
        let mut plain = World::new();
        plain.found("plain-hold").expect("founds");
        plain.committed()
    };
    let v2 = codec::state_bytes(&unsealed);
    let mut v1 = v2.clone();
    v1[0] = 1; // SCHEMA_V1
    v1.truncate(v1.len() - 1); // drop the trailing presence byte
    let old = codec::from_state_bytes(&v1).expect("v1 reads whole");
    assert_eq!(old.taxon, None, "v1 carries no taxon — lawfully unrecorded");
}
