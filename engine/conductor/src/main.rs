//! The conductor binary — the board's heartbeat.
//!
//! Usage:
//!   conductor [--var var] [--cycles N] [--cadence SECONDS] [--fold]
//!
//! ERA 1 (docs/78, docs/82): the universe is the pulse's (`<var>/pulse.bin`,
//! deployed by the `pulse` binary), breathed a bell a beat; the original
//! civilizations are brought over as its first peoples, each on its own star
//! system.
//! `--cycles 0` (default) beats forever at the cadence. Watch live:
//!   python3 tools/observer.py --watch 2

use conductor::{observe_crossings, roster, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use std::path::PathBuf;

fn main() {
    let mut var_root = String::from("var");
    let mut cycles: u64 = 0;
    let mut cadence_secs: u64 = 2;
    let mut do_fold = false;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let mut value = || args.next().expect("flag value");
        match flag.as_str() {
            "--var" => var_root = value(),
            "--cycles" => cycles = value().parse().expect("--cycles N"),
            "--cadence" => cadence_secs = value().parse().expect("--cadence SECONDS"),
            "--fold" => do_fold = true,
            other => {
                eprintln!("conductor: unknown flag {other}");
                std::process::exit(2);
            }
        }
    }

    // AEON-1b (11 §5): the FOLD — seal the live era into the archive
    // sequence and stop; the next boot converges it onto the new
    // aeon's G-sheet. Nothing is destroyed.
    if do_fold {
        match conductor::fold(&var_root) {
            Ok((era, gauge)) => {
                println!(
                    "conductor: aeon FOLDED — era {era} sealed into {var_root}-era{era}; the new sheet seats at G = {gauge}"
                );
                return;
            }
            Err(refusal) => {
                eprintln!("conductor: the fold refused — {refusal}");
                std::process::exit(1);
            }
        }
    }
    std::fs::create_dir_all(&var_root).expect("var root");
    let registry = Registry::at(format!("{var_root}/registry"));
    // ERA 1 (docs/82, operator 2026-10-14): the original civilizations and
    // polities are brought over as the universe's first peoples (the
    // compiled founding lines and every registered line), each seated on an
    // era-1 star system below; life also arises by the ladder. The universe
    // itself is the pulse's, deployed beforehand (the `pulse` binary).
    let mut specs = roster(&registry);
    let book = Ledger::at(format!("{var_root}/worlds"));
    let Some((pulse_seed, mut universe)) = conductor::pulse::read(std::path::Path::new(&var_root)) else {
        eprintln!("conductor: no universe at {var_root}/pulse.bin — deploy it first (the pulse binary)");
        std::process::exit(1);
    };
    // THE UNIVERSAL PULSE: TUniversal sets it at founding; 60 bells stands in.
    // One beat is one bell.
    const T_U: u64 = 60;
    let unit = conductor::breath::unit_mass(&universe);
    // The pulse's cores charted as galaxies, and every line seated on its
    // own star system (stand-in places until the sphere, A1).
    match conductor::seating::chart(&universe, &book, 0) {
        Ok(n) => println!("conductor: {n} galaxies charted from the pulse's cores"),
        Err(e) => eprintln!("conductor: the galaxies were not charted: {e}"),
    }
    let era1_worlds: std::collections::BTreeMap<Vec<u32>, conductor::life::World> =
        conductor::life::worlds(&universe).into_iter().map(|w| (w.path.clone(), w)).collect();
    conductor::seating::seat(&universe, &era1_worlds, &mut specs);
    // Motion (era 1): each system's galactic period, which its lines' orbits
    // read; refreshed when the breath changes the universe.
    let mut system_periods = conductor::seating::periods(&universe);
    // LIFE (docs/82 step 3b): every world's climb is state, advanced by the
    // system's activity at its own coupling's recoherences; its reading is
    // refreshed when the universe changes. A mind wakes when its climb is
    // written, and founds a line.
    let mut climbs = conductor::life::read_climbs(std::path::Path::new(&var_root));
    let mut life_readings = conductor::life::readings(&universe, &pulse_seed);
    let mut readings_at: u64 = 0;
    let mut woken_galaxies: std::collections::BTreeSet<u32> =
        climbs.iter().filter(|(_, c)| c.woken).filter_map(|(p, _)| p.first().copied()).collect();
    println!(
        "conductor: life — {} worlds read, {} climbing, {} minds woken",
        life_readings.len(),
        life_readings.values().filter(|r| r.climb.is_some()).count(),
        climbs.values().filter(|c| c.woken).count()
    );
    println!("conductor: the pulse universe — {} cores, {} systems; T_u {T_U} bells", universe.cores.len(), universe.cores.iter().map(|c| c.satellites.len()).sum::<usize>());

    // CONVERGE-1 (07 §1.6): converge the archived eras BEFORE genesis —
    // boot adopts the past instead of founding blind beside it.
    let eras = ledger::converge::default_era_roots(&var_root);
    let converged = ledger::converge::converge(&book, &eras).expect("convergence");
    println!(
        "conductor: converged {} worlds ({} already annexed, {} quarantined) — epoch base {}",
        converged.adopted.len(),
        converged.already,
        converged.quarantined.len(),
        converged.epoch_base,
    );
    for q in &converged.quarantined {
        println!("conductor: QUARANTINED {}/{} (break at {})", q.era, q.world, q.at);
    }
    let epoch_base = converged.epoch_base;

    // 11 §5.2 (wired 2026-07-30): the LIVE aeon seats on its own
    // G-sheet, above every archived one — sheet N/12 with N archived
    // eras (the fold law's placeholder advance). Spec keys ride above
    // the base; without this the live board would stack onto the
    // folded aeons' height and recur.
    let live_base = cfe::world::wrap_gauge(cfe::ratio::ratio(eras.len() as i64, 12));
    // docs/58 §3 — THE BIG BANG, recorded once on this universe's ledger
    // before any world is founded (or, for a universe that predates the law,
    // once and late as its re-founding).
    match book.found_universe(conductor::roster_digest(), cfe::almanac::matter_law_digest(), epoch_base) {
        Ok((seed, true)) => println!(
            "conductor: THE BIG BANG — this universe is {} (recorded now)",
            seed.iter().take(8).map(|b| format!("{b:02x}")).collect::<String>()
        ),
        Ok((seed, false)) => println!(
            "conductor: the universe is {}",
            seed.iter().take(8).map(|b| format!("{b:02x}")).collect::<String>()
        ),
        Err(e) => eprintln!("conductor: the Big Bang was not recorded: {e}"),
    }
    // docs/63 — OBSERVATION: only the observed lines are unrolled; the rest
    // stay compact (their snapshots on disk, their specs here) until an
    // observer arrives.
    let takes = PathBuf::from(std::env::var("NETSTRATUM_HOUSE").unwrap_or_else(|_| "var/uplink/house".into())).join("takes");
    let watch = PathBuf::from(&var_root).join("watch");
    let specs: Vec<conductor::PolitySpec> = specs
        .into_iter()
        .map(|mut spec| {
            let opening = spec
                .gauge
                .take()
                .unwrap_or_else(|| cfe::ratio::ratio(0, 1));
            spec.gauge = Some(cfe::world::wrap_gauge(opening + live_base.clone()));
            spec
        })
        .filter(|spec| !book.world_dir(&spec.world_name).join("collapsed").exists())
        .collect();
    let observed = {
        let refs: Vec<&conductor::PolitySpec> = specs.iter().collect();
        conductor::observe::observed(&refs, &registry, &takes, &watch, &book.universe_span())
    };
    let (awake, mut dormant): (Vec<conductor::PolitySpec>, Vec<conductor::PolitySpec>) =
        specs.into_iter().partition(|spec| observed.contains(&spec.world_name));
    println!("conductor: {} lines observed and unrolled, {} compact", awake.len(), dormant.len());
    let mut worlds: Vec<Polity> = awake.into_iter().map(|spec| Polity::genesis(spec, &registry, &book)).collect();
    // A line resumed already collapsed never re-enters the beat.
    for name in conductor::retire_collapsed(&mut worlds, &book, epoch_base) {
        println!("conductor: {name} resumed collapsed; it stays out of the roster");
    }
    println!(
        "conductor: {} civilizations seated on the pulse's systems — {}",
        worlds.len(),
        if cycles == 0 {
            format!("beating every {cadence_secs}s (Ctrl-C to stop)")
        } else {
            format!("running {cycles} heartbeats")
        }
    );
    // Block-buffered stdout under systemd otherwise sits on this line for
    // minutes (measured 8+ minutes live) — see the flush note further down.
    let _ = std::io::Write::flush(&mut std::io::stdout());

    // OPERATOR CONTROLS (2026-07-30 ruling): `<var>/conductor.ctl`,
    // polled each beat — the player's hand on the autonomous board.
    //   paused          the beats hold (nothing ticks, nothing ages)
    //   cadence N       seconds between beats (0 = flat out)
    //   advance N       PUNCTUATED PLAY (task #11): run N beats flat
    //                   out, then pause — the player owns the clock.
    //                   The countdown lives in the FILE, so a crash
    //                   mid-advance resumes honestly where it stood.
    // The CLI writes it (`npcs pause|resume|speed N|advance N`);
    // hand-editing is equally lawful. Missing/malformed = defaults.
    let control_path = PathBuf::from(&var_root).join("conductor.ctl");
    let read_control = |fallback: u64| -> (bool, u64, Option<u64>) {
        let mut paused = false;
        let mut cadence = fallback;
        let mut advance = None;
        if let Ok(text) = std::fs::read_to_string(&control_path) {
            for line in text.lines() {
                let line = line.trim();
                if line == "paused" {
                    paused = true;
                } else if let Some(word) = line.strip_prefix("cadence ") {
                    if let Ok(n) = word.trim().parse() {
                        cadence = n;
                    }
                } else if let Some(word) = line.strip_prefix("advance ") {
                    if let Ok(n) = word.trim().parse() {
                        advance = Some(n);
                    }
                }
            }
        }
        (paused, cadence, advance)
    };
    // Rewrite ONLY the lines this loop owns; every other line (pen
    // yields etc.) survives verbatim — the CLI's own rewrite law.
    let write_control = |paused: bool, advance: Option<u64>| {
        let mut text = String::new();
        if let Ok(existing) = std::fs::read_to_string(&control_path) {
            for line in existing.lines() {
                let line = line.trim();
                if line == "paused" || line.starts_with("advance ") || line.is_empty() {
                    continue;
                }
                text.push_str(line);
                text.push('\n');
            }
        }
        if paused {
            text.push_str("paused\n");
        }
        if let Some(n) = advance {
            text.push_str(&format!("advance {n}\n"));
        }
        let _ = std::fs::write(&control_path, text);
    };

    let beat_path = PathBuf::from(&var_root).join("conductor.beat");
    let mut beat: u64 = std::fs::read_to_string(&beat_path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    if beat > 0 {
        println!("conductor: resumed beat {beat} from {}", beat_path.display());
    }
    let mut total_cleaves: u64 = 0;
    let mut ancient: Option<(String, u64)> = None;
    let mut last_living: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    // docs/66 Y1: every action any line has closed this process.
    let mut universe_closed: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
    // docs/67 N8 / 78.156: the last final point read, and where the dilation
    // reading starts (beat, h).
    let mut hubs_cleared_at: Option<String> = None;
    // docs/75: the term's completed exchanges between lines, for the realms.
    let mut contacts = conductor::realms::Contacts::default();
    let mut passages = conductor::realms::Passages::default();
    // docs/77: the road tolls each gate earned this term.
    let mut gate_tolls: std::collections::BTreeMap<(String, String), cfe::ratio::Ratio> = std::collections::BTreeMap::new();
    // THE UNIVERSE JOURNAL: the readings below kept in `universe.jrn`, the
    // relay handed off by the file (a resumed conductor writes after the
    // last beat kept).
    let mut annal = conductor::annal::Annal::open(std::path::Path::new(&var_root));
    // CHRONOMETRY (one law with xylarium): the keepers' chain, committed
    // every pulse.
    let mut chain = conductor::chronometry::Chain::open(std::path::Path::new(&var_root));
    let mut dilation_from: Option<(u64, u64)> = None;
    let mut announced_pause = false;
    loop {
        let (paused, cadence, advance) = read_control(cadence_secs);
        // PUNCTUATED PLAY: an advance overrides the pause and runs
        // flat out; zero parks the board paused with its summary.
        let (paused, cadence) = match advance {
            Some(0) => {
                write_control(true, None);
                println!(
                    "conductor: ADVANCE complete — the board pauses; `npcs resume` or `npcs advance N` to continue"
                );
                (true, cadence)
            }
            Some(n) => {
                write_control(false, Some(n - 1));
                (false, 0)
            }
            None => (paused, cadence),
        };
        if paused {
            if !announced_pause {
                println!("conductor: PAUSED — npcs resume (or edit {}) to continue", control_path.display());
                announced_pause = true;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }
        announced_pause = false;
        beat += 1;
        let _ = std::fs::write(&beat_path, format!("{beat}\n"));
        // TUniversal T = epoch base + this era's beat (07 §1.3): live
        // ticks stack on the converged timeline, never restart it.
        let _ = book.set_universal_clock(epoch_base + beat);
        // THE BREATH (docs/78): the echoes arriving this bell compress the
        // media; stars shed by their phase; the clocks say what comes due.
        let events = conductor::profile::timed("breath", || conductor::breath::breathe(&mut universe, &pulse_seed, T_U, beat - 1, beat));
        let mut breath_reading = conductor::annal::BreathReading {
            echoes: events.len() as u64,
            births: events.iter().filter(|e| e.born.is_some()).count() as u64,
            returned: events.iter().fold(cfe::ratio::ratio(0, 1), |s, e| s + e.returned.clone()),
            dissolved: events.iter().map(|e| e.dissolved as u64).sum(),
            ..Default::default()
        };
        if !events.is_empty() {
            let _ = conductor::pulse::write(std::path::Path::new(&var_root), &pulse_seed, &universe);
            println!(
                "beat {beat}: BREATH — {} echoes, {} systems born, {} returned to the media, {} stars dissolved",
                breath_reading.echoes, breath_reading.births, breath_reading.returned.clone().round(), breath_reading.dissolved
            );
        }
        // The readings follow the universe: refreshed when the breath changed
        // it, at most once a pulse.
        if !events.is_empty() && beat >= readings_at + T_U {
            life_readings = conductor::profile::timed("life:read", || conductor::life::readings(&universe, &pulse_seed));
            system_periods = conductor::seating::periods(&universe);
            readings_at = beat;
        }
        let (woke, measured) = conductor::profile::timed("life:climb", || conductor::life::advance(&life_readings, &mut climbs, T_U, beat));
        breath_reading.measurements += measured;
        if beat.is_multiple_of(10) || !woke.is_empty() {
            conductor::life::write_climbs(std::path::Path::new(&var_root), &climbs);
        }
        for w in woke {
            let galaxy = w.path.first().copied().unwrap_or(0);
            let first = woken_galaxies.insert(galaxy);
            let world = life_readings.get(&w.path).map(|r| &r.world);
            if let Some(spec) = conductor::waking::found(&universe, &pulse_seed, &w, &registry, first, world) {
                println!(
                    "beat {beat}: A MIND WAKES — {} ({}, {} medium) on world {:?}{}",
                    spec.civilization.name,
                    spec.civilization.species,
                    w.medium,
                    w.path,
                    spec.civilization.station.as_deref().map(|s| format!("; the {s} of its galaxy")).unwrap_or_default()
                );
                breath_reading.wakings += 1;
                worlds.push(Polity::genesis(spec, &registry, &book));
            }
        }
        conductor::profile::timed("clocks", || {
            for i in 0..universe.cores.len() {
                breath_reading.terms += conductor::clocks::galaxy_terms(&universe, i, T_U, beat - 1, beat);
                breath_reading.epochs += conductor::clocks::galaxy_epochs(&universe, i, &unit, T_U, beat - 1, beat);
            }
            breath_reading.local_years = universe
                .cores
                .iter()
                .flat_map(|c| &c.satellites)
                .flat_map(|s| &s.satellites)
                .map(|o| conductor::clocks::local_years(&o.period, beat - 1, beat))
                .sum();
        });
        // docs/63 — every 10 beats, the observed set: an unobserved line is
        // compacted (carried and unloaded), an observed compact line unrolled.
        if beat.is_multiple_of(10) {
            let observed = {
                let refs: Vec<&conductor::PolitySpec> = worlds.iter().map(|p| &p.spec).chain(dormant.iter()).collect();
                conductor::observe::observed(&refs, &registry, &takes, &watch, &book.universe_span())
            };
            let mut at = 0;
            while at < worlds.len() {
                if !worlds[at].penned && !observed.contains(&worlds[at].spec.world_name) {
                    let spec = worlds.remove(at).compact(&book);
                    println!("beat {beat}: COMPACT — {} has no observer; its state carried, its world unloaded", spec.world_name);
                    dormant.push(spec);
                } else {
                    at += 1;
                }
            }
            let (wake, still): (Vec<_>, Vec<_>) = dormant.into_iter().partition(|spec| observed.contains(&spec.world_name));
            dormant = still;
            for spec in wake {
                println!("beat {beat}: UNROLLED — {} is observed; its world runs", spec.world_name);
                worlds.push(Polity::genesis(spec, &registry, &book));
            }
        }
        // docs/61 G1: every line's place at this beat, on its orbit.
        conductor::orbit_step(&mut worlds, &book, &system_periods);
        // MESH-1b (#2): the pen door — yields seal at the beat
        // boundary; penned worlds wait, and the file poll notices
        // the pen coming home.
        for line in conductor::profile::timed("pen_service", || conductor::pen_service(
            &mut worlds, &registry, &book, &control_path, epoch_base + beat,
        )) {
            println!("beat {beat}: {line}");
        }
        // COG-DIRECTION Slice 3b: the population cohesion census BEFORE the cycle
        // loop, so each line reads its direction relative to the live cohort.
        conductor::profile::timed("direction_census_step", || {
            conductor::direction_census_step(&mut worlds)
        });
        // BEAT-PARALLEL (#152) — the per-civ cognitive phase is INDEPENDENT: each
        // line reads its own tasks/surplus/caste-census and mutates only its own
        // world. `book` is per-world filesystem (race-free across worlds),
        // `registry.seat` is lock-guarded, and the membrane/worth/profile statics
        // are Mutex-guarded — so it fans out across threads over disjoint slices.
        // The mesh-level settlement/trade/consilience steps BELOW stay sequential
        // (they need the whole roster). Order is preserved (chunks joined in spawn
        // order = world order), so newborn cleaves and replay stay deterministic.
        let newborns: Vec<conductor::PolitySpec> = {
            let threads = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
                .min(16);
            let book = &book;
            let registry = &registry;
            // BEAT-PARALLEL follow-up: balance by MEASURED cost (`spent +
            // swept` — the cognitive-attempt counters each line already
            // tracks), not civ count. Equal-count chunking let one heavy
            // civ dominate its thread while lighter civs on other threads
            // sat idle. Greedy LPT bin-packing (heaviest civ first, always
            // into the currently lightest bucket) approximates a balanced
            // partition. Determinism no longer rides on spawn/finish order:
            // each result carries its ORIGINAL index, and results are
            // sorted back into world order before use — the same guarantee
            // the old chunk-order trick gave, robust to reassignment.
            // docs/71 M1: each line's share of the universe's memory, by wealth.
            conductor::assign_memory(&mut worlds);
            let mut indexed: Vec<(usize, &mut conductor::Polity)> =
                worlds.iter_mut().enumerate().collect();
            indexed.sort_by_key(|(_, p)| std::cmp::Reverse(p.spent + p.swept));
            let mut buckets: Vec<Vec<(usize, &mut conductor::Polity)>> =
                (0..threads).map(|_| Vec::new()).collect();
            let mut bucket_cost: Vec<u64> = vec![0; threads];
            for (idx, polity) in indexed {
                let lightest = bucket_cost
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, &c)| c)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                bucket_cost[lightest] += polity.spent + polity.swept;
                buckets[lightest].push((idx, polity));
            }
            let mut results: Vec<(usize, conductor::PolitySpec)> = std::thread::scope(|s| {
                let handles: Vec<_> = buckets
                    .into_iter()
                    .map(|bucket| {
                        s.spawn(move || {
                            let mut local = Vec::new();
                            for (idx, polity) in bucket {
                                if polity.penned {
                                    continue;
                                }
                                if let Some(spec) = polity.cycle(book, registry) {
                                    local.push((idx, spec));
                                }
                            }
                            local
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .flat_map(|h| h.join().unwrap_or_default())
                    .collect()
            });
            results.sort_by_key(|(idx, _)| *idx);
            results.into_iter().map(|(_, spec)| spec).collect()
        };
        // docs/66 Y1: the universe's closed actions, merged after the beat and
        // shared read-only with every line for the next.
        {
            let mut fresh = 0;
            for polity in worlds.iter_mut() {
                for orbit in polity.fresh_closed.drain(..) {
                    fresh += usize::from(universe_closed.insert(orbit));
                }
            }
            if fresh > 0 || beat == 1 {
                let shared = std::sync::Arc::new(universe_closed.clone());
                for polity in worlds.iter_mut() {
                    polity.universe_closed = shared.clone();
                }
            }
        }
        // DIRECTIVE 19: the gate speaks on the board — SEQUENTIAL after the parallel
        // phase, so verdicts are not interleaved and each line drains once. The
        // deed still STANDS in the chronicle (`replay` is the live transition until
        // the Phase 3 flip); this reports a gate overruled by a proposer.
        for polity in &mut worlds {
            if polity.penned {
                continue;
            }
            let world_name = polity.spec.world_name.clone();
            for (sequence, unlawful) in polity.unreported_refusals() {
                println!(
                    "beat {beat}: THE JUDGE REFUSES — {world_name}: deed {sequence} is unlawful ({unlawful:?}); the deed stands, the verdict is on record"
                );
            }
        }
        // THE ONE CLOSURE'S EXIT: a line that collapsed this cycle leaves the
        // roster now, its chronicle synced whole, before any cross-world step
        // can name it as a party.
        for name in conductor::retire_collapsed(&mut worlds, &book, epoch_base + beat) {
            println!("beat {beat}: COLLAPSE — {name} has no people left; it leaves the roster, its chronicle kept whole");
        }
        // THE LARGEST LIVING LINEAGE, read live and announced when it
        // changes: the progenitor candidate (an ancient arises only at a core).
        let living: std::collections::BTreeSet<String> =
            worlds.iter().map(|p| p.spec.civilization.sigil.clone()).collect();
        // The lineage reads the whole registry from disk: only when the
        // living set changed (a birth or a death), never on a quiet beat.
        let now_ancient = if living == last_living { ancient.clone() } else { conductor::largest_lineage(&registry, &living) };
        last_living = living.clone();
        if now_ancient.as_ref().map(|(s, _)| s) != ancient.as_ref().map(|(s, _)| s) {
            match &now_ancient {
                Some((sigil, tree)) => println!(
                    "beat {beat}: THE PROGENITOR CANDIDATE — {sigil}'s line holds {tree} of {} living lines (an ancient arises only at a core)",
                    living.len()
                ),
                None => println!("beat {beat}: no progenitor candidate — no line's living tree is the strict largest"),
            }
        }
        ancient = now_ancient;
        total_cleaves += newborns.len() as u64;
        for spec in newborns {
            println!(
                "beat {beat}: DIVERGENCE — {} ({}) cleaves from the line of {}, settling {}/{}",
                spec.civilization.name,
                spec.civilization.species,
                spec.civilization.lineage.first().map(String::as_str).unwrap_or("?"),
                spec.world_name,
                spec.planet,
            );
            worlds.push(Polity::genesis(spec, &registry, &book));
        }
        // FLEET-1: salvage/mint/assign before hauls need capacity.
        let (built, released) = conductor::profile::timed("fleet_step", || conductor::fleet_step(&mut worlds));
        if released > 0 {
            println!("beat {beat}: FLEET — {released} hulls return to the idle pool from lapsed routes");
        }
        // CORES SLICE 3: the core's bell delivers the dead's words that are due.
        let heard = conductor::inheritance_step(&mut worlds, epoch_base + beat);
        if heard > 0 {
            println!("beat {beat}: {heard} words of the dead reach the living");
        }
        // The universe's carried span: every distance below wraps on it.
        let span = book.universe_span();
        // OFFER-1a: the market beat — intents post, crossings clear
        // (one aperture pair each way, receipted both sides).
        for (seller, buyer) in conductor::profile::timed("market_step", || conductor::market_step(&mut worlds, beat, &span)) {
            contacts.add(&seller, &buyer);
            println!(
                "beat {beat}: THE CROSSING CLEARS — {seller} ⇌ {buyer}; the autarkies trade"
            );
        }
        // MERCHANT-1: the caravan bridges what the void walled.
        for (m, a, b) in conductor::profile::timed("merchant_step", || conductor::merchant_step(&mut worlds, beat, &span)) {
            contacts.add(&m, &a);
            contacts.add(&m, &b);
            println!(
                "beat {beat}: THE CARAVAN — {m} bridges {a} ⇌ {b}; the walled trade flows"
            );
        }
        // E3.3: pure NPC element aperture hauls (Wave-2).
        for (from, to) in conductor::profile::timed("element_trade_step", || conductor::element_trade_step(&mut worlds, beat, &span)) {
            contacts.add(&from, &to);
            println!(
                "beat {beat}: ELEMENT CROSSING — {from} → {to}; open matter rides the void"
            );
        }
        // STAMP MM-1: mint + spot clear (Wave-4).
        // One line per beat, not one per clear (it was ~90 lines a second).
        // docs/67 N8, as 78.156 binds it: the world runs FREE on its own axis
        // u (the beat) at its own pace; TUniversal is met only through ledger
        // entries. The hubs clear once a beat of u. The final point the seat
        // relays is read for what is bound to the fleet (substratum origins,
        // percepts) and for the dilation reading, du/dh, never a fault.
        let point = std::fs::read_to_string(format!("{var_root}/tuniversal")).ok().map(|p| p.trim().to_string());
        if beat.is_multiple_of(T_U) {
            let heard = point.as_deref().and_then(|p| p.split('/').nth(1)).and_then(|l| l.split(':').next()).and_then(|h| u64::from_str_radix(h, 16).ok());
            match conductor::profile::timed("chronometry", || chain.commit(&universe, beat, heard)) {
                Ok(rims) => {
                    let out_of_phase = rims.iter().filter(|(_, r)| *r != cfe::ratio::ratio(0, 1)).count();
                    println!("beat {beat}: CHRONOMETRY — committed {} entries in all; {} rims, {out_of_phase} with a residual against their mantle; point {:?}", chain.entries, rims.len(), heard);
                }
                Err(e) => println!("beat {beat}: CHRONOMETRY — not committed: {e}"),
            }
        }
        let h_now = point.as_deref().and_then(|p| p.split('/').nth(1)).and_then(|l| l.split(':').next()).and_then(|h| u64::from_str_radix(h, 16).ok());
        if let (Some(h), None) = (h_now, dilation_from) {
            dilation_from = Some((beat, h));
        }
        hubs_cleared_at = point;
        // docs/70 P2, docs/72: the beat's ONE reading of the throats' keepers
        // (G3, the envoys counted) and the mapped gates' roads, for the hubs.
        let keepers_now = conductor::ancients::keepers(&worlds, &book, &registry, std::path::Path::new(&var_root));
        let roads = conductor::ascension::roads(&worlds, &book, &registry, std::path::Path::new(&var_root), &keepers_now);
        let (spots, hubs) = conductor::profile::timed("stamp_market_step", || conductor::stamp_market_step(&mut worlds, &book, std::path::Path::new(&var_root), &keepers_now, &roads, beat, &span, &built));
        let tolls = spots.iter().fold(cfe::ratio::ratio(0, 1), |t, (_, _, toll, _)| t + toll.clone());
        let gated = spots.iter().fold(cfe::ratio::ratio(0, 1), |t, (_, _, _, gate)| t + gate.clone());
        for (line, hub) in &hubs.met {
            contacts.add(line, hub);
        }
        for (line, core) in &hubs.passed {
            passages.add(line, core);
        }
        for (from, core, arrived) in &hubs.gate_tolls {
            *gate_tolls.entry((from.clone(), core.clone())).or_insert_with(|| cfe::ratio::ratio(0, 1)) += arrived.clone();
        }
        let refused: Vec<String> = hubs.refused.iter().map(|(why, n)| format!("{n} {why}")).collect();
        println!(
            "beat {beat}: HUB — {} galaxies cleared at their hubs (STAMP for cargo, one price each); road tolls {tolls} STAMP to the keepers, gate tolls {gated} STAMP to the builders; not cleared: {} alone, {} closed, {} unmet; buyers' STAMP legs: {} carried in part, refused [{}]",
            spots.len(),
            hubs.alone,
            hubs.closed,
            hubs.unmet,
            hubs.partial,
            refused.join(", ")
        );
        // docs/73 DECIDE step 1: the hull decisions captured this beat.
        let d = &hubs.decided;
        println!(
            "beat {beat}: DECIDE hull — {} lines seen, {} lost for want of hull; persistence {} of {} rows this beat, {} of {} over the books' windows; {} books, {} triples",
            d.rows, d.lost, d.correct, d.scored, hubs.persistence.0, hubs.persistence.1, hubs.books.0, hubs.books.1
        );
        if let Err(e) = annal.beat(beat, &hubs, spots.len(), &tolls, &gated, released, &breath_reading) {
            println!("beat {beat}: THE UNIVERSE JOURNAL is not kept this beat: {e}");
        }
        // Concordance 3.9: the councils the house seats, published for it.
        if beat.is_multiple_of(10) {
            let _ = conductor::ancients::write_council(&worlds, std::path::Path::new(&var_root));
        }
        // docs/66 Y2 — COMMISSIONS: one attempt a beat at an open need.
        let commission = conductor::profile::timed("commissions_step", || {
            conductor::commissions::step(&mut worlds, &book, std::path::Path::new(&var_root), beat)
        });
        if let Some((core, admitted)) = &commission.ambiguous {
            println!("beat {beat}: COMMISSION OPEN — {core}: the pairs still admit {admitted} orders; no closure by a tie-break");
        }
        if let Some((core, world, arrived, by, admitted)) = &commission.closed {
            println!("beat {beat}: COMMISSION CLOSED — {world} sites the hub of {core} by its own word ({by}; {admitted} orders admitted); bounty {arrived} STAMP arrived");
        }
        // docs/69 — THE SPACING GUILD in the substrata: the ancients' tubes
        // open, and one task a beat is traversed below.
        let below = conductor::profile::timed("substrata_step", || {
            conductor::substrata::step(&mut worlds, &book, std::path::Path::new(&var_root), beat, epoch_base + beat)
        });
        for (sigil, origin, depth) in &below.opened {
            println!("beat {beat}: A TUBE OPENS — {sigil} opens a substratum {origin} at depth {depth}");
        }
        if let Some((origin, depth, first)) = &below.closed {
            println!("beat {beat}: BELOW — substratum {origin} (depth {depth}) closed {}", if *first { "a first action" } else { "an action again" });
        }
        // CONSILIENCE (38 §5): independent lines that resolved their
        // knowledge to the same invariant crystallize a shared axiom.
        for (world, mass, hardness) in conductor::profile::timed("consilience_step", || conductor::consilience_step(&mut worlds, beat)) {
            println!(
                "beat {beat}: CONSILIENCE — {world} shares a truth {mass} lines reached (hardness {hardness})"
            );
        }
        // THE KNOWLEDGE MARKET (ARC C): a prosperous line licenses a word
        // it lacks from the roster's pool — ideas circulate, not just matter.
        for (world, price) in conductor::profile::timed("knowledge_market_step", || conductor::knowledge_market_step(&mut worlds, beat)) {
            println!(
                "beat {beat}: KNOWLEDGE MARKET — {world} licenses an idea (depth {price}); vocabulary spreads"
            );
        }
        // SPEECH BETWEEN LINES (docs/60 D2): each line hears its nearest
        // neighbour; a word outside its senses is misheard.
        if let Some(line) = conductor::profile::timed("conversation_step", || conductor::conversation_step(&mut worlds, beat, &span)) {
            println!("beat {beat}: {line}");
        }
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — journals + the Terminal Wall regenerate lens-side.
        // THE CLOCKS (docs/78): the galaxies' business closes on their terms
        // (echoes received), not on a shared 60-beat year.
        if breath_reading.terms > 0 {
            // ROUTE-1: the year voice draws lane maintenance — an
            // unpaid lane lapses by receipt and raw space returns.
            let mut year_lines: Vec<String> = Vec::new();
            for polity in worlds.iter_mut().filter(|e| !e.penned) {
                for deal in polity.world.maintain_routes(cfe::world::FIRST_ORB) {
                    year_lines.push(format!("LANE {deal} LAPSES on {} — raw space takes the path back", polity.spec.world_name));
                }
            }
            conductor::profile::timed("year:journals", || {
                for polity in worlds.iter().filter(|e| !e.penned) {
                    let _ = scribe::record_journal(
                        &book,
                        &polity.spec.world_name,
                        &polity.world,
                        &scribe::journal::Voice::of(&polity.spec.civilization),
                    );
                }
            });
            let _ = conductor::profile::timed("year:wall", || scribe::journal::render_wall(&book));
            println!(
                "beat {beat}: year {} journaled — the wall is fresh (var/worlds/wall.html)",
                beat / scribe::journal::YEAR_TICKS
            );
            // THE YEAR'S BOUNCE (cores, ancients, ascension, parliaments,
            // councils): one library function.
            year_lines.extend(conductor::profile::timed("year:bounce", || conductor::year_bounce(&mut worlds, &book, &registry, std::path::Path::new(&var_root), beat, epoch_base)));
            // docs/75: the term's realms, read from its contacts, then a new term.
            year_lines.extend(conductor::profile::timed("year:realms", || conductor::realms::bounce(&worlds, &registry, std::path::Path::new(&var_root), &contacts, &passages, beat / scribe::journal::YEAR_TICKS)));
            contacts.clear();
            passages.clear();
            // docs/77: each gate's account, its term's road tolls added.
            year_lines.extend(conductor::ascension::accounts(std::path::Path::new(&var_root), &gate_tolls));
            gate_tolls.clear();
            for line in &year_lines {
                println!("beat {beat}: {line}");
            }
            if let Err(e) = annal.year(beat, &year_lines) {
                println!("beat {beat}: THE UNIVERSE JOURNAL did not keep the year's close: {e}");
            }
            // MEMORY (operator 2026-10-09): what the allocator holds freed but
            // unreturned is given back at the year's close, and the RSS read
            // before and after says how much of the process was live data.
            {
                let rss = || {
                    std::fs::read_to_string("/proc/self/statm")
                        .ok()
                        .and_then(|t| t.split_whitespace().nth(1).and_then(|p| p.parse::<u64>().ok()))
                        .map(|pages| pages * 4)
                        .unwrap_or(0)
                };
                let before = rss();
                // SAFETY: malloc_trim only returns free heap pages to the system.
                unsafe {
                    libc::malloc_trim(0);
                }
                println!("beat {beat}: MEMORY — rss {} MB before the trim, {} MB after", before / 1024, rss() / 1024);
            }
            // CARRY STATE FORWARD: every world's snapshot at the year's close,
            // so a boot resumes from here, never from genesis.
            for polity in worlds.iter_mut().filter(|e| !e.penned) {
                if let Err(e) = polity.carry(&book) {
                    eprintln!("beat {beat}: {} was not carried forward: {e}", polity.spec.world_name);
                }
            }
            // GALAXY-1b: the observation sweep rides the year voice.
            for galaxy in observe_crossings(&worlds, &registry, &book, epoch_base + beat) {
                println!(
                    "beat {beat}: OBSERVATION — the galaxy {} enters the chart, reach {}",
                    galaxy.name,
                    galaxy.reach
                );
            }
        }
        // THE LIVE CENSUS (operator methodology 2026-08-20): the beat-level
        // signal to deduce, LIVE, whether cognition and the civilizations
        // advanced — tests and pins prove lawfulness, never improvement; that is
        // proven by running the civs and reading the network. Emitted on stderr
        // (line-buffered, reaches the log at once) so the running kernel is
        // judged without a fold. Cognition = the words each line has minted (its
        // own grown vocabulary) and the peers cleaved at mastery; civ health =
        // population, and how many lines the larder still feeds.
        if beat.is_multiple_of(25) {
            use std::io::Write;
            let mut pop: u64 = 0;
            let mut words: usize = 0;
            let mut fed: usize = 0;
            let live: Vec<&Polity> = worlds.iter().filter(|p| !p.penned).collect();
            for polity in &live {
                pop += polity.world.population_of(cfe::world::FIRST_ORB);
                words += polity.owned_count();
                if polity.ration_survives(cfe::world::FIRST_ORB) {
                    fed += 1;
                }
            }
            let n = live.len();
            eprintln!(
                "census beat {beat}: lines={n} compact={} pop={pop} words_minted={words} cleaved={total_cleaves} fed={fed}/{n}",
                dormant.len()
            );
            let mut sizes = [0usize; 9];
            for polity in &live {
                for (sum, n) in sizes.iter_mut().zip(polity.sizes()) {
                    *sum += n;
                }
            }
            eprintln!(
                "census beat {beat}: held tail={} kept={}KB library={} inherited={} owned={} stated={} pending={} minds={} arriving={} closed={} tuniversal={}",
                sizes[0], sizes[1] / 1024, sizes[2], sizes[3], sizes[4], sizes[5], sizes[6], sizes[7], sizes[8], universe_closed.len(),
                hubs_cleared_at.as_deref().and_then(|p| p.split('/').nth(1)).unwrap_or("none")
            );
            // docs/71: the minds' memory against its budget, and the witness alarm.
            let held: usize = live.iter().map(|p| p.memory_held()).sum();
            let hits: u64 = live.iter().map(|p| p.memory_witness_hits).sum();
            eprintln!(
                "census beat {beat}: memory entries={held} budget={} witness={hits}{}",
                conductor::MIND_ENTRY_BUDGET,
                if hits > 0 { " ALARM: a basin met the byte witness" } else { "" }
            );
            // 78.156: the dilation du/dh since the first point read, a reading.
            let h = hubs_cleared_at.as_deref().and_then(|p| p.split('/').nth(1)).and_then(|l| l.split(':').next()).and_then(|h| u64::from_str_radix(h, 16).ok());
            if let (Some((b0, h0)), Some(h)) = (dilation_from, h) {
                eprintln!("census beat {beat}: dilation du/dh = {}/{}", beat - b0, h.saturating_sub(h0));
            }
            let _ = std::io::stderr().flush();
        }
        // THE REFUSALS SPEAK (operator ruling 2026-08-02: stop
        // swallowing them). Both were collected and neither was ever
        // read: the gate's per-deed verdicts, and the lane charters
        // that could not be paid for. A refusal names its cure, and a
        // cure nobody reads is not a cure. Census, never a panic.
        if beat.is_multiple_of(10) || (cycles > 0 && beat == cycles) {
            let mut lane_tally: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            let mut judged_tally: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            for polity in worlds.iter_mut() {
                for refusal in polity.lane_refused.drain(..) {
                    *lane_tally.entry(format!("{refusal:?}")).or_insert(0) += 1;
                }
                for (_, unlawful) in polity.refused.drain(..) {
                    *judged_tally.entry(format!("{unlawful:?}")).or_insert(0) += 1;
                }
            }
            if !lane_tally.is_empty() {
                println!("beat {beat}: LANE REFUSED — {lane_tally:?}");
            }
            if !judged_tally.is_empty() {
                println!("beat {beat}: JUDGE REFUSED — {judged_tally:?}");
            }
        }
        // THE SHARED-LANGUAGE CENSUS — prove on the wire that the civilizations
        // speak one AUTIC tongue (common vocabulary + cross-civ comprehension vs a
        // control). Bounded to <=8 civs x 40 words, so it is cheap; a global beat
        // over 107 civs is slow, so run it every few beats, not every 30.
        if beat.is_multiple_of(3) || (cycles > 0 && beat == cycles) {
            // BEAT-PARALLEL (#152): these were invisible to the phase profile
            // even though the comment above already names them the suspected
            // cost (a global pass over 107 civs) — wrapped so the census that
            // finds the bottleneck can find THIS one too.
            conductor::profile::timed("shared_language_census", || {
                conductor::shared_language_census(&worlds, epoch_base + beat)
            });
            // Make the wired-but-silent economy + societies visible on the wire.
            conductor::profile::timed("emergent_census", || {
                conductor::emergent_census(&worlds, epoch_base + beat)
            });
        }
        if beat.is_multiple_of(10) || (cycles > 0 && beat == cycles) {
            // Re-announce the roster count periodically, not just at boot: the
            // per-beat COG-* instrumentation volume rotates a one-time boot
            // line out of journald's ring buffer within minutes, which blanked
            // this process's own "N civilizations" line the mycelium monitor's
            // civs column depends on (`engine_stats()`'s `(\d+) civilizations`
            // regex, tools/observe.py) long before an operator could read it.
            println!("conductor: {} civilizations live — beat {beat}", worlds.len());
            let summary: Vec<String> = worlds
                .iter()
                .map(|b| {
                    format!(
                        "{}(t{} z{} s{} L{})",
                        b.spec.civilization.sigil,
                        b.world.now(),
                        b.world.orb().map(|o| o.zones.len()).unwrap_or(0),
                        b.solved,
                        b.level
                    )
                })
                .collect();
            println!("beat {beat}: {}", summary.join("  "));
            // Rust's stdout is BLOCK buffered once it isn't a TTY (systemd pipes
            // it to journald) — unlike every status eprintln! elsewhere in this
            // codebase, which flushes immediately. Confirmed live: the boot
            // line above this block sat unflushed for 8+ minutes under
            // systemd. Force it out so an external reader (mycelium's
            // observe.py) sees this beat's line within the beat, not whenever
            // an unrelated stdout write happens to fill the buffer.
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        // Task #8: the opt-in phase profile (NETSTRATUM_PROFILE=1).
        for (name, spent) in conductor::profile::drain() {
            println!("beat {beat}: PROFILE {name} {spent:?}");
        }
        if cycles > 0 && beat >= cycles {
            break;
        }
        if cadence > 0 {
            std::thread::sleep(std::time::Duration::from_secs(cadence));
        }
    }
    let repeat = conductor::PROBE_DIGEST_REPEAT.load(std::sync::atomic::Ordering::Relaxed);
    let total = conductor::PROBE_DIGEST_TOTAL.load(std::sync::atomic::Ordering::Relaxed);
    println!(
        "R5 probe: {repeat}/{total} forge draws reused an unchanged digest ({}%)",
        repeat * 100 / total.max(1)
    );
    let choice_points =
        conductor::COGNITION_CHOICE_POINTS.load(std::sync::atomic::Ordering::Relaxed);
    let lookahead_decided =
        conductor::COGNITION_LOOKAHEAD_DECIDED.load(std::sync::atomic::Ordering::Relaxed);
    println!(
        "COGNITION-1/2 probe: {choice_points} genuine settlement choices, {lookahead_decided} decided by the lookahead"
    );
    println!("era complete: {beat} heartbeats, {} civilizations live", worlds.len());
}
