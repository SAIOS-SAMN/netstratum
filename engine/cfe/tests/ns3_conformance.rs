//! NS-3 §4 conformance witnesses, vector-first: the digests here are
//! the REAL v0.4 chronicle outputs inscribed in NS-3 §3 — extracted,
//! never fabricated. A fresh implementation must reproduce them from
//! the stated byte layout alone.

use cfe::dop::ClosureEvent;
use cfe::vrp::{
    receipt_digest, verify_receipts, Attestation, ChainRead, Chronicle, Receipt, ReceiptHash,
    TickRead, Timekeeper, WorldClock,
};

fn hash_from_hex(hex: &str) -> ReceiptHash {
    let mut bytes = [0u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex vector");
    }
    ReceiptHash(bytes)
}

const RULE: &str = "ObserverFrame { plane: PlaneElement { exchange: Keep, row: Keep, col: Keep }, relabel: Relabel { pairs: [(1, 4), (4, 1)] } }";

#[test]
fn the_ns3_vectors_reproduce_from_the_stated_layout() {
    // NS-3 §3.2 — three receipts, byte-exact, event bytes verbatim.
    let receipt_0 = receipt_digest(
        0,
        0,
        &ReceiptHash::GENESIS,
        format!("Consilience {{ rule: {RULE} }}").as_bytes(),
        &[3, 7],
    );
    assert_eq!(
        receipt_0,
        hash_from_hex("c86c17aa50e457a9e5ef0104af55c198d14c90a3647b27021c5f26989954ba28")
    );
    let receipt_1 = receipt_digest(
        1,
        0,
        &receipt_0,
        format!(
            "Crystal {{ rule: {RULE}, vantage_provenance: [Provenance(\"arc_3c9b0459\")] }}"
        )
        .as_bytes(),
        &[11],
    );
    assert_eq!(
        receipt_1,
        hash_from_hex("aca043784d14a85836406a564d68defd3dd77b89267603eb3a90be50677eaf05")
    );
    let receipt_2 = receipt_digest(
        2,
        1,
        &receipt_1,
        format!("SunInstall {{ rule: {RULE} }}").as_bytes(),
        &[13],
    );
    assert_eq!(
        receipt_2,
        hash_from_hex("877cf37994b93df067efb3ec363e3f00f503cb0b36df8ac54424ca6918496250")
    );
}

#[test]
fn the_tamper_vector_diverges() {
    // NS-3 §3.3 — one changed provenance u64 diverges the digest.
    let altered = receipt_digest(
        0,
        0,
        &ReceiptHash::GENESIS,
        format!("Consilience {{ rule: {RULE} }}").as_bytes(),
        &[3, 8],
    );
    assert_eq!(
        altered,
        hash_from_hex("366aa785fd766cb2e698e76d56e0cc26636559206174e70661fe809041339057")
    );
}

#[test]
fn the_clock_refuses_partial_skip_and_replay() {
    // NS-3 §3.1 — the tick law measured on the fresh implementation.
    let mut clock = WorldClock::new();
    assert_eq!(clock.now(), 0);
    let ticked = clock.tick(&[
        Attestation::Attests { next: 1 },
        Attestation::Attests { next: 1 },
        Attestation::Attests { next: 1 },
    ]);
    assert_eq!(ticked, TickRead::Ticked { now: 1 });
    let partial = clock.tick(&[
        Attestation::Attests { next: 2 },
        Attestation::Attests { next: 2 },
        Attestation::Refuses,
    ]);
    assert!(matches!(partial, TickRead::QuorumOpen { .. }));
    assert_eq!(clock.now(), 1, "2-of-3 moves nothing");
    let skip = clock.tick(&[
        Attestation::Attests { next: 5 },
        Attestation::Attests { next: 5 },
        Attestation::Attests { next: 5 },
    ]);
    assert!(matches!(skip, TickRead::QuorumOpen { .. }));
    assert_eq!(clock.now(), 1, "the clock cannot jump");
    let replay = clock.tick(&[
        Attestation::Attests { next: 1 },
        Attestation::Attests { next: 1 },
        Attestation::Attests { next: 1 },
    ]);
    assert!(matches!(replay, TickRead::QuorumOpen { .. }));
    assert_eq!(clock.now(), 1, "the clock cannot replay");
}

#[test]
fn the_seated_keepers_cannot_disagree_today() {
    // THE PLACEHOLDER, PINNED (2026-08-03). The test above proves the
    // tick LAW: handed 2-of-3, a skip or a replay, the clock refuses.
    // This one records what the live world actually hands it. Three
    // seated keepers share one clock in one process and `attest` reads
    // nothing else, so the production quorum is unanimous BY
    // CONSTRUCTION and the live clock always walks.
    //
    // Pinned so the vacuity reads as a known placeholder and never as
    // an enforced quorum. Keepers become real parties when identity
    // does (NS-5 processes, NS-7 identity); on that day this pin goes
    // red and is TRUTH-UPDATED, not deleted.
    let clock = WorldClock::new();
    let seated = [
        Timekeeper { id: 0 },
        Timekeeper { id: 1 },
        Timekeeper { id: 2 },
    ];
    let answers: Vec<Attestation> = seated.iter().map(|keeper| keeper.attest(&clock)).collect();
    assert!(
        answers.iter().all(|answer| *answer == answers[0]),
        "three keepers, one clock, one input — the answers cannot differ today"
    );
    assert_eq!(
        answers[0],
        Attestation::Attests { next: 1 },
        "a keeper attests the EXACT successor, never a jump"
    );

    // The consequence at the world: the clock walks every tick, and
    // `QuorumOpen` never reaches a caller through the seated path.
    let mut world = cfe::world::World::new();
    let outcome = world.tick();
    assert_eq!(
        outcome.read,
        TickRead::Ticked { now: 1 },
        "the seated quorum is unanimous, so the world clock always walks"
    );
}

#[test]
fn the_chain_links_and_names_its_first_break() {
    let mut chronicle = Chronicle::new();
    chronicle.append(
        ClosureEvent::ColonyFounded {
            orb_id: 1,
            name: "meridian".into(),
        },
        0,
        vec![],
    );
    chronicle.append(
        ClosureEvent::ZoneBuilt {
            orb_id: 1,
            tile: 4,
            kind: cfe::dop::ZoneKind::Extractor,
        },
        1,
        vec![],
    );
    assert_eq!(chronicle.verify_chain(), ChainRead::Intact);
    assert_eq!(
        chronicle.receipts()[0].parent,
        ReceiptHash::GENESIS,
        "sequence 0 carries the genesis parent"
    );
    assert_eq!(
        chronicle.receipts()[1].parent, chronicle.receipts()[0].own,
        "the chain is parent-linked"
    );

    // A receipt list a peer hands us with one altered stamp: the
    // verification read names the first broken sequence.
    let mut forged: Vec<Receipt> = chronicle.receipts().to_vec();
    forged[1].stamp = 9;
    assert_eq!(verify_receipts(&forged), ChainRead::Broken { at: 1 });
}
