//! Era 1's peoples on era 1's worlds (docs/82, operator 2026-10-14: the
//! original civilizations and polities are brought over as era 1's first
//! peoples; life still arises by the ladder beside them).
//!
//! - **The galaxies are charted** on the universe ledger from the pulse's
//!   cores, so the conductor's galaxy and core readings find them. Each
//!   stands at a STAND-IN box position (a 3 × 3 × 3 lattice on the 64-cell
//!   box, 21 cells apart, beyond the 16-cell reach), nearest the universal
//!   core first: docs/78's flat-site table replaces it with the sphere (A1).
//! - **Each line is seated on its own star system:** the roster's order
//!   (the progenitor band first) takes the first systems of every galaxy in
//!   turn (the innermost, the oldest), then the second, so every galaxy
//!   receives peoples. A system's stand-in position sits within its
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
pub fn place_of(u: &Universe, path: &[u32]) -> Option<Place> {
    let order = by_distance(u);
    let i = u.cores.iter().position(|c| c.path.first() == path.first())?;
    let k = order.iter().position(|&o| o == i)?;
    let j = u.cores[i].satellites.iter().position(|s| s.path.get(1) == path.get(1))?;
    Some(system_place(&galaxy_place(k), j))
}

/// A world's composition, its grains by Z (the pulse's `Z###` keys).
pub fn composition_of(u: &Universe, path: &[u32]) -> Option<Vec<(u16, cfe::ratio::Ratio)>> {
    let core = u.cores.iter().find(|c| c.path.first() == path.first())?;
    let sys = core.satellites.iter().find(|s| s.path.get(1) == path.get(1))?;
    let orb = sys.satellites.iter().find(|o| o.path.get(2) == path.get(2))?;
    Some(orb.kept.iter().filter_map(|(k, a)| k.strip_prefix('Z').and_then(|z| z.parse::<u16>().ok()).map(|z| (z, a.clone()))).collect())
}

/// Each system's stand-in place and its galactic period (bells a turn about
/// its core, the turning law's `P = r / M(<r)`): what a line's orbit reads.
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
    let orbit = cfe::thermal::thermal_k() * class.luminosity() / heat.clone();
    Some((class, ratio(0, 1), orbit))
}

/// Seat each spec on the unclaimed era-1 world, anywhere, where the physics
/// reads its life most VIABLE (`siting::viability_at`: its clade and
/// metabolism at that world's siting; ties to the galaxy holding the fewest
/// lines so far): a line lands where its life can live (2026-10-15: one
/// system per line, and then heat alone, left 67 then 78 of 157 collapsed at
/// founding). The spec takes that world's era-1 siting and its
/// system's place. A line with no world left keeps its place.
pub fn seat(u: &Universe, worlds: &std::collections::BTreeMap<Vec<u32>, crate::life::World>, specs: &mut [PolitySpec]) {
    let order = by_distance(u);
    let rank: std::collections::BTreeMap<u32, usize> = order.iter().enumerate().filter_map(|(k, &i)| u.cores[i].path.first().map(|g| (*g, k))).collect();
    let mut claimed: std::collections::BTreeSet<Vec<u32>> = std::collections::BTreeSet::new();
    let mut per_galaxy: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
    let sited: Vec<(&crate::life::World, (cfe::thermal::StarClass, cfe::ratio::Ratio, cfe::ratio::Ratio))> =
        worlds.values().filter_map(|w| site_of(&w.heat, &w.burn).map(|s| (w, s))).collect();
    let mut viability: std::collections::BTreeMap<(String, String), Vec<cfe::ratio::Ratio>> = std::collections::BTreeMap::new();
    for spec in specs.iter_mut() {
        let key = (spec.civilization.clade.clone(), spec.civilization.metabolism.clone());
        let civ = spec.civilization.clone();
        let v = viability.entry(key).or_insert_with(|| sited.iter().map(|(_, (c, a, o))| crate::siting::viability_at(&civ, *c, a, o)).collect());
        let best = sited
            .iter()
            .enumerate()
            .filter(|(_, (w, _))| !claimed.contains(&w.path))
            .max_by(|(ia, (a, _)), (ib, (b, _))| {
                let ga = per_galaxy.get(&a.path[0]).copied().unwrap_or(0);
                let gb = per_galaxy.get(&b.path[0]).copied().unwrap_or(0);
                v[*ia].cmp(&v[*ib]).then(gb.cmp(&ga)).then(b.path.cmp(&a.path))
            })
            .map(|(_, (w, _))| *w);
        let Some(w) = best else { continue };
        let Some((class, albedo, orbit)) = site_of(&w.heat, &w.burn) else { continue };
        let Some(k) = rank.get(&w.path[0]).copied() else { continue };
        let Some(i) = order.get(k).copied() else { continue };
        let Some(j) = u.cores[i].satellites.iter().position(|s| s.path.get(1) == w.path.get(1)) else { continue };
        spec.position = system_place(&galaxy_place(k), j);
        spec.star = Some(class);
        spec.albedo = Some(albedo);
        spec.orbit = Some(orbit);
        spec.composition = composition_of(u, &w.path);
        claimed.insert(w.path.clone());
        *per_galaxy.entry(w.path[0]).or_insert(0) += 1;
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
