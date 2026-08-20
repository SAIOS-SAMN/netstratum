//! N-I3 Slice 3 witness — the ARC seat conductor `think_task`. It solves ONLY
//! by rendering a frame the ONE reduce organ GLUED across the train pairs, and
//! withholds (Unsolved, no output) on every other verdict. Pin A: a Solved
//! program reproduces every train output AND its certificate verifies Intact.
//! Pin D: ByDrain / Unglueable / Withheld / Insufficient emit no grid.

use reduct::Crystal;
use solver::{forge_task, think_task, Difficulty, Grid, Op, SolveCertificate, SolveRead, Task, TaskPair};
use solver_verify::certificate::{verify_certificate, CertificateRead};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("lawful grid")
}

fn task(train: Vec<(Grid, Grid)>, test_in: Grid, test_expected: Option<Grid>) -> Task {
    Task {
        name: "witness".into(),
        train: train.into_iter().map(|(input, output)| TaskPair { input, output }).collect(),
        test_in,
        test_expected,
    }
}

/// Apply a single op to build an expected output — the test never hand-writes a
/// transformed grid, it uses the same vocabulary the seat renders into.
fn under(op: &Op, g: &Grid) -> Grid {
    op.apply(g)
}

#[test]
fn a_reflection_task_glues_and_solves() {
    // Two DIFFERENT chiral inputs (repeated hues, no mirror symmetry), each
    // reflected left↔right. Here the plane is LOAD-BEARING: FlipH cannot be
    // absorbed into a recolor (a recolor preserves which cells are equal), so
    // both pairs deduce (plane FlipH, identity relabel) and glue one frame — the
    // render exercises `render_plane`, not just the recolor arm.
    let in1 = grid(3, 3, &[1, 1, 0, 1, 0, 0, 0, 0, 0]);
    let in2 = grid(3, 3, &[2, 0, 0, 2, 2, 0, 0, 0, 0]);
    let test_in = grid(3, 3, &[1, 0, 0, 1, 1, 0, 0, 0, 1]);
    let t = task(
        vec![(in1.clone(), under(&Op::FlipH, &in1)), (in2.clone(), under(&Op::FlipH, &in2))],
        test_in.clone(),
        Some(under(&Op::FlipH, &test_in)),
    );
    let mut grown: Vec<Crystal> = Vec::new();
    match think_task(&t, &mut grown) {
        SolveRead::Solved { program, test_output, matches_expected, certificate } => {
            assert_eq!(program, vec![Op::FlipH], "the frame renders to a single reflection op");
            assert_eq!(test_output, under(&Op::FlipH, &test_in), "the render carries the test input");
            assert_eq!(matches_expected, Some(true), "and it matches the held-out answer");
            assert_eq!(
                verify_certificate(&certificate, &t, &program, &test_output),
                CertificateRead::Intact,
                "the proof chain verifies (Pin A)",
            );
            // The seat expressed a nameable frame — it grew nothing.
            assert!(grown.is_empty(), "a gauge task grows no crystal");
        }
        SolveRead::Unsolved => panic!("a glued reflection must solve"),
    }
}

#[test]
fn a_rainbow_rotation_reads_as_recolor_and_does_not_glue() {
    // A subtle, honest truth of the organ: on a 2×2 of all-distinct hues,
    // rotating is INDISTINGUISHABLE from recoloring (moving four unique cells is
    // the same as relabeling four unique hues). So each "rotation" pair deduces
    // an identity-plane RECOLOR — and two different rainbows deduce DIFFERENT
    // recolors, which do not glue. The seat withholds rather than force a rule.
    let in1 = grid(2, 2, &[0, 1, 2, 3]);
    let in2 = grid(2, 2, &[1, 2, 3, 0]);
    let t = task(
        vec![(in1.clone(), under(&Op::Rotate90, &in1)), (in2.clone(), under(&Op::Rotate90, &in2))],
        in1,
        None,
    );
    let mut grown: Vec<Crystal> = Vec::new();
    assert!(
        matches!(think_task(&t, &mut grown), SolveRead::Unsolved),
        "inconsistent recolor readings do not glue — honest withhold",
    );
}

