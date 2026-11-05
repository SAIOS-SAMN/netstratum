// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The coordinate (docs/120; Mycelium LAW-COORDINATE, proposal 78.173,
//! FORMS-COORDINATE v3): every body stands at a TUniversal point inside the
//! universe, `origin / h : S : G : T : H : K`, and is addressed by it.
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
//! - **A lane is a vector at every tick** (operator 2026-11-05: "for each
//!   sexigesimal tick you contain your entire universe on a dimension as a
//!   vector"; docs/120 §8b): its components on its axes, never one number.
//!   `H`'s components are the planes' phases and the turnings between planes,
//!   stored in degrees with the plane named (docs/117 §11.1).
//! - `K` is the orbits closed, per component of `H`: `K_c = ⌊|θ_c|/360⌋`.
//!
//! The coordinate is the state. Motion is the closed-form advance of the
//! lanes from one bell to another, one addition per component, never a bell
//! stepped; an event's bell is solved from a component, never stepped to; no
//! period is assumed. Components are never summed into one another.
//!
//! Bodies are held in the canonical order ([`Bodies`]): a lane's components
//! in axis order. The neighbours on one component are two bisections, never
//! a test of every body. A sector of the sexagesimal reading is the product
//! cell holding every component, `[m·60⁻ᵈ, (m + 1)·60⁻ᵈ)` on each, pure base
//! 60 (on `H`, degrees;minutes,seconds). The places are display only.
//!
//! The interim form of Mycelium's `forms::coordinate` v3, its names and
//! writing the same, so the switch is an import at its tag.

use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;

/// The reading's base.
pub const BASE: u32 = 60;

/// Degrees in a whole turn: `H`'s closed orbits count by it.
pub const TURN: i64 = 360;

/// The four lanes, in the network's meanings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lane {
    /// Spatial: boost and dilation, the reach.
    S,
    /// Gauge: the change of kind.
    G,
    /// Temporal: null flux, the beat and the passages.
    T,
    /// Holonomic: the turnings, in degrees.
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

/// A lane's value: its components on its axes, in axis order, each exact and
/// signed. Ordered by its components in axis order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Value(pub Vec<Ratio>);

impl Value {
    /// A one-component lane.
    pub fn one(x: Ratio) -> Value {
        Value(vec![x])
    }

    /// Its components.
    pub fn components(&self) -> &[Ratio] {
        &self.0
    }

    /// Its exact writing: a one-component lane bare, `n/d`; a vector
    /// `(n₁/d₁,n₂/d₂)`, components in axis order, lowercase hex.
    pub fn written(&self) -> String {
        match self.0.as_slice() {
            [x] => lane_text(x),
            xs => format!(
                "({})",
                xs.iter().map(lane_text).collect::<Vec<_>>().join(",")
            ),
        }
    }
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

/// The whole turns of each of `H`'s components, both senses counted:
/// `⌊|θ|/360⌋`.
fn closed(h: &Value) -> Vec<u64> {
    h.0.iter()
        .map(|th| {
            (th.abs() / Ratio::from_integer(TURN.into()))
                .floor()
                .to_integer()
                .try_into()
                .unwrap_or(u64::MAX)
        })
        .collect()
}

/// A body's place within its origin: the bell, the four lanes in lane order,
/// the orbits closed per component of `H`. Ordered as written.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Local {
    pub h: u64,
    pub lanes: [Value; 4],
    pub k: Vec<u64>,
}

impl Local {
    /// A body standing at `lanes` at bell `h`, its closed orbits read from H.
    pub fn at(h: u64, lanes: [Value; 4]) -> Local {
        let k = closed(&lanes[Lane::H.place()]);
        Local { h, lanes, k }
    }

    /// One lane's value.
    pub fn lane(&self, lane: Lane) -> &Value {
        &self.lanes[lane.place()]
    }

