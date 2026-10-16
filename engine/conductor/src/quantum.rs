//! docs/79, the first build: the hidden phase and measurement (operator
//! 2026-10-14: quantum in matter; any recorded interaction is a
//! measurement).
//!
//! - **The torsion phase** of a galaxy: its mode's phase in the pulse's
//!   spectrum, in turns, read from the seed (a spectrum field until the
//!   vacuum's integers define it).
//! - **The hidden phase** where a world interacts at bell b:
//!   `u = frac(φ_galaxy + b / P_world)`, the galaxy's torsion phase plus the
//!   world's own orbital phase at that bell, in turns. Exact, deterministic,
//!   unreadable from inside the universe.
//! - **A measurement** of an outcome of chance p: it happens when `u < p`.
//!   Every measurement is counted in the universe journal (tag 58); the
//!   outcome is the annal's, never replayed.
use cfe::ratio::{ratio, Ratio};

/// A galaxy's torsion phase, in turns (3,600ths: the base-60 circle).
pub fn torsion_phase(seed: &[u8; 32], galaxy: u64) -> Ratio {
    ratio((crate::pulse::draw(seed, &[galaxy], "torsion", 0) % 3600) as i64, 3600)
}

/// The hidden phase at bell `bell` of a world turning once in `period` bells.
pub fn hidden_phase(torsion: &Ratio, bell: u64, period: &Ratio) -> Ratio {
    let turn = if *period > ratio(0, 1) { ratio(bell as i64, 1) / period.clone() } else { ratio(0, 1) };
    let u = torsion.clone() + turn;
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
    fn the_hidden_phase_turns_with_the_world() {
        let t = ratio(1, 4);
        assert_eq!(hidden_phase(&t, 30, &ratio(60, 1)), ratio(3, 4));
        assert_eq!(hidden_phase(&t, 60, &ratio(60, 1)), ratio(1, 4));
        assert!(measure(&ratio(1, 4), &ratio(1, 2)));
        assert!(!measure(&ratio(3, 4), &ratio(1, 2)));
    }
}
