//! Multi-field exact-rational rational arithmetic over `BigRational` (LAW IV).
//!
//! `Rational<F>` carries an exact-rational value tagged by a phantom field `F`.
//! The phantom tag enforces the multi-field discipline at the type level:
//! `Rational<Space>` and `Rational<Temporal>` are DISTINCT TYPES — cross-field
//! arithmetic is a compile-time error. Per-field smooth-prime profile
//! validation is correctness-driven (naming WHICH algebra a value lives in),
//! not a representation cap: backing is unbounded `BigRational`.
//!
//! Lifted whole from the sealed v0.3 line (shared_primitives::rational,
//! B1 landing record) minus the serde derives and the observability hook —
//! neither is on the v0.4 allowlist.

use core::marker::PhantomData;

pub use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use thiserror::Error;

/// A field of rational arithmetic: a human-readable name and the field's
/// smooth-prime profile — every denominator factor must lie in this set.
///
/// Canonical substrate profiles (01 §/13 §1a):
///   * `Space`    → `[2, 3, 5]`
///   * `Temporal` → `[2, 3, 5, 7]`
///   * `Gauge`    → `[2, 3, 5, 11, 13]`
pub trait Field: 'static {
    /// Human-readable name for diagnostics.
    const FIELD_NAME: &'static str;

    /// The field's smooth-prime profile. Every denominator factor must be
    /// in this set.
    const SMOOTH_PRIMES: &'static [u64];
}

/// Marker for the spatial rational field `exact-rational_Space`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Space;

impl Field for Space {
    const FIELD_NAME: &'static str = "ℚ_Space";
    const SMOOTH_PRIMES: &'static [u64] = &[2, 3, 5];
}

/// Marker for the temporal rational field `exact-rational_Time`. The temporal axis owns
/// the prime 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temporal;

impl Field for Temporal {
    const FIELD_NAME: &'static str = "ℚ_Time";
    const SMOOTH_PRIMES: &'static [u64] = &[2, 3, 5, 7];
}

/// Marker for the gauge rational field `exact-rational_Gauge`. The gauge axis owns the
/// primes 11 and 13; 7 is Temporal-exclusive and NOT in this profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gauge;

impl Field for Gauge {
    const FIELD_NAME: &'static str = "ℚ_Gauge";
    const SMOOTH_PRIMES: &'static [u64] = &[2, 3, 5, 11, 13];
}

/// A spatial rational — values living on the spatial manifold.
pub type SpatialRational = Rational<Space>;
/// A temporal rational — values living on the temporal manifold.
pub type TemporalRational = Rational<Temporal>;
/// A gauge rational — values living on the gauge manifold.
pub type GaugeRational = Rational<Gauge>;

/// An exact-rational rational tagged by its field of origin.
///
/// Heap-backed via `num_rational::BigRational` (unbounded `BigInt`
/// num/denom). `Clone` only — not `Copy`.
pub struct Rational<F: Field> {
    value: BigRational,
    _field: PhantomData<fn() -> F>,
}

impl<F: Field> Clone for Rational<F> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _field: PhantomData,
        }
    }
}

impl<F: Field> PartialEq for Rational<F> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<F: Field> Eq for Rational<F> {}

impl<F: Field> core::hash::Hash for Rational<F> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<F: Field> core::fmt::Debug for Rational<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rational")
            .field("field", &F::FIELD_NAME)
            .field("numer", self.value.numer())
            .field("denom", self.value.denom())
            .finish()
    }
}

impl<F: Field> Rational<F> {
    /// **Canonical constructor** — build a `Rational<F>` from
    /// arbitrary-precision `BigInt` operands. Callers wrap integer literals
    /// with `BigInt::from(literal)` explicitly; that explicitness IS the
    /// discipline.
    ///
    /// Returns:
    ///   * `OverflowError::DivisionByZero` if `denominator == 0`.
    ///   * `OverflowError::PrimeProfileViolation` if the gcd-reduced
    ///     denominator carries a prime outside `F::SMOOTH_PRIMES`.
    pub fn try_new(numerator: BigInt, denominator: BigInt) -> Result<Self, OverflowError> {
        if denominator.is_zero() {
            return Err(OverflowError::DivisionByZero {
                dividend: numerator,
            });
        }
        let value = BigRational::new(numerator, denominator);
        Self::validate_smooth_primes(&value)?;
        Ok(Self {
            value,
            _field: PhantomData,
        })
    }

    /// Construct an integer-valued rational (`n / 1`). Always valid.
    pub fn from_integer(n: BigInt) -> Self {
        Self {
            value: BigRational::from_integer(n),
            _field: PhantomData,
        }
    }

