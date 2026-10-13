//! THE ONE TRANSPORT PAIR (LAW XV) — every upward or downward movement in
//! the engine is this pair applied at a surface.
//!
//! `Transport<F>` carries Φ AND Φ⁻¹ at construction; `Φ·Φ⁻¹ = 1` is
//! verified exact-rational on entry, refused typed as `NotInverse{residual}` with
//! the residual carried whole. The inverse is CARRIED, never derived,
//! never recomputed. A movement (learn, deposit, promote, drain, absorb,
//! exhale, reduce, peel, recall, project — every one of them) is DATA:
//! `Movement { stroke, arity, altitude, axis }` on the one pair — never a
//! suffix, never a clone, never a counterpart function family (01 §2).
//!
//! Ascent is descent read backwards off the carried inverse: `Stroke::Up`
//! applies Φ, `Stroke::Down` applies Φ⁻¹ — one transport, two reads.

// NOTE (IAE-1, netstratum vendor): both `letters` (the Cayley alphabet /
// word composition) and `equivalence` (the represent read) bridge the
// transport pair to the CHAIN carrier (`crate::congruence::ChainDelta`,
// `crate::peel::CellFrame`), which the geometry-first ruling defers to a
// later slice. They land with the chain carrier; the pair itself — Φ with
// its carried Φ⁻¹, composed by the geometric product — stands on
// `substrate` alone, which is exactly what IAE-1 is meant to establish.

use substrate::multivector::{Multivector, MultivectorError, UnityVerdict};
use substrate::rational::Field;
use thiserror::Error;

/// The two reads of the one pair (01 §1): the up stroke applies Φ (1+N),
/// the down stroke applies the carried Φ⁻¹ (1−N). They are not two
/// mechanisms — one transport read in two directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stroke {
    /// Synthesis direction — apply Φ.
    Up,
    /// Recognition direction — apply the carried Φ⁻¹.
    Down,
}

/// The three sovereign content axes of the {S,G,T}+K chart (02 §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Spatial,
    Temporal,
    Gauge,
}

/// A movement is DATA on the one pair — never a mechanism of its own.
/// `arity` is the composition count (how many times the stroke's operator
/// applies); `altitude` and `axis` are the SURFACE ADDRESS the movement
/// fires at (the peel altitude and the chart axis — read by the organ at
/// that surface; the pair itself acts on whole content and is
/// altitude-blind by design). All three are structural bookkeeping
/// (LAW XIV permitted), never algebraic content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub stroke: Stroke,
    pub arity: usize,
    pub altitude: u8,
    pub axis: Axis,
}

/// THE pair: Φ with its exact inverse carried from birth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transport<F: Field> {
    phi: Multivector<F>,
    phi_inverse: Multivector<F>,
}

impl<F: Field> Transport<F> {
    /// The one constructor: both halves presented, `Φ·Φ⁻¹ = 1` verified
    /// exact-rational on entry. A pair that does not close is refused with the
    /// residual `Φ·Φ⁻¹ − 1` carried whole (LAW V — measured, never a bool).
    pub fn try_new(
        phi: Multivector<F>,
        phi_inverse: Multivector<F>,
    ) -> Result<Self, TransportError<F>> {
        // Typed-sum dispatch (LAW I): the closure comparison lives in the
        // substrate's unity_verdict constructor; this surface only routes
        // on the arms — no boolean gate exists here.
        match phi
            .unity_verdict(&phi_inverse)
            .map_err(|source| TransportError::Arithmetic { source })?
        {
            UnityVerdict::Closes => Ok(Self { phi, phi_inverse }),
            UnityVerdict::Open { residual } => Err(TransportError::NotInverse { residual }),
        }
    }

    /// Φ — the up-stroke operator.
    #[inline]
    pub fn phi(&self) -> &Multivector<F> {
        &self.phi
    }

    /// The CARRIED Φ⁻¹ — the down-stroke operator (a field, never derived).
    #[inline]
    pub fn phi_inverse(&self) -> &Multivector<F> {
        &self.phi_inverse
    }

    /// Compose two transports: `Φ = Φ_a·Φ_b`, and the inverse composes
    /// REVERSED (`(AB)⁻¹ = B⁻¹A⁻¹`) — carried through `try_new`, so the
    /// composed pair is re-verified `Φ·Φ⁻¹ = 1` on entry.
    pub fn compose(&self, other: &Self) -> Result<Self, TransportError<F>> {
        let phi = self
            .phi
            .geometric_product(&other.phi)
            .map_err(|source| TransportError::Arithmetic { source })?;
        let phi_inverse = other
            .phi_inverse
            .geometric_product(&self.phi_inverse)
            .map_err(|source| TransportError::Arithmetic { source })?;
        Self::try_new(phi, phi_inverse)
    }

    /// Apply a movement — the stroke's operator composed `arity` times
    /// onto the content by the geometric product. The SAME pair serves
    /// every arity/altitude/axis; the movement only addresses it.
    pub fn apply(
        &self,
        movement: &Movement,
        content: &Multivector<F>,
    ) -> Result<Multivector<F>, TransportError<F>> {
        let operator = match movement.stroke {
            Stroke::Up => &self.phi,
            Stroke::Down => &self.phi_inverse,
        };
        (0..movement.arity).try_fold(content.clone(), |acc, _| {
            operator
                .geometric_product(&acc)
                .map_err(|source| TransportError::Arithmetic { source })
        })
    }
}

/// Typed refusals of the transport surface.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TransportError<F: Field> {
    /// The presented pair does not close: `Φ·Φ⁻¹ − 1` carried whole.
    #[error("the presented pair does not close: Φ·Φ⁻¹ − 1 = {residual:?}")]
    NotInverse { residual: Multivector<F> },

    /// Underlying algebra refusal, propagated typed.
    #[error("transport arithmetic: {source}")]
    Arithmetic {
        #[from]
        source: MultivectorError,
    },
}
