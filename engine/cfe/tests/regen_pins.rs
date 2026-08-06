//! **REGEN-1 — regeneration is a technology** (operator directive
//! 2026-08-06). The regenerator's every number DERIVES from the
//! standing zone table (the shell's frame + a double hearth; the
//! ration resource at the extractor's own rate; the habitat's
//! life-support line) — zero new dials. Placing one requires the
//! DEEPENING (condition 3) crystallized on the orb.

use cfe::dop::{Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

#[test]
fn regeneration_is_gated_then_restores() {
    let mut world = World::new();
    world.found("verdant-hold").expect("founds");
    // Ungated: the technology is not held — the fact refuses.
    assert!(matches!(
        world.zone(0, ZoneKind::Regenerator),
        Err(ActionRefused::ConditionUncrystallized { condition: 3 })
    ));
    // The deepening arrives (the granted path), and the ground opens.
    let agent = world.register_agent("gifting-hand").expect("registers");
    world.grant(agent, Scope::Build).expect("granted");
    world.grant_tech_as(agent, FIRST_ORB, 3).expect("condition 3 crystallizes");
    // Bank the derived cost: encasement iron (8) + habitat bio ×2 (10).
    let cost = ZoneKind::Regenerator.cost();
    assert_eq!(
        cost,
        vec![(Resource::Iron, ratio(8, 1)), (Resource::Biomass, ratio(10, 1))],
        "the cost derives from the standing table"
    );
    world.zone(0, ZoneKind::Regenerator).expect("the regenerator plants");
    // The yield derives: the ration resource at the extractor's rate.
    assert_eq!(
        cfe::dop::zone_yield(ZoneKind::Regenerator, 2),
        (Resource::Biomass, ratio(3, 2)),
        "what extraction takes, regeneration returns"
    );
    // And it runs: ticks accrue the ration resource.
    for _ in 0..4 {
        world.tick();
    }
    let accrued = world
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Biomass)
        .map(|(_, amount)| amount)
        .unwrap_or_else(|| ratio(0, 1));
    assert!(accrued > ratio(0, 1), "the restoration flows: {accrued}");
    // SEAL B: the arc replays.
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
