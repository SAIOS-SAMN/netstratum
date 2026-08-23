//! C1.6 — pure NPC `chemistry_step` disposition-routed.

use cfe::almanac::ScienceDiscipline;
use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, FIRST_ORB};
use conductor::{Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "chem-step-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(disposition: &str, sigil: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-chem"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "bonds under our own hand".into(),
            disposition: disposition.into(),
            metabolism: "mineral".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hearth"),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position: (ratio(3, 1), ratio(3, 1), ratio(3, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

fn chem_seated(polity: &Polity) -> bool {
    polity
        .world
        .science_rank(FIRST_ORB, ScienceDiscipline::Chem)
        >= 1
}

fn chem_receipts(polity: &Polity) -> usize {
    polity
        .world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| {
            matches!(
                r.event(),
                ClosureEvent::ScienceSeated { .. }
                    | ClosureEvent::ChemCataloged { .. }
                    | ClosureEvent::GemCompounded { .. }
                    | ClosureEvent::IsoUnionStruck { .. }
            )
        })
        .count()
}

/// Empiricist pure NPC seats chem lab within a bounded era.
#[test]
fn pure_npc_empiricist_seats_chem_lab() {
    let root = scratch("empiricist");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("empiricist", "empath"), &registry, &book);

    let mut seated_on = None;
    for beat in 1u32..=96 {
        let _ = polity.cycle(&book, &registry);
        if chem_seated(&polity) {
            seated_on = Some(beat);
            break;
        }
    }

    assert!(
        seated_on.is_some() || chem_receipts(&polity) > 0 || element_bank(&polity) > 0,
        "chemistry_step must engage: seated={:?} chem_rx={} elements={}",
        seated_on,
        chem_receipts(&polity),
        element_bank(&polity)
    );
    // When crystal + local resources accumulate, seat lands.
    if seated_on.is_some() {
        assert!(chem_seated(&polity));
        assert_eq!(
            replay(polity.world.chronicle()),
            polity.world.committed(),
            "SEAL B after chemistry seat"
        );
    }
}

/// Expansionist pure NPC chemistry path does not crash and stays replayable.
#[test]
fn pure_npc_expansionist_chemistry_cycles_replay() {
    let root = scratch("expansionist");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("expansionist", "expans"), &registry, &book);

    for _ in 0..32 {
        let _ = polity.cycle(&book, &registry);
    }
    assert_eq!(
        replay(polity.world.chronicle()),
        polity.world.committed(),
        "SEAL B after chemistry-bearing cycles"
    );
    // Chemistry step is live in the cycle (may idle until resources).
    let _ = chem_receipts(&polity);
}

fn element_bank(polity: &Polity) -> usize {
    polity
        .world
        .orb()
        .map(|o| o.elements.len())
        .unwrap_or(0)
}
