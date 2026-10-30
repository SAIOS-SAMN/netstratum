//! Measure the hardness ladder: forge N tasks per level from distinct
//! digests, think each through the ARC seat (the one reduce organ — search is
//! gone), report the solve rate. The difficulty knob READ, not asserted.
//!
//! Usage: cargo run -p solver --release --example forge_run -- [tasks_per_level]

use solver::{forge_task, think_task, Difficulty, SolveRead};

fn main() {
    let mut args = std::env::args().skip(1);
    let per_level: u8 = args.next().map(|n| n.parse().expect("count")).unwrap_or(20);

    println!("level  tier  word  grid   solved");
    for level in 1..=5u32 {
        let difficulty = Difficulty::level(level);
        let mut solved = 0;
        for i in 0..per_level {
            let mut digest = [0u8; 32];
            digest[0] = level as u8;
            digest[1] = i;
            let task = forge_task(digest, difficulty);
            // A fresh mind per task (no shared basin) — the seat's own reach.
            if matches!(
                think_task(&task, &mut Vec::new()),
                SolveRead::Solved { matches_expected: Some(true), .. }
            ) {
                solved += 1;
            }
        }
        println!(
            "  {level}     {}     {}    {}x{}    {solved}/{per_level}",
            difficulty.tier, difficulty.word_len, difficulty.rows, difficulty.cols
        );
    }
}
