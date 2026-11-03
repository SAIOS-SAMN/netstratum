//! #27 read: each standing world's composition as era 0 received it before
//! the fix (keyed by nucleon count, clusters dropped) and after (keyed by
//! proton count, clusters split by nucleons).
use cfe::ratio::{ratio, Ratio};
use conductor::{chem, nuclides, pulse, seating};
use std::path::Path;

fn main() {
    let dir = std::env::args().nth(1).expect("a universe directory");
    let (_, u) = pulse::read(Path::new(&dir)).expect("a universe");
    let mut worlds = 0;
    for core in &u.cores {
        for sys in &core.satellites {
            for orb in &sys.satellites {
                if orb.kept.is_empty() {
                    continue;
                }
                worlds += 1;
                let all = orb.kept.values().fold(ratio(0, 1), |s, a| s + a.clone());
                let mut old: Vec<(u32, Ratio)> = Vec::new();
                let mut dropped = ratio(0, 1);
                for (k, a) in &orb.kept {
                    match chem::atom_w(k) {
                        Some(w) => old.push((w, a.clone())),
                        None => dropped += a.clone(),
                    }
                }
                let new = seating::composition(&orb.kept);
                let pct = |a: &Ratio| a.clone() * ratio(100, 1) / all.clone();
                let top = |v: &mut Vec<(u32, Ratio)>| {
                    v.sort_by(|a, b| b.1.cmp(&a.1));
                    v.iter()
                        .take(4)
                        .map(|(z, a)| format!("{z}:{:.1}%", f(&pct(a))))
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                let mut o = old.clone();
                let mut n: Vec<(u32, Ratio)> = new
                    .iter()
                    .map(|(z, a)| (u32::from(*z), a.clone()))
                    .collect();
                let wrong = old
                    .iter()
                    .filter(|(w, _)| nuclides::protons(*w) != Some(*w))
                    .fold(ratio(0, 1), |s, (_, a)| s + a.clone());
                println!(
                    "world {:?}: {} kinds; before (key = nucleons) {} | after (key = Z) {}",
                    orb.path,
                    orb.kept.len(),
                    top(&mut o),
                    top(&mut n)
                );
                println!("   grains keyed to the wrong element before: {:.1}%; dropped as clusters before: {:.1}%; kept after: {:.1}%",
                    f(&pct(&wrong)), f(&pct(&dropped)), f(&pct(&n.iter().fold(ratio(0, 1), |s, (_, a)| s + a.clone()))));
            }
        }
    }
    println!("{worlds} worlds read");
}

fn f(x: &Ratio) -> f64 {
    use num_traits::ToPrimitive;
    x.to_f64().unwrap_or(f64::NAN)
}
