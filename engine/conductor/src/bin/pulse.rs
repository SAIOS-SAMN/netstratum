//! Found an era-1 universe: `pulse <era1 var> found <grains>`, the founding's
//! size in grains of nucleons, exact (a decimal, or `2^N`); written to the
//! var's pulse.bin. A var that already holds a universe is refused.
use conductor::pulse;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (Some(era1), Some("found"), Some(size)) =
        (args.get(1), args.get(2).map(String::as_str), args.get(3))
    else {
        println!("usage: pulse <era1 var> found <grains>");
        return;
    };
    let grains = match size.split_once('^') {
        Some((b, e)) => b
            .parse::<num_bigint::BigInt>()
            .ok()
            .zip(e.parse::<u32>().ok())
            .map(|(b, e)| b.pow(e)),
        None => size.parse::<num_bigint::BigInt>().ok(),
    };
    let Some(grains) = grains.filter(|g| *g > num_bigint::BigInt::from(0)) else {
        println!("refused: the founding's size is a positive whole number of grains");
        return;
    };
    if Path::new(era1).join("pulse.bin").exists() {
        println!("refused: {era1} already holds a universe");
        return;
    }
    // The founding distribution from the bounce's nucleons: the grains given,
    // as nuclei.
    let n = grains * pulse::nuclei_per_grain() * num_bigint::BigInt::from(1024);
    let manifest = pulse::founding(&n);
    println!("FOUNDING — {} nuclides as grains", manifest.len());
    // The isomorphic reading: the founding beside nature's (never imposed).
    if let Some((np, he, h)) = conductor::nuclides::beside_nature(&manifest) {
        let f = |x: &cfe::ratio::Ratio| conductor::reading::sector(x).to_string();
        println!("BESIDE NATURE — neutrons per proton {} (nature about 1/7), weight-4 mass share {} (about 1/4), hydrogen {} (about 3/4)", f(&np), f(&he), f(&h));
    }
    let seed = *blake3::hash(b"netstratum era 1, unfounded stand-in").as_bytes();
    let Some(u) = pulse::deploy(&manifest) else {
        println!("refused: the founding holds no disc, or its echo carries no ring");
        return;
    };
    let unit = conductor::breath::unit_mass(&u);
    println!(
        "DEPLOYED — {} cores; conservation {}; spin {}",
        u.cores.len(),
        u.total() == manifest,
        pulse::spin_total(&u, &unit)
    );
    match pulse::write(Path::new(era1), &seed, &u) {
        Ok(()) => println!("written {era1}/pulse.bin"),
        Err(e) => println!("not written: {e}"),
    }
}
