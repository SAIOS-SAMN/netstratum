//! N-I5 Pin G — the survival observation gates the beat, purely and replay-exact.
//!
//! The reduce-organ reading of the ration book (Fed/Short) is witnessed at the
//! seat (`solver/tests/survival_pins.rs`). Here the wire: `ration_survives` is a
//! PURE read (same state → same verdict, or replay would diverge), and a line
//! whose beats are gated on the ration book still replays byte-exact — the gate
//! adds no committed state. Fed lines still research and mint (mint_loop_pins);
//! this pins that the gate itself is SEAL-B-clean.

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::replay;
use conductor::{Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("survival-gate-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str, metabolism: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-keeper"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "the mouths eat before the mind thinks".into(),
            disposition: "empiricist".into(),
            metabolism: metabolism.into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hearth"),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position: (ratio(5, 1), ratio(5, 1), ratio(5, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

#[test]
fn pin_g_the_survival_gate_is_pure_and_replay_exact() {
    let root = scratch("pure");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("hearth", "mineral"), &registry, &book);
    // Drive the line far enough to seat mouths and move its larder.
    for _ in 0..24 {
        let _ = polity.cycle(&book, &registry);
    }
    let orb = cfe::world::FIRST_ORB;

    // THE OBSERVATION IS PURE: the same committed state reads the same verdict.
    // A non-deterministic gate would fork the beat and shatter replay.
    let first = polity.ration_survives(orb);
    let again = polity.ration_survives(orb);
    assert_eq!(first, again, "the survival observation is deterministic");

    // AND IT ADDS NO COMMITTED STATE: a line whose every beat was gated on the
    // ration book replays byte-exact — Pin G is off the SEAL-B surface.
    assert_eq!(
        replay(polity.world.chronicle()),
        polity.world.committed(),
        "a ration-gated line replays MATCHES (SEAL B held across the gate)",
    );
}

#[test]
fn pin_g_a_species_reads_its_own_ration_column() {
    // The gate reads the LINE's ration, not a hardcoded Biomass — a mineral line
    // and a biotic line each survive their own column. Both drive without panic
    // and replay exact; the reading is species-derived, never assumed.
    let root = scratch("column");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    for metabolism in ["mineral", "biotic"] {
        let mut polity = Polity::genesis(line(metabolism, metabolism), &registry, &book);
        for _ in 0..12 {
            let _ = polity.cycle(&book, &registry);
        }
        let _ = polity.ration_survives(cfe::world::FIRST_ORB); // reads its own column, no panic
        assert_eq!(
            replay(polity.world.chronicle()),
            polity.world.committed(),
            "{metabolism} line replays MATCHES under the ration gate",
        );
    }
}
