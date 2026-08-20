//! **THE TILE SEAT** (#131) — net's ARC seat now expresses the first frame
//! MORPHISM: a tiling. The render-expressiveness ceiling (D₄×recolor is a
//! composition-closed group, so no rung of that curriculum mints again) is
//! lifted — a tier-3 tile task now PROVES, so its pair crosses the membrane and
//! the mesh installs the first non-D₄ net word. These witnesses pin: a pure
//! tiling solves, a tiling AFTER a symmetry solves (tile-last with a D₄ base),
//! and a plain D₄ task still solves through the same-frame seat (no regression).
//!
//! The base grids are 3×3 CHIRAL (repeated hues, no mirror symmetry) so the
//! plane is load-bearing — on all-distinct hues a rotation reads as a recolor
//! and would not glue (the organ's honest ambiguity, pinned in think_task_pins).

use solver::search::SolveRead;
use solver::{forge_task, think_task, Difficulty};
use solver_verify::grid::{run, Grid, Op, TileMirror};
use solver_verify::loader::{Task, TaskPair};

fn g(rows: usize, cols: usize, cells: Vec<u8>) -> Grid {
    Grid::new(rows, cols, cells).expect("lawful test grid")
}

/// Forge a task from a known secret word applied to a few `rows×cols` inputs
/// (last input is the held-out test).
fn task_from(rows: usize, cols: usize, word: &[Op], inputs: &[Vec<u8>]) -> Task {
    let mk = |cells: &Vec<u8>| g(rows, cols, cells.clone());
    let train: Vec<TaskPair> = inputs[..inputs.len() - 1]
        .iter()
        .map(|c| {
            let input = mk(c);
            let output = run(&word.to_vec(), &input);
            TaskPair { input, output }
        })
        .collect();
    let test_in = mk(inputs.last().unwrap());
    let test_expected = Some(run(&word.to_vec(), &test_in));
    Task { name: "tile_probe".into(), train, test_in, test_expected }
}

/// The chiral 3×3 inputs (mirror the think_task_pins reflection witness).
fn chiral() -> Vec<Vec<u8>> {
    vec![
        vec![1, 1, 0, 1, 0, 0, 0, 0, 0],
        vec![2, 0, 0, 2, 2, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 1, 0, 0, 0, 1],
    ]
}

fn assert_solves(task: &Task) {
    let mut grown = Vec::new();
    match think_task(task, &mut grown) {
        SolveRead::Solved { program, matches_expected, .. } => {
            assert_eq!(matches_expected, Some(true), "the word solves the held-out test");
            for pair in &task.train {
                assert_eq!(run(&program, &pair.input), pair.output, "Pin A: reproduces every train pair");
            }
        }
        _ => panic!("the seat withheld a task it should prove"),
    }
}

#[test]
fn the_tile_seat_proves_a_pure_tiling() {
    // output = Tile 2×2 of the input, odd bands mirrored — a frame CHANGE the
    // same-frame D₄ seat cannot see. Base transform is the identity.
    let word = [Op::Tile { down: 2, across: 2, odd_row: TileMirror::Rows, odd_col: TileMirror::Cols }];
    assert_solves(&task_from(3, 3, &word, &chiral()));
}

#[test]
fn the_tile_seat_proves_a_tiling_after_a_symmetry() {
    // Tile-LAST with a D₄ base: flip the input, then tile it 2×1. The base
    // recognition must glue FlipV (hence the chiral grids), then the tile wraps.
    let word = [
        Op::FlipV,
        Op::Tile { down: 2, across: 1, odd_row: TileMirror::None, odd_col: TileMirror::None },
    ];
    assert_solves(&task_from(3, 3, &word, &chiral()));
}

#[test]
fn a_plain_d4_task_still_solves_through_the_same_frame_seat() {
    // No regression: a same-frame rotation proves as before (the tile seat is
    // never consulted — the output shares the input's frame).
    let word = [Op::Rotate180];
    assert_solves(&task_from(3, 3, &word, &chiral()));
}

#[test]
fn a_forged_tier3_tile_task_now_proves_and_crosses() {
    // THE ACCEPTANCE (#131): over real forge draws at the tier-3 rung, at least
    // one TILE task now proves with a Tile in its word — before the tile seat,
    // every tile task withheld (that was the render-expressiveness ceiling). A
    // fresh mind per task (no shared basin) isolates the render capability.
    let mut tile_solves = 0;
    for n in 0..64u8 {
        let task = forge_task([n; 32], Difficulty::level(4)); // tier 3, word_len 2
        let mut grown = Vec::new();
        if let SolveRead::Solved { program, matches_expected: Some(true), .. } =
            think_task(&task, &mut grown)
        {
            if program.iter().any(|op| matches!(op, Op::Tile { .. })) {
                tile_solves += 1;
            }
        }
    }
    assert!(tile_solves > 0, "a real forge tile task must now prove; got {tile_solves}");
}
