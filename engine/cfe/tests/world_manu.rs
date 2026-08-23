//! MANU-1 witnesses (10 §3f) — the instrument manufacturing deed:
//! manufacturing overlays technology + understanding + industry.
//! Understanding gates (no crystallized condition → refuses), the
//! chain gates (no smelter → refuses), the forge banks alloy for the
//! frame, and replay rebuilds the held instrument from receipts.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// A smelting colony with alloy banked (the world_industry staging).
fn manufacturer() -> World {
    let mut world = World::new();
    world.found("lens-home").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(1, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(9, ZoneKind::Habitat).expect("zones");
    for _ in 0..10 {
        world.tick();
    }
    world.collect().expect("smelter affordable");
    world.zone(16, ZoneKind::Smelter).expect("the chain stands");
    for _ in 0..8 {
        world.tick();
    }
    world.collect().expect("alloy banks");
    world
}

#[test]
fn the_forge_gates_then_holds() {
    use cfe::world::survey::{forgeable_grade, ForgeableGrade};
    let mut world = manufacturer();
    // Understanding gates: condition 1 has not crystallized.
    assert_eq!(
        world.forge_instrument(FIRST_ORB, forgeable_grade(1)),
        Err(ActionRefused::ConditionUncrystallized { condition: 1 }),
        "the understanding comes before the instrument"
    );
    // Sharpened by the door (29_CONSTRUCTION): an unforgeable grade
    // cannot be CONSTRUCTED — the leap (0), the lesson (7) and the
    // uplift works never become a ForgeableGrade, so the dead
    // InstrumentGradeUnknown arm has no case left to catch.
    for outside in [0u8, 7, 8, 99] {
        assert!(
            ForgeableGrade::new(outside).is_none(),
            "condition {outside} must not construct a forgeable grade"
        );
    }
    // Crystallize condition 1 (the granted path — understanding lands).
    let agent = world.register_agent("gifting-hand").expect("registers");
    world.grant(agent, Scope::Build).expect("granted");
    world.grant_tech_as(agent, FIRST_ORB, 1).expect("condition 1 crystallizes");
    // Grade 3 needs the synthesizer leg too — the chain gates.
    world.grant_tech_as(agent, FIRST_ORB, 3).expect("condition 3 crystallizes");
    assert_eq!(
        world.forge_instrument(FIRST_ORB, forgeable_grade(3)),
        Err(ActionRefused::ChainIncomplete { needs: ZoneKind::Synthesizer }),
    );
    // Grade 1 forges: one alloy for the frame, receipted.
    let alloy_before = world.orb().unwrap().banked_amount(Resource::Alloy);
    world.forge_instrument(FIRST_ORB, forgeable_grade(1)).expect("the instrument forges");
    let orb = world.orb().unwrap();
    assert_eq!(
        orb.banked_amount(Resource::Alloy),
        alloy_before - ratio(1, 1),
        "the frame drew alloy = grade"
    );
    assert_eq!(world.instruments().len(), 1);
    assert_eq!(world.instruments()[0].2, 1, "grade IS the condition");
    let receipted = world.chronicle().receipts().iter().any(|receipt| {
        matches!(receipt.event(), ClosureEvent::InstrumentForged { grade: 1, .. })
    });
    assert!(receipted, "the forge is a receipt");
    // Replay rebuilds the held instrument — the file is the world.
    let committed = replay(world.chronicle());
    assert_eq!(committed.instruments, world.instruments(), "replay MATCHES");
}
