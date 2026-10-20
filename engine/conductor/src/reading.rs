//! A reading as exact as its carrier (MU-RECURSION law 4): a value served
//! exactly where it stands on a mark of the sexagesimal protractor, else the
//! sector between the two marks it lies in.
use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// The sexagesimal places a sector carries below its leading place.
pub const PLACES: i64 = 2;

/// A value read on the sexagesimal protractor: `mark·60^step`, the value
/// itself where it lies on that mark (`exact`), else in `[mark, mark + 1)·60^step`
/// (by magnitude, the sign carried apart).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sector {
    pub negative: bool,
    pub mark: BigInt,
    pub step: i64,
    pub exact: bool,
}

fn sixty(e: i64) -> Ratio {
    let p = Ratio::from_integer(BigInt::from(60).pow(e.unsigned_abs() as u32));
    if e >= 0 { p } else { Ratio::one() / p }
}

impl Sector {
    /// The lower mark, `mark·60^step`, signed.
    pub fn lo(&self) -> Ratio {
        let m = Ratio::from_integer(self.mark.clone()) * sixty(self.step);
        if self.negative { -m } else { m }
    }

    /// The upper mark, `(mark + 1)·60^step` (the lower where exact), signed.
    pub fn hi(&self) -> Ratio {
        if self.exact {
            return self.lo();
        }
        let m = Ratio::from_integer(self.mark.clone() + BigInt::one()) * sixty(self.step);
        if self.negative { -m } else { m }
    }
}

/// The sector of `x`: its leading sexagesimal place `e` (`60^e ≤ |x| < 60^(e+1)`)
/// and `PLACES` places below it.
pub fn sector(x: &Ratio) -> Sector {
    if x.is_zero() {
        return Sector { negative: false, mark: BigInt::zero(), step: 0, exact: true };
    }
    let a = x.abs();
    let mut e: i64 = 0;
    while a >= sixty(e + 1) {
        e += 1;
    }
    while a < sixty(e) {
        e -= 1;
    }
    let step = e - PLACES;
    let scaled = a / sixty(step);
    let mark = scaled.floor().to_integer();
    Sector { negative: x.is_negative(), exact: scaled.is_integer(), mark, step }
}

impl std::fmt::Display for Sector {
    /// `=` and the mark where exact, else the open sector; each in sexagesimal
    /// places `d;d,d` and its power of sixty.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = |m: &BigInt| -> String {
            let mut d: Vec<String> = Vec::new();
            let mut v = m.clone();
            let base = BigInt::from(60);
            for _ in 0..PLACES {
                d.push(format!("{:02}", &v % &base));
                v /= &base;
            }
            d.reverse();
            format!("{v};{}", d.join(","))
        };
        if self.exact && self.mark.is_zero() {
            return write!(f, "=0");
        }
        let sign = if self.negative { "-" } else { "" };
        let at = self.step + PLACES;
        let scale = if at == 0 { String::new() } else { format!("·60^{at}") };
        if self.exact {
            write!(f, "={sign}{}{scale}", digits(&self.mark))
        } else {
            write!(f, "{sign}[{}…{}){scale}", digits(&self.mark), digits(&(self.mark.clone() + BigInt::one())))
        }
    }
}

/// A share read as a sector of its whole (zero where the whole is empty).
pub fn share(part: &Ratio, whole: &Ratio) -> Sector {
    if whole.is_zero() { sector(&ratio(0, 1)) } else { sector(&(part.clone() / whole.clone())) }
}

#[cfg(test)]
mod tests {
    use super::{sector, PLACES};
    use cfe::ratio::{ratio, Ratio};

    #[test]
    fn a_sector_brackets_its_value_between_two_marks_one_place_apart() {
        for x in [ratio(1, 7), ratio(355, 113), ratio(-2, 3), ratio(-2, 7), ratio(1, 10_000_000), ratio(123_456_789, 1)] {
            let s = sector(&x);
            let (lo, hi) = (s.lo(), s.hi());
            if s.exact {
                assert_eq!(lo, x);
                continue;
            }
            let (low, high) = if x < ratio(0, 1) { (hi.clone(), lo.clone()) } else { (lo.clone(), hi.clone()) };
            assert!(low < x && x < high, "{x}: {lo} {hi}");
            let width = high - low;
            assert_eq!(width, Ratio::from_integer(60.into()).pow(i32::try_from(s.step).unwrap_or(0)), "{x}");
        }
    }

    #[test]
    fn a_value_on_a_mark_is_served_exactly() {
        for x in [ratio(1, 2), ratio(3, 4), ratio(7, 3600), ratio(90, 1), ratio(0, 1)] {
            let s = sector(&x);
            assert!(s.exact, "{x}");
            assert_eq!(s.lo(), x);
            assert_eq!(s.hi(), x);
        }
        let s = sector(&ratio(1, 7));
        assert!(!s.exact);
        assert!(s.mark >= 3600.into() && s.mark < 216_000.into(), "{PLACES} places below the leading one");
    }
}
