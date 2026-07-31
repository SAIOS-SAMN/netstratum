//! The ledger pins: the export is faithful and incremental, the
//! snapshot is atomic-replace, directives round-trip and archive, the
//! exported chain carries checkable digest linkage, and the registry
//! enforces the closed axis vocabulary.

use cfe::dop::ZoneKind;
use cfe::world::World;
use ledger::registry::{Civilization, Registry, RegistryRefused};
use ledger::{Directive, Disposition, Ledger};
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
        sigil: "humans".into(),
        species: "human".into(),
        clade: String::new(),
        motto: "we go anyway".into(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
        founder: "operator".into(),
        lineage: Vec::new(),
        station: None,
    }
}

/// The reflection lands at the TOPOLOGY: world container + planet
/// snapshot, complete and INCREMENTAL — a second record with no new
/// receipts appends nothing; further play appends only the delta.
#[test]
fn the_reflection_is_faithful_and_incremental() {
    let root = scratch("incremental");
    let book = Ledger::at(&root);
    let mut world = played_world();
    book.record("sol", "mars", "humans", &world).expect("records");

    let dir = book.world_dir("sol");
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

    book.record("sol", "mars", "humans", &world).expect("re-records");
    assert_eq!(lines(&dir), mirrored, "no duplicate reflection");

    world.zone(1, ZoneKind::Extractor).expect("zones");
    book.record("sol", "mars", "humans", &world).expect("records the delta");
    assert_eq!(lines(&dir), mirrored + 1, "exactly the delta appended");
}

/// ONE chronicle per world: the stream lives at the world root, and
/// line n's parent equals line n−1's digest — the linkage an outside
/// observer checks without recomputing hashes.
#[test]
fn the_mirrored_chain_links_at_world_altitude() {
    let root = scratch("chain");
    let book = Ledger::at(&root);
    let world = played_world();
    book.record("sol", "mars", "humans", &world).expect("records");

    let text =
        fs::read_to_string(book.world_dir("sol").join("chronicle.jsonl")).expect("read");
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
    let book = Ledger::at(&root);
    let world = played_world();
    book.record("sol", "mars", "humans", &world).expect("records");

    let inbox = book.world_dir("sol").join("inbox");
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

    let pending = book.pending_directives("sol").expect("reads");
    assert_eq!(pending.len(), 2);
    assert_eq!(
        pending[0].directive,
        Directive {
            verb: "zone".into(),
            args: vec!["12".into(), "ext".into()],
            acting_as: "steward".into(),
        }
    );
    book.archive(&pending[0], Disposition::Applied).expect("archives");
    book.archive(&pending[1], Disposition::Refused).expect("archives");
    assert!(inbox.join("applied/000001-zone.json").exists());
    assert!(inbox.join("refused/000002-collect.json").exists());
    assert!(book.pending_directives("sol").expect("reads").is_empty());
}

/// The registry: create, list, join; the closed axis vocabulary
/// refuses typed; a duplicate sigil refuses typed.
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
            sigil: "humans".into()
        })
    );

    // An axis outside the closed vocabulary refuses, naming the axis.
    let mut wrong = humans();
    wrong.sigil = "wrong".into();
    wrong.perception = "psychic".into();
    assert_eq!(
        registry.create(&wrong),
        Err(RegistryRefused::UnknownAxis {
            axis: "perception",
            given: "psychic".into()
        })
    );
}

/// SUBSTRATE-1 (12_UNION §5a): the axis vocabulary grows APPEND-ONLY —
/// `plasmic` metabolism and `umbral` perception register and round-trip
/// whole; the door still refuses what was never added (the vocabulary
/// stays closed, it just closed around more).
#[test]
fn the_axes_grow_append_only() {
    let root = scratch("registry-substrate");
    let registry = Registry::at(&root);

    let mut sheet = humans();
    sheet.name = "Coronal Choir".into();
    sheet.sigil = "coronal-choir".into();
    sheet.species = "current sheet".into();
    sheet.motto = "we are the weather of the star".into();
    sheet.metabolism = "plasmic".into();
    sheet.perception = "umbral".into();
    registry.create(&sheet).expect("the plasmic line registers");
    assert_eq!(registry.load("coronal-choir"), Some(sheet));

    // Growth is append-only, never a reopening: unknown values still
    // refuse typed on every axis.
    let mut wrong = humans();
    wrong.sigil = "etheric".into();
    wrong.metabolism = "etheric".into();
    assert_eq!(
        registry.create(&wrong),
        Err(RegistryRefused::UnknownAxis {
            axis: "metabolism",
            given: "etheric".into()
        })
    );
}

