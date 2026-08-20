//! **THE ASYMMETRIC REDUCE** (N-I1, doc `53` D2, seat of the one reduction) —
//! four small algebras plus a twenty-line conductor. Re-expressed from v0.4
//! `iae/src/{reduction,conjugate}` onto netstratum `CellFrame` carriers; the
//! orbit-min quotient constructor (`canonicalize`) lives here (charter D2 keeps
//! the `reduction::canonicalize` call path — `conjugate` is not ported).
//!
//! Landing order, each with its own witness in `tests/reduction.rs`:
//!   Slice 1 — `PlaneElement` (D₄) + `Relabel` + `ObserverFrame` (where a silent
//!             sign lives; pinned by reconstruction before `reduce_pair` exists).
//!   Slice 2 — `canonicalize` (orbit-min, first-appearance relabel) +
//!             `deduce_gauge_move` (form equality → the rule).
//!   Slice 3 — `drain_exact` (identity on empty basis) + `route_per_pair` +
//!             `reduce_pair` (the conductor).
//!
//! No `reduce_pair_indexed` / `SigilIndex` / valence routing / `think` /
//! `observe` — later landings. `reduce_cochain`/SNF untouched: two functions,
//! two jobs (the SNF verifier gates commits; this is the think drain).

use crate::congruence::{signed_entry_sum, signed_remainder, ChainDelta};
use crate::peel::{CellFrame, FullMeasurement, GaugeDirection, MeasurementRefusal, Site};
use thiserror::Error;

// ─────────────────────────────────────────────────────────────────────
// SLICE 1 — the plane group, the relabel, the observer frame.
// ─────────────────────────────────────────────────────────────────────

/// Exchange the two axes, or keep them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exchange {
    Keep,
    Swap,
}

impl Exchange {
    fn compose(self, other: Exchange) -> Exchange {
        match (self, other) {
            (Exchange::Keep, x) | (x, Exchange::Keep) => x,
            (Exchange::Swap, Exchange::Swap) => Exchange::Keep,
        }
    }
}

/// Mirror an axis, or keep it. An involution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flip {
    Keep,
    Mirror,
}

impl Flip {
    fn compose(self, other: Flip) -> Flip {
        match (self, other) {
            (Flip::Keep, x) | (x, Flip::Keep) => x,
            (Flip::Mirror, Flip::Mirror) => Flip::Keep,
        }
    }
}

/// A plane-group element in signed-permutation form: exchange the axes, then
/// mirror per axis — the eight combinations ARE the plane group D₄ (the
/// observer's spatial gauge). No `rot90` constructor: a quarter turn is one of
/// these eight, read off its components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaneElement {
    pub exchange: Exchange,
    pub row: Flip,
    pub col: Flip,
}

impl PlaneElement {
    pub const IDENTITY: Self = Self { exchange: Exchange::Keep, row: Flip::Keep, col: Flip::Keep };

    /// All eight elements — the group-order sweep's domain (a structural
    /// constant, never a content search).
    pub fn all() -> [Self; 8] {
        let flips = [Flip::Keep, Flip::Mirror];
        let mut out = [Self::IDENTITY; 8];
        let mut at = 0;
        for exchange in [Exchange::Keep, Exchange::Swap] {
            for row in flips {
                for col in flips {
                    out[at] = Self { exchange, row, col };
                    at += 1;
                }
            }
        }
        out
    }

    /// The transformed frame (Swap transposes rows↔cols).
    pub fn frame(&self, frame: CellFrame) -> CellFrame {
        match self.exchange {
            Exchange::Keep => frame,
            Exchange::Swap => CellFrame { rows: frame.cols, cols: frame.rows },
        }
    }

    /// The transformed site (exchange first, then mirrors on the post-exchange
    /// frame).
    pub fn site(&self, site: Site, frame: CellFrame) -> Site {
        let (mut r, mut c, out_frame) = match self.exchange {
            Exchange::Keep => (site.row, site.col, frame),
            Exchange::Swap => (site.col, site.row, self.frame(frame)),
        };
        r = match self.row {
            Flip::Keep => r,
            Flip::Mirror => i64::from(out_frame.rows) - 1 - r,
        };
        c = match self.col {
            Flip::Keep => c,
            Flip::Mirror => i64::from(out_frame.cols) - 1 - c,
        };
        Site { row: r, col: c }
    }

