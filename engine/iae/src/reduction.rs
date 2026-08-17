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
    #[error("frame application broke measurement wholeness: {0:?}")]
    Measurement(MeasurementRefusal),
}
