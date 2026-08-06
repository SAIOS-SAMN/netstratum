//! **THE CARRIER LAW, PINNED** (P4, operator ruling 2026-08-06).
//!
//! One pricing function serves the gate arm and the sealed warrant
//! (call-not-copy), and it rules the three carriers three ways:
//! forge priced by the ladder (unchanged until the corpus rates are
//! measured), the volumetric carrier RETIRED by name, and a verified
//! corpus proof refused UNPRICED — never broken — until the blind
//! benchmark fills the rate table. This landing moves no verdict:
//! volume and corpus refused before and refuse still; only the
//! reasons stopped being accidents.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::world::{Committed, World, SOVEREIGN};
use kernel::{admits, Refused, Step};
use solver_verify::volume::{
    forge_volume, run_volume, VOp, VolDifficulty, Volume, VolumeCertificate,
};
use solver_verify::{
    corpus, encode_corpus_work, encode_volume_work, CorpusProof, Grid, Op, Program,
    SolveCertificate, Task, TaskPair, VolumeProof, WorkBroken,
};

const HAND: u64 = 100;

fn seated() -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "carrier".into(), ..Default::default() };
    orb.bank(Resource::Iron, ratio(4, 1));
    committed.orbs = vec![orb];
    committed.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "sovereign".into(), principal: None, grants: vec![] },
        Entity {
            entity_id: HAND,
            handle: "hand".into(),
            principal: Some(SOVEREIGN),
            grants: vec![Scope::Collect],
        },
    ];
    committed
}

fn mint_step(work: Vec<u8>) -> Step {
    Step {
        event: ClosureEvent::DiscoveryMinted {
            orb_id: 1,
            resource: Resource::Iron,
            amount: ratio(0, 1),
            work,
        },
        actor: HAND,
        stamp: 1,
    }
}

