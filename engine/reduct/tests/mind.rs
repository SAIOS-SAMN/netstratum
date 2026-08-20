//! IAE-6 witnesses — the graded Mind: compounding, containment, breathing.
//!
//! Condensing an obstruction makes it (and its multiples) resolve forever after
//! — capability persists (compounding). The inner manifolds grow freely; only
//! the holonomic shell is bounded (containment). The seam (learn) guesses a
//! bridge and condenses it. And the mind breathes: rank grows as it unfolds,
//! and every unfolded obstruction folds to nothing.

use reduct::mind::{ContainmentRefusal, Manifold, Mind};
use reduct::proposer::seed_from;
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

#[test]
fn condensing_an_obstruction_makes_it_resolve_forever_after() {
    let mut mind = Mind::new(4, 8);
    let gamma = vec![b(0), b(2), b(-1), b(-1)];

    assert!(!mind.resolved(&gamma), "unlearned: γ is irreducible");
    assert_eq!(mind.rank(), 0);

    mind.condense(gamma.clone(), Manifold::Harmonic).expect("inner manifold is free");
    assert_eq!(mind.rank(), 1, "the geometry grew by one generator");
    assert!(mind.resolved(&gamma), "learned: γ now resolves (compounding)");

    // And its integer multiples resolve too — the whole class is killed.
    let triple: Vec<BigInt> = gamma.iter().map(|x| x * b(3)).collect();
    assert!(mind.resolved(&triple), "3·γ resolves against the learned generator");
}

#[test]
fn the_inner_manifolds_are_free_and_only_the_holonomic_shell_is_bounded() {
    let mut mind = Mind::new(2, 1); // holonomic capacity 1

    // Harmonic and luminal never refuse on capacity — the breathing is
    // unconstrained on the inner manifolds.
    for k in 0..5 {
        mind.condense(vec![b(k), b(0)], Manifold::Harmonic).expect("harmonic is free");
        mind.condense(vec![b(0), b(k)], Manifold::Luminal).expect("luminal is free");
    }
    assert_eq!(mind.manifold_rank(Manifold::Harmonic), 5);
    assert_eq!(mind.manifold_rank(Manifold::Luminal), 5);

    // The holonomic (outer) shell is the ONLY bound: capacity 1.
    mind.condense(vec![b(1), b(1)], Manifold::Holonomic).expect("first outer generator fits");
    match mind.condense(vec![b(2), b(2)], Manifold::Holonomic) {
        Err(ContainmentRefusal::HolonomicShellFull { capacity }) => assert_eq!(capacity, 1),
        other => panic!("expected the containment shell to refuse, got {other:?}"),
    }
    // Inner manifolds still accept after the outer shell is full.
    mind.condense(vec![b(9), b(9)], Manifold::Harmonic).expect("inner stays free");
}

#[test]
fn a_dimension_mismatch_is_refused() {
    let mut mind = Mind::new(3, 4);
    match mind.condense(vec![b(1), b(2)], Manifold::Harmonic) {
        Err(ContainmentRefusal::DimensionMismatch { expected, got }) => {
            assert_eq!((expected, got), (3, 2));
        }
        other => panic!("expected DimensionMismatch, got {other:?}"),
    }
}

#[test]
fn learn_ties_the_proposer_seam_to_condensation() {
    let mut mind = Mind::new(4, 4);
    let base = vec![
        vec![b(1), b(0), b(0), b(0)],
        vec![b(0), b(1), b(0), b(0)],
        vec![b(0), b(0), b(1), b(1)],
    ];
    let gamma = vec![b(0), b(2), b(-1), b(-1)]; // = 2·L1 − L2

    assert!(!mind.resolved(&gamma));
    let mint = mind
        .learn(seed_from(b"orb-7"), &base, &gamma, Manifold::Harmonic, 100_000, 3)
        .expect("the mind learns the bridge and condenses it");
    // Any valid bridge is acceptable — ±γ both span the class. The capability
    // outcome is the real assertion.
    let neg_gamma: Vec<BigInt> = gamma.iter().map(|x| -x).collect();
    assert!(
        mint.generator == gamma || mint.generator == neg_gamma,
        "the minted generator spans γ, got {:?}",
        mint.generator
    );
    assert_eq!(mind.rank(), 1);
    assert!(mind.resolved(&gamma), "after learning, the obstruction resolves");
}

#[test]
fn the_mind_breathes_rank_grows_and_every_unfolded_obstruction_folds() {
    // Unfold three distinct obstructions across the manifolds; each folds to
    // nothing afterward, and the rank is the geometry the breathing built.
    let mut mind = Mind::new(3, 4);
    let obstructions = [
        (vec![b(1), b(0), b(0)], Manifold::Harmonic),
        (vec![b(0), b(1), b(0)], Manifold::Luminal),
        (vec![b(0), b(0), b(1)], Manifold::Holonomic),
    ];
    for (g, m) in &obstructions {
        assert!(!mind.resolved(g), "unlearned before the in-breath");
        mind.condense(g.clone(), *m).expect("condense");
        assert!(mind.resolved(g), "resolves after the in-breath (fold)");
    }
    assert_eq!(mind.rank(), 3, "the geometry the breathing constructed");
    // A combination of all three now resolves — the learned basis spans them.
    assert!(mind.resolved(&[b(5), b(-2), b(7)]));
}
