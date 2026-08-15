// `SingularityMirror<F>` — the substrate's bounded-horizon singularity
// inversion primitive.
//
// When a coordinate falls below `Δ_min(d)`, the substrate does NOT
// collapse — it inverts the value into the macro envelope via a discrete
// bounded singularity mirror. `invert_below_horizon(x) → Λ_max · x⁻¹`
// per coefficient is an executable function; the operation is structural,
// not a divide-by-zero workaround. The mint (Phase 3) is the ONLY organ
// that fires this crossing on cognitive content (04 §7.4).
//
// Lifted whole from the sealed v0.3 line (shared_primitives::singularity
// → mirror.rs per the 12 §5 tree; B1 record) minus `ResidualReading`,
// whose `TorsionResidual` carrier is not on the Phase-1 allowlist — the
// v0.4 residual carrier is defined at 18 §1 and lands with its organ.
//
// LAW IV exact rational + LAW V (a) per-axis preserved + LAW XIV
// polynomial-per-plane: the singularity mirror operates per-axis on
// `VectorModule<F>` polynomial coordinates.
//
// **Correction 3 (polynomial-degree-N inversion).** The inversion
// operates per-coefficient over the entire polynomial, preserving
// polynomial degree. For input `Σ aᵢ xⁱ`, output has the inverted
// coefficient `Λ_max(F, d) · 1/aᵢ` at every non-zero position and a
// zero at every structural-zero position; the positions of structural
// zeros are preserved as a residual list (per-coefficient structured
// residual). Degree-N inversion is substrate-foundational, not
// deferred; whole-polynomial rejection at the degree-N boundary is
// not part of the surface.

use thiserror::Error;

use crate::horizon::HorizonContext;
use crate::rational::{Field, OverflowError, Rational};
use crate::vector_module::VectorModule;

/// `SingularityMirror<F>` — singularity-inversion primitive at the
/// per-region horizon boundary. The singularity mirror carries the
/// horizon-context that parameterizes `Δ_min` / `Λ_max`; the
/// invert operation reflects below-horizon coordinates into the
/// macro envelope.
pub struct SingularityMirror<F: Field> {
    horizon: HorizonContext,
    _field: core::marker::PhantomData<fn() -> F>,
}

impl<F: Field> Clone for SingularityMirror<F> {
    fn clone(&self) -> Self {
        Self {
            horizon: self.horizon,
            _field: core::marker::PhantomData,
        }
    }
}

impl<F: Field> core::fmt::Debug for SingularityMirror<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SingularityMirror")
            .field("field", &F::FIELD_NAME)
            .field("horizon", &self.horizon)
            .finish()
    }
}

impl<F: Field> SingularityMirror<F> {
    /// Canonical singularity mirror at the default horizon.
    pub const CANONICAL: Self = Self {
        horizon: HorizonContext::DEFAULT,
        _field: core::marker::PhantomData,
    };

    /// Construct at an explicit horizon.
    pub fn at_horizon(horizon: HorizonContext) -> Self {
        Self {
            horizon,
            _field: core::marker::PhantomData,
        }
    }

    /// Borrowed horizon context.
    #[inline]
    pub fn horizon(&self) -> HorizonContext {
        self.horizon
    }

