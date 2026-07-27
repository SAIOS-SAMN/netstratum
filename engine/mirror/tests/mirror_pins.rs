//! The mirror pins: the reflection is faithful and incremental, the
//! snapshot is atomic-replace, directives round-trip and archive, the
//! mirrored chain carries checkable digest linkage, and the registry
//! enforces the closed axis vocabulary.

use cfe::dop::ZoneKind;
use cfe::world::World;
use mirror::registry::{Civilization, Registry, RegistryRefused};
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
    world.found("first-landing").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    world
}

fn humans() -> Civilization {
    Civilization {
        name: "Humans".into(),
        slug: "humans".into(),
        species: "human".into(),
        motto: "we go anyway".into(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
        founder: "operator".into(),
        lineage: None,
    }
}

/// The reflection lands at the TOPOLOGY: world container + planet
/// snapshot, complete and INCREMENTAL — a second record with no new
/// receipts appends nothing; further play appends only the delta.
#[test]
fn the_reflection_is_faithful_and_incremental() {
    let root = scratch("incremental");
    let mirror = Mirror::at(&root);
    let mut world = played_world();
    mirror.record("sol", "mars", "humans", &world).expect("records");

    let dir = mirror.world_dir("sol");
    let state = fs::read_to_string(dir.join("planets/mars/state.json")).expect("state exists");
    for key in [
        "\"civilization\":\"humans\"",
        "\"world\":\"sol\"",
        "\"planet\":\"mars\"",
        "\"kind\":\"extractor\"",
    ] {
        assert!(state.contains(key), "snapshot carries {key}");
    }
    let card = fs::read_to_string(dir.join("world.json")).expect("world card");
    assert!(card.contains("\"planets\":[\"mars\"]"));

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

    mirror.record("sol", "mars", "humans", &world).expect("re-records");
    assert_eq!(lines(&dir), mirrored, "no duplicate reflection");

    world.zone(1, ZoneKind::Extractor).expect("zones");
    mirror.record("sol", "mars", "humans", &world).expect("records the delta");
    assert_eq!(lines(&dir), mirrored + 1, "exactly the delta appended");
}

/// ONE chronicle per world: the stream lives at the world root, and
/// line n's parent equals line n−1's digest — the linkage an outside
/// observer checks without recomputing hashes.
#[test]
fn the_mirrored_chain_links_at_world_altitude() {
    let root = scratch("chain");
    let mirror = Mirror::at(&root);
    let world = played_world();
    mirror.record("sol", "mars", "humans", &world).expect("records");

    let text =
        fs::read_to_string(mirror.world_dir("sol").join("chronicle.jsonl")).expect("read");
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

/// Directives round-trip through the WORLD's inbox in name order and
/// archive under their disposition — never deleted.
#[test]
fn directives_round_trip_and_archive() {
    let root = scratch("directives");
    let mirror = Mirror::at(&root);
    let world = played_world();
    mirror.record("sol", "mars", "humans", &world).expect("records");

    let inbox = mirror.world_dir("sol").join("inbox");
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

    let pending = mirror.pending_directives("sol").expect("reads");
    assert_eq!(pending.len(), 2);
    assert_eq!(
        pending[0].directive,
        Directive {
            verb: "zone".into(),
            args: vec!["12".into(), "ext".into()],
            acting_as: "steward".into(),
        }
    );
    mirror.archive(&pending[0], Disposition::Applied).expect("archives");
    mirror.archive(&pending[1], Disposition::Refused).expect("archives");
    assert!(inbox.join("applied/000001-zone.json").exists());
    assert!(inbox.join("refused/000002-collect.json").exists());
    assert!(mirror.pending_directives("sol").expect("reads").is_empty());
}

/// The registry: create, list, join; the closed axis vocabulary
/// refuses typed; a duplicate slug refuses typed.
#[test]
fn the_registry_enforces_the_vocabulary() {
    let root = scratch("registry");
    let registry = Registry::at(&root);
    registry.create(&humans()).expect("registers");

    // Join: the directory lists it; load round-trips whole.
    let listed = registry.list();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0], humans());
    assert_eq!(registry.load("humans"), Some(humans()));

    // A duplicate refuses typed.
    assert_eq!(
        registry.create(&humans()),
        Err(RegistryRefused::AlreadyRegistered {
            slug: "humans".into()
        })
    );

    // An axis outside the closed vocabulary refuses, naming the axis.
    let mut wrong = humans();
    wrong.slug = "wrong".into();
    wrong.perception = "psychic".into();
    assert_eq!(
        registry.create(&wrong),
        Err(RegistryRefused::UnknownAxis {
            axis: "perception",
            given: "psychic".into()
        })
    );
}

