//! JOURNAL-1 witnesses (08 §4): the journal regenerates
//! byte-identically from the chronicle; a famine year raises a red
//! alert naming the resource; growth math is exact; the surplus/deficit
//! narratives flip on the exact boundary; the wall carries the entries
//! with only inline assets; a journal-free replay is unaffected — the
//! feed is a RENDERING, never state (08 §1.1); the terminology law
//! holds in every generated word.

use std::fs;
use std::path::PathBuf;

use cfe::dop::ZoneKind;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::ChainRead;
use cfe::world::{replay, World};
use scribe::journal::{journal, joule_stance, sink_offer, Alert, JouleStance, Voice, YEAR_TICKS};
use ledger::Ledger;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "journal-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn voice() -> Voice {
    Voice {
        civilization: "Humans".into(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
    }
}

/// Two full years of steady growth: 2 zones founded in year one, a
/// third built in year two — the exact-growth ladder.
fn growth_world() -> World {
    let mut world = World::new();
    world.found("first-landing").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..60 {
        world.tick();
    }
    world.collect().expect("collects");
    world.zone(1, ZoneKind::Extractor).expect("zones");
    for _ in 0..60 {
        world.tick();
    }
    world
}

/// A biomass famine on schedule: two extractors, no habitat — the
/// stores run dry at exactly tick 11 (D1 law) and year one closes red.
fn famine_world() -> World {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(1, ZoneKind::Extractor).expect("zones");
    for _ in 0..60 {
        world.tick();
    }
    world
}

/// A sub-mesic orbit with no fuel: ECON-J heating strikes a joule
/// famine on the first tick and the colony freeze-locks.
fn frozen_world() -> World {
    let mut world = World::new();
    world.found("frostfall").expect("founds");
    world.commission_star(StarClass::M).expect("commissions");
    world.site_planet(1, ratio(0, 1), ratio(1, 1), None).expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..60 {
        world.tick();
    }
    world
}

/// A hot orbit minting joules through an encasement shell (05 §1.2):
/// banked joules with no heating bill — the surplus narrative.
fn furnace_world() -> World {
    let mut world = World::new();
    world.found("emberhold").expect("founds");
    world.commission_star(StarClass::A).expect("commissions");
    world.site_planet(1, ratio(0, 1), ratio(1, 1), None).expect("sites");
    world.zone(0, ZoneKind::Encasement).expect("zones");
    for _ in 0..30 {
        world.tick();
    }
    world.collect().expect("collects");
    for _ in 0..30 {
        world.tick();
    }
    world
}

/// 08 §1.1–§1.2: same chronicle ⇒ byte-identical journal AND wall; a
/// re-record appends nothing (no duplicate year); a deleted file
/// regenerates to the same bytes.
#[test]
fn the_journal_regenerates_byte_identically() {
    let root = scratch("regen");
    let book = Ledger::at(&root);
    let world = growth_world();
    let appended = scribe::record_journal(&book, "sol", &world, &voice())
        .expect("records");
    assert_eq!(appended, 2, "two completed years, two entries");
    let path = book.world_dir("sol").join("journal.jsonl");
    let first = fs::read(&path).expect("journal exists");
    let wall_first = fs::read(root.join("wall.html")).expect("wall exists");

    // Idempotence: a re-record appends NOTHING.
    let again = scribe::record_journal(&book, "sol", &world, &voice()).expect("re-records");
    assert_eq!(again, 0, "no duplicate year, ever");
    assert_eq!(fs::read(&path).expect("read"), first);

    // Regeneration: delete everything, regenerate byte-identically.
    fs::remove_file(&path).expect("deletes journal");
    fs::remove_file(root.join("wall.html")).expect("deletes wall");
    scribe::record_journal(&book, "sol", &world, &voice()).expect("regenerates");
    assert_eq!(fs::read(&path).expect("read"), first, "byte-identical journal");
    assert_eq!(
        fs::read(root.join("wall.html")).expect("read"),
        wall_first,
        "byte-identical wall"
    );

    // The pure generator is deterministic on its own.
    assert_eq!(
        journal("sol", &world, &voice(), YEAR_TICKS),
        journal("sol", &world, &voice(), YEAR_TICKS)
    );
}

