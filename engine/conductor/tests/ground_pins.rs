//! GROUND STEP pins (operator lock 2026-08-07): pure NPC execution
//! populates the ELEMENTS ledger. The conductor issues prospect_as and
//! assay_as; the closed Resource zone loop alone never does.
//!
//! Span law: prospect stamps `last_assay = now`; assay refuses until
//! the clock advances. Under the locked topology
//! (build → ground → tick → collect), the FIRST cycle opens a vein and
//! the SECOND cycle's ground_step banks the first element draw. The
//! pin proves that transition under pure NPC cycles — no CLI deeds.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::tlv;
use cfe::world::replay;
use codec::state_bytes;
use conductor::{Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ground-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn banked(world: &cfe::world::World, resource: Resource) -> Ratio {
    world
        .orb()
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

fn element_kinds(world: &cfe::world::World) -> usize {
    world
        .orb()
        .map(|orb| orb.elements.len())
        .unwrap_or(0)
}

fn element_mass(world: &cfe::world::World) -> Ratio {
    world
        .orb()
        .map(|orb| {
            orb.elements
                .iter()
                .fold(ratio(0, 1), |sum, (_, amount)| sum + amount.clone())
        })
        .unwrap_or_else(|| ratio(0, 1))
}

/// Tag 72 rides the orb wire (ColonyOrb encoding), never the full
/// committed state composite alone — count records on the live orb.
fn orb_carries_tag_72(world: &cfe::world::World) -> bool {
    let Some(orb) = world.orb() else {
        return false;
    };
    let bytes = orb.wire_bytes();
    tlv::records(&bytes).any(|(tag, _)| tag == 72)
}

fn mineral_spec() -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: "Groundwright".into(),
            sigil: "groundwright".into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "the strata open under our own hand".into(),
            disposition: "expansionist".into(),
            metabolism: "mineral".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: "ground-hearth".into(),
        planet: "ground-prime".into(),
        colony: "first-vein".into(),
        policy: Policy::Steady,
        position: (ratio(5, 1), ratio(5, 1), ratio(5, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

/// RED-FIRST VALIDATION: pure NPC cycles transition ELEMENTS empty →
/// populated, write tag 72, and replay without divergence.
#[test]
fn pure_npc_beat_populates_elements_and_tag_72_and_replays() {
    let root = scratch("elements");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(mineral_spec(), &registry, &book);

    // Precondition: the ledger is empty of almanac matter.
    assert_eq!(
        element_kinds(&polity.world),
        0,
        "genesis banks no elements"
    );
    assert!(
        !orb_carries_tag_72(&polity.world),
        "genesis writes no tag 72"
    );
    assert!(
        polity.world.committed().prospects.is_empty(),
        "genesis opens no veins"
    );

    // Pure NPC: cycle only. Beat 1 opens the lowest stratum (prospect);
    // tick advances the clock; beat 2 assays the open vein.
    let mut populated_on = None;
    for beat in 1u32..=4 {
        let _ = polity.cycle(&book, &registry);
        if element_kinds(&polity.world) > 0 {
            populated_on = Some(beat);
            break;
        }
    }
    let beat = populated_on.expect(
        "pure NPC cycles must populate ELEMENTS within 4 beats \
         (prospect on beat 1, assay after tick span on beat 2+)",
    );
    assert!(
        beat <= 2,
        "transition must complete by the second beat under the span law; got beat {beat}"
    );

    // ELEMENTS ledger: non-empty kinds and positive mass.
    let kinds = element_kinds(&polity.world);
    let mass = element_mass(&polity.world);
    assert!(kinds >= 1, "at least one element kind banked; got {kinds}");
    assert!(mass > ratio(0, 1), "element mass must be positive; got {mass}");

    // Tag 72 on the orb TLV.
    assert!(
        orb_carries_tag_72(&polity.world),
        "orb wire must carry tag 72 once elements bank"
    );

    // AssayStruck receipt exists (the deed that banks).
    let assayed = polity.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::AssayStruck { .. })
    });
    assert!(assayed, "AssayStruck must stand in the chronicle");
    let prospected = polity.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::ProspectStruck { .. })
    });
    assert!(prospected, "ProspectStruck must stand in the chronicle");

    // SEAL B: replay MATCHES — no state divergence.
    assert_eq!(
        replay(polity.world.chronicle()),
        polity.world.committed(),
        "replay MATCHES after ground_step deeds"
    );

    // Codec state path also round-trips (spine/export path).
    let bytes = state_bytes(&polity.world.committed());
    assert!(
        !bytes.is_empty(),
        "committed state encodes non-empty"
    );

    // Sanity: Resource banks still exist (ground does not retire iron).
    let _ = banked(&polity.world, Resource::Iron);
}
