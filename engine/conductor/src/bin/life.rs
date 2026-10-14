//! docs/82 step 1: what the universe's worlds offer life. `life <era1 var>`.
use conductor::{life, pulse};
use cfe::ratio::{ratio, Ratio};
use std::path::Path;

fn main() {
    let era1 = std::env::args().nth(1).unwrap_or_else(|| "var-e1".into());
    let Some((_, u)) = pulse::read(Path::new(&era1)) else {
        println!("no pulse at {era1}/pulse.bin");
        return;
    };
    let worlds = life::worlds(&u);
    let mut pressures: Vec<Ratio> = worlds.iter().map(|w| w.pressure.clone()).collect();
    pressures.sort();
    let reference = pressures.get(pressures.len() / 2).cloned().filter(|p| *p > ratio(0, 1)).unwrap_or_else(|| ratio(1, 1));
    let band = |t: &Ratio| if *t < ratio(250, 1) { 0 } else if *t < ratio(600, 1) { 1 } else if *t < ratio(2273, 1) { 2 } else { 3 };
    let names = ["cryogenic <250 K", "mesic 250-600 K", "hot 600-2273 K", "refractory >2273 K"];
    let mut by_band = [[0usize; 5]; 4];
    let mut airless = 0;
    let mut any = 0;
    for w in &worlds {
        let c = w.pressure.clone() / reference.clone();
        if w.pressure == ratio(0, 1) {
            airless += 1;
        }
        let carries = |x: &Option<Ratio>| x.as_ref().is_some_and(|x| life::halvings(x, &c) >= ratio(1, 1));
        let b = band(&w.heat);
        by_band[b][0] += 1;
        let media = [carries(&w.chemical), carries(&w.geologic), carries(&w.plasmic), carries(&w.informational)];
        for (i, m) in media.iter().enumerate() {
            if *m {
                by_band[b][i + 1] += 1;
            }
        }
        if media.iter().any(|m| *m) {
            any += 1;
        }
    }
    let mut kept_share: Vec<Ratio> = worlds.iter().filter(|w| !w.gases.is_empty()).map(|w| {
        let held = w.gases.iter().fold(ratio(0, 1), |s, (a, g)| s + g.clone() * ratio(i64::from(*a), 1));
        w.atmosphere.clone() / held
    }).collect();
    kept_share.sort();
    let grav: Vec<Ratio> = { let mut g: Vec<Ratio> = worlds.iter().map(|w| w.gravity.clone()).collect(); g.sort(); g };
    let light: Vec<usize> = worlds.iter().map(|w| w.gases.iter().filter(|(a, _)| *a <= 4).count()).collect();
    println!("GRAVITY — least {}, median {}, most {} (hundredths); worlds holding a light gas (A ≤ 4): {}", grav.first().map(|g| (g.clone() * ratio(100, 1)).round()).unwrap_or_default(), grav.get(grav.len() / 2).map(|g| (g.clone() * ratio(100, 1)).round()).unwrap_or_default(), grav.last().map(|g| (g.clone() * ratio(100, 1)).round()).unwrap_or_default(), light.iter().filter(|&&n| n > 0).count());
    let cores: Vec<Ratio> = { let mut c: Vec<Ratio> = worlds.iter().map(|w| w.core.clone()).collect(); c.sort(); c };
    println!("WORLDS — {} read; {airless} airless (no gas kept); {any} carry a pattern in at least one medium", worlds.len());
    println!("ATMOSPHERE — gas kept (share of what the world holds as gas): least {}%, median {}%, most {}%; core share median {}%", kept_share.first().map(|k| (k.clone() * ratio(100, 1)).round()).unwrap_or_default(), kept_share.get(kept_share.len() / 2).map(|k| (k.clone() * ratio(100, 1)).round()).unwrap_or_default(), kept_share.last().map(|k| (k.clone() * ratio(100, 1)).round()).unwrap_or_default(), cores.get(cores.len() / 2).map(|c| (c.clone() * ratio(100, 1)).round()).unwrap_or_default());
    for (b, row) in by_band.iter().enumerate() {
        println!("  {:20} {:5} worlds: chemical {:5}, geologic {:5}, plasmic {:5}, informational {:5}", names[b], row[0], row[1], row[2], row[3], row[4]);
    }
    let heats: Vec<Ratio> = { let mut h: Vec<Ratio> = worlds.iter().map(|w| w.heat.clone()).collect(); h.sort(); h };
    println!("HEAT — coldest {} K, median {} K, hottest {} K; pressure reference (the median world) {}", heats.first().map(|h| h.round()).unwrap_or_default(), heats.get(heats.len() / 2).map(|h| h.round()).unwrap_or_default(), heats.last().map(|h| h.round()).unwrap_or_default(), reference.round());
}
