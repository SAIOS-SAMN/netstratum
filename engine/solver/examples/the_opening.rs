//! **CAN THE OPENING BE MEASURED?** — the experiment behind task #12.
//!
//! Operator: *the door is the wrong shape — measure `derive()` to see if
//! you can measure the opening, otherwise you are blind and have no
//! senses.*
//!
//! Today `derive()` returns a LIST and `closes()` answers a BOOL. That
//! is a door with no senses: a candidate either shuts the task or it
//! does not, and nothing distinguishes one that got within a cell of
//! the answer from one that made things worse. The proposer then TRIES
//! them, which is search — the thing that should not be needed if the
//! layer is deterministic.
//!
//! The ancestry already answers this. `convergence-engine`'s thinking
//! module does not gate, it MEASURES: `AcceptResult::Accepted {
//! coherence_reduction }`, `NonCongruent { conflict_magnitude }`,
//! `NoImprovement { delta_coherence }` — a magnitude, never a verdict.
//! And its nested peel reports `value_cocycle`, `object_cocycle` and
//! `cell_l1` at every level: the residual, per manifold.
//!
//! So the measure taken here is the L1 residual, which is also this
//! tree's own law (PROPORTION-1: `Σ|R|`, no squares, no roots).
//!
//! **What this decides.** If the residual RANKS the reach — if the
//! candidate that reduces it most is the one that closes — then the
//! opening is measurable, derivation can pick rather than try, and the
//! door can report a magnitude. If it does not rank, the reach is
//! genuinely ambiguous and that ambiguity is the finding.
//!
//! Usage: cargo run -p solver --release --example the_opening -- <dir>

use solver::{derive, load_task, Need, Task};
use solver_verify::grid::{run, Grid};

/// THE RESIDUAL — `Σ|R|` over the training pairs, in cells.
///
/// Shape-aware: a program that renders the wrong SHAPE has not got
/// close, so it reads maximally far rather than being compared
/// cell-wise against a grid of another size. Exact integers, no floats.
fn residual(program: &[solver_verify::grid::Op], task: &Task) -> u64 {
    task.train
        .iter()
        .map(|pair| {
            let seen = run(&program.to_vec(), &pair.input);
            distance(&seen, &pair.output)
        })
        .sum()
}

fn distance(a: &Grid, b: &Grid) -> u64 {
    if a.rows != b.rows || a.cols != b.cols {
        return (a.cells.len() + b.cells.len()) as u64;
    }
    a.cells.iter().zip(&b.cells).filter(|(x, y)| x != y).count() as u64
}

fn main() {
    let dir = std::env::args().nth(1).expect("usage: the_opening <corpus_dir>");
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();

    let (mut blind, mut single, mut ambiguous) = (0u32, 0u32, 0u32);
    let (mut ranked_right, mut ranked_wrong, mut ties) = (0u32, 0u32, 0u32);
    let (mut reduces_none_but_closes, mut closed) = (0u32, 0u32);
    let mut examined = 0u32;

    for path in &paths {
        let Ok(task) = load_task(path) else { continue };
        examined += 1;

        let start = residual(&[], &task);
        let reach = derive(&task);

        // What each derived candidate DOES to the residual — the
        // opening, if there is one.
        let mut openings: Vec<(u64, bool)> = reach
            .iter()
            .map(|c| {
                let after = residual(std::slice::from_ref(&c.op), &task);
                (start.saturating_sub(after), after == 0)
            })
            .collect();

        let any_closes = openings.iter().any(|(_, shut)| *shut);
        if any_closes {
            closed += 1;
        }

        let reducing: Vec<&(u64, bool)> =
            openings.iter().filter(|(gain, _)| *gain > 0).collect();

        match reducing.len() {
            0 => {
                blind += 1;
                if any_closes {
                    // A candidate shut the task while reducing NOTHING —
                    // the measure would be blind to the answer.
                    reduces_none_but_closes += 1;
                }
            }
            1 => single += 1,
            _ => ambiguous += 1,
        }

        // DOES THE MEASURE RANK? Take the largest opening; is it a
        // closer, when any closer exists?
        if any_closes && !reducing.is_empty() {
            openings.sort_by(|a, b| b.0.cmp(&a.0));
            let best = openings[0].0;
            let winners: Vec<&(u64, bool)> =
                openings.iter().filter(|(g, _)| *g == best).collect();
            match winners.iter().all(|(_, shut)| *shut) {
                true if winners.len() == 1 => ranked_right += 1,
                true => ties += 1,
                false => ranked_wrong += 1,
            }
        }
    }

    println!("THE OPENING — measured over {examined} tasks in {dir}\n");
    println!("DOES DERIVATION SEE ANYTHING AT ALL?");
    println!("  no candidate reduces the residual   {blind:5}   blind at this step");
    println!("  exactly one reduces                 {single:5}   DETERMINISTIC — one way to go");
    println!("  more than one reduces               {ambiguous:5}   ambiguous — a choice remains");
    println!("\nDOES THE MEASURE RANK THE REACH?");
    println!("  largest opening is the closer, alone {ranked_right:5}");
    println!("  largest opening ties among closers   {ties:5}");
    println!("  largest opening is NOT a closer      {ranked_wrong:5}   the measure misleads here");
    println!("\nCONTROL");
    println!("  tasks a single derived op closes     {closed:5}");
    println!("  closed while reducing NOTHING        {reduces_none_but_closes:5}   invisible to the measure");

    let decisive = single + ambiguous;
    println!("\nVERDICT");
    match (decisive, ranked_wrong) {
        (0, _) => println!("  the residual sees nothing — this measure is not a sense"),
        (_, 0) => println!(
            "  the residual NEVER misleads: where it ranks, the largest opening is a closer.\n  \
             The opening is measurable and derivation can PICK rather than try."
        ),
        _ => println!(
            "  the residual ranks on {decisive} tasks and misleads on {ranked_wrong}.\n  \
             Measurable, but not alone sufficient — picking by it would lose those."
        ),
    }
    let _ = Need::sated();
}
