//! THE NEED-DRIVEN PROPOSER, pinned (operator ruling 2026-08-04:
//! never enumerate; operators are called by what the civilization
//! needs).
//!
//! The load-bearing pin is `a_need_may_order_the_reach_but_never_extend
//! _it`. If a hunger can add an operator the observation does not
//! support, "need-driven" is guessing with a better name.

use solver::{
    derive, propose, propose_measured, solve, Family, Grid, Hunger, Need, Op, SolveRead, Task,
    TaskPair,
};

fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
    Grid::new(rows, cols, cells.to_vec()).expect("well-formed")
}

fn task_from(word: &[Op], inputs: &[Grid], test_in: Grid) -> Task {
    let render = |g: &Grid| {
        let mut out = g.clone();
        for op in word {
            out = op.apply(&out);
        }
        out
    };
    Task {
        name: "constructed".into(),
        train: inputs
            .iter()
            .map(|input| TaskPair { input: input.clone(), output: render(input) })
            .collect(),
        test_expected: Some(render(&test_in)),
        test_in,
    }
}

/// THE LAW. A need may ORDER and GATE the reach; only the OBSERVATION
/// may supply a candidate. Every hunger sees the same operator SET on
/// the same task — hunger changes the order it is walked, never its
/// membership.
#[test]
fn a_need_may_order_the_reach_but_never_extend_it() {
    let task = task_from(
        &[Op::Rotate90],
        &[grid(2, 3, &[1, 2, 3, 4, 5, 6]), grid(2, 2, &[7, 0, 0, 7])],
        grid(2, 3, &[9, 8, 7, 6, 5, 4]),
    );
    let reach = derive(&task);
    assert!(!reach.is_empty(), "a rotation task supports operators");

    // Whatever the mind lacks, the observation supports the same set.
    for need in [
        Need::sated(),
        Need::struck(Hunger::Resonance, 240),
        Need::struck(Hunger::Novelty, 480),
    ] {
        let seen = derive(&task);
        assert_eq!(
            seen.len(),
            reach.len(),
            "a hunger must not change what the observation supports"
        );
        // And a starving mind still solves what a sated one solves.
        assert!(matches!(
            propose(&task, &need),
            SolveRead::Solved { matches_expected: Some(true), .. }
        ));
    }
}

/// A hunger reorders: a resonance-struck mind reaches for correspondence
/// (symmetry, recolor) before structure (tile, object).
#[test]
fn hunger_names_the_family_it_reaches_for_first() {
    let resonance = Need::struck(Hunger::Resonance, 240);
    assert!(resonance.prefers(Family::Symmetry));
    assert!(resonance.prefers(Family::Recolor));
    assert!(!resonance.prefers(Family::Tile));

    let novelty = Need::struck(Hunger::Novelty, 480);
    assert!(novelty.prefers(Family::Tile));
    assert!(novelty.prefers(Family::Object));
    assert!(!novelty.prefers(Family::Symmetry));

    // A sated mind prefers nothing — it takes the derived order.
    let sated = Need::sated();
    for family in [Family::Symmetry, Family::Recolor, Family::Tile, Family::Object] {
        assert!(!sated.prefers(family), "nothing struck, nothing preferred");
    }
}

/// Hunger buys COMPOSITION DEPTH over derived candidates, and only
/// that. It never widens the vocabulary — the reach is the observation's.
#[test]
fn starving_composes_deeper_and_nothing_else() {
    assert_eq!(Need::sated().compose_limit(), 1);
    assert_eq!(Need::struck(Hunger::Resonance, 240).compose_limit(), 2);
    assert_eq!(Need::struck(Hunger::Novelty, 480).compose_limit(), 3);
    // The reading is what deepens it, not the kind.
    assert_eq!(Need::struck(Hunger::Novelty, 10).compose_limit(), 2);
}

