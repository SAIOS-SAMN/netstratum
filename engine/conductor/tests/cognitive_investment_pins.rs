//! **COG-8 — BUILD-TO-THINK.** A fed line with thinkers invests surplus in a
//! cognitive-science seat (Math/Physics/Research), lifting `cognitive_capacity`
//! 2→3 (COG-6's depth-3 frontier). A COSTED investment (the seat bill), chosen
//! stochastically by species affinity, replay-exact.

use cfe::almanac::ScienceDiscipline;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, FIRST_ORB};
use conductor::{Polity, Policy, PolitySpec};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cog8-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-academy"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "we plan in stone and fire".into(),
            disposition: "empiricist".into(),
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

fn cognitive_seated(polity: &Polity) -> Option<ScienceDiscipline> {
    use ScienceDiscipline::{Math, Physics, Research};
    [Math, Physics, Research]
        .into_iter()
        .find(|d| polity.world.science_rank(FIRST_ORB, *d) >= 1)
}

/// A developed line invests in a bigger brain and reaches the depth-3 frontier —
/// and whatever it does, the beat replays exactly.
#[test]
fn a_developed_line_builds_to_think() {
    let root = scratch("build-to-think");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("sage"), &registry, &book);

    let mut seated_on = None;
    for beat in 1u32..=400 {
        let _ = polity.cycle(&book, &registry);
        if cognitive_seated(&polity).is_some() {
            seated_on = Some(beat);
            break;
        }
    }

    let capacity = polity.world.cognitive_capacity(FIRST_ORB);
    let leisure = polity.world.leisure_bank_of(FIRST_ORB);
    let chem = polity.world.science_rank(FIRST_ORB, ScienceDiscipline::Chem);
    println!(
        "COG-8: seated={:?} discipline={:?} leisure={} chem_rank={} capacity={}",
        seated_on,
        cognitive_seated(&polity),
        leisure,
        chem,
        capacity
    );

    // A fed line with a leisure surplus invests in a bigger brain — build-to-think
    // FIRES, seats a cognitive science, and lifts the depth cap to the frontier.
    assert!(
        seated_on.is_some(),
        "build-to-think fires: a fed line with leisure seats a cognitive science"
    );
    assert_eq!(capacity, 3, "a cognitive-science seat lifts the depth cap 2→3");
    assert!(leisure > ratio(0, 1), "the drive is the leisure surplus it invests");
    // The build-to-think seat is a lawful, deterministic act — the beat replays.
    assert_eq!(
        replay(polity.world.chronicle()),
        polity.world.committed(),
        "SEAL B holds across the build-to-think era"
    );
}
