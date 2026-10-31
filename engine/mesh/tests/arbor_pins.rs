//! SPINE-1 pins (NS-5 §9): the arbor walks and reloads whole; the
//! arrow and the address hold structurally; the nursery owns its own
//! addresses; the store verifies on every read; and the gate stands
//! at the door — lawful passes, unlawful refuses, the sealed default
//! refuses, and the unjudged hole is stamped, never hidden.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{World, SOVEREIGN};
use mesh::arbor::{Arbor, ArborRefused, Coordinate};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("netstratum-arbor-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// A mainline coordinate at tick `t`, sub-tick 0 (unchartered S, base gauge).
fn coord(t: u64) -> Coordinate {
    Coordinate {
        branch: String::new(),
        s: [ratio(0, 1), ratio(0, 1), ratio(0, 1)],
        g: ratio(0, 1),
        t,
        sub: 0,
    }
}

/// A mainline coordinate at tick `t`, an explicit sub-tick — the
/// Phase-3 flip's addressing law (`39_PHASE3_FLIP` §3): several deeds
/// share one `t`, distinguished by `sub`.
fn coord_at(t: u64, sub: u32) -> Coordinate {
    Coordinate { sub, ..coord(t) }
}

fn branch_coord(branch: &str, t: u64) -> Coordinate {
    Coordinate {
        branch: branch.into(),
        ..coord(t)
    }
}

/// One orb holding `iron` iron — the gate's test material.
fn orb(id: u64, iron: (i64, i64)) -> ColonyOrb {
    let mut held = ColonyOrb {
        orb_id: id,
        name: format!("orb-{id}"),
        ..Default::default()
    };
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
        Entity {
            entity_id: SOVEREIGN,
            handle: "sovereign".into(),
            principal: None,
            grants: vec![],
        },
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
    world
        .zone(0, cfe::dop::ZoneKind::Extractor)
        .expect("builds");
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
    let mut arbor = Arbor::open(&root).expect("opens empty");
    let bytes_a = played_state(None);
    let bytes_b = played_state(Some(ratio(1, 3)));

    let genesis = arbor
        .extend(coord(0), &bytes_a, None)
        .expect("genesis")
        .clone();
    assert_eq!(genesis.parent, [0u8; 32], "genesis is parentless");
    // The record repeats the blob's own leading byte — read against
    // the blob, not a frozen version constant (re-pinned 2026-08-12:
    // the codec is at v19 now, and every schema bump since v5 broke a
    // hardcoded assertion; the law is "the record echoes the blob").
    assert_eq!(
        genesis.schema, bytes_a[0],
        "the record repeats the blob's form"
    );
    assert_eq!(genesis.kernel, kernel::VERSION);
    assert!(!genesis.judged, "genesis is honestly unjudged");

    let second = arbor
        .extend(coord(4), &bytes_b, None)
        .expect("walks")
        .clone();
    assert_eq!(second.parent, genesis.commit, "the path links");
    assert_eq!(
        arbor.resolve(&coord(4)).expect("resolves").commit,
        second.commit,
        "one coordinate, one commitment"
    );
    assert_eq!(
        arbor.fetch(&second.state).expect("the geometry unpacks"),
        bytes_b
    );

    // A step offered at genesis refuses: nothing prior to gate from.
    let fresh = scratch("walks-genesis");
    let mut fresh_spine = Arbor::open(&fresh).expect("opens");
    let refused = fresh_spine
        .extend(coord(0), &bytes_a, Some(&transfer_step(0)))
        .unwrap_err();
    assert_eq!(refused, ArborRefused::GenesisJudged);

    // SEAL B for the arbor: reload from disk, identical maps.
    drop(arbor);
    let reloaded = Arbor::open(&root).expect("reloads whole");
    assert_eq!(
        reloaded.head("").expect("head stands").commit,
        second.commit
    );
    assert_eq!(
        reloaded.resolve(&coord(0)).expect("genesis stands").commit,
        genesis.commit
    );
    assert_eq!(
        reloaded.fetch(&second.state).expect("store survives"),
        bytes_b
    );
}

#[test]
fn the_arrow_and_the_address_hold() {
    let root = scratch("arrow");
    let mut arbor = Arbor::open(&root).expect("opens");
    let bytes = played_state(None);
    arbor.extend(coord(0), &bytes, None).expect("genesis");
    arbor.extend(coord(4), &bytes, None).expect("walks");

    // One value per address: the exact head address refuses occupied;
    // the clock only walks forward: a backward tick refuses the arrow.
    assert_eq!(
        arbor.extend(coord(4), &bytes, None).unwrap_err(),
        ArborRefused::Occupied
    );
    assert!(matches!(
        arbor.extend(coord(2), &bytes, None).unwrap_err(),
        ArborRefused::ArrowBroken { .. }
    ));

    // A hand-forged record at an occupied address — valid commit, real
    // parent — refuses the whole file on open: one value per address
    // is structural, not policed.
    let head = arbor.head("").expect("stands").clone();
    let forged = mesh::arbor::seal(
        coord(4),
        head.schema,
        head.kernel,
        false,
        head.state,
        head.commit,
    );
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(root.join("arbor.tlv"))
        .expect("the file opens");
    use std::io::Write as _;
    file.write_all(&forged.wire_bytes())
        .expect("the forgery lands");
    drop(file);
    drop(arbor);
    assert!(matches!(
        Arbor::open(&root).unwrap_err(),
        ArborRefused::ArrowBroken { .. } | ArborRefused::Occupied
    ));
}

#[test]
fn the_nursery_owns_its_addresses() {
    let root = scratch("nursery");
    let mut arbor = Arbor::open(&root).expect("opens");
    let bytes_a = played_state(None);
    let bytes_b = played_state(Some(ratio(1, 3)));
    let bytes_c = played_state(Some(ratio(2, 5)));
    arbor.extend(coord(0), &bytes_a, None).expect("genesis");
    let mainline_head = arbor
        .extend(coord(4), &bytes_b, None)
        .expect("walks")
        .clone();

    // The fork: the same state entering a new address space,
    // parent-linked to the mainline commitment (NS-5 §7.6/§9.2).
    let fork = arbor
        .branch(&coord(4), "fork-1")
        .expect("the nursery opens")
        .clone();
    assert_eq!(fork.state, mainline_head.state, "the same geometry");
    assert_eq!(fork.parent, mainline_head.commit, "linked to the mainline");
    assert_eq!(fork.coordinate.branch, "fork-1");
    assert_eq!(fork.coordinate.t, 4, "the fork stands at the fork position");

    // The nursery walks on its own addresses; the mainline is
    // untouched from inside it.
    arbor
        .extend(branch_coord("fork-1", 5), &bytes_c, None)
        .expect("the nursery plays");
    assert_eq!(
        arbor.head("").expect("mainline").commit,
        mainline_head.commit
    );
    assert_eq!(
        arbor.resolve(&coord(4)).expect("mainline address").commit,
        mainline_head.commit,
        "the mainline is unwritable from the nursery"
    );
    arbor
        .extend(coord(7), &bytes_c, None)
        .expect("the mainline still walks");

    assert_eq!(
        arbor.branch(&coord(4), "fork-1").unwrap_err(),
        ArborRefused::BranchTaken {
            name: "fork-1".into()
        }
    );
    assert_eq!(
        arbor.branch(&coord(2), "fork-2").unwrap_err(),
        ArborRefused::ForkUnknown
    );
    assert!(matches!(
        arbor
            .extend(branch_coord("ghost", 9), &bytes_c, None)
            .unwrap_err(),
        ArborRefused::BranchUnknown { .. }
    ));
}

#[test]
fn the_store_and_the_record_verify() {
    let root = scratch("verify");
    let mut arbor = Arbor::open(&root).expect("opens");
    let bytes = played_state(None);
    let genesis = arbor
        .extend(coord(0), &bytes, None)
        .expect("genesis")
        .clone();

    // Tamper the store blob: fetch re-hashes and refuses.
    let hex: String = genesis.state.iter().map(|b| format!("{b:02x}")).collect();
    let blob = root.join("store").join(format!("{hex}.bin"));
    let mut held = fs::read(&blob).expect("the blob stands");
    held[10] ^= 0xff;
    fs::write(&blob, held).expect("the tamper lands");
    assert!(matches!(
        arbor.fetch(&genesis.state).unwrap_err(),
        ArborRefused::Forged { .. }
    ));

    // Tamper the record stream: the commit does not recompute; the
    // whole file refuses on open.
    let mut stream = fs::read(root.join("arbor.tlv")).expect("the stream stands");
    let at = stream.len() - 5;
    stream[at] ^= 0xff;
    fs::write(root.join("arbor.tlv"), stream).expect("the tamper lands");
    drop(arbor);
    assert!(matches!(
        Arbor::open(&root).unwrap_err(),
        ArborRefused::Forged { .. }
    ));
}

#[test]
fn the_judge_stands_at_the_door() {
    let root = scratch("judge");
    let mut arbor = Arbor::open(&root).expect("opens");
    let prior = two_orb_state((6, 1), (4, 1));
    let lawful_next = two_orb_state((3, 1), (7, 1));
    let minting_next = two_orb_state((6, 1), (6, 1));
    arbor.extend(coord(0), &prior, None).expect("genesis");

    // A lawful transfer gates and lands stamped gated.
    let judged = arbor
        .extend(coord(1), &lawful_next, Some(&transfer_step(1)))
        .expect("the lawful step enters")
        .clone();
    assert!(judged.judged, "the judgment is stamped into the commitment");

    // A minting transfer refuses with the residual carried whole.
    let refused = arbor
        .extend(coord(2), &minting_next, Some(&transfer_step(2)))
        .unwrap_err();
    assert!(matches!(
        refused,
        ArborRefused::Refused(kernel::Refused::ConservationBroken {
            resource: Resource::Iron,
            ..
        })
    ));

    // A judged refusal reaches the arbor and refuses. Witness
    // truth-update (recorded): this block rode the last-uncovered deed
    // (ZoneRazed → EraAnnexed → AssayStruck → TaxonSealed) to prove
    // the sealed default passed through. With coverage now total (all
    // 64 deeds gated, kernel `taxon_pins.rs`), there is no Unjudged
    // deed to carry — so the pin shows the arbor carrying a
    // containment refusal instead: TaxonSealed needs Build, and the
    // Collect-hand (100) lacks it, so the kernel's NotPermitted rides
    // the arbor to a typed refusal, the verdict never swallowed.
    let refused = kernel::Step {
        event: ClosureEvent::TaxonSealed { class: 1, clade: 1 },
        actor: 100,
        stamp: 2,
    };
    assert_eq!(
        arbor
            .extend(coord(2), &lawful_next, Some(&refused))
            .unwrap_err(),
        ArborRefused::Refused(kernel::Refused::NotPermitted {
            actor: 100,
            scope: cfe::ipsal::Scope::Build,
        })
    );

    // A step-less extend still stamps gated = false (the pre-chain
    // adoption shape — no judgment claimed, honestly recorded).
    let hole = arbor
        .extend(coord(2), &minting_next, None)
        .expect("admitted unjudged")
        .clone();
    assert!(
        !hole.judged,
        "no judgment claimed is visible in the record, never hidden"
    );
}

/// The commitment's schema field speaks the blob's leading byte — the
/// record never misdeclares the form it binds (the codec monopoly:
/// one component owns the form; the arbor repeats it, never asserts
/// its own).
#[test]
fn the_record_speaks_the_blobs_schema() {
    let root = scratch("schema-truth");
    let mut arbor = Arbor::open(&root).expect("opens empty");
    let bytes = played_state(None);
    let genesis = arbor
        .extend(coord(0), &bytes, None)
        .expect("genesis")
        .clone();
    assert_eq!(
        genesis.schema, bytes[0],
        "the record's schema is the blob's leading byte, never a constant"
    );
    assert_eq!(
        genesis.schema,
        codec::SCHEMA_V26,
        "the codec emits v26 today — GALACTIC-1 appended region_seed \
         (a deliberate version tripwire: bump this WITH the codec)"
    );
}

/// The adoption door (#36): a played, pre-chain world enters the
/// mainline once — the debt stamped unjudged, never hidden; a second
/// adoption refuses; and the chained economy continues on top through
/// the ordinary gated door.
#[test]
fn adoption_is_once_and_names_its_debt() {
    let root = scratch("adoption");
    let mut arbor = Arbor::open(&root).expect("opens empty");

    // A world with pre-chain history: its chronicle is the Tier-A
    // debt. Adoption seats its current state, gated=0 stamped.
    let debt = two_orb_state((10, 1), (0, 1));
    let adopted = arbor
        .adopt(coord(1), &debt)
        .expect("the debt is adopted")
        .clone();
    assert_eq!(
        adopted.parent, [0u8; 32],
        "adoption is the mainline's birth"
    );
    assert!(
        !adopted.judged,
        "the pre-chain debt is stamped unjudged, named never hidden"
    );

    // One truth, one birth: the door shuts behind the first pass.
    assert!(matches!(
        arbor.adopt(coord(2), &debt).unwrap_err(),
        ArborRefused::AlreadyAdopted
    ));
    // A nursery is born by `branch`, never adopted.
    assert!(matches!(
        arbor.adopt(branch_coord("nursery", 3), &debt).unwrap_err(),
        ArborRefused::BranchUnknown { .. }
    ));

    // Life after adoption: the ordinary judged door carries the
    // economy — a lawful transfer chains on top of the adopted state.
    let next = two_orb_state((7, 1), (3, 1));
    let step = transfer_step(2);
    let chained = arbor
        .extend(coord(2), &next, Some(&step))
        .expect("judged")
        .clone();
    assert!(
        chained.judged,
        "the step after adoption is judged, the flag stamped"
    );
    assert_eq!(
        chained.parent, adopted.commit,
        "the path links through the door"
    );
}

/// **The PHASE-3 addressing law** (`39_PHASE3_FLIP` §3, operator
/// ruling 2026-08-13, option A): several deeds sharing one world tick
/// commit as separate records, addressed by an ascending `sub` — not
/// one commitment per beat, one per deed.
#[test]
fn several_deeds_share_one_tick_addressed_by_sub() {
    let root = scratch("sub-index");
    let mut arbor = Arbor::open(&root).expect("opens");
    let prior = two_orb_state((6, 1), (4, 1));
    let after_first = two_orb_state((3, 1), (7, 1));
    let after_second = two_orb_state((0, 1), (10, 1));
    arbor.extend(coord(0), &prior, None).expect("genesis");

    // Two deeds, same tick (t=1), distinct sub — both must land.
    let first = arbor
        .extend(coord_at(1, 0), &after_first, Some(&transfer_step(1)))
        .expect("the first deed at t=1 commits")
        .clone();
    assert!(first.judged);

    let second = arbor
        .extend(coord_at(1, 1), &after_second, Some(&transfer_step(1)))
        .expect("a second deed at the SAME t, next sub, commits")
        .clone();
    assert!(second.judged);
    assert_eq!(
        second.parent, first.commit,
        "the second deed chains off the first"
    );

    // The old law — one record per tick — is gone: a second deed at
    // t=1 sub=0 (repeating the first's exact address) still refuses
    // occupied, but sub=1 no longer collides the way a same-t retry
    // used to (measured, pre-flip: "per-deed commitment refuses
    // Occupied on the second deed of any beat").
    assert_eq!(
        arbor
            .extend(coord_at(1, 0), &after_first, None)
            .unwrap_err(),
        ArborRefused::Occupied
    );

    // A third tick, sub back at 0, still ascends cleanly on top of
    // (t=1, sub=1) — the arrow reads (t, sub) lexicographically, not
    // sub alone.
    let third = arbor
        .extend(coord_at(2, 0), &prior, None)
        .expect("a fresh tick, sub 0, still ascends past (1, 1)")
        .clone();
    assert_eq!(third.parent, second.commit);

    // And sub cannot regress within a held tick even on a fresh open.
    drop(arbor);
    let mut reopened = Arbor::open(&root).expect("reopens and reverifies the whole stream");
    assert!(matches!(
        reopened.extend(coord_at(2, 0), &prior, None).unwrap_err(),
        ArborRefused::Occupied
    ));
    assert!(matches!(
        // A fresh address (t=1, sub=5 was never committed) that is
        // still behind the head (t=2, sub=0) — Occupied cannot fire
        // (the address is free); the arrow catches it instead.
        reopened
            .extend(coord_at(1, 5), &after_first, None)
            .unwrap_err(),
        ArborRefused::ArrowBroken { .. }
    ));
}

#[test]
fn the_judged_door_carries_a_verdict_only_from_the_head_it_was_reached_on() {
    // Reconstruct (iii): the judge's prior must be the head (by digest);
    // then the arbor appends judged without fetching, decoding or
    // re-admitting. A prior the arbor does not hold refuses.
    let root = scratch("judged-door");
    let mut arbor = Arbor::open(&root).unwrap();
    let genesis = two_orb_state((10, 1), (0, 1));
    let next = two_orb_state((7, 1), (3, 1));
    let head = arbor.extend(coord(0), &genesis, None).unwrap().clone();
    assert_eq!(head.state, *blake3::hash(&genesis).as_bytes());

    let stranger = *blake3::hash(&next).as_bytes();
    assert!(matches!(
        arbor.extend_judged(coord(1), &next, stranger),
        Err(ArborRefused::PriorDiverged)
    ));
    let committed = arbor
        .extend_judged(coord(1), &next, head.state)
        .unwrap()
        .clone();
    assert!(committed.judged, "a carried verdict is stamped judged");
    assert_eq!(committed.parent, head.commit);
    assert_eq!(arbor.fetch(&committed.state).unwrap(), next);
    // the arrow and the address still hold at this door
    assert!(matches!(
        arbor.extend_judged(coord(1), &genesis, committed.state),
        Err(ArborRefused::Occupied)
    ));
    assert!(arbor
        .extend_judged(coord(0), &genesis, committed.state)
        .is_err());
    // and it reloads whole
    drop(arbor);
    let back = Arbor::open(&root).unwrap();
    assert_eq!(back.head("").unwrap().state, committed.state);
}

#[test]
fn a_heads_only_arbor_verifies_whole_and_holds_one_record_a_branch() {
    // The memory bound: the live judge reads heads alone.
    let root = scratch("heads-only");
    let mut full = Arbor::open(&root).unwrap();
    let genesis = two_orb_state((10, 1), (0, 1));
    full.extend(coord(0), &genesis, None).unwrap();
    let mut prior = full.head("").unwrap().state;
    for t in 1..6u64 {
        let next = two_orb_state((10 - t as i64, 1), (t as i64, 1));
        prior = full.extend_judged(coord(t), &next, prior).unwrap().state;
    }
    let head = full.head("").unwrap().clone();
    drop(full);
    let mut heads = Arbor::open_heads(&root).unwrap();
    assert_eq!(heads.records().len(), 1, "one record a branch");
    assert_eq!(heads.head("").unwrap(), &head);
    let next = two_orb_state((1, 1), (9, 1));
    let appended = heads
        .extend_judged(coord(6), &next, head.state)
        .unwrap()
        .clone();
    assert_eq!(heads.records().len(), 1, "an append replaces the head");
    assert!(
        matches!(
            heads.extend_judged(coord(6), &next, appended.state),
            Err(ArborRefused::Occupied | ArborRefused::ArrowBroken { .. })
        ),
        "a position is never written twice"
    );
    drop(heads);
    // the file stayed whole: a full open walks all seven
    assert_eq!(Arbor::open(&root).unwrap().records().len(), 7);
}
