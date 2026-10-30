//! **The congruence READ** (N-I1, doc `53` D2, seat 1 of the one reduction) —
//! over a cover pair (two vantages of one structure), the per-direction
//! relational Δ carried as a whole object: signed exact coefficients at
//! (site, direction), with the re-forming inverse riding along (the seed law —
//! nothing is emitted that cannot re-lift). Re-expressed from v0.4
//! `iae/src/congruence` onto the `CellFrame` carrier (the frame endpoints ride
//! as frames, not a rank-general `SpatialGauge` — matching the arms, thinner
//! than the uncollapsed carrier). No boolean form: the reading is read, never
//! gated.

use crate::peel::{
    peel, CellFrame, Cover, FullMeasurement, GaugeDirection, Site, ValueManifold,
};
use num_bigint::BigInt;
use std::collections::BTreeMap;
use thiserror::Error;

/// One direction's asymmetry between two vantages: the sites that departed (in
/// A, not B) and those that arrived (in B, not A). Content and support are one —
/// the sites are the where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectionAsymmetry {
    pub direction: GaugeDirection,
    pub departed: Vec<Site>,
    pub arrived: Vec<Site>,
}

/// The temporal arm at the single-pair altitude: typed absence. A pair carries
/// no path content; fabricating a zero here would collide with a genuine zero
/// class downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalArm {
    Unmeasured,
}

/// The grade-0 emission: the whole per-axis relational Δ of one cover pair. No
/// boolean accessor exists; the reading is read, never gated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CongruenceReading {
    pub source_frame: CellFrame,
    pub target_frame: CellFrame,
    /// Per-direction gauge asymmetries, canonical order, present only where real.
    pub gauge: Vec<DirectionAsymmetry>,
    pub temporal: TemporalArm,
}

impl CongruenceReading {
    /// The carrier expands its own dimensions and calls `visit` per term:
    /// arrived carries +1, departed carries −1 (the reading as an integer chain).
    pub fn expand_signed_terms(&self, visit: &mut impl FnMut(Site, GaugeDirection, i64)) {
        for asymmetry in &self.gauge {
            for site in &asymmetry.arrived {
                visit(*site, asymmetry.direction, 1);
            }
            for site in &asymmetry.departed {
                visit(*site, asymmetry.direction, -1);
            }
        }
    }

}

/// Read the grade-0 Δ of a cover pair. Frames must agree for the site-conserving
/// read at this altitude (frame-shifting pairs are the affine story — a later
/// organ; refusing typed is honest, a fabricated same-frame Δ is not).
pub fn congruence_read(a: &Cover, b: &Cover) -> Result<CongruenceReading, CongruenceRefusal> {
    if a.cell.frame != b.cell.frame {
        return Err(CongruenceRefusal::FramesDisagree { from: a.cell.frame, to: b.cell.frame });
    }
    let mut directions: Vec<GaugeDirection> = a
        .value
        .supports
        .iter()
        .chain(b.value.supports.iter())
        .map(|s| s.direction)
        .collect();
    directions.sort_by_key(|d| d.index);
    directions.dedup();
    let gauge = directions
        .into_iter()
        .filter_map(|direction| {
            let in_a = support_sites(&a.value, direction);
            let in_b = support_sites(&b.value, direction);
            let departed = difference(in_a, in_b);
            let arrived = difference(in_b, in_a);
            let real = !(departed.is_empty() && arrived.is_empty());
            real.then_some(DirectionAsymmetry { direction, departed, arrived })
        })
        .collect();
    Ok(CongruenceReading {
        source_frame: a.cell.frame,
        target_frame: b.cell.frame,
        gauge,
        temporal: TemporalArm::Unmeasured,
    })
}

fn support_sites(value: &ValueManifold, direction: GaugeDirection) -> &[Site] {
    value
        .supports
        .iter()
        .find(|s| s.direction == direction)
        .map(|s| s.sites.as_slice())
        .unwrap_or(&[])
}

/// Sites of `left` not in `right` (both canonically sorted).
fn difference(left: &[Site], right: &[Site]) -> Vec<Site> {
    left.iter()
        .filter(|site| {
            right
                .binary_search_by_key(&(site.layer, site.row, site.col), |s| {
                    (s.layer, s.row, s.col)
                })
                .is_err()
        })
        .copied()
        .collect()
}

/// One exact integer coefficient at a (site, direction) — the entry of the chain
/// carrier at this grade (where, per axis, never a norm).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainEntry {
    pub site: Site,
    pub direction: GaugeDirection,
    pub coefficient: BigInt,
}

/// The grade-0 Δ in chain form: signed exact coefficients at (site, direction),
/// with both frame endpoints carried. The one chain carrier — its inverse is
/// negation, carried trivially by `negated()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainDelta {
    pub from: CellFrame,
    pub to: CellFrame,
    /// Non-zero entries only, canonical (direction, row, col) order.
    pub entries: Vec<ChainEntry>,
}

