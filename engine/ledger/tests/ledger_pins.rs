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
        "ledger-pin-{name}-{}",
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

/// The reflection lands as TLV: state.tlv + world.card (no JSON).
#[test]
fn the_reflection_is_faithful_and_incremental() {
    let root = scratch("incremental");
    let book = Ledger::at(&root);
    let mut world = played_world();
    scribe::record(&book, "sol", "mars", "humans", &world).expect("records");

    let dir = book.world_dir("sol");
    let tlv = dir.join("planets/mars/state.tlv");
    assert!(tlv.exists(), "state.tlv machine mirror");
    let wire = fs::read(&tlv).expect("read tlv");
    assert!(!wire.is_empty());
    // Zone entries present (tag 17).
    assert!(wire.windows(1).any(|w| w[0] == 17) || wire.contains(&17u8));
    let card = fs::read(dir.join("world.card")).expect("world.card");
    assert!(
        card.windows(3).any(|w| w == b"sol") || card.windows(4).any(|w| w == b"mars"),
        "world.card carries names"
    );
    // No JSON colony files.
    assert!(!dir.join("planets/mars/state.json").exists());
    assert!(!dir.join("world.json").exists());
    assert!(!dir.join("chronicle.jsonl").exists());

    // Re-record is idempotent on mirror (same wire when no new deeds).
    let before = fs::read(&tlv).expect("before");
    scribe::record(&book, "sol", "mars", "humans", &world).expect("re-records");
    assert_eq!(fs::read(&tlv).expect("after"), before);

    world.zone(1, ZoneKind::Extractor).expect("zones");
    scribe::record(&book, "sol", "mars", "humans", &world).expect("records the delta");
    let after = fs::read(&tlv).expect("delta");
    assert_ne!(after, before, "zone deed changes wire");
}

/// Chain integrity is chronicle.tlv (SEAL B), not a JSONL export.
#[test]
fn the_exported_chain_links_at_world_altitude() {
    let root = scratch("chain");
    let book = Ledger::at(&root);
    let world = played_world();
    scribe::record(&book, "sol", "mars", "humans", &world).expect("records");
    let path = book.world_dir("sol").join("chronicle.tlv");
    cfe::persist::sync(world.chronicle(), &path).expect("sync");
    let loaded = cfe::persist::load(&path).expect("load verifies");
    assert_eq!(loaded.verify_chain(), cfe::vrp::ChainRead::Intact);
    assert_eq!(
        loaded.receipts().len(),
        world.chronicle().receipts().len()
    );
}

