//! **THE ARC SEAT** (N-I3, doc `53_INTELLIGENCE_PORT` D1/D2) — the solver reads
//! its train grids through the ONE reduce organ. It owns NO search: it peels the
//! observed pairs into measurements, hands them to `reduct::think`, and — only when
//! the task GLUES a nameable frame — renders that frame to a program and seals
//! the proof. Every other verdict WITHHOLDS. `search::solve` / `propose_*` /
//! `sweep_vocabulary` are the costume this seat replaces; none is called here.
//!
//! Slice 2 (this landing): the `Grid → FullMeasurement` transducer only.
//! Slice 3 adds `think_task`, the render table, and the certificate.

use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, MeasurementRefusal, Site};
use reduct::reduction::{ObserverFrame, PlaneElement, Relabel};
use reduct::{think, Crystal, PairClosure, TaskThought};
use solver_verify::certificate::{verify_certificate, CertificateRead, SolveCertificate};
use solver_verify::grid::{run, Grid, Op, Program, TileMirror, HUES};
use solver_verify::loader::Task;
use solver_verify::volume::{
    run_volume, verify_volume_certificate, VOp, VProgram, Volume, VolumeCertificate,
    VolumeCertificateRead, VolumeTask,
};
use crate::volume::VolumeSolveRead;

/// The typed solve reading — the ARC seat's verdict (moved here from the retired
/// `search` module, deleted 2026-08-19: the mind observes, it does not search).
/// `Solved` carries the proved program, the rendered test output, and its
/// certificate; `Unsolved` is the honest withhold (Pin D).
#[derive(Debug, Clone)]
pub enum SolveRead {
    Solved {
        program: Program,
        test_output: Grid,
        matches_expected: Option<bool>,
        certificate: SolveCertificate,
    },
    Unsolved,
}

/// Transduce a colored grid into a full measurement: every cell is a site
/// carrying its hue as a gauge direction.
///
/// **The fiber is the FIXED ten-hue ladder (`HUES`), NOT the grid's own max.**
/// A uniform fiber is load-bearing: a full measurement carries its fiber rank in
/// its identity, and `canonicalize` preserves it, so `deduce_gauge_move` reads
/// two forms equal ONLY when their ranks match. Were the fiber the per-grid max,
/// an all-5s grid (rank 6) and an all-6s grid (rank 7) would never deduce a
/// gauge move — the 0d3d703e discriminator (a pure hue permutation is
/// CongruentThroughP) would break at the seat. The ARC gauge fiber IS the wire's
/// ten-hue ladder; every grid lives in it.
///
/// Background 0 is a direction like any other. A grid is dense by construction,
/// so it measures whole; `try_new` re-verifies wholeness regardless
/// (whole-or-refused — a grid we cannot measure is never fabricated into one).
pub fn measure_grid(grid: &Grid) -> Result<FullMeasurement, MeasurementRefusal> {
    let occupancy = (0..grid.rows)
        .flat_map(|r| (0..grid.cols).map(move |c| (r, c)))
        .map(|(r, c)| {
            (Site { row: r as i64, col: c as i64, layer: 0 }, GaugeDirection { index: grid.at(r, c) })
        })
        .collect();
    FullMeasurement::try_new(
        HUES,
        CellFrame { rows: grid.rows as u32, cols: grid.cols as u32, layers: 1 },
        occupancy,
    )
}

/// VOLUMETRIC-1b — the `measure_grid` counterpart for a stack of congruent
/// layers: every cell of every layer is a site carrying its hue, `layer`
/// riding the stack index. Same fixed ten-hue fiber as `measure_grid` — a
/// volumetric task lives in the SAME gauge fiber a planar one does (there is
/// nothing about stacking layers that widens the hue ladder), so the two
/// seats' measurements are directly comparable at the fiber. `Volume::new`
/// already guarantees every layer shares one (rows, cols); this only reads
/// that guarantee, never re-derives it.
pub fn measure_volume(volume: &Volume) -> Result<FullMeasurement, MeasurementRefusal> {
    let layers = volume.layers();
    let Some(first) = layers.first() else {
        // An empty stack has no frame to measure — the same "nothing to
        // measure" case `Volume::new` itself already refuses to construct
        // (`layers.first()?` there), so this arm is unreachable through the
        // public API; kept as an explicit, honest refusal rather than a panic.
        return FullMeasurement::try_new(HUES, CellFrame { rows: 0, cols: 0, layers: 0 }, Vec::new());
    };
    let (rows, cols) = (first.rows, first.cols);
    let occupancy = (0..layers.len())
        .flat_map(|l| (0..rows).flat_map(move |r| (0..cols).map(move |c| (l, r, c))))
        .map(|(l, r, c)| {
            (
                Site { row: r as i64, col: c as i64, layer: l as i64 },
                GaugeDirection { index: layers[l].at(r, c) },
            )
        })
        .collect();
    FullMeasurement::try_new(
        HUES,
        CellFrame { rows: rows as u32, cols: cols as u32, layers: layers.len() as u32 },
        occupancy,
    )
}

// ─────────────────────────────────────────────────────────────────────
// SLICE 3 — the render table and the seat conductor.
// ─────────────────────────────────────────────────────────────────────

