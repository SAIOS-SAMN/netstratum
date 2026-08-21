//! The conductor binary — the board's heartbeat.
//!
//! Usage:
//!   conductor [--var var] [--cycles N] [--cadence SECONDS] [--fold]
//!
//! THE DEFINITION LAW (`polities.rs`): the roster is COMPILED. There is
//! no `polities.json` to seed, adopt, or hand-edit — a line is added
//! through the receipted door (`civ new`), never by editing a file.
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
    // THE DEFINITION LAW: the roster compiles. The legacy-adoption shim
    // for `egregores.json`/`bots.json` is struck with the file itself —
    // there is no longer a roster on disk for a legacy name to become.
    let registry = Registry::at(format!("{var_root}/registry"));
    // THE DEFINITION LAW: the compiled founding lines plus every line
    // the operator has registered since. No roster file is read.
    let specs = roster(&registry);
    let book = Ledger::at(format!("{var_root}/worlds"));

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
    let mut worlds: Vec<Polity> = specs
        .into_iter()
        .map(|mut spec| {
            let opening = spec
                .gauge
                .take()
                .unwrap_or_else(|| cfe::ratio::ratio(0, 1));
            spec.gauge = Some(cfe::world::wrap_gauge(opening + live_base.clone()));
            Polity::genesis(spec, &registry, &book)
        })
        .collect();
    println!(
        "conductor: {} civilizations from the compiled roster — {}",
        worlds.len(),
        if cycles == 0 {
            format!("beating every {cadence_secs}s (Ctrl-C to stop)")
        } else {
            format!("running {cycles} heartbeats")
        }
    );

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

    let mut beat: u64 = 0;
    let mut total_cleaves: u64 = 0;
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
        // TUniversal T = epoch base + this era's beat (07 §1.3): live
        // ticks stack on the converged timeline, never restart it.
        let _ = book.set_universal_clock(epoch_base + beat);
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
        let mut newborns = Vec::new();
        for polity in &mut worlds {
            if polity.penned {
                continue;
            }
            if let Some(spec) = polity.cycle(&book, &registry) {
                newborns.push(spec);
            }
            // DIRECTIVE 19: the gate speaks on the board. Every deed
            // `kernel::admits` refused this beat is named here, once —
            // the verdict stays on the polity's permanent record either
            // way. The deed still STANDS in the chronicle: `replay` is
            // the live transition until the Phase 3 flip demotes it
            // (U_DIRECTIVES §5), so this line reports a gate overruled
            // by a proposer, which is exactly the gap worth seeing.
            let world_name = polity.spec.world_name.clone();
            for (sequence, unlawful) in polity.unreported_refusals() {
                println!(
                    "beat {beat}: THE JUDGE REFUSES — {world_name}: deed {sequence} is unlawful ({unlawful:?}); the deed stands, the verdict is on record"
                );
            }
        }
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
        conductor::profile::timed("fleet_step", || conductor::fleet_step(&mut worlds));
        // OFFER-1a: the market beat — intents post, crossings clear
        // (one aperture pair each way, receipted both sides).
        for (seller, buyer) in conductor::profile::timed("market_step", || conductor::market_step(&mut worlds, beat)) {
            println!(
                "beat {beat}: THE CROSSING CLEARS — {seller} ⇌ {buyer}; the autarkies trade"
            );
        }
        // MERCHANT-1: the caravan bridges what the void walled.
        for (m, a, b) in conductor::profile::timed("merchant_step", || conductor::merchant_step(&mut worlds, beat)) {
            println!(
                "beat {beat}: THE CARAVAN — {m} bridges {a} ⇌ {b}; the walled trade flows"
            );
        }
        // E3.3: pure NPC element aperture hauls (Wave-2).
        for (from, to) in conductor::profile::timed("element_trade_step", || conductor::element_trade_step(&mut worlds, beat)) {
            println!(
                "beat {beat}: ELEMENT CROSSING — {from} → {to}; open matter rides the void"
            );
        }
        // STAMP MM-1: mint + spot clear (Wave-4).
        for (buyer, seller) in conductor::profile::timed("stamp_market_step", || conductor::stamp_market_step(&mut worlds, beat)) {
            println!(
                "beat {beat}: STAMP SPOT — {buyer} ⇌ {seller}; crystal clears for STAMP"
            );
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
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — journals + the Terminal Wall regenerate lens-side.
        if beat.is_multiple_of(scribe::journal::YEAR_TICKS) {
            // ROUTE-1: the year voice draws lane maintenance — an
            // unpaid lane lapses by receipt and raw space returns.
            for polity in worlds.iter_mut().filter(|e| !e.penned) {
                for deal in polity.world.maintain_routes(cfe::world::FIRST_ORB) {
                    println!(
                        "beat {beat}: LANE {deal} LAPSES on {} — raw space takes the path back",
                        polity.spec.world_name
                    );
                }
            }
            for polity in worlds.iter().filter(|e| !e.penned) {
                let _ = scribe::record_journal(
                    &book,
                    &polity.spec.world_name,
                    &polity.world,
                    &scribe::journal::Voice::of(&polity.spec.civilization),
                );
            }
            println!(
                "beat {beat}: year {} journaled — the wall is fresh (var/worlds/wall.html)",
                beat / scribe::journal::YEAR_TICKS
            );
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
                words += cfe::vocabulary::solver_words(polity.world.chronicle()).len();
                if polity.ration_survives(cfe::world::FIRST_ORB) {
                    fed += 1;
                }
            }
            let n = live.len();
            eprintln!(
                "census beat {beat}: lines={n} pop={pop} words_minted={words} cleaved={total_cleaves} fed={fed}/{n}"
            );
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
        if beat.is_multiple_of(10) || (cycles > 0 && beat == cycles) {
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
