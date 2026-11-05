// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The coordinate (docs/120): every body stands at a TUniversal point inside
//! the universe, `origin / h : S : G : T : H : K`, and is addressed by it.
//!
//! - `origin` is the body's enclosure, named by its path from the universal
//!   core; two points of two origins are never compared.
//! - `h` is the bell.
//! - The four lanes are exact and signed, each the body's cumulative change
//!   of one kind, in the network's meanings: S spatial (boost and dilation,
//!   the reach), G gauge (the change of kind), T temporal (null flux: the
//!   beat and the passages), H holonomic (the turnings).
//! - `K` is the orbits closed: the whole turns of H.
//!
//! The coordinate is the state. Motion is the closed-form advance of the
//! lanes from one bell to another, one addition per lane, never a bell
//! stepped; an event's bell is solved from the lanes, never stepped to; a
//! lane is held exactly and no period is assumed (a lane's return is
//! sheared, never a closed repeat). Lanes are never summed into one another.
//!
//! Bodies of one enclosure are addressed through [`LaneIndex`]: one lane's
//! values in order, so a range on that lane is two bisections, never a test
//! of every body. A sector of the sexagesimal reading at a depth is an exact
//! interval on a lane (360 degrees, then five places of 60), so reading a
//! region at a depth is a range, and expansion and contraction are depth,
//! never iteration. The places are display only.
//!
//! Built in the form Mycelium's shared coordinate is to take (Xylarium's
//! `xyl-core::coordinate` the base), to be replaced by it, never kept beside
//! it.

use cfe::ratio::{ratio, Ratio};
use num_traits::{Signed, Zero};

/// The four lanes, in the network's meanings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lane {
    /// Spatial: boost and dilation, the reach.
    S,
    /// Gauge: the change of kind.
    G,
    /// Temporal: null flux, the beat and the passages.
    T,
    /// Holonomic: the turnings.
    H,
}

impl Lane {
    /// Every lane, in lane order.
    pub const ALL: [Lane; 4] = [Lane::S, Lane::G, Lane::T, Lane::H];

    fn at(self) -> usize {
        match self {
            Lane::S => 0,
            Lane::G => 1,
            Lane::T => 2,
            Lane::H => 3,
        }
    }
}

/// A body's enclosure: its path from the universal core.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Origin(pub Vec<u64>);

impl Origin {
    /// A child enclosure, named by its parent and its own name.
    pub fn branch(&self, name: u64) -> Origin {
        let mut path = self.0.clone();
        path.push(name);
        Origin(path)
    }
}

/// A point: a body's coordinate. Ordered as it is written (origin, bell,
/// each lane in lane order, the closed orbits): a canonical key, never a
/// measure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub origin: Origin,
    pub h: u64,
    pub lanes: [Ratio; 4],
    pub k: u64,
}

/// A body's generator read per lane: what each lane advances by a bell, by
/// motion type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rate(pub [Ratio; 4]);

impl Rate {
    /// No motion.
    pub fn still() -> Rate {
        Rate(std::array::from_fn(|_| ratio(0, 1)))
    }
}

/// The whole turns of a holonomic value, both senses counted.
fn closed(h: &Ratio) -> u64 {
    h.abs().floor().to_integer().try_into().unwrap_or(u64::MAX)
}

impl Point {
    /// A body standing at `lanes` at bell `h`, its closed orbits read from H.
    pub fn at(origin: Origin, h: u64, lanes: [Ratio; 4]) -> Point {
        let k = closed(&lanes[Lane::H.at()]);
        Point {
            origin,
            h,
            lanes,
            k,
        }
    }

    /// One lane's value.
    pub fn lane(&self, lane: Lane) -> &Ratio {
        &self.lanes[lane.at()]
    }

    /// The point advanced in closed form to bell `to` under `rate`: each
    /// lane plus its rate times the bells between, the closed orbits read
    /// from H. None where `to` stands before the point's own bell.
    pub fn advanced(&self, rate: &Rate, to: u64) -> Option<Point> {
        let bells = Ratio::from_integer(to.checked_sub(self.h)?.into());
        let lanes =
            std::array::from_fn(|i| self.lanes[i].clone() + rate.0[i].clone() * bells.clone());
        Some(Point::at(self.origin.clone(), to, lanes))
    }

