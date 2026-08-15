//! `Volume` — the {S,G,T} block-diagonal volume carrier, and
//! `BlockDiagonalOperator` — the algebraic guarantor of per-axis
//! preservation.
//!
//! One `VectorModule<F>` per sovereign manifold:
//!
//!   spatial:  VectorModule<Space>    polynomial over exact-rational_Space
//!   temporal: VectorModule<Temporal> polynomial over exact-rational_Time
//!   gauge:    VectorModule<Gauge>    polynomial over exact-rational_Gauge
//!
//! Cross-field arithmetic is impossible at the type system. There is no
//! flattening method — the algebra has no homomorphism from `Volume` to a
//! single the exact rationals (LAW XIV; 05 App. B form 4). Every transformation on the triple
//! is STRICTLY BLOCK-DIAGONAL: `U_total = diag(U_Space, U_Time, U_Gauge)`,
//! each block acting only on its own axis — off-diagonal coupling does not
//! exist as a constructor, so it is topologically forbidden at the type
//! level.
//!
//! Lifted whole from the sealed v0.3 line (shared_primitives::
//! {polynomial_triple, block_diagonal}; B1 record) minus serde and minus
//! `scale_torsion` (its `TorsionResidual` carrier is not on the Phase-1
//! allowlist). The v0.3 anchor name `PolynomialTriple` becomes `Volume` —
//! the v0.4 corpus names this carrier V (13 §1a).

use crate::rational::{
    Field, Gauge, GaugeRational, OverflowError, Space, SpatialRational, Temporal, TemporalRational,
};
use crate::vector_module::{VectorModule, VectorModuleError};
use crate::wire::{read_bigint_le, write_bigint_le};

/// Map a `VectorModuleError` from polynomial add/sub back to the canonical
/// `OverflowError`. `try_add`/`try_sub` use no axis-indexed access; the
/// `AxisOutOfBounds` arm is unreachable.
fn unwrap_volume_overflow(err: VectorModuleError) -> OverflowError {
    match err {
        VectorModuleError::Arithmetic { source } => source,
        VectorModuleError::AxisOutOfBounds { .. } => panic!(
            "Volume polynomial ops use only try_add/try_sub (no axis-indexed access); \
             AxisOutOfBounds arm is unreachable — substrate invariant break",
        ),
    }
}

/// Per-axis polynomial triple — one `VectorModule<F>` per sovereign
/// manifold. The canonical carrier for any per-axis measurement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Volume {
    pub spatial: VectorModule<Space>,
    pub temporal: VectorModule<Temporal>,
    pub gauge: VectorModule<Gauge>,
}

impl Volume {
    /// Construct from explicit per-axis `VectorModule<F>` components.
    pub fn new(
        spatial: VectorModule<Space>,
        temporal: VectorModule<Temporal>,
        gauge: VectorModule<Gauge>,
    ) -> Self {
        Self {
            spatial,
            temporal,
            gauge,
        }
    }

    /// The zero element: every axis at the empty polynomial form.
    pub fn zero() -> Self {
        Self {
            spatial: VectorModule::<Space>::zero_polynomial(),
            temporal: VectorModule::<Temporal>::zero_polynomial(),
            gauge: VectorModule::<Gauge>::zero_polynomial(),
        }
    }

    /// **Unity** — every axis carries the constant polynomial `1` (the
    /// per-field algebraic identity; SINGLETON-BOUNDARY class 2 at each
    /// per-axis singleton constructor — growth begins at the first
    /// convolve/add in a consumer).
    pub fn unity() -> Self {
        Self {
            spatial: VectorModule::<Space>::singleton(SpatialRational::one()),
            temporal: VectorModule::<Temporal>::singleton(TemporalRational::one()),
            gauge: VectorModule::<Gauge>::singleton(GaugeRational::one()),
        }
    }

    /// True iff every per-axis polynomial equals the constant `1/1` in its
    /// own field.
    pub fn is_unity(&self) -> bool {
        let unity = Self::unity();
        self == &unity
    }

