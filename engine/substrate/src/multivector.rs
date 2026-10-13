// `Multivector<F>` — polynomial-aware per-field Clifford algebra carrier.
//
// CLAUDE.md "Multi-field rational discipline" + #62.2 doctrine
// (00_COGNITION_DOCTRINE.md §9): "`Multivector<F>` polynomial planes =
// cognitive content multi-grade structure (superposition state during
// K-step accumulation)". The Multivector is the substrate's primary
// algebraic carrier for cognitive content; each plane's coefficient
// is a `VectorModule<F>` polynomial (NOT a scalar `Rational<F>`),
// admitting the `p^N` denominator growth produced by successive
// transport compositions during K-step accumulation.
//
// THE LOAD-BEARING #62.2 LIFT: plane coefficients lift from
// `Rational<F>` to `VectorModule<F>`. The geometric product on two
// multivectors composes plane-pair sign × plane-pair POLYNOMIAL
// CONVOLVE — the substrate's natural `p^N` denominator growth lives
// in the convolve operation that fires inside every transport
// composition.
//
// LAW IV exact rational — coefficient field is `Rational<F>` (BigRational).
// LAW V (a) per-axis preserved — multivectors live in distinct fields
// (`Multivector<Space>` vs `Multivector<Temporal>`); cross-field
// arithmetic compile-time impossible.
// LAW XIV no scalars on derivation path — plane coefficients are
// `VectorModule<F>` polynomial, never scalar `Rational<F>`.
//
// Storage: sparse `Vec<(PlaneIndex, VectorModule<F>)>` sorted by
// `PlaneIndex` ascending; empty-polynomial coefficients stripped
// after every operation. The empty multivector IS the algebraic
// identity (additive zero).
//
// Mathematical specification:
//   Basis vectors:   {e_0^F, e_1^F, ..., e_{r-1}^F}
//   Signature:       (e_i^F)² = +1 (Euclidean per field)
//   Anticommutation: e_i · e_j = -e_j · e_i for i ≠ j
//   Basis planes:    All wedges e_{i_1} ∧ … ∧ e_{i_k}, i_1 < … < i_k
//   PlaneIndex:      u64 bitmask; bit i set ⇔ e_i present in wedge
//   Grade:           grade(plane) = popcount(PlaneIndex)
//
// Geometric product on basis planes:
//   result_bitmask = a XOR b              (shared e_i² = +1 drops out)
//   result_sign    = product_sign(a, b)   (count plane-swap inversions)
//
// Foundational invariants carried:
//   * Grade preservation: transport composition preserves grade structure
//   * Containment: cross-field arithmetic is a compile-time error
//   * Pair-plane construction: e_i ∧ e_j with canonical i < j ordering
//
// Lifted whole from the sealed v0.3 line (shared_primitives::multivector;
// B1 record) minus serde and minus `first_difference_lift` (its consumer,
// the residual pipeline, lands in Phase 2 and lifts it then).

use thiserror::Error;

use crate::horizon::HorizonContext;
use crate::rational::{Field, OverflowError, Rational};
use crate::vector_module::{VectorModule, VectorModuleError};

/// `PlaneIndex` — u64 bitmask identifying a basis plane in `Cl(F)_r`.
///
/// Bit `i` set ⇔ basis vector `e_i` is present in the wedge product.
/// `PlaneIndex(0)` is the scalar (grade 0); `PlaneIndex(0b11)` is
/// `e_0 ∧ e_1` (grade 2); etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct PlaneIndex(pub u64);

/// `PLANE_BIT_WIDTH` — the u64 backing's bit-width. Hard cap on
/// basis-vector index `i`; bit positions `i ∈ [0, 64)`. NOT a doctrinal
/// dimensional ceiling — purely the backing-type width. (Named
/// `BLADE_BIT_WIDTH` until the rotor/blade decommission, 2026-06-11 —
/// "blade" imports the flat 2-D slice; the carrier components are
/// PLANES, doc 40 / #30.)
pub const PLANE_BIT_WIDTH: u8 = 64;

/// **`PairPlane`** — the canonical grade-2 pair-plane index `(i, j)`
/// with `i < j`, identifying the plane `e_i ∧ e_j`. (The rotor-era
/// `BivectorIndex` re-homed and renamed at the decommission,
/// 2026-06-11 — "bivector" imports the flat 2-D projection; the object
/// is a PLANE of the carrier, named by its generating pair.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PairPlane(PlaneIndex);

impl PairPlane {
    /// Construct from `(i, j)` with canonical `i < j`; refuses typed
    /// otherwise.
    pub fn try_new(i: u8, j: u8) -> Result<Self, MultivectorError> {
        PlaneIndex::pair(i, j)
            .map(Self)
            .ok_or(MultivectorError::InvalidPairPlane { i, j })
    }

}

/// **`PLUS_SIG_BIT`** — conformal-extension positive-signature basis
/// vector `e_+`. Per CGA (Conformal Geometric Algebra) signature
/// `(p+1, 1)`, bit 14 is the extra positive-signature axis used in
/// the null-vector construction:
///
///   `n_∞ = e_+ + e_-`   (`n_∞² = e_+² + e_-² = +1 + (-1) = 0`)
///   `n_0 = (1/2)(e_- − e_+)`   (`n_0² = (1/4)(-1 + 1 - 0) = 0`)
///   `n_0 · n_∞ = -1`
///
/// Bit 14 retains the Euclidean +1 signature unchanged (same as bits
/// 0..13). The CGA semantics emerge from bit 15's negative signature
/// in combination with the specific plane combinations representing
/// translation, inversion, and dilation transports.
pub const PLUS_SIG_BIT: u8 = 14;