/// The eight D₄ words over the grid vocabulary — the RENDER TARGET. A complete
/// set of DISTINCT plane-group representatives (verified distinct on a 2×2
/// probe): the empty word, the six single-op symmetries, and the anti-diagonal
/// as a two-op word. A structural constant (group order eight), NEVER a content
/// search over programs.
fn d4_words() -> [Program; 8] {
    [
        Vec::new(),
        vec![Op::Rotate90],
        vec![Op::Rotate180],
        vec![Op::Rotate270],
        vec![Op::FlipH],
        vec![Op::FlipV],
        vec![Op::Transpose],
        vec![Op::Rotate90, Op::FlipV],
    ]
}

/// Render a plane element to the grid word that reproduces it — by MATCHING
/// against a probe, never a hand table that could silently invert a Swap/Mirror
/// convention. A 2×2 probe with four distinct hues separates all eight D₄
/// elements; the word whose application equals the plane's action on the probe
/// IS the render. Exactly one matches (a bijection of one group onto its
/// representatives); a plane with no match is impossible for D₄ — withhold, not
/// invent. The certificate (Pin A) re-checks the render on the real train sizes.
fn render_plane(plane: PlaneElement) -> Option<Program> {
    let probe = Grid::new(2, 2, vec![0, 1, 2, 3]).expect("the 2×2 probe is lawful");
    let probe_m = measure_grid(&probe).ok()?;
    let target = ObserverFrame { plane, relabel: Relabel::new(Vec::new()) }.apply(&probe_m).ok()?;
    d4_words()
        .into_iter()
        .find(|word| measure_grid(&run(word, &probe)).map(|m| m == target).unwrap_or(false))
}

/// Render a relabel (a hue permutation carried as its moved points) to a single
/// `Recolor` op — identity on unmoved hues. Refuses if any label is off the
/// ten-hue ladder (never fabricate a hue). An empty relabel renders to NO op.
fn render_relabel(relabel: &Relabel) -> Option<Program> {
    if relabel.pairs.is_empty() {
        return Some(Vec::new());
    }
    let mut map = [0u8; HUES as usize];
    for (hue, slot) in map.iter_mut().enumerate() {
        *slot = hue as u8;
    }
    for (from, to) in &relabel.pairs {
        let (from, to) = (*from as usize, *to as usize);
        if from >= map.len() || to >= map.len() {
            return None;
        }
        map[from] = to as u8;
    }
    Some(vec![Op::Recolor(map)])
}

/// Render an observer frame (plane + relabel) to a program: the geometric word
/// then the recolor. The two commute (one moves cells, the other hues), so the
/// order re-seals the same output either way; geometry-first is the pinned
/// convention. `None` for an unnameable frame — the seat withholds.
fn render_frame(frame: &ObserverFrame) -> Option<Program> {
    let mut program = render_plane(frame.plane)?;
    program.extend(render_relabel(&frame.relabel)?);
    Some(program)
}

// ─────────────────────────────────────────────────────────────────────
// VOLUMETRIC-1c — the render table's PerLayer subset.
// ─────────────────────────────────────────────────────────────────────

/// The eight D₄ words over the PER-LAYER vocabulary — the SAME eight words
/// `d4_words()` already proved distinct, each op wrapped in `VOp::PerLayer`.
/// No new group theory: `PlaneElement` passes `layer` through unchanged
/// (VOLUMETRIC-1a), so applying one D₄ word to EVERY layer identically is
/// exactly what `PlaneElement` already expresses over a volumetric
/// measurement — the render target is a re-labeling of the planar one, not a
/// new one.
fn d4_vwords() -> [VProgram; 8] {
    d4_words().map(|word| word.into_iter().map(VOp::PerLayer).collect())
}

/// Render a plane element to the per-layer volumetric word that reproduces
/// it — the same match-by-probe technique `render_plane` uses (never a hand
/// table), proved on a `depth`-layer probe instead of a bare grid.
///
/// **THE SCOPE LIMIT, PROVED not ASSUMED:** `VOp::FlipStack`/`CycleLayers`
/// genuinely permute sites BETWEEN layers; `PlaneElement` cannot express
/// that (its transform never touches `layer`), so a gauge move this render
/// cannot find among the eight `PerLayer` words is not necessarily
/// unreal — it may be a stack move outside this render's reach. A task
/// whose gluing needs one WITHHOLDS honestly (Pin D) rather than being
/// invented. Widening past `PerLayer` needs its own layer-acting group
/// (VOLUMETRIC-2), not this landing.
fn render_vplane(plane: PlaneElement, depth: usize) -> Option<VProgram> {
    let layer = Grid::new(2, 2, vec![0, 1, 2, 3]).expect("the 2×2 probe is lawful");
    let probe = Volume::new(vec![layer; depth.max(1)])?;
    let probe_m = measure_volume(&probe).ok()?;
    let target = ObserverFrame { plane, relabel: Relabel::new(Vec::new()) }.apply(&probe_m).ok()?;
    d4_vwords().into_iter().find(|word| {
        measure_volume(&run_volume(word, &probe)).map(|m| m == target).unwrap_or(false)
    })
}

