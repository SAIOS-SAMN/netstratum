//! THE ELEMENT LAW — the element ledger enters conservation with
//! strict symmetry to the banked ledger (operator ruling 2026-08-01).
//! Red-first.
//!
//! An assay mints `richness · elapsed` into a vein's element — sign-
//! guarded (amount ≥ 0) and bounded by the PRIOR vein's richness and
//! last_assay, both already committed state, so the bound is
//! deterministic from (prior, step). Every other deed holds every
//! element key still: an element total changed under a deed that
//! declares no element move refuses, exactly as a smuggled bank does.

use cfe::dop::{ClosureEvent, ColonyOrb};
use cfe::ipsal::Entity;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, Prospected, World, SOVEREIGN};
use kernel::{admits, Step, Refused};

const ELEMENT: [u8; 32] = [3u8; 32];

/// A one-orb state with an opened vein: element ELEMENT, richness 2,
/// last assayed at stamp 1. Sovereign seated.
fn with_vein(richness: (i64, i64), last_assay: u64) -> Committed {
    let mut committed = World::new().committed();
    committed.orbs = vec![ColonyOrb { orb_id: 1, name: "site".into(), ..Default::default() }];
    committed.entities = vec![Entity {
        entity_id: SOVEREIGN,
        handle: "sovereign".into(),
        principal: None,
        grants: vec![],
    }];
    committed.prospects = vec![Prospected {
        orb: 1,
        stratum: 0,
        element: ELEMENT,
        richness: ratio(richness.0, richness.1),
        last_assay,
    }];
    committed
}

fn banked_element(state: &Committed, element: [u8; 32], delta: Ratio) -> Committed {
    let mut next = state.clone();
    next.orbs[0].bank_element(element, delta);
    next
}

fn assay(amount: (i64, i64), stamp: u64) -> Step {
    Step {
        event: ClosureEvent::AssayStruck { orb_id: 1, element: ELEMENT, amount: ratio(amount.0, amount.1) },
        actor: SOVEREIGN,
        stamp,
    }
}

#[test]
fn an_honest_assay_mints_exactly_the_vein_yield() {
    // richness 2 × elapsed (4 − 1 = 3) = 6, minted into the element.
    let prior = with_vein((2, 1), 1);
    let next = banked_element(&prior, ELEMENT, ratio(6, 1));
    assert_eq!(admits(&prior, &next, &assay((6, 1), 4)), Ok(()));
}

#[test]
fn an_assay_past_the_vein_yield_refuses_bounded() {
    // The same vein yields 6 at stamp 4; a claim of 7 refuses with the
    // bound and the claim carried whole.
    let prior = with_vein((2, 1), 1);
    let greedy = banked_element(&prior, ELEMENT, ratio(7, 1));
    assert_eq!(
        admits(&prior, &greedy, &assay((7, 1), 4)),
        Err(Refused::AssayExceedsVein {
            bound: Box::new(ratio(6, 1)),
            claimed: Box::new(ratio(7, 1)),
        }),
        "value enters the element ledger only up to the vein's yield"
    );
}

#[test]
fn a_negative_assay_refuses_by_sign() {
    let prior = with_vein((2, 1), 1);
    let inverted = banked_element(&prior, ELEMENT, ratio(-6, 1));
    assert_eq!(
        admits(&prior, &inverted, &assay((-6, 1), 4)),
        Err(Refused::ElementNotPositive {
            element: ELEMENT,
            amount: Box::new(ratio(-6, 1)),
        }),
        "an element mint is non-negative by declaration"
    );
}

#[test]
fn an_assay_with_no_vein_yields_nothing() {
    // No prior vein for this element: the bound is zero, so any
    // positive claim refuses.
    let mut prior = with_vein((2, 1), 1);
    prior.prospects.clear();
    let claimed = banked_element(&prior, ELEMENT, ratio(6, 1));
    assert_eq!(
        admits(&prior, &claimed, &assay((6, 1), 4)),
        Err(Refused::AssayExceedsVein {
            bound: Box::new(ratio(0, 1)),
            claimed: Box::new(ratio(6, 1)),
        }),
        "no vein, no yield"
    );
}

#[test]
fn an_element_change_under_a_neutral_deed_refuses() {
    // Zero-invariance: a covered deed that declares no element move
    // must leave every element key still. A smuggled element mint
    // under a neutral deed refuses, exactly as a smuggled bank does.
    let prior = with_vein((2, 1), 1);
    let smuggled = banked_element(&prior, ELEMENT, ratio(6, 1));
    let neutral = Step {
        event: ClosureEvent::OverheatCleared { orb_id: 1 },
        actor: 999,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &smuggled, &neutral),
        Err(Refused::ElementBroken {
            element: ELEMENT,
            expected: Box::new(ratio(0, 1)),
            actual: Box::new(ratio(6, 1)),
        }),
        "no deed mints an element undeclared"
    );
}