/// **`NEG_SIG_BIT`** — conformal-extension negative-signature basis
/// vector `e_-`. Bit 15 is the ONLY basis vector in the substrate's
/// `Cl(F)_{p+1, 1}` with `(e_-)² = -1`; this is the substrate's CGA
/// negative-signature axis carrier. When the geometric product
/// `(a · b)` is computed and bit 15 is set in both operands, the
/// shared `(e_-)² = -1` contribution flips the result sign once
/// (relative to pure-Euclidean treatment).
pub const NEG_SIG_BIT: u8 = 15;

/// Bitmask form of `NEG_SIG_BIT` — used for fast negative-signature
/// detection in the geometric product.
pub const NEG_SIG_MASK: u64 = 1u64 << NEG_SIG_BIT;

impl PlaneIndex {
    /// The scalar (grade-0) plane. Bitmask 0; grade 0.
    pub const SCALAR: PlaneIndex = PlaneIndex(0);

    /// Construct from a raw bitmask.
    #[inline]
    pub const fn from_raw(bits: u64) -> Self {
        Self(bits)
    }

    /// **Grade** — popcount of the bitmask. The `as u8` cast is
    /// safe by construction: `u64::count_ones()` returns at most 64,
    /// which fits in u8. Documented at the cast site for clippy
    /// suppression.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub const fn grade(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// True iff basis vector `e_i` is present in this plane.
    #[inline]
    pub const fn contains(self, i: u8) -> bool {
        (self.0 & (1u64 << i)) != 0
    }

    /// Construct a grade-1 plane from a single basis-vector index.
    /// Returns `None` if `i ≥ PLANE_BIT_WIDTH`.
    pub const fn single(i: u8) -> Option<Self> {
        if i >= PLANE_BIT_WIDTH {
            None
        } else {
            Some(Self(1u64 << i))
        }
    }

    /// Construct a grade-2 PAIR plane `e_i ∧ e_j` with canonical
    /// `i < j`. (Named `bivector` until the rotor/blade decommission —
    /// "bivector" imports the flat 2-D projection; the object is a
    /// grade-2 plane of the carrier, named by its generating pair.)
    pub const fn pair(i: u8, j: u8) -> Option<Self> {
        if i >= j || j >= PLANE_BIT_WIDTH {
            None
        } else {
            Some(Self((1u64 << i) | (1u64 << j)))
        }
    }

    /// **Inversion-parity sign** of the geometric product `self · other`
    /// on basis planes. Returns +1 or -1. Branch-free arithmetic on
    /// bit parities; no `if parity == 1` LAW I gate.
    pub const fn product_sign(self, other: PlaneIndex) -> i8 {
        let mut sign: i8 = 1;
        let a = self.0;
        let mut b = other.0;
        let mut j: u32 = 0;
        while b != 0 {
            let b_bit = (b & 1) as i8;
            let above = a >> (j + 1);
            let above_parity = (above.count_ones() & 1) as i8;
            let flip = b_bit * above_parity;
            sign *= 1 - 2 * flip;
            b >>= 1;
            j += 1;
        }
        sign
    }

    /// **Geometric product** on basis planes — CGA-aware signature
    /// `(p+1, 1)`. Returns `(result_index, result_sign)` with
    /// `result_sign ∈ {-1, +1}`.
    ///
    /// Signature handling:
    ///   * Bits 0..=14 are positive-signature (`e_i² = +1`) —
    ///     shared bits contribute +1 each, dropping out via XOR.
    ///   * Bit 15 (`NEG_SIG_BIT`) is negative-signature
    ///     (`e_-² = -1`) — when shared between operands, contributes
    ///     -1, flipping the overall sign once.
    ///
    /// The XOR result_index is unchanged (shared bits drop out via
    /// either +1 or -1 contribution; the wedge orientation parity
    /// is computed separately by `product_sign`).
    pub const fn geometric_product(self, other: PlaneIndex) -> (PlaneIndex, i8) {
        let shared = self.0 & other.0;
        let result = PlaneIndex(self.0 ^ other.0);
        let mut sign = self.product_sign(other);
        // CGA negative-signature contribution: when bit 15 is shared
        // between two operands, e_-² = -1 flips the result sign once.
        // Each bit position can be shared at most once between two
        // operands, so a single bit-check suffices (not a parity
        // count).
        if (shared & NEG_SIG_MASK) != 0 {
            sign = -sign;
        }
        (result, sign)
    }

    /// **Reverse sign** for this plane. The reverse `Ã` of grade-k
    /// flips sign iff `k(k-1)/2` is odd, i.e., `k ≡ 2, 3 (mod 4)`.
    /// Branch-free.
    pub const fn reverse_sign(self) -> i8 {
        let k = self.grade() as i8;
        let parity = ((k & 3) >> 1) & 1;
        1 - 2 * parity
    }
}

/// **Multivector** — sparse plane-indexed element of `Cl(F)_r` with
/// **polynomial-aware** plane coefficients.
///
/// Storage invariants:
/// * Planes sorted by `PlaneIndex` ascending.
/// * No empty-polynomial coefficients (`VectorModule::is_zero()`
///   entries stripped after every operation).
/// * Unbounded plane count — heap `Vec` grows on push.
/// * All coefficients are `VectorModule<F>` polynomials, NOT scalar
///   `Rational<F>`. This is the #62.2 load-bearing lift.
pub struct Multivector<F: Field> {
    planes: Vec<(PlaneIndex, VectorModule<F>)>,
    active_rank: u8,
    horizon: HorizonContext,
}

