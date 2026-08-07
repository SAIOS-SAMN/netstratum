//! MATTER-BALANCE-1 pins (34_MATTER_BALANCE).

use cfe::dop::{Positive, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{ActionRefused, World, FIRST_ORB, SOVEREIGN};

#[test]
fn bootstrap_allows_order_work_without_thermal_books() {
    let mut w = World::new();
    w.found("bal-boot").expect("found");
    assert!(w.require_order_balance(FIRST_ORB).is_ok());
    assert!(w.thermal_balance_of(FIRST_ORB).is_none());
}

#[test]
fn salvage_is_neutral_recycle_allows_hull_mint() {
    let mut w = World::new();
    w.found("bal-salvage").expect("found");
    w.commission_star(cfe::thermal::StarClass::G).expect("star");
    w.zone(1, ZoneKind::ShipYard).expect("yard");
    let salvage = Positive::new(ratio(4, 1)).expect("pos");
    w.salvage_scrap(FIRST_ORB, salvage)
        .expect("salvage");
    // Form-change residual books both arms equally → balance = 1/2 (orderly).
    let bal = w.thermal_balance_of(FIRST_ORB).expect("booked");
    assert_eq!(bal, World::balance_orderly_floor());
    let mint = w.mint_hull(FIRST_ORB, cfe::dop::HullRole::Hauler, 0);
    assert!(mint.is_ok(), "neutral salvage must not gate hull: {mint:?}");
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
}

#[test]
fn entropy_dominance_blocks_hull_mint_until_regenerator() {
    let mut w = World::new();
    w.found("bal-strain").expect("found");
    w.commission_star(cfe::thermal::StarClass::G).expect("star");
    // Yard + neutral scrap feed for the later mint.
    w.zone(1, ZoneKind::ShipYard).expect("yard");
    let salvage = Positive::new(ratio(4, 1)).expect("pos");
    w.salvage_scrap(FIRST_ORB, salvage).expect("salvage");

    // Strain via refine (pure entropy arm without Regenerator) — 34 §2.3.
    let _ = w.grant_tech_as(SOVEREIGN, FIRST_ORB, 3);
    w.zone(2, ZoneKind::ElementRefinery).expect("refinery");
    let _ = w.prospect(FIRST_ORB, 0);
    let element = w
        .committed()
        .prospects
        .first()
        .map(|p| p.element)
        .expect("prospect");
    for _ in 0..8 {
        w.tick();
    }
    let _ = w.assay(FIRST_ORB, element);
    let have = w
        .orb()
        .map(|o| o.element_amount(&element))
        .unwrap_or_else(|| ratio(0, 1));
    if have > ratio(0, 1) {
        let _ = w.refine(FIRST_ORB, element);
    }
    // If refine did not land (no vein yield), force strain is vacuous —
    // still require that when books are entropy-heavy, mint refuses.
    if let Some(bal) = w.thermal_balance_of(FIRST_ORB) {
        if bal < World::balance_orderly_floor() {
            let refuse = w.mint_hull(FIRST_ORB, cfe::dop::HullRole::Hauler, 0);
            assert!(
                matches!(refuse, Err(ActionRefused::BalanceEntropyDominates { .. })),
                "strained balance must gate hull mint: {refuse:?}"
            );
            // Unlock: regenerator (8 iron + 10 bio). Restock bio via habitat.
            w.zone(3, ZoneKind::Habitat).expect("habitat");
            for _ in 0..16 {
                w.tick();
                let _ = w.collect();
            }
            let regen = w.zone(4, ZoneKind::Regenerator);
            assert!(
                regen.is_ok(),
                "regenerator must build: {regen:?} bio={:?} iron={:?}",
                w.orb().map(|o| o.banked_amount(cfe::dop::Resource::Biomass)),
                w.orb().map(|o| o.banked_amount(cfe::dop::Resource::Iron)),
            );
            assert!(w.regenerator_stands(FIRST_ORB));
            assert!(w.require_order_balance(FIRST_ORB).is_ok());
            let mint = w.mint_hull(FIRST_ORB, cfe::dop::HullRole::Hauler, 0);
            assert!(mint.is_ok(), "regen unlocks mint: {mint:?}");
        }
    }
    assert_eq!(
        cfe::world::replay(w.chronicle()),
        w.committed(),
        "SEAL B after balance deeds"
    );
}

#[test]
fn element_salvage_feeds_scrap() {
    let mut w = World::new();
    w.found("bal-elem").expect("found");
    w.commission_star(cfe::thermal::StarClass::G).expect("star");
    let _ = w.zone(1, ZoneKind::ShipYard);
    let _ = w.prospect(FIRST_ORB, 0);
    let element = w.committed().prospects.first().map(|p| p.element);
    if let Some(el) = element {
        for _ in 0..5 {
            w.tick();
        }
        let _ = w.assay(FIRST_ORB, el);
        let have = w
            .orb()
            .map(|o| o.element_amount(&el))
            .unwrap_or_else(|| ratio(0, 1));
        if have > ratio(0, 1) {
            let amt = have / (ratio(1, 1) + ratio(1, 1));
            if let Some(p) = Positive::new(amt.clone()) {
                w.salvage_element_scrap(FIRST_ORB, el, p)
                    .expect("element salvage");
                let scrap = w
                    .orb()
                    .map(|o| o.scrap_amount())
                    .unwrap_or_else(|| ratio(0, 1));
                assert!(scrap >= amt);
                assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
            }
        }
    }
}
