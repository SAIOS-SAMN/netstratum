//! **THE PEEL** (N-I1, doc `53_INTELLIGENCE_PORT` D2) — the cover. From a full
//! measurement, the three local manifolds: **value** (what transforms — the
//! per-direction occupancy profile over the gauge fiber), **object** (what moves
//! — the 4-adjacent connected bodies), **cell** (the exact 2-D frame). Outer
//! constrains inner. Re-expressed from the v0.4 convergence engine
//! (`iae/src/peel`) onto netstratum carriers — CellFrame-based, exact, no
//! `SpatialGauge`/`Multivector` (the rank-general uncollapsed carrier and the
//! generic nested-peel law are later landings, not N-I1).
//!
//! The peel produces covers; it makes NO judgment — no direction privileged, no
//! ambient region assumed. A full measurement is WHOLE by construction or
//! refused typed: a partial cover cannot exist as a value of this type (the
//! single-row flatten is structurally impossible). The congruence read
//! (`crate::congruence`) takes a `Cover`; there is no path from perception to a
//! reduction that bypasses it.

use thiserror::Error;

/// A site — an element of ℤ² (translations act here).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Site {
    pub row: i64,
    pub col: i64,
}

/// A direction of the gauge fiber — WHICH basis direction (a hue at the ARC
/// seat; a resource index at the survival seat). The index is a label the
/// fiber permutes, never arithmetic content; the fiber rank rides the
/// measurement (the kernel fixes no modality rank).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GaugeDirection {
    pub index: u8,
}

/// The cell manifold's exact 2-D frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellFrame {
    pub rows: u32,
    pub cols: u32,
}

/// A FULL measurement: every site of the frame carried exactly once with its
/// gauge direction, every direction inside the fiber rank — whole by
/// construction or refused typed (a partial cover cannot exist as a value of
/// this type). The COMMON carrier both seats peel into (grid cells → sites +
/// hues; books → sites + resource indices) — never a `Vec<BigInt>`, never a
/// wrapper on a cochain residual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullMeasurement {
    fiber_rank: u8,
    frame: CellFrame,
    occupancy: Vec<(Site, GaugeDirection)>,
}

impl FullMeasurement {
    /// The whole-or-refused constructor. Every comparison lives inside this
    /// verdict constructor: a `FullMeasurement` that exists is whole.
    pub fn try_new(
        fiber_rank: u8,
        frame: CellFrame,
        occupancy: Vec<(Site, GaugeDirection)>,
    ) -> Result<Self, MeasurementRefusal> {
        match occupancy.iter().find(|(_, d)| d.index >= fiber_rank) {
            Some((_, d)) => Err(MeasurementRefusal::DirectionOutsideFiber {
                index: d.index,
                fiber_rank,
            }),
            None => Self::admit_coverage(fiber_rank, frame, occupancy),
        }
    }

    /// Coverage half of the constructor: exactly-once per frame site.
    fn admit_coverage(
        fiber_rank: u8,
        frame: CellFrame,
        occupancy: Vec<(Site, GaugeDirection)>,
    ) -> Result<Self, MeasurementRefusal> {
        let area = (frame.rows as usize)
            .checked_mul(frame.cols as usize)
            .ok_or(MeasurementRefusal::FrameOverflow { frame })?;
        let mut carried: Vec<u8> = vec![0; area];
        for (site, _) in &occupancy {
            let index = frame_index(*site, frame)
                .ok_or(MeasurementRefusal::SiteOutsideFrame { site: *site })?;
            carried[index] = carried[index].saturating_add(1);
        }
        let duplicated = occupancy.iter().find(|(site, _)| {
            frame_index(*site, frame).map(|index| carried[index] > 1).unwrap_or(false)
        });
        let uncarried = carried.iter().position(|count| *count == 0);
        match (duplicated, uncarried) {
            (Some((site, _)), _) => Err(MeasurementRefusal::SiteCarriedTwice { site: *site }),
            (None, Some(index)) => Err(MeasurementRefusal::SiteUncarried {
                site: site_of(index, frame),
            }),
            (None, None) => Ok(Self { fiber_rank, frame, occupancy }),
        }
    }

    #[inline]
    pub fn fiber_rank(&self) -> u8 {
        self.fiber_rank
    }

    #[inline]
    pub fn frame(&self) -> CellFrame {
        self.frame
    }

    /// The whole occupancy — carriage order preserved (a pinned gauge choice).
    #[inline]
    pub fn occupancy(&self) -> &[(Site, GaugeDirection)] {
        &self.occupancy
    }
}

/// A connected body: sites of ONE gauge direction, 4-adjacent — what moves.
/// Sites in canonical (row, col) order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body {
    pub direction: GaugeDirection,
    pub sites: Vec<Site>,
}

/// The object manifold — every body of every direction, none privileged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectManifold {
    pub bodies: Vec<Body>,
}

/// One direction's whole support across the frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectionSupport {
    pub direction: GaugeDirection,
    pub sites: Vec<Site>,
}

/// The value manifold — the occupancy profile over the gauge fiber: one support
/// per PRESENT direction (absent directions carry no support — absence is
/// structural, never a fabricated empty).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueManifold {
    pub supports: Vec<DirectionSupport>,
}

/// The cell manifold — the exact 2-D base.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellManifold {
    pub frame: CellFrame,
}

/// THE COVER — the three manifolds together. The congruence read takes this
/// type; a flat sequence cannot be presented in its place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cover {
    pub fiber_rank: u8,
    pub cell: CellManifold,
    pub object: ObjectManifold,
    pub value: ValueManifold,
}

