// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The coordinate (docs/120; Mycelium LAW-COORDINATE v1, proposal 78.173):
//! every body stands at a TUniversal point inside the universe,
//! `origin / h : S : G : T : H : K`, and is addressed by it.
//!
//! - The `origin` is the body's enclosure: the universe's root (genesis
//!   hash and epoch) and the enclosures it stands in, outermost first, each
//!   written out as the coordinate its branch was cut at. Two points of two
//!   origins are never compared.
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
//! lane is held exactly and no period is assumed. Lanes are never summed
//! into one another.
//!
//! Bodies are held in the canonical order ([`Bodies`]), so the neighbours
//! on one lane are two bisections, never a test of every body. A sector of
//! the sexagesimal reading is an exact interval `[m·60⁻ᵈ, (m + 1)·60⁻ᵈ)`
//! on a lane: pure base 60, degrees being a caller's unit. Reading a region
//! at a depth is a range, and expansion and contraction are depth, never
//! iteration. The places are display only.
//!
//! The interim form of Mycelium's `forms::coordinate`, its names and
//! writing the same, so the switch is an import. Replaced by it when it is
//! released at a tag, never kept beside it. The motion (advance, the
//! event's bell, merger with its offsets and fission, re-origin) is stated
//! to Mycelium as fixtures 7–10 and moves with it.

use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;

/// The reading's base.
pub const BASE: u32 = 60;

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

    /// The lane's place in the lanes.
    pub fn place(self) -> usize {
        match self {
            Lane::S => 0,
            Lane::G => 1,
            Lane::T => 2,
            Lane::H => 3,
        }
    }
}

/// A body's place within its origin: the bell, the four signed exact lanes
/// in lane order, the orbits closed. Ordered as written.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Local {
    pub h: u64,
    pub lanes: [Ratio; 4],
    pub k: u64,
}

/// The whole turns of a holonomic value, both senses counted.
fn closed(h: &Ratio) -> u64 {
    h.abs().floor().to_integer().try_into().unwrap_or(u64::MAX)
}

/// A lane's one writing: `n/d` in lowercase hex, lowest terms, a negative
/// numerator signed `-`.
fn lane_text(q: &Ratio) -> String {
    format!(
        "{}/{}",
        q.numer().to_str_radix(16),
        q.denom().to_str_radix(16)
    )
}

impl Local {
    /// A body standing at `lanes` at bell `h`, its closed orbits read from H.
    pub fn at(h: u64, lanes: [Ratio; 4]) -> Local {
        let k = closed(&lanes[Lane::H.place()]);
        Local { h, lanes, k }
    }

    /// One lane's value.
    pub fn lane(&self, lane: Lane) -> &Ratio {
        &self.lanes[lane.place()]
    }

    /// The writing `h:S:G:T:H:K`, `h` and `K` in 16 lowercase hex digits.
    pub fn written(&self) -> String {
        let mut fields = vec![format!("{:016x}", self.h)];
        fields.extend(self.lanes.iter().map(lane_text));
        fields.push(format!("{:016x}", self.k));
        fields.join(":")
    }

    /// The place advanced in closed form to bell `to` under `rate`: each
    /// lane plus its rate times the bells between, the closed orbits read
    /// from H. None where `to` stands before the place's own bell.
    pub fn advanced(&self, rate: &Rate, to: u64) -> Option<Local> {
        let bells = Ratio::from_integer(to.checked_sub(self.h)?.into());
        let lanes =
            std::array::from_fn(|i| self.lanes[i].clone() + rate.0[i].clone() * bells.clone());
        Some(Local::at(to, lanes))
    }

    /// The bell (exact, possibly between whole bells) at which `lane`
    /// reaches `value` under `rate`, solved in first degree from the lane's
    /// closed form. None where the lane does not move toward it, or reached
    /// it before the place's own bell.
    pub fn when(&self, rate: &Rate, lane: Lane, value: &Ratio) -> Option<Ratio> {
        let r = &rate.0[lane.place()];
        if r.is_zero() {
            return None;
        }
        let bells = (value.clone() - self.lane(lane).clone()) / r.clone();
        if bells.is_negative() {
            return None;
        }
        Some(Ratio::from_integer(self.h.into()) + bells)
    }
}

