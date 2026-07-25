//! Measure the hardness ladder: forge N tasks per level from distinct
//! digests, solve each at a budgeted depth, report the rate. This is
//! the difficulty knob READ, not asserted.
//!
//! Usage: cargo run -p solver --release --example forge_run -- [tasks_per_level] [max_depth]

use solver::{forge_task, solve, Difficulty, SolveRead};

fn main() {
    let mut args = std::env::args().skip(1);
    let per_level: u8 = args.next().map(|n| n.parse().expect("count")).unwrap_or(20);
    let budget: usize = args.next().map(|d| d.parse().expect("depth")).unwrap_or(3);

    println!("level  tier  word  grid   solved@depth{budget}");
    for level in 1..=5u32 {
        let difficulty = Difficulty::level(level);
        let mut solved = 0;
        for i in 0..per_level {
            let mut digest = [0u8; 32];
            digest[0] = level as u8;
            digest[1] = i;
            let task = forge_task(digest, difficulty);
            if matches!(
                solve(&task, budget),
                SolveRead::Solved {
                    matches_expected: Some(true),
                    ..
                }
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
