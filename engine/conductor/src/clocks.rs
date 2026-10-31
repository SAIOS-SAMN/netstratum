//! docs/78 "The clocks": which duty rides which clock. A duty rides the clock
//! of the layer it reads, never one shared year.
//!
//! | layer | its clock | its duties |
//! |---|---|---|
//! | the universal core | the echo, every `T_u` bells | the breath (births, shedding) |
//! | a galaxy, operational | each echo it receives (`T_u`, after its distance) | the realms' term, parliaments, keepers' judgement, gate accounts |
//! | a galaxy, epochal | the rungs of its half-mass layer (its recoherences) | ascension by deed (a full rung kept), the core's re-emission, the aeon's marks |
//! | a star system | its own rungs (its nested slip) | the system's epochs |
//! | a world (orb) | its LOCAL YEAR (one turn about its star) | its journal, upkeep, demography |
//! | the process | a count of echoes | memory, profiling (not physics) |
//!
//! Why two clocks for a galaxy: the galaxy nearest the universal core runs at
//! rate 1 and never slips, and its orbits' own dilation is weak-field (about
//! 1/3,600), so it climbs almost no rungs. Its ordinary business must still
//! run, so it rides the echoes; what marks an epoch happens only where time
//! layers (rungs). An undilated galaxy has no epochs: its ancients cannot
//! arise by deed there. Counts are closed forms between events (`⌊Δ⌋`
//! differences, `⌊n / P⌋`), never stepped.
use crate::breath::{rungs, slip};
use crate::motion::galactic_year;
use crate::pulse::Universe;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The nested clock rate of galaxy `i`'s half-mass system: the galaxy's
/// about the universal core, times the system's about the mass it orbits.
pub fn half_mass_rate(u: &Universe, i: usize, unit: &Ratio) -> Option<Ratio> {
    let (k, _) = galactic_year(&u.cores[i])?;
    Some(crate::breath::system_clock(u, i, k, unit))
}

/// A galaxy's EPOCHS (rungs of its half-mass layer) between two bells.
/// None where the galaxy has no half-mass layer or the count passes the
/// carrier.
pub fn galaxy_epochs(
    u: &Universe,
    i: usize,
    unit: &Ratio,
    t_u: u64,
    from: u64,
    to: u64,
) -> Option<u64> {
    let r = half_mass_rate(u, i, unit)?;
    rungs(&slip(&r, to, t_u))?.checked_sub(rungs(&slip(&r, from, t_u))?)
}

/// A galaxy's operational TERMS between two bells: the echoes it received,
/// each `t_u` after its distance.
pub fn galaxy_terms(u: &Universe, i: usize, t_u: u64, from: u64, to: u64) -> Option<u64> {
    let delay = crate::ring::whole(&u.cores.get(i)?.radius)?;
    let first = delay.checked_add(t_u)?;
    let echoes = |n: u64| {
        if n < first || t_u == 0 {
            0
        } else {
            (n - delay) / t_u
        }
    };
    echoes(to).checked_sub(echoes(from))
}

/// A world's local years completed between two bells: one turn of its orbit
/// about its star (`period` bells a turn), `⌊n / P⌋`.
pub fn local_years(period: &Ratio, from: u64, to: u64) -> Option<u64> {
    if period.is_zero() {
        return None;
    }
    let turns = |n: u64| crate::ring::whole(&(ratio(i64::try_from(n).ok()?, 1) / period.clone()));
    turns(to)?.checked_sub(turns(from)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_years_count_turns_between_bells() {
        assert_eq!(local_years(&ratio(60, 1), 0, 180), Some(3));
        assert_eq!(local_years(&ratio(60, 1), 59, 61), Some(1));
        assert_eq!(local_years(&ratio(90, 1), 100, 170), Some(0));
        assert_eq!(
            local_years(&ratio(0, 1), 0, 10),
            None,
            "a world with no period keeps no years"
        );
    }
}