/// Render an observer frame to a volumetric program: the per-layer geometric
/// word then a per-layer recolor (same commuting order `render_frame` uses —
/// a hue relabel applies uniformly regardless of layer, so it wraps in
/// `VOp::PerLayer` exactly like a symmetry does). `None` for an unnameable
/// frame or one that needs a stack move this render cannot reach.
fn render_vframe(frame: &ObserverFrame, depth: usize) -> Option<VProgram> {
    let mut program = render_vplane(frame.plane, depth)?;
    program.extend(render_relabel(&frame.relabel)?.into_iter().map(VOp::PerLayer));
    Some(program)
}

// ─────────────────────────────────────────────────────────────────────
// VOLUMETRIC-1d — the volumetric seat conductor.
// ─────────────────────────────────────────────────────────────────────

/// The volumetric `recognize_frame`: peel every pair, hand the family to the
/// SAME reduce organ (`think`) the planar seat uses — `think`/`TaskThought`/
/// `PairClosure` are generic over `FullMeasurement`, never grid-specific —
/// and render the glued frame. `VOp` has no tile or object operator (only
/// `PerLayer` + the two stack ops, per `solver_verify::volume`'s own
/// vocabulary), so unlike `think_task` there is no frame-change/tile/object
/// path to try: every volumetric task IS the same-frame case.
fn recognize_vframe(
    pairs: &[(&Volume, &Volume)],
    grown: &mut Vec<Crystal>,
    depth: usize,
) -> Option<VProgram> {
    let mut observations = Vec::with_capacity(pairs.len());
    for (input, output) in pairs {
        match (measure_volume(input), measure_volume(output)) {
            (Ok(a), Ok(b)) => observations.push((a, b)),
            _ => return None,
        }
    }
    match think(&observations, grown) {
        TaskThought::Glued(PairClosure::Frame(frame)) => render_vframe(&frame, depth),
        // ByDrain / Unglueable / Withheld — including every task whose
        // gluing needs FlipStack/CycleLayers, which this render cannot
        // reach (VOLUMETRIC-1c) — close no single renderable frame.
        _ => None,
    }
}

/// **THINK THE VOLUMETRIC TASK** — the volumetric mirror of `think_task`,
/// same law: peel, read through the ONE reduce organ, render only when the
/// task glues a nameable frame, seal a re-derivable certificate. Every other
/// verdict WITHHOLDS (Pin D). This calls neither `solve_volume` (the
/// RETIRED search prover, kept prover-side and non-constitutional) nor any
/// enumeration — `research_step`'s volumetric civs reach this exactly the
/// way its planar civs reach `think_task`.
pub fn think_volume_task(task: &VolumeTask, grown: &mut Vec<Crystal>) -> VolumeSolveRead {
    let Some((first_in, _)) = task.train.first() else {
        return VolumeSolveRead::Unsolved; // no pairs proves nothing
    };
    let depth = first_in.depth();
    let pairs: Vec<(&Volume, &Volume)> = task.train.iter().map(|(i, o)| (i, o)).collect();
    let Some(program) = recognize_vframe(&pairs, grown, depth) else {
        return VolumeSolveRead::Unsolved;
    };
    // Pin A: the program must reproduce EVERY train output exactly.
    if task.train.iter().any(|(input, expected)| &run_volume(&program, input) != expected) {
        return VolumeSolveRead::Unsolved;
    }
    let test_output = run_volume(&program, &task.test_in);
    let certificate = VolumeCertificate::seal(task, &program, &test_output);
    if verify_volume_certificate(&certificate, task, &program, &test_output)
        != VolumeCertificateRead::Intact
    {
        return VolumeSolveRead::Unsolved;
    }
    let matches_expected = task.test_expected.as_ref().map(|expected| expected == &test_output);
    VolumeSolveRead::Solved { program, test_output, matches_expected, certificate }
}

#[cfg(test)]
mod volumetric_render_tests {
    use super::*;
    use reduct::reduction::{reduce_pair, ReduceVerdict};

    fn probe(depth: usize) -> Volume {
        let layer = Grid::new(2, 2, vec![0, 1, 2, 3]).expect("lawful");
        Volume::new(vec![layer; depth]).expect("congruent layers")
    }

    /// PROVES the design premise: a `PerLayer` symmetry applied to a real
    /// volume deduces as a gauge move through `reduce_pair` (the SAME organ
    /// `think_task` uses), and `render_vplane` renders it back to the exact
    /// program that reproduces it — round-tripping through the reduce organ,
    /// not just matching against the probe render_vplane itself uses.
    #[test]
    fn a_per_layer_rotation_deduces_and_renders_correctly() {
        let a = probe(3);
        let program = vec![VOp::PerLayer(Op::Rotate90)];
        let b = run_volume(&program, &a);
        let ma = measure_volume(&a).expect("whole");
        let mb = measure_volume(&b).expect("whole");
        match reduce_pair(&ma, &mb, &[]) {
            ReduceVerdict::CongruentThroughP { p } => {
                // `deduce_gauge_move` may legitimately split the transform between
                // `p.plane` and `p.relabel` — a probe where every hue occupies
                // exactly one site makes geometric movement and hue substitution
                // indistinguishable (the same ambiguity `render_frame` resolves by
                // rendering BOTH pieces, never `render_plane` alone), so the full
                // frame renders through `render_vframe`, mirroring `render_frame`.
                let rendered = render_vframe(&p, 3).expect("a per-layer frame renders");
                assert_eq!(
                    measure_volume(&run_volume(&rendered, &a)).expect("whole"),
                    mb,
                    "the rendered word reproduces the SAME target the reduce organ deduced"
                );
            }
            other => panic!("a per-layer rotation is a gauge move, got {other:?}"),
        }
    }

