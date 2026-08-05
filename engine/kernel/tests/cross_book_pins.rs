//! THE CROSS-BOOK READ — `project_complex_across` / `read_entangled_across`
//! (operator ruling 2026-08-02: *"open the cross-book read via
//! entanglement"*).
//!
//! What these pin, and why each matters:
//!
//! 1. **A lane is an EDGE across books.** Two books sharing a chartered
//!    `deal` relate; two that share none do not.
//! 2. **A TRIANGLE OF LANES CARRIES RANK.** This is the whole point: no
//!    single-book projection can ever read it, because every leg has
//!    one end in another world. It is the reading that makes capacity
//!    reachable at all.
//! 3. **The channels stay complementary.** The lane channel is empty in
//!    the intra-book read and the obligation channels are empty in the
//!    cross-book read — nothing is counted twice.
//! 4. **`unknown` is carried, never assumed.** An observer that cannot
//!    see every pair says so, and `rank_is_defined` refuses.
//! 5. **A TREE CARRIES NOTHING** — the refutation of the entity door,
//!    held as a test rather than as prose: holonomy comes from edges
//!    OUTSIDE a spanning forest, and a forest has none.

use cfe::ratio::{ratio};
use cfe::world::Committed;
use kernel::bridge::{
    project_complex_across, read_entangled_across, poles_per_dimension_across, CHANNELS,
    CHANNEL_LANE,
};

/// A committed book holding exactly the lanes named. Everything else
/// stays at its default — the lane channel is what is under test.
fn book_with_lanes(lanes: &[(u64, cfe::dop::Unit)]) -> Committed {
    Committed { routes: lanes.to_vec(), ..Default::default() }
}

#[test]
fn a_shared_lane_is_an_edge_and_an_unshared_one_is_not() {
    let a = book_with_lanes(&[(7, cfe::dop::unit(1, 8))]);
    let b = book_with_lanes(&[(7, cfe::dop::unit(1, 8))]);
    let c = book_with_lanes(&[(9, cfe::dop::unit(1, 8))]);

    let (delta, support) = project_complex_across(&[&a, &b, &c]);
    assert_eq!(delta.fiber(), CHANNELS, "one coordinate per channel");
    assert!(support.has(0, 1, CHANNEL_LANE), "A and B share deal 7 — they relate");
    assert!(!support.has(0, 2, CHANNEL_LANE), "A and C share nothing — absent, not zero");
    assert_eq!(
        *delta.entry(0, 1, CHANNEL_LANE),
        ratio(1, 8),
        "the edge carries the lane's eta"
    );
    assert_eq!(
        *delta.entry(1, 0, CHANNEL_LANE),
        ratio(-1, 8),
        "antisymmetric by construction, like every other channel"
    );
}

#[test]
fn a_triangle_of_lanes_carries_rank_that_no_book_can_see_alone() {
    // Three worlds, pairwise laned: A~B, B~C, C~A. Three edges over
    // three vertices — a spanning forest takes two, so exactly ONE
    // edge stands outside it and closes one independent cycle.
    let a = book_with_lanes(&[(1, cfe::dop::unit(1, 8)), (3, cfe::dop::unit(1, 8))]);
    let b = book_with_lanes(&[(1, cfe::dop::unit(1, 8)), (2, cfe::dop::unit(1, 8))]);
    let c = book_with_lanes(&[(2, cfe::dop::unit(1, 8)), (3, cfe::dop::unit(1, 8))]);

    let read = read_entangled_across(&[&a, &b, &c], 0);
    assert_eq!(read.rank, 1, "the closed triangle carries exactly one independent cycle");
    assert!(
        read.holonomy.iter().all(|h| h.l == CHANNEL_LANE),
        "the charge sits in the lane channel and nowhere else"
    );

    // AND THE SAME BOOKS READ ALONE CARRY NOTHING. This is the finding
    // the whole cross-book door exists for: the structure is real and
    // is invisible to every single-book projection.
    for (name, book) in [("a", &a), ("b", &b), ("c", &c)] {
        let alone = kernel::bridge::read_entangled(book);
        assert_eq!(alone.rank, 0, "{name} alone sees no structure — its lanes end elsewhere");
    }
}

#[test]
fn the_channels_stay_complementary() {
    let a = book_with_lanes(&[(1, cfe::dop::unit(1, 8))]);
    let b = book_with_lanes(&[(1, cfe::dop::unit(1, 8))]);
    let (delta, support) = project_complex_across(&[&a, &b]);
    for channel in 0..CHANNELS {
        if channel == CHANNEL_LANE {
            continue;
        }
        assert!(
            !support.has(0, 1, channel),
            "channel {channel} is INTRA-book and must stay empty in the cross-book read"
        );
        assert_eq!(*delta.entry(0, 1, channel), ratio(0, 1));
    }
    // And the intra-book read leaves the lane channel alone.
    let (_, intra) = kernel::bridge::project_complex(&a);
    assert!(
        !intra.has(0, 0, CHANNEL_LANE),
        "the lane channel is never filled by the single-book projection"
    );
}

#[test]
fn a_lone_book_relates_to_nothing_and_neither_pole_stands() {
    let a = book_with_lanes(&[(1, cfe::dop::unit(1, 8))]);
    let read = read_entangled_across(&[&a], 0);
    assert_eq!(read.rank, 0, "one book has no boundary to relate across");
    // The VERDICT form is struck with the vacuity removal; the poles
    // themselves are still the fact worth pinning.
    let per = poles_per_dimension_across(&[&a]);
    assert_eq!(per[CHANNEL_LANE], (0, 0), "neither pole — and it says so");
}

#[test]
fn unknown_is_carried_and_refuses_the_reading() {
    let a = book_with_lanes(&[(1, cfe::dop::unit(1, 8))]);
    let b = book_with_lanes(&[(1, cfe::dop::unit(1, 8))]);
    let blind = read_entangled_across(&[&a, &b], 3);
    assert_eq!(blind.unknown, 3, "the observer's blindness rides the read");
    assert!(
        !blind.rank_is_defined(),
        "while any pair is unobserved no rank may be read off the complex — \
         absence is a CLAIM, and an unchecked claim is a phantom cycle"
    );
    let seen = read_entangled_across(&[&a, &b], 0);
    assert!(seen.rank_is_defined(), "LockOn: absence confirmed at the present reach");
}

#[test]
fn a_tree_of_lanes_carries_no_cycle() {
    // THE ENTITY DOOR, REFUTED AS A TEST. Containment is a tree, and
    // this is what a tree reads: A~B, B~C, no closing edge. Two edges
    // over three vertices is exactly a spanning forest, so NOTHING
    // stands outside it and the rank is zero however deep the tree.
    let a = book_with_lanes(&[(1, cfe::dop::unit(1, 8))]);
    let b = book_with_lanes(&[(1, cfe::dop::unit(1, 8)), (2, cfe::dop::unit(1, 8))]);
    let c = book_with_lanes(&[(2, cfe::dop::unit(1, 8))]);
    let read = read_entangled_across(&[&a, &b, &c], 0);
    assert_eq!(
        read.rank, 0,
        "a chain is a tree; holonomy comes from edges OUTSIDE the spanning forest, \
         and a tree has none — this is why projecting the entity containment \
         hierarchy could never have raised the rank"
    );
}
