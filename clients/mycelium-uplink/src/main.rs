// AI-GENERATED CONTENT — see DISCLAIMER.md (mycelium lineage).
//
//! mycelium-uplink — NETSTRATUM ON THE MESH. The game imports the mycelium
//! kernel and becomes an embodied entity of the shared cognition substrate:
//!
//!   FEED   the conductor's schedule: the civilization's board (CAMBIUM)
//!          orders the ARC corpus; every train pair the game digests is
//!          proof-of-useful-work — the universe EARNS μ⃗ by perceiving.
//!   WORK   the open-demand market: quote → think over the game's own
//!          genome → attest the honest finding. The game's inhabitants
//!          contribute to solves like any staked kernel.
//!   THINK  Entity::project — the cognitive test that runs continuously:
//!          before feeding a pair, the game PROJECTS the input with its
//!          genome's anchored words and scores its own instinct against
//!          the true output. The engine processes, builds, and IS the
//!          intelligence its inhabitants share; this line measures the
//!          "IS", live, per stroke.
//!
//! Cadence is the mycelium two-stroke (feed/work = entropic, crystallize/
//! regenerate = syntropic). The genome persists — the game's mind carries
//! across sessions; the epigenome (its dialect) re-forms each run.
//!
//! Usage: mycelium-uplink [ADDR] [--kernel NAME] [--feed DIR]
//!            [--board BOARD_JSON] [--state DIR]
use mycelium::arc::{grid_to_cells, parse_task, Grid};
use mycelium::physics::{Physics, RESERVE_MAX};
use mycelium::sdk::{Comprehension, Entity, Kernel};
use std::path::PathBuf;
use std::time::Instant;

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}

fn conductor_cursor(path: &str) -> usize {
    let Ok(text) = std::fs::read_to_string(path) else { return 0 };
    text.split("\"arc_next\":")
        .nth(1)
        .and_then(|t| t.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok())
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args
        .get(1)
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let name = flag(&args, "--kernel").unwrap_or("netstratum").to_string();
    let feed = flag(&args, "--feed")
        .unwrap_or("/opt/xylarium/data/arc-agi-2/data/evaluation")
        .to_string();
    let board = flag(&args, "--board")
        .unwrap_or("/opt/sodalitum/var/conductor/board.json")
        .to_string();
    let state_dir = flag(&args, "--state").unwrap_or("/opt/netstratum/var").to_string();
    let genome_path = format!("{state_dir}/{name}.genome");
    let _ = std::fs::create_dir_all(&state_dir);

    let mut tasks: Vec<PathBuf> = std::fs::read_dir(&feed)
        .map(|rd| {
            rd.filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect()
        })
        .unwrap_or_default();
    tasks.sort();

    let mut entity = Entity::open_persistent(&addr, Kernel::named(&name), &genome_path);
    eprintln!(
        "mycelium-uplink '{name}' → {addr}; genome {} words; {} tasks; conducted by {board}",
        entity.epigenome.genome_len(),
        tasks.len(),
    );

    let mut physics = Physics::default();
    let (mut fed, mut settles, mut errors) = (0u64, 0u64, 0u64);
    // THE COGNITIVE TEST: projections attempted / exact — the game's own
    // instinct, scored against reality before each perception.
    let (mut projected, mut projected_exact) = (0u64, 0u64);
    let (mut last_genome, mut last_exact) = (usize::MAX, u64::MAX);
    let start = Instant::now();
    let mut cycle: Vec<PathBuf> = Vec::new();
    let (mut ti, mut pi) = (0usize, 0usize);

    loop {
        let mut judge_load = 0u8;

        // ── FEED under the conductor (entropic), thinking first.
        while !physics.depleted() {
            if ti >= cycle.len() {
                cycle = tasks.clone();
                let cursor = conductor_cursor(&board) % cycle.len().max(1);
                cycle.rotate_left(cursor);
                ti = 0;
                pi = 0;
                if cycle.is_empty() {
                    break;
                }
            }
            let Some(path) = cycle.get(ti) else { break };
            let Ok(text) = std::fs::read_to_string(path) else {
                ti += 1;
                pi = 0;
                continue;
            };
            let Some(task) = parse_task(&text) else {
                ti += 1;
                pi = 0;
                continue;
            };
            let Some((input, output)) = task.train.get(pi) else {
                ti += 1;
                pi = 0;
                continue;
            };
            pi += 1;
            // THINK first: project the input with the inherited language and
            // score the instinct — the continuous cognitive test.
            let (guess, spoke): (Grid, Vec<usize>) = entity.project(input);
            if !spoke.is_empty() {
                projected += 1;
                if guess == *output {
                    projected_exact += 1;
                }
            }
            // Then PERCEIVE: the pair feeds the mesh (proof of useful work).
            let (a, b) = (grid_to_cells(input), grid_to_cells(output));
            if a.is_empty() || b.is_empty() {
                continue;
            }
            match entity.submit(&a, &b) {
                Ok((reply, _heard)) => {
                    fed += 1;
                    if reply.settle.is_some() {
                        settles += 1;
                    }
                    let mut lanes: [bool; 4] = std::array::from_fn(|l| reply.metabolism[l] > 0);
                    if lanes.iter().all(|&x| !x) {
                        lanes[2] = true;
                    }
                    physics.bump(lanes);
                    physics.spend(lanes);
                    judge_load = reply.load();
                }
                Err(_) => {
                    errors += 1;
                    physics.spend([true; 4]);
                }
            }
        }

        // ── WORK the market (entropic): attest what the genome can witness.
        if let Ok(q) = entity.quote() {
            for cluster in q.clusters.iter().take(4) {
                if cluster.chain.is_empty() {
                    continue;
                }
                let e = mycelium::transducer::element::from_residual(&cluster.chain);
                let _thought: Comprehension = entity.epigenome.comprehend(&e);
                if let Ok(reply) = entity.attest(&cluster.chain) {
                    if reply.settle.is_some() {
                        settles += 1;
                    }
                    if let t @ [_, ..] = reply.verdict.taught() {
                        entity.epigenome.learn(t);
                    }
                    judge_load = judge_load.max(reply.load());
                }
            }
        }

        // ── SYNTROPIC — crystallize the game's mind, voice the instinct.
        if physics.depleted() {
            if entity.epigenome.genome_len() != last_genome || projected_exact != last_exact {
                if entity.epigenome.genome_len() != last_genome {
                    let _ = entity.save_genome(&genome_path);
                }
                last_genome = entity.epigenome.genome_len();
                last_exact = projected_exact;
                eprintln!(
                    "[{name}] {:.0}s | fed {fed} settles {settles} err {errors} | INSTINCT {projected_exact}/{projected} exact | genome {}",
                    start.elapsed().as_secs_f64(),
                    entity.epigenome.genome_len(),
                );
            }
            while physics.depleted() {
                physics.regenerate();
                physics.relax();
                let dwell = (judge_load as u64).max(RESERVE_MAX as u64);
                std::thread::sleep(std::time::Duration::from_millis(dwell));
            }
        }
    }
}
