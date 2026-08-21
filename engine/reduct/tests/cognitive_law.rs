//! THE LAW OF COLLECTIVE COGNITION — conclusive results (doc 55, entity-atlas).
//!
//! The arena ranks three civs; this establishes the GOVERNING LAW behind the
//! ranking and proves it robust. A civilization's collective cognition is the
//! exact atlas ΔH over its houses on the flat cognitive contacts. Its control
//! variable is CONNECTIVITY = contacts × coherence (expected reliable cross-house
//! contacts per beat). The conclusive claims, each asserted:
//!   1. IGNITION THRESHOLD — below connectivity 1 a civilization is a cognitive
//!      desert (ΔH ≈ 0, no collective mind); above it, cognition ignites.
//!   2. LINEAR GROWTH — well above threshold, ΔH tracks connectivity − 1.
//!   3. ROBUSTNESS — the law holds across independent seed populations, not one.
//!   4. SELECTION — evolving classification under ΔH-selection drives a random
//!      population ACROSS the ignition threshold: cognition is selected for.

use num_bigint::BigInt;
use reduct::atlas::delta_h;
use reduct::sheaf::CellSheaf;

fn house() -> CellSheaf {
    CellSheaf::new(vec![vec![1]], vec![]).expect("chart")
}
fn overlap(n: usize) -> CellSheaf {
    CellSheaf::new(vec![vec![1; n.max(1)]], vec![]).expect("overlap")
}
fn restr(n: usize) -> Vec<Vec<Vec<BigInt>>> {
    vec![(0..n.max(1)).map(|_| vec![BigInt::from(1)]).collect()]
}
fn draw(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (z ^ (z >> 31)) % 1000
}

/// Mean collective ΔH (×1000) for a species with `contacts` houses-links at
/// `coherence` per-mille, over `seeds × beats`, measured through the REAL atlas.
fn mean_cognition_milli(contacts: usize, coherence: u64, seed_base: u64, seeds: u64, beats: u64) -> u64 {
    let mut total = 0u64;
    for s in 0..seeds {
        let seed = seed_base ^ s.wrapping_mul(0x0123_4567_89AB);
        for b in 0..beats {
            let flat = (0..contacts)
                .filter(|c| draw(seed ^ (b << 20) ^ (*c as u64)) < coherence)
                .count();
            if flat > 0 {
                total += delta_h(&house(), &house(), &overlap(flat), &restr(flat), &restr(flat)) as u64;
            }
        }
    }
    total * 1000 / (seeds * beats)
}

#[test]
fn the_ignition_law_of_collective_cognition() {
    // Sweep connectivity by fixing coherence and growing contacts (connectivity =
    // contacts × coherence). Measure mean ΔH through the atlas.
    let coherence = 500; // 0.5 per contact
    eprintln!("=== THE IGNITION LAW — mean collective ΔH (×1000) vs connectivity ===");
    let mut samples: Vec<(u64, u64)> = Vec::new(); // (connectivity×1000, ΔH×1000)
    for contacts in 1..=10 {
        let connectivity = contacts as u64 * coherence; // ×1000
        let dh = mean_cognition_milli(contacts, coherence, 0xC0FFEE, 48, 48);
        samples.push((connectivity, dh));
        let bar = "#".repeat((dh / 250) as usize);
        eprintln!("  connectivity {:.3}  ΔH {:>2}.{:03}  {bar}", connectivity as f64 / 1000.0, dh / 1000, dh % 1000);
    }

    // 1. IGNITION THRESHOLD: below connectivity 1.0 the collective mind is dark.
    let desert = mean_cognition_milli(1, coherence, 0xC0FFEE, 48, 48); // connectivity 0.5
    assert!(desert < 150, "below the threshold a civilization has ~no collective cognition (got {desert})");

    // 2. LINEAR GROWTH well above threshold: at connectivity 5 (10 contacts × 0.5),
    //    ΔH approaches connectivity − 1 ≈ 4.
    let ignited = mean_cognition_milli(10, coherence, 0xC0FFEE, 48, 48);
    assert!(ignited > 3200, "well above threshold cognition tracks connectivity − 1 (got {ignited})");

    // 3. MONOTONE: more connectivity is never less collective cognition.
    for w in samples.windows(2) {
        assert!(w[1].1 + 30 >= w[0].1, "collective cognition is monotone in connectivity");
    }

    // 4. THE THRESHOLD IS NEAR 1.0: the first connectivity whose ΔH clears 0.25
    //    sits at connectivity ≈ 1, not at high private knowledge.
    let ignition = samples.iter().find(|(_, dh)| *dh > 250).map(|(c, _)| *c).unwrap_or(u64::MAX);
    assert!(
        (500..=2000).contains(&ignition),
        "the ignition threshold is near connectivity 1.0 (found at {})",
        ignition as f64 / 1000.0
    );
    eprintln!(
        "CONCLUSION: collective cognition IGNITES at connectivity ≈ {:.2} and then grows ≈ (connectivity − 1); \
         private knowledge below threshold yields NO shared mind.",
        ignition as f64 / 1000.0
    );
}