impl<F: Field> Clone for Multivector<F> {
    fn clone(&self) -> Self {
        Self {
            planes: self.planes.clone(),
            active_rank: self.active_rank,
            horizon: self.horizon,
        }
    }
}

impl<F: Field> core::fmt::Debug for Multivector<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Multivector")
            .field("field", &F::FIELD_NAME)
            .field("active_rank", &self.active_rank)
            .field("horizon", &self.horizon)
            .field("planes", &self.planes)
            .finish()
    }
}

impl<F: Field> PartialEq for Multivector<F> {
    fn eq(&self, other: &Self) -> bool {
        self.planes == other.planes
            && self.active_rank == other.active_rank
            && self.horizon == other.horizon
    }
}

impl<F: Field> Eq for Multivector<F> {}

impl<F: Field> PartialOrd for Multivector<F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: Field> Ord for Multivector<F> {
    /// Total order over the FULL grade structure — the canonical sorted
    /// plane list `(PlaneIndex, VectorModule<F>)`, then `active_rank`,
    /// then `horizon` — consistent with the three-field `PartialEq`.
    /// Per LAW XIV: orders the whole multivector, never a grade-0
    /// projection. Order is a PINNED GAUGE CHOICE (LAW II) — it picks
    /// representatives; it never stands as identity or verdict.
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.planes
            .cmp(&other.planes)
            .then(self.active_rank.cmp(&other.active_rank))
            .then(self.horizon.cmp(&other.horizon))
    }
}

impl<F: Field> Multivector<F> {
    /// Zero multivector — empty plane list. The additive identity.
    pub fn zero(active_rank: u8, horizon: HorizonContext) -> Self {
        Self {
            planes: Vec::new(),
            active_rank,
            horizon,
        }
    }

    /// Construct from explicit `(PlaneIndex, VectorModule<F>)` pairs.
    /// Empty-polynomial coefficients are stripped; the result is
    /// sorted by `PlaneIndex` ascending. Duplicate indices add
    /// element-wise per polynomial.
    pub fn from_planes(
        planes: &[(PlaneIndex, VectorModule<F>)],
        active_rank: u8,
        horizon: HorizonContext,
    ) -> Result<Self, MultivectorError> {
        let mut out: Vec<(PlaneIndex, VectorModule<F>)> = Vec::with_capacity(planes.len());
        for (idx, coeff) in planes {
            // `checked_shr` guards the shift-overflow panic for
            // `active_rank ≥ 64` (u64 shift ≥ width): None ⇒ every plane fits
            // in ≥64 bits ⇒ in-rank.
            if idx.0.checked_shr(u32::from(active_rank)).unwrap_or(0) != 0 {
                return Err(MultivectorError::PlaneOutOfRank {
                    plane: *idx,
                    active_rank,
                });
            }
            if coeff.is_zero() {
                continue;
            }
            // Insert sorted; merge duplicates via polynomial add.
            match out.binary_search_by_key(&idx.0, |(b, _)| b.0) {
                Ok(found) => {
                    let merged = out[found].1.try_add(coeff).map_err(unwrap_plane_overflow)?;
                    if merged.is_zero() {
                        out.remove(found);
                    } else {
                        out[found].1 = merged;
                    }
                }
                Err(insert_at) => out.insert(insert_at, (*idx, coeff.clone())),
            }
        }
        Ok(Self {
            planes: out,
            active_rank,
            horizon,
        })
    }

    /// **A grade-0-only multivector** — the single plane `PlaneIndex::SCALAR` carries the polynomial
    /// `coeff`. A general, LAWFUL building block: grade-0 value-in-degree content is legitimate and
    /// pervasive across the substrate (spectral signatures, word cochains, shear classes, soundness
    /// spectra — "honest, no higher grade to lose"). The retired `from_scalar_seed` collapse was the
    /// SAME shape; it differed only in PROVENANCE (a value fabricated from an integer vs. a real
    /// measurement), which structure cannot see — so the collapse is guarded at the source (the
    /// deleted constructor + the grep-gate), never by a structural ban on grade-0 here.
    /// (Renamed from `scalar`: it builds a grade-0 component, it is not "a scalar".)
    pub fn grade0(coeff: VectorModule<F>, active_rank: u8, horizon: HorizonContext) -> Self {
        if coeff.is_zero() {
            return Self::zero(active_rank, horizon);
        }
        Self {
            planes: vec![(PlaneIndex::SCALAR, coeff)],
            active_rank,
            horizon,
        }
    }

    /// Iterator over `(PlaneIndex, &VectorModule<F>)` for the present
    /// planes. Sorted by `PlaneIndex` ascending.
    pub fn iter_planes(&self) -> impl Iterator<Item = (PlaneIndex, &VectorModule<F>)> {
        self.planes.iter().map(|(idx, c)| (*idx, c))
    }

    /// Number of present planes (count of non-empty-polynomial entries).
    pub fn plane_count(&self) -> usize {
        self.planes.len()
    }

    /// True iff every present plane carries an empty polynomial / all-zero
    /// coefficients (vacuously, empty plane list).
    pub fn is_zero(&self) -> bool {
        self.planes.is_empty() || self.planes.iter().all(|(_, c)| c.is_zero())
    }

    #[inline]
    pub fn active_rank(&self) -> u8 {
        self.active_rank
    }

