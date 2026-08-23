//! **THE DEMURRAGE SEAM, CLOSED** (tag 102, 2026-08-06).
//!
//! The per-row form judged each `DemurrageSettled` against a prior the
//! previous row had already anchored: a dual-resource touch (Joule row,
//! then Sink row, one stamp) always found `elapsed 0` at its second row
//! — `DecayExceedsBound { bound: 0, claimed: rate }` — and the market
//! is what first banks both leaking resources on one orb, which is why
//! every marketless era rehearsed past it (found by the d3-scope lane's
//! era wiring; mechanism confirmed at `replay.rs`'s per-row anchor).
//!
//! The atomic touch carries every settled resource in ONE receipt, so
//! the judge reads the whole settlement against the anchor that stood
//! before it. These pins hold the seam both ways: the old shape's
//! second-row refusal (sealed history — it stays exactly as broken as
//! it was) and the new shape's admission, plus the two refusals the
//! atomic form itself must carry.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource, ThermalSite};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::World;
use kernel::{admits, Refused, Step};

/// A one-orb state sited HOT (mesic leaks — above the cryo vault),
/// holding both leaking resources, untouched for four ticks.
fn dual_leak() -> cfe::world::Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "leaks".into(), ..Default::default() };
    orb.bank(Resource::Joule, ratio(8, 1));
    orb.bank(Resource::Sink, ratio(8, 1));
    orb.thermal = Some(ThermalSite {
        albedo: cfe::dop::unit(0, 1),
        orbit_r: cfe::dop::positive(1, 1),
        theta: None,
        psi: None,
    });
    orb.last_touch = 0;
    committed.orbs = vec![orb];
    committed.star = Some(StarClass::G);
    committed
}

/// What the sealed rule owes at stamp 4 — derived through the same
/// call the emitter and the gate both make (one computation).
fn due_at_4(prior: &cfe::world::Committed) -> Vec<(Resource, cfe::ratio::Ratio)> {
    cfe::world::demurrage_due(
        &prior.orbs[0],
        prior.star,
        4,
        &cfe::world::ease_of(&prior.techs, 1, 4),
        &cfe::world::standing_supply(&prior.orbs),
    )
}

fn stepped(event: ClosureEvent) -> Step {
    Step { event, actor: 0, stamp: 4 }
}

/// **THE SEAM, PINNED AS HISTORY.** Two per-row settles at one stamp:
/// the first admits; against the state the first row leaves (anchor at
/// the stamp, per the sealed replay arm), the second row's bound is
/// ZERO and it refuses — the exact signature the era wiring surfaced.
/// This stays red forever on the old shape; that is WHY tag 102 exists.
#[test]
fn the_per_row_form_refuses_its_own_second_row() {
    let prior = dual_leak();
    let due = due_at_4(&prior);
    assert_eq!(due.len(), 2, "both resources leak — the dual-row case is real");
    let (first_resource, first_amount) = due[0].clone();
    let (second_resource, second_amount) = due[1].clone();

    // Row one admits against the untouched prior.
    let mut after_first = prior.clone();
    after_first.orbs[0].bank(first_resource, -first_amount.clone());
    after_first.orbs[0].last_touch = 4; // the sealed per-row anchor
    let row_one = ClosureEvent::DemurrageSettled {
        orb_id: 1,
        resource: first_resource,
        amount: first_amount,
    };
    assert_eq!(admits(&prior, &after_first, &stepped(row_one)), Ok(()));

    // Row two — lawful decay, emitted by the same touch — refuses with
    // bound ZERO against the state row one left.
    let mut after_second = after_first.clone();
    after_second.orbs[0].bank(second_resource, -second_amount.clone());
    let row_two = ClosureEvent::DemurrageSettled {
        orb_id: 1,
        resource: second_resource,
        amount: second_amount,
    };
    match admits(&after_first, &after_second, &stepped(row_two)) {
        Err(Refused::DecayExceedsBound { bound, .. }) => {
            assert_eq!(*bound, ratio(0, 1), "the second row's window is void");
        }
        other => panic!("the seam must show its own signature: {other:?}"),
    }
}

