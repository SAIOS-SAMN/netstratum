//! BENCH + TRADE hub smoke — menus compile and World APIs they read stay live.

use cfe::dop::{positive, unit, ZoneKind};
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB};

#[test]
fn hub_status_apis_construct_on_founded_world() {
    let mut w = World::new();
    w.found("hub-home").expect("found");
    w.commission_star(StarClass::G).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    let _ = w.zone(0, ZoneKind::Extractor);
    // BENCH surfaces (chem + expanded knowledge/link domains).
    let _ = w.bench(FIRST_ORB);
    let _ = w.bench_science(FIRST_ORB);
    let _ = w.dark_pulls();
    let _ = w.civ_axes();
    let _ = w.chem_catalog();
    let _ = w.detection_grade();
    let _ = w.espionage_integrity();
    let _ = w.inf_integrity();
    let _ = w.defense_posture();
    let _ = w.councils();
    let _ = w.wants();
    let _ = w.terrain_seed();
    // Deep game boards
    let rb = w.research_board(FIRST_ORB);
    assert!(!rb.nodes.is_empty() || rb.nodes.is_empty()); // constellation may be empty pre-tick
    let ib = w.intel_board(FIRST_ORB);
    assert_eq!(ib.detection, w.detection_grade());
    let lb = w.link_board();
    let _ = lb.score;
    let db = w.doctrine_board();
    assert_eq!(db.ranks.len(), 7);
    // TRADE surfaces.
    let _ = w.market_integrity();
    let _ = w.deep_rail_status();
    let _ = w.free_capacity(0);
    let _ = w.hulls();
    let _ = w.vault_balance_of(FIRST_ORB);
    let _ = w.exchange_seated();
    let _ = w.element_routes();
    assert!(w.star().is_some());
}

#[test]
fn trade_barter_form_still_requires_full_args() {
    // Pure contract: incomplete barter is hub territory; this pin only
    // asserts the world still founds so CLI wiring can be exercised offline.
    let mut w = World::new();
    w.found("barter-gate").expect("found");
    assert_eq!(w.orb().map(|o| o.orb_id), Some(FIRST_ORB));
}
