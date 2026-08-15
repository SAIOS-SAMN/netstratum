// `VectorModule<F>` — the polynomial-per-plane carrier (FOUNDATIONAL).
//
// Each axis of every per-axis structure carries a polynomial
// `Σ_{i=0..N-1} c_i x^i` with `c_i ∈ Rational<F>`, stored as a
// heap-backed `Vec<Rational<F>>` with NO static dimension cap. The
// substrate's motion drives `N` upward through transport composition
// (denominator growth in `p^N` per smooth-prime profile) and through
// coboundary reduction. The polynomial degree is structural content;
// truncation at any consumer is a LAW V per-axis preservation violation.
//
// Lifted whole from the sealed v0.3 line (shared_primitives::
// vector_module; B1 record) minus serde and minus `twist`/
// `value_multiset_eq` (their carriers/consumers are not on the Phase-1
// allowlist — they return with the organs that need them).
//
// LAW IV exact rational: each coefficient is `Rational<F>` (BigRational-backed,
// unbounded). LAW XIV no scalars on derivation path: `VectorModule<F>`
// is the substrate's carrier for "a per-axis value at any derivation
// site"; producers and consumers operate on the full polynomial via
// element-wise add + convolve.
//
// Block-diagonal type discipline: `VectorModule<Space>` and
// `VectorModule<Temporal>` are DISTINCT TYPES. Cross-field arithmetic
// is a compile-time error. Inter-field crossing routes through
// `FieldIsomorphism<F1, F2>` at a node vertex (deferred to #62.2).
//
// Primitive transformations on the inner pocket:
//   * `stretch` — multiply each component by a field-typed scalar
//                 (bounded radial scaling: `c · Σ aᵢ xⁱ`).
//   * `bend`    — apply a shear between two component indices
//                 `components[i] += magnitude · components[j]`.
//   (`twist` — defect accumulation — stays behind with its InversePiMetric
//   carrier until the dynamics organ that consumes it lands; B1 record.)
//
// Polynomial arithmetic:
//   * `try_add` / `try_sub` — polynomial addition / subtraction with
//                              auto-pad-shorter-with-zeros semantics.
//                              Result length = max of operand lengths.
//   * `convolve`            — polynomial multiplication
//                              `(Σ aᵢ xⁱ) · (Σ bⱼ xʲ) =
//                               Σₖ (Σᵢ aᵢ · bₖ₋ᵢ) xᵏ`.
//                              Result length = m + n − 1 for non-zero
//                              operands. THE primitive that produces
//                              `p^N` denominator growth from per-axis
//                              torsion accumulation.
//
// All operations exact rational. Per-coefficient overflow propagates as
// `VectorModuleError::Arithmetic`.

use core::marker::PhantomData;

use crate::rational::{BigInt, Field, OverflowError, Rational};

/// Polynomial-per-plane carrier living entirely within one field `F`.
/// Heap-backed `Vec<Rational<F>>`; no fixed cap on polynomial degree.
pub struct VectorModule<F: Field> {
    components: Vec<Rational<F>>,
    _field: PhantomData<fn() -> F>,
}

impl<F: Field> Clone for VectorModule<F> {
    fn clone(&self) -> Self {
        Self {
            components: self.components.clone(),
            _field: PhantomData,
        }
    }
}

impl<F: Field> core::fmt::Debug for VectorModule<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VectorModule")
            .field("field", &F::FIELD_NAME)
            .field("components", &self.components)
            .finish()
    }
}

impl<F: Field> PartialEq for VectorModule<F> {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl<F: Field> Eq for VectorModule<F> {}

impl<F: Field> core::hash::Hash for VectorModule<F> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.components.hash(state);
    }
}

impl<F: Field> PartialOrd for VectorModule<F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: Field> Ord for VectorModule<F> {
    /// Lexicographic component-wise ordering over the polynomial
    /// coefficient vector.
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.components.cmp(&other.components)
    }
}

impl<F: Field> VectorModule<F> {
    /// The algebraic zero polynomial — an empty coefficient vector.
    /// Substrate-canonical "no content" marker.
    pub fn zero_polynomial() -> Self {
        Self {
            components: Vec::new(),
            _field: PhantomData,
        }
    }

    /// All-zero polynomial of explicit dimension `dim`. Infallible.
    pub fn zero(dim: usize) -> Self {
        Self {
            components: vec![Rational::<F>::zero(); dim],
            _field: PhantomData,
        }
    }

    /// Build from a slice of coefficients (low-to-high degree).
    pub fn from_slice(values: &[Rational<F>]) -> Self {
        Self {
            components: values.to_vec(),
            _field: PhantomData,
        }
    }

    /// Build by owning a `Vec<Rational<F>>` of coefficients
    /// (low-to-high degree).
    pub fn from_vec(components: Vec<Rational<F>>) -> Self {
        Self {
            components,
            _field: PhantomData,
        }
    }

    /// **THE LIFT SURFACE** — wrap a single `Rational<F>` value as a
    /// degree-0 polynomial (a single-component coefficient vector).
    ///
    /// Per LAW XIV no-scalars-on-derivation-path, this constructor is
    /// the SOLE entry point that lifts a scalar into the polynomial
    /// carrier. Every call site MUST carry a SINGLETON-BOUNDARY
    /// comment naming the boundary class and the specific operation
    /// that will produce polynomial growth from this seed:
    ///
    /// ```text
    /// // SINGLETON-BOUNDARY [<tag>]: <axis> coefficient at K=0 is degree-0
    /// // by construction — <enforcing invariant>. Polynomial growth begins
    /// // at <SPECIFIC operation name>; this site is the seed, not a collapse.
    /// let seed = VectorModule::<F>::singleton(scalar);
    /// ```
    ///
    /// Three legitimate boundary classes:
    /// 1. K=0 initial-state constructors (origin / unity / zero / identity)
    /// 2. Algebraic identity values
    /// 3. Wire-format decode of declared-single-coefficient values at
    ///    versioned IPC boundary
    ///
    /// Anything else is dimensional collapse and fails the #62.1
    /// acceptance grep against `VectorModule::singleton` call sites.
    pub fn singleton(value: Rational<F>) -> Self {
        Self {
            components: vec![value],
            _field: PhantomData,
        }
    }

