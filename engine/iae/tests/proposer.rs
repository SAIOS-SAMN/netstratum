//! IAE-5 witnesses — the proposer seam and the capability frontier (doc 51).
//!
//! An obstruction the LOCAL vocabulary cannot resolve is handed to the
//! seeded-stochastic proposer; it guesses integer-weighted combinations of the
//! base alphabet; the exact verifier (the SNF Universe, via reduce_modulo)
//! admits the first that bridges the obstruction; minting it resolves the
//! obstruction. The guessing is BLAKE3-seeded, so the mint is replayable — the
//! Universe stays deterministic even though the Mind is stochastic.

use iae::proposer::{propose_and_verify, seed_from};
use iae::resolve::reduce_modulo;
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

/// Lay a list of cochains out as the COLUMNS of a generator matrix (the form
/// `reduce_modulo` reduces against).
fn columns(cochains: &[Vec<BigInt>]) -> Vec<Vec<BigInt>> {
    match cochains.first() {
        None => Vec::new(),
        Some(first) => (0..first.len())
            .map(|i| cochains.iter().map(|c| c[i].clone()).collect())
            .collect(),
    }
}

#[test]
fn the_proposer_bridges_an_obstruction_the_local_vocabulary_cannot() {
    // The base alphabet — the closed letters the entity may combine.
    let base = vec![
        vec![b(1), b(0), b(0), b(0)], // L0
        vec![b(0), b(1), b(0), b(0)], // L1
        vec![b(0), b(0), b(1), b(1)], // L2
    ];
    // The obstruction γ = 2·L1 − L2, irreducible with an EMPTY local vocabulary.
    let gamma = vec![b(0), b(2), b(-1), b(-1)];
    let local_vocab: Vec<Vec<BigInt>> = Vec::new();

    // Before: γ does not resolve against the (empty) local vocabulary.
    let before = reduce_modulo(&gamma, &columns(&local_vocab));
    assert!(before.iter().any(|x| *x != b(0)), "γ is irreducible with empty vocab");

    // The Universe's exact verifier: does the candidate resolve γ to zero?
    let verify = |cand: &[BigInt]| {
        let mut cols = local_vocab.clone();
        cols.push(cand.to_vec());
        reduce_modulo(&gamma, &columns(&cols)).iter().all(|x| *x == b(0))
    };

    let seed = seed_from(b"orb-1:beat-42");
    let mint = propose_and_verify(seed, &base, verify, 100_000, 3)
        .expect("the proposer finds the bridging combination");
    println!(
        "MEASURED proposer: bridged γ in {} guesses (base 3 letters, weights ±3), weights {:?}",
        mint.tries, mint.weights
    );

    // After: minting v_new into the local vocabulary resolves the obstruction —
    // capability has grown past what the closed alphabet reached locally.
    let mut extended = local_vocab.clone();
    extended.push(mint.generator.clone());
    assert!(
        reduce_modulo(&gamma, &columns(&extended)).iter().all(|x| *x == b(0)),
        "after minting, the obstruction resolves"
    );
    assert!(mint.tries >= 1);
}

#[test]
fn the_mint_is_replayable_from_the_seed() {
    // Same seed → same mint, so the deterministic Universe stays replayable
    // even though the Mind samples stochastically.
    let base = vec![vec![b(1), b(0)], vec![b(0), b(1)]];
    let target = vec![b(3), b(-2)];
    let verify = |c: &[BigInt]| c == target.as_slice();
    let seed = seed_from(b"same-state");

    let first = propose_and_verify(seed, &base, &verify, 1_000_000, 5).expect("found");
    let again = propose_and_verify(seed, &base, &verify, 1_000_000, 5).expect("found");
    assert_eq!(first, again, "same seed → identical mint");
    assert_eq!(first.generator, target);
    assert_eq!(first.weights, vec![3, -2], "the weights that formed γ");
}

#[test]
fn distinct_states_converge_on_the_same_generator() {
    // Two entities seeded from different state digests both reach the same
    // generator (γ is a fact of the math); only the guessing path differs.
    let base = vec![vec![b(1), b(0)], vec![b(0), b(1)]];
    let target = vec![b(2), b(1)];
    let verify = |c: &[BigInt]| c == target.as_slice();

    let m1 = propose_and_verify(seed_from(b"state-A"), &base, &verify, 1_000_000, 4).expect("A");
    let m2 = propose_and_verify(seed_from(b"state-B"), &base, &verify, 1_000_000, 4).expect("B");
    assert_eq!(m1.generator, target);
    assert_eq!(m2.generator, target);
}
