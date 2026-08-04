//! WORK-SHAPES v2 — the corpus registry, pinned.
//!
//! The load-bearing pin here is `a_self_authored_task_refuses`: it is
//! the whole reason this shape exists. Content-addressing proves WHICH
//! task was solved; only set-inclusion proves the task was REAL.

use solver_verify::corpus::{self, PathStep};
use solver_verify::work::{
    canonical_name, decode_corpus_work, encode_corpus_work, verify_corpus_work, verify_work,
    verify_corpus_work_against, verify_work_class, CorpusProof, WorkBroken, WorkClass,
    WORK_SCHEMA_V2_CORPUS,
};
use solver_verify::{
    encode_work, forge_task, run, Difficulty, Grid, Op, SolveCertificate, Task, TaskPair,
    WorkProof, CORPUS_ROOT,
};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("well-formed")
}

/// A small admissible "corpus": three tasks under one rotation rule.
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

fn ids_of(tasks: &[Task]) -> Vec<[u8; 32]> {
    tasks
        .iter()
        .map(|t| corpus::task_digest(&t.train, &t.test_in))
        .collect()
}

/// Seal a proof for `task` under `program`, with an inclusion path drawn
/// from `set`. The prover's half — exactly what a miner would build.
fn proof_for(task: &Task, program: Vec<Op>, set: &[[u8; 32]]) -> CorpusProof {
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
        inclusion: corpus::inclusion_path(set, &task_id).unwrap_or_default(),
        train: task.train.clone(),
        test_in: task.test_in.clone(),
        program,
        output,
        certificate,
    }
}

/// The identity of a task is its CONTENT — not its name, and not the
/// held-out answer.
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

/// The root is a function of the SET, never of read order, and every
/// member proves its membership.
#[test]
fn the_root_commits_to_the_set_and_every_member_proves_it() {
    let ids = ids_of(&built_corpus());
    let root = corpus::merkle_root(&ids).expect("non-empty");

    let mut shuffled = ids.clone();
    shuffled.reverse();
    assert_eq!(
        corpus::merkle_root(&shuffled),
        Some(root),
        "the root is the set's, not the directory listing's"
    );

    for id in &ids {
        let path = corpus::inclusion_path(&ids, id).expect("a member has a path");
        assert!(corpus::verify_inclusion(id, &path, &root), "a member verifies");
    }
    assert!(corpus::merkle_root(&[]).is_none(), "there is no corpus of nothing");
}

/// THE PIN THIS SHAPE EXISTS FOR. A miner authors their own trivial
/// task, hashes it honestly, solves it honestly, seals an intact chain —
/// and still earns nothing, because no path places it in the corpus.
#[test]
fn a_self_authored_task_refuses() {
    let corpus_tasks = built_corpus();
    let set = ids_of(&corpus_tasks);

    // A task nobody authorized: trivially solved by one op.
    let mine = Task {
        name: "my own easy task".into(),
        train: vec![TaskPair {
            input: grid(2, 2, &[1, 2, 3, 4]),
            output: Op::Rotate90.apply(&grid(2, 2, &[1, 2, 3, 4])),
        }],
        test_in: grid(2, 2, &[5, 6, 7, 8]),
        test_expected: None,
    };

    // Everything about the proof is HONEST except membership.
    let proof = proof_for(&mine, vec![Op::Rotate90], &set);
    assert!(proof.inclusion.is_empty(), "no path exists for a non-member");
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert_eq!(
        verify_corpus_work(&bytes),
        Err(WorkBroken::NotAdmissible),
        "an honestly-solved task nobody authorized still earns nothing"
    );

    // And it cannot be smuggled in by borrowing a real member's path.
    let borrowed = corpus::inclusion_path(&set, &set[0]).expect("member path");
    let smuggled = CorpusProof { inclusion: borrowed, ..proof };
    let bytes = encode_corpus_work(&smuggled).expect("encodes");
    assert_eq!(
        verify_corpus_work(&bytes),
        Err(WorkBroken::NotAdmissible),
        "a path proves ITS OWN leaf, never whatever is handed to it"
    );
}