    /// **`invert_below_horizon`** — singularity-inversion map at the
    /// per-region horizon boundary. Operates per-coefficient over the
    /// entire polynomial, preserving polynomial degree.
    ///
    /// For input `x = Σ aᵢ xⁱ` (`VectorModule<F>` with coefficients
    /// `a₀, a₁, …, a_{n−1}`), output is `y = Σ bᵢ xⁱ` where
    /// `bᵢ = Λ_max(F, d) · 1/aᵢ` for `aᵢ ≠ 0` and `bᵢ = 0` for
    /// `aᵢ = 0`. The structural-zero positions are preserved in
    /// `InversionRecord::zero_positions` — that list IS the per-
    /// coefficient structured residual.
    ///
    /// Polynomial degree is preserved: if the highest-degree input
    /// coefficient is non-zero, the highest-degree output coefficient
    /// is non-zero. (If the input had trailing-zero coefficients the
    /// substrate's trim invariant already stripped them upstream.)
    ///
    /// **Errors:**
    /// - `DivisionByZeroValue` — input polynomial is structurally
    ///   empty (`is_zero()`); inversion is not defined.
    pub fn invert_below_horizon(
        &self,
        x: &VectorModule<F>,
    ) -> Result<InversionOutcome<F>, SingularityError> {
        if x.is_zero() {
            return Err(SingularityError::DivisionByZeroValue);
        }
        let trimmed = x.trim_trailing_zeros();
        // THE HORIZON GATE (the vacuity cure — operator-ordered sweep):
        // the mirror's NAME says below-horizon, but no code consulted
        // Δ_min — every value inverted unconditionally, WithinBounds
        // was unconstructible, and the mint's refusal arm downstream
        // could never fire. A coordinate is BELOW the horizon iff some
        // non-zero coefficient's magnitude falls under Δ_min(F, d);
        // a value standing wholly at or above the horizon passes
        // through UNINVERTED (doctrine: coordinates bounce off Δ_min
        // via inversion — only the below-horizon cross the mirror).
        let delta_min_value =
            crate::horizon::delta_min::<F>(self.horizon).map_err(SingularityError::Arithmetic)?;
        let below = trimmed
            .components()
            .iter()
            .any(|coeff| match coeff.is_zero() {
                true => false,
                false => {
                    let magnitude = match coeff.is_negative() {
                        true => coeff.negate_ref(),
                        false => coeff.clone(),
                    };
                    magnitude < delta_min_value
                }
            });
        match below {
            true => {}
            false => return Ok(InversionOutcome::WithinBounds(trimmed)),
        }
        self.reflect(&trimmed).map(InversionOutcome::Inverted)
    }

    /// **`reflect`** — the crossing ASSERTED: per-coefficient inversion
    /// with NO horizon gate. For consumers whose feed is below-horizon
    /// BY CONSTRUCTION, not by magnitude — the mint's obstruction
    /// reached the mirror only after every reduction exhausted (the
    /// drain's own proof of irreducibility: a TOPOLOGICAL below-Δ_min,
    /// carried as exact integer coefficients whose numeric size is a chart
    /// artifact). Coordinate consumers gate through
    /// `invert_below_horizon`; asserting consumers name the crossing
    /// here — one mirror, two doors, neither vacuous.
    pub fn reflect(&self, x: &VectorModule<F>) -> Result<InversionRecord<F>, SingularityError> {
        if x.is_zero() {
            return Err(SingularityError::DivisionByZeroValue);
        }
        let trimmed = x.trim_trailing_zeros();
        let lambda_max_value = crate::horizon::lambda_max::<F>(self.horizon);
        let mut inverted = Vec::with_capacity(trimmed.dim());
        let mut zero_positions = Vec::new();
        // SUPPORT form (LAW XXIII): the walk is over the carrier's OWN
        // cells — content that exists — never a `0..dim` rank range with
        // per-coefficient demand access written by this consumer.
        for (i, coeff) in trimmed.components().iter().enumerate() {
            if coeff.is_zero() {
                // Per-coefficient structured-residual: this position
                // had no content to invert (vacuous singularity).
                zero_positions.push(i);
                inverted.push(Rational::<F>::zero());
            } else {
                let one_over_c = Rational::<F>::one()
                    .try_div(coeff)
                    .map_err(SingularityError::Arithmetic)?;
                let reflected = lambda_max_value
                    .try_mul(&one_over_c)
                    .map_err(SingularityError::Arithmetic)?;
                inverted.push(reflected);
            }
        }
        // VectorModule::from_vec preserves polynomial degree;
        // trim_trailing_zeros at emit per Section 6b.3 minima
        // invariant. Since the highest-degree input coefficient was
        // non-zero (input was already trim), the highest-degree
        // output coefficient is also non-zero (1/c is non-zero), so
        // the trim is a no-op — degree-preserving by construction.
        let coordinates = VectorModule::<F>::from_vec(inverted).trim_trailing_zeros();
        Ok(InversionRecord {
            coordinates,
            zero_positions,
        })
    }
}

