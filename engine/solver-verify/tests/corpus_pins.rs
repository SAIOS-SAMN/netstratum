//! WORK-SHAPES v2 — corpus payloads, pinned.
//!
//! **THE ADMISSION PINS WERE STRUCK 2026-08-04** with the Merkle
//! mechanism they tested (operator ruling: no Merkle trees). Gone:
//! `a_self_authored_task_refuses`, `the_root_commits_to_the_set_and_
//! every_member_proves_it`, `the_tree_separates_its_domains`,
//! `an_empty_path_proves_nothing_in_a_real_corpus`, and
//! `an_admissible_corpus_solve_verifies_and_reports_its_class`.
//!
//! The first of those was called load-bearing in its own landing, and
//! it was — for the mechanism it guarded. The FINDING it protected
//! stands and is not struck: `task_id = H(content)` proves WHICH task
//! was solved and never that the task was admissible. **Identity is not
//! admission.** What went is the answer, not the question, and until
//! the replacement predicate lands (base-fails-at-depth-d ∧
//! candidate-closes) this judge does not admit at all — lawfully, since
//! corpus work has no rate and THE MOTION clause 4 makes imported
//! content a benchmark, never a mint source.
//!
//! Pinning a strike rather than deleting quietly: a pin that tests a
//! struck mechanism is worse than no pin, because it reads as coverage.

use solver_verify::corpus;
use solver_verify::work::{
    canonical_name, decode_corpus_work, encode_corpus_work, verify_corpus_work, verify_work,
    verify_work_class, CorpusProof, WorkBroken, WorkClass, WORK_SCHEMA_V2_CORPUS,
};
use solver_verify::{
    encode_work, forge_task, run, Difficulty, Grid, Op, SolveCertificate, Task, TaskPair,
    WorkProof,
};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("well-formed")
}

fn built_corpus() -> Vec<Task> {
    let make = |n: u8, op: Op| {
        let inputs = [
            grid(2, 2, &[n, n + 1, n + 2, n + 3]),
            grid(2, 2, &[n + 4, n + 5, n + 6, n + 1]),
        ];
        let train: Vec<TaskPair> = inputs
            .iter()
            .map(|g| TaskPair { input: g.clone(), output: op.apply(g) })
            .collect();
        Task {
            name: format!("built-{n}"),
            train,
            test_in: grid(2, 2, &[1, 2, 3, n]),
            test_expected: None,
        }
    };
    vec![make(0, Op::Rotate90), make(1, Op::Rotate180), make(2, Op::FlipH)]
}

/// Seal a proof for `task` under `program` — the prover's half.
fn proof_for(task: &Task, program: Vec<Op>) -> CorpusProof {
    let task_id = corpus::task_digest(&task.train, &task.test_in);
    let output = run(&program, &task.test_in);
    let sealed = Task {
        name: canonical_name(&task_id),
        train: task.train.clone(),
        test_in: task.test_in.clone(),
        test_expected: None,
    };
    let certificate = SolveCertificate::seal(&sealed, &program, &output);
    CorpusProof {
        task_id,
        train: task.train.clone(),
        test_in: task.test_in.clone(),
        program,
        output,
        certificate,
    }
}

/// The identity of a task is its CONTENT — not its name, and not the
/// held-out answer. SURVIVES THE STRIKE unchanged: this is what binds a
/// payload to the task it claims, which is all hashing does here now.
#[test]
fn the_task_id_is_content_alone() {
    let tasks = built_corpus();
    let base = &tasks[0];
    let id = corpus::task_digest(&base.train, &base.test_in);

    let renamed = Task { name: "something else entirely".into(), ..base.clone() };
    assert_eq!(
        corpus::task_digest(&renamed.train, &renamed.test_in),
        id,
        "a filename is provenance, never content"
    );

    let answered = Task { test_expected: Some(grid(1, 1, &[7])), ..base.clone() };
    assert_eq!(
        corpus::task_digest(&answered.train, &answered.test_in),
        id,
        "the held-out answer is not part of the task's identity"
    );

    let altered = Task { test_in: grid(2, 2, &[9, 9, 9, 9]), ..base.clone() };
    assert_ne!(
        corpus::task_digest(&altered.train, &altered.test_in),
        id,
        "changing an observation changes the task"
    );
}

/// IDENTITY IS NOT ADMISSION, and the judge no longer pretends
/// otherwise. A task nobody authorized now verifies — because there is
/// no admission test, and saying so out loud is the point of this pin.
/// It earns nothing: corpus work has no rate (THE MOTION clause 4).
///
/// When the replacement predicate lands (the base vocabulary provably
/// fails at depth d ∧ the candidate closes), THIS PIN SHOULD FLIP and a
/// self-authored trivial task should refuse again on a re-checkable
/// ground rather than a pinned list.
#[test]
fn a_self_authored_task_verifies_and_that_is_now_explicit() {
    let mine = Task {
        name: "my own easy task".into(),
        train: vec![TaskPair {
            input: grid(2, 2, &[1, 2, 3, 4]),
            output: Op::Rotate90.apply(&grid(2, 2, &[1, 2, 3, 4])),
        }],
        test_in: grid(2, 2, &[5, 6, 7, 8]),
        test_expected: None,
    };
    let proof = proof_for(&mine, vec![Op::Rotate90]);
    let want = proof.task_id;
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert_eq!(
        verify_corpus_work(&bytes),
        Ok(WorkClass::Corpus { task_id: want }),
        "the judge proves internal consistency and replay, never provenance"
    );
}