    /// Group composition `self ∘ other` (other applies first). The exchange of
    /// `self` re-homes `other`'s per-axis mirrors.
    pub fn compose(&self, other: &Self) -> Self {
        let (other_row, other_col) = match self.exchange {
            Exchange::Keep => (other.row, other.col),
            Exchange::Swap => (other.col, other.row),
        };
        Self {
            exchange: self.exchange.compose(other.exchange),
            row: self.row.compose(other_row),
            col: self.col.compose(other_col),
        }
    }

    /// The exact group inverse.
    pub fn inverse(&self) -> Self {
        match self.exchange {
            Exchange::Keep => *self,
            Exchange::Swap => Self { exchange: Exchange::Swap, row: self.col, col: self.row },
        }
    }
}

/// A relabel of gauge directions — a PERMUTATION of one chart, carried
/// canonically as its moved points only (a permutation IS its action; fixed
/// points and listing order are presentation, so identity over two explicit
/// domains is ONE permutation). Rules speak this carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relabel {
    pub pairs: Vec<(u8, u8)>,
}

impl Relabel {
    /// The canonical carrier: MOVED POINTS ONLY, sorted by source label.
    pub fn new(mut pairs: Vec<(u8, u8)>) -> Self {
        pairs.retain(|(from, to)| from != to);
        pairs.sort_unstable();
        Self { pairs }
    }

    /// Where a direction goes — total: an unmoved direction is fixed.
    pub fn image(&self, direction: GaugeDirection) -> GaugeDirection {
        self.pairs
            .iter()
            .find(|(from, _)| *from == direction.index)
            .map(|(_, to)| GaugeDirection { index: *to })
            .unwrap_or(direction)
    }

    /// The exact inverse: every moved pair reversed.
    pub fn inverse(&self) -> Self {
        Self::new(self.pairs.iter().map(|(from, to)| (*to, *from)).collect())
    }

    /// Composition `self ∘ other` (other applies first) — total: the moved
    /// domain is the union of both permutations' moved points.
    pub fn compose(&self, other: &Self) -> Self {
        let mut domain: Vec<u8> =
            self.pairs.iter().chain(other.pairs.iter()).map(|(from, _)| *from).collect();
        domain.sort_unstable();
        domain.dedup();
        Self::new(
            domain
                .into_iter()
                .map(|from| {
                    let mid = other.image(GaugeDirection { index: from });
                    (from, self.image(mid).index)
                })
                .collect(),
        )
    }
}

/// The observer frame: a plane element and a relabel, applied plane-first.
/// Carried with exact inverses at every component — the achieving frame of a
/// canonicalization and the RULE of a deduced gauge move are both this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverFrame {
    pub plane: PlaneElement,
    pub relabel: Relabel,
}

impl ObserverFrame {
    /// Apply to a measurement: sites through the plane element, directions
    /// through the relabel (total), wholeness re-verified by the measurement's
    /// own constructor.
    pub fn apply(&self, measurement: &FullMeasurement) -> Result<FullMeasurement, FrameRefusal> {
        let in_frame = measurement.frame();
        let out_frame = self.plane.frame(in_frame);
        let occupancy: Vec<(Site, GaugeDirection)> = measurement
            .occupancy()
            .iter()
            .map(|(site, direction)| {
                (self.plane.site(*site, in_frame), self.relabel.image(*direction))
            })
            .collect();
        FullMeasurement::try_new(measurement.fiber_rank(), out_frame, occupancy)
            .map_err(FrameRefusal::Measurement)
    }

    /// `self ∘ other` (other applies first) — both components compose.
    pub fn compose(&self, other: &Self) -> Self {
        Self { plane: self.plane.compose(&other.plane), relabel: self.relabel.compose(&other.relabel) }
    }

    /// The exact inverse frame.
    pub fn inverse(&self) -> Self {
        Self { plane: self.plane.inverse(), relabel: self.relabel.inverse() }
    }
}

/// The identity observer frame — identity plane, empty relabel.
pub fn identity_frame() -> ObserverFrame {
    ObserverFrame { plane: PlaneElement::IDENTITY, relabel: Relabel::new(Vec::new()) }
}

/// Typed refusals of the frame surface.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FrameRefusal {
    #[error("direction {index} outside the renaming's chart")]
    DirectionNotInChart { index: u8 },

    #[error("frame application broke measurement wholeness: {0:?}")]
    Measurement(MeasurementRefusal),
}

// ─────────────────────────────────────────────────────────────────────
// SLICE 2 — the orbit-min canonical form and the deduced gauge move.
// ─────────────────────────────────────────────────────────────────────