    /// Polynomial degree-plus-one — length of the coefficient vector.
    /// (A truly empty `zero_polynomial()` returns 0; a constant
    /// polynomial returns 1; a degree-N polynomial returns N+1.)
    #[inline]
    pub fn dim(&self) -> usize {
        self.components.len()
    }

    /// Slice view of the coefficients (low-to-high degree).
    #[inline]
    /// THE WIRE FORM at the storage boundary — the carrier OWNS its expansion
    /// loop (LAW XXIII): count, then per component the numerator and
    /// denominator as length-prefixed signed-LE bytes. Exact-the exact rationals, no loss.
    pub fn to_wire(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let count = u32::try_from(self.components.len())
            .expect("a carrier's width fits the wire count at the storage boundary");
        buf.extend_from_slice(&count.to_le_bytes());
        for component in &self.components {
            for half in [component.numer(), component.denom()] {
                let bytes = half.to_signed_bytes_le();
                let len = u32::try_from(bytes.len()).expect("a bounded wire length");
                buf.extend_from_slice(&len.to_le_bytes());
                buf.extend_from_slice(&bytes);
            }
        }
        buf
    }

    /// Re-lift from the wire, advancing `at`; every component re-validates
    /// through `try_new` (the field profile re-checked at the trust
    /// boundary). `None` = torn or off-profile bytes (the caller names it).
    pub fn from_wire(bytes: &[u8], at: &mut usize) -> Option<Self> {
        let take = |at: &mut usize, n: usize| -> Option<&[u8]> {
            let end = at.checked_add(n).filter(|e| *e <= bytes.len())?;
            let slice = &bytes[*at..end];
            *at = end;
            Some(slice)
        };
        let u32_at = |at: &mut usize| -> Option<usize> {
            let b = take(at, 4)?;
            Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
        };
        let count = u32_at(at)?;
        let mut components = Vec::with_capacity(count);
        for _ in 0..count {
            let n_len = u32_at(at)?;
            let numer = num_bigint::BigInt::from_signed_bytes_le(take(at, n_len)?);
            let d_len = u32_at(at)?;
            let denom = num_bigint::BigInt::from_signed_bytes_le(take(at, d_len)?);
            components.push(Rational::try_new(numer, denom).ok()?);
        }
        Some(Self::from_vec(components))
    }

    pub fn components(&self) -> &[Rational<F>] {
        &self.components
    }

    /// Get the i-th coefficient as `Option`. Returns `None` if `i ≥ dim`.
    /// Polynomial convention: coefficients past `dim` are implicitly
    /// zero — callers wanting that semantic use `coefficient_at`.
    #[inline]
    pub fn get(&self, i: usize) -> Option<Rational<F>> {
        self.components.get(i).cloned()
    }

    /// Coefficient at index `i` with implicit-zero past-dim semantic —
    /// `i ≥ dim` returns `Rational::zero()`. Standard polynomial ring
    /// behavior.
    ///
    /// **WARNING.** Per LAW XIV, calling `coefficient_at(0)` on a value
    /// of arbitrary polynomial degree is a derivation-path collapse —
    /// it discards every higher-degree coefficient. Legitimate
    /// boundaries (wire-format encode of degree-0 known values; test
    /// fixture assertions about the constant term) must carry an
    /// inline SINGLETON-BOUNDARY comment. The #62.1 acceptance gate
    /// greps for literal `coefficient_at(0)` and requires every match
    /// to be tagged. General-index `coefficient_at(i)` with loop
    /// variable `i` is polynomial-arithmetic-internal and not flagged.
    pub fn coefficient_at(&self, i: usize) -> Rational<F> {
        self.components
            .get(i)
            .cloned()
            .unwrap_or_else(Rational::<F>::zero)
    }

    /// True iff every coefficient is exactly zero (including the empty
    /// polynomial — vacuously all-zero).
    pub fn is_zero(&self) -> bool {
        self.components.iter().all(|c| c.is_zero())
    }

    /// True iff this is exactly the empty (zero-polynomial) form. Used
    /// by `convolve` to short-circuit.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Return a new `VectorModule<F>` with trailing zero coefficients
    /// removed. The substrate's canonical form for polynomial equality
    /// comparisons — `[1, 0]` and `[1, 0, 0]` are algebraically equal
    /// but `PartialEq` is strict; canonicalize through this helper when
    /// comparing two polynomials of unequal stored length.
    pub fn trim_trailing_zeros(&self) -> Self {
        let mut end = self.components.len();
        while end > 0 && self.components[end - 1].is_zero() {
            end -= 1;
        }
        Self {
            components: self.components[..end].to_vec(),
            _field: PhantomData,
        }
    }

    /// Polynomial addition with auto-pad-shorter-with-zeros semantics.
    /// Result has `max(self.dim(), other.dim())` coefficients **before
    /// trim**; trailing-zero coefficients are stripped at emit per the
    /// substrate's minima-enforcement invariant (CLAUDE.md / Section
    /// 6b.3: every emit site produces a trimmed polynomial).
    pub fn try_add(&self, other: &Self) -> Result<Self, VectorModuleError> {
        let max_dim = self.dim().max(other.dim());
        let mut components = Vec::with_capacity(max_dim);
        for i in 0..max_dim {
            let a = self.coefficient_at(i);
            let b = other.coefficient_at(i);
            components.push(a.try_add(&b)?);
        }
        Ok(Self {
            components,
            _field: PhantomData,
        }
        .trim_trailing_zeros())
    }

