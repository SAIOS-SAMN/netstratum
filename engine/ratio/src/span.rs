//! THE SPAN (docs/78, design law 11, the number grammar; operator
//! 2026-10-17): ℤ and ℚ are the singularity's two poles, ℝ the span between
//! them. ℝ is a SPAN, never a stored value: it is held by its two exact
//! bounds, and anything read inside it is a certified enclosure narrowed
//! until it decides. No real and no float stands in a law or on the ledger;
//! a quantity that is neither a count (ℤ), a share (ℚ), nor a span read at
//! its exact bounds, is in the wrong place.

use crate::Ratio;
use num_traits::Zero;

/// A span of ℝ, held only by its two exact rational bounds, `lo ≤ hi`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    lo: Ratio,
    hi: Ratio,
}

impl Span {
    /// The span between two exact bounds, in either order.
    pub fn between(a: Ratio, b: Ratio) -> Span {
        if a <= b { Span { lo: a, hi: b } } else { Span { lo: b, hi: a } }
    }

    /// A span of no extent: an exact value (a count or a share).
    pub fn exact(x: Ratio) -> Span {
        Span { lo: x.clone(), hi: x }
    }

    pub fn lo(&self) -> &Ratio {
        &self.lo
    }

    pub fn hi(&self) -> &Ratio {
        &self.hi
    }

    /// Its extent, `hi − lo`: the distance between its poles.
    pub fn extent(&self) -> Ratio {
        self.hi.clone() - self.lo.clone()
    }

    /// Whether it has narrowed to a single exact value.
    pub fn is_exact(&self) -> bool {
        self.extent().is_zero()
    }

    /// Whether an exact value lies within it (bounds included).
    pub fn contains(&self, x: &Ratio) -> bool {
        *x >= self.lo && *x <= self.hi
    }

    /// Its middle, exact.
    pub fn mid(&self) -> Ratio {
        (self.lo.clone() + self.hi.clone()) / Ratio::from_integer(2.into())
    }

    /// The two halves, about its middle.
    pub fn halves(&self) -> (Span, Span) {
        let m = self.mid();
        (Span { lo: self.lo.clone(), hi: m.clone() }, Span { lo: m, hi: self.hi.clone() })
    }

    /// The sum of two spans: bounds add (exact interval arithmetic).
    pub fn add(&self, o: &Span) -> Span {
        Span { lo: self.lo.clone() + o.lo.clone(), hi: self.hi.clone() + o.hi.clone() }
    }

    /// The product of two spans: the least and greatest of the bounds'
    /// products (distinct readings; never a square of one value).
    pub fn mul(&self, o: &Span) -> Span {
        let p = [self.lo.clone() * o.lo.clone(), self.lo.clone() * o.hi.clone(), self.hi.clone() * o.lo.clone(), self.hi.clone() * o.hi.clone()];
        let lo = p.iter().min().cloned().unwrap_or_else(Ratio::zero);
        let hi = p.iter().max().cloned().unwrap_or_else(Ratio::zero);
        Span { lo, hi }
    }

    /// Where it meets another; None if they are apart.
    pub fn meet(&self, o: &Span) -> Option<Span> {
        let lo = self.lo.clone().max(o.lo.clone());
        let hi = self.hi.clone().min(o.hi.clone());
        (lo <= hi).then_some(Span { lo, hi })
    }

    /// DECIDE a comparison against an exact threshold, narrowing a span that
    /// a reading refines (`refine` returns a narrower span inside the one
    /// given): Some(true) once the whole span stands at or above the
    /// threshold, Some(false) once it stands below, None if `rounds`
    /// refinements leave it straddling (refused, never rounded).
    pub fn decide(&self, threshold: &Ratio, rounds: u32, refine: impl Fn(&Span) -> Span) -> Option<bool> {
        let mut s = self.clone();
        for _ in 0..=rounds {
            if s.lo >= *threshold {
                return Some(true);
            }
            if s.hi < *threshold {
                return Some(false);
            }
            let next = refine(&s);
            if next == s {
                return None;
            }
            s = next;
        }
        None
    }
}

/// THE CORE'S POLES (design law 11): a manifest's HOLDING is its ℤ pole
/// (closed pairs, counted whole), its MAGNETISM its ℚ pole (the open frames);
/// the core's STRENGTH is the span between them, holding plus magnetism (3Z
/// a nucleus); its BALANCE is the share at the ℤ pole, holding over
/// strength (burning moves magnetism into holding with 3Z fixed: a core
/// hardens toward 1).
pub fn poles(holding: &Ratio, magnetism: &Ratio) -> (Span, Ratio) {
    let strength = holding.clone() + magnetism.clone();
    let balance = if strength.is_zero() { Ratio::zero() } else { holding.clone() / strength.clone() };
    (Span::between(-magnetism.clone(), holding.clone()), balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio;

    #[test]
    fn a_span_is_held_only_by_its_exact_bounds() {
        let s = Span::between(ratio(3, 4), ratio(1, 4));
        assert_eq!((s.lo(), s.hi()), (&ratio(1, 4), &ratio(3, 4)));
        assert_eq!(s.extent(), ratio(1, 2));
        assert!(s.contains(&ratio(1, 2)) && !s.contains(&ratio(1, 1)));
        assert!(Span::exact(ratio(2, 3)).is_exact());
    }

    #[test]
    fn deciding_narrows_and_refuses_rather_than_rounds() {
        // A span straddling ½ narrowed by halving toward a value it encloses.
        let target = ratio(5, 8);
        let narrow = |s: &Span| { let (a, b) = s.halves(); if a.contains(&target) { a } else { b } };
        assert_eq!(Span::between(ratio(0, 1), ratio(1, 1)).decide(&ratio(1, 2), 8, narrow), Some(true));
        // A value exactly at the threshold never resolves below: decided true.
        let at = |s: &Span| { let (a, b) = s.halves(); if a.contains(&ratio(1, 2)) && a.hi() != &ratio(1, 2) { a } else { b } };
        assert_eq!(Span::between(ratio(0, 1), ratio(1, 1)).decide(&ratio(1, 2), 8, at), Some(true));
        // No refinement: refused.
        assert_eq!(Span::between(ratio(0, 1), ratio(1, 1)).decide(&ratio(1, 2), 8, |s| s.clone()), None);
    }

    #[test]
    fn a_cores_strength_is_the_span_between_its_poles() {
        // Two hydrogen nuclei: holding 4, magnetism 2 → strength 6 (3Z·2).
        let (span, balance) = poles(&ratio(4, 1), &ratio(2, 1));
        assert_eq!(span.extent(), ratio(6, 1));
        assert_eq!(balance, ratio(2, 3));
        // Fused to helium: holding 6, magnetism 0: the same strength, hardened.
        let (span, balance) = poles(&ratio(6, 1), &ratio(0, 1));
        assert_eq!((span.extent(), balance), (ratio(6, 1), ratio(1, 1)));
    }
}