    /// The scope limit, PROVED rather than assumed: `layer` never enters
    /// `PlaneElement`'s transform, so a genuine stack move (data crossing
    /// between layers) is NOT among the eight `PerLayer` words — the render
    /// correctly returns `None` rather than inventing a wrong word.
    #[test]
    fn a_genuine_stack_move_is_outside_the_per_layer_render() {
        let a = probe(2);
        let b = run_volume(&vec![VOp::FlipStack], &a);
        let ma = measure_volume(&a).expect("whole");
        let mb = measure_volume(&b).expect("whole");
        // FlipStack is genuinely a different measurement (layer 0 and layer 1
        // swap content), so it either fails to deduce as ANY plane element,
        // or — if it coincidentally matches one on this probe — render_vplane
        // must still fail to find a PerLayer word that reproduces it, since
        // no PerLayer word can move a site between layers.
        if let ReduceVerdict::CongruentThroughP { p } = reduce_pair(&ma, &mb, &[]) {
            assert_eq!(
                render_vplane(p.plane, 2), None,
                "no PerLayer word can express a move between layers"
            );
        }
    }

    /// A frame combining a symmetry with a recolor renders whole, matching
    /// `render_frame`'s own commuting-order guarantee one level up.
    #[test]
    fn a_combined_symmetry_and_recolor_frame_renders() {
        let plane = PlaneElement::all()[1]; // a non-identity element
        let relabel = Relabel::new(vec![(0, 1), (1, 0)]);
        let frame = ObserverFrame { plane, relabel };
        let program = render_vframe(&frame, 2).expect("a named frame renders");
        let a = probe(2);
        let ma = measure_volume(&a).expect("whole");
        let expected = frame.apply(&ma).expect("whole");
        assert_eq!(measure_volume(&run_volume(&program, &a)).expect("whole"), expected);
    }
}

/// **THINK THE TASK** — the ARC seat. Peel the train pairs into measurements,
/// read them through the ONE reduce organ (`reduct::think`), and — only when the
/// task GLUES a nameable frame — render it to a program, apply it to the
/// held-out input, and seal the proof. Every other verdict WITHHOLDS: a task
/// that drained without expressing a frame (ByDrain), disagreed across pairs
/// (Unglueable), learned a wall (Withheld), or gave too few observations returns
/// `Unsolved` with NO output (Pin D). The render never invents an Op, never
/// sweeps; the certificate + train reproduction is the pin (Pin A) — a frame
/// that does not reproduce every train output is withheld, not emitted. This
/// path calls neither `search::solve` nor any `propose_*` / `sweep_vocabulary`.
/// **THE SAME-FRAME SEAT** — the D₄×recolor recognition, factored so the tile
/// seat can reuse it on the untiled base. Peel every pair, hand the family to
/// the ONE reduce organ, render the glued plane+relabel frame. `None` when a
/// grid will not measure, the family glues no single rule, or the frame is
/// unnameable. Runs `think` exactly once (the caller ensures one think/task).
fn recognize_frame(pairs: &[(&Grid, &Grid)], grown: &mut Vec<Crystal>) -> Option<Program> {
    recognize_frame_conjecturing(pairs, grown, &mut Vec::new())
}

/// [`recognize_frame`], keeping what an unglued thought leaves behind: every
/// frame that closed SOME pairs, rendered, into `conjectures` (Concordance
/// 1.10, restore 1). A conjecture is never the answer here.
fn recognize_frame_conjecturing(
    pairs: &[(&Grid, &Grid)],
    grown: &mut Vec<Crystal>,
    conjectures: &mut Vec<Conjecture>,
) -> Option<Program> {
    let mut observations = Vec::with_capacity(pairs.len());
    for (input, output) in pairs {
        match (measure_grid(input), measure_grid(output)) {
            (Ok(a), Ok(b)) => observations.push((a, b)),
            _ => return None,
        }
    }
    match think(&observations, grown) {
        TaskThought::Glued(PairClosure::Frame(frame)) => render_frame(&frame),
        TaskThought::UnglueableAcrossObservations { conjectures: held, .. } => {
            for (frame, closed) in held {
                if let Some(program) = render_frame(&frame) {
                    conjectures.push(Conjecture { program, closed, of: pairs.len() });
                }
            }
            None
        }
        // ByDrain / Withheld close no single renderable frame.
        _ => None,
    }
}

/// **A HELD CONJECTURE** (Concordance 1.10, the audit's restore 1: a mind
/// may hold and test its own hypotheses) — a program an earlier thought
/// rendered from a frame that closed `closed` of its task's `of` pairs. It is
/// PRIVATE thought: held by its vessel, tried first on every later task, and
/// a CLAIM only where it closes every pair of one (Pin A, unchanged).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjecture {
    pub program: Program,
    pub closed: usize,
    pub of: usize,
}

