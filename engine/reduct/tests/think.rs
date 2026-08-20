//! N-I3 witness (slice 1) — think GLUES a rule across observations, or refuses.
//! A rule read from one pair is a coincidence; a rule every pair shares is
//! understood. A wall withholds and learns; the next think of the same class
//! drains. A gauge glues without growing the basis. One observation is
//! insufficient; disagreeing pairs do not glue.

use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use reduct::reduction::{Exchange, Flip, ObserverFrame, PlaneElement, Relabel};
use reduct::{think, Crystal, PairClosure, TaskThought};

fn measurement(rows: i64, cols: i64, fiber: u8, hues: &[u8]) -> FullMeasurement {
    let occupancy = (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .enumerate()
        .map(|(i, (row, col))| (Site { row, col }, GaugeDirection { index: hues[i] }))
        .collect();
    FullMeasurement::try_new(fiber, CellFrame { rows: rows as u32, cols: cols as u32 }, occupancy)
        .expect("whole admits")
}

fn plane(p: PlaneElement, m: &FullMeasurement) -> FullMeasurement {
    ObserverFrame { plane: p, relabel: Relabel::new(Vec::new()) }.apply(m).expect("whole")
}

const ROT90: PlaneElement = PlaneElement { exchange: Exchange::Swap, row: Flip::Keep, col: Flip::Mirror };
const FLIP_H: PlaneElement = PlaneElement { exchange: Exchange::Keep, row: Flip::Keep, col: Flip::Mirror };

#[test]
fn two_identical_pairs_glue_the_identity() {
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let mut grown: Vec<Crystal> = Vec::new();
    let obs = vec![(a.clone(), a.clone()), (a.clone(), a.clone())];
    assert!(
        matches!(think(&obs, &mut grown), TaskThought::Glued(PairClosure::Frame(_))),
        "two identical pairs glue an identity frame",
    );
    assert!(grown.is_empty(), "an identity glues nothing new");
}

#[test]
fn two_rotation_pairs_glue_the_gauge_and_it_reconstructs() {
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let b = plane(ROT90, &a);
    let mut grown: Vec<Crystal> = Vec::new();
    let obs = vec![(a.clone(), b.clone()), (a.clone(), b.clone())];
    match think(&obs, &mut grown) {
        TaskThought::Glued(PairClosure::Frame(p)) => {
            assert_eq!(p.apply(&a).expect("whole"), b, "the glued rule carries A → B");
            assert_eq!(p.inverse().apply(&b).expect("whole"), a, "and its inverse B → A");
        }
        other => panic!("two rotations glue a frame, got {other:?}"),
    }
    assert!(grown.is_empty(), "a gauge glues nothing into the basis");
}

#[test]
fn two_hue_permutation_pairs_glue_without_growing() {
    // The 0d3d703e shape at task scope: all-5s → all-6s glues a relabel, mints
    // NOTHING.
    let a = measurement(2, 2, 7, &[5, 5, 5, 5]);
    let b = measurement(2, 2, 7, &[6, 6, 6, 6]);
    let mut grown: Vec<Crystal> = Vec::new();
    let obs = vec![(a.clone(), b.clone()), (a, b)];
    assert!(matches!(think(&obs, &mut grown), TaskThought::Glued(PairClosure::Frame(_))));
    assert!(grown.is_empty(), "a hue gauge grows nothing (the false-wall refusal)");
}

#[test]
fn a_novel_hue_task_withholds_then_glues_by_drain() {
    // Pin B at task scope: two novel-hue pairs of the SAME class. First think
    // walls and learns (Withheld, basis grows); second think drains (ByDrain).
    let a = measurement(2, 2, 3, &[0, 1, 0, 1]);
    let b = measurement(2, 2, 3, &[0, 1, 0, 2]);
    let mut grown: Vec<Crystal> = Vec::new();
    let obs = vec![(a.clone(), b.clone()), (a, b)];

    assert_eq!(think(&obs, &mut grown), TaskThought::Withheld, "the wall is learned, not closed");
    assert_eq!(grown.len(), 1, "one crystal for the shared class");

    assert_eq!(
        think(&obs, &mut grown),
        TaskThought::Glued(PairClosure::ByDrain),
        "the same class drains against the grown basis",
    );
}

#[test]
fn one_observation_is_insufficient() {
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let mut grown: Vec<Crystal> = Vec::new();
    assert_eq!(
        think(&[(a.clone(), a)], &mut grown),
        TaskThought::InsufficientObservations { count: 1 },
        "a rule cannot be told from a coincidence on one pair",
    );
}

#[test]
fn disagreeing_pairs_do_not_glue() {
    // Pair one rotates, pair two flips — the rule read from one does not
    // generalize, so the task is unglueable.
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let mut grown: Vec<Crystal> = Vec::new();
    let obs = vec![(a.clone(), plane(ROT90, &a)), (a.clone(), plane(FLIP_H, &a))];
    assert!(
        matches!(think(&obs, &mut grown), TaskThought::UnglueableAcrossObservations { .. }),
        "disagreeing frames do not glue one rule",
    );
}
