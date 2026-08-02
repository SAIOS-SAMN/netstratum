//! THE OBSERVER PINS — capacity and resonance (`19_LENS` §4, §5).
//!
//! 1. CAPACITY BOUNDS DEPTH — min(requested, κ), and a request beyond
//!    standing is answered, never refused.
//! 2. THE BOUND IS APPLIED BEFORE THE WORK — a shallow observer asking
//!    for the near view materialises the far view and nothing else.
//! 3. THE PEEL IS THE TOLERANCE — the same pair reads captured far and
//!    mismatched near. That IS the dynamic tolerance.
//! 4. TWO OBSERVERS AT ONE DEPTH ALWAYS AGREE — LAW 6, which a dialed
//!    tolerance could not satisfy.
//! 5. NEAR IS EXACT — byte-identity, the strictest reading.
//! 6. SILENCE IS NOT MISMATCH — nothing to compare is its own answer.

use cfe::ratio::{ratio, Ratio};
use lens::observer::{observe, resonance, Capacity, Capture};
use lens::peel::Depth;
use rfa_core::{Delta, Support};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

fn triangle(d01: Ratio, d12: Ratio, d02: Ratio) -> Delta {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![d01]);
    delta.set_antisym(1, 2, vec![d12]);
    delta.set_antisym(0, 2, vec![d02]);
    delta
}

fn full_support() -> Support {
    let mut s = Support::new(3, 1);
    for (i, j) in [(0, 1), (1, 2), (0, 2)] {
        s.connect(i, j, 0);
    }
    s
}

#[test]
fn capacity_bounds_depth_and_never_refuses() {
    let shallow = Capacity::GROUND;
    let middling = Capacity::of(1);
    let deep = Capacity::full();

    // min(requested, κ) — a request beyond standing is ANSWERED at the
    // depth the observer can hold, never rejected.
    assert_eq!(shallow.admits(Depth::Near), Depth::Far);
    assert_eq!(shallow.admits(Depth::Mid), Depth::Far);
    assert_eq!(middling.admits(Depth::Near), Depth::Mid);
    assert_eq!(middling.admits(Depth::Far), Depth::Far, "κ is a ceiling, not a floor");
    assert_eq!(deep.admits(Depth::Near), Depth::Near);

    assert!(shallow.bounds(Depth::Near), "standing bound the request");
    assert!(!deep.bounds(Depth::Near), "full standing grants it");
}

#[test]
fn the_bound_is_applied_before_any_work_is_done() {
    // 19_LENS §4: LAZY, never memoised. A shallow observer asking for
    // the near view must MATERIALISE the far view — one reading — not
    // build the near view and then discard it.
    let delta = triangle(q(2, 1), q(3, 1), q(9, 1));
    let support = full_support();

    let bounded = observe(&delta, &support, Depth::Near, &Capacity::GROUND);
    assert_eq!(bounded.depth(), Depth::Far, "the bound applied");
    assert_eq!(bounded.materialised(), 1, "and only the far view was built");

    let granted = observe(&delta, &support, Depth::Near, &Capacity::full());
    assert_eq!(granted.depth(), Depth::Near);
    assert!(granted.materialised() > 1, "the deep observer built the entries");
}

#[test]
fn the_peel_is_the_tolerance() {
    // THE LOAD-BEARING PIN. The SAME pair, read at two depths, gives two
    // answers — and that is not inconsistency, it IS the dynamic
    // tolerance. Stepping back loosens; approaching tightens.
    let support = full_support();
    // Both carry exactly one independent cycle, so they stand at the
    // same elevation — but their entries differ.
    let a = triangle(q(2, 1), q(3, 1), q(9, 1));
    let b = triangle(q(2, 1), q(3, 1), q(10, 1));

    let far = resonance(&a, &b, &support, Depth::Far, &Capacity::full());
    assert!(
        matches!(far, Capture::Captured { at: Depth::Far }),
        "at distance the classes coincide: {far:?}"
    );

    let near = resonance(&a, &b, &support, Depth::Near, &Capacity::full());
    assert!(
        matches!(near, Capture::Mismatch { at: Depth::Near, .. }),
        "up close the entries differ: {near:?}"
    );
}

#[test]
fn two_observers_at_one_depth_always_agree() {
    // LAW 6: an experiment that does not repeat for every observer is
    // not evidence. A DIALED tolerance could not satisfy this; a derived
    // one satisfies it by construction, because a depth is a position
    // anyone can occupy.
    let support = full_support();
    let a = triangle(q(2, 1), q(3, 1), q(9, 1));
    let b = triangle(q(5, 1), q(7, 1), q(20, 1));

    for depth in [Depth::Far, Depth::Mid, Depth::Near] {
        let first = resonance(&a, &b, &support, depth, &Capacity::full());
        let second = resonance(&a, &b, &support, depth, &Capacity::full());
        assert_eq!(first, second, "{depth:?} must read the same for everyone");
    }
}

#[test]
fn near_is_exact_and_identity_always_resonates() {
    let support = full_support();
    let a = triangle(q(2, 1), q(3, 1), q(9, 1));

    // A structure against itself resonates at every depth.
    for depth in [Depth::Far, Depth::Mid, Depth::Near] {
        let seen = resonance(&a, &a, &support, depth, &Capacity::full());
        assert!(
            matches!(seen, Capture::Captured { .. }),
            "identity must resonate at {depth:?}: {seen:?}"
        );
    }

    // And the smallest difference breaks it at the near depth.
    let b = triangle(q(2, 1), q(3, 1), q(9, 2));
    let near = resonance(&a, &b, &support, Depth::Near, &Capacity::full());
    assert!(matches!(near, Capture::Mismatch { .. }), "near tolerance is zero");
}

#[test]
fn silence_is_not_mismatch() {
    // Two structures that circulate nowhere have nothing to compare at
    // the middle depth. That is an absence of subject, not a
    // disagreement — and reporting it as a mismatch would be inventing
    // a difference.
    let support = full_support();
    // Both compose: pure frame, no circulation.
    let a = triangle(q(1, 1), q(1, 2), q(3, 2));
    let b = triangle(q(2, 1), q(1, 1), q(3, 1));

    let mid = resonance(&a, &b, &support, Depth::Mid, &Capacity::full());
    assert!(
        matches!(mid, Capture::Silent { at: Depth::Mid }),
        "nothing circulates on either side: {mid:?}"
    );

    // And at distance they agree — both stand on the ground.
    let far = resonance(&a, &b, &support, Depth::Far, &Capacity::full());
    assert!(matches!(far, Capture::Captured { .. }), "both at the floor");
}

#[test]
fn a_mismatch_carries_its_separation_whole() {
    // A refusal names what it found — never a bare verdict.
    let support = full_support();
    let a = triangle(q(2, 1), q(3, 1), q(9, 1));
    let b = triangle(q(2, 1), q(3, 1), q(40, 1));

    match resonance(&a, &b, &support, Depth::Mid, &Capacity::full()) {
        Capture::Mismatch { separation, at } => {
            assert_eq!(at, Depth::Mid);
            assert!(separation > q(0, 1), "the separation is carried, not summarised away");
        }
        other => panic!("expected a mismatch at MID, got {other:?}"),
    }
}
