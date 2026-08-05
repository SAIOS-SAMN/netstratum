//! **REPLAY FIDELITY — the guard clippy was standing in for.**
//!
//! Twice in one session `replay.rs` bound a receipt field and wrote
//! `None` instead of it: `theta`, then `psi`, in the same arm. Both
//! times the only thing between that and a silent live/replay
//! divergence was an unused-variable WARNING. That is not a law.
//!
//! **Exhaustiveness cannot catch it.** The struct literal named every
//! field; the VALUE was wrong. What catches it is the invariant:
//!
//! **for any world, `replay(chronicle) == committed()`.**
//!
//! These pins populate fields a default world leaves empty, so a
//! dropped field fails HERE and names itself, rather than surfacing
//! three tests away as a resource that drifted.

use cfe::dop::{positive, unit, Slot, ZoneKind};

use cfe::thermal::StarClass;
use cfe::world::{replay, World, SOVEREIGN};

fn fidelity(world: &World, what: &str) {
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "REPLAY DROPPED SOMETHING after {what} — the chain does not rebuild the world"
    );
}

/// THE PIN THAT WOULD HAVE CAUGHT BOTH DROPS.
#[test]
fn a_sited_world_replays_with_both_turns_intact() {
    let mut world = World::new();
    world.found("placed").expect("founds");
    world.commission_star(StarClass::G).expect("star");
    let orb = world.committed().orbs[0].orb_id;
    world
        .site_planet(orb, unit(1, 4), positive(3, 2), Some(Slot::from_index(17)), Some(Slot::from_index(43)))
        .expect("sites with a place and an inclination");

    let live = world.committed().orbs[0].thermal.clone().expect("sited");
    let rebuilt = replay(world.chronicle()).orbs[0].thermal.clone().expect("replay sites it");
    assert_eq!(rebuilt.theta, live.theta, "REPLAY DROPPED θ");
    assert_eq!(rebuilt.psi, live.psi, "REPLAY DROPPED ψ");
    fidelity(&world, "siting with both turns");
}

/// Absent must stay absent — `None` must never become a ZERO anywhere
/// in the round trip, or every legacy orb silently moves to the equator.
///
/// TRUTH-UPDATED (THE PLACEMENT DRAW, 2026-08-06): a `theta: None`
/// DEED no longer records placeless — the world draws the longitude,
/// chronicle-seeded, and the receipt carries the resolved slot. That is
/// not the zero-coercion this pin guards against: the drawn angle is
/// receipted, on-slot, and replays exactly. What this pin still holds:
/// ψ is NOT drawn, so absence survives there — and legacy receipts
/// carrying `None` replay verbatim, because replay reads receipts and
/// the draw lives only at the deed.
#[test]
fn the_draw_places_theta_and_absence_stays_absent_in_psi() {
    let mut world = World::new();
    world.found("bare").expect("founds");
    world.commission_star(StarClass::G).expect("star");
    let orb = world.committed().orbs[0].orb_id;
    world.site_planet(orb, unit(1, 4), positive(3, 2), None, None).expect("sites");

    let live = world.committed().orbs[0].thermal.clone().expect("sited");
    let site = replay(world.chronicle()).orbs[0].thermal.clone().expect("sited");
    assert!(site.theta.is_some(), "the draw placed it — placeless is not a place");
    assert_eq!(site.theta, live.theta, "the RECEIPTED slot replays exactly, never re-drawn");
    assert!(site.psi.is_none(), "ψ is not drawn — absent stays absent, never zero");
    fidelity(&world, "siting through the placement draw");
}

/// The vertical axis through the same guard, with a RE-seating so the
/// replacement law is exercised rather than an append.
#[test]
fn seatings_replay_including_a_replacement() {
    let mut world = World::new();
    world.found("seated").expect("founds");
    world.seat_station_as(SOVEREIGN, "abyssal-vocalis", 3).expect("seats");
    world.seat_station_as(SOVEREIGN, "brine-cortex", 1).expect("seats");
    world.seat_station_as(SOVEREIGN, "abyssal-vocalis", 2).expect("re-seats");

    assert_eq!(
        replay(world.chronicle()).stations,
        world.committed().stations,
        "REPLAY DROPPED A SEAT, or applied the replacement differently"
    );
    fidelity(&world, "three seatings including a replacement");
}

/// A world that has actually RUN. The narrow pins name a field; this
/// one catches whatever they did not think to look at.
#[test]
fn a_world_that_has_run_still_rebuilds_from_its_chain() {
    let mut world = World::new();
    world.found("busy").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(1, ZoneKind::Habitat).expect("zones");
    world.commission_star(StarClass::G).expect("star");
    let orb = world.committed().orbs[0].orb_id;
    world
        .site_planet(orb, unit(1, 4), positive(1, 1), Some(Slot::from_index(5)), None)
        .expect("sites");

    for _ in 0..24 {
        world.tick();
    }
    let _ = world.collect();
    fidelity(&world, "24 ticks, a siting and a collection");
}
