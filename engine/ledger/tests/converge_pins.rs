//! CONVERGE-1 witnesses (07 §5): the archived timelines annex onto one
//! chart and one T axis — verified, quarantined when broken, placed by
//! measurement, idempotent, and read by the boot that follows.

use std::fs;
use std::path::PathBuf;

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::world::World;
use ledger::converge::{converge, default_era_roots};
use ledger::Ledger;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "converge-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// A short played life: found, zone, tick, collect.
fn played_world(colony: &str, ticks: u32) -> World {
    let mut world = World::new();
    world.found(colony).expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..ticks {
        world.tick();
    }
    world.collect().expect("collects");
    world
}

/// A life with different loop content: a build razed, a grant revoked.
fn looped_world(colony: &str, ticks: u32) -> World {
    let mut world = World::new();
    world.found(colony).expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.raze(8).expect("razes");
    let agent = world.register_agent("loop-bot").expect("registers");
    world.grant(agent, cfe::ipsal::Scope::Collect).expect("grants");
    world.revoke(agent, cfe::ipsal::Scope::Collect).expect("revokes");
    for _ in 0..ticks {
        world.tick();
    }
    world.collect().expect("collects");
    world
}

/// Archive one world into an era root, returning its terminal stamp.
fn archive(era_root: &std::path::Path, world_name: &str, world: &World) -> u64 {
    let book = Ledger::at(era_root.join("worlds"));
    book.record(world_name, "prime", "pin-civ", world)
        .expect("records");
    world
        .chronicle()
        .receipts()
        .iter()
        .map(|r| r.stamp)
        .max()
        .unwrap_or(0)
}

#[test]
fn the_archived_timelines_annex_onto_one_axis() {
    let base = scratch("annex");
    let era1 = base.join("var-era1");
    let era2 = base.join("var-era2");
    let live = base.join("var");
    let t1 = archive(&era1, "amber", &played_world("amber-colony", 4));
    let t2 = archive(&era2, "moss", &played_world("moss-colony", 6));
    assert!(t1 > 0 && t2 > 0);

    let book = Ledger::at(live.join("worlds"));
    let eras = default_era_roots(live.to_str().expect("utf8 path"));
    // Discovery finds the sibling eras name-ascending — and NEVER the
    // live root itself (naming ruling, 2026-07-30): self-annexation
    // minted era-qualified aliases of living worlds; the fold is the
    // one lawful door from live to archive.
    assert_eq!(
        eras.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>(),
        vec!["var-era1", "var-era2"],
    );
    let read = converge(&book, &eras).expect("converges");
    assert_eq!(read.adopted.len(), 2);
    assert!(read.quarantined.is_empty());
    // Epoch shift (07 §1.3): eras stack end-to-end, no overlap.
    assert_eq!(read.epoch_base, t1 + t2);

    // The ledger: real chain machinery, offsets in era order.
    let ledger = cfe::persist::load(&live.join("worlds/convergence.tlv")).expect("ledger loads");
    let offsets: Vec<u64> = ledger
        .receipts()
        .iter()
        .map(|r| match &r.event {
            ClosureEvent::EraAnnexed { offset, .. } => *offset,
            other => panic!("only annexations in the ledger, got {other:?}"),
        })
        .collect();
    assert_eq!(offsets, vec![0, t1]);
    // TUniversal stamps: offset + local terminal.
    let stamps: Vec<u64> = ledger.receipts().iter().map(|r| r.stamp).collect();
    assert_eq!(stamps, vec![t1, t1 + t2]);

    // The universe card persisted at the epoch base, odometer rendered.
    let card = fs::read_to_string(live.join("worlds/universe.json")).expect("card");
    assert!(card.contains(&format!("\"ticks\":{}", read.epoch_base)));
    assert!(card.contains(&ledger::universal_clock(read.epoch_base)));

    // Spawns read the converged chart (07 §1.6): both worlds charted.
    assert!(book.charted_position("var-era1--amber").is_some());
    assert!(book.charted_position("var-era2--moss").is_some());
}

#[test]
fn placement_is_a_measurement_not_a_hash() {
    let base = scratch("measure");
    let era1 = base.join("var-era1");
    archive(&era1, "amber", &played_world("amber-colony", 4));
    archive(&era1, "loops", &looped_world("loop-colony", 4));

    // Converged into two INDEPENDENT live roots → identical coordinates.
    let live_a = base.join("var");
    let live_b = base.join("elsewhere-var");
    let book_a = Ledger::at(live_a.join("worlds"));
    let book_b = Ledger::at(live_b.join("worlds"));
    let eras_a = vec![("var-era1".to_string(), era1.join("worlds"))];
    converge(&book_a, &eras_a).expect("converges");
    converge(&book_b, &eras_a).expect("converges");
    let placed_a = book_a.charted_position("var-era1--amber").expect("charted");
    let placed_b = book_b.charted_position("var-era1--amber").expect("charted");
    assert_eq!(placed_a, placed_b);

    // Different loop content → different fiber cards (COHESION, 07 §2):
    // the razed build and the revoked grant leave residuals the plain
    // life does not carry.
    let fiber_plain =
        fs::read_to_string(live_a.join("worlds/fiber/var-era1--amber.json")).expect("fiber");
    let fiber_loops =
        fs::read_to_string(live_a.join("worlds/fiber/var-era1--loops.json")).expect("fiber");
    assert_ne!(fiber_plain, fiber_loops);
}