    /// The rational zero: `0 / 1`. Always valid.
    pub fn zero() -> Self {
        Self {
            value: BigRational::zero(),
            _field: PhantomData,
        }
    }

    /// The rational one: `1 / 1`. Always valid.
    pub fn one() -> Self {
        Self {
            value: BigRational::one(),
            _field: PhantomData,
        }
    }

    /// True iff the value equals zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// True iff strictly positive.
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    /// True iff strictly negative.
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.value.is_negative()
    }

    /// Borrowed numerator as `&BigInt` — the substrate has NO
    /// lossy-projection boundary.
    #[inline]
    pub fn numerator(&self) -> &BigInt {
        self.value.numer()
    }

    /// Borrowed denominator as `&BigInt`.
    #[inline]
    pub fn denominator(&self) -> &BigInt {
        self.value.denom()
    }

    /// Short alias for `numerator()`.
    #[inline]
    pub fn numer(&self) -> &BigInt {
        self.value.numer()
    }

    /// Short alias for `denominator()`.
    #[inline]
    pub fn denom(&self) -> &BigInt {
        self.value.denom()
    }

    /// Negate (consumes).
    pub fn negate(self) -> Self {
        Self {
            value: -self.value,
            _field: PhantomData,
        }
    }

    /// Borrowed-operand negate.
    pub fn negate_ref(&self) -> Self {
        Self {
            value: -self.value.clone(),
            _field: PhantomData,
        }
    }

    /// Checked addition. Reference operands, owned result; canonicalized
    /// (BigRational gcd-reduces internally) and profile-validated.
    pub fn try_add(&self, other: &Self) -> Result<Self, OverflowError> {
        let sum = &self.value + &other.value;
        Self::validate_smooth_primes(&sum)?;
        Ok(Self {
            value: sum,
            _field: PhantomData,
        })
    }

    /// Checked subtraction.
    pub fn try_sub(&self, other: &Self) -> Result<Self, OverflowError> {
        let diff = &self.value - &other.value;
        Self::validate_smooth_primes(&diff)?;
        Ok(Self {
            value: diff,
            _field: PhantomData,
        })
    }

    /// Checked multiplication.
    pub fn try_mul(&self, other: &Self) -> Result<Self, OverflowError> {
        let prod = &self.value * &other.value;
        Self::validate_smooth_primes(&prod)?;
        Ok(Self {
            value: prod,
            _field: PhantomData,
        })
    }

    /// Checked division. `Err(DivisionByZero)` if `other == 0`.
    pub fn try_div(&self, other: &Self) -> Result<Self, OverflowError> {
        if other.is_zero() {
            return Err(OverflowError::DivisionByZero {
                dividend: self.value.numer().clone(),
            });
        }
        let quot = &self.value / &other.value;
        Self::validate_smooth_primes(&quot)?;
        Ok(Self {
            value: quot,
            _field: PhantomData,
        })
    }

    /// Validate the value's denominator against the field's smooth-prime
    /// profile.
    fn validate_smooth_primes(value: &BigRational) -> Result<(), OverflowError> {
        if F::SMOOTH_PRIMES.is_empty() {
            return Ok(());
        }
        let denom = value.denom();
        // BigRational's denom is always > 0 and gcd-reduced.
        let mut residual = denom.clone();
        for &p in F::SMOOTH_PRIMES {
            let p_big = BigInt::from(p);
            while (&residual % &p_big).is_zero() {
                residual /= &p_big;
            }
        }
        if !residual.is_one() {
            return Err(OverflowError::PrimeProfileViolation {
                observed: denom.clone(),
                residual,
            });
        }
        Ok(())
    }
}

impl<F: Field> PartialOrd for Rational<F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: Field> Ord for Rational<F> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

use num_integer::Integer as _;

/// The least-common-multiple of `F::SMOOTH_PRIMES` — by construction
/// F-smooth, so usable as a `Rational<F>` denominator without
/// `PrimeProfileViolation`.
///
/// Canonical values: Space 30, Temporal 210, Gauge 4290.
pub fn lcm_smooth_primes<F: Field>() -> BigInt {
    let mut acc = BigInt::from(1);
    for &p in F::SMOOTH_PRIMES {
        acc = acc.lcm(&BigInt::from(p));
    }
    acc
}

