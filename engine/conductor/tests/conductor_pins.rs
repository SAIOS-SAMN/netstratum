//! The conductor pins: an era is deterministic, the board comes alive
//! without a human, policies diverge, the steward leash closes
//! end-to-end, research unlocks industry, and a thriving line
//! speciates — chronicle-chosen, lineage-recorded.

use conductor::{founding_bots, load_bots, run_era, write_default_bots, BotWorld};
use mirror::registry::Registry;
use mirror::Mirror;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "conductor-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The board lives: after an era every founding world has receipts, a
/// linked mirrored chain, and confirmed solves with depth rungs — no
/// human involved.
#[test]
fn an_era_brings_the_board_alive() {
    let root = scratch("alive");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 12);
    assert!(worlds.len() >= 3);

    for bot in worlds.iter().take(3) {
        assert!(bot.world.now() >= 12, "the clock advanced");
        assert!(
            bot.world.chronicle().receipts().len() > 5,
            "{}: receipts accumulated",
            bot.spec.civilization.slug
        );
        assert!(bot.solved > 0, "{}: research confirmed solves", bot.spec.civilization.slug);
        let world_dir = root.join("worlds").join(&bot.spec.world_name);
        assert!(world_dir.join("chronicle.jsonl").exists());
        let solves = fs::read_dir(world_dir.join("solves")).expect("solves dir").count();
        assert!(solves > 0, "solve certificates mirrored");
    }
    let registry = Registry::at(root.join("registry"));
    assert!(registry.list().len() >= 3, "the registry filled itself");
}

/// Determinism: two eras from genesis produce byte-identical
/// chronicles — including any speciations (no wall clock, no ambient
/// entropy anywhere, even in evolution).
#[test]
fn eras_are_deterministic() {
    let (root_a, root_b) = (scratch("det-a"), scratch("det-b"));
    let a = run_era(&root_a.to_string_lossy(), 40);
    let b = run_era(&root_b.to_string_lossy(), 40);
    assert_eq!(a.len(), b.len(), "same era, same population");
    for (x, y) in a.iter().zip(&b) {
        assert_eq!(
            x.world.chronicle().receipts(),
            y.world.chronicle().receipts(),
            "{}: same era, same receipts, byte-for-byte",
            x.spec.civilization.slug
        );
    }
}

/// The policies actually diverge — different temperaments, different
/// cities by mid-era.
#[test]
fn policies_diverge() {
    let root = scratch("diverge");
    let worlds = run_era(&root.to_string_lossy(), 15);
    let zones: Vec<usize> = worlds
        .iter()
        .map(|b| b.world.orb().map(|o| o.zones.len()).unwrap_or(0))
        .collect();
    assert!(
        zones.windows(2).any(|w| w[0] != w[1]),
        "temperaments produce different cities: {zones:?}"
    );
}

/// RESEARCH UNLOCKS INDUSTRY: rung 3 opens smelters; by era's end a
/// rich civilization is smelting — the pyramid loop closed (surplus →
/// research → technique → industry).
#[test]
fn research_unlocks_industry() {
    let root = scratch("unlock");
    let worlds = run_era(&root.to_string_lossy(), 45);
    let smelting = worlds.iter().any(|bot| {
        bot.level >= 3
            && bot.world.orb().is_some_and(|orb| {
                orb.zones
                    .iter()
                    .any(|z| z.kind == cfe::dop::ZoneKind::Smelter)
            })
    });
    assert!(smelting, "an earned rung turned surplus into smelters");
}

