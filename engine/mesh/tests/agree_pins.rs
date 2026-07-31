//! THE AGREEMENT LAW pins (SYSTEM_SPEC §5, mesh #3) — Σ_mesh, the
//! re-sync trigger, and resonance at torsion period m=10, re-derived
//! from the v0.1 ancestor's `consensus.rs` (equations TAKEN,
//! re-witnessed here; no sandbox code imported). What is DELIBERATELY
//! not carried: the majority anchor (weight > 1/2) — the house law is
//! unanimous-or-refused (NS-5 §1), disagreement held in stasis, never
//! voted away — and the σ fixed-point encoding (exact Ratio instead).

use cfe::ratio::{ratio, Ratio};
use mesh::agree::{
    drift_resyncs, phi_r, resonance_at_step, sigma_mesh, PeerReading, RESONANCE_M10_COS,
    TORSION_PERIOD,
};

fn peer(weight: (i64, i64), sigma: (i64, i64)) -> PeerReading {
    PeerReading { weight: ratio(weight.0, weight.1), sigma: ratio(sigma.0, sigma.1) }
}

/// Σ_mesh with no peers IS the self reading — the self seat has
/// weight 1 structurally, so the ancestor's vacuous zero-denominator
/// arm is UNREPRESENTABLE here (the audit's no-vacuous-pass law).
#[test]
fn sigma_mesh_alone_is_the_self_reading() {
    let sigma = sigma_mesh(&ratio(1, 2), &[]);
    assert_eq!(sigma, ratio(1, 2), "no peers: the aggregate is the self sigma");
}

/// The weighted mean, exact: (σ_self + Σ ω·σ) / (1 + Σ ω).
#[test]
fn sigma_mesh_weighs_agreement_exactly() {
    // Self σ=1; one full-weight peer at σ=0 → (1 + 0) / 2 = 1/2.
    assert_eq!(
        sigma_mesh(&ratio(1, 1), &[peer((1, 1), (0, 1))]),
        ratio(1, 2)
    );
    // Self σ=1; two half-weight peers at σ=1 and σ=0
    // → (1 + 1/2 + 0) / (1 + 1) = 3/4.
    assert_eq!(
        sigma_mesh(&ratio(1, 1), &[peer((1, 2), (1, 1)), peer((1, 2), (0, 1))]),
        ratio(3, 4)
    );
}

/// A weightless (or malformed negative-weight) peer is SILENCE — it
/// moves nothing. Weight is earned at the PoUW anchor; until then it
/// is assigned and advisory, and zero means unheard.
#[test]
fn a_weightless_peer_moves_nothing() {
    let sigma = sigma_mesh(
        &ratio(1, 3),
        &[peer((0, 1), (1, 1)), peer((-1, 1), (1, 1))],
    );
    assert_eq!(sigma, ratio(1, 3), "zero and negative weights are excluded whole");
}

/// The hard re-sync trigger: |dΣ/dK| > τ_Σ — drift beyond the
/// threshold re-syncs; sitting exactly AT the threshold does not
/// (the bound is strict, the ancestor's form).
#[test]
fn drift_beyond_tau_resyncs_and_at_tau_holds() {
    assert!(drift_resyncs(&ratio(3, 10), &ratio(1, 10), &ratio(1, 20)));
    assert!(!drift_resyncs(&ratio(1, 10), &ratio(1, 10), &ratio(1, 20)));
    // Exactly at τ: |2/10 − 1/10| = 1/10 = τ → holds, strictly.
    assert!(!drift_resyncs(&ratio(2, 10), &ratio(1, 10), &ratio(1, 10)));
    // Drift is a magnitude: the same gap downward triggers alike.
    assert!(drift_resyncs(&ratio(1, 10), &ratio(3, 10), &ratio(1, 20)));
}

/// The phase proxy φ_r = T_k / T_k_max; a zero maximum reads zero —
/// before the clock has walked there is no phase to hold.
#[test]
fn phi_reads_the_clock_fraction() {
    assert_eq!(phi_r(&ratio(1, 2), &ratio(1, 1)), ratio(1, 2));
    assert_eq!(phi_r(&ratio(1, 2), &ratio(0, 1)), ratio(0, 1));
}

/// Resonance windows sit at K ≡ 0 (mod m); between windows the check
/// is NON-BLOCKING by design (documented pass, not a vacuous one —
/// the window is where the orbit must align).
#[test]
fn resonance_gates_only_at_the_window() {
    let tau = ratio(1, 100);
    // k=3 is no window: passes regardless of phase.
    assert!(resonance_at_step(3, &ratio(1, 2), &ratio(1, 1), &tau));
    // k=10 IS a window: φ=1 aligns with cos(0)=1 exactly…
    assert!(resonance_at_step(10, &ratio(1, 1), &ratio(1, 1), &tau));
    // …and φ=1/2 misses it by 1/2 ≫ τ.
    assert!(!resonance_at_step(10, &ratio(1, 2), &ratio(1, 1), &tau));
    // k=0, the first window, aligns at φ=1.
    assert!(resonance_at_step(0, &ratio(1, 1), &ratio(1, 1), &tau));
}

/// The taken table is cos-symmetric — cos(k) = cos(m−k) for k=1..4 —
/// and exact at the poles (SYSTEM_SPEC §5's RESONANCE_M10_COS).
#[test]
fn the_resonance_table_is_taken_whole() {
    assert_eq!(TORSION_PERIOD, 10, "mesh torsion period m = 10");
    for k in 1..5usize {
        assert_eq!(
            RESONANCE_M10_COS[k],
            RESONANCE_M10_COS[10 - k],
            "cos-symmetry: cos({k}) = cos({})",
            10 - k
        );
    }
    let pole = |k: usize| -> Ratio {
        let (n, d) = RESONANCE_M10_COS[k];
        ratio(n, d)
    };
    assert_eq!(pole(0), ratio(1, 1), "cos(0) is exactly 1");
    assert_eq!(pole(5), ratio(-1, 1), "cos(π) is exactly −1");
}
