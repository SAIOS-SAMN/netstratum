//! **THE ARC SEAT** (N-I3, doc `53_INTELLIGENCE_PORT` D1/D2) — the solver reads
//! its train grids through the ONE reduce organ. It owns NO search: it peels the
//! observed pairs into measurements, hands them to `reduct::think`, and — only when
//! the task GLUES a nameable frame — renders that frame to a program and seals
//! the proof. Every other verdict WITHHOLDS. `search::solve` / `propose_*` /
//! `sweep_vocabulary` are the costume this seat replaces; none is called here.
//!
//! Slice 2 (this landing): the `Grid → FullMeasurement` transducer only.
//! Slice 3 adds `think_task`, the render table, and the certificate.

use crate::search::SolveRead;
use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, MeasurementRefusal, Site};
use reduct::reduction::{ObserverFrame, PlaneElement, Relabel};
use reduct::{think, Crystal, PairClosure, TaskThought};
use solver_verify::certificate::{verify_certificate, CertificateRead, SolveCertificate};
use solver_verify::grid::{run, Grid, Op, Program, TileMirror, HUES};
use solver_verify::loader::Task;

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
            (Site { row: r as i64, col: c as i64 }, GaugeDirection { index: grid.at(r, c) })
        })
        .collect();
    FullMeasurement::try_new(
        HUES,
        CellFrame { rows: grid.rows as u32, cols: grid.cols as u32 },
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
    let mut observations = Vec::with_capacity(pairs.len());
    for (input, output) in pairs {
        match (measure_grid(input), measure_grid(output)) {
            (Ok(a), Ok(b)) => observations.push((a, b)),
            _ => return None,
        }
    }
    match think(&observations, grown) {
        TaskThought::Glued(PairClosure::Frame(frame)) => render_frame(&frame),
        // ByDrain / Unglueable / Withheld close no single renderable frame.
        _ => None,
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

/// **THINK THE TASK** — the ARC seat. Read the train pairs through the ONE
/// reduce organ and render its verdict to a program, or WITHHOLD. Dispatch on
/// the frame: when the output shares the input's frame (up to a D₄ transpose)
/// the same-frame seat names a D₄×recolor word; when the output is a strict
/// MULTIPLE of it, the tile seat names a frame morphism. Every other verdict
/// returns `Unsolved` with NO output (Pin D). The render never invents an Op;
/// the certificate + train reproduction is the pin (Pin A). No `search::solve`,
/// no `propose_*`, no `sweep_vocabulary`.
pub fn think_task(task: &Task, grown: &mut Vec<Crystal>) -> SolveRead {
    let Some(first) = task.train.first() else {
        return SolveRead::Unsolved; // no pairs proves nothing
    };
    let (in0, out0) = (&first.input, &first.output);
    let pairs: Vec<(&Grid, &Grid)> = task.train.iter().map(|p| (&p.input, &p.output)).collect();
    // A D₄ image preserves the frame or transposes it; anything else is a frame
    // CHANGE — the tile seat's business (one think per task either way).
    let same_frame = (out0.rows, out0.cols) == (in0.rows, in0.cols)
        || (out0.rows, out0.cols) == (in0.cols, in0.rows);
    let program = if same_frame {
        recognize_frame(&pairs, grown)
    } else {
        recognize_tile(&pairs, grown)
    };
    let Some(program) = program else {
        return SolveRead::Unsolved;
    };
    // Pin A: the program must reproduce EVERY train output exactly. A render
    // that does not reproduce is a withhold, never an emitted guess.
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
