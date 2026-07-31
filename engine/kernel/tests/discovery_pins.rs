//! KERNEL invariant witnesses — I4 completes on the discovery mint
//! (U1 KERNEL-VERIFY, U_DIRECTIVES §2; red-first, the red recorded
//! against the unflipped coverage).
//!
//! The pins: the sealed-shut law (valid work + ANY positive amount
//! refuses with bound 0 carried); a tampered chain refuses naming its
//! exact link; an INTACT chain over a non-solution refuses (the
//! fabricated-render attack, dead — the replay-complete law); a wrong
//! forge digest never verifies; a malformed payload refuses as form;
//! the zero-amount mint with valid work admits; tag 86 round-trips
//! byte-exact and the work payload round-trips its own encoding.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::tlv;
use cfe::world::{Committed, World, SOVEREIGN};
use kernel::{lawful, Step, Unlawful};
use solver::{solve, SolveRead};
use solver_verify::{
    decode_work, encode_work, forge_task, run, Difficulty, Op, SolveCertificate, WorkBroken,
    WorkProof,
};

const HAND: u64 = 100;
const DIGEST: [u8; 32] = [7u8; 32];

/// A one-orb committed state holding `iron`, with the sovereign
/// seated and the acting hand contained under exactly Collect —
/// identity precedes the world (I2).
fn seated(iron: (i64, i64)) -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "assay".into(), ..Default::default() };
    orb.bank(Resource::Iron, ratio(iron.0, iron.1));
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

/// An HONEST proof: a real level-1 forge task, really solved, really
/// sealed. The prover (dev-dependency only) searches; the fixture
/// carries the result.
fn honest_proof() -> WorkProof {
    let difficulty = Difficulty::level(1);
    let task = forge_task(DIGEST, difficulty);
    let SolveRead::Solved { program, test_output, certificate, .. } =
        solve(&task, difficulty.word_len)
    else {
        panic!("a level-1 forge task solves at its own word length");
    };
    WorkProof { forge_digest: DIGEST, difficulty, program, output: test_output, certificate }
}

fn mint_step(amount: (i64, i64), work: Vec<u8>) -> Step {
    Step {
        event: ClosureEvent::DiscoveryMinted {
            orb_id: 1,
            resource: Resource::Iron,
            amount: ratio(amount.0, amount.1),
            work,
        },
        actor: HAND,
        stamp: 1,
    }
}

#[test]
fn the_sealed_shut_bound_refuses_every_positive_mint() {
    // Valid work, honest claim of 1 iron — and the bound is ZERO
    // until Ruling Box 1 opens the rate. The refusal carries both
    // sides whole: what the law allows, what the deed claimed.
    let work = encode_work(&honest_proof()).expect("the honest proof encodes");
    let prior = seated((6, 1));
    let next = seated((7, 1)); // the claimed mint banked — conservation holds
    assert_eq!(
        lawful(&prior, &next, &mint_step((1, 1), work)),
        Err(Unlawful::MintExceedsWork {
            bound: Box::new(ratio(0, 1)),
            claimed: Box::new(ratio(1, 1)),
        }),
        "the mechanism precedes the economics: sealed shut, by law"
    );
}

#[test]
fn a_tampered_chain_refuses_at_its_exact_link() {
    let mut proof = honest_proof();
    proof.certificate.steps[0][0] ^= 0xff;
    let work = encode_work(&proof).expect("encodes");
    let verdict = lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), work));
    assert_eq!(
        verdict,
        Err(Unlawful::WorkRefused {
            broken: WorkBroken::ChainDiverged { at_step: 1 }
        }),
        "tampering with link 1 is named as link 1 — the break whole"
    );
}

#[test]
fn an_intact_chain_over_a_non_solution_refuses() {
    // THE FABRICATED-RENDER ATTACK: seal a perfectly valid hash chain
    // over a program that does NOT solve the task. The chain
    // re-derives Intact — and the replay-complete law still refuses,
    // because verification re-APPLIES the program, not just re-hashes.
    let difficulty = Difficulty::level(1);
    let task = forge_task(DIGEST, difficulty);
    let sym = [Op::Rotate90, Op::Rotate180, Op::Rotate270, Op::FlipH, Op::FlipV, Op::Transpose];
    let wrong_op = sym
        .iter()
        .find(|op| run(&vec![(*op).clone()], &task.train[0].input) != task.train[0].output)
        .expect("some symmetry misses the first pair")
        .clone();
    let wrong_program = vec![wrong_op];
    let wrong_output = run(&wrong_program, &task.test_in);
    let certificate = SolveCertificate::seal(&task, &wrong_program, &wrong_output);
    let proof = WorkProof {
        forge_digest: DIGEST,
        difficulty,
        program: wrong_program,
        output: wrong_output,
        certificate,
    };
    let work = encode_work(&proof).expect("encodes");
    assert_eq!(
        lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), work)),
        Err(Unlawful::WorkRefused {
            broken: WorkBroken::TrainingMiss { pair: 0 }
        }),
        "an intact chain proves nothing without the replay"
    );
}