/// A body's enclosure: the universe's root and the enclosures it stands
/// in, outermost first, each the coordinate its branch was cut at.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Origin {
    pub genesis: [u8; 32],
    pub epoch: u32,
    pub enclosures: Vec<Local>,
}

impl Origin {
    /// The universe's root.
    pub fn root(genesis: [u8; 32], epoch: u32) -> Origin {
        Origin {
            genesis,
            epoch,
            enclosures: Vec::new(),
        }
    }

    /// A child enclosure, cut at `at` within this one (TU-6 BRANCH).
    pub fn branch(&self, at: Local) -> Origin {
        let mut child = self.clone();
        child.enclosures.push(at);
        child
    }

    /// The writing: genesis in 64 lowercase hex digits, `.`, the epoch in
    /// 8, then `/` and each enclosure's writing, outermost first.
    pub fn written(&self) -> String {
        let mut out: String = self.genesis.iter().map(|b| format!("{b:02x}")).collect();
        out.push_str(&format!(".{:08x}", self.epoch));
        for e in &self.enclosures {
            out.push('/');
            out.push_str(&e.written());
        }
        out
    }
}

/// A body's coordinate: its origin and its place within it. Ordered as the
/// canonical key: origin, bell, each lane in lane order, the closed orbits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Coordinate {
    pub origin: Origin,
    pub local: Local,
}

/// Why a reading between coordinates is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordinateRefusal {
    /// Two bodies of different origins: never compared.
    OriginsDiffer {
        held: Box<Origin>,
        asked: Box<Origin>,
    },
}

/// Where a value stands against a sector, decided exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Below,
    Within,
    Above,
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

/// Two bodies joined (TU-6 SUPERPOSE): the merged coordinate and each
/// body's offset from it per lane, kept so nothing is discarded. The
/// offsets' holding-weighted sum is zero on every lane, and fission is the
/// inverse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Merger {
    pub point: Coordinate,
    pub weights: [Ratio; 2],
    pub offsets: [[Ratio; 4]; 2],
}

impl Merger {
    /// The two bodies the merger was made of, exactly.
    pub fn fission(&self) -> [Coordinate; 2] {
        std::array::from_fn(|b| {
            let lanes = std::array::from_fn(|i| {
                self.point.local.lanes[i].clone() + self.offsets[b][i].clone()
            });
            Coordinate {
                origin: self.point.origin.clone(),
                local: Local::at(self.point.local.h, lanes),
            }
        })
    }
}

impl Coordinate {
    /// The full writing `origin/h:S:G:T:H:K`.
    pub fn written(&self) -> String {
        format!("{}/{}", self.origin.written(), self.local.written())
    }

    /// One lane's value.
    pub fn lane(&self, lane: Lane) -> &Ratio {
        self.local.lane(lane)
    }

    /// The sexagesimal reading of `lane` at `depth`: the sector holding it.
    pub fn reading(&self, lane: Lane, depth: u32) -> Sector {
        Sector::holding(self.lane(lane), depth)
    }

    fn same_origin(&self, other: &Coordinate) -> Result<(), CoordinateRefusal> {
        match self.origin == other.origin {
            true => Ok(()),
            false => Err(CoordinateRefusal::OriginsDiffer {
                held: Box::new(self.origin.clone()),
                asked: Box::new(other.origin.clone()),
            }),
        }
    }

    /// Where this body stands on `lane` against the sector at `depth` that
    /// holds `other`'s value there, on the sector's exact bounds.
    pub fn within(
        &self,
        other: &Coordinate,
        lane: Lane,
        depth: u32,
    ) -> Result<Side, CoordinateRefusal> {
        self.same_origin(other)?;
        Ok(other.reading(lane, depth).side(self.lane(lane)))
    }

