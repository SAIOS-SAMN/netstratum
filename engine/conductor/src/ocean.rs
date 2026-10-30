//! The ocean: the return leg of the recycling circuit (docs/83, "Recycling
//! and recyclotronic propulsion"; with Xylarium). The ocean is motes
//! (operator 2026-10-25: "jetstreams of vacuum/continuum via the ocean of
//! space"; "build the ocean as motes"): `Universe::ocean`, each mote in
//! transit on the vacuum leg (`pulse::orbit_galaxies` torques it, its reach
//! read by `pulse::ocean_frame`), landing whole in the one galaxy whose pull
//! is greatest at its reach among those whose swing holds it (`breath`). The
//! standing crests and the gathered share below are RETIRED (they divided the
//! ocean by holdings); the swing, the crossing's turning and the pump stay.
//!
//! - **The crests.** The universal core regenerates its pulse at full
//!   strength (operator ruling (i)); in universe units `H_u` (`universe_hold`:
//!   its state 1 and every galactic core with its galaxy) and its bell rings once in `P_b = P(H_u) = 2` own rings, so
//!   the crests stand at `R_{p/q} = H_u(P_b·p/q − 1) = H_u(2p/q − 1)` core rings
//!   for `½ < p/q ≤ 1`. A crest couples to the torque by `a_q = 1/q` for odd q,
//!   0 for even (Xylarium's ruling).
//! - **Crests form with age** (proposed): a crest of order q is in phase only
//!   after q passes of the torque, so at echo n the crests with `q ≤ n` stand.
//!   No constant bounds them; the structure grows with the universe.
//! - **The ocean on the crests** (proposed): the torque collects it by coupling,
//!   crest p/q holding `a_q / Σ a` of it; stateless, re-read every echo.
//! - **The swing** (docs/83, "The swing"): a galactic core swings about its
//!   reach R between `R(1 − e)` and `R(1 + e)`, `e = τ/(τ + h_g)`, the
//!   universal core's torsion `τ = R/(R + H_u)` (the face it twists, not the
//!   face it holds) against the galaxy's own holding `h_g`.
//! - **Gathering by crossings, not time** (Xylarium (C)): every half-swing
//!   crosses each standing crest within the swing once; at each crossing
//!   the galaxy takes the in-phase share `a_q` of the medium on it, scaled by
//!   its own pull `h_g/(h_g + m)` (a pair share). Crossings are counted on
//!   whole half-swings elapsed, exact, never stepped.

use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The universal core's holding (operator: it must be strong enough to
/// contain every galaxy's orbit, the load distributed between the galactic
/// cores; with Xylarium): one law at every tier, a core holds its state and
/// what it holds. In universe units (the founding's grains 1) its state is
/// 1, set at the bounce and never drained, and it holds every galactic core
/// with its galaxy: `H_u = 1 + Σ_g (state_g + grains_g)` (3 at a founding
/// with no ocean). Matter in the ocean is not held by a galaxy, so the hold
/// breathes with the circuit. The magnetism counts too (operator
/// 2026-10-16; Xylarium 11.0 (1), the galaxies' magnetism sets the core's
/// strength): each held nucleus bears on the universal core at its whole
/// 3Z, holding plus magnetism (its unpaired frames), not its holding face
/// alone: `H_u = 1 + Σ_g (state_g + (holding_g + magnetism_g))`, read in the
/// founding's unit (the universe's grains' holding 1).
pub fn universe_hold(u: &crate::pulse::Universe) -> Ratio {
    // The founding's unit, fixed: the galaxies' states (their grains at the
    // bounce, never drained). The current total holding rises as burning
    // transduces magnetism into holding, so it would drift the strength the
    // span (3Z, conserved) already holds (read 2026-10-17: 3.162 -> 3.060).
    let whole = u.cores.iter().fold(Ratio::from_integer(0.into()), |s, c| s + crate::pulse::core_state(c));
    if whole.is_zero() {
        return ratio(2, 1);
    }
    u.cores.iter().fold(ratio(1, 1), |s, c| {
        let all = c.total();
        s + crate::ring::grain(&((crate::pulse::core_state(c) + crate::pulse::mass(&all)) / whole.clone()))
    })
}

/// A galaxy's swing about reach `r` (core rings), its holding `h_g` in
/// universe units: `(R₋, R₊)`.
pub fn swing(r: &Ratio, h_g: &Ratio, h_u: &Ratio) -> (Ratio, Ratio) {
    let h_u = h_u.clone();
    let tau = r.clone() / (r.clone() + h_u);
    if (tau.clone() + h_g.clone()).is_zero() {
        return (r.clone(), r.clone());
    }
    let e = tau.clone() / (tau + h_g.clone());
    (r.clone() * (ratio(1, 1) - e.clone()), r.clone() * (ratio(1, 1) + e))
}


/// The swing's radial share at a crossing of the crest at `at` (proposed; a
/// reading, never a trajectory): the crossing's place along the swing as a
/// share `u = (at − R₋)/(R₊ − R₋)`, read by the coherence form
/// `e·(1 − |2u − 1|)`: zero at the turnings, e at the guiding reach, never
/// more than e; no square, no root.
pub fn radial_share(r: &Ratio, h_g: &Ratio, at: &Ratio, h_u: &Ratio) -> Ratio {
    let (lo, hi) = swing(r, h_g, h_u);
    if hi <= lo {
        return ratio(0, 1);
    }
    let e = (hi.clone() - r.clone()) / r.clone();
    let u = (at.clone() - lo.clone()) / (hi - lo);
    let off = ratio(2, 1) * u - ratio(1, 1);
    let off = if off < ratio(0, 1) { -off } else { off };
    e * (ratio(1, 1) - off).max(ratio(0, 1))
}


/// The pump (convection, with Xylarium; docs/83): the universal tide a
/// swinging galaxy feels differs between its turnings,
/// `π = δ_u(R₋) − δ_u(R₊)`, zero for a circle. The outside tide enters the
/// galaxy as one received quantity, so it varies only in time, with the
/// swing.
pub fn pump(r: &Ratio, h_g: &Ratio, h_u: &Ratio) -> Ratio {
    let h = h_u.clone();
    let (lo, hi) = swing(r, h_g, h_u);
    let d = |x: &Ratio| h.clone() / (x.clone() + h.clone());
    d(&lo) - d(&hi)
}
