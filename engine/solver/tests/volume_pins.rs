//! The volumetric pins: determinism, fair-depth solvability, the
//! stack operators' algebra, the honest withhold, and perception
//! separation (a volumetric forge task is NOT a planar task).

use solver::{
    forge_volume, run_volume, solve_volume, Grid, Op, VOp, VolDifficulty, Volume,
    VolumeSolveRead,
};

fn volume(layers: Vec<Vec<u8>>, rows: usize, cols: usize) -> Volume {
    Volume::new(
        layers
            .into_iter()
            .map(|cells| Grid::new(rows, cols, cells).expect("well-formed"))
            .collect(),
    )
    .expect("congruent")
}

/// Same digest + same rung → byte-identical volumetric task.
#[test]
fn the_volumetric_forge_is_a_reading_of_history() {
    for level in 1..=5 {
        let d = VolDifficulty::level(level);
        assert_eq!(
            forge_volume([21u8; 32], d),
            forge_volume([21u8; 32], d),
            "level {level} forges deterministically"
        );
    }
    assert_ne!(
        forge_volume([21u8; 32], VolDifficulty::level(2)),
        forge_volume([22u8; 32], VolDifficulty::level(2)),
        "a different chronicle forges a different task"
    );
}

/// Fairness: forged volumetric tasks close at their stated depth and
/// match the held-out answer.
#[test]
fn forged_volumes_close_at_fair_depth() {
    for seed in 0..6u8 {
        let d = VolDifficulty::level(1 + (seed % 3) as u32);
        let task = forge_volume([seed.wrapping_add(100); 32], d);
        match solve_volume(&task, d.word_len) {
            VolumeSolveRead::Solved {
                matches_expected, ..
            } => assert_eq!(matches_expected, Some(true), "seed {seed}"),
            VolumeSolveRead::Unsolved => panic!("seed {seed}: fair depth must close"),
        }
    }
}

/// The stack algebra: FlipStack is self-inverse; CycleLayers has
/// order = layer count; PerLayer commutes with FlipStack.
#[test]
fn the_stack_operators_hold_their_algebra() {
    let v = volume(
        vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 0, 1, 2]],
        2,
        2,
    );
    let flip = |x: &Volume| VOp::FlipStack.apply(x);
    assert_eq!(flip(&flip(&v)), v, "flip_stack ∘ flip_stack = identity");

    let cycled = run_volume(
        &vec![VOp::CycleLayers, VOp::CycleLayers, VOp::CycleLayers],
        &v,
    );
    assert_eq!(cycled, v, "cycle³ = identity on three layers");

    let a = run_volume(
        &vec![VOp::PerLayer(Op::Rotate180), VOp::FlipStack],
        &v,
    );
    let b = run_volume(
        &vec![VOp::FlipStack, VOp::PerLayer(Op::Rotate180)],
        &v,
    );
    assert_eq!(a, b, "per-layer ops commute with the stack mirror");
}

/// The honest withhold: an output with a different layer COUNT is
/// unreachable by the whole vocabulary (every operator preserves the
/// layer count) — Unsolved, never a guess.
#[test]
fn an_unreachable_volume_reads_unsolved() {
    let task = solver::VolumeTask {
        name: "constructed-unreachable".into(),
        train: vec![(
            volume(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]], 2, 2),
            volume(vec![vec![1, 2, 3, 4]], 2, 2),
        )],
        test_in: volume(vec![vec![0, 0, 0, 0], vec![1, 1, 1, 1]], 2, 2),
        test_expected: None,
    };
    assert!(matches!(
        solve_volume(&task, 3),
        VolumeSolveRead::Unsolved
    ));
}

/// Provenance: the volumetric name carries the volf_ prefix, layer
/// depth, and word length — an observer separates volumetric solves
/// from planar ones by name alone.
#[test]
fn the_volumetric_name_carries_provenance() {
    let task = forge_volume([0xCD; 32], VolDifficulty::level(4));
    assert!(task.name.starts_with("volf_cdcdcdcdcdcd_d4w3"), "{}", task.name);
}
