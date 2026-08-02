//! R-D6 step 1 probe — does the pair encoder read the TRANSFORMATION or
//! just the grid?
//!
//! A forged task applies ONE secret word to several sampled inputs. If
//! the encoding carries the transformation, the training pairs of one
//! task should agree on their class and tasks with different words
//! should differ. If it carries the grid, every pair reads differently
//! and the encoding is not yet an invariant.
//!
//! This reports the measurement. It does not assert a conclusion.
use solver::pair_delta::{class_rank, fingerprint};
use solver_verify::forge::{forge_task, Difficulty};

fn main() {
    let mut agree = 0u32;
    let mut tasks = 0u32;
    let mut rank_hist = std::collections::BTreeMap::new();
    let mut collide = 0u32;
    let mut first_prints: Vec<[u8; 32]> = Vec::new();

    for level in [1u32, 2, 3, 4, 5] {
        let difficulty = Difficulty::level(level);
        for seed in 0..24u32 {
            let mut digest = [0u8; 32];
            digest[0] = seed as u8;
            digest[1] = level as u8;
            let task = forge_task(digest, difficulty);
            let prints: Vec<[u8; 32]> = task
                .train
                .iter()
                .map(|p| fingerprint(&p.input, &p.output))
                .collect();
            for pair in &task.train {
                *rank_hist.entry(class_rank(&pair.input, &pair.output)).or_insert(0u32) += 1;
            }
            tasks += 1;
            if prints.windows(2).all(|w| w[0] == w[1]) {
                agree += 1;
            }
            if let Some(head) = prints.first() {
                if first_prints.contains(head) {
                    collide += 1;
                }
                first_prints.push(*head);
            }
        }
    }
    println!("tasks {tasks}");
    println!("  pairs of one task AGREE on class: {agree}/{tasks} ({}%)", agree * 100 / tasks);
    println!("  distinct tasks sharing a first-pair class: {collide}");
    println!("  class rank -> pairs: {rank_hist:?}");
}
