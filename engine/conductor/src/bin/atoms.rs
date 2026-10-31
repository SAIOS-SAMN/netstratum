//! docs/78 "The atom": build Z = 1 to 118 by the law and read whether a
//! periodic chemistry emerges (the unpaired counts recurring from depth to
//! depth).
use cfe::almanac::atom::atom;
use std::collections::BTreeMap;

fn main() {
    // `atoms tau Z…`: each kind's depth, enclosed charge and tightness.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("tau") {
        for z in args[1..].iter().filter_map(|a| a.parse::<u32>().ok()) {
            if let Some(a) = atom(z) {
                println!(
                    "Z{z}: depth {} outer {} enclosed {} unpaired {} tightness {}",
                    a.depth, a.outer, a.enclosed, a.unpaired, a.tightness
                );
            }
        }
        return;
    }
    let atoms: Vec<_> = (1..=118).filter_map(atom).collect();
    let closed: Vec<u32> = atoms.iter().filter(|a| a.closed).map(|a| a.z).collect();
    println!("CLOSED (inert) at Z = {closed:?}");
    let mut by_depth: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for a in &atoms {
        by_depth.entry(a.depth).or_default().push(a.unpaired);
    }
    for (d, u) in &by_depth {
        let first = atoms
            .iter()
            .find(|a| a.depth == *d)
            .map(|a| a.z)
            .unwrap_or(0);
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
    let tight: Vec<String> = atoms
        .iter()
        .take(30)
        .map(|a| format!("{}:{}", a.z, a.tightness))
        .collect();
    println!("tightness (Z:Q/P), first 30: {}", tight.join(" "));
    // Compounds of two and three atoms among Z 1..=30 (cfe::almanac::atom::closes).
    let open: Vec<u32> = (1..=30)
        .filter(|&z| atom(z).is_some_and(|a| !a.closed))
        .collect();
    let pairs: Vec<(u32, u32)> = open
        .iter()
        .flat_map(|&a| open.iter().filter(move |&&b| b >= a).map(move |&b| (a, b)))
        .filter(|&(a, b)| cfe::almanac::atom::closes(&[a, b]))
        .collect();
    let mut triples = 0;
    let mut bonding: BTreeMap<u32, usize> = BTreeMap::new();
    for (i, &a) in open.iter().enumerate() {
        for (j, &b) in open.iter().enumerate().skip(i) {
            for &c in open.iter().skip(j) {
                if cfe::almanac::atom::closes(&[a, b, c]) {
                    triples += 1;
                    for z in [a, b, c] {
                        *bonding.entry(z).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    println!(
        "COMPOUNDS among the {} open atoms of Z 1..30: {} pairs, {triples} triples",
        open.len(),
        pairs.len()
    );
    let few: Vec<String> = pairs
        .iter()
        .take(12)
        .map(|(a, b)| format!("{a}+{b}"))
        .collect();
    println!("  first pairs: {}", few.join(" "));
    let busiest: Vec<String> = {
        let mut v: Vec<_> = bonding.into_iter().collect();
        v.sort_by(|x, y| y.1.cmp(&x.1));
        v.into_iter()
            .take(8)
            .map(|(z, n)| format!("Z{z}:{n}"))
            .collect()
    };
    println!("  atoms in the most triples: {}", busiest.join(" "));
}
