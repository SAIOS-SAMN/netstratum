//! **FAIR-DEPTH SOLVABILITY AT EVERY RANK** — the end-to-end claim.
//!
//! A forged task must be findable by a search bounded at the forged
//! word's own length. If it is not, the ladder prices work nobody can
//! do, and the mint is unreachable by honest play.

use solver::nsearch::{solve_field, FieldSolveRead};
use solver_verify::rung::{forge_field, Rung};

/// The claim, measured across ranks and levels: forge a task, search to
/// the rung's own `word_len`, and the search closes it AND agrees with
/// the forge's own held-back answer.
#[test]
fn a_forged_task_is_solvable_at_its_own_depth() {
    let mut checked = 0;
    for rank in 2..=3usize {
        for level in 0..=2u32 {
            let rung = Rung::level(rank, level);
            // Tier 1 draws only mirrors and swaps, which the search's
            // vocabulary carries whole. Recolor is deliberately outside
            // it (10! permutations is not a fair-depth search), so this
            // measures the rungs the forge path is meant to serve.
            if rung.tier() > 1 {
                continue;
            }
            for seed in 0..6u8 {
                let mut digest = [0u8; 32];
                digest[0] = seed;
                digest[1] = rank as u8;
                digest[2] = level as u8;
                let task = forge_field(digest, &rung);
                match solve_field(&task, rung.word_len()) {
                    FieldSolveRead::Solved { matches_expected, word, .. } => {
                        assert!(
                            word.len() <= rung.word_len(),
                            "rank {rank} level {level}: found a word longer than the bound"
                        );
                        assert_eq!(
                            matches_expected,
                            Some(true),
                            "rank {rank} level {level} seed {seed}: closed the training \
                             pairs but rendered the wrong test output — the search found \
                             a word that agrees on the observations and not on the answer"
                        );
                        checked += 1;
                    }
                    FieldSolveRead::Unsolved => panic!(
                        "rank {rank} level {level} seed {seed}: a forged task was \
                         unsolvable at its own depth {} — the ladder prices work \
                         nobody can do",
                        rung.word_len()
                    ),
                }
            }
        }
    }
    assert!(checked >= 12, "the sweep must actually measure something: {checked}");
}

/// THE VACUOUS SOLVE IS REFUSED — "closes every training pair" is true
/// over no pairs, so an empty task must NOT read Solved at depth 0.
#[test]
fn an_empty_task_never_reads_solved() {
    let rung = Rung::level(2, 0);
    let mut task = forge_field([1u8; 32], &rung);
    task.train.clear();
    assert_eq!(solve_field(&task, 3), FieldSolveRead::Unsolved);
}
