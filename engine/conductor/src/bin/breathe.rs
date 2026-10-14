//! docs/78 "The breath": breathe the pulse universe and read it.
//! `breathe <era1 var> [bells] [T_u]`.
use conductor::{breath, motion, pulse};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let era1 = args.get(1).map(String::as_str).unwrap_or("var-e1");
    let until: u64 = args.get(2).and_then(|b| b.parse().ok()).unwrap_or(36_000);
    // The universal pulse: TUniversal sets it at founding; 60 bells stands in.
    let t_u: u64 = args.get(3).and_then(|b| b.parse().ok()).unwrap_or(60);
    let Some((seed, mut u)) = pulse::read(Path::new(era1)) else {
        println!("no pulse at {era1}/pulse.bin");
        return;
    };
    let before = u.total();
    let events = breath::breathe(&mut u, &seed, t_u, until);
    let unit = breath::unit_mass(&u);
    let births = events.iter().filter(|e| e.born.is_some()).count();
    let dissolved: usize = events.iter().map(|e| e.dissolved).sum();
    let returned = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.returned.clone());
    println!("BREATH — {} bells, T_u {t_u}: {} echoes (perpendicular writes down); recohered {births} systems born, decohered {} returned and {dissolved} stars dissolved (writes up)", until, events.len(), returned.round());
    for (i, c) in u.cores.iter().enumerate() {
        let echoes = events.iter().filter(|e| e.galaxy == c.path[0]).count();
        let born = c.satellites.len();
        let orbs: usize = c.satellites.iter().map(|s| s.satellites.len()).sum();
        let g_rate = breath::galaxy_rate(&u, i, &unit);
        let aeon = g_rate.clone() * cfe::ratio::ratio(until as i64, 1);
        let rungs = breath::rungs(&breath::slip(&g_rate, until, t_u));
        let coh = breath::coherence(&breath::slip(&g_rate, until, t_u));
        let year = motion::galactic_year(c).map(|(k, t)| format!("half-mass system {}, year {} bells", k + 1, t.period.round())).unwrap_or_else(|| "no year yet".into());
        let medium = pulse::mass(&c.medium) / unit.clone();
        println!("  galaxy {:?} at {} bells out: {echoes} echoes, {rungs} rungs (recoherences), coherence now {}%, aeon {} bells (rate {}); {born} systems, {orbs} orbs; medium left {}; {year}", c.path, c.radius, (coh * cfe::ratio::ratio(100, 1)).round(), aeon.round(), g_rate, medium.round());
    }
    println!("CONSERVATION — the breath created nothing: {}", u.total() == before);
}
