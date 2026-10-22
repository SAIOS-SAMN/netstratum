//! Era 1's peoples on era 1's worlds (docs/82, operator 2026-10-14: the
//! original civilizations and polities are brought over as era 1's first
//! peoples; life still arises by the ladder beside them).
//!
//! - **The galaxies are charted** on the universe ledger from the pulse's
//!   cores, so the conductor's galaxy and core readings find them. Each
//!   stands at a STAND-IN box position (a 3 × 3 × 3 lattice on the 64-cell
//!   box, 21 cells apart, beyond the 16-cell reach), nearest the universal
//!   core first: docs/78's flat-site table replaces it with the sphere (A1).
//! - **Each lineage's head is seated in its own star system** while one is
//!   free, on the world it is most viable on, the galaxies taken in turn so
//!   every galaxy receives peoples; **its
//!   descendants settle the free worlds nearest it** (`seat`). A system's stand-in position sits within its
//!   galaxy's reach. A line keeps its own star class and orbit until the
//!   ladder's temperature law (docs/82) reads them from its world.
use crate::pulse::Universe;
use crate::PolitySpec;
use cfe::ratio::{ratio, Ratio};
use ledger::Ledger;

type Place = (Ratio, Ratio, Ratio);

/// The stand-in centre of the k-th galaxy (by distance from the universal
/// core): a 3 × 3 × 3 lattice, 21 cells apart.
fn galaxy_place(k: usize) -> Place {
    let at = |n: usize| ratio((n % 3) as i64 * 21, 1);
    (at(k), at(k / 3), at(k / 9))
}

/// The stand-in place of system j of a galaxy at `centre`: an offset of at
/// most 3 + 3 + 3 cells, inside the reach.
fn system_place(centre: &Place, j: usize) -> Place {
    let off = |n: usize| ratio((n % 7) as i64 - 3, 1);
    (centre.0.clone() + off(j), centre.1.clone() + off(j / 7), centre.2.clone() + ratio(((j / 49) % 7) as i64 - 3, 1))
}

/// The galaxies by distance from the universal core (core indices).
fn by_distance(u: &Universe) -> Vec<usize> {
    let mut order: Vec<usize> = (0..u.cores.len()).collect();
    order.sort_by(|a, b| u.cores[*a].radius.cmp(&u.cores[*b].radius).then(a.cmp(b)));
    order
}

/// The stand-in place of the system at `path` ([core, system, ...]).
pub fn place_of(u: &Universe, path: &[u64]) -> Option<Place> {
    let order = by_distance(u);
    let i = u.cores.iter().position(|c| c.path.first() == path.first())?;
    let k = order.iter().position(|&o| o == i)?;
    let j = u.cores[i].satellites.iter().position(|s| s.path.get(1) == path.get(1))?;
    Some(system_place(&galaxy_place(k), j))
}

/// A world's composition, its grains by Z (the pulse's `Z###` keys).
pub fn composition_of(u: &Universe, path: &[u64]) -> Option<Vec<(u16, cfe::ratio::Ratio)>> {
    let core = u.cores.iter().find(|c| c.path.first() == path.first())?;
    let sys = core.satellites.iter().find(|s| s.path.get(1) == path.get(1))?;
    let orb = sys.satellites.iter().find(|o| o.path.get(2) == path.get(2))?;
    Some(orb.kept.iter().filter_map(|(k, a)| crate::chem::atom_z(k).and_then(|z| u16::try_from(z).ok()).map(|z| (z, a.clone()))).collect())
}

/// Each system's stand-in place and its galactic period (bells a turn about
/// its core, the motion law's `P = 1 + R / H`): what a line's orbit reads.
pub fn periods(u: &Universe) -> std::collections::BTreeMap<Place, cfe::ratio::Ratio> {
    let mut out = std::collections::BTreeMap::new();
    for (k, &i) in by_distance(u).iter().enumerate() {
        for (j, sys) in u.cores[i].satellites.iter().enumerate() {
            if sys.period > ratio(0, 1) {
                out.insert(system_place(&galaxy_place(k), j), sys.period.clone());
            }
        }
    }
    out
}