/// 08 §2: a famine year closes as a red alert NAMING the resource,
/// with the dormant zones named and the want on the trade line.
#[test]
fn a_famine_year_raises_a_red_alert_naming_the_resource() {
    let world = famine_world();
    let entries = journal("meridian", &world, &voice(), YEAR_TICKS);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(entry.alert, Alert::Warning);
    assert_eq!(entry.famine, vec!["biomass".to_string()]);
    assert!(entry.headline.contains("RED ALERT"), "the headline is red");
    assert!(entry.headline.contains("biomass"), "the resource is named");
    assert!(
        entry.body.contains("famine on biomass"),
        "the body names the famine"
    );
    assert!(
        entry.body.contains("dormant: extractor"),
        "the dormant zones are named"
    );
    assert_eq!(
        entry.trade_request.as_deref(),
        Some("seeking biomass"),
        "the deficit derives the trade request"
    );
}

/// 08 §2: growth is exact rational arithmetic — 2 zones over a bare
/// founding, then 1 over 2 standing is EXACTLY 1/2 per year; the entry
/// header carries the base-60 odometer cycle.
#[test]
fn growth_math_is_exact() {
    let world = growth_world();
    let entries = journal("sol", &world, &voice(), YEAR_TICKS);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].year, 1);
    assert_eq!(entries[0].economics.zones, 2);
    assert_eq!(entries[0].economics.zones_built, 2);
    assert_eq!(entries[0].economics.growth_per_year, "2");
    assert_eq!(entries[0].cycle, "00:00:00:00:01:00", "the odometer cycle");
    assert_eq!(entries[0].alert, Alert::Milestone, "a building year is blue");
    assert_eq!(entries[1].year, 2);
    assert_eq!(entries[1].economics.zones, 3);
    assert_eq!(entries[1].economics.zones_built, 1);
    assert_eq!(
        entries[1].economics.growth_per_year, "1/2",
        "one zone over two standing — exact, never a percentage"
    );
    assert_eq!(entries[1].cycle, "00:00:00:00:02:00");
    // No decimal ever enters the feed.
    for entry in &entries {
        let line = scribe::journal::render_entry(entry);
        assert!(!line.contains("0.5"), "no float rendering: {line}");
    }
}

/// 08 §4: the surplus/deficit narratives flip on the EXACT boundary —
/// covered at banked == need, deficit one part in a million below; and
/// the two narratives land in real entries (the furnace builds, the
/// frozen colony begs fuel before freeze-lock).
#[test]
fn surplus_and_deficit_flip_on_the_exact_boundary() {
    // The pure boundary: exact rational comparison, no rounding.
    let need = ratio(30, 1);
    assert_eq!(joule_stance(&ratio(30, 1), &need), JouleStance::Covered);
    assert_eq!(
        joule_stance(&(ratio(30, 1) - ratio(1, 1_000_000)), &need),
        JouleStance::Deficit
    );

    // The surplus narrative: banked joules, no bill — building.
    let hot = furnace_world();
    let entries = journal("emberhold", &hot, &voice(), YEAR_TICKS);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].economics.banked_joules, "225/4", "minted exactly");
    assert!(
        entries[0].body.contains("building and expanding"),
        "the surplus builds"
    );
    assert!(!entries[0].body.contains("freeze-lock"), "no false alarm");

    // The deficit narrative: a cold orbit with no fuel — RED, fuel
    // sought before freeze-lock, and the want on the trade line.
    let cold = frozen_world();
    let entries = journal("frostfall", &cold, &voice(), YEAR_TICKS);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].alert, Alert::Warning);
    assert!(entries[0].body.contains("fuel is sought before freeze-lock"));
    assert!(entries[0].body.contains("FREEZE-LOCK"));
    assert!(entries[0]
        .trade_request
        .as_deref()
        .expect("a frozen colony trades")
        .contains("seeking joules"));
}

