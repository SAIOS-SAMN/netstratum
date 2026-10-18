//! THE OBSERVER: read the live universe on demand, never write. The binary
//! run is the witness (operator); this is how it is read.
//!
//! `observe [--var var-e1] <view> [args]`, views:
//! - `universe`               the whole: galaxies, stars, worlds, gas, ledger
//! - `galaxy G`               one galaxy: its core, disc, gas, systems, clock
//! - `star G S`               one star: matter, orbit, heartbeat, coupling
//! - `world G S W`            one world: matter, orbit, what it offers life
//! - `gas G [bell]`           a galaxy's gas, stretch by stretch: why it
//!                            does or does not collapse
//! - `formation`              each galaxy's cut, gas, echo and stars
//! - `heartbeat`              every star's burn: fuel left, life, light
//! - `worlds`                 every world "held down to Z" (the planetary
//!                            cores' cut): how many worlds hold down to each
//! - `clock`                  the keepers, epicenter to rims; the time
//!                            inversion through the enclosures
//! - `ledger`                 mass, spin, magnetism, conserved sums
//! - `recycle [N]`            the recycling circuit (docs/83): the ledger,
//!                            the media tier by tier, what the last N
//!                            echo-beats shed, and each galaxy's return
//! - `civs`                   the civilizations: live and collapsed
//! - `events [N]`             the last N creation/collapse events (journal)
//! - `pace [N]`               beat pace and the top costs over the last N
//!                            beats; who holds the build lock (contention)
use cfe::ratio::{ratio, Ratio};
use conductor::{breath, chronometry, enclosure, formation, life, pulse};
use std::path::Path;
use std::process::Command;

fn f(r: &Ratio) -> f64 {
    r.numer().to_string().parse::<f64>().unwrap_or(0.0) / r.denom().to_string().parse::<f64>().unwrap_or(1.0)
}

/// `a / b`, 0 where `b` holds nothing (a galaxy whose matter all left).
fn of(a: &Ratio, b: &Ratio) -> f64 {
    if *b == ratio(0, 1) { 0.0 } else { f(&(a.clone() / b.clone())) }
}

/// The heaviest elements of a manifest by holding, as `Z:share`.
fn composition(m: &pulse::Manifest, top: usize) -> String {
    let total = pulse::mass(m);
    if total == ratio(0, 1) {
        return "—".into();
    }
    let mut parts: Vec<(String, f64)> = m.iter().map(|(k, a)| (k.clone(), f(&(pulse::mass(&[(k.clone(), a.clone())].into()) / total.clone())))).collect();
    parts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    parts.iter().take(top).map(|(k, s)| format!("{k}:{s:.3}")).collect::<Vec<_>>().join(" ")
}

fn beat(var: &str) -> u64 {
    std::fs::read_to_string(Path::new(var).join("conductor.beat")).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0)
}

fn journal(lines: usize) -> Vec<String> {
    let out = Command::new("journalctl").args(["--user", "-u", "netstratum-conductor", "--no-pager", "-o", "short-iso", "-n", &lines.to_string()]).output();
    out.map(|o| String::from_utf8_lossy(&o.stdout).lines().map(String::from).collect()).unwrap_or_default()
}

