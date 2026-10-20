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
    // Bell by bell, as the conductor beats (the galaxies re-read their
    // orbits on each universal echo).
    // From a CHECKPOINT (TUniversal 11.0 (23)): a universe written at a beat
    // resumes there (`conductor.beat`), never from the founding.
    let start: u64 = std::fs::read_to_string(Path::new(era1).join("conductor.beat")).ok().and_then(|t| t.trim().parse().ok()).unwrap_or(0).min(until);
    if start > 0 {
        println!("RESUMED at bell {start} (the checkpoint)");
    }
    let mut events = Vec::new();
    for b in start..until {
        events.extend(breath::breathe(&mut u, &seed, t_u, b, b + 1));
    }
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
        let rungs = breath::rungs(&breath::slip(&g_rate, until, t_u)).map(|r| r.to_string()).unwrap_or_else(|| "unread".into());
        let coh = breath::coherence(&breath::slip(&g_rate, until, t_u));
        let year = motion::galactic_year(c).map(|(k, t)| format!("half-mass system {}, year {} bells", k + 1, t.period.round())).unwrap_or_else(|| "no year yet".into());
        let medium = pulse::mass(&c.medium) / unit.clone();
        println!("  galaxy {:?} at {} bells out: {echoes} echoes, {rungs} rungs (recoherences), coherence now {}%, aeon {} bells (rate {}); {born} systems, {orbs} orbs; medium left {}; {year}", c.path, c.radius, (coh * cfe::ratio::ratio(100, 1)).round(), aeon.round(), g_rate, medium.round());
    }
    // THE CLOCKS (docs/78): which duty comes due how often, by layer.
    let terms: Vec<Option<u64>> = (0..u.cores.len()).map(|i| conductor::clocks::galaxy_terms(&u, i, t_u, 0, until)).collect();
    let epochs: Vec<Option<u64>> = (0..u.cores.len()).map(|i| conductor::clocks::galaxy_epochs(&u, i, &unit, t_u, 0, until)).collect();
    let worlds: Vec<u64> = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).filter_map(|o| conductor::clocks::local_years(&o.period, 0, until)).collect();
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
    let count = |m: &pulse::Manifest, k: Option<conductor::chem::Kind>| k.and_then(|k| m.get(&k).cloned()).unwrap_or_else(|| cfe::ratio::ratio(0, 1));
    let whole = |m: &pulse::Manifest| m.values().fold(cfe::ratio::ratio(0, 1), |s, a| s + a.clone());
    let share = |m: &pulse::Manifest, z: u32| conductor::reading::share(&count(m, conductor::chem::Kind::atom(z)), &whole(m));
    let commonest = all.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _)| k.to_string()).unwrap_or_else(|| "none".into());
    println!("AGE — by nuclei, H {} -> {}, He {} -> {} (founding -> now); commonest now {commonest}", share(&before, 1), share(&all, 1), share(&before, 2), share(&all, 2));
    // THE PARITY (with xylarium): the odd share of the Z >= 3 atoms by
    // nuclei, clusters counted by their parts (equal-halves recycling sorts
    // the heavy matter by parity).
    let odd = |m: &pulse::Manifest| {
        let (mut o, mut t) = (cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1));
        for (k, a) in m {
            for &(n, z) in conductor::chem::components(k) {
                if z >= 3 {
                    let x = a.clone() * cfe::ratio::ratio(i64::from(n), 1);
                    t += x.clone();
                    if z % 2 == 1 {
                        o += x;
                    }
                }
            }
        }
        conductor::reading::share(&o, &t)
    };
    println!("PARITY — odd share of Z >= 3 by nuclei: {} -> {} (founding -> now)", odd(&before), odd(&all));
    // RECYCLING at the galaxies' throats (`conductor::recycle`, 11.0 (18)):
    // pressed (constructed) and peeled (deconstructed), the light and the
    // opened holding, and what was trapped at the throats.
    let sum = |f: &dyn Fn(&breath::Event) -> cfe::ratio::Ratio| events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + f(e));
    let f = conductor::reading::sector;
    println!("RECYCLING — the throats pressed {} and peeled {} (grains of nuclei); light given {}, holding opened {}, trapped {} (units); {} echoes recycled", f(&sum(&|e| e.recycled.pressed.clone())), f(&sum(&|e| e.recycled.peeled.clone())), f(&sum(&|e| e.recycled.light.clone())), f(&sum(&|e| e.recycled.spent.clone())), f(&sum(&|e| e.recycled.trapped.clone())), events.iter().filter(|e| e.recycled != Default::default()).count());
    // THE BURN AND THE RETURN AS RATES (TUniversal 11.0 (23)): hydrogen fused
    // by the stars and returned net by the throats, per universal echo, as
    // shares of the hydrogen standing now, over the run's windows (tenths).
    // Where the two meet is the hydrogen's steady state, a fixed point; a
    // read this early is the first instant, never a judgement.
    {
        let h_now = all.get(&conductor::chem::hydrogen()).cloned().unwrap_or_else(|| cfe::ratio::ratio(0, 1));
        let echoes = ((until - start) / t_u.max(1)).max(1);
        let window = (echoes / 10).max(1);
        let mut rows: Vec<String> = Vec::new();
        let mut w = 0;
        while w * window < echoes {
            let (lo, hi) = (start + w * window * t_u, (start + (w + 1) * window * t_u).min(until));
            let sel = events.iter().filter(|e| e.bell > lo && e.bell <= hi);
            let (mut b, mut r) = (cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1));
            for e in sel {
                b += e.burned.get(&conductor::chem::hydrogen()).cloned().unwrap_or_else(|| cfe::ratio::ratio(0, 1));
                r += e.recycled.hydrogen.clone();
            }
            let per = |x: cfe::ratio::Ratio| conductor::reading::share(&x, &(h_now.clone() * cfe::ratio::ratio(window as i64, 1)));
            rows.push(format!("u {}-{}: burned {} returned {}", start / t_u.max(1) + w * window, start / t_u.max(1) + (w + 1) * window, per(b), per(r)));
            w += 1;
        }
        println!("RATES — hydrogen per universal echo, as a share of the hydrogen now: {}", rows.join("; "));
    }
    if args.get(4).map(String::as_str) == Some("write") {
        let _ = pulse::write(Path::new(era1), &seed, &u);
        let _ = std::fs::write(Path::new(era1).join("conductor.beat"), until.to_string());
    }
}
