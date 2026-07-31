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
        name: "Humans".into(),
        sigil: "humans".into(),
        species: "human".into(),
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
        species: "blue whale".into(),
        clade: String::new(),
        motto: "we heard it before you saw it".into(),
        disposition: "epicurean".into(),
        metabolism: "biotic".into(),
        perception: "harmonic".into(),
        founder: "operator".into(),
        lineage: Vec::new(),
        station: None,
    });

    // Humans settle mars, in the sol world: the aggressive opening.
    let mut mars = World::new();
    mars.found("first-landing").unwrap();
    mars.zone(0, ZoneKind::Extractor).unwrap();
    mars.zone(1, ZoneKind::Extractor).unwrap();
    mars.zone(8, ZoneKind::Habitat).unwrap();
    for _ in 0..6 {
        mars.tick();
    }
    mars.collect().unwrap();
    scribe::record(&book, "sol", "mars", "humans", &mars).unwrap();
    scribe::record_solve(&book, "sol", "arc_6150a2bd", "rotate180", "deadbeef", 1).unwrap();

    // Deep Chorus settles tidefall, in the kepler-passage world:
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
    scribe::record(&book, "kepler-passage", "tidefall", "deep-chorus", &tidefall).unwrap();

    println!("registered 2 civilizations, exported 2 worlds under {var}");
}
