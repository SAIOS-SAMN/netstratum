//! BOT-THERM pins: NPC thermal adoption — the conductor's
//! civilizations live under REAL suns. Genesis commissions the star
//! and sites the planet (spec-pinned or chronicle-derived — CHARTER
//! §9, no wall clock); the Furnace mints joules through its shells;
//! the Frost banks cold, answers the joule runway, and is honestly
//! allowed to freeze-lock; the Terminal Wall carries the drama.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use conductor::{
    founding_egregores, joule_runway_allows, load_egregores, run_era, write_default_egregores, EgregoreSpec,
    Egregore, Policy,
};
use mirror::registry::{Civilization, Registry};
use mirror::Mirror;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "egregore-therm-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn banked(egregore: &Egregore, resource: Resource) -> Ratio {
    egregore.world
        .orb()
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

fn kind_count(egregore: &Egregore, kind: ZoneKind) -> usize {
    egregore.world
        .orb()
        .map(|orb| orb.zones.iter().filter(|z| z.kind == kind).count())
        .unwrap_or(0)
}

fn zones(egregore: &Egregore) -> usize {
    egregore.world.orb().map(|orb| orb.zones.len()).unwrap_or(0)
}

/// Thermal genesis is DETERMINISTIC: the same spec founds the same
/// star and the same orbit twice — receipts byte-identical — on both
/// the derived path (no thermal fields: the civilization's own genesis
/// receipts seed the beacon) and the pinned path (explicit fields).
#[test]
fn thermal_genesis_is_deterministic() {
    let found = |name: &str, spec: EgregoreSpec| -> Egregore {
        let root = scratch(name);
        Egregore::genesis(
            spec,
            &Registry::at(root.join("registry")),
            &Mirror::at(root.join("worlds")),
        )
    };
    let derived = founding_egregores()
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

    let pinned = founding_egregores()
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

/// A default-seeded era is thermally ALIVE: at least one Furnace egregore
/// mints joules through encasement shells, at least one Frost egregore
/// raises a cryo-lattice array and banks sink — and the thermal state
/// flows to `state.json` (the mirror pipeline, witnessed end-to-end).
#[test]
fn a_default_era_mints_hot_and_banks_cold() {
    let root = scratch("alive");
    let worlds = run_era(&root.to_string_lossy(), 30);
    let furnace = worlds
        .iter()
        .find(|b| {
            kind_count(b, ZoneKind::Encasement) > 0 && banked(b, Resource::Joule) > ratio(0, 1)
        })
        .expect("a Furnace egregore minted joules through its shells");
    let frost = worlds
        .iter()
        .find(|b| kind_count(b, ZoneKind::CryoArray) > 0)
        .expect("a Frost egregore raised a cryo-lattice array");
    assert!(
        banked(frost, Resource::Sink) > ratio(0, 1),
        "the Frost banks cold"
    );
    assert_ne!(furnace.spec.civilization.slug, frost.spec.civilization.slug);

    // The observer pipeline: the M sun reaches state.json unchanged.
    let state = fs::read_to_string(
        root.join("worlds")
            .join(&frost.spec.world_name)
            .join("planets")
            .join(&frost.spec.planet)
            .join("state.json"),
    )
    .expect("state.json mirrored");
    assert!(
        state.contains("\"star_class\":7"),
        "the M sun reaches the observer: {state}"
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
    let mirror = Mirror::at(root.join("worlds"));
    let seat = |slug: &str, star: StarClass| EgregoreSpec {
        civilization: Civilization {
            name: slug.to_string(),
            slug: slug.to_string(),
            species: "wood frog".into(),
            motto: "measured against the cold".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: None,
            station: None,
        },
        world_name: format!("{slug}-hold"),
        planet: format!("{slug}-p"),
        colony: format!("{slug}-c"),
        policy: Policy::Aggressive,
        position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
        star: Some(star),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
    };
    let mut frost = Egregore::genesis(seat("stillside", StarClass::M), &registry, &mirror);
    let mut mesic = Egregore::genesis(seat("plainside", StarClass::G), &registry, &mirror);
    for _ in 0..12 {
        frost.cycle(&mirror, &registry);
        mesic.cycle(&mirror, &registry);
    }
    assert_eq!(zones(&frost), 1, "the Frost stopped at its sink charter");
    assert_eq!(kind_count(&frost, ZoneKind::CryoArray), 1);
    assert!(
        zones(&mesic) >= 4,
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

/// A year of a thermally sited era SPEAKS: the journals carry the
/// band register and the joule/famine vocabulary — the Terminal Wall
/// reports real thermodynamic drama, derived, never invented.
#[test]
fn a_thermal_year_reaches_the_wall() {
    let root = scratch("wall");
    let _ = run_era(&root.to_string_lossy(), 60);
    let mut journals = String::new();
    for entry in fs::read_dir(root.join("worlds")).expect("worlds") {
        let dir = entry.expect("entry").path();
        if let Ok(text) = fs::read_to_string(dir.join("journal.jsonl")) {
            journals.push_str(&text);
        }
    }
    assert!(!journals.is_empty(), "year boundaries journal");
    for phrase in [
        "joule bank",                     // the joule stance line
        "heating bill",                   // ECON-J vocabulary
        "FREEZE-LOCK",                    // the Frost's honest failure
        "S_cool",                         // sink offered to Furnace traders
        "Emberkin (Radiotroph)",          // the hot band register
        "Coherent (Cryo-Superconductor)", // the vault register
    ] {
        assert!(journals.contains(phrase), "the wall carries {phrase:?}");
    }
}

/// The operator's file: thermal fields round-trip through the seeded
/// egregores.json; files WITHOUT thermal fields (the pre-BOT-THERM shape)
/// still load and found thermally-derived worlds — compat held.
#[test]
fn old_bots_files_still_load_and_derive() {
    let root = scratch("compat");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("egregores.json");
    write_default_egregores(&path).expect("seeds");
    let seeded = load_egregores(&path).expect("loads");
    assert_eq!(seeded.len(), founding_egregores().len());
    assert!(
        seeded.iter().any(|s| s.star == Some(StarClass::M)),
        "the Frost seat survives the round-trip"
    );
    assert!(
        seeded
            .iter()
            .any(|s| s.star == Some(StarClass::G) && s.orbit == Some(ratio(1, 2))),
        "the Furnace seat survives the round-trip"
    );
    assert!(
        seeded
            .iter()
            .any(|s| s.star.is_none() && s.albedo.is_none() && s.orbit.is_none()),
        "mesic seats stay derived in the seeded file"
    );

    // A pre-BOT-THERM entry — no thermal fields at all — still loads…
    let legacy = root.join("legacy.json");
    fs::write(
        &legacy,
        r#"[{"name":"Fernwake","slug":"fernwake","species":"monarch butterfly",
           "motto":"drift and arrive","disposition":"epicurean","metabolism":"biotic",
           "perception":"volumetric","world":"lantern-sea","planet":"petal",
           "colony":"alight","policy":"steady","x":"3","y":"1/2","z":"-4"}]"#,
    )
    .expect("writes");
    let specs = load_egregores(&legacy).expect("the old shape still loads");
    assert!(
        specs[0].star.is_none() && specs[0].albedo.is_none() && specs[0].orbit.is_none(),
        "absent fields stay absent on the spec"
    );

    // …and its genesis derives a real sun and siting anyway.
    let egregore = Egregore::genesis(
        specs[0].clone(),
        &Registry::at(root.join("registry")),
        &Mirror::at(root.join("worlds")),
    );
    assert!(egregore.world.star().is_some(), "the derived sun is commissioned");
    assert!(
        egregore.world.orb().expect("orb").thermal.is_some(),
        "the derived orbit is sited"
    );
}
