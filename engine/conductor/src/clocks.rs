//! docs/78 "The clocks": which duty rides which clock. A duty rides the clock
//! of the layer it reads, never one shared year.
//!
//! | layer | its clock | its duties |
//! |---|---|---|
//! | the universal core | the echo, every `T_u` bells | the breath (births, shedding) |
//! | a galaxy, operational | each echo it receives (`T_u`, after its distance) | the realms' term, parliaments, keepers' judgement, gate accounts |
//! | a galaxy, epochal | the RUNGS of its half-mass layer (its recoherences) | ascension by deed (a full rung kept), the core's re-emission, the aeon's marks |
//! | a star system | its own rungs (its nested slip) | the system's epochs |
//! | a world (orb) | its LOCAL YEAR (one turn about its star) | its journal, upkeep, demography |
//! | the process | a count of echoes | memory, profiling (not physics) |
//!
//! Why two clocks for a galaxy: the galaxy nearest the universal core runs at
//! rate 1 and never slips, and its orbits' own dilation is weak-field (about
//! 1/3,600), so it climbs almost no rungs. Its ordinary business must still
//! run, so it rides the echoes; what marks an EPOCH happens only where time
//! layers (rungs). An undilated galaxy has no epochs: its ancients cannot
//! arise by deed there. Counts are closed forms between events (`⌊Δ⌋`
//! differences, `⌊n / P⌋`), never stepped.
use crate::breath::{galaxy_rate, rate, rungs, slip};
use crate::motion::galactic_year;
use crate::pulse::{mass, Universe};
use cfe::ratio::{ratio, Ratio};

/// The nested clock rate of galaxy `i`'s half-mass system: the galaxy's
/// about the universal core, times the system's about the mass it orbits.
pub fn half_mass_rate(u: &Universe, i: usize, unit: &Ratio) -> Option<Ratio> {
    let core = &u.cores[i];
    let (k, _) = galactic_year(core)?;
    let enclosed = core.satellites[..k].iter().fold(mass(&core.kept), |s, sys| s + mass(&sys.total())) / unit.clone();
    Some(galaxy_rate(u, i, unit) * rate(&core.satellites[k].radius, &enclosed))
}

/// A galaxy's EPOCHS (rungs of its half-mass layer) between two bells.
pub fn galaxy_epochs(u: &Universe, i: usize, unit: &Ratio, t_u: u64, from: u64, to: u64) -> u64 {
    half_mass_rate(u, i, unit).map(|r| rungs(&slip(&r, to, t_u)) - rungs(&slip(&r, from, t_u))).unwrap_or(0)
}

/// A galaxy's operational TERMS between two bells: the echoes it received,
/// each `t_u` after its distance.
pub fn galaxy_terms(u: &Universe, i: usize, t_u: u64, from: u64, to: u64) -> u64 {
    let delay: u64 = u.cores[i].radius.floor().to_integer().try_into().unwrap_or(0);
    let echoes = |n: u64| if n < delay + t_u { 0 } else { (n - delay) / t_u };
    echoes(to) - echoes(from)
}

/// A world's local years completed between two bells: one turn of its orbit
/// about its star (`period` bells a turn), `⌊n / P⌋`.
pub fn local_years(period: &Ratio, from: u64, to: u64) -> u64 {
    let turns = |n: u64| -> u64 { (ratio(n as i64, 1) / period.clone()).floor().to_integer().try_into().unwrap_or(0) };
    turns(to) - turns(from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_years_count_turns_between_bells() {
        assert_eq!(local_years(&ratio(60, 1), 0, 180), 3);
        assert_eq!(local_years(&ratio(60, 1), 59, 61), 1);
        assert_eq!(local_years(&ratio(90, 1), 100, 170), 0);
    }
}