    /// The writing `h:S:G:T:H:K`, `h` and each `K` in 16 lowercase hex
    /// digits, a vector `K` in parentheses as its lane is.
    pub fn written(&self) -> String {
        let mut fields = vec![format!("{:016x}", self.h)];
        fields.extend(self.lanes.iter().map(Value::written));
        let k: Vec<String> = self.k.iter().map(|k| format!("{k:016x}")).collect();
        fields.push(match k.as_slice() {
            [one] => one.clone(),
            [] => format!("{:016x}", 0),
            many => format!("({})", many.join(",")),
        });
        fields.join(":")
    }

    /// The place advanced in closed form to bell `to` under `rate`: each
    /// component plus its rate times the bells between, the closed orbits
    /// read from H. None where `to` stands before the place's own bell, or a
    /// rate's components do not match its lane's.
    pub fn advanced(&self, rate: &Rate, to: u64) -> Option<Local> {
        let bells = Ratio::from_integer(to.checked_sub(self.h)?.into());
        let mut lanes: [Value; 4] = Default::default();
        for (i, lane) in lanes.iter_mut().enumerate() {
            let (x, r) = (&self.lanes[i].0, &rate.0[i].0);
            if x.len() != r.len() {
                return None;
            }
            *lane = Value(
                x.iter()
                    .zip(r)
                    .map(|(x, r)| x.clone() + r.clone() * bells.clone())
                    .collect(),
            );
        }
        Some(Local::at(to, lanes))
    }

    /// When component `c` of `lane` reaches `value` under `rate`, solved in
    /// first degree from its closed form.
    pub fn bell_of(&self, rate: &Rate, lane: Lane, c: usize, value: &Ratio) -> Option<When> {
        let r = rate.0[lane.place()].0.get(c)?;
        let x = self.lane(lane).0.get(c)?;
        if r.is_zero() {
            return Some(When::Unmoving);
        }
        let at = Ratio::from_integer(self.h.into()) + (value.clone() - x.clone()) / r.clone();
        Some(if at < Ratio::from_integer(self.h.into()) {
            When::Behind(at)
        } else {
            When::At(at)
        })
    }
}

/// When a component reaches a value: at an exact bell, never (it does not
/// move), or before the place's own bell (the bell it stood there).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum When {
    At(Ratio),
    Unmoving,
    Behind(Ratio),
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
/// canonical key: origin, bell, each lane's components in lane and axis
/// order, the closed orbits.
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
    /// Two lanes with different numbers of components.
    ComponentsDiffer { lane: Lane },
}

/// Where a value stands against a sector, decided exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Below,
    Within,
    Above,
}

/// A body's rate per lane, per component: what each advances by a bell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rate(pub [Value; 4]);

impl Rate {
    /// No motion on lanes of these component counts.
    pub fn still(shape: [usize; 4]) -> Rate {
        Rate(std::array::from_fn(|i| Value(vec![ratio(0, 1); shape[i]])))
    }
}

/// Two bodies joined (TU-6 SUPERPOSE): the merged coordinate and each
/// body's offset from it, per lane and component, kept so nothing is
/// discarded. The offsets' holding-weighted sum is zero on every component,
/// and fission is the inverse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Merger {
    pub point: Coordinate,
    pub weights: [Ratio; 2],
    pub offsets: [[Value; 4]; 2],
}

