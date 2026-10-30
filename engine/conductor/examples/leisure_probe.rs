//! Leisure probe — measure the exact-rational gap between per-beat leisure
//! income and the flat `observe_price` gate (N-I7), across every world on
//! disk. A read-only instrument: loads and resumes each chronicle exactly
//! as `Polity::genesis` does, computes the same quantities `cognition_step`/
//! `research_step` read live, and reports — appends nothing, writes nothing.
//!
//! ```text
//! cargo run --release -p conductor --example leisure_probe
//! ```

use cfe::ratio::ratio;
use cfe::world::FIRST_ORB;
use conductor::civic;

fn main() {
    let price = conductor::Polity::observe_price();
    println!("observe_price (flat, all civs) = {}/{}", price.numer(), price.denom());
    println!();

    let root = std::path::Path::new("var/worlds");
    let mut rows: Vec<(String, cfe::ratio::Ratio, cfe::ratio::Ratio, cfe::ratio::Ratio, u64)> =
        Vec::new();
    let mut no_orb = 0usize;
    let mut load_failed = 0usize;

    let Ok(entries) = std::fs::read_dir(root) else {
        println!("cannot read {}", root.display());
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let carrier = path.join("chronicle.tlv");
        if !carrier.exists() {
            continue;
        }
        let Ok(chronicle) = cfe::persist::load(&carrier) else {
            load_failed += 1;
            continue;
        };
        let world = cfe::world::World::resume(chronicle);
        if world.orb().is_none() {
            no_orb += 1;
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let bank = world.leisure_bank_of(FIRST_ORB);
        let cascade = civic::cascade_reading(&world, FIRST_ORB);
        let fill = civic::fasting_fill(cascade.leisure.clone(), world.syntropic_flow(FIRST_ORB));
        let decay = bank.clone() * world.leisure_decay_rate(FIRST_ORB);
        let net = fill - decay;
        let pop = world.population_of(FIRST_ORB);
        rows.push((name, bank, cascade.leisure, net, pop));
    }

    rows.sort_by(|a, b| a.0.cmp(&b.0));

    println!(
        "{:<24} {:>18} {:>18} {:>18} {:>8}",
        "world", "bank", "cascade.leisure", "net Δ/beat", "pop"
    );
    let mut negative_or_zero = 0usize;
    let mut cannot_afford_now = 0usize;
    let mut beats_list: Vec<f64> = Vec::new();
    for (name, bank, cascade_leisure, net, pop) in &rows {
        let afford_now = *bank >= price;
        if !afford_now {
            cannot_afford_now += 1;
        }
        let beats_str = if afford_now {
            "0 (affords now)".to_string()
        } else if *net <= ratio(0, 1) {
            negative_or_zero += 1;
            "NEVER (net <= 0)".to_string()
        } else {
            let remaining = price.clone() - bank.clone();
            // beats = remaining / net, evaluated in f64 for a human-readable estimate only
            use num_traits::ToPrimitive;
            let r_n = remaining.numer().to_f64().unwrap_or(f64::NAN)
                / remaining.denom().to_f64().unwrap_or(f64::NAN);
            let n_n = net.numer().to_f64().unwrap_or(f64::NAN)
                / net.denom().to_f64().unwrap_or(f64::NAN);
            let beats = r_n / n_n;
            beats_list.push(beats);
            format!("{beats:.1}")
        };
        println!(
            "{:<24} {:>7}/{:<10} {:>7}/{:<10} {:>7}/{:<10} {:>8} beats-to-afford={}",
            name,
            bank.numer(), bank.denom(),
            cascade_leisure.numer(), cascade_leisure.denom(),
            net.numer(), net.denom(),
            pop,
            beats_str,
        );
    }

    println!();
    println!("worlds with an orb: {}", rows.len());
    println!("worlds with no orb (pre-material, skipped): {no_orb}");
    println!("worlds that failed to load: {load_failed}");
    println!("cannot afford observe_price RIGHT NOW: {cannot_afford_now}/{}", rows.len());
    println!("of those, net per-beat delta <= 0 (will NEVER afford it without an external change): {negative_or_zero}");
    if !beats_list.is_empty() {
        beats_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = beats_list[beats_list.len() / 2];
        let max = beats_list[beats_list.len() - 1];
        println!(
            "of the rest ({} civs with net > 0), median beats-to-afford = {:.1}, worst = {:.1}",
            beats_list.len(), median, max
        );
    }
}
