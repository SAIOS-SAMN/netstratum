//! THE AGREEMENT LAW (SYSTEM_SPEC §5) — Σ_mesh, the re-sync trigger,
//! and resonance at torsion period m=10, re-derived from the v0.1
//! ancestor's `consensus.rs` (equations TAKEN, re-witnessed red-first
//! in `tests/agree_pins.rs`; no sandbox code imported).
//!
//! Three readings, NO votes:
//!
//! - `sigma_mesh` — the weighted agreement aggregate, self weight 1:
//!   `Σ_mesh = (σ_self + Σ_j ω_j·σ_j) / (1 + Σ_j ω_j)`, σ ∈ [0,1].
//! - `drift_resyncs` — the hard re-sync trigger `|dΣ/dK| > τ_Σ`.
//! - `resonance_at_step` — agreement confirmed when the phase proxy
//!   aligns with the m=10 cyclotomic table at a window (K ≡ 0 mod m);
//!   between windows the check is non-blocking by design.
//!
//! What is DELIBERATELY not carried from the ancestor:
//!
//! - **The majority anchor (weight > 1/2) — unlawful here.** The
//!   house law is unanimous-or-refused (NS-5 §1); disagreement is
//!   held in stasis, named, never voted away. Fork resolution beyond
//!   stasis is the open Byzantine rung (SYSTEM_SPEC §7).
//! - **The sluice-weight formula.** Weights ω_j arrive ASSIGNED and
//!   advisory (kernel CHARTER §1/§5b: no canonicalizing weight before
//!   cryptographic identity); the weight-EARNING law is the PoUW
//!   anchor — weight = certificate-verified genuine difference — and
//!   lands at that seam, not here.
//! - **The σ fixed-point encoding** (u32/u32::MAX): exact `Ratio`
//!   throughout, the no-float law.
//!
//! The ancestor's zero-total-weight arm returned 0 — a vacuous pass
//! shape. Here the self seat holds weight 1 STRUCTURALLY, so the
//! denominator is never zero and that arm is unrepresentable.

use cfe::ratio::{ratio, Ratio};

/// The mesh torsion period m — resonance windows sit at K ≡ 0 (mod m).
/// The deep period (m = 1024) lands with its own table when the deep
/// lane boards.
pub const TORSION_PERIOD: u64 = 10;

/// Re(ζ₁₀ᵏ) as exact 6-digit rationals, TAKEN whole from the v0.1
/// ancestor (`constants.rs`); cos-symmetric — cos(k) = cos(m−k) —
/// and exact at the poles.
pub const RESONANCE_M10_COS: [(i64, i64); 10] = [
    (1_000_000, 1_000_000),  // k=0: cos(0)    =  1
    (809_017, 1_000_000),    // k=1: cos(36°)  ≈  0.809017
    (309_017, 1_000_000),    // k=2: cos(72°)  ≈  0.309017
    (-309_017, 1_000_000),   // k=3: cos(108°) ≈ −0.309017
    (-809_017, 1_000_000),   // k=4: cos(144°) ≈ −0.809017
    (-1_000_000, 1_000_000), // k=5: cos(180°) = −1
    (-809_017, 1_000_000),   // k=6: cos(216°) ≈ −0.809017
    (-309_017, 1_000_000),   // k=7: cos(252°) ≈ −0.309017
    (309_017, 1_000_000),    // k=8: cos(288°) ≈  0.309017
    (809_017, 1_000_000),    // k=9: cos(324°) ≈  0.809017
];

/// One peer's reading: its assigned weight and its normalized sigma.
/// Weight is advisory until the PoUW anchor lands; a weight ≤ 0 is
/// silence — the peer is unheard, never negatively counted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerReading {
    pub weight: Ratio,
    pub sigma: Ratio,
}

/// Σ_mesh — the weighted agreement aggregate, self weight 1:
/// `(σ_self + Σ_j ω_j·σ_j) / (1 + Σ_j ω_j)`. The denominator is at
/// least 1 (the self seat), so no input renders this vacuous.
pub fn sigma_mesh(sigma_self: &Ratio, peers: &[PeerReading]) -> Ratio {
    let mut weighted = sigma_self.clone();
    let mut weights = ratio(1, 1);
    for peer in peers {
        if peer.weight <= ratio(0, 1) {
            continue;
        }
        weighted += peer.weight.clone() * peer.sigma.clone();
        weights += peer.weight.clone();
    }
    weighted / weights
}

/// The magnitude of a reading — L1, the proportional law's aggregator.
fn magnitude(value: Ratio) -> Ratio {
    if value < ratio(0, 1) {
        -value
    } else {
        value
    }
}

/// The hard re-sync trigger (SYSTEM_SPEC §5): `|dΣ/dK| > τ_Σ` — one
/// K-step's drift is the difference itself. Strict: sitting exactly
/// at the threshold holds.
pub fn drift_resyncs(sigma_new: &Ratio, sigma_prev: &Ratio, tau_sigma: &Ratio) -> bool {
    magnitude(sigma_new.clone() - sigma_prev.clone()) > *tau_sigma
}

/// The phase proxy φ_r = T_k / T_k_max. A zero maximum reads zero:
/// before the clock has walked there is no phase to hold.
pub fn phi_r(t_k: &Ratio, t_k_max: &Ratio) -> Ratio {
    if *t_k_max == ratio(0, 1) {
        return ratio(0, 1);
    }
    t_k.clone() / t_k_max.clone()
}

/// Resonance at one K-step: windows sit at K ≡ 0 (mod m); between
/// windows the check is NON-BLOCKING by design (the window is where
/// the orbit must align — a documented pass, not a vacuous one). At
/// a window, the phase proxy must sit within τ of Re(ζ_m^K).
pub fn resonance_at_step(k: u64, t_k: &Ratio, t_k_max: &Ratio, tau: &Ratio) -> bool {
    if !k.is_multiple_of(TORSION_PERIOD) {
        return true;
    }
    let (numer, denom) = RESONANCE_M10_COS[(k % TORSION_PERIOD) as usize];
    let aligned = magnitude(phi_r(t_k, t_k_max) - ratio(numer, denom));
    aligned < *tau
}
