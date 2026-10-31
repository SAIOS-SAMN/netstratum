//! Read each era-0 world's banked matter and STAMP from its snapshot (a
//! diagnostic for the hubs' unmet clearings).
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds"))
        .into_iter()
        .flatten()
        .filter_map(|d| d.ok())
        .map(|d| d.path())
        .collect();
    dirs.sort();
    for dir in dirs {
        let Some((_, _, state)) = conductor::snapshot::read_state(&dir) else {
            continue;
        };
        let Some(orb) = state.orbs.first() else {
            continue;
        };
        let b = |r| {
            orb.banked
                .iter()
                .find(|(x, _)| *x == r)
                .map(|(_, a)| a.to_string())
                .unwrap_or_else(|| "0".into())
        };
        use cfe::dop::Resource::*;
        println!(
            "{:24} stamp {:>10} crystal {:>10} iron {:>10} biomass {:>10} hulls {}",
            dir.file_name().unwrap().to_string_lossy(),
            orb.stamp.to_string(),
            b(Crystal),
            b(Iron),
            b(Biomass),
            state.hulls.len()
        );
    }
}
