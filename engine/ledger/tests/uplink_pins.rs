//! CONSTEL-1 — constellation is a derived World reading, not a JSON
//! export. The machine mirror is `state.tlv` (orb wire); the base-7
//! stack lives on the World and re-derives from the chain.

use cfe::dop::{positive, unit};
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB};
use ledger::Ledger;
use std::fs;

#[test]
fn the_constellation_is_world_derived_and_mirror_is_tlv() {
    let root = std::env::temp_dir().join(format!("uplink-pin-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let mut world = World::new();
    world.found("dimlight-hold").expect("founds");
    world
        .commission_star(StarClass::M)
        .expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites cold");
    world
        .zone(8, cfe::dop::ZoneKind::Habitat)
        .expect("zones");
    for _ in 0..70 {
        world.tick();
    }
    // Constellation is a World derivation — not an export file.
    let stack = world.constellation(FIRST_ORB);
    assert!(
        stack.iter().any(|(condition, held, _, _)| *condition == 0 && *held),
        "solar kernel held on the World"
    );
    assert!(
        stack.iter().any(|(condition, _, progress, _)| *condition == 1 && *progress > 0),
        "insulation progress accrues on the World"
    );

    let book = Ledger::at(&root);
    scribe::record(&book, "dimlight-hold", "hearth", "test-line", &world)
        .expect("reflects");
    let tlv = root.join("dimlight-hold/planets/hearth/state.tlv");
    assert!(tlv.exists(), "machine mirror is state.tlv");
    assert!(
        !root
            .join("dimlight-hold/planets/hearth/state.json")
            .exists(),
        "no JSON mirror on the write path"
    );
    let _ = fs::remove_dir_all(&root);
}
