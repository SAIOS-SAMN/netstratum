//! **THE N-DIMENSIONAL CARRIER**, pinned.
//!
//! The load-bearing pins are the two SUBSUMPTION proofs: a rank-2 field
//! reproduces every `Grid` symmetry, and a rank-3 field reproduces
//! `Volume`'s stack operators — from the SAME vocabulary. If those
//! hold, the two carriers were one carrier written twice.

use solver_verify::field::{quarter_turn, run, Axis, Field};
use solver_verify::{Grid, Op, VOp, Volume};

fn field(shape: &[usize], cells: &[u8]) -> Field {
    Field::new(shape.to_vec(), cells.to_vec()).expect("well-formed")
}

/// SUBSUMPTION, RANK 2. Every `Grid` symmetry is an axis word, and the
/// two agree cell for cell.
#[test]
fn a_rank_two_field_reproduces_every_grid_symmetry() {
    let cells = [1u8, 2, 3, 4, 5, 6];
    let grid = Grid::new(2, 3, cells.to_vec()).expect("well-formed");
    let plane = field(&[2, 3], &cells);

    let cases: [(Op, Vec<Axis>); 4] = [
        (Op::FlipV, vec![Axis::Reverse(0)]),
        (Op::FlipH, vec![Axis::Reverse(1)]),
        (Op::Transpose, vec![Axis::Swap(0, 1)]),
        (Op::Rotate90, quarter_turn(0, 1)),
    ];
    for (op, word) in cases {
        let by_grid = op.apply(&grid);
        let by_field = run(&word, &plane);
        assert_eq!(
            by_field.shape(),
            &[by_grid.rows, by_grid.cols],
            "{}: shape agrees",
            op.describe()
        );
        assert_eq!(
            by_field.cells(),
            by_grid.cells.as_slice(),
            "{}: cells agree — the matrix operator was an axis operator",
            op.describe()
        );
    }
}

/// SUBSUMPTION, RANK 3 — and the finding that justifies the whole
/// change. `Volume::FlipStack` and `Grid::FlipV` are THE SAME
/// OPERATOR, one axis apart; they were written twice because one lived
/// in a 2-D type and the other in a 3-D one.
#[test]
fn a_rank_three_field_reproduces_the_stack_operators() {
    let layers: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 0, 1, 2]];
    let volume = Volume::new(
        layers
            .iter()
            .map(|cells| Grid::new(2, 2, cells.clone()).expect("well-formed"))
            .collect(),
    )
    .expect("congruent");
    let flat: Vec<u8> = layers.concat();
    let solid = field(&[3, 2, 2], &flat);

    // FlipStack reverses the layer axis — axis 0 of a rank-3 field.
    let by_volume = VOp::FlipStack.apply(&volume);
    let by_field = Axis::Reverse(0).apply(&solid);
    let expected: Vec<u8> = by_volume.layers().iter().flat_map(|g| g.cells.clone()).collect();
    assert_eq!(by_field.cells(), expected.as_slice(), "FlipStack IS Reverse(0)");

    // CycleLayers rolls the layer axis by one.
    let by_volume = VOp::CycleLayers.apply(&volume);
    let by_field = Axis::Roll(0, 1).apply(&solid);
    let expected: Vec<u8> = by_volume.layers().iter().flat_map(|g| g.cells.clone()).collect();
    assert_eq!(by_field.cells(), expected.as_slice(), "CycleLayers IS Roll(0, 1)");

    // And a per-layer planar op is the same axis word one axis in.
    let by_volume = VOp::PerLayer(Op::FlipH).apply(&volume);
    let by_field = Axis::Reverse(2).apply(&solid);
    let expected: Vec<u8> = by_volume.layers().iter().flat_map(|g| g.cells.clone()).collect();
    assert_eq!(by_field.cells(), expected.as_slice(), "per-layer FlipH IS Reverse(2)");
}

