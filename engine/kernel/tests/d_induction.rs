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
        let mut orb = cfe::dop::ColonyOrb::default();
        orb.orb_id = id as u64 + 1;
        orb.banked = vec![(cfe::dop::Resource::Iron, ratio(10, 1))];
        state.orbs.push(orb);
        let _ = id;
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
