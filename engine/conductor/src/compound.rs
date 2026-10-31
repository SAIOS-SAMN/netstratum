//! A share compounds FRAME by FRAME, read on Netstratum's carrier. The law
//! is Mycelium's form (`forms::shares::share_compounds`, concordance 2.10):
//! where a share `x` of what remains is taken on each ring of a body's own
//! clock, `n` rings leave `(1 − x)^n`, held exactly as its runs
//! `∏(1 − x_i)^n_i` (`Remaining`). This shell adds only the reading of that
//! exact object onto the marks a law lands on (a floor on the carrier's
//! grain, a comparison), landing where the exact value lands while
//! expanding the rational no further than the marks ask.
use cfe::ratio::{ratio, Ratio};
pub use forms::shares::share_compounds::Remaining;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// What `n` rings leave at share `x`, as the form holds it; None where `x`
/// is no share of what remains (the form refuses it by name).
pub fn after(x: &Ratio, n: u64) -> Option<Remaining> {
    Remaining::whole().after(x, n).ok()
}

/// `⌊x / 2^p⌋` and `⌈x / 2^p⌉` of a non-negative whole.
fn down(x: &BigInt, p: u64) -> BigInt {
    x >> p
}
fn up(x: &BigInt, p: u64) -> BigInt {
    (x + ((BigInt::one() << p) - BigInt::one())) >> p
}

/// The enclosure `[lo, hi]·2^−p` of a remaining's runs, each product rounded
/// outward.
fn enclose(runs: &[(Ratio, u64)], p: u64) -> (BigInt, BigInt) {
    let one = BigInt::one() << p;
    let (mut lo, mut hi) = (one.clone(), one);
    for (b, n) in runs {
        let scaled = b.clone() * Ratio::from_integer(BigInt::one() << p);
        let (mut bl, mut bh) = (scaled.floor().to_integer(), scaled.ceil().to_integer());
        let mut e = *n;
        while e > 0 {
            if e & 1 == 1 {
                lo = down(&(&lo * &bl), p);
                hi = up(&(&hi * &bh), p);
            }
            e >>= 1;
            if e > 0 {
                bl = down(&(&bl * &bl), p);
                bh = up(&(&bh * &bh), p);
            }
        }
    }
    (lo, hi)
}

/// The reading of `a · remaining` (`a ≥ 0`) through `at`, a reading that
/// never turns back as its value rises (a floor on a grain, a comparison):
/// read at both ends of an enclosure of the exact value, which narrows until
/// the two agree, and so land where the exact value lands. An enclosure that
/// cannot part the value from a mark it stands on gives way to the form's
/// own exact reading. None for `a < 0`.
pub fn read<T: PartialEq>(a: &Ratio, remaining: &Remaining, at: impl Fn(&Ratio) -> T) -> Option<T> {
    if a.is_negative() {
        return None;
    }
    let runs = remaining.factors();
    // The exact value's size in bits: past it an enclosure buys nothing.
    let exact_bits: u64 = runs
        .iter()
        .map(|(b, n)| n.saturating_mul(b.numer().bits().max(b.denom().bits())))
        .fold(0u64, u64::saturating_add);
    let mut p: u64 = 64;
    while p < exact_bits {
        let (lo, hi) = enclose(runs, p);
        let scale = Ratio::from_integer(BigInt::one() << p);
        let (l, h) = (
            at(&(a.clone() * Ratio::from_integer(lo) / scale.clone())),
            at(&(a.clone() * Ratio::from_integer(hi) / scale)),
        );
        if l == h {
            return Some(l);
        }
        p = p.saturating_mul(2);
    }
    Some(at(&(a.clone() * remaining.reading())))
}

/// The fewest rings at share `x` that leave less than `below` of what stood:
/// the least `n` with `(1 − x)^n < below`. None where nothing is taken
/// (`x = 0`), for no share of what remains, or past the carrier.
pub fn rings_until(x: &Ratio, below: &Ratio) -> Option<u64> {
    after(x, 0)?;
    if x.is_zero() {
        return None;
    }
    let under = |n: u64| read(&ratio(1, 1), &after(x, n)?, |v| *v < *below);
    if under(0)? {
        return Some(0);
    }
    let mut hi: u64 = 1;
    while !under(hi)? {
        hi = hi.checked_mul(2)?;
    }
    let mut lo = hi / 2;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if under(mid)? {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Some(hi)
}

#[cfg(test)]
mod tests {
    use super::{after, read, rings_until};
    use cfe::ratio::{ratio, Ratio};

    fn floor_on(grid: i64) -> impl Fn(&Ratio) -> Ratio {
        move |v: &Ratio| (v.clone() * ratio(grid, 1)).floor() / ratio(grid, 1)
    }

    #[test]
    fn a_reading_lands_where_the_exact_value_lands() {
        let grid = 216_000;
        for (a, x, n) in [
            (ratio(5, 3), ratio(1, 3), 17u64),
            (ratio(1, 1), ratio(2, 7), 200),
            (ratio(9, 4), ratio(1, 1000), 1500),
            (ratio(1, 2), ratio(0, 1), 9),
        ] {
            let r = after(&x, n).expect("a share");
            assert_eq!(
                read(&a, &r, floor_on(grid)),
                Some(floor_on(grid)(&(a.clone() * r.reading()))),
                "{a} {x} {n}"
            );
        }
        // Two shares on one clock read as one product.
        let both = after(&ratio(1, 5), 30)
            .and_then(|r| r.after(&ratio(1, 9), 30).ok())
            .expect("shares");
        assert_eq!(
            read(&ratio(1, 1), &both, floor_on(grid)),
            Some(floor_on(grid)(&both.reading()))
        );
        // A value standing on a mark: (1/2)^3 = 1/8 on eighths.
        assert_eq!(
            read(
                &ratio(1, 1),
                &after(&ratio(1, 2), 3).expect("a share"),
                floor_on(8)
            ),
            Some(ratio(1, 8))
        );
    }

    #[test]
    fn a_reading_does_not_see_how_the_rings_were_grouped() {
        let x = ratio(7, 97);
        let once = after(&x, 40).expect("a share");
        let twice = after(&x, 25)
            .and_then(|r| r.after(&x, 15).ok())
            .expect("a share");
        assert_eq!(
            read(&ratio(3, 1), &once, floor_on(216_000)),
            read(&ratio(3, 1), &twice, floor_on(216_000))
        );
    }

    #[test]
    fn what_is_no_share_is_refused() {
        assert!(after(&ratio(3, 2), 2).is_none());
        assert!(after(&ratio(-1, 4), 2).is_none());
        assert!(read(
            &ratio(-1, 1),
            &after(&ratio(1, 4), 1).expect("a share"),
            floor_on(60)
        )
        .is_none());
        assert!(rings_until(&ratio(0, 1), &ratio(1, 2)).is_none());
    }

    #[test]
    fn the_rings_until_a_share_is_left_are_the_least_that_reach_it() {
        assert_eq!(rings_until(&ratio(1, 2), &ratio(1, 8)), Some(4));
        assert_eq!(rings_until(&ratio(1, 2), &ratio(1, 7)), Some(3));
        let n = rings_until(&ratio(1, 50), &ratio(1, 1000)).expect("a count");
        assert!(after(&ratio(1, 50), n).is_some_and(|k| k.reading() < ratio(1, 1000)));
        assert!(after(&ratio(1, 50), n - 1).is_some_and(|k| k.reading() >= ratio(1, 1000)));
    }
}
