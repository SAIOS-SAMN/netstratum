//! U2d — the fold trio enters coverage under THE GAUGE LAW (I6's
//! kernel half, per the operator's Delta-modeling ruling: every fold
//! transition emits its EXPLICIT structural delta — the declared key
//! move — and nothing else; no state jump is special-cased).
//!
//! The kernel half judges the LOCAL transition: the deed's only
//! lawful effect on the gauge vector is exactly what it declares
//! (declaration == effect, the banked law's shape applied to the
//! key), with banks held still by total conservation. The PAIR half
//! of I6 — orbit_lock between the two MERGING chains — is the merge
//! site's law (converge/mesh admission calling the sealed
//! `rfa_core::f_fold_star`), the aperture-pair seam pattern: the
//! kernel enforces the local half; the seam enforces the pair.
//!
//! The strengthening rides along: gauge declaration==effect binds
//! EVERY covered deed — a key move smuggled under a neutral deed now
//! refuses, exactly as a smuggled bank does.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::Entity;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, World, SOVEREIGN};
use kernel::{lawful, Step, Unlawful};

/// A one-orb committed state, sovereign seated, gauge at the base key.
fn seated() -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "assay".into(), ..Default::default() };
    orb.bank(Resource::Iron, ratio(6, 1));
    committed.orbs = vec![orb];
    committed.entities = vec![Entity {
        entity_id: SOVEREIGN,
        handle: "sovereign".into(),
        principal: None,
        grants: vec![],
    }];
    committed
}

fn with_key(state: &Committed, gauge: Vec<Ratio>) -> Committed {
    let mut next = state.clone();
    next.gauge = gauge;
    next
}

fn step(event: ClosureEvent) -> Step {
    Step { event, actor: SOVEREIGN, stamp: 1 }
}

#[test]
fn the_fold_trio_moves_exactly_its_declared_key() {
    let prior = seated();
    // Confluent: the key settles at the declared interference — the
    // deed's ONLY effect.
    let confluent = ClosureEvent::Confluent { partner: "far-line".into(), gauge: ratio(1, 3) };
    let next = with_key(&prior, vec![ratio(1, 3)]);
    assert_eq!(lawful(&prior, &next, &step(confluent)), Ok(()));
    // EraFolded: the new sheet seats at the declared key.
    let folded = ClosureEvent::EraFolded { era: 2, gauge: ratio(1, 6) };
    let next = with_key(&prior, vec![ratio(1, 6)]);
    assert_eq!(lawful(&prior, &next, &step(folded)), Ok(()));
    // EraAnnexed: a universe-ledger record — the world's key holds
    // still, its banks hold still; the pair-half (the annexed chain's
    // verification) is converge's law at the seam.
    let annexed = ClosureEvent::EraAnnexed {
        era: "era1".into(),
        world: "amber".into(),
        terminal: [7u8; 32],
        offset: 4,
    };
    assert_eq!(lawful(&prior, &prior.clone(), &step(annexed)), Ok(()));
}

#[test]
fn a_wrong_key_refuses_with_both_sides_whole() {
    let prior = seated();
    let confluent = ClosureEvent::Confluent { partner: "far-line".into(), gauge: ratio(1, 3) };
    // The state moved to a key the deed did not declare.
    let smuggled = with_key(&prior, vec![ratio(1, 2)]);
    assert_eq!(
        lawful(&prior, &smuggled, &step(confluent)),
        Err(Unlawful::GaugeBroken {
            grade: 0,
            expected: Box::new(ratio(1, 3)),
            actual: Box::new(ratio(1, 2)),
        }),
        "the fold's only lawful effect is its declared key move"
    );
}

#[test]
fn a_key_move_smuggled_under_a_neutral_deed_refuses() {
    // THE STRENGTHENING: gauge declaration==effect binds every covered
    // deed. A neutral deed declares NO key move — the vector must hold
    // still, exactly as the banks must.
    let prior = seated();
    let neutral = ClosureEvent::OverheatCleared { orb_id: 1 };
    let smuggled = with_key(&prior, vec![ratio(1, 4)]);
    assert_eq!(
        lawful(&prior, &smuggled, &step(neutral)),
        Err(Unlawful::GaugeBroken {
            grade: 0,
            expected: Box::new(ratio(0, 1)),
            actual: Box::new(ratio(1, 4)),
        }),
        "no deed moves the key undeclared"
    );
    // And the declared movers are held to their declaration too: a
    // GaugeShifted whose state landed elsewhere refuses.
    let shifted = ClosureEvent::GaugeShifted { gauge: ratio(1, 3) };
    let wrong = with_key(&prior, vec![ratio(2, 3)]);
    assert!(matches!(
        lawful(&prior, &wrong, &step(shifted)),
        Err(Unlawful::GaugeBroken { grade: 0, .. })
    ));
}

#[test]
fn the_graded_shift_resizes_exactly() {
    // GaugeShiftedAt opens a plane: the vector grows to grade+1,
    // zero-filled, the declared key at its grade — exactly the replay
    // law, verified not assumed.
    let prior = seated();
    let shift = ClosureEvent::GaugeShiftedAt { grade: 2, gauge: ratio(1, 4) };
    let next = with_key(&prior, vec![ratio(0, 1), ratio(0, 1), ratio(1, 4)]);
    assert_eq!(lawful(&prior, &next, &step(shift.clone())), Ok(()));
    // An unresized vector (the plane not opened) refuses.
    let flat = with_key(&prior, vec![ratio(1, 4)]);
    assert!(matches!(
        lawful(&prior, &flat, &step(shift)),
        Err(Unlawful::GaugeBroken { .. })
    ));
}