impl Cover {
    /// **The lift back (the seed law):** re-form the FULL measurement from the
    /// cover alone — the value supports carry every (site, direction) whole; the
    /// constructor re-verifies wholeness. A cover that could not re-form its
    /// measurement would be a projection without its inverse — a key, and banned.
    pub fn re_form_measurement(&self) -> Result<FullMeasurement, MeasurementRefusal> {
        let mut occupancy: Vec<(Site, GaugeDirection)> = self
            .value
            .supports
            .iter()
            .flat_map(|support| support.sites.iter().map(|site| (*site, support.direction)))
            .collect();
        occupancy.sort_by_key(|(site, _)| (site.row, site.col));
        FullMeasurement::try_new(self.fiber_rank, self.cell.frame, occupancy)
    }
}

/// Peel a full measurement into its cover. Pure measurement — no judgment, no
/// transform, no comparison of covers (that is the congruence read's seat).
pub fn peel(measurement: &FullMeasurement) -> Cover {
    let frame = measurement.frame();
    let bodies = connected_bodies(measurement);
    let supports = direction_supports(&bodies);
    Cover {
        fiber_rank: measurement.fiber_rank(),
        cell: CellManifold { frame },
        object: ObjectManifold { bodies },
        value: ValueManifold { supports },
    }
}

/// The frame-lattice index of an in-frame site.
fn frame_index(site: Site, frame: CellFrame) -> Option<usize> {
    let inside = site.row >= 0
        && site.col >= 0
        && site.row < i64::from(frame.rows)
        && site.col < i64::from(frame.cols);
    match inside {
        true => usize::try_from(site.row * i64::from(frame.cols) + site.col).ok(),
        false => None,
    }
}

/// The site of a frame-lattice index.
fn site_of(index: usize, frame: CellFrame) -> Site {
    let position = i64::try_from(index).expect("frames are boundedly small");
    Site {
        row: position / i64::from(frame.cols).max(1),
        col: position % i64::from(frame.cols).max(1),
    }
}

/// The 4-neighbors of a lattice index, staying in frame.
fn neighbors(index: usize, frame: CellFrame) -> Vec<usize> {
    let here = site_of(index, frame);
    [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(dr, dc)| {
            frame_index(Site { row: here.row + dr, col: here.col + dc }, frame)
        })
        .collect()
}

/// The 4-adjacent connected bodies per direction. The flood walks the
/// measurement's own occupancy (the content, never a carrier's dimensions).
fn connected_bodies(measurement: &FullMeasurement) -> Vec<Body> {
    let frame = measurement.frame();
    let area = measurement.occupancy().len();
    // The lattice-ordered directions: direction_at[frame_index] — a full
    // measurement guarantees every slot fills exactly once.
    let mut direction_at: Vec<GaugeDirection> = vec![GaugeDirection { index: 0 }; area];
    for (site, direction) in measurement.occupancy() {
        let index = frame_index(*site, frame).expect("FullMeasurement admits only in-frame sites");
        direction_at[index] = *direction;
    }
    let mut body_of: Vec<Option<usize>> = vec![None; area];
    let mut bodies: Vec<Body> = Vec::new();
    for seed in 0..area {
        if body_of[seed].is_some() {
            continue;
        }
        let direction = direction_at[seed];
        let body_index = bodies.len();
        let mut sites = Vec::new();
        let mut frontier = vec![seed];
        body_of[seed] = Some(body_index);
        while let Some(here) = frontier.pop() {
            sites.push(site_of(here, frame));
            let joiners: Vec<usize> = neighbors(here, frame)
                .into_iter()
                .filter(|n| body_of[*n].is_none() && direction_at[*n] == direction)
                .collect();
            for neighbor in joiners {
                body_of[neighbor] = Some(body_index);
                frontier.push(neighbor);
            }
        }
        sites.sort_by_key(|s| (s.row, s.col));
        bodies.push(Body { direction, sites });
    }
    bodies.sort_by_key(|b| {
        let first = b.sites.first().copied().unwrap_or(Site { row: 0, col: 0 });
        (b.direction.index, first.row, first.col)
    });
    bodies
}

/// Per-direction supports as the exact union of that direction's bodies.
/// Canonical direction order.
fn direction_supports(bodies: &[Body]) -> Vec<DirectionSupport> {
    let mut supports: Vec<DirectionSupport> = Vec::new();
    for body in bodies {
        match supports.iter_mut().find(|s| s.direction == body.direction) {
            Some(support) => support.sites.extend(body.sites.iter().copied()),
            None => supports.push(DirectionSupport {
                direction: body.direction,
                sites: body.sites.clone(),
            }),
        }
    }
    for support in &mut supports {
        support.sites.sort_by_key(|s| (s.row, s.col));
    }
    supports.sort_by_key(|s| s.direction.index);
    supports
}

/// Typed refusals of the measurement/peel surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum MeasurementRefusal {
    #[error("gauge direction {index} outside the fiber rank {fiber_rank}")]
    DirectionOutsideFiber { index: u8, fiber_rank: u8 },

    #[error("frame {frame:?} overflows the lattice")]
    FrameOverflow { frame: CellFrame },

    #[error("site ({0}, {1}) lies outside the frame", site.row, site.col)]
    SiteOutsideFrame { site: Site },

    #[error("site ({0}, {1}) carried twice — a measurement is whole", site.row, site.col)]
    SiteCarriedTwice { site: Site },

    #[error("site ({0}, {1}) never carried — a measurement is whole", site.row, site.col)]
    SiteUncarried { site: Site },
}
