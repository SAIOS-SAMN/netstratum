//! THE SECTOR DOOR, exercised by its first real caller.
//!
//! `solve` is the dirtiest work in the system — an enumerative sweep
//! with no claim to purity. `solve_consulted` runs it INSIDE the sector
//! and lets nothing out that does not survive exact re-derivation.

use solver::{forge_task, solve, solve_consulted, Difficulty, SolveRead};
use solver_verify::grid::run;

fn task(seed: u8) -> solver_verify::loader::Task {
    forge_task([seed; 32], Difficulty::level(3))
}

#[test]
fn a_consulted_solve_agrees_with_the_bare_search_when_the_search_is_right() {
    // The door is not a filter on correct work: a sound proposal
    // crosses unchanged.
    let t = task(7);
    let bare = solve(&t, Difficulty::level(3).word_len);
    let consulted = solve_consulted(&t, Difficulty::level(3).word_len);
    match (&bare, &consulted) {
        (SolveRead::Solved { program: a, .. }, SolveRead::Solved { program: b, .. }) => {
            assert_eq!(a, b, "a verified proposal crosses unchanged")
        }
        (SolveRead::Unsolved, SolveRead::Unsolved) => {}
        other => panic!("the door changed a sound verdict: {other:?}"),
    }
}

#[test]
fn what_crosses_maps_every_training_pair_exactly() {
    // THE HONEST WITHHOLD LAW at the door. Whatever comes out, if it
    // claims Solved, it has been re-derived against every pair — by the
    // OUTSIDE, never on the search's word.
    for seed in [3u8, 11, 29] {
        let t = task(seed);
        if let SolveRead::Solved { program, .. } = solve_consulted(&t, Difficulty::level(3).word_len)
        {
            for (n, pair) in t.train.iter().enumerate() {
                assert_eq!(
                    run(&program, &pair.input),
                    pair.output,
                    "pair {n} does not close — this should never have left the sector"
                );
            }
        }
    }
}

#[test]
fn an_unsolved_proposal_crosses_and_claims_nothing() {
    // Unsolved is lawful and costs nothing: it asserts no program, so
    // there is nothing to re-derive and nothing to refuse.
    let t = task(5);
    // Depth 0 cannot close a rung-3 task; the search honestly says so.
    assert!(matches!(solve_consulted(&t, 0), SolveRead::Unsolved));
}

/// THE THREE OUTCOMES — `solve_consulted` collapses Withdrawn and Silent
/// into one `Unsolved` for its callers, and `consult_observed` keeps the
/// difference. It is the only tell for a degrading search: a proposal
/// that FAILED re-derivation and a search that found nothing look
/// identical once collapsed.
#[test]
fn the_door_keeps_what_happened_not_only_what_crossed() {
    use solver::{consult_observed, Consultation};

    let t = forge_task([31u8; 32], Difficulty::level(3));

    // A real consultation lands in exactly one arm.
    let seen = consult_observed(&t, Difficulty::level(3).word_len);
    match &seen {
        Consultation::Crossed(SolveRead::Solved { program, .. }) => {
            // If it crossed, it MUST close every training pair — the
            // door's whole law.
            assert!(t
                .train
                .iter()
                .all(|p| solver_verify::grid::run(program, &p.input) == p.output));
        }
        Consultation::Crossed(SolveRead::Unsolved) => {
            panic!("Crossed must never carry an Unsolved — that is Silent");
        }
        Consultation::Silent | Consultation::Withdrawn => {}
    }

    // And the collapsed form agrees with the observed one, so the two
    // can never drift: one implementation, two readings of it.
    let collapsed = solve_consulted(&t, Difficulty::level(3).word_len);
    match (&seen, &collapsed) {
        (Consultation::Crossed(SolveRead::Solved { .. }), SolveRead::Solved { .. }) => {}
        (Consultation::Silent, SolveRead::Unsolved) => {}
        (Consultation::Withdrawn, SolveRead::Unsolved) => {}
        other => panic!("observed and collapsed disagree: {other:?}"),
    }

    // A zero-depth search proposes nothing: SILENT, never Withdrawn.
    // Nothing was refused, because nothing was offered.
    assert!(matches!(consult_observed(&t, 0), Consultation::Silent));
}
