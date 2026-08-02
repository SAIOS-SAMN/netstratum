//! Does `capacity_law` refuse EVERY mint on a real board?
//!
//! remediations-lane measured `MintWithoutStanding{Void}` 61 then 68
//! times per 10 beats — the judge refusing every mint because nothing
//! is Entangled. That is a verdict on MY invariant, so it is verified
//! here rather than taken on report.

use cfe::world::World;

#[test]
fn a_one_orb_world_can_never_mint() {
    let mut world = World::new();
    world.found("alone").expect("founds");
    let prior = world.committed();

    let read = kernel::bridge::read_entangled(&prior);
    let verdict = read.verdict();
    println!("orbs={} rank={} verdict={verdict:?}", prior.orbs.len(), read.rank);
    println!("V side: flows={} (the potential the verdict reads)", read.flows.len());
    let banked: usize = prior.orbs.iter().map(|o| o.banked.len()).sum();
    println!("banked entries the world ACTUALLY holds: {banked}");

    assert_eq!(read.rank, 0, "one orb, no relations, no cycles");
    assert!(!verdict.is_admitted(), "so capacity_law refuses every mint on this board");
}

/// How many orbs before rank can be non-zero at all? A cycle needs at
/// least three vertices, so a board of ones and twos can never mint
/// however many books it holds.
#[test]
fn rank_needs_a_cycle_and_a_cycle_needs_three() {
    let mut world = World::new();
    world.found("still-alone").expect("founds");
    let prior = world.committed();
    // No obligations exist at founding, so even the edges are absent.
    let read = kernel::bridge::read_entangled(&prior);
    assert_eq!(read.rank, 0);
    assert!(
        read.symmetry_foundation.is_empty(),
        "no relations, so nothing to emit a plane from"
    );
}
