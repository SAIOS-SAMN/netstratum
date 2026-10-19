//! docs/78 "The breath": breathe the pulse universe and read it.
//! `breathe <era1 var> [bells] [_] [write]`: `write` saves the breathed (the third argument, once T_u, is unread: the period is derived at the founding)
//! universe back (scratch vars only), for `observe --var`.
use conductor::{breath, motion, pulse};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let era1 = args.get(1).map(String::as_str).unwrap_or("var-e1");
    let until: u64 = args.get(2).and_then(|b| b.parse().ok()).unwrap_or(36_000);
    let Some((seed, mut u)) = pulse::read(Path::new(era1)) else {
        println!("no pulse at {era1}/pulse.bin");
        return;
    };
    // The universal pulse: the bell's period derived at the founding
    // (`Universe::t_u`); a T_u argument is no longer read.
    let t_u: u64 = u.t_u;
    let before = u.total();
    let events = breath::breathe(&mut u, &seed, t_u, 0, until);
    let unit = breath::unit_mass(&u);
    let births: usize = events.iter().map(|e| e.stars).sum();
    let dissolved: usize = events.iter().map(|e| e.dissolved).sum();
    let returned = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.returned.clone());
    println!("STARS MET — {} merged, {} bounced", events.iter().map(|e| e.merged).sum::<usize>(), events.iter().map(|e| e.encounters).sum::<usize>());
    println!("BREATH — {} bells, T_u {t_u}: {} echoes (perpendicular writes down); recohered {births} systems born, decohered {} returned and {dissolved} stars dissolved (writes up)", until, events.len(), returned.round());
    for (i, c) in u.cores.iter().enumerate() {
        let echoes = events.iter().filter(|e| u64::from(e.galaxy) == c.path[0]).count();
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
    // THE CLOCKS (docs/78): which duty comes due how often, by layer.
    let terms: Vec<u64> = (0..u.cores.len()).map(|i| conductor::clocks::galaxy_terms(&u, i, t_u, 0, until)).collect();
    let epochs: Vec<u64> = (0..u.cores.len()).map(|i| conductor::clocks::galaxy_epochs(&u, i, &unit, t_u, 0, until)).collect();
    let worlds: Vec<u64> = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).map(|o| conductor::clocks::local_years(&o.period, 0, until)).collect();
    let journaled = worlds.iter().filter(|&&y| y > 0).count();
    let mut w = worlds.clone();
    w.sort();
    println!("CLOCKS — over {until} bells: galaxy terms (echoes received) {terms:?}; galaxy epochs (half-mass rungs) {epochs:?}; worlds' journals: {} worlds, {journaled} with a completed local year (most years {:?}, median {:?}); universal echoes {}", worlds.len(), w.last(), w.get(w.len() / 2), until / t_u);
    // Fusion transmutes kinds (H + H → He), so the manifest itself changes;
    // what the breath conserves is each nucleus's 3Z, holding plus
    // magnetism (transduction via magnetism).
    let three_z = |m: &pulse::Manifest| pulse::mass(m) + pulse::magnetism(m);
    println!("CONSERVATION — the breath created nothing (holding + magnetism, 3Z): {}; the manifest unchanged: {}", three_z(&u.total()) == three_z(&before), u.total() == before);
    // THE AGE (operator 2026-10-19: "universes age from their conception on
    // the TUniversal sexagesimal logged at their birth (distribution of
    // resources)"; hydrogen the commonest at the founding, a reading, then the
    // physics of recycling): hydrogen's and helium's shares by nuclei at the
    // founding and now, a reading of age, never a flag.
    let all = u.total();
    let count = |m: &pulse::Manifest, k: &str| m.get(k).cloned().unwrap_or_else(|| cfe::ratio::ratio(0, 1));
    let whole = |m: &pulse::Manifest| m.values().fold(cfe::ratio::ratio(0, 1), |s, a| s + a.clone());
    let share = |m: &pulse::Manifest, k: &str| { use num_traits::ToPrimitive; let w = whole(m); if w == cfe::ratio::ratio(0, 1) { 0.0 } else { (count(m, k) / w).to_f64().unwrap_or(0.0) } };
    let commonest = all.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _)| k.clone()).unwrap_or_default();
    println!("AGE — by nuclei, H {:.3} -> {:.3}, He {:.3} -> {:.3} (founding -> now); commonest now {commonest}", share(&before, "Z001"), share(&all, "Z001"), share(&before, "Z002"), share(&all, "Z002"));
    // RECYCLING at the galaxies' throats (`conductor::recycle`): what the
    // cores' pulse opened, what releasing splits gave back, the nuclei split.
    let spent = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.recycled.spent.clone());
    let released = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.recycled.released.clone());
    let nuclei = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.recycled.nuclei.clone());
    let f = |x: &cfe::ratio::Ratio| { use num_traits::ToPrimitive; x.to_f64().unwrap_or(0.0) };
    println!("RECYCLING — the throats split {:.3e} (grains of nuclei) over {} echoes with splits; pulse spent {:.3e}, released back {:.3e} (holding, units)", f(&nuclei), events.iter().filter(|e| e.recycled.nuclei > cfe::ratio::ratio(0, 1)).count(), f(&spent), f(&released));
    if args.get(4).map(String::as_str) == Some("write") {
        let _ = pulse::write(Path::new(era1), &seed, &u);
        let _ = std::fs::write(Path::new(era1).join("conductor.beat"), until.to_string());
    }
}
