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
//! - **A settler's world** (stage 6): a line cleaved from a line on an era-1
//!   world settles the free world nearest its parent's (`nearest`): its
//!   parent's own system first, then its parent's galaxy, the system whose
//!   reach is nearest its parent's first; a world its land feeds before one
//!   it does not (docs/86). A galaxy with no free world leaves
//!   the settler where its cleave put it.
//!
//! The reading is taken whenever the universe or the seating changes
//! (`set`) and read by the decisions that place a line (`core_of`,
//! `in_bulge`, `distance`, `settle`). The worlds claimed are kept across
//! readings (`claim`).
use crate::pulse::{mass, Universe};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

/// A reading of where the universe's systems stand.
#[derive(Debug, Clone, Default)]
pub struct Places {
    /// Each galaxy (its path's first link) and the ledger core it is.
    pub cores: BTreeMap<u64, String>,
    /// Each system ([galaxy, star]): its reach in its galaxy's own units and
    /// its distance from its core.
    pub systems: BTreeMap<(u64, u64), (Ratio, Ratio)>,
    /// Every era-1 world a line can stand on.
    pub sites: BTreeMap<Vec<u64>, crate::seating::Site>,
    /// The worlds lines stand on.
    pub claimed: BTreeSet<Vec<u64>>,
    /// Each system's worlds ([galaxy, star]), and each galaxy's stars.
    pub worlds_in: BTreeMap<(u64, u64), usize>,
    pub stars_in: BTreeMap<u64, usize>,
}

impl Places {
    /// Read the universe against the ledger's charted cores.
    pub fn read(u: &Universe, book: &ledger::Ledger) -> Places {
        let worlds: BTreeMap<Vec<u64>, crate::life::World> = crate::life::worlds(u).into_iter().map(|w| (w.path.clone(), w)).collect();
        let mut out = Places { sites: crate::seating::sites(u, &worlds), ..Places::default() };
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
                out.worlds_in.insert((g, s), sys.satellites.len());
                *out.stars_in.entry(g).or_insert(0) += 1;
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

/// Take the reading (after a seating or a breath that moved the universe),
/// keeping the worlds claimed.
pub fn set(u: &Universe, book: &ledger::Ledger) {
    let mut p = Places::read(u, book);
    if let Ok(mut g) = now().lock() {
        if let Some(old) = g.take() {
            p.claimed = old.claimed;
        }
        *g = Some(p);
    }
}

/// THE SKY of the world at `path` (`gauges`): its own world, its star's
/// other worlds and its galaxy's stars, counted; zero with no reading.
pub fn sky(path: &[u64]) -> usize {
    with(|p| {
        let (g, s) = (*path.first()?, *path.get(1)?);
        Some(p.worlds_in.get(&(g, s)).copied().unwrap_or(1).max(1) + p.stars_in.get(&g).copied().unwrap_or(0))
    })
    .unwrap_or(0)
}

/// The site of the world at `path`, if a line can stand on it.
pub fn site(path: &[u64]) -> Option<crate::seating::Site> {
    with(|p| p.sites.get(path).cloned())
}

/// Why a settler from `parent` found no world: the reading's free worlds in
/// its galaxy, and whether its parent's system still stands.
pub fn unsettled(parent: &[u64]) -> String {
    with(|p| {
        let g = *parent.first()?;
        let free = p.sites.keys().filter(|k| k.first() == Some(&g) && !p.claimed.contains(*k)).count();
        let sites = p.sites.keys().filter(|k| k.first() == Some(&g)).count();
        let stands = parent.get(1).is_some_and(|s| p.systems.contains_key(&(g, *s)));
        Some(format!("{free} of {sites} worlds free in galaxy {g}; the parent's system {}", if stands { "stands" } else { "is gone" }))
    })
    .unwrap_or_else(|| "no reading".into())
}

/// A line stands on the world at `path`.
pub fn claim(path: &[u64]) {
    if let Ok(mut g) = now().lock() {
        if let Some(p) = g.as_mut() {
            p.claimed.insert(path.to_vec());
        }
    }
}

/// Whether a line stands on the world at `path`.
pub fn claimed(path: &[u64]) -> bool {
    with(|p| Some(p.claimed.contains(path))).unwrap_or(false)
}

/// THE NEAREST FREE WORLD to a parent's at `parent`: in its own system
/// first, then in its galaxy by the nearest reach to its own system's (to
/// the core's, where the breath re-anchored its system), then by path. None
/// when its galaxy has no free world.
pub fn nearest<'a>(parent: &[u64], free: impl Iterator<Item = &'a Vec<u64>>, reach: &BTreeMap<(u64, u64), Ratio>) -> Option<Vec<u64>> {
    let (&g, &s) = (parent.first()?, parent.get(1)?);
    // A parent whose system the breath has since re-anchored stands, for
    // the reading, at its galaxy's core: the systems nearest the core first.
    let zero = ratio(0, 1);
    let home = reach.get(&(g, s)).unwrap_or(&zero);
    free.filter(|p| p.first() == Some(&g) && p.len() > 1)
        .filter_map(|p| {
            let r = reach.get(&(g, p[1]))?;
            let apart = if r > home { r.clone() - home.clone() } else { home.clone() - r.clone() };
            Some(((p[1] != s, apart, (*p).clone()), (*p).clone()))
        })
        .min_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, p)| p)
}

/// A settler of `metabolism` from the line at `parent` takes the free world
/// nearest it that its land feeds, else the nearest free world, claimed.
pub fn settle(parent: &[u64], metabolism: &str) -> Option<crate::seating::Site> {
    let mut g = now().lock().ok()?;
    let p = g.as_mut()?;
    let reach: BTreeMap<(u64, u64), Ratio> = p.systems.iter().map(|(k, (_, d))| (*k, d.clone())).collect();
    let path = {
        let (sites, claimed) = (&p.sites, &p.claimed);
        let free = |livable: bool| sites.iter().filter(move |(k, s)| !claimed.contains(*k) && (!livable || s.wild_for(metabolism) > ratio(0, 1))).map(|(k, _)| k);
        nearest(parent, free(true), &reach).or_else(|| nearest(parent, free(false), &reach))?
    };
    let site = p.sites.get(&path)?.clone();
    p.claimed.insert(path);
    Some(site)
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

    #[test]
    fn a_settler_takes_its_parents_system_first_then_the_nearest_reach() {
        let mut reach = BTreeMap::new();
        reach.insert((21, 7), ratio(5, 1));
        reach.insert((21, 8), ratio(6, 1));
        reach.insert((21, 9), ratio(15, 1));
        reach.insert((22, 1), ratio(5, 1));
        let free = [vec![21, 9, 0], vec![21, 8, 0], vec![22, 1, 0], vec![21, 7, 3]];
        assert_eq!(super::nearest(&[21, 7, 1], free.iter(), &reach), Some(vec![21, 7, 3]));
        assert_eq!(super::nearest(&[21, 7, 1], free[..3].iter(), &reach), Some(vec![21, 8, 0]));
        assert_eq!(super::nearest(&[21, 7, 1], free[2..3].iter(), &reach), None);
        // A parent whose system is gone reads from its galaxy's core.
        assert_eq!(super::nearest(&[21, 4, 1], free[..3].iter(), &reach), Some(vec![21, 8, 0]));
    }
}