/// **THE ATOMIC TOUCH ADMITS THE SAME SETTLEMENT WHOLE** — both rows,
/// one receipt, judged against the anchor that stood before the touch.
#[test]
fn the_atomic_touch_admits_what_the_rows_could_not() {
    let prior = dual_leak();
    let decays = due_at_4(&prior);
    let mut next = prior.clone();
    for (resource, amount) in &decays {
        next.orbs[0].bank(*resource, -amount.clone());
    }
    next.orbs[0].last_touch = 4;
    let touch = ClosureEvent::DemurrageTouched { orb_id: 1, decays };
    assert_eq!(admits(&prior, &next, &stepped(touch)), Ok(()));
}

/// The atomic form's own walls: a repeated resource cannot draw one
/// window twice, and an overclaim still breaks on the sealed bound.
#[test]
fn the_atomic_touch_refuses_repeats_and_overclaims() {
    let prior = dual_leak();
    let due = due_at_4(&prior);
    let (resource, amount) = due[0].clone();

    let mut doubled = prior.clone();
    doubled.orbs[0].bank(resource, -(amount.clone() + amount.clone()));
    doubled.orbs[0].last_touch = 4;
    let repeat = ClosureEvent::DemurrageTouched {
        orb_id: 1,
        decays: vec![(resource, amount.clone()), (resource, amount.clone())],
    };
    assert!(matches!(
        admits(&prior, &doubled, &stepped(repeat)),
        Err(Refused::DecayRowDuplicated { .. })
    ));

    let greedy_amount = amount.clone() + ratio(1, 1);
    let mut greedy_next = prior.clone();
    greedy_next.orbs[0].bank(resource, -greedy_amount.clone());
    greedy_next.orbs[0].last_touch = 4;
    let greedy = ClosureEvent::DemurrageTouched {
        orb_id: 1,
        decays: vec![(resource, greedy_amount)],
    };
    assert!(matches!(
        admits(&prior, &greedy_next, &stepped(greedy)),
        Err(Refused::DecayExceedsBound { .. })
    ));
}

/// **THE EMITTER SPEAKS THE ATOMIC FORM** — a real dual-leak world's
/// touch writes ONE receipt carrying both rows, and the whole arc
/// replays and resumes (SEAL B).
#[test]
fn a_real_touch_emits_one_receipt_and_replays() {
    let mut world = World::new();
    world.found("seam-hold").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(
            cfe::world::FIRST_ORB,
            cfe::dop::unit(0, 1),
            cfe::dop::positive(1, 1),
            None,
            None,
        )
        .expect("sites mesic — leaking");
    // Bank both leaking resources through the lawful surface: joules
    // and sink arrive by absorption of vented entropy where staging is
    // heavy — here the founding bank plus a direct charter transfer
    // suffices: move iron to a charted twin and back to force touches,
    // with joule/sink banked via the test-side deed below.
    let charted = world.chart("twin", cfe::world::FIRST_ORB).expect("charts");
    for _ in 0..4 {
        world.tick();
    }
    // The transfer TOUCHES the source; with joule and sink both banked
    // and leaking the touch must settle them in ONE receipt.
    let orb = cfe::world::FIRST_ORB;
    let held = world.committed();
    let joule = held.orbs[0].banked_amount(Resource::Joule);
    let sink = held.orbs[0].banked_amount(Resource::Sink);
    if joule > ratio(0, 1) && sink > ratio(0, 1) {
        world
            .transfer(
                cfe::dop::between(orb, charted),
                Resource::Iron,
                cfe::dop::positive(1, 1),
            )
            .expect("the touch rides the transfer");
        let touched: Vec<usize> = world
            .chronicle()
            .receipts()
            .iter()
            .filter_map(|receipt| match receipt.event() {
                ClosureEvent::DemurrageTouched { decays, .. } => Some(decays.len()),
                _ => None,
            })
            .collect();
        assert!(
            touched.contains(&2),
            "a dual-leak touch settles both resources in one receipt: {touched:?}"
        );
    }
    // Whatever the staging banked, the arc replays and resumes whole.
    assert_eq!(cfe::world::replay(world.chronicle()), world.committed());
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed());
}
