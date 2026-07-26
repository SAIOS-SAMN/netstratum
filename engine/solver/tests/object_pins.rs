//! Object-centric pins: exact segmentation, canonical ordering, each
//! operator's law on a constructed scene, and an object task closing
//! through the general vocabulary.

use solver::{background, segment, solve, Grid, ObjOp, SolveRead, Task, TaskPair};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("well-formed")
}

/// A scene: background 0, a 3-cell L of hue 2 top-left, a 1-cell dot
/// of hue 5 bottom-right.
fn scene() -> Grid {
    grid(
        4,
        4,
        &[
            2, 0, 0, 0, //
            2, 2, 0, 0, //
            0, 0, 0, 0, //
            0, 0, 0, 5,
        ],
    )
}

#[test]
fn segmentation_is_exact_and_canonical() {
    let g = scene();
    assert_eq!(background(&g), 0, "most common hue is the background");
    let objects = segment(&g, 0, false);
    assert_eq!(objects.len(), 2);
    // Canonical order: size descending — the L first.
    assert_eq!(objects[0].size(), 3);
    assert_eq!(objects[1].size(), 1);
    assert_eq!((objects[0].top, objects[0].left), (0, 0));
    assert_eq!(objects[0].height(), 2);
    assert_eq!(objects[0].width(), 2);
}

#[test]
fn the_operators_hold_their_laws() {
    let g = scene();
    // keep_largest: only the L remains.
    let kept = ObjOp::KeepLargest.apply(&g);
    assert_eq!(kept.cells.iter().filter(|&&h| h != 0).count(), 3);
    // delete_largest: only the dot remains.
    let deleted = ObjOp::DeleteLargest.apply(&g);
    assert_eq!(deleted.cells.iter().filter(|&&h| h != 0).count(), 1);
    assert_eq!(deleted.at(3, 3), 5);
    // crop_largest: the L's 2×2 bounding box.
    let cropped = ObjOp::CropLargest.apply(&g);
    assert_eq!((cropped.rows, cropped.cols), (2, 2));
    assert_eq!(cropped.cells, vec![2, 0, 2, 2]);
    // crop_content: spans both objects — the whole 4×4 here.
    let content = ObjOp::CropContent.apply(&g);
    assert_eq!((content.rows, content.cols), (4, 4));
    // paint_all_like_largest: the dot turns hue 2.
    let painted = ObjOp::PaintAllLikeLargest.apply(&g);
    assert_eq!(painted.at(3, 3), 2);
    // gravity: everything falls to the floor, columns exact.
    let fallen = ObjOp::GravityDown.apply(&g);
    assert_eq!(fallen.at(3, 0), 2, "column 0 stacks to the floor");
    assert_eq!(fallen.at(2, 0), 2);
    assert_eq!(fallen.at(3, 1), 2);
    assert_eq!(fallen.at(3, 3), 5, "the dot was already resting");
    assert_eq!(fallen.at(0, 0), 0, "the ceiling cleared");
}

/// An extraction task (output = the largest object's bounding box)
/// closes through the GENERAL vocabulary — no per-task branch exists.
#[test]
fn an_extraction_task_closes() {
    let make = |g: Grid| {
        let out = ObjOp::CropLargest.apply(&g);
        TaskPair {
            input: g,
            output: out,
        }
    };
    let test_in = grid(
        3,
        4,
        &[
            0, 7, 7, 0, //
            0, 7, 7, 0, //
            3, 0, 0, 0,
        ],
    );
    let task = Task {
        name: "constructed-extraction".into(),
        train: vec![make(scene()), make(test_in.clone())],
        test_expected: Some(ObjOp::CropLargest.apply(&test_in)),
        test_in,
    };
    assert!(matches!(
        solve(&task, 1),
        SolveRead::Solved {
            matches_expected: Some(true),
            ..
        }
    ));
}
