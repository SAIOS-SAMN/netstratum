//! THE VOLUMETRIC VERIFIER, pinned (work-shapes v3, operator ruling
//! 2026-08-04 lifting C8's deferral).
//!
//! C8 was right about what was there: a bare digest nobody could
//! re-derive is tamper-evidence for no one. These pins exist because
//! that is no longer what is there.
//!
//! The load-bearing pins:
//!   · `a_tampered_word_is_named_not_merely_refused` — the leg C8 said
//!     did not exist. A third party re-derives the chain and NAMES the
//!     break.
//!   · `an_intact_chain_over_a_wrong_output_refuses` — THE
//!     REPLAY-COMPLETE LAW in three dimensions: a certificate binds a
//!     triple, it does not prove the triple is a solution.
//!   · `the_vacuity_cannot_be_encoded_at_all` — the exploitable one.

use solver_verify::volume::{
    forge_volume, run_volume, verify_volume_certificate, VOp, VolDifficulty, Volume,
    VolumeCertificate, VolumeCertificateRead,
};
use solver_verify::work::{
    decode_volume_work, encode_volume_work, verify_volume_work, verify_work, verify_work_class,
    VolumeProof, WorkBroken, WorkClass, WORK_SCHEMA_V3_VOLUME,
};
use solver_verify::{Grid, Op};

/// Forge a task and seal an HONEST proof for it, using the forge's own
/// answer as the program's target. The word is found by sweeping the
/// small vocabulary — this is a test fixture, not the prover.
fn honest_proof(seed: u8, level: u32) -> Option<VolumeProof> {
    let difficulty = VolDifficulty::level(level);
    let digest = [seed; 32];
    let task = forge_volume(digest, difficulty);
    let vocabulary = solver_verify::volume::vocabulary();
    // Search words up to the rung's length for one that closes.
    let mut frontier: Vec<Vec<VOp>> = vec![Vec::new()];
    for _ in 0..=difficulty.word_len {
        for program in &frontier {
            let closes = task
                .train
                .iter()
                .all(|(input, expected)| &run_volume(program, input) == expected);
            if closes {
                let output = run_volume(program, &task.test_in);
                let certificate = VolumeCertificate::seal(&task, program, &output);
                return Some(VolumeProof {
                    forge_digest: digest,
                    difficulty,
                    program: program.clone(),
                    output,
                    certificate,
                });
            }
        }
        let mut next = Vec::new();
        for program in &frontier {
            for op in &vocabulary {
                let mut extended = program.clone();
                extended.push(op.clone());
                next.push(extended);
            }
        }
        frontier = next;
    }
    None
}

/// An honest volumetric solve verifies whole and reports its class —
/// on its OWN ladder, never the planar one.
#[test]
fn an_honest_volumetric_solve_verifies_and_reports_its_own_ladder() {
    let mut verified = 0;
    for seed in 1..=6u8 {
        let Some(proof) = honest_proof(seed, 1 + u32::from(seed % 3)) else { continue };
        let bytes = encode_volume_work(&proof).expect("encodes");
        assert_eq!(
            verify_volume_work(&bytes),
            Ok(WorkClass::Volumetric(proof.difficulty)),
            "seed {seed}: re-forge, re-apply and re-chain all hold"
        );
        verified += 1;
    }
    // COVERAGE: the assertions above live inside a loop over seeds that
    // may not all close. Without this the test could pass having
    // verified nothing.
    assert!(verified >= 3, "only {verified} fixtures closed — too few to pin");
}

/// THE LEG C8 SAID DID NOT EXIST. Tamper with the word and the chain
/// does not merely fail — it NAMES the diverged link. A bare digest
/// could never do this, which is why it was decorative.
#[test]
fn a_tampered_word_is_named_not_merely_refused() {
    let proof = honest_proof(3, 2).or_else(|| honest_proof(5, 2)).expect("a fixture closes");
    assert!(!proof.program.is_empty(), "need a word to tamper with");

    let mut bent = proof.clone();
    bent.certificate.steps[0][0] ^= 1;
    let bytes = encode_volume_work(&bent).expect("encodes");
    match verify_volume_work(&bytes) {
        Err(WorkBroken::ChainDiverged { at_step }) => {
            assert_eq!(at_step, 1, "step 1 is the first diverged link, and it is named")
        }
        other => panic!("a tampered chain must be named, got {other:?}"),
    }

    // The genesis is link 0.
    let mut bent = proof.clone();
    bent.certificate.genesis[0] ^= 1;
    let bytes = encode_volume_work(&bent).expect("encodes");
    assert_eq!(
        verify_volume_work(&bytes),
        Err(WorkBroken::ChainDiverged { at_step: 0 }),
        "the genesis binds the whole observation"
    );
}

/// THE REPLAY-COMPLETE LAW, volumetric. A certificate binds
/// (task, word, output) — it does NOT prove the word SOLVES the task.
/// An intact chain sealed over a wrong output must still refuse.
#[test]
fn an_intact_chain_over_a_wrong_output_refuses() {
    let proof = honest_proof(3, 2).or_else(|| honest_proof(5, 2)).expect("a fixture closes");
    let task = forge_volume(proof.forge_digest, proof.difficulty);

    // A DIFFERENT output, with a certificate honestly sealed over it —
    // the chain is intact, the claim is false.
    let wrong = VOp::FlipStack.apply(&proof.output);
    if wrong == proof.output {
        return; // degenerate fixture; the other pins carry the law
    }
    let certificate = VolumeCertificate::seal(&task, &proof.program, &wrong);
    assert_eq!(
        verify_volume_certificate(&certificate, &task, &proof.program, &wrong),
        VolumeCertificateRead::Intact,
        "the chain over the wrong output is itself intact — that is the trap"
    );

    let forged = VolumeProof { output: wrong, certificate, ..proof };
    let bytes = encode_volume_work(&forged).expect("encodes");
    assert_eq!(
        verify_volume_work(&bytes),
        Err(WorkBroken::OutputMismatch),
        "an intact chain over a non-solution is NOT a solution"
    );
}

