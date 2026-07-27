//! The chronicle-beacon pins (substrate contract §6): determinism,
//! domain separation, exactness, and golden reference vectors.
//!
//! The golden values are extracted from the running implementation on
//! the fixed digest `[7u8; 32]` and pinned forever: BLAKE3 is a stable
//! function, so any machine, any build, any year must reproduce these
//! bytes — that is the whole point of the beacon.

use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use physics::Beacon;
use std::str::FromStr;

fn fixed() -> Beacon {
    Beacon::from_digest([7u8; 32])
}

/// GOLDEN VECTOR — one draw, byte-pinned: the same digest and domain
/// yield this exact rational on every machine, forever.
#[test]
fn the_golden_draw_reproduces_exactly() {
    let expected = Ratio::new(
        BigInt::from_str("57663352482953230225323643610273460337").expect("numer"),
        BigInt::from_str("170141183460469231731687303715884105728").expect("2^127"),
    );
    assert_eq!(fixed().draw("hazard.meteor"), expected);
}

/// GOLDEN VECTORS — uniform indices, pinned (rejection sampling walks
/// deterministically, so these are replay facts, not luck).
#[test]
fn the_golden_indices_reproduce_exactly() {
    assert_eq!(fixed().uniform_index("market.demand", 64), 6);
    assert_eq!(fixed().uniform_index("tiles", 7), 4);
}

/// Determinism: two beacons on the same digest are the same beacon —
/// every draw byte-identical. This is what makes the randomness
/// replayable and mesh-verifiable.
#[test]
fn same_digest_same_draws_always() {
    let a = Beacon::from_digest([42u8; 32]);
    let b = Beacon::from_digest([42u8; 32]);
    for domain in ["hazard.meteor", "market.demand", "spoilage"] {
        assert_eq!(a.draw(domain), b.draw(domain));
        assert_eq!(a.uniform_index(domain, 8), b.uniform_index(domain, 8));
    }
}

/// Domain separation: different domain tags draw independently — no
/// system can shadow another's randomness; a different digest moves
/// every draw.
#[test]
fn domains_and_digests_separate() {
    let b = fixed();
    assert_ne!(b.draw("hazard.meteor"), b.draw("hazard.quake"));
    let other = Beacon::from_digest([8u8; 32]);
    assert_ne!(b.draw("hazard.meteor"), other.draw("hazard.meteor"));
}

/// Exactness: every draw sits in the exact rationals ∩ [0, 1); a Bernoulli trial with
/// p = 0 never fires and with p = 1 always fires — the boundary is an
/// exact comparison, not a float tolerance.
#[test]
fn draws_are_exact_and_bounded() {
    let b = fixed();
    for domain in ["a", "b", "c", "d"] {
        let d = b.draw(domain);
        assert!(d >= ratio(0, 1) && d < ratio(1, 1));
    }
    assert!(!b.bernoulli("anything", &ratio(0, 1)), "p = 0 never fires");
    assert!(b.bernoulli("anything", &ratio(1, 1)), "p = 1 always fires");
}

/// Every uniform index lands inside its range across many domains
/// (rejection sampling never returns out-of-range, whatever the tail).
#[test]
fn uniform_indices_stay_in_range() {
    let b = Beacon::from_digest([99u8; 32]);
    for i in 0..50u64 {
        let domain = format!("system.{i}");
        for n in [1u64, 2, 3, 7, 64, 1000] {
            assert!(b.uniform_index(&domain, n) < n);
        }
    }
}