    /// Polynomial subtraction with auto-pad-shorter-with-zeros semantics.
    /// Output trimmed per emit-site invariant.
    pub fn try_sub(&self, other: &Self) -> Result<Self, VectorModuleError> {
        let max_dim = self.dim().max(other.dim());
        let mut components = Vec::with_capacity(max_dim);
        for i in 0..max_dim {
            let a = self.coefficient_at(i);
            let b = other.coefficient_at(i);
            components.push(a.try_sub(&b)?);
        }
        Ok(Self {
            components,
            _field: PhantomData,
        }
        .trim_trailing_zeros())
    }

    /// Polynomial multiplication: `(Σ aᵢ xⁱ) · (Σ bⱼ xʲ) =
    /// Σₖ (Σᵢ aᵢ bₖ₋ᵢ) xᵏ`. Either-zero short-circuits to the
    /// zero polynomial. Result length is `m + n − 1` for non-trivial
    /// operands; trimmed at emit per minima-enforcement invariant.
    ///
    /// THIS is the operation that produces `p^N` denominator growth
    /// from successive per-axis multiplications — the canonical
    /// witness exercised by the #62.1 polynomial-growth test.
    pub fn convolve(&self, other: &Self) -> Result<Self, VectorModuleError> {
        if self.is_empty() || other.is_empty() {
            return Ok(Self::zero_polynomial());
        }
        let result_len = self.dim() + other.dim() - 1;
        let mut components = vec![Rational::<F>::zero(); result_len];
        for (i, a) in self.components.iter().enumerate() {
            for (j, b) in other.components.iter().enumerate() {
                let prod = a.try_mul(b)?;
                components[i + j] = components[i + j].try_add(&prod)?;
            }
        }
        Ok(Self {
            components,
            _field: PhantomData,
        }
        .trim_trailing_zeros())
    }

    /// Negate every coefficient — substrate's polynomial negation.
    /// Output trimmed per emit-site invariant (negation can introduce
    /// `-0` artifacts at trailing positions for degenerate inputs;
    /// trim ensures minimal representation).
    pub fn negate(&self) -> Self {
        Self {
            components: self.components.iter().map(|c| c.negate_ref()).collect(),
            _field: PhantomData,
        }
        .trim_trailing_zeros()
    }

    /// **Stretch** — bounded radial scaling. Multiplies every
    /// coefficient by a field-typed scalar. Polynomial scaling:
    /// `c · (Σ aᵢ xⁱ)`. Output trimmed per emit-site invariant
    /// (scaling by zero produces an all-zero polynomial, trimmed
    /// to empty).
    pub fn stretch(&self, factor: Rational<F>) -> Result<Self, VectorModuleError> {
        let mut components = Vec::with_capacity(self.dim());
        for a in &self.components {
            components.push(a.try_mul(&factor)?);
        }
        Ok(Self {
            components,
            _field: PhantomData,
        }
        .trim_trailing_zeros())
    }

    /// **Bend** — boundary ramification shear between two indices.
    /// `components[i] += magnitude · components[j]`.
    ///
    /// Out-of-bounds indices return the structural `AxisOutOfBounds`
    /// residual (LAW V (a)). The diagonal case `i == j` is valid
    /// (scaling component `i` by `1 + magnitude`); no boolean gate
    /// rejects it.
    pub fn bend(
        &self,
        axis_pair: (usize, usize),
        magnitude: Rational<F>,
    ) -> Result<Self, VectorModuleError> {
        let (i, j) = axis_pair;
        let v_j = self.get(j).ok_or(VectorModuleError::AxisOutOfBounds {
            axis: j,
            dim: self.dim(),
        })?;
        let v_i = self.get(i).ok_or(VectorModuleError::AxisOutOfBounds {
            axis: i,
            dim: self.dim(),
        })?;
        let mut out = self.clone();
        let kick = magnitude.try_mul(&v_j)?;
        out.components[i] = v_i.try_add(&kick)?;
        Ok(out)
    }

    /// **Degree re-seat (shift down)** — drop the lowest `n` coefficients and
    /// re-index the remainder to degree 0 (`Σ_{d≥n} c_d x^d ↦ Σ c_{d+n} x^d`).
    /// A VectorModule→VectorModule transform: the carrier shifts ITSELF, so no
    /// bare coefficient `Vec<Rational<F>>` is exposed at the call site (LAW XIV —
    /// the per-axis polynomial stays the multi-axial carrier end to end, never a
    /// loose scalar list). `n ≥ dim` yields the zero polynomial.
    pub fn shift_down_degrees(&self, n: usize) -> Self {
        let mut components = Vec::with_capacity(self.dim().saturating_sub(n));
        for c in self.components.iter().skip(n) {
            components.push(c.clone());
        }
        Self {
            components,
            _field: PhantomData,
        }
        .trim_trailing_zeros()
    }

    /// The constant-1 polynomial — multiplicative identity in `F[x]`.
    pub fn one() -> Self {
        Self {
            components: vec![Rational::<F>::one()],
            _field: PhantomData,
        }
    }

    /// Polynomial degree — the index of the highest non-zero
    /// coefficient. For the empty/zero polynomial returns 0 by
    /// convention; callers needing to distinguish the zero case
    /// pattern-match on `is_zero()` first.
    pub fn degree(&self) -> usize {
        for i in (0..self.components.len()).rev() {
            if !self.components[i].is_zero() {
                return i;
            }
        }
        0
    }

