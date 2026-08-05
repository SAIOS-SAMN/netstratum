//! SPINE-UPLINK pins (NS-5 §9): the replication seam. A arbor PRESENTS
//! itself as a segment; a peer RECEIVES it and reaches an EQUAL arbor —
//! byte-for-byte, because admission is verification, not trust. A
//! tampered record refuses at its break; a withheld blob refuses; a
//! chain that does not extend the receiver's head refuses; and a
//! JUDGED record is admitted ONLY when its step re-runs lawful under
//! the receiver's own kernel — a judgment that cannot be reproduced is
//! not admitted.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::tlv;
use cfe::world::{World, SOVEREIGN};
use mesh::arbor::{
    Coordinate, Arbor, ArborRefused, SEGMENT_STEP_TAG,
};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "netstratum-uplink-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn coord(t: u64) -> Coordinate {
    Coordinate { branch: String::new(), s: [ratio(0, 1), ratio(0, 1), ratio(0, 1)], g: ratio(0, 1), t }
}

fn orb(id: u64, iron: (i64, i64)) -> ColonyOrb {
    let mut held = ColonyOrb { orb_id: id, name: format!("orb-{id}"), ..Default::default() };
    held.bank(Resource::Iron, ratio(iron.0, iron.1));
    held
}

/// Two orbs, iron split a/b; the sovereign + a Collect-scoped hand 100.
fn two_orb_state(a: (i64, i64), b: (i64, i64)) -> Vec<u8> {
    let mut committed = World::new().committed();
    committed.orbs = vec![orb(1, a), orb(2, b)];
    committed.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "sovereign".into(), principal: None, grants: vec![] },
        Entity { entity_id: 100, handle: "hand".into(), principal: Some(SOVEREIGN), grants: vec![Scope::Collect] },
    ];
    codec::state_bytes(&committed)
}

fn transfer_step(stamp: u64) -> kernel::Step {
    kernel::Step {
        event: ClosureEvent::ResourceTransferred { from_orb: 1, to_orb: 2, resource: Resource::Iron, amount: ratio(3, 1) },
        actor: 100,
        stamp,
    }
}

/// Encode a segment step frame (tag 88), mirroring `receive`'s decode:
/// commit ‖ actor ‖ stamp ‖ event_bytes.
fn step_frame(commit: [u8; 32], step: &kernel::Step) -> Vec<u8> {
    let mut value = commit.to_vec();
    value.extend_from_slice(&step.actor.to_le_bytes());
    value.extend_from_slice(&step.stamp.to_le_bytes());
    value.extend_from_slice(&step.event.wire_bytes());
    let mut out = Vec::new();
    tlv::put(SEGMENT_STEP_TAG, &value, &mut out);
    out
}

/// A source arbor plays a chain; the presented segment carries a peer
/// to the IDENTICAL arbor, and re-receiving it is idempotent.
#[test]
fn present_and_receive_reach_an_equal_spine() {
    let src_root = scratch("src");
    let dst_root = scratch("dst");
    let mut src = Arbor::open(&src_root).expect("opens");
    src.adopt(coord(1), &two_orb_state((10, 1), (0, 1))).expect("adopts");
    src.extend(coord(2), &two_orb_state((7, 1), (3, 1)), None).expect("walks");
    src.extend(coord(3), &two_orb_state((5, 1), (5, 1)), None).expect("walks");

    let segment = src.present();
    let mut dst = Arbor::open(&dst_root).expect("opens empty");
    assert_eq!(dst.receive(&segment).expect("admits"), 3, "three records cross");
    assert_eq!(dst.records(), src.records(), "the spines are equal, byte-for-byte");
    // Idempotent: re-presenting the whole arbor admits nothing new.
    assert_eq!(dst.receive(&segment).expect("idempotent"), 0, "already held, skipped");
    // The store carried too: the head geometry serves on the receiver.
    let head = dst.head("").expect("stands").state;
    assert_eq!(dst.fetch(&head).expect("served"), two_orb_state((5, 1), (5, 1)));
}

/// A tampered record refuses at its break — the commit no longer
/// reseals.
#[test]
fn a_tampered_record_refuses_at_its_break() {
    let src_root = scratch("tamper-src");
    let dst_root = scratch("tamper-dst");
    let mut src = Arbor::open(&src_root).expect("opens");
    src.adopt(coord(1), &two_orb_state((10, 1), (0, 1))).expect("adopts");
    src.extend(coord(2), &two_orb_state((7, 1), (3, 1)), None).expect("walks");
    let mut segment = src.present();
    // Flip a byte in the record region (the first frame's value).
    segment[8] ^= 0xff;
    let mut dst = Arbor::open(&dst_root).expect("opens");
    assert!(matches!(
        dst.receive(&segment),
        Err(ArborRefused::Forged { .. }) | Err(ArborRefused::Malformed { .. })
    ));
    assert!(dst.head("").is_none(), "nothing admitted past the break");
}

