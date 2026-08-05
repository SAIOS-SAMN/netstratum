//! THE PEEL PINS — magnification held by witnesses (`19_LENS` §3).
//!
//! What these fix:
//!
//! 1. THE DEPTHS AGREE — the load-bearing one. Three magnifications of
//!    one relation must tell one story, or zoom is lying.
//! 2. FAR STANDS ON THE GROUND — a relation that circulates nowhere
//!    still reads an elevation, never zero.
//! 3. MID LOCATES — the elevation says how high, the middle says WHERE.
//! 4. NEAR IS EXACT — the entries come back as held, unsummed.
//! 5. ABSENT IS ABSENT — an unobserved pair is never read as a zero.
//! 6. ONE DEPTH MATERIALISES ONE DEPTH — the far view does not build the
//!    near view, which is what `19_LENS` §4 forbids.

use cfe::ratio::{ratio, Ratio};
use lens::peel::{peel, Depth, View};
use rfa_core::capacity::Degree;
use rfa_core::{Delta, Support};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// A 3-entity relation on one channel, every pair observed.
fn triangle(d01: Ratio, d12: Ratio, d02: Ratio) -> (Delta, Support) {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![d01]);
    delta.set_antisym(1, 2, vec![d12]);
    delta.set_antisym(0, 2, vec![d02]);
    let mut support = Support::new(3, 1);
    support.connect(0, 1, 0);
    support.connect(1, 2, 0);
    support.connect(0, 2, 0);
    (delta, support)
}

#[test]
fn the_depths_agree() {
    // THE LOAD-BEARING PIN. Three magnifications of ONE relation must
    // tell one story. If the elevation says "two charges above the
    // ground" while the middle view names three, magnification is
    // lying and every reading built on it inherits the lie.
    for (a, b, c) in [
        (q(2, 1), q(3, 1), q(9, 1)),  // does not compose — carries charge
        (q(1, 1), q(1, 2), q(3, 2)),  // composes — pure frame
        (q(-5, 1), q(-5, 1), q(3, 1)),
        (q(1, 7), q(1, 11), q(9, 13)),
    ] {
        let (delta, support) = triangle(a, b, c);

        let View::Far(degree) = peel(&delta, &support, Depth::Far) else {
            panic!("Far must return an elevation");
        };
        let View::Mid(charges) = peel(&delta, &support, Depth::Mid) else {
            panic!("Mid must return the charges");
        };

        assert_eq!(
            degree.above(),
            charges.len(),
            "the elevation above ground IS the count the middle view names"
        );
        assert_eq!(degree, Degree::above_ground(charges.len()));
    }
}

#[test]
fn far_stands_on_the_ground_and_never_reads_zero() {
    // A composing triangle circulates nowhere — and has not vanished.
    let (delta, support) = triangle(q(1, 1), q(1, 2), q(3, 2));
    let View::Far(degree) = peel(&delta, &support, Depth::Far) else {
        panic!("Far");
    };
    assert_eq!(degree, Degree::GROUND);
    assert_eq!(degree.get(), 1, "the floor reads ONE, never zero");
    assert!(degree.is_ground());

    // And the empty relation, which is as far down as anything goes.
    let empty = Delta::zero(4, 2);
    let none = Support::new(4, 2);
    let View::Far(degree) = peel(&empty, &none, Depth::Far) else {
        panic!("Far");
    };
    assert_eq!(degree, Degree::GROUND, "an empty relation still stands");
    assert!(degree.get() >= 1);
}

#[test]
fn mid_locates_what_far_only_counts() {
    // The elevation says HOW HIGH. Only the middle view says WHERE —
    // which pair, on which channel, carrying how much.
    let (delta, support) = triangle(q(2, 1), q(3, 1), q(9, 1));

    let View::Far(degree) = peel(&delta, &support, Depth::Far) else {
        panic!("Far");
    };
    let View::Mid(charges) = peel(&delta, &support, Depth::Mid) else {
        panic!("Mid");
    };

    assert_eq!(degree.above(), 1, "one independent cycle stands");
    assert_eq!(charges.len(), 1);
    // The charge is LOCATED — indices intact, value whole.
    let charge = &charges[0];
    assert!(charge.i < 3 && charge.j < 3, "the charge names its pair");
    assert_eq!(charge.l, 0, "and its channel");
    assert_ne!(charge.value, q(0, 1), "a charge that carries nothing is not a charge");
}