#[test]
fn a_recolor_task_glues_and_solves() {
    // Two inputs recolored by the SAME hue map (0→2, 1→3). A pure recolor is a
    // gauge move; the seat renders it to one Recolor op.
    let recolor = Op::Recolor([2, 3, 2, 3, 4, 5, 6, 7, 8, 9]);
    let in1 = grid(2, 2, &[0, 1, 0, 1]);
    let in2 = grid(2, 2, &[1, 1, 0, 0]);
    let test_in = grid(1, 3, &[0, 1, 0]);
    let t = task(
        vec![(in1.clone(), under(&recolor, &in1)), (in2.clone(), under(&recolor, &in2))],
        test_in.clone(),
        Some(under(&recolor, &test_in)),
    );
    let mut grown: Vec<Crystal> = Vec::new();
    match think_task(&t, &mut grown) {
        SolveRead::Solved { program, test_output, matches_expected, certificate } => {
            assert_eq!(test_output, grid(1, 3, &[2, 3, 2]), "0→2, 1→3 on the test input");
            assert_eq!(matches_expected, Some(true));
            assert_eq!(
                SolveCertificate::seal(&t, &program, &test_output),
                certificate,
                "the sealed certificate is the returned one",
            );
        }
        SolveRead::Unsolved => panic!("a glued recolor must solve"),
    }
}

#[test]
fn disagreeing_pairs_withhold_with_no_output() {
    // One pair rotates, the other flips — no single frame generalizes.
    let a = grid(2, 2, &[0, 1, 2, 3]);
    let t = task(
        vec![(a.clone(), under(&Op::Rotate90, &a)), (a.clone(), under(&Op::FlipH, &a))],
        a.clone(),
        None,
    );
    let mut grown: Vec<Crystal> = Vec::new();
    assert!(
        matches!(think_task(&t, &mut grown), SolveRead::Unsolved),
        "disagreeing frames withhold, never guess",
    );
}

#[test]
fn a_novel_hue_task_withholds_and_learns_then_drains_to_no_render() {
    // Pin B + Pin D at the seat: a novel-hue wall is learned (Withheld →
    // Unsolved, basis grows); the same class next drains (ByDrain), which is a
    // close but names no renderable frame — still Unsolved, no output.
    let inp = grid(2, 2, &[0, 1, 0, 1]);
    let out = grid(2, 2, &[0, 1, 0, 2]);
    let t = task(vec![(inp.clone(), out.clone()), (inp, out)], grid(2, 2, &[0, 1, 0, 1]), None);
    let mut grown: Vec<Crystal> = Vec::new();

    assert!(matches!(think_task(&t, &mut grown), SolveRead::Unsolved), "the wall withholds");
    assert_eq!(grown.len(), 1, "the seat learned one crystal for the class");

    assert!(
        matches!(think_task(&t, &mut grown), SolveRead::Unsolved),
        "the drained close names no frame — no output (Pin D)",
    );
    assert_eq!(grown.len(), 1, "draining grows nothing");
}

#[test]
fn pin_a_a_live_forge_draw_solves_with_an_intact_certificate() {
    // Pin A (N-I4, ON THE BEAT): a forge draw whose net rule is a frame
    // difference is Solved by the ONE organ with an Intact certificate — no
    // search anywhere in the path. `think_task` returns Solved only AFTER
    // `verify_certificate == Intact`, so a Solved forge draw IS an intact proof;
    // this re-checks it on the forged task the beat actually draws. Level 1
    // (word_len 1) draws are single frames — most solve; we assert at least one
    // does and that its proof verifies.
    let mut proven = false;
    for seed in 0u64..16 {
        let mut digest = [0u8; 32];
        digest[..8].copy_from_slice(&seed.to_le_bytes());
        let task = forge_task(digest, Difficulty::level(1));
        let mut grown: Vec<Crystal> = Vec::new();
        if let SolveRead::Solved { program, test_output, certificate, .. } =
            think_task(&task, &mut grown)
        {
            assert!(task.name.starts_with("forge_"), "a live forge draw, not a fixture");
            assert_eq!(
                verify_certificate(&certificate, &task, &program, &test_output),
                CertificateRead::Intact,
                "the beat's proof verifies (Pin A) — no search produced it",
            );
            proven = true;
            break;
        }
    }
    assert!(proven, "a level-1 forge draw is a frame difference the one organ solves on the beat");
}

#[test]
fn one_training_pair_is_insufficient() {
    let a = grid(2, 2, &[0, 1, 2, 3]);
    let t = task(vec![(a.clone(), under(&Op::Rotate90, &a))], a, None);
    let mut grown: Vec<Crystal> = Vec::new();
    assert!(
        matches!(think_task(&t, &mut grown), SolveRead::Unsolved),
        "one pair cannot tell a rule from a coincidence",
    );
}