/// A RENAMING — the FULL chart map of gauge directions (from → to), every
/// present direction carried explicitly, identity-acting pairs INCLUDED (for a
/// chart-to-chart map `0 → 0` is real action, not presentation; dropping it
/// destroys the domain and fabricates action outside the chart). The achieving
/// frames of canonicalization are renamings; RULES are `Relabel` permutations —
/// two carriers, two jobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renaming {
    pub pairs: Vec<(u8, u8)>,
}

impl Renaming {
    /// Canonical: pairs sorted by source, ALL pairs kept.
    pub fn new(mut pairs: Vec<(u8, u8)>) -> Self {
        pairs.sort_unstable();
        Self { pairs }
    }

    /// Where a direction goes; a direction outside the chart refuses typed.
    pub fn image(&self, direction: GaugeDirection) -> Result<GaugeDirection, FrameRefusal> {
        self.pairs
            .iter()
            .find(|(from, _)| *from == direction.index)
            .map(|(_, to)| GaugeDirection { index: *to })
            .ok_or(FrameRefusal::DirectionNotInChart { index: direction.index })
    }

    /// The exact inverse chart map.
    pub fn inverse(&self) -> Self {
        Self::new(self.pairs.iter().map(|(from, to)| (*to, *from)).collect())
    }
}

/// The achieving frame of a canonicalization: a plane element and the full chart
/// RENAMING, applied plane-first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenamingFrame {
    pub plane: PlaneElement,
    pub renaming: Renaming,
}

impl RenamingFrame {
    /// Apply to a measurement: sites through the plane, directions through the
    /// chart map (out-of-chart refuses typed), wholeness re-verified.
    pub fn apply(&self, measurement: &FullMeasurement) -> Result<FullMeasurement, FrameRefusal> {
        let in_frame = measurement.frame();
        let out_frame = self.plane.frame(in_frame);
        let occupancy = measurement
            .occupancy()
            .iter()
            .map(|(site, direction)| {
                self.renaming.image(*direction).map(|d| (self.plane.site(*site, in_frame), d))
            })
            .collect::<Result<Vec<_>, _>>()?;
        FullMeasurement::try_new(measurement.fiber_rank(), out_frame, occupancy)
            .map_err(FrameRefusal::Measurement)
    }
}

/// The canonical form — a full measurement in the pinned representative gauge.
/// Re-liftable by construction, never a digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalForm {
    measurement: FullMeasurement,
}

impl CanonicalForm {
    /// The form IS a measurement — whole, re-liftable.
    pub fn to_measurement(&self) -> FullMeasurement {
        self.measurement.clone()
    }
}

/// The canonicalization emission: the form and the achieving frame (the chart
/// map into the canonical gauge). The whole stabilizer COSET (glue across pairs)
/// is N-I3, not this slice — one achieving frame is enough for Pin A.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canonicalized {
    pub form: CanonicalForm,
    pub achieving_frame: RenamingFrame,
}

/// **ONE CANONICALIZATION PER MEASUREMENT** — orbit-min over the plane group
/// (eight elements, a structural constant), first-appearance relabel per image,
/// row-major lex-min over (frame, direction bytes). Orbit-min IS how the
/// canonical form is built; the sweep is group-order bounded, NEVER a content
/// search, and NEVER a no-op (a no-op canonicalize reads a pure hue permutation
/// as a generation gap — the sodalitum false wall).
pub fn canonicalize(measurement: &FullMeasurement) -> Canonicalized {
    let candidates: Vec<(Vec<u8>, CellFrame, RenamingFrame, FullMeasurement)> = PlaneElement::all()
        .iter()
        .map(|element| {
            let frame = RenamingFrame {
                plane: *element,
                renaming: first_appearance_renaming(measurement, element),
            };
            let image = frame
                .apply(measurement)
                .expect("the first-appearance chart covers every present direction");
            let bytes: Vec<u8> = image.occupancy().iter().map(|(_, d)| d.index).collect();
            (bytes, image.frame(), frame, image)
        })
        .collect();
    let winner = candidates
        .iter()
        .min_by_key(|(bytes, frame, _, _)| (frame.rows, frame.cols, bytes.clone()))
        .expect("the plane group is never empty");
    Canonicalized {
        form: CanonicalForm { measurement: winner.3.clone() },
        achieving_frame: winner.2.clone(),
    }
}

