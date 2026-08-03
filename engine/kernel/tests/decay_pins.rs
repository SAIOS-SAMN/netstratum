//! KERNEL invariant witnesses — I5 completes: the decay bound
//! (U1f, U_DIRECTIVES §3; red-first, the red recorded against the
//! sign-only kernel).
//!
//! The bound is THE SEALED DEMURRAGE RULE, called never copied
//! (`cfe::world::demurrage_due` — flat rate × ease × elapsed, capped
//! at banked, gated on the thermal leak): a decay claiming exactly
//! its due admits; a decay past the due refuses with both sides
//! whole; at elapsed zero only the zero decay is lawful. `last_touch`
//! already rides the orb wire (tag 28, VENT-1) — committed state, no
//! schema change spent.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{lawful, Step, Refused};

/// The thermal_flow fixture, re-played to the pre-decay point: a G
/// star, a leaking site (albedo 0, orbit 1/2), 15/8 joules banked at
/// `touch`. Rate 1/32 · ease 1 · elapsed 4 → due 1/8.
fn leaky_world() -> (World, u64) {
    let mut world = World::new();
    world.found("assay").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None).expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones");
    world.tick();
    world.collect().expect("collects");
    let touch = world.orb().expect("orb").last_touch;
    (world, touch)
}

fn decay_step(amount: (i64, i64), stamp: u64) -> Step {
    Step {
        event: ClosureEvent::DemurrageSettled {
            orb_id: FIRST_ORB,
            resource: Resource::Joule,
            amount: ratio(amount.0, amount.1),
        },
        actor: SOVEREIGN,
        stamp,
    }
}

#[test]
fn the_exact_due_admits_and_the_excess_refuses_both_sides_whole() {
    let (world, touch) = leaky_world();
    let prior = world.committed();
    let mut next = prior.clone();
    next.orbs[0].bank(Resource::Joule, -ratio(1, 8));
    // The exact due at elapsed 4: 1/32 × 1 × 4 = 1/8 — admitted.
    assert_eq!(
        admits(&prior, &next, &decay_step((1, 8), touch + 4)),
        Ok(()),
        "the sealed rule's own answer is lawful"
    );
    // One thirty-second past the due: refused, both sides carried.
    let mut greedy = prior.clone();
    greedy.orbs[0].bank(Resource::Joule, -ratio(5, 32));
    assert_eq!(
        admits(&prior, &greedy, &decay_step((5, 32), touch + 4)),
        Err(Refused::DecayExceedsBound {
            bound: Box::new(ratio(1, 8)),
            claimed: Box::new(ratio(5, 32)),
        }),
        "value leaves only by the flat law — never faster"
    );
}

#[test]
fn at_elapsed_zero_only_the_zero_decay_is_lawful() {
    let (world, touch) = leaky_world();
    let prior = world.committed();
    // No time has passed: the zero decay admits…
    assert_eq!(admits(&prior, &prior.clone(), &decay_step((0, 1), touch)), Ok(()));
    // …and ANY positive decay refuses with bound zero carried.
    let mut next = prior.clone();
    next.orbs[0].bank(Resource::Joule, -ratio(1, 32));
    assert_eq!(
        admits(&prior, &next, &decay_step((1, 32), touch)),
        Err(Refused::DecayExceedsBound {
            bound: Box::new(ratio(0, 1)),
            claimed: Box::new(ratio(1, 32)),
        }),
        "no elapsed time, no decay — the clock is the law"
    );
}

#[test]
fn the_sign_law_still_stands_beneath_the_bound() {
    let (world, touch) = leaky_world();
    let prior = world.committed();
    let mut next = prior.clone();
    next.orbs[0].bank(Resource::Joule, ratio(1, 8));
    assert_eq!(
        admits(&prior, &next, &decay_step((-1, 8), touch + 4)),
        Err(Refused::DecayNotPositive {
            resource: Resource::Joule,
            amount: Box::new(ratio(-1, 8)),
        }),
        "a decay that creates value is refused before any bound is read"
    );
}
