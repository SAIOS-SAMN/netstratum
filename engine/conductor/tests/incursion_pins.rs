//! INCURSION-1a pins (07 §3.1): the archived-sheet door. A sealed
//! derelict wakes as an incursion line — the archive is never edited,
//! the living are never raided, the succession law carries the
//! chronicle whole, and the door receipts on the universe ledger.

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::persist;
use cfe::world::World;
use conductor::{incursion, IncursionRefused};
use mirror::registry::Registry;
use mirror::Mirror;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("incursion-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// A sealed era holding one DERELICT (three drills, no hearth — the
/// famine cascade) and one THRIVING world.
fn sealed_era(root: &std::path::Path) -> PathBuf {
    let era = root.join("var-era1");
    // The derelict: extractors starve their own crews; every orb
    // famishes and stays famished.
    let mut doomed = World::new();
    doomed.found("first-dig").expect("founds");
    // Two drills, no hearth: the founding reserves afford exactly two
    // extractors, and their crews starve unrelieved.
    doomed.zone(0, ZoneKind::Extractor).expect("zones");
    doomed.zone(1, ZoneKind::Extractor).expect("zones");
    for _ in 0..40 {
        doomed.tick();
        let _ = doomed.collect();
    }
    assert!(
        doomed.committed().orbs.iter().all(|orb| !orb.famine.is_empty()),
        "the fixture famishes whole"
    );
    let dir = era.join("worlds").join("doomed");
    fs::create_dir_all(&dir).expect("era dir");
    persist::sync(doomed.chronicle(), &dir.join("chronicle.tlv")).expect("seals");
    // The thriving: a homestead that never starves.
    let mut alive = World::new();
    alive.found("still-warm").expect("founds");
    alive.zone(0, ZoneKind::Extractor).expect("zones");
    alive.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..8 {
        alive.tick();
    }
    let _ = alive.collect();
    let dir = era.join("worlds").join("alive");
    fs::create_dir_all(&dir).expect("era dir");
    persist::sync(alive.chronicle(), &dir.join("chronicle.tlv")).expect("seals");
    era
}

#[test]
fn the_door_wakes_the_derelict_and_only_the_derelict() {
    let root = scratch("door");
    let era = sealed_era(&root);
    let registry = Registry::at(root.join("var/registry"));
    let mirror = Mirror::at(root.join("var/worlds"));

    // The living are never raided.
    assert!(matches!(
        incursion(&era, "alive", &registry, &mirror, 60),
        Err(IncursionRefused::TheLivingStand)
    ));
    // No seal, no door.
    assert!(matches!(
        incursion(&era, "never-was", &registry, &mirror, 60),
        Err(IncursionRefused::NoSuchSeal)
    ));

    let sealed_bytes = fs::read(era.join("worlds/doomed/chronicle.tlv")).expect("reads");
    let woken = incursion(&era, "doomed", &registry, &mirror, 60).expect("the door opens");

    // The archive is NEVER edited — byte-identical after the wake.
    assert_eq!(
        fs::read(era.join("worlds/doomed/chronicle.tlv")).expect("reads"),
        sealed_bytes,
        "the seal stands untouched"
    );

    // The continuation carries the whole record + the succession.
    let receipts = woken.world.chronicle().receipts();
    assert!(receipts.iter().any(|receipt| {
        matches!(&receipt.event,
            ClosureEvent::LineSucceeded { successor } if successor == "doomed-returned")
    }), "the succession names the woken line");
    assert!(
        receipts.iter().any(|receipt| matches!(receipt.event, ClosureEvent::FamineStruck { .. })),
        "the derelict's own famine rides the continuation"
    );
    // The woken line is registered, revenant-claded.
    let line = registry.load("doomed-returned").expect("the line registers");
    assert_eq!(line.species, "revenant automaton");
    assert_eq!(line.clade, "revenant");

    // The door receipts on the universe ledger, naming all three.
    let ledger = persist::load(&root.join("var/worlds/convergence.tlv")).expect("ledger");
    assert!(ledger.receipts().iter().any(|receipt| {
        receipt.stamp == 60
            && matches!(&receipt.event,
                ClosureEvent::IncursionStruck { era, world, line }
                    if era == "var-era1" && world == "doomed" && line == "doomed-returned")
    }));

    // One wake per seal.
    assert!(matches!(
        incursion(&era, "doomed", &registry, &mirror, 61),
        Err(IncursionRefused::AlreadyWoken)
    ));
}
