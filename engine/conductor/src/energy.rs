//! The energy book, read only (docs/122 §5 step 1, docs/124 §2, docs/106
//! S1): what the universe holds as heat, potential and bond, summed over
//! every mote and body, so a bell's change in each can be read beside the
//! light it made. Nothing here writes the universe.
//!
//! Each form is booked on its own level (docs/124 §1–§2), in units of the
//! unit mass (`breath::unit_mass`):
//! - **heat** (phase level, motion out of step): a mote's `a·m`, its
//!   agitation times its holding; a star's heat read back from its ignition
//!   share (`formation::agitation_of`) plus its bell's carried heat, times
//!   its kept mass; a world's `heat·m`;
//! - **potential** (state level): `m·Φ`, `Φ = 1 − δ`, the hold a body stands
//!   in (docs/106 §1): galaxy motes and stars in their galaxy's frame
//!   (`breath::galaxy_frame`), a system's motes and worlds in its frame
//!   (`formation::system_frame`), the ocean's motes in the ocean's
//!   (`pulse::ocean_frame`); a core's own matter at its throat is not read
//!   here;
//! - **bond** (quantum level): the open frames still to close, `Σ a·unpaired`
//!   over all matter (`pulse::magnetism` over the bodies and the ocean, and
//!   the galaxies' and systems' motes one by one), so a fusion pair lowers it by its
//!   rung light `2u(z) − u(2z)` (`recycle::rung_light`).
//!
//! Motion (`Σ m·ℓ`) is read by the spin identities already (`breath`'s
//! `Event.identity`), each galaxy's motes in that galaxy's own units, so it
//! is not summed again here. The forms do not yet share one unit (heat and
//! potential are shares of holding, bond is frames), so the book reports
//! each form's change and makes no claim that their sum closes: the first
//! reading is the size of each one's change against the light made
//! (docs/122 §5 step 1; docs/124 §8 item 1).
use crate::flows::Mote;
use crate::pulse::{mass, Body, Universe};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// One reading of the universe's energy, by form, in units of the unit mass.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Book {
    /// Heat held by motes (`a·m`).
    pub heat_motes: Ratio,
    /// Heat held by stars (their ignition share's agitation plus their
    /// carried heat, times their kept mass).
    pub heat_stars: Ratio,
    /// Heat held by worlds and their moons (`heat·m`).
    pub heat_worlds: Ratio,
    /// Potential, `m·(1 − δ)`, of every mote and body read in its frame.
    pub potential: Ratio,
    /// Bond: the open frames still to close, `Σ a·unpaired`.
    pub bond: Ratio,
}

impl Book {
    /// All the heat the book holds.
    pub fn heat(&self) -> Ratio {
        self.heat_motes.clone() + self.heat_stars.clone() + self.heat_worlds.clone()
    }
}

/// Exact sums of many terms whose denominators differ (amounts carry wide
/// ones): added in a balanced tree, never one by one, so each addition
/// meets a partner of its own size. The sum is the same exact value; a
/// running sum's denominator grows with every term and makes each later
/// addition dearer, quadratic over the universe's motes.
#[derive(Default)]
struct Terms(Vec<Ratio>);

impl Terms {
    fn push(&mut self, x: Ratio) {
        if !x.is_zero() {
            self.0.push(x);
        }
    }

    fn sum(self) -> Ratio {
        let mut level = self.0;
        if level.is_empty() {
            return ratio(0, 1);
        }
        while level.len() > 1 {
            let mut next = Vec::with_capacity(level.len().div_ceil(2));
            let mut it = level.into_iter();
            while let Some(a) = it.next() {
                next.push(match it.next() {
                    Some(b) => a + b,
                    None => a,
                });
            }
            level = next;
        }
        level.pop().unwrap_or_else(|| ratio(0, 1))
    }
}

fn mote_mass(x: &Mote, unit: &Ratio) -> Ratio {
    x.holding() / unit.clone()
}

/// A mote's open frames (its part of the bond), in units of the unit mass.
/// `Universe::total` reads the bodies and the ocean, not the galaxies' and
/// systems' motes, so these are added mote by mote.
fn open_frames(x: &Mote, unit: &Ratio) -> Ratio {
    x.open_frames() / unit.clone()
}

fn world_heat(b: &Body, unit: &Ratio) -> Ratio {
    crate::formation::world_heat(b) / unit.clone()
}