/// **DECLARED BOUND**: how many conjectures a vessel holds at once. Memory is
/// finite; the oldest gives way. A dial, recorded in docs/59, not a law.
pub const HELD_CONJECTURES: usize = 32;

/// Hold `fresh` conjectures in `held`: each program once, the oldest giving
/// way past [`HELD_CONJECTURES`].
pub fn hold_conjectures(held: &mut Vec<Conjecture>, fresh: Vec<Conjecture>) {
    for c in fresh {
        if held.iter().any(|h| h.program == c.program) {
            continue;
        }
        if held.len() >= HELD_CONJECTURES {
            held.remove(0);
        }
        held.push(c);
    }
}

/// The top-left `br × bc` block of a grid — the tiling's (0,0) cell, which is
/// unmirrored by construction, so it IS the base the tile repeats.
fn top_left_block(g: &Grid, br: usize, bc: usize) -> Option<Grid> {
    if br == 0 || bc == 0 || br > g.rows || bc > g.cols {
        return None;
    }
    let mut cells = Vec::with_capacity(br * bc);
    for r in 0..br {
        for c in 0..bc {
            cells.push(g.at(r, c));
        }
    }
    Grid::new(br, bc, cells)
}

/// The `(tile_r, tile_c)` block of a tiled grid — `br × bc` at offset
/// `(tile_r·br, tile_c·bc)`.
fn block_at(g: &Grid, tile_r: usize, tile_c: usize, br: usize, bc: usize) -> Option<Grid> {
    let (r0, c0) = (tile_r * br, tile_c * bc);
    if br == 0 || bc == 0 || r0 + br > g.rows || c0 + bc > g.cols {
        return None;
    }
    let mut cells = Vec::with_capacity(br * bc);
    for r in 0..br {
        for c in 0..bc {
            cells.push(g.at(r0 + r, c0 + c));
        }
    }
    Grid::new(br, bc, cells)
}

/// A `TileMirror` as the grid word that realises it — `None`=identity,
/// `Rows`=vertical flip (row index reversed), `Cols`=horizontal flip, `Both`=180°.
/// The tile transport reversed into the D₄ vocabulary the render already speaks.
fn mirror_word(m: TileMirror) -> Program {
    match m {
        TileMirror::None => Vec::new(),
        TileMirror::Rows => vec![Op::FlipV],
        TileMirror::Cols => vec![Op::FlipH],
        TileMirror::Both => vec![Op::Rotate180],
    }
}

/// **READ the band mirror off the carrier** — a `TileMirror` is one direction of
/// the tile transport, so an odd band `block` IS `base` under exactly one of the
/// four reflections. A single walk over the 4-element `TileMirror` carrier reads
/// it directly — no cartesian product, no re-tiling of the whole output.
fn read_mirror(block: &Grid, base: &Grid) -> Option<TileMirror> {
    TileMirror::ALL
        .into_iter()
        .find(|m| run(&mirror_word(*m), base) == *block)
}

/// **THE TILE SEAT** — the first frame-MORPHISM the ARC seat expresses (the
/// render-expressiveness ceiling that saturated the D₄×recolor curriculum). A
/// frame morphism CHANGES the frame — `output = Tile{d,a,mirrors}(base)`, a
/// `br × bc` base repeated `d × a` with the odd bands mirrored — so the
/// same-frame gauge organ cannot see it. This PEELS the tile-LAST word (no
/// sweep, per the peels-and-shears law):
///   1. derive `(d, a)` from the dimension ratio (the base is a D₄ image of the
///      input, so its dims are the input's, or transposed);
///   2. READ the odd-band mirrors straight off the carrier — `odd_row` from
///      block (1,0), `odd_col` from block (0,1) — each a single walk over the
///      4-element `TileMirror`, never the 16-way product;
///   3. verify the tile CHEAPLY (`Tile(base0) == out0`, one apply) BEFORE paying
///      the reduce organ, then recognise the base transform `input → base0`
///      through it exactly ONCE.
/// Pin A (the caller) is the authority: a candidate that does not replay every
/// pair withholds.
fn recognize_tile(pairs: &[(&Grid, &Grid)], grown: &mut Vec<Crystal>) -> Option<Program> {
    let (in0, out0) = *pairs.first()?;
    // The base is a D₄ image of the input: its dims are the input's, or swapped.
    for (br, bc) in [(in0.rows, in0.cols), (in0.cols, in0.rows)] {
        if br == 0 || bc == 0 || out0.rows % br != 0 || out0.cols % bc != 0 {
            continue;
        }
        let (down, across) = (out0.rows / br, out0.cols / bc);
        if down * across <= 1 {
            continue; // no growth — the same-frame seat already owns this
        }
        let Some(base0) = top_left_block(out0, br, bc) else { continue };
        // READ each odd-band mirror off its own block — two 1-D reads, not a
        // 2-D product. A band that is not a reflection of the base is no tiling.
        let odd_row = if down >= 2 {
            match block_at(out0, 1, 0, br, bc).and_then(|b| read_mirror(&b, &base0)) {
                Some(m) => m,
                None => continue,
            }
        } else {
            TileMirror::None
        };
        let odd_col = if across >= 2 {
            match block_at(out0, 0, 1, br, bc).and_then(|b| read_mirror(&b, &base0)) {
                Some(m) => m,
                None => continue,
            }
        } else {
            TileMirror::None
        };
        let tile = Op::Tile { down, across, odd_row, odd_col };
        // CHEAP structural pin (one apply) — the derived tile must reproduce the
        // whole output from its own (0,0) block. Only then pay the reduce organ.
        if run(&vec![tile.clone()], &base0) != *out0 {
            continue;
        }
        // The ONE reduce-organ call: the base transform `input → base0`, family-wide.
        let bases: Option<Vec<(Grid, Grid)>> = pairs
            .iter()
            .map(|(input, output)| top_left_block(output, br, bc).map(|b| ((*input).clone(), b)))
            .collect();
        let Some(bases) = bases else { continue };
        let base_refs: Vec<(&Grid, &Grid)> = bases.iter().map(|(i, b)| (i, b)).collect();
        let Some(mut word) = recognize_frame(&base_refs, grown) else { continue };
        word.push(tile);
        return Some(word);
    }
    None
}

