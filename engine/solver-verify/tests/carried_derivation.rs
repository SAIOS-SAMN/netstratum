//! THE CARRIED DERIVATION (Concordance 4.4 annex): a schema-v4 forge proof is
//! checked step by step — state 0 against its beacon samples, the forge's
//! word and the solve's word each walked over it — and every break is named
//! where it happens.

use solver_verify::{
    decode_carried_work, derive, encode_carried_work, forge_task, forge_word, verify_carried_work,
    verify_work, verify_work_class, walk, CarriedProof, Derivation, Difficulty, Op, WorkBroken,
    WorkClass,
};

fn digest(n: u8) -> [u8; 32] {
    let mut d = [0u8; 32];
    d[0] = n;
    d[31] = 7;
    d
}

/// A solve of the forged task: the forge's own word closes it.
fn sealed(level: u32) -> CarriedProof {
    let difficulty = Difficulty::level(level);
    let word = forge_word(digest(level as u8), difficulty);
    CarriedProof::seal(digest(level as u8), difficulty, &word).expect("the forge word closes its task")
}

#[test]
fn a_carried_proof_verifies_without_the_forge_and_reads_back_whole() {
    for level in 1..=6 {
        let proof = sealed(level);
        let bytes = encode_carried_work(&proof).expect("encodes");
        assert_eq!(decode_carried_work(&bytes), Some(proof.clone()));
        assert_eq!(verify_carried_work(&bytes), Ok(proof.difficulty));
        assert_eq!(verify_work(&bytes), Ok(proof.difficulty), "forge work by every door");
        assert_eq!(verify_work_class(&bytes), Ok(WorkClass::Forge(proof.difficulty)));
        // the carried task is the forged task
        let task = forge_task(proof.forge_digest, proof.difficulty);
        assert_eq!(proof.output(), task.test_expected.as_ref());
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert_eq!(verify_carried_work(&trailing), Err(WorkBroken::Malformed));
    }
}

#[test]
fn a_wrong_solve_does_not_seal() {
    let difficulty = Difficulty::level(5);
    let mut wrong = forge_word(digest(5), difficulty);
    wrong.push(Op::Rotate90);
    assert!(CarriedProof::seal(digest(5), difficulty, &wrong).is_none());
}

#[test]
fn every_break_is_named_where_it_happens() {
    let honest = sealed(5);
    let verdict = |p: &CarriedProof| verify_carried_work(&encode_carried_work(p).expect("encodes"));

    // State 0: a cell that is not its beacon sample.
    let mut p = honest.clone();
    p.inputs[1].cells[0] = (p.inputs[1].cells[0] + 1) % 10;
    assert_eq!(verdict(&p), Err(WorkBroken::SampleDiverged { pair: Some(1) }));

    // The forge word: an op that is not its sample.
    let mut p = honest.clone();
    p.forge_word[0] = if p.forge_word[0] == Op::FlipV { Op::FlipH } else { Op::FlipV };
    assert_eq!(verdict(&p), Err(WorkBroken::SampleDiverged { pair: None }));

    // The forge derivation: a carried digest that is not the step's state.
    let mut p = honest.clone();
    p.forge_derivation.states[1][0] ^= 1;
    assert_eq!(verdict(&p), Err(WorkBroken::ForgeDiverged { at_step: 1 }));

    // The outputs: the forge's walk does not end at them.
    let mut p = honest.clone();
    let last = p.outputs.len() - 1;
    p.outputs[last].cells[0] = (p.outputs[last].cells[0] + 1) % 10;
    assert_eq!(verdict(&p), Err(WorkBroken::ForgeDiverged { at_step: p.forge_word.len() + 1 }));

    // The solve derivation: step 2's carried digest is broken.
    let mut p = honest.clone();
    p.derivation.states[2][5] ^= 1;
    assert_eq!(verdict(&p), Err(WorkBroken::ChainDiverged { at_step: 2 }));

    // A solve that walks honestly but does not close the task.
    let mut p = honest.clone();
    p.program = vec![Op::Rotate90];
    p.derivation = derive(&p.program, &p.inputs).expect("derives").0;
    assert!(matches!(verdict(&p), Err(WorkBroken::TrainingMiss { .. }) | Err(WorkBroken::OutputMismatch)));

    // No path: no claim.
    let mut p = honest.clone();
    p.program.clear();
    p.derivation = derive(&p.program, &p.inputs).expect("derives").0;
    assert_eq!(verdict(&p), Err(WorkBroken::PathAbsent));
}

#[test]
fn a_walk_names_a_short_or_long_derivation() {
    let p = sealed(5);
    let mut short = p.derivation.clone();
    short.states.pop();
    assert_eq!(walk(&short, &p.program, &p.inputs), Err(p.program.len()));
    let mut long = p.derivation.clone();
    long.states.push([0; 32]);
    assert_eq!(walk(&long, &p.program, &p.inputs), Err(p.program.len() + 1));
    let empty = Derivation { states: Vec::new() };
    assert_eq!(walk(&empty, &p.program, &p.inputs), Err(0));
}
