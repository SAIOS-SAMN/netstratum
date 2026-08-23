//! **ONT-2 — THE VEIN AS A DERIVED PROJECTION.** Extraction is not free at
//! the ground, but a vein is no longer a STORED spring a recurrence steps:
//! its richness is DERIVED on every read as `natural − extracted` — the
//! hash's endowment ([`cfe::dop::element_properties`]) minus the exact ledger
//! integral the assays banked. The draw is a geometric decay of that derived
//! richness ([`cfe::dop::vein_flow`]), and the extraction integral is a
//! division-FREE sum, so the vein's exact form can never explode — the
//! 115-beat scalar collapse is structurally impossible, not patched.
//!
//! Recovery is no longer an in-assay term: a depleted vein STAYS depleted
//! until rejuvenation derives it back (ONT-3: orbital forcing for ambient
//! flux, topological cycle injection for physical veins). This file pins the
//! depletion and the anti-collapse bound; ONT-3 will pin the return.

use cfe::dop::{element_properties, positive, unit, vein_flow, vein_richness, Resource, ZoneKind};
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

/// The vein's geometric endowment (`natural`) — a pure reading of the hash.
fn natural_of(world: &World, element: &[u8; 32]) -> Ratio {
    let star = world.committed().star.expect("a star stands");
    element_properties(element, star).2
}

/// The vein's DERIVED standing richness = `natural − extracted`, never stored.
fn standing(world: &World, element: &[u8; 32]) -> Ratio {
    let star = world.committed().star.expect("a star stands");
    world
        .committed()
        .prospects
        .iter()
        .find(|vein| vein.element == *element)
        .map(|vein| vein_richness(&element_properties(element, star).2, &vein.extracted, &vein.rejuvenated))
        .expect("the vein stands")
}

#[test]
fn the_draw_depletes_the_vein_and_no_draw_pays_more_than_the_last() {
    let mut world = charted("worn-hold");
    let struck = world.prospect(FIRST_ORB, 1).expect("the stratum opens");
    let natural = natural_of(&world, &struck.element);
    assert_eq!(
        standing(&world, &struck.element),
        natural,
        "a fresh vein runs at its nature (extracted nothing)"
    );

    // Four equal spans, drawn back to back — the span is identical every
    // time, so any fall in the take is the GROUND's depletion, not the clock.
    let mut takes: Vec<Ratio> = Vec::new();
    for _ in 0..4 {
        for _ in 0..4 {
            world.tick();
        }
        takes.push(world.assay(FIRST_ORB, struck.element).unwrap_or_else(|_| ratio(0, 1)));
    }
    // The take is NON-INCREASING: the derived richness only falls (the
    // integral only grows), so each draw pays no more than the one before.
    for pair in takes.windows(2) {
        assert!(pair[1] <= pair[0], "no draw pays more than the last: {takes:?}");
    }
    // The vein stands below its nature once worked, and never below zero (the
    // richness floor is a max, the integral bounded by the endowment).
    let after = standing(&world, &struck.element);
    assert!(after < natural, "the vein stands below its nature once worked: {after}");
    assert!(after >= ratio(0, 1), "and never below zero: {after}");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_extraction_law_is_bounded_by_construction() {
    // The two bounds the derivation rests on, over a spread of inputs — now
    // theorems of the projection, not clamps on a recurrence: the derived
    // richness never rises above nature (extracted ≥ 0 ⇒ richness ≤ natural)
    // and never falls below zero (the max floor); the flow never exceeds the
    // richness that stands (so the integral never crosses the endowment).
    let natural = ratio(1, 2);
    for extracted in [ratio(0, 1), ratio(1, 4), ratio(1, 2), ratio(97, 1)] {
        let richness = vein_richness(&natural, &extracted, &ratio(0, 1));
        assert!(richness <= natural, "no vein stands past its nature: {richness}");
        assert!(richness >= ratio(0, 1), "and none below zero: {richness}");
        for elapsed in [0u64, 1, 40] {
            let flow = vein_flow(&richness, &natural, elapsed);
            assert!(flow <= richness, "no draw takes more than stands: {flow} vs {richness}");
            assert!(flow >= ratio(0, 1), "and none negative: {flow}");
        }
    }
}

#[test]
fn the_worked_vein_stays_cheap_to_carry_forever() {
    // THE ANTI-COLLAPSE THEOREM, measured. Richness derives as
    // `natural − extracted`, and `extracted` grows by
    // `min(richness · natural · Δt, richness)` — a product-and-subtraction,
    // NEVER a division — so the denominator is bounded forever by
    // `denom(natural)² ≤ 36` and the vein's exact form stays short across any
    // number of draws. This is the exact spot the old `vein_wear` recurrence
    // detonated (~beat 115): here it cannot, by construction.
    let mut world = charted("carry-cost");
    let struck = world.prospect(FIRST_ORB, 1).expect("opens");
    let mut widest = 0usize;
    let mut prev = standing(&world, &struck.element);
    for _ in 1..=60 {
        for _ in 0..4 {
            world.tick();
        }
        let _ = world.assay(FIRST_ORB, struck.element);
        let flow = standing(&world, &struck.element);
        widest = widest.max(format!("{flow}").len());
        // Monotone depletion: the integral only grows, so richness only falls.
        assert!(flow <= prev, "the vein depletes monotonically: {flow} vs {prev}");
        prev = flow;
    }
    // THE CARRY COST: the written form does not grow — the denominator is
    // pinned to the geometry, not the causal history.
    assert!(widest <= 12, "a worked vein's exact form stays short: {widest} chars");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_worked_vein_rejuvenates_at_a_collect_touch() {
    // ONT-3, LIVE — a vein drawn down RECOVERS under Keplerian orbital forcing.
    // At orbit_r = 1 under a G star the local year IS the beat (P = 1), so the
    // ticks between the draw and the collect close that many orbits and the vein
    // grows back a quantum apiece, receipted and replay-exact.
    let mut world = charted("regrow-hold");
    let struck = world.prospect(FIRST_ORB, 1).expect("the stratum opens");
    let natural = natural_of(&world, &struck.element);
    // Work the vein down so there is headroom to recover into.
    for _ in 0..4 {
        world.tick();
    }
    let _ = world.assay(FIRST_ORB, struck.element);
    let depleted = standing(&world, &struck.element);
    assert!(depleted < natural, "the vein is worked below its nature: {depleted}");
    // Let many local years pass (P = 360 beats at r = 1), then collect —
    // rejuvenation settles the veins over the orbits that closed.
    for _ in 0..4000 {
        world.tick();
    }
    world.collect().expect("collects");
    let recovered = standing(&world, &struck.element);
    assert!(recovered > depleted, "the vein recovered: {depleted} → {recovered}");
    assert!(recovered <= natural, "but never past its endowment: {recovered} vs {natural}");
    // A VeinRejuvenated receipt carries the recovery; replay banks the same.
    let receipted = world
        .chronicle()
        .receipts()
        .iter()
        .any(|r| matches!(r.event(), cfe::dop::ClosureEvent::VeinRejuvenated { .. }));
    assert!(receipted, "a VeinRejuvenated receipt stands");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_regenerators_ration_line_still_runs() {
    // The Regenerator's ration yield (REGEN-1) is untouched by ONT-2; only
    // its vein-recovery second arm is gone (it moves to ONT-3, derived).
    assert_eq!(
        cfe::dop::zone_yield(ZoneKind::Regenerator, 2),
        (Resource::Biomass, ratio(3, 2)),
        "what extraction takes, regeneration returns"
    );
}
