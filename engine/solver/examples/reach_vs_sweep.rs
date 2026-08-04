//! THE MEASUREMENT the ruling deserves: the derivation-first, need-
//! driven proposer against the enumerative sweep it replaces, on the
//! same corpus and the same scorer.
//!
//! Two numbers matter, not one. CONFIRMED is how often the answer was
//! right. FALSE CLOSURES is how often a program closed the training
//! pairs and got the held-out answer WRONG — which the judge cannot
//! detect and a corpus rate would therefore pay for. A proposer that
//! confirms slightly fewer with far fewer false closures is the better
//! citizen of this economy.
//!
//! Usage: cargo run -p solver --release --example reach_vs_sweep -- <dir> [sweep_depth]

use solver::{load_task, propose_measured, solve, Hunger, Need, SolveRead};

#[derive(Default)]
struct Tally {
    closed: u32,
    confirmed: u32,
    false_closures: u32,
    tested: u64,
    reach: u64,
}

impl Tally {
    fn take(&mut self, read: &SolveRead, tested: usize, reach: usize) {
        self.tested += tested as u64;
        self.reach += reach as u64;
        if let SolveRead::Solved { matches_expected, .. } = read {
            self.closed += 1;
            match matches_expected {
                Some(true) => self.confirmed += 1,
                Some(false) => self.false_closures += 1,
                None => {}
            }
        }
    }
    fn row(&self, label: &str, n: u32) -> String {
        format!(
            "{label:<26} {:>7} {:>10} {:>15} {:>13}",
            self.closed,
            self.confirmed,
            self.false_closures,
            if n > 0 { self.tested / n as u64 } else { 0 }
        )
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args.next().expect("usage: reach_vs_sweep <corpus_dir> [sweep_depth]");
    let sweep_depth: usize = args.next().map(|d| d.parse().expect("depth")).unwrap_or(3);

    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();

    // The needs a corpus task can be met with. A corpus file carries no
    // civilization, so these are the reaches themselves under test —
    // NOT a claim that any particular task "belongs" to a hunger.
    let needs: [(&str, Need); 3] = [
        ("sated", Need::sated()),
        ("resonance-struck", Need::struck(Hunger::Resonance, 240)),
        ("novelty-struck (long)", Need::struck(Hunger::Novelty, 480)),
    ];

    let mut sweep = Tally::default();
    let mut reaches: Vec<Tally> = (0..needs.len()).map(|_| Tally::default()).collect();
    let mut attempted = 0u32;

    for path in &paths {
        let Ok(task) = load_task(path) else { continue };
        attempted += 1;

        let swept = solve(&task, sweep_depth);
        // The sweep's cost is its budget shape; count the frontier it
        // would have walked as its own constant rather than guessing.
        sweep.take(&swept, 0, 0);

        for (n, (_, need)) in needs.iter().enumerate() {
            let proposal = propose_measured(&task, need);
            reaches[n].take(&proposal.read, proposal.tested, proposal.reach);
        }
    }

    println!("corpus: {dir} — {attempted} tasks\n");
    println!(
        "{:<26} {:>7} {:>10} {:>15} {:>13}",
        "", "closed", "confirmed", "false closures", "tested/task"
    );
    println!("{}", "-".repeat(76));
    println!(
        "{:<26} {:>7} {:>10} {:>15} {:>13}",
        format!("SWEEP (depth {sweep_depth})"),
        sweep.closed,
        sweep.confirmed,
        sweep.false_closures,
        "up to 200k"
    );
    for (n, (label, need)) in needs.iter().enumerate() {
        println!(
            "{}",
            reaches[n].row(
                &format!("REACH {label} (≤{})", need.compose_limit()),
                attempted
            )
        );
    }
    let avg_reach = if attempted > 0 { reaches[0].reach / attempted as u64 } else { 0 };
    println!("\naverage reach: {avg_reach} operators the observation supported, per task");
}