/// A system's nested clock: its galaxy's rate times its orbit's.
fn system_rate(u: &pulse::Universe, g: usize, s: usize, unit: &Ratio) -> Ratio {
    let core = &u.cores[g];
    let inside = core.satellites[..s].iter().fold(pulse::mass(&core.kept) + pulse::core_state(core), |a, sys| a + pulse::mass(&sys.total()));
    breath::galaxy_rate(u, g, unit) * conductor::ring::rate(&core.satellites[s].radius, &(inside / unit.clone()))
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut var = String::from("var-e1");
    if let Some(i) = args.iter().position(|a| a == "--var") {
        if i + 1 < args.len() {
            var = args.remove(i + 1);
        }
        args.remove(i);
    }
    let view = args.first().cloned().unwrap_or_else(|| "universe".into());
    let num = |i: usize| args.get(i).and_then(|a| a.parse::<u64>().ok());
    // Views read from the journal need no universe.
    match view.as_str() {
        "events" => return events(num(1).unwrap_or(40) as usize),
        "pace" => return pace(num(1).unwrap_or(30) as u64),
        "civs" => return civs(&var),
        _ => {}
    }
    let Some((seed, u)) = pulse::read(Path::new(&var)) else {
        println!("no universe at {var}/pulse.bin");
        return;
    };
    let now = beat(&var);
    let unit = breath::unit_mass(&u);
    let universe_hold = pulse::mass(&u.total());
    let galaxy = |g: u64| u.cores.iter().position(|c| c.path[0] == g);
    match view.as_str() {
        "universe" => {
            let stars: usize = u.cores.iter().map(|c| c.satellites.len()).sum();
            let worlds: usize = u.cores.iter().flat_map(|c| &c.satellites).map(|s| s.satellites.len()).sum();
            println!("UNIVERSE at beat {now}: {} galaxies, {stars} stars, {worlds} worlds; intergalactic medium {:.3} of the whole", u.cores.len(), f(&(pulse::mass(&u.medium) / universe_hold.clone())));
            for (i, c) in u.cores.iter().enumerate() {
                let total = pulse::mass(&c.total());
                println!(
                    "  galaxy {:>2}: share {:.3}, disc {:.3}, gas {:.3} ({} motes), {} stars; distance {} (first echo at bell {}); clock {:.6}",
                    c.path[0], f(&(total.clone() / universe_hold.clone())), of(&pulse::mass(&c.kept), &total), of(&pulse::mass(&c.medium), &total),
                    c.gas.len(), c.satellites.len(), c.radius, 60 + f(&c.radius) as u64, f(&breath::galaxy_rate(&u, i, &unit))
                );
            }
        }
        "galaxy" => {
            let Some(i) = num(1).and_then(galaxy) else { return println!("galaxy G: no such galaxy") };
            let c = &u.cores[i];
            let total = pulse::mass(&c.total());
            println!("GALAXY {:?} at beat {now}: share {:.4} of the universe; distance {} light-rings, first echo at bell {}", c.path, f(&(total.clone() / universe_hold.clone())), c.radius, 60 + f(&c.radius) as u64);
            println!("  core state hold {:.3e} (units of the deepest hole), disc {:.3e}: {}", f(&(pulse::core_state(c) / unit.clone())), f(&(pulse::mass(&c.kept) / unit.clone())), composition(&c.kept, 6));
            println!("  gas {:.3} of the galaxy in {} motes: {}; gas spin {:.3e}", of(&pulse::mass(&c.medium), &total), c.gas.len(), composition(&c.medium, 6), f(&c.medium_spin));
            println!("  clock against the universal core {:.6}", f(&breath::galaxy_rate(&u, i, &unit)));
            for (s, sys) in c.satellites.iter().enumerate() {
                println!("  star {:?}: holding {:.3e}, born bell {}, reach {:.3e}, {} worlds, clock {:.6}; {}", sys.path, f(&(pulse::mass(&sys.kept) / unit.clone())), sys.born, f(&sys.radius), sys.satellites.len(), f(&system_rate(&u, i, s, &unit)), composition(&sys.kept, 3));
            }
        }
        "star" => {
            let (Some(i), Some(s)) = (num(1).and_then(galaxy), num(2)) else { return println!("star G S") };
            let Some(k) = u.cores[i].satellites.iter().position(|x| x.path[1] == s) else { return println!("no such star") };
            let sys = &u.cores[i].satellites[k];
            let rate = system_rate(&u, i, k, &unit);
            let received: u64 = (rate.clone() * ratio(now.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0);
            println!("STAR {:?} at beat {now}: born bell {}, holding {:.4e} (units of the deepest hole), magnetism {:.3e}", sys.path, sys.born, f(&(pulse::mass(&sys.kept) / unit.clone())), f(&(pulse::magnetism(&sys.kept) / unit.clone())));
            println!("  matter: {}", composition(&sys.kept, 8));
            println!("  orbit: reach {:.4e}, period {:.4e} rings, spin ℓ {:.4e}; clock {:.6}, {received} rings received since birth", f(&sys.radius), f(&sys.period), f(&sys.ell), f(&rate));
            match formation::heartbeat(&sys.kept, &(pulse::mass(&sys.kept) / pulse::mass(&sys.total())), received) {
                Some((every, left, light)) => println!("  HEARTBEAT: fuel halves every {every} received rings; fuel left {:.4}; light {:.3e} per received ring", f(&left), f(&light)),
                None => println!("  HEARTBEAT: no hydrogen fuel"),
            }
            println!("  coupling κ {:.4e}", formation::coupling(&sys.kept, &universe_hold).map(|k| f(&k)).unwrap_or(0.0));
            // The dance: each world's period against the dominant world's.
            let dom = sys.satellites.iter().max_by(|a, b| pulse::mass(&a.kept).cmp(&pulse::mass(&b.kept))).map(|o| o.period.clone());
            for o in &sys.satellites {
                let lock = dom.as_ref().filter(|p| **p > ratio(0, 1)).map(|p| o.period.clone() / p.clone()).map(|x| if x.denom() < &num_bigint::BigInt::from(1000) { format!("{x}") } else { format!("{:.4} (free)", f(&x)) }).unwrap_or_default();
                println!("  world {:?}: reach {:.3e}, period {:.3e}, against the dominant {lock}; held down to {}; {}", o.path, f(&o.radius), f(&o.period), formation::lightest(&o.kept).unwrap_or_default(), composition(&o.kept, 4));
            }
        }
        "world" => {
            let path: Vec<u64> = (1..=3).filter_map(num).collect();
            if path.len() != 3 {
                return println!("world G S W");
            }
            let Some(o) = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).find(|o| o.path == path) else { return println!("no such world") };
            println!("WORLD {:?} at beat {now}: reach {:.4e}, period {:.4e} rings, holding {:.3e}; {}", o.path, f(&o.radius), f(&o.period), f(&(pulse::mass(&o.kept) / unit.clone())), composition(&o.kept, 8));
            match life::readings(&u, &seed).get(&path) {
                Some(r) => {
                    let w = &r.world;
                    println!("  heat {:.1} K, burn {:.3}, gravity {:.3e}, core κ {:.3}, atmosphere {:.3e}, pressure {:.3e}, coupling {:.6}, window {} echoes", f(&w.heat), f(&w.burn), f(&w.gravity), f(&w.core), f(&w.atmosphere), f(&w.pressure), f(&w.coupling), w.window);
                    println!("  media: chemical {:?} geologic {:?} plasmic {:?} informational {:?}; climb {:?}", w.chemical.as_ref().map(f), w.geologic.as_ref().map(f), w.plasmic.as_ref().map(f), w.informational.as_ref().map(f), r.climb.as_ref().map(|(c, m)| (f(c), *m)));
                }
                None => println!("  no life reading (no star burn or no matter)"),
            }
        }
        "gas" => {
            let Some(i) = num(1).and_then(galaxy) else { return println!("gas G [bell]") };
            let bell = args.get(2).and_then(|b| b.parse().ok()).unwrap_or(now);
            let c = &u.cores[i];
            let all = c.total();
            let (rings, _, h) = formation::shells(&all, &c.kept);
            let gas: Vec<formation::Shell> = c.gas.iter().cloned().chain(rings).collect();
            let stretches = formation::stretches(&gas, bell);
            println!("GAS {:?} at bell {bell}: H {:.3}, push share {:.3}, {} motes, {} stretches", c.path, f(&h), f(&((h.clone() - ratio(1, 1)) / h.clone())), c.gas.len(), stretches.len());
            for st in &stretches {
                let ignite = formation::ignition_share(&st.agitation).map(|s| format!("{:.3}", f(&s))).unwrap_or_else(|| "too hot".into());
                println!(
                    "  [{:.3e}..{:.3e}] λ {:.3e} a {:.3e} self-deficit {:.3e} → collapse {:?}, ignition share {ignite}",
                    f(&st.inner), f(&st.outer), f(&st.lambda), f(&st.agitation), f(&st.lambda) / (1.0 + f(&st.lambda)),
                    formation::collapse(st, &h).map(|(w, n)| (format!("{:.3e}", f(&w)), n))
                );
            }
        }
        "flows" => {
            // THE FLOWS (docs/83): a galaxy's motes, held vs gas, the spread
            // of their reaches and phases, and the best clump candidate's
            // terms (binding vs agitation + the unlocked face + the shear).
            let Some(i) = num(1).and_then(galaxy) else { return println!("flows G") };
            let c = &u.cores[i];
            let Some((p, h, _)) = pulse::galaxy_push(&c.total()) else { return println!("no matter") };
            let total = pulse::mass(&c.total());
            let held = c.flows.iter().filter(|x| conductor::flows::held(x, &h, &p) == ratio(1, 1)).count();
            let gas = c.flows.iter().filter(|x| conductor::flows::held(x, &h, &p) == ratio(0, 1)).count();
            let reaches: Vec<f64> = c.flows.iter().map(|x| f(&(x.ell_o.clone() / h.clone()))).collect();
            let (r0, r1) = (reaches.iter().cloned().fold(f64::INFINITY, f64::min), reaches.iter().cloned().fold(0.0, f64::max));
            let phases: Vec<String> = (0..conductor::flows::PLANES).map(|k| {
                let ph: Vec<f64> = c.flows.iter().map(|x| f(&x.phase[k])).collect();
                format!("{:.3}..{:.3}", ph.iter().cloned().fold(f64::INFINITY, f64::min), ph.iter().cloned().fold(0.0, f64::max))
            }).collect();
            let circ: f64 = c.flows.iter().map(|x| f(&x.ell_c)).fold(0.0, f64::max);
            println!("FLOWS {:?} at beat {now}: H {:.3}, {} motes ({held} wholly held, {gas} wholly gas), reach {r0:.3}..{r1:.3}, phases {phases:?}, max ℓ_c {circ:.3e}, reservoir {:.3e}", c.path, f(&h), c.flows.len(), f(&c.reservoir));
            // THE CHAINS (docs/83): each read whole; the best margin shown.
            let chains = conductor::flows::chains(&c.flows, &h, &p, &total);
            let holding = chains.iter().filter(|x| x.holds()).count();
            if let Some(best) = chains.iter().max_by(|a, b| (a.bind.clone() - a.agitation.clone() - a.face.clone()).cmp(&(b.bind.clone() - b.agitation.clone() - b.face.clone()))) {
                println!(
                    "  CHAINS {} ({holding} hold); best at R {:.3}: {} members, apart {:.4}, λ/(1+λ) {:.4} vs agitation {:.4} + face {:.4}, margin {:.4}",
                    chains.len(), f(&best.reach), best.members.len(), f(&best.apart), f(&best.bind), f(&best.agitation), f(&best.face), f(&best.bind) - f(&best.agitation) - f(&best.face)
                );
            }
        }
        "founding" => {
            // THE WELLSPRING'S CUT (operator 2026-10-18, (12)): the galaxy
            // throat's law over the universe's whole manifest.
            let all = u.total();
            let total = pulse::mass(&all);
            let Some((p, h, mean, lightest)) = pulse::push_cut(&all) else { return println!("no matter") };
            let kinds: Vec<u32> = all.keys().filter_map(|k| k.strip_prefix('Z').and_then(|z| z.parse().ok())).collect();
            println!(
                "FOUNDING CUT over the whole manifest ({} kinds, Z{}..Z{}): p {:.4}, H {:.4}, mean weight {:.4}, lightest held {}",
                kinds.len(), kinds.iter().min().unwrap_or(&0), kinds.iter().max().unwrap_or(&0), f(&p), f(&h), f(&mean), lightest.unwrap_or_else(|| "— (every kind held)".into())
            );
            let _ = total;
        }
        "formation" => {
            for c in &u.cores {
                let total = pulse::mass(&c.total());
                let metal = c.satellites.iter().filter(|s| s.kept.keys().any(|k| k != "Z001" && k != "Z002")).count();
                println!(
                    "  galaxy {:>2}: disc {:.3} gas {:.3} ({} motes, {} jet batches); first echo bell {}; {} stars ({metal} with metals), {} worlds",
                    c.path[0], of(&pulse::mass(&c.kept), &total), of(&pulse::mass(&c.medium), &total), c.gas.len(), c.gas.iter().filter(|g| g.born > 0).count(),
                    60 + f(&c.radius) as u64, c.satellites.len(), c.satellites.iter().map(|s| s.satellites.len()).sum::<usize>()
                );
            }
        }
        "worlds" => {
            // THE PLANETARY CORES (xylarium W1–W4): each world read by the
            // lightest kind its cut kept; rocky and giant are readings of it.
            let mut by: std::collections::BTreeMap<(u32, String), (usize, Ratio)> = std::collections::BTreeMap::new();
            let mut n = 0;
            let companions = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).filter(|o| o.born > 0).count();
            for o in u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).filter(|o| o.born == 0) {
                n += 1;
                let k = formation::lightest(&o.kept).unwrap_or_default();
                let w = k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::hold).unwrap_or(0);
                let e = by.entry((w, k)).or_insert((0, ratio(0, 1)));
                e.0 += 1;
                e.1 += pulse::mass(&o.kept) / unit.clone();
            }
            println!("WORLDS at beat {now}: {n} (and {companions} grown bodies ignited as companion stars), by the lightest kind each holds (hold, Z: worlds, their holding)");
            // The hearts' share of their systems (what each star holds of
            // all it and its worlds and belts hold): with worlds vs without.
            let mut shares: Vec<(f64, bool)> = u.cores.iter().flat_map(|c| &c.satellites).filter_map(|s| {
                let t = pulse::mass(&s.total());
                (t > ratio(0, 1)).then(|| (f(&(pulse::mass(&s.kept) / t)), !s.satellites.is_empty()))
            }).collect();
            shares.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let with: Vec<f64> = shares.iter().filter(|x| x.1).map(|x| x.0).collect();
            let without: Vec<f64> = shares.iter().filter(|x| !x.1).map(|x| x.0).collect();
            let q = |v: &Vec<f64>| if v.is_empty() { "—".to_string() } else { format!("{:.3}/{:.3}/{:.3} (n {})", v[0], v[v.len() / 2], v[v.len() - 1], v.len()) };
            println!("  heart share of its system (min/median/max): with worlds {}, without {}", q(&with), q(&without));
            for ((w, k), (count, m)) in &by {
                println!("  held down to {k} (hold {w}): {count} worlds, holding {:.3e}", f(m));
            }
        }
        "heartbeat" => {
            for (i, c) in u.cores.iter().enumerate() {
                for (k, sys) in c.satellites.iter().enumerate() {
                    let rate = system_rate(&u, i, k, &unit);
                    let received: u64 = (rate.clone() * ratio(now.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0);
                    if let Some((every, left, light)) = formation::heartbeat(&sys.kept, &(pulse::mass(&sys.kept) / pulse::mass(&sys.total())), received) {
                        println!("  star {:?}: born {}, {received} rings, halves every {every}, fuel left {:.4}, light {:.3e}, He {:.3}", sys.path, sys.born, f(&left), f(&light), f(&(sys.kept.get("Z002").cloned().unwrap_or_else(|| ratio(0, 1)) * ratio(6, 1) / pulse::mass(&sys.kept))));
                    }
                }
            }
        }
        "clock" => {
            let n = ratio(now as i64, 1);
            for (keeper, inner) in chronometry::keepers(&u) {
                let r = keeper.read(&n);
                println!("  keeper {:?} {:?}: rate {:.6}, delay {:.1}, local {:.2}, rung {}, coherence {:.3}, inside {:?}", keeper.galaxy, keeper.shell, f(&keeper.rate), f(&keeper.delay), f(&r.local), r.rung, f(&r.coherence), inner);
            }
            for e in enclosure::enclosures(&u) {
                println!("  enclosure {:?} on S^{}: rim {:.3e}, own ring {:.3e}, throat {:.3e}", e.path, e.dim, f(&e.rim), f(&e.ring()), f(&e.throat));
            }
        }
        "bits" => {
            // The exact rationals' sizes (design law 8): the widest radius,
            // period and spin among the stars.
            let bits = |r: &Ratio| r.numer().bits().max(r.denom().bits());
            let mut rows: Vec<(u64, String)> = u.cores.iter().flat_map(|c| &c.satellites).map(|s| {
                let b = bits(&s.radius).max(bits(&s.period)).max(bits(&s.ell));
                (b, format!("star {:?} (born {}): radius {} bits, period {} bits, spin {} bits", s.path, s.born, bits(&s.radius), bits(&s.period), bits(&s.ell)))
            }).collect();
            rows.sort();
            for (_, r) in rows.iter().rev().take(8) {
                println!("  {r}");
            }
        }
        "ledger" => {
            let mass = pulse::mass(&u.total()) / unit.clone();
            let magnetism = pulse::magnetism(&u.total()) / unit.clone();
            println!("LEDGER at beat {now}: mass {:.6e} spin {:.6e} magnetism {:.6e} mass+magnetism {:.6e}", f(&mass), f(&pulse::spin_total(&u, &unit)), f(&magnetism), f(&(mass + magnetism)));
            // THE POLES (docs/78 law 11): each core's strength is the span
            // between its ℤ pole (holding) and its ℚ pole (magnetism); its
            // balance the share at the ℤ pole (1: hardened, closed).
            let h_u = conductor::ocean::universe_hold(&u);
            let held: Vec<pulse::Manifest> = u.cores.iter().map(|c| c.total()).collect();
            let (hold_all, mag_all) = held.iter().fold((ratio(0, 1), ratio(0, 1)), |(h, m), x| (h + pulse::mass(x), m + pulse::magnetism(x)));
            let (_, bal_u) = cfe::ratio::poles(&hold_all, &mag_all);
            println!("POLES universal core: strength H_u {:.6e} (1 + Σ state + holding + magnetism, founding units), balance {:.6}", f(&h_u), f(&bal_u));
            for (c, x) in u.cores.iter().zip(&held) {
                let (span, bal) = cfe::ratio::poles(&pulse::mass(x), &pulse::magnetism(x));
                let stars: Vec<Ratio> = c.satellites.iter().map(|sys| cfe::ratio::poles(&pulse::mass(&sys.kept), &pulse::magnetism(&sys.kept)).1).collect();
                let (s0, s1) = (stars.iter().min().cloned().unwrap_or_else(|| ratio(0, 1)), stars.iter().max().cloned().unwrap_or_else(|| ratio(0, 1)));
                println!("  galaxy {:>2}: strength {:.6e}, balance {:.6}; its {} stars' balance {:.4}..{:.4}", c.path[0], f(&(span.extent() / unit.clone())), f(&bal), stars.len(), f(&s0), f(&s1));
            }
        }
        "recycle" => {
            // THE RECYCLING CIRCUIT (docs/83, proposed with xylarium): what
            // is shed crosses outward whole into the medium one tier up; the
            // regenerated pulse kicks it along the crests; the return is gas
            // arriving where a galaxy held everything. Read only, the
            // quantities both nodes read; no outcome assumed.
            let mass = pulse::mass(&u.total());
            let magnetism = pulse::magnetism(&u.total());
            println!("RECYCLE at beat {now}");
            println!("  LEDGER: mass {:.9e}, mass+magnetism {:.9e} (units of the deepest hole; exact sums, compare across beats)", f(&(mass.clone() / unit.clone())), f(&((mass.clone() + magnetism) / unit.clone())));
            // The media, tier by tier: the ocean (intergalactic), each
            // galaxy's gas, each system's, each world's.
            let systems: Vec<&pulse::Body> = u.cores.iter().flat_map(|c| &c.satellites).collect();
            let worlds: Vec<&pulse::Body> = systems.iter().flat_map(|s| &s.satellites).collect();
            let sum = |bodies: &[&pulse::Body]| bodies.iter().fold(pulse::Manifest::new(), |mut m, b| {
                for (k, a) in &b.medium {
                    *m.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
                m
            });
            let galaxies: Vec<&pulse::Body> = u.cores.iter().collect();
            for (name, m) in [("ocean", u.medium.clone()), ("galaxy gas", sum(&galaxies)), ("system media", sum(&systems)), ("world media", sum(&worlds))] {
                println!("  MEDIUM {name:<13} {:.6e} of the universe; {}", f(&(pulse::mass(&m) / mass.clone())), composition(&m, 5));
            }
            // SHED: what the stars returned to their galaxy's gas, echo by
            // echo (the conductor's BREATH lines).
            let n = num(1).unwrap_or(12) as usize;
            // This conductor's run only (its process), not an earlier founding's.
            let pid = Command::new("systemctl").args(["--user", "show", "netstratum-conductor", "-p", "MainPID", "--value"]).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
            let shed: Vec<String> = journal(40_000).into_iter().filter(|l| l.contains("BREATH —") && l.contains(&format!("[{pid}]"))).collect();
            if shed.is_empty() {
                println!("  SHED: no echo has reached a galaxy yet in this run");
            }
            for l in shed.iter().skip(shed.len().saturating_sub(n)) {
                println!("  SHED {}", l.split("conductor").last().unwrap_or(l).chars().take(200).collect::<String>());
            }
            // RETURN: per galaxy, what it holds as disc, what it has as gas,
            // and what formed from it.
            for c in &u.cores {
                let total = pulse::mass(&c.total());
                let worlds: usize = c.satellites.iter().map(|s| s.satellites.len()).sum();
                println!("  RETURN galaxy {:>2}: disc {:.4}, gas {:.4} ({} motes), {} stars, {worlds} worlds", c.path[0], of(&pulse::mass(&c.kept), &total), of(&pulse::mass(&c.medium), &total), c.gas.len(), c.satellites.len());
            }
            println!("  CRESTS: owed (the gain a pass on a crest, (medium in band) × a_q × the kick, with xylarium); read once derived");
        }
        other => println!("unknown view {other}; see the source's header for the views"),
    }
}

