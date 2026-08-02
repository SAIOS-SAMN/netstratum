//! THE IDENTITY PIN — `solve` must return the SAME program, for the same
//! forged task, across the R1/R3 rewrite (`docs/FORGE_REMEDIATION.md`).
//!
//! **Why a literal table and not a property.** R1 (carry the prefix
//! outputs) and R3 (stream the frontier) are pure performance: same
//! enumeration order, same first match, same certificate. A property
//! test — "the answer closes every training pair" — would pass even if
//! the rewrite silently returned a DIFFERENT closing program, and a
//! different program means a different certificate and a different deed.
//! The only pin that catches that is the answer itself, recorded.
//!
//! **Recorded from measurement, not from hope**: these 30 rows were
//! dumped from the implementation as it stood BEFORE the rewrite. The
//! rewrite is correct exactly when they still hold.
//!
//! Rows deliberately span the whole ladder: single symmetries, two-op
//! words, derived recolors, derived tiles, and genuine `UNSOLVED` — a
//! table of only-solved rows would not notice a rewrite that started
//! solving things it should not.

use solver::search::{solve, SolveRead};
use solver_verify::forge::{forge_task, Difficulty};

/// `(level, seed, expected word)` — `"UNSOLVED"` is a lawful answer and
/// is pinned as firmly as any solve (the honest-withhold law).
const RECORDED: &[(u32, u8, &str)] = &[
    (1, 0, "rotate270"),
    (1, 1, "rotate90"),
    (1, 2, "transpose"),
    (1, 3, "rotate180"),
    (1, 4, "rotate270"),
    (1, 5, "transpose"),
    (2, 0, "rotate180"),
    (2, 1, "flip_v"),
    (2, 2, "rotate90"),
    (2, 3, "rotate270"),
    (2, 4, "flip_v"),
    (2, 5, "rotate90"),
    (3, 0, "recolor[9,7,6,8,0,2,4,3,1,5]"),
    (3, 1, "rotate90|flip_v"),
    (3, 2, "flip_h|recolor[9,5,2,6,4,0,3,1,8,7]"),
    (3, 3, "recolor[1,7,8,5,2,9,3,4,0,6]"),
    (3, 4, "transpose|recolor[0,8,5,2,9,4,1,3,7,6]"),
    (3, 5, "rotate180"),
    (4, 0, "rotate90|tile3x2(odd_row=cols,odd_col=both)"),
    (4, 1, "flip_v|tile2x1(odd_row=rows,odd_col=none)"),
    (4, 2, "UNSOLVED"),
    (4, 3, "UNSOLVED"),
    (4, 4, "transpose|tile2x2(odd_row=rows,odd_col=rows)"),
    (4, 5, "UNSOLVED"),
    (5, 0, "UNSOLVED"),
    (5, 1, "transpose|recolor[8,0,2,5,6,7,3,9,1,4]"),
    (5, 2, "rotate270|tile2x2(odd_row=none,odd_col=cols)"),
    (5, 3, "UNSOLVED"),
    (5, 4, "recolor[2,8,1,0,5,4,3,7,9,6]"),
    (5, 5, "UNSOLVED"),
];

fn digest_for(level: u32, seed: u8) -> [u8; 32] {
    let mut digest = [0u8; 32];
    digest[0] = seed;
    digest[1] = level as u8;
    digest
}

fn word_for(level: u32, seed: u8) -> String {
    let difficulty = Difficulty::level(level);
    let task = forge_task(digest_for(level, seed), difficulty);
    match solve(&task, difficulty.word_len) {
        SolveRead::Solved { program, .. } => {
            program.iter().map(|op| op.describe()).collect::<Vec<_>>().join("|")
        }
        SolveRead::Unsolved => "UNSOLVED".into(),
    }
}

#[test]
fn the_answer_is_byte_identical_across_the_rewrite() {
    let mut drifted = Vec::new();
    for (level, seed, expected) in RECORDED {
        let seen = word_for(*level, *seed);
        if seen != *expected {
            drifted.push(format!("  level {level} seed {seed}: {expected}  ->  {seen}"));
        }
    }
    assert!(
        drifted.is_empty(),
        "solve returned a DIFFERENT program than the recorded one.\n\
         R1/R3 are performance-only: same enumeration order, same first match.\n\
         A different program is a different certificate and a different deed.\n{}",
        drifted.join("\n")
    );
}

#[test]
fn solve_is_deterministic_within_one_build() {
    // The forge's whole premise (`forge.rs` head): same digest, same
    // difficulty → byte-identical task on every machine. The search
    // over it must be equally stable or nothing downstream replays.
    for (level, seed, _) in RECORDED.iter().take(8) {
        assert_eq!(
            word_for(*level, *seed),
            word_for(*level, *seed),
            "level {level} seed {seed} answered differently on a second call"
        );
    }
}

#[test]
fn every_solved_answer_actually_closes_its_task() {
    // The pin above fixes WHICH program; this one fixes that the program
    // is real. Both are needed: a table can be updated wrongly, and a
    // closing check alone cannot see a substitution.
    for (level, seed, expected) in RECORDED {
        if *expected == "UNSOLVED" {
            continue;
        }
        let difficulty = Difficulty::level(*level);
        let task = forge_task(digest_for(*level, *seed), difficulty);
        let SolveRead::Solved { program, .. } = solve(&task, difficulty.word_len) else {
            panic!("level {level} seed {seed} was recorded as solved and is not");
        };
        for (at, pair) in task.train.iter().enumerate() {
            assert_eq!(
                solver_verify::grid::run(&program, &pair.input),
                pair.output,
                "level {level} seed {seed}: the answer misses training pair {at}"
            );
        }
    }
}
