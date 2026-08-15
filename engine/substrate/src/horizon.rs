// `HorizonContext` — per-region angular-subdivision depth tag, and the
// per-region exact-rational topological thresholds Δ_min(d) / Λ_max(d).
//
// Each topological region attaches its own `HorizonContext`; `dilate()`
// (d → d+1) and `compress()` (d → d−1) execute exact rational refinements
// of the per-region thresholds. `depth` parameterizes the smooth-prime-
// profile-derived thresholds; the thresholds are TOPOLOGICAL BOUNDARIES
// (exact rational functions of `(depth, F)`), never arithmetic-safety caps.
//
// Lifted whole from the sealed v0.3 line (shared_primitives::{horizon,
// bounds} — folded into one module per the 12 §5 tree; serde dropped).

use num_bigint::BigInt;
use num_traits::One;
use thiserror::Error;

use crate::rational::{lcm_smooth_primes, Field, OverflowError, Rational};

/// Per-region active angular-subdivision depth tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HorizonContext {
    depth: u64,
}

/// Substrate-default per-region depth. The substrate's regions
/// start at depth 10; `dilate()` refines (d → d+1), `compress()`
/// coarsens (d → d−1).
pub const HORIZON_DEFAULT_DEPTH: u64 = 10;

impl HorizonContext {
    /// Default horizon. Const-able for callers that need a static
    /// HorizonContext.
    pub const DEFAULT: Self = Self {
        depth: HORIZON_DEFAULT_DEPTH,
    };

    /// Construct with explicit depth. Depth 0 is structurally rejected
    /// — every horizon names a real angular subdivision.
    pub fn new(depth: u64) -> Result<Self, HorizonError> {
        if depth == 0 {
            return Err(HorizonError::DepthOutOfRange {
                observed: 0,
                max: u64::MAX,
            });
        }
        Ok(Self { depth })
    }

    /// Per-region depth. Used by `delta_min` /
    /// `lambda_max` to compute the per-axis exact-rational thresholds at
    /// this context.
    #[inline]
    pub const fn depth(&self) -> u64 {
        self.depth
    }

    /// Refine the region — increase depth by 1 (`d → d+1`).
    /// `Δ_min` shrinks and `Λ_max` grows by a factor of
    /// `lcm_smooth_primes::<F>()` per axis.
    pub fn dilate(&self) -> Result<Self, HorizonError> {
        if self.depth == u64::MAX {
            return Err(HorizonError::DepthOutOfRange {
                observed: self.depth,
                max: u64::MAX,
            });
        }
        Ok(Self {
            depth: self.depth + 1,
        })
    }

    /// Coarsen the region — decrease depth by 1 (`d → d−1`).
    /// Depth 1 cannot be coarsened further (depth 0 is rejected).
    pub fn compress(&self) -> Result<Self, HorizonError> {
        if self.depth <= 1 {
            return Err(HorizonError::DepthOutOfRange {
                observed: self.depth.saturating_sub(1),
                max: u64::MAX,
            });
        }
        Ok(Self {
            depth: self.depth - 1,
        })
    }
}