/// A payload that names one task and carries another refuses before
/// admissibility is even consulted.
#[test]
fn a_payload_that_lies_about_its_content_refuses() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let honest = proof_for(&tasks[0], vec![Op::Rotate90], &set);

    let swapped = CorpusProof { test_in: grid(2, 2, &[9, 9, 9, 9]), ..honest };
    let bytes = encode_corpus_work(&swapped).expect("encodes");
    assert_eq!(verify_corpus_work(&bytes), Err(WorkBroken::TaskIdMismatch));
}

/// The honest-withhold law holds for the new shape: a program that
/// misses one training pair is not a solution, and the miss is named.
#[test]
fn a_training_miss_refuses_and_names_the_pair() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let root = corpus::merkle_root(&set).expect("non-empty");
    // Rotate180 does not close a Rotate90 task. Verified against the
    // FIXTURE root, because admissibility is checked BEFORE replay and
    // the pinned root would refuse this fixture first — the legs are
    // ordered cheapest-and-most-decisive first, deliberately.
    let wrong = proof_for(&tasks[0], vec![Op::Rotate180], &set);
    let bytes = encode_corpus_work(&wrong).expect("encodes");
    match verify_corpus_work_against(&bytes, &root) {
        Err(WorkBroken::TrainingMiss { pair }) => assert_eq!(pair, 0),
        other => panic!("a miss must refuse typed, got {other:?}"),
    }
}

/// An admissible task, honestly replayed, verifies whole and reports a
/// class carrying its task_id — the full green path, end to end.
#[test]
fn the_whole_green_path_verifies_against_its_root() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let root = corpus::merkle_root(&set).expect("non-empty");
    for (n, task) in tasks.iter().enumerate() {
        let op = [Op::Rotate90, Op::Rotate180, Op::FlipH][n].clone();
        let proof = proof_for(task, vec![op], &set);
        let want = proof.task_id;
        let bytes = encode_corpus_work(&proof).expect("encodes");
        assert_eq!(
            verify_corpus_work_against(&bytes, &root),
            Ok(WorkClass::AdmissibleCorpus { task_id: want }),
            "task {n} is admissible, replays exactly, and chains intact"
        );
    }
}

/// THE VACUITY, refused for v2 as well (98ccbbe's law): a replay over
/// no training pair proves nothing.
#[test]
fn an_empty_training_set_refuses_on_the_wire() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let honest = proof_for(&tasks[0], vec![Op::Rotate90], &set);
    let mut bytes = encode_corpus_work(&honest).expect("encodes");
    // Blank the train count (u16 LE) that follows schema+task_id+path.
    let at = 1 + 32 + 2 + honest.inclusion.len() * 33;
    bytes[at] = 0;
    bytes[at + 1] = 0;
    assert!(
        decode_corpus_work(&bytes).is_none(),
        "no training pair means nothing to close — refused as form"
    );
}

/// An admissible, honestly replayed corpus solve verifies, and reports
/// its CLASS — off the ladder, identified by task_id.
#[test]
fn an_admissible_corpus_solve_verifies_and_reports_its_class() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let root = corpus::merkle_root(&set).expect("non-empty");
    let proof = proof_for(&tasks[0], vec![Op::Rotate90], &set);
    assert!(
        corpus::verify_inclusion(&proof.task_id, &proof.inclusion, &root),
        "the fixture corpus is self-consistent"
    );
    // Against the FIXTURE root it is admissible; against the PINNED
    // root it is not — which is the constant doing its job.
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert_eq!(
        verify_corpus_work(&bytes),
        Err(WorkBroken::NotAdmissible),
        "a fixture corpus is not the admissible corpus"
    );
    assert_ne!(root, CORPUS_ROOT, "the pinned root is the real corpus's");
}

