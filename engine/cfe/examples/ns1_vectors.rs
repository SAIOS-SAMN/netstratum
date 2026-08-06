//! NS-1 vector extraction — prints the REAL wire bytes of the first
//! slice's orb and events, for inscription in the spec. Run:
//! `cargo run -p cfe --example ns1_vectors`

use cfe::dop::{ClosureEvent, ColonyOrb, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{replay, World};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn main() {
    // The session the spec narrates.
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    world.zone(4, ZoneKind::Extractor).expect("zoning closes");
    for _ in 0..3 {
        world.tick();
    }
    world.collect().expect("accrual closes");

    let orb = world.orb().expect("the colony stands");
    println!("orb wire bytes: {}", hex(&orb.wire_bytes()));
    let reloaded = ColonyOrb::from_wire(&orb.wire_bytes()).expect("decodes");
    println!("round trip byte-exact: {}", reloaded.wire_bytes() == orb.wire_bytes());

    for event in [
        ClosureEvent::ColonyFounded { orb_id: 1, name: "meridian".into() },
        ClosureEvent::ZoneBuilt { orb_id: 1, tile: 4, kind: ZoneKind::Extractor, base: None },
        ClosureEvent::YieldCollected {
            orb_id: 1,
            resource: Resource::Iron,
            amount: ratio(9, 2),
        },
        ClosureEvent::EntityRegistered {
            entity_id: 2,
            principal: 1,
            handle: "harvest-bot".into(),
        },
        ClosureEvent::ScopeGranted { agent: 2, scope: cfe::ipsal::Scope::Collect },
        ClosureEvent::ScopeRevoked { agent: 2, scope: cfe::ipsal::Scope::Collect },
    ] {
        println!("event {:?}", event);
        println!("  bytes: {}", hex(&event.wire_bytes()));
    }

    for receipt in world.chronicle().receipts() {
        println!(
            "receipt {} stamp {} parent {} own {}",
            receipt.sequence,
            receipt.stamp,
            hex(&receipt.parent.0),
            hex(&receipt.own.0)
        );
    }
    println!("replay matches committed: {}", replay(world.chronicle()) == world.committed());
}