    /// **LAW-XI INHALE — losslessly re-home every plane into a WIDER `active_rank`.**
    ///
    /// The dual of `breath::exhale`: a widen carries ALL planes unchanged (each
    /// already fits the smaller rank, so it a-fortiori fits the wider one), so the
    /// operation is exactly reversible (an `exhale` back to the original rank
    /// recovers it byte-for-byte — no plane is ever dropped or fabricated). This is
    /// the inhale the #70 singularity-injection needs: a base rank-4 word lifted to
    /// meet a rank-16 conformal-fiber word so the two can form a geometric product
    /// on a common basis width (the e₊/e₋ fiber planes pair with inhaled base
    /// content) instead of being refused as an `ActiveRankMismatch`.
    ///
    /// Refuses a NARROWING target: shrinking the rank is an `exhale` (it would drop
    /// out-of-rank planes), which must route through `breath::exhale` so the
    /// displaced content is preserved in the residual — never silently truncated.
    pub fn widened_to_rank(&self, new_rank: u8) -> Result<Self, MultivectorError> {
        if new_rank < self.active_rank {
            return Err(MultivectorError::ActiveRankMismatch {
                left: new_rank,
                right: self.active_rank,
            });
        }
        Ok(Self {
            planes: self.planes.clone(),
            active_rank: new_rank,
            horizon: self.horizon,
        })
    }

    #[inline]
    pub fn horizon(&self) -> HorizonContext {
        self.horizon
    }

    /// Coefficient of the given plane. Returns `None` if absent.
    pub fn coefficient(&self, idx: PlaneIndex) -> Option<&VectorModule<F>> {
        self.planes
            .binary_search_by_key(&idx.0, |(b, _)| b.0)
            .ok()
            .map(|i| &self.planes[i].1)
    }

    /// Maximum polynomial dimension (`degree + 1`) across all present
    /// planes — the multivector's polynomial extent. `0` iff the
    /// multivector is empty. This is the WITHIN-plane polynomial-degree
    /// measure (the DIM-LIFT.A degree regime), distinct from geometric
    /// grade; a non-empty multivector always reports `dim ≥ 1`.
    ///
    /// **Scope (auditor inscription):** max-across-planes here is a
    /// WITHIN-CARRIER, WITHIN-AXIS extent read (LAW XIV §4 method
    /// read on whole content). It is NOT precedent for a cross-axis
    /// `max()` — aggregating an extent across the per-axis sovereign
    /// fields remains the convicted D4-band shape (LAW V (a)).
    pub fn dim(&self) -> usize {
        self.planes.iter().map(|(_, p)| p.dim()).max().unwrap_or(0)
    }

    /// **Reverse** `Ã` — applies the per-plane reverse sign to every
    /// plane's polynomial coefficient.
    pub fn reverse(&self) -> Self {
        let planes = self
            .planes
            .iter()
            .map(|(idx, coeff)| {
                let sign = idx.reverse_sign();
                let signed = if sign < 0 {
                    coeff.negate()
                } else {
                    coeff.clone()
                };
                (*idx, signed)
            })
            .collect();
        Self {
            planes,
            active_rank: self.active_rank,
            horizon: self.horizon,
        }
    }

    /// Checked per-plane polynomial addition. Auto-pads shorter-degree
    /// polynomial coefficients with zero coefficients.
    ///
    /// **RANK MEET BY INHALE (2a supersession — per-axis fiber ranks are real):**
    /// same-field operands of different `active_rank` meet at the WIDER rank via
    /// the LAW-XI inhale `widened_to_rank` (the canonical inclusion `Cl_r ⊂ Cl_R`
    /// — every plane carried unchanged, lossless, byte-reversible). Capacity is
    /// never content; the former refusal guarded a one-rank era that the rank-10
    /// gauge palette fiber superseded. The inhale's own guard still refuses any
    /// NARROWING (that is an exhale and must route through `breath::exhale`).
    pub fn try_add(&self, other: &Self) -> Result<Self, MultivectorError> {
        if self.active_rank != other.active_rank {
            let meet = self.active_rank.max(other.active_rank);
            return self
                .widened_to_rank(meet)?
                .try_add(&other.widened_to_rank(meet)?);
        }
        let mut merged: Vec<(PlaneIndex, VectorModule<F>)> = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < self.planes.len() && j < other.planes.len() {
            let (a_idx, a_coeff) = &self.planes[i];
            let (b_idx, b_coeff) = &other.planes[j];
            match a_idx.0.cmp(&b_idx.0) {
                core::cmp::Ordering::Less => {
                    merged.push((*a_idx, a_coeff.clone()));
                    i += 1;
                }
                core::cmp::Ordering::Greater => {
                    merged.push((*b_idx, b_coeff.clone()));
                    j += 1;
                }
                core::cmp::Ordering::Equal => {
                    let sum = a_coeff.try_add(b_coeff).map_err(unwrap_plane_overflow)?;
                    if !sum.is_zero() {
                        merged.push((*a_idx, sum));
                    }
                    i += 1;
                    j += 1;
                }
            }
        }
        while i < self.planes.len() {
            merged.push(self.planes[i].clone());
            i += 1;
        }
        while j < other.planes.len() {
            merged.push(other.planes[j].clone());
            j += 1;
        }
        Ok(Self {
            planes: merged,
            active_rank: self.active_rank,
            horizon: self.horizon,
        })
    }

    /// Per-plane polynomial negation.
    pub fn negate(&self) -> Self {
        Self {
            planes: self
                .planes
                .iter()
                .map(|(idx, c)| (*idx, c.negate()))
                .collect(),
            active_rank: self.active_rank,
            horizon: self.horizon,
        }
    }

    /// Checked per-plane polynomial subtraction.
    pub fn try_sub(&self, other: &Self) -> Result<Self, MultivectorError> {
        self.try_add(&other.negate())
    }