    /// The bell (exact, possibly between whole bells) at which `lane`
    /// reaches `value` under `rate`, solved in first degree from the lane's
    /// closed form. None where the lane does not move toward it, or reached
    /// it before the point's own bell.
    pub fn when(&self, rate: &Rate, lane: Lane, value: &Ratio) -> Option<Ratio> {
        let r = &rate.0[lane.at()];
        if r.is_zero() {
            return None;
        }
        let bells = (value.clone() - self.lane(lane).clone()) / r.clone();
        if bells < ratio(0, 1) {
            return None;
        }
        Some(Ratio::from_integer(self.h.into()) + bells)
    }

    /// The gap from `self` to `other` on one lane, at one origin and one
    /// bell. None across origins (never compared) or bells (advance one to
    /// the other's bell first).
    pub fn gap(&self, other: &Point, lane: Lane) -> Option<Ratio> {
        if self.origin != other.origin || self.h != other.h {
            return None;
        }
        Some(other.lane(lane).clone() - self.lane(lane).clone())
    }

    /// Two bodies joined at one coordinate (a merger, Mycelium TU-6
    /// SUPERPOSE): each lane the holding-weighted mean of the two, at one
    /// origin and bell. None across origins or bells, or with no holding.
    pub fn superposed(&self, w_self: &Ratio, other: &Point, w_other: &Ratio) -> Option<Point> {
        if self.origin != other.origin || self.h != other.h {
            return None;
        }
        let whole = w_self.clone() + w_other.clone();
        if whole.is_zero() {
            return None;
        }
        let lanes = std::array::from_fn(|i| {
            (self.lanes[i].clone() * w_self.clone() + other.lanes[i].clone() * w_other.clone())
                / whole.clone()
        });
        Some(Point::at(self.origin.clone(), self.h, lanes))
    }

    /// A body carried into another enclosure across a throat (a re-origin):
    /// written at `into`, its lanes kept and its passage recorded on T by
    /// `flux`. No reach moves (LAW-PASSAGE): S is unchanged.
    pub fn re_origined(&self, into: Origin, flux: &Ratio) -> Point {
        let mut lanes = self.lanes.clone();
        lanes[Lane::T.at()] += flux.clone();
        Point::at(into, self.h, lanes)
    }
}

/// Degrees in a turn, the reading's first place.
pub const DEGREES: i64 = 360;
/// The reading's places: degrees, then five places of 60.
pub const PLACES: usize = 6;

/// The width of a sector at `depth` places, in turns: a whole turn at 0,
/// then a degree, then each finer place a sixtieth.
pub fn sector_width(depth: usize) -> Ratio {
    let mut w = ratio(1, 1);
    for p in 0..depth.min(PLACES) {
        w /= ratio(if p == 0 { DEGREES } else { 60 }, 1);
    }
    w
}

/// The sector holding `value` at `depth`: its exact interval
/// `[lo, lo + width)` on the lane, decided by exact comparison, never
/// rounded. A coarser sector holds every finer one it contains.
pub fn sector(value: &Ratio, depth: usize) -> (Ratio, Ratio) {
    let width = sector_width(depth);
    let lo = (value.clone() / width.clone()).floor() * width.clone();
    (lo.clone(), lo + width)
}

/// A reading's places, for display only: the whole turns and, of the share
/// of the turn, degrees then five places of 60. Nothing compares or stores
/// these.
pub fn places(value: &Ratio) -> (Ratio, [u16; PLACES]) {
    let rings = value.floor();
    let mut rest = value.clone() - rings.clone();
    let mut out = [0u16; PLACES];
    for (p, slot) in out.iter_mut().enumerate() {
        rest *= ratio(if p == 0 { DEGREES } else { 60 }, 1);
        let whole = rest.floor();
        *slot = whole.to_integer().try_into().unwrap_or(0);
        rest -= whole;
    }
    (rings, out)
}

/// One lane's values of an enclosure's bodies, in order: the index a law
/// addresses its bodies by on that lane. Built from points at one origin
/// and one bell.
#[derive(Debug, Clone, Default)]
pub struct LaneIndex {
    entries: Vec<(Ratio, usize)>,
}