    /// True iff every per-axis polynomial is the field's zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.spatial.is_zero() && self.temporal.is_zero() && self.gauge.is_zero()
    }

    /// Component-wise polynomial addition — each axis sums independently in
    /// its own field; cross-field arithmetic is impossible at the type level.
    pub fn try_add(&self, other: &Self) -> Result<Self, OverflowError> {
        Ok(Self {
            spatial: self
                .spatial
                .try_add(&other.spatial)
                .map_err(unwrap_volume_overflow)?,
            temporal: self
                .temporal
                .try_add(&other.temporal)
                .map_err(unwrap_volume_overflow)?,
            gauge: self
                .gauge
                .try_add(&other.gauge)
                .map_err(unwrap_volume_overflow)?,
        })
    }

    /// Component-wise polynomial subtraction.
    pub fn try_sub(&self, other: &Self) -> Result<Self, OverflowError> {
        Ok(Self {
            spatial: self
                .spatial
                .try_sub(&other.spatial)
                .map_err(unwrap_volume_overflow)?,
            temporal: self
                .temporal
                .try_sub(&other.temporal)
                .map_err(unwrap_volume_overflow)?,
            gauge: self
                .gauge
                .try_sub(&other.gauge)
                .map_err(unwrap_volume_overflow)?,
        })
    }

    /// Negate per-axis (polynomial negation).
    pub fn negate(&self) -> Self {
        Self {
            spatial: self.spatial.negate(),
            temporal: self.temporal.negate(),
            gauge: self.gauge.negate(),
        }
    }

    /// Wire format (bounded-binary seed, 06 §5). Per-axis Space → Temporal
    /// → Gauge; each axis = `[u32 LE dim_F][dim_F × (numer, denom)]`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(48);
        write_vector_module(&mut buf, &self.spatial);
        write_vector_module(&mut buf, &self.temporal);
        write_vector_module(&mut buf, &self.gauge);
        buf
    }

    /// Deserialize from the wire format. Returns `(value, bytes_consumed)`;
    /// per-field smooth-prime validation runs inside each `Rational::try_new`.
    pub fn from_bytes_with_len(bytes: &[u8]) -> Result<(Self, usize), OverflowError> {
        let mut cursor = 0;
        let (spatial, n) = read_vector_module::<Space>(&bytes[cursor..])?;
        cursor = cursor.checked_add(n).ok_or(OverflowError::CursorOverflow {
            offset: cursor,
            advance: n,
        })?;
        let (temporal, n) = read_vector_module::<Temporal>(&bytes[cursor..])?;
        cursor = cursor.checked_add(n).ok_or(OverflowError::CursorOverflow {
            offset: cursor,
            advance: n,
        })?;
        let (gauge, n) = read_vector_module::<Gauge>(&bytes[cursor..])?;
        cursor = cursor.checked_add(n).ok_or(OverflowError::CursorOverflow {
            offset: cursor,
            advance: n,
        })?;
        Ok((
            Self {
                spatial,
                temporal,
                gauge,
            },
            cursor,
        ))
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::zero()
    }
}

/// Write a `VectorModule<F>` polynomial as `[u32 LE dim][dim × (numer, denom)]`.
fn write_vector_module<F: Field>(buf: &mut Vec<u8>, value: &VectorModule<F>) {
    let dim_u32 = u32::try_from(value.dim()).expect(
        "Volume VectorModule polynomial dim fits in u32: substrate K-step bounded growth \
         produces <<2^32 coefficients per polynomial",
    );
    buf.extend_from_slice(&dim_u32.to_le_bytes());
    for coeff in value.components() {
        write_bigint_le(buf, coeff.numer());
        write_bigint_le(buf, coeff.denom());
    }
}

