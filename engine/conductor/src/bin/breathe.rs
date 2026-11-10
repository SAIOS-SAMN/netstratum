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
    // From a checkpoint (TUniversal 11.0 (23)): a universe written at a beat
    // resumes there (`conductor.beat`), never from the founding.
    let start: u64 = std::fs::read_to_string(Path::new(era1).join("conductor.beat"))
        .ok()
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or(0)
        .min(until);
    if start > 0 {
        println!("resumed at bell {start} (the checkpoint)");
    }
    let spin_before = pulse::spin_total(&u, &breath::unit_mass(&u));
    let mut events = Vec::new();
    // The carrier's GROWTH, read as the run goes (ruling of 2026-10-23: a
    // carrier growing uncontrolled with no end in sight needs a course of
    // action): each tenth universal echo, the motes held and the widest spin
    // and phase in bits, counted, never timed (MU-RECURSION rule 13). A chunk
    // that more than doubles the motes of the one before stops the run: the
    // growth is the reading, and what was read so far is reported.
    let bits = |r: &cfe::ratio::Ratio| r.numer().bits().max(r.denom().bits());
    let carrier = |u: &pulse::Universe| -> (usize, u64, usize) {
        // The firmament stands in the home's flows (B1), counted with them.
        let mut n = 0;
        let mut in_systems = 0usize;
        let mut widest = 0u64;
        let mut see = |x: &conductor::flows::Mote| {
            widest = widest.max(bits(&x.ell_o())).max(bits(&x.ell_orth));
            for p in &x.phase {
                widest = widest.max(bits(p));
            }
        };
        for c in &u.cores {
            n += c.flows.len();
            c.flows.iter().for_each(&mut see);
            for s in &c.satellites {
                n += s.flows.len();
                in_systems += s.flows.len();
                s.flows.iter().for_each(&mut see);
            }
        }
        (n, widest, in_systems)
    };
    // Every stored field's width (the scalar-collapse audit: a value stored
    // and stepped through a recurrence widens chunk on chunk; a bounded one
    // levels): the widest exact value held per field, per tier, in bits.
    let fields = |u: &pulse::Universe| -> std::collections::BTreeMap<String, u64> {
        let mut w: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
        let mut put = |k: String, r: &cfe::ratio::Ratio| {
            let b = bits(r);
            let e = w.entry(k).or_insert(0);
            *e = (*e).max(b);
        };
        fn body(tier: &str, b: &pulse::Body, put: &mut dyn FnMut(String, &cfe::ratio::Ratio)) {
            let f = |n: &str| format!("{tier}.{n}");
            for (n, r) in [
                ("ell", &b.ell),
                ("ell_orth", &b.ell_orth),
                ("radius", &b.radius),
                ("period", &b.period),
                ("medium_spin", &b.medium_spin),
                ("state", &b.state),
                ("reservoir", &b.reservoir),
                ("turning", &b.turning),
                ("load", &b.load),
                ("heat", &b.heat),
            ] {
                put(f(n), r);
            }
            for p in b.phase.iter() {
                put(f("phase"), p);
            }
            for p in b.bell.iter() {
                put(f("bell"), p);
            }
            for a in b.kept.values() {
                put(f("kept"), a);
            }
            for a in b.medium.values() {
                put(f("medium"), a);
            }
            for x in &b.flows {
                for (n, r) in [
                    ("amount", &x.amount),
                    ("ell_o", &x.ell_o()),
                    ("ell_orth", &x.ell_orth),
                    ("agitation", &x.agitation),
                    ("lever_place", &x.lever_place),
                ] {
                    put(format!("{tier}.mote.{n}"), r);
                }
                for p in x.phase.iter() {
                    put(format!("{tier}.mote.phase"), p);
                }
                for l in x.lever.iter() {
                    put(format!("{tier}.mote.lever"), l);
                }
            }
        }
        for x in &u.ocean {
            for (n, r) in [
                ("amount", &x.amount),
                ("ell_o", &x.ell_o()),
                ("ell_orth", &x.ell_orth),
                ("agitation", &x.agitation),
            ] {
                put(format!("ocean.{n}"), r);
            }
            for p in x.phase.iter() {
                put("ocean.phase".into(), p);
            }
        }
        for c in &u.cores {
            body("galaxy", c, &mut put);
            for s in &c.satellites {
                body("system", s, &mut put);
                for x in &s.satellites {
                    body("world", x, &mut put);
                }
            }
        }
        w
    };
    let show = |w: &std::collections::BTreeMap<String, u64>| {
        w.iter()
            .map(|(k, b)| format!("{k} {b}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    eprintln!("fields at bell {start}: {}", show(&fields(&u)));
    let started = std::time::Instant::now();
    let mut grown_past: Option<u64> = None;
    let mut last_motes = carrier(&u).0;
    // The energy book (docs/122 step 1, read only): each form's change per
    // universal echo against the light made, received and drained in it.
    let book_start = conductor::energy::read(&u);
    let mut book_prev = book_start.clone();
    let mut light_from = events.len();
    for b in start..until {
        events.extend(breath::breathe(&mut u, &seed, t_u, b, b + 1));
        // Progress by the universe's own count, never a clock: each tenth
        // universal echo.
        // Each universal echo: the motes, the systems' bodies, the time.
        if t_u > 0 && (b + 1) % t_u == 0 {
            let bodies: usize = u
                .cores
                .iter()
                .flat_map(|c| &c.satellites)
                .map(|s| s.satellites.len() + s.flows.len())
                .sum();
            let systems: usize = u.cores.iter().map(|c| c.satellites.len()).sum();
            let (merged_n, dissolved_n, born_n, worlds_n): (usize, usize, usize, usize) =
                events.iter().fold((0, 0, 0, 0), |a, e| {
                    (
                        a.0 + e.merged,
                        a.1 + e.dissolved,
                        a.2 + e.stars,
                        a.3 + e.fates.0,
                    )
                });
            let worlds_now: usize = u
                .cores
                .iter()
                .flat_map(|c| &c.satellites)
                .map(|s| s.satellites.len())
                .sum();
            {
                let book = conductor::energy::read(&u);
                let since = &events[light_from..];
                let lit = |f: &dyn Fn(&breath::Event) -> cfe::ratio::Ratio| {
                    since.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + f(e))
                };
                let (made, got, gone) = (
                    lit(&|e| e.light.made.clone()),
                    lit(&|e| e.light.received.clone()),
                    lit(&|e| e.light.drained.clone()),
                );
                eprintln!(
                    "{}",
                    energy_line(
                        &format!("at bell {}", b + 1),
                        &book_prev,
                        &book,
                        &made,
                        &got,
                        &gone
                    )
                );
                book_prev = book;
                light_from = events.len();
            }
            if let Some((t_e, r_rim)) = pulse::established_period(&u) {
                eprintln!(
                    "rim at bell {}: R_rim {}, established period T_u = 2·a(R_rim) = {} bells; stored T_u {}; whole holding {}, within the rim {}",
                    b + 1,
                    conductor::reading::sector(&r_rim),
                    conductor::reading::sector(&t_e),
                    t_u,
                    pulse::home_frame(&u).map(|e| conductor::reading::sector(&e.whole()).to_string()).unwrap_or_default(),
                    pulse::home_frame(&u).map(|e| conductor::reading::sector(&e.hold_within(&r_rim)).to_string()).unwrap_or_default()
                );
            }
            if std::env::var_os("NETSTRATUM_PROFILE").is_some() {
                conductor::profile::mark("breathe:echo");
                let spent = conductor::profile::drain();
                let whole: f64 = spent.iter().map(|(_, d)| d.as_secs_f64()).sum();
                for (name, d) in spent.iter().take(14) {
                    eprintln!(
                        "profile echo at bell {} {name}: {:.1} s ({:.0} %)",
                        b + 1,
                        d.as_secs_f64(),
                        100.0 * d.as_secs_f64() / whole.max(1e-9)
                    );
                }
            }
            let front = pulse::contained_front(&u);
            for plane in 0..conductor::flows::SPACE.len() {
                let cells: Vec<&pulse::FrontCell> =
                    front.iter().filter(|c| c.plane == plane).collect();
                let (Some(lo), Some(hi)) = (
                    cells.iter().map(|c| c.period.clone()).min(),
                    cells.iter().map(|c| c.period.clone()).max(),
                ) else {
                    continue;
                };
                eprintln!(
                    "front at bell {} plane {plane}: {} cells, round trips {}..{} bells; stored T_u {}",
                    b + 1,
                    cells.len(),
                    conductor::reading::sector(&lo),
                    conductor::reading::sector(&hi),
                    t_u
                );
            }
            // The lean (the founding's orientation, `pulse::orient`): how far
            // the planes have come apart. The universal core's tilt and the
            // galaxy motes' (by holding) as offsets from a third, the widest
            // single mote's, and how many motes stand at different phases in
            // different planes.
            {
                use num_traits::ToPrimitive;
                let third = cfe::ratio::ratio(1, 3);
                let off = |t: &[cfe::ratio::Ratio]| -> Vec<f64> {
                    conductor::flows::tilt_of(t)
                        .iter()
                        .map(|x| (x.clone() - third.clone()).to_f64().unwrap_or(f64::NAN))
                        .collect()
                };
                let home = u
                    .cores
                    .iter()
                    .find(|c| pulse::is_home(c))
                    .map(|c| off(&c.tilt))
                    .unwrap_or_default();
                let motes: Vec<&conductor::flows::Mote> =
                    u.cores.iter().flat_map(|c| c.flows.iter()).collect();
                let mut sum = [0f64; 3];
                let (mut whole, mut widest, mut apart) = (0f64, 0f64, 0usize);
                for x in &motes {
                    let w = x.amount.to_f64().unwrap_or(0.0);
                    for (j, o) in off(&x.tilt()).iter().enumerate() {
                        sum[j] += w * o;
                        widest = widest.max(o.abs());
                    }
                    whole += w;
                    let ph = conductor::flows::space_of(&x.phase);
                    if ph.iter().any(|p| *p != ph[0]) {
                        apart += 1;
                    }
                }
                let mean: Vec<f64> = sum.iter().map(|s| s / whole.max(1e-300)).collect();
                eprintln!(
                    "lean at bell {}: core {:+.3e} {:+.3e} {:+.3e}; motes {:+.3e} {:+.3e} {:+.3e}, widest {:.3e}; {apart} of {} motes apart in phase",
                    b + 1,
                    home.first().copied().unwrap_or(f64::NAN),
                    home.get(1).copied().unwrap_or(f64::NAN),
                    home.get(2).copied().unwrap_or(f64::NAN),
                    mean[0],
                    mean[1],
                    mean[2],
                    widest,
                    motes.len()
                );
            }
            // The planes' horizons (docs/117 §8–§9), read: no motion changes.
            {
                let h = conductor::horizons::read(&u);
                eprintln!(
                    "horizon at bell {}: {} motes; whole inside {}; planes inside by the whole's hold {:?}, by their own {:?}; a turning plane inside with the whole outside {}, the reverse {}; tilt ties {}; the two holds disagree {}",
                    b + 1,
                    h.motes,
                    h.whole_inside,
                    h.by_whole,
                    h.by_own,
                    h.plane_in_whole_out,
                    h.plane_out_whole_in,
                    h.ties,
                    h.holds_disagree
                );
            }
            eprintln!("echo at bell {}: {} systems ({} merged, {} dissolved, {} born so far), {} worlds standing ({} grown), {} bodies and motes in systems, {} galaxy motes, {} ocean motes, {:?} since start", b + 1, systems, merged_n, dissolved_n, born_n, worlds_now, worlds_n, bodies, u.cores.iter().map(|c| c.flows.len()).sum::<usize>(), u.ocean.len(), started.elapsed());
        }
        if t_u > 0 && (b + 1) % (10 * t_u) == 0 {
            let (motes, widest, in_systems) = carrier(&u);
            let (rung, read) = events
                .iter()
                .fold((0usize, 0usize), |a, e| (a.0 + e.rang.0, a.1 + e.rang.1));
            eprintln!("bell {}: {} echoes read; {motes} motes ({in_systems} in the systems), widest {widest} bits; star bells rang {rung} of {read} star-echoes", b + 1, events.len());
            eprintln!("fields at bell {}: {}", b + 1, show(&fields(&u)));
            if last_motes > 0 && motes > 2 * last_motes {
                grown_past = Some(b + 1);
                break;
            }
            last_motes = motes;
        }
    }
    if let Some(at) = grown_past {
        println!("growth — the motes more than doubled in the chunk ending at bell {at}: the run stopped there");
    }
    let until = grown_past.unwrap_or(until);
    let unit = breath::unit_mass(&u);
    let births: usize = events.iter().map(|e| e.stars).sum();
    let dissolved: usize = events.iter().map(|e| e.dissolved).sum();
    let novae: usize = events.iter().map(|e| e.novae).sum();
    let lifted_n: usize = events.iter().map(|e| e.lifted.0).sum();
    let lifted_du = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.lifted.1.clone());
    let returned = events
        .iter()
        .fold(cfe::ratio::ratio(0, 1), |s, e| s + e.returned.clone());
    println!(
        "stars met — {} merged, {} bounced",
        events.iter().map(|e| e.merged).sum::<usize>(),
        events.iter().map(|e| e.encounters).sum::<usize>()
    );
    {
        let sticks: usize = events.iter().map(|e| e.grown.0).sum();
        let bounces: usize = events.iter().map(|e| e.grown.1).sum();
        let escaped = events
            .iter()
            .fold(cfe::ratio::ratio(0, 1), |s, e| s + e.escaped.clone());
        let last: std::collections::BTreeMap<u32, (usize, usize)> = events
            .iter()
            .map(|e| (e.galaxy, (e.grown.2, e.grown.3)))
            .collect();
        let born = events
            .iter()
            .fold(cfe::ratio::ratio(0, 1), |s, e| s + e.discs.0.clone());
        let out = events
            .iter()
            .fold(cfe::ratio::ratio(0, 1), |s, e| s + e.discs.1.clone());
        let (cl, fell) = events.iter().fold(
            (cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1)),
            |s, e| (s.0 + e.collapse.0.clone(), s.1 + e.collapse.1.clone()),
        );
        println!(
            "infall — bindings {} (holding); fell into the star cores {}, the rest their discs",
            conductor::reading::sector(&cl),
            conductor::reading::sector(&fell)
        );
        let f = events
            .iter()
            .fold((0usize, 0usize, 0usize, 0usize), |s, e| {
                (
                    s.0 + e.fates.0,
                    s.1 + e.fates.1,
                    s.2 + e.fates.2,
                    s.3 + e.fates.3,
                )
            });
        println!("grown — bodies grown by meetings: {} worlds, {} ignited as stars, {} whose core held nothing, {} that left past the rim", f.0, f.1, f.2, f.3);
        let w = events.iter().fold(
            [
                cfe::ratio::ratio(0, 1),
                cfe::ratio::ratio(0, 1),
                cfe::ratio::ratio(0, 1),
                cfe::ratio::ratio(0, 1),
            ],
            |mut s, e| {
                for (a, b) in s.iter_mut().zip(e.weights.iter()) {
                    *a += b.clone();
                }
                s
            },
        );
        let mean = |m: &cfe::ratio::Ratio, n: &cfe::ratio::Ratio| {
            if n.is_zero() {
                "none".to_string()
            } else {
                conductor::reading::sector(&(m.clone() / n.clone())).to_string()
            }
        };
        println!("weight — a nucleus's mean hold: grown groups that left past the rim {}, worlds grown {}", mean(&w[0], &w[1]), mean(&w[2], &w[3]));
        let im = events.iter().fold((0usize, 0usize, 0usize), |s, e| {
            (s.0 + e.impacts.0, s.1 + e.impacts.1, s.2 + e.impacts.2)
        });
        println!("impacts — bounces that struck as matter {}, tugs between neighbouring worlds {}; grown groups carried past the rim by their orthogonal orbit's swing alone {}", im.0, im.1, im.2);
        let ms = events.iter().fold([0usize; 6], |mut a, e| {
            for i in 0..5 {
                a[i] += e.motes[i];
            }
            a
        });
        let held: usize = {
            let mut last: std::collections::BTreeMap<u32, usize> =
                std::collections::BTreeMap::new();
            for e in &events {
                last.insert(e.galaxy, e.motes[5]);
            }
            last.values().sum()
        };
        println!("motes — entered the galaxies' flows: arrivals {}, shed and through the stars' throats {}, recycling {}, binding members returned {}; folded by the merge {}; held at the end {held}", ms[0], ms[1], ms[2], ms[3], ms[4]);
        let rel = events
            .iter()
            .fold((0usize, cfe::ratio::ratio(0, 1)), |s, e| {
                (s.0 + e.released.0, s.1 + e.released.1.clone())
            });
        let throat_left: cfe::ratio::Ratio = {
            let mut last: std::collections::BTreeMap<u32, cfe::ratio::Ratio> =
                std::collections::BTreeMap::new();
            for e in &events {
                last.insert(e.galaxy, e.released.2.clone());
            }
            last.values()
                .fold(cfe::ratio::ratio(0, 1), |s, x| s + x.clone())
        };
        println!("release — the bounces released {} motes from the throats, holding {}; the throats still hold {}", rel.0, conductor::reading::sector(&rel.1), conductor::reading::sector(&throat_left));
        println!("discs — released at the stars' births {} (holding), at or past the rim when released {}", conductor::reading::sector(&born), conductor::reading::sector(&out));
        println!("systems met — {sticks} stuck, {bounces} bounced; overstepped the rims {} (holding); after the last echo, motes and bodies by galaxy {last:?}", conductor::reading::sector(&escaped));
    }
    for (name, spent) in conductor::profile::drain() {
        eprintln!("profile {name}: {:.1}s", spent.as_secs_f64());
    }
    println!("breath — {} bells, T_u {t_u}: {} echoes (perpendicular writes down); recohered {births} systems born, decohered {} returned and {dissolved} stars dissolved (writes up); {novae} novae (a remnant's gathered fuel thrown off); the magnet on reach moved {lifted_n} lifts by {} in all", until, events.len(), returned.round(), conductor::reading::sector(&lifted_du));
    for (i, c) in u.cores.iter().enumerate() {
        let echoes = events
            .iter()
            .filter(|e| u64::from(e.galaxy) == c.path[0])
            .count();
        let born = c.satellites.len();
        let orbs: usize = c.satellites.iter().map(|s| s.satellites.len()).sum();
        // Its galactic year and the bell its next one closes (`motion`).
        let year = motion::galactic_year(c)
            .map(|(k, t)| {
                let next = t
                    .turns(until)
                    .and_then(|n| t.bell_of_turn(n + 1))
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "past the carrier".into());
                format!(
                    "half-mass system {}, year {} bells, next closes at bell {next}",
                    k + 1,
                    t.period.round()
                )
            })
            .unwrap_or_else(|| "no year yet".into());
        // The pump (convection, `ocean::pump`): the universal tide's
        // difference across the galaxy's swing.
        let pump = {
            let whole = pulse::mass(&u.total());
            let h_g = if num_traits::Zero::is_zero(&whole) {
                cfe::ratio::ratio(0, 1)
            } else {
                (pulse::core_state(c) + pulse::mass(&c.total())) / whole
            };
            let ring = cfe::ratio::ratio(t_u as i64, 4);
            conductor::reading::sector(&conductor::ocean::pump(
                &(c.radius.clone() / ring),
                &h_g,
                &conductor::ocean::universe_hold(&u),
            ))
            .to_string()
        };
        let year = format!("{year}; pump {pump}");
        let medium = pulse::mass(&c.medium) / unit.clone();
        match breath::galaxy_rate(&u, i, &unit) {
            Some(g_rate) => {
                let aeon = g_rate.clone() * cfe::ratio::ratio(until as i64, 1);
                let rungs = breath::rungs(&breath::slip(&g_rate, until, t_u))
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "unread".into());
                let coh = breath::coherence(&breath::slip(&g_rate, until, t_u));
                println!("  galaxy {:?} at {} bells out: {echoes} echoes, {rungs} rungs (recoherences), coherence now {}%, aeon {} bells (rate {}); {born} systems, {orbs} orbs; medium left {}; {year}", c.path, c.radius, (coh * cfe::ratio::ratio(100, 1)).round(), aeon.round(), conductor::reading::sector(&g_rate), medium.round());
            }
            None => {
                // The home's bodies read their own clocks: the spread of its
                // systems' rates against the source.
                let rates: Vec<cfe::ratio::Ratio> = (0..c.satellites.len())
                    .map(|s| breath::system_clock(&u, i, s, &unit))
                    .collect();
                let span = match (rates.iter().min(), rates.iter().max()) {
                    (Some(a), Some(b)) => format!(
                        "{}..{}",
                        conductor::reading::sector(a),
                        conductor::reading::sector(b)
                    ),
                    _ => "none".into(),
                };
                println!("  the home {:?} at the source: {echoes} echoes; no clock of its own, its {born} systems' clocks {span}; {orbs} orbs; medium left {}; {year}", c.path, medium.round());
            }
        }
    }
    // The clocks (docs/78): which duty comes due how often, by layer.
    let terms: Vec<Option<u64>> = (0..u.cores.len())
        .map(|i| conductor::clocks::galaxy_terms(&u, i, t_u, 0, until))
        .collect();
    let epochs: Vec<Option<u64>> = (0..u.cores.len())
        .map(|i| conductor::clocks::galaxy_epochs(&u, i, &unit, t_u, 0, until))
        .collect();
    let worlds: Vec<u64> = u
        .cores
        .iter()
        .flat_map(|c| &c.satellites)
        .flat_map(|s| &s.satellites)
        .filter_map(|o| conductor::clocks::local_years(&o.period, 0, until))
        .collect();
    let journaled = worlds.iter().filter(|&&y| y > 0).count();
    let mut w = worlds.clone();
    w.sort();
    println!("clocks — over {until} bells: galaxy terms (echoes received) {terms:?}; galaxy epochs (half-mass rungs) {epochs:?}; worlds' journals: {} worlds, {journaled} with a completed local year (most years {:?}, median {:?}); universal echoes {}", worlds.len(), w.last(), w.get(w.len() / 2), until / t_u);
    // Fusion transmutes kinds (H + H → He), so the manifest itself changes;
    // what the breath conserves is each nucleus's 3Z, holding plus
    // magnetism (transduction via magnetism).
    // Mass is already the whole 3Z (holding and magnetism converged).
    let three_z = |m: &pulse::Manifest| pulse::mass(m);
    println!("conservation — the breath created nothing (holding + magnetism, 3Z): {}; the manifest unchanged: {}", three_z(&u.total()) == three_z(&before), u.total() == before);
    // The age (operator 2026-10-19: "universes age from their conception on
    // the TUniversal sexagesimal logged at their birth (distribution of
    // resources)"; hydrogen the commonest at the founding, a reading, then the
    // physics of recycling): hydrogen's and helium's shares by nuclei at the
    // founding and now, a reading of age, never a flag.
    let all = u.total();
    let count = |m: &pulse::Manifest, k: Option<conductor::chem::Kind>| {
        k.and_then(|k| m.get(&k).cloned())
            .unwrap_or_else(|| cfe::ratio::ratio(0, 1))
    };
    let whole = |m: &pulse::Manifest| {
        m.values()
            .fold(cfe::ratio::ratio(0, 1), |s, a| s + a.clone())
    };
    let share = |m: &pulse::Manifest, z: u32| {
        conductor::reading::share(&count(m, conductor::chem::Kind::element(z)), &whole(m))
    };
    let commonest = all
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _)| k.to_string())
        .unwrap_or_else(|| "none".into());
    println!(
        "age — by nuclei, H {} -> {}, He {} -> {} (founding -> now); commonest now {commonest}",
        share(&before, 1),
        share(&all, 1),
        share(&before, 2),
        share(&all, 2)
    );
    // The parity (with Xylarium): the odd share of the Z >= 3 atoms by
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
    println!(
        "parity — odd share of Z >= 3 by nuclei: {} -> {} (founding -> now)",
        odd(&before),
        odd(&all)
    );
    // Recycling at the galaxies' throats (`conductor::recycle`, 11.0 (18)):
    // pressed (constructed) and peeled (deconstructed), the light and the
    // opened holding, and what was trapped at the throats.
    let sum = |f: &dyn Fn(&breath::Event) -> cfe::ratio::Ratio| {
        events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + f(e))
    };
    let f = conductor::reading::sector;
    {
        // The heaviest nuclide standing anywhere (docs/97 §10).
        let all = u.total();
        let top = all
            .keys()
            .flat_map(|k| k.parts().iter().map(|(_, w)| *w).collect::<Vec<_>>())
            .max()
            .unwrap_or(0);
        let heavy: cfe::ratio::Ratio = all
            .iter()
            .filter(|(k, _)| conductor::chem::weight(k) > 4)
            .fold(cfe::ratio::ratio(0, 1), |s, (k, a)| {
                s + a.clone() * cfe::ratio::ratio(i64::from(conductor::chem::weight(k)), 1)
            })
            / pulse::mass(&all);
        println!(
            "heaviest — weight {top} (Q {}, Z {}); matter past weight 4 by mass {}",
            conductor::nuclides::protons(top).unwrap_or(0),
            conductor::nuclides::neutrons(top).unwrap_or(0),
            f(&heavy)
        );
    }
    println!("circulation — the shear moved {} outward, the pressure {} radially and {} around the shells' circles ({} shell planes parted into cells) (spin, ledger units); the luminal bound turned {} motes onto the Möbius leg; the stream read {} on the vacuum; the trapped carried {} spin to the reservoirs; {} jets left past the rims", f(&sum(&|e| e.circulation.0.clone())), f(&sum(&|e| e.circulation.1.clone())), f(&sum(&|e| e.circulation.4.clone())), events.iter().map(|e| e.circulation.5).sum::<usize>(), events.iter().map(|e| e.circulation.2).sum::<usize>(), events.iter().map(|e| e.circulation.3).sum::<usize>(), f(&sum(&|e| e.recycled.spun.clone())), events.iter().map(|e| e.jets).sum::<usize>());
    let closing: Vec<u32> = motion::years_closing(&u, until);
    let rungs: Vec<u64> = events.iter().map(|e| e.rung).collect();
    let parallel = events
        .iter()
        .filter(|e| matches!(e.write, breath::Write::Parallel))
        .count();
    println!("echoes — rungs read {}..{}, {} written parallel and {} perpendicular; galactic years closing at bell {until}: {closing:?}", rungs.iter().min().copied().unwrap_or(0), rungs.iter().max().copied().unwrap_or(0), parallel, events.len() - parallel);
    println!("recycling — the throats pressed {} and peeled {} (grains of nuclei); heat given {}, light given {}, holding opened {}, trapped {} (units); {} echoes recycled", f(&sum(&|e| e.recycled.pressed.clone())), f(&sum(&|e| e.recycled.peeled.clone())), f(&sum(&|e| e.recycled.heat.clone())), f(&sum(&|e| e.recycled.light.clone())), f(&sum(&|e| e.recycled.spent.clone())), f(&sum(&|e| e.recycled.trapped.clone())), events.iter().filter(|e| e.recycled != Default::default()).count());
    // The burn and the return as rates (TUniversal 11.0 (23)): hydrogen fused
    // by the stars and returned net by the throats, per universal echo, as
    // shares of the hydrogen standing now, over the run's windows (tenths).
    // Where the two meet is the hydrogen's steady state, a fixed point; a
    // read this early is the first instant, never a judgement.
    {
        let h_now = all
            .get(&conductor::chem::hydrogen())
            .cloned()
            .unwrap_or_else(|| cfe::ratio::ratio(0, 1));
        let echoes = ((until - start) / t_u.max(1)).max(1);
        let window = (echoes / 10).max(1);
        let mut rows: Vec<String> = Vec::new();
        let mut w = 0;
        while w * window < echoes {
            let (lo, hi) = (
                start + w * window * t_u,
                (start + (w + 1) * window * t_u).min(until),
            );
            let sel = events.iter().filter(|e| e.bell > lo && e.bell <= hi);
            let (mut b, mut r) = (cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1));
            for e in sel {
                b += e
                    .burned
                    .get(&conductor::chem::hydrogen())
                    .cloned()
                    .unwrap_or_else(|| cfe::ratio::ratio(0, 1));
                r += e.recycled.hydrogen.clone();
            }
            let per = |x: cfe::ratio::Ratio| {
                conductor::reading::share(
                    &x,
                    &(h_now.clone() * cfe::ratio::ratio(window as i64, 1)),
                )
            };
            rows.push(format!(
                "u {}-{}: burned {} returned {}",
                start / t_u.max(1) + w * window,
                start / t_u.max(1) + (w + 1) * window,
                per(b),
                per(r)
            ));
            w += 1;
        }
        println!(
            "rates — hydrogen per universal echo, as a share of the hydrogen now: {}",
            rows.join("; ")
        );
    }
    // The carrier's size (design law 8; the cut-over's measure): the widest
    // exact rational held, by what it carries, in bits.
    {
        let bits = |r: &cfe::ratio::Ratio| r.numer().bits().max(r.denom().bits());
        let mut amounts = 0u64;
        let mut spins = 0u64;
        let mut phases = 0u64;
        let mut agitation = 0u64;
        let seen = |m: &pulse::Manifest, w: &mut u64| {
            for a in m.values() {
                *w = (*w).max(bits(a));
            }
        };
        seen(&u.medium(), &mut amounts);
        let mut motes: Vec<&conductor::flows::Mote> = Vec::new();
        for c in &u.cores {
            seen(&c.total(), &mut amounts);
            motes.extend(c.flows.iter());
            for s in &c.satellites {
                seen(&s.total(), &mut amounts);
                motes.extend(s.flows.iter());
                spins = spins.max(bits(&s.ell)).max(bits(&s.radius));
                for p in s.phase.iter().chain(&s.bell) {
                    phases = phases.max(bits(p));
                }
            }
        }
        for x in &motes {
            amounts = amounts.max(bits(&x.amount));
            spins = spins.max(bits(&x.ell_o())).max(bits(&x.ell_orth));
            agitation = agitation.max(bits(&x.agitation));
            for p in &x.phase {
                phases = phases.max(bits(p));
            }
        }
        println!("carrier — widest exact value held, in bits: amounts {amounts}, spins and reaches {spins}, phases {phases}, agitation {agitation}; {} motes", motes.len());
    }
    // The cross (the eight pivots): every residue's fate over the run, by
    // fate, every plane together.
    {
        let mut all = conductor::enfold::Cross::default();
        for e in &events {
            all.join(&e.cross);
        }
        let row: Vec<String> = all
            .counts()
            .iter()
            .map(|(f, n)| format!("{f:?} {n}"))
            .collect();
        println!(
            "cross — residues by fate over the run: {}",
            if row.is_empty() {
                "none".into()
            } else {
                row.join(", ")
            }
        );
    }
    // The energy book over the run (docs/122 step 1, read only).
    {
        let sum = |f: &dyn Fn(&breath::Event) -> cfe::ratio::Ratio| {
            events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + f(e))
        };
        let book = conductor::energy::read(&u);
        println!(
            "{}",
            energy_line(
                "over the run",
                &book_start,
                &book,
                &sum(&|e| e.light.made.clone()),
                &sum(&|e| e.light.received.clone()),
                &sum(&|e| e.light.drained.clone()),
            )
        );
    }
    // The light ledger: light made (the ℚ → ℤ transduction's flux), received
    // as heat by the bodies on its continuum face, drained into the vacuum.
    {
        let sum = |f: &dyn Fn(&breath::Event) -> cfe::ratio::Ratio| {
            events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + f(e))
        };
        let (made, got, gone) = (
            sum(&|e| e.light.made.clone()),
            sum(&|e| e.light.received.clone()),
            sum(&|e| e.light.drained.clone()),
        );
        println!("light — made {}, received {} by the bodies on the continuum face, drained {} into the vacuum (units); balanced: {}", conductor::reading::sector(&made), conductor::reading::sector(&got), conductor::reading::sector(&gone), made == got.clone() + gone.clone());
    }
    // The firmament's collective (docs/111 §4): its bonds read at each
    // universal bounce; the last bounce's reading and the greatest.
    {
        let read: Vec<&pulse::Collective> = events
            .iter()
            .filter_map(|e| e.collective.as_ref())
            .collect();
        if let Some(last) = read.last() {
            let most = read.iter().map(|c| c.collections).max().unwrap_or(0);
            let share = if last.held == cfe::ratio::ratio(0, 1) {
                cfe::ratio::ratio(0, 1)
            } else {
                last.in_collections.clone() / last.held.clone()
            };
            println!("collective — {} bounces read; the last: {} motes, {} collections ({} motes bonded, the largest {}), {} of the holding bonded; at most {} collections; the transmission τ {}; weights held {}..={}, the collective's mean {}", read.len(), last.motes, last.collections, last.bonded, last.largest, conductor::reading::sector(&share), most, conductor::reading::sector(&last.tau), last.lightest, last.heaviest, conductor::reading::sector(&last.mean));
        }
    }
    // The galaxies' formation (docs/111 §5.4), from each bounce's own record:
    // its candidates, its collections of two or more, the self-held and the
    // formed, and the collection nearest its pinch (its widest extent over its
    // pinch on its worse plane; at most 1 holds).
    for (n, f) in events.iter().filter_map(|e| e.formation.as_ref()).enumerate() {
        let worst = |g: &pulse::GroupReading| {
            g.planes
                .iter()
                .map(|p| {
                    if p.pinch == cfe::ratio::ratio(0, 1) {
                        None
                    } else {
                        Some(p.widest.clone() / p.pinch.clone())
                    }
                })
                .max_by(|a, b| match (a, b) {
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, _) => std::cmp::Ordering::Greater,
                    (_, None) => std::cmp::Ordering::Less,
                    (Some(x), Some(y)) => x.cmp(y),
                })
                .flatten()
        };
        let nearest = f
            .groups
            .iter()
            .filter_map(|g| worst(g).map(|w| (w, g.members)))
            .min_by(|a, b| a.0.cmp(&b.0));
        let held = f.groups.iter().filter(|g| g.self_held).count();
        println!(
            "formation — bounce {}: {} candidates, {} collections of two or more, {} self-held, {} formed; the nearest its pinch {}",
            n + 1,
            f.candidates,
            f.groups.len(),
            held,
            f.formed,
            match nearest {
                Some((w, m)) => format!("{} of its pinch ({} members)", conductor::reading::sector(&w), m),
                None => "none".to_string(),
            }
        );
    }
    // The tolls: every star that passed, struck on its own bell.
    {
        let tolls: Vec<&breath::Toll> = events.iter().flat_map(|e| e.tolls.iter()).collect();
        println!("tolls — {} stars passed", tolls.len());
        for t in tolls {
            let heard: Vec<String> = (0..u.cores.len())
                .filter_map(|g| {
                    t.heard_after(&u, g).map(|d| {
                        format!(
                            "{:?} after {}",
                            u.cores[g].path,
                            conductor::reading::sector(&d)
                        )
                    })
                })
                .collect();
            println!("  star {:?} at bell {}: {:?}, {} strokes, returned {} holding and {} spin; heard by {}", t.star, t.bell, t.passing, t.strokes.map(|n| n.to_string()).unwrap_or_else(|| "uncounted".into()), conductor::reading::sector(&t.returned), conductor::reading::sector(&t.spin), heard.join(", "));
        }
    }
    // The spin identities (docs/90): each check's residual, `after − before`
    // of its ledger's whole sum, exact where 0.
    {
        let mut kinds: std::collections::BTreeMap<&str, (u64, u64, cfe::ratio::Ratio)> =
            std::collections::BTreeMap::new();
        for (kind, r) in events.iter().flat_map(|e| e.identity.iter()) {
            let abs = if *r < cfe::ratio::ratio(0, 1) {
                -r.clone()
            } else {
                r.clone()
            };
            let e = kinds.entry(kind).or_insert((0, 0, cfe::ratio::ratio(0, 1)));
            e.0 += 1;
            if abs == cfe::ratio::ratio(0, 1) {
                e.1 += 1;
            }
            if abs > e.2 {
                e.2 = abs;
            }
        }
        for (kind, (n, exact, worst)) in &kinds {
            println!(
                "identity {kind} — {exact} of {n} exact; greatest residual {}",
                conductor::reading::sector(worst)
            );
        }
        let spin_after = pulse::spin_total(&u, &breath::unit_mass(&u));
        println!("identity ledger — the bodies' spin (both orbits) and media, before {} after {}: moved {} (motes and reservoirs are outside it)", conductor::reading::sector(&spin_before), conductor::reading::sector(&spin_after), conductor::reading::sector(&(spin_after.clone() - spin_before.clone())));
    }
    use num_traits::Zero as _;
    // The swing (docs/90): each galaxy's stars' and worlds' elongations, their
    // orthogonal spin over their orbit spin, `e = |ℓ_⊥|/ℓ` (the reach swings over
    // `R(1 ± e)` each orthogonal turn): how many carry one, their greatest and their
    // holding-weighted mean; and the orthogonal spin the ledger holds (Σ m·ℓ_⊥).
    {
        let unit = breath::unit_mass(&u);
        let abs = |x: &cfe::ratio::Ratio| {
            if *x < cfe::ratio::ratio(0, 1) {
                -x.clone()
            } else {
                x.clone()
            }
        };
        let e_of = |b: &pulse::Body| {
            if b.ell.is_zero() {
                cfe::ratio::ratio(0, 1)
            } else {
                abs(&b.ell_orth) / abs(&b.ell)
            }
        };
        let read = |bodies: &[&pulse::Body]| -> String {
            let swung: Vec<&&pulse::Body> =
                bodies.iter().filter(|b| !b.ell_orth.is_zero()).collect();
            let most = bodies
                .iter()
                .map(|b| e_of(b))
                .max()
                .unwrap_or_else(|| cfe::ratio::ratio(0, 1));
            let held = bodies
                .iter()
                .fold(cfe::ratio::ratio(0, 1), |s, b| s + pulse::mass(&b.total()));
            let mean = if held.is_zero() {
                cfe::ratio::ratio(0, 1)
            } else {
                bodies.iter().fold(cfe::ratio::ratio(0, 1), |s, b| {
                    s + pulse::mass(&b.total()) * e_of(b)
                }) / held
            };
            format!(
                "{} of {} swinging, greatest e {}, mean e {}",
                swung.len(),
                bodies.len(),
                conductor::reading::sector(&most),
                conductor::reading::sector(&mean)
            )
        };
        let mut orth_spin = cfe::ratio::ratio(0, 1);
        for core in &u.cores {
            let stars: Vec<&pulse::Body> = core.satellites.iter().collect();
            let worlds: Vec<&pulse::Body> = core
                .satellites
                .iter()
                .flat_map(|s| s.satellites.iter())
                .collect();
            for b in stars.iter().chain(worlds.iter()) {
                orth_spin += pulse::mass(&b.total()) / unit.clone() * b.ell_orth.clone();
            }
            println!(
                "swing {:?} — stars: {}; worlds: {}",
                core.path,
                read(&stars),
                read(&worlds)
            );
        }
        println!(
            "swing — the bodies' orthogonal spin on the ledger, Σ m·ℓ_⊥ = {}",
            conductor::reading::sector(&orth_spin)
        );
    }
    // The fuel (issue #119): where hydrogen stands at the end, read from the
    // universe's own state: the stars' kept matter, the galaxies' flows and
    // throats, the ocean; and each star's lightest kept element.
    {
        let h = conductor::chem::Kind::atom(1).expect("hydrogen");
        let he = conductor::chem::Kind::atom(2).expect("weight 2");
        let of = |m: &pulse::Manifest, k: &conductor::chem::Kind| {
            m.get(k).cloned().unwrap_or_else(|| cfe::ratio::ratio(0, 1))
        };
        let (mut kept, mut flows, mut throats, mut stars, mut holding_h) = (
            cfe::ratio::ratio(0, 1),
            cfe::ratio::ratio(0, 1),
            cfe::ratio::ratio(0, 1),
            0usize,
            0usize,
        );
        let mut lightest: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
        let mut kept_he = cfe::ratio::ratio(0, 1);
        let mut held_by: Vec<String> = Vec::new();
        for core in &u.cores {
            throats += of(&core.throat, &h);
            flows += core
                .flows
                .iter()
                .filter(|x| x.key == h)
                .fold(cfe::ratio::ratio(0, 1), |a, x| a + x.amount.clone());
            for s in &core.satellites {
                stars += 1;
                kept += of(&s.kept, &h);
                kept_he += of(&s.kept, &he);
                if of(&s.kept, &h) > cfe::ratio::ratio(0, 1) {
                    holding_h += 1;
                    held_by.push(format!(
                        "{:?} remnant {} born {} hydrogen {} of kept {}",
                        s.path,
                        s.remnant,
                        s.born,
                        conductor::reading::sector(&of(&s.kept, &h)),
                        conductor::reading::sector(&pulse::mass(&s.kept))
                    ));
                }
                if let Some(z) = s.kept.keys().filter_map(conductor::chem::atom_w).min() {
                    *lightest.entry(z).or_insert(0) += 1;
                }
            }
        }
        let ocean = u
            .ocean
            .iter()
            .filter(|x| x.key == h)
            .fold(cfe::ratio::ratio(0, 1), |a, x| a + x.amount.clone());
        let f = |x: &cfe::ratio::Ratio| conductor::reading::sector(x).to_string();
        println!(
            "fuel — weight 1, hydrogen (grains): the stars keep {} ({} of {} stars hold any; weight 2 kept {}), the galaxies' flows {}, their throats {}, the ocean {}; the stars' lightest kept kind, by weight: {:?}",
            f(&kept),
            holding_h,
            stars,
            f(&kept_he),
            f(&flows),
            f(&throats),
            f(&ocean),
            lightest
        );
        for line in &held_by {
            println!("fuel — a star holding hydrogen: {line}");
        }
        // Each birth's own record: hydrogen's share by mass of what fell, of
        // it after the fibres' transition, and of what the star keeps.
        let births: Vec<&(cfe::ratio::Ratio, cfe::ratio::Ratio, cfe::ratio::Ratio)> =
            events.iter().flat_map(|e| e.births_h.iter()).collect();
        let zero = cfe::ratio::ratio(0, 1);
        let none = |k: usize| births.iter().filter(|b| [&b.0, &b.1, &b.2][k] == &zero).count();
        let most = |k: usize| births.iter().map(|b| [&b.0, &b.1, &b.2][k].clone()).max().unwrap_or_else(|| zero.clone());
        println!(
            "fuel — births: {}; hydrogen absent from what fell in {}, after the fibres' transition in {}, from the star kept in {}; the greatest hydrogen share by mass, fell {}, after the transition {}, kept {}",
            births.len(),
            none(0),
            none(1),
            none(2),
            f(&most(0)),
            f(&most(1)),
            f(&most(2))
        );
    }
    // The joint READ (concordance 2.10, the landing rule): each star's burn
    // as the form holds it, its runs `(1 − x_i, n_i)` on its own clock in the
    // order lived, and the reading of what remains on the 60⁻⁶ grain; written
    // whole to `<var>/runs.txt`.
    {
        let mut bodies: std::collections::BTreeMap<
            (Vec<u64>, u32),
            conductor::compound::Remaining,
        > = std::collections::BTreeMap::new();
        let mut refused = 0usize;
        for e in &events {
            for (path, z, x, n) in &e.burns {
                let was = bodies
                    .remove(&(path.clone(), *z))
                    .unwrap_or_else(conductor::compound::Remaining::whole);
                match was.clone().after(x, *n) {
                    Ok(r) => {
                        bodies.insert((path.clone(), *z), r);
                    }
                    Err(_) => {
                        refused += 1;
                        bodies.insert((path.clone(), *z), was);
                    }
                }
            }
        }
        let mut text = String::new();
        let mut runs = 0usize;
        for ((path, z), r) in &bodies {
            runs += r.factors().len();
            let factors: Vec<String> = r
                .factors()
                .iter()
                .map(|(b, n)| format!("({b}, {n})"))
                .collect();
            // Joint reads compare exact values (Mycelium, 2026-10-21: no grid):
            // the runs are written exactly; their product is read off them.
            text.push_str(&format!(
                "star {path:?} Z{z}: runs [{}]\n",
                factors.join(", ")
            ));
        }
        let _ = std::fs::write(Path::new(era1).join("runs.txt"), text);
        println!("joint read — {} burning bodies (star, rung), {runs} runs on their own clocks, {refused} refused; written {era1}/runs.txt", bodies.len());
    }
    if args.get(4).map(String::as_str) == Some("write") {
        let _ = pulse::write(Path::new(era1), &seed, &u);
        let _ = std::fs::write(Path::new(era1).join("conductor.beat"), until.to_string());
    }
}