/// LINEAGE-1 (12_UNION §5): the lineage pointer resolves hop-by-hop to
/// its HEAD; headless is lawful (a rootless line resolves to itself);
/// a dangling pointer resolves to the deepest recorded ancestor; a
/// corrupt cycle stops deterministically. The domain reading derives
/// from metabolism + band, never stored — no line answers to a planner
/// of an alien physiology.
#[test]
fn a_lineage_resolves_to_its_own_head() {
    use cfe::thermal::SubstrateBand as Band;
    use ledger::registry::lineage_domain;

    let root = scratch("registry-lineage");
    let registry = Registry::at(&root);

    // elder <- branch <- leaf: the chain resolves to the elder.
    let mut elder = humans();
    elder.sigil = "elder".into();
    let mut branch = humans();
    branch.sigil = "branch".into();
    branch.lineage = vec!["elder".into()];
    let mut leaf = humans();
    leaf.sigil = "leaf".into();
    leaf.lineage = vec!["branch".into()];
    for line in [&elder, &branch, &leaf] {
        registry.create(line).expect("registers");
    }
    assert_eq!(registry.resolve_head("leaf"), Some("elder".into()));
    // Headless is LAWFUL: the elder resolves to itself.
    assert_eq!(registry.resolve_head("elder"), Some("elder".into()));

    // A dangling pointer resolves to the deepest RECORDED ancestor.
    let mut orphan = humans();
    orphan.sigil = "orphan".into();
    orphan.lineage = vec!["unrecorded-past".into()];
    registry.create(&orphan).expect("registers");
    assert_eq!(registry.resolve_head("orphan"), Some("orphan".into()));

    // A corrupt cycle stops deterministically, never hangs.
    let mut ouro = humans();
    ouro.sigil = "ouro".into();
    ouro.lineage = vec!["boros".into()];
    let mut boros = humans();
    boros.sigil = "boros".into();
    boros.lineage = vec!["ouro".into()];
    registry.create(&ouro).expect("registers");
    registry.create(&boros).expect("registers");
    assert_eq!(registry.resolve_head("ouro"), Some("boros".into()));

    // The domain reading: metabolism + band, the founding four + the
    // visible-next; a tardigrade line under a Furnace sun elevates
    // toward the THERMAL seats — never toward an alien physiology.
    // (BAND-1 seat witnesses live in `the_seats_are_space_not_rank`.)
    assert_eq!(lineage_domain("mineral", Some(Band::HyperThermal)), "thermal");
    assert_eq!(lineage_domain("mineral", None), "lithic");
    assert_eq!(lineage_domain("biotic", Some(Band::Cryogenic)), "cryo");
    assert_eq!(lineage_domain("biotic", Some(Band::Mesic)), "mesic");
    assert_eq!(lineage_domain("biotic", None), "mesic");
    assert_eq!(lineage_domain("plasmic", Some(Band::HyperThermal)), "plasmic");
}

