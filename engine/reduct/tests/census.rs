//! N-I1 — THE DRAIN CENSUS (doc 53 D9 slice 4). Runs `reduce_pair` over the
//! named Pin A fixtures (empty basis) and dumps one row per pair to
//! `tests/census/here.tsv`. The same fixtures dumped from v0.4 land in
//! `v04.tsv`; `diff.txt` is the second witness. This is a ONE-SHOT dump, not a
//! running engine, and there is NO crate-link to ipsal — the two trees dump
//! independently and the tsvs are compared as text. Disagreement is lawful; the
//! ABSENCE of the table is not.

use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use reduct::reduction::{
    reduce_pair, Dir13Route, Exchange, Flip, ObserverFrame, PlaneElement, Relabel, ReduceVerdict,
    Remainder,
};
use std::fmt::Write as _;

fn measurement(rows: i64, cols: i64, fiber: u8, hues: &[u8]) -> FullMeasurement {
    let occupancy = (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .enumerate()
        .map(|(i, (row, col))| (Site { row, col }, GaugeDirection { index: hues[i] }))
        .collect();
    FullMeasurement::try_new(fiber, CellFrame { rows: rows as u32, cols: cols as u32 }, occupancy)
        .expect("whole admits")
}

fn route_name(route: Dir13Route) -> &'static str {
    match route {
        Dir13Route::CompositionGap => "CompositionGap",
        Dir13Route::GenerationGap => "GenerationGap",
        Dir13Route::CategoryError => "CategoryError",
    }
}

/// The census row for one named pair: `name arm route bounding_len
/// remainder_terms inverse_carried`. `inverse_carried` is the reconstruction
/// witness on a CongruentThroughP (`p⁻¹(b) == a`).
fn row(name: &str, a: &FullMeasurement, b: &FullMeasurement) -> String {
    let (arm, route, bounding_len, remainder_terms, inverse_carried) =
        match reduce_pair(a, b, &[]) {
            ReduceVerdict::Congruent { bounding } => {
                ("Congruent", "-".into(), bounding.len().to_string(), "-".into(), "-".into())
            }
            ReduceVerdict::CongruentThroughP { p } => {
                let carried = p.inverse().apply(b).map(|r| r == *a).unwrap_or(false);
                ("CongruentThroughP", "-".into(), "-".into(), "-".into(), carried.to_string())
            }
            ReduceVerdict::Remainder { remainder, route } => {
                let terms = match &remainder {
                    Remainder::ChainClass { chain } => chain.entries.len().to_string(),
                    Remainder::FrameShift { .. } => "-".into(),
                };
                ("Remainder", route_name(route).to_string(), "-".into(), terms, "-".into())
            }
        };
    format!("{name}\t{arm}\t{route}\t{bounding_len}\t{remainder_terms}\t{inverse_carried}")
}

#[test]
fn dump_here() {
    let a = measurement(2, 2, 4, &[0, 1, 2, 3]);
    let rot90 = PlaneElement { exchange: Exchange::Swap, row: Flip::Keep, col: Flip::Mirror };
    let rotated = ObserverFrame { plane: rot90, relabel: Relabel::new(Vec::new()) }
        .apply(&a)
        .expect("a plane move stays whole");

    let pairs: Vec<(&str, FullMeasurement, FullMeasurement)> = vec![
        ("identical", a.clone(), a.clone()),
        ("rotation", a.clone(), rotated),
        ("hue_permutation", measurement(2, 2, 7, &[5, 5, 5, 5]), measurement(2, 2, 7, &[6, 6, 6, 6])),
        ("novel_hue", measurement(2, 2, 3, &[0, 1, 0, 1]), measurement(2, 2, 3, &[0, 1, 0, 2])),
        (
            "frame_change",
            measurement(2, 2, 1, &[0, 0, 0, 0]),
            measurement(3, 3, 1, &[0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ),
    ];

    let mut out = String::from("name\tarm\troute\tbounding_len\tremainder_terms\tinverse_carried\n");
    for (name, a, b) in &pairs {
        let _ = writeln!(out, "{}", row(name, a, b));
    }

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/census/here.tsv");
    std::fs::write(path, &out).expect("the census dir exists (committed)");

    // A live pin too: the reconstruction column is true on every gauge row.
    assert!(out.contains("hue_permutation\tCongruentThroughP\t-\t-\t-\ttrue"), "the 0d3d703e row is a reconstructing gauge:\n{out}");
    assert!(out.contains("novel_hue\tRemainder\tGenerationGap"), "the wall routes generation:\n{out}");
}
