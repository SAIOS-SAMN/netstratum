//! How far each G2 line's founding home stands from its core, by the torus's
//! L1 distance, in cells: the reading the bulge is derived against.
use ledger::registry::Registry;
use ledger::Ledger;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let registry = Registry::at(format!("{root}/registry"));
    let book = Ledger::at(format!("{root}/worlds"));
    let span = book.universe_span();
    let galaxies = book.galaxies();
    let mut rows = Vec::new();
    for spec in conductor::roster(&registry) {
        if book.world_dir(&spec.world_name).join("collapsed").exists() {
            continue;
        }
        let Some(core) = book.core_of(&spec.position) else { continue };
        let Some(g) = galaxies.iter().find(|g| g.name == core) else { continue };
        let d = ledger::torus_distance(&spec.position, &g.center, &span);
        rows.push((d, core, spec.world_name.clone()));
    }
    rows.sort();
    for (d, core, name) in &rows {
        println!("{:>10} {core:28} {name}", format!("{:.3}", num_traits::ToPrimitive::to_f64(d).unwrap_or(-1.0)));
    }
}