/// Read a `VectorModule<F>` polynomial from the canonical layout.
fn read_vector_module<F: Field>(bytes: &[u8]) -> Result<(VectorModule<F>, usize), OverflowError> {
    if bytes.len() < 4 {
        return Err(OverflowError::BufferTooShort {
            needed: 4,
            actual: bytes.len(),
        });
    }
    let dim = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    let mut cursor: usize = 4;
    let mut coefficients = Vec::with_capacity(dim);
    for _ in 0..dim {
        let (numer, end) = read_bigint_le(&bytes[cursor..])?;
        cursor = cursor
            .checked_add(end)
            .ok_or(OverflowError::CursorOverflow {
                offset: cursor,
                advance: end,
            })?;
        let (denom, end) = read_bigint_le(&bytes[cursor..])?;
        cursor = cursor
            .checked_add(end)
            .ok_or(OverflowError::CursorOverflow {
                offset: cursor,
                advance: end,
            })?;
        coefficients.push(crate::rational::Rational::<F>::try_new(numer, denom)?);
    }
    Ok((VectorModule::<F>::from_vec(coefficients), cursor))
}

/// Block-diagonal per-axis transformation witness. Three per-axis
/// operations, each on its own field's `VectorModule<F>` — cross-axis
/// coupling is structurally impossible: the generic `Fn` bounds enforce
/// same-field input/output. The block-diagonal discipline IS the type
/// signature.
#[allow(clippy::type_complexity)]
pub struct BlockDiagonalOperator<S, T, G>
where
    S: Fn(&VectorModule<Space>) -> Result<VectorModule<Space>, OverflowError>,
    T: Fn(&VectorModule<Temporal>) -> Result<VectorModule<Temporal>, OverflowError>,
    G: Fn(&VectorModule<Gauge>) -> Result<VectorModule<Gauge>, OverflowError>,
{
    spatial: S,
    temporal: T,
    gauge: G,
}

impl<S, T, G> BlockDiagonalOperator<S, T, G>
where
    S: Fn(&VectorModule<Space>) -> Result<VectorModule<Space>, OverflowError>,
    T: Fn(&VectorModule<Temporal>) -> Result<VectorModule<Temporal>, OverflowError>,
    G: Fn(&VectorModule<Gauge>) -> Result<VectorModule<Gauge>, OverflowError>,
{
    /// Construct from three per-axis transformations.
    pub fn new(spatial: S, temporal: T, gauge: G) -> Self {
        Self {
            spatial,
            temporal,
            gauge,
        }
    }

    /// Apply the three per-axis transformations to a `Volume`. The type
    /// system prevents any cross-axis substitution at the call site.
    pub fn apply(&self, volume: &Volume) -> Result<Volume, OverflowError> {
        Ok(Volume {
            spatial: (self.spatial)(&volume.spatial)?,
            temporal: (self.temporal)(&volume.temporal)?,
            gauge: (self.gauge)(&volume.gauge)?,
        })
    }
}

/// The identity block-diagonal operator — per-axis content passes through
/// unchanged.
#[allow(clippy::type_complexity)]
pub fn identity() -> BlockDiagonalOperator<
    impl Fn(&VectorModule<Space>) -> Result<VectorModule<Space>, OverflowError>,
    impl Fn(&VectorModule<Temporal>) -> Result<VectorModule<Temporal>, OverflowError>,
    impl Fn(&VectorModule<Gauge>) -> Result<VectorModule<Gauge>, OverflowError>,
