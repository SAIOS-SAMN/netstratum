//! N3 witnesses — NS-6 ipsal-os first cut: the sovereign's tag rides
//! every receipt, agents act only inside their grants, revocation is
//! immediate, and the entity registry replays from receipts alone.

use cfe::dop::ZoneKind;
use cfe::ipsal::Scope;
use cfe::vrp::ChainRead;
use cfe::world::{ActionRefused, World, FIRST_ORB, SOVEREIGN};

#[test]
fn the_sovereign_is_registered_and_tags_every_receipt() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    let receipts = world.chronicle().receipts();
    assert!(matches!(
        receipts[0].event(),
        cfe::dop::ClosureEvent::EntityRegistered { entity_id: SOVEREIGN, principal: 0, .. }
    ));
    for receipt in receipts {
        assert_eq!(
            receipt.provenance(),
            vec![SOVEREIGN],
            "the generation tag rides every sovereign closure"
        );
    }
}

#[test]
fn an_agent_acts_only_inside_its_grant() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    let bot = world.register_agent("harvest-bot").expect("registers");
    // No grants yet: containment refuses every scope, typed.
    for _ in 0..2 {
        world.tick();
    }
    assert_eq!(
        world.collect_as(bot, FIRST_ORB),
        Err(ActionRefused::NotPermitted { entity: bot, scope: Scope::Collect })
    );
    assert_eq!(
        world.zone_as(bot, FIRST_ORB, 1, ZoneKind::Extractor),
        Err(ActionRefused::NotPermitted { entity: bot, scope: Scope::Build })
    );
    // One grant: exactly that scope opens, the others stay shut.
    world.grant(bot, Scope::Collect).expect("the sovereign grants");
    world.collect_as(bot, FIRST_ORB).expect("the granted scope acts");
    assert_eq!(
        world.zone_as(bot, FIRST_ORB, 1, ZoneKind::Extractor),
        Err(ActionRefused::NotPermitted { entity: bot, scope: Scope::Build }),
        "a grant is a scope, not a blanket"
    );
    // The agent's receipts carry ITS tag, not the sovereign's.
    let last = world.chronicle().receipts().last().expect("collected");
    assert_eq!(last.provenance(), vec![bot], "generation-tagged to the forging agent");
}

#[test]
fn revocation_is_immediate() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    let bot = world.register_agent("harvest-bot").expect("registers");
    world.grant(bot, Scope::Collect).expect("granted");
    world.tick();
    world.collect_as(bot, FIRST_ORB).expect("acts inside the grant");
    world.revoke(bot, Scope::Collect).expect("revoked");
    world.tick();
    assert_eq!(
        world.collect_as(bot, FIRST_ORB),
        Err(ActionRefused::NotPermitted { entity: bot, scope: Scope::Collect }),
        "revocation needs no cooperation from the agent"
    );
}

#[test]
fn containment_refuses_the_unknown_and_the_sovereign_shortcut() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    assert_eq!(
        world.collect_as(99, FIRST_ORB),
        Err(ActionRefused::NoSuchEntity { entity: 99 })
    );
    let bot = world.register_agent("bot").expect("registers");
    assert_eq!(
        world.register_agent("bot"),
        Err(ActionRefused::HandleTaken { handle: "bot".into() })
    );
    assert_eq!(
        world.grant(SOVEREIGN, Scope::Build),
        Err(ActionRefused::NotAnAgent { entity: SOVEREIGN }),
        "sovereign authority is by construction, never by grant"
    );
    let _ = bot;
}

#[test]
fn a_revoked_scope_stays_revoked() {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    let bot = world.register_agent("harvest-bot").expect("registers");
    world.grant(bot, Scope::Collect).expect("granted");
    world.grant(bot, Scope::Raze).expect("granted");
    world.revoke(bot, Scope::Raze).expect("revoked");
    world.tick();
    world.collect_as(bot, FIRST_ORB).expect("acts");
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    let committed = world.committed();
    let agent = committed
        .entities
        .iter()
        .find(|held| held.entity_id == bot)
        .expect("the agent stands");
    assert_eq!(agent.grants, vec![Scope::Collect], "the revoked scope stays revoked");
}
