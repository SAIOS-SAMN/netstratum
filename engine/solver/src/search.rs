//! The proposer/verifier loop — enumerative search over short programs.
//!
//! The verifier is the authority: a candidate is a solution ONLY if it
//! maps EVERY training pair exactly. The honest withhold outranks a
//! fabricated render — `Unsolved` is a lawful reading, a guess is not.

use crate::derive::derive;
use crate::need::Need;
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

/// THE SWEEP — SUPERSEDED (operator ruling 2026-08-04: *never
/// enumerate*). Kept as the measured baseline, not as the intended
/// path; [`propose`] is the ruled proposer.
///
/// Enumerative search over programs up to `max_depth` ops drawn from
/// the general vocabulary plus operators derived from the observation.
/// Bounded by `SEARCH_BUDGET` candidates, deterministically.
///
/// **Why it was struck.** It sweeps a fixed vocabulary identically for
/// every mind in every era, reading nothing about what the civilization
/// lacks — the definition of brute force. Measured on the 960-task
/// corpus it also saturates: confirmed stops at 33 from depth 3, and
/// depth 4 buys ONLY overfitting (37 closed, still 33 confirmed — four
/// false closures where there had been one). Its marginal product is
/// negative past depth 3.
///
/// **What it still buys, stated honestly.** It composes over the whole
/// vocabulary, so it can pass through intermediate states that no
/// observation motivates, and it finds 7 tasks the derived reach does
/// not (33 confirmed vs 26). Those are exactly the solves that are not
/// derivable — reached by exhaustion rather than by reading. Whether
/// the economy should pay for them is the operator's call; the numbers
/// are in `examples/reach_vs_sweep.rs`.
pub fn solve(task: &Task, max_depth: usize) -> SolveRead {
    // THE VACUOUS SOLVE, REFUSED (operator ruling C8, U_DIRECTIVES §10
    // S1). With no training pair, `closes` is true over nothing and the
    // empty program would "solve" at depth 0 — a claim nobody earned.
    // The honest withhold is the lawful reading: nothing to close means
    // nothing proved. This guard is also the TOTALITY guard — the
    // derived-operator path reads `train[0]` and an empty train drove it
    // to panic in-process, the same class as audit HOLE B.
    if task.train.is_empty() {
        return SolveRead::Unsolved;
    }
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

// ── THE NEED-DRIVEN PROPOSER (operator ruling 2026-08-04) ─────────────

/// What a proposal cost, so "we stopped enumerating" is a measurement
/// rather than a claim.
#[derive(Debug, Clone)]
pub struct Proposal {
    pub read: SolveRead,
    /// Candidate programs actually tested.
    pub tested: usize,
    /// Operators the observation supported — the whole reach.
    pub reach: usize,
}

/// THE PROPOSER. The observation supplies the reach; the need orders it;
/// exactness decides.
///
/// This replaces the sweep. [`solve`] enumerated the cross-product of a
/// fixed vocabulary to a fixed depth, bounded only by a 200,000
/// candidate budget. Here the candidates are DERIVED from what changed
/// between input and output ([`crate::derive`]), so a typical task
/// offers a handful rather than a hundred thousand, and composition is
/// bounded by what the mind is starved for rather than by a constant.
///
/// The need never extends the reach — [`Need::prefers`] reorders it and
/// [`Need::compose_limit`] says how deep it will compose. An operator
/// the observation does not support is never proposed, whatever the
/// mind wants, because wanting a thing is not evidence of it.
pub fn propose_measured(task: &Task, need: &Need) -> Proposal {
    // The vacuity law holds here as everywhere: nothing to close means
    // nothing proved (98ccbbe).
    if task.train.is_empty() {
        return Proposal { read: SolveRead::Unsolved, tested: 0, reach: 0 };
    }

    let first_reach = derive(task).len();
    let mut tested = 0usize;
    let seal = |program: &Program, tested: usize| {
        let test_output = run(program, &task.test_in);
        let matches_expected = task.test_expected.as_ref().map(|want| want == &test_output);
        let certificate = SolveCertificate::seal(task, program, &test_output);
        Proposal {
            read: SolveRead::Solved {
                program: program.clone(),
                test_output,
                matches_expected,
                certificate,
            },
            tested,
            reach: first_reach,
        }
    };

    // The identity word: a copy task is a lawful task, and costs one
    // test rather than a depth-0 frontier.
    tested += 1;
    if closes(&Vec::new(), task) {
        return seal(&Vec::new(), tested);
    }

    // RE-DERIVATION AT EACH STEP. The reach is not computed once and
    // then composed over — that would still be a sweep, just a shorter
    // one, and it misses every operator that only becomes visible after
    // a prefix moves the frame (gravity reads as nothing until the grid
    // is turned the right way up).
    //
    // Instead, after each applied operator the RESIDUAL is re-derived:
    // the training inputs are advanced through the program so far, and
    // the observation is asked again — *what remains to be done?* The
    // mind looks at what is still wrong and reaches for the operator
    // that answers it.
    let mut frontier: Vec<Program> = vec![Vec::new()];
    for _step in 0..need.compose_limit() {
        let mut next: Vec<Program> = Vec::new();
        for program in &frontier {
            for op in ordered_reach(&residual(task, program), need) {
                let mut extended = program.clone();
                extended.push(op);
                tested += 1;
                if closes(&extended, task) {
                    return seal(&extended, tested);
                }
                next.push(extended);
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    Proposal { read: SolveRead::Unsolved, tested, reach: first_reach }
}

/// THE RESIDUAL — what is still wrong after the program so far.
///
/// Each training input is advanced through the program; the outputs are
/// untouched. Deriving on this asks "what remains?" rather than "what
/// did the whole task need?", which is what lets an operator be found
/// whose precondition only appears mid-composition.
fn residual(task: &Task, program: &Program) -> Task {
    if program.is_empty() {
        return task.clone();
    }
    Task {
        name: task.name.clone(),
        train: task
            .train
            .iter()
            .map(|pair| solver_verify::loader::TaskPair {
                input: run(program, &pair.input),
                output: pair.output.clone(),
            })
            .collect(),
        test_in: run(program, &task.test_in),
        test_expected: task.test_expected.clone(),
    }
}

/// The reach the observation supports, ordered by what the mind lacks.
/// Preferred families first, derivation order preserved inside each
/// group — a stable partition, never a score.
fn ordered_reach(task: &Task, need: &Need) -> Vec<Op> {
    let reach = derive(task);
    let mut ordered: Vec<Op> = reach
        .iter()
        .filter(|c| need.prefers(c.family))
        .map(|c| c.op.clone())
        .collect();
    ordered.extend(
        reach
            .iter()
            .filter(|c| !need.prefers(c.family))
            .map(|c| c.op.clone()),
    );
    ordered
}

/// The proposer's reading alone, for callers that do not want the cost.
pub fn propose(task: &Task, need: &Need) -> SolveRead {
    propose_measured(task, need).read
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
    match consult_observed(task, max_depth) {
        Consultation::Crossed(read) => read,
        Consultation::Silent => SolveRead::Unsolved,
        Consultation::Withdrawn => SolveRead::Unsolved,
    }
}

/// WHAT HAPPENED AT THE DOOR — three outcomes, not two.
///
/// [`solve_consulted`] collapses `Withdrawn` and `Silent` into the same
/// `Unsolved`, because for a CALLER they are the same: neither crossed,
/// and neither claims anything. But they are not the same FACT, and the
/// difference is the only tell for a search that is degrading:
///
/// ```text
///     Crossed    proposed a program; the door re-derived it and it held
///     Silent     proposed nothing — lawful, claims nothing, costs nothing
///     Withdrawn  proposed a program that FAILED re-derivation
/// ```
///
/// A degrading search and a genuinely unsolvable task look identical
/// from outside precisely because that collapse throws `Withdrawn`
/// away. A rising withdrawal count means approximation is proposing
/// wrong programs and the door is catching them — which is the door
/// working, and worth knowing before it stops.
///
/// **This records nothing.** It reports one consultation to its caller;
/// where a FREQUENCY is wanted it is measured by experiment, never
/// tallied in the live system (`19_LENS` §7a).
#[derive(Debug, Clone)]
pub enum Consultation {
    /// Proposed and verified — it crossed.
    Crossed(SolveRead),
    /// Proposed nothing.
    Silent,
    /// Proposed a program the door refused.
    Withdrawn,
}

/// Consult the sector and keep WHAT HAPPENED, not merely what crossed.
///
/// Same door, same law, same re-derivation as [`solve_consulted`] —
/// this is the one implementation and that one is expressed through it.
/// Approximation may propose; only exactness may dispose.
pub fn consult_observed(task: &Task, max_depth: usize) -> Consultation {
    let proposal = rfa_core::consult((task, max_depth), |(t, d)| solve(t, d));
    match proposal.withdraw(|read| closes_every_training_pair(task, read)) {
        Some(SolveRead::Unsolved) => Consultation::Silent,
        Some(read) => Consultation::Crossed(read),
        None => Consultation::Withdrawn,
    }
}

/// The exact rule the OUTSIDE applies: re-run the proposed program on
/// every training pair and demand byte-exact agreement. An `Unsolved`
/// proposal is lawful and crosses unchanged — it claims nothing.
fn closes_every_training_pair(task: &Task, read: &SolveRead) -> bool {
    match read {
        SolveRead::Unsolved => true,
        // A claim of Solved over NO training pairs is vacuously true
        // under `all` — the door would pass it on a technicality, and
        // the door is what "only exactness may dispose" rests on. The
        // guard lives here as well as in `solve` deliberately: nothing
        // about having come from the search makes a program true, so
        // the door refuses this shape whoever proposes it.
        SolveRead::Solved { .. } if task.train.is_empty() => false,
        SolveRead::Solved { program, .. } => task
            .train
            .iter()
            .all(|pair| solver_verify::grid::run(program, &pair.input) == pair.output),
    }
}
