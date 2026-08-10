//! E-MKT-1 — MARKET-1 offers carry ElementId cargo; clear_trade hauls
//! open matter through the element aperture; conservation exact; SEAL B.

use cfe::dop::{
    exchange, positive, unit, Cargo, Exchange, HullRole, Resource, ZoneKind,
};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{clear_trade, replay, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world
}

/// When orbital/key melt is positive, hauls need hull capacity (FLEET-1).
/// Grade-0 hauler holds 4 capacity units — enough for a unit fill.
fn equip_hauler(world: &mut World) {
    world.zone(4, ZoneKind::ShipYard).expect("yard");
    world
        .salvage_scrap(FIRST_ORB, positive(4, 1))
        .expect("scrap");
    world
        .mint_hull(FIRST_ORB, HullRole::Hauler, 0)
        .expect("hauler");
}

fn bank_element(world: &mut World) -> [u8; 32] {
    let struck = world.prospect(FIRST_ORB, 1).expect("prospects");
    for _ in 0..16 {
        world.tick();
    }
    let drawn = world.assay(FIRST_ORB, struck.element).expect("assays");
    assert!(drawn > ratio(0, 1), "assay banks mass");
    struck.element
}

#[test]
fn an_element_offer_rests_and_replays() {
    let mut world = sunlit("element-bazaar");
    let element = bank_element(&mut world);
    let stock = world.orb().unwrap().element_amount(&element);
    let give = cfe::dop::Positive::new(stock.clone() / ratio(2, 1)).expect("half positive");
    world
        .post_offer(
            FIRST_ORB,
            Exchange::cargo(
                Cargo::Element(element),
                Cargo::Resource(Resource::Iron),
            )
            .unwrap(),
            give,
            positive(1, 1),
            42,
        )
        .expect("posts");
    assert_eq!(world.offers().len(), 1);
    match &world.offers()[0].give {
        Cargo::Element(id) => assert_eq!(*id, element),
        Cargo::Resource(_) => panic!("expected element cargo"),
    }
    assert_eq!(
        replay(world.chronicle()).offers.len(),
        1,
        "element offer book replays"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn element_for_iron_crossing_conserves() {
    let mut seller = sunlit("ore-side");
    let mut buyer = sunlit("iron-side");
    // Keep clocks matched so orbital melt stays zero (same site, same now).
    let struck = seller.prospect(FIRST_ORB, 1).expect("prospects");
    let element = struck.element;
    for _ in 0..16 {
        seller.tick();
        buyer.tick();
    }
    let drawn = seller.assay(FIRST_ORB, element).expect("assays");
    assert!(drawn >= ratio(1, 1), "enough mass for a unit fill: {drawn}");
    equip_hauler(&mut seller);
    equip_hauler(&mut buyer);
    // Unit fill stays under grade-0 hauler capacity (4).
    let give = positive(1, 1);

    seller
        .post_offer(
            FIRST_ORB,
            Exchange::cargo(
                Cargo::Element(element),
                Cargo::Resource(Resource::Iron),
            )
            .unwrap(),
            give,
            positive(2, 1),
            7,
        )
        .expect("seller: element for iron");
    buyer
        .post_offer(
            FIRST_ORB,
            Exchange::cargo(
                Cargo::Resource(Resource::Iron),
                Cargo::Element(element),
            )
            .unwrap(),
            positive(4, 1),
            positive(1, 1),
            9,
        )
        .expect("buyer: iron for element");

    let el_before = seller.orb().unwrap().element_amount(&element)
        + buyer.orb().unwrap().element_amount(&element);
    let iron_before = seller.orb().unwrap().banked_amount(Resource::Iron)
        + buyer.orb().unwrap().banked_amount(Resource::Iron);

    clear_trade(&mut seller, 7, &mut buyer, 9, 100, ratio(0, 1))
        .expect("element↔iron crossing clears");

    let el_after = seller.orb().unwrap().element_amount(&element)
        + buyer.orb().unwrap().element_amount(&element);
    let iron_after = seller.orb().unwrap().banked_amount(Resource::Iron)
        + buyer.orb().unwrap().banked_amount(Resource::Iron);
    // MARKET-2 melt may take a void share; never mints either ledger.
    assert!(el_after <= el_before, "element not minted: {el_after} ≤ {el_before}");
    assert!(iron_after <= iron_before, "iron not minted: {iron_after} ≤ {iron_before}");
    assert!(
        buyer.orb().unwrap().element_amount(&element) > ratio(0, 1),
        "buyer receives element cargo"
    );
    assert!(seller.offers().is_empty() && buyer.offers().is_empty());
    assert_eq!(replay(seller.chronicle()), seller.committed());
    assert_eq!(replay(buyer.chronicle()), buyer.committed());
}

#[test]
fn resource_only_exchange_still_posts() {
    let mut world = sunlit("legacy-book");
    world
        .post_offer(
            FIRST_ORB,
            exchange(Resource::Iron, Resource::Biomass),
            positive(4, 1),
            positive(2, 1),
            1,
        )
        .expect("legacy resource offer");
    assert!(matches!(
        world.offers()[0].give,
        Cargo::Resource(Resource::Iron)
    ));
    assert_eq!(replay(world.chronicle()), world.committed());
}
