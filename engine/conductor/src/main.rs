//! The conductor binary — the board's heartbeat.
//!
//! Usage:
//!   conductor [--var var] [--cycles N] [--cadence SECONDS]
//!
//! Bot lore lives in `<var>/bots.json` — seeded with the founding
//! NPCs on first run, then the operator's file to edit freely.
//! `--cycles 0` (default) beats forever at the cadence. Watch live:
//!   python3 tools/observer.py --watch 2

use conductor::{founding_bots, load_bots, write_default_bots, BotWorld};
use mirror::registry::Registry;
use mirror::Mirror;
use std::path::PathBuf;

fn main() {
    let mut var_root = String::from("var");
    let mut cycles: u64 = 0;
    let mut cadence_secs: u64 = 2;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let mut value = || args.next().expect("flag value");
        match flag.as_str() {
            "--var" => var_root = value(),
            "--cycles" => cycles = value().parse().expect("--cycles N"),
            "--cadence" => cadence_secs = value().parse().expect("--cadence SECONDS"),
            other => {
                eprintln!("conductor: unknown flag {other}");
                std::process::exit(2);
            }
        }
    }

    std::fs::create_dir_all(&var_root).expect("var root");
    let bots_path = PathBuf::from(&var_root).join("bots.json");
    write_default_bots(&bots_path).expect("seed bots.json");
    let specs = load_bots(&bots_path).unwrap_or_else(founding_bots);

    let registry = Registry::at(format!("{var_root}/registry"));
    let mirror = Mirror::at(format!("{var_root}/worlds"));

    // CONVERGE-1 (07 §1.6): converge the archived eras BEFORE genesis —
    // boot adopts the past instead of founding blind beside it.
    let eras = mirror::converge::default_era_roots(&var_root);
    let converged = mirror::converge::converge(&mirror, &eras).expect("convergence");
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

    let mut worlds: Vec<BotWorld> = specs
        .into_iter()
        .map(|spec| BotWorld::genesis(spec, &registry, &mirror))
        .collect();
    println!(
        "conductor: {} civilizations from {} — {}",
        worlds.len(),
        bots_path.display(),
        if cycles == 0 {
            format!("beating every {cadence_secs}s (Ctrl-C to stop)")
        } else {
            format!("running {cycles} heartbeats")
        }
    );

    let mut beat: u64 = 0;
    loop {
        beat += 1;
        // TUniversal T = epoch base + this era's beat (07 §1.3): live
        // ticks stack on the converged timeline, never restart it.
        let _ = mirror.set_universal_clock(epoch_base + beat);
        let mut newborns = Vec::new();
        for bot in &mut worlds {
            if let Some(spec) = bot.cycle(&mirror, &registry) {
                newborns.push(spec);
            }
        }
        for spec in newborns {
            println!(
                "beat {beat}: SPECIATION — {} ({}) descends from {}, settling {}/{}",
                spec.civilization.name,
                spec.civilization.species,
                spec.civilization.lineage.as_deref().unwrap_or("?"),
                spec.world_name,
                spec.planet,
            );
            worlds.push(BotWorld::genesis(spec, &registry, &mirror));
        }
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — journals + the Terminal Wall regenerate mirror-side.
        if beat.is_multiple_of(mirror::journal::YEAR_TICKS) {
            for bot in &worlds {
                let _ = mirror.record_journal(
                    &bot.spec.world_name,
                    &bot.world,
                    &mirror::journal::Voice::of(&bot.spec.civilization),
                );
            }
            println!(
                "beat {beat}: year {} journaled — the wall is fresh (var/worlds/wall.html)",
                beat / mirror::journal::YEAR_TICKS
            );
        }
        if beat.is_multiple_of(10) || (cycles > 0 && beat == cycles) {
            let summary: Vec<String> = worlds
                .iter()
                .map(|b| {
                    format!(
                        "{}(t{} z{} s{} L{})",
                        b.spec.civilization.slug,
                        b.world.now(),
                        b.world.orb().map(|o| o.zones.len()).unwrap_or(0),
                        b.solved,
                        b.level
                    )
                })
                .collect();
            println!("beat {beat}: {}", summary.join("  "));
        }
        if cycles > 0 && beat >= cycles {
            break;
        }
        if cadence_secs > 0 {
            std::thread::sleep(std::time::Duration::from_secs(cadence_secs));
        }
    }
    println!("era complete: {beat} heartbeats, {} civilizations live", worlds.len());
}
