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
    // ABOVE THE CEILING, DERIVED — never a hand-picked number. This
    // probe read clade 19 and expected a refusal, which encoded the
    // ceiling of the day: `15_TAXON` §3 then seated 19 (draconic) and 20
    // (metashifting) by operator ruling, and the pin would have kept
    // insisting a ruled clade was unknown. Reading one past the ceiling
    // keeps the property (out-of-vocabulary refuses) without pinning
    // WHICH codes are in it.
    let past = cfe::dop::clade_ceiling() + 1;
    assert!(matches!(
        world.seal_taxon(2, past),
        Err(ActionRefused::TaxonUnknown { class: 2, clade })
            if clade == past
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
    let back = codec::from_state_bytes(&bytes).expect("v3 reads whole");
    assert_eq!(back, committed, "the taxon rides the snapshot");
    assert_eq!(back.taxon, Some((5, 15)));

    // A v1 stream is the same bytes with the old schema byte and no
    // trailing taxon field: it must still read, taxon lawfully None.
    let unsealed = {
        let mut plain = World::new();
        plain.found("plain-hold").expect("founds");
        plain.committed()
    };
    // EVERY SCHEMA SINCE v1 APPENDS, so an old stream is this one with
    // each later era's trailing run removed. This fixture holds no
    // techs, so v4's magnitude run is empty and contributes nothing;
    // what must come off is v5's pact count (4 bytes, #42), v3's seat
    // count (1) and v2's taxon presence byte (1).
    //
    // The hand-truncation is what makes this pin BITE: it fails the
    // moment a new field is added anywhere but the tail, which is
    // exactly how #42 was caught inserting its block mid-stream
    // between the routes and the practices.
    let current = codec::state_bytes(&unsealed);
    let mut v1 = current.clone();
    v1[0] = 1; // SCHEMA_V1
    v1.truncate(v1.len() - 6);
    let old = codec::from_state_bytes(&v1).expect("v1 reads whole");
    assert_eq!(old.taxon, None, "v1 carries no taxon — lawfully unrecorded");
}

/// **EVERY ARCHETYPE CARRIES A SIGIL AND A WORD** (operator ruling
/// 2026-08-05: *each archetype needs a sigil as well as the
/// vocabulary*).
///
/// A sigil is a symbol with a DEFINITION — which is why the word is
/// lawful here and in `cfe::query::Sigil` alike. This pin holds the
/// three properties that make it one rather than an ornament: every
/// class in the ratified range has one, no two classes share one, and
/// each has a word beside it.
#[test]
fn every_archetype_carries_a_sigil_and_a_word() {
    let mut seen: Vec<&str> = Vec::new();
    for class in 0..=cfe::dop::class_ceiling() {
        let sigil = cfe::dop::archetype_sigil(class);
        let word = cfe::dop::archetype_word(class);
        assert_ne!(sigil, "?", "class {class} has no sigil");
        assert_ne!(word, "unknown", "class {class} has no word");
        assert!(
            !seen.contains(&sigil),
            "class {class} reuses the sigil {sigil} — a mark that names two \
             things defines neither"
        );
        seen.push(sigil);
    }
    // Past the ratified ceiling there is nothing to mark.
    assert_eq!(cfe::dop::archetype_sigil(cfe::dop::class_ceiling() + 1), "?");

    // DISCREDITED PHYSICS IS NOT VOCABULARY. Class 6 read
    // "etherial-field" until 2026-08-05 — the luminiferous aether,
    // measured out of existence by Michelson-Morley. The mechanism is a
    // plasma and the word now says so.
    assert_eq!(cfe::dop::archetype_word(6), "plasma-field");
}

/// **THE RULED CLADES ARE SEALABLE** — the ceiling admits what the
/// operator seated.
///
/// `15_TAXON` §3 seated clade 19 (draconic) and clade 20 (metashifting)
/// on 2026-08-01, and `clade_ceiling` stayed at 18 until 2026-08-05. For
/// those four days `seal_taxon` refused both: a landed ruling that the
/// deed recording it could not express.
#[test]
fn the_ruled_clades_can_actually_be_sealed() {
    assert!(
        cfe::dop::clade_ceiling() >= 20,
        "clade 19 (draconic) and 20 (metashifting) are ruled — the \
         ceiling must admit them or the ruling has no deed"
    );
    for (class, clade) in [(4u8, 19u8), (3, 20)] {
        let mut world = World::new();
        world.found("clade-hold").expect("founds");
        world
            .seal_taxon(class, clade)
            .unwrap_or_else(|e| panic!("class {class} clade {clade} refused: {e:?}"));
        assert_eq!(world.taxon(), Some((class, clade)));
    }
}