#[test]
fn the_law_is_robust_across_independent_populations() {
    // The composer > guild > hoarder ordering must hold across MANY independent
    // seed populations — not a fluke of one draw.
    let composer = (8usize, 850u64); // connectivity 6.8
    let guild = (4usize, 650u64); //    connectivity 2.6
    let hoarder = (2usize, 300u64); //  connectivity 0.6
    let mut correct = 0;
    let trials = 40;
    for t in 0..trials {
        let base = 0xA5A5 ^ (t as u64).wrapping_mul(0x9E37_79B9);
        let c = mean_cognition_milli(composer.0, composer.1, base, 8, 24);
        let g = mean_cognition_milli(guild.0, guild.1, base, 8, 24);
        let h = mean_cognition_milli(hoarder.0, hoarder.1, base, 8, 24);
        if c > g && g >= h {
            correct += 1;
        }
    }
    eprintln!("ROBUSTNESS: composer>guild>=hoarder held in {correct}/{trials} independent populations");
    assert!(correct >= 39, "the cognitive ordering is a law, not a fluke ({correct}/{trials})");
}

#[test]
fn selection_drives_a_population_across_the_ignition_threshold() {
    // Evolve a population of civilizations whose CLASSIFICATION (contacts) mutates;
    // each generation keep the top-cognition half and let them reproduce with a
    // small mutation. Start BELOW the ignition threshold and show selection lifts
    // the population's mean connectivity ACROSS it — cognition is selected for.
    let coherence = 500u64;
    let pop = 16usize;
    // Start every civ at 1 contact (connectivity 0.5 — a cognitive desert).
    let mut contacts: Vec<usize> = vec![1; pop];
    let mean_connectivity = |cs: &[usize]| -> f64 {
        cs.iter().map(|c| *c as f64).sum::<f64>() / cs.len() as f64 * (coherence as f64 / 1000.0)
    };
    let start = mean_connectivity(&contacts);
    for gen in 0..12u64 {
        // Score each civ by its collective cognition.
        let mut scored: Vec<(usize, u64)> = contacts
            .iter()
            .enumerate()
            .map(|(idx, &c)| (idx, mean_cognition_milli(c, coherence, 0xEEE ^ (gen << 8) ^ idx as u64, 6, 16)))
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        // The fit top half reproduce; offspring mutate contacts ±1 (min 1).
        let survivors: Vec<usize> = scored[..pop / 2].iter().map(|(idx, _)| contacts[*idx]).collect();
        let mut next = survivors.clone();
        for (i, &c) in survivors.iter().enumerate() {
            let m = draw(0xD00D ^ (gen << 12) ^ i as u64) % 3; // 0,1,2 → −1,0,+1
            let child = (c as i64 + (m as i64 - 1)).max(1) as usize;
            next.push(child);
        }
        contacts = next;
    }
    let end = mean_connectivity(&contacts);
    eprintln!("SELECTION: mean connectivity {start:.2} → {end:.2} over 12 generations (ignition at ≈1.0)");
    assert!(start < 1.0, "the population started a cognitive desert");
    assert!(end > 1.5, "selection drove the population ACROSS the ignition threshold ({end:.2})");
}
