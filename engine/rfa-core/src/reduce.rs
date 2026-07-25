//! Coboundary reduction: project onto the canonical H¹ representative,
//! then integer-normalize.
//!
//! Two Deltas that differ only by a coboundary (a coordinate-frame
//! distortion) reduce to the same representative; a pure coboundary
//! reduces to zero. What survives is the structural content — the
//! cocycle class.

use crate::delta::Delta;
use cfe::ratio::Ratio;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// Reduce `delta` to the canonical representative of its H¹ class.
///
/// Phase 1 — projection: for every pair `i < j` and coordinate `l`,
/// `Δ'_ij,l = Δ_ij,l − Δ_0j,l + Δ_0i,l` — subtract the coboundary
/// induced by anchoring every relation at vertex 0. Every relation
/// touching vertex 0 lands exactly on zero; the surviving entries are
/// the frame-free content.
///
/// Phase 2 — integer normalization: multiply through by the lcm of all
/// denominators, then divide by the gcd of all numerators. Every
/// coordinate becomes an integer (denominator 1) and the representative
/// is scale-canonical. The zero tensor passes through unchanged.
pub fn coboundary_reduce(delta: &Delta) -> Delta {
    let dim = delta.dim();
    let fiber = delta.fiber();
    let mut reduced = Delta::zero(dim, fiber);

    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| delta.entry(i, j, l) - delta.entry(0, j, l) + delta.entry(0, i, l))
                .collect();
            reduced.set_antisym(i, j, coords);
        }
    }

    // Integer normalization over the independent entries (i < j).
    let mut lcm_denom = BigInt::one();
    let mut gcd_numer = BigInt::zero();
    for i in 0..dim {
        for j in (i + 1)..dim {
            for l in 0..fiber {
                let e = reduced.entry(i, j, l);
                if e.is_zero() {
                    continue;
                }
                lcm_denom = lcm_denom.lcm(e.denom());
                gcd_numer = gcd_numer.gcd(&e.numer().abs());
            }
        }
    }
    if gcd_numer.is_zero() {
        // Every entry is zero — nothing to normalize.
        return reduced;
    }

    let scale = Ratio::new(lcm_denom, gcd_numer);
    let mut normalized = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| reduced.entry(i, j, l) * &scale)
                .collect();
            normalized.set_antisym(i, j, coords);
        }
    }
    normalized
}
