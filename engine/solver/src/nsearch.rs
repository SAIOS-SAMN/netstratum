//! **THE RANK-GENERIC SEARCH** — one prover for every rank.
//!
//! `search::solve` searched planar programs and `volume::solve_volume`
//! searched volumetric ones, with the same breadth-first shape written
//! twice because their carriers shared no type. This searches `Field`,
//! so it searches both — and rank 4, which neither could reach.

use solver_verify::field::{run, Field, Word};
use solver_verify::rung::{vocabulary, FieldTask};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldSolveRead {
    Solved {
        word: Word,
        test_output: Field,
        matches_expected: Option<bool>,
    },
    Unsolved,
}

/// Breadth-first to `max_depth`, over the rank's own closed vocabulary.
///
/// THE VACUOUS SOLVE IS REFUSED: "closes every training pair" is true
/// over NO pairs, so an empty task would read Solved at depth 0 having
/// proved nothing. Same guard the verifier applies at decode, so the two
/// agree rather than merely coinciding.
pub fn solve_field(task: &FieldTask, max_depth: usize) -> FieldSolveRead {
    if task.train.is_empty() {
        return FieldSolveRead::Unsolved;
    }
    let rank = task.test_in.rank();
    let words = vocabulary(rank, task.test_in.shape());

    let closes = |word: &Word| {
        task.train
            .iter()
            .all(|(input, expected)| &run(word, input) == expected)
    };

    let mut frontier: Vec<Word> = vec![Vec::new()];
    for depth in 0..=max_depth {
        for word in &frontier {
            if closes(word) {
                let test_output = run(word, &task.test_in);
                let matches_expected =
                    task.test_expected.as_ref().map(|want| want == &test_output);
                return FieldSolveRead::Solved {
                    word: word.clone(),
                    test_output,
                    matches_expected,
                };
            }
        }
        // The last depth grows a frontier nothing would read.
        if depth == max_depth {
            break;
        }
        let mut next = Vec::with_capacity(frontier.len() * words.len().max(1));
        for word in &frontier {
            for axis in &words {
                let mut grown = word.clone();
                grown.push(axis.clone());
                next.push(grown);
            }
        }
        frontier = next;
    }
    FieldSolveRead::Unsolved
}
