//! GALAXY-1b pins (13_GALAXY §3, §5): the crossing observes and the
//! observation creates, wired through the conductor's year-voice
//! sweep; the sweep is idempotent by geometry; the creator resolves
//! through the lineage law (the head's seat, 12 §5).

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{aperture_transfer, FIRST_ORB};
use conductor::{observe_crossings, Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "galaxy-trigger-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn seeded(
    name: &str,
    sigil: &str,
    metabolism: &str,
    lineage: Vec<String>,
    world_name: &str,
    position: (Ratio, Ratio, Ratio),
) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: name.into(),
            sigil: sigil.into(),
            species: "witness".into(),
            clade: String::new(),
            motto: "we pin the law".into(),
            disposition: "expansionist".into(),
            metabolism: metabolism.into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage,
            station: None,
        },
        world_name: world_name.into(),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position,
        star: Some(StarClass::G),
        albedo: Some(ratio(1, 2)),
        orbit: Some(ratio(1, 2)),
        gauge: None,
    }
}

#[test]
fn the_crossing_observes_and_the_observation_creates() {
    let root = scratch("sweep");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));

    // The tardigrade head seated first: the far line's creator
    // resolves through its lineage pointer (12 §5).
    registry
        .create(&Civilization {
            name: "Tunborn".into(),
            sigil: "tunborn".into(),
            species: "tardigrade progenitor".into(),
            clade: String::new(),
            motto: "the dark between stars is a bed, not a wall".into(),
            disposition: "expansionist".into(),
            metabolism: "mineral".into(),
            perception: "planar".into(),
            founder: "operator".into(),
            lineage: Vec::new(),
            station: Some("progenitor".into()),
        })
        .expect("the head seats");

    // Two sited worlds: one charted FAR (wrapped-L1 24 from origin —
    // past the founding reach 16), one NEAR (L1 3 — inside it).
    let far = seeded(
        "Slagborn", "slagborn", "mineral", vec!["tunborn".into()],
        "cinder-reach", (ratio(40, 1), ratio(0, 1), ratio(0, 1)),
    );
    let near = seeded(
        "Deep Chorus", "deep-chorus", "biotic", Vec::new(),
        "kepler-passage", (ratio(1, 1), ratio(1, 1), ratio(1, 1)),
    );
    let mut worlds = vec![
        Polity::genesis(far, &registry, &book),
        Polity::genesis(near, &registry, &book),
    ];

    // Before any crossing the sweep observes nothing — not even the
    // founding galaxy charters (13_GALAXY §2: lazy, never pre-tiled).
    assert!(observe_crossings(&worlds, &registry, &book, 60).is_empty());
    assert!(book.galaxies().is_empty());

    // The first aperture crosses BOTH worlds into the union
    // (THRESHOLD-1), and the next year-voice sweep observes: the far
    // anchor creates its galaxy, the near one sits in the founding.
    let (far_half, near_half) = worlds.split_at_mut(1);
    aperture_transfer(
        &mut far_half[0].world, FIRST_ORB,
        &mut near_half[0].world, FIRST_ORB,
        Resource::Iron, cfe::dop::positive(5, 1), 9,
    )
    .expect("the aperture opens");
    assert!(worlds[0].world.has_crossed() && worlds[1].world.has_crossed());

    let created = observe_crossings(&worlds, &registry, &book, 60);
    assert_eq!(created.len(), 1, "one galaxy observed past the frontier");
    assert_eq!(created[0].name, "slagborn-sky-1");
    assert_eq!(book.galaxies().len(), 2, "the founding + the observed");

    // Idempotent by geometry: the created galaxy encloses its
    // observer, so the next sweep creates nothing.
    assert!(observe_crossings(&worlds, &registry, &book, 120).is_empty());
    assert_eq!(book.galaxies().len(), 2);

    // Membership derives from position, never stored.
    assert_eq!(book.galaxy_of("cinder-reach").as_deref(), Some("slagborn-sky-1"));
    assert_eq!(book.galaxy_of("kepler-passage").as_deref(), Some("the-founding"));

    // The creator resolved through the lineage law: the head's seat
    // executed the deed, at the sweep's universal stamp.
    let ledger = cfe::persist::load(&root.join("worlds/convergence.tlv")).expect("ledger");
    assert!(ledger.receipts().iter().any(|receipt| {
        receipt.stamp == 60
            && matches!(
                &receipt.event,
                ClosureEvent::GalaxyObserved { creator, name, observer, .. }
                    if creator == "tunborn" && observer == "slagborn" && name == "slagborn-sky-1"
            )
    }));
}
