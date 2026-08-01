//! ELEMENT-GRAMMAR-1 witnesses (14 §2–§4, §7): the periodic table is
//! discovered, never typed. The mint deed reads what the genesis seed
//! always held (identity + properties hash-derived from receipt 0's
//! digest ‖ orb ‖ stratum — re-prospecting cannot re-roll); the assay
//! banks richness × elapsed in ONE receipt (the bond-settle shape);
//! the arbor binds the draw to the sky; two worlds mint DIFFERENT
//! elements from ONE grammar; replay MATCHES.

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn charted(name: &str, class: StarClass) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(class).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world
}

#[test]
fn the_mint_reads_the_seed_and_never_rerolls() {
    let mut world = charted("assay-hold", StarClass::G);
    let struck = world.prospect(FIRST_ORB, 1).expect("the stratum opens");
    assert!(struck.tier >= 1 && struck.tier <= 3, "tier drawn in range");
    assert!(struck.richness > ratio(0, 1), "richness is a positive exact rational");
    assert!(!struck.name.is_empty(), "named in the line's own voice");
    // Once per stratum: the second prospect refuses PURE.
    let receipts_before = world.chronicle().receipts().len();
    let refusal = world.prospect(FIRST_ORB, 1).expect_err("one stratum, one reading");
    assert_eq!(
        refusal,
        ActionRefused::StratumProspected { orb: FIRST_ORB, stratum: 1 }
    );
    assert_eq!(world.chronicle().receipts().len(), receipts_before);
    // A different stratum mints a DIFFERENT identity from the same
    // seed — the world's own diversity.
    let second = world.prospect(FIRST_ORB, 2).expect("the next stratum opens");
    assert_ne!(struck.element, second.element, "strata differ; identities differ");
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(replay(world.chronicle()), world.committed());
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries the registry");
}

#[test]
fn two_worlds_mint_different_elements_from_one_grammar() {
    // Different genesis roots, same grammar, same stratum: the seeds
    // differ, so the elements differ — content written by the
    // chronicle, never by us.
    let mut here = charted("first-sky", StarClass::G);
    let mut there = charted("other-sky", StarClass::G);
    let ours = here.prospect(FIRST_ORB, 1).expect("opens");
    let theirs = there.prospect(FIRST_ORB, 1).expect("opens");
    assert_ne!(ours.element, theirs.element, "two seeds, two identities");
    // And the SAME world re-derives identically through replay: the
    // receipt carries the drawn properties; replay reads, never rolls.
    assert_eq!(replay(here.chronicle()), here.committed());
    assert_eq!(replay(there.chronicle()), there.committed());
}

#[test]
fn the_spine_binds_the_draw_to_the_sky() {
    // The abundance arbor is exact: across many strata, every family
    // drawn under a class is one the arbor WEIGHTS for that class —
    // a zero-weighted family never yields (14 §4).
    let mut world = charted("arbor-read", StarClass::M);
    for stratum in 1..=8u8 {
        let struck = world.prospect(FIRST_ORB, stratum).expect("opens");
        assert!(
            cfe::dop::spine_weight(StarClass::M, struck.family) > ratio(0, 1),
            "family {} drawn under M must carry arbor weight",
            struck.family
        );
    }
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_assay_banks_richness_times_elapsed_in_one_receipt() {
    let mut world = charted("assay-hold", StarClass::G);
    let struck = world.prospect(FIRST_ORB, 1).expect("opens");
    let opened_at = world.now();
    for _ in 0..8 {
        world.tick();
    }
    let receipts_before = world.chronicle().receipts().len();
    let drawn = world.assay(FIRST_ORB, struck.element).expect("the vein pays");
    let elapsed = ratio((world.now() - opened_at) as i64, 1);
    assert_eq!(drawn, struck.richness.clone() * elapsed, "richness × elapsed, exact");
    assert_eq!(
        world.chronicle().receipts().len(),
        receipts_before + 1,
        "one receipt moves the whole span — the bond-settle shape"
    );
    assert_eq!(
        world.orb().expect("stands").element_amount(&struck.element),
        drawn,
        "banked element stores carry the draw"
    );
    // A second assay immediately banks only the new delta: zero.
    let refusal = world.assay(FIRST_ORB, struck.element).expect_err("no new span");
    assert_eq!(refusal, ActionRefused::NothingAccrued);
    // An unprospected identity refuses typed.
    let unknown = [7u8; 32];
    assert!(matches!(
        world.assay(FIRST_ORB, unknown),
        Err(ActionRefused::NoSuchElement { .. })
    ));
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_deed_round_trips_byte_exact() {
    let deed = ClosureEvent::ProspectStruck {
        orb_id: 1,
        stratum: 2,
        element: [9u8; 32],
        family: 3,
        tier: 2,
        richness: ratio(5, 4),
        name: "adamant vein".to_string(),
    };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 70);
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    let assay = ClosureEvent::AssayStruck {
        orb_id: 1,
        element: [9u8; 32],
        amount: ratio(10, 1),
    };
    let bytes = assay.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 71);
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), assay);
}
