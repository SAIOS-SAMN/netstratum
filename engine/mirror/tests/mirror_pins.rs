//! The mirror pins: the reflection is faithful and incremental, the
//! snapshot is atomic-replace, directives round-trip and archive, and
//! the mirrored chain carries the digest linkage an outside reader
//! can check.

use cfe::dop::ZoneKind;
use cfe::world::World;
use mirror::{Directive, Disposition, Mirror};
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mirror-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn played_world() -> World {
    let mut world = World::new();
    world.found("aurora").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    world
}

/// The reflection is complete and INCREMENTAL: a second record with no
/// new receipts appends nothing; further play appends only the delta.
#[test]
fn the_reflection_is_faithful_and_incremental() {
    let root = scratch("incremental");
    let mirror = Mirror::at(&root);
    let mut world = played_world();
    mirror.record("aurora", &world).expect("records");

    let dir = mirror.civ_dir("aurora");
    let state = fs::read_to_string(dir.join("state.json")).expect("state exists");
    assert!(state.contains("\"civilization\":\"aurora\""));
    assert!(state.contains("\"kind\":\"extractor\""));

    let lines = |p: &PathBuf| {
        fs::read_to_string(p.join("chronicle.jsonl"))
            .expect("chronicle exists")
            .lines()
            .count()
    };
    let mirrored = lines(&dir);
    assert_eq!(
        mirrored,
        world.chronicle().receipts().len(),
        "every receipt reflected"
    );

    mirror.record("aurora", &world).expect("re-records");
    assert_eq!(lines(&dir), mirrored, "no duplicate reflection");

    world.zone(1, ZoneKind::Extractor).expect("zones");
    mirror.record("aurora", &world).expect("records the delta");
    assert_eq!(lines(&dir), mirrored + 1, "exactly the delta appended");
}

/// The mirrored stream carries the digest chain: line n's parent
/// equals line n−1's digest — the linkage an outside observer checks
/// without recomputing hashes.
#[test]
fn the_mirrored_chain_links() {
    let root = scratch("chain");
    let mirror = Mirror::at(&root);
    let world = played_world();
    mirror.record("aurora", &world).expect("records");

    let text = fs::read_to_string(mirror.civ_dir("aurora").join("chronicle.jsonl")).expect("read");
    let field = |line: &str, key: &str| {
        let needle = format!("\"{key}\":\"");
        let start = line.find(&needle).expect("field") + needle.len();
        line[start..start + 64].to_string()
    };
    let mut previous_digest: Option<String> = None;
    for line in text.lines() {
        if let Some(prev) = &previous_digest {
            assert_eq!(&field(line, "parent"), prev, "the chain links");
        }
        previous_digest = Some(field(line, "digest"));
    }
}

/// Directives round-trip through the inbox in name order and archive
/// under their disposition — never deleted.
#[test]
fn directives_round_trip_and_archive() {
    let root = scratch("directives");
    let mirror = Mirror::at(&root);
    let world = played_world();
    mirror.record("aurora", &world).expect("records");

    let inbox = mirror.civ_dir("aurora").join("inbox");
    fs::write(
        inbox.join("000001-zone.json"),
        r#"{"verb":"zone","args":["12","ext"],"as":"steward"}"#,
    )
    .expect("writes");
    fs::write(
        inbox.join("000002-collect.json"),
        r#"{"verb":"collect","args":[],"as":"steward"}"#,
    )
    .expect("writes");

    let pending = mirror.pending_directives("aurora").expect("reads");
    assert_eq!(pending.len(), 2);
    assert_eq!(
        pending[0].directive,
        Directive {
            verb: "zone".into(),
            args: vec!["12".into(), "ext".into()],
            acting_as: "steward".into(),
        }
    );
    assert_eq!(pending[1].directive.verb, "collect");

    mirror
        .archive(&pending[0], Disposition::Applied)
        .expect("archives");
    mirror
        .archive(&pending[1], Disposition::Refused)
        .expect("archives");
    assert!(inbox.join("applied/000001-zone.json").exists());
    assert!(inbox.join("refused/000002-collect.json").exists());
    assert!(mirror.pending_directives("aurora").expect("reads").is_empty());
}

/// Solve certificates land one file per task, sanitized names.
#[test]
fn solves_record_per_task() {
    let root = scratch("solves");
    let mirror = Mirror::at(&root);
    mirror
        .record_solve("aurora", "arc_00576224", "tile3x3(odd_row=cols)", "ab12")
        .expect("records");
    let text = fs::read_to_string(
        mirror
            .civ_dir("aurora")
            .join("solves/arc_00576224.json"),
    )
    .expect("reads");
    assert!(text.contains("\"certificate\":\"ab12\""));
}

/// The mirrored state parses as JSON at the shape the observer reads
/// (spot-checked structurally: balanced braces, key fields present).
#[test]
fn the_snapshot_is_wellformed() {
    let root = scratch("wellformed");
    let mirror = Mirror::at(&root);
    let world = played_world();
    mirror.record("aurora", &world).expect("records");
    let state = fs::read_to_string(mirror.civ_dir("aurora").join("state.json")).expect("read");
    assert_eq!(
        state.matches('{').count(),
        state.matches('}').count(),
        "balanced objects"
    );
    for key in ["\"banked\"", "\"zones\"", "\"entities\"", "\"clock\"", "\"accrued\""] {
        assert!(state.contains(key), "snapshot carries {key}");
    }
    // Exactness discipline: fractions render as p/q, never decimals.
    assert!(!state.contains("0.5"), "no decimal ever enters the mirror");
}
