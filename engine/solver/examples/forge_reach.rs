//! REACH ON THE FORGE — the yield curve for the workload the conductor
//! ACTUALLY runs.
//!
//! `reach_vs_sweep` measures the ARC corpus. The live board does not
//! solve ARC: `conductor::research_step` calls `forge_task(digest,
//! Difficulty::level(n))` and solves that. Corpus hit rate is 3.4%;
//! forge hit rates run 42.5–100%. A yield ratio measured on one says
//! nothing certain about the other, and the conductor's call site
//! cannot be ruled on without this.
//!
//! Forged tasks carry their answer by construction — the secret word is
//! drawn, applied, then discarded — so `test_expected` is always
//! present and CONFIRMED is exact here in a way it never is on a corpus.
//!
//! Usage: cargo run -p solver --release --example forge_reach -- [per_rung]

use solver::{forge_task, propose_measured, solve, Difficulty, Hunger, Need, SolveRead};

#[derive(Default, Clone)]
struct Tally {
    closed: u32,
    confirmed: u32,
    false_closures: u32,
    tested: u64,
}

impl Tally {
    fn take(&mut self, read: &SolveRead, tested: usize) {
        self.tested += tested as u64;
        if let SolveRead::Solved { matches_expected, .. } = read {
            self.closed += 1;
            match matches_expected {
                Some(true) => self.confirmed += 1,
                Some(false) => self.false_closures += 1,
                None => {}
            }
        }
    }
}

/// A deterministic digest per (rung, index) — the forge is seeded, so
/// the whole run replays byte-identically.
fn digest(rung: u32, n: u32) -> [u8; 32] {
    let mut seed = [0u8; 32];
    seed[0] = rung as u8;
    seed[1..5].copy_from_slice(&n.to_le_bytes());
    *blake3::hash(&seed).as_bytes()
}

fn main() {
    let per_rung: u32 = std::env::args()
        .nth(1)
        .map(|n| n.parse().expect("per_rung"))
        .unwrap_or(200);

    let needs: [(&str, Need); 3] = [
        ("sated", Need::sated()),
        ("struck", Need::struck(Hunger::Resonance, 240)),
        ("long-starved", Need::struck(Hunger::Novelty, 480)),
    ];

    println!("REACH ON THE FORGE — {per_rung} tasks per rung, rungs 3..=5\n");
    println!(
        "{:<8} {:<14} {:>7} {:>10} {:>8} {:>12}",
        "rung", "proposer", "closed", "confirmed", "false", "tested/task"
    );
    println!("{}", "-".repeat(64));

    // Rungs 3, 4, 5 are the ones that PAY: mint_bound f(0..=2) = 0.
    for rung in 3..=5u32 {
        let difficulty = Difficulty::level(rung);
        let mut sweep = Tally::default();
        let mut reaches: Vec<Tally> = vec![Tally::default(); needs.len()];

        for n in 0..per_rung {
            let task = forge_task(digest(rung, n), difficulty);
            let swept = solve(&task, difficulty.word_len);
            sweep.take(&swept, 0);
            for (i, (_, need)) in needs.iter().enumerate() {
                let proposal = propose_measured(&task, need);
                reaches[i].take(&proposal.read, proposal.tested);
            }
        }

        println!(
            "{:<8} {:<14} {:>7} {:>10} {:>8} {:>12}",
            rung, "SWEEP", sweep.closed, sweep.confirmed, sweep.false_closures, "up to 200k"
        );
        for (i, (label, _)) in needs.iter().enumerate() {
            let t = &reaches[i];
            println!(
                "{:<8} {:<14} {:>7} {:>10} {:>8} {:>12}",
                "",
                label,
                t.closed,
                t.confirmed,
                t.false_closures,
                t.tested / u64::from(per_rung).max(1)
            );
        }
        println!();
    }

    println!("CONFIRMED is exact here: a forged task always carries its answer.");
}