/// One energy line (docs/122 step 1): each form's change between two books,
/// beside the light made, received and drained between them. Heat's change
/// less the light received is the heat made or lost by every other channel
/// (the torque, encounters, rounding, cooling), none of it booked yet; the
/// bond closed beside the light made shows how much starlight its bonds
/// account for. The forms do not share one unit yet (docs/124 §8), so no
/// sum is claimed to close.
fn energy_line(
    when: &str,
    a: &conductor::energy::Book,
    b: &conductor::energy::Book,
    made: &cfe::ratio::Ratio,
    got: &cfe::ratio::Ratio,
    gone: &cfe::ratio::Ratio,
) -> String {
    let s = conductor::reading::sector;
    let dh = b.heat() - a.heat();
    format!(
        "energy {when}: heat {} (motes {}, stars {}, worlds {}), change {}, less light received {}; potential {}, change {}; bond {}, closed {}; light made {}, received {}, drained {}",
        s(&b.heat()),
        s(&b.heat_motes),
        s(&b.heat_stars),
        s(&b.heat_worlds),
        s(&dh),
        s(&(dh.clone() - got.clone())),
        s(&b.potential),
        s(&(b.potential.clone() - a.potential.clone())),
        s(&b.bond),
        s(&(a.bond.clone() - b.bond.clone())),
        s(made),
        s(got),
        s(gone),
    )
}