#[test]
fn a_wrong_forge_digest_never_verifies() {
    let mut proof = honest_proof();
    proof.forge_digest = [8u8; 32]; // a different task entirely
    let work = encode_work(&proof).expect("encodes");
    let verdict = lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), work));
    assert!(
        matches!(verdict, Err(Unlawful::WorkRefused { .. })),
        "the task is not the one claimed: {verdict:?}"
    );
}

#[test]
fn a_malformed_payload_refuses_as_form() {
    // Trailing bytes refuse — the payload is exact, never a guess.
    let mut work = encode_work(&honest_proof()).expect("encodes");
    work.push(0);
    assert_eq!(
        lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), work)),
        Err(Unlawful::WorkRefused { broken: WorkBroken::Malformed }),
        "truncation, trailing bytes, unknown codes — all form refusals"
    );
    // And an empty payload is no proof at all.
    assert_eq!(
        lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), Vec::new())),
        Err(Unlawful::WorkRefused { broken: WorkBroken::Malformed }),
        "no certificate → no discovery mint"
    );
}

#[test]
fn the_zero_mint_with_valid_work_admits() {
    // The degenerate mint is lawful: 0 ≤ bound 0; conservation holds
    // (nothing banked); the deed and its proof enter the record.
    let work = encode_work(&honest_proof()).expect("encodes");
    let prior = seated((6, 1));
    let next = seated((6, 1));
    assert_eq!(lawful(&prior, &next, &mint_step((0, 1), work)), Ok(()));
}

#[test]
fn degenerate_difficulty_refuses_as_form_never_mints() {
    // HOLE A (audit 2026-07-31 §4): a work payload carries its OWN
    // difficulty, and an empty training set makes "maps EVERY training
    // pair" vacuously true — an empty program over train_pairs: 0
    // would pass the replay and mint for free once the rate opens.
    // The fix: degenerate difficulty is a FORM refusal at decode (the
    // opaque-payload law's refuse-never-guess, extended).
    let mut proof = honest_proof();
    proof.difficulty.train_pairs = 0;
    let work = encode_work(&proof).expect("encodes");
    assert_eq!(
        lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), work)),
        Err(Unlawful::WorkRefused { broken: WorkBroken::Malformed }),
        "no training pairs, no verifiable work — refused as form"
    );
}

#[test]
fn zero_dimension_difficulty_refuses_never_panics() {
    // HOLE B (audit 2026-07-31 §4): rows/cols: 0 drove the forge's
    // Grid::new(...).expect() to PANIC inside the kernel's call path —
    // a crafted payload crashing the node (DoS), violating the
    // kernel's total-never-panics law. The guard refuses it as form,
    // and the judge stays total.
    for degenerate in [(0usize, 3usize), (3, 0), (0, 0)] {
        let mut proof = honest_proof();
        proof.difficulty.rows = degenerate.0;
        proof.difficulty.cols = degenerate.1;
        let work = encode_work(&proof).expect("encodes");
        assert_eq!(
            lawful(&seated((6, 1)), &seated((7, 1)), &mint_step((1, 1), work)),
            Err(Unlawful::WorkRefused { broken: WorkBroken::Malformed }),
            "a zero dimension refuses as form — the judge never panics"
        );
    }
}

#[test]
fn the_deed_and_the_payload_round_trip_byte_exact() {
    // The work payload: solver-verify's own encoding monopoly.
    let proof = honest_proof();
    let work = encode_work(&proof).expect("encodes");
    assert_eq!(decode_work(&work).as_ref(), Some(&proof), "value round trip");
    // Tag 86: the deed carries its proof whole through the wire.
    let event = ClosureEvent::DiscoveryMinted {
        orb_id: 1,
        resource: Resource::Iron,
        amount: ratio(3, 2),
        work,
    };
    let bytes = event.wire_bytes();
    let (tag, value) = tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 86, "the claimed tag, forever");
    let decoded = ClosureEvent::from_wire(tag, value).expect("tag 86 decodes");
    assert_eq!(decoded, event, "value round trip");
    assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");
}