#[test]
fn a_tampered_archive_quarantines_named_and_the_rest_still_annex() {
    let base = scratch("tamper");
    let era1 = base.join("var-era1");
    archive(&era1, "clean", &played_world("clean-colony", 3));
    archive(&era1, "dirty", &played_world("dirty-colony", 3));
    // Flip one digest hex character in the dirty world's line 2.
    let path = era1.join("worlds/dirty/chronicle.jsonl");
    let text = fs::read_to_string(&path).expect("reads");
    let tampered: Vec<String> = text
        .lines()
        .enumerate()
        .map(|(i, line)| match i {
            2 => {
                let at = line.find("\"digest\":\"").expect("digest field") + "\"digest\":\"".len();
                let mut chars: Vec<char> = line.chars().collect();
                chars[at] = match chars[at] {
                    'f' => '0',
                    _ => 'f',
                };
                chars.into_iter().collect()
            }
            _ => line.to_string(),
        })
        .collect();
    fs::write(&path, tampered.join("\n") + "\n").expect("writes");

    let live = base.join("var");
    let book = Ledger::at(live.join("worlds"));
    let eras = vec![("var-era1".to_string(), era1.join("worlds"))];
    let read = converge(&book, &eras).expect("converges");
    assert_eq!(read.adopted, vec![("var-era1".into(), "clean".into())]);
    assert_eq!(read.quarantined.len(), 1);
    assert_eq!(read.quarantined[0].world, "dirty");
    assert_eq!(read.quarantined[0].at, 2, "the break is NAMED at its sequence");
    // Quarantined, never deleted: the archive is untouched on disk.
    assert!(path.exists());
}

#[test]
fn a_torn_tail_annexes_at_the_verified_prefix() {
    let base = scratch("torn");
    let era1 = base.join("var-era1");
    let world = played_world("torn-colony", 3);
    let receipts_before = world.chronicle().receipts().len();
    let t_before = archive(&era1, "torn", &world);
    // The killed-writer artifacts, both flavours: a NUL-filled line and
    // a half-written record.
    let path = era1.join("worlds/torn/chronicle.jsonl");
    let mut text = fs::read_to_string(&path).expect("reads");
    text.push_str(&"\0".repeat(120));
    text.push('\n');
    text.push_str("{\"seq\":99,\"stamp\":9,\"parent\":\"ab");
    text.push('\n');
    fs::write(&path, text).expect("writes");

    let live = base.join("var");
    let book = Ledger::at(live.join("worlds"));
    let eras = vec![("var-era1".to_string(), era1.join("worlds"))];
    let read = converge(&book, &eras).expect("converges");
    // Annexed at the verified prefix; the tear recorded, not fatal.
    assert_eq!(read.adopted, vec![("var-era1".into(), "torn".into())]);
    assert_eq!(read.torn, vec![("var-era1".into(), "torn".into())]);
    assert!(read.quarantined.is_empty());
    assert_eq!(read.epoch_base, t_before, "terminal T is the PREFIX's");
    let ledger = cfe::persist::load(&live.join("worlds/convergence.tlv")).expect("ledger");
    assert_eq!(ledger.receipts().len(), 1);
    // The archive itself is untouched beyond our simulated tear.
    let lines = fs::read_to_string(&path).expect("still on disk");
    assert_eq!(
        lines.lines().filter(|l| l.contains("\"digest\"")).count(),
        receipts_before,
    );
}

#[test]
fn convergence_is_idempotent() {
    let base = scratch("idem");
    let era1 = base.join("var-era1");
    archive(&era1, "amber", &played_world("amber-colony", 4));
    let live = base.join("var");
    let book = Ledger::at(live.join("worlds"));
    let eras = vec![("var-era1".to_string(), era1.join("worlds"))];

    let first = converge(&book, &eras).expect("converges");
    assert_eq!(first.adopted.len(), 1);
    let ledger_once = fs::read(live.join("worlds/convergence.tlv")).expect("ledger");

    let second = converge(&book, &eras).expect("converges again");
    assert_eq!(second.adopted.len(), 0);
    assert_eq!(second.already, 1);
    assert_eq!(second.epoch_base, first.epoch_base);
    let ledger_twice = fs::read(live.join("worlds/convergence.tlv")).expect("ledger");
    assert_eq!(ledger_once, ledger_twice, "a second boot appends NOTHING");
}

#[test]
fn the_annexation_event_round_trips_byte_exact() {
    let event = ClosureEvent::EraAnnexed {
        era: "var-era1".into(),
        world: "amber".into(),
        terminal: [7u8; 32],
        offset: 65,
    };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 21, "the permanent CONVERGE-1 tag");
    let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
    assert_eq!(decoded, event);
    assert_eq!(decoded.wire_bytes(), bytes, "byte-exact round trip");
}
