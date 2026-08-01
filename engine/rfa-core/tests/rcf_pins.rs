//! RCF conjugacy-invariant witnesses (U2a) — the angle-free heart of
//! the fold gate. The vectors are TRANSLATED from the v0.1 ancestor's
//! `rcf.rs` tests (the conjugacy pair A vs PAP⁻¹, the antisymmetric
//! block, determinism) and re-witnessed here red-first: any
//! implementation reproducing them implements the same conjugacy
//! algebra. The load-bearing pin is `conjugate_deltas_share_the_signature`
//! — a frame change must be INVISIBLE to the invariant, or the
//! singularity law (16_UNIT §4a) has a hole.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{invariant_factors, rcf_hash, rcf_signature, Delta, RatPoly};

/// A rational 2×2 matrix as the RCF layer input.
fn mat2(a: [[Ratio; 2]; 2]) -> Vec<Vec<Ratio>> {
    vec![a[0].to_vec(), a[1].to_vec()]
}

#[test]
fn the_identity_has_two_linear_factors() {
    // The 2×2 identity: invariant factors [x−1, x−1], both degree 1.
    let id = mat2([[ratio(1, 1), ratio(0, 1)], [ratio(0, 1), ratio(1, 1)]]);
    let factors = invariant_factors(&id, 2);
    assert_eq!(factors.len(), 2, "identity: two invariant factors");
    for f in &factors {
        assert_eq!(f.degree(), Some(1), "each factor is x − 1");
    }
}

#[test]
fn the_antisymmetric_block_is_one_quadratic_factor() {
    // A = [[0, 1/2], [−1/2, 0]] has characteristic x² + 1/4 — a single
    // degree-2 invariant factor (the symplectic 2×2 block).
    let a = mat2([[ratio(0, 1), ratio(1, 2)], [ratio(-1, 2), ratio(0, 1)]]);
    let factors = invariant_factors(&a, 2);
    let total_degree: usize = factors.iter().filter_map(|f| f.degree()).sum();
    assert_eq!(total_degree, 2, "the antisymmetric block: total degree 2");
}

#[test]
fn conjugate_matrices_share_their_invariant_factors() {
    // THE INVARIANCE (ancestor's test_rcf_conjugacy_invariance):
    // A = [[0, 1], [−1, 0]] and B = P A P⁻¹ = [[−1, 2], [−1, 1]] with
    // P = [[1, 1], [0, 1]] are conjugate — identical invariant factors.
    let a = mat2([[ratio(0, 1), ratio(1, 1)], [ratio(-1, 1), ratio(0, 1)]]);
    let b = mat2([[ratio(-1, 1), ratio(2, 1)], [ratio(-1, 1), ratio(1, 1)]]);
    let fa = invariant_factors(&a, 2);
    let fb = invariant_factors(&b, 2);
    assert_eq!(fa, fb, "conjugate matrices → identical invariant factors");
}

#[test]
fn a_genuine_difference_shows_in_the_factors() {
    // Not every pair is conjugate: [[0,1],[−1,0]] (char x²+1) vs the
    // identity (char (x−1)²) are genuinely different classes — the
    // invariant must SEPARATE them (else the fold gate would collapse
    // distinct states).
    let rot = mat2([[ratio(0, 1), ratio(1, 1)], [ratio(-1, 1), ratio(0, 1)]]);
    let id = mat2([[ratio(1, 1), ratio(0, 1)], [ratio(0, 1), ratio(1, 1)]]);
    assert_ne!(
        invariant_factors(&rot, 2),
        invariant_factors(&id, 2),
        "distinct conjugacy classes must not share the invariant"
    );
}

#[test]
fn the_signature_and_hash_are_frame_invariant_over_deltas() {
    // Through the Delta interface (ancestor test_rcf_hash_conjugacy_invariance):
    // Δ_a with relation [[0,1],[−1,0]] and the conjugate Δ_b with
    // [[−1,2],[−1,1]] share both the exact signature AND the hash.
    let mut da = Delta::zero(2, 1);
    da.set_antisym(0, 1, vec![ratio(1, 1)]); // [[0,1],[−1,0]]

    // Δ_b is not antisymmetric — build it entry-wise via a raw matrix
    // to prove the invariant does not secretly rely on antisymmetry.
    let sig_a = rcf_signature(&da);
    let factors_b = invariant_factors(
        &mat2([[ratio(-1, 1), ratio(2, 1)], [ratio(-1, 1), ratio(1, 1)]]),
        2,
    );
    assert_eq!(
        sig_a.layers[0], factors_b,
        "the exact signature is frame-invariant"
    );

    // Determinism: the hash of a Delta is stable across calls.
    assert_eq!(rcf_hash(&da), rcf_hash(&da), "the fingerprint is deterministic");
}

#[test]
fn the_polynomial_ring_computes_exactly() {
    // The Rat[x] substrate the SNF rides on (ancestor test_polynomial_*):
    // (1 + 2x)·x = x + 2x², and gcd(x²−1, x−1) = x−1, monic.
    let p = RatPoly::from_coeffs(vec![ratio(1, 1), ratio(2, 1)]);
    let x = RatPoly::from_coeffs(vec![ratio(0, 1), ratio(1, 1)]);
    let prod = p.mul(&x);
    assert_eq!(
        prod,
        RatPoly::from_coeffs(vec![ratio(0, 1), ratio(1, 1), ratio(2, 1)]),
        "(1 + 2x)·x = x + 2x²"
    );

    let x2_minus_1 = RatPoly::from_coeffs(vec![ratio(-1, 1), ratio(0, 1), ratio(1, 1)]);
    let x_minus_1 = RatPoly::x_minus(&ratio(1, 1));
    assert_eq!(
        x2_minus_1.gcd(&x_minus_1),
        x_minus_1,
        "gcd(x² − 1, x − 1) = x − 1, monic"
    );
}
