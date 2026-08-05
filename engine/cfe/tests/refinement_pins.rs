//! **THE REFINEMENT LAW REPRODUCES THE TABLE** (operator ruling
//! 2026-08-05: *too many constants — you aren't building algorithms or
//! theorems, you're building static content*).
//!
//! The structural economy read as six tuned literals. It is two
//! formulas. This is the witness that says so — and the witness that
//! would catch it if the law and the table ever disagreed, which is the
//! only thing that makes stating a law worth anything.

use cfe::dop::{
    refinement_input, refinement_tier, refinement_yield, Resource, ZoneKind,
};
use cfe::ratio::ratio;

/// **THE LAW IS THE TABLE.** Every structural converter's landed input
/// and yield fall out of `2^(t−1)` and `4^(2−t)` — so the law reprices
/// nothing, it only stops repeating itself.
#[test]
fn the_law_reproduces_every_structural_converter() {
    // (converter, what it consumes, what it emits)
    let chain = [
        (ZoneKind::Smelter, Resource::Iron, Resource::Alloy),
        (ZoneKind::Synthesizer, Resource::Alloy, Resource::Diamond),
    ];
    for (kind, consumes, emits) in chain {
        let tier = refinement_tier(emits).expect("a structural tier");
        assert_eq!(
            refinement_tier(consumes),
            Some(tier - 1),
            "a converter consumes exactly one tier below what it emits"
        );

        // THE YIELD, derived.
        let (resource, rate) = kind.yield_per_tick();
        assert_eq!(resource, emits);
        assert_eq!(
            rate,
            refinement_yield(tier),
            "{kind:?} yields {rate}, the law says {}",
            refinement_yield(tier)
        );

        // THE INPUT, derived — the structural component, not the food
        // slot (biomass rides the ration law, a different one).
        let structural = kind
            .upkeep_per_tick()
            .into_iter()
            .find(|(r, _)| *r == consumes)
            .map(|(_, amount)| amount)
            .expect("a converter consumes its own input tier");
        assert_eq!(
            structural,
            refinement_input(tier),
            "{kind:?} consumes {structural}, the law says {}",
            refinement_input(tier)
        );
    }
}

/// **THE COST PER REFINED UNIT RISES BY EIGHT PER STEP** — the reason a
/// refined tier is worth more than what it is made of, as one number
/// instead of an inference from six.
#[test]
fn each_step_costs_eight_times_the_last() {
    for tier in 2..=4u32 {
        let here = refinement_input(tier) / refinement_yield(tier);
        let next = refinement_input(tier + 1) / refinement_yield(tier + 1);
        assert_eq!(next, here * ratio(8, 1), "the ladder's step is not eight at {tier}");
    }
    // Measured at the landed rungs: 2 iron per alloy, 16 alloy per diamond.
    assert_eq!(refinement_input(2) / refinement_yield(2), ratio(2, 1));
    assert_eq!(refinement_input(3) / refinement_yield(3), ratio(16, 1));
}

/// **THE LAW DECLINES WHAT IT DOES NOT GOVERN.** Joule, Sink and Crystal
/// are off the structural chain — the heat law mints joules, the sink
/// law banks cold, Crystal is the survey currency. A law that claimed
/// them would be overreach, and `None` is the honest answer.
#[test]
fn the_law_names_its_own_boundary() {
    for off in [Resource::Joule, Resource::Sink, Resource::Crystal] {
        assert_eq!(refinement_tier(off), None, "{off:?} is not a refinement");
    }
    for on in [Resource::Iron, Resource::Biomass, Resource::Alloy, Resource::Diamond] {
        assert!(refinement_tier(on).is_some(), "{on:?} stands on the chain");
    }
    // And the off-chain converters genuinely yield zero on the chain's
    // terms — their output comes from elsewhere.
    assert_eq!(ZoneKind::Encasement.yield_per_tick().1, ratio(0, 1));
    assert_eq!(ZoneKind::CryoArray.yield_per_tick().1, ratio(0, 1));
}

/// The raw tier is where the ladder starts: taken from the world,
/// refined from nothing.
#[test]
fn the_raw_tier_is_refined_from_nothing() {
    assert_eq!(refinement_tier(Resource::Iron), Some(1));
    assert_eq!(refinement_tier(Resource::Biomass), Some(1));
    // An extractor consumes no structural input — only the food slot.
    let structural: Vec<_> = ZoneKind::Extractor
        .upkeep_per_tick()
        .into_iter()
        .filter(|(r, _)| refinement_tier(*r) == Some(0))
        .collect();
    assert!(structural.is_empty(), "raw extraction refines nothing");
}
