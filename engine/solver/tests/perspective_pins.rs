//! PERSPECTIVE ALGEBRA pins (38 §5, task #14).

use solver::perspective::{compound, Perspective};
use solver_verify::grid::Grid;

fn g(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("grid")
}

/// An exact view carries an empty residual and zero silhouettes; a
/// wrong view carries the cells it missed, clustered.
#[test]
fn a_perspective_reads_its_own_residual() {
    let want = g(2, 3, &[1, 2, 3, 4, 5, 6]);
    let exact = Perspective::read(1, want.clone(), Some(&want));
    assert!(exact.is_exact());
    assert_eq!(exact.silhouettes, Some(0));

    // Two wrong cells, adjacent — one silhouette (one missed rule).
    let near = g(2, 3, &[1, 2, 0, 0, 5, 6]);
    let p = Perspective::read(2, near, Some(&want));
    assert_eq!(p.residual.as_deref(), Some(&[(0usize, 2usize), (1, 0)][..]));
    // (0,2) and (1,0) are not 4-adjacent → two separate silhouettes.
    assert_eq!(p.silhouettes, Some(2), "two disjoint misses, two sub-rules");

    // Two ADJACENT wrong cells — one cluster.
    let clustered = g(2, 3, &[1, 2, 3, 0, 0, 6]);
    let q = Perspective::read(3, clustered, Some(&want));
    assert_eq!(q.silhouettes, Some(1), "one contiguous miss, one sub-rule");
}

/// A shape mismatch is the whole grid — maximal structure, not an error.
#[test]
fn a_wrong_shape_is_a_whole_residual() {
    let want = g(2, 2, &[1, 2, 3, 4]);
    let wrong_shape = g(1, 2, &[1, 2]);
    let p = Perspective::read(1, wrong_shape, Some(&want));
    assert_eq!(p.residual.as_ref().map(|c| c.len()), Some(4), "every cell of the larger grid");
}

/// COMPOUND: per-cell majority assembles the output; disagreements
/// localize; consensus is exact.
#[test]
fn views_compound_by_per_cell_majority() {
    // Three views of a 1x4. Cell 0: all agree on 1. Cell 1: 2 vs 2 vs
    // 3 → majority 2. Cell 2: all 3. Cell 3: 4 vs 5 vs 4 → majority 4.
    let views = vec![
        Perspective::read(1, g(1, 4, &[1, 2, 3, 4]), None),
        Perspective::read(2, g(1, 4, &[1, 2, 3, 5]), None),
        Perspective::read(3, g(1, 4, &[1, 3, 3, 4]), None),
    ];
    let c = compound(&views).expect("aligned views compound");
    assert_eq!(c.compound.cells, vec![1, 2, 3, 4], "the majority hue at each cell");
    // Cells 1 and 3 saw disagreement; 0 and 2 were unanimous.
    assert_eq!(c.divergent, vec![(0, 1), (0, 3)], "disagreement localizes the gap");
    assert_eq!(c.consensus, (2, 4), "two of four cells unanimous, exact");
}

/// SYSTEMATIC GAP: when every residual-bearing view agrees on the
/// silhouette count, the mesh knows what it is missing.
#[test]
fn agreement_on_the_gap_is_systematic() {
    let want = g(1, 3, &[1, 2, 3]);
    // Two views, each missing exactly one (different) cell — both read
    // ONE silhouette, so the gap is systematic at count 1.
    let a = Perspective::read(1, g(1, 3, &[0, 2, 3]), Some(&want));
    let b = Perspective::read(2, g(1, 3, &[1, 2, 0]), Some(&want));
    let c = compound(&[a, b]).expect("compound");
    assert_eq!(c.systematic_gap, Some(1), "both views miss one rule — the mesh knows it");

    // Disagreeing counts → not systematic.
    let d = Perspective::read(3, g(1, 3, &[0, 0, 3]), Some(&want)); // 1 cluster
    let e = Perspective::read(4, g(1, 3, &[0, 2, 0]), Some(&want)); // 2 clusters
    let f = compound(&[d, e]).expect("compound");
    assert_eq!(f.systematic_gap, None, "the views disagree on what is missing");
}

/// A view of a different shape drops from the compound rather than
/// corrupting the per-cell majority.
#[test]
fn a_misshapen_view_drops_from_the_compound() {
    let views = vec![
        Perspective::read(1, g(1, 2, &[1, 2]), None),
        Perspective::read(2, g(1, 2, &[1, 3]), None),
        Perspective::read(3, g(2, 2, &[9, 9, 9, 9]), None), // wrong shape
    ];
    let c = compound(&views).expect("the two aligned views compound");
    assert_eq!(c.compound.rows, 1);
    assert_eq!(c.consensus, (1, 2), "cell 0 unanimous among the aligned pair");
}