/// The first-appearance RENAMING of a plane image: every present direction
/// renamed in the order the transformed row-major walk first meets it — the
/// fiber-group quotient's pinned convention. A full chart map: identity-acting
/// pairs are kept (they ARE the chart).
fn first_appearance_renaming(measurement: &FullMeasurement, element: &PlaneElement) -> Renaming {
    let in_frame = measurement.frame();
    let mut transformed: Vec<(Site, GaugeDirection)> = measurement
        .occupancy()
        .iter()
        .map(|(site, direction)| (element.site(*site, in_frame), *direction))
        .collect();
    transformed.sort_by_key(|(site, _)| (site.row, site.col));
    let mut pairs: Vec<(u8, u8)> = Vec::new();
    for (_, direction) in &transformed {
        if !pairs.iter().any(|(from, _)| *from == direction.index) {
            let next = u8::try_from(pairs.len()).expect("fiber ranks are small");
            pairs.push((direction.index, next));
        }
    }
    Renaming::new(pairs)
}

/// The deduction's typed verdict: a pure gauge move with its derived rule, or
/// real content difference (which enters the reduction ladder).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GaugeDeduction {
    GaugeMove { rule: ObserverFrame },
    ContentDiffers,
}

/// Equal canonical forms ⟺ a gauge move; the rule is the difference of achieved
/// frames. Form equality ONLY — no content walk.
pub fn deduce_gauge_move(a: &Canonicalized, b: &Canonicalized) -> GaugeDeduction {
    if a.form == b.form {
        GaugeDeduction::GaugeMove { rule: rule_between_frames(&a.achieving_frame, &b.achieving_frame) }
    } else {
        GaugeDeduction::ContentDiffers
    }
}

/// The difference of two achieved frames — `g_B⁻¹ ∘ g_A` on the plane; the
/// relabel walks A's FULL chart through the inverse of B's chart
/// (raw_A → canonical → raw_B), so the rule is a bijective permutation whose
/// inverse reconstructs (the componentwise `Relabel.compose` fabricates a
/// non-bijective rule on shared images — the reconstruction pin catches it).
fn rule_between_frames(g_a: &RenamingFrame, g_b: &RenamingFrame) -> ObserverFrame {
    let plane = g_b.plane.inverse().compose(&g_a.plane);
    let back = g_b.renaming.inverse();
    let pairs = g_a
        .renaming
        .pairs
        .iter()
        .map(|(raw_a, canonical_label)| {
            back.image(GaugeDirection { index: *canonical_label })
                .map(|raw_b| (*raw_a, raw_b.index))
                .expect("equal forms share one canonical chart — every label maps back")
        })
        .collect();
    ObserverFrame { plane, relabel: Relabel::new(pairs) }
}

// ─────────────────────────────────────────────────────────────────────
// SLICE 3 — the drain, the route, the conductor.
// ─────────────────────────────────────────────────────────────────────

/// A basis element — an installed class the drain accounts against. Class-only
/// this landing (the self-addressing canonical sigil is N-I2+); `seal` carries
/// the class whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crystal {
    pub class: ChainDelta,
}

impl Crystal {
    /// Seal a class into a crystal (class-only — no canonical-key addressing).
    pub fn seal(class: ChainDelta) -> Self {
        Crystal { class }
    }
}

/// THE BOUNDING RECORD: the accounted image WITH the address of the crystal that
/// accounted it, inscribed at the instant of accounting. Re-adding every image
/// reconstructs the input byte-exact (the image is the proof, untouched).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundingRecord {
    pub image: ChainDelta,
    pub basis_at: usize,
}

/// The DIR-13 per-pair routing of a remainder — a decision about THIS pair's
/// structure, never a corpus statistic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir13Route {
    /// The letters' own axes carry this shape — unlearned, not unreachable; the
    /// K-step loop reduces it when its pieces crystallize.
    CompositionGap,
    /// Content no letter reaches (a gauge direction absent from the source's
    /// world) — mint feedstock, on measured irreducibility only.
    GenerationGap,
    /// The frame's own shape changed — the translation arm's business.
    CategoryError,
}

/// A remainder class, whole at its coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Remainder {
    /// A same-frame chain class with full content and support.
    ChainClass { chain: ChainDelta },
    /// The frame's own shape shifted — typed, never squeezed into a fabricated
    /// same-frame chain.
    FrameShift { from: CellFrame, to: CellFrame },
}

impl Remainder {
    /// The chain content, where this remainder is a chain class.
    pub fn chain(&self) -> Option<&ChainDelta> {
        match self {
            Remainder::ChainClass { chain } => Some(chain),
            Remainder::FrameShift { .. } => None,
        }
    }
}

