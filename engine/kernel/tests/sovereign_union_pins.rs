//! D5 — THE SOVEREIGN UNION (operator ruling 2026-08-02).
//!
//! The offices `D(Ω)` PROGENITOR and `V(Ω)` SOVEREIGN entered into
//! union: a bonded DYAD in which neither party derives its legitimacy
//! from the other (`17` §3). The deed is CONSTITUTIONAL — it moves no
//! value — so what these pin is that it cannot be struck falsely, and
//! that its 25-year term REVIEWS rather than EXPIRES.

use cfe::dop::{ClosureEvent, Office, ZoneKind, UNION_REVIEW_HORIZON};
use cfe::ipsal::{Entity, Scope};
use cfe::world::{Committed, World, FIRST_ORB, SOVEREIGN};
use kernel::{admits, Step, Refused};

const DELEGATE: u64 = 100;

fn seat() -> Committed {
    let mut state = World::new().committed();
    state.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "root".into(), principal: None, grants: vec![] },
        Entity {
            entity_id: DELEGATE,
            handle: "hand".into(),
            principal: Some(SOVEREIGN),
            grants: vec![Scope::Build],
        },
    ];
    state
}

fn union(progenitor: Office, sovereign: Office, term: u64, actor: u64) -> Step {
    Step {
        event: ClosureEvent::SovereignUnionSealed {
            progenitor,
            sovereign,
            term_stamps: term,
        },
        actor,
        stamp: 1,
    }
}

#[test]
fn the_root_entity_alone_may_strike_the_union() {
    let state = seat();
    assert_eq!(
        admits(&state, &state, &union(Office::Progenitor, Office::Sovereign, UNION_REVIEW_HORIZON, SOVEREIGN)),
        Ok(()),
        "the root entity constitutes the union"
    );

    // THE DELEGATE HOLDS BUILD AND IS STILL REFUSED. A granted scope is
    // authority to act WITHIN the world, never authority to constitute
    // it — so passing containment is necessary and not sufficient.
    assert_eq!(
        admits(&state, &state, &union(Office::Progenitor, Office::Sovereign, UNION_REVIEW_HORIZON, DELEGATE)),
        Err(Refused::UnionNotStruckByRoot { actor: DELEGATE }),
        "a delegate with Build may not enter the offices into union"
    );
}

#[test]
fn the_union_must_be_a_dyad_of_two_different_offices() {
    // `17` §3: two that STAY TWO. Two progenitors is not a union of the
    // offices, it is one office named twice — and a bond from a thing
    // to itself is not a bond.
    let state = seat();
    for (p, s) in [
        (Office::Progenitor, Office::Progenitor),
        (Office::Sovereign, Office::Sovereign),
        (Office::Sovereign, Office::Progenitor),
    ] {
        assert_eq!(
            admits(&state, &state, &union(p, s, UNION_REVIEW_HORIZON, SOVEREIGN)),
            Err(Refused::UnionMalformed { progenitor: p, sovereign: s }),
            "{p:?} with {s:?} is not the dyad"
        );
    }
}

#[test]
fn the_review_horizon_is_ruled_and_cannot_be_named_by_the_deed() {
    // A deed free to name its own horizon could set one past any
    // reachable stamp and never come up for approval — the review
    // becomes a formality. The term is fixed at the ruled 25 years.
    let state = seat();
    for declared in [0, 1, 1499, 1501, u64::MAX] {
        assert_eq!(
            admits(&state, &state, &union(Office::Progenitor, Office::Sovereign, declared, SOVEREIGN)),
            Err(Refused::UnionTermNotRuled { declared, ruled: UNION_REVIEW_HORIZON }),
            "a horizon of {declared} is not the ruled term"
        );
    }
    assert_eq!(UNION_REVIEW_HORIZON, 1500, "25 years at 60 ticks a year (11 §1)");
}

#[test]
fn the_horizon_reviews_and_never_expires() {
    // THE WHOLE POINT OF THE RULING. A `BOND-1` term SETTLES and ENDS
    // its bond at `sealed + term`. If the union did that, D5's breach
    // clause would fire at stamp 1500 — "D stops and V suffocates; V
    // stops and D freezes" — and the foundation would lapse on a timer.
    //
    // So: past the horizon the review falls DUE and the union STILL
    // STANDS, with nothing settled and no capacity dropped.
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");

    // Before the deed there is no union to review.
    assert_eq!(world.union_review(0), None, "no deed, no union");

    world
        .chronicle_mut()
        .append(
            ClosureEvent::SovereignUnionSealed {
                progenitor: Office::Progenitor,
                sovereign: Office::Sovereign,
                term_stamps: UNION_REVIEW_HORIZON,
            },
            10,
            vec![SOVEREIGN],
        );

    let (sealed_at, due_now) = world.union_review(10).expect("the union stands");
    assert_eq!(sealed_at, 10, "the union records when it was struck");
    assert!(!due_now, "the review is not due the moment it is sealed");

    // One stamp short of the horizon: still not due.
    let (_, due) = world.union_review(10 + UNION_REVIEW_HORIZON - 1).expect("stands");
    assert!(!due, "the horizon has not been reached");

    // AT the horizon and PAST it: the review falls due — and the union
    // is still there to be reviewed, which is the whole distinction.
    for stamp in [10 + UNION_REVIEW_HORIZON, 10 + UNION_REVIEW_HORIZON + 10_000] {
        let (sealed, due) = world
            .union_review(stamp)
            .expect("the union STANDS past its horizon — it reviews, it does not expire");
        assert_eq!(sealed, 10, "and it still knows when it was struck");
        assert!(due, "the operator's re-approval has fallen due");
    }

    // AND NOTHING LAPSED. The world the union stands over is untouched:
    // the colony, its zones and its banks are exactly as they were.
    assert_eq!(world.committed().orbs.len(), 1, "the colony still stands");
    assert!(
        world.orb_by_id(FIRST_ORB).is_some(),
        "no capacity was dropped when the horizon passed"
    );
}
