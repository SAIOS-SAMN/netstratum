//! G1 witnesses — GENESIS + DISPOSITION (04 §1, §5, §6): the axes seal
//! as a WORLD-CHRONICLE receipt (04 §5 binding 2 — the registry is a
//! cache of receipted truth, never ground truth); genesis is entry in
//! the bands (04 §6a — never a tier-0 seating); the disposition is a
//! value weighting over the same receipts, so identical arcs score
//! differently under different seals; an unsealed world reads None —
//! no receipt, no read.

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World};

/// Axis wire codes (the registry vocabulary, closed and append-only):
/// dispositions 1 expansionist ‖ 2 empiricist ‖ 3 epicurean;
/// metabolisms 1 mineral ‖ 2 biotic ‖ 3 plasmic;
/// perceptions 1 planar ‖ 2 volumetric ‖ 3 harmonic ‖ 4 umbral.
const EXPANSIONIST: u8 = 1;
const EPICUREAN: u8 = 3;
const BIOTIC: u8 = 2;
const PLANAR: u8 = 1;

/// One short arc, identical wherever it plays: found, two zones, four
/// ticks, collect — the same receipts for every disposition to weigh.
fn played_arc(disposition: u8) -> World {
    let mut world = World::new();
    world.found("weighed-hold").expect("founds");
    world
        .seal_genesis("deep-chorus", disposition, BIOTIC, PLANAR)
        .expect("the axes seal at founding");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    world
}

#[test]
fn genesis_seals_at_founding_and_replays() {
    let world = played_arc(EXPANSIONIST);
    let genesis = world.genesis().expect("sealed");
    assert_eq!(genesis.civilization, "deep-chorus");
    assert_eq!(
        (genesis.disposition, genesis.metabolism, genesis.perception),
        (EXPANSIONIST, BIOTIC, PLANAR)
    );
    assert!(
        world.chronicle().receipts().iter().any(|receipt| matches!(
            &receipt.event,
            ClosureEvent::GenesisSealed { civilization, .. } if civilization == "deep-chorus"
        )),
        "the binding copy is the receipt in the world's chronicle"
    );
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "genesis is a replay fact"
    );
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.genesis(), world.genesis(), "resume carries the seal");
}

#[test]
fn a_second_seal_refuses_and_orphans_nothing() {
    let mut world = played_arc(EXPANSIONIST);
    let receipts_before = world.chronicle().receipts().len();
    let refusal = world
        .seal_genesis("late-claimant", EPICUREAN, BIOTIC, PLANAR)
        .expect_err("one line per world");
    assert_eq!(refusal, ActionRefused::GenesisAlreadySealed);
    assert_eq!(world.chronicle().receipts().len(), receipts_before);
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn an_unknown_axis_refuses_typed_and_pure() {
    let mut world = World::new();
    world.found("babel-hold").expect("founds");
    let receipts_before = world.chronicle().receipts().len();
    let refusal = world
        .seal_genesis("babel", 9, BIOTIC, PLANAR)
        .expect_err("the vocabulary is closed");
    assert_eq!(refusal, ActionRefused::UnknownAxisValue { axis: 1, code: 9 });
    assert_eq!(
        world.chronicle().receipts().len(),
        receipts_before,
        "a refusal orphans nothing"
    );
}

#[test]
fn genesis_needs_a_colony_first() {
    // The N3 law holds here too: identity precedes the colony — on an
    // unfounded world the refusal is "no entity", not "no colony"
    // (containment precedes the world, NS-6 §1.2).
    let mut world = World::new();
    let refusal = world
        .seal_genesis("eager", EXPANSIONIST, BIOTIC, PLANAR)
        .expect_err("entry is in the bands, on a founded world");
    assert_eq!(refusal, ActionRefused::NoSuchEntity { entity: 1 });
}

#[test]
fn the_scored_read_diverges_by_disposition_alone() {
    // The same receipts, two seals: 2 zones built, 1 tech node (none
    // here), collected yield. The weights are the placeholder tables
    // in dop.rs (exact rationals summing to 1); the PIN is that the
    // two reads DIFFER while the physics is identical — three civs on
    // identical maps see three different winning numbers (04 §4).
    let expansionist = played_arc(EXPANSIONIST);
    let epicurean = played_arc(EPICUREAN);
    let read_a = expansionist.scored_progress().expect("sealed reads read");
    let read_b = epicurean.scored_progress().expect("sealed reads read");
    assert_ne!(
        read_a, read_b,
        "identical physics, different worth — the disposition weighs, never rules"
    );
    assert!(read_a > ratio(0, 1) && read_b > ratio(0, 1));
}

#[test]
fn an_unsealed_world_reads_none() {
    let mut world = World::new();
    world.found("silent-hold").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    assert_eq!(
        world.scored_progress(),
        None,
        "no receipt, no read — identity is never law (04 §5 binding 2)"
    );
}

#[test]
fn the_seal_round_trips_byte_exact() {
    let deed = ClosureEvent::GenesisSealed {
        civilization: "deep-chorus".to_string(),
        disposition: EXPANSIONIST,
        metabolism: BIOTIC,
        perception: PLANAR,
    };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 61, "the claimed tag");
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
}
