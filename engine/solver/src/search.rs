//! The proposer/verifier loop — enumerative search over short programs.
//!
//! The verifier is the authority: a candidate is a solution ONLY if it
//! maps EVERY training pair exactly. The honest withhold outranks a
//! fabricated render — `Unsolved` is a lawful reading, a guess is not.

use solver_verify::certificate::SolveCertificate;
use solver_verify::grid::{run, Grid, Op, Program};
use solver_verify::loader::Task;

/// The typed solve reading.
#[derive(Debug, Clone)]
pub enum SolveRead {
    /// A program closed every training pair; the rendered test output
    /// and its proof chain ride along. `matches_expected` is measured
    /// only when the archive carries the held-out answer.
    Solved {
        program: Program,
        test_output: Grid,
        matches_expected: Option<bool>,
        certificate: SolveCertificate,
    },
    /// No program in the searched vocabulary closes the training
    /// pairs. Typed, honest, final for this vocabulary and depth.
    Unsolved,
}

/// Derive candidate recolor maps from the training pairs: for each
/// geometric normalization prefix (identity + the six symmetries), if
/// every pair reads cell-wise consistent between `prefix(input)` and
/// `output`, the observed hue mapping is a candidate operator. The
/// maps are observed content, never a rule table.
fn derived_recolors(task: &Task) -> Vec<Op> {
    let prefixes: [Option<Op>; 7] = [
        None,
        Some(Op::Rotate90),
        Some(Op::Rotate180),
        Some(Op::Rotate270),
        Some(Op::FlipH),
        Some(Op::FlipV),
        Some(Op::Transpose),
    ];
    let mut found: Vec<Op> = Vec::new();
    'prefix: for prefix in &prefixes {
        let mut map = [255u8; 10];
        for pair in &task.train {
            let seen = match prefix {
                None => pair.input.clone(),
                Some(op) => op.apply(&pair.input),
            };
            if seen.rows != pair.output.rows || seen.cols != pair.output.cols {
                continue 'prefix;
            }
            for (i, &from) in seen.cells.iter().enumerate() {
                let to = pair.output.cells[i];
                match map[from as usize] {
                    255 => map[from as usize] = to,
                    already if already == to => {}
                    _ => continue 'prefix, // inconsistent under this frame
                }
            }
        }
        // Unobserved hues map to themselves.
        for (h, slot) in map.iter_mut().enumerate() {
            if *slot == 255 {
                *slot = h as u8;
            }
        }
        let op = Op::Recolor(map);
        if !found.contains(&op) {
            found.push(op);
        }
    }
    found
}

/// Derive candidate tilings from the first pair's shape ratio —
/// structure read off the observation, parameters searched generally.
fn derived_tiles(task: &Task) -> Vec<Op> {
    let first = &task.train[0];
    let (ir, ic) = (first.input.rows, first.input.cols);
    let (or, oc) = (first.output.rows, first.output.cols);
    if or % ir != 0 || oc % ic != 0 {
        return Vec::new();
    }
    let (down, across) = (or / ir, oc / ic);
    if down * across <= 1 || down > 6 || across > 6 {
        return Vec::new();
    }
    let mut ops = Vec::new();
    for odd_row in solver_verify::grid::TileMirror::ALL {
        for odd_col in solver_verify::grid::TileMirror::ALL {
            ops.push(Op::Tile {
                down,
                across,
                odd_row,
                odd_col,
            });
        }
    }
    ops
}

/// A program closes the task iff it maps every training pair exactly.
fn closes(program: &Program, task: &Task) -> bool {
    task.train
        .iter()
        .all(|pair| run(program, &pair.input) == pair.output)
}

/// FORGE-BOUND: the deterministic search budget — a COUNT of candidate
/// programs tested, never a clock. Same task, same outcome, bounded
/// work: a seed whose solution lies past the budget reads Unsolved and
/// the line rests that beat (measured live: an unbudgeted rung-5 seed
/// cost 102 s of exhaustive search in one heartbeat). This is an
/// ENGINE LAW — the bound on one beat's forge work — and is NEVER a
/// test knob (operator ruling 2026-07-30): tests scale their eras,
/// not the physics.
pub const SEARCH_BUDGET: usize = 200_000;

