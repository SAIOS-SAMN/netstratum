//! docs/78 step 3: read the pulse universe's motion. `motion <era1 var> [bells]`.
use conductor::{motion, pulse};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let era1 = args.get(1).map(String::as_str).unwrap_or("var-e1");
    let horizon: u64 = args.get(2).and_then(|b| b.parse().ok()).unwrap_or(1_000_000);
    let Some((_, u)) = pulse::read(Path::new(era1)) else {
        println!("no pulse at {era1}/pulse.bin");
        return;
    };
    let mut closes: Vec<u64> = Vec::new();
    for c in &u.cores {
        match motion::galactic_year(c) {
            Some((k, t)) => {
                let first = t.bell_of_turn(1);
                closes.push(first);
                println!("galaxy {:?}: year = the turn of system {} (half-mass), {} bells; first close at bell {first}; {} years by bell {horizon}", c.path, k + 1, t.period, t.turns(horizon));
            }
            None => println!("galaxy {:?}: no bound systems, no year", c.path),
        }
    }
    closes.sort();
    closes.dedup();
    println!("YEARS — {} galaxies with a year; their first closes fall at {} distinct bells", u.cores.iter().filter(|c| motion::galactic_year(c).is_some()).count(), closes.len());
    let orbs: Vec<motion::Turning> = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).filter_map(motion::turning).collect();
    let mut periods: Vec<cfe::ratio::Ratio> = orbs.iter().map(|t| t.period.clone()).collect();
    periods.sort();
    println!("LOCAL YEARS — {} orbs; shortest {} bells, median {} bells, longest {} bells", orbs.len(), periods.first().map(|p| p.to_string()).unwrap_or_default(), periods.get(periods.len() / 2).map(|p| p.to_string()).unwrap_or_default(), periods.last().map(|p| p.to_string()).unwrap_or_default());
    let years: Vec<motion::Turning> = u.cores.iter().filter_map(|c| motion::galactic_year(c).map(|(_, t)| t)).collect();
    let mut at: Vec<u64> = years.iter().flat_map(|t| (1..=t.turns(horizon.min(200_000))).map(move |k| t.bell_of_turn(k))).collect();
    let galaxy_closes = at.len();
    at.sort();
    at.dedup();
    let bells = at.len();
    println!("ECHOES — over {} bells, {galaxy_closes} galaxy years closed at {bells} distinct bells (the 60-beat close would be {})", horizon.min(200_000), horizon.min(200_000) / 60);
}