    /// **Geometric product** — the substrate's load-bearing
    /// composition. For each pair of input planes `(a, p_a)` and
    /// `(b, p_b)`:
    ///
    ///   * Plane index: `a XOR b` (shared bits drop out)
    ///   * Sign: `a.product_sign(b)` (inversion parity)
    ///   * **Polynomial coefficient: `convolve(p_a, p_b)` × sign**
    ///
    /// THIS is where the substrate's `p^N` denominator growth from
    /// successive transport compositions lives. Each product convolves
    /// the input plane polynomials, multiplying their effective
    /// denominators by the smooth-prime profile base.
    pub fn geometric_product(&self, other: &Self) -> Result<Self, MultivectorError> {
        if self.active_rank != other.active_rank {
            // RANK MEET BY INHALE (see `try_add`) — the doc at `widened_to_rank`
            // names exactly this use: meet on a common basis width instead of an
            // `ActiveRankMismatch` refusal.
            let meet = self.active_rank.max(other.active_rank);
            return self
                .widened_to_rank(meet)?
                .geometric_product(&other.widened_to_rank(meet)?);
        }
        let mut accumulator: Vec<(PlaneIndex, VectorModule<F>)> = Vec::new();
        for (a_idx, a_coeff) in &self.planes {
            for (b_idx, b_coeff) in &other.planes {
                let (out_idx, sign) = a_idx.geometric_product(*b_idx);
                let convolved = a_coeff.convolve(b_coeff).map_err(unwrap_plane_overflow)?;
                let signed = if sign < 0 {
                    convolved.negate()
                } else {
                    convolved
                };
                if signed.is_zero() {
                    continue;
                }
                match accumulator.binary_search_by_key(&out_idx.0, |(b, _)| b.0) {
                    Ok(found) => {
                        let merged = accumulator[found]
                            .1
                            .try_add(&signed)
                            .map_err(unwrap_plane_overflow)?;
                        if merged.is_zero() {
                            accumulator.remove(found);
                        } else {
                            accumulator[found].1 = merged;
                        }
                    }
                    Err(insert_at) => accumulator.insert(insert_at, (out_idx, signed)),
                }
            }
        }
        Ok(Self {
            planes: accumulator,
            active_rank: self.active_rank,
            horizon: self.horizon,
        })
    }