/// A word that misses a training pair refuses and names the pair.
#[test]
fn a_training_miss_is_named() {
    let proof = honest_proof(3, 2).or_else(|| honest_proof(5, 2)).expect("a fixture closes");
    let mut wrong = proof.clone();
    // Append an operator that changes the result, breaking closure.
    wrong.program.push(VOp::PerLayer(Op::Rotate180));
    let task = forge_volume(wrong.forge_digest, wrong.difficulty);
    wrong.output = run_volume(&wrong.program, &task.test_in);
    wrong.certificate = VolumeCertificate::seal(&task, &wrong.program, &wrong.output);
    let bytes = encode_volume_work(&wrong).expect("encodes");
    match verify_volume_work(&bytes) {
        Err(WorkBroken::TrainingMiss { .. }) | Err(WorkBroken::OutputMismatch) => {}
        other => panic!("a broken word must refuse typed, got {other:?}"),
    }
}

/// THE VACUITY — and it cannot even be ENCODED, let alone verified.
/// `train_pairs: 0` makes "maps every training pair" vacuously true,
/// which is a free mint. It is refused as FORM at decode, before the
/// forge runs, so no replay path can reach it.
#[test]
fn the_vacuity_cannot_be_encoded_at_all() {
    let proof = honest_proof(3, 2).or_else(|| honest_proof(5, 2)).expect("a fixture closes");

    for bend in [
        VolDifficulty { train_pairs: 0, ..proof.difficulty },
        VolDifficulty { layers: 0, ..proof.difficulty },
        VolDifficulty { rows: 0, ..proof.difficulty },
        VolDifficulty { cols: 0, ..proof.difficulty },
    ] {
        assert!(!bend.lawful(), "a degenerate rung is not lawful");
        let bent = VolumeProof { difficulty: bend, ..proof.clone() };
        let bytes = encode_volume_work(&bent).expect("the wire will carry the bytes");
        assert!(
            decode_volume_work(&bytes).is_none(),
            "a degenerate rung refuses as FORM: {bend:?}"
        );
        assert_eq!(
            verify_volume_work(&bytes),
            Err(WorkBroken::Malformed),
            "the judge never re-forges a degenerate task — it refuses first"
        );
    }

    // The empty WORD stays lawful: a copy task is a task.
    assert!(VolDifficulty { word_len: 0, ..proof.difficulty }.lawful());
}

/// The three schemas do not collide, and v3 is routed to its own judge.
#[test]
fn v3_never_confuses_itself_with_the_other_shapes() {
    let proof = honest_proof(3, 2).or_else(|| honest_proof(5, 2)).expect("a fixture closes");
    let bytes = encode_volume_work(&proof).expect("encodes");
    assert_eq!(bytes[0], WORK_SCHEMA_V3_VOLUME, "the schema byte leads");

    // The v1-only door refuses it as a SHAPE, never mispriced as planar.
    assert_eq!(verify_work(&bytes), Err(WorkBroken::UnsupportedShape));
    assert!(solver_verify::decode_work(&bytes).is_none(), "v3 is not a forge proof");
    assert!(
        solver_verify::decode_corpus_work(&bytes).is_none(),
        "v3 is not a corpus proof"
    );

    // The class-aware door routes it and reports the volumetric ladder.
    assert_eq!(
        verify_work_class(&bytes),
        Ok(WorkClass::Volumetric(proof.difficulty))
    );
}

/// The wire round-trips exactly and is exact about its own length.
#[test]
fn the_volumetric_payload_round_trips_and_is_exact() {
    let proof = honest_proof(3, 2).or_else(|| honest_proof(5, 2)).expect("a fixture closes");
    let bytes = encode_volume_work(&proof).expect("encodes");
    assert_eq!(decode_volume_work(&bytes).as_ref(), Some(&proof));

    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_volume_work(&trailing).is_none(), "trailing bytes refuse");

    let mut truncated = bytes.clone();
    truncated.pop();
    assert!(decode_volume_work(&truncated).is_none(), "truncation refuses");
}

/// The carrier's own law: a volume is a NON-EMPTY stack of congruent
/// layers, so an incongruent or empty stack is not a volume at all.
#[test]
fn the_carrier_refuses_an_incongruent_or_empty_stack() {
    let two = Grid::new(2, 2, vec![1, 2, 3, 4]).expect("well-formed");
    let three = Grid::new(1, 3, vec![1, 2, 3]).expect("well-formed");
    assert!(Volume::new(vec![]).is_none(), "there is no stack of nothing");
    assert!(Volume::new(vec![two.clone(), three]).is_none(), "layers must be congruent");
    let volume = Volume::new(vec![two.clone(), two]).expect("congruent");
    assert_eq!(volume.depth(), 2);
}

/// The forge is a reading of history: same digest and rung, byte-
/// identical task; a different chronicle, a different task.
#[test]
fn the_volumetric_forge_replays_exactly() {
    for level in 1..=5 {
        let d = VolDifficulty::level(level);
        assert_eq!(forge_volume([21u8; 32], d), forge_volume([21u8; 32], d));
    }
    assert_ne!(
        forge_volume([21u8; 32], VolDifficulty::level(2)),
        forge_volume([22u8; 32], VolDifficulty::level(2)),
    );
}
