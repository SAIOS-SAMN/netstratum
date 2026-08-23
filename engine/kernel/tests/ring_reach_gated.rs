//! THE MAGNETIC REACH, GATED — the kernel holds the ring rule the
//! emitter refuses on, by calling the SAME sealed rule (2026-08-06).

use cfe::dop::ClosureEvent;

use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{admits, Refused, Step};

fn sited_o_system() -> (World, Vec<u64>) {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.commission_star(StarClass::O).expect("star");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("mine");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("farm");
    for _ in 0..120 {
        world.tick();
    }
    world.collect().expect("banks");
    for k in 1..3 {
        world.chart(&format!("orb{k}"), FIRST_ORB).expect("charts");
    }
    let ids: Vec<u64> = world.orbs().iter().map(|o| o.orb_id).collect();
    (world, ids)
}

#[test]
fn the_gate_holds_the_ring_rule() {
    let (mut world, ids) = sited_o_system();
    world.site_planet(ids[0], cfe::dop::unit(1, 2), cfe::dop::positive(1, 1), None, None).expect("ring one");
    world.site_planet(ids[1], cfe::dop::unit(1, 2), cfe::dop::positive(2, 1), None, None).expect("ring two");
    let prior = world.committed();

    // A lawful third siting ON an anchored ring admits.
    world.site_planet(ids[2], cfe::dop::unit(1, 2), cfe::dop::positive(1, 1), None, None).expect("fills ring one");
    let next = world.committed();
    let receipt = world.chronicle().receipts().last().expect("siting").clone();
    let step = Step { event: receipt.event().clone(), actor: SOVEREIGN, stamp: receipt.stamp() };
    assert_eq!(admits(&prior, &next, &step), Ok(()), "an anchored ring fills lawfully");

    // The SAME transition claimed as a THIRD ring refuses — the gate
    // computes the reach from the same rule the emitter drew.
    let forged = Step {
        event: ClosureEvent::PlanetSited {
            orb_id: ids[2],
            albedo: cfe::dop::unit(1, 2),
            orbit_r: cfe::dop::positive(3, 1),
            theta: None,
            psi: None,
        },
        actor: SOVEREIGN,
        stamp: step.stamp,
    };
    assert!(
        matches!(
            admits(&prior, &next, &forged),
            Err(Refused::RingBeyondReach { held: 2, reach: 2 })
        ),
        "a ring beyond the field refuses at the gate too"
    );
    assert_eq!(StarClass::O.ring_reach(), 2, "and the figure is the dial's");
}
