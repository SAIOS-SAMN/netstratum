//! **Think** (N-I3, doc `53` D1/D2) — the seat conductor. It owns no algebra: it
//! runs the two moves that exist (derive via `reduce_pair`'s gauge deduction,
//! reduce via `drain_exact`), glues the per-pair closures across observations,
//! and mints the leftover a vessel could not account. There is no decline
//! (conjugate) and no compose (sentence) here — the four-move refusal order with
//! two organs is honest; a fabricated decline would be a rendered lie.
//!
//! Gluing is the whole point: a rule read from one pair is a coincidence; a rule
//! every observation shares is understood. A single wall (GenerationGap) is
//! learned (minted + adopted) and the pass is WITHHELD — the next think of the
//! same class drains (Glued ByDrain, Pin B at task scope). Fewer than two
//! observations cannot tell a rule from a coincidence.

use crate::mint::mint;
use crate::observation::adopt;
use crate::peel::FullMeasurement;
use crate::reduction::{
    identity_frame, reduce_pair, Crystal, Dir13Route, ObserverFrame, ReduceVerdict, Remainder,
};

/// How a single pair closed, when it closes (a gluable outcome).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairClosure {
    /// The pair reduced through a deduced frame (identity, or a gauge) — the rule
    /// the glue compares across observations.
    Frame(ObserverFrame),
    /// The pair was accounted by the grown basis — no frame expressed, but a
    /// close (Pin B). Every pair closing this way glues ByDrain.
    ByDrain,
}

/// The task-scope verdict — the only vocabulary a seat renders from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskThought {
    /// Every observation closed the same way — the task is understood. Carries
    /// the shared closure (a frame all pairs express, or ByDrain).
    Glued(PairClosure),
    /// The observations closed differently, or a composition/frame remainder
    /// survived — no single rule generalizes. Residuals carried whole.
    ///
    /// Conjectures (Concordance 1.10, the audit's restore 1): every frame that
    /// closed some of the pairs, with how many it closed — a mind's own
    /// unglued hypotheses, kept rather than discarded, in first-seen order.
    /// A conjecture is private thought: it is never a claim until it closes
    /// every pair of a task.
    UnglueableAcrossObservations { residuals: Vec<Remainder>, conjectures: Vec<(ObserverFrame, usize)> },
    /// A GenerationGap wall was minted and adopted this pass — the task learned
    /// but did not close; the next think of the same class drains.
    Withheld,
    /// Fewer than two observations — a rule cannot be told from a coincidence.
    InsufficientObservations { count: usize },
}

/// Think a task's observations against a vessel's grown basis. Derive + reduce +
/// glue; a wall is minted and adopted (never pushed directly); express-or-withhold.
pub fn think(
    observations: &[(FullMeasurement, FullMeasurement)],
    grown: &mut Vec<Crystal>,
) -> TaskThought {
    if observations.len() < 2 {
        return TaskThought::InsufficientObservations { count: observations.len() };
    }
    // Judge every pair against the same pre-think basis, so two same-class walls
    // both wall (and mint one crystal) this pass rather than the second draining
    // off the first's fresh adopt — the pass either learns (Withheld) or closes.
    let basis = grown.clone();
    let mut closures: Vec<PairClosure> = Vec::new();
    let mut walls: Vec<Crystal> = Vec::new();
    let mut residuals: Vec<Remainder> = Vec::new();
    for (a, b) in observations {
        match reduce_pair(a, b, &basis) {
            ReduceVerdict::Congruent { bounding } => {
                if bounding.is_empty() {
                    closures.push(PairClosure::Frame(identity_frame()));
                } else {
                    closures.push(PairClosure::ByDrain);
                }
            }
            ReduceVerdict::CongruentThroughP { p } => closures.push(PairClosure::Frame(p)),
            ReduceVerdict::Remainder {
                remainder: Remainder::ChainClass { chain },
                route: Dir13Route::GenerationGap,
            } => {
                if let Ok(crystal) = mint(chain) {
                    walls.push(crystal);
                }
            }
            ReduceVerdict::Remainder { remainder, .. } => residuals.push(remainder),
        }
    }
    // A wall means the task learned — adopt (idempotent) and withhold this pass.
    if !walls.is_empty() {
        for crystal in walls {
            adopt(grown, crystal);
        }
        return TaskThought::Withheld;
    }
    if !residuals.is_empty() {
        let conjectures = conjectures(&closures);
        return TaskThought::UnglueableAcrossObservations { residuals, conjectures };
    }
    glue(closures)
}

/// Every frame that closed at least one pair, with its support, first-seen
/// order — the hypotheses an unglued thought leaves behind.
fn conjectures(closures: &[PairClosure]) -> Vec<(ObserverFrame, usize)> {
    let mut held: Vec<(ObserverFrame, usize)> = Vec::new();
    for closure in closures {
        if let PairClosure::Frame(frame) = closure {
            match held.iter_mut().find(|(f, _)| f == frame) {
                Some((_, n)) => *n += 1,
                None => held.push((frame.clone(), 1)),
            }
        }
    }
    held
}

/// Glue across observations: every pair must have closed the same way — the same
/// frame rule, or all ByDrain. Any disagreement is Unglueable (the rule read
/// from one pair did not generalize).
fn glue(closures: Vec<PairClosure>) -> TaskThought {
    match closures.first() {
        None => TaskThought::UnglueableAcrossObservations { residuals: Vec::new(), conjectures: Vec::new() },
        Some(first) => {
            if closures.iter().all(|c| c == first) {
                TaskThought::Glued(first.clone())
            } else {
                TaskThought::UnglueableAcrossObservations { residuals: Vec::new(), conjectures: conjectures(&closures) }
            }
        }
    }
}
