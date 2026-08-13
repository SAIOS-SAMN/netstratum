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
use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::treaty::{TreatyKind, TreatyTerms, PAIR_MELT};
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

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

// ── task #30: the pressure relief — quarantine is not permanent ──

/// A collectable world driven to `rogue` rogue-pressure by receipt (an
/// extractor over a stocked bank, so a collect always settles).
fn quarantined_collectable(rogue: u8) -> World {
    let mut world = World::new();
    world.found("quar-hold").expect("founds");
    world.commission_star(StarClass::G).expect("star");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    let stamp = world.now();
    for res in [Resource::Biomass, Resource::Iron] {
        world.chronicle_mut().append(
            ClosureEvent::YieldCollected { orb_id: FIRST_ORB, resource: res, amount: ratio(1_000_000, 1) },
            stamp,
            vec![SOVEREIGN],
        );
    }
    world.chronicle_mut().append(
        ClosureEvent::RogueFactionStruck { orb_id: FIRST_ORB, pressure: rogue },
        stamp,
        vec![SOVEREIGN],
    );
    let mut world = World::resume(world.chronicle().clone());
    world.zone(0, ZoneKind::Extractor).expect("extractor");
    world
}

fn collect_once(world: &mut World) {
    for _ in 0..4 {
        world.tick();
    }
    let _ = world.collect();
}

#[test]
fn the_pressure_deed_round_trips() {
    let deed = ClosureEvent::PressureEbbed { orb_id: 42 };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 168, "PressureEbbed is tag 168");
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
}

#[test]
fn a_quarantined_world_recovers_a_step_at_the_touch_and_lifts() {
    // Maxed rogue pressure quarantines the world; a collect touch ebbs it
    // one step, dropping below the threshold — quarantine LIFTS. It was
    // permanent before this settle deed.
    let mut world = quarantined_collectable(7);
    assert!(world.world_quarantined(), "the modification maxed out — quarantined");
    collect_once(&mut world);
    assert_eq!(world.rogue_pressure_of(FIRST_ORB), 6, "the pressure ebbed one step");
    assert!(!world.world_quarantined(), "the apertures reopen once below the threshold");
    assert!(
        world
            .chronicle()
            .receipts()
            .iter()
            .any(|r| matches!(r.event, ClosureEvent::PressureEbbed { .. })),
        "the recovery is receipted"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "pressure-relief replay MATCHES");
    // Below the threshold the ebb stops — the residual is a lasting scar.
    collect_once(&mut world);
    assert_eq!(world.rogue_pressure_of(FIRST_ORB), 6, "recovered below quarantine, the ebb rests");
}

#[test]
fn a_sub_threshold_pressure_never_ebbs_so_it_is_byte_identical() {
    // A world modified but NOT past the ceiling (pressure 3) is not
    // quarantined; the collect emits no relief deed and leaves the
    // pressure untouched — byte-identical to the pre-#30 era.
    let mut world = quarantined_collectable(3);
    assert!(!world.world_quarantined(), "below the ceiling, no quarantine");
    collect_once(&mut world);
    assert_eq!(world.rogue_pressure_of(FIRST_ORB), 3, "a sub-threshold pressure does not ebb");
    assert!(
        !world
            .chronicle()
            .receipts()
            .iter()
            .any(|r| matches!(r.event, ClosureEvent::PressureEbbed { .. })),
        "no quarantine → no relief deed → byte-identical"
    );
}