    /// The coordinate advanced in closed form to bell `to`, at its origin.
    pub fn advanced(&self, rate: &Rate, to: u64) -> Option<Coordinate> {
        Some(Coordinate {
            origin: self.origin.clone(),
            local: self.local.advanced(rate, to)?,
        })
    }

    /// Two bodies joined at one coordinate: each lane the holding-weighted
    /// mean, each body's offset from it kept. Refused across origins; None
    /// across bells (advance one to the other's first) or with no holding.
    pub fn merged(
        &self,
        w_self: &Ratio,
        other: &Coordinate,
        w_other: &Ratio,
    ) -> Result<Option<Merger>, CoordinateRefusal> {
        self.same_origin(other)?;
        let whole = w_self.clone() + w_other.clone();
        if self.local.h != other.local.h || whole.is_zero() {
            return Ok(None);
        }
        let lanes: [Ratio; 4] = std::array::from_fn(|i| {
            (self.local.lanes[i].clone() * w_self.clone()
                + other.local.lanes[i].clone() * w_other.clone())
                / whole.clone()
        });
        let offsets = [self, other]
            .map(|b| std::array::from_fn(|i| b.local.lanes[i].clone() - lanes[i].clone()));
        Ok(Some(Merger {
            point: Coordinate {
                origin: self.origin.clone(),
                local: Local::at(self.local.h, lanes),
            },
            weights: [w_self.clone(), w_other.clone()],
            offsets,
        }))
    }

    /// A body carried into another enclosure across a throat (a re-origin):
    /// written at `into`, its lanes kept and its passage recorded on T by
    /// `flux`. No reach moves (LAW-PASSAGE): S is unchanged.
    pub fn re_origined(&self, into: Origin, flux: &Ratio) -> Coordinate {
        let mut lanes = self.local.lanes.clone();
        lanes[Lane::T.place()] += flux.clone();
        Coordinate {
            origin: into,
            local: Local::at(self.local.h, lanes),
        }
    }
}

/// One sector of the sexagesimal reading: at `depth`, the exact interval
/// `[index·60⁻ᵈᵉᵖᵗʰ, (index + 1)·60⁻ᵈᵉᵖᵗʰ)`. The value read is never
/// replaced by it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sector {
    pub depth: u32,
    pub index: BigInt,
}

/// The display of a sector: its lower bound as a signed whole part and
/// `depth` places in `0..60`, `lo = whole + Σ placeᵢ·60⁻ⁱ`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Places {
    pub whole: BigInt,
    pub places: Vec<BigInt>,
}

/// `60ᵈᵉᵖᵗʰ`, exactly.
fn scale(depth: u32) -> BigInt {
    BigInt::from(BASE).pow(depth)
}

impl Sector {
    /// The sector at `depth` holding `value`: the one integer `m` with
    /// `m·60⁻ᵈ ≤ value < (m + 1)·60⁻ᵈ`, fixed by exact comparison.
    pub fn holding(value: &Ratio, depth: u32) -> Sector {
        let index = (value * Ratio::from_integer(scale(depth)))
            .floor()
            .to_integer();
        Sector { depth, index }
    }

    /// The sector's width, `60⁻ᵈᵉᵖᵗʰ`.
    pub fn width(&self) -> Ratio {
        Ratio::new(BigInt::one(), scale(self.depth))
    }

    /// The lower bound, held by the sector.
    pub fn lo(&self) -> Ratio {
        Ratio::new(self.index.clone(), scale(self.depth))
    }

    /// The upper bound, not held by the sector.
    pub fn hi(&self) -> Ratio {
        Ratio::new(&self.index + BigInt::one(), scale(self.depth))
    }

    /// The sector one depth coarser that holds this one; None at depth 0.
    pub fn parent(&self) -> Option<Sector> {
        Some(Sector::holding(&self.lo(), self.depth.checked_sub(1)?))
    }

    /// Where `value` stands against `[lo, hi)`, by exact comparison.
    pub fn side(&self, value: &Ratio) -> Side {
        match (value.cmp(&self.lo()), value.cmp(&self.hi())) {
            (Ordering::Less, _) => Side::Below,
            (_, Ordering::Less) => Side::Within,
            _ => Side::Above,
        }
    }