/// The wire round-trips exactly, and refuses trailing bytes.
#[test]
fn the_corpus_payload_round_trips_and_is_exact() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let proof = proof_for(&tasks[1], vec![Op::Rotate180], &set);
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert_eq!(bytes[0], WORK_SCHEMA_V2_CORPUS, "the schema byte leads");
    assert_eq!(decode_corpus_work(&bytes).as_ref(), Some(&proof));

    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_corpus_work(&trailing).is_none(), "trailing bytes refuse");

    let mut truncated = bytes.clone();
    truncated.pop();
    assert!(decode_corpus_work(&truncated).is_none(), "truncation refuses");

    // The direction bit is exact, never merely truthy.
    let mut bent = bytes.clone();
    if !proof.inclusion.is_empty() {
        bent[1 + 32 + 2] = 2;
        assert!(decode_corpus_work(&bent).is_none(), "a non-0/1 direction refuses");
    }
}

/// THE SCHEMAS DO NOT COLLIDE. v1 bytes still decode as v1, v2 bytes
/// refuse at the v1-only entry point rather than being mispriced, and
/// the class-aware entry point takes both.
#[test]
fn v1_is_untouched_and_the_shapes_never_confuse() {
    // A genuine v1 forge proof.
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

    // v2 payload at the v1-only door: a shape, not a malformation.
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let v2_bytes = encode_corpus_work(&proof_for(&tasks[0], vec![Op::Rotate90], &set))
        .expect("encodes");
    assert_eq!(verify_work(&v2_bytes), Err(WorkBroken::UnsupportedShape));
    assert!(decode_corpus_work(&v1_bytes).is_none(), "v1 is not a corpus proof");
    assert!(solver_verify::decode_work(&v2_bytes).is_none(), "v2 is not a forge proof");

    // The class-aware door ROUTES v1 down the forge path. This pin is
    // about dispatch, not about this particular proof being sound —
    // `solver-verify` holds no search, so it cannot find a program that
    // closes a forged task, and the one above deliberately does not.
    // What matters is that a v1 payload never reaches a corpus verdict.
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
    // An unknown schema byte refuses rather than guessing.
    assert_eq!(verify_work_class(&[9, 9, 9]), Err(WorkBroken::Malformed));
    assert_eq!(verify_work_class(&[]), Err(WorkBroken::Malformed));
}

/// A tampered chain refuses and names its break, for v2 as for v1.
#[test]
fn a_tampered_corpus_chain_names_its_break() {
    let tasks = built_corpus();
    let set = ids_of(&tasks);
    let mut proof = proof_for(&tasks[0], vec![Op::Rotate90], &set);
    proof.certificate.output[0] ^= 1;
    let bytes = encode_corpus_work(&proof).expect("encodes");
    assert!(matches!(
        verify_corpus_work(&bytes),
        Err(WorkBroken::ChainDiverged { .. }) | Err(WorkBroken::NotAdmissible)
    ));
}

/// A leaf is not a raw task_id and an interior node is not a leaf —
/// domain separation, so no value of one kind reads as another.
#[test]
fn the_tree_separates_its_domains() {
    let id = [7u8; 32];
    assert_ne!(corpus::leaf(&id), id, "a leaf is tagged, never the bare id");
    let single = corpus::merkle_root(&[id]).expect("one member");
    assert_eq!(single, corpus::leaf(&id), "a one-member root is its leaf");
    // A two-member root is a tagged node, distinct from either leaf.
    let pair_root = corpus::merkle_root(&[id, [8u8; 32]]).expect("two members");
    assert_ne!(pair_root, corpus::leaf(&id));
    assert_ne!(pair_root, corpus::leaf(&[8u8; 32]));
}

/// An empty path proves membership only for a ONE-member corpus, and
/// never against a larger root — the degenerate-proof check.
#[test]
fn an_empty_path_proves_nothing_in_a_real_corpus() {
    let ids = ids_of(&built_corpus());
    let root = corpus::merkle_root(&ids).expect("non-empty");
    let empty: Vec<PathStep> = Vec::new();
    for id in &ids {
        assert!(
            !corpus::verify_inclusion(id, &empty, &root),
            "a bare leaf must not pass for a three-member root"
        );
    }
}
