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
    let spin_before = pulse::spin_total(&u, &breath::unit_mass(&u));
    let mut events = Vec::new();
    // THE CARRIER'S GROWTH, read as the run goes (operator 2026-10-23: "if its
    // growing uncontrolled and there is no end in sight, what is our course of
    // action?"): each tenth universal echo, the motes held and the widest spin
    // and phase in bits, counted, never timed (MU-RECURSION rule 13). A chunk
    // that more than doubles the motes of the one before stops the run: the
    // growth is the reading, and what was read so far is reported.
    let bits = |r: &cfe::ratio::Ratio| r.numer().bits().max(r.denom().bits());
    let carrier = |u: &pulse::Universe| -> (usize, u64, usize) {
        let mut n = u.firmament.len();
        let mut in_systems = 0usize;
        let mut widest = 0u64;
        let mut see = |x: &conductor::flows::Mote| {
            widest = widest.max(bits(&x.ell_o)).max(bits(&x.ell_orth));
            for p in &x.phase {
                widest = widest.max(bits(p));
            }
        };
        u.firmament.iter().for_each(&mut see);
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
    // EVERY STORED FIELD'S WIDTH (the scalar-collapse audit: a value stored
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
            for (n, r) in [("ell", &b.ell), ("ell_orth", &b.ell_orth), ("radius", &b.radius), ("period", &b.period), ("medium_spin", &b.medium_spin), ("state", &b.state), ("reservoir", &b.reservoir), ("turning", &b.turning), ("load", &b.load), ("heat", &b.heat)] {
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
                for (n, r) in [("amount", &x.amount), ("ell_o", &x.ell_o), ("ell_orth", &x.ell_orth), ("agitation", &x.agitation), ("lever_place", &x.lever_place)] {
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
        for x in &u.firmament {
            put("firmament.ell_o".into(), &x.ell_o);
            put("firmament.amount".into(), &x.amount);
        }
        for a in u.medium.values() {
            put("universe.medium".into(), a);
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
    let show = |w: &std::collections::BTreeMap<String, u64>| w.iter().map(|(k, b)| format!("{k} {b}")).collect::<Vec<_>>().join(", ");
    eprintln!("FIELDS at bell {start}: {}", show(&fields(&u)));
    let mut grown_past: Option<u64> = None;
    let mut last_motes = carrier(&u).0;
    for b in start..until {
        events.extend(breath::breathe(&mut u, &seed, t_u, b, b + 1));
        // Progress by the universe's own count, never a clock: each tenth
        // universal echo.
        if t_u > 0 && (b + 1) % (10 * t_u) == 0 {
            let (motes, widest, in_systems) = carrier(&u);
            let (rung, read) = events.iter().fold((0usize, 0usize), |a, e| (a.0 + e.rang.0, a.1 + e.rang.1));
            eprintln!("bell {}: {} echoes read; {motes} motes ({in_systems} in the systems), widest {widest} bits; star bells rang {rung} of {read} star-echoes", b + 1, events.len());
            eprintln!("FIELDS at bell {}: {}", b + 1, show(&fields(&u)));
            if last_motes > 0 && motes > 2 * last_motes {
                grown_past = Some(b + 1);
                break;
            }
            last_motes = motes;
        }
    }
    if let Some(at) = grown_past {
        println!("GROWTH — the motes more than doubled in the chunk ending at bell {at}: the run stopped there");
    }
    let until = grown_past.unwrap_or(until);
    let unit = breath::unit_mass(&u);
    let births: usize = events.iter().map(|e| e.stars).sum();
    let dissolved: usize = events.iter().map(|e| e.dissolved).sum();
    let returned = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.returned.clone());
    println!("STARS MET — {} merged, {} bounced", events.iter().map(|e| e.merged).sum::<usize>(), events.iter().map(|e| e.encounters).sum::<usize>());
    {
        let sticks: usize = events.iter().map(|e| e.grown.0).sum();
        let bounces: usize = events.iter().map(|e| e.grown.1).sum();
        let escaped = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.escaped.clone());
        let last: std::collections::BTreeMap<u32, (usize, usize)> = events.iter().map(|e| (e.galaxy, (e.grown.2, e.grown.3))).collect();
        let born = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.discs.0.clone());
        let out = events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.discs.1.clone());
        let (cl, fell) = events.iter().fold((cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1)), |s, e| (s.0 + e.collapse.0.clone(), s.1 + e.collapse.1.clone()));
        println!("INFALL — clumps {} (holding); fell into the hearts {}, the rest their discs", conductor::reading::sector(&cl), conductor::reading::sector(&fell));
        let f = events.iter().fold((0usize, 0usize, 0usize, 0usize), |s, e| (s.0 + e.fates.0, s.1 + e.fates.1, s.2 + e.fates.2, s.3 + e.fates.3));
        println!("GROWN — bodies grown by meetings: {} worlds, {} ignited as stars, {} whose core held nothing, {} that left past the rim", f.0, f.1, f.2, f.3);
        let w = events.iter().fold([cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1)], |mut s, e| { for (a, b) in s.iter_mut().zip(e.weights.iter()) { *a += b.clone(); } s });
        let mean = |m: &cfe::ratio::Ratio, n: &cfe::ratio::Ratio| if n.is_zero() { "none".to_string() } else { conductor::reading::sector(&(m.clone() / n.clone())).to_string() };
        println!("WEIGHT — a nucleus's mean hold: grown groups that left past the rim {}, worlds grown {}", mean(&w[0], &w[1]), mean(&w[2], &w[3]));
        let im = events.iter().fold((0usize, 0usize, 0usize), |s, e| (s.0 + e.impacts.0, s.1 + e.impacts.1, s.2 + e.impacts.2));
        println!("IMPACTS — bounces that struck as matter {}, tugs between neighbouring worlds {}; grown groups carried past the rim by their orthogonal orbit's swing alone {}", im.0, im.1, im.2);
        let ms = events.iter().fold([0usize; 6], |mut a, e| { for i in 0..5 { a[i] += e.motes[i]; } a });
        let held: usize = { let mut last: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new(); for e in &events { last.insert(e.galaxy, e.motes[5]); } last.values().sum() };
        println!("MOTES — entered the galaxies' flows: arrivals {}, shed and through the stars' throats {}, recycling {}, clump members returned {}; folded by the merge {}; held at the end {held}", ms[0], ms[1], ms[2], ms[3], ms[4]);
        println!("DISCS — released at the stars' births {} (holding), at or past the rim when released {}", conductor::reading::sector(&born), conductor::reading::sector(&out));
        println!("SYSTEMS MET — {sticks} stuck, {bounces} bounced; overstepped the rims {} (holding); after the last echo, motes and bodies by galaxy {last:?}", conductor::reading::sector(&escaped));
    }
    println!("BREATH — {} bells, T_u {t_u}: {} echoes (perpendicular writes down); recohered {births} systems born, decohered {} returned and {dissolved} stars dissolved (writes up)", until, events.len(), returned.round());
    for (i, c) in u.cores.iter().enumerate() {
        let echoes = events.iter().filter(|e| u64::from(e.galaxy) == c.path[0]).count();
        let born = c.satellites.len();
        let orbs: usize = c.satellites.iter().map(|s| s.satellites.len()).sum();
        let year = motion::galactic_year(c).map(|(k, t)| format!("half-mass system {}, year {} bells", k + 1, t.period.round())).unwrap_or_else(|| "no year yet".into());
        let medium = pulse::mass(&c.medium) / unit.clone();
        match breath::galaxy_rate(&u, i, &unit) {
            Some(g_rate) => {
                let aeon = g_rate.clone() * cfe::ratio::ratio(until as i64, 1);
                let rungs = breath::rungs(&breath::slip(&g_rate, until, t_u)).map(|r| r.to_string()).unwrap_or_else(|| "unread".into());
                let coh = breath::coherence(&breath::slip(&g_rate, until, t_u));
                println!("  galaxy {:?} at {} bells out: {echoes} echoes, {rungs} rungs (recoherences), coherence now {}%, aeon {} bells (rate {}); {born} systems, {orbs} orbs; medium left {}; {year}", c.path, c.radius, (coh * cfe::ratio::ratio(100, 1)).round(), aeon.round(), conductor::reading::sector(&g_rate), medium.round());
            }
            None => {
                // The home's bodies read their own clocks: the spread of its
                // systems' rates against the source.
                let rates: Vec<cfe::ratio::Ratio> = (0..c.satellites.len()).map(|s| breath::system_clock(&u, i, s, &unit)).collect();
                let span = match (rates.iter().min(), rates.iter().max()) {
                    (Some(a), Some(b)) => format!("{}..{}", conductor::reading::sector(a), conductor::reading::sector(b)),
                    _ => "none".into(),
                };
                println!("  the HOME {:?} at the source: {echoes} echoes; no clock of its own, its {born} systems' clocks {span}; {orbs} orbs; medium left {}; {year}", c.path, medium.round());
            }
        }
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
    // THE CARRIER'S SIZE (design law 8; the cut-over's measure): the widest
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
        seen(&u.medium, &mut amounts);
        let mut motes: Vec<&conductor::flows::Mote> = u.firmament.iter().collect();
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
            spins = spins.max(bits(&x.ell_o)).max(bits(&x.ell_orth));
            agitation = agitation.max(bits(&x.agitation));
            for p in &x.phase {
                phases = phases.max(bits(p));
            }
        }
        println!("CARRIER — widest exact value held, in bits: amounts {amounts}, spins and reaches {spins}, phases {phases}, agitation {agitation}; {} motes", motes.len());
    }
    // THE CROSS (the eight pivots): every residue's fate over the run, by
    // fate, every plane together.
    {
        let mut all = conductor::enfold::Cross::default();
        for e in &events {
            all.join(&e.cross);
        }
        let row: Vec<String> = all.counts().iter().map(|(f, n)| format!("{f:?} {n}")).collect();
        println!("CROSS — residues by fate over the run: {}", if row.is_empty() { "none".into() } else { row.join(", ") });
    }
    // THE LIGHT LEDGER: light made (the ℚ → ℤ transduction's flux), received
    // as heat by the bodies on its continuum face, drained into the vacuum.
    {
        let sum = |f: &dyn Fn(&breath::Event) -> cfe::ratio::Ratio| events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + f(e));
        let (made, got, gone) = (sum(&|e| e.light.made.clone()), sum(&|e| e.light.received.clone()), sum(&|e| e.light.drained.clone()));
        println!("LIGHT — made {}, received {} by the bodies on the continuum face, drained {} into the vacuum (units); balanced: {}", conductor::reading::sector(&made), conductor::reading::sector(&got), conductor::reading::sector(&gone), made == got.clone() + gone.clone());
    }
    // THE TOLLS: every star that passed, struck on its own bell.
    {
        let tolls: Vec<&breath::Toll> = events.iter().flat_map(|e| e.tolls.iter()).collect();
        println!("TOLLS — {} stars passed", tolls.len());
        for t in tolls {
            let heard: Vec<String> = (0..u.cores.len()).filter_map(|g| t.heard_after(&u, g).map(|d| format!("{:?} after {}", u.cores[g].path, conductor::reading::sector(&d)))).collect();
            println!("  star {:?} at bell {}: {:?}, {} strokes, returned {} holding and {} spin; heard by {}", t.star, t.bell, t.passing, t.strokes.map(|n| n.to_string()).unwrap_or_else(|| "uncounted".into()), conductor::reading::sector(&t.returned), conductor::reading::sector(&t.spin), heard.join(", "));
        }
    }
    // THE SPIN IDENTITIES (docs/90): each check's residual, `after − before`
    // of its ledger's whole sum, exact where 0.
    {
        let mut kinds: std::collections::BTreeMap<&str, (u64, u64, cfe::ratio::Ratio)> = std::collections::BTreeMap::new();
        for (kind, r) in events.iter().flat_map(|e| e.identity.iter()) {
            let abs = if *r < cfe::ratio::ratio(0, 1) { -r.clone() } else { r.clone() };
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
            println!("IDENTITY {kind} — {exact} of {n} exact; greatest residual {}", conductor::reading::sector(worst));
        }
        let spin_after = pulse::spin_total(&u, &breath::unit_mass(&u));
        println!("IDENTITY ledger — the bodies' spin (both orbits) and media, before {} after {}: moved {} (motes and reservoirs are outside it)", conductor::reading::sector(&spin_before), conductor::reading::sector(&spin_after), conductor::reading::sector(&(spin_after.clone() - spin_before.clone())));
    }
    use num_traits::Zero as _;
    // THE SWING (docs/90): each galaxy's stars' and worlds' elongations, their
    // orthogonal spin over their orbit spin, `e = |ℓ_⊥|/ℓ` (the reach swings over
    // `R(1 ± e)` each orthogonal turn): how many carry one, their greatest and their
    // holding-weighted mean; and the orthogonal spin the ledger holds (Σ m·ℓ_⊥).
    {
        let unit = breath::unit_mass(&u);
        let abs = |x: &cfe::ratio::Ratio| if *x < cfe::ratio::ratio(0, 1) { -x.clone() } else { x.clone() };
        let e_of = |b: &pulse::Body| if b.ell.is_zero() { cfe::ratio::ratio(0, 1) } else { abs(&b.ell_orth) / abs(&b.ell) };
        let read = |bodies: &[&pulse::Body]| -> String {
            let swung: Vec<&&pulse::Body> = bodies.iter().filter(|b| !b.ell_orth.is_zero()).collect();
            let most = bodies.iter().map(|b| e_of(b)).max().unwrap_or_else(|| cfe::ratio::ratio(0, 1));
            let held = bodies.iter().fold(cfe::ratio::ratio(0, 1), |s, b| s + pulse::mass(&b.total()));
            let mean = if held.is_zero() { cfe::ratio::ratio(0, 1) } else { bodies.iter().fold(cfe::ratio::ratio(0, 1), |s, b| s + pulse::mass(&b.total()) * e_of(b)) / held };
            format!("{} of {} swinging, greatest e {}, mean e {}", swung.len(), bodies.len(), conductor::reading::sector(&most), conductor::reading::sector(&mean))
        };
        let mut orth_spin = cfe::ratio::ratio(0, 1);
        for core in &u.cores {
            let stars: Vec<&pulse::Body> = core.satellites.iter().collect();
            let worlds: Vec<&pulse::Body> = core.satellites.iter().flat_map(|s| s.satellites.iter()).collect();
            for b in stars.iter().chain(worlds.iter()) {
                orth_spin += pulse::mass(&b.total()) / unit.clone() * b.ell_orth.clone();
            }
            println!("SWING {:?} — stars: {}; worlds: {}", core.path, read(&stars), read(&worlds));
        }
        println!("SWING — the bodies' orthogonal spin on the ledger, Σ m·ℓ_⊥ = {}", conductor::reading::sector(&orth_spin));
    }
    // THE JOINT READ (CONCORDANCE 2.10, the landing rule): each star's burn
    // as the form holds it, its runs `(1 − x_i, n_i)` on its own clock in the
    // order lived, and the reading of what remains on the 60⁻⁶ grain; written
    // whole to `<var>/runs.txt`.
    {
        let mut bodies: std::collections::BTreeMap<(Vec<u64>, u32), conductor::compound::Remaining> = std::collections::BTreeMap::new();
        let mut refused = 0usize;
        for e in &events {
            for (path, z, x, n) in &e.burns {
                let was = bodies.remove(&(path.clone(), *z)).unwrap_or_else(conductor::compound::Remaining::whole);
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
            let factors: Vec<String> = r.factors().iter().map(|(b, n)| format!("({b}, {n})")).collect();
            // Joint reads compare EXACT values (mycelium, 2026-10-21: no grid):
            // the runs are written exactly; their product is read off them.
            text.push_str(&format!("star {path:?} Z{z}: runs [{}]\n", factors.join(", ")));
        }
        let _ = std::fs::write(Path::new(era1).join("runs.txt"), text);
        println!("JOINT READ — {} burning bodies (star, rung), {runs} runs on their own clocks, {refused} refused; written {era1}/runs.txt", bodies.len());
    }
    if args.get(4).map(String::as_str) == Some("write") {
        let _ = pulse::write(Path::new(era1), &seed, &u);
        let _ = std::fs::write(Path::new(era1).join("conductor.beat"), until.to_string());
    }
}
