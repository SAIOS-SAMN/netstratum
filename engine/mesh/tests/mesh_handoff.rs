//! NS-5 §7 witnesses (MESH-1b): the handoff walks (and survives the
//! file — SEAL B); an uncited first append refuses with both
//! sequences named; a stale hand refuses with the lawful hand named;
//! a stale seal refuses at the door with the chronicle unchanged; the
//! refusal kinds round-trip byte-exact.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::persist::{load, sync};
use cfe::ratio::ratio;
use cfe::tlv;
use cfe::vrp::Chronicle;
use mesh::{append_as, authority, yield_authority, MeshRefused};

/// A per-test scratch directory (deterministic name, cleaned before use).
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "netstratum-handoff-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// A pre-mesh arbor: three receipts, no seal anywhere — the era every
/// landed world lives in today (actor 100's hand, unconstrained).
fn a_premesh_chronicle() -> Chronicle {
    let mut chronicle = Chronicle::new();
    chronicle.append(
        ClosureEvent::ColonyFounded { orb_id: 1, name: "meridian".into() },
        0,
        vec![100],
    );
    chronicle.append(
        ClosureEvent::ZoneBuilt { orb_id: 1, tile: 4, kind: ZoneKind::Extractor, base: None },
        1,
        vec![100],
    );
    chronicle.append(
        ClosureEvent::YieldCollected { orb_id: 1, resource: Resource::Iron, amount: ratio(3, 2) },
        2,
        vec![100],
    );
    chronicle
}

#[test]
fn the_handoff_walks() {
    let mut chronicle = a_premesh_chronicle();
    assert_eq!(authority(&chronicle), Ok(None), "pre-mesh is single-handed");

    // The first seal self-declares the incumbent; the pen moves.
    let seal = yield_authority(&mut chronicle, 7, 9, 3).expect("the first seal");
    assert_eq!(seal, 3, "the seal's sequence");
    assert_eq!(authority(&chronicle), Ok(Some(9)), "the pen moved");

    // The taking hand's first append carries the hand AND cites the seal.
    let at = append_as(
        &mut chronicle,
        9,
        ClosureEvent::ZoneRazed { orb_id: 1, tile: 4 },
        4,
        Vec::new(),
    )
    .expect("the cited append");
    assert_eq!(at, 4);
    let receipt = &chronicle.receipts()[4];
    assert!(receipt.provenance.contains(&9), "the hand is carried");
    assert!(receipt.provenance.contains(&seal), "the seal is cited");

    // The new hand yields onward; a fresh seal standing last is lawful
    // with its citation outstanding.
    yield_authority(&mut chronicle, 9, 12, 5).expect("the second seal");
    assert_eq!(authority(&chronicle), Ok(Some(12)));

    // SEAL B: the walk derives identically across the file boundary.
    let path = scratch("walks").join("chronicle.tlv");
    sync(&chronicle, &path).expect("syncs");
    let resumed = load(&path).expect("returns");
    assert_eq!(authority(&resumed), Ok(Some(12)), "authority survives the file");
}

#[test]
fn an_uncited_first_append_refuses() {
    let mut chronicle = a_premesh_chronicle();
    let seal = yield_authority(&mut chronicle, 7, 9, 3).expect("seals");
    // The taking hand writes through the raw door, carrying itself but
    // never citing where the pen came from.
    chronicle.append(ClosureEvent::ZoneRazed { orb_id: 1, tile: 4 }, 4, vec![9]);
    assert_eq!(
        authority(&chronicle),
        Err(MeshRefused::Uncited { at: 4, handoff: seal }),
        "the append and the seal are both named"
    );
}

#[test]
fn a_stale_hand_refuses() {
    let mut chronicle = a_premesh_chronicle();
    yield_authority(&mut chronicle, 7, 9, 3).expect("seals");
    // The OLD hand keeps writing: it carries itself, not the authority.
    chronicle.append(ClosureEvent::ZoneRazed { orb_id: 1, tile: 4 }, 4, vec![7]);
    assert_eq!(
        authority(&chronicle),
        Err(MeshRefused::Usurped { at: 4, lawful: 9 }),
        "one writer per clock — authorship judges before citation"
    );
}

#[test]
fn a_stale_seal_refuses_at_the_door() {
    let mut chronicle = a_premesh_chronicle();
    yield_authority(&mut chronicle, 7, 9, 3).expect("the first seal");
    append_as(&mut chronicle, 9, ClosureEvent::ZoneRazed { orb_id: 1, tile: 4 }, 4, Vec::new())
        .expect("the cited append");
    let held = chronicle.receipts().len();

    // The yielded hand tries to seal onward; the door refuses BEFORE
    // anything appends.
    let refused = yield_authority(&mut chronicle, 7, 30, 5).unwrap_err();
    assert_eq!(refused, MeshRefused::Usurped { at: held as u64, lawful: 9 });
    assert_eq!(chronicle.receipts().len(), held, "the chronicle is unchanged");
}

#[test]
fn the_refusal_kinds_round_trip() {
    for refusal in [
        MeshRefused::Usurped { at: 4, lawful: 9 },
        MeshRefused::Uncited { at: 4, handoff: 3 },
    ] {
        let bytes = refusal.wire_bytes();
        let (tag, value) = tlv::records(&bytes).next().expect("one record");
        let decoded = MeshRefused::from_wire(tag, value).expect("the refusal decodes");
        assert_eq!(decoded, refusal);
        assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");
    }
}
