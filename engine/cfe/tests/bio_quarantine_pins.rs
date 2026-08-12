//! **BIO QUARANTINE (task #25, ruled 2026-08-12).**
//!
//! When a world's bio-pressure — cascade (BioChemCross) or rogue
//! (SynthRogue) — crosses the synth-grade ceiling, its modification has
//! run past what it can safely hold and the world is QUARANTINED: its
//! external apertures are severed. The Agent Door seals to inbound
//! commerce (trade treaties, melt reciprocity) until the pressure drops.
//! The crossing is exempt — quarantine seals the market, never escape.
//!
//! The gate is a REFUSAL, so it leaves no receipt and cannot retro-break
//! a sealed era: an old chronicle's already-admitted deeds replay
//! unchanged; only NEW inbound proposals meet the seal.

use cfe::almanac::ExplosionKind;
use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::treaty::{TreatyKind, TreatyTerms, PAIR_MELT};
use cfe::world::{ActionRefused, World, FIRST_ORB, SOVEREIGN};

/// A world driven to `bumps` cascade-pressure strikes by receipt, then
/// resumed so the pressure is chain-true (replay rebuilds it from the
/// BioChemCross explosions).
fn pressured(name: &str, bumps: u64) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    let stamp = world.now();
    for _ in 0..bumps {
        world.chronicle_mut().append(
            ClosureEvent::ScienceExploded {
                orb_id: FIRST_ORB,
                path: 0,
                kind: ExplosionKind::BioChemCross.as_wire(),
                resource: cfe::dop::Resource::Biomass as u8,
                slash: ratio(0, 1),
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    World::resume(world.chronicle().clone())
}

#[test]
fn the_threshold_is_the_synth_grade_ceiling_and_quarantine_trips_at_it() {
    // One short of the ceiling is not yet quarantine.
    let below = pressured("below", World::QUARANTINE_PRESSURE - 1);
    assert!(!below.world_quarantined(), "under the ceiling, commerce stands");
    assert!(!below.is_quarantined(FIRST_ORB));

    // At the ceiling, the world seals.
    let at = pressured("at", World::QUARANTINE_PRESSURE);
    assert!(at.is_quarantined(FIRST_ORB), "the pressure crossed the ceiling");
    assert!(at.world_quarantined(), "and the world stands quarantined");
}

#[test]
fn a_quarantined_world_cannot_open_a_trade_pact() {
    let mut world = pressured("sealed-market", World::QUARANTINE_PRESSURE);
    let trade = TreatyTerms::new(TreatyKind::Trade, vec![(PAIR_MELT, ratio(1, 2))]).expect("trade");
    assert!(
        matches!(
            world.treaty(1, "farhold", trade, 0),
            Err(ActionRefused::WorldQuarantined { .. })
        ),
        "the Agent Door is sealed to inbound trade"
    );
}

#[test]
fn quarantine_seals_the_market_never_the_politics_or_the_escape() {
    // War and PEACE are political, not commercial — a quarantined world
    // may still end or start a war. The seal is on trade alone.
    let mut world = pressured("still-at-war", World::QUARANTINE_PRESSURE);
    let peace = TreatyTerms::new(TreatyKind::Peace, vec![]).expect("peace");
    assert!(
        !matches!(
            world.treaty(1, "farhold", peace, 0),
            Err(ActionRefused::WorldQuarantined { .. })
        ),
        "peace is not barred by quarantine"
    );
}

#[test]
fn a_clean_world_is_never_quarantined_by_this_gate() {
    // The gate must not fire on a world that never crossed the
    // threshold: a trade pact refused here would be refused for its own
    // reasons, never for a quarantine that does not stand.
    let mut world = pressured("clean", 0);
    assert!(!world.world_quarantined());
    let trade = TreatyTerms::new(TreatyKind::Trade, vec![(PAIR_MELT, ratio(1, 2))]).expect("trade");
    assert!(
        !matches!(
            world.treaty(1, "farhold", trade, 0),
            Err(ActionRefused::WorldQuarantined { .. })
        ),
        "a clean world is never quarantine-sealed"
    );
}