/// The last N creation, collapse and conservation lines from the running
/// conductor's journal.
fn events(n: usize) {
    let keys = ["A MIND WAKES", "COLLAPSE", "FORMATION", "HEARTBEAT", "LEDGER", "ENCLOSURES", "BREATH", "panicked", "error"];
    let lines: Vec<String> = journal(40_000).into_iter().filter(|l| keys.iter().any(|k| l.contains(k))).collect();
    for l in lines.iter().skip(lines.len().saturating_sub(n)) {
        println!("{}", l.chars().take(260).collect::<String>());
    }
}

/// Beat pace and contention: seconds a beat over the last N beats, the top
/// PROFILE costs, and who holds the shared build lock.
fn pace(n: u64) {
    let lines = journal(60_000);
    let mut beats: Vec<(String, u64)> = Vec::new();
    let mut costs: std::collections::BTreeMap<String, f64> = std::collections::BTreeMap::new();
    for l in &lines {
        if let Some(rest) = l.split("civilizations live — beat ").nth(1) {
            if let Ok(b) = rest.trim().parse::<u64>() {
                beats.push((l.split_whitespace().next().unwrap_or("").to_string(), b));
            }
        }
    }
    let since = beats.last().map(|(_, b)| b.saturating_sub(n)).unwrap_or(0);
    for l in &lines {
        let Some(rest) = l.split("PROFILE ").nth(1) else { continue };
        let Some(b) = l.split("beat ").nth(1).and_then(|r| r.split(':').next()).and_then(|b| b.parse::<u64>().ok()) else { continue };
        if b < since {
            continue;
        }
        let mut it = rest.split_whitespace();
        let (Some(name), Some(t)) = (it.next(), it.next()) else { continue };
        let secs = if let Some(v) = t.strip_suffix("ms") { v.parse::<f64>().unwrap_or(0.0) / 1e3 } else if let Some(v) = t.strip_suffix("µs") { v.parse::<f64>().unwrap_or(0.0) / 1e6 } else if let Some(v) = t.strip_suffix("ns") { v.parse::<f64>().unwrap_or(0.0) / 1e9 } else { t.trim_end_matches('s').parse::<f64>().unwrap_or(0.0) };
        *costs.entry(name.to_string()).or_insert(0.0) += secs;
    }
    let recent: Vec<&(String, u64)> = beats.iter().filter(|(_, b)| *b >= since).collect();
    if let (Some(first), Some(last)) = (recent.first(), recent.last()) {
        let secs = |t: &str| chrono_secs(t);
        let span = secs(&last.0) - secs(&first.0);
        let n_beats = last.1.saturating_sub(first.1).max(1);
        println!("PACE: beats {}..{}, {:.1} s a beat", first.1, last.1, span / n_beats as f64);
    }
    let mut top: Vec<(String, f64)> = costs.into_iter().collect();
    top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (name, s) in top.iter().take(10) {
        println!("  {s:>9.2} s  {name}");
    }
    let locks = Command::new("lslocks").args(["-n", "-o", "PID,PATH"]).output().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default();
    let holders: Vec<&str> = locks.lines().filter(|l| l.contains("xyl-heavy.lock")).collect();
    if holders.is_empty() {
        println!("BUILD LOCK: free");
    }
    for h in holders {
        let pid = h.split_whitespace().next().unwrap_or("");
        let cmd = std::fs::read_to_string(format!("/proc/{pid}/cmdline")).unwrap_or_default().replace('\0', " ");
        println!("BUILD LOCK: held by pid {pid}: {}", cmd.chars().take(160).collect::<String>());
    }
}