/// What a range read: the bodies in it, and the comparisons it took (the
/// work counter, never a clock).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub bodies: Vec<usize>,
    pub comparisons: usize,
}

impl LaneIndex {
    /// The index of `points` on `lane`, each by its place in `points`. None
    /// where the points stand at more than one origin or bell.
    pub fn of(points: &[Point], lane: Lane) -> Option<LaneIndex> {
        if let Some(first) = points.first() {
            if points
                .iter()
                .any(|p| p.origin != first.origin || p.h != first.h)
            {
                return None;
            }
        }
        let mut entries: Vec<(Ratio, usize)> = points
            .iter()
            .enumerate()
            .map(|(i, p)| (p.lane(lane).clone(), i))
            .collect();
        entries.sort();
        Some(LaneIndex { entries })
    }

    /// The bodies whose lane value stands in `[lo, hi)`: two bisections.
    pub fn range(&self, lo: &Ratio, hi: &Ratio) -> Range {
        let mut comparisons = 0usize;
        let mut first = |bound: &Ratio| {
            let (mut a, mut b) = (0usize, self.entries.len());
            while a < b {
                let mid = a + (b - a) / 2;
                comparisons += 1;
                if self.entries[mid].0 < *bound {
                    a = mid + 1;
                } else {
                    b = mid;
                }
            }
            a
        };
        let (from, to) = (first(lo), first(hi));
        Range {
            bodies: self.entries[from..to.max(from)]
                .iter()
                .map(|(_, i)| *i)
                .collect(),
            comparisons,
        }
    }

    /// The bodies within `width` of `value` on the lane, either side.
    pub fn near(&self, value: &Ratio, width: &Ratio) -> Range {
        self.range(
            &(value.clone() - width.clone()),
            &(value.clone() + width.clone()),
        )
    }

