//! **THE ENVIRONMENT GRAIN** (operator ruling 2026-08-20) — the civilization
//! WORLD quantizing its OWN physics at a defined joint, and the law that says
//! where that quantization may never spread.
//!
//! ## The doctrinal boundary — read this before bounding any denominator
//!
//! The exactness law protects the **judge**. Anything that becomes an ω, a word,
//! an offer, a replay, or a verdict stays exact end-to-end, forever, no
//! exceptions. This grain is **not** the judge — it is the *environment* (melt,
//! η, Keplerian orbital separation: the civilization world's own physics)
//! bounding the exact-rational denominator growth that prices its pressure,
//! exactly as the leisure-bank truncation precedent established. Two hard walls,
//! ENFORCED, not merely documented:
//!
//!  1. **GRAIN RATES, NEVER STOCKS.** A conserved ledger — element amounts,
//!     banked matter — must be bit-exact `before == after` a trade. Rounding a
//!     stock is matter creation and is refused. Only the COEFFICIENTS that price
//!     a trade (melt, η, orbital separation) are grained; the trade amount, once
//!     priced, moves exactly.
//!  2. **UNREACHABLE FROM THE JUDGE.** `grain_rate` is `pub(crate)` — the
//!     `solver`, `scribe`, and `membrane` crates cannot name it — and a negative
//!     witness (`cfe/tests/grain_boundary.rs`) fails loudly if any judge-side
//!     source ever references it.
//!
//! If you came here to grain something, the boundary above is the law: is your
//! value a *rate the world prices with*, or a *thing the judge will weigh*? Only
//! the first may pass through here.

use crate::ratio::Ratio;
use num_bigint::BigInt;

/// The largest denominator the environment grain keeps. **DERIVED, not chosen**
/// (Directive 1 — no constant wearing a beard): the bound whose per-pair
/// `route_melt` cost holds the beat budget, measured live — the landing commit
/// records the measurement that fixes this number, and it is never hand-edited;
/// the derivation re-runs if the wall moves. (Placeholder until the live
/// measurement lands; wiring is inert until then.)
#[allow(dead_code)] // grain organ landed unwired (8ccf6f7); re-wired only when the trace convicts an arithmetic input
pub(crate) const GRAIN_DENOM_MAX: i64 = 4096;

/// **GRAIN A RATE** — the deterministic best-rational-approximation of `x` to a
/// bounded denominator `q <= GRAIN_DENOM_MAX`, by continued-fraction convergents
/// (the last convergent whose denominator fits the bound).
///
/// - **Deterministic:** a pure function of `x`.
/// - **Idempotent:** `grain_rate(grain_rate(x)) == grain_rate(x)` — a value whose
///   denominator already fits is returned untouched (the fast path).
/// - **Bounded:** `|x - grain_rate(x)| <= 1/(q · GRAIN_DENOM_MAX)`, where `q` is
///   the returned denominator: the convergent bound `|x - h/k| < 1/(k·k_next)`
///   with `k_next > GRAIN_DENOM_MAX`.
///
/// **Rates only** — see the module boundary. Never call this on a conserved stock.
#[allow(dead_code)] // grain organ landed unwired; wired only on a trace-convicted input
pub(crate) fn grain_rate(x: &Ratio) -> Ratio {
    let zero = BigInt::from(0);
    let bound = BigInt::from(GRAIN_DENOM_MAX);
    // num_rational keeps `denom() > 0` and the fraction reduced.
    if x.denom() <= &bound {
        return x.clone(); // already within the grain — idempotent fast path
    }
    // Sign out; work on |x| = n/d, both positive. grain_rate is odd in x.
    let neg = *x.numer() < zero;
    let mut n = if neg { -x.numer() } else { x.numer().clone() };
    let mut d = x.denom().clone();
    // Convergent recurrence: h_{-2}=0, h_{-1}=1 ; k_{-2}=1, k_{-1}=0.
    let mut h_prev = BigInt::from(0);
    let mut h_cur = BigInt::from(1);
    let mut k_prev = BigInt::from(1);
    let mut k_cur = BigInt::from(0);
    loop {
        let a = &n / &d; // floor division (n, d > 0)
        let k_next = &a * &k_cur + &k_prev;
        if k_next > bound {
            break; // this convergent overshoots the bound — keep the last that fit
        }
        let h_next = &a * &h_cur + &h_prev;
        h_prev = std::mem::replace(&mut h_cur, h_next);
        k_prev = std::mem::replace(&mut k_cur, k_next);
        let rem = &n - &a * &d;
        if rem == zero {
            break; // x terminated exactly at a fitting convergent
        }
        n = std::mem::replace(&mut d, rem);
    }
    let numer = if neg { -h_cur } else { h_cur };
    Ratio::new(numer, k_cur)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::ratio;

    // A rate with a denominator far past the bound — the CF must reduce it.
    fn wild() -> Ratio {
        ratio(31_415_926, 100_000_000) // ~pi/10, denom 10^8 (reduced)
    }

    #[test]
    fn grain_is_deterministic_and_fits_the_bound() {
        let g = grain_rate(&wild());
        assert_eq!(g, grain_rate(&wild()), "a pure function of x");
        assert!(
            *g.denom() <= BigInt::from(GRAIN_DENOM_MAX),
            "the grained denominator fits the bound: {}",
            g.denom()
        );
    }

    #[test]
    fn grain_is_idempotent() {
        let g = grain_rate(&wild());
        assert_eq!(grain_rate(&g), g, "grain_rate(grain_rate(x)) == grain_rate(x)");
    }

    #[test]
    fn grain_holds_the_approximation_bound() {
        let x = wild();
        let g = grain_rate(&x);
        let diff = &x - &g;
        let err = if diff < ratio(0, 1) { -diff } else { diff };
        // |x - grain(x)| <= 1/(q · Q_max)
        let ceiling = ratio(1, 1) / (g.denom().clone() * BigInt::from(GRAIN_DENOM_MAX));
        assert!(err <= ceiling, "err {err} exceeds 1/(q·Qmax) {ceiling}");
    }

    #[test]
    fn a_fitting_denominator_passes_through_untouched() {
        // Already within the grain — returned bit-identical (the fast path).
        let fine = ratio(3, 64);
        assert_eq!(grain_rate(&fine), fine);
        let edge = ratio(7, GRAIN_DENOM_MAX);
        assert_eq!(grain_rate(&edge), edge, "a denominator == the bound still fits");
    }

    #[test]
    fn grain_is_odd_sign_is_carried() {
        let x = wild();
        assert_eq!(grain_rate(&(-&x)), -grain_rate(&x), "grain(-x) == -grain(x)");
        assert_eq!(grain_rate(&ratio(0, 1)), ratio(0, 1), "zero grains to zero");
    }
}
