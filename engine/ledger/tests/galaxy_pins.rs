//! GALAXY-1a pins (13_GALAXY §4–§5) — RED FIRST: these witnesses name
//! the mechanic before it exists. One universe structurally; galaxies
//! lazy, created only by the observation deed; membership derived,
//! never stored; the card is a rendering, the ledger is the truth.

use cfe::ratio::ratio;
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("galaxy-pin-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The founding galaxy is chartered LAZILY: no deed, no galaxies —
/// the board does not open pre-tiled (13_GALAXY §2).
#[test]
fn no_galaxy_before_the_first_observation() {
    let m = Ledger::at(scratch("lazy"));
    m.chart_world("first-hearth", (&ratio(1, 1), &ratio(1, 1), &ratio(1, 1)), &ratio(0, 1))
        .expect("charts");
    assert!(m.galaxies().is_empty(), "the count starts at zero cards; one galaxy IMPLICIT");
}

/// The first observation deed charters the founding galaxy
/// retroactively (placeholder center (0,0,0), reach 16) — and an
/// anchor INSIDE that reach creates nothing further: observation
/// within the charted sky is not discovery (13_GALAXY §3–§4).
#[test]
fn first_deed_charters_the_founding_galaxy_and_inside_reach_creates_nothing() {
    let m = Ledger::at(scratch("founding"));
    m.chart_world("home", (&ratio(0, 1), &ratio(0, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts");
    // A crossing anchored at (2,-3,1): L1 = 6 < 16 — inside the founding reach.
    let created = m
        .observe_galaxy("slagborn", "tunborn", (&ratio(2, 1), &ratio(-3, 1), &ratio(1, 1)), 60)
        .expect("the deed executes");
    let galaxies = m.galaxies();
    assert_eq!(galaxies.len(), 1, "the founding galaxy chartered lazily, nothing else");
    assert!(created.is_none(), "inside every reach, observation creates nothing");
}

/// A crossing anchored PAST every charted reach (L1-wrapped) CREATES
/// the galaxy it observes — the observation law (13_GALAXY §3).
#[test]
fn past_the_frontier_the_observation_creates_the_galaxy() {
    let m = Ledger::at(scratch("frontier"));
    // Anchor at (30,0,0): L1 from (0,0,0) = 30 > 16 — past the founding reach.
    let created = m
        .observe_galaxy("slagborn", "tunborn", (&ratio(30, 1), &ratio(0, 1), &ratio(0, 1)), 60)
        .expect("the deed executes");
    let galaxy = created.expect("past the frontier, the galaxy IS created");
    assert_eq!(m.galaxies().len(), 2, "founding + observed");
    // Membership is DERIVED, never stored: a world charted beside the
    // new center belongs to it; the far founding worlds do not.
    m.chart_world("far-hold", (&ratio(31, 1), &ratio(1, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts");
    m.chart_world("home", (&ratio(0, 1), &ratio(0, 1), &ratio(0, 1)), &ratio(0, 1))
        .expect("charts");
    assert_eq!(m.galaxy_of("far-hold").as_deref(), Some(galaxy.name.as_str()));
    assert_ne!(m.galaxy_of("home").as_deref(), Some(galaxy.name.as_str()));
}

/// The L1 law wraps: an anchor at span-1 sits distance 1 from the
/// origin, the short way around — never past the frontier (no edge,
/// no squaring).
#[test]
fn the_frontier_reading_wraps_on_the_torus() {
    let m = Ledger::at(scratch("wrap"));
    // (63,0,0) on span 64 wraps to L1 = 1 from the founding center.
    let created = m
        .observe_galaxy("slagborn", "tunborn", (&ratio(63, 1), &ratio(0, 1), &ratio(0, 1)), 60)
        .expect("the deed executes");
    assert!(created.is_none(), "wrapped L1 = 1 stands INSIDE the founding reach");
}

/// SEAL C: `galaxies.card` is a rendering. Delete the card — replay
/// from the universe ledger rebuilds it exactly (the receipt is the
/// truth, 13_GALAXY §4).
#[test]
fn the_card_is_a_rendering_the_ledger_is_the_truth() {
    let root = scratch("rendering");
    let m = Ledger::at(root.clone());
    m.observe_galaxy("slagborn", "tunborn", (&ratio(30, 1), &ratio(0, 1), &ratio(0, 1)), 60)
        .expect("the deed executes");
    let before = m.galaxies();
    assert_eq!(before.len(), 2);
    fs::remove_file(root.join("galaxies.card")).expect("the card is deletable");
    assert_eq!(m.galaxies(), before, "rebuilt from receipts, identical reading");
}

/// NS-3 discipline: the deed's wire form round-trips byte-exact at
/// its inscribed tag (54).
#[test]
fn the_observation_deed_round_trips_byte_exact() {
    use cfe::dop::ClosureEvent;
    let deed = ClosureEvent::GalaxyObserved {
        observer: "slagborn".into(),
        creator: "tunborn".into(),
        name: "slagborn-sky-1".into(),
        center: (ratio(30, 1), ratio(0, 1), ratio(1, 2)),
        reach: ratio(16, 1),
    };
    let bytes = deed.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 54, "the inscribed tag");
    assert_eq!(ClosureEvent::from_wire(tag, value), Some(deed));
}
