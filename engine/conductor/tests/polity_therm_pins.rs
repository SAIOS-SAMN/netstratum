//! BOT-THERM pins: NPC thermal adoption — the conductor's
//! civilizations live under REAL suns. Genesis commissions the star
//! and sites the planet (spec-pinned or chronicle-derived — CHARTER
//! §9, no wall clock); the Furnace mints joules through its shells;
//! the Frost banks cold, answers the joule runway, and is honestly
//! allowed to freeze-lock; the Terminal Wall carries the drama.

use cfe::dop::Resource;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use conductor::{
    founding_polities, joule_runway_allows, PolitySpec,
    Polity, Policy,
};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "polity-therm-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn zones(polity: &Polity) -> usize {
    polity.world.orb().map(|orb| orb.zones.len()).unwrap_or(0)
}

/// Thermal genesis is DETERMINISTIC: the same spec founds the same
/// star and the same orbit twice — receipts byte-identical — on both
/// the derived path (no thermal fields: the civilization's own genesis
/// receipts seed the beacon) and the pinned path (explicit fields).
#[test]
fn thermal_genesis_is_deterministic() {
    let found = |name: &str, spec: PolitySpec| -> Polity {
        let root = scratch(name);
        Polity::genesis(
            spec,
            &Registry::at(root.join("registry")),
            &Ledger::at(root.join("worlds")),
        )
    };
    let derived = founding_polities()
        .into_iter()
        .find(|s| s.star.is_none())
        .expect("a derived (axis-sited) seat is seeded");
    let a = found("tg-a", derived.clone());
    let b = found("tg-b", derived);
    assert!(a.world.star().is_some(), "genesis commissions the sun");
    assert!(
        a.world.orb().expect("orb").thermal.is_some(),
        "genesis sites the orbit"
    );
    assert_eq!(a.world.star(), b.world.star());
    assert_eq!(
        a.world.chronicle().receipts(),
        b.world.chronicle().receipts(),
        "same spec, same star/site receipts, byte-for-byte"
    );

    let pinned = founding_polities()
        .into_iter()
        .find(|s| s.star == Some(StarClass::M))
        .expect("the Frost seat is seeded");
    let c = found("tg-c", pinned.clone());
    let d = found("tg-d", pinned);
    assert_eq!(c.world.star(), Some(StarClass::M), "the pinned sun rules");
    assert_eq!(
        c.world.chronicle().receipts(),
        d.world.chronicle().receipts()
    );
}