/// **THE UPGRADE ITSELF: rank 4 costs nothing rank 2 did not pay.**
/// No new type, no new vocabulary, no new apply path.
#[test]
fn rank_four_needs_no_new_machinery() {
    let cells: Vec<u8> = (0..2 * 2 * 2 * 3u8).map(|n| n % 10).collect();
    let hyper = field(&[2, 2, 2, 3], &cells);
    assert_eq!(hyper.rank(), 4);
    assert_eq!(hyper.len(), 24);

    // Every axis reverses, and reversing twice is the identity — at
    // every rank, from the same operator.
    for axis in 0..4 {
        let once = Axis::Reverse(axis).apply(&hyper);
        assert_ne!(once, hyper, "axis {axis} genuinely moves");
        assert_eq!(Axis::Reverse(axis).apply(&once), hyper, "and is self-inverse");
    }
    // A swap of the outer axes carries the shape with it.
    let swapped = Axis::Swap(0, 3).apply(&hyper);
    assert_eq!(swapped.shape(), &[3, 2, 2, 2]);
    assert_eq!(Axis::Swap(0, 3).apply(&swapped), hyper, "swap is self-inverse");
}

/// The algebra holds at any rank: a roll of the full extent is the
/// identity, and rolls compose additively.
#[test]
fn the_axis_algebra_holds_at_any_rank() {
    let solid = field(&[3, 2], &[1, 2, 3, 4, 5, 6]);
    assert_eq!(Axis::Roll(0, 3).apply(&solid), solid, "a full roll is the identity");
    let twice = run(&vec![Axis::Roll(0, 1), Axis::Roll(0, 1)], &solid);
    assert_eq!(twice, Axis::Roll(0, 2).apply(&solid), "rolls compose additively");
    assert_eq!(
        run(&vec![Axis::Swap(0, 1), Axis::Swap(0, 1)], &solid),
        solid,
        "swap twice is the identity"
    );
}

/// TOTAL, NEVER PANICKING. An operator naming an axis the field does
/// not have leaves it unchanged — the verifier rejects a useless word
/// by failing to close, never by crashing inside the gate.
#[test]
fn an_axis_the_field_does_not_have_changes_nothing() {
    let plane = field(&[2, 2], &[1, 2, 3, 4]);
    for stray in [Axis::Reverse(7), Axis::Swap(0, 9), Axis::Roll(4, 1)] {
        assert_eq!(stray.apply(&plane), plane, "{} is inapplicable, not fatal", stray.describe());
    }
}

/// THE CARRIER'S OWN LAW. Rank 0, a zero extent, a mismatched cell
/// count, and a hue past the ladder are each not a field.
#[test]
fn the_carrier_refuses_what_is_not_a_field() {
    assert!(Field::new(vec![], vec![]).is_none(), "rank 0 is the degenerate arm");
    assert!(Field::new(vec![2, 0], vec![]).is_none(), "an axis with no extent");
    assert!(Field::new(vec![2, 2], vec![1, 2, 3]).is_none(), "cell count must be the product");
    assert!(Field::new(vec![1, 2], vec![1, 10]).is_none(), "a hue past the ladder is not a field");
    assert!(Field::new(vec![1, 2], vec![0, 9]).is_some(), "the full lawful range admits");
}

/// Coordinates address exactly the field and nothing else — the
/// totality the apply path depends on.
#[test]
fn coordinates_address_the_field_and_nothing_beyond_it() {
    let solid = field(&[2, 3], &[0, 1, 2, 3, 4, 5]);
    assert_eq!(solid.at(&[0, 0]), Some(0));
    assert_eq!(solid.at(&[1, 2]), Some(5), "row-major");
    assert_eq!(solid.at(&[2, 0]), None, "past the first axis");
    assert_eq!(solid.at(&[0, 3]), None, "past the second");
    assert_eq!(solid.at(&[0]), None, "the wrong rank is not a coordinate");
    assert_eq!(solid.at(&[0, 0, 0]), None, "nor is a longer one");
}
