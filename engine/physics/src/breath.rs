//! The breath organ: exhale/inhale — lossless rank reduction.
//!
//! Exhale NEVER discards: content beyond the exhaled rank moves WHOLE
//! to the typed residual. The full exhale→inhale cycle restores the
//! carrier byte-exact. This is the engine's elasticity primitive — a
//! carrier under pressure breathes down (storage compression, eviction,
//! spoilage staging) and every plane remains exactly re-inhalable.

use crate::axis::AxisVector;

/// An exhaled carrier: the kept content at the exhaled rank plus the
/// residual — everything beyond it, whole. The residual is sufficient
/// for exact inhale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exhaled {
    pub kept: AxisVector,
    /// The out-of-rank coefficients, content intact, in degree order
    /// starting at `kept rank` (exhale never discards planes).
    pub residual: AxisVector,
    /// The rank the carrier breathed out to (the inhale seam).
    pub to_rank: usize,
}

/// Typed refusals of the breath.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreathError {
    /// An exhale to a rank at or above the carrier's own is not an
    /// exhale — the inhale arm exists for that direction.
    NotAnExhale { from: usize, to: usize },
}

/// Exhale — breathe the carrier down to `to_rank`: in-rank planes
/// stay, out-of-rank planes move whole to the residual. Nothing is
/// discarded, ever.
pub fn exhale(carrier: &AxisVector, to_rank: usize) -> Result<Exhaled, BreathError> {
    let from = carrier.components().len();
    if to_rank >= from {
        return Err(BreathError::NotAnExhale { from, to: to_rank });
    }
    let kept = AxisVector::from_vec(carrier.components()[..to_rank].to_vec());
    let residual = AxisVector::from_vec(carrier.components()[to_rank..].to_vec());
    Ok(Exhaled {
        kept,
        residual,
        to_rank,
    })
}

/// Inhale — restore the carrier byte-exact from an exhaled pair: the
/// kept planes re-take degrees `0..to_rank` (zero-padded to the seam
/// if canonicalization stripped trailing zeros), the residual planes
/// re-take their degrees above the seam.
pub fn inhale(exhaled: &Exhaled) -> AxisVector {
    use num_traits::Zero;
    let mut components = exhaled.kept.components().to_vec();
    components.resize(exhaled.to_rank, ratio::Ratio::zero());
    components.extend(exhaled.residual.components().iter().cloned());
    AxisVector::from_vec(components)
}
