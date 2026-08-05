//! VENTURE-1 remainder — the composition witness (10 §3d): the joint
//! research venture measured WHOLE. The parts landed separately (the
//! council, the licensed leap, the aperture pair, the venture CLI
//! sugar); what stood proven-by-argument was the COMPOSITION — one
//! deal id binding the leap and the exchange across two books, each
//! chronicle carrying its own legs, conservation exact both ways,
//! both worlds replaying. Measured here; the HANDOFF §9 idiom paid.

use cfe::dop::{positive, unit, ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::ChainRead;
use cfe::world::{aperture_transfer, license_tech, replay, Leap, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None).expect("sites");
    world
}

/// The whole venture, one deal: a council-gated leap OUT and the
/// exchange BACK under deal 21 — two sciences crossing what neither
/// tree holds alone, paid in matter the other way.
#[test]
fn the_venture_composes_whole_under_one_deal() {
    // The granting world: three chartered orbs seal the council.
    let mut grantor = sunlit("eridu-forge");
    let b = grantor.chart("midhold", FIRST_ORB).expect("charts");
    let c = grantor.chart("farhold", b).expect("charts");
    grantor.transfer(c, FIRST_ORB, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor.transfer(c, b, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor
        .seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 600, ratio(1, 1))
        .expect("the council stands open");
    // The receiving world: sunlit and crossed (deal 20, the crossing).
    let mut receiver = sunlit("kepler-passage");
    aperture_transfer(
        &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 20,
    )
    .expect("both cross");

    // THE VENTURE, deal 21: the leap out...
    license_tech(
        &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
        Leap { condition: 3, name: "lattice doctrine".into(), deal: 21, council: 1 },
    )
    .expect("the leap licenses through the council");
    // ...and the exchange back, the SAME deal — the receiver pays in
    // banked matter across the same opening.
    let grantor_iron_before = grantor.orb().expect("stands").banked_amount(Resource::Iron);
    let receiver_iron_before = receiver.orb().expect("stands").banked_amount(Resource::Iron);
    aperture_transfer(
        &mut receiver, FIRST_ORB, &mut grantor, FIRST_ORB,
        Resource::Iron, ratio(3, 1), 21,
    )
    .expect("the exchange returns under the venture's deal");

    // ONE deal id binds three receipts across two books.
    let deal_of = |world: &World, want_out: bool| -> Vec<u64> {
        world
            .chronicle()
            .receipts()
            .iter()
            .filter_map(|receipt| match &receipt.event {
                ClosureEvent::TechLicensed { deal, .. } => Some(*deal),
                ClosureEvent::TechDiscovered { .. } if !want_out => None,
                ClosureEvent::ApertureSent { deal, .. } => Some(*deal),
                ClosureEvent::ApertureReceived { deal, .. } => Some(*deal),
                _ => None,
            })
            .filter(|deal| *deal == 21)
            .collect()
    };
    assert_eq!(
        deal_of(&grantor, true).len(),
        2,
        "the grantor's book: the leap out (TechLicensed 21) and the exchange in (ApertureReceived 21)"
    );
    assert_eq!(
        deal_of(&receiver, false).len(),
        1,
        "the receiver's book: the exchange out (ApertureSent 21); its leap leg arrived as TechDiscovered"
    );
    // The same node derived on both sides — the venture teaches ONE
    // technology, not two copies.
    let node_out = grantor.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::TechLicensed { deal: 21, node, .. } => Some(*node),
        _ => None,
    });
    let node_in = receiver.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::TechDiscovered { condition: 3, node, .. } => Some(*node),
        _ => None,
    });
    assert_eq!(node_out.expect("issued"), node_in.expect("arrived"), "one node, two skies");

    // Conservation exact on the exchange: what left one bank arrived
    // in the other, to the fraction — nothing minted, nothing lost.
    let grantor_iron_after = grantor.orb().expect("stands").banked_amount(Resource::Iron);
    let receiver_iron_after = receiver.orb().expect("stands").banked_amount(Resource::Iron);
    assert_eq!(grantor_iron_after - grantor_iron_before, ratio(3, 1));
    assert_eq!(receiver_iron_before - receiver_iron_after, ratio(3, 1));

    // Both books whole: each chronicle carries only its own legs, and
    // each replays its committed world exactly.
    assert_eq!(grantor.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(receiver.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(replay(grantor.chronicle()), grantor.committed());
    assert_eq!(replay(receiver.chronicle()), receiver.committed());
}