    /// The places, for display: the whole part (the integer at or below the
    /// lower bound) and `depth` places in `0..60`, coarsest first.
    pub fn places(&self) -> Places {
        let s = scale(self.depth);
        let whole = Ratio::new(self.index.clone(), s.clone())
            .floor()
            .to_integer();
        let mut rest = &self.index - &whole * &s;
        let mut places = Vec::new();
        for i in (0..self.depth).rev() {
            let unit = scale(i);
            places.push(&rest / &unit);
            rest %= unit;
        }
        Places { whole, places }
    }

    /// The writing, for display only: `0;08,34,17` for `1/7` at depth 3; a
    /// sector at depth 0 writes its whole part alone.
    pub fn written(&self) -> String {
        let p = self.places();
        match p.places.is_empty() {
            true => p.whole.to_string(),
            false => {
                let places: Vec<String> = p.places.iter().map(|d| format!("{d:02}")).collect();
                format!("{};{}", p.whole, places.join(","))
            }
        }
    }
}

/// The bodies held, in the canonical order: the ordered key is the index,
/// so a neighbour is found by bisection on it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bodies {
    held: Vec<Coordinate>,
}

/// The bodies of one range on one lane, and the probes the two bisections
/// that found it took (the work counter, never a clock).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Neighbours<'a> {
    pub bodies: &'a [Coordinate],
    pub probes: u32,
}

/// The first place in `held` whose body does not stand before the bound,
/// by bisection, and the probes it took.
fn bisect(held: &[Coordinate], before: impl Fn(&Coordinate) -> bool) -> (usize, u32) {
    let (mut lo, mut hi, mut probes) = (0usize, held.len(), 0u32);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        probes += 1;
        match held.get(mid).is_some_and(&before) {
            true => lo = mid + 1,
            false => hi = mid,
        }
    }
    (lo, probes)
}

impl Bodies {
    /// No bodies.
    pub fn new() -> Bodies {
        Bodies { held: Vec::new() }
    }

    /// The bodies, in the canonical order.
    pub fn held(&self) -> &[Coordinate] {
        &self.held
    }

    /// Hold a body at its place in the canonical order, after any body at
    /// the same coordinate; returns the place.
    pub fn hold(&mut self, body: Coordinate) -> usize {
        let at = self.held.partition_point(|c| c <= &body);
        self.held.insert(at, body);
        at
    }

