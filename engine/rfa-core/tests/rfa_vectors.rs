//! Port-verification witnesses for the RFA core.
//!
//! The vectors derive from the convergence-engine's own semantics
//! (engine.rs: `for_each_residual` over ALL ordered distinct triples;
//! `coboundary_reduce` = vertex-0 anchoring + lcm/gcd integer
//! normalization), recomputed by hand against the lifted loop. THE
//! PROPORTIONAL LAW (operator ruling 2026-07-31): the coherence functional
//! is `C(Δ) = Σ |R|` — the ancestor's Σ R² is BARRED as an aggregator
//! of difference (squares are the mathematics of minimization; ours is
//! measurement — each residual counts exactly its size, in its own
//! units). Any implementation that reproduces these values implements
//! the same algebra.

use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use rfa_core::{
    coboundary_reduce, coherence_functional, for_each_residual, magnitude, validate_step,
    Delta, GateVerdict, StepBound,
};

fn delta3(d01: Ratio, d12: Ratio, d02: Ratio) -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![d01]);
    d.set_antisym(1, 2, vec![d12]);
    d.set_antisym(0, 2, vec![d02]);
    d
}

/// VECTOR 1 — the coherent state: Δ01 = 1, Δ12 = 1/2, Δ02 = 3/2
/// satisfies the cocycle condition on every triple: C(Δ) = 0 exactly.
#[test]
fn coherent_delta_measures_zero() {
    let d = delta3(ratio(1, 1), ratio(1, 2), ratio(3, 2));
    assert!(coherence_functional(&d).is_zero());
    let mut fired = false;
    for_each_residual(&d, |_, _, _, _, _| fired = true);
    assert!(!fired, "a coherent Delta yields no residual at all");
}

/// VECTOR 2 — the incoherent state: Δ01 = 2, Δ12 = 3, Δ02 = 7. Every
/// ordered distinct triple of {0,1,2} carries residual ±2 (six
/// permutations), so C(Δ) = 6 × |±2| = 12 exactly (the proportional law).
#[test]
fn incoherent_delta_measures_its_residual_exactly() {
    let d = delta3(ratio(2, 1), ratio(3, 1), ratio(7, 1));
    assert_eq!(coherence_functional(&d), ratio(12, 1));
    let mut count = 0;
    for_each_residual(&d, |_, _, _, _, r| {
        assert!(r == &ratio(2, 1) || r == &ratio(-2, 1));
        count += 1;
    });
    assert_eq!(count, 6, "all six ordered distinct triples fire");
}

/// The conservation invariant is a measured distance, zero on every
/// lawfully-constructed Delta.
#[test]
fn constructed_deltas_conserve_exactly() {
    let d = delta3(ratio(1042, 1000), ratio(542, 1000), ratio(1584, 1000));
    assert!(d.verify_conservation());
    assert!(d.conservation_measurement().is_zero());
}

/// VECTOR 3 — a pure coboundary reduces to ZERO: the coherent state is
/// exactly a frame distortion (Δ_ij = p_j − p_i for potentials p), and
/// the H¹ projection strips it whole.
#[test]
fn a_pure_coboundary_reduces_to_zero() {
    let d = delta3(ratio(1, 1), ratio(1, 2), ratio(3, 2));
    let reduced = coboundary_reduce(&d);
    assert!(magnitude(&reduced).is_zero());
    assert!(reduced.verify_conservation());
}

/// VECTOR 4 — the incoherent state's structural content survives
/// reduction: vertex-0 anchoring zeroes Δ01 and Δ02, leaves
/// Δ'12 = 3 − 7 + 2 = −2, and integer normalization (gcd 2) lands the
/// canonical representative Δ'12 = −1.
#[test]
fn reduction_lands_the_canonical_integer_representative() {
    let d = delta3(ratio(2, 1), ratio(3, 1), ratio(7, 1));
    let reduced = coboundary_reduce(&d);
    assert!(reduced.entry(0, 1, 0).is_zero());
    assert!(reduced.entry(0, 2, 0).is_zero());
    assert_eq!(reduced.entry(1, 2, 0), &ratio(-1, 1));
    assert_eq!(reduced.entry(2, 1, 0), &ratio(1, 1), "antisymmetry held");
}

