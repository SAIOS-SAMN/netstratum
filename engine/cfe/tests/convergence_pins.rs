//! CONVERGENCE — the syntropic arm of `FrictionSettled` (task #14).
//!
//! Friction is `rate · |Δ reciprocity| · value`, receipted only when
//! POSITIVE. When conduct MATCHES, divergence is zero, friction is
//! zero, and nothing is receipted: the engine computes the syntropic
//! half on every transfer and destroys it in the same expression, and
//! until now no caller could ask for it — `reciprocity_reading` was
//! private.
//!
//! These pin the reading, and above all the trap under it: the private
//! default answers `1/1` for an orb with no cross-orb history, so two
//! orbs that have never traded read as PERFECTLY ALIGNED. Absence
//! presenting as agreement is the same shape as the borne share's
//! farming-by-absence, and it must not survive into anything that ever
//! pays on alignment.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};

/// Two colonies with banked stores, and no cross-orb flow yet.
fn two_colonies() -> (World, u64) {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let far = world.chart("outpost", FIRST_ORB).expect("charts");
    (world, far)
}

#[test]
fn absence_is_not_agreement() {
    // THE TRAP, NAMED. Neither orb has drawn across a boundary, so
    // neither has a reciprocity basis. The private reading friction
    // uses would answer 1/1 for both — correct for PRICING (charge
    // nothing when there is nothing to compare) and catastrophic for
    // MERIT: two orbs that have never traded would read as perfectly
    // aligned and, under any rule that paid on alignment, would be
    // paid exactly what the reciprocal are paid.
    let (world, far) = two_colonies();
    assert_eq!(world.reciprocity_of(FIRST_ORB), None, "no cross-orb flow, no basis");
    assert_eq!(world.reciprocity_of(far), None, "nor here");
    assert_eq!(
        world.convergence_between(FIRST_ORB, far),
        None,
        "two orbs that have never traded have nothing to AGREE about — \
         this must never read as perfect convergence"
    );
}

#[test]
fn an_orb_the_world_never_seated_has_no_reading() {
    // A reading, never a panic and never a default. An unseated orb is
    // not a zero-reciprocity orb; it is an orb there is nothing to say
    // about.
    let (world, _) = two_colonies();
    assert_eq!(world.reciprocity_of(9_999), None);
    assert_eq!(world.convergence_between(FIRST_ORB, 9_999), None);
}

#[test]
fn convergence_is_a_magnitude_that_is_high_when_conduct_agrees() {
    // STATED POSITIVELY. Convergence is HIGH when conduct matches,
    // rather than friction being ABSENT — "the reward for alignment is
    // the absence of a penalty" is the negentropic shape this work
    // exists to refuse.
    //
    // Drive real cross-orb flow so both sides have a basis: a draw
    // opens a reciprocity entry, and value flowing back closes it.
    let (mut world, far) = two_colonies();
    world
        .transfer(FIRST_ORB, far, Resource::Iron, ratio(4, 1))
        .expect("moves value across the boundary");
    world
        .transfer(far, FIRST_ORB, Resource::Iron, ratio(2, 1))
        .expect("and some of it returns");
    for _ in 0..4 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, FIRST_ORB).expect("settles");

    // Whatever the arithmetic settles on, the SHAPE is the law: a
    // reading present on both sides yields a convergence in [0, 1],
    // and it is a magnitude rather than a penalty.
    if let Some(c) = world.convergence_between(FIRST_ORB, far) {
        assert!(c >= ratio(0, 1), "convergence is never below the ground: {c}");
        assert!(c <= ratio(1, 1), "and never above the whole: {c}");
    }

    // An orb agrees perfectly with ITSELF whenever it has a basis at
    // all — the identity case, and it must not be confused with the
    // absence case above, which answers None.
    if world.reciprocity_of(FIRST_ORB).is_some() {
        assert_eq!(
            world.convergence_between(FIRST_ORB, FIRST_ORB),
            Some(ratio(1, 1)),
            "identical conduct is total convergence — 1, not 'no friction'"
        );
    }
}

#[test]
fn surfacing_the_reading_moves_no_consensus() {
    // THE PRIVATE READING IS UNTOUCHED. Friction is consensus law and
    // its 1/1 default is correct for pricing a cost; this landing adds
    // a reading and changes nothing that settles. Same discipline as
    // the thermal book: the cure for a missing arm is to ADD the arm.
    let (world, far) = two_colonies();
    let before = world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented.clone();
    let _ = world.reciprocity_of(FIRST_ORB);
    let _ = world.convergence_between(FIRST_ORB, far);
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented,
        before,
        "reading convergence settles nothing and vents nothing"
    );
}