/// A payload that names one task and carries another refuses. This is
/// the whole of what `task_id` still enforces, and it is real.
#[test]
fn a_payload_that_lies_about_its_content_refuses() {
    let tasks = built_corpus();
    let honest = proof_for(&tasks[0], vec![Op::Rotate90]);
    let swapped = CorpusProof { test_in: grid(2, 2, &[9, 9, 9, 9]), ..honest };
    let bytes = encode_corpus_work(&swapped).expect("encodes");
    assert_eq!(verify_corpus_work(&bytes), Err(WorkBroken::TaskIdMismatch));
}

/// The honest-withhold law: a program that misses one training pair is
/// not a solution, and the miss is named.
#[test]
fn a_training_miss_refuses_and_names_the_pair() {
    let tasks = built_corpus();
    let wrong = proof_for(&tasks[0], vec![Op::Rotate180]);
    let bytes = encode_corpus_work(&wrong).expect("encodes");
    match verify_corpus_work(&bytes) {
        Err(WorkBroken::TrainingMiss { pair }) => assert_eq!(pair, 0),
        other => panic!("a miss must refuse typed, got {other:?}"),
    }
}

/// An honest corpus solve replays whole and reports its class.
#[test]
fn the_whole_green_path_verifies() {
    let tasks = built_corpus();
    for (n, task) in tasks.iter().enumerate() {
        let op = [Op::Rotate90, Op::Rotate180, Op::FlipH][n].clone();
        let proof = proof_for(task, vec![op]);
        let want = proof.task_id;
        let bytes = encode_corpus_work(&proof).expect("encodes");
        assert_eq!(
            verify_corpus_work(&bytes),
            Ok(WorkClass::Corpus { task_id: want }),
            "task {n} replays exactly and chains intact"
        );
    }
}

/// THE VACUITY, refused for this shape too (98ccbbe's law).
#[test]
fn an_empty_training_set_refuses_on_the_wire() {
    let tasks = built_corpus();
    let honest = proof_for(&tasks[0], vec![Op::Rotate90]);
    let mut bytes = encode_corpus_work(&honest).expect("encodes");
    // schema(1) + task_id(32), then the train count (u16 LE). The
    // inclusion path that used to sit between them is struck.
    let at = 1 + 32;
    bytes[at] = 0;
    bytes[at + 1] = 0;
    assert!(
        decode_corpus_work(&bytes).is_none(),
        "no training pair means nothing to close — refused as form"
    );
}

/// The wire round-trips exactly, and refuses trailing or missing bytes.
#[test]
fn the_corpus_payload_round_trips_and_is_exact() {
    let tasks = built_corpus();
    let proof = proof_for(&tasks[1], vec![Op::Rotate180]);
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert_eq!(bytes[0], WORK_SCHEMA_V2_CORPUS, "the schema byte leads");
    assert_eq!(decode_corpus_work(&bytes).as_ref(), Some(&proof));

    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_corpus_work(&trailing).is_none(), "trailing bytes refuse");

    let mut truncated = bytes.clone();
    truncated.pop();
    assert!(decode_corpus_work(&truncated).is_none(), "truncation refuses");
}

/// THE SCHEMAS DO NOT COLLIDE.
#[test]
fn v1_is_untouched_and_the_shapes_never_confuse() {
    let difficulty = Difficulty::level(3);
    let task = forge_task([21u8; 32], difficulty);
    let program = vec![Op::Rotate90];
    let output = run(&program, &task.test_in);
    let v1 = WorkProof {
        forge_digest: [21u8; 32],
        difficulty,
        program: program.clone(),
        output: output.clone(),
        certificate: SolveCertificate::seal(&task, &program, &output),
    };
    let v1_bytes = encode_work(&v1).expect("encodes");
    assert_eq!(v1_bytes[0], 1, "v1 keeps its schema byte");

    let tasks = built_corpus();
    let v2_bytes = encode_corpus_work(&proof_for(&tasks[0], vec![Op::Rotate90])).expect("encodes");
    assert_eq!(verify_work(&v2_bytes), Err(WorkBroken::UnsupportedShape));
    assert!(decode_corpus_work(&v1_bytes).is_none(), "v1 is not a corpus proof");
    assert!(solver_verify::decode_work(&v2_bytes).is_none(), "v2 is not a forge proof");

    match verify_work_class(&v1_bytes) {
        Ok(WorkClass::Forge(d)) => assert_eq!(d, difficulty),
        Err(WorkBroken::TrainingMiss { .. })
        | Err(WorkBroken::OutputMismatch)
        | Err(WorkBroken::ChainDiverged { .. }) => {}
        other => panic!("v1 must be judged as forge work, got {other:?}"),
    }
    assert_eq!(
        verify_work(&v1_bytes),
        verify_work_class(&v1_bytes).map(|c| match c {
            WorkClass::Forge(d) => d,
            other => unreachable!("v1 is forge work, never {other:?}"),
        }),
        "one implementation, two readings — the doors cannot drift"
    );
    assert_eq!(verify_work_class(&[9, 9, 9]), Err(WorkBroken::Malformed));
    assert_eq!(verify_work_class(&[]), Err(WorkBroken::Malformed));
}

/// A tampered chain refuses and names its break.
#[test]
fn a_tampered_corpus_chain_names_its_break() {
    let tasks = built_corpus();
    let mut proof = proof_for(&tasks[0], vec![Op::Rotate90]);
    proof.certificate.output[0] ^= 1;
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert!(matches!(
        verify_corpus_work(&bytes),
        Err(WorkBroken::ChainDiverged { .. })
    ));
}
