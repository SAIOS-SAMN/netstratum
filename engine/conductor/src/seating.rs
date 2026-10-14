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

/// Seat each spec on its own system (see the module note); specs beyond the
/// systems available keep their place.
pub fn seat(u: &Universe, specs: &mut [PolitySpec]) {
    let order = by_distance(u);
    let mut systems: Vec<Place> = Vec::new();
    let most = u.cores.iter().map(|c| c.satellites.len()).max().unwrap_or(0);
    for j in 0..most {
        for (k, &i) in order.iter().enumerate() {
            if j < u.cores[i].satellites.len() {
                systems.push(system_place(&galaxy_place(k), j));
            }
        }
    }
    for (spec, place) in specs.iter_mut().zip(systems) {
        spec.position = place;
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