/// An honest volumetric proof, found by sweeping the small vocabulary
/// (a fixture, not a prover — cribbed from `volume_verify_pins`).
fn volume_proof() -> Option<VolumeProof> {
    for seed in 1..=6u8 {
        let difficulty = VolDifficulty::level(1 + u32::from(seed % 3));
        let digest = [seed; 32];
        let task = forge_volume(digest, difficulty);
        let vocabulary = solver_verify::volume::vocabulary();
        let mut frontier: Vec<Vec<VOp>> = vec![Vec::new()];
        for _ in 0..=difficulty.word_len {
            for program in &frontier {
                let closes = task
                    .train
                    .iter()
                    .all(|(input, expected): &(Volume, Volume)| {
                        &run_volume(program, input) == expected
                    });
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
    }
    None
}

/// **THE VOLUMETRIC CARRIER IS RETIRED BY NAME.** An honest, verifying
/// volume proof reaches the mint and refuses `CarrierRetired` — the
/// same outcome the old accident produced, now the ruling itself.
#[test]
fn a_volumetric_mint_refuses_as_retired() {
    let proof = volume_proof().expect("the small vocabulary closes a rung");
    let bytes = encode_volume_work(&proof).expect("encodes");
    let prior = seated();
    assert!(matches!(
        admits(&prior, &prior.clone(), &mint_step(bytes)),
        Err(Refused::WorkRefused { broken: WorkBroken::CarrierRetired })
    ));
}

/// **A VERIFIED CORPUS PROOF REFUSES UNPRICED, NEVER BROKEN.** The
/// rate table fills from the blind benchmark; until then the mint
/// names exactly what is missing.
#[test]
fn a_corpus_mint_refuses_as_unpriced() {
    let grid = |cells: &[u8]| Grid::new(2, 2, cells.to_vec()).expect("a small grid");
    let task = Task {
        name: "carrier pin task".into(),
        train: vec![TaskPair {
            input: grid(&[1, 2, 3, 4]),
            output: Op::Rotate90.apply(&grid(&[1, 2, 3, 4])),
        }],
        test_in: grid(&[5, 6, 7, 8]),
        test_expected: None,
    };
    // The certificate seals over the CANONICAL name — identity is the
    // content digest, and the chain re-derives from it (cribbed from
    // corpus_pins::proof_for).
    let program: Program = vec![Op::Rotate90];
    let task_id = corpus::task_digest(&task.train, &task.test_in);
    let output = Op::Rotate90.apply(&task.test_in);
    let sealed = Task {
        name: solver_verify::canonical_name(&task_id),
        train: task.train.clone(),
        test_in: task.test_in.clone(),
        test_expected: None,
    };
    let certificate = SolveCertificate::seal(&sealed, &program, &output);
    let proof = CorpusProof {
        task_id,
        train: task.train.clone(),
        test_in: task.test_in.clone(),
        program,
        output,
        certificate,
    };
    let bytes = encode_corpus_work(&proof).expect("encodes");
    let prior = seated();
    match admits(&prior, &prior.clone(), &mint_step(bytes)) {
        Err(Refused::WorkRefused { broken: WorkBroken::CarrierUnpriced }) => {}
        other => panic!("expected CarrierUnpriced, got {other:?}"),
    }
}

/// **THE MEASURED TABLE PRICES EXACTLY** — 34 confirmed rows, one
/// blind; W = scarcity × VisitedNodes₃ / word_len with the ratified
/// scarcities (1243/34 standard, 283/1 blind); an unmeasured task is
/// unpriced, never guessed.
#[test]
fn the_measured_table_prices_exactly() {
    use kernel::carrier_table::MEASURED;
    assert_eq!(MEASURED.len(), 34, "the benchmark confirmed 34");
    assert_eq!(
        MEASURED.iter().filter(|(_, _, _, blind)| *blind).count(),
        1,
        "exactly one blind-spot solve"
    );
    let (id, visited, word_len, blind) = &MEASURED[0];
    assert!(!blind);
    assert_eq!(
        kernel::corpus_weight(id),
        Some(ratio(1243, 34) * ratio(*visited as i64, i64::from(*word_len)))
    );
    let (id, visited, word_len, _) =
        MEASURED.iter().find(|(_, _, _, blind)| *blind).expect("the blind row");
    assert_eq!(
        kernel::corpus_weight(id),
        Some(ratio(283, 1) * ratio(*visited as i64, i64::from(*word_len)))
    );
    assert_eq!(kernel::corpus_weight(&[0u8; 32]), None, "unmeasured is unpriced");
}

/// **THE SPECTRUM IS A READING** — derived whole from opaque payloads
/// through the sealed verifier: a priced forge mint lands on its rung
/// and in the mass; garbage is broken, never guessed.
#[test]
fn the_spectrum_reads_the_work_whole() {
    use solver::{solve, SolveRead};
    use solver_verify::{encode_work, forge_task, Difficulty, WorkProof};
    let difficulty = Difficulty::level(1);
    let digest = [3u8; 32];
    let task = forge_task(digest, difficulty);
    let SolveRead::Solved { program, test_output, certificate, .. } =
        solve(&task, difficulty.word_len)
    else {
        panic!("a level-1 forge task solves");
    };
    let bytes = encode_work(&WorkProof {
        forge_digest: digest,
        difficulty,
        program,
        output: test_output,
        certificate,
    })
    .expect("encodes");
    let read = kernel::spectrum_of([bytes.as_slice(), b"garbage".as_slice()].into_iter());
    // The rung is the LADDER's own reading (saturated low tiers
    // coincide structurally — rung() finds the first match), so the
    // pin asserts against the derived position, never an assumed one.
    let rung = difficulty.rung().expect("a laddered shape") as usize;
    assert_eq!(read.forge_rungs[rung], 1, "read: {read:?}");
    assert_eq!(read.broken, 1);
    assert_eq!(read.weighted, kernel::mint_bound(Some(rung as u32)));
}