/// The ledger core charted for the k-th galaxy by distance (`chart`).
pub fn charted_core(book: &Ledger, k: usize) -> Option<String> {
    let p = galaxy_place(k);
    book.core_of(&(p.0, p.1, p.2))
}

/// Chart the pulse's cores as galaxies on the universe ledger, once (the
/// first charters the founding galaxy at the origin; each next stands past
/// every reach, so each is created).
pub fn chart(u: &Universe, book: &Ledger, stamp: u64) -> std::io::Result<usize> {
    if !book.galaxies().is_empty() {
        return Ok(0);
    }
    for k in 0..u.cores.len() {
        let p = galaxy_place(k);
        book.observe_galaxy("the-pulse", "the-pulse", (&p.0, &p.1, &p.2), stamp)?;
    }
    Ok(book.galaxies().len())
}

/// A world's era-1 siting in cfe's terms (the refactor onto era 1's physics,
/// 2026-10-15): the star class nearest its star's burn by luminosity, albedo
/// 0, and the orbit at which cfe's linear law gives the world's own heat,
/// `r = K · L / T` (K the Sol anchor, 279 K): exact, no root.
pub fn site_of(heat: &cfe::ratio::Ratio, burn: &cfe::ratio::Ratio) -> Option<(cfe::thermal::StarClass, cfe::ratio::Ratio, cfe::ratio::Ratio)> {
    use cfe::thermal::StarClass as C;
    if *heat <= ratio(0, 1) {
        return None;
    }
    let distance = |c: &C| { let d = c.luminosity() - burn.clone(); if d < ratio(0, 1) { -d } else { d } };
    let class = [C::O, C::B, C::A, C::F, C::G, C::K, C::M].into_iter().min_by(|a, b| distance(a).cmp(&distance(b)))?;
    // Six sexagesimal places of its own size (design law 8): the era-0
    // thermal machinery reads this orbit every tick, and the exact quotient
    // carried the heat's eighty-digit terms into every product; a fixed grain
    // floored the smallest orbits to nothing.
    let exact = cfe::thermal::thermal_k() * class.luminosity() / heat.clone();
    let floor = ratio(60, 1).pow(6);
    let mut scale = ratio(1, 1);
    while exact.clone() * scale.clone() < floor {
        scale *= ratio(60, 1);
    }
    let orbit = (exact * scale.clone()).floor() / scale;
    Some((class, ratio(0, 1), orbit))
}

/// Seat each spec on the unclaimed era-1 world, anywhere, where the physics
/// reads its life most VIABLE (`siting::viability_at`: its clade and
/// metabolism at that world's siting; ties to the galaxy holding the fewest
/// lines so far): a line lands where its life can live (2026-10-15: one
/// system per line, and then heat alone, left 67 then 78 of 157 collapsed at
/// founding). The spec takes that world's era-1 siting and its
/// system's place. A line with no world left keeps its place.
/// A world a line can stand on: its stand-in place, its star's class, its
/// albedo and orbit (`site_of`), and its composition.
#[derive(Debug, Clone)]
pub struct Site {
    pub path: Vec<u64>,
    pub position: Place,
    pub star: cfe::thermal::StarClass,
    pub albedo: Ratio,
    pub orbit: Ratio,
    pub composition: Option<Vec<(u16, Ratio)>>,
    /// Its land: the heads each medium it carries feeds wild (`life::wild`).
    pub wild: std::collections::BTreeMap<&'static str, Ratio>,
    /// Its figure, the geoid its peoples stand and move on (docs/88).
    pub figure: Option<crate::geoid::Figure>,
}

