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
use cfe::ratio::{ratio, Ratio};
use conductor::{breath, chronometry, enclosure, formation, life, pulse};
use std::path::Path;
use std::process::Command;

/// A reading served on the sexagesimal protractor (law 4).
fn f(r: &Ratio) -> conductor::reading::Sector {
    conductor::reading::sector(r)
}

/// `a / b`, zero where `b` holds nothing (a galaxy whose matter all left).
fn of(a: &Ratio, b: &Ratio) -> conductor::reading::Sector {
    conductor::reading::share(a, b)
}

/// The least and greatest of exact readings, served as one span.
fn span(v: &[Ratio]) -> String {
    match (v.iter().min(), v.iter().max()) {
        (Some(a), Some(b)) => format!("{}..{}", f(a), f(b)),
        _ => "none".into(),
    }
}

/// The heaviest elements of a manifest by holding, as `Z:share`.
fn composition(m: &pulse::Manifest, top: usize) -> String {
    let total = pulse::mass(m);
    if total == ratio(0, 1) {
        return "—".into();
    }
    let mut parts: Vec<(conductor::chem::Kind, Ratio)> = m.iter().map(|(k, a)| (k.clone(), pulse::mass(&[(k.clone(), a.clone())].into()) / total.clone())).collect();
    parts.sort_by(|a, b| b.1.cmp(&a.1));
    parts.iter().take(top).map(|(k, s)| format!("{k}:{}", f(s))).collect::<Vec<_>>().join(" ")
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
    breath::system_clock(u, g, s, unit)
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
            println!("UNIVERSE at beat {now}: {} galaxies, {stars} stars, {worlds} worlds; intergalactic medium {} of the whole", u.cores.len(), f(&(pulse::mass(&u.medium()) / universe_hold.clone())));
            for (i, c) in u.cores.iter().enumerate() {
                let total = pulse::mass(&c.total());
                println!(
                    "  galaxy {:>2}: share {}, disc {}, gas {} ({} motes), {} stars; distance {} (first echo at bell {}); clock {}",
                    c.path[0], f(&(total.clone() / universe_hold.clone())), of(&pulse::mass(&c.kept), &total), of(&pulse::mass(&c.medium), &total),
                    c.gas.len(), c.satellites.len(), c.radius, (ratio(60, 1) + c.radius.clone()).floor(), breath::galaxy_rate(&u, i, &unit).map(|g| f(&g).to_string()).unwrap_or_else(|| "the source's own (home)".into())
                );
            }
        }
        "galaxy" => {
            let Some(i) = num(1).and_then(galaxy) else { return println!("galaxy G: no such galaxy") };
            let c = &u.cores[i];
            let total = pulse::mass(&c.total());
            println!("GALAXY {:?} at beat {now}: share {} of the universe; distance {} light-rings, first echo at bell {}", c.path, f(&(total.clone() / universe_hold.clone())), c.radius, (ratio(60, 1) + c.radius.clone()).floor());
            println!("  core state hold {} (units of the deepest hole), disc {}: {}", f(&(pulse::core_state(c) / unit.clone())), f(&(pulse::mass(&c.kept) / unit.clone())), composition(&c.kept, 6));
            println!("  gas {} of the galaxy in {} motes: {}; gas spin {}", of(&pulse::mass(&c.medium), &total), c.gas.len(), composition(&c.medium, 6), f(&c.medium_spin));
            match breath::galaxy_rate(&u, i, &unit) {
                Some(g) => println!("  clock against the universal core {}", f(&g)),
                None => println!("  the home: its core is the source itself, so it has no clock of its own; each body reads its own share"),
            }
            for (s, sys) in c.satellites.iter().enumerate() {
                println!("  star {:?}: holding {}, born bell {}, reach {}, {} worlds, clock {}; {}", sys.path, f(&(pulse::mass(&sys.kept) / unit.clone())), sys.born, f(&sys.radius), sys.satellites.len(), f(&system_rate(&u, i, s, &unit)), composition(&sys.kept, 3));
            }
        }
        "star" => {
            let (Some(i), Some(s)) = (num(1).and_then(galaxy), num(2)) else { return println!("star G S") };
            let Some(k) = u.cores[i].satellites.iter().position(|x| x.path[1] == s) else { return println!("no such star") };
            let sys = &u.cores[i].satellites[k];
            let rate = system_rate(&u, i, k, &unit);
            let received: u64 = (rate.clone() * ratio(now.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0);
            println!("STAR {:?} at beat {now}: born bell {}, holding {} (units of the deepest hole), magnetism {}", sys.path, sys.born, f(&(pulse::mass(&sys.kept) / unit.clone())), f(&(pulse::magnetism(&sys.kept) / unit.clone())));
            println!("  matter: {}", composition(&sys.kept, 8));
            println!("  orbit: reach {}, period {} rings, spin ℓ {}; clock {}, {received} rings received since birth", f(&sys.radius), f(&sys.period), f(&sys.ell), f(&rate));
            match formation::burn(&sys.kept, &(pulse::mass(&sys.kept) / pulse::mass(&sys.total())), 1, &ratio(1, 1)) {
                Some((feed, light)) => match conductor::compound::after(&feed, received).and_then(|left| conductor::compound::read(&ratio(1, 1), &left, f)) {
                    Some(left) => println!("  HEARTBEAT: each received ring fuses {} of the fuel that remains; fuel left {left}; light {} per received ring", f(&feed), f(&light)),
                    None => println!("  HEARTBEAT: refused: the burn's share lies outside the whole"),
                },
                None => println!("  HEARTBEAT: no hydrogen fuel"),
            }
            println!("  coupling κ {}", formation::coupling(&sys.kept, &universe_hold).map(|k| f(&k).to_string()).unwrap_or_else(|| "none".into()));
            // The dance: each world's period against the dominant world's.
            let dom = sys.satellites.iter().max_by(|a, b| pulse::mass(&a.kept).cmp(&pulse::mass(&b.kept))).map(|o| o.period.clone());
            for o in &sys.satellites {
                let lock = dom.as_ref().filter(|p| **p > ratio(0, 1)).map(|p| o.period.clone() / p.clone()).map(|x| if x.denom() < &num_bigint::BigInt::from(1000) { format!("{x}") } else { format!("{} (free)", f(&x)) }).unwrap_or_default();
                println!("  world {:?}: reach {}, period {}, against the dominant {lock}; held down to {}; {}", o.path, f(&o.radius), f(&o.period), formation::lightest(&o.kept).map(|k| k.to_string()).unwrap_or_else(|| "none".into()), composition(&o.kept, 4));
            }
        }
        "world" => {
            let path: Vec<u64> = (1..=3).filter_map(num).collect();
            if path.len() != 3 {
                return println!("world G S W");
            }
            let Some(o) = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).find(|o| o.path == path) else { return println!("no such world") };
            println!("WORLD {:?} at beat {now}: reach {}, period {} rings, holding {}; {}", o.path, f(&o.radius), f(&o.period), f(&(pulse::mass(&o.kept) / unit.clone())), composition(&o.kept, 8));
            // Its compounds and their permanent dipoles (`chem::dipole`): how
            // polar each medium it holds is.
            let compounds: Vec<String> = o.kept.keys().filter(|k| conductor::chem::atom_w(k).is_none()).map(|k| format!("{k} dipole {}", f(&conductor::chem::dipole(k)))).collect();
            if !compounds.is_empty() {
                println!("  compounds: {}", compounds.join(", "));
            }
            match life::readings(&u, &seed).get(&path) {
                Some(r) => {
                    let w = &r.world;
                    println!("  heat {}, burn {}, gravity {}, core κ {}, atmosphere {}, pressure {}, coupling {}, window {} echoes", f(&w.heat), f(&w.burn), f(&w.gravity), f(&w.core), f(&w.atmosphere), f(&w.pressure), f(&w.coupling), w.window);
                    println!("  media: chemical {:?} geologic {:?} plasmic {:?} informational {:?}; climb {:?}", w.chemical.as_ref().map(|x| f(x).to_string()), w.geologic.as_ref().map(|x| f(x).to_string()), w.plasmic.as_ref().map(|x| f(x).to_string()), w.informational.as_ref().map(|x| f(x).to_string()), r.climb.as_ref().map(|(c, m)| (f(c).to_string(), *m)));
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
            let Some(stretches) = formation::stretches(&gas, bell) else { return println!("refused: a gas mote's cooling share lies outside the whole") };
            println!("GAS {:?} at bell {bell}: H {}, push share {}, {} motes, {} stretches", c.path, f(&h), f(&((h.clone() - ratio(1, 1)) / h.clone())), c.gas.len(), stretches.len());
            for st in &stretches {
                let ignite = formation::ignition_share(&st.agitation).map(|s| f(&s).to_string()).unwrap_or_else(|| "too hot".into());
                println!(
                    "  [{}..{}] λ {} a {} self-deficit {} → collapse {:?}, ignition share {ignite}",
                    f(&st.inner), f(&st.outer), f(&st.lambda), f(&st.agitation), f(&(st.lambda.clone() / (ratio(1, 1) + st.lambda.clone()))),
                    formation::collapse(st, &h).map(|(w, n)| (f(&w).to_string(), n))
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
            let Some(frame) = conductor::breath::galaxy_frame(&u, i, &conductor::breath::unit_mass(&u)) else { return println!("no frame") };
            let held = c.flows.iter().filter(|x| conductor::flows::held(x, &frame, &p) == ratio(1, 1)).count();
            let gas = c.flows.iter().filter(|x| conductor::flows::held(x, &frame, &p) == ratio(0, 1)).count();
            let reaches: Vec<Ratio> = c.flows.iter().map(|x| x.ell_o.clone() / h.clone()).collect();
            let phases: Vec<String> = (0..conductor::flows::PLANES).map(|k| span(&c.flows.iter().map(|x| x.phase[k].clone()).collect::<Vec<_>>())).collect();
            let circ = span(&c.flows.iter().map(|x| x.ell_orth.clone()).collect::<Vec<_>>());
            println!("FLOWS {:?} at beat {now}: H {}, {} motes ({held} wholly held, {gas} wholly gas), reach {}, phases {phases:?}, ℓ_⊥ {circ}, reservoir {}", c.path, f(&h), c.flows.len(), span(&reaches), f(&c.reservoir));
            // THE CHAINS (docs/83): each read whole; the best margin shown.
            let Some(enc) = conductor::breath::galaxy_frame(&u, i, &conductor::breath::unit_mass(&u)) else { return println!("no frame") };
            let chains = conductor::flows::chains(&c.flows, &enc, &p, &total);
            let holding = chains.iter().filter(|x| x.holds()).count();
            if let Some(best) = chains.iter().max_by(|a, b| (a.bind.clone() - a.agitation.clone() - a.face.clone()).cmp(&(b.bind.clone() - b.agitation.clone() - b.face.clone()))) {
                println!(
                    "  CHAINS {} ({holding} hold); best at R {}: {} members, apart {}, λ/(1+λ) {} vs agitation {} + face {}, margin {}",
                    chains.len(), f(&best.reach), best.members.len(), f(&best.apart), f(&best.bind), f(&best.agitation), f(&best.face), f(&(best.bind.clone() - best.agitation.clone() - best.face.clone()))
                );
            }
        }
        "position" => {
            // A GALACTIC CORE WEIGHED, AND THE ELEMENTS AT A POSITION IN ITS
            // DISC (operator 2026-10-19: "use our own measurements in order
            // to weigh the galactic core properly as a derivation ... a quick
            // idea as to the elemental core composition in alignment with
            // earths position"). Every reading is the galaxy's own: its
            // core's state and what its cut holds (the kinds, heaviest and
            // lightest, their share of the galaxy), and the kinds standing in
            // the band about a share `f` of the way to its rim (R = ℓ/H, the
            // rim at R = H; f given in percent, default 50, "halfway out"),
            // the band spread by the galaxy's push, `fH(1 ± p)`.
            let f_pct = num(1).unwrap_or(50);
            let fr = ratio(f_pct as i64, 100);
            for c in &u.cores {
                let all = c.total();
                let Some((p, h, _)) = pulse::galaxy_push(&all) else { continue };
                let g = pulse::mass(&all);
                let state = pulse::core_state(c);
                let zs = |m: &pulse::Manifest| {
                    let z: Vec<u32> = m.keys().filter_map(|k| conductor::chem::atom_w(k)).collect();
                    (z.iter().min().copied().unwrap_or(0), z.iter().max().copied().unwrap_or(0), m.len())
                };
                let (klo, khi, kn) = zs(&c.kept);
                let (_, ghi, gn) = zs(&all);
                println!("GALAXY {:?} receives {gn} kinds, the heaviest Z{ghi}", c.path);
                println!("GALAXY {:?}: core state {} + held {} of the galaxy's {} (core {} of it); held {kn} kinds, Z{klo}..Z{khi}; push p {}, H {}", c.path, f(&state), f(&pulse::mass(&c.kept)), f(&g), f(&((state.clone() + pulse::mass(&c.kept)) / (state + g.clone()))), f(&p), f(&h));
                let (lo, hi) = (fr.clone() * h.clone() * (ratio(1, 1) - p.clone()), fr.clone() * h.clone() * (ratio(1, 1) + p.clone()));
                let mut band = pulse::Manifest::new();
                for x in &c.flows {
                    let r = x.ell_o.clone() / h.clone();
                    if r >= lo && r <= hi {
                        *band.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
                    }
                }
                let bm = pulse::mass(&band);
                let mut top: Vec<(conductor::chem::Kind, Ratio)> = band.iter().map(|(k, a)| (k.clone(), a.clone() * conductor::flows::weight(k))).collect();
                top.sort_by(|a, b| b.1.cmp(&a.1));
                let (blo, bhi, bn) = zs(&band);
                let list: Vec<String> = top.iter().take(12).map(|(k, w)| format!("{k}:{}", of(w, &bm))).collect();
                println!("  AT {f_pct}% OF THE WAY TO THE RIM (R {}..{}): {bn} kinds, Z{blo}..Z{bhi}, {} of the galaxy; by holding: {}", f(&lo), f(&hi), of(&bm, &g), list.join(" "));
            }
        }
        "firmament" => {
            // THE FIRMAMENT'S ORDER OVER MANY PULSES (a reading on a copy):
            // the singularity's pulses alone (`pulse::orbit_galaxies`, the
            // torque and the repulsion), N echoes, read every N/10. Evenness:
            // each mote's distance on the sphere to its nearest neighbour
            // (1 − closeness, the product over planes of in-step), their
            // mean and spread (coefficient of variation; an ordered
            // arrangement reads near 0).
            let echoes = num(1).unwrap_or(400);
            let mut w = u.clone();
            // Read for the whole firmament and for its VESSEL, the kinds at
            // or above its heaviest seam (the twin weights where the peel
            // releases: Z106 on S^11, found by the atom, not named).
            let top = u.firmament.iter().filter_map(|x| conductor::chem::atom_w(&x.key)).max().unwrap_or(0);
            let seam = (2..=top).rev().find(|&z| cfe::almanac::atom::hold(z) == cfe::almanac::atom::hold(z - 1)).unwrap_or(0);
            let read_of = |ms: &[conductor::flows::Mote], at: u64, label: &str| {
                let n = ms.len();
                if n < 2 {
                    return println!("echo {at} {label}: {n} motes");
                }
                let nn: Vec<Ratio> = (0..n).filter_map(|i| (0..n).filter(|&j| j != i).map(|j| {
                    let c = (0..ms[i].phase.len().min(ms[j].phase.len())).fold(ratio(1, 1), |c, k| c * conductor::flows::in_step(&ms[i].phase[k], &ms[j].phase[k]));
                    ratio(1, 1) - c
                }).min()).collect();
                let count = Ratio::from_integer(n.into());
                let mean = nn.iter().fold(ratio(0, 1), |s, d| s + d.clone()) / count.clone();
                // The spread: the mean absolute deviation over the mean (0 for
                // an arrangement whose every neighbour stands equally far).
                let dev = nn.iter().fold(ratio(0, 1), |s, d| s + if *d > mean { d.clone() - mean.clone() } else { mean.clone() - d.clone() }) / count;
                let coincide = nn.iter().filter(|d| **d == ratio(0, 1)).count();
                println!("echo {at:5} {label} ({n}): nearest-neighbour distance mean {} spread {} range {}; {coincide} motes sitting on another", f(&mean), of(&dev, &mean), span(&nn));
            };
            let read = |w: &pulse::Universe, at: u64| {
                read_of(&w.firmament, at, "all");
                let vessel: Vec<conductor::flows::Mote> = w.firmament.iter().filter(|x| conductor::chem::atom_w(&x.key).is_some_and(|z| z >= seam)).cloned().collect();
                read_of(&vessel, at, &format!("vessel Z>={seam}"));
            };
            read(&w, 0);
            let step = (echoes / 10).max(1);
            for e in 1..=echoes {
                let _ = pulse::orbit_galaxies(&mut w);
                if e % step == 0 {
                    read(&w, e);
                }
            }
        }
        "census" => {
            // THE CENSUS (operator 2026-10-18: everything the universe makes,
            // read, never built): the medium, galaxies, stars, clouds,
            // systems by their worlds, their orbits, belts as debris, and the
            // worlds' resources.
            let whole = pulse::mass(&u.total());
            let share = |m: &pulse::Manifest| of(&pulse::mass(m), &whole);
            let h_of = |m: &pulse::Manifest| m.get(&conductor::chem::hydrogen()).cloned().unwrap_or_else(|| ratio(0, 1));
            let stars: Vec<&pulse::Body> = u.cores.iter().flat_map(|c| &c.satellites).collect();
            let remnants = stars.iter().filter(|s| s.remnant).count();
            // THE GALAXIES' ORBITS (`pulse::orbit_galaxies`): each core's
            // live reach and period in bells, its orbit's phase, its share.
            let gal: Vec<String> = u.cores.iter().map(|c| format!("[{}] R {} P {} phase {} share {}", c.path[0], f(&c.radius), f(&c.period), c.phase.first().map(|x| f(x).to_string()).unwrap_or_else(|| "none".into()), share(&c.total()))).collect();
            println!("GALAXIES: {}", gal.join("; "));
            // THE FIRMAMENT (`pulse::Universe::firmament`): its kinds, its
            // share, and where its orbit phases GATHER (a reading: phases
            // sorted round the turn, a new group wherever the gap between
            // neighbours exceeds the mean gap; each group's holding-weighted
            // centre and its kinds).
            if !u.firmament.is_empty() {
                let mut ph: Vec<(Ratio, Ratio, u32)> = u.firmament.iter().map(|x| (x.phase[0].clone(), x.amount.clone() * conductor::flows::weight(&x.key), conductor::chem::charge(&x.key))).collect();
                ph.sort_by(|a, b| a.0.cmp(&b.0));
                let n = ph.len();
                let gaps: Vec<Ratio> = (0..n).map(|i| if i + 1 < n { ph[i + 1].0.clone() - ph[i].0.clone() } else { ph[0].0.clone() + ratio(1, 1) - ph[i].0.clone() }).collect();
                let mean = Ratio::new(1.into(), n.max(1).into());
                let mut groups: Vec<Vec<usize>> = vec![Vec::new()];
                for i in 0..n {
                    if let Some(g) = groups.last_mut() {
                        g.push(i);
                    }
                    if gaps[i] > mean && i + 1 < n {
                        groups.push(Vec::new());
                    }
                }
                let fh = u.firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * conductor::flows::weight(&x.key));
                let desc: Vec<String> = groups.iter().filter(|g| !g.is_empty()).map(|g| {
                    let w = g.iter().fold(ratio(0, 1), |s, &i| s + ph[i].1.clone());
                    let centre = if w > ratio(0, 1) { g.iter().fold(ratio(0, 1), |s, &i| s + ph[i].0.clone() * ph[i].1.clone()) / w } else { ph[g[0]].0.clone() };
                    let zs: Vec<u32> = g.iter().map(|&i| ph[i].2).collect();
                    format!("{}:{}k(Z{}-{})", f(&centre), g.len(), zs.iter().min().copied().unwrap_or(0), zs.iter().max().copied().unwrap_or(0))
                }).collect();
                println!("FIRMAMENT: {} motes, share {}; phases gather in {} groups: {}", n, of(&fh, &whole), desc.len(), desc.join(" "));
            }
            let worlds_of = |s: &pulse::Body| s.satellites.iter().filter(|w| w.born == 0).count();
            let mut by: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
            for s in &stars {
                *by.entry(worlds_of(s)).or_insert(0) += 1;
            }
            let gas_h = u.cores.iter().map(|c| h_of(&c.medium)).chain(stars.iter().map(|s| h_of(&s.medium))).fold(ratio(0, 1), |a, b| a + b);
            let all_h = h_of(&u.total());
            println!("CENSUS at beat {now}: medium {} of the universe, {} galaxies, {} stars ({} burning, {} remnants), {} companions",
                share(&u.medium()), u.cores.len(), stars.len(), stars.len() - remnants, remnants,
                stars.iter().map(|s| s.satellites.iter().filter(|w| w.born > 0).count()).sum::<usize>());
            let sys_h = stars.iter().map(|s| h_of(&s.medium)).fold(ratio(0, 1), |a, b| a + b);
            println!("  CLOUDS: hydrogen in the galaxies' and systems' gas {} of all hydrogen; in the SYSTEMS' media alone {} ({} of all)", of(&gas_h, &all_h), f(&sys_h), of(&sys_h, &all_h));
            println!("  SYSTEMS by worlds: {}", by.iter().map(|(k, v)| format!("{k}:{v}")).collect::<Vec<_>>().join(" "));
            // Orbits of the multi-world systems: reaches and period ratios.
            let mut shown = 0;
            for s in stars.iter().filter(|s| worlds_of(s) >= 2) {
                let mut ws: Vec<&pulse::Body> = s.satellites.iter().filter(|w| w.born == 0).collect();
                ws.sort_by(|a, b| a.radius.cmp(&b.radius));
                let reaches: Vec<String> = ws.iter().map(|w| f(&w.radius).to_string()).collect();
                let ratios: Vec<String> = ws.windows(2).map(|p| if p[0].period == ratio(0, 1) { "-".into() } else { f(&(p[1].period.clone() / p[0].period.clone())).to_string() }).collect();
                println!("  ORBITS star {:?}: {} worlds at {} (period ratios {})", s.path, ws.len(), reaches.join(" "), ratios.join(" "));
                shown += 1;
                if shown >= 6 {
                    break;
                }
            }
            // Belts: the debris standing in a system's own medium as motes.
            let debris: Vec<Ratio> = stars.iter().map(|s| s.flows.iter().fold(ratio(0, 1), |a, x| a + x.amount.clone())).collect();
            let with_belts = debris.iter().filter(|d| **d > ratio(0, 1)).count();
            println!("  BELTS: {} systems carry debris motes, total {} (units); remnants' released systems re-anchored count in SYSTEMS",
                with_belts, f(&debris.iter().fold(ratio(0, 1), |a, b| a + b.clone())));
            // Where the debris stands against its system's rim (R = ℓ/H_s, rim 1).
            let mut rs: Vec<Ratio> = Vec::new();
            for s in &stars {
                let tot = pulse::mass(&s.total());
                if tot == ratio(0, 1) {
                    continue;
                }
                let h_s = pulse::mass(&s.kept) / tot;
                if h_s == ratio(0, 1) {
                    continue;
                }
                rs.extend(s.flows.iter().map(|x| x.ell_o.clone() / h_s.clone()));
            }
            rs.sort();
            if let (Some(lo), Some(hi)) = (rs.first(), rs.last()) {
                println!("  DEBRIS reach against the rim (1): {} motes, min {} median {} max {}", rs.len(), f(lo), f(&rs[(rs.len() - 1) / 2]), f(hi));
            }
            // LIGHT AT THE ORBIT (operator 2026-10-18: "let there be light"; with
            // xylarium): each burning star's light a ring, the transduced flux of
            // its rungs (formation::burn, in full step), and each world's
            // INSOLATION, that flux times the share of directions the star
            // covers from the world's reach, δ_s(r) = H_s/(r + H_s), never 1/r².
            let mut lit: Vec<(Ratio, Ratio, usize)> = Vec::new();
            for s in &stars {
                let tot = pulse::mass(&s.total());
                if tot == ratio(0, 1) {
                    continue;
                }
                let h_s = pulse::mass(&s.kept) / tot.clone();
                let share = h_s.clone();
                if s.satellites.iter().any(|w| w.born == 0) {
                    println!("  HOST star {:?}: {}, heart {:?}", s.path, if s.remnant { "a REMNANT" } else { "burning" }, s.kept.keys().take(4).collect::<Vec<_>>());
                }
                let flux = s.kept.keys().filter_map(conductor::chem::atom_w).filter_map(|z| formation::burn(&s.kept, &share, z, &ratio(1, 1)).map(|(_, l)| l)).fold(ratio(0, 1), |a, b| a + b);
                let scale = tot / u.unit.clone();
                for w in s.satellites.iter().filter(|w| w.born == 0) {
                    let r = if scale == ratio(0, 1) { ratio(0, 1) } else { w.radius.clone() / scale.clone() };
                    let cover = h_s.clone() / (r + h_s.clone());
                    lit.push((flux.clone(), flux.clone() * cover, w.kept.len()));
                }
            }
            if !lit.is_empty() {
                let ins: Vec<String> = lit.iter().take(12).map(|(fl, i, k)| format!("{}/{}({}k)", f(fl), f(i), k)).collect();
                println!("  LIGHT (star flux / world insolation (kinds)): {}", ins.join(" "));
            }
            // CHEMISTRY: molecules in the worlds and in the systems' media,
            // with water (O + 4H on S^11) named.
            let in_worlds: Vec<(&conductor::chem::Kind, &Ratio)> = stars.iter().flat_map(|s| &s.satellites).flat_map(|w| w.kept.iter()).filter(|(k, _)| conductor::chem::atom_w(k).is_none()).collect();
            let in_media: Vec<(&conductor::chem::Kind, &Ratio)> = stars.iter().flat_map(|s| s.flows.iter().map(|x| (&x.key, &x.amount))).filter(|(k, _)| conductor::chem::atom_w(k).is_none()).collect();
            let sum = |v: &[(&conductor::chem::Kind, &Ratio)], k: Option<&str>| f(&v.iter().filter(|(kk, _)| k.map_or(true, |k| kk.to_string() == k)).fold(ratio(0, 1), |a, (_, x)| a + Ratio::clone(x)));
            let kinds_m: std::collections::BTreeSet<&conductor::chem::Kind> = in_worlds.iter().chain(in_media.iter()).map(|(k, _)| *k).collect();
            println!("  CHEMISTRY: {} molecule kinds; water in worlds {}, in system media {}; all molecules in worlds {}, in media {}",
                kinds_m.len(), sum(&in_worlds, Some("W0010·4W0001")), sum(&in_media, Some("W0010·4W0001")), sum(&in_worlds, None), sum(&in_media, None));
            // Resources: the worlds' mixes.
            let worlds: Vec<&pulse::Body> = stars.iter().flat_map(|s| &s.satellites).filter(|w| w.born == 0).collect();
            let mut kinds: std::collections::BTreeSet<conductor::chem::Kind> = std::collections::BTreeSet::new();
            let mut lightest: std::collections::BTreeMap<conductor::chem::Kind, usize> = std::collections::BTreeMap::new();
            for w in &worlds {
                kinds.extend(w.kept.keys().cloned());
                if let Some(k) = formation::lightest(&w.kept) {
                    *lightest.entry(k).or_insert(0) += 1;
                }
            }
            println!("  RESOURCES: {} worlds hold {} kinds between them; cores held down to {}", worlds.len(), kinds.len(), lightest.iter().map(|(k, v)| format!("{k}:{v}")).collect::<Vec<_>>().join(" "));
            // Layers: how many kinds each world holds, and its top kind's share.
            let layers: Vec<String> = worlds.iter().take(12).map(|w| {
                let m = pulse::mass(&w.kept);
                let top = w.kept.iter().map(|(k, a)| (k.clone(), pulse::mass(&[(k.clone(), a.clone())].into()))).max_by(|a, b| a.1.cmp(&b.1));
                match top {
                    Some((k, t)) => format!("{}k/{k}:{}", w.kept.len(), of(&t, &m)),
                    None => format!("{}k", w.kept.len()),
                }
            }).collect();
            println!("  LAYERS (kinds per world / top kind:share): {}", layers.join(" "));
        }
        "manifest" => {
            // The universe's whole manifest, one kind a line: Z, amount, hold.
            for (k, a) in u.total() {
                println!("{k} {} {}", f(&a), conductor::chem::hold(&k));
            }
        }
        "cascade" => {
            // THE FOUNDING CASCADE (operators 2026-10-18; a READING only, with
            // xylarium): the stack, heaviest first, beads by its weight profile
            // (amount × hold per kind; pinches at maxima, partings at minima)
            // into stacks. Bounce k releases the top stack at the full push
            // over what is still stacked, b_k = p·M₀/M_k (regeneration (i)),
            // each kind carried ℓ = b_k·w̄/w; the beat is the round trip to the
            // top of what REMAINS, 1 + R_top/H, settling at the throat's own.
            let all = u.total();
            let w = |k: &conductor::chem::Kind| conductor::chem::hold(k);
            let mut kinds: Vec<(conductor::chem::Kind, Ratio)> = all.iter().filter(|(_, a)| **a > ratio(0, 1)).map(|(k, a)| (k.clone(), a.clone())).collect();
            kinds.sort_by(|a, b| w(&b.0).cmp(&w(&a.0)).then(a.0.cmp(&b.0)));
            let total = pulse::mass(&all);
            let count = all.values().fold(ratio(0, 1), |s, a| s + a.clone());
            let mean = total.clone() / count;
            let prof: Vec<Ratio> = kinds.iter().map(|(k, a)| a.clone() * ratio(i64::from(w(k)), 1)).collect();
            let mut stacks: Vec<Vec<usize>> = vec![vec![0]];
            for i in 1..kinds.len() {
                let parting = i + 1 < kinds.len() && prof[i] < prof[i - 1] && prof[i] <= prof[i + 1];
                stacks.last_mut().expect("a stack").push(i);
                if parting {
                    stacks.push(Vec::new());
                }
            }
            stacks.retain(|s| !s.is_empty());
            let h_u = ratio(2, 1);
            let p = ratio(1, 2);
            let mut left = total.clone();
            println!("CASCADE over {} kinds: {} stacks (heaviest first), H {}", kinds.len(), stacks.len(), f(&h_u));
            for (k, st) in stacks.iter().enumerate() {
                let m: Ratio = st.iter().map(|&i| prof[i].clone()).fold(ratio(0, 1), |a, b| a + b);
                let b = p.clone() * total.clone() / left.clone();
                let reach = |i: usize| b.clone() * mean.clone() / ratio(i64::from(w(&kinds[i].0)), 1) / h_u.clone();
                let (r_in, r_out) = (reach(st[0]), reach(*st.last().expect("a kind")));
                left -= m.clone();
                // What remains: its top kind's reach at this push.
                let top = stacks.get(k + 1).and_then(|s| s.first()).map(|&i| reach(i));
                let beat = top.map(|r| ratio(1, 1) + r / h_u.clone());
                println!(
                    "  stack {:>2}: {}..{} ({} kinds), share {}, push {}, reach {}..{}; next beat {}",
                    k + 1, kinds[st[0]].0, kinds[*st.last().expect("a kind")].0, st.len(), f(&(m / total.clone())), f(&b), f(&r_in), f(&r_out),
                    beat.map(|x| f(&x).to_string()).unwrap_or_else(|| "— (settled: the throat's own, 1)".into())
                );
            }
        }
        "founding" => {
            // THE WELLSPRING'S CUT (operator 2026-10-18, (12)): the galaxy
            // throat's law over the universe's whole manifest.
            let all = u.total();
            let total = pulse::mass(&all);
            let Some((p, h, mean, lightest)) = pulse::push_cut(&all) else { return println!("no matter") };
            let kinds: Vec<u32> = all.keys().filter_map(conductor::chem::atom_w).collect();
            println!(
                "FOUNDING CUT over the whole manifest ({} kinds, Z{}..Z{}): p {}, H {}, mean weight {}, lightest held {}",
                kinds.len(), kinds.iter().min().unwrap_or(&0), kinds.iter().max().unwrap_or(&0), f(&p), f(&h), f(&mean), lightest.map(|k| k.to_string()).unwrap_or_else(|| "— (every kind held)".into())
            );
            let _ = total;
        }
        "hearts" => {
            // What a galaxy's stars hold: each heart's top kinds by share.
            let Some(i) = num(1).and_then(galaxy) else { return println!("hearts G") };
            for s in u.cores[i].satellites.iter().take(8) {
                let m = pulse::mass(&s.kept);
                let mut ks: Vec<(&conductor::chem::Kind, &Ratio)> = s.kept.iter().collect();
                ks.sort_by(|a, b| b.1.cmp(a.1));
                let top: Vec<String> = ks.iter().take(6).map(|(k, a)| format!("{k}:{}", f(&(Ratio::clone(a) * cfe::ratio::ratio(i64::from(conductor::chem::hold(k)), 1) / m.clone())))).collect();
                println!("  star born {} remnant {} heart {}: {}", s.born, s.remnant, f(&m), top.join(" "));
            }
        }
        "formation" => {
            for c in &u.cores {
                let total = pulse::mass(&c.total());
                let metal = c.satellites.iter().filter(|s| s.kept.keys().any(|k| conductor::chem::charge(k) > 2)).count();
                let remnants = c.satellites.iter().filter(|s| s.remnant).count();
                let with_h = c.satellites.iter().filter(|s| s.kept.get(&conductor::chem::hydrogen()).is_some_and(|a| *a > ratio(0, 1))).count();
                println!(
                    "  galaxy {:>2}: disc {} gas {} ({} motes, {} jet batches); first echo bell {}; {} stars ({metal} with metals, {remnants} remnants, {with_h} holding hydrogen), {} worlds",
                    c.path[0], of(&pulse::mass(&c.kept), &total), of(&pulse::mass(&c.medium), &total), c.gas.len(), c.gas.iter().filter(|g| g.born > 0).count(),
                    (ratio(60, 1) + c.radius.clone()).floor(), c.satellites.len(), c.satellites.iter().map(|s| s.satellites.len()).sum::<usize>()
                );
            }
        }
        "worlds" => {
            // THE PLANETARY CORES (xylarium W1–W4): each world read by the
            // lightest kind its cut kept; rocky and giant are readings of it.
            let mut by: std::collections::BTreeMap<(u32, conductor::chem::Kind), (usize, Ratio)> = std::collections::BTreeMap::new();
            let mut n = 0;
            let companions = u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).filter(|o| o.born > 0).count();
            for o in u.cores.iter().flat_map(|c| &c.satellites).flat_map(|s| &s.satellites).filter(|o| o.born == 0) {
                n += 1;
                let Some(k) = formation::lightest(&o.kept) else { continue };
                let w = conductor::chem::hold(&k);
                let e = by.entry((w, k)).or_insert((0, ratio(0, 1)));
                e.0 += 1;
                e.1 += pulse::mass(&o.kept) / unit.clone();
            }
            println!("WORLDS at beat {now}: {n} (and {companions} grown bodies ignited as companion stars), by the lightest kind each holds (hold, Z: worlds, their holding)");
            // The hearts' share of their systems (what each star holds of
            // all it and its worlds and belts hold): with worlds vs without.
            let mut shares: Vec<(Ratio, bool)> = u.cores.iter().flat_map(|c| &c.satellites).filter_map(|s| {
                let t = pulse::mass(&s.total());
                (t > ratio(0, 1)).then(|| (pulse::mass(&s.kept) / t, !s.satellites.is_empty()))
            }).collect();
            shares.sort_by(|a, b| a.0.cmp(&b.0));
            let with: Vec<Ratio> = shares.iter().filter(|x| x.1).map(|x| x.0.clone()).collect();
            let without: Vec<Ratio> = shares.iter().filter(|x| !x.1).map(|x| x.0.clone()).collect();
            let q = |v: &Vec<Ratio>| match (v.first(), v.last()) {
                (Some(a), Some(b)) => format!("{}/{}/{} (n {})", f(a), f(&v[v.len() / 2]), f(b), v.len()),
                _ => "—".to_string(),
            };
            println!("  heart share of its system (min/median/max): with worlds {}, without {}", q(&with), q(&without));
            for ((w, k), (count, m)) in &by {
                println!("  held down to {k} (hold {w}): {count} worlds, holding {}", f(m));
            }
        }
        "heartbeat" => {
            for (i, c) in u.cores.iter().enumerate() {
                for (k, sys) in c.satellites.iter().enumerate() {
                    let rate = system_rate(&u, i, k, &unit);
                    let received: u64 = (rate.clone() * ratio(now.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0);
                    let burn = formation::burn(&sys.kept, &(pulse::mass(&sys.kept) / pulse::mass(&sys.total())), 1, &ratio(1, 1));
                    if let Some((feed, light, left)) = burn.and_then(|(feed, light)| conductor::compound::after(&feed, received).and_then(|left| conductor::compound::read(&ratio(1, 1), &left, f)).map(|left| (feed, light, left))) {
                        println!("  star {:?}: born {}, {received} rings, fuses {} a ring, fuel left {left}, light {}, He {}", sys.path, sys.born, f(&feed), f(&light), f(&(conductor::chem::Kind::atom(2).and_then(|he| sys.kept.get(&he).cloned()).unwrap_or_else(|| ratio(0, 1)) * ratio(6, 1) / pulse::mass(&sys.kept))));
                    }
                }
            }
        }
        "clock" => {
            let n = ratio(now as i64, 1);
            for (keeper, inner) in chronometry::keepers(&u) {
                let r = keeper.read(&n);
                println!("  keeper {:?} {:?}: rate {}, delay {}, local {}, rung {}, coherence {}, inside {:?}", keeper.galaxy, keeper.shell, f(&keeper.rate), f(&keeper.delay), f(&r.local), r.rung, f(&r.coherence), inner);
            }
            for e in enclosure::enclosures(&u) {
                println!("  enclosure {:?} on S^{}: rim {}, own ring {}, throat {}", e.path, e.dim, f(&e.rim), f(&e.ring()), f(&e.throat));
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
            println!("LEDGER at beat {now}: mass {} spin {} magnetism {} mass+magnetism {}", f(&mass), f(&pulse::spin_total(&u, &unit)), f(&magnetism), f(&(mass + magnetism)));
            // THE POLES (docs/78 law 11): each core's strength is the span
            // between its ℤ pole (holding) and its ℚ pole (magnetism); its
            // balance the share at the ℤ pole (1: hardened, closed).
            let h_u = conductor::ocean::universe_hold(&u);
            let held: Vec<pulse::Manifest> = u.cores.iter().map(|c| c.total()).collect();
            let (hold_all, mag_all) = held.iter().fold((ratio(0, 1), ratio(0, 1)), |(h, m), x| (h + pulse::held(x), m + pulse::magnetism(x)));
            let (_, bal_u) = cfe::ratio::poles(&hold_all, &mag_all);
            println!("POLES universal core: strength H_u {} (1 + Σ state + holding + magnetism, founding units), balance {}", f(&h_u), f(&bal_u));
            for (c, x) in u.cores.iter().zip(&held) {
                let (span, bal) = cfe::ratio::poles(&pulse::held(x), &pulse::magnetism(x));
                let stars: Vec<Ratio> = c.satellites.iter().map(|sys| cfe::ratio::poles(&pulse::held(&sys.kept), &pulse::magnetism(&sys.kept)).1).collect();
                let (s0, s1) = (stars.iter().min().cloned().unwrap_or_else(|| ratio(0, 1)), stars.iter().max().cloned().unwrap_or_else(|| ratio(0, 1)));
                println!("  galaxy {:>2}: strength {}, balance {}; its {} stars' balance {}..{}", c.path[0], f(&(span.extent() / unit.clone())), f(&bal), stars.len(), f(&s0), f(&s1));
            }
        }
        "recycle" => {
            // THE RECYCLING CIRCUIT (docs/83, proposed with xylarium): what
            // is shed crosses outward whole into the medium one tier up; the
            // regenerated pulse torques it along the crests; the return is gas
            // arriving where a galaxy held everything. Read only, the
            // quantities both nodes read; no outcome assumed.
            let mass = pulse::mass(&u.total());
            let magnetism = pulse::magnetism(&u.total());
            println!("RECYCLE at beat {now}");
            println!("  LEDGER: mass {}, mass+magnetism {} (units of the deepest hole; exact sums, compare across beats)", f(&(mass.clone() / unit.clone())), f(&((mass.clone() + magnetism) / unit.clone())));
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
            for (name, m) in [("ocean", u.medium()), ("galaxy gas", sum(&galaxies)), ("system media", sum(&systems)), ("world media", sum(&worlds))] {
                println!("  MEDIUM {name:<13} {} of the universe; {}", f(&(pulse::mass(&m) / mass.clone())), composition(&m, 5));
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
                println!("  RETURN galaxy {:>2}: disc {}, gas {} ({} motes), {} stars, {worlds} worlds", c.path[0], of(&pulse::mass(&c.kept), &total), of(&pulse::mass(&c.medium), &total), c.gas.len(), c.satellites.len());
            }
            println!("  CRESTS: owed (the gain a pass on a crest, (medium in band) × a_q × the torque, with xylarium); read once derived");
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
