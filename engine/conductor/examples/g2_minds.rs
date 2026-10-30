//! Concordance 3.8 reading: every living G2 world's minds (its registered
//! entities, by role) against its non-minds (its population of mouths).
use conductor::snapshot;
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(Path::new(&root).join("worlds")).unwrap().flatten().map(|e| e.path())
        .filter(|p| p.join("chronicle.tlv").exists() && !p.to_string_lossy().contains("--closed-")).collect();
    dirs.sort();
    let (mut worlds, mut population, mut collapsed) = (0u64, 0u64, 0u64);
    let mut roles: BTreeMap<String, u64> = BTreeMap::new();
    for d in dirs {
        let Some((w, _, _)) = snapshot::boot(&d) else { continue };
        if w.committed().collapsed.is_some() { collapsed += 1; continue; }
        worlds += 1;
        let pop: u64 = w.committed().orbs.iter().map(|o| w.population_of(o.orb_id)).sum();
        population += pop;
        for e in &w.committed().entities {
            let role = if e.handle.ends_with("-conductor") { "automaton (thinking vessel)".to_string() }
                else if e.handle == "sibling" { "sibling (thinking vessel)".to_string() }
                else { e.handle.clone() };
            *roles.entry(role).or_default() += 1;
        }
    }
    println!("living worlds {worlds} (collapsed {collapsed})");
    println!("population (non-minds, mouths): {population}");
    for (role, n) in &roles { println!("entity {role}: {n}"); }
}
