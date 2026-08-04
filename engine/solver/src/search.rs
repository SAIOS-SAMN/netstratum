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

/// **THE DERIVATION LIMIT, and it is why ~30 % of forged tasks are
/// unreachable** (measured 2026-08-03, 35 of 120).
///
/// Both derivations below compute `seen = prefix(input)` and compare it
/// against `output` directly. That derives an operator ONLY when it is
/// the LAST op of the word, with at most one geometric op before it.
///
/// `forge_task` composes freely, so at `word_len` 3 a word like
/// `[Recolor, Rotate90, Tile]` or `[Tile, Recolor, FlipH]` leaves its
/// specific hue permutation or tiling outside the vocabulary entirely —
/// and no search depth reaches an operator that is not there. This is a
/// REACHABILITY limit, not a search-budget one; every optimisation to
/// the sweep leaves it exactly where it is.
///
/// The cure is to invert the trailing ops before deriving: for each
/// short candidate suffix, apply its inverse to the output and run the
/// derivation against that. Bounded work, and it extends what is here
/// rather than replacing it.
///
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

/// Derive candidate tilings from the shape ratio — structure read off
/// the observation, parameters searched generally.
///
/// **Reads EVERY pair, and under the same seven geometric prefixes
/// `derived_recolors` uses.** Both fixes close the same reachability
/// gap: the forge composes its secret word freely, so a `Tile` may sit
/// AFTER a shape-changing op. A 4×5 input rotated to 5×4 and then tiled
/// 3×2 gives a 15×8 output, and 15/4 is not an integer — so a ratio
/// taken from the raw input dimensions of the first pair alone finds
/// nothing, the tile operator never enters the vocabulary, and no depth
/// of search can reach it.
///
/// A ratio must hold on EVERY pair to be admitted; one pair agreeing is
/// a coincidence of that sample.
fn derived_tiles(task: &Task) -> Vec<Op> {
    let prefixes: [Option<Op>; 7] = [
        None,
        Some(Op::Rotate90),
        Some(Op::Rotate180),
        Some(Op::Rotate270),
        Some(Op::FlipH),
        Some(Op::FlipV),
        Some(Op::Transpose),
    ];
    let mut ops: Vec<Op> = Vec::new();
    for prefix in &prefixes {
        // The ratio this prefix implies, required to agree on every pair.
        let mut ratio: Option<(usize, usize)> = None;
        let mut holds = true;
        for pair in &task.train {
            let seen = match prefix {
                None => pair.input.clone(),
                Some(op) => op.apply(&pair.input),
            };
            let (ir, ic) = (seen.rows, seen.cols);
            let (or, oc) = (pair.output.rows, pair.output.cols);
            if ir == 0 || ic == 0 || or % ir != 0 || oc % ic != 0 {
                holds = false;
                break;
            }
            let here = (or / ir, oc / ic);
            match ratio {
                None => ratio = Some(here),
                Some(held) if held == here => {}
                Some(_) => {
                    holds = false;
                    break;
                }
            }
        }
        let Some((down, across)) = ratio.filter(|_| holds) else { continue };
        if down * across <= 1 || down > 6 || across > 6 {
            continue;
        }
        for odd_row in solver_verify::grid::TileMirror::ALL {
            for odd_col in solver_verify::grid::TileMirror::ALL {
                let op = Op::Tile { down, across, odd_row, odd_col };
                if !ops.contains(&op) {
                    ops.push(op);
                }
            }
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

/// The same reading, run through ONE reused buffer.
///
/// `run` allocates a fresh grid per op; at 3×3 and 4×4 the allocation
/// costs more than the transformation, and the sweep does this tens of
/// thousands of times per beat. Same arithmetic, same answer — the
/// buffer is the only difference, and `closes` above stays as the
/// allocating reference the witnesses read.
fn closes_reusing(
    program: &Program,
    task: &Task,
    carry: &mut Grid,
    scratch: &mut Grid,
) -> bool {
    for pair in &task.train {
        carry.clone_from(&pair.input);
        for op in program {
            op.apply_into(carry, scratch);
            std::mem::swap(carry, scratch);
        }
        if *carry != pair.output {
            return false;
        }
    }
    true
}

/// **THE FRONTIER** — `Σ_{k=0}^{d} V^k`, the exhaustive sweep to
/// depth `d` over a vocabulary of `V` operators.
///
/// **This replaces the flat `SEARCH_BUDGET = 200_000`** (operator
/// ruling 2026-08-02: *"do not cap rungs and do not set a static
/// SEARCH_BUDGET — uncap or no cap"*). That literal was never an
/// administrative ceiling: `frontier(37, 3) = 52_060` is the exact
/// exhaustive expansion of the relational substrate at depth 3, and the
/// constant was a number chosen to sit above it after one observed
/// 102 s stall. Caging `V` behind a constant choked the expansion it
/// was meant to describe.
///
/// **DEPTH IS THE ONLY LEVER NOW.** The bound is `max_depth`, supplied
/// by the caller; the candidate count is its consequence, not a second
/// gate. Because this returns the EXACT number of candidates the sweep
/// will test, a budget check against it can never fire — which is why
/// the truncation branch is gone rather than merely raised.
///
/// Determinism is unchanged and is what mattered: same task, same
/// depth, same outcome on every machine. What is gone is the arbitrary
/// number, not the law (`EQUATIONS` §7 B7 — `β_cap` is DERIVED).
///
/// `u128` and saturating throughout: the frontier grows geometrically
/// and an accounting read must never wrap.
pub fn frontier(vocabulary: usize, depth: usize) -> u128 {
    (0..=depth)
        .map(|k| (vocabulary as u128).saturating_pow(k as u32))
        .fold(0u128, |sum, term| sum.saturating_add(term))
}

/// THE SWEEP — SUPERSEDED (operator ruling 2026-08-04: *never
/// enumerate*). Kept as the measured baseline, not as the intended
/// path; [`propose`] is the ruled proposer.
///
/// Enumerative search over programs up to `max_depth` ops drawn from
/// the general vocabulary plus operators derived from the observation.
/// **Exhaustive to `max_depth`, uncapped** — see [`frontier`].
///
/// **Why it was struck.** It sweeps a fixed vocabulary identically for
/// every mind in every era, reading nothing about what the civilization
/// lacks — the definition of brute force. Measured on the 960-task
/// corpus it also saturates: confirmed stops at 33 from depth 3, and
/// depth 4 buys ONLY overfitting (37 closed, still 33 confirmed — four
/// false closures where there had been one). Its marginal product is
/// negative past depth 3.
///
/// **Those measurements were taken UNDER the old `SEARCH_BUDGET`**,
/// which the remediations lane has since deleted by operator ruling
/// (*"uncap or no cap"*) in favour of [`frontier`]. The saturation
/// finding is unaffected — it was reached at depths 1–4, far inside any
/// cap — but the numbers should be re-read once uncapped before anyone
/// quotes them as a ceiling rather than a plateau.
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

    // ONE SCRATCH BUFFER for the whole sweep. A grid here is 9 to 16
    // bytes, so a fresh Vec per candidate is mostly allocator
    // bookkeeping; `apply_into` refills this and keeps the capacity.
    let mut scratch = Grid { rows: 0, cols: 0, cells: Vec::new() };
    let mut carry = Grid { rows: 0, cols: 0, cells: Vec::new() };

    // Depth 0: the identity word (a copy task is a lawful task).
    let mut frontier: Vec<Program> = vec![Vec::new()];
    for depth in 0..=max_depth {
        for program in &frontier {
            if closes_reusing(program, task, &mut carry, &mut scratch) {
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
        // THE LAST DEPTH GROWS NOTHING. Building the depth-(d+1)
        // frontier only to discard it cost V^(d+1) clones every
        // call — at V=37, max_depth=3 that is 1.87M programs
        // allocated and thrown away. The old `room` clamp hid this by
        // truncating; with the clamp gone it must be said outright.
        if depth == max_depth {
            break;
        }
        let mut next = Vec::with_capacity(frontier.len() * vocabulary.len());
        for program in &frontier {
            for op in &vocabulary {
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

/// THE PROPOSER, THROUGH THE SECTOR DOOR — what the live board calls.
///
/// The same law [`solve_consulted`] runs under: approximation may
/// PROPOSE, only exactness may DISPOSE. A proposal is withdrawn unless
/// the program it names maps EVERY training pair exactly, re-derived by
/// the OUTSIDE rather than taken on the search's word.
///
/// The need shapes what is reached for; it never shapes what is
/// accepted. A starving mind may look in a different order and compose
/// deeper, and still nothing crosses that does not close — which is why
/// letting hunger steer the search costs the judge nothing.
pub fn propose_consulted(task: &Task, need: &Need) -> SolveRead {
    propose_consulted_measured(task, need).read
}

/// THE DOOR AND THE COST IN ONE PASS.
///
/// A caller that wants both must not run the proposer twice — the
/// integration did exactly that for one commit, measuring with
/// [`propose_measured`] and then disposing with [`propose_consulted`],
/// which doubled the live board's search for no change in verdict.
///
/// **The cost is a MEASUREMENT; only the CLAIM goes through the door.**
/// Candidates tested is a fact about work already done, true whether or
/// not anything crossed — offering it to the door would be asking
/// exactness to verify an expenditure. What the door judges is the
/// read, exactly as [`propose_consulted`] judges it.
pub fn propose_consulted_measured(task: &Task, need: &Need) -> Proposal {
    let measured = propose_measured(task, need);
    let (tested, reach) = (measured.tested, measured.reach);
    let read = rfa_core::Proposal::offer(measured.read)
        .withdraw(|read| closes_every_training_pair(task, read))
        .unwrap_or(SolveRead::Unsolved);
    Proposal { read, tested, reach }
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
