//! What each G2 line holds for the hubs: STAMP and the three cargoes, read
//! from the carried snapshots, one world at a time.
use cfe::dop::Resource;
use conductor::snapshot;
use std::path::Path;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(Path::new(&root).join("worlds"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("state.snap").exists() && !p.to_string_lossy().contains("--closed-"))
        .collect();
    dirs.sort();
    let (mut with_stamp, mut with_cargo, mut n) = (0, 0, 0);
    for d in dirs {
        let Some((w, _, _)) = snapshot::boot(&d) else {
            continue;
        };
        let Some(orb) = w.orb() else { continue };
        n += 1;
        let stamp = orb.stamp_amount();
        let [c, i, b] =
            [Resource::Crystal, Resource::Iron, Resource::Biomass].map(|r| orb.banked_amount(r));
        let zero = cfe::ratio::ratio(0, 1);
        with_stamp += usize::from(stamp > zero);
        with_cargo += usize::from(c > zero || i > zero || b > zero);
        let f = |x: &cfe::ratio::Ratio| x.floor().to_integer().to_string();
        println!(
            "{:24} stamp {:>12} crystal {:>12} iron {:>12} biomass {:>12} ration {:?}",
            d.file_name().unwrap().to_string_lossy(),
            f(&stamp),
            f(&c),
            f(&i),
            f(&b),
            w.ration_of(cfe::world::FIRST_ORB)
        );
    }
    println!("{n} lines: {with_stamp} hold STAMP, {with_cargo} hold cargo");
}
