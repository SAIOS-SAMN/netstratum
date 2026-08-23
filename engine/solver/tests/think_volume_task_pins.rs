//! VOLUMETRIC-1d witness — the volumetric seat conductor `think_volume_task`,
//! the mirror of `think_task_pins.rs`. It solves ONLY by rendering a frame
//! the ONE reduce organ GLUED across the train pairs, and withholds
//! (Unsolved, no output) on every other verdict.

use reduct::Crystal;
use solver::think::think_volume_task;
use solver::volume::VolumeSolveRead;
use solver_verify::grid::{Grid, Op};
use solver_verify::volume::{run_volume, VOp, Volume, VolumeTask};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("lawful grid")
}

fn volume(layers: Vec<Grid>) -> Volume {
    Volume::new(layers).expect("congruent layers")
}

fn task(train: Vec<(Volume, Volume)>, test_in: Volume, test_expected: Option<Volume>) -> VolumeTask {
    VolumeTask { name: "witness".into(), train, test_in, test_expected }
}

/// Apply a single VProgram to build an expected output — the test never
/// hand-writes a transformed volume, it uses the same vocabulary the seat
/// renders into.
fn under(program: &[VOp], v: &Volume) -> Volume {
    run_volume(&program.to_vec(), v)
}

#[test]
fn a_per_layer_reflection_task_glues_and_solves() {
    // REPEATED hues (not distinct singletons per cell) so the plane is
    // LOAD-BEARING — the exact discipline think_task_pins.rs's own reflection
    // test uses, and the fix VOLUMETRIC-1c's own render test needed once it
    // hit the ambiguity a 4-distinct-singleton probe creates.
    let in1 = volume(vec![
        grid(3, 3, &[1, 1, 0, 1, 0, 0, 0, 0, 0]),
        grid(3, 3, &[0, 1, 1, 0, 0, 1, 0, 0, 0]),
    ]);
    let in2 = volume(vec![
        grid(3, 3, &[2, 0, 0, 2, 2, 0, 0, 0, 0]),
        grid(3, 3, &[0, 0, 2, 0, 2, 2, 0, 0, 0]),
    ]);
    let test_in = volume(vec![
        grid(3, 3, &[1, 0, 0, 1, 1, 0, 0, 0, 1]),
        grid(3, 3, &[0, 0, 1, 0, 1, 1, 1, 0, 0]),
    ]);
    let flip = [VOp::PerLayer(Op::FlipH)];
    let t = task(
        vec![(in1.clone(), under(&flip, &in1)), (in2.clone(), under(&flip, &in2))],
        test_in.clone(),
        Some(under(&flip, &test_in)),
    );
    let mut grown: Vec<Crystal> = Vec::new();
    match think_volume_task(&t, &mut grown) {
        VolumeSolveRead::Solved { program, test_output, matches_expected, certificate } => {
            assert_eq!(
                program,
                vec![VOp::PerLayer(Op::FlipH)],
                "the frame renders to a single per-layer reflection op"
            );
            assert_eq!(test_output, under(&flip, &test_in), "the render carries the test input");
            assert_eq!(matches_expected, Some(true), "and it matches the held-out answer");
            assert_eq!(
                solver_verify::volume::verify_volume_certificate(&certificate, &t, &program, &test_output),
                solver_verify::volume::VolumeCertificateRead::Intact,
                "the proof chain verifies (Pin A)",
            );
            assert!(grown.is_empty(), "a gauge task grows no crystal");
        }
        VolumeSolveRead::Unsolved => panic!("a glued per-layer reflection must solve"),
    }
}

#[test]
fn a_stack_move_task_honestly_withholds() {
    // FlipStack is outside the PerLayer render's reach (VOLUMETRIC-1c) — the
    // seat must withhold rather than emit a wrong program.
    let in1 = volume(vec![
        grid(2, 2, &[1, 1, 0, 1]),
        grid(2, 2, &[0, 1, 1, 0]),
    ]);
    let in2 = volume(vec![
        grid(2, 2, &[2, 0, 2, 2]),
        grid(2, 2, &[0, 2, 0, 2]),
    ]);
    let test_in = volume(vec![
        grid(2, 2, &[1, 0, 1, 1]),
        grid(2, 2, &[1, 1, 0, 0]),
    ]);
    let stack = [VOp::FlipStack];
    let t = task(
        vec![(in1.clone(), under(&stack, &in1)), (in2.clone(), under(&stack, &in2))],
        test_in.clone(),
        Some(under(&stack, &test_in)),
    );
    let mut grown: Vec<Crystal> = Vec::new();
    assert!(
        matches!(think_volume_task(&t, &mut grown), VolumeSolveRead::Unsolved),
        "a stack move honestly withholds rather than being invented"
    );
}

#[test]
fn a_single_train_pair_is_insufficient() {
    let a = volume(vec![grid(2, 2, &[1, 1, 0, 1])]);
    let b = under(&[VOp::PerLayer(Op::Rotate90)], &a);
    let t = task(vec![(a.clone(), b)], a.clone(), None);
    let mut grown: Vec<Crystal> = Vec::new();
    assert!(
        matches!(think_volume_task(&t, &mut grown), VolumeSolveRead::Unsolved),
        "one observation is insufficient, exactly like the planar seat (Pin D)"
    );
}