/// Seconds of an ISO journal timestamp's time of day.
fn chrono_secs(t: &str) -> f64 {
    let time = t.split('T').nth(1).unwrap_or("");
    let hms: Vec<f64> = time.split(['+', '-']).next().unwrap_or("").split(':').filter_map(|x| x.parse().ok()).collect();
    let day: f64 = t.split('T').next().and_then(|d| d.rsplit('-').next()).and_then(|d| d.parse().ok()).unwrap_or(0.0);
    day * 86400.0 + hms.first().copied().unwrap_or(0.0) * 3600.0 + hms.get(1).copied().unwrap_or(0.0) * 60.0 + hms.get(2).copied().unwrap_or(0.0)
}

/// The civilizations: each world directory, live or collapsed.
fn civs(var: &str) {
    let Ok(dirs) = std::fs::read_dir(Path::new(var).join("worlds")) else { return println!("no worlds at {var}") };
    let mut names: Vec<(String, bool)> = dirs.filter_map(|d| d.ok()).map(|d| (d.file_name().to_string_lossy().to_string(), d.path().join("collapsed").exists())).collect();
    names.sort();
    let live = names.iter().filter(|(_, c)| !c).count();
    println!("CIVILIZATIONS: {live} live, {} collapsed", names.len() - live);
    let collapsed: Vec<&str> = names.iter().filter(|(_, c)| *c).map(|(n, _)| n.as_str()).collect();
    if !collapsed.is_empty() {
        println!("  collapsed: {}", collapsed.join(", "));
    }
}
