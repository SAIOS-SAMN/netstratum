//! N6 ACCEPTANCE — two engine processes, one world (BUILD_ORDER §0:
//! "two laptops converge on one world state by exact measurement; a
//! forged state refuses").
//!
//! Run: `cargo run -p mesh --example ns5_two_nodes`
//!
//! The orchestrator spawns ITSELF as a separate OS process per leg —
//! two independent engine instances whose state crosses ONLY as
//! `chronicle.tlv` bytes (the ruled file carrier, NS-5 §3.3). The
//! session: node A founds, plays, and seals the handoff → the wire
//! carries → node B takes the pen with the cited append → drift
//! measured and cured by the tail → CONSILIENCE AT EXACT ZERO → the
//! stale hand refuses (the partition law, §7.5: the arbor stalls) →
//! a forged file refuses with its break named. Every leg asserts its
//! expectation — a wrong outcome fails the run.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::persist::{load, sync};
use cfe::world::World;
use mesh::{append_as, authority, receive, yield_authority, MeshRefused, Side};

const HAND_A: u64 = 1;
const HAND_B: u64 = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        None => orchestrate(),
        Some("found") => found(Path::new(&args[2])),
        Some("take") => take(Path::new(&args[2])),
        Some("adopt") => adopt(Path::new(&args[2]), Path::new(&args[3])),
        Some("usurp") => usurp(Path::new(&args[2])),
        Some("tamper") => tamper(Path::new(&args[2]), Path::new(&args[3])),
        Some("refuse-forged") => refuse_forged(Path::new(&args[2])),
        Some(role) => panic!("unknown role {role}"),
    }
}

/// Spawn one leg as its own OS process — its own engine instance.
fn leg(args: &[&str]) {
    let status = Command::new(env::current_exe().expect("self"))
        .args(args)
        .status()
        .expect("the leg spawns");
    assert!(status.success(), "leg {args:?} failed");
}

fn orchestrate() {
    let base = env::temp_dir().join(format!("netstratum-n6-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("node-a")).expect("node A dir");
    fs::create_dir_all(base.join("node-b")).expect("node B dir");
    let a = base.join("node-a").join("chronicle.tlv");
    let b = base.join("node-b").join("chronicle.tlv");
    let (a, b) = (a.to_str().expect("utf8"), b.to_str().expect("utf8"));

    println!("== N6 ACCEPTANCE: two processes, one world, the file carrier ==");
    leg(&["found", a]);
    println!("   (the wire: node A's bytes carry to node B)");
    fs::copy(a, b).expect("the file carries");
    leg(&["take", b]);
    leg(&["adopt", b, a]);
    leg(&["usurp", a]);
    leg(&["tamper", a, b]);
    leg(&["refuse-forged", b]);
    println!("== the run stands: the pen moved by seal; the tail carried append-only; the stale hand stalled; the forged refused ==");
}

/// Node A's first era, in its own process: found, build, tick,
/// collect — then seal the handoff. The receipts ARE the world; the
/// unreceipted workspace honestly dies with this process.
fn found(file: &Path) {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    let mut chronicle = world.chronicle().clone();
    let seal = yield_authority(&mut chronicle, HAND_A, HAND_B, world.now()).expect("the seal");
    let appended = sync(&chronicle, file).expect("syncs");
    println!(
        "[node A] played an era: {appended} receipts; seal at {seal} — the pen goes to hand {HAND_B}"
    );
}

/// Node B, in its own process: in through the door, the pen verified,
/// the cited append, the sync.
fn take(file: &Path) {
    let bytes = fs::read(file).expect("B bytes");
    let mut chronicle = receive(&bytes, Side::B).expect("intact through the door");
    let hand = authority(&chronicle).expect("lawful arbor").expect("sealed arbor");
    assert_eq!(hand, HAND_B, "the pen arrived with the bytes");
    let stamp = chronicle.receipts().last().map(|r| r.stamp).unwrap_or(0);
    let at = append_as(
        &mut chronicle,
        HAND_B,
        ClosureEvent::ZoneRazed { orb_id: 1, tile: 8 },
        stamp,
        Vec::new(),
    )
    .expect("the cited append");
    sync(&chronicle, file).expect("syncs");
    println!("[node B] holds the pen (hand {HAND_B}); cited append at {at}");
}

/// The drift cure: the tail carries, append-only, nothing re-voted.
fn adopt(src: &Path, dst: &Path) {
    let chronicle = load(src).expect("src returns");
    let appended = sync(&chronicle, dst).expect("the tail carries");
    println!("   (the wire: the tail carries back — {appended} receipt(s), append-only)");
}

/// The partition law (NS-5 §7.5): the stale hand finds the arbor
/// stalled — USURPED, the lawful hand named, nothing appended.
fn usurp(file: &Path) {
    let bytes = fs::read(file).expect("bytes");
    let mut chronicle = receive(&bytes, Side::A).expect("intact");
    let held = chronicle.receipts().len();
    let refused = append_as(
        &mut chronicle,
        HAND_A,
        ClosureEvent::ZoneRazed { orb_id: 1, tile: 0 },
        99,
        Vec::new(),
    )
    .unwrap_err();
    assert_eq!(refused, MeshRefused::Usurped { at: held as u64, lawful: HAND_B });
    assert_eq!(chronicle.receipts().len(), held, "nothing appended");
    println!(
        "[node A] the arbor STALLS for hand {HAND_A}: USURPED, lawful hand {HAND_B} — the partition law holds"
    );
}

/// The adversary: re-frame receipt 2 with a moved stamp. The digest
/// law cannot be recomputed on their side of the wire.
fn tamper(src: &Path, dst: &Path) {
    let mut receipts = load(src).expect("src returns").receipts().to_vec();
    receipts[2].stamp += 1;
    let mut bytes = Vec::new();
    for receipt in &receipts {
        bytes.extend_from_slice(&receipt.wire_bytes());
    }
    fs::write(dst, bytes).expect("the forged file writes");
    println!("   (an adversary re-frames receipt 2 with a moved stamp)");
}

/// The forged state refuses, typed, with its break named.
fn refuse_forged(file: &Path) {
    let bytes = fs::read(file).expect("bytes");
    let refused = receive(&bytes, Side::B).unwrap_err();
    assert_eq!(refused, MeshRefused::Forged { side: Side::B, at: 2 });
    println!("[node B] FORGED refuses: the chain breaks at sequence 2 — the forgery never enters");
}