/// **A PURE TILING** — the `Tile` op that reproduces `tiled` from `base` (base
/// repeated `down×across` with the odd bands mirrored), or `None` if `tiled` is
/// not such a tiling of `base`. Derive `(down,across)` from the dimension ratio
/// and READ the mirrors off blocks (1,0)/(0,1); the cheap apply is the pin.
fn recognize_pure_tile(base: &Grid, tiled: &Grid) -> Option<Op> {
    if base.rows == 0 || base.cols == 0 || tiled.rows % base.rows != 0 || tiled.cols % base.cols != 0
    {
        return None;
    }
    let (down, across) = (tiled.rows / base.rows, tiled.cols / base.cols);
    if down * across <= 1 {
        return None;
    }
    let odd_row = if down >= 2 {
        read_mirror(&block_at(tiled, 1, 0, base.rows, base.cols)?, base)?
    } else {
        TileMirror::None
    };
    let odd_col = if across >= 2 {
        read_mirror(&block_at(tiled, 0, 1, base.rows, base.cols)?, base)?
    } else {
        TileMirror::None
    };
    let tile = Op::Tile { down, across, odd_row, odd_col };
    (run(&vec![tile.clone()], base) == *tiled).then_some(tile)
}

/// The D₄ inverse permutation over `d4_words()` — `inv[i]` is the word that
/// undoes `d4_words()[i]` (matched on a 2×2 probe, a structural constant, never
/// a search). Used to un-apply a tile-FIRST word's trailing transform.
fn d4_inverse_map() -> [usize; 8] {
    let probe = Grid::new(2, 2, vec![0, 1, 2, 3]).expect("the 2×2 probe is lawful");
    let words = d4_words();
    let mut inv = [0usize; 8];
    for (i, wi) in words.iter().enumerate() {
        for (j, wj) in words.iter().enumerate() {
            if run(wi, &run(wj, &probe)) == probe {
                inv[i] = j;
                break;
            }
        }
    }
    inv
}

/// **THE TILE-FIRST SEAT** — the frame morphism the OTHER way round: `output =
/// transform(Tile(input))`, the tile applied FIRST then a D₄ word. The tile seat
/// proper reads tile-LAST; here, for each of the 8 D₄ finals, un-apply it (its
/// inverse) to recover the pre-transform tiled grid, and check THAT is a pure
/// tiling of the input. A structural enumeration bounded by the plane group,
/// never a program search; Pin A (the caller) is the authority.
fn recognize_tile_first(pairs: &[(&Grid, &Grid)]) -> Option<Program> {
    let (in0, out0) = *pairs.first()?;
    let words = d4_words();
    let inv = d4_inverse_map();
    for (i, final_word) in words.iter().enumerate() {
        let tiled0 = run(&words[inv[i]], out0); // final⁻¹(output) = the tiled grid
        let Some(tile) = recognize_pure_tile(in0, &tiled0) else { continue };
        let mut word = vec![tile];
        word.extend(final_word.iter().cloned());
        if run(&word, in0) == *out0 {
            return Some(word);
        }
    }
    None
}

/// **THE OBJECT SEAT** — the segmentation morphisms (keep/delete/crop over the
/// 4-connected non-background objects; `CropContent` IS extract-non-background).
/// These are neither a plane word nor a tiling, so the frame and tile seats
/// withhold them. Recognise the PURE object op: the single `ObjOp` whose action
/// reproduces every train pair. A bounded walk over the fixed object vocabulary
/// (`OBJ_OPS`), never a program search; Pin A (the caller) is the authority.
fn recognize_object(pairs: &[(&Grid, &Grid)]) -> Option<Program> {
    for objop in solver_verify::objects::OBJ_OPS {
        let program = vec![Op::Object(objop)];
        if pairs.iter().all(|&(input, output)| run(&program, input) == *output) {
            return Some(program);
        }
    }
    None
}