/// The three verdicts — the ONLY verdict vocabulary of the reduce. No boolean
/// form, no None-miss, no census.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReduceVerdict {
    /// Drained whole. The bounding records carry every drained part in drain
    /// order WITH its source address — re-adding the images reconstructs the
    /// input exactly.
    Congruent { bounding: Vec<BoundingRecord> },
    /// One class through a deduced frame, carried with P⁻¹ (`ObserverFrame::inverse`).
    CongruentThroughP { p: ObserverFrame },
    /// The irreducible part, whole, routed per pair.
    Remainder { remainder: Remainder, route: Dir13Route },
}

/// Exact accounting: every entry of the class matches an identical entry of the
/// residual (site, direction, AND exact coefficient) — sub-multiset, never
/// nearness, never ranked.
fn accounts_exactly(residual: &ChainDelta, class: &ChainDelta) -> bool {
    !class.entries.is_empty()
        && class.entries.iter().all(|needed| {
            residual.entries.iter().any(|held| {
                held.site == needed.site
                    && held.direction == needed.direction
                    && held.coefficient == needed.coefficient
            })
        })
}

/// THE DRAIN: account the residual against the basis by EXACT accounting,
/// terminating by EXHAUSTION (a full pass draining nothing); what survives is
/// the remainder. Each accounted image subtracts (monotone — entries cancel to
/// zero and drop, nothing is created) and inscribes its source address. An
/// EMPTY basis drains nothing — the residual is the whole (the Pin A path).
/// (No refraction / valence routing / support addressing this landing.)
fn drain_exact(mut residual: ChainDelta, basis: &[Crystal]) -> (ChainDelta, Vec<BoundingRecord>) {
    let mut bounding: Vec<BoundingRecord> = Vec::new();
    let mut drained_this_pass = true;
    while drained_this_pass {
        drained_this_pass = false;
        for (basis_at, crystal) in basis.iter().enumerate() {
            if accounts_exactly(&residual, &crystal.class) {
                let image = crystal.class.clone();
                residual = ChainDelta {
                    from: residual.from,
                    to: residual.to,
                    entries: signed_entry_sum(&[(&residual, 1), (&image, -1)]),
                };
                bounding.push(BoundingRecord { image, basis_at });
                drained_this_pass = true;
            }
        }
    }
    (residual, bounding)
}

/// DIR-13, per pair: a residual carrying a gauge direction the SOURCE never
/// expressed is generation feedstock; everything else on the letters' own axes
/// is a composition gap. (The frame-shift arm routes `CategoryError` upstream.)
/// Route on ABSENCE-FROM-SOURCE, never on a literal hue value.
fn route_per_pair(a: &FullMeasurement, residual: &ChainDelta) -> Dir13Route {
    let mut source_directions: Vec<u8> =
        a.occupancy().iter().map(|(_, direction)| direction.index).collect();
    source_directions.sort_unstable();
    source_directions.dedup();
    let novel = residual
        .entries
        .iter()
        .any(|entry| source_directions.binary_search(&entry.direction.index).is_err());
    match novel {
        true => Dir13Route::GenerationGap,
        false => Dir13Route::CompositionGap,
    }
}

/// **THE REDUCE (the conductor).** Canonical-equal pairs resolve through the
/// deduced frame (identity → Congruent, else CongruentThroughP); unequal pairs
/// take the pair's same-frame difference (`signed_remainder`, NOT a second
/// peel), drain it against the basis, and route what survives. Frame-shifting
/// pairs refuse at the difference and route CategoryError. There is no second
/// difference and no candidate loop.
pub fn reduce_pair(a: &FullMeasurement, b: &FullMeasurement, basis: &[Crystal]) -> ReduceVerdict {
    let ca = canonicalize(a);
    let cb = canonicalize(b);
    match deduce_gauge_move(&ca, &cb) {
        GaugeDeduction::GaugeMove { rule } => {
            if rule == identity_frame() {
                ReduceVerdict::Congruent { bounding: Vec::new() }
            } else {
                ReduceVerdict::CongruentThroughP { p: rule }
            }
        }
        GaugeDeduction::ContentDiffers => match signed_remainder(a, b) {
            Err(_) => ReduceVerdict::Remainder {
                remainder: Remainder::FrameShift { from: a.frame(), to: b.frame() },
                route: Dir13Route::CategoryError,
            },
            Ok(whole) => {
                let (residual, bounding) = drain_exact(whole, basis);
                if residual.entries.is_empty() {
                    ReduceVerdict::Congruent { bounding }
                } else {
                    let route = route_per_pair(a, &residual);
                    ReduceVerdict::Remainder {
                        remainder: Remainder::ChainClass { chain: residual },
                        route,
                    }
                }
            }
        },
    }
}