/// The figure of the world at `path`: its reach about the holding inside
/// its orbit (its star and the worlds within it), its own holding and core
/// share (`geoid::figure`).
pub fn figure_of(u: &Universe, path: &[u64], kappa: &Ratio) -> Option<crate::geoid::Figure> {
    let core = u.cores.iter().find(|c| c.path.first() == path.first())?;
    let sys = core.satellites.iter().find(|s| s.path.get(1) == path.get(1))?;
    let orb = sys.satellites.iter().find(|o| o.path.get(2) == path.get(2))?;
    let unit = crate::breath::unit_mass(u);
    if unit <= ratio(0, 1) {
        return None;
    }
    let held = |b: &crate::pulse::Body| crate::pulse::mass(&b.total()) / unit.clone();
    let inside = sys.satellites.iter().filter(|o| o.radius < orb.radius).fold(crate::pulse::mass(&sys.kept) / unit.clone(), |s, o| s + held(o));
    crate::geoid::figure(&orb.radius, &held(orb), &inside, kappa)
}

impl Site {
    /// The heads its land feeds wild for a people of `metabolism`: its
    /// medium's (none where the world does not carry it).
    pub fn wild_for(&self, metabolism: &str) -> Ratio {
        crate::life::medium_of(metabolism).and_then(|m| self.wild.get(m)).cloned().unwrap_or_else(|| ratio(0, 1))
    }
}

/// Every era-1 world a line can stand on, by path, with its land.
pub fn sites(u: &Universe, worlds: &std::collections::BTreeMap<Vec<u64>, crate::life::World>) -> std::collections::BTreeMap<Vec<u64>, Site> {
    let all: Vec<crate::life::World> = worlds.values().cloned().collect();
    let reference = crate::life::reference_pressure(&all);
    let home = crate::life::home_heat(u).unwrap_or_else(|| ratio(0, 1));
    worlds
        .values()
        .filter_map(|w| {
            let (star, albedo, orbit) = site_of(&w.heat, &w.burn)?;
            let position = place_of(u, &w.path)?;
            let compression = w.pressure.clone() / reference.clone();
            let wild = ["chemical", "geologic", "plasmic"].into_iter().map(|m| (m, crate::life::wild(w, &compression, m, &home))).collect();
            let figure = figure_of(u, &w.path, &w.core);
            Some((w.path.clone(), Site { path: w.path.clone(), position, star, albedo, orbit, composition: composition_of(u, &w.path), wild, figure }))
        })
        .collect()
}

/// Each system's reach from its core ([galaxy, star]).
pub fn reaches(u: &Universe) -> std::collections::BTreeMap<(u64, u64), Ratio> {
    u.cores
        .iter()
        .flat_map(|c| c.satellites.iter().filter_map(|s| Some(((*s.path.first()?, *s.path.get(1)?), s.radius.clone()))))
        .collect()
}

/// Stand a line on a site.
pub fn apply(spec: &mut PolitySpec, site: &Site) {
    spec.position = site.position.clone();
    spec.star = Some(site.star);
    spec.albedo = Some(site.albedo.clone());
    spec.orbit = Some(site.orbit.clone());
    spec.composition = site.composition.clone();
    spec.place = Some(site.path.clone());
    spec.wild = Some(site.wild_for(&spec.civilization.metabolism));
}

