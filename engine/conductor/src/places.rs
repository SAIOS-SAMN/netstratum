//! ERA 1'S PLACES FOR THE PEOPLES (operator 2026-10-21: "ancients near the
//! core ... realms EMERGE"): a line seated on an era-1 world stands where that
//! world stands, so its core, whether it stands in the core's bulge, and how
//! far it is from the core are read from the universe, never from the era-0
//! box. A line with no era-1 world keeps the box's readings.
//!
//! - **Its core** is its galaxy's (the world path's first link), named as the
//!   ledger charted it (`seating::chart`, nearest the universal core first).
//! - **The bulge** is the core's own region: a system whose star stands
//!   within its galaxy's throat (reach at most 1 in the galaxy's own units,
//!   where the enclosure law puts the throat, throat 1 and rim 2; the
//!   universal core's for the home).
//! - **Its distance from the core** is its star's reach, the route's arc to
//!   the core (`enclosure::route`).
//!
//! The reading is taken whenever the universe or the seating changes
//! (`set`) and read by the decisions that place a line (`core_of`,
//! `in_bulge`, `distance`).
use crate::pulse::{mass, Universe};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

/// A reading of where the universe's systems stand.
#[derive(Debug, Clone, Default)]
pub struct Places {
    /// Each galaxy (its path's first link) and the ledger core it is.
    pub cores: BTreeMap<u64, String>,
    /// Each system ([galaxy, star]): its reach in its galaxy's own units and
    /// its distance from its core.
    pub systems: BTreeMap<(u64, u64), (Ratio, Ratio)>,
}

impl Places {
    /// Read the universe against the ledger's charted cores.
    pub fn read(u: &Universe, book: &ledger::Ledger) -> Places {
        let mut out = Places::default();
        let mut order: Vec<usize> = (0..u.cores.len()).collect();
        order.sort_by(|a, b| u.cores[*a].radius.cmp(&u.cores[*b].radius).then(a.cmp(b)));
        let unit = crate::breath::unit_mass(u);
        let whole = mass(&u.total());
        for (k, &i) in order.iter().enumerate() {
            let core = &u.cores[i];
            let Some(&g) = core.path.first() else { continue };
            if let Some(name) = crate::seating::charted_core(book, k) {
                out.cores.insert(g, name);
            }
            let held = if crate::pulse::is_home(core) { whole.clone() } else { mass(&core.total()) };
            if unit.is_zero() || held.is_zero() {
                continue;
            }
            let scale = held / unit.clone();
            for sys in &core.satellites {
                let Some(&s) = sys.path.get(1) else { continue };
                out.systems.insert((g, s), (sys.radius.clone() / scale.clone(), sys.radius.clone()));
            }
        }
        out
    }
}

fn now() -> &'static Mutex<Option<Places>> {
    static NOW: OnceLock<Mutex<Option<Places>>> = OnceLock::new();
    NOW.get_or_init(|| Mutex::new(None))
}

/// Take the reading (after a seating or a breath that moved the universe).
pub fn set(u: &Universe, book: &ledger::Ledger) {
    let p = Places::read(u, book);
    if let Ok(mut g) = now().lock() {
        *g = Some(p);
    }
}

fn with<T>(f: impl FnOnce(&Places) -> Option<T>) -> Option<T> {
    now().lock().ok().and_then(|g| g.as_ref().and_then(f))
}

/// The ledger core of the line standing on the world at `path`.
pub fn core_of(path: &[u64]) -> Option<String> {
    with(|p| p.cores.get(path.first()?).cloned())
}

/// Whether the world at `path` stands in its core's bulge: its star within
/// the galaxy's throat (reach ≤ 1 in the galaxy's own units).
pub fn in_bulge(path: &[u64]) -> Option<bool> {
    with(|p| p.systems.get(&(*path.first()?, *path.get(1)?)).map(|(r, _)| *r <= ratio(1, 1)))
}

/// The world at `path`'s distance from its core: its star's reach.
pub fn distance(path: &[u64]) -> Option<Ratio> {
    with(|p| p.systems.get(&(*path.first()?, *path.get(1)?)).map(|(_, d)| d.clone()))
}

#[cfg(test)]
mod tests {
    use super::Places;
    use cfe::ratio::ratio;
    use std::collections::BTreeMap;

    #[test]
    fn a_systems_bulge_and_distance_are_read_from_its_galaxy() {
        // A galaxy whose own units put its throat at 1: a star at own reach ½
        // stands in the bulge, one at 3/2 does not.
        let mut p = Places::default();
        p.cores.insert(21, "core-a".into());
        p.systems.insert((21, 7), (ratio(1, 2), ratio(5, 1)));
        p.systems.insert((21, 9), (ratio(3, 2), ratio(15, 1)));
        let bulge = |path: &[u64]| p.systems.get(&(path[0], path[1])).map(|(r, _)| *r <= ratio(1, 1));
        assert_eq!(bulge(&[21, 7, 1]), Some(true));
        assert_eq!(bulge(&[21, 9, 1]), Some(false));
        assert_eq!(p.cores.get(&21).cloned(), Some("core-a".to_string()));
        let _: BTreeMap<u64, String> = p.cores.clone();
    }
}
