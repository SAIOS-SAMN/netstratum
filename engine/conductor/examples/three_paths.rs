//! THE THREE PATHS, benchmarked on the same era.
//!
//! Same beat count, same founding roster, same seeded forge — the only
//! thing that varies is how a line searches.
//!
//!   UNCOUPLED   always exhaust. The control.
//!   COUPLED     the trajectory routes: read while reading pays,
//!               escalate the hunger on stagnation, exhaust on decline.
//!   V ∪ D       the union: every task gets BOTH. Read first, exhaust
//!               whenever the reading did not close. Neither path
//!               derives its standing from the other.
//!
//! DEEDS PROPOSED is reported beside the economy because a path that
//! mints less may simply be DOING less — and the two are not the same
//! failure. Deeds are every receipt the world appended, so it measures
//! the whole life of the board and not just its research arm.
//!
//! SPEND IS NEVER BLENDED. Read-candidates are counted exactly;
//! sweep invocations are bounded by `SEARCH_BUDGET` and reported as
//! invocations. Adding a measured number to a bounded one would make
//! both yields meaningless.
//!
//! Usage: cargo run -p conductor --release --example three_paths -- [beats]

use cfe::dop::ClosureEvent;
use conductor::{run_era_with, SearchMode};

struct Row {
    label: &'static str,
    solves: u32,
    mints: u64,
    earned: cfe::ratio::Ratio,
    deeds: u64,
    refused: u64,
    read_spend: u64,
    sweeps: u64,
    secs: f64,
}

fn run(label: &'static str, mode: SearchMode, beats: u32) -> Row {
    let mut root = std::env::temp_dir();
    root.push(format!("netstratum-three-paths-{label}"));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch");
    let var = root.to_string_lossy().into_owned();

    let started = std::time::Instant::now();
    let worlds = run_era_with(&var, beats, mode);
    let secs = started.elapsed().as_secs_f64();

    let mut mints = 0u64;
    let mut earned = cfe::ratio::ratio(0, 1);
    let mut deeds = 0u64;
    for polity in &worlds {
        let receipts = polity.world.chronicle().receipts();
        deeds += receipts.len() as u64;
        for receipt in receipts {
            if let ClosureEvent::DiscoveryMinted { amount, .. } = &receipt.event {
                mints += 1;
                earned += amount.clone();
            }
        }
    }
    Row {
        label,
        solves: worlds.iter().map(|p| p.solved).sum(),
        mints,
        earned,
        deeds,
        refused: worlds.iter().map(|p| p.refusals().len() as u64).sum(),
        read_spend: worlds.iter().map(|p| p.spent).sum(),
        sweeps: worlds.iter().map(|p| p.swept).sum(),
        secs,
    }
}

fn main() {
    let beats: u32 = std::env::args()
        .nth(1)
        .map(|b| b.parse().expect("beats"))
        .unwrap_or(60);

    let rows = [
        run("uncoupled", SearchMode::Uncoupled, beats),
        run("coupled", SearchMode::Coupled, beats),
        run("union", SearchMode::Union, beats),
    ];

    println!("THE THREE PATHS — {beats} beats, same roster, same seeded forge\n");
    println!(
        "{:<12} {:>7} {:>6} {:>8} {:>8} {:>8} {:>10} {:>8} {:>9}",
        "path", "solves", "mints", "earned", "deeds", "refused", "read-cand", "sweeps", "runtime"
    );
    println!("{}", "-".repeat(84));
    for r in &rows {
        println!(
            "{:<12} {:>7} {:>6} {:>8} {:>8} {:>8} {:>10} {:>8} {:>8.2}s",
            r.label, r.solves, r.mints, r.earned, r.deeds, r.refused, r.read_spend, r.sweeps, r.secs
        );
    }

    // Yield in the motion's own terms, and only where the denominator
    // is a COUNT. A path whose spend is bounded rather than counted has
    // no honest W/S and says so.
    println!("\nyield  (earned per counted read-candidate; '—' where spend is a bound)");
    for r in &rows {
        let cell = match (r.read_spend, r.sweeps) {
            (0, _) => "— (no counted spend: this path only exhausts)".to_string(),
            (spend, sweeps) => {
                let y = r.earned.clone() / cfe::ratio::ratio(spend as i64, 1);
                match sweeps {
                    0 => format!("{y}"),
                    n => format!("{y}   (+{n} bounded sweep invocations, not in the denominator)"),
                }
            }
        };
        println!("  {:<12} {cell}", r.label);
    }

    println!("\ndeeds per solve — is a path minting less, or DOING less?");
    for r in &rows {
        let per = match r.solves {
            0 => "—".to_string(),
            s => format!("{}", cfe::ratio::ratio(r.deeds as i64, s as i64)),
        };
        println!("  {:<12} {per}", r.label);
    }
}