/// The joule runway measurably DEFERS the Frost: same policy, same
/// beats — the vault-sited colony stops at its sink charter (one
/// array) while the mesic control builds a city. The Frost line is
/// honestly allowed to fail: its joule famine (freeze-lock) is
/// legitimate drama, not a bug. The runway law itself flips on the
/// exact boundary.
#[test]
fn the_joule_runway_defers_the_frost() {
    let root = scratch("runway");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let seat = |sigil: &str, star: StarClass| PolitySpec {
        civilization: Civilization {
            name: sigil.to_string(),
            sigil: sigil.to_string(),
            species: "amphiform-rime".into(),
            clade: "amphiform".into(),
            motto: "measured against the cold".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hold"),
        planet: format!("{sigil}-p"),
        colony: format!("{sigil}-c"),
        policy: Policy::Aggressive,
        position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
        star: Some(star),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    };
    let mut frost = Polity::genesis(seat("stillside", StarClass::M), &registry, &book);
    let mut mesic = Polity::genesis(seat("plainside", StarClass::G), &registry, &book);
    for _ in 0..40 {
        frost.cycle(&book, &registry);
        mesic.cycle(&book, &registry);
    }
    // RE-PINNED 2026-08-23: `joule_runway_allows` (BOT-THERM) was written
    // against `build_step`'s zone-raising and orphaned when N-I6 moved
    // zone-raising to `express_survival` (`seat_zone`) without carrying
    // the gate — a Frost colony built with NO runway check at all, which
    // is why this pin previously inverted (frost outbuilt mesic). Wired
    // the gate into `seat_zone`, additionally to its ordinary per-zone
    // cost check. Re-measured on the current (independently slower —
    // #138/ONT-2b/GALACTIC-1 tightened economy pace across many
    // unrelated landings) economy: the Frost freeze-locks at its FIRST
    // zone and holds there permanently (stable to 100+ beats, checked);
    // the mesic control keeps building past it (also independently
    // slower than when this pin was last written — it now plateaus at
    // 3, not the stale 4, stable to 100+ beats too). `CryoArray` is
    // never built anywhere on the live conductor path
    // (`ration_source_zone` only ever returns Habitat or Extractor) —
    // the "sink charter" assertion tested dead functionality and is
    // dropped, not re-pinned.
    //
    // RE-PINNED 2026-08-23 (again, same beat): #88 gave the Deepen tension a
    // second, capacity-scaled consumer (`abundance_tensions`) so it no longer
    // dies the instant one hard science seats — it now competes against Expand
    // for longer, so the mesic control's 40-beat plateau measures one zone
    // lower (2, not 3). The invariants this pin actually guards — the Frost
    // freeze-locks at 1, and Frost < mesic — are untouched.
    assert_eq!(zones(&frost), 1, "the Frost freeze-locks at its first zone, its runway spent");
    assert!(
        zones(&mesic) >= 2,
        "the mesic control kept building: {}",
        zones(&mesic)
    );
    assert!(
        zones(&frost) < zones(&mesic),
        "the runway measurably defers expansion"
    );
    assert!(
        frost
            .world
            .orb()
            .expect("orb")
            .famine
            .contains(&Resource::Joule),
        "honestly failed: the joule famine (freeze-lock) is the drama"
    );

    // The runway law flips on the EXACT boundary: a one-zone colony
    // growing to two owes heating_per_zone × 2 = 1/2 per tick; the
    // bill met exactly holds the runway, one grain short refuses.
    assert!(joule_runway_allows(&ratio(1, 2), 1));
    assert!(!joule_runway_allows(&ratio(49, 100), 1));
}


/// THE AERIC DECK BAND IS UNREACHABLE — pinned, not repaired.
///
/// `deck_band4` clamps a reading to the mesic ceiling for an `"aeric"`
/// home (the RELEVANCE ENTRY ruling, 2026-07-31), but it tests the
/// METABOLISM axis and no metabolism can be aeric: the vocabulary is
/// closed at mineral/biotic/plasmic, and `12_UNION` §5a lists aeric as a
/// SUBSTRATE — a line that is itself biotic, living in a gas giant's
/// banded column. Substrates live in `clade`, whose vocabulary is a
/// closed u8 set (`15_TAXON` §3) that does not carry aeric either.
///
/// So the ruling has never applied to anything. This pin records that
/// as a KNOWN gap rather than letting the dead branch read as landed
/// law. It goes red the day the reading is wired to a field that can
/// actually say aeric — truth-update it then, and delete the clamp
/// assertion below, not this test.
#[test]
fn the_aeric_deck_band_is_unreachable_today() {
    use ledger::registry::METABOLISMS;
    // A reading DEFINITIONALLY past the mesic ceiling — derived from
    // the band itself, never a hand-picked constant. (Measured
    // 2026-08-03: the ceiling is 10_485_760_000 in T⁴ terms, and a
    // guessed "big number" of 10_000_000_000 sat under it — the guess
    // would have made every assertion below pass vacuously.)
    let (_, mesic_hi) = cfe::thermal::SubstrateBand::Mesic.bounds4();
    let scorching = mesic_hi.clone() * ratio(2, 1);
    for metabolism in METABOLISMS {
        assert_eq!(
            conductor::deck_band4(metabolism, scorching.clone()),
            scorching,
            "no metabolism in the closed vocabulary reads as aeric, so \
             the deck band never clamps: {metabolism}"
        );
    }
    // The clamp itself is real code and works — it simply has no
    // caller that can reach it. Handed the string directly, it does
    // displace to the ceiling.
    assert_eq!(
        conductor::deck_band4("aeric", scorching.clone()),
        mesic_hi,
        "the mapping is correct; only its input is unreachable"
    );
    assert!(
        !METABOLISMS.contains(&"aeric"),
        "the day aeric enters the metabolism vocabulary, this pin is the \
         place that says the wiring question was decided"
    );
}

/// **THE SITING TRIPLE STANDS OR FALLS WHOLE.**
///
/// This was a round-trip through the seeded `polities.json`. THE
/// DEFINITION LAW struck the file, and with it the hazard the round-trip
/// was really watching: the old shape let a file pin a star and omit its
/// albedo, and the loader filled the gap silently. The compiled roster
/// takes one `Option<(StarClass, Ratio, Ratio)>`, so the incoherent
/// middle is now UNREPRESENTABLE rather than merely untested — but the
/// seats it named still need pinning, so they are asserted directly.
#[test]
fn every_seat_is_wholly_pinned_or_wholly_derived() {
    for spec in founding_polities() {
        let pinned = [spec.star.is_some(), spec.albedo.is_some(), spec.orbit.is_some()];
        assert!(
            pinned.iter().all(|p| *p) || pinned.iter().all(|p| !*p),
            "{} is half-sited — star, albedo and orbit must stand or fall together",
            spec.civilization.sigil
        );
    }
    let founding = founding_polities();
    assert!(
        founding.iter().any(|s| s.star == Some(StarClass::M)),
        "the Frost seat stands in the roster"
    );
    assert!(
        founding
            .iter()
            .any(|s| s.star == Some(StarClass::G) && s.orbit == Some(ratio(1, 2))),
        "the Furnace seat stands in the roster"
    );
    assert!(
        founding
            .iter()
            .any(|s| s.star.is_none() && s.albedo.is_none() && s.orbit.is_none()),
        "and mesic seats stay derived"
    );
}

/// An UNSITED seat still founds a real sun at genesis — the derived
/// path stays exercised, which is the whole reason mesic seats are left
/// unpinned in the roster rather than filled in for tidiness.
#[test]
fn an_unsited_seat_still_derives_its_sun_at_genesis() {
    let root = scratch("derive");
    fs::create_dir_all(&root).expect("scratch");
    let spec = founding_polities()
        .into_iter()
        .find(|s| s.star.is_none())
        .expect("a derived seat stands in the roster");

    let polity = Polity::genesis(
        spec,
        &Registry::at(root.join("registry")),
        &Ledger::at(root.join("worlds")),
    );
    assert!(polity.world.star().is_some(), "the derived sun is commissioned");
    assert!(
        polity.world.orb().expect("orb").thermal.is_some(),
        "the derived orbit is sited"
    );
}
