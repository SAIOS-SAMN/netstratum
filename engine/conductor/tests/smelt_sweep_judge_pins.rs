//! **#136 — THE SMELT-SWEEP PASSES THE KERNEL JUDGE** (the seam SEAL-B did not
//! catch). SEAL-B proves `replay == committed` (state equality); it does NOT run
//! the kernel's element-conservation JUDGE (`kernel::admits` -> `element_move`),
//! which reconstructs each element's balance from the deed's DECLARED per-element
//! deltas and refuses a mismatch (`ElementBroken`). The migration deed
//! (`ElementStocksSwept`) carries only a scalar dust, so without a judge arm the
//! declared delta is empty while the stocks floored -> `after != before` -> every
//! world refused (measured live: 68 ElementBroken, one per world).
//!
//! This witness drives the judge exactly as the conductor does — `prior =
//! replay(receipts[..at])`, `next = replay(receipts[..=at])`, `admits(prior, next,
//! step)` — over a resumed, swept chronicle, and asserts EVERY step is lawful,
//! including the sweep. The judge re-derives the floor deltas from `prior`
//! (call-not-copy), mirroring `sweep_element_grain`, so conservation holds.

use cfe::dop::{positive, unit, ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::Chronicle;
use cfe::world::{World, FIRST_ORB};

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

fn bank_element(world: &mut World) -> [u8; 32] {
    let struck = world.prospect(FIRST_ORB, 1).expect("surface opens");
    for _ in 0..8 {
        world.tick();
    }
    world.assay(FIRST_ORB, struck.element).expect("surface banks");
    struck.element
}

/// Every step of a resumed, swept chronicle is lawful under the kernel judge —
/// the sweep deed included. Without the `ElementStocksSwept` arm in `element_move`
/// this fails with `ElementBroken` on the sweep step.
#[test]
fn the_smelt_sweep_is_lawful_under_the_kernel_judge() {
    let mut world = sunlit("judge-sweep");
    let element = bank_element(&mut world);
    let orb_id = world.orb().expect("orb").orb_id;
    let stamp = world.now() + 1;
    // A raw legacy fill: an un-smelted third banked straight onto the ledger — the
    // pre-ruling-B shape that drifts the stock off the grain.
    world.chronicle_mut().append(
        ClosureEvent::ElementApertureReceived { deal: 7, to_orb: orb_id, element, amount: ratio(1, 3) },
        stamp,
        vec![1],
    );

    // Resume floors the off-grain stock and appends the ElementStocksSwept deed.
    let resumed = World::resume(world.chronicle().clone());

    // Confirm the sweep deed actually landed (else the witness proves nothing).
    let receipts = resumed.chronicle().receipts().to_vec();
    let sweep_at = receipts
        .iter()
        .position(|r| matches!(r.event(), ClosureEvent::ElementStocksSwept { .. }))
        .expect("resume minted the sweep deed");

    // Drive the judge over EVERY step, exactly as the conductor does.
    let mut judged = 0u64;
    for at in 0..receipts.len() {
        let prior = cfe::world::replay(
            &Chronicle::from_receipts(receipts[..at].to_vec()).expect("prefix decodes"),
        );
        let next = cfe::world::replay(
            &Chronicle::from_receipts(receipts[..=at].to_vec()).expect("prefix decodes"),
        );
        let receipt = &receipts[at];
        let step = kernel::Step {
            event: receipt.event().clone(),
            actor: receipt.provenance().first().copied().unwrap_or(cfe::world::SOVEREIGN),
            stamp: receipt.stamp(),
        };
        kernel::admits(&prior, &next, &step).unwrap_or_else(|refused| {
            panic!("step {at} ({:?}) refused by the judge: {refused:?}", receipt.event())
        });
        judged += 1;
    }
    assert!(sweep_at < receipts.len(), "the sweep step was among those judged");
    eprintln!("#136 judge: {judged} steps lawful, sweep at {sweep_at} passed (0 ElementBroken)");
}

/// #138 — THE RESOURCE SMELT-SWEEP PASSES THE KERNEL JUDGE (the banked twin of
/// the element seam above). A raw legacy aperture banks an off-grain 1/3 Crystal;
/// resume floors it and appends `ResourceStocksSwept`. The banked-conservation
/// judge (`admits` -> `conservation` -> `admitted_deltas`) reconstructs each
/// resource's total from the deed's DECLARED per-resource deltas and refuses a
/// mismatch (`ConservationBroken`). Without the `ResourceStocksSwept` arm the
/// declared delta is empty while the stock floored -> every world refused. This
/// drives the judge over every step and asserts the sweep is admitted — the
/// resource analog of #136's 68->0 seam.
#[test]
fn the_resource_smelt_sweep_is_lawful_under_the_kernel_judge() {
    let mut world = sunlit("judge-resource-sweep");
    let orb_id = world.orb().expect("orb").orb_id;
    let stamp = world.now() + 1;
    // A raw legacy fill: an un-smelted third of Crystal banked straight onto the
    // ledger — the pre-#138 aperture shape that drifts the stock off the grain.
    world.chronicle_mut().append(
        ClosureEvent::ApertureReceived {
            deal: 7,
            to_orb: orb_id,
            resource: cfe::dop::Resource::Crystal,
            amount: ratio(1, 3),
        },
        stamp,
        vec![1],
    );

    // Resume floors the off-grain stock and appends the ResourceStocksSwept deed.
    let resumed = World::resume(world.chronicle().clone());
    let receipts = resumed.chronicle().receipts().to_vec();
    let sweep_at = receipts
        .iter()
        .position(|r| matches!(r.event(), ClosureEvent::ResourceStocksSwept { .. }))
        .expect("resume minted the resource sweep deed");

    let mut judged = 0u64;
    for at in 0..receipts.len() {
        let prior = cfe::world::replay(
            &Chronicle::from_receipts(receipts[..at].to_vec()).expect("prefix decodes"),
        );
        let next = cfe::world::replay(
            &Chronicle::from_receipts(receipts[..=at].to_vec()).expect("prefix decodes"),
        );
        let receipt = &receipts[at];
        let step = kernel::Step {
            event: receipt.event().clone(),
            actor: receipt.provenance().first().copied().unwrap_or(cfe::world::SOVEREIGN),
            stamp: receipt.stamp(),
        };
        kernel::admits(&prior, &next, &step).unwrap_or_else(|refused| {
            panic!("step {at} ({:?}) refused by the judge: {refused:?}", receipt.event())
        });
        judged += 1;
    }
    assert!(sweep_at < receipts.len(), "the resource sweep step was among those judged");
    eprintln!("#138 judge: {judged} steps lawful, resource sweep at {sweep_at} passed (0 ConservationBroken)");
}
