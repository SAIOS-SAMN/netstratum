//! Direct timing of `solve`, and the solve rate beside it.
//!
//! The conductor beat swings ±4 % on identical code and cannot resolve
//! a change of this size, so it is the wrong instrument. This one runs
//! the search alone. Report `min` and the spread across samples: the
//! minimum is the least noise-contaminated estimator, and the spread is
//! itself a reading — allocator pressure shows there before it shows in
//! the mean.
//!
//! ```text
//! cargo run --release -p solver --example bench_solve -- [seeds] [samples]
//! ```
use solver_verify::forge::{forge_task, Difficulty};
use std::time::{Duration, Instant};

fn one_pass(seeds: u32) -> (Duration, u32, u32, Duration, Duration) {
    let mut total = Duration::ZERO;
    let (mut hit, mut miss) = (Duration::ZERO, Duration::ZERO);
    let (mut solved, mut count) = (0u32, 0u32);
    for level in [3u32, 4, 5] {
        let difficulty = Difficulty::level(level);
        for seed in 0..seeds {
            let mut digest = [0u8; 32];
            digest[0] = (seed & 0xff) as u8;
            digest[1] = level as u8;
            digest[2] = (seed >> 8) as u8;
            let task = forge_task(digest, difficulty);
            let at = Instant::now();
            let read = solver::search::solve(&task, difficulty.word_len);
            total += at.elapsed();
            count += 1;
            if matches!(read, solver::search::SolveRead::Solved { .. }) {
                solved += 1;
                hit += at.elapsed();
            } else {
                miss += at.elapsed();
            }
        }
    }
    (total, solved, count, hit, miss)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let seeds: u32 = args.next().and_then(|a| a.parse().ok()).unwrap_or(40);
    let samples: u32 = args.next().and_then(|a| a.parse().ok()).unwrap_or(5);
    let mut times = Vec::new();
    let (mut solved, mut count) = (0, 0);
    let (mut hit, mut miss) = (Duration::ZERO, Duration::ZERO);
    for _ in 0..samples {
        let (total, s, c, h, m) = one_pass(seeds);
        hit = h;
        miss = m;
        times.push(total);
        solved = s;
        count = c;
    }
    times.sort();
    println!(
        "{count} solves/pass · {solved} solved ({}%) · min {:?} · max {:?} · spread {:?}",
        solved * 100 / count.max(1),
        times[0],
        times[times.len() - 1],
        times[times.len() - 1] - times[0]
    );
    println!(
        "  SOLVED {solved} cost {hit:?} · UNSOLVED {} cost {miss:?} ({}% of time)",
        count - solved,
        miss.as_millis() * 100 / (hit + miss).as_millis().max(1)
    );
}
