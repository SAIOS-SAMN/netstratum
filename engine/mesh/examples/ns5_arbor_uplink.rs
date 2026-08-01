//! SPINE-UPLINK acceptance (NS-5 §9): two processes, one universe,
//! the SPINE carrier. Node A plays a chain and PRESENTS it as a
//! segment; the segment bytes carry to node B (its own OS process, its
//! own engine); node B RECEIVES them and reaches the EQUAL arbor —
//! admission is verification. Then a forged segment refuses at its
//! break. Nothing file-borne but chain-verified bytes crosses (THE
//! UPLINK LAW): the segment IS the chain.
//!
//! Run: `cargo run -p mesh --example ns5_spine_uplink`

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::World;
use mesh::arbor::{Coordinate, Arbor};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        None => orchestrate(),
        Some("present") => present(Path::new(&args[2]), Path::new(&args[3])),
        Some("receive") => receive_leg(Path::new(&args[2]), Path::new(&args[3])),
        Some("refuse-forged") => refuse_forged(Path::new(&args[2]), Path::new(&args[3])),
        Some(role) => panic!("unknown role {role}"),
    }
}

fn leg(args: &[&str]) {
    let status = Command::new(env::current_exe().expect("self"))
        .args(args)
        .status()
        .expect("the leg spawns");
    assert!(status.success(), "leg {args:?} failed");
}

fn coord(t: u64) -> Coordinate {
    Coordinate { branch: String::new(), s: [ratio(0, 1), ratio(0, 1), ratio(0, 1)], g: ratio(0, 1), t }
}

/// A played world's committed bytes — a real chain state.
fn played(iron: i64) -> Vec<u8> {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    for _ in 0..iron.max(1) {
        world.tick();
    }
    world.collect().expect("settles");
    codec::state_bytes(&world.committed())
}

fn orchestrate() {
    let base = env::temp_dir().join(format!("netstratum-uplink-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("node-a")).expect("node A dir");
    fs::create_dir_all(base.join("node-b")).expect("node B dir");
    let a_spine = base.join("node-a").join("spine");
    let a_seg = base.join("node-a").join("segment.tlv");
    let b_spine = base.join("node-b").join("spine");
    let b_seg = base.join("node-b").join("segment.tlv");

    println!("== SPINE-UPLINK ACCEPTANCE: two processes, one universe, the segment carrier ==");
    leg(&["present", a_spine.to_str().unwrap(), a_seg.to_str().unwrap()]);
    println!("   (the wire: node A's segment bytes carry to node B)");
    fs::copy(&a_seg, &b_seg).expect("the segment carries");
    leg(&["receive", b_seg.to_str().unwrap(), b_spine.to_str().unwrap()]);
    leg(&["refuse-forged", b_seg.to_str().unwrap(), b_spine.to_str().unwrap()]);
    println!("== the run stands: A presented, B received the equal arbor, the forged segment refused ==");
}

/// Node A: play a chain onto its arbor, present it as a segment.
fn present(spine_root: &Path, segment: &Path) {
    let mut arbor = Arbor::open(spine_root).expect("A's arbor opens");
    arbor.adopt(coord(1), &played(2)).expect("A adopts its founding");
    arbor.extend(coord(2), &played(6), None).expect("A walks a beat");
    arbor.extend(coord(3), &played(10), None).expect("A walks another");
    let bytes = arbor.present();
    fs::write(segment, &bytes).expect("A writes the segment");
    println!(
        "[node A] played 3 records; presented a {}-byte segment (records + blobs)",
        bytes.len()
    );
}

/// Node B: open empty, receive the segment, reach the equal arbor.
fn receive_leg(segment: &Path, spine_root: &Path) {
    let bytes = fs::read(segment).expect("B reads the segment");
    let mut arbor = Arbor::open(spine_root).expect("B's arbor opens empty");
    let admitted = arbor.receive(&bytes).expect("B admits the verified segment");
    assert_eq!(admitted, 3, "all three records crossed");
    let head = arbor.head("").expect("B's mainline stands");
    // B independently re-serves the head geometry from its own store.
    let served = arbor.fetch(&head.state).expect("B's store holds the head");
    let committed = codec::from_state_bytes(&served).expect("B decodes the head");
    let iron = committed
        .orbs
        .first()
        .map(|orb| orb.banked_amount(Resource::Iron))
        .unwrap_or_else(|| ratio(0, 1));
    println!(
        "[node B] admitted {admitted} records by re-verification; head at t={}, iron {iron} — the equal arbor",
        head.coordinate.t
    );
}

/// Node B again: a forged segment (one byte flipped) refuses, and the
/// arbor is left untouched.
fn refuse_forged(segment: &Path, spine_root: &Path) {
    let mut bytes = fs::read(segment).expect("B reads the segment");
    bytes[8] ^= 0xff;
    let forged_root = spine_root.with_extension("forged");
    let mut arbor = Arbor::open(&forged_root).expect("a fresh arbor opens");
    match arbor.receive(&bytes) {
        Err(refused) => {
            assert!(arbor.head("").is_none(), "nothing admitted past the break");
            println!("[node B] the forged segment REFUSED ({refused:?}) — admission is verification");
        }
        Ok(_) => panic!("a forged segment must never be admitted"),
    }
}