/// BAND-1 (12_UNION §1–§3): seats are SPACE, not rank. A progenitor
/// species holds 8 seats PER LINEAGE DOMAIN — the ninth mineral-line
/// claim refuses with the room named full, while a biotic-line seat
/// still opens (the band grows by domains). An unknown office refuses
/// typed; a stationless people is the common, lawful state; the seat
/// round-trips whole.
#[test]
fn the_seats_are_space_not_rank() {
    use ledger::registry::RegistryRefused;
    let root = scratch("registry-band");
    let registry = Registry::at(&root);

    // Fill the lithic progenitor room: 8 mineral-line seats.
    for n in 0..8 {
        let mut elder = humans();
        elder.name = format!("Elder {n}");
        elder.sigil = format!("elder-{n}");
        elder.metabolism = "mineral".into();
        elder.station = Some("progenitor".into());
        registry.create(&elder).expect("a seat stands open");
    }
    // The ninth: the room is FULL — a room filled, not a rank refused.
    let mut ninth = humans();
    ninth.sigil = "ninth".into();
    ninth.metabolism = "mineral".into();
    ninth.station = Some("progenitor".into());
    assert_eq!(
        registry.create(&ninth),
        Err(RegistryRefused::NoSeatOpen {
            station: "progenitor".into(),
            domain: "lithic".into(),
            held: 8,
            width: 8,
        })
    );

    // A DIFFERENT domain still seats — the band grows by domains.
    let mut warm = humans();
    warm.sigil = "warm-elder".into();
    warm.station = Some("progenitor".into());
    registry.create(&warm).expect("the mesic room stands open");
    assert_eq!(
        registry.load("warm-elder").expect("loads").station.as_deref(),
        Some("progenitor"),
        "the seat round-trips whole"
    );

    // An unknown office refuses typed; no station is the common state.
    let mut odd = humans();
    odd.sigil = "odd".into();
    odd.station = Some("emperor".into());
    assert_eq!(
        registry.create(&odd),
        Err(RegistryRefused::UnknownStation { given: "emperor".into() })
    );
    let mut common = humans();
    common.sigil = "common".into();
    registry.create(&common).expect("a stationless people registers");
}

/// The star-chart holds TUniversal{S, G}: exact-rational S = (x, y, z) on
/// the 3-torus (negative coordinates wrap), the gauge phase rides
/// along, distance is L1 (NO SQUARING — the L1 law), and the
/// universal clock is a pure base-60 odometer of ticks.
#[test]
fn the_starchart_holds_tuniversal_coordinates() {
    use cfe::ratio::ratio;
    let root = scratch("starchart");
    let book = Ledger::at(&root);
    book
        .chart_world("sol", (&ratio(0, 1), &ratio(0, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts");
    // Negative coordinates WRAP onto the torus: −7/2 mod 64 = 121/2.
    book
        .chart_world(
            "kepler-passage",
            (&ratio(-7, 2), &ratio(9, 1), &ratio(63, 1)),
            &ratio(1, 4),
        )
        .expect("charts");
    assert_eq!(
        book.charted_position("kepler-passage"),
        Some(((ratio(121, 2), ratio(9, 1), ratio(63, 1)), ratio(1, 4)))
    );
    // L1, the short way around: x-leg min(121/2, 64−121/2) = 7/2,
    // y-leg 9, z-leg min(63, 1) = 1 → total 27/2. No squaring.
    assert_eq!(book.l1_distance("sol", "kepler-passage"), Some(ratio(27, 2)));

    // Re-charting moves the world; the entry count stays one.
    book
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
    assert_eq!(ledger::universal_clock(0), "00:00:00:00:00:00");
    assert_eq!(ledger::universal_clock(61), "00:00:00:00:01:01");
    assert_eq!(ledger::universal_clock(3600), "00:00:00:01:00:00");
    book.set_universal_clock(61).expect("writes the card");
    let card = fs::read_to_string(root.join("universe.json")).expect("card");
    assert!(card.contains("\"tuniversal\":\"00:00:00:00:01:01\""));
}

/// Exactness discipline holds through the topology: fractions render
/// p/q, never decimals; the snapshot stays balanced JSON.
#[test]
fn the_snapshot_is_wellformed_and_exact() {
    let root = scratch("wellformed");
    let book = Ledger::at(&root);
    let world = played_world();
    book.record("sol", "mars", "humans", &world).expect("records");
    let state = fs::read_to_string(
        book.world_dir("sol").join("planets/mars/state.json"),
    )
    .expect("read");
    assert_eq!(state.matches('{').count(), state.matches('}').count());
    assert!(!state.contains("0.5"), "no decimal ever enters the mirror");
}
