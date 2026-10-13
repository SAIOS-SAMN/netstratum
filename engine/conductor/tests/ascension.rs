//! docs/72 — ASCENSION, driven through the universe's own loop on a real galaxy.
//!
//! The fixture is a copy of one live G2 galaxy, the core veinmarch-sky-13: its
//! ancient (vein-march, of forgeborn's lineage, which keeps the throat), a
//! master of that lineage (ember-choir, at the forge's top rung, earned in
//! play), stela-reach (stelahold, also forgeborn's) and reef-choir. Nothing is
//! hand-set: levels resume from the lines' own solve cards, positions and
//! lineages from the registry, the ancients from the galaxy's own book. The
//! universe is stepped by `run_universe` (the same beat and the same
//! `year_bounce` the conductor runs) and every assertion reads what the engine
//! wrote.
//!
//! The one boundary: the court's seal on a map act is an external system. The
//! house writes `gates-mapped.bin` after a real MAP act seals (ns-hypha
//! `refound::map_gates`, tested against the court's own decoder in
//! engine/hypha/tests/refound.rs); here that file is written as the house would.
//!
//! Ignored by default: it copies ~200 MB of live G2 state. Run with
//! `cargo test --release -p conductor --test ascension -- --ignored`.

use conductor::{ascension, run_universe, Polity};
use std::path::{Path, PathBuf};

const LIVE: &str = "/opt/netstratum/var-g2";
const CORE: &str = "veinmarch-sky-13";
const WORLDS: [&str; 4] = ["vein-march", "stela-reach", "ember-choir", "reef-choir"];

fn copy_dir(from: &Path, to: &Path, skip: &[&str]) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap().flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        if skip.contains(&name.as_str()) {
            continue;
        }
        let p = e.path();
        if p.is_dir() {
            copy_dir(&p, &to.join(&name), skip);
        } else {
            std::fs::copy(&p, to.join(&name)).unwrap();
        }
    }
}

/// The galaxy at veinmarch-sky-13, copied whole from the live universe.
fn fixture() -> PathBuf {
    let live = Path::new(LIVE);
    assert!(live.join("worlds").exists(), "this test needs the live G2 universe at {LIVE}");
    let base = std::env::var("TMPDIR").map(PathBuf::from).unwrap_or_else(|_| std::env::temp_dir());
    let var = base.join(format!("ascension-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&var);
    copy_dir(&live.join("registry"), &var.join("registry"), &[]);
    std::fs::create_dir_all(var.join("worlds")).unwrap();
    for f in ["convergence.tlv", "galaxies.card", "starchart.card", "universe.card"] {
        std::fs::copy(live.join("worlds").join(f), var.join("worlds").join(f)).unwrap();
    }
    for w in WORLDS {
        copy_dir(&live.join("worlds").join(w), &var.join("worlds").join(w), &["spine", "annals.dig"]);
    }
    for f in ["ancients.bin", "ancients", "trust", "trust.bin", "conductor.beat"] {
        if live.join(f).exists() {
            std::fs::copy(live.join(f), var.join(f)).unwrap();
        }
    }
    var
}

fn council(var: &Path) -> Vec<String> {
    let bytes = std::fs::read(var.join("council.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes).filter(|(t, _)| *t == 1).map(|(_, v)| String::from_utf8(v.to_vec()).unwrap()).collect()
}

#[test]
#[ignore = "copies a live G2 galaxy (~200 MB); run with --ignored"]
fn a_master_is_gated_crosses_a_mapped_gate_keeps_the_throat_and_ascends() {
    let var = fixture();
    let root = var.to_str().unwrap().to_string();
    let year = scribe::journal::YEAR_TICKS as u32;

    // A1: at a year's close the core's parliament folds a gate for its masters.
    let gated = |_: &[Polity]| ascension::read(&var).0.iter().any(|g| g.core == CORE);
    run_universe(&root, 2 * year, &gated);
    let (gates, envoys) = ascension::read(&var);
    let gate = gates.iter().find(|g| g.core == CORE).expect("the parliament folds a gate to its core within two years");
    assert_eq!(gate.builder, "forgeborn", "the lineage keeping the throat builds it");
    assert!(!gate.mapped, "a gate starts unmapped");
    assert!(envoys.is_empty(), "5.9: nothing crosses an unmapped gate");
    let gate_ends = std::fs::read(var.join("gates-map.bin")).expect("the gate is offered to the house to map");
    assert!(!gate_ends.is_empty());

    // Still unmapped after another year: still nothing crosses.
    run_universe(&root, year, &|_| false);
    assert!(ascension::read(&var).1.is_empty(), "5.9 holds a year on");

    // A4's seal, as the house writes it once the court seals the map act: the
    // gate's id and the place it was sealed at.
    let mut seal = Vec::new();
    seal.extend_from_slice(&(gate.id().len() as u16).to_le_bytes());
    seal.extend_from_slice(gate.id().as_bytes());
    for c in [&gate.place.0, &gate.place.1, &gate.place.2] {
        cfe::ratio::encode(c, &mut seal);
    }
    let mut sealed = Vec::new();
    cfe::tlv::put(2, &seal, &mut sealed);
    std::fs::write(var.join("gates-mapped.bin"), sealed).unwrap();

    // A2: at the next close the gate reads mapped and the masters' envoys cross.
    let crossed = |_: &[Polity]| ascension::read(&var).1.iter().any(|e| e.core == CORE);
    run_universe(&root, 2 * year, &crossed);
    let (gates, envoys) = ascension::read(&var);
    assert!(gates.iter().any(|g| g.core == CORE && g.mapped), "the sealed gate reads mapped");
    assert!(envoys.iter().any(|e| e.line == "emberchoir" && e.core == CORE), "the master of the keeping lineage crosses");

    // A3: keeping the throat a full core period, the envoy's line ascends: it
    // joins the core's ancients and the council lists its world.
    let ascended = |_: &[Polity]| conductor::ancients::arisen(&var).iter().any(|(c, s)| c == CORE && s == "emberchoir");
    run_universe(&root, 3 * year, &ascended);
    let arisen = conductor::ancients::arisen(&var);
    assert!(arisen.iter().any(|(c, s)| c == CORE && s == "emberchoir"), "ascension by deed within a core period: {arisen:?}");
    assert!(arisen.iter().any(|(c, s)| c == CORE && s == "veinmarch"), "the ancient by birth keeps its place: several ancients a core");
    assert!(council(&var).iter().any(|w| w == "ember-choir"), "the house lists the ascended line's council seats");
    // The ascended line's passage is over: it is no longer an envoy.
    assert!(!ascension::read(&var).1.iter().any(|e| e.line == "emberchoir"), "an ascended envoy leaves the passage");
    // UNPROVEN HERE: an envoy's withdrawal. Both masters of this galaxy
    // (emberchoir, stelahold) are forgeborn's, the keeping lineage, so no
    // envoy here can fail the judgement.

    let _ = std::fs::remove_dir_all(&var);
}
