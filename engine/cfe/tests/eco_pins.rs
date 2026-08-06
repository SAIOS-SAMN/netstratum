//! **ECO-1 (#40) — THE ECOLOGICAL LOOP.** Extraction is no longer
//! free at the ground: a vein is a SPRING whose standing flow the
//! draw tires, and whose recovery is the Regenerator's second arm.
//!
//! Every number derives — the spring's natural flow is the hash's
//! ([`cfe::dop::element_properties`], the same reading the mint
//! receipted), the wear is priced by the draw against that nature,
//! and the recovery runs at the Regenerator's own working rate. No
//! new dial enters. The wear settles in the SAME receipt as the draw
//! (the atomic-touch shape), so replay reads the vein's new flow
//! rather than re-deriving it.

use cfe::dop::{positive, unit, vein_wear, Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

/// A charted world under a G star, its extractor standing.
fn charted(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world
}

/// The vein's standing flow, as the live registry holds it.
fn standing(world: &World, element: &[u8; 32]) -> Ratio {
    world
        .committed()
        .prospects
        .iter()
        .find(|vein| vein.element == *element)
        .map(|vein| vein.richness.clone())
        .expect("the vein stands")
}

#[test]
fn the_draw_tires_the_spring_and_every_draw_after_it_pays_less() {
    let mut world = charted("worn-hold");
    let struck = world.prospect(FIRST_ORB, 1).expect("the stratum opens");
    let natural = struck.richness.clone();
    assert_eq!(standing(&world, &struck.element), natural, "a fresh vein runs at its nature");

    // Four equal spans, drawn back to back. The span is identical
    // every time, so any fall in the take is the GROUND's, not the
    // clock's.
    let mut takes: Vec<Ratio> = Vec::new();
    for _ in 0..4 {
        for _ in 0..4 {
            world.tick();
        }
        takes.push(world.assay(FIRST_ORB, struck.element).expect("the vein pays"));
    }
    for pair in takes.windows(2) {
        assert!(pair[1] < pair[0], "each draw pays less than the last: {takes:?}");
    }
    assert!(
        standing(&world, &struck.element) < natural,
        "the spring stands below its nature once worked"
    );
    assert!(
        standing(&world, &struck.element) > ratio(0, 1),
        "and never below zero — the wear is a fraction, never a subtraction"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_regenerator_restores_what_the_draw_took() {
    // TWO WORLDS, ONE HISTORY apart from the regenerator: same name,
    // same star, same stratum — so the elements, the natural flows and
    // the spans are identical, and the ONLY difference in the standing
    // flow at the end is the restoration.
    let mut bare = charted("mirror-hold");
    let mut tended = charted("mirror-hold");

    // The deepening arrives on the tended world, and its regenerator
    // plants. (Condition 3 is REGEN-1's gate; the grant is the
    // licensed path.)
    let hand = tended.register_agent("gifting-hand").expect("registers");
    tended.grant(hand, Scope::Build).expect("granted");
    tended.grant_tech_as(hand, FIRST_ORB, 3).expect("condition 3 crystallizes");
    tended.zone(1, ZoneKind::Regenerator).expect("the regenerator plants");

    let worked = |world: &mut World| {
        let struck = world.prospect(FIRST_ORB, 1).expect("opens");
        for _ in 0..3 {
            for _ in 0..4 {
                world.tick();
            }
            world.assay(FIRST_ORB, struck.element).expect("pays");
        }
        (struck.element, struck.richness, standing(world, &struck.element))
    };
    let (bare_element, natural, bare_flow) = worked(&mut bare);
    let (tended_element, tended_natural, tended_flow) = worked(&mut tended);

    assert_eq!(bare_element, tended_element, "one grammar, one seed, one element");
    assert_eq!(natural, tended_natural, "and one natural flow");
    assert!(
        tended_flow > bare_flow,
        "the tended vein stands higher: {tended_flow} vs {bare_flow}"
    );
    assert!(
        tended_flow <= natural,
        "and never above its nature — the cap is a theorem: {tended_flow} vs {natural}"
    );
    assert_eq!(replay(tended.chronicle()), tended.committed(), "replay MATCHES");
}

#[test]
fn the_wear_law_is_bounded_by_construction() {
    // The two bounds the law rests on, over a spread of inputs: the
    // worn flow never rises above nature however many regenerators
    // stand, and never falls to or below zero however hard the draw.
    let natural = ratio(1, 2);
    for standing_flow in [ratio(1, 2), ratio(1, 4), ratio(1, 16)] {
        for drawn in [ratio(0, 1), ratio(1, 1), ratio(97, 1)] {
            for regenerators in [0u64, 1, 8] {
                for elapsed in [0u64, 1, 40] {
                    let after =
                        vein_wear(&standing_flow, &natural, &drawn, regenerators, elapsed);
                    assert!(after <= natural, "no spring runs past its nature: {after}");
                    assert!(after > ratio(0, 1), "and no draw empties one: {after}");
                }
            }
        }
    }
    // More regenerators restore more; a longer span restores more.
    let worked = |regenerators, elapsed| {
        vein_wear(&ratio(1, 8), &natural, &ratio(3, 2), regenerators, elapsed)
    };
    assert!(worked(2, 4) > worked(1, 4), "two hands restore more than one");
    assert!(worked(1, 8) > worked(1, 4), "and a longer span restores more");
    assert_eq!(
        worked(0, 40),
        worked(0, 4),
        "with no regenerator standing, time alone restores nothing"
    );
}

#[test]
fn the_worked_vein_stays_cheap_to_carry_forever() {
    // A CHARTER CONCERN, measured rather than assumed. The wear law is
    // a rational RECURRENCE and this engine is exact-rational with no
    // rounding anywhere, so the obvious hazard is that each draw
    // multiplies the denominator and a long-worked vein becomes
    // ruinous to compute, encode and replay.
    //
    // It does not happen, and the reason is structural. With `s` the
    // standing flow and `N` the natural one, an untended draw over a
    // span gives `s' = s·N/(N + span·s)`; in the reciprocal that is
    // `1/s' = 1/s + span/N` — a straight LINE. So the numerator stays
    // 1 and the denominator grows by a fixed step per draw: the vein
    // depletes hyperbolically toward zero without ever reaching it,
    // and its exact form stays short forever.
    //
    // Measured at 60 draws of four ticks each: the standing flow walks
    // 1/162, 1/322, 1/482 … — numerator 1, denominator stepping by a
    // constant. The literal numbers are this WORLD's (the element
    // derives from its founding digest), so the pin asserts the shape
    // the law guarantees rather than one world's draw.
    let mut world = charted("carry-cost");
    let struck = world.prospect(FIRST_ORB, 1).expect("opens");
    let one = ratio(1, 1);
    let mut reciprocals: Vec<Ratio> = Vec::new();
    let mut widest = 0usize;
    for _ in 1..=60 {
        for _ in 0..4 {
            world.tick();
        }
        world.assay(FIRST_ORB, struck.element).expect("pays");
        let flow = standing(&world, &struck.element);
        widest = widest.max(format!("{flow}").len());
        reciprocals.push(one.clone() / flow);
    }
    // THE LINE: every step of the reciprocal is the same size.
    let steps: Vec<Ratio> = reciprocals
        .windows(2)
        .map(|pair| pair[1].clone() - pair[0].clone())
        .collect();
    assert!(
        steps.windows(2).all(|pair| pair[0] == pair[1]),
        "the reciprocal walks a straight line: {steps:?}"
    );
    assert!(steps[0] > ratio(0, 1), "and it walks upward — the vein depletes");
    // THE CARRY COST: no growth in the written form across 60 draws.
    assert!(widest <= 12, "a worked vein's exact form stays short: {widest} chars");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_regenerators_ration_line_still_runs() {
    // ECO-1 gives the Regenerator its SECOND arm; the first one — the
    // ration yield REGEN-1 landed — is untouched by this amendment.
    assert_eq!(
        cfe::dop::zone_yield(ZoneKind::Regenerator, 2),
        (Resource::Biomass, ratio(3, 2)),
        "what extraction takes, regeneration returns"
    );
}