/// Directives round-trip through the WORLD's inbox in name order and
/// archive under their disposition — never deleted.
#[test]
fn directives_round_trip_and_archive() {
    let root = scratch("directives");
    let book = Ledger::at(&root);
    let world = played_world();
    scribe::record(&book, "sol", "mars", "humans", &world).expect("records");

    let inbox = book.world_dir("sol").join("inbox");
    ledger::write_directive_tlv(
        &inbox.join("000001-zone.dir"),
        &Directive {
            verb: "zone".into(),
            args: vec!["12".into(), "ext".into()],
            acting_as: "steward".into(),
        },
    )
    .expect("writes");
    ledger::write_directive_tlv(
        &inbox.join("000002-collect.dir"),
        &Directive {
            verb: "collect".into(),
            args: vec![],
            acting_as: "steward".into(),
        },
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
    assert!(inbox.join("applied/000001-zone.dir").exists());
    assert!(inbox.join("refused/000002-collect.dir").exists());
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

    // Fill the lithic progenitor room (base 8 + 25 expansion = 33).
    let width = ledger::registry::seat_width("progenitor").expect("w");
    for n in 0..width {
        let mut elder = humans();
        elder.name = format!("Elder {n}");
        elder.sigil = format!("elder-{n}");
        elder.metabolism = "mineral".into();
        elder.station = Some("progenitor".into());
        registry.create(&elder).expect("a seat stands open");
    }
    // One past width: the room is FULL — a room filled, not a rank refused.
    let mut overflow = humans();
    overflow.sigil = "ninth".into();
    overflow.metabolism = "mineral".into();
    overflow.station = Some("progenitor".into());
    assert_eq!(
        registry.create(&overflow),
        Err(RegistryRefused::NoSeatOpen {
            station: "progenitor".into(),
            domain: "lithic".into(),
            held: width,
            width,
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
    let chart = fs::read(root.join("starchart.card")).expect("read starchart.card");
    // Two worlds ⇒ two WORLD tags.
    let worlds = chart.iter().filter(|&&b| b == 1).count();
    assert!(worlds >= 2, "one WORLD tag per charted world");
    assert!(
        chart.windows(3).any(|w| w == b"1/3"),
        "exact fraction rendering in TLV"
    );
    assert!(!chart.windows(4).any(|w| w == b"0.33"), "no float");

    // T: the six-field odometer, base 60, tick-derived only.
    assert_eq!(ledger::universal_clock(0), "00:00:00:00:00:00");
    assert_eq!(ledger::universal_clock(61), "00:00:00:00:01:01");
    assert_eq!(ledger::universal_clock(3600), "00:00:00:01:00:00");
    book.set_universal_clock(61).expect("writes the card");
    let card = fs::read(root.join("universe.card")).expect("card");
    let od = ledger::universal_clock(61);
    assert!(card.windows(od.len()).any(|w| w == od.as_bytes()));
}

/// THE SPAN SURVIVES THE CLOCK. `universe.card` span is the authority
/// `universe_span` reads — carried forward on every clock rewrite.
#[test]
fn the_span_survives_a_clock_advance() {
    use cfe::ratio::ratio;
    use ledger::card;
    let root = scratch("span");
    let book = Ledger::at(&root);
    book
        .chart_world("sol", (&ratio(0, 1), &ratio(0, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts, seeding the card at the default span");
    assert_eq!(book.universe_span(), ratio(64, 1), "the default seeds once");

    // A universe of another size — rewrite TLV span field via write_universe path.
    let mut out = Vec::new();
    card::put_str(card::uni::TOPOLOGY, "torus", &mut out);
    card::put_str(card::uni::SPAN, "16", &mut out);
    card::put_str(card::uni::TUNIVERSAL, &ledger::universal_clock(0), &mut out);
    card::put_u64(card::uni::TICKS, 0, &mut out);
    card::put_str(card::uni::FIBER, "polynomial", &mut out);
    card::write_atomic(&root.join("universe.card"), &out).expect("rewrite span");
    assert_eq!(book.universe_span(), ratio(16, 1));

    book.set_universal_clock(1).expect("advances the clock");
    assert_eq!(
        book.universe_span(),
        ratio(16, 1),
        "the clock carries the span forward; it does not re-assert the default"
    );

    book
        .chart_world("kepler-passage", (&ratio(-7, 2), &ratio(1, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts");
    assert_eq!(
        book.charted_position("kepler-passage").map(|(s, _)| s.0),
        Some(ratio(25, 2)),
        "the world wraps on the span that stands, not on the default"
    );
}

/// Exactness discipline: state.tlv exists; no JSON snapshot; no decimals in wire.
#[test]
fn the_snapshot_is_wellformed_and_exact() {
    let root = scratch("wellformed");
    let book = Ledger::at(&root);
    let world = played_world();
    scribe::record(&book, "sol", "mars", "humans", &world).expect("records");
    let tlv = book.world_dir("sol").join("planets/mars/state.tlv");
    let bytes = fs::read(&tlv).expect("read tlv");
    assert!(!bytes.is_empty());
    assert!(!book.world_dir("sol").join("planets/mars/state.json").exists());
    // Wire must not embed ASCII decimal floats as balance text.
    assert!(!bytes.windows(3).any(|w| w == b"0.5"));
}

/// THE TIER AUTHORITY (19_REGISTRY §3, closed 2026-08-03).
///
/// Every station in the closed vocabulary reads a tier. Adding a
/// station without deciding its band fails HERE rather than silently
/// landing it in the common — the `kernel::layer_of` shape.
#[test]
fn tier_covers_every_station() {
    use ledger::registry::{tier_of, STATIONS};
    for station in STATIONS {
        assert!(
            tier_of(Some(station)).is_some(),
            "station {station:?} has no tier — decide its band before seating it"
        );
    }
}

/// The ladder, pinned against the reading that lived in Python — and
/// Wave-1's T4–T9 economic offices (30_OFFICES_AND_RAILS).
#[test]
fn the_ladder_reads_as_ruled() {
    use ledger::registry::{tier_of, Tier};
    assert_eq!(tier_of(Some("progenitor")), Some(Tier::T0));
    assert_eq!(tier_of(Some("watcher")), Some(Tier::T1));
    assert_eq!(tier_of(Some("observer")), Some(Tier::T1));
    assert_eq!(tier_of(Some("sovereign")), Some(Tier::T2));
    assert_eq!(tier_of(None), Some(Tier::T3), "the common is the absence of a seat");
    assert_eq!(tier_of(Some("caravan")), Some(Tier::T4));
    assert_eq!(tier_of(Some("crucible")), Some(Tier::T5));
    assert_eq!(tier_of(Some("exchange")), Some(Tier::T6));
    assert_eq!(tier_of(Some("assay")), Some(Tier::T7));
    assert_eq!(tier_of(Some("vault")), Some(Tier::T8));
    assert_eq!(tier_of(Some("anchor")), Some(Tier::T9));
    assert_eq!(tier_of(Some("herald")), Some(Tier::T10));
    assert_eq!(tier_of(Some("lattice")), Some(Tier::T11));
    assert_eq!(tier_of(Some("chorus")), Some(Tier::T12));
    assert_eq!(tier_of(Some("warden")), Some(Tier::T13));
    assert_eq!(tier_of(Some("foundry")), Some(Tier::T14));
    assert_eq!(tier_of(Some("nexus")), Some(Tier::T15));
}

/// AN UNREADABLE SEAT REFUSES. It must not fall through to the common —
/// T3 is reached by holding no seat, never by holding an unknown one.
#[test]
fn an_unknown_station_refuses_rather_than_defaulting() {
    use ledger::registry::{tier_of, Tier};
    assert_eq!(tier_of(Some("glasskeeper")), None, "an unknown seat defaulted");
    assert_eq!(tier_of(Some("")), None, "an empty seat defaulted");
    assert_ne!(
        tier_of(Some("archon")),
        Some(Tier::T3),
        "an unreadable seat landed in the common — that is the hole a gate would leak through"
    );
}

/// The serving-seat reading: any seated office (T0–T2, T4–T9) serves;
/// T3 is the common; an unreadable seat answers neither.
#[test]
fn serving_seats_are_the_seated_offices() {
    use ledger::registry::holds_a_serving_seat;
    for seated in [
        "progenitor", "watcher", "observer", "sovereign",
        "caravan", "crucible", "exchange", "assay", "vault", "anchor",
        "herald", "lattice", "chorus", "warden", "foundry", "nexus",
    ] {
        assert_eq!(holds_a_serving_seat(Some(seated)), Some(true), "{seated} did not read as seated");
    }
    assert_eq!(holds_a_serving_seat(None), Some(false), "the common read as a seated office");
    assert_eq!(holds_a_serving_seat(Some("nonesuch")), None, "an unknown seat answered a gate");
}

/// T1.3 — seat widths for the economic offices (base + 25 expansion;
/// Anchor/Sovereign/Nexus unbounded).
#[test]
fn economic_office_seat_widths() {
    use ledger::registry::seat_width;
    for office in ["caravan", "crucible", "exchange", "assay", "vault", "herald", "lattice"] {
        assert_eq!(seat_width(office), Some(41), "{office} width = 16+25");
    }
    assert_eq!(seat_width("anchor"), None, "anchor is unbounded");
    assert_eq!(seat_width("sovereign"), None, "sovereign is unbounded");
    assert_eq!(seat_width("nexus"), None, "nexus is unbounded");
    assert_eq!(seat_width("progenitor"), Some(33), "progenitor = 8+25");
}

/// T1.2 — typed preconditions name the cure; band-1 offices need none.
#[test]
fn office_preconditions_name_the_cure() {
    use ledger::registry::{office_preconditions, OfficeEvidence};
    let empty = OfficeEvidence::default();
    for band1 in ["progenitor", "watcher", "observer", "sovereign"] {
        assert!(office_preconditions(band1, &empty).is_ok(), "{band1}");
    }
    assert!(
        office_preconditions("caravan", &empty).is_err(),
        "caravan without evidence must refuse"
    );
    let with_elements = OfficeEvidence { element_kinds: 1, ..Default::default() };
    assert!(office_preconditions("caravan", &with_elements).is_ok());
    assert!(office_preconditions("crucible", &with_elements).is_ok());
    assert!(office_preconditions("exchange", &with_elements).is_ok());
    assert!(
        office_preconditions("assay", &with_elements).is_err(),
        "assay needs AssayStruck"
    );
    let assayed = OfficeEvidence { assays_struck: 1, ..Default::default() };
    assert!(office_preconditions("assay", &assayed).is_ok());
    assert!(
        office_preconditions("anchor", &with_elements).is_err(),
        "anchor needs a void crossing"
    );
    let crossed = OfficeEvidence { has_crossed: true, ..Default::default() };
    assert!(office_preconditions("anchor", &crossed).is_ok());
    assert!(office_preconditions("herald", &crossed).is_ok());
    assert!(office_preconditions("nexus", &crossed).is_ok());
}

/// T1.3 — NoSeatOpen for caravan when the room is full (width 16+25).
#[test]
fn caravan_room_fills_at_width_sixteen() {
    use ledger::registry::{Registry, RegistryRefused};
    let root = scratch("caravan-room");
    let registry = Registry::at(&root);
    let width = ledger::registry::seat_width("caravan").expect("w");
    for n in 0..width {
        let mut line = humans();
        line.name = format!("Caravan {n}");
        line.sigil = format!("caravan-{n}");
        line.station = Some("caravan".into());
        registry.create(&line).expect("a seat stands open");
    }
    let mut overflow = humans();
    overflow.sigil = "caravan-full".into();
    overflow.station = Some("caravan".into());
    assert_eq!(
        registry.create(&overflow),
        Err(RegistryRefused::NoSeatOpen {
            station: "caravan".into(),
            domain: "mesic".into(),
            held: width,
            width,
        })
    );
}
