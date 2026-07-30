//! NS-5 MESH witnesses (MESH-1a, §3): two independent replays consile
//! at exactly zero; a forged file refuses with its break named; drift
//! names its range and the tail cures it; divergence carries the
//! residual list whole; the mesh records round-trip byte-exact and
//! unknown tags skip (SEAL A). The transport throughout is the ruled
//! file-exchange adapter — bytes between directories, never a socket.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{Resource, ZoneKind};
use cfe::persist::{load, sync};
use cfe::ratio::ratio;
use cfe::tlv;
use cfe::world::{replay, World};
use mesh::{consile, head, receive, ConsilienceReceipt, MeshHead, MeshRefused, Side};

/// A per-test scratch directory (deterministic name, cleaned before use).
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "netstratum-mesh-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// The persist_boot session: one founding, two zones, four ticks, one
/// collection — enough spine to measure.
fn a_first_era() -> World {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    world
}

#[test]
fn two_independent_replays_consile_at_exact_zero() {
    let dir = scratch("consile");
    let a_path = dir.join("node-a").join("chronicle.tlv");
    let b_path = dir.join("node-b").join("chronicle.tlv");
    let world = a_first_era();
    sync(world.chronicle(), &a_path).expect("node A syncs");
    // The file-exchange adapter: the bytes carry, nothing else.
    fs::create_dir_all(b_path.parent().expect("has parent")).expect("node B dir");
    fs::copy(&a_path, &b_path).expect("the file carries");

    let a = receive(&fs::read(&a_path).expect("A bytes"), Side::A).expect("A intact");
    let b = receive(&fs::read(&b_path).expect("B bytes"), Side::B).expect("B intact");
    assert_eq!(head(&a), head(&b), "one spine, one announcement");

    let receipt = consile(&a, &b).expect("one spine consiles");
    assert_eq!(receipt.coherence, ratio(0, 1), "C measured exactly zero");
    assert_eq!(receipt.length, a.receipts().len() as u64);
    assert_eq!(receipt.arity, 2);
    // The v1 manifold: one gauge grade + one orb x seven resources.
    assert_eq!(receipt.dimensions, 8);
    let spine_head = a.receipts().last().expect("nonempty");
    assert_eq!(receipt.head, spine_head.own.0);
    assert_eq!(receipt.stamp, spine_head.stamp);
}

#[test]
fn a_forged_file_refuses_with_its_break_named() {
    let dir = scratch("forged");
    let a_path = dir.join("chronicle.tlv");
    let world = a_first_era();
    sync(world.chronicle(), &a_path).expect("syncs");

    // The adversary re-frames receipt 2 with a moved stamp; the digest
    // law cannot be recomputed on their side of the wire.
    let mut receipts = load(&a_path).expect("intact").receipts().to_vec();
    receipts[2].stamp += 1;
    let mut bytes = Vec::new();
    for receipt in &receipts {
        bytes.extend_from_slice(&receipt.wire_bytes());
    }
    let refused = receive(&bytes, Side::B).unwrap_err();
    assert_eq!(refused, MeshRefused::Forged { side: Side::B, at: 2 });
}

#[test]
fn a_drifted_peer_names_its_range_and_the_tail_cures_it() {
    let dir = scratch("drift");
    let a_path = dir.join("node-a").join("chronicle.tlv");
    let b_path = dir.join("node-b").join("chronicle.tlv");
    let world = a_first_era();
    sync(world.chronicle(), &a_path).expect("node A syncs");
    fs::create_dir_all(b_path.parent().expect("has parent")).expect("node B dir");
    fs::copy(&a_path, &b_path).expect("the file carries");
    let behind_len = world.chronicle().receipts().len() as u64;

    // Node A plays on; node B holds the same history, less of it.
    let mut second = World::resume(load(&a_path).expect("returns"));
    second.zone(9, ZoneKind::Habitat).expect("era two builds");
    sync(second.chronicle(), &a_path).expect("the tail appends");

    let a = receive(&fs::read(&a_path).expect("A bytes"), Side::A).expect("A intact");
    let b = receive(&fs::read(&b_path).expect("B bytes"), Side::B).expect("B intact");
    let refused = consile(&a, &b).unwrap_err();
    assert_eq!(
        refused,
        MeshRefused::Drift { behind: Side::B, from: behind_len, to: behind_len + 1 },
        "the missing range is named, half-open"
    );

    // The cure is the tail, never a re-vote: sync appends ONLY what
    // node B lacks, and the next judgment consiles at zero.
    let appended = sync(&a, &b_path).expect("the tail carries");
    assert_eq!(appended, 1, "only the missing receipt crosses");
    let b = receive(&fs::read(&b_path).expect("B bytes again"), Side::B).expect("B intact");
    let receipt = consile(&a, &b).expect("cured spines consile");
    assert_eq!(receipt.coherence, ratio(0, 1));
    assert_eq!(receipt.length, behind_len + 1);
}

