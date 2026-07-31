//! SPINE-1 pins (NS-5 §9): the spine walks and reloads whole; the
//! arrow and the address hold structurally; the nursery owns its own
//! addresses; the store verifies on every read; and the judge stands
//! at the door — lawful passes, unlawful refuses, the sealed default
//! refuses, and the unjudged hole is STAMPED, never hidden.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{World, SOVEREIGN};
use mesh::spine::{Coordinate, Spine, SpineRefused};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "netstratum-spine-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// A mainline coordinate at tick `t` (unchartered S, base gauge).
fn coord(t: u64) -> Coordinate {
    Coordinate {
        branch: String::new(),
        s: [ratio(0, 1), ratio(0, 1), ratio(0, 1)],
        g: ratio(0, 1),
        t,
    }
}

fn branch_coord(branch: &str, t: u64) -> Coordinate {
    Coordinate { branch: branch.into(), ..coord(t) }
}

/// One orb holding `iron` iron — the judge's test material.
fn orb(id: u64, iron: (i64, i64)) -> ColonyOrb {
    let mut held = ColonyOrb { orb_id: id, name: format!("orb-{id}"), ..Default::default() };
    held.bank(Resource::Iron, ratio(iron.0, iron.1));
    held
}

/// A two-orb committed state: iron split `a` / `b`. Identity precedes
/// the world (I2): the sovereign is seated, and the acting hand (100)
/// is a contained agent holding exactly Collect — the scope the
/// transfer deed requires, no more.
fn two_orb_state(a: (i64, i64), b: (i64, i64)) -> Vec<u8> {
    let mut committed = World::new().committed();
    committed.orbs = vec![orb(1, a), orb(2, b)];
    committed.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "sovereign".into(), principal: None, grants: vec![] },
        Entity {
            entity_id: 100,
            handle: "hand".into(),
            principal: Some(SOVEREIGN),
            grants: vec![Scope::Collect],
        },
    ];
    codec::state_bytes(&committed)
}

/// A played-world state and a gauge-shifted variant.
fn played_state(shift: Option<Ratio>) -> Vec<u8> {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    if let Some(gauge) = shift {
        world.shift_gauge(gauge);
    }
    codec::state_bytes(&world.committed())
}

fn transfer_step(t: u64) -> kernel::Step {
    kernel::Step {
        event: ClosureEvent::ResourceTransferred {
            from_orb: 1,
            to_orb: 2,
            resource: Resource::Iron,
            amount: ratio(3, 1),
        },
        actor: 100,
        stamp: t,
    }
}

#[test]
fn the_spine_walks_and_reloads_whole() {
    let root = scratch("walks");
    let mut spine = Spine::open(&root).expect("opens empty");
    let bytes_a = played_state(None);
    let bytes_b = played_state(Some(ratio(1, 3)));

    let genesis = spine.extend(coord(0), &bytes_a, None).expect("genesis").clone();
    assert_eq!(genesis.parent, [0u8; 32], "genesis is parentless");
    assert_eq!(genesis.schema, codec::SCHEMA_V1);
    assert_eq!(genesis.kernel, kernel::VERSION);
    assert!(!genesis.judged, "genesis is honestly unjudged");

    let second = spine.extend(coord(4), &bytes_b, None).expect("walks").clone();
    assert_eq!(second.parent, genesis.commit, "the path links");
    assert_eq!(
        spine.resolve(&coord(4)).expect("resolves").commit,
        second.commit,
        "one coordinate, one commitment"
    );
    assert_eq!(spine.fetch(&second.state).expect("the geometry unpacks"), bytes_b);

    // A step offered at genesis refuses: nothing prior to judge from.
    let fresh = scratch("walks-genesis");
    let mut fresh_spine = Spine::open(&fresh).expect("opens");
    let refused = fresh_spine.extend(coord(0), &bytes_a, Some(&transfer_step(0))).unwrap_err();
    assert_eq!(refused, SpineRefused::GenesisJudged);

    // SEAL B for the spine: reload from disk, identical maps.
    drop(spine);
    let reloaded = Spine::open(&root).expect("reloads whole");
    assert_eq!(reloaded.head("").expect("head stands").commit, second.commit);
    assert_eq!(reloaded.resolve(&coord(0)).expect("genesis stands").commit, genesis.commit);
    assert_eq!(reloaded.fetch(&second.state).expect("store survives"), bytes_b);
}

#[test]
fn the_arrow_and_the_address_hold() {
    let root = scratch("arrow");
    let mut spine = Spine::open(&root).expect("opens");
    let bytes = played_state(None);
    spine.extend(coord(0), &bytes, None).expect("genesis");
    spine.extend(coord(4), &bytes, None).expect("walks");

    // One value per address: the exact head address refuses OCCUPIED;
    // the clock only walks forward: a backward tick refuses the ARROW.
    assert_eq!(spine.extend(coord(4), &bytes, None).unwrap_err(), SpineRefused::Occupied);
    assert!(matches!(
        spine.extend(coord(2), &bytes, None).unwrap_err(),
        SpineRefused::ArrowBroken { .. }
    ));

    // A hand-forged record at an occupied address — valid commit, real
    // parent — refuses the whole file on open: one value per address
    // is structural, not policed.
    let head = spine.head("").expect("stands").clone();
    let forged = mesh::spine::seal(
        coord(4),
        head.schema,
        head.kernel,
        false,
        head.state,
        head.commit,
    );
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(root.join("spine.tlv"))
        .expect("the file opens");
    use std::io::Write as _;
    file.write_all(&forged.wire_bytes()).expect("the forgery lands");
    drop(file);
    drop(spine);
    assert!(matches!(
        Spine::open(&root).unwrap_err(),
        SpineRefused::ArrowBroken { .. } | SpineRefused::Occupied
    ));
}

