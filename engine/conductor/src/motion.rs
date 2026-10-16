//! docs/78 era 1, build step 3: MOTION. Every body turns about its centre by
//! the motion law, `ω = δ` turns a received ring with `δ = H / (R + H)`
//! (`crate::ring`), so its turn takes `P = 1 + R / H` rings (the pulse records P beside r). A body's phase at bell n is
//! `φ + n / P` turns, its completed turns `⌊φ + n / P⌋`: a closed form whose
//! denominator never grows (design laws 5 and 8). Nothing is stepped.
//!
//! - **The galactic year** is one turn of the orbit at the galaxy's
//!   HALF-MASS radius: the first system whose orbit encloses half the
//!   galaxy's bound mass (the core and its systems; the medium lies beyond
//!   every orbit). Each galaxy closes its own year at its own pace, and the
//!   core's echo of the pulse sounds at it.
//! - **The local year** of an orb is one turn about its star.
//! - **φ at the pulse is 0** until the quantum layer gives it (design law
//!   7, docs/79); the core's bell is the universal beat until the clock's
//!   rate law (docs/80) is stated.
use crate::pulse::{mass, Body, Universe};
use cfe::ratio::{ratio, Ratio};

/// A turning: its period in bells and its phase at the pulse, in turns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turning {
    pub period: Ratio,
    pub phase0: Ratio,
}

impl Turning {
    /// Phase at bell n, in turns.
    pub fn phase(&self, n: u64) -> Ratio {
        self.phase0.clone() + ratio(n as i64, 1) / self.period.clone()
    }

    /// Turns completed by bell n.
    pub fn turns(&self, n: u64) -> u64 {
        self.phase(n).floor().to_integer().try_into().unwrap_or(0)
    }

    /// The first bell at which turn `k` is complete.
    pub fn bell_of_turn(&self, k: u64) -> u64 {
        let need = (ratio(k as i64, 1) - self.phase0.clone()) * self.period.clone();
        need.ceil().to_integer().try_into().unwrap_or(0)
    }
}

/// A body's turning about its centre (none for a core: cores do not orbit).
pub fn turning(b: &Body) -> Option<Turning> {
    (b.period > ratio(0, 1)).then(|| Turning { period: b.period.clone(), phase0: ratio(0, 1) })
}

/// The galaxy's year: the turning of the orbit at its half-mass radius.
pub fn galactic_year(core: &Body) -> Option<(usize, Turning)> {
    let bound = core.satellites.iter().fold(mass(&core.kept), |s, sys| s + mass(&sys.total()));
    let mut enclosed = mass(&core.kept);
    for (k, sys) in core.satellites.iter().enumerate() {
        enclosed += mass(&sys.total());
        if enclosed.clone() * ratio(2, 1) >= bound {
            return turning(sys).map(|t| (k, t));
        }
    }
    None
}

/// Every galaxy's year closes due at bell n: the galaxies (by path) whose
/// half-mass orbit completed a turn at exactly n.
pub fn years_closing(u: &Universe, n: u64) -> Vec<u32> {
    u.cores
        .iter()
        .filter_map(|c| galactic_year(c).map(|(_, t)| (c.path[0] as u32, t)))
        .filter(|(_, t)| n > 0 && t.turns(n) > t.turns(n - 1))
        .map(|(p, _)| p)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_turning_counts_its_turns_exactly() {
        let t = Turning { period: ratio(7, 2), phase0: ratio(0, 1) };
        assert_eq!(t.turns(3), 0);
        assert_eq!(t.turns(4), 1);
        assert_eq!(t.bell_of_turn(2), 7);
        assert_eq!(t.phase(1_000_000_000).denom(), &num_bigint::BigInt::from(7), "the denominator never grows");
    }
}
