//! **IS `D` A REDUCTION OR AN INDUCTION?**
//!
//! ARTICLE IV: *capacity never retreats — monotone non-decreasing in
//! `D`*. That is an INDUCTION property: a base case plus steps that
//! only add. A REDUCTION has no such guarantee — it re-reads the whole
//! complex each time, so it rises and falls with the complex.
//!
//! `rank` is a reduction (`holonomies` strips a spanning forest and
//! counts what survives). If it can fall, the implementation
//! contradicts the article, and nothing in the tree tests it over time.

use cfe::ratio::ratio;
use cfe::world::Committed;
use kernel::bridge::{read_entangled, CHANNEL_BOND};

/// Build a committed state with `orbs` orbs and the given bond edges.
fn board(orbs: usize, edges: &[(usize, usize)]) -> Committed {
    let mut state = Committed::default();
    for id in 0..orbs {
        state.orbs.push(cfe::dop::ColonyOrb {
            orb_id: id as u64 + 1,
            banked: vec![(cfe::dop::Resource::Iron, ratio(10, 1))],
            ..Default::default()
        });
    }
    for (from, to) in edges {
        state.bonds.push(cfe::world::Bond {
            source: *from as u64 + 1,
            target: *to as u64 + 1,
            resource: cfe::dop::Resource::Iron,
            rate: ratio(1, 1),
            term: 0,
            sealed: 0,
            anchored: 0,
        });
    }
    state
}

#[test]
fn rank_rises_when_a_cycle_closes_and_falls_when_it_opens() {
    // A triangle: three orbs, three bonds. One independent cycle.
    let closed = board(3, &[(0, 1), (1, 2), (2, 0)]);
    let open = board(3, &[(0, 1), (1, 2)]);
    let empty = board(3, &[]);

    let r_closed = read_entangled(&closed).rank;
    let r_open = read_entangled(&open).rank;
    let r_empty = read_entangled(&empty).rank;

    println!("rank: empty={r_empty} open-path={r_open} closed-triangle={r_closed}");
    let _ = CHANNEL_BOND;

    // THIS ASSERTS THE DEFECT and must be inverted when the induction
    // lands. Discharging ONE bond takes a closed triangle to an open
    // path and rank falls 1 -> 0. ARTICLE IV: *capacity never retreats,
    // monotone non-decreasing in D*. A reduction cannot honour that —
    // it re-reads the whole complex, so it tracks the complex up AND
    // down.
    assert_eq!(r_closed, 1, "a closed triangle carries one independent cycle");
    assert_eq!(r_open, 0, "and opening it takes the reading to zero");
    assert!(
        r_closed > r_open,
        "MEASURED VIOLATION: rank retreats when an obligation is discharged"
    );
    assert_eq!(r_empty, 0, "and an unrelated board reads zero — no base case");
}

/// THE SAME DEFECT AT THE OTHER END — a reduction has no base case.
///
/// Induction starts from a base and adds; a reduction starts from the
/// whole and strips. Strip a board with nothing on it and you get zero,
/// which is why a founded world reads `Void` and every mint refuses
/// (`21_KERNEL_PARAMETERS` §14-15). **Both symptoms have one cause.**
#[test]
fn a_reduction_has_no_base_case_which_is_the_bootstrap_deadlock() {
    for orbs in 1..=4 {
        let standing = board(orbs, &[]);
        let read = read_entangled(&standing);
        assert_eq!(
            read.rank, 0,
            "{orbs} orbs holding real reserves still read rank 0 — a reduction \
             cannot see occupancy, only cycles"
        );
    }
}

/// **THE TWO INVERSIONS — why both readings above are wrong.**
///
/// `rank: empty=0, triangle=1` looks like a measurement. It is an
/// artifact of computing the homology of a bare 1-complex when the
/// object is a WRAPPED 3-SPACE carrying a GRADED fiber.
///
/// INVERSION 1 · THE SPACE IS NOT COUNTED. The coordinate wraps —
/// `arbor::Coordinate` carries `s: [Ratio; 3]`, reach is a wrapped-L1
/// distance, and the T∞ purge exists precisely because the torus wraps
/// finitely. **A 3-torus has `b₁ = 3` before anything is drawn on it.**
/// The space does not vanish because the board is empty, so `empty = 0`
/// is not a reading of `D` — it is a reading of a graph that forgot
/// what it was drawn on.
///
/// INVERSION 2 · THE TRIANGLE IS AN ARTIFACT. On a complex with
/// 2-cells a triangle BOUNDS: it is the boundary of a face and
/// contributes nothing to `H₁`. It registered as `1` only because
/// `Delta` has no 2-cells to fill it (`18_TORSION` §2m: *the complex
/// tops out at 2-simplices*). **The one non-zero reading measures a
/// MISSING CELL, not a present cycle.**
///
/// So `0` should be `3` and `1` should be `0`, and neither error is
/// arithmetic — both come from projecting a torus onto a graph.
#[test]
fn the_reduction_is_given_no_wrapping_at_all() {
    // The measurable half of the claim: whatever the right answer is,
    // the reduction cannot reach it, because nothing hands it the
    // wrap. `Delta` is dim × dim × fiber and `Support` is presence —
    // no period, no modulus, no torus.
    let board = board(3, &[(0, 1), (1, 2), (2, 0)]);
    let read = read_entangled(&board);

    // The projection carries obligations and nothing about the space
    // they sit in. If a wrap were ever threaded through, this reading
    // would change without any obligation changing — which is exactly
    // the test that should replace this one.
    assert_eq!(
        read.rank, 1,
        "the triangle reads 1 on a graph; on a wrapped space with faces \
         it reads differently, and the reduction is never told which it is"
    );
}

/// **DOES A COMPOSING TRIANGLE ALREADY READ ZERO?**
///
/// `holonomies` pushes a cycle only `if !charge.is_zero()` — a non-tree
/// edge that the forest's frame fully explains is already discarded.
/// If that is so, the "fill the 2-cell" half of inversion 2 is ALREADY
/// in the code for the flat case, and §16d overstated it.
#[test]
fn a_triangle_whose_obligations_compose_carries_no_cycle() {
    // Rates chosen so the triangle COMPOSES: what 0 owes 2 directly
    // equals 0→1 plus 1→2. A flat face.
    let mut composing = board(3, &[]);
    let bond = |from: u64, to: u64, rate: (i64, i64)| cfe::world::Bond {
        source: from,
        target: to,
        resource: cfe::dop::Resource::Iron,
        rate: ratio(rate.0, rate.1),
        term: 0,
        sealed: 0,
        anchored: 0,
    };
    composing.bonds.push(bond(1, 2, (1, 1)));
    composing.bonds.push(bond(2, 3, (1, 1)));
    composing.bonds.push(bond(1, 3, (2, 1))); // 1 + 1 = 2 — it composes

    let flat = read_entangled(&composing).rank;

    // And the same triangle that does NOT compose.
    let mut warped = board(3, &[]);
    warped.bonds.push(bond(1, 2, (1, 1)));
    warped.bonds.push(bond(2, 3, (1, 1)));
    warped.bonds.push(bond(1, 3, (7, 1))); // 1 + 1 ≠ 7

    let curved = read_entangled(&warped).rank;

    println!("composing triangle rank={flat}   non-composing rank={curved}");
    assert_eq!(
        flat, 0,
        "a triangle the frame explains is ALREADY discarded — the flat 2-cell \
         case needs no amendment"
    );
    assert_eq!(curved, 1, "and a triangle that does not compose carries real charge");
}