/// THE SEATING: a lineage's head takes a world its land feeds (docs/86), in
/// the system the fewest heads hold, the one feeding most of its people there, the galaxies taken in turn; a line descended from a seated line SETTLES the
/// free world nearest its parent's that its land feeds, else the nearest
/// free world (`places::nearest`: its parent's own system first, then its
/// parent's galaxy by reach), so a lineage spreads
/// through its own system before beyond it. A line whose parent is unseated
/// is seated as a head.
pub fn seat(u: &Universe, worlds: &std::collections::BTreeMap<Vec<u64>, crate::life::World>, specs: &mut [PolitySpec]) {
    let sites = sites(u, worlds);
    let reach = reaches(u);
    let mut claimed: std::collections::BTreeSet<Vec<u64>> = std::collections::BTreeSet::new();
    let mut per_galaxy: std::collections::BTreeMap<u64, usize> = std::collections::BTreeMap::new();
    let mut per_system: std::collections::BTreeMap<(u64, u64), usize> = std::collections::BTreeMap::new();
    let sited: Vec<&Site> = sites.values().collect();
    // A head's viability on a world is its land: the heads its own medium
    // feeds wild there (docs/86).
    let mut viability: std::collections::BTreeMap<String, Vec<cfe::ratio::Ratio>> = std::collections::BTreeMap::new();
    let parent_of = |spec: &PolitySpec| spec.civilization.lineage.first().cloned();
    let sigils: std::collections::BTreeSet<String> = specs.iter().map(|s| s.civilization.sigil.clone()).collect();
    let mut placed: std::collections::BTreeMap<String, Vec<u64>> = std::collections::BTreeMap::new();
    // The heads (no seated parent among the specs), by viability.
    for spec in specs.iter_mut() {
        if parent_of(spec).is_some_and(|p| sigils.contains(&p)) {
            continue;
        }
        let metabolism = spec.civilization.metabolism.clone();
        let v = viability.entry(metabolism.clone()).or_insert_with(|| sited.iter().map(|s| s.wild_for(&metabolism)).collect());
        let best = sited
            .iter()
            .enumerate()
            .filter(|(_, s)| !claimed.contains(&s.path))
            .max_by(|(ia, a), (ib, b)| {
                let zero = ratio(0, 1);
                let sa = per_system.get(&(a.path[0], a.path[1])).copied().unwrap_or(0);
                let sb = per_system.get(&(b.path[0], b.path[1])).copied().unwrap_or(0);
                let ga = per_galaxy.get(&a.path[0]).copied().unwrap_or(0);
                let gb = per_galaxy.get(&b.path[0]).copied().unwrap_or(0);
                (v[*ia] > zero).cmp(&(v[*ib] > zero)).then(sb.cmp(&sa)).then(v[*ia].cmp(&v[*ib])).then(gb.cmp(&ga)).then(b.path.cmp(&a.path))
            })
            .map(|(_, s)| *s);
        let Some(site) = best else { continue };
        apply(spec, site);
        claimed.insert(site.path.clone());
        placed.insert(spec.civilization.sigil.clone(), site.path.clone());
        *per_galaxy.entry(site.path[0]).or_insert(0) += 1;
        *per_system.entry((site.path[0], site.path[1])).or_insert(0) += 1;
    }
    // The descendants, generation by generation: each settles nearest its
    // parent's world.
    loop {
        let mut moved = false;
        for spec in specs.iter_mut() {
            if placed.contains_key(&spec.civilization.sigil) {
                continue;
            }
            let Some(parent) = parent_of(spec).and_then(|p| placed.get(&p).cloned()) else { continue };
            // A settler takes the nearest free world its land feeds, else the
            // nearest free world.
            let metabolism = spec.civilization.metabolism.clone();
            let found = {
                let (taken, metabolism) = (&claimed, &metabolism);
                let free = |livable: bool| sites.iter().filter(move |(p, s)| !taken.contains(*p) && (!livable || s.wild_for(metabolism) > ratio(0, 1))).map(|(p, _)| p);
                crate::places::nearest(&parent, free(true), &reach).or_else(|| crate::places::nearest(&parent, free(false), &reach))
            };
            let Some(path) = found else { continue };
            let Some(site) = sites.get(&path) else { continue };
            apply(spec, site);
            claimed.insert(path.clone());
            placed.insert(spec.civilization.sigil.clone(), path);
            moved = true;
        }
        if !moved {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Signed;

    #[test]
    fn galaxies_stand_beyond_each_others_reach_and_systems_within_their_own() {
        let l1 = |a: &Place, b: &Place| (a.0.clone() - b.0.clone()).abs() + (a.1.clone() - b.1.clone()).abs() + (a.2.clone() - b.2.clone()).abs();
        for a in 0..13 {
            for b in (a + 1)..13 {
                assert!(l1(&galaxy_place(a), &galaxy_place(b)) > ratio(16, 1));
            }
            for j in 0..343 {
                assert!(l1(&galaxy_place(a), &system_place(&galaxy_place(a), j)) <= ratio(9, 1));
            }
        }
    }
}