/// SPECIATION: a long era mints at least one daughter — lineage
/// recorded in the registry, a mutated axis, a new world beating
/// alongside its parent.
#[test]
fn a_thriving_line_speciates() {
    let root = scratch("speciate");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 60);
    assert!(worlds.len() > 3, "a daughter joined the board");

    let registry = Registry::at(root.join("registry"));
    let daughters: Vec<_> = registry
        .list()
        .into_iter()
        .filter(|c| c.lineage.is_some())
        .collect();
    assert!(!daughters.is_empty(), "lineage recorded in the registry");

    for daughter in &daughters {
        let parent = registry
            .load(daughter.lineage.as_ref().expect("lineage"))
            .expect("the parent exists");
        let mutations = [
            daughter.disposition != parent.disposition,
            daughter.metabolism != parent.metabolism,
            daughter.perception != parent.perception,
        ]
        .iter()
        .filter(|m| **m)
        .count();
        assert_eq!(mutations, 1, "exactly one axis mutates per speciation");
        assert!(daughter.species.contains("descendant"));
    }

    // The daughter's world is mirrored and beating.
    let newborn = worlds.get(3).expect("newborn world");
    assert!(newborn.world.now() > 0, "the daughter's clock runs");
    assert!(root
        .join("worlds")
        .join(&newborn.spec.world_name)
        .join("chronicle.jsonl")
        .exists());
}

/// The leash works end-to-end: a steward directive queued in the
/// inbox applies through the contained steward entity on the next
/// heartbeat; an unknown verb refuses. Both archive.
#[test]
fn a_steward_directive_lands_through_the_leash() {
    let root = scratch("steward");
    let var = root.to_string_lossy().into_owned();
    let registry = Registry::at(format!("{var}/registry"));
    let mirror = Mirror::at(format!("{var}/worlds"));

    let mut worlds: Vec<BotWorld> = founding_bots()
        .into_iter()
        .map(|spec| BotWorld::genesis(spec, &registry))
        .collect();
    for _ in 0..2 {
        for bot in &mut worlds {
            bot.cycle(&mirror, &registry);
        }
    }

    let inbox = mirror.world_dir("cinder-reach").join("inbox");
    fs::write(
        inbox.join("000001-zone.json"),
        r#"{"verb":"zone","args":["63","hab"],"as":"steward"}"#,
    )
    .expect("queues");
    fs::write(
        inbox.join("000002-smite.json"),
        r#"{"verb":"smite","args":[],"as":"steward"}"#,
    )
    .expect("queues");

    for bot in &mut worlds {
        bot.cycle(&mirror, &registry);
    }

    assert!(inbox.join("applied/000001-zone.json").exists(), "the zone applied");
    assert!(inbox.join("refused/000002-smite.json").exists(), "the smite refused");
    assert!(
        worlds[0]
            .world
            .orb()
            .expect("orb")
            .zones
            .iter()
            .any(|z| z.tile == 63),
        "tile 63 carries the steward's habitat"
    );
}

/// Lore lives in data: the default bots seed a file the operator
/// owns; loading it round-trips the specs; a hand-added entry loads.
#[test]
fn bots_load_from_the_operators_file() {
    let root = scratch("bots");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("bots.json");
    write_default_bots(&path).expect("seeds");
    let loaded = load_bots(&path).expect("loads");
    assert_eq!(loaded.len(), founding_bots().len());
    assert_eq!(loaded[0].civilization.slug, "slagborn");

    // Seeding again never clobbers the operator's file.
    let before = fs::read_to_string(&path).expect("read");
    write_default_bots(&path).expect("no-op");
    assert_eq!(before, fs::read_to_string(&path).expect("read"));

    // A hand-added entry loads alongside the defaults.
    let extended = before.replace(
        "]",
        r#",{"name":"Fernwake","slug":"fernwake","species":"monarch butterfly",
           "motto":"drift and arrive","disposition":"epicurean","metabolism":"biotic",
           "perception":"volumetric","world":"lantern-sea","planet":"petal",
           "colony":"alight","policy":"steady"}]"#,
    );
    fs::write(&path, extended).expect("edit");
    let loaded = load_bots(&path).expect("loads");
    assert_eq!(loaded.len(), founding_bots().len() + 1);
    assert_eq!(loaded.last().expect("fernwake").civilization.name, "Fernwake");
}