    /// Leading coefficient. For a non-zero polynomial this is the
    /// coefficient at `degree()`. For the zero polynomial returns the
    /// field zero.
    pub fn leading_coefficient(&self) -> Rational<F> {
        if self.is_zero() {
            return Rational::<F>::zero();
        }
        self.components[self.degree()].clone()
    }

    /// Polynomial division-with-remainder over `F[x]`. Returns
    /// `(quotient, remainder)` such that
    /// `self == quotient · divisor + remainder` and
    /// `deg(remainder) < deg(divisor)` (or remainder is zero).
    /// Errors on the zero divisor (the field zero has no inverse).
    pub fn try_div_rem(&self, divisor: &Self) -> Result<(Self, Self), RingError> {
        if divisor.is_zero() {
            return Err(RingError::DivisionByZeroPolynomial);
        }
        let mut remainder = self.clone();
        let divisor_lead = divisor.leading_coefficient();
        let divisor_deg = divisor.degree();
        let mut quotient = Self::zero_polynomial();
        while !remainder.is_zero() && remainder.degree() >= divisor_deg {
            let lead_q = remainder.leading_coefficient().try_div(&divisor_lead)?;
            let shift = remainder.degree() - divisor_deg;
            let mut q_term = vec![Rational::<F>::zero(); shift + 1];
            q_term[shift] = lead_q.clone();
            let q_term_vm = Self::from_vec(q_term);
            quotient = quotient.try_add(&q_term_vm)?;
            let subtractor = q_term_vm.convolve(divisor)?;
            remainder = remainder.try_sub(&subtractor)?;
        }
        Ok((
            quotient.trim_trailing_zeros(),
            remainder.trim_trailing_zeros(),
        ))
    }

    /// Greatest common divisor over `F[x]`, returned in **primitive (content-
    /// normalized) canonical form** — NOT monic. Monic normalization (divide
    /// every coefficient by the LEADING one) is a privileged-axis scalar
    /// collapse: it elects one coefficient, forces it to 1, and rewrites the
    /// rest as ratios to it — discarding the absolute scale and REQUIRING
    /// field-division (`c / lead`), which the smooth-prime carrier `Rational<F>`
    /// does not support (a non-smooth leading numerator → a non-smooth
    /// denominator → `PrimeProfileViolation`). The lawful canonical form is the
    /// CONTENT-democratic one: divide every coefficient by the content (the gcd
    /// of all of them), which divides EXACTLY (stays in-ring, never a non-smooth
    /// denominator) and privileges no axis. Computed via the in-ring `ring_gcd`
    /// (pseudo-division — no field-division anywhere on the path).
    pub fn gcd(&self, other: &Self) -> Result<Self, RingError> {
        self.ring_gcd(other)
    }

    /// **In-ring polynomial gcd via the pseudo-remainder sequence** — no field-
    /// division, so it never produces a non-smooth denominator on the smooth-
    /// prime carrier (`Rational<F>` is NOT a field; Euclidean-with-division gcd
    /// assumes one and panics on a non-smooth leading coefficient). Each step
    /// takes the pseudo-remainder (multiply-through, never divide) then CONTENT-
    /// REDUCES it to primitive form — the content reduction keeps the pseudo-
    /// remainder coefficients BOUNDED (the subresultant discipline, simplified
    /// to full content removal: no coefficient explosion across the loop).
    /// Terminates: `deg(b)` strictly decreases each iteration. The result is the
    /// primitive (content-normalized, positive-leading) associate of the true
    /// gcd — Law XII deterministic.
    pub fn ring_gcd(&self, other: &Self) -> Result<Self, RingError> {
        let mut a = self.primitive_part();
        let mut b = other.primitive_part();
        while !b.is_zero() {
            let r = a.pseudo_remainder(&b)?.primitive_part();
            a = b;
            b = r;
        }
        Ok(a) // already primitive by loop invariant; the zero polynomial stays zero
    }

    /// **Pseudo-division remainder** `prem(self, divisor)` — the remainder of
    /// `lead(divisor)^k · self ÷ divisor`, computed with ONLY multiplications
    /// (`stretch` by `lead(divisor)`) and subtractions, never a coefficient
    /// division. So it stays inside the smooth-prime ring (denominators only
    /// MULTIPLY → stay smooth) — no `PrimeProfileViolation`. It is the true
    /// field-division remainder scaled by the nonzero factor `lead^k`, so
    /// `prem == 0 ⟺ divisor | self` exactly (the divisibility test the
    /// reachability read needs). Errors on the zero divisor.
    pub fn pseudo_remainder(&self, divisor: &Self) -> Result<Self, RingError> {
        if divisor.is_zero() {
            return Err(RingError::DivisionByZeroPolynomial);
        }
        let divisor_lead = divisor.leading_coefficient();
        let divisor_deg = divisor.degree();
        let mut remainder = self.clone();
        // `deg(remainder)` strictly decreases each step (the lead terms cancel),
        // so the loop is bounded by the initial degree — no unbounded iteration.
        while !remainder.is_zero() && remainder.degree() >= divisor_deg {
            let r_lead = remainder.leading_coefficient();
            let shift = remainder.degree() - divisor_deg;
            // subtractor = (r_lead · x^shift) · divisor
            let mut sx = vec![Rational::<F>::zero(); shift + 1];
            sx[shift] = r_lead;
            let subtractor = Self::from_vec(sx).convolve(divisor)?;
            // remainder ← lead(divisor)·remainder − subtractor (lead terms cancel)
            remainder = remainder
                .stretch(divisor_lead.clone())?
                .try_sub(&subtractor)?;
        }
        Ok(remainder.trim_trailing_zeros())
    }