/// The star-chart holds TUniversal{S, G}: exact-rational S = (x, y, z) on
/// the 3-torus (negative coordinates wrap), the gauge phase rides
/// along, distance is L1 (NO SQUARING — the L1 law), and the
/// universal clock is a pure base-60 odometer of ticks.
#[test]
fn the_starchart_holds_tuniversal_coordinates() {
    use cfe::ratio::ratio;
    let root = scratch("starchart");
    let mirror = Mirror::at(&root);
    mirror
        .chart_world("sol", (&ratio(0, 1), &ratio(0, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts");
    // Negative coordinates WRAP onto the torus: −7/2 mod 64 = 121/2.
    mirror
        .chart_world(
            "kepler-passage",
            (&ratio(-7, 2), &ratio(9, 1), &ratio(63, 1)),
            &ratio(1, 4),
        )
        .expect("charts");
    assert_eq!(
        mirror.charted_position("kepler-passage"),
        Some(((ratio(121, 2), ratio(9, 1), ratio(63, 1)), ratio(1, 4)))
    );
    // L1, the short way around: x-leg min(121/2, 64−121/2) = 7/2,
    // y-leg 9, z-leg min(63, 1) = 1 → total 27/2. No squaring.
    assert_eq!(mirror.l1_distance("sol", "kepler-passage"), Some(ratio(27, 2)));

    // Re-charting moves the world; the entry count stays one.
    mirror
        .chart_world(
            "kepler-passage",
            (&ratio(1, 3), &ratio(9, 1), &ratio(0, 1)),
            &ratio(0, 1),
        )
        .expect("re-charts");
    let text = fs::read_to_string(root.join("starchart.json")).expect("read");
    assert_eq!(text.lines().count(), 2, "one line per world");
    assert!(text.contains("\"x\":\"1/3\""), "exact fraction rendering");
    assert!(!text.contains("0.33"), "no float ever enters the universe");

    // T: the six-field odometer, base 60, tick-derived only.
    assert_eq!(mirror::universal_clock(0), "00:00:00:00:00:00");
    assert_eq!(mirror::universal_clock(61), "00:00:00:00:01:01");
    assert_eq!(mirror::universal_clock(3600), "00:00:00:01:00:00");
    mirror.set_universal_clock(61).expect("writes the card");
    let card = fs::read_to_string(root.join("universe.json")).expect("card");
    assert!(card.contains("\"tuniversal\":\"00:00:00:00:01:01\""));
}

/// Exactness discipline holds through the topology: fractions render
/// p/q, never decimals; the snapshot stays balanced JSON.
#[test]
fn the_snapshot_is_wellformed_and_exact() {
    let root = scratch("wellformed");
    let mirror = Mirror::at(&root);
    let world = played_world();
    mirror.record("sol", "mars", "humans", &world).expect("records");
    let state = fs::read_to_string(
        mirror.world_dir("sol").join("planets/mars/state.json"),
    )
    .expect("read");
    assert_eq!(state.matches('{').count(), state.matches('}').count());
    assert!(!state.contains("0.5"), "no decimal ever enters the mirror");
}