    /// The bodies in the sector holding `value` at `depth`: a region of the
    /// reading, read as a range.
    pub fn in_sector(&self, value: &Ratio, depth: usize) -> Range {
        let (lo, hi) = sector(value, depth);
        self.range(&lo, &hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lanes(s: Ratio, h: Ratio) -> [Ratio; 4] {
        [s, ratio(0, 1), ratio(0, 1), h]
    }

    #[test]
    fn advance_is_closed_form_and_composes() {
        let p = Point::at(Origin(vec![0]), 10, lanes(ratio(1, 3), ratio(1, 4)));
        let rate = Rate([ratio(1, 7), ratio(0, 1), ratio(1, 2), ratio(1, 5)]);
        // One advance of 30 bells equals two of 12 and 18.
        let once = p.advanced(&rate, 40).expect("forward");
        let twice = p
            .advanced(&rate, 22)
            .and_then(|q| q.advanced(&rate, 40))
            .expect("forward");
        assert_eq!(once, twice);
        assert_eq!(once.lane(Lane::S), &(ratio(1, 3) + ratio(30, 7)));
        // H at 1/4 + 6 is six whole turns closed.
        assert_eq!(once.k, 6);
        assert!(p.advanced(&rate, 9).is_none());
    }

    #[test]
    fn an_events_bell_is_solved_from_the_lane() {
        let p = Point::at(Origin(vec![0]), 4, lanes(ratio(0, 1), ratio(0, 1)));
        let rate = Rate([ratio(2, 3), ratio(0, 1), ratio(0, 1), ratio(0, 1)]);
        // S reaches 1 after 3/2 bells: at bell 11/2, exactly.
        assert_eq!(p.when(&rate, Lane::S, &ratio(1, 1)), Some(ratio(11, 2)));
        let reached = p.advanced(&rate, 6).expect("forward");
        assert!(reached.lane(Lane::S) >= &ratio(1, 1));
        // A still lane, or a value behind, has no bell.
        assert_eq!(p.when(&rate, Lane::G, &ratio(1, 1)), None);
        assert_eq!(p.when(&rate, Lane::S, &ratio(-1, 1)), None);
    }

    #[test]
    fn two_origins_are_never_compared() {
        let a = Point::at(Origin(vec![0]), 1, lanes(ratio(1, 2), ratio(0, 1)));
        let b = Point::at(Origin(vec![0, 3]), 1, lanes(ratio(1, 3), ratio(0, 1)));
        assert_eq!(a.gap(&b, Lane::S), None);
        assert_eq!(a.superposed(&ratio(1, 1), &b, &ratio(1, 1)), None);
        assert_eq!(Origin(vec![0]).branch(3), b.origin);
        // Re-origined into b's enclosure, a keeps its reach and records its
        // passage on T.
        let carried = a.re_origined(b.origin.clone(), &ratio(1, 1));
        assert_eq!(carried.lane(Lane::S), a.lane(Lane::S));
        assert_eq!(carried.lane(Lane::T), &ratio(1, 1));
        assert_eq!(carried.gap(&b, Lane::S), Some(ratio(-1, 6)));
    }

    #[test]
    fn a_merger_is_the_holding_weighted_coordinate() {
        let o = Origin(vec![0]);
        let a = Point::at(o.clone(), 2, lanes(ratio(1, 1), ratio(2, 1)));
        let b = Point::at(o, 2, lanes(ratio(3, 1), ratio(0, 1)));
        let m = a
            .superposed(&ratio(1, 1), &b, &ratio(3, 1))
            .expect("one origin");
        assert_eq!(m.lane(Lane::S), &ratio(5, 2));
        assert_eq!(m.lane(Lane::H), &ratio(1, 2));
        assert_eq!(a.superposed(&ratio(1, 1), &a, &ratio(1, 1)), Some(a));
    }

    #[test]
    fn a_sector_is_an_exact_interval_and_nests() {
        let v = ratio(1, 3);
        // A third of a turn is 120 degrees exactly, every finer place 0.
        assert_eq!(places(&v), (ratio(0, 1), [120, 0, 0, 0, 0, 0]));
        // A value just below a mark lies in the sector before it.
        let below = v.clone() - Ratio::new(1.into(), num_bigint::BigInt::from(10).pow(15));
        assert_eq!(places(&below).1, [119, 59, 59, 59, 59, 59]);
        for depth in 0..PLACES {
            let (lo, hi) = sector(&v, depth);
            assert!(lo <= v && v < hi);
            let (flo, fhi) = sector(&v, depth + 1);
            assert!(lo <= flo && fhi <= hi, "a finer sector lies in the coarser");
        }
    }

    #[test]
    fn a_range_reads_the_same_bodies_as_testing_every_one_in_log_n() {
        // n bodies on S at one origin and bell; a range read by bisection
        // returns exactly the bodies a test of every body would, and its
        // comparisons grow by a step or two when n grows fourfold.
        let make = |n: i64| -> Vec<Point> {
            (0..n)
                .map(|i| {
                    Point::at(
                        Origin(vec![0]),
                        0,
                        lanes(ratio((i * 7919) % n, n), ratio(0, 1)),
                    )
                })
                .collect()
        };
        let (lo, hi) = (ratio(1, 5), ratio(2, 5));
        let mut work = Vec::new();
        for n in [256i64, 1024] {
            let points = make(n);
            let index = LaneIndex::of(&points, Lane::S).expect("one origin and bell");
            let read = index.range(&lo, &hi);
            let mut every: Vec<usize> = (0..points.len())
                .filter(|&i| points[i].lane(Lane::S) >= &lo && points[i].lane(Lane::S) < &hi)
                .collect();
            let mut got = read.bodies.clone();
            got.sort_unstable();
            every.sort_unstable();
            assert_eq!(got, every);
            work.push(read.comparisons);
        }
        assert!(
            work[1] <= work[0] + 4,
            "N to 4N adds a bisection step per bound"
        );
        // A sector at a depth is a range: the bodies within one degree.
        let points = make(1024);
        let index = LaneIndex::of(&points, Lane::S).expect("one origin and bell");
        let sector_read = index.in_sector(&ratio(1, 3), 1);
        assert!(sector_read
            .bodies
            .iter()
            .all(|&i| sector(points[i].lane(Lane::S), 1) == sector(&ratio(1, 3), 1)));
        // Points of two origins make no index.
        let mixed = vec![
            points[0].clone(),
            Point::at(Origin(vec![1]), 0, lanes(ratio(0, 1), ratio(0, 1))),
        ];
        assert!(LaneIndex::of(&mixed, Lane::S).is_none());
    }
}
