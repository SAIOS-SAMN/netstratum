//! NOVEL-1 witnesses — the succession law (10 §3c): a successor line
//! inherits a CHRONICLE, not a tree. The deed refuses on a living
//! line; the successor's conditions evaluate over receipts that
//! include the predecessor's whole history; its node ids seed from
//! roots that CONTAIN that history — novel by construction, never
//! overwriting a node the predecessor held; the kernel (the world's
//! own sun) stays with the world; replay MATCHES.

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// The predecessor: the TECH-1b Frost arc — a cold-sited habitat with
/// no fuel strikes joule famine and survives it to insulation
/// (condition 1 at the year voice once the span reaches 480). The
/// famine stands unrelieved, so the world ends the arc DERELICT in
/// the v1 shape: every orb famished (the cascade, 07 §3 triage).
fn derelict_with_insulation() -> World {
    let mut world = World::new();
    world.found("dimlight-hold").expect("founds");
    world.commission_star(StarClass::M).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1), None).expect("sites cold");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 1..=541u32 {
        world.tick();
    }
    assert!(
        world.techs().iter().any(|(_, condition, _)| *condition == 1),
        "the predecessor earned its insulation"
    );
    assert!(
        !world.orb().expect("stands").famine.is_empty(),
        "the famine stands — the line is expiring"
    );
    world
}

#[test]
fn succession_refuses_on_a_living_line() {
    let mut world = World::new();
    world.found("hale-hold").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    let receipts_before = world.chronicle().receipts().len();
    let refusal = world.succeed("mycelial-chord").expect_err("a fed line is not succeeded");
    assert_eq!(refusal, ActionRefused::LineStillLiving { orbs: 1, famished: 0 });
    assert_eq!(
        world.chronicle().receipts().len(),
        receipts_before,
        "a refusal orphans nothing — no deed, no receipt"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_successor_inherits_the_chronicle_not_the_tree() {
    let mut world = derelict_with_insulation();
    let predecessor_node = world
        .techs()
        .iter()
        .find(|(_, condition, _)| *condition == 1)
        .map(|(_, _, node)| *node)
        .expect("held above");
    let kernel = world
        .techs()
        .iter()
        .find(|(_, condition, _)| *condition == 0)
        .map(|(_, _, node)| *node)
        .expect("the sited world knows its sun");

    world.succeed("mycelial-chord").expect("the derelict admits a successor");

    // The tree is NOT inherited: the living tree drops to the kernel
    // alone — the sun is known to the WORLD, not to the line.
    assert_eq!(
        world.techs().iter().map(|(_, condition, _)| *condition).collect::<Vec<_>>(),
        vec![0],
        "the successor starts with the world's kernel and nothing else"
    );
    assert!(
        world.techs().iter().any(|(_, _, node)| *node == kernel),
        "the kernel node is the SAME node — the world's own fact"
    );

    // The chronicle IS inherited: the famine span accumulated by the
    // predecessor stands in the deposit sheet, so the successor
    // crystallizes insulation at its FIRST year boundary — no new
    // famine-years paid after the deed.
    for _ in 0..60 {
        world.tick();
    }
    let successor_node = world
        .techs()
        .iter()
        .find(|(_, condition, _)| *condition == 1)
        .map(|(_, _, node)| *node)
        .expect("the inherited span crystallizes for the successor");
    assert_ne!(
        successor_node, predecessor_node,
        "novel by construction — the root contains the deed and the whole history"
    );

    // Never overwriting: BOTH discoveries stand in the chronicle
    // forever; the deed stands between them.
    let discovered: Vec<[u8; 32]> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|receipt| match &receipt.event {
            ClosureEvent::TechDiscovered { condition: 1, node, .. } => Some(*node),
            _ => None,
        })
        .collect();
    assert_eq!(
        discovered,
        vec![predecessor_node, successor_node],
        "two insulations, two lines, one chronicle"
    );
    assert!(
        world.chronicle().receipts().iter().any(|receipt| matches!(
            &receipt.event,
            ClosureEvent::LineSucceeded { successor } if successor == "mycelial-chord"
        )),
        "the deed is receipted"
    );

    // The chain still holds after the lens: replay and resume both
    // rebuild the succession boundary exactly.
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "the succession boundary is a replay fact"
    );
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.techs(), world.techs(), "resume carries the living tree");
}

#[test]
fn a_second_succession_moves_the_boundary_again() {
    let mut world = derelict_with_insulation();
    world.succeed("mycelial-chord").expect("first succession");
    for _ in 0..60 {
        world.tick();
    }
    assert_eq!(world.techs().len(), 2, "kernel + the successor's insulation");
    // The successor's line also expires (the famine was never
    // relieved); a third line takes the chronicle.
    world.succeed("lichen-verdict").expect("the cascade still stands");
    assert_eq!(
        world.techs().iter().map(|(_, condition, _)| *condition).collect::<Vec<_>>(),
        vec![0],
        "each succession clears the tree back to the world's kernel"
    );
    for _ in 0..60 {
        world.tick();
    }
    let insulations: Vec<[u8; 32]> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|receipt| match &receipt.event {
            ClosureEvent::TechDiscovered { condition: 1, node, .. } => Some(*node),
            _ => None,
        })
        .collect();
    assert_eq!(insulations.len(), 3, "three lines, three insulations, all standing");
    assert_eq!(
        insulations.iter().collect::<std::collections::BTreeSet<_>>().len(),
        3,
        "every node id is novel — no root repeats"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_successor_seals_its_own_axes() {
    // The audit finding closed (G1 × NOVEL-1): the deed releases the
    // GENESIS with the tree — the mushroom does not eat what the
    // elephant ate. The predecessor's seal stands in the chronicle
    // forever (the record law); the LIVING seal is the successor's to
    // make, entry in the bands, on the inherited world.
    let mut world = derelict_with_insulation();
    world
        .seal_genesis("elephant-line", 1, 2, 1)
        .expect("the predecessor's axes seal");
    world.succeed("mycelial-chord").expect("the derelict admits a successor");
    assert_eq!(
        world.genesis(),
        None,
        "the deed releases the seal — a line's axes never outlive the line"
    );
    world
        .seal_genesis("mycelial-chord", 3, 2, 2)
        .expect("the successor seals ANEW — one living line per world");
    assert_eq!(world.genesis().expect("sealed").civilization, "mycelial-chord");
    // Both seals stand in the chronicle forever; the living one rules.
    let seals: Vec<String> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|receipt| match &receipt.event {
            ClosureEvent::GenesisSealed { civilization, .. } => Some(civilization.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(seals, vec!["elephant-line".to_string(), "mycelial-chord".to_string()]);
    assert_eq!(replay(world.chronicle()), world.committed());
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.genesis(), world.genesis());
}

#[test]
fn the_deed_round_trips_byte_exact() {
    let deed = ClosureEvent::LineSucceeded { successor: "mycelial-chord".to_string() };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 59, "the claimed tag");
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
}