/// A grid's hue profile: the counts of the hues it carries, sorted. A plane
/// motion and a hue relabel both preserve it, so two grids whose profiles
/// differ cannot be one D4 x recolor apart.
fn hue_profile(g: &Grid) -> Vec<usize> {
    let mut counts = [0usize; HUES as usize];
    for &h in &g.cells {
        if let Some(c) = counts.get_mut(h as usize) {
            *c += 1;
        }
    }
    let mut profile: Vec<usize> = counts.into_iter().filter(|&c| c > 0).collect();
    profile.sort_unstable();
    profile
}

/// **THE LAYERED SEAT, OBJECT THEN FRAME** (docs/60 D7, slice 1): the bodies
/// first, then what is left of the change read through the ONE reduce organ.
/// For each object op that ACTS on the family, the staged grids (`op(input)`)
/// are paired with the outputs; where every staged grid could be one plane
/// motion and hue relabel from its output (same frame up to a transpose, the
/// same hue profile), the organ is asked once for that frame, and the word is
/// the object op followed by it. A bounded walk over the fixed object
/// vocabulary with a cheap structural check before each think; Pin A (the
/// caller) is the authority.
fn recognize_object_then_frame(pairs: &[(&Grid, &Grid)], grown: &mut Vec<Crystal>) -> Option<Program> {
    for objop in solver_verify::objects::OBJ_OPS {
        let first = vec![Op::Object(objop)];
        let staged: Vec<Grid> = pairs.iter().map(|(input, _)| run(&first, input)).collect();
        if pairs.iter().zip(&staged).all(|((input, _), s)| s == *input) {
            continue; // the op does nothing to this family
        }
        let fits = pairs.iter().zip(&staged).all(|((_, output), s)| {
            ((s.rows, s.cols) == (output.rows, output.cols) || (s.rows, s.cols) == (output.cols, output.rows))
                && hue_profile(s) == hue_profile(output)
        });
        if !fits {
            continue;
        }
        let staged_pairs: Vec<(&Grid, &Grid)> = staged.iter().zip(pairs).map(|(s, (_, output))| (s, *output)).collect();
        if let Some(rest) = recognize_frame(&staged_pairs, grown) {
            let mut word = first;
            word.extend(rest);
            return Some(word);
        }
    }
    None
}

/// **THE LAYERED SEAT, FRAME THEN OBJECT** (docs/60 D7b): a plane motion
/// first, then an object op on what it made. A bounded walk over the seven
/// non-identity D4 words and the fixed object vocabulary, each candidate
/// checked by direct application on every pair (no think, no search beyond
/// the two finite carriers); Pin A (the caller) is the authority.
fn recognize_frame_then_object(pairs: &[(&Grid, &Grid)]) -> Option<Program> {
    for motion in d4_words().into_iter().filter(|w| !w.is_empty()) {
        for objop in solver_verify::objects::OBJ_OPS {
            let mut program = motion.clone();
            program.push(Op::Object(objop));
            if pairs.iter().all(|&(input, output)| run(&program, input) == *output) {
                return Some(program);
            }
        }
    }
    None
}

/// **THINK THE TASK** — the ARC seat. Read the train pairs through the ONE
/// reduce organ and render its verdict to a program, or WITHHOLD. Dispatch on
/// the frame: when the output shares the input's frame (up to a D₄ transpose)
/// the same-frame seat names a D₄×recolor word; when the output is a strict
/// MULTIPLE of it, the tile seat names a frame morphism. Every other verdict
/// returns `Unsolved` with NO output (Pin D). The render never invents an Op;
/// the certificate + train reproduction is the pin (Pin A). No `search::solve`,
/// no `propose_*`, no `sweep_vocabulary`.
pub fn think_task(task: &Task, grown: &mut Vec<Crystal>) -> SolveRead {
    think_task_held(task, grown, &mut Vec::new())
}

/// [`think_task`] by a mind that HOLDS CONJECTURES (Concordance 1.10, restore
/// 1). Its held hypotheses are tried FIRST: one that reproduces every train
/// pair of this task is no longer a conjecture but this task's answer, and
/// leaves the held set. Otherwise the organ thinks as ever, and the frames an
/// unglued thought closed SOME pairs with are held for later tasks. Every
/// answer still passes Pin A and its certificate — a claim closes or it is
/// none.
pub fn think_task_held(task: &Task, grown: &mut Vec<Crystal>, held: &mut Vec<Conjecture>) -> SolveRead {
    think_task_sensing(task, grown, held, crate::senses::Senses::ALL)
}