    /// **#40 Fork-2 — e₀-split idempotent** `P = (1 + e₀)/2`.
    ///
    /// `e₀` (plane bit 0) is a metric-verified `+1` vector — the field
    /// signature is Euclidean `(e_i)² = +1` (only the CGA bit-15 carries
    /// negative signature), so `e₀² = +1`, in-rank for any `active_rank ≥ 1`.
    /// Hence `P² = (1 + 2e₀ + e₀²)/4 = (2 + 2e₀)/4 = P`: a genuine idempotent.
    /// It defines the splitting that makes [`unipotent_generator`] nilpotent;
    /// the `1/2` denominator lies in every field's smooth-prime profile.
    ///
    /// [`unipotent_generator`]: Multivector::unipotent_generator
    fn e0_split_idempotent(active_rank: u8, horizon: HorizonContext) -> Self {
        let half = Rational::<F>::try_new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2))
            .expect("1/2 ∈ every smooth-prime profile (denominator 2)");
        let half_vm = VectorModule::<F>::singleton(half);
        let e0 = PlaneIndex::single(0).expect("e₀ (bit 0) is in-rank for active_rank ≥ 1");
        Self::from_planes(
            &[(PlaneIndex::SCALAR, half_vm.clone()), (e0, half_vm)],
            active_rank,
            horizon,
        )
        .expect("e₀-split idempotent planes are in-rank and in-profile")
    }

    /// The algebraic identity `1` — the unit polynomial `[1]` at the grade-0
    /// COORDINATE (`PlaneIndex::SCALAR`), built through the coordinate-addressed
    /// constructor. The grade-0 coordinate is a legitimate *address* (the
    /// identity lives there); this is NOT the `scalar`-confinement collapse — it
    /// is the unit of the FULL algebra, extensible by any other coordinate.
    /// Used wherever `1` is structurally required (`1+N`, the `e₀` split).
    pub fn identity(active_rank: u8, horizon: HorizonContext) -> Self {
        Self::from_planes(
            &[(
                PlaneIndex::SCALAR,
                VectorModule::<F>::singleton(Rational::<F>::one()),
            )],
            active_rank,
            horizon,
        )
        .expect("identity [1] at the grade-0 coordinate is in-rank and in-profile")
    }

    /// **#40 Fork-2 — unipotent generator** `N = (1−P)·X·P` from the residual
    /// `X = self`, with `P` the `e₀`-split idempotent. Nilpotent by
    /// construction: `N² = (1−P)·X·[P(1−P)]·X·P = (1−P)·X·0·X·P = 0` (since
    /// `P(1−P)=0`), so the unipotent `1+N` has the exact, profile-free inverse
    /// `1−N` — no `1+β²` denominator ever forms. `X` is the grade-wise cocycle
    /// residual (the orb's direction); `N` is its projection onto the nilpotent
    /// face of the `e₀` split. The native closure gate `(1+N)(1−N)=1`
    /// empirically self-checks both the nilpotency and the idempotent.
    /// **The typed unity read — the ONE structural home of the closure
    /// comparison (LAW I).** Does `self·other` close on the algebra's 1?
    /// The comparison lives inside this verdict constructor and nowhere
    /// else; engine code consumes the arms (`Closes` / `Open{residual}`)
    /// and never writes the comparison. The open arm carries the residual
    /// `self·other − 1` whole (LAW V).
    pub fn unity_verdict(&self, other: &Self) -> Result<UnityVerdict<F>, MultivectorError> {
        let product = self.geometric_product(other)?;
        let one = Self::identity(product.active_rank(), product.horizon());
        let residual = product.try_sub(&one)?;
        match residual.is_zero() {
            true => Ok(UnityVerdict::Closes),
            false => Ok(UnityVerdict::Open { residual }),
        }
    }

    pub fn unipotent_generator(&self) -> Result<Self, MultivectorError> {
        let p = Self::e0_split_idempotent(self.active_rank(), self.horizon());
        let one = Self::identity(self.active_rank(), self.horizon());
        let one_minus_p = one.try_sub(&p)?;
        one_minus_p.geometric_product(self)?.geometric_product(&p)
    }

    /// Wire format. Per-plane: `[u64 LE plane_bits][polynomial wire]`.
    /// Polynomial wire: `[u32 LE dim][dim × (BigInt-numer, BigInt-denom)]`.
    /// Header: `[u8 active_rank][u64 LE horizon_depth][u32 LE plane_count]`.
    pub fn to_bytes(&self) -> Vec<u8> {
        use crate::wire::write_bigint_le;
        let mut buf = Vec::with_capacity(32);
        buf.push(self.active_rank);
        buf.extend_from_slice(&self.horizon.depth().to_le_bytes());
        let count = u32::try_from(self.planes.len()).expect(
            "Multivector plane count fits in u32 — substrate K-step bounded growth produces \
             <<2^32 planes per multivector",
        );
        buf.extend_from_slice(&count.to_le_bytes());
        for (idx, poly) in &self.planes {
            buf.extend_from_slice(&idx.0.to_le_bytes());
            let dim = u32::try_from(poly.dim()).expect("polynomial dim fits in u32");
            buf.extend_from_slice(&dim.to_le_bytes());
            for coeff in poly.components() {
                write_bigint_le(&mut buf, coeff.numer());
                write_bigint_le(&mut buf, coeff.denom());
            }
        }
        buf
    }

    /// Decode from wire format. Per-axis smooth-prime profile
    /// validation runs inside each `Rational::try_new`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MultivectorWireError> {
        use crate::wire::read_bigint_le;
        if bytes.len() < 13 {
            return Err(MultivectorWireError::Truncated {
                needed: 13,
                actual: bytes.len(),
            });
        }
        let active_rank = bytes[0];
        // [u64 LE horizon_depth] at bytes[1..9] (was a single byte before the depth widened to u64).
        let depth = u64::from_le_bytes([
            bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
        ]);
        let horizon = HorizonContext::new(depth)
            .map_err(|_| MultivectorWireError::InvalidHorizon { depth })?;
        let plane_count = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]) as usize;
        let mut cursor = 13usize;
        let mut planes: Vec<(PlaneIndex, VectorModule<F>)> = Vec::new();
        for _ in 0..plane_count {
            if cursor + 12 > bytes.len() {
                return Err(MultivectorWireError::Truncated {
                    needed: cursor + 12,
                    actual: bytes.len(),
                });
            }
            let plane_bits = u64::from_le_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
                bytes[cursor + 4],
                bytes[cursor + 5],
                bytes[cursor + 6],
                bytes[cursor + 7],
            ]);
            cursor += 8;
            let dim = u32::from_le_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
            ]) as usize;
            cursor += 4;
            let mut coefficients = Vec::with_capacity(dim);
            for _ in 0..dim {
                let (numer, consumed) =
                    read_bigint_le(&bytes[cursor..]).map_err(MultivectorWireError::Decode)?;
                cursor = cursor
                    .checked_add(consumed)
                    .ok_or(MultivectorWireError::CursorOverflow { offset: cursor })?;
                let (denom, consumed) =
                    read_bigint_le(&bytes[cursor..]).map_err(MultivectorWireError::Decode)?;
                cursor = cursor
                    .checked_add(consumed)
                    .ok_or(MultivectorWireError::CursorOverflow { offset: cursor })?;
                let coeff =
                    Rational::<F>::try_new(numer, denom).map_err(MultivectorWireError::Decode)?;
                coefficients.push(coeff);
            }
            planes.push((
                PlaneIndex(plane_bits),
                VectorModule::<F>::from_vec(coefficients),
            ));
        }
        Ok(Self {
            planes,
            active_rank,
            horizon,
        })
    }
}

fn unwrap_plane_overflow(err: VectorModuleError) -> MultivectorError {
    match err {
        VectorModuleError::Arithmetic { source } => MultivectorError::Arithmetic { source },
        VectorModuleError::AxisOutOfBounds { axis, dim } => {
            MultivectorError::PolynomialAxisOutOfBounds { axis, dim }
        }
    }
}

/// The typed verdict of the unity read (`Multivector::unity_verdict`) —
/// the closure comparison's one structural home. Consumers dispatch on
/// arms; the open arm carries the whole residual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnityVerdict<F: Field> {
    /// `a·b` closes on the algebra's 1.
    Closes,
    /// `a·b − 1`, carried whole.
    Open { residual: Multivector<F> },
}

/// Structured residual of a multivector operation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MultivectorError {
    #[error("plane {plane:?} exceeds active rank {active_rank}")]
    PlaneOutOfRank { plane: PlaneIndex, active_rank: u8 },

    #[error("invalid pair plane ({i}, {j}): require i < j and j < {PLANE_BIT_WIDTH}")]
    InvalidPairPlane { i: u8, j: u8 },

    #[error("active-rank mismatch: left={left}, right={right}")]
    ActiveRankMismatch { left: u8, right: u8 },

    #[error("multivector arithmetic overflow: {source}")]
    Arithmetic {
        #[from]
        source: OverflowError,
    },

    #[error("polynomial axis index {axis} out of bounds (dim={dim})")]
    PolynomialAxisOutOfBounds { axis: usize, dim: usize },
}