    /// **Primitive (content-normalized) form** — the lawful canonical
    /// representative, replacing monic. Clears denominators by their lcm (a
    /// product of the field's smooth primes ⇒ always a valid denominator) so
    /// every coefficient is an integer, divides by the content (the BigInt gcd
    /// of those integer numerators) so the coefficients are collectively
    /// primitive, then fixes the sign so the leading coefficient is positive
    /// (Law XII determinism). EVERY step is in-ring: multiply by the smooth lcm,
    /// divide by the content which divides EXACTLY — no non-smooth denominator
    /// ever appears, so the panic monic produced is structurally impossible.
    /// Content-democratic: no coefficient is privileged. Zero is its own
    /// primitive form.
    pub fn primitive_part(&self) -> Self {
        use num_integer::Integer as _;
        use num_traits::{One, Signed, Zero};
        if self.is_zero() {
            return self.clone();
        }
        // L = lcm of all denominators (smooth · smooth = smooth ⇒ valid).
        let mut l = BigInt::one();
        for c in &self.components {
            l = l.lcm(c.denom());
        }
        // G = gcd of the denominator-cleared integer numerators (L/d)·n.
        let mut g = BigInt::zero();
        for c in &self.components {
            if c.is_zero() {
                continue;
            }
            g = g.gcd(&((&l / c.denom()) * c.numer()));
        }
        if g.is_zero() {
            return self.clone(); // defensive: all-zero was handled above
        }
        // Sign convention: make the leading non-zero coefficient positive.
        let signed_g = if self.leading_coefficient().numer().is_negative() {
            -g
        } else {
            g
        };
        let components = self
            .components
            .iter()
            .map(|c| {
                let cleared = (&l / c.denom()) * c.numer();
                Rational::<F>::try_new(&cleared / &signed_g, BigInt::one())
                    .expect("content divides each cleared numerator exactly ⇒ integer, denom 1")
            })
            .collect();
        Self {
            components,
            _field: PhantomData,
        }
        .trim_trailing_zeros()
    }

    /// Exact-the exact rationals Cauchy upper bound on the absolute value of polynomial
    /// roots:
    ///
    /// ```text
    ///   M = 1 + max_{i < deg} |c_i / c_deg|
    /// ```
    ///
    /// Returns the bound as a `Rational<F>`. Errors on the zero or
    /// constant polynomial (no roots / undefined bound).
    pub fn cauchy_bound(&self) -> Result<Rational<F>, RingError> {
        if self.is_zero() {
            return Err(RingError::CauchyBoundOnZero);
        }
        let deg = self.degree();
        if deg == 0 {
            return Ok(Rational::<F>::zero());
        }
        let c_deg = self.components[deg].clone();
        let c_deg_abs = abs_rational(&c_deg);
        let mut max_ratio = Rational::<F>::zero();
        for c_i in self.components.iter().take(deg) {
            let c_i_abs = abs_rational(c_i);
            let ratio = c_i_abs.try_div(&c_deg_abs)?;
            if ratio > max_ratio {
                max_ratio = ratio;
            }
        }
        Ok(max_ratio.try_add(&Rational::<F>::one())?)
    }
}

/// Exact-the exact rationals absolute value helper for `cauchy_bound`. Preserves the
/// `Rational<F>` field tag; per LAW IV exact-rational, no approximation.
fn abs_rational<F: Field>(r: &Rational<F>) -> Rational<F> {
    if r.is_negative() {
        r.negate_ref()
    } else {
        r.clone()
    }
}

/// Structured residual of a `VectorModule<F>` operation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VectorModuleError {
    #[error("axis index {axis} out of bounds (dim={dim})")]
    AxisOutOfBounds { axis: usize, dim: usize },

    #[error("vector arithmetic overflow: {source}")]
    Arithmetic {
        #[from]
        source: OverflowError,
    },
}

/// Structured residual of a polynomial-ring operation on
/// `VectorModule<F>` (`try_div_rem`, `gcd`/`ring_gcd`, `pseudo_remainder`,
/// `cauchy_bound`).
/// Kept separate from `VectorModuleError` so the additive/scaling
/// surface's exhaustive `match` sites across the workspace are
/// untouched.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RingError {
    #[error("polynomial arithmetic overflow: {source}")]
    Arithmetic {
        #[from]
        source: OverflowError,
    },

    #[error("polynomial division by the zero polynomial")]
    DivisionByZeroPolynomial,

    #[error("cauchy bound on the zero polynomial is undefined")]
    CauchyBoundOnZero,
}