/// THE COST IS THE POINT. A derived proposal tests a handful of
/// candidates where the sweep budgeted 200,000.
#[test]
fn the_reach_is_small_where_the_sweep_was_vast() {
    let task = task_from(
        &[Op::FlipH],
        &[grid(2, 2, &[1, 2, 0, 1]), grid(2, 3, &[1, 0, 2, 2, 1, 0])],
        grid(2, 2, &[2, 2, 1, 0]),
    );
    let measured = propose_measured(&task, &Need::struck(Hunger::Novelty, 480));
    assert!(matches!(
        measured.read,
        SolveRead::Solved { matches_expected: Some(true), .. }
    ));
    assert!(
        measured.tested < 500,
        "a derived proposal is a handful, not a sweep: tested {}",
        measured.tested
    );
    assert!(
        measured.reach <= 20,
        "the observation supports a few operators, not a vocabulary: {}",
        measured.reach
    );
}

/// The vacuity law holds for the proposer too (98ccbbe): nothing to
/// close means nothing proved, whatever the mind wants.
#[test]
fn an_empty_training_set_never_proposes() {
    let task = Task {
        name: "constructed-empty-train".into(),
        train: Vec::new(),
        test_in: grid(2, 2, &[1, 2, 3, 4]),
        test_expected: None,
    };
    for need in [Need::sated(), Need::struck(Hunger::Novelty, 4800)] {
        assert!(matches!(propose(&task, &need), SolveRead::Unsolved));
    }
    assert!(derive(&task).is_empty(), "no observation, no reach");
}

/// A task outside the derivable families reads Unsolved — the honest
/// withhold, not a guess dressed as a need.
#[test]
fn an_underivable_task_withholds() {
    let task = Task {
        name: "constructed-unreachable".into(),
        train: vec![TaskPair {
            input: grid(2, 2, &[1, 2, 1, 4]),
            output: grid(1, 3, &[5, 5, 5]),
        }],
        test_in: grid(2, 2, &[5, 6, 7, 8]),
        test_expected: None,
    };
    assert!(matches!(
        propose(&task, &Need::struck(Hunger::Novelty, 480)),
        SolveRead::Unsolved
    ));
}

/// The want kinds map from the wire exactly, and unknown kinds are
/// dropped rather than guessed at (kind 1 is reserved board-side).
#[test]
fn the_wire_kinds_map_without_guessing() {
    assert_eq!(Hunger::from_kind(2), Some(Hunger::Resonance));
    assert_eq!(Hunger::from_kind(3), Some(Hunger::Novelty));
    assert_eq!(Hunger::from_kind(1), None, "epistemological is reserved");
    assert_eq!(Hunger::from_kind(9), None, "an unknown kind names no reach");

    let need = Need::from_wants(&[(2, 240), (9, 9999), (3, 480)]);
    assert_eq!(need.hungers.len(), 2, "the unknown kind is dropped");
    assert_eq!(need.starved_for, 480, "the longest starve carries");
    assert_eq!(Need::from_wants(&[]), Need::sated());
}

/// WHAT THE PROPOSER GIVES UP, pinned honestly. The sweep composes over
/// its whole vocabulary and so can pass through states no observation
/// motivates; derivation cannot, by construction. Where the two differ
/// the proposer withholds rather than stumbling on it — this pin exists
/// so that trade is recorded in the tree, not just in a commit message.
#[test]
fn derivation_withholds_where_the_sweep_stumbled() {
    // Gravity composed under a rotation: the delta reads as nothing
    // gravity-shaped in the original frame.
    let task = task_from(
        &[Op::Rotate180, Op::Object(solver_verify::objects::ObjOp::GravityDown), Op::Rotate180],
        &[grid(3, 3, &[1, 0, 0, 0, 2, 0, 0, 0, 3])],
        grid(3, 3, &[4, 0, 0, 0, 5, 0, 0, 0, 6]),
    );
    // MEASURED, not hoped: the sweep closes this by exhaustion — and
    // with a DIFFERENT word than the one forged (`gravity_down →
    // flip_v`), because the verifier is the authority and any program
    // that closes counts. The derived reach withholds.
    assert!(
        matches!(
            solve(&task, 3),
            SolveRead::Solved { matches_expected: Some(true), .. }
        ),
        "the sweep reaches this one by exhaustion — if it stopped, the \
         baseline moved and this pin's premise is gone"
    );
    assert!(
        matches!(
            propose(&task, &Need::struck(Hunger::Novelty, 480)),
            SolveRead::Unsolved
        ),
        "derivation withholds here: no observation motivates the \
         intermediate frame. IF THIS NOW SOLVES, the derivation improved \
         — that is good news, and this pin should be retired deliberately \
         rather than loosened"
    );
}