#[test]
fn divergent_histories_carry_their_residuals_whole() {
    let dir = scratch("diverged");
    let a_path = dir.join("node-a").join("chronicle.tlv");
    let b_path = dir.join("node-b").join("chronicle.tlv");
    let world = a_first_era();
    sync(world.chronicle(), &a_path).expect("node A syncs");
    fs::create_dir_all(b_path.parent().expect("has parent")).expect("node B dir");
    fs::copy(&a_path, &b_path).expect("the file carries");
    let fork = world.chronicle().receipts().len() as u64;

    // Node A collects two more ticks; node B builds instead. One
    // past, two futures.
    let mut node_a = World::resume(load(&a_path).expect("returns"));
    node_a.tick();
    node_a.tick();
    node_a.collect().expect("A settles");
    sync(node_a.chronicle(), &a_path).expect("A syncs its future");
    let mut node_b = World::resume(load(&b_path).expect("returns"));
    node_b.zone(9, ZoneKind::Habitat).expect("B builds its future");
    sync(node_b.chronicle(), &b_path).expect("B syncs its future");

    let a = receive(&fs::read(&a_path).expect("A bytes"), Side::A).expect("A intact");
    let b = receive(&fs::read(&b_path).expect("B bytes"), Side::B).expect("B intact");
    let refused = consile(&a, &b).unwrap_err();
    let MeshRefused::Diverged { at, residuals, coherence } = refused else {
        panic!("divergence must refuse DIVERGED, got {refused:?}");
    };
    assert_eq!(at, fork, "the first differing sequence is named");
    assert!(!residuals.is_empty(), "the disagreement is carried, not discarded");

    // Ground truth from cfe's own replay, independent of the mesh's
    // projection: every differing banked dimension appears, exactly
    // once, with its exact rational difference (dimension l maps to
    // resource code l on this one-grade one-orb manifold).
    let committed_a = replay(&a);
    let committed_b = replay(&b);
    let mut expected = Vec::new();
    for code in 1u8..=7 {
        let resource = Resource::from_wire(code).expect("a resource code");
        let ours = committed_a.orbs[0].banked_amount(resource);
        let theirs = committed_b.orbs[0].banked_amount(resource);
        if ours != theirs {
            expected.push((code as u32, theirs - ours));
        }
    }
    assert_eq!(residuals, expected, "the residual list, whole and exact");

    // C(delta) = 6 * sum R^2 — the summary scalar recomputed.
    let mut expected_c = ratio(0, 1);
    for (_, residual) in &residuals {
        expected_c += ratio(6, 1) * residual * residual;
    }
    assert_eq!(coherence, expected_c);
    assert_ne!(coherence, ratio(0, 1), "divergence measures nonzero");
}

#[test]
fn the_mesh_records_round_trip_and_unknown_tags_skip() {
    let announced = MeshHead { genesis: [7u8; 32], length: 9, stamp: 4, head: [9u8; 32] };
    let receipt = ConsilienceReceipt {
        length: 9,
        stamp: 4,
        head: [9u8; 32],
        arity: 2,
        dimensions: 8,
        coherence: ratio(0, 1),
    };
    let refusals = [
        MeshRefused::Malformed { side: Side::B, frame: 3 },
        MeshRefused::Forged { side: Side::B, at: 2 },
        MeshRefused::Drift { behind: Side::A, from: 5, to: 9 },
        MeshRefused::Diverged {
            at: 5,
            residuals: vec![(1, ratio(-3, 7)), (4, ratio(9, 2))],
            coherence: ratio(731, 14),
        },
    ];

    let bytes = announced.wire_bytes();
    let (tag, value) = tlv::records(&bytes).next().expect("one record");
    let decoded = MeshHead::from_wire(tag, value).expect("the head decodes");
    assert_eq!(decoded, announced);
    assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");

    let bytes = receipt.wire_bytes();
    let (tag, value) = tlv::records(&bytes).next().expect("one record");
    let decoded = ConsilienceReceipt::from_wire(tag, value).expect("the receipt decodes");
    assert_eq!(decoded, receipt);
    assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");

    for refusal in refusals {
        let bytes = refusal.wire_bytes();
        let (tag, value) = tlv::records(&bytes).next().expect("one record");
        let decoded = MeshRefused::from_wire(tag, value).expect("the refusal decodes");
        assert_eq!(decoded, refusal);
        assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");
    }

    // SEAL A: a future engine speaks a record this version has never
    // heard of; the reader skips it and still finds the head.
    let mut stream = Vec::new();
    tlv::put(200, b"a-future-mesh-record", &mut stream);
    stream.extend_from_slice(&announced.wire_bytes());
    let mut found = None;
    for (tag, value) in tlv::records(&stream) {
        if let Some(decoded) = MeshHead::from_wire(tag, value) {
            found = Some(decoded);
        }
    }
    assert_eq!(found, Some(announced), "unknown tags skip, never crash");
}