/// VECTOR 5 — normalization clears denominators AND scale: Δ01 = 1/2,
/// Δ12 = 1/3, Δ02 = 7/4 projects to Δ'12 = 1/3 − 7/4 + 1/2 = −11/12;
/// lcm(denominators) = 12 and gcd(numerators) = 11 land the canonical
/// integer representative Δ'12 = −1.
#[test]
fn normalization_clears_every_denominator() {
    let d = delta3(ratio(1, 2), ratio(1, 3), ratio(7, 4));
    let reduced = coboundary_reduce(&d);
    assert_eq!(reduced.entry(1, 2, 0), &ratio(-1, 1));
    for i in 0..3 {
        for j in 0..3 {
            let e = reduced.entry(i, j, 0);
            assert!(
                e.is_integer(),
                "entry ({i},{j}) = {e} must carry denominator 1"
            );
        }
    }
}

/// Two states differing ONLY by a coboundary reduce to the same
/// representative — the class is the content, the frame is not.
#[test]
fn coboundary_shift_does_not_change_the_class() {
    let base = delta3(ratio(2, 1), ratio(3, 1), ratio(7, 1));
    // Shift by the coboundary of potentials p = (0, 5, 1/3):
    // Δ_ij += p_j − p_i.
    let shifted = delta3(
        ratio(2, 1) + ratio(5, 1),               // Δ01 + (p1 − p0)
        ratio(3, 1) + ratio(1, 3) - ratio(5, 1), // Δ12 + (p2 − p1)
        ratio(7, 1) + ratio(1, 3),               // Δ02 + (p2 − p0)
    );
    assert_eq!(
        coboundary_reduce(&base),
        coboundary_reduce(&shifted),
        "the H¹ representative is frame-free"
    );
}

/// The gates: a conserving, bounded step ACCEPTS in EITHER direction
/// along C; each surviving gate refuses typed with its measurement
/// carried whole.
#[test]
fn the_two_gates_rule_typed() {
    let before = delta3(ratio(2, 1), ratio(3, 1), ratio(7, 1)); // C = 12
    let after = delta3(ratio(2, 1), ratio(3, 1), ratio(6, 1)); // C = 6
    let bound = StepBound {
        reference: magnitude(&before),
        epsilon: ratio(1, 1),
    };
    assert_eq!(validate_step(&before, &after, &bound), GateVerdict::Accepted);

    // AND THE SAME STEP REVERSED (C rises 6 → 12) is equally lawful.
    // The struck productivity gate refused this as `Unproductive`,
    // binding the lineage to one direction on C's axis. C may rise; the
    // lineage was never travelling C's lane.
    assert_eq!(
        validate_step(&after, &before, &bound),
        GateVerdict::Accepted,
        "direction along C governs nothing — the lane is struck"
    );

    // Magnitude drift: a tight band refuses a large excursion.
    let tight = StepBound {
        reference: ratio(1, 1),
        epsilon: ratio(1, 100),
    };
    match validate_step(&before, &after, &tight) {
        GateVerdict::MagnitudeDrift { excursion, bound } => {
            assert!(excursion > bound);
        }
        other => panic!("expected magnitude refusal, measured {other:?}"),
    }
}

/// Holding still is lawful: a step that changes nothing accepts.
#[test]
fn the_fixed_point_is_lawful() {
    let d = delta3(ratio(1, 1), ratio(1, 2), ratio(3, 2));
    let bound = StepBound {
        reference: magnitude(&d),
        epsilon: ratio(1, 1),
    };
    assert_eq!(validate_step(&d, &d, &bound), GateVerdict::Accepted);
}