/// A withheld blob refuses: the geometry a record names is unheld.
#[test]
fn a_withheld_blob_refuses() {
    let src_root = scratch("blob-src");
    let dst_root = scratch("blob-dst");
    let mut src = Arbor::open(&src_root).expect("opens");
    src.adopt(coord(1), &two_orb_state((10, 1), (0, 1))).expect("adopts");
    let full = src.present();
    // Keep only the record frame (tag 84), drop the blob frame (87).
    let mut records_only = Vec::new();
    for (tag, value) in tlv::records(&full) {
        if tag == mesh::arbor::SPINE_RECORD_TAG {
            tlv::put(tag, value, &mut records_only);
        }
    }
    let mut dst = Arbor::open(&dst_root).expect("opens");
    assert!(matches!(dst.receive(&records_only), Err(ArborRefused::BlobUnheld { .. })));
}

/// A segment that does not extend the receiver's head refuses: two
/// independent genesis chains cannot be spliced.
#[test]
fn a_foreign_chain_refuses_to_splice() {
    let a_root = scratch("splice-a");
    let b_root = scratch("splice-b");
    let mut a = Arbor::open(&a_root).expect("opens");
    a.adopt(coord(1), &two_orb_state((10, 1), (0, 1))).expect("adopts");

    // B has its OWN genesis (different state) at the same address.
    let mut b = Arbor::open(&b_root).expect("opens");
    b.adopt(coord(1), &two_orb_state((9, 1), (1, 1))).expect("adopts");

    // A tries to receive B's segment: B's genesis sits at coord(1),
    // already held by A with a DIFFERENT commit — one value per address.
    assert!(matches!(a.receive(&b.present()), Err(ArborRefused::Occupied)));
}

/// The trustless core: a JUDGED record is admitted only when its step
/// re-runs `lawful` under the receiver's kernel. No step → refused; a
/// lawful step → admitted; an unlawful step → refused by the kernel.
#[test]
fn a_judged_record_is_re_judged_or_refused() {
    let src_root = scratch("judge-src");
    let mut src = Arbor::open(&src_root).expect("opens");
    src.adopt(coord(1), &two_orb_state((10, 1), (0, 1))).expect("adopts");
    // A JUDGED extend: the transfer re-runs lawful at the source too.
    let judged = src
        .extend(coord(2), &two_orb_state((7, 1), (3, 1)), Some(&transfer_step(2)))
        .expect("judged and lawful")
        .clone();
    assert!(judged.judged, "the record is judged");

    // present() carries records + blobs but NO step (the arbor stores
    // none — the Phase-3 seam): the gated record cannot be re-gated,
    // so the receiver refuses it.
    let base_segment = src.present();
    let mut dst = Arbor::open(scratch("judge-dst1")).expect("opens");
    assert!(matches!(
        dst.receive(&base_segment),
        Err(ArborRefused::JudgedWithoutProof { .. })
    ));

    // Carry the LAWFUL step alongside: now the receiver re-gates and
    // admits, reaching the equal arbor.
    let mut with_step = base_segment.clone();
    with_step.extend_from_slice(&step_frame(judged.commit, &transfer_step(2)));
    let mut dst2 = Arbor::open(scratch("judge-dst2")).expect("opens");
    assert_eq!(dst2.receive(&with_step).expect("re-judged lawful"), 2);
    assert_eq!(dst2.records(), src.records(), "equal arbor after re-judge");

    // Carry an UNLAWFUL step for the same record: the transition is
    // claimed by actor 999, who is no entity in `prior` (I2 —
    // identity precedes the world). The commit still reseals (the
    // gated flag is unchanged; the step is not in the seal), so the
    // break is the KERNEL's verdict, not the chain's. (A wrong
    // transfer AMOUNT would NOT refuse — conservation reads the world
    // total, which is state-derived and unchanged; the unlawfulness
    // must be one the kernel actually gates.)
    let liar = kernel::Step {
        event: ClosureEvent::ResourceTransferred { from_orb: 1, to_orb: 2, resource: Resource::Iron, amount: ratio(3, 1) },
        actor: 999,
        stamp: 2,
    };
    let mut with_liar = base_segment.clone();
    with_liar.extend_from_slice(&step_frame(judged.commit, &liar));
    let mut dst3 = Arbor::open(scratch("judge-dst3")).expect("opens");
    assert!(matches!(dst3.receive(&with_liar), Err(ArborRefused::Refused(_))));
}
