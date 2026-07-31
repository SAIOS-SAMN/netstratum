//! OPPOSITION-1 witnesses (NS-5 law 2, the mesh board's #4 — the
//! anti-mirror): the first measurement whose inputs are NOT copies of
//! each other. Deal-linked deeds across two DIFFERENT books must sum
//! to exact zero; a forged amount measures its residual WHOLE; an
//! unpaired leg breaks the opposition by value, never by digest
//! alone. Pure readings over receipt slices — the lens law: a reading
//! never receipts, never mutates.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::opposition::{opposition_read, OppositionBreak};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::Receipt;
use cfe::world::{aperture_transfer, license_tech, Leap, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    world
}

/// The venture pair: a council-gated leap out and the exchange back
/// under one deal — the same arc world_venture measures in-harness,
/// here read as two INDEPENDENT books.
fn venture_pair() -> (World, World) {
    let mut grantor = sunlit("eridu-forge");
    let b = grantor.chart("midhold", FIRST_ORB).expect("charts");
    let c = grantor.chart("farhold", b).expect("charts");
    grantor.transfer(c, FIRST_ORB, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor.transfer(c, b, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor
        .seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 600, ratio(1, 1))
        .expect("the council stands");
    let mut receiver = sunlit("kepler-passage");
    receiver.zone(0, ZoneKind::Extractor).expect("zones");
    aperture_transfer(
        &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 20,
    )
    .expect("both cross");
    license_tech(
        &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
        Leap { condition: 3, name: "lattice doctrine".into(), deal: 21, council: 1 },
    )
    .expect("the leap licenses");
    aperture_transfer(
        &mut receiver, FIRST_ORB, &mut grantor, FIRST_ORB,
        Resource::Iron, ratio(3, 1), 21,
    )
    .expect("the exchange returns");
    (grantor, receiver)
}

#[test]
fn linked_deeds_sum_zero_across_two_books() {
    let (grantor, receiver) = venture_pair();
    let breaks = opposition_read(
        grantor.chronicle().receipts(),
        receiver.chronicle().receipts(),
    );
    assert_eq!(breaks, vec![], "every deal's legs oppose to exact zero — law 2 holds");
}

#[test]
fn a_forged_amount_measures_its_exact_residual() {
    let (grantor, receiver) = venture_pair();
    // The forger writes the right SHAPE with the wrong number: the
    // received leg of deal 21 claims 5 iron against the 3 sent. The
    // opposition measures the value, not the digest — the residual
    // reads WHOLE.
    let forged: Vec<Receipt> = grantor
        .chronicle()
        .receipts()
        .iter()
        .cloned()
        .map(|mut receipt| {
            if let ClosureEvent::ApertureReceived { deal: 21, amount, .. } =
                &mut receipt.event
            {
                *amount = ratio(5, 1);
            }
            receipt
        })
        .collect();
    let breaks = opposition_read(&forged, receiver.chronicle().receipts());
    assert_eq!(
        breaks,
        vec![OppositionBreak::ValueUnbalanced {
            deal: 21,
            resource: Resource::Iron,
            sent: ratio(3, 1),
            received: ratio(5, 1),
        }],
        "the forged value breaks by measurement — sent and received carried whole"
    );
}

#[test]
fn an_unpaired_leg_breaks_by_value_never_by_digest_alone() {
    let (grantor, receiver) = venture_pair();
    // A partial book: the receiver's copy lost deal 20's received
    // leg AND the leap's arrival — the opposition names both, each
    // with its whole reading.
    let partial: Vec<Receipt> = receiver
        .chronicle()
        .receipts()
        .iter()
        .filter(|receipt| {
            !matches!(&receipt.event, ClosureEvent::ApertureReceived { deal: 20, .. })
                && !matches!(&receipt.event, ClosureEvent::TechDiscovered { condition: 3, .. })
        })
        .cloned()
        .collect();
    let breaks = opposition_read(grantor.chronicle().receipts(), &partial);
    assert!(
        breaks.iter().any(|held| matches!(
            held,
            OppositionBreak::ValueUnbalanced { deal: 20, resource: Resource::Iron, received, .. }
                if *received == ratio(0, 1)
        )),
        "the missing value leg reads sent-without-received: {breaks:?}"
    );
    assert!(
        breaks.iter().any(|held| matches!(
            held,
            OppositionBreak::LeapUnpaired { deal: 21, issued: Some(_), arrived: None }
        )),
        "the missing leap arrival reads issued-without-arrived: {breaks:?}"
    );
}
