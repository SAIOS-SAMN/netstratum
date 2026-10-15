//! docs/83: read the formation chain to stars on a deployed universe.
//! `formation <era1 var> [horizon rings]`.
use conductor::{formation, pulse};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let var = args.get(1).map(String::as_str).unwrap_or("var-e1");
    let horizon: u64 = args.get(2).and_then(|h| h.parse().ok()).unwrap_or(100_000);
    let Some((_, u)) = pulse::read(Path::new(var)) else {
        println!("no pulse at {var}/pulse.bin");
        return;
    };
    let f = |r: &cfe::ratio::Ratio| r.numer().to_string().parse::<f64>().unwrap_or(0.0) / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
    let universe = pulse::mass(&u.total());
    let (mut stars_all, mut first_all) = (0u64, u64::MAX);
    for core in &u.cores {
        let all = core.total();
        let share = pulse::mass(&all) / universe.clone();
        let (mut shells, h) = formation::shells(&all, &core.kept, &share);
        let jetted = formation::jets(&shells, &h, horizon);
        let jet_count = jetted.len();
        shells.extend(jetted);
        let gas: f64 = shells.iter().filter(|s| s.outer > s.inner && s.born == 0).map(|s| f(&s.held)).sum();
        let hot = shells.iter().filter(|s| s.halving.is_none()).count();
        let events = formation::first_collapse(&shells, &h, horizon);
        let stars: u64 = events.iter().map(|e| e.3).sum();
        let collapsed: f64 = events.iter().map(|(_, st, w, c)| f(&st.lambda) * f(w) * *c as f64).sum();
        let masses: Vec<f64> = events.iter().map(|(_, st, w, _)| f(&st.lambda) * f(w)).collect();
        let first = events.iter().map(|e| e.0).min();
        // Stars born where gas passes over the disc's rings carry its elements.
        let mixed: Vec<String> = events
            .iter()
            .filter(|(_, st, _, _)| st.members.iter().any(|(i, _)| shells[*i].born > 0 || shells[*i].outer == shells[*i].inner))
            .map(|(n, st, _, c)| format!("ring {n}: {}×[{}]", c, st.members.iter().map(|(i, _)| shells[*i].key.as_str()).collect::<Vec<_>>().join(",")))
            .collect();
        println!("  galaxy {:?}: {} jet batches landed; MIXED stars: {}", core.path, jet_count, if mixed.is_empty() { "none".to_string() } else { mixed.join("  ") });
        if let Some(n) = first {
            first_all = first_all.min(n);
        }
        stars_all += stars;
        println!("  galaxy {:?}: first echo at bell {}", core.path, 60 + f(&core.radius) as u64);
        println!(
            "galaxy {:?}: share {:.3}, disc {:.3}, gas {:.3} in {} shells ({} closed-shell, never cool); {} collapse events, first at ring {:?}, {} stars, star holding {:.2e}..{:.2e} of the galaxy, gas collapsed {:.3}",
            core.path, f(&share), f(&(pulse::mass(&core.kept) / pulse::mass(&all))), gas, shells.len(), hot, events.len(), first, stars,
            masses.iter().cloned().fold(f64::INFINITY, f64::min), masses.iter().cloned().fold(0.0, f64::max), collapsed
        );
    }
    println!("FORMATION — {} stars across {} galaxies; the first at ring {}", stars_all, u.cores.len(), if first_all == u64::MAX { "never".into() } else { first_all.to_string() });
}