/// Structured refusals from `Rational<F>` construction and arithmetic.
/// Every variant preserves the coordinate residual of the violation
/// (LAW V) — never a bare bool or message.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OverflowError {
    /// Denominator carries a prime outside the field's smooth profile.
    /// The residual is the remaining factor after dividing out every
    /// permitted prime — the structured coordinate of which prime entered.
    #[error(
        "denominator {observed} carries a prime outside the field's smooth profile \
         (irreducible residual after dividing permitted primes: {residual})"
    )]
    PrimeProfileViolation { observed: BigInt, residual: BigInt },

    /// Division by zero — `dividend` is the numerator that was about to be
    /// divided by zero, carried exact.
    #[error("division by zero: attempted to divide by {dividend} / 0")]
    DivisionByZero { dividend: BigInt },

    /// Cochain ↔ chain coefficient-count mismatch at pairing time —
    /// a structural violation carrying both lengths.
    #[error(
        "cochain ↔ chain length mismatch at pairing: cochain has {cochain_len} coefficients, \
         chain has {chain_len} components"
    )]
    DomainMismatch {
        cochain_len: usize,
        chain_len: usize,
    },

    /// Wire-format buffer shorter than the decoded value required.
    #[error("wire-format buffer too short: needed {needed} bytes, got {actual}")]
    BufferTooShort { needed: usize, actual: usize },

    /// Wire-format sign marker outside {0 = zero, 1 = positive,
    /// 2 = negative} — the observed byte carried as what it is (LAW V:
    /// the residual names its own coordinate, never another refusal's).
    #[error("wire-format sign marker {observed} invalid (0 = zero, 1 = positive, 2 = negative)")]
    InvalidSignMarker { observed: u8 },

    /// Wire-format varint carries content past the 64-bit width at the
    /// named byte offset — refused whole (LAW IV: no silent truncation
    /// of decoded content).
    #[error("wire-format varint exceeds 64 bits at byte offset {at}")]
    VarintTooWide { at: usize },

    /// Wire-format cursor arithmetic exceeded the underlying buffer.
    #[error("wire-format cursor overflow at offset {offset} advancing by {advance}")]
    CursorOverflow { offset: usize, advance: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_admits_smooth_denominator() {
        let r = Rational::<Space>::try_new(BigInt::from(1), BigInt::from(30)).expect("ok");
        assert_eq!(r.denominator(), &BigInt::from(30));
    }

    #[test]
    fn space_rejects_temporal_prime() {
        let err = Rational::<Space>::try_new(BigInt::from(1), BigInt::from(7))
            .expect_err("Space rejects prime 7");
        assert!(matches!(err, OverflowError::PrimeProfileViolation { .. }));
    }

    #[test]
    fn gauge_rejects_prime_7() {
        let err = Rational::<Gauge>::try_new(BigInt::from(1), BigInt::from(7))
            .expect_err("Gauge rejects prime 7 (Temporal-exclusive)");
        assert!(matches!(err, OverflowError::PrimeProfileViolation { .. }));
    }

    #[test]
    fn division_by_zero_carries_dividend() {
        let err = Rational::<Space>::try_new(BigInt::from(5), BigInt::from(0))
            .expect_err("denominator 0 rejected");
        match err {
            OverflowError::DivisionByZero { dividend } => {
                assert_eq!(dividend, BigInt::from(5));
            }
            _ => panic!("expected DivisionByZero, got {err:?}"),
        }
    }

    #[test]
    fn lcm_per_field_matches_smooth_prime_product() {
        assert_eq!(lcm_smooth_primes::<Space>(), BigInt::from(30));
        assert_eq!(lcm_smooth_primes::<Temporal>(), BigInt::from(210));
        assert_eq!(lcm_smooth_primes::<Gauge>(), BigInt::from(4290));
    }

    #[test]
    fn try_add_canonicalizes() {
        let a = Rational::<Space>::try_new(BigInt::from(1), BigInt::from(2)).unwrap();
        let b = Rational::<Space>::try_new(BigInt::from(1), BigInt::from(3)).unwrap();
        let sum = a.try_add(&b).expect("smooth-prime ok");
        // 1/2 + 1/3 = 5/6
        assert_eq!(sum.numerator(), &BigInt::from(5));
        assert_eq!(sum.denominator(), &BigInt::from(6));
    }

    #[test]
    fn arithmetic_accumulates_unbounded() {
        // Multiply Space rationals to grow the denominator into p^N
        // territory. BigRational backing means no overflow.
        let mut acc = Rational::<Space>::one();
        let half = Rational::<Space>::try_new(BigInt::from(1), BigInt::from(2)).unwrap();
        for _ in 0..32 {
            acc = acc.try_mul(&half).expect("smooth-prime ok");
        }
        // acc == 1 / 2^32
        let expected_denom = BigInt::from(2).pow(32);
        assert_eq!(acc.denominator(), &expected_denom);
        assert_eq!(acc.numerator(), &BigInt::from(1));
    }
}