#[test]
fn the_nursery_owns_its_addresses() {
    let root = scratch("nursery");
    let mut spine = Spine::open(&root).expect("opens");
    let bytes_a = played_state(None);
    let bytes_b = played_state(Some(ratio(1, 3)));
    let bytes_c = played_state(Some(ratio(2, 5)));
    spine.extend(coord(0), &bytes_a, None).expect("genesis");
    let mainline_head = spine.extend(coord(4), &bytes_b, None).expect("walks").clone();

    // The fork: the same state entering a new address space,
    // parent-linked to the mainline commitment (NS-5 §7.6/§9.2).
    let fork = spine.branch(&coord(4), "fork-1").expect("the nursery opens").clone();
    assert_eq!(fork.state, mainline_head.state, "the same geometry");
    assert_eq!(fork.parent, mainline_head.commit, "linked to the mainline");
    assert_eq!(fork.coordinate.branch, "fork-1");
    assert_eq!(fork.coordinate.t, 4, "the fork stands at the fork position");

    // The nursery walks on its own addresses; the mainline is
    // untouched from inside it.
    spine.extend(branch_coord("fork-1", 5), &bytes_c, None).expect("the nursery plays");
    assert_eq!(spine.head("").expect("mainline").commit, mainline_head.commit);
    assert_eq!(
        spine.resolve(&coord(4)).expect("mainline address").commit,
        mainline_head.commit,
        "the mainline is unwritable from the nursery"
    );
    spine.extend(coord(7), &bytes_c, None).expect("the mainline still walks");

    assert_eq!(
        spine.branch(&coord(4), "fork-1").unwrap_err(),
        SpineRefused::BranchTaken { name: "fork-1".into() }
    );
    assert_eq!(
        spine.branch(&coord(2), "fork-2").unwrap_err(),
        SpineRefused::ForkUnknown
    );
    assert!(matches!(
        spine.extend(branch_coord("ghost", 9), &bytes_c, None).unwrap_err(),
        SpineRefused::BranchUnknown { .. }
    ));
}

#[test]
fn the_store_and_the_record_verify() {
    let root = scratch("verify");
    let mut spine = Spine::open(&root).expect("opens");
    let bytes = played_state(None);
    let genesis = spine.extend(coord(0), &bytes, None).expect("genesis").clone();

    // Tamper the store blob: fetch re-hashes and refuses.
    let hex: String = genesis.state.iter().map(|b| format!("{b:02x}")).collect();
    let blob = root.join("store").join(format!("{hex}.bin"));
    let mut held = fs::read(&blob).expect("the blob stands");
    held[10] ^= 0xff;
    fs::write(&blob, held).expect("the tamper lands");
    assert!(matches!(spine.fetch(&genesis.state).unwrap_err(), SpineRefused::Forged { .. }));

    // Tamper the record stream: the commit does not recompute; the
    // whole file refuses on open.
    let mut stream = fs::read(root.join("spine.tlv")).expect("the stream stands");
    let at = stream.len() - 5;
    stream[at] ^= 0xff;
    fs::write(root.join("spine.tlv"), stream).expect("the tamper lands");
    drop(spine);
    assert!(matches!(Spine::open(&root).unwrap_err(), SpineRefused::Forged { .. }));
}

#[test]
fn the_judge_stands_at_the_door() {
    let root = scratch("judge");
    let mut spine = Spine::open(&root).expect("opens");
    let prior = two_orb_state((6, 1), (4, 1));
    let lawful_next = two_orb_state((3, 1), (7, 1));
    let minting_next = two_orb_state((6, 1), (6, 1));
    spine.extend(coord(0), &prior, None).expect("genesis");

    // A lawful transfer judges and lands STAMPED judged.
    let judged = spine
        .extend(coord(1), &lawful_next, Some(&transfer_step(1)))
        .expect("the lawful step enters")
        .clone();
    assert!(judged.judged, "the judgment is stamped into the commitment");

    // A minting transfer refuses with the residual carried whole.
    let refused = spine
        .extend(coord(2), &minting_next, Some(&transfer_step(2)))
        .unwrap_err();
    assert!(matches!(
        refused,
        SpineRefused::Unlawful(kernel::Unlawful::ConservationBroken { resource: Resource::Iron, .. })
    ));

    // The sealed default reaches the spine: an uncovered deed refuses.
    let uncovered = kernel::Step {
        event: ClosureEvent::ZoneRazed { orb_id: 1, tile: 0 },
        actor: 100,
        stamp: 2,
    };
    assert_eq!(
        spine.extend(coord(2), &lawful_next, Some(&uncovered)).unwrap_err(),
        SpineRefused::Unlawful(kernel::Unlawful::Unjudged)
    );

    // The unjudged hole is lawful at this phase — and STAMPED.
    let hole = spine.extend(coord(2), &minting_next, None).expect("admitted unjudged").clone();
    assert!(!hole.judged, "the coverage hole is visible in the record, never hidden");
}