/// [`think_task_held`] by a mind with `senses` (docs/60 D1): a program the
/// mind could not have perceived (it acts in a channel these senses lack) is
/// withheld, and never held as a conjecture. A sense bounds derivation only;
/// the caller's library words are used whole before this is reached.
pub fn think_task_sensing(
    task: &Task,
    grown: &mut Vec<Crystal>,
    held: &mut Vec<Conjecture>,
    senses: crate::senses::Senses,
) -> SolveRead {
    use crate::senses::Senses;
    let Some(first) = task.train.first() else {
        return SolveRead::Unsolved; // no pairs proves nothing
    };
    if let Some(at) = held.iter().position(|c| task.train.iter().all(|pair| run(&c.program, &pair.input) == pair.output)) {
        let program = held.remove(at).program;
        return seal_answer(task, program);
    }
    let (in0, out0) = (&first.input, &first.output);
    let pairs: Vec<(&Grid, &Grid)> = task.train.iter().map(|p| (&p.input, &p.output)).collect();
    // A D₄ image preserves the frame or transposes it; anything else is a frame
    // CHANGE — the tile seat's business (one think per task either way).
    let same_frame = (out0.rows, out0.cols) == (in0.rows, in0.cols)
        || (out0.rows, out0.cols) == (in0.cols, in0.rows);
    let mut fresh = Vec::new();
    let program = if same_frame {
        recognize_frame_conjecturing(&pairs, grown, &mut fresh)
    } else {
        // Frame change: tile-LAST first (the reduce organ recognises the base
        // transform), then tile-FIRST (a D₄ word after the tile) — both peel.
        match recognize_tile(&pairs, grown) {
            Some(p) => Some(p),
            None => recognize_tile_first(&pairs),
        }
    }
    // A pure object op (segmentation morphism) is neither a plane word nor a
    // tiling — try it when the frame/tile seats withhold, in either frame regime.
    .or_else(|| recognize_object(&pairs))
    // docs/60 D7: a body changed AND moved or recoloured — layered.
    .or_else(|| recognize_object_then_frame(&pairs, grown))
    .or_else(|| recognize_frame_then_object(&pairs))
    .filter(|program| senses.covers(Senses::of_word(program)));
    let Some(program) = program else {
        fresh.retain(|c| senses.covers(Senses::of_word(&c.program)));
        hold_conjectures(held, fresh);
        return SolveRead::Unsolved;
    };
    seal_answer(task, program)
}

/// Pin A and the certificate: an answer reproduces EVERY train output
/// exactly, or it is a withhold — never an emitted guess.
fn seal_answer(task: &Task, program: Program) -> SolveRead {
    if task.train.iter().any(|pair| run(&program, &pair.input) != pair.output) {
        return SolveRead::Unsolved;
    }
    let test_output = run(&program, &task.test_in);
    let certificate = SolveCertificate::seal(task, &program, &test_output);
    if verify_certificate(&certificate, task, &program, &test_output) != CertificateRead::Intact {
        return SolveRead::Unsolved;
    }
    let matches_expected = task.test_expected.as_ref().map(|expected| *expected == test_output);
    SolveRead::Solved { program, test_output, matches_expected, certificate }
}

// ─────────────────────────────────────────────────────────────────────
// N-I10 — HEAR: the crystal-exchange primitive (an utterance drained or walled).
// ─────────────────────────────────────────────────────────────────────

/// How a hearer received an utterance — the ONLY verdicts hearing renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heard {
    /// The hearer's basin DRAINED the utterance — already accounted (understood).
    /// Nothing adopted; the hearer knew it (or it is a universal gauge).
    Understood,
    /// The utterance WALLED — the hearer could not account it; the crystal is
    /// adopted onto the hearer's OWN basin (learned).
    Learned,
}

/// **HEAR** (N-I10) — a folded word / crystal offered to a hearer, as a
/// measurement pair, reduced against the hearer's OWN basin (`reduct::observe` — the
/// SAME reduce organ as think). `Accounted` → `Understood` (drain, nothing
/// adopted); a wall → `Learned`, and the crystal is adopted onto `grown` ONLY.
/// No chat, no score, and NEVER a copy of the speaker's basin — the hearer
/// reaches the utterance through its own reduce or walls and mints its own
/// crystal. The caller prices the look (N-I7); this owns no leisure.
pub fn hear(a: &FullMeasurement, b: &FullMeasurement, grown: &mut Vec<Crystal>) -> Heard {
    match reduct::observe(a, b, grown) {
        reduct::Observed::Accounted => Heard::Understood,
        reduct::Observed::Minted(crystal) => {
            reduct::adopt(grown, crystal);
            Heard::Learned
        }
        reduct::Observed::Carried => Heard::Learned,
    }
}

/// N-I10 — HEAR a folded word (a `Program`): the word is the utterance, offered
/// as the pair `(probe, word(probe))` and heard against the hearer's basin. A
/// pure-geometry word is a universal gauge (drains → `Understood`); a word that
/// recolors a probe hue to one the probe never carried is a generation gap
/// (walls → `Learned`, adopted onto the hearer's basin only). The transducer
/// twin of `measure_grid` for the survival/ARC seats — no `cfe`, no leisure (the
/// caller prices the look).
pub fn hear_word(expansion: &[Op], grown: &mut Vec<Crystal>) -> Heard {
    // A probe with a small palette so a recolor OUT of it reads as novel.
    let probe = match Grid::new(3, 3, vec![1, 0, 2, 0, 1, 0, 2, 0, 1]) {
        Some(g) => g,
        None => return Heard::Understood,
    };
    let uttered = run(&expansion.to_vec(), &probe);
    match (measure_grid(&probe), measure_grid(&uttered)) {
        (Ok(a), Ok(b)) => hear(&a, &b, grown),
        _ => Heard::Understood,
    }
}