impl From<VectorModuleError> for RingError {
    fn from(value: VectorModuleError) -> Self {
        match value {
            VectorModuleError::Arithmetic { source } => Self::Arithmetic { source },
            VectorModuleError::AxisOutOfBounds { .. } => {
                unreachable!("polynomial-ring ops do not touch axis-bound indexing")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rational::{Space, SpatialRational};
    use num_bigint::BigInt;

    fn r(num: i64, denom: i64) -> SpatialRational {
        SpatialRational::try_new(BigInt::from(num), BigInt::from(denom)).unwrap()
    }

    #[test]
    fn zero_polynomial_is_empty_and_is_zero() {
        let z = VectorModule::<Space>::zero_polynomial();
        assert!(z.is_empty());
        assert!(z.is_zero());
        assert_eq!(z.dim(), 0);
    }

    #[test]
    fn try_add_pads_shorter_operand() {
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(3, 1)]);
        let sum = a.try_add(&b).expect("ok");
        assert_eq!(sum.dim(), 2);
        assert_eq!(sum.coefficient_at(0), r(4, 1));
        assert_eq!(sum.coefficient_at(1), r(2, 1));
    }

    #[test]
    fn convolve_grows_degree() {
        // (1 + 2x) * (3 + 4x) = 3 + 10x + 8x^2 — degree 1+1 = 2 (length 3)
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(3, 1), r(4, 1)]);
        let prod = a.convolve(&b).expect("ok");
        assert_eq!(prod.dim(), 3);
        assert_eq!(prod.coefficient_at(0), r(3, 1));
        assert_eq!(prod.coefficient_at(1), r(10, 1));
        assert_eq!(prod.coefficient_at(2), r(8, 1));
    }

    #[test]
    fn convolve_with_zero_short_circuits() {
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1)]);
        let z = VectorModule::<Space>::zero_polynomial();
        assert!(a.convolve(&z).unwrap().is_empty());
    }

    /// **#62.1 ACCEPTANCE WITNESS** — five successive `convolve`
    /// operations grow polynomial degree monotonically per axis.
    /// Proves the algebra produces `p^N`-style growth from per-axis
    /// composition, not collapse.
    #[test]
    fn convolve_five_times_grows_degree_monotonically() {
        // SINGLETON-BOUNDARY [test fixture]: linear-polynomial seed at K=0
        // by construction — test exercises polynomial-degree growth across
        // five convolve operations. Polynomial growth begins at the first
        // convolve(seed) call below; this site is the seed, not a collapse.
        let seed = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 1)]);
        let mut acc = seed.clone();
        let mut last_dim = acc.dim();
        for step in 1..=5 {
            acc = acc.convolve(&seed).expect("ok");
            assert!(
                acc.dim() > last_dim,
                "convolve step {step} did not grow polynomial degree (was {last_dim}, now {})",
                acc.dim()
            );
            last_dim = acc.dim();
        }
        // (1+x)^6 has degree 6 → length 7
        assert_eq!(acc.dim(), 7);
    }

    #[test]
    fn convolve_grows_denominator_for_smooth_prime_seed() {
        // (1/2)^5 = 1/32 — denominator growth in p^N (here p=2, N=5).
        // Witnesses that the polynomial backbone admits exact unbounded
        // BigRational denominators.
        let half = VectorModule::<Space>::from_vec(vec![r(1, 2)]);
        let mut acc = half.clone();
        for _ in 0..4 {
            acc = acc.convolve(&half).expect("ok");
        }
        // singleton convolve singleton: constant polynomial of length 1
        assert_eq!(acc.dim(), 1);
        let c0 = acc.coefficient_at(0);
        assert_eq!(c0.numerator(), &BigInt::from(1));
        assert_eq!(c0.denominator(), &BigInt::from(32));
    }

    #[test]
    fn trim_trailing_zeros_canonicalizes() {
        let v = VectorModule::<Space>::from_vec(vec![r(1, 1), r(0, 1), r(0, 1)]);
        let trimmed = v.trim_trailing_zeros();
        assert_eq!(trimmed.dim(), 1);
        assert_eq!(trimmed.coefficient_at(0), r(1, 1));
    }

    #[test]
    fn stretch_scales_every_coefficient() {
        let v = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1)]);
        let s = v.stretch(r(3, 1)).expect("ok");
        assert_eq!(s.coefficient_at(0), r(3, 1));
        assert_eq!(s.coefficient_at(1), r(6, 1));
    }

    #[test]
    fn bend_shear_between_indices() {
        let v = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1), r(3, 1)]);
        // components[0] += 5 * components[2] = 1 + 5*3 = 16
        let b = v.bend((0, 2), r(5, 1)).expect("ok");
        assert_eq!(b.coefficient_at(0), r(16, 1));
        assert_eq!(b.coefficient_at(1), r(2, 1));
        assert_eq!(b.coefficient_at(2), r(3, 1));
    }

    /// **Correction 4 witness — minima at try_add emit site.**
    /// Adding polynomials whose result has trailing zeros produces a
    /// trimmed output (trim_trailing_zeros is a no-op on the result).
    #[test]
    fn try_add_emits_trimmed_polynomial() {
        // [3, 5, -5] + [0, 0, 5] = [3, 5, 0] — trailing zero must be stripped.
        let a = VectorModule::<Space>::from_vec(vec![r(3, 1), r(5, 1), r(-5, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(0, 1), r(0, 1), r(5, 1)]);
        let sum = a.try_add(&b).expect("ok");
        // Result was [3, 5, 0]; after trim → [3, 5].
        assert_eq!(sum.dim(), 2);
        assert_eq!(sum, sum.trim_trailing_zeros());
    }

    /// **Correction 4 witness — minima at try_sub emit site.**
    #[test]
    fn try_sub_emits_trimmed_polynomial() {
        // [3, 5, 7] - [0, 0, 7] = [3, 5, 0] → trimmed to [3, 5].
        let a = VectorModule::<Space>::from_vec(vec![r(3, 1), r(5, 1), r(7, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(0, 1), r(0, 1), r(7, 1)]);
        let diff = a.try_sub(&b).expect("ok");
        assert_eq!(diff.dim(), 2);
        assert_eq!(diff, diff.trim_trailing_zeros());
    }

    /// **Correction 4 witness — minima at convolve emit site.**
    /// (Convolve naturally produces minimal-length output for
    /// non-zero inputs, but verify trim is a no-op.)
    #[test]
    fn convolve_emits_trimmed_polynomial() {
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(3, 1), r(4, 1)]);
        let prod = a.convolve(&b).expect("ok");
        assert_eq!(prod, prod.trim_trailing_zeros());
    }

    /// **Correction 4.b witness — integer-coefficient preservation
    /// through arithmetic.** Adding two integer-coefficient polynomials
    /// produces an integer-coefficient result (every denominator = 1).
    #[test]
    fn integer_add_preserves_integrality() {
        let p1 = VectorModule::<Space>::from_vec(vec![r(3, 1), r(5, 1), r(7, 1)]);
        let p2 = VectorModule::<Space>::from_vec(vec![r(2, 1), r(4, 1), r(6, 1)]);
        let sum = p1.try_add(&p2).expect("ok");
        // [5, 9, 13] — all integer.
        for coeff in sum.components() {
            assert_eq!(
                coeff.denominator(),
                &BigInt::from(1),
                "integer-coefficient preservation: every denominator must be 1, got {}",
                coeff.denominator(),
            );
        }
    }

    #[test]
    fn bend_out_of_bounds_returns_structured_axis_error() {
        let v = VectorModule::<Space>::from_vec(vec![r(1, 1)]);
        let err = v.bend((0, 5), r(1, 1)).expect_err("axis 5 oob");
        match err {
            VectorModuleError::AxisOutOfBounds { axis, dim } => {
                assert_eq!(axis, 5);
                assert_eq!(dim, 1);
            }
            _ => panic!("expected AxisOutOfBounds"),
        }
    }

    // ── Ring-axiom witnesses on F[x] surface (post-#1736 migration). ──
    //
    // The witnesses below pin the polynomial-ring operations
    // (`one`, `degree`, `leading_coefficient`, `try_div_rem`, `gcd`/
    // `ring_gcd`, `pseudo_remainder`, `primitive_part`, `cauchy_bound`)
    // onto `VectorModule<F>` after the retirement of the standalone
    // `Polynomial<F>` carrier.

    #[test]
    fn one_is_multiplicative_identity_left_and_right() {
        // 1 · p == p ; p · 1 == p
        let p = VectorModule::<Space>::from_vec(vec![r(2, 1), r(3, 1), r(5, 1)]);
        let one = VectorModule::<Space>::one();
        assert_eq!(one.convolve(&p).unwrap(), p);
        assert_eq!(p.convolve(&one).unwrap(), p);
    }

    #[test]
    fn degree_of_one_is_zero_and_degree_of_x_is_one() {
        assert_eq!(VectorModule::<Space>::one().degree(), 0);
        let x = VectorModule::<Space>::from_vec(vec![r(0, 1), r(1, 1)]);
        assert_eq!(x.degree(), 1);
    }

    #[test]
    fn leading_coefficient_picks_high_degree_term() {
        let p = VectorModule::<Space>::from_vec(vec![r(2, 1), r(3, 1), r(5, 1)]);
        assert_eq!(p.leading_coefficient(), r(5, 1));
        let z = VectorModule::<Space>::zero_polynomial();
        assert_eq!(z.leading_coefficient(), Rational::<Space>::zero());
    }

    #[test]
    fn try_div_rem_satisfies_division_algorithm() {
        // (x² + 3x + 2) = (x + 1)(x + 2) + 0
        let dividend = VectorModule::<Space>::from_vec(vec![r(2, 1), r(3, 1), r(1, 1)]);
        let divisor = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 1)]);
        let (q, rem) = dividend.try_div_rem(&divisor).unwrap();
        assert_eq!(rem, VectorModule::<Space>::zero_polynomial());
        // q · divisor + rem == dividend
        let reconstructed = q.convolve(&divisor).unwrap().try_add(&rem).unwrap();
        assert_eq!(reconstructed, dividend);
    }

    #[test]
    fn try_div_rem_remainder_has_smaller_degree() {
        // (x² + 1) / (x + 1) = x − 1 + 2 / (x+1) → quotient x − 1, remainder 2
        let dividend = VectorModule::<Space>::from_vec(vec![r(1, 1), r(0, 1), r(1, 1)]);
        let divisor = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 1)]);
        let (q, rem) = dividend.try_div_rem(&divisor).unwrap();
        assert!(rem.is_zero() || rem.degree() < divisor.degree());
        let reconstructed = q.convolve(&divisor).unwrap().try_add(&rem).unwrap();
        assert_eq!(reconstructed, dividend);
    }

    #[test]
    fn try_div_rem_by_zero_returns_typed_residual() {
        let p = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 1)]);
        let z = VectorModule::<Space>::zero_polynomial();
        match p.try_div_rem(&z) {
            Err(RingError::DivisionByZeroPolynomial) => {}
            other => panic!("expected DivisionByZeroPolynomial, got {other:?}"),
        }
    }

    #[test]
    fn gcd_divides_both_operands() {
        // gcd(x² − 1, x − 1) = x − 1 (monic). Both operands divisible by it.
        let p1 = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(0, 1), r(1, 1)]);
        let p2 = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(1, 1)]);
        let g = p1.gcd(&p2).unwrap();
        let (_, r1) = p1.try_div_rem(&g).unwrap();
        let (_, r2) = p2.try_div_rem(&g).unwrap();
        assert!(r1.is_zero());
        assert!(r2.is_zero());
    }

    #[test]
    fn gcd_is_primitive_not_monic() {
        // gcd(x² − 1, x − 1) = x − 1, in PRIMITIVE form: integer coeffs,
        // content 1, positive leading — never a divide-by-leading (no monic).
        let p1 = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(0, 1), r(1, 1)]);
        let p2 = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(1, 1)]);
        let g = p1.gcd(&p2).unwrap();
        assert_eq!(g, VectorModule::<Space>::from_vec(vec![r(-1, 1), r(1, 1)]));
    }

    #[test]
    fn gcd_with_zero_is_other_primitive() {
        // gcd(2x, 0) == x (primitive: 2x ÷ content 2 = x), NEVER 2x ÷ lead.
        let p = VectorModule::<Space>::from_vec(vec![r(0, 1), r(2, 1)]);
        let z = VectorModule::<Space>::zero_polynomial();
        let g = p.gcd(&z).unwrap();
        assert_eq!(g, p.primitive_part());
        assert_eq!(g, VectorModule::<Space>::from_vec(vec![r(0, 1), r(1, 1)]));
    }

    #[test]
    fn primitive_part_is_content_division_not_leading_division() {
        // 6x² + 4x → CONTENT-reduce by gcd(6,4)=2 → 3x² + 2x (integer, primitive).
        // Monic would have divided by the LEADING 6 → x² + (2/3)x (a privileged-axis
        // collapse introducing a denominator). Primitive keeps every coefficient whole.
        let p = VectorModule::<Space>::from_vec(vec![r(0, 1), r(4, 1), r(6, 1)]);
        let prim = p.primitive_part();
        assert_eq!(
            prim,
            VectorModule::<Space>::from_vec(vec![r(0, 1), r(2, 1), r(3, 1)]),
            "content-democratic reduction, not divide-by-leading"
        );
    }

    #[test]
    fn primitive_part_clears_denominators_in_ring_and_normalizes_sign() {
        // (−1/2)x − 1 → clear denom (×2) → −x − 2 → content 1 → sign-normalize
        // leading positive → x + 2. All in-ring (×2 smooth, ÷content exact).
        let p = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(-1, 2)]);
        let prim = p.primitive_part();
        assert_eq!(
            prim,
            VectorModule::<Space>::from_vec(vec![r(2, 1), r(1, 1)])
        );
    }

    #[test]
    fn primitive_part_of_zero_is_zero() {
        let z = VectorModule::<Space>::zero_polynomial();
        assert!(z.primitive_part().is_zero());
    }

    #[test]
    fn pseudo_remainder_zero_iff_divisible() {
        // x² − 1 = (x − 1)(x + 1) ⇒ (x−1) | (x²−1) ⇒ pseudo-remainder zero.
        let p = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(0, 1), r(1, 1)]);
        let d = VectorModule::<Space>::from_vec(vec![r(-1, 1), r(1, 1)]);
        assert!(
            p.pseudo_remainder(&d).unwrap().is_zero(),
            "exact divisor ⇒ prem 0"
        );
        // x² + 1 is NOT divisible by x − 1 ⇒ non-zero pseudo-remainder.
        let q = VectorModule::<Space>::from_vec(vec![r(1, 1), r(0, 1), r(1, 1)]);
        assert!(
            !q.pseudo_remainder(&d).unwrap().is_zero(),
            "non-divisor ⇒ prem ≠ 0"
        );
    }

    #[test]
    fn cauchy_bound_is_exact_q_and_strictly_above_unity_ratios() {
        // p = x² + 3x + 2  → max ratio = 3 → bound = 4
        let p = VectorModule::<Space>::from_vec(vec![r(2, 1), r(3, 1), r(1, 1)]);
        let m = p.cauchy_bound().unwrap();
        assert_eq!(m, r(4, 1));
    }

    #[test]
    fn cauchy_bound_of_zero_returns_typed_residual() {
        let z = VectorModule::<Space>::zero_polynomial();
        match z.cauchy_bound() {
            Err(RingError::CauchyBoundOnZero) => {}
            other => panic!("expected CauchyBoundOnZero, got {other:?}"),
        }
    }

    #[test]
    fn add_is_commutative() {
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1), r(2, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(3, 1), r(4, 1), r(5, 1)]);
        assert_eq!(a.try_add(&b).unwrap(), b.try_add(&a).unwrap());
    }

    #[test]
    fn add_is_associative() {
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(2, 1), r(3, 1)]);
        let c = VectorModule::<Space>::from_vec(vec![r(0, 1), r(0, 1), r(1, 1)]);
        let lhs = a.try_add(&b).unwrap().try_add(&c).unwrap();
        let rhs = a.try_add(&b.try_add(&c).unwrap()).unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn convolve_is_commutative() {
        let a = VectorModule::<Space>::from_vec(vec![r(2, 1), r(3, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(5, 1), r(0, 1), r(7, 1)]);
        assert_eq!(a.convolve(&b).unwrap(), b.convolve(&a).unwrap());
    }

    #[test]
    fn convolve_is_associative() {
        let a = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(0, 1), r(1, 1)]);
        let c = VectorModule::<Space>::from_vec(vec![r(1, 1), r(0, 1), r(1, 1)]);
        let lhs = a.convolve(&b).unwrap().convolve(&c).unwrap();
        let rhs = a.convolve(&b.convolve(&c).unwrap()).unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn convolve_distributes_over_add() {
        // a · (b + c) == a·b + a·c
        let a = VectorModule::<Space>::from_vec(vec![r(2, 1), r(1, 1)]);
        let b = VectorModule::<Space>::from_vec(vec![r(1, 1), r(0, 1), r(1, 1)]);
        let c = VectorModule::<Space>::from_vec(vec![r(0, 1), r(3, 1)]);
        let lhs = a.convolve(&b.try_add(&c).unwrap()).unwrap();
        let rhs = a
            .convolve(&b)
            .unwrap()
            .try_add(&a.convolve(&c).unwrap())
            .unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn negate_then_add_yields_zero() {
        let p = VectorModule::<Space>::from_vec(vec![r(2, 1), r(-3, 1), r(5, 1)]);
        let n = p.negate();
        assert!(p.try_add(&n).unwrap().is_zero());
    }
}
