//! docs/78 "The atom": build Z = 1 to 118 by the law and read whether a
//! periodic chemistry emerges (the unpaired counts recurring from depth to
//! depth).
use cfe::almanac::atom::atom;
use std::collections::BTreeMap;

fn main() {
    let atoms: Vec<_> = (1..=118).filter_map(atom).collect();
    let closed: Vec<u32> = atoms.iter().filter(|a| a.closed).map(|a| a.z).collect();
    println!("CLOSED (inert) at Z = {closed:?}");
    let mut by_depth: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for a in &atoms {
        by_depth.entry(a.depth).or_default().push(a.unpaired);
    }
    for (d, u) in &by_depth {
        let first = atoms.iter().find(|a| a.depth == *d).map(|a| a.z).unwrap_or(0);
        println!("depth {d} (Z {first}..): {} atoms, unpaired {u:?}", u.len());
    }
    // Recurrence: each bonding count, the depths it appears in.
    let mut recur: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for a in &atoms {
        let e = recur.entry(a.unpaired).or_default();
        if e.last() != Some(&a.depth) {
            e.push(a.depth);
        }
    }
    for (u, depths) in &recur {
        println!("bonds {u}: recurs at depths {depths:?}");
    }
    let tight: Vec<String> = atoms.iter().take(30).map(|a| format!("{}:{}", a.z, a.tightness)).collect();
    println!("tightness (Z:Q/P), first 30: {}", tight.join(" "));
}
