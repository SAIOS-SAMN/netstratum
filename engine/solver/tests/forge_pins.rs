//! The forge pins: determinism (a task is a reading of history),
//! digest separation (a different chronicle forges a different task),
//! fairness (a forged task closes at its stated search depth), and
//! the certificate replays on forged solves.

use solver::{forge_task, solve, verify_certificate, CertificateRead, Difficulty, SolveRead};

/// Same digest + same difficulty → byte-identical task, on every
/// machine, forever (CHARTER §9 at the forge).
#[test]
fn the_forge_is_a_reading_of_history() {
    for level in 1..=5 {
        let difficulty = Difficulty::level(level);
        let a = forge_task([11u8; 32], difficulty);
        let b = forge_task([11u8; 32], difficulty);
        assert_eq!(a, b, "level {level} forges deterministically");
    }
}

/// A different digest forges a different task; a different difficulty
/// forges a different task from the same digest.
#[test]
fn digests_and_difficulties_separate() {
    let base = forge_task([11u8; 32], Difficulty::level(2));
    let other_digest = forge_task([12u8; 32], Difficulty::level(2));
    let other_level = forge_task([11u8; 32], Difficulty::level(3));
    assert_ne!(base, other_digest);
    assert_ne!(base, other_level);
}

/// FAIRNESS: a forged task is solvable at its stated word length —
/// the solver, searching to that depth, closes the training pairs and
/// matches the held-out answer; the certificate replays intact.
#[test]
fn forged_tasks_close_at_fair_depth() {
    let mut confirmed = 0;
    for seed in 0..8u8 {
        let difficulty = Difficulty::level(1 + (seed % 3) as u32);
        let task = forge_task([seed; 32], difficulty);
        match solve(&task, difficulty.word_len) {
            SolveRead::Solved {
                program,
                test_output,
                matches_expected,
                certificate,
            } => {
                assert_eq!(matches_expected, Some(true), "seed {seed}: render matches");
                assert_eq!(
                    verify_certificate(&certificate, &task, &program, &test_output),
                    CertificateRead::Intact
                );
                confirmed += 1;
            }
            SolveRead::Unsolved => panic!(
                "seed {seed}: a forged level-{} task must close at depth {}",
                1 + (seed % 3),
                difficulty.word_len
            ),
        }
    }
    assert_eq!(confirmed, 8);
}

/// The forged name carries provenance: digest prefix + tier + word
/// length — an observer reads the difficulty off the certificate file.
#[test]
fn the_forged_name_carries_provenance() {
    let task = forge_task([0xAB; 32], Difficulty::level(4));
    assert!(task.name.starts_with("forge_abababababab_t3w2"), "{}", task.name);
}
