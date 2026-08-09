//! P8.1 — shared AP budget (build · prospect · research · quote).

use cfe::dop::{positive, unit, Exchange, Positive, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{
    replay, ActionRefused, ApKind, World, AP_YEAR_TICKS, FIRST_ORB,
};

fn founded() -> World {
    let mut w = World::new();
    w.found("ap-home").expect("found");
    w.commission_star(StarClass::G).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    w
}

#[test]
fn capacity_starts_at_base_and_grows_with_habitats() {
    let mut w = founded();
    let base = w.ap_capacity();
    assert!(base >= 16, "base capacity is founding-roomy");
    assert_eq!(w.ap_spent(), 0);
    assert_eq!(w.ap_available(), base);

    // Each zone costs 1 AP (Build).
    w.zone(0, ZoneKind::Extractor).expect("ext");
    assert_eq!(w.ap_spent(), 1);
    assert_eq!(w.ap_available(), base - 1);

    w.zone(1, ZoneKind::Habitat).expect("hab");
    // Habitat raises capacity for subsequent reads (+4).
    let after_hab = w.ap_capacity();
    assert!(after_hab > base, "habitat expands AP capacity");
    assert_eq!(w.ap_spent(), 2);
    assert_eq!(replay(w.chronicle()), w.committed());
}

#[test]
fn opportunity_cost_shared_across_families() {
    let mut w = founded();
    // Drain almost all AP with builds, leave 1.
    let cap = w.ap_capacity();
    let mut tile = 0u16;
    while w.ap_spent() + 1 < cap {
        // Prefer free tiles; mix kinds that don't need tech.
        let kind = if tile % 2 == 0 {
            ZoneKind::Extractor
        } else {
            ZoneKind::Habitat
        };
        if w.zone(tile, kind).is_err() {
            break;
        }
        tile = tile.saturating_add(1);
        if tile > 40 {
            break;
        }
    }
    // Spend remaining AP on prospects until one left or zero.
    let mut stratum = 1u8;
    while w.ap_available() > 1 && stratum < 30 {
        let _ = w.prospect(FIRST_ORB, stratum);
        stratum = stratum.saturating_add(1);
    }
    // If any AP remains, spend it on prospect; next gated deed refuses.
    if w.ap_available() >= 1 {
        let _ = w.prospect(FIRST_ORB, stratum);
    }
    assert_eq!(w.ap_available(), 0, "window fully spent");
    let refuse = w.zone(50, ZoneKind::Extractor).expect_err("AP exhausted");
    match refuse {
        ActionRefused::ApExhausted { kind, need, have, capacity } => {
            assert_eq!(kind, "build");
            assert_eq!(need, 1);
            assert_eq!(have, 0);
            assert_eq!(capacity, w.ap_capacity());
        }
        other => panic!("expected ApExhausted, got {other:?}"),
    }
    // Quote also refuses — shared pool.
    let pair = Exchange::new(Resource::Iron, Resource::Biomass).expect("pair");
    let give = Positive::new(ratio(1, 1)).expect("g");
    let want = Positive::new(ratio(1, 1)).expect("w");
    let refuse_q = w
        .post_offer(FIRST_ORB, pair, give, want, 9001)
        .expect_err("quote AP exhausted");
    assert!(matches!(refuse_q, ActionRefused::ApExhausted { .. }));
}

#[test]
fn year_voice_refills_ap_window() {
    let mut w = founded();
    w.zone(0, ZoneKind::Extractor).expect("z");
    assert_eq!(w.ap_spent(), 1);
    // Advance clock past year-voice boundary.
    for _ in 0..AP_YEAR_TICKS {
        w.tick();
    }
    // Spent count only receipts in the new window — prior ZoneBuilt is old.
    assert_eq!(w.ap_window_start() % AP_YEAR_TICKS, 0);
    assert_eq!(
        w.ap_spent(),
        0,
        "new year-voice window resets AP spend count"
    );
    assert_eq!(w.ap_available(), w.ap_capacity());
    // Can build again (extractor only needs iron — founding residual).
    w.zone(1, ZoneKind::Extractor).expect("ext after refill");
    assert_eq!(w.ap_spent(), 1);
}

#[test]
fn ap_board_reports_families() {
    let w = founded();
    let b = w.ap_board();
    assert_eq!(b.capacity, w.ap_capacity());
    assert_eq!(b.available, b.capacity);
    assert!(b.next_hint.contains("AP"));
    assert_eq!(ApKind::Build.cost(), 1);
    assert_eq!(ApKind::Prospect.word(), "prospect");
}
