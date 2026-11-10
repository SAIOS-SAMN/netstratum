//! The planes' horizons, read (docs/117 §8–§9): where each mote's plane
//! directions stand against their horizons, before the passage of docs/117
//! §9.3 is built. A reading only: nothing here writes the universe.
//!
//! A mote's plane k carries the part `R·t_k` of its reach (its tilt's share
//! of it) against the hold `H`; the plane is inside its horizon where
//! `|R·t_k| < H`, its direction past null. Read two ways: against the whole
//! enclosure's hold at the mote's spin (as the breath reads every share now),
//! and against the plane's own enclosure, the bodies sorted by their spin in
//! that plane (docs/117 §9.6). The built luminal bound reads the whole mote,
//! `R < H`; where a plane's side differs from the whole's, the passage would
//! move the mote where the built bound does not.
use crate::flows::{tilt_of, weight, Enclosed};
use crate::pulse::{mass, Body, Universe};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// One echo's reading over every galaxy's motes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Horizons {
    /// The motes read.
    pub motes: usize,
    /// Motes inside the whole's horizon, `R < H` (the built luminal bound).
    pub whole_inside: usize,
    /// Motes by which of their planes stand inside, bit k set for plane k,
    /// read against the whole enclosure's hold (`by_whole`) and against each
    /// plane's own enclosure (`by_own`).
    pub by_whole: [usize; 8],
    pub by_own: [usize; 8],
    /// Motes whose whole stands outside while a plane that turns (its tilt
    /// share above zero) stands inside, and the reverse, read against the
    /// whole's hold: where the passage per plane and the built bound part. A
    /// plane with no spin stands inside for good and never crosses.
    pub plane_in_whole_out: usize,
    pub plane_out_whole_in: usize,
    /// Motes where two planes' parts are exactly equal (a tie at any
    /// crossing they make together, docs/117 §9.5).
    pub ties: usize,
    /// Motes whose planes' sides differ between the two holds.
    pub holds_disagree: usize,
}

fn enclosure(core: &Body, unit: &Ratio, spin: &dyn Fn(&Ratio, &[Ratio]) -> Ratio) -> Enclosed {
    Enclosed::new(
        ratio(1, 1),
        core.flows
            .iter()
            .map(|x| {
                (
                    spin(&x.ell_o(), &x.tilt()),
                    x.amount.clone() * weight(&x.key) / unit.clone(),
                )
            })
            .chain(
                core.satellites
                    .iter()
                    .map(|b| (spin(&b.ell, &b.tilt), mass(&b.total()) / unit.clone())),
            )
            .collect(),
    )
}

/// Which of the planes stand inside: bit k set where plane k's part stands
/// below its hold, `|R·t_k| < H` (the closure gives each plane's part and
/// hold). A plane carrying no spin stands inside: its direction is the hold
/// alone.
fn inside(part_and_hold: &dyn Fn(usize) -> (Ratio, Ratio)) -> u8 {
    let mut bits = 0u8;
    for k in 0..crate::flows::SPACE.len() {
        let (part, hold) = part_and_hold(k);
        if part < hold {
            bits |= 1 << k;
        }
    }
    bits
}

/// Read every galaxy's motes. Each galaxy is read in its own enclosure, as
/// the breath builds it: the base 1 and every mote and body by its spin, in
/// the galaxy's mass (the universe's for the home).
pub fn read(u: &Universe) -> Horizons {
    let mut out = Horizons::default();
    let whole_u = mass(&u.total());
    for core in &u.cores {
        if core.flows.is_empty() {
            continue;
        }
        let unit = if crate::pulse::is_home(core) {
            whole_u.clone()
        } else {
            let mut with = core.total();
            for x in &core.flows {
                *with.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
            }
            mass(&with)
        };
        if unit.is_zero() {
            continue;
        }
        let whole = enclosure(core, &unit, &|ell, _| ell.clone());
        let planes: Vec<Enclosed> = (0..crate::flows::SPACE.len())
            .map(|k| {
                enclosure(core, &unit, &|ell, tilt| {
                    let t = tilt_of(tilt);
                    let l = if *ell < ratio(0, 1) {
                        -ell.clone()
                    } else {
                        ell.clone()
                    };
                    l * t[k].clone()
                })
            })
            .collect();
        for x in &core.flows {
            let l = if x.ell_o() < ratio(0, 1) {
                -x.ell_o()
            } else {
                x.ell_o()
            };
            let (r, _) = whole.reach(&l);
            let h = whole.hold(&l);
            let t = tilt_of(&x.tilt());
            out.motes += 1;
            if r < h {
                out.whole_inside += 1;
            }
            let by_whole = inside(&|k| (r.clone() * t[k].clone(), h.clone()));
            let by_own = inside(&|k| {
                let lk = l.clone() * t[k].clone();
                (planes[k].reach(&lk).0, planes[k].hold(&lk))
            });
            out.by_whole[by_whole as usize] += 1;
            out.by_own[by_own as usize] += 1;
            let turning = (0..t.len())
                .filter(|&k| t[k] > ratio(0, 1))
                .fold(0u8, |m, k| m | (1 << k));
            if r >= h && by_whole & turning != 0 {
                out.plane_in_whole_out += 1;
            }
            if r < h && by_whole & turning != turning {
                out.plane_out_whole_in += 1;
            }
            if t[0] == t[1] || t[1] == t[2] || t[0] == t[2] {
                out.ties += 1;
            }
            if by_whole != by_own {
                out.holds_disagree += 1;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flows::Mote;

    fn mote(ell: Ratio, tilt: Vec<Ratio>) -> Mote {
        Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ell, &tilt),
            ell_orth: ratio(0, 1),
            orientation: crate::orientation::Orientation::from_phases(&crate::flows::rest(ratio(1, 2))),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        }
    }

    #[test]
    fn a_planes_horizon_is_read_against_the_whole_and_a_disc_agrees_with_it() {
        // One enclosure, base 1, nothing inside a lone mote: its hold is 1,
        // its reach its spin. An isotropic mote at reach 2 stands outside the
        // whole's horizon (2 > 1) while every plane's part, 2/3, stands
        // inside: the passage per plane and the built bound part there.
        let whole = Enclosed::new(ratio(1, 1), Vec::new());
        let (r, _) = whole.reach(&ratio(2, 1));
        assert_eq!(r, ratio(2, 1));
        let iso = tilt_of(&[]);
        assert_eq!(
            inside(&|k| (r.clone() * iso[k].clone(), ratio(1, 1))),
            0b111
        );
        // A disc-settled mote at the same reach: its one plane carries the
        // whole reach, outside, as the whole does; the planes with no spin
        // stand inside.
        let disc = [ratio(1, 1), ratio(0, 1), ratio(0, 1)];
        assert_eq!(
            inside(&|k| (r.clone() * disc[k].clone(), ratio(1, 1))),
            0b110
        );
    }

    #[test]
    fn reading_leaves_the_universe_as_it_was() {
        let table: crate::pulse::Manifest = [("W0001", 3000), ("W0004", 900)]
            .iter()
            .map(|(k, a)| (crate::chem::Kind::parse(k).expect("a kind"), ratio(*a, 1)))
            .collect();
        let mut u = crate::pulse::deploy(&table).expect("a universe");
        u.cores[0].flows.push(mote(ratio(2, 1), Vec::new()));
        let before = u.clone();
        let h = read(&u);
        assert_eq!(h.motes, 1);
        assert_eq!(u.cores, before.cores);
    }
}