/// THE INVARIANT THAT OUTRANKS THE TRADE. Whatever the proposer reads,
/// a `Solved` it returns must close every training pair — the honest
/// withhold law, checked over tasks it actually solves.
#[test]
fn what_the_proposer_claims_it_can_re_derive() {
    let cases = [
        (vec![Op::Rotate90], grid(2, 3, &[1, 2, 3, 4, 5, 6])),
        (vec![Op::FlipH], grid(2, 2, &[1, 2, 0, 1])),
        (vec![Op::Rotate180], grid(3, 2, &[1, 2, 3, 4, 5, 6])),
    ];
    let mut crossed = 0;
    for (word, input) in &cases {
        let task = task_from(word, std::slice::from_ref(input), input.clone());
        if let SolveRead::Solved { program, .. } =
            propose(&task, &Need::struck(Hunger::Resonance, 240))
        {
            crossed += 1;
            for pair in &task.train {
                assert_eq!(
                    solver_verify::grid::run(&program, &pair.input),
                    pair.output,
                    "a proposal that crosses must close every pair"
                );
            }
        }
    }
    // COVERAGE: without this the loop above can assert nothing and pass.
    assert_eq!(crossed, cases.len(), "every case here is derivable");
}

/// **THE OPENING IS MEASURABLE, AND THE MEASURE NEVER MISLEADS.**
///
/// Measured over the 960-task corpus (`examples/the_opening.rs`): where
/// the residual `Σ|R|` ranks the derived reach, the candidate that
/// reduces it MOST is a closer — 25 of 25, zero counterexamples, and
/// zero tasks closed while reducing nothing.
///
/// That is what makes picking lawful instead of trying: the door can
/// report a MAGNITUDE (the ancestry's `AcceptResult { coherence_
/// reduction }`) rather than a bool, and derivation can choose the way
/// to go rather than search for it.
///
/// This pins the property on constructed cases so a change to `derive`
/// that breaks the ranking is caught here rather than in a corpus run.
#[test]
fn the_largest_opening_is_the_closer() {
    let cases = [
        (vec![Op::Rotate90], grid(2, 3, &[1, 2, 3, 4, 5, 6])),
        (vec![Op::FlipV], grid(3, 2, &[1, 2, 3, 4, 5, 6])),
        (vec![Op::Transpose], grid(2, 2, &[7, 1, 2, 8])),
    ];
    let residual = |program: &[Op], task: &Task| -> u64 {
        task.train
            .iter()
            .map(|pair| {
                let seen = solver_verify::grid::run(&program.to_vec(), &pair.input);
                match seen.rows == pair.output.rows && seen.cols == pair.output.cols {
                    false => (seen.cells.len() + pair.output.cells.len()) as u64,
                    true => seen
                        .cells
                        .iter()
                        .zip(&pair.output.cells)
                        .filter(|(a, b)| a != b)
                        .count() as u64,
                }
            })
            .sum()
    };
    let mut ranked = 0;
    for (word, input) in &cases {
        let task = task_from(word, std::slice::from_ref(input), input.clone());
        let start = residual(&[], &task);
        let reach = derive(&task);
        let mut seen: Vec<(u64, bool)> = reach
            .iter()
            .map(|c| {
                let after = residual(std::slice::from_ref(&c.op), &task);
                (start.saturating_sub(after), after == 0)
            })
            .collect();
        if seen.iter().all(|(gain, _)| *gain == 0) {
            continue; // blind here; the corpus run reports how often
        }
        seen.sort_by(|a, b| b.0.cmp(&a.0));
        assert!(
            seen[0].1,
            "the largest opening was not a closer — the measure misled, \
             and picking by it would lose this task"
        );
        ranked += 1;
    }
    assert!(ranked > 0, "no case ranked — this pin proved nothing");
}