/// Typed-sum verdict of an inversion attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InversionOutcome<F: Field> {
    /// Value was already within the per-region horizon; pass-through.
    WithinBounds(VectorModule<F>),
    /// Value was below `Δ_min`; reflected per-coefficient via the
    /// bounded singularity mirror into the macro envelope. The record carries
    /// the inverted polynomial AND the per-coefficient structured
    /// residual (positions of structural zeros).
    Inverted(InversionRecord<F>),
}

/// **`InversionRecord<F>`** — per-coefficient inversion result with
/// structural-residual metadata. The inverted polynomial preserves
/// degree; the `zero_positions` vector lists every input coefficient
/// position that was structurally zero (and therefore could not be
/// inverted — the residual is the list itself).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InversionRecord<F: Field> {
    /// Inverted polynomial coordinates: `Σ bᵢ xⁱ` with
    /// `bᵢ = Λ_max(F, d) · 1/aᵢ` for non-zero positions, zero at
    /// structural-zero positions.
    pub coordinates: VectorModule<F>,
    /// Indices `i` where the input polynomial had `aᵢ = 0`. This
    /// IS the per-coefficient structured residual — empty when the
    /// input had no interior zero coefficients.
    pub zero_positions: Vec<usize>,
}

/// Structured residual of a singularity-inversion operation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SingularityError {
    #[error("singularity inversion: input value is structurally zero")]
    DivisionByZeroValue,

    #[error("singularity inversion arithmetic: {0}")]
    Arithmetic(OverflowError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rational::{Space, SpatialRational};
    use num_bigint::BigInt;

    fn rs(num: i64, denom: i64) -> SpatialRational {
        SpatialRational::try_new(BigInt::from(num), BigInt::from(denom)).unwrap()
    }

    /// A coefficient genuinely BELOW the canonical horizon:
    /// Δ_min(Space, depth 10) = 1/30¹⁰, so 1/30¹¹ is under it (the
    /// old fixtures used 1/30 — far ABOVE the horizon — and only the
    /// unconditionally-inverting vacuous mirror let them pass as
    /// "below"; the horizon gate exposed them).
    fn below_horizon() -> SpatialRational {
        let denom = BigInt::from(30).pow(11);
        SpatialRational::try_new(BigInt::from(1), denom).unwrap()
    }

    /// THE PASS-THROUGH WITNESS (the cured arm): a value standing at
    /// or above Δ_min crosses nothing — WithinBounds carries it whole,
    /// uninverted, and the arm is finally constructible.
    #[test]
    fn within_bounds_passes_through_uninverted() {
        let mirror = SingularityMirror::<Space>::CANONICAL;
        let x = VectorModule::<Space>::from_vec(vec![rs(1, 30), rs(1, 5)]);
        match mirror.invert_below_horizon(&x).expect("ok") {
            InversionOutcome::WithinBounds(kept) => {
                assert_eq!(kept, x.trim_trailing_zeros(), "carried whole, uninverted");
            }
            InversionOutcome::Inverted(_) => {
                panic!("a value above the horizon must not cross the mirror")
            }
        }
    }

    #[test]
    fn invert_degree_zero_singleton_below_horizon() {
        let mirror = SingularityMirror::<Space>::CANONICAL;
        // SINGLETON-BOUNDARY [test fixture]: degree-0 below-horizon
        // coordinate at K=0 by construction — test exercises
        // singularity inversion. Polynomial growth begins at any
        // subsequent algebraic operation against the inverted result;
        // this site is the seed, not a collapse.
        let x = VectorModule::<Space>::singleton(below_horizon());
        let outcome = mirror.invert_below_horizon(&x).expect("ok");
        match outcome {
            InversionOutcome::Inverted(rec) => {
                assert_eq!(rec.coordinates.dim(), 1);
                assert!(rec.zero_positions.is_empty());
            }
            _ => panic!("expected Inverted"),
        }
    }

    #[test]
    fn zero_input_rejected() {
        let mirror = SingularityMirror::<Space>::CANONICAL;
        let z = VectorModule::<Space>::zero_polynomial();
        let err = mirror.invert_below_horizon(&z).expect_err("zero rejected");
        assert!(matches!(err, SingularityError::DivisionByZeroValue));
    }

    /// **Correction 3 acceptance test.** A degree-N polynomial
    /// (`[a₀, a₁, …, a_{n−1}]` with every coefficient non-zero)
    /// inverts coefficient-by-coefficient, preserving degree.
    #[test]
    fn polynomial_degree_n_inversion_preserves_degree_per_coefficient() {
        let mirror = SingularityMirror::<Space>::CANONICAL;
        // Three non-zero coefficients → degree-2 polynomial.
        let p = VectorModule::<Space>::from_vec(vec![below_horizon(), rs(1, 15), rs(1, 5)]);
        let input_dim = p.dim();
        let outcome = mirror.invert_below_horizon(&p).expect("ok");
        match outcome {
            InversionOutcome::Inverted(rec) => {
                // Polynomial degree preserved.
                assert_eq!(
                    rec.coordinates.dim(),
                    input_dim,
                    "polynomial degree preserved through inversion"
                );
                // Every output coefficient is non-zero (no structural
                // residual recorded).
                assert!(rec.zero_positions.is_empty());
                for i in 0..rec.coordinates.dim() {
                    assert!(
                        !rec.coordinates.coefficient_at(i).is_zero(),
                        "coefficient {i} non-zero",
                    );
                }
            }
            _ => panic!("expected Inverted"),
        }
    }

    /// **Correction 3 acceptance test.** A polynomial with a zero
    /// coefficient at an interior position surfaces the position as
    /// a per-coefficient structured residual.
    #[test]
    fn zero_coefficient_surfaces_structured_residual() {
        let mirror = SingularityMirror::<Space>::CANONICAL;
        // [a₀=1/30, a₁=0, a₂=1/5] — interior zero at position 1.
        let p = VectorModule::<Space>::from_vec(vec![below_horizon(), rs(0, 1), rs(1, 5)]);
        let outcome = mirror.invert_below_horizon(&p).expect("ok");
        match outcome {
            InversionOutcome::Inverted(rec) => {
                // Degree preserved (3 coefficient positions).
                assert_eq!(rec.coordinates.dim(), 3);
                // Position 1 recorded as structural-zero residual.
                assert_eq!(rec.zero_positions, vec![1]);
                // Position 1 output coefficient is zero (structurally
                // preserved); positions 0 and 2 are inverted.
                assert!(rec.coordinates.coefficient_at(1).is_zero());
                assert!(!rec.coordinates.coefficient_at(0).is_zero());
                assert!(!rec.coordinates.coefficient_at(2).is_zero());
            }
            _ => panic!("expected Inverted"),
        }
    }

    /// **Correction 3 acceptance test.** Composing singularity
    /// inversion with polynomial convolution at the horizon boundary
    /// preserves polynomial degree end-to-end (convolution grows
    /// degree, inversion preserves it; verify the composition does
    /// not collapse). (Renamed at the rotor-vocabulary sweep
    /// 2026-06-11 — the body always used `convolve`; the old name was
    /// a ghost.)
    #[test]
    fn inversion_composes_with_convolution_at_horizon_boundary_without_collapse() {
        let mirror = SingularityMirror::<Space>::CANONICAL;
        // Below-horizon multi-coefficient polynomial.
        let p = VectorModule::<Space>::from_vec(vec![below_horizon(), rs(2, 15)]);
        let outcome = mirror.invert_below_horizon(&p).expect("ok");
        let inverted = match outcome {
            InversionOutcome::Inverted(rec) => rec.coordinates,
            _ => panic!("expected Inverted"),
        };
        // The inverted polynomial admits per-coefficient algebraic
        // operations (e.g., convolve with itself) without collapsing.
        let squared = inverted.convolve(&inverted).expect("ok");
        // Degree grows: (n + n − 1) = 2n − 1 for non-degenerate
        // convolution. Input degree 2 ⇒ output degree 3.
        assert_eq!(squared.dim(), 2 * inverted.dim() - 1);
    }
}