/// Enumerative search over programs up to `max_depth` ops drawn from
/// the general vocabulary plus operators derived from the observation.
/// Bounded by `SEARCH_BUDGET` candidates, deterministically.
pub fn solve(task: &Task, max_depth: usize) -> SolveRead {
    let mut vocabulary = vec![
        Op::Rotate90,
        Op::Rotate180,
        Op::Rotate270,
        Op::FlipH,
        Op::FlipV,
        Op::Transpose,
    ];
    vocabulary.extend(solver_verify::objects::OBJ_OPS.iter().cloned().map(Op::Object));
    vocabulary.extend(derived_recolors(task));
    vocabulary.extend(derived_tiles(task));

    // Depth 0: the identity word (a copy task is a lawful task).
    let mut tested: usize = 0;
    let mut frontier: Vec<Program> = vec![Vec::new()];
    for _depth in 0..=max_depth {
        for program in &frontier {
            tested += 1;
            if tested > SEARCH_BUDGET {
                return SolveRead::Unsolved;
            }
            if closes(program, task) {
                let test_output = run(program, &task.test_in);
                let matches_expected =
                    task.test_expected.as_ref().map(|want| want == &test_output);
                let certificate = SolveCertificate::seal(task, program, &test_output);
                return SolveRead::Solved {
                    program: program.clone(),
                    test_output,
                    matches_expected,
                    certificate,
                };
            }
        }
        // Never materialize past the budget — the frontier a beat
        // cannot afford to test is a frontier it never builds.
        let room = SEARCH_BUDGET.saturating_sub(tested) + 1;
        let mut next = Vec::new();
        'grow: for program in &frontier {
            for op in &vocabulary {
                if next.len() >= room {
                    break 'grow;
                }
                let mut extended = program.clone();
                extended.push(op.clone());
                next.push(extended);
            }
        }
        frontier = next;
    }
    SolveRead::Unsolved
}

// ── THE SECTOR DOOR ────────────────────────────────────────────────────

/// `SolveRead` is exactly representable — grids, ops and a hash chain,
/// no float anywhere — so it may cross the cage door.
impl rfa_core::Exact for SolveRead {}

/// SEARCH, CONSULTED — the first caller of the sector door.
///
/// [`solve`] is the dirtiest work this system does: an enumerative sweep
/// over a vocabulary, bounded only by a candidate budget, with no claim
/// to purity and every freedom to become heuristic, parallel or
/// GPU-borne later. That is exactly what the sector is FOR, and this is
/// the door it goes through.
///
/// **Approximation may PROPOSE; only exactness may DISPOSE.** The search
/// runs inside and proposes a `SolveRead`. It is withdrawn only if the
/// program it names maps EVERY training pair exactly — the honest
/// withhold law, one miss and it does not cross. A proposal that fails
/// is not an error and costs nothing: it reads `Unsolved`, which is what
/// an unproved solve has always meant here.
///
/// Nothing about having come from the search makes a program true. This
/// re-derivation is the same one `verify_work` step 2 performs later at
/// the kernel; doing it here means a candidate that could never pass the
/// judge never leaves the workshop.
pub fn solve_consulted(task: &Task, max_depth: usize) -> SolveRead {
    rfa_core::consult((task, max_depth), |(t, d)| solve(t, d))
        .withdraw(|read| closes_every_training_pair(task, read))
        .unwrap_or(SolveRead::Unsolved)
}

/// The exact rule the OUTSIDE applies: re-run the proposed program on
/// every training pair and demand byte-exact agreement. An `Unsolved`
/// proposal is lawful and crosses unchanged — it claims nothing.
fn closes_every_training_pair(task: &Task, read: &SolveRead) -> bool {
    match read {
        SolveRead::Unsolved => true,
        SolveRead::Solved { program, .. } => task
            .train
            .iter()
            .all(|pair| solver_verify::grid::run(program, &pair.input) == pair.output),
    }
}
