//! Register two civilizations and export two settled worlds — the
//! observer/steward smoke fixture at the full topology.
//! Usage: cargo run -p scribe --example scribe_demo -- <var-root>
//! (writes <var-root>/registry and <var-root>/worlds)

use cfe::dop::ZoneKind;
use cfe::world::World;
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;

fn main() {
    let var = std::env::args().nth(1).expect("usage: ledger_demo <var-root>");
    let registry = Registry::at(format!("{var}/registry"));
    let book = Ledger::at(format!("{var}/worlds"));

    let _ = registry.create(&Civilization {
        name: "Bimaniform".into(),
        sigil: "bimaniform-line".into(),
        species: "bimaniform".into(),
        clade: String::new(),
        motto: "we go anyway".into(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
        founder: "operator".into(),
        lineage: Vec::new(),
        station: None,
    });
    let _ = registry.create(&Civilization {
        name: "Deep Chorus".into(),
        sigil: "deep-chorus".into(),
        species: "soniform-depth".into(),
        clade: String::new(),
        motto: "we heard it before you saw it".into(),
        disposition: "epicurean".into(),
        metabolism: "biotic".into(),
        perception: "harmonic".into(),
        founder: "operator".into(),
        lineage: Vec::new(),
        station: None,
    });

    // The Bimaniform settle cinder, in the first-light world: the aggressive opening.
    let mut cinder = World::new();
    cinder.found("first-landing").unwrap();
    cinder.zone(0, ZoneKind::Extractor).unwrap();
    cinder.zone(1, ZoneKind::Extractor).unwrap();
    cinder.zone(8, ZoneKind::Habitat).unwrap();
    for _ in 0..6 {
        cinder.tick();
    }
    cinder.collect().unwrap();
    scribe::record(&book, "first-light", "cinder", "bimaniform-line", &cinder).unwrap();
    scribe::record_solve(&book, "first-light", "arc_6150a2bd", "rotate180", "deadbeef", 1).unwrap();

    // Deep Chorus settles tidefall, in the lantern-passage world:
    // safe and slow, an agent on a leash.
    let mut tidefall = World::new();
    tidefall.found("tidefall-reach").unwrap();
    tidefall.zone(0, ZoneKind::Extractor).unwrap();
    tidefall.zone(8, ZoneKind::Habitat).unwrap();
    let rover = tidefall.register_agent("rover").unwrap();
    tidefall.grant(rover, cfe::ipsal::Scope::Collect).unwrap();
    for _ in 0..3 {
        tidefall.tick();
    }
    tidefall.collect_as(rover, cfe::world::FIRST_ORB).unwrap();
    scribe::record(&book, "lantern-passage", "tidefall", "deep-chorus", &tidefall).unwrap();

    println!("registered 2 civilizations, exported 2 worlds under {var}");
}
