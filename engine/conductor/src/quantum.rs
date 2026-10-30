//! docs/79, the first build: the hidden phase and measurement (operator
//! 2026-10-14: quantum in matter; any recorded interaction is a
//! measurement).
//!
//! - **The twist** of a galaxy (docs/100 §2; Xylarium 11.4: "twist per turn
//!   is the deficit"): its core winds each plane by its deficit at its own
//!   ring, ½ a turn a ring, shared by its tilt; in its disc plane, `½·t_disc`
//!   a ring, `φ = ½·t_disc·rate` a bell (rate its rings a bell). Derived,
//!   never drawn (the seed's torsion phase retired).
//! - **The hidden phase** where a world interacts at bell b:
//!   `u = frac(b·φ + b / P_world)`, its galaxy's winding plus the world's own
//!   orbital phase at that bell, in turns. Exact, deterministic, unreadable
//!   from inside the universe.
//! - **A measurement** of an outcome of chance p: it happens when `u < p`.
//!   Every measurement is counted in the universe journal (tag 58); the
//!   outcome is the annal's, never replayed.
use cfe::ratio::{ratio, Ratio};

/// A galaxy's twist a bell as a body in it reads it (docs/100 §2): the core
/// winds each plane by ½ a turn a ring by that plane's tilt share, `φ_k =
/// ½·t_k`, over the rings it receives a bell; a body takes the winding of
/// every plane by its own share of that plane, `Σ_k t_body,k·φ_k` (the versor
/// projected on the body's planes, never flattened to one plane). Untilted
/// reads isotropic (a third a plane).
pub fn twist(rate: &Ratio, core_tilt: &[Ratio], body_tilt: &[Ratio]) -> Ratio {
    let (g, b) = (crate::flows::tilt_of(core_tilt), crate::flows::tilt_of(body_tilt));
    ratio(1, 2) * g.iter().zip(&b).fold(ratio(0, 1), |s, (gk, bk)| s + gk.clone() * bk.clone()) * rate.clone()
}

/// The hidden phase at bell `bell` of a world turning once in `period` bells
/// in a galaxy twisting `twist` a bell.
pub fn hidden_phase(twist: &Ratio, bell: u64, period: &Ratio) -> Ratio {
    let b = ratio(bell as i64, 1);
    let turn = if *period > ratio(0, 1) { b.clone() / period.clone() } else { ratio(0, 1) };
    let u = b * twist.clone() + turn;
    u.clone() - u.floor()
}

/// The outcome of chance `p` at hidden phase `u`.
pub fn measure(u: &Ratio, p: &Ratio) -> bool {
    u < p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hidden_phase_turns_with_the_world_and_its_galaxys_twist() {
        // A core at rate 1 with its disc plane holding half its tilt winds
        // ¼ a bell; at bell 30 of a 60-bell year: 30/4 + ½ = 8, phase 0.
        // A body lying in the core's disc plane takes that plane's winding.
        let t = twist(&ratio(1, 1), &[ratio(1, 2), ratio(1, 4), ratio(1, 4)], &[ratio(1, 1), ratio(0, 1), ratio(0, 1)]);
        assert_eq!(t, ratio(1, 4));
        // Every plane counts by the body's own share: a body across all three.
        assert_eq!(twist(&ratio(1, 1), &[ratio(1, 2), ratio(1, 4), ratio(1, 4)], &[]), ratio(1, 6));
        assert_eq!(hidden_phase(&t, 30, &ratio(60, 1)), ratio(0, 1));
        assert_eq!(hidden_phase(&t, 1, &ratio(60, 1)), ratio(1, 4) + ratio(1, 60));
        // Untilted: a third a plane, ⅙ a ring.
        assert_eq!(twist(&ratio(1, 1), &[], &[]), ratio(1, 6));
        // Closed form: the phase at 4N is the phase at N moved by 3N's turn.
        let (n, p) = (37u64, ratio(60, 1));
        let step = |b: u64| ratio(b as i64, 1) * (t.clone() + ratio(1, 1) / p.clone());
        let f = |x: Ratio| x.clone() - x.floor();
        assert_eq!(hidden_phase(&t, 4 * n, &p), f(hidden_phase(&t, n, &p) + step(3 * n)));
        assert!(measure(&ratio(1, 4), &ratio(1, 2)));
        assert!(!measure(&ratio(3, 4), &ratio(1, 2)));
    }
}