> {
    BlockDiagonalOperator::new(
        |v: &VectorModule<Space>| Ok(v.clone()),
        |v: &VectorModule<Temporal>| Ok(v.clone()),
        |v: &VectorModule<Gauge>| Ok(v.clone()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn zero_round_trips_through_wire() {
        let z = Volume::zero();
        let bytes = z.to_bytes();
        let (back, consumed) = Volume::from_bytes_with_len(&bytes).expect("ok");
        assert_eq!(back, z);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn unity_is_unity_per_field() {
        let u = Volume::unity();
        assert!(u.is_unity());
        let c0_s = u.spatial.coefficient_at(0);
        let c0_t = u.temporal.coefficient_at(0);
        let c0_g = u.gauge.coefficient_at(0);
        assert!(c0_s.numerator() == &BigInt::from(1) && c0_s.denominator() == &BigInt::from(1));
        assert!(c0_t.numerator() == &BigInt::from(1) && c0_t.denominator() == &BigInt::from(1));
        assert!(c0_g.numerator() == &BigInt::from(1) && c0_g.denominator() == &BigInt::from(1));
    }

    #[test]
    fn add_per_axis_independent_fields() {
        let half_s = VectorModule::<Space>::from_vec(vec![SpatialRational::try_new(
            BigInt::from(1),
            BigInt::from(2),
        )
        .unwrap()]);
        let third_s = VectorModule::<Space>::from_vec(vec![SpatialRational::try_new(
            BigInt::from(1),
            BigInt::from(3),
        )
        .unwrap()]);
        let a = Volume::new(
            half_s,
            VectorModule::<Temporal>::zero_polynomial(),
            VectorModule::<Gauge>::zero_polynomial(),
        );
        let b = Volume::new(
            third_s,
            VectorModule::<Temporal>::zero_polynomial(),
            VectorModule::<Gauge>::zero_polynomial(),
        );
        let sum = a.try_add(&b).expect("ok");
        // 1/2 + 1/3 = 5/6 on the spatial axis only
        let c0 = sum.spatial.coefficient_at(0);
        assert_eq!(c0.numerator(), &BigInt::from(5));
        assert_eq!(c0.denominator(), &BigInt::from(6));
        assert!(sum.temporal.is_zero());
        assert!(sum.gauge.is_zero());
    }

    #[test]
    fn identity_operator_preserves_per_axis_content() {
        let op = identity();
        let v = Volume::new(
            VectorModule::<Space>::from_vec(vec![SpatialRational::try_new(
                BigInt::from(1),
                BigInt::from(2),
            )
            .unwrap()]),
            VectorModule::<Temporal>::from_vec(vec![TemporalRational::try_new(
                BigInt::from(1),
                BigInt::from(7),
            )
            .unwrap()]),
            VectorModule::<Gauge>::from_vec(vec![GaugeRational::try_new(
                BigInt::from(1),
                BigInt::from(11),
            )
            .unwrap()]),
        );
        let out = op.apply(&v).expect("ok");
        assert_eq!(out, v);
    }

    #[test]
    fn block_diagonal_negate_per_axis() {
        let op = BlockDiagonalOperator::new(
            |v: &VectorModule<Space>| Ok(v.negate()),
            |v: &VectorModule<Temporal>| Ok(v.negate()),
            |v: &VectorModule<Gauge>| Ok(v.negate()),
        );
        // SINGLETON-BOUNDARY [test fixture]: degree-0 per-axis seeds at K=0
        // by construction — growth begins at any production convolve.
        let v = Volume::new(
            VectorModule::<Space>::singleton(
                SpatialRational::try_new(BigInt::from(3), BigInt::from(5)).unwrap(),
            ),
            VectorModule::<Temporal>::singleton(
                TemporalRational::try_new(BigInt::from(2), BigInt::from(7)).unwrap(),
            ),
            VectorModule::<Gauge>::singleton(
                GaugeRational::try_new(BigInt::from(1), BigInt::from(13)).unwrap(),
            ),
        );
        let out = op.apply(&v).expect("ok");
        assert_eq!(out.spatial.coefficient_at(0).numerator(), &BigInt::from(-3));
        assert_eq!(
            out.temporal.coefficient_at(0).numerator(),
            &BigInt::from(-2)
        );
        assert_eq!(out.gauge.coefficient_at(0).numerator(), &BigInt::from(-1));
    }

    // The structural witness: cross-axis arithmetic is rejected by the type
    // system — a test feeding VectorModule<Temporal> into the spatial slot
    // cannot even be written. The absence is a property of the signatures.
    #[allow(dead_code, clippy::type_complexity)]
    fn structural_no_cross_axis_witness(
        op: BlockDiagonalOperator<
            impl Fn(&VectorModule<Space>) -> Result<VectorModule<Space>, OverflowError>,
            impl Fn(&VectorModule<Temporal>) -> Result<VectorModule<Temporal>, OverflowError>,
            impl Fn(&VectorModule<Gauge>) -> Result<VectorModule<Gauge>, OverflowError>,
        >,
        v: &Volume,
    ) {
        let _ = op.apply(v);
    }
}
