//! Does the deadlock survive main's per-orb standing?
//!
//! `21_KERNEL_PARAMETERS` §14–15 measured `capacity_law` refusing every
//! mint because a one-orb world reads `rank = 0` and `Vacuity::Void`.
//! main then replaced the world-wide `Entangled` gate with a PER-ORB
//! reading — `standing_of(state, orb)` and `MintWithoutOwnStanding`.
//! Whether that closes the deadlock is a measurement, not an inference.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::world::{World, SOVEREIGN};
use kernel::{bridge, lawful, Step, Unlawful};

#[test]
fn what_a_founded_world_now_reads() {
    let mut world = World::new();
    world.found("alone").expect("founds");
    let prior = world.committed();
    let orb = prior.orbs[0].orb_id;

    let read = bridge::read_entangled(&prior);
    println!("orbs={} rank={}", prior.orbs.len(), read.rank);
    println!("standing_of(orb {orb}) = {}", bridge::standing_of(&prior, orb));
    println!("axial = {:?}", bridge::standing_axial_of(&prior, orb));

    let step = Step {
        event: ClosureEvent::DiscoveryMinted {
            orb_id: orb,
            resource: Resource::Iron,
            amount: ratio(0, 1),
            work: Vec::new(),
        },
        actor: SOVEREIGN,
        stamp: 1,
    };
    let verdict = lawful(&prior, &prior, &step);
    println!("mint verdict = {verdict:?}");

    // THE CLAIM UNDER TEST, either way: is a founded world still barred
    // from minting? Recorded as whatever it measures.
    // MEASURED: the WORLD-WIDE gate fires first and short-circuits, so
    // main's per-orb narrowing is never reached on a fresh world. The
    // deadlock stands under the older variant name.
    assert!(
        matches!(verdict, Err(Unlawful::MintWithoutStanding { .. })),
        "a founded world is still barred from minting: {verdict:?}"
    );
    assert_eq!(bridge::standing_of(&prior, orb), ratio(0, 1), "and holds no standing of its own");
}

/// **THE NARROWING IS UNREACHABLE FROM GENESIS.**
///
/// main added `MintWithoutOwnStanding` — the free-rider gate, asking
/// what THIS orb holds up rather than what the world does. It is
/// strictly a narrowing, and correct. But `capacity_law` checks the
/// world verdict FIRST and returns on failure, so an orb can only be
/// refused for its OWN standing on a world that already has some.
///
/// A fresh world therefore never reaches it, and the two gates cannot
/// both be exercised until the world-level deadlock is ruled
/// (`21_KERNEL_PARAMETERS` §14-16).
#[test]
fn the_free_rider_gate_needs_a_world_that_already_stands() {
    let mut world = World::new();
    world.found("alone").expect("founds");
    let prior = world.committed();

    let read = bridge::read_entangled(&prior);
    assert!(!read.verdict().is_admitted(), "the world gate refuses first");
    assert_eq!(read.rank, 0, "because nothing relates yet");

    // So every mint on this board returns the WORLD variant, never the
    // per-orb one, however the orb is chosen.
    for orb in prior.orbs.iter().map(|o| o.orb_id) {
        let step = Step {
            event: ClosureEvent::DiscoveryMinted {
                orb_id: orb,
                resource: Resource::Iron,
                amount: ratio(0, 1),
                work: Vec::new(),
            },
            actor: SOVEREIGN,
            stamp: 1,
        };
        assert!(
            matches!(lawful(&prior, &prior, &step), Err(Unlawful::MintWithoutStanding { .. })),
            "orb {orb} is refused by the WORLD gate, not its own"
        );
    }
}