/// Read the universe's energy book.
pub fn read(u: &Universe) -> Book {
    let unit = crate::breath::unit_mass(u);
    let mut book = Book {
        bond: crate::pulse::magnetism(&u.total()),
        ..Book::default()
    };
    if unit.is_zero() {
        return book;
    }
    book.bond /= unit.clone();
    let (mut heat_motes, mut heat_stars, mut heat_worlds) =
        (Terms::default(), Terms::default(), Terms::default());
    let (mut potential, mut bond) = (Terms::default(), Terms::default());
    let phi = |delta: Ratio| ratio(1, 1) - delta;
    // The ocean's motes, in the ocean's frame.
    let ocean = crate::pulse::ocean_frame(u);
    for x in &u.ocean {
        let m = mote_mass(x, &unit);
        heat_motes.push(x.agitation.clone() * m.clone());
        if let Some(enc) = &ocean {
            potential.push(m * phi(x.reach_in(enc).1));
        }
    }
    for (i, core) in u.cores.iter().enumerate() {
        let frame = crate::breath::galaxy_frame(u, i, &unit);
        for x in &core.flows {
            let m = mote_mass(x, &unit);
            bond.push(open_frames(x, &unit));
            heat_motes.push(x.agitation.clone() * m.clone());
            if let Some(enc) = &frame {
                potential.push(m * phi(x.reach_in(enc).1));
            }
        }
        for sys in &core.satellites {
            let kept = mass(&sys.kept) / unit.clone();
            heat_stars
                .push((crate::formation::agitation_of(&sys.reservoir) + sys.heat.clone()) * kept);
            if let Some(enc) = &frame {
                potential.push(mass(&sys.total()) / unit.clone() * phi(enc.reach(&sys.ell).1));
            }
            let inner = crate::formation::system_frame(sys, &unit);
            for x in &sys.flows {
                let m = mote_mass(x, &unit);
                bond.push(open_frames(x, &unit));
                heat_motes.push(x.agitation.clone() * m.clone());
                if let Some(enc) = &inner {
                    potential.push(m * phi(x.reach_in(enc).1));
                }
            }
            for w in &sys.satellites {
                heat_worlds.push(world_heat(w, &unit));
                if let Some(enc) = &inner {
                    potential.push(mass(&w.total()) / unit.clone() * phi(enc.reach(&w.ell).1));
                }
            }
        }
    }
    book.heat_motes = heat_motes.sum();
    book.heat_stars = heat_stars.sum();
    book.heat_worlds = heat_worlds.sum();
    book.potential = potential.sum();
    book.bond += bond.sum();
    book
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mote(key: &str, amount: Ratio, ell: Ratio, agitation: Ratio) -> Mote {
        Mote {
            parts: crate::flows::Mote::one(crate::chem::Kind::parse(key).expect("a kind"), amount),
            spins: Mote::spins_of(&ell, &[]),
            spins_orth: vec![ratio(0, 1); 3],
            orientation: crate::orientation::Orientation::from_phases(&crate::flows::rest(ratio(
                1, 2,
            ))),
            agitation,
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        }
    }

    fn universe() -> Universe {
        let table: crate::pulse::Manifest = [("W0001", 3000), ("W0004", 900)]
            .iter()
            .map(|(k, a)| (crate::chem::Kind::parse(k).expect("a kind"), ratio(*a, 1)))
            .collect();
        crate::pulse::deploy(&table).expect("a universe")
    }

    #[test]
    fn reading_leaves_the_universe_as_it_was() {
        let mut u = universe();
        u.cores[0]
            .flows
            .push(mote("W0001", ratio(5, 1), ratio(2, 1), ratio(1, 3)));
        let before = u.clone();
        let book = read(&u);
        assert!(book.heat() > ratio(0, 1));
        assert_eq!(u.cores, before.cores);
        assert_eq!(u.ocean, before.ocean);
    }

    #[test]
    fn a_mote_split_in_two_leaves_the_book_as_it_was() {
        // Every form is additive over holding: two halves at the same spin
        // and heat hold what the whole held, and enclose what it enclosed.
        let mut whole = universe();
        whole.cores[0]
            .flows
            .push(mote("W0001", ratio(6, 1), ratio(2, 1), ratio(1, 4)));
        let mut halves = universe();
        for _ in 0..2 {
            halves.cores[0]
                .flows
                .push(mote("W0001", ratio(3, 1), ratio(2, 1), ratio(1, 4)));
        }
        assert_eq!(read(&whole), read(&halves));
    }

    #[test]
    fn a_fusion_pair_lowers_the_bond_by_its_rung_light() {
        // H + H → He closes `2u(1) − u(2)` open frames: the bond the burn
        // transduces (docs/122 law 1), the same count `rung_light` books.
        let h = crate::chem::Kind::atom(1).expect("hydrogen");
        let he = crate::chem::Kind::atom(2).expect("its product");
        let mut before = universe();
        before.cores[0]
            .flows
            .push(mote(&h.to_string(), ratio(2, 1), ratio(2, 1), ratio(0, 1)));
        let mut after = universe();
        after.cores[0]
            .flows
            .push(mote(&he.to_string(), ratio(1, 1), ratio(2, 1), ratio(0, 1)));
        let light = crate::recycle::rung_light(1).expect("the rung runs");
        let unit = crate::breath::unit_mass(&before);
        assert_eq!(
            read(&before).bond - read(&after).bond,
            ratio(light, 1) / unit
        );
    }
}