impl Default for HorizonContext {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Structured residual of a `HorizonContext` construction error.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HorizonError {
    #[error("horizon depth {observed} out of range (must be 1..={max})")]
    DepthOutOfRange { observed: u64, max: u64 },
}

/// The horizon depth as a `pow` exponent. Depth is a `u64` tag with
/// unbounded refinement headroom; a materialized bound at an exponent
/// beyond `u32::MAX` has billions of digits — physically uncomputable,
/// never reached. The exponent clamps; the depth TAG keeps its range.
fn depth_exponent(horizon: HorizonContext) -> u32 {
    u32::try_from(horizon.depth().min(u64::from(u32::MAX)))
        .expect("clamped to u32::MAX — try_from is total")
}

/// Per-region lower bound `Δ_min(F, depth) = 1 / lcm_smooth_primes::<F>()^depth`.
/// Values below this threshold route through the singularity mirror
/// (`crate::mirror`) — inversion, never collapse.
pub fn delta_min<F: Field>(horizon: HorizonContext) -> Result<Rational<F>, OverflowError> {
    let lcm = lcm_smooth_primes::<F>();
    let denom = pow_bigint(&lcm, depth_exponent(horizon));
    Rational::<F>::try_new(BigInt::one(), denom)
}

/// Per-region upper bound `Λ_max(F, depth) = lcm_smooth_primes::<F>()^depth`
/// — the macro envelope.
pub fn lambda_max<F: Field>(horizon: HorizonContext) -> Rational<F> {
    let lcm = lcm_smooth_primes::<F>();
    let value = pow_bigint(&lcm, depth_exponent(horizon));
    Rational::<F>::from_integer(value)
}

/// BigInt power by repeated multiplication — exact, no fixed-width.
fn pow_bigint(base: &BigInt, exp: u32) -> BigInt {
    let mut acc = BigInt::one();
    for _ in 0..exp {
        acc *= base;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_horizon_carries_default_depth() {
        assert_eq!(HorizonContext::default().depth(), HORIZON_DEFAULT_DEPTH);
    }

    #[test]
    fn depth_zero_rejected() {
        assert!(HorizonContext::new(0).is_err());
    }

    #[test]
    fn dilate_increments_depth() {
        let h = HorizonContext::new(5).unwrap();
        assert_eq!(h.dilate().unwrap().depth(), 6);
    }

    #[test]
    fn compress_decrements_depth() {
        let h = HorizonContext::new(5).unwrap();
        assert_eq!(h.compress().unwrap().depth(), 4);
    }

    #[test]
    fn compress_at_one_rejected() {
        let h = HorizonContext::new(1).unwrap();
        assert!(h.compress().is_err());
    }

    #[test]
    fn dilate_at_max_rejected() {
        let h = HorizonContext::new(u64::MAX).unwrap();
        assert!(h.dilate().is_err());
    }

    use crate::rational::{Gauge, Space, Temporal};

    #[test]
    fn delta_min_space_at_depth_1_is_one_thirtieth() {
        let h = HorizonContext::new(1).unwrap();
        let d = delta_min::<Space>(h).unwrap();
        assert_eq!(d.numerator(), &BigInt::from(1));
        assert_eq!(d.denominator(), &BigInt::from(30));
    }

    #[test]
    fn delta_min_temporal_at_depth_1_is_one_two_hundred_tenth() {
        let h = HorizonContext::new(1).unwrap();
        let d = delta_min::<Temporal>(h).unwrap();
        assert_eq!(d.numerator(), &BigInt::from(1));
        assert_eq!(d.denominator(), &BigInt::from(210));
    }

    #[test]
    fn delta_min_gauge_at_depth_1_is_the_gauge_lcm_reciprocal() {
        let h = HorizonContext::new(1).unwrap();
        let d = delta_min::<Gauge>(h).unwrap();
        assert_eq!(d.numerator(), &BigInt::from(1));
        assert_eq!(d.denominator(), &BigInt::from(4290));
    }

    #[test]
    fn lambda_max_space_at_depth_2_grows_quadratically() {
        let h = HorizonContext::new(2).unwrap();
        let l = lambda_max::<Space>(h);
        // 30^2 = 900
        assert_eq!(l.numerator(), &BigInt::from(900));
        assert_eq!(l.denominator(), &BigInt::from(1));
    }

    #[test]
    fn delta_min_and_lambda_max_are_reciprocal_per_field() {
        let h = HorizonContext::default();
        let d = delta_min::<Space>(h).unwrap();
        let l = lambda_max::<Space>(h);
        // d · l == 1 ⟺ numerator-product equals denominator-product.
        let num_product = d.numerator() * l.numerator();
        let den_product = d.denominator() * l.denominator();
        assert_eq!(num_product, den_product);
    }
}
