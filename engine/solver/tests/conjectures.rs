//! Concordance 1.10, the audit's restore 1: a mind holds its own unglued
//! hypotheses and tests them on later tasks — and a conjecture is a claim
//! only where it closes every pair of one.

use solver::grid::run;
use solver::{think_task_held, Conjecture, Grid, Op, SolveRead, Task, TaskPair};

fn g(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("a grid")
}

fn pair(input: Grid, program: &[Op]) -> TaskPair {
    let output = run(&program.to_vec(), &input);
    TaskPair { input, output }
}

fn task(train: Vec<TaskPair>, test_in: Grid) -> Task {
    Task {
        name: "conjecture".into(),
        train,
        test_in,
        test_expected: None,
    }
}

#[test]
fn an_unglued_thought_leaves_its_frames_held_and_a_later_task_closes_on_one() {
    let a = g(3, 3, &[1, 2, 0, 0, 3, 0, 4, 0, 5]);
    let b = g(3, 3, &[0, 6, 0, 7, 0, 0, 0, 1, 8]);
    let c = g(3, 3, &[2, 0, 0, 0, 0, 9, 3, 4, 0]);
    // Two observations that disagree: one rotated, one flipped.
    let disagreeing = task(
        vec![
            pair(a.clone(), &[Op::Rotate90]),
            pair(b.clone(), &[Op::FlipH]),
        ],
        c.clone(),
    );
    let mut grown = Vec::new();
    let mut held: Vec<Conjecture> = Vec::new();
    assert!(
        matches!(
            think_task_held(&disagreeing, &mut grown, &mut held),
            SolveRead::Unsolved
        ),
        "no single rule: no claim"
    );
    assert!(
        !held.is_empty(),
        "the frames that closed some pairs are held"
    );
    assert!(held.iter().all(|h| h.of == 2 && h.closed == 1));

    // A later task the held rotation closes whole: answered from it, and it
    // leaves the held set (it is no longer a conjecture).
    let rotation = held
        .iter()
        .find(|h| run(&h.program, &c) == run(&vec![Op::Rotate90], &c))
        .cloned()
        .expect("the rotation is held");
    let later = task(
        vec![
            pair(b.clone(), &[Op::Rotate90]),
            pair(c.clone(), &[Op::Rotate90]),
        ],
        a.clone(),
    );
    let before = held.len();
    match think_task_held(&later, &mut grown, &mut held) {
        SolveRead::Solved {
            program,
            test_output,
            ..
        } => {
            assert_eq!(program, rotation.program, "the held conjecture answered");
            assert_eq!(test_output, run(&program, &a));
            assert!(
                later
                    .train
                    .iter()
                    .all(|p| run(&program, &p.input) == p.output),
                "Pin A: the claim closes"
            );
        }
        SolveRead::Unsolved => panic!("a held conjecture closes this task"),
    }
    assert_eq!(
        held.len(),
        before - 1,
        "the answered conjecture leaves the held set"
    );
    assert!(!held.contains(&rotation));
}

#[test]
fn a_conjecture_that_does_not_close_claims_nothing() {
    let a = g(2, 3, &[1, 2, 3, 4, 5, 6]);
    let b = g(2, 3, &[6, 5, 4, 3, 2, 1]);
    let mut grown = Vec::new();
    let mut held = vec![Conjecture {
        program: vec![Op::Transpose],
        closed: 1,
        of: 2,
    }];
    // A task the held transpose does not close (and the organ cannot glue).
    let t = task(
        vec![pair(a.clone(), &[Op::FlipV]), pair(b.clone(), &[Op::FlipH])],
        a.clone(),
    );
    assert!(matches!(
        think_task_held(&t, &mut grown, &mut held),
        SolveRead::Unsolved
    ));
    assert!(
        held.iter().any(|h| h.program == vec![Op::Transpose]),
        "an untested conjecture is kept, never claimed"
    );
}