#[test]
fn near_returns_the_entries_exactly_and_unsummed() {
    // The microscope reads past the reduction to what is actually held.
    let (delta, support) = triangle(q(2, 1), q(3, 1), q(9, 1));
    let View::Near(entries) = peel(&delta, &support, Depth::Near) else {
        panic!("Near");
    };

    assert_eq!(entries.len(), 3, "three observed pairs, three entries");
    // Exact values, not aggregates — a lens that pre-summed would hand
    // back a number the caller could not take apart (24_FORM §2b).
    let mut seen: Vec<Ratio> = entries.iter().map(|e| e.value.clone()).collect();
    seen.sort();
    assert_eq!(seen, vec![q(2, 1), q(3, 1), q(9, 1)]);
    // Over the independent half only: Δ_ji = −Δ_ij is the same fact.
    assert!(entries.iter().all(|e| e.i < e.j), "i < j — no fact reported twice");
}

#[test]
fn an_unobserved_pair_is_never_read_as_a_zero() {
    // A path 0—1—2 with no 0—2 observation. The near view must report
    // TWO entries, not three-with-a-zero: absent is absent, and reading
    // it as zero manufactures topology nobody receipted.
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![q(2, 1)]);
    delta.set_antisym(1, 2, vec![q(3, 1)]);
    let mut support = Support::new(3, 1);
    support.connect(0, 1, 0);
    support.connect(1, 2, 0);
    // (0,2) deliberately NOT connected.

    let View::Near(entries) = peel(&delta, &support, Depth::Near) else {
        panic!("Near");
    };
    assert_eq!(entries.len(), 2, "two observed pairs — the third is not a zero");
    assert!(
        !entries.iter().any(|e| e.i == 0 && e.j == 2),
        "the unobserved pair must not appear at any value"
    );

    // And a tree carries no cycle, so the far view stands on the ground.
    let View::Far(degree) = peel(&delta, &support, Depth::Far) else {
        panic!("Far");
    };
    assert_eq!(degree, Degree::GROUND, "a spanning tree is pure frame");
}

#[test]
fn one_depth_materialises_one_depth() {
    // 19_LENS §4: the far view must not BUILD the near view. The
    // observable proxy is what each view carries — Far is one number
    // however large the relation, Near grows with it.
    let mut delta = Delta::zero(8, 2);
    let mut support = Support::new(8, 2);
    for i in 0..8usize {
        for j in (i + 1)..8 {
            for l in 0..2usize {
                delta.set_antisym(i, j, vec![q((i + j + l) as i64 + 1, 1), q(1, 1)]);
                support.connect(i, j, l);
            }
        }
    }

    let far = peel(&delta, &support, Depth::Far);
    let near = peel(&delta, &support, Depth::Near);

    assert_eq!(far.materialised(), 1, "the far view carries ONE reading");
    assert!(
        near.materialised() > far.materialised(),
        "the near view carries its own size: {} vs {}",
        near.materialised(),
        far.materialised()
    );
    assert_eq!(far.depth(), Depth::Far);
    assert_eq!(near.depth(), Depth::Near);
}

#[test]
fn the_peel_is_pure_and_repeatable() {
    // A lens stores nothing, so the same question twice must give the
    // same answer — and asking one depth must not disturb another.
    let (delta, support) = triangle(q(2, 1), q(3, 1), q(9, 1));
    for depth in [Depth::Far, Depth::Mid, Depth::Near] {
        let once = peel(&delta, &support, depth);
        let _interleave = peel(&delta, &support, Depth::Near);
        let twice = peel(&delta, &support, depth);
        assert_eq!(once, twice, "{depth:?} is not repeatable");
    }
}
