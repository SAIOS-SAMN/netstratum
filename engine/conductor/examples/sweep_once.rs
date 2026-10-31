//! The year-voice observation sweep, fired once by the operator's hand
//! (13_GALAXY §5): resume every rostered world (the boot resumes
//! before it founds — the resume carrier), read the terminal clock,
//! sweep. No beats, no research, no state advanced but the sweep's
//! own receipts.

use conductor::{observe_crossings, roster, Polity};
use ledger::registry::Registry;
use ledger::Ledger;

fn main() {
    let var = std::env::args().nth(1).unwrap_or_else(|| "var".into());
    let registry = Registry::at(format!("{var}/registry"));
    let book = Ledger::at(format!("{var}/worlds"));
    let specs = roster(&registry);
    let worlds: Vec<Polity> = specs
        .into_iter()
        .map(|spec| Polity::genesis(spec, &registry, &book))
        .collect();
    let stamp = worlds
        .iter()
        .map(|held| held.world.now())
        .max()
        .unwrap_or(0);
    println!("{} worlds resumed at terminal clock {stamp}", worlds.len());
    let created = observe_crossings(&worlds, &registry, &book, stamp);
    if created.is_empty() {
        println!(
            "the sweep observed nothing new — every crossed anchor stands inside a charted reach"
        );
    }
    for galaxy in created {
        println!(
            "OBSERVATION — the galaxy {} enters the chart, reach {}",
            galaxy.name, galaxy.reach
        );
    }
    for galaxy in book.galaxies() {
        println!(
            "charted: {} at ({}, {}, {})",
            galaxy.name, galaxy.center.0, galaxy.center.1, galaxy.center.2
        );
    }
}