    /// The neighbours of `at` on `lane` over `[lo, hi)`: the bodies at
    /// `at`'s origin and bell whose lanes before `lane` equal `at`'s and
    /// whose value on `lane` lies in `[lo, hi)`. In the canonical order they
    /// stand together, so two bisections find them.
    pub fn range(&self, at: &Coordinate, lane: Lane, lo: &Ratio, hi: &Ratio) -> Neighbours<'_> {
        let n = lane.place();
        let key = |c: &Coordinate, v: &Ratio| -> Ordering {
            (&c.origin, c.local.h, &c.local.lanes[..n], &c.local.lanes[n]).cmp(&(
                &at.origin,
                at.local.h,
                &at.local.lanes[..n],
                v,
            ))
        };
        let (start, p) = bisect(&self.held, |c| key(c, lo) == Ordering::Less);
        let (end, q) = bisect(&self.held, |c| key(c, hi) == Ordering::Less);
        // An empty span (`hi ≤ lo`) ends at or before it starts: no body.
        let bodies = match self.held.get(start..end) {
            Some(b) => b,
            None => &[],
        };
        Neighbours {
            bodies,
            probes: p + q,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mycelium 78.173 §5: R is genesis 6a×32 at epoch 0x92.
    fn root() -> Origin {
        Origin::root([0x6a; 32], 0x92)
    }

    fn q(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    fn body(origin: &Origin, h: u64, lanes: [Ratio; 4], k: u64) -> Coordinate {
        Coordinate {
            origin: origin.clone(),
            local: Local { h, lanes, k },
        }
    }

    fn sh(s: Ratio, h: Ratio) -> [Ratio; 4] {
        [s, q(0, 1), q(0, 1), h]
    }

    fn tenth_15() -> Ratio {
        Ratio::new(BigInt::one(), BigInt::from(10).pow(15))
    }

    #[test]
    fn fixture_1_the_order_and_the_writing() {
        let r = root();
        let z = || q(0, 1);
        let enclosed = r.branch(Local {
            h: 3,
            lanes: [q(2, 1), z(), z(), z()],
            k: 0,
        });
        let order = [
            body(&r, 5, [q(-1, 2), z(), z(), z()], 1),
            body(&r, 5, [q(1, 3), q(-1, 1), z(), z()], 9),
            body(&r, 5, [q(1, 3), z(), z(), z()], 1),
            body(&r, 5, [q(1, 3), z(), z(), z()], 2),
            body(&r, 5, [q(1, 2), z(), z(), z()], 1),
            body(&r, 6, [q(-7, 1), z(), z(), z()], 0),
            body(&enclosed, 0, [z(), z(), z(), z()], 0),
        ];
        for pair in order.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} before {}",
                pair[0].written(),
                pair[1].written()
            );
        }
        let g = "6a".repeat(32);
        assert_eq!(
            order[2].written(),
            format!("{g}.00000092/0000000000000005:1/3:0/1:0/1:0/1:0000000000000001")
        );
        assert!(order[0].written().contains(":-1/2:"));
        assert_eq!(
            order[6].written(),
            format!(
                "{g}.00000092/0000000000000003:2/1:0/1:0/1:0/1:0000000000000000\
                 /0000000000000000:0/1:0/1:0/1:0/1:0000000000000000"
            )
        );
        // TPoint's case.
        let top = Ratio::from_integer(BigInt::from(u64::MAX));
        let tp = body(&r, 40961, [q(3, 1), top, z(), q(17, 1)], 812);
        assert_eq!(
            tp.written(),
            format!(
                "{g}.00000092/000000000000a001:3/1:ffffffffffffffff/1:0/1:11/1:000000000000032c"
            )
        );
        let local = Local {
            h: 10,
            lanes: [q(7, 30), z(), q(1, 900), q(2, 1)],
            k: 3,
        };
        assert_eq!(
            local.written(),
            "000000000000000a:7/1e:0/1:1/384:2/1:0000000000000003"
        );
    }

    #[test]
    fn fixture_2_the_sexagesimal_reading() {
        let third = q(1, 3) - tenth_15();
        let rows: [(Ratio, u32, i64, &str); 7] = [
            (q(7, 3), 0, 2, "2"),
            (q(7, 3), 1, 140, "2;20"),
            (q(7, 3), 2, 8400, "2;20,00"),
            (q(1, 7), 3, 30857, "0;08,34,17"),
            (q(360, 7), 2, 185142, "51;25,42"),
            (q(-1, 3), 1, -20, "-1;40"),
            (q(-1, 7), 1, -9, "-1;51"),
        ];
        for (v, d, m, w) in rows {
            let s = Sector::holding(&v, d);
            assert_eq!(s.index, BigInt::from(m));
            assert_eq!(s.written(), w);
            assert!(s.lo() <= v && v < s.hi());
        }
        assert_eq!(Sector::holding(&q(7, 3), 1).hi(), q(47, 20));
        assert_eq!(Sector::holding(&q(-1, 7), 1).lo(), q(-3, 20));
        let s = Sector::holding(&third, 6);
        assert_eq!(s.index, BigInt::from(15_551_999_999i64));
        assert_eq!(s.hi(), q(1, 3));
        assert_eq!(s.written(), "0;19,59,59,59,59,59");
    }

    #[test]
    fn fixture_3_sectors_are_exact_and_nested() {
        for v in [q(1, 7), q(-22, 7), q(7, 3), q(0, 1), q(-1, 3)] {
            for d in 0..=5 {
                let (coarse, fine) = (Sector::holding(&v, d), Sector::holding(&v, d + 1));
                assert!(coarse.lo() <= fine.lo() && fine.hi() <= coarse.hi());
                assert_eq!(Sector::holding(&fine.lo(), d), coarse);
                assert_eq!(fine.parent(), Some(coarse));
            }
        }
    }

    #[test]
    fn fixture_4_within_on_and_off_a_bound() {
        let r = root();
        let other = body(&r, 0, sh(q(1, 3), q(0, 1)), 0);
        let sixth = Ratio::new(BigInt::one(), BigInt::from(3) * scale(6));
        let rows = [
            (q(1, 3), 1, Side::Within),
            (q(7, 20), 1, Side::Above),
            (q(1, 3) - tenth_15(), 1, Side::Below),
            (q(1, 3) + sixth, 6, Side::Within),
            (q(2, 5), 0, Side::Within),
            (q(2, 5), 1, Side::Above),
        ];
        for (s, d, side) in rows {
            let b = body(&r, 0, sh(s, q(0, 1)), 0);
            assert_eq!(b.within(&other, Lane::S, d), Ok(side));
        }
    }

    #[test]
    fn fixture_5_origins_are_never_compared() {
        let r = root();
        let inner = r.branch(Local::at(3, sh(q(2, 1), q(0, 1))));
        let a = body(&r, 5, sh(q(1, 3), q(0, 1)), 0);
        let b = body(&inner, 5, sh(q(1, 3), q(0, 1)), 0);
        let refused = Err(CoordinateRefusal::OriginsDiffer {
            held: Box::new(r.clone()),
            asked: Box::new(inner.clone()),
        });
        assert_eq!(a.within(&b, Lane::S, 1), refused);
        assert!(a.merged(&q(1, 1), &b, &q(1, 1)).is_err());
    }

    #[test]
    fn fixture_6_a_range_is_the_test_of_every_body() {
        let r = root();
        let later = Origin::root([0x6a; 32], 0x93);
        let at = |o: &Origin, h: u64, s: Ratio, g: Ratio| body(o, h, [s, g, q(0, 1), q(0, 1)], 0);
        let mut held = Bodies::new();
        for i in 0..20 {
            held.hold(at(&r, 5, q(i, 10), q(0, 1)));
        }
        held.hold(at(&r, 6, q(1, 2), q(0, 1)));
        held.hold(at(&later, 6, q(1, 2), q(0, 1)));
        for g in [q(-1, 1), q(1, 4), q(2, 1)] {
            held.hold(at(&r, 5, q(1, 2), g));
        }
        let probe = at(&r, 5, q(0, 1), q(0, 1));
        let read = held.range(&probe, Lane::S, &q(1, 2), &q(9, 10));
        let got: Vec<(Ratio, Ratio)> = read
            .bodies
            .iter()
            .map(|b| (b.lane(Lane::S).clone(), b.lane(Lane::G).clone()))
            .collect();
        let half = || q(1, 2);
        assert_eq!(
            got,
            vec![
                (half(), q(-1, 1)),
                (half(), q(0, 1)),
                (half(), q(1, 4)),
                (half(), q(2, 1)),
                (q(3, 5), q(0, 1)),
                (q(7, 10), q(0, 1)),
                (q(4, 5), q(0, 1)),
            ]
        );
        let every: Vec<&Coordinate> = held
            .held()
            .iter()
            .filter(|b| {
                b.origin == r
                    && b.local.h == 5
                    && b.lane(Lane::S) >= &q(1, 2)
                    && b.lane(Lane::S) < &q(9, 10)
            })
            .collect();
        assert_eq!(read.bodies.iter().collect::<Vec<_>>(), every);
        let on_g = held.range(&at(&r, 5, q(1, 2), q(0, 1)), Lane::G, &q(0, 1), &q(1, 1));
        let gs: Vec<Ratio> = on_g
            .bodies
            .iter()
            .map(|b| b.lane(Lane::G).clone())
            .collect();
        assert_eq!(gs, vec![q(0, 1), q(1, 4)]);
        assert!(held
            .range(&probe, Lane::S, &q(9, 10), &q(1, 2))
            .bodies
            .is_empty());
        // N to 4N: each bisection is at most ⌊log₂ L⌋ + 1 probes.
        for n in [128i64, 512] {
            let mut many = Bodies::new();
            for i in 0..n {
                many.hold(at(&r, 5, q((i * 7919) % n, n), q(0, 1)));
            }
            let read = many.range(&probe, Lane::S, &q(1, 5), &q(2, 5));
            let bound = 2 * (63 - (n as u64).leading_zeros() + 1);
            assert!(read.probes <= bound, "{} probes over {n}", read.probes);
        }
    }

    #[test]
    fn fixture_7_advance_is_closed_form_and_composes() {
        let r = root();
        let p = body(&r, 10, sh(q(1, 3), q(1, 4)), 0);
        let rate = Rate([q(1, 7), q(0, 1), q(1, 2), q(1, 5)]);
        // One advance of 30 bells equals two of 12 and 18.
        let once = p.advanced(&rate, 40).expect("forward");
        let twice = p
            .advanced(&rate, 22)
            .and_then(|c| c.advanced(&rate, 40))
            .expect("forward");
        assert_eq!(once, twice);
        assert_eq!(once.lane(Lane::S), &(q(1, 3) + q(30, 7)));
        assert_eq!(once.lane(Lane::T), &q(15, 1));
        assert_eq!(once.lane(Lane::H), &q(25, 4));
        assert_eq!(once.local.k, 6);
        assert!(p.advanced(&rate, 9).is_none());
    }

    #[test]
    fn fixture_8_an_events_bell_is_solved_from_the_lane() {
        let p = Local::at(4, sh(q(0, 1), q(0, 1)));
        let rate = Rate([q(2, 3), q(0, 1), q(0, 1), q(0, 1)]);
        assert_eq!(p.when(&rate, Lane::S, &q(1, 1)), Some(q(11, 2)));
        assert_eq!(p.when(&rate, Lane::G, &q(1, 1)), None);
        assert_eq!(p.when(&rate, Lane::S, &q(-1, 1)), None);
    }

    #[test]
    fn fixtures_9_and_10_a_merger_keeps_its_offsets_and_fission_inverts_it() {
        let r = root();
        let a = body(&r, 2, sh(q(1, 1), q(2, 1)), 2);
        let b = body(&r, 2, sh(q(3, 1), q(0, 1)), 0);
        let m = a
            .merged(&q(1, 1), &b, &q(3, 1))
            .expect("one origin")
            .expect("one bell");
        assert_eq!(m.point.lane(Lane::S), &q(5, 2));
        assert_eq!(m.point.lane(Lane::H), &q(1, 2));
        assert_eq!(m.point.local.k, 0);
        assert_eq!(m.offsets[0], sh(q(-3, 2), q(3, 2)));
        assert_eq!(m.offsets[1], sh(q(1, 2), q(-1, 2)));
        for i in 0..4 {
            let sum = m.offsets[0][i].clone() * m.weights[0].clone()
                + m.offsets[1][i].clone() * m.weights[1].clone();
            assert!(sum.is_zero());
        }
        assert_eq!(m.fission(), [a.clone(), b.clone()]);
        let later = b.advanced(&Rate::still(), 3).expect("forward");
        assert_eq!(a.merged(&q(1, 1), &later, &q(1, 1)), Ok(None));
    }

    #[test]
    fn a_re_origin_keeps_the_reach_and_records_the_passage() {
        let r = root();
        let inner = r.branch(Local::at(1, sh(q(0, 1), q(0, 1))));
        let a = body(&r, 1, sh(q(1, 2), q(0, 1)), 0);
        let carried = a.re_origined(inner.clone(), &q(1, 1));
        assert_eq!(carried.origin, inner);
        assert_eq!(carried.lane(Lane::S), a.lane(Lane::S));
        assert_eq!(carried.lane(Lane::T), &q(1, 1));
    }
}
