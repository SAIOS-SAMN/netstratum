//! THE THERMAL CLOSURE — the temperature line, closed, and the orbit
//! grain that keeps it holdable.
//!
//! `crate::thermal::band_of` returns `Option<SubstrateBand>` and answers
//! NONE in four places, which the tree's own witnesses already record:
//! below the cryogenic floor, between cryogenic and mesic, between
//! mesic and hyperthermal, and above hyperthermal.
//!
//! A `None` is not a temperature. Every orb has one, so a reading that
//! declines to name it is a hole in the vocabulary rather than a
//! refusal. This closes the line by NAMING the uncovered stretches, and
//! changes no admission: `SubstrateBand` still rules siting and every
//! temperature that had a band keeps it.

use super::periodic::{periodic_of, PHASES};
use crate::ratio::{ratio, Ratio};

/// The temperature line, closed. Every `t_eq4` names one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThermalRegion {
    /// Below the cryogenic floor. The deepest cold, and a place — not
    /// an absence.
    Quenched,
    /// 10 K – 100 K.
    Cryogenic,
    /// 100 K – 250 K. A stretch `band_of` answered NONE for.
    Sublime,
    /// 250 K – 320 K.
    Mesic,
    /// 320 K – 600 K. The second silence.
    Torrid,
    /// 600 K – 1500 K.
    HyperThermal,
    /// Above 1500 K, up to the ceiling.
    Incandescent,
    /// **ABOVE THE CEILING.** Reachable only by an UNGRAINED orbit; with
    /// [`orbit_on_grain`] applied it cannot occur. The arm exists to be
    /// unreachable, and naming it is what let that be checked.
    Beyond,
}

/// **THE ORBIT GRAIN** — the lattice a lawful orbit sits on.
///
/// Heat is amplified by closeness — `t_eq4 = L(1-a)k / r²` — and the
/// division is where irrationality enters: `r -> 0` is unbounded in
/// VALUE, and `r = 1/10^k` is unbounded in DENOMINATOR (~`10^(2k)`, the
/// blowup that killed proportional demurrage at ~2^1100).
///
/// The engine already answers this one layer down: the melt law posts
/// in whole lots on a `1/64` grid *"so exact arithmetic stays cheap"*.
/// An orbit is the same kind of quantity and takes the same discipline.
/// This is that grid, not a new number.
pub const ORBIT_GRAIN: i64 = 64;

/// Snap an orbit to the grain, never below one step. An orbit inside
/// the first step is not a nearer orbit; it is not an orbit. Floors
/// rather than rounds, so nothing is moved CLOSER than it asked.
pub fn orbit_on_grain(orbit_r: &Ratio) -> Ratio {
    use num_bigint::BigInt;
    let grid = BigInt::from(ORBIT_GRAIN);
    let stepped = orbit_r.numer() * &grid / orbit_r.denom();
    let at_least_one = if stepped < BigInt::from(1) { BigInt::from(1) } else { stepped };
    Ratio::new(at_least_one, grid)
}

/// **THE DERIVED CEILING** — the brightest class, zero albedo, one grain
/// out. No number is chosen: it falls out of the luminosity table and
/// the grain, both already present.
pub fn ceiling4() -> Ratio {
    crate::thermal::t_eq4(
        &crate::thermal::StarClass::O.luminosity(),
        &ratio(0, 1),
        &orbit_on_grain(&ratio(0, 1)),
    )
}

/// Read the region of a temperature. **TOTAL** — every `t_eq4` names a
/// region, and that is the closure.
pub fn region_of(t_eq4: &Ratio) -> ThermalRegion {
    use crate::thermal::SubstrateBand as B;
    let (cryo_lo, cryo_hi) = B::Cryogenic.bounds4();
    let (mesic_lo, mesic_hi) = B::Mesic.bounds4();
    let (hyper_lo, hyper_hi) = B::HyperThermal.bounds4();
    match t_eq4 {
        t if *t < cryo_lo => ThermalRegion::Quenched,
        t if *t <= cryo_hi => ThermalRegion::Cryogenic,
        t if *t < mesic_lo => ThermalRegion::Sublime,
        t if *t <= mesic_hi => ThermalRegion::Mesic,
        t if *t < hyper_lo => ThermalRegion::Torrid,
        t if *t <= hyper_hi => ThermalRegion::HyperThermal,
        t if *t <= ceiling4() => ThermalRegion::Incandescent,
        _ => ThermalRegion::Beyond,
    }
}

/// The heat an orb takes from its star, distance quantised. Identical
/// to `crate::thermal::t_eq4` but for putting the orbit on the grain
/// first — as fierce as the physics allows, as bounded as the
/// arithmetic requires.
pub fn grained_t_eq4(luminosity: &Ratio, albedo: &Ratio, orbit_r: &Ratio) -> Ratio {
    crate::thermal::t_eq4(luminosity, albedo, &orbit_on_grain(orbit_r))
}

/// Is an element workable at this temperature? The TOTAL form of
/// [`super::locality::workable_in`]. The refractory phase works in any
/// NAMED region; nothing is workable `Beyond`.
pub fn workable_at(element: &[u8; 32], t_eq4: &Ratio) -> bool {
    let region = region_of(t_eq4);
    if region == ThermalRegion::Beyond {
        return false;
    }
    let phase = periodic_of(element).phase;
    if phase == PHASES {
        return true;
    }
    matches!(
        (phase, region),
        (1, ThermalRegion::Quenched | ThermalRegion::Cryogenic | ThermalRegion::Sublime)
            | (2, ThermalRegion::Mesic)
            | (3, ThermalRegion::Torrid | ThermalRegion::HyperThermal | ThermalRegion::Incandescent)
    )
}
