//! CONSTEL-1 uplink pin — the §9.2 lesson applied at landing: the
//! constellation block's BYTES are asserted, never just emitted. The
//! state.json carries condition · held · progress · threshold as
//! counters (the crystallizer's own deposit sheet); ratios derive
//! surface-side.

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB};
use ledger::Ledger;
use std::fs;

#[test]
fn the_constellation_rides_the_uplink_with_its_bytes_pinned() {
    let root = std::env::temp_dir().join(format!("uplink-pin-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    // A sited world holds its kernel (the solar leap receipts at the
    // opt-in) and a cold famine accrues insulation PROGRESS.
    let mut world = World::new();
    world.found("dimlight-hold").expect("founds");
    world.commission_star(StarClass::M).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites cold");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones");
    for _ in 0..70 {
        world.tick();
    }
    let book = Ledger::at(&root);
    book
        .record("dimlight-hold", "hearth", "test-line", &world)
        .expect("reflects");
    let state = fs::read_to_string(
        root.join("dimlight-hold/planets/hearth/state.json"),
    )
    .expect("state exists");
    assert!(
        state.contains(r#""condition":"solar""#) && state.contains(r#""held":1"#),
        "the kernel rides the uplink held: {state}"
    );
    assert!(
        state.contains(r#""condition":"insulation""#),
        "the famine's progress rides as a counter"
    );
    assert!(
        !state.contains("_ratio\":") || state.contains("cohesion"),
        "the constellation carries counters only — ratios derive surface-side"
    );
    let _ = fs::remove_dir_all(&root);
}
