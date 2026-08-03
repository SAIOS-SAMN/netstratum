//! The harness pins: the verifier is the authority, the withhold is
//! honest, the certificate replays, and the no-hardcoding law is
//! structural (every task below is constructed in the test — the
//! solver code never sees a name it could branch on).

use solver::{
    load_task, solve, verify_certificate, CertificateRead, Grid, Op, SolveRead, Task, TaskPair,
};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("well-formed")
}

fn task_from(op_word: &[Op], inputs: &[Grid], test_in: Grid) -> Task {
    let render = |g: &Grid| {
        let mut out = g.clone();
        for op in op_word {
            out = op.apply(&out);
        }
        out
    };
    Task {
        name: "constructed".into(),
        train: inputs
            .iter()
            .map(|input| TaskPair {
                input: input.clone(),
                output: render(input),
            })
            .collect(),
        test_expected: Some(render(&test_in)),
        test_in,
    }
}

/// A rotation task solves, the render matches the held-out answer, and
/// the certificate replays intact.
#[test]
fn a_rotation_task_closes_and_proves() {
    let task = task_from(
        &[Op::Rotate90],
        &[grid(2, 3, &[1, 2, 3, 4, 5, 6]), grid(2, 2, &[7, 0, 0, 7])],
        grid(2, 3, &[9, 8, 7, 6, 5, 4]),
    );
    match solve(&task, 2) {
        SolveRead::Solved {
            program,
            test_output,
            matches_expected,
            certificate,
        } => {
            assert_eq!(matches_expected, Some(true));
            assert_eq!(
                verify_certificate(&certificate, &task, &program, &test_output),
                CertificateRead::Intact
            );
        }
        SolveRead::Unsolved => panic!("a one-op rotation must close at depth 2"),
    }
}

/// A recolor task closes through the derived-operator path — the map
/// is read from the observation, never from a rule table.
#[test]
fn a_recolor_task_closes_via_observation() {
    let task = task_from(
        &[Op::Recolor([0, 5, 2, 3, 4, 1, 6, 7, 8, 9])], // 1↔5 swap
        &[grid(2, 2, &[1, 5, 1, 0]), grid(1, 3, &[5, 1, 5])],
        grid(2, 2, &[0, 1, 5, 1]),
    );
    assert!(matches!(
        solve(&task, 1),
        SolveRead::Solved {
            matches_expected: Some(true),
            ..
        }
    ));
}

/// A composed word (flip then recolor) closes at depth 2 — programs
/// compose; the vocabulary does not grow per task.
#[test]
fn a_two_op_word_closes_at_depth_two() {
    let task = task_from(
        &[Op::FlipH, Op::Recolor([0, 2, 1, 3, 4, 5, 6, 7, 8, 9])],
        &[grid(2, 2, &[1, 2, 0, 1]), grid(2, 3, &[1, 0, 2, 2, 1, 0])],
        grid(2, 2, &[2, 2, 1, 0]),
    );
    assert!(matches!(
        solve(&task, 2),
        SolveRead::Solved {
            matches_expected: Some(true),
            ..
        }
    ));
}

/// The honest withhold: a task outside the vocabulary reads Unsolved —
/// never a guess.
#[test]
fn an_unreachable_task_reads_unsolved() {
    // Output shape 1×3 from input shape 2×2 is unreachable by the
    // whole vocabulary: symmetries and recolors preserve the cell
    // count, tiles multiply the dimensions — no word renders 1×3.
    let task = Task {
        name: "constructed-unreachable".into(),
        train: vec![TaskPair {
            input: grid(2, 2, &[1, 2, 1, 4]),
            output: grid(1, 3, &[5, 5, 5]),
        }],
        test_in: grid(2, 2, &[5, 6, 7, 8]),
        test_expected: None,
    };
    assert!(matches!(solve(&task, 3), SolveRead::Unsolved));
}

/// THE VACUOUS SOLVE, REFUSED (operator ruling C8, U_DIRECTIVES §10 S1).
/// An empty training set proves nothing, so the empty program "closes"
/// it vacuously — `train.iter().all(..)` is true over no pairs. That is
/// a solve claim nobody earned. The honest withhold outranks it: with
/// nothing to close, there is no solution to report.
#[test]
fn an_empty_training_set_never_solves() {
    let task = Task {
        name: "constructed-empty-train".into(),
        train: Vec::new(),
        test_in: grid(2, 2, &[1, 2, 3, 4]),
        test_expected: None,
    };
    // Depth 0 is the sharp case: the frontier IS the empty program.
    assert!(matches!(solve(&task, 0), SolveRead::Unsolved));
    assert!(matches!(solve(&task, 3), SolveRead::Unsolved));
}

/// A tampered certificate names the diverged link.
#[test]
fn a_tampered_certificate_diverges_typed() {
    let task = task_from(&[Op::FlipV], &[grid(2, 1, &[3, 7])], grid(2, 1, &[1, 2]));
    let SolveRead::Solved {
        program,
        test_output,
        mut certificate,
        ..
    } = solve(&task, 1)
    else {
        panic!("closes");
    };
    certificate.output[0] ^= 1;
    assert!(matches!(
        verify_certificate(&certificate, &task, &program, &test_output),
        CertificateRead::Diverged { .. }
    ));
}

/// The loader round-trips the archived pair format (fixture copied
/// whole from the provenance-verified v0.4 corpus — data, never fed:
/// deleting it changes no code path).
#[test]
fn the_loader_reads_the_archived_format() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/arc_00576224.txt");
    let task = load_task(&path).expect("parses");
    assert_eq!(task.name, "arc_00576224");
    assert_eq!(task.train.len(), 2);
    assert_eq!(task.train[0].input.rows, 2);
    assert_eq!(task.train[0].output.rows, 6);
    assert!(task.test_expected.is_some());
}

/// The quilt family: the archived task 00576224 (3×3 tiling with
/// alternating vertical mirror) closes through the derived-tile path
/// and matches its held-out answer exactly.
#[test]
fn the_archived_quilt_task_confirms() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/arc_00576224.txt");
    let task = load_task(&path).expect("parses");
    match solve(&task, 2) {
        SolveRead::Solved {
            matches_expected, ..
        } => assert_eq!(matches_expected, Some(true)),
        SolveRead::Unsolved => panic!("the quilt closes via Tile"),
    }
}