/// Structured residual of a multivector wire decode.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MultivectorWireError {
    #[error("multivector wire-format truncated: needed {needed}, got {actual}")]
    Truncated { needed: usize, actual: usize },

    #[error("multivector wire-format invalid horizon depth {depth}")]
    InvalidHorizon { depth: u64 },

    #[error("multivector wire-format cursor overflow at offset {offset}")]
    CursorOverflow { offset: usize },

    #[error("multivector wire-format decode failure: {0}")]
    Decode(OverflowError),
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
    fn zero_multivector_is_zero() {
        let m = Multivector::<Space>::zero(4, HorizonContext::default());
        assert!(m.is_zero());
        assert_eq!(m.plane_count(), 0);
    }

    #[test]
    fn from_planes_strips_empty_polynomials() {
        let planes = vec![
            (
                PlaneIndex::SCALAR,
                VectorModule::<Space>::singleton(r(3, 1)),
            ),
            (
                PlaneIndex::from_raw(0b11),
                VectorModule::<Space>::zero_polynomial(),
            ),
        ];
        // SINGLETON-BOUNDARY [test fixture]: degree-0 scalar coefficient
        // at K=0 by construction — test exercises empty-polynomial
        // stripping. Polynomial growth begins at the first geometric_product
        // call against this fixture in subsequent composition tests;
        // this site is the seed, not a collapse.
        let m =
            Multivector::<Space>::from_planes(&planes, 4, HorizonContext::default()).expect("ok");
        assert_eq!(m.plane_count(), 1);
        assert_eq!(m.iter_planes().next().unwrap().0, PlaneIndex::SCALAR);
    }

    #[test]
    fn product_sign_canonical_cases() {
        let e0 = PlaneIndex::from_raw(0b01);
        let e1 = PlaneIndex::from_raw(0b10);
        // e_0 * e_1 = +e_0 e_1
        assert_eq!(e0.product_sign(e1), 1);
        // e_1 * e_0 = -e_0 e_1
        assert_eq!(e1.product_sign(e0), -1);
    }

    #[test]
    fn geometric_product_anticommutes_grade_1() {
        let e0 = PlaneIndex::from_raw(0b01);
        let e1 = PlaneIndex::from_raw(0b10);
        let (idx_01, s_01) = e0.geometric_product(e1);
        let (idx_10, s_10) = e1.geometric_product(e0);
        assert_eq!(idx_01, PlaneIndex::from_raw(0b11));
        assert_eq!(idx_10, PlaneIndex::from_raw(0b11));
        assert_eq!(s_01, 1);
        assert_eq!(s_10, -1);
    }

    /// **Correction 1 witness — CGA signature.** `e_-² = -1` flips
    /// the sign of the geometric product when bit-15 is shared.
    #[test]
    fn cga_negative_signature_e_minus_squared_is_negative_one() {
        let e_minus = PlaneIndex::from_raw(1u64 << NEG_SIG_BIT);
        let (idx, sign) = e_minus.geometric_product(e_minus);
        // e_- · e_- = -1 (scalar, negative sign)
        assert_eq!(idx, PlaneIndex::SCALAR);
        assert_eq!(sign, -1);
    }

    /// **Correction 1 witness.** `e_+² = +1` matches Euclidean
    /// signature (no extra sign flip at bit 14).
    #[test]
    fn cga_positive_signature_e_plus_squared_is_positive_one() {
        let e_plus = PlaneIndex::from_raw(1u64 << PLUS_SIG_BIT);
        let (idx, sign) = e_plus.geometric_product(e_plus);
        assert_eq!(idx, PlaneIndex::SCALAR);
        assert_eq!(sign, 1);
    }

    /// **Correction 1 witness.** The CGA null-vector pair-plane
    /// `e_0 ∧ e_∞`-style content (here `e_+ ∧ e_-`) squares to
    /// `(e_+ e_-)(e_+ e_-) = -e_+ e_+ e_- e_- = -(+1)(-1) = +1`.
    /// Tests the signature handling combined with anticommutation.
    #[test]
    fn cga_plus_wedge_minus_squared_is_positive_one() {
        let e_plus = 1u64 << PLUS_SIG_BIT;
        let e_minus_mask = 1u64 << NEG_SIG_BIT;
        let plus_wedge_minus = PlaneIndex::from_raw(e_plus | e_minus_mask);
        let (idx, sign) = plus_wedge_minus.geometric_product(plus_wedge_minus);
        // (e_+ ∧ e_-)² = e_+ e_- e_+ e_- = -e_+ e_+ e_- e_- = -(1)(-1) = +1
        assert_eq!(idx, PlaneIndex::SCALAR);
        assert_eq!(sign, 1);
    }

    #[test]
    fn reverse_sign_per_grade() {
        // grade 0: +1, grade 1: +1, grade 2: -1, grade 3: -1, grade 4: +1.
        assert_eq!(PlaneIndex::SCALAR.reverse_sign(), 1);
        assert_eq!(PlaneIndex::single(0).unwrap().reverse_sign(), 1);
        assert_eq!(PlaneIndex::pair(0, 1).unwrap().reverse_sign(), -1);
        let trivector = PlaneIndex::from_raw(0b111);
        assert_eq!(trivector.reverse_sign(), -1);
    }

    #[test]
    fn try_add_per_plane_polynomial() {
        let mv1 = Multivector::<Space>::from_planes(
            &[(
                PlaneIndex::SCALAR,
                VectorModule::<Space>::singleton(r(1, 2)),
            )],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        let mv2 = Multivector::<Space>::from_planes(
            &[(
                PlaneIndex::SCALAR,
                VectorModule::<Space>::singleton(r(1, 3)),
            )],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        let sum = mv1.try_add(&mv2).expect("ok");
        let c = sum.coefficient(PlaneIndex::SCALAR).expect("scalar present");
        // 1/2 + 1/3 = 5/6 (degree-0)
        assert_eq!(c.coefficient_at(0).numerator(), &BigInt::from(5));
        assert_eq!(c.coefficient_at(0).denominator(), &BigInt::from(6));
    }

    #[test]
    fn geometric_product_grows_polynomial_via_convolve() {
        // Multivectors with linear-polynomial plane content:
        //   A = (1 + x) · e_0 ∧ e_1
        //   B = (1 + x) · e_0 ∧ e_1
        // A · B has pair-plane × pair-plane product:
        //   (e_0 ∧ e_1) · (e_0 ∧ e_1) = e_0 e_1 e_0 e_1 = -e_0 e_0 e_1 e_1 = -1
        // So result plane is SCALAR with polynomial (1+x)·(1+x)·(-1) = -(1 + 2x + x^2)
        let linear_p = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 1)]);
        let pair_plane = PlaneIndex::pair(0, 1).unwrap();
        let a = Multivector::<Space>::from_planes(
            &[(pair_plane, linear_p.clone())],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        let prod = a.geometric_product(&a).expect("ok");
        let scalar_coeff = prod
            .coefficient(PlaneIndex::SCALAR)
            .expect("scalar present");
        // -(1 + 2x + x^2) → dim = 3 (degree 2)
        assert_eq!(scalar_coeff.dim(), 3);
        assert_eq!(
            scalar_coeff.coefficient_at(0).numerator(),
            &BigInt::from(-1)
        );
        assert_eq!(
            scalar_coeff.coefficient_at(1).numerator(),
            &BigInt::from(-2)
        );
        assert_eq!(
            scalar_coeff.coefficient_at(2).numerator(),
            &BigInt::from(-1)
        );
    }

    #[test]
    fn wire_round_trip_preserves_polynomial_plane_content() {
        let pair_plane = PlaneIndex::pair(0, 1).unwrap();
        let linear_p = VectorModule::<Space>::from_vec(vec![r(1, 1), r(1, 2)]);
        let mv = Multivector::<Space>::from_planes(
            &[(pair_plane, linear_p)],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        let bytes = mv.to_bytes();
        let back = Multivector::<Space>::from_bytes(&bytes).expect("ok");
        assert_eq!(back, mv);
    }

    #[test]
    fn reverse_negates_grade_2_planes_only() {
        let e0 = PlaneIndex::single(0).unwrap();
        let pair_plane = PlaneIndex::pair(0, 1).unwrap();
        // SINGLETON-BOUNDARY [test fixture]: per-plane degree-0 coefficient
        // by construction — test exercises reverse on per-grade signs.
        // Polynomial growth begins at any subsequent geometric_product in
        // production composition code; this site is the seed, not a collapse.
        let mv = Multivector::<Space>::from_planes(
            &[
                (e0, VectorModule::<Space>::singleton(r(2, 1))),
                (pair_plane, VectorModule::<Space>::singleton(r(3, 1))),
            ],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        let rev = mv.reverse();
        // grade-1 stays positive; grade-2 flips
        assert_eq!(
            rev.coefficient(e0).unwrap().coefficient_at(0).numerator(),
            &BigInt::from(2)
        );
        assert_eq!(
            rev.coefficient(pair_plane)
                .unwrap()
                .coefficient_at(0)
                .numerator(),
            &BigInt::from(-3)
        );
    }

    #[test]
    fn the_unity_verdict_open_arm_carries_the_residual_whole() {
        let one = Multivector::<Space>::identity(4, HorizonContext::default());
        assert_eq!(
            one.unity_verdict(&one).expect("composes"),
            UnityVerdict::Closes
        );
        let two = Multivector::<Space>::grade0(
            VectorModule::<Space>::from_vec(vec![r(2, 1)]),
            4,
            HorizonContext::default(),
        );
        match one.unity_verdict(&two).expect("composes") {
            UnityVerdict::Open { residual } => {
                // 1·2 − 1 = 1 — the residual is the whole grade-0 [1].
                assert_eq!(residual, one);
            }
            UnityVerdict::Closes => panic!("1·2 must not close"),
        }
    }

    #[test]
    fn identity_is_the_multiplicative_unit() {
        let one = Multivector::<Space>::identity(4, HorizonContext::default());
        // A grade-1 carrier with a non-trivial polynomial on e_1.
        let v = Multivector::<Space>::from_planes(
            &[(
                PlaneIndex::single(1).unwrap(),
                VectorModule::<Space>::from_vec(vec![r(1, 2), r(1, 3)]),
            )],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        assert_eq!(one.geometric_product(&v).unwrap(), v);
    }

    #[test]
    fn scalar_content_lands_in_the_unipotent_kernel() {
        // **Collapse witness (the verified=0 base cause).** Grade-0 (scalar)
        // content commutes with e₀, so N=(1−P)·X·P = X·(P−P²) = 0: a scalar
        // produces NO morphic motion. This is why content folded onto the
        // scalar plane left the signature byte-identical.
        let scalar_x = Multivector::<Space>::from_planes(
            &[(
                PlaneIndex::SCALAR,
                VectorModule::<Space>::singleton(r(5, 1)),
            )],
            4,
            HorizonContext::default(),
        )
        .unwrap();
        let n = scalar_x.unipotent_generator().unwrap();
        assert!(n.is_zero(), "scalar content must be in the kernel of N");
    }

}