impl Merger {
    /// The two bodies the merger was made of, exactly.
    pub fn fission(&self) -> [Coordinate; 2] {
        std::array::from_fn(|b| {
            let lanes = std::array::from_fn(|i| {
                Value(
                    self.point.local.lanes[i]
                        .0
                        .iter()
                        .zip(&self.offsets[b][i].0)
                        .map(|(x, o)| x.clone() + o.clone())
                        .collect(),
                )
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
    pub fn lane(&self, lane: Lane) -> &Value {
        self.local.lane(lane)
    }

    /// The sexagesimal reading of `lane` at `depth`: the product cell holding
    /// every component.
    pub fn reading(&self, lane: Lane, depth: u32) -> Cell {
        Cell::holding(self.lane(lane), depth)
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

    /// Where this body stands on `lane` against the cell at `depth` that
    /// holds `other`'s value there: a side per component, on the cell's
    /// exact bounds. Within the cell only where every side is `Within`.
    pub fn within(
        &self,
        other: &Coordinate,
        lane: Lane,
        depth: u32,
    ) -> Result<Vec<Side>, CoordinateRefusal> {
        self.same_origin(other)?;
        other.reading(lane, depth).sides(self.lane(lane))
    }

    /// The coordinate advanced in closed form to bell `to`, at its origin.
    pub fn advanced(&self, rate: &Rate, to: u64) -> Option<Coordinate> {
        Some(Coordinate {
            origin: self.origin.clone(),
            local: self.local.advanced(rate, to)?,
        })
    }

    /// Two bodies joined at one coordinate: each component the
    /// holding-weighted mean, each body's offset from it kept. Refused across
    /// origins or component counts; None across bells (advance one to the
    /// other's first) or with no holding.
    pub fn merged(
        &self,
        w_self: &Ratio,
        other: &Coordinate,
        w_other: &Ratio,
    ) -> Result<Option<Merger>, CoordinateRefusal> {
        self.same_origin(other)?;
        for lane in Lane::ALL {
            if self.lane(lane).0.len() != other.lane(lane).0.len() {
                return Err(CoordinateRefusal::ComponentsDiffer { lane });
            }
        }
        let whole = w_self.clone() + w_other.clone();
        if self.local.h != other.local.h || whole.is_zero() {
            return Ok(None);
        }
        let lanes: [Value; 4] = std::array::from_fn(|i| {
            Value(
                self.local.lanes[i]
                    .0
                    .iter()
                    .zip(&other.local.lanes[i].0)
                    .map(|(a, b)| {
                        (a.clone() * w_self.clone() + b.clone() * w_other.clone()) / whole.clone()
                    })
                    .collect(),
            )
        });
        let offsets = [self, other].map(|b| {
            std::array::from_fn(|i| {
                Value(
                    b.local.lanes[i]
                        .0
                        .iter()
                        .zip(&lanes[i].0)
                        .map(|(x, m)| x.clone() - m.clone())
                        .collect(),
                )
            })
        });
        Ok(Some(Merger {
            point: Coordinate {
                origin: self.origin.clone(),
                local: Local::at(self.local.h, lanes),
            },
            weights: [w_self.clone(), w_other.clone()],
            offsets,
        }))
    }

    /// The passage through the throat (docs/117 §12): the pole swap, an exact
    /// involution. The span on `S`'s first component reverses (its sheet
    /// flips, its reach `|S|` kept: no reach moves), and `T`'s first
    /// component gains the span's change, `−2·(A − Q)`.
    pub fn passed(&self) -> Coordinate {
        let mut lanes = self.local.lanes.clone();
        let span = lanes[Lane::S.place()]
            .0
            .first()
            .cloned()
            .unwrap_or_else(|| ratio(0, 1));
        if let Some(s) = lanes[Lane::S.place()].0.first_mut() {
            *s = -span.clone();
        }
        if let Some(t) = lanes[Lane::T.place()].0.first_mut() {
            *t -= ratio(2, 1) * span;
        }
        Coordinate {
            origin: self.origin.clone(),
            local: Local::at(self.local.h, lanes),
        }
    }

    /// A body carried into another enclosure across a throat (a re-origin):
    /// its passage (`passed`), written at `into`.
    pub fn re_origined(&self, into: Origin) -> Coordinate {
        Coordinate {
            origin: into,
            ..self.passed()
        }
    }
}

/// One sector of the sexagesimal reading of one component: at `depth`, the
/// exact interval `[index·60⁻ᵈᵉᵖᵗʰ, (index + 1)·60⁻ᵈᵉᵖᵗʰ)`. The value read is
/// never replaced by it.
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

/// The sexagesimal reading of a lane at a depth: the product cell, one
/// sector per component, holding every component at once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell(pub Vec<Sector>);

impl Cell {
    /// The cell at `depth` holding every component of `value`.
    pub fn holding(value: &Value, depth: u32) -> Cell {
        Cell(value.0.iter().map(|x| Sector::holding(x, depth)).collect())
    }

    /// The side of each component of `value` against the cell's sectors.
    pub fn sides(&self, value: &Value) -> Result<Vec<Side>, CoordinateRefusal> {
        if value.0.len() != self.0.len() {
            return Err(CoordinateRefusal::ComponentsDiffer { lane: Lane::H });
        }
        Ok(self
            .0
            .iter()
            .zip(&value.0)
            .map(|(s, x)| s.side(x))
            .collect())
    }

    /// Whether `value` stands within the cell: every component within.
    pub fn holds(&self, value: &Value) -> bool {
        self.sides(value)
            .is_ok_and(|s| s.iter().all(|x| *x == Side::Within))
    }

    /// The writing, for display only: a one-component cell bare, components
    /// joined by ` × `.
    pub fn written(&self) -> String {
        self.0
            .iter()
            .map(Sector::written)
            .collect::<Vec<_>>()
            .join(" × ")
    }
}

/// The bodies held, in the canonical order: the ordered key is the index,
/// so a neighbour is found by bisection on it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bodies {
    held: Vec<Coordinate>,
}

/// The bodies of one range on one component, and the probes the two
/// bisections that found it took (the work counter, never a clock).
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

    /// The neighbours of `at` on component `c` of `lane` over `[lo, hi)`:
    /// the bodies at `at`'s origin and bell whose lanes before `lane`, and
    /// components before `c`, equal `at`'s, and whose component `c` lies in
    /// `[lo, hi)`. In the canonical order they stand together, so two
    /// bisections find them; any other component is a filter within them.
    pub fn range(
        &self,
        at: &Coordinate,
        lane: Lane,
        c: usize,
        lo: &Ratio,
        hi: &Ratio,
    ) -> Neighbours<'_> {
        let n = lane.place();
        let key = |x: &Coordinate, v: &Ratio| -> Ordering {
            let mine = x.local.lanes[n].0.get(..c).unwrap_or(&[]);
            let theirs = at.local.lanes[n].0.get(..c).unwrap_or(&[]);
            let value = x.local.lanes[n].0.get(c);
            (&x.origin, x.local.h, &x.local.lanes[..n], mine, value).cmp(&(
                &at.origin,
                at.local.h,
                &at.local.lanes[..n],
                theirs,
                Some(v),
            ))
        };
        let (start, p) = bisect(&self.held, |x| key(x, lo) == Ordering::Less);
        let (end, q) = bisect(&self.held, |x| key(x, hi) == Ordering::Less);
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

    fn v(x: Ratio) -> Value {
        Value::one(x)
    }

    fn z() -> Value {
        v(q(0, 1))
    }

    /// One-component lanes S, G, T, H.
    fn lanes(s: Ratio, g: Ratio, t: Ratio, h: Ratio) -> [Value; 4] {
        [v(s), v(g), v(t), v(h)]
    }

    fn body(origin: &Origin, h: u64, lanes: [Value; 4], k: Vec<u64>) -> Coordinate {
        Coordinate {
            origin: origin.clone(),
            local: Local { h, lanes, k },
        }
    }

    fn tenth_15() -> Ratio {
        Ratio::new(BigInt::one(), BigInt::from(10).pow(15))
    }

    #[test]
    fn fixture_1_the_order_and_the_writing() {
        let r = root();
        let o = || q(0, 1);
        let enclosed = r.branch(Local {
            h: 3,
            lanes: lanes(q(2, 1), o(), o(), o()),
            k: vec![0],
        });
        let order = [
            body(&r, 5, lanes(q(-1, 2), o(), o(), o()), vec![1]),
            body(&r, 5, lanes(q(1, 3), q(-1, 1), o(), o()), vec![9]),
            body(&r, 5, lanes(q(1, 3), o(), o(), o()), vec![1]),
            body(&r, 5, lanes(q(1, 3), o(), o(), o()), vec![2]),
            body(&r, 5, lanes(q(1, 2), o(), o(), o()), vec![1]),
            body(&r, 6, lanes(q(-7, 1), o(), o(), o()), vec![0]),
            body(&enclosed, 0, lanes(o(), o(), o(), o()), vec![0]),
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
        let top = Ratio::from_integer(BigInt::from(u64::MAX));
        let tp = body(&r, 40961, lanes(q(3, 1), top, o(), q(17, 1)), vec![812]);
        assert_eq!(
            tp.written(),
            format!(
                "{g}.00000092/000000000000a001:3/1:ffffffffffffffff/1:0/1:11/1:000000000000032c"
            )
        );
        let local = Local {
            h: 10,
            lanes: lanes(q(7, 30), o(), q(1, 900), q(2, 1)),
            k: vec![3],
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
        for (x, d, m, w) in rows {
            let s = Sector::holding(&x, d);
            assert_eq!(s.index, BigInt::from(m));
            assert_eq!(s.written(), w);
            assert!(s.lo() <= x && x < s.hi());
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
        for x in [q(1, 7), q(-22, 7), q(7, 3), q(0, 1), q(-1, 3)] {
            for d in 0..=5 {
                let (coarse, fine) = (Sector::holding(&x, d), Sector::holding(&x, d + 1));
                assert!(coarse.lo() <= fine.lo() && fine.hi() <= coarse.hi());
                assert_eq!(Sector::holding(&fine.lo(), d), coarse);
                assert_eq!(fine.parent(), Some(coarse));
            }
        }
    }

    #[test]
    fn fixture_4_within_on_and_off_a_bound() {
        let r = root();
        let other = body(&r, 0, lanes(q(1, 3), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
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
            let b = body(&r, 0, lanes(s, q(0, 1), q(0, 1), q(0, 1)), vec![0]);
            assert_eq!(b.within(&other, Lane::S, d), Ok(vec![side]));
        }
    }

    #[test]
    fn fixture_5_origins_are_never_compared() {
        let r = root();
        let inner = r.branch(Local::at(3, lanes(q(2, 1), q(0, 1), q(0, 1), q(0, 1))));
        let a = body(&r, 5, lanes(q(1, 3), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
        let b = body(
            &inner,
            5,
            lanes(q(1, 3), q(0, 1), q(0, 1), q(0, 1)),
            vec![0],
        );
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
        let at = |o: &Origin, h: u64, s: Ratio, g: Ratio| {
            body(o, h, lanes(s, g, q(0, 1), q(0, 1)), vec![0])
        };
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
        let read = held.range(&probe, Lane::S, 0, &q(1, 2), &q(9, 10));
        let got: Vec<(Value, Value)> = read
            .bodies
            .iter()
            .map(|b| (b.lane(Lane::S).clone(), b.lane(Lane::G).clone()))
            .collect();
        let half = || v(q(1, 2));
        assert_eq!(
            got,
            vec![
                (half(), v(q(-1, 1))),
                (half(), z()),
                (half(), v(q(1, 4))),
                (half(), v(q(2, 1))),
                (v(q(3, 5)), z()),
                (v(q(7, 10)), z()),
                (v(q(4, 5)), z()),
            ]
        );
        let on_g = held.range(&at(&r, 5, q(1, 2), q(0, 1)), Lane::G, 0, &q(0, 1), &q(1, 1));
        let gs: Vec<Value> = on_g
            .bodies
            .iter()
            .map(|b| b.lane(Lane::G).clone())
            .collect();
        assert_eq!(gs, vec![z(), v(q(1, 4))]);
        assert!(held
            .range(&probe, Lane::S, 0, &q(9, 10), &q(1, 2))
            .bodies
            .is_empty());
        // N to 4N: each bisection is at most ⌊log₂ L⌋ + 1 probes.
        for n in [128i64, 512] {
            let mut many = Bodies::new();
            for i in 0..n {
                many.hold(at(&r, 5, q((i * 7919) % n, n), q(0, 1)));
            }
            let read = many.range(&probe, Lane::S, 0, &q(1, 5), &q(2, 5));
            let bound = 2 * (63 - (n as u64).leading_zeros() + 1);
            assert!(read.probes <= bound, "{} probes over {n}", read.probes);
        }
    }

    #[test]
    fn fixture_7_advance_is_closed_form_and_composes_in_degrees() {
        // H in degrees: 90° at bell 10, turning 72° a bell; 30 bells on it
        // stands at 2250°, six whole turns.
        let r = root();
        let p = body(&r, 10, lanes(q(1, 3), q(0, 1), q(0, 1), q(90, 1)), vec![0]);
        let rate = Rate(lanes(q(1, 7), q(0, 1), q(1, 2), q(72, 1)));
        let once = p.advanced(&rate, 40).expect("forward");
        let twice = p
            .advanced(&rate, 22)
            .and_then(|c| c.advanced(&rate, 40))
            .expect("forward");
        assert_eq!(once, twice);
        assert_eq!(once.lane(Lane::S), &v(q(1, 3) + q(30, 7)));
        assert_eq!(once.lane(Lane::T), &v(q(15, 1)));
        assert_eq!(once.lane(Lane::H), &v(q(2250, 1)));
        assert_eq!(once.local.k, vec![6]);
        assert!(p.advanced(&rate, 9).is_none());
    }

    #[test]
    fn fixture_8_an_events_bell_is_solved_from_the_lane() {
        let p = Local::at(4, lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1)));
        let rate = Rate(lanes(q(2, 3), q(0, 1), q(0, 1), q(0, 1)));
        assert_eq!(
            p.bell_of(&rate, Lane::S, 0, &q(1, 1)),
            Some(When::At(q(11, 2)))
        );
        assert_eq!(p.bell_of(&rate, Lane::G, 0, &q(1, 1)), Some(When::Unmoving));
        assert_eq!(
            p.bell_of(&rate, Lane::S, 0, &q(-1, 1)),
            Some(When::Behind(q(5, 2)))
        );
    }

    #[test]
    fn fixtures_9_and_10_a_merger_keeps_its_offsets_and_fission_inverts_it() {
        // H in degrees: a at 720° (two whole turns), b at 0.
        let r = root();
        let a = body(&r, 5, lanes(q(1, 1), q(0, 1), q(0, 1), q(720, 1)), vec![2]);
        let b = body(&r, 5, lanes(q(3, 1), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
        let m = a
            .merged(&q(1, 1), &b, &q(3, 1))
            .expect("one origin")
            .expect("one bell");
        assert_eq!(m.point.lane(Lane::S), &v(q(5, 2)));
        assert_eq!(m.point.lane(Lane::H), &v(q(180, 1)));
        assert_eq!(m.point.local.k, vec![0]);
        assert_eq!(m.offsets[0][0], v(q(-3, 2)));
        assert_eq!(m.offsets[0][3], v(q(540, 1)));
        assert_eq!(m.offsets[1][0], v(q(1, 2)));
        assert_eq!(m.offsets[1][3], v(q(-180, 1)));
        assert_eq!(m.fission(), [a.clone(), b.clone()]);
        let later = b.advanced(&Rate::still([1; 4]), 6).expect("forward");
        assert_eq!(a.merged(&q(1, 1), &later, &q(1, 1)), Ok(None));
    }

    /// Bodies at R, bell 5, S = G = T = 0, H = (θ₁, θ₂) in degrees.
    fn planar(h1: Ratio, h2: Ratio) -> Coordinate {
        let lanes = [z(), z(), z(), Value(vec![h1, h2])];
        Coordinate {
            origin: root(),
            local: Local::at(5, lanes),
        }
    }

    #[test]
    fn fixture_11_a_vector_lane_is_ordered_and_read_by_its_cell() {
        let a = planar(q(1, 3), q(7, 4));
        let b = planar(q(1, 3), q(-1, 2));
        let c = planar(q(1, 2), q(0, 1));
        assert!(b < a && a < c, "component 1, the tie by component 2");
        assert_eq!(a.lane(Lane::H).written(), "(1/3,7/4)");
        assert_eq!(a.reading(Lane::H, 0).written(), "0 × 1");
        let cell = a.reading(Lane::H, 1);
        assert_eq!(cell.written(), "0;20 × 1;45");
        assert_eq!(cell.0[0].lo(), q(1, 3));
        assert_eq!(cell.0[0].hi(), q(7, 20));
        assert_eq!(cell.0[1].lo(), q(7, 4));
        assert_eq!(cell.0[1].hi(), q(53, 30));
        assert_eq!(Sector::holding(&q(-1, 2), 0).written(), "-1");
        assert_eq!(Sector::holding(&q(-1, 2), 1).written(), "-1;30");
        assert_eq!(
            c.within(&a, Lane::H, 0),
            Ok(vec![Side::Within, Side::Below])
        );
        assert!(!a.reading(Lane::H, 0).holds(c.lane(Lane::H)));
    }

    #[test]
    fn fixture_12_a_range_on_one_component_then_a_filter() {
        let (a, b, c, d) = (
            planar(q(1, 3), q(7, 4)),
            planar(q(1, 3), q(-1, 2)),
            planar(q(1, 2), q(0, 1)),
            planar(q(2, 5), q(7, 4)),
        );
        let mut held = Bodies::new();
        for x in [&a, &b, &c, &d] {
            held.hold(x.clone());
        }
        let probe = planar(q(0, 1), q(0, 1));
        let read = held.range(&probe, Lane::H, 0, &q(1, 3), &q(1, 2));
        assert_eq!(read.bodies, &[b.clone(), a.clone(), d.clone()]);
        let filtered: Vec<&Coordinate> = read
            .bodies
            .iter()
            .filter(|x| {
                let t = &x.lane(Lane::H).0[1];
                *t >= q(1, 1) && *t < q(2, 1)
            })
            .collect();
        assert_eq!(filtered, vec![&a, &d]);
    }

    #[test]
    fn fixture_13_a_merger_and_fission_per_component() {
        let a = planar(q(1, 3), q(7, 4));
        let e = planar(q(1, 1), q(-1, 4));
        let m = a
            .merged(&q(1, 1), &e, &q(2, 1))
            .expect("one origin")
            .expect("one bell");
        assert_eq!(m.point.lane(Lane::H), &Value(vec![q(7, 9), q(5, 12)]));
        assert_eq!(m.offsets[0][3], Value(vec![q(-4, 9), q(4, 3)]));
        assert_eq!(m.offsets[1][3], Value(vec![q(2, 9), q(-2, 3)]));
        for k in 0..2 {
            let sum = m.offsets[0][3].0[k].clone() + q(2, 1) * m.offsets[1][3].0[k].clone();
            assert!(sum.is_zero());
        }
        assert_eq!(m.point.reading(Lane::H, 1).written(), "0;46 × 0;25");
        assert_eq!(m.fission(), [a, e]);
        // Lanes of different component counts are refused.
        let one = body(&root(), 5, [z(), z(), z(), z()], vec![0]);
        assert_eq!(
            planar(q(0, 1), q(0, 1)).merged(&q(1, 1), &one, &q(1, 1)),
            Err(CoordinateRefusal::ComponentsDiffer { lane: Lane::H })
        );
    }

    #[test]
    fn a_passage_is_the_pole_swap_and_writes_its_span_change_on_t() {
        // docs/117 §12: span 3/5 crosses to −3/5, T gains −6/5; the return
        // restores both exactly (M∘M = 1); at the throat nothing changes.
        let r = root();
        let a = body(&r, 1, lanes(q(3, 5), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
        let once = a.passed();
        assert_eq!(once.lane(Lane::S), &v(q(-3, 5)));
        assert_eq!(once.lane(Lane::T), &v(q(-6, 5)));
        assert_eq!(once.passed(), a);
        let throat = body(&r, 1, lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
        assert_eq!(throat.passed(), throat);
        // A re-origin is the passage written at the other enclosure.
        let inner = r.branch(Local::at(1, lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1))));
        let carried = a.re_origined(inner.clone());
        assert_eq!(carried.origin, inner);
        assert_eq!(carried.local, once.local);
    }
}
