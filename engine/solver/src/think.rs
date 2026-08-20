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
use solver_verify::grid::{run, Grid, Op, Program, HUES};
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
pub fn think_task(task: &Task, grown: &mut Vec<Crystal>) -> SolveRead {
    // Peel every train pair; a grid we cannot measure withholds the whole task.
    let mut observations = Vec::with_capacity(task.train.len());
    for pair in &task.train {
        match (measure_grid(&pair.input), measure_grid(&pair.output)) {
            (Ok(a), Ok(b)) => observations.push((a, b)),
            _ => return SolveRead::Unsolved,
        }
    }
    // The ONE reduce organ owns the judgment; the seat only renders its verdict.
    let program = match think(&observations, grown) {
        TaskThought::Glued(PairClosure::Frame(frame)) => match render_frame(&frame) {
            Some(program) => program,
            None => return SolveRead::Unsolved, // unnameable frame — withhold
        },
        // ByDrain closed against the grown basis but expressed no renderable
        // frame; the other verdicts closed no single rule. No output (Pin D).
        _ => return SolveRead::Unsolved,
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