/// 08 §3: the wall carries every entry, newest first, with ONLY inline
/// assets — no external request of any kind — and the inscribed
/// cyber-noir palette.
#[test]
fn the_wall_holds_the_entries_with_only_inline_assets() {
    let root = scratch("wall");
    let book = Ledger::at(&root);
    scribe::record_journal(&book, "sol", &growth_world(), &voice())
        .expect("records");
    scribe::record_journal(&book, "meridian", &famine_world(), &voice())
        .expect("records");
    let wall = fs::read_to_string(root.join("wall.html")).expect("wall exists");

    // The entries are present, with the filter bar's data attributes.
    assert!(wall.contains("sol"), "the growth world is on the wall");
    assert!(wall.contains("meridian"), "the famine world is on the wall");
    assert!(wall.contains("RED ALERT"), "the red alert shows");
    assert!(wall.contains("data-alert=\"warning\""));
    assert!(wall.contains("id=\"f-world\""), "the filter bar is present");
    assert!(wall.contains("id=\"f-archetype\""));
    assert!(wall.contains("id=\"f-alert\""));

    // Newest first: sol's year 2 card precedes every year-1 card.
    let year2 = wall.find("year 2 ·").expect("year 2 card");
    let year1 = wall.find("year 1 ·").expect("year 1 card");
    assert!(year2 < year1, "the feed is newest-first");

    // The palette (08 §3).
    for color in ["#0a0a10", "#00f3ff", "#00ffbf", "#ff0055"] {
        assert!(wall.contains(color), "palette carries {color}");
    }

    // Self-contained: inline CSS/JS only, nothing fetched.
    assert!(wall.contains("<style>") && wall.contains("<script>"));
    for external in ["http://", "https://", "<link", "@import", "url(", "src="] {
        assert!(!wall.contains(external), "no external request: {external}");
    }
}

/// 08 §1.1, the reflection law: the journal is a RENDERING. Deleting
/// every journal artifact changes no engine state — the chronicle
/// stays intact, replay still grounds the same committed world, and
/// the exported chronicle bytes never move.
#[test]
fn a_journal_free_replay_is_unaffected() {
    let root = scratch("rendering-only");
    let book = Ledger::at(&root);
    let world = growth_world();
    let committed = world.committed();
    scribe::record(&book, "sol", "prime", "humans", &world).expect("records");
    let chronicle_path = book.world_dir("sol").join("chronicle.jsonl");
    let chronicle_bytes = fs::read(&chronicle_path).expect("chronicle");
    let receipts = world.chronicle().receipts().len();

    scribe::record_journal(&book, "sol", &world, &voice()).expect("journals");
    fs::remove_file(book.world_dir("sol").join("journal.jsonl")).expect("deletes");
    fs::remove_file(root.join("wall.html")).expect("deletes");

    // Nothing moved: no receipt appeared, the chain verifies, replay
    // grounds the same world, the exported stream is byte-identical.
    assert_eq!(world.chronicle().receipts().len(), receipts);
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(replay(world.chronicle()), committed);
    assert_eq!(fs::read(&chronicle_path).expect("chronicle"), chronicle_bytes);
}

/// The terminology law (CHARTER §8, 08 §1.4): the sink offer speaks of
/// the Furnace — never the refused word — and no generated text ever
/// carries a set-letter glyph.
#[test]
fn the_terminology_law_holds_in_the_feed() {
    let offer = sink_offer(&ratio(5, 1)).expect("a banked sink offers");
    assert!(offer.contains("Furnace"), "the Furnace is named");
    assert!(offer.contains("5 S_cool"), "the capacity is exact");
    assert_eq!(sink_offer(&ratio(0, 1)), None, "nothing banked, nothing offered");

    let mut feed = String::new();
    for (world_name, world) in [
        ("sol", growth_world()),
        ("meridian", famine_world()),
        ("frostfall", frozen_world()),
        ("emberhold", furnace_world()),
    ] {
        for entry in journal(world_name, &world, &voice(), YEAR_TICKS) {
            feed.push_str(&scribe::journal::render_entry(&entry));
            feed.push('\n');
        }
    }
    feed.push_str(&offer);
    assert!(!feed.contains("Crucible"), "Furnace, never Crucible");
    // The double-struck set-letter glyphs (U+211A, U+2124), named by
    // code point so the refused glyphs never enter this source either.
    assert!(
        !feed.chars().any(|c| c == '\u{211A}' || c == '\u{2124}'),
        "no set-letter glyph"
    );
}