impl ChainDelta {
    /// A reading as an integer chain — the reading expands itself into the
    /// ledger's call site (the chain never reaches into the reading's dimensions).
    pub fn from_reading(reading: &CongruenceReading) -> Self {
        let mut ledger = Ledger::new();
        reading.expand_signed_terms(&mut |site, direction, value| {
            ledger.add(site, direction, BigInt::from(value));
        });
        Self { from: reading.source_frame, to: reading.target_frame, entries: ledger.into_entries() }
    }

    /// The empty (identity) chain between one frame and itself.
    pub fn empty(frame: CellFrame) -> Self {
        Self { from: frame, to: frame, entries: Vec::new() }
    }

    /// The chain expands its own entries (each scaled by `sign`) into `visit`.
    pub fn expand_entries(&self, sign: i64, visit: &mut impl FnMut(Site, GaugeDirection, BigInt)) {
        for entry in &self.entries {
            visit(entry.site, entry.direction, &entry.coefficient * BigInt::from(sign));
        }
    }

    /// The carried inverse: endpoints swapped, coefficients negated.
    pub fn negated(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            entries: self
                .entries
                .iter()
                .map(|e| ChainEntry {
                    site: e.site,
                    direction: e.direction,
                    coefficient: -e.coefficient.clone(),
                })
                .collect(),
        }
    }

    /// Path composition: `self: a→b` then `other: b→c` gives `a→c` by exact entry
    /// addition. Refuses typed when the endpoints disagree.
    pub fn compose(&self, other: &Self) -> Result<Self, CongruenceRefusal> {
        if self.to != other.from {
            return Err(CongruenceRefusal::ChainsDoNotCompose);
        }
        Ok(Self { from: self.from, to: other.to, entries: signed_entry_sum(&[(self, 1), (other, 1)]) })
    }
}

/// The accumulation carrier — the call site the chains expand into. Keyed by
/// `(row, col, layer, direction)` in a `BTreeMap`, so accumulation is O(log n) per
/// contribution, not the linear slot-scan it was (a 30×30, 10-hue wall makes
/// ~9000 possible entries — the scan was O(entries²)). `into_entries` sorts the
/// output canonically regardless of insertion order, so this is output-identical
/// (the map's own key order is normalised away — SEAL-B untouched).
pub struct Ledger {
    slots: BTreeMap<(i64, i64, i64, u8), BigInt>,
}

impl Ledger {
    pub fn new() -> Self {
        Self { slots: BTreeMap::new() }
    }

    /// One signed exact contribution at (site, direction). VOLUMETRIC-1a: keyed
    /// on the whole site (layer included) — output-identical for every 2-D
    /// caller, whose sites all carry `layer: 0`.
    pub fn add(&mut self, site: Site, direction: GaugeDirection, value: BigInt) {
        let key = (site.row, site.col, site.layer, direction.index);
        self.slots
            .entry(key)
            .and_modify(|coefficient| *coefficient += &value)
            .or_insert(value);
    }

    /// Close the ledger: non-zero entries only, canonical order.
    pub fn into_entries(self) -> Vec<ChainEntry> {
        let mut entries: Vec<ChainEntry> = self
            .slots
            .into_iter()
            .filter(|(_, coefficient)| coefficient != &BigInt::from(0))
            .map(|((row, col, layer, index), coefficient)| ChainEntry {
                site: Site { row, col, layer },
                direction: GaugeDirection { index },
                coefficient,
            })
            .collect();
        entries.sort_by_key(|e| (e.direction.index, e.site.layer, e.site.row, e.site.col));
        entries
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}

/// The one entry-arithmetic core: each chain expands itself into the ledger.
pub fn signed_entry_sum(terms: &[(&ChainDelta, i64)]) -> Vec<ChainEntry> {
    let mut ledger = Ledger::new();
    for (chain, sign) in terms {
        chain.expand_entries(*sign, &mut |site, direction, value| ledger.add(site, direction, value));
    }
    ledger.into_entries()
}

/// **The pair's same-FRAME asymmetry as A chain** — peel both, read the
/// congruence, cast to the chain carrier. The reduce's difference (stage 3) on
/// this altitude. Frame-shifting pairs refuse typed (`FramesDisagree`).
pub fn signed_remainder(
    a: &FullMeasurement,
    b: &FullMeasurement,
) -> Result<ChainDelta, CongruenceRefusal> {
    let reading = congruence_read(&peel(a), &peel(b))?;
    Ok(ChainDelta::from_reading(&reading))
}

/// Typed refusals of the congruence surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CongruenceRefusal {
    #[error("frames disagree: from {from:?}, to {to:?}")]
    FramesDisagree { from: CellFrame, to: CellFrame },

    #[error("departure from a site the source does not carry: direction {}, site ({}, {}, layer {})", direction.index, site.row, site.col, site.layer)]
    DepartureFromAbsentSite { direction: GaugeDirection, site: Site },

    #[error("the chains do not compose a→b→c")]
    ChainsDoNotCompose,
}
