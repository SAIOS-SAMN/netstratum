//! THE CAPACITY PINS — the D meter held by witnesses, not by prose.
//!
//! Closes `18_TORSION.md` §6.2's first half. What these pin:
//!
//! 1. THE PAIRWISE LAW — a Δ built from node-local scalar differences
//!    is a pure coboundary and reads D = 0 EVERYWHERE. The meter is
//!    only as real as its feed.
//! 2. THE HOUSE FIXTURE IS VACUOUS — `anchor_3`, the canonical
//!    equilibrium anchor, is itself a coboundary. Recorded, not hidden.
//! 3. A NON-TRANSITIVE TRIANGLE CARRIES CAPACITY — the positive case.
//! 4. FRAME-FREE — a gauge shift leaves D identical (D1: frame
//!    artifacts contribute NOTHING).
//! 5. IDEMPOTENT — capacity(Δ) == capacity(Π_G(Δ)); why D is monotone
//!    by algebra, not by decree.
//! 6. CONTAINER ≠ CAPACITY — arity is not D (18 §5).

use rfa_core::capacity::{capacity, ext_x, Extent};
use rfa_core::{coboundary_reduce, Delta};

use cfe::ratio::{ratio, Ratio};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// A 3-entity Delta from three pair values on one fiber layer.
fn triangle(d01: Ratio, d12: Ratio, d02: Ratio) -> Delta {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![d01]);
    delta.set_antisym(1, 2, vec![d12]);
    delta.set_antisym(0, 2, vec![d02]);
    delta
}

/// The house's canonical equilibrium anchor (`fold_pins::anchor_3`):
/// Δ01 = 1, Δ12 = 1/2, Δ02 = 3/2.
fn anchor_3() -> Delta {
    triangle(q(1, 1), q(1, 2), q(3, 2))
}

/// Build a Delta PURELY from node-local potentials: `Δ_ij = p_j − p_i`.
/// This is the shape the meter must read as vacuous.
fn from_potentials(potentials: &[Ratio]) -> Delta {
    let dim = potentials.len();
    let mut delta = Delta::zero(dim, 1);
    for i in 0..dim {
        for j in (i + 1)..dim {
            delta.set_antisym(i, j, vec![&potentials[j] - &potentials[i]]);
        }
    }
    delta
}

#[test]
fn a_pure_coboundary_has_no_capacity() {
    // THE PAIRWISE LAW. Δ_ij = p_j − p_i for ANY potential p reduces to
    // exactly zero: Δ'_ij = (p_j−p_i) − (p_j−p_0) + (p_i−p_0) = 0.
    // A meter fed node-local scalar differences reads D = 0 forever —
    // working meter, vacuous feed.
    for potentials in [
        vec![q(0, 1), q(1, 1), q(3, 2)],
        vec![q(5, 3), q(-7, 2), q(11, 4), q(0, 1)],
        vec![q(2, 1), q(2, 1), q(2, 1), q(2, 1), q(9, 5)],
    ] {
        let delta = from_potentials(&potentials);
        let read = capacity(&delta);
        assert!(
            read.is_vacuous(),
            "a pure coboundary must carry NO capacity, read {read:?} \
             for potentials {potentials:?}"
        );
        assert_eq!(read.entities, 0, "no entity stands in a pure frame");
        assert_eq!(read.layers, 0, "no layer stands in a pure frame");
    }
}

#[test]
fn the_equilibrium_anchor_is_a_coboundary() {
    // RECORDED, not hidden: the canonical fixture every other pin is
    // built on satisfies Δ02 = Δ01 + Δ12 exactly, so it IS a coboundary
    // (potentials 0, 1, 3/2) and carries zero capacity. Its RAW extent
    // is non-zero — three standing pairs — which is precisely the
    // container/capacity distinction of 18 §5.
    let anchor = anchor_3();
    assert_eq!(
        ext_x(&anchor),
        Extent { entities: 3, layers: 1, coordinates: 3 },
        "the raw extent counts what the CONTAINER carries"
    );
    assert!(
        capacity(&anchor).is_vacuous(),
        "the equilibrium anchor is pure frame — D = 0"
    );
}

#[test]
fn a_non_transitive_triangle_carries_capacity() {
    // The positive case: Δ02 ≠ Δ01 + Δ12 is a genuine H¹ cocycle
    // defect — an obligation triangle that does not close. On three
    // entities exactly one independent coordinate survives Π_G
    // (rank C(n−1,2) = 1), and it is the (1,2) entry, since anchoring
    // at vertex 0 sends every relation touching 0 to zero.
    let broken = triangle(q(1, 1), q(1, 1), q(0, 1));
    let read = capacity(&broken);
    assert!(!read.is_vacuous(), "a non-transitive triangle MUST carry capacity");
    assert_eq!(
        read,
        Extent { entities: 2, layers: 1, coordinates: 1 },
        "three entities admit exactly one surviving independent coordinate"
    );
}

#[test]
fn capacity_is_frame_free() {
    // D1: frame artifacts contribute NOTHING. Adding any coboundary
    // (Δ_ij += p_j − p_i) must leave the capacity reading identical —
    // this is what makes D a property of the universe and not of the
    // observer's coordinates.
    let broken = triangle(q(1, 1), q(1, 1), q(0, 1));
    let base = capacity(&broken);

    let potentials = [q(7, 3), q(-2, 5), q(4, 1)];
    let mut shifted = Delta::zero(3, 1);
    for i in 0..3 {
        for j in (i + 1)..3 {
            let gauge = &potentials[j] - &potentials[i];
            shifted.set_antisym(i, j, vec![broken.entry(i, j, 0) + gauge]);
        }
    }

    assert_eq!(
        capacity(&shifted),
        base,
        "a gauge shift must not move the capacity reading"
    );
}

#[test]
fn capacity_is_idempotent() {
    // Π_G is idempotent, so the reading is too. This is WHY D is
    // monotone by algebra rather than by decree: what was frame was
    // never in D, so reducing again cannot lower it (EQUATIONS D4).
    for delta in [
        anchor_3(),
        triangle(q(1, 1), q(1, 1), q(0, 1)),
        triangle(q(-3, 7), q(5, 2), q(1, 9)),
    ] {
        let once = capacity(&delta);
        let twice = capacity(&coboundary_reduce(&delta));
        assert_eq!(once, twice, "capacity(Δ) must equal capacity(Π_G(Δ))");
    }
}

#[test]
fn the_container_is_not_the_capacity() {
    // 18 §5: the arities Delta{dim, fiber} are the CONTAINER the
    // relation rides on, never D. A wide, empty universe holds enormous
    // arity and zero capacity — the Article II vacuous case, and the
    // shape Article III prices at zero when deeds accompany it.
    let empty = Delta::zero(8, 4);
    assert_eq!(empty.dim(), 8);
    assert_eq!(empty.fiber(), 4);
    assert!(
        capacity(&empty).is_vacuous(),
        "arity is not capacity — a wide empty container carries D = 0"
    );
}
