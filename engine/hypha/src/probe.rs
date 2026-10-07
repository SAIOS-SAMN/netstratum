//! Transitions with known verdicts, for proving a live court reads as this
//! kernel does: a lawful turn, and the same turn with time standing still.

use mycelium::physics::laws::{BodyState, Transition};
use mycelium::physics::tensegrity::{carried_body, Body};
use mycelium::transducer::cga::{rotator, sphere_at, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(n))
}

fn body(centre: [i64; 3]) -> Body {
    Body {
        centre: centre.iter().map(|&c| q(c)).collect(),
        mass: q(1),
        moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]],
    }
}

/// The (3,4,5) turn, `2 + e₁₂`: a rational rotor, no angle.
pub fn turn() -> Mv {
    rotator(&q(2), &q(1))
}

/// Two bodies carried exactly by `motion`. `None` if it cannot carry them
/// rigidly.
pub fn carried(motion: &Mv) -> Option<Vec<(BodyState, BodyState)>> {
    [[1, 0, 0], [0, 2, 0]]
        .iter()
        .map(|&c| {
            let b = body(c);
            let nucleus = sphere_at(&b.centre, &q(1));
            let after = BodyState { nucleus: motion.sandwich(&nucleus), body: carried_body(&b, motion)? };
            Some((BodyState { nucleus, body: b }, after))
        })
        .collect()
}

/// A lawful step: two bodies turned together, the height advancing.
pub fn lawful(h: u64) -> Option<Transition> {
    let m = turn();
    Some(Transition { motion: m.clone(), heights: (h, h + 1), bodies: carried(&m)?, hues: Vec::new(), fluxes: Vec::new() })
}

/// The same step with the height standing still: temporal monotonicity
/// breached, and only it.
pub fn timeless(h: u64) -> Option<Transition> {
    let m = turn();
    Some(Transition { motion: m.clone(), heights: (h, h), bodies: carried(&m)?, hues: Vec::new(), fluxes: Vec::new() })
}
