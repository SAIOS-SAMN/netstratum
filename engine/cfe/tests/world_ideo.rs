//! IDEO-1 — THE COHESION SURFACE + IDEOLOGICAL FRICTION (09 §2.5,
//! §3.2, BUILD_ORDER §3). The final ideology-board landing opens NO
//! books — it reads them: the eight committed counters enter the orb
//! wire (tag 41, emitted only when a book is nonzero — a zero-book
//! orb encodes byte-identical to a pre-ideology one), the four ratios
//! DERIVE at read (never stored), and ideological friction prices
//! DIVERGENCE, not vice: `ΔS_ideo = φ × abs(reciprocity_A −
//! reciprocity_B) × volume` accrues as workspace against the mover
//! and seals by `FrictionSettled` (tag 38, the VentSettled shape)
//! into the vented ledger. Aligned partners — ANY alignment —
//! generate exactly zero. The engine judges nothing.

use cfe::dop::{cohesion_ratio, ClosureEvent, ColonyOrb, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// The furnace↔coldstore fixture (the ABSORB-1 shape): ends at stamp
/// 19 with the source holding 1 un-sealed pending overflow (struck)
/// and the charted banking sink.
fn furnace_and_coldstore() -> (World, u64) {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let charted = world.chart("coldstore", FIRST_ORB).expect("charts");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world
        .zone_as(SOVEREIGN, charted, 0, ZoneKind::CryoArray)
        .expect("zones the array");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites hot");
    world
        .site_planet(charted, ratio(0, 1), ratio(100, 1), None, None)
        .expect("sites cold");
    world.tick();
    world.collect().expect("banks the joules");
    world
        .transfer(FIRST_ORB, charted, Resource::Joule, ratio(1, 1))
        .expect("relieves the freeze");
    world.tick();
    world.tick();
    world
        .collect_as(SOVEREIGN, charted)
        .expect("banks the sink");
    (world, charted)
}

/// The orb cohesion entry (tag 41): a zero-book orb emits NO entry —
/// byte-identical to a pre-ideology orb — and a written ledger
/// round-trips whole.
#[test]
fn the_cohesion_entry_round_trips_and_zero_books_encode_nothing() {
    let mut orb = ColonyOrb {
        orb_id: 7,
        name: "ledger".into(),
        ..ColonyOrb::default()
    };
    // All books zero: no tag-41 record anywhere.
    assert!(
        cfe::tlv::records(&orb.wire_bytes()).all(|(tag, _)| tag != 41),
        "the empty ledger costs no bytes"
    );
    orb.given_in_crisis = ratio(3, 2);
    orb.drawn_in_crisis = ratio(1, 1);
    orb.bond_flow = ratio(15, 1);
    orb.severance_load = ratio(2, 1);
    orb.covenants_fulfilled = ratio(3, 1);
    orb.covenants_breached = ratio(3, 4);
    orb.returned_across = ratio(1, 2);
    orb.extracted_across = ratio(1, 4);
    let bytes = orb.wire_bytes();
    assert!(cfe::tlv::records(&bytes).any(|(tag, _)| tag == 41));
    let decoded = ColonyOrb::from_wire(&bytes).expect("decodes");
    assert_eq!(decoded, orb, "the eight books round-trip byte-exact");
    assert_eq!(decoded.wire_bytes(), bytes);
}

/// The axis reading derives at read — never stored: syntropic /
/// (syntropic + entropic), and the empty book reads as no deed at all.
#[test]
fn the_ratio_derives_and_never_stores() {
    assert_eq!(cohesion_ratio(&ratio(3, 2), &ratio(0, 1)), Some(ratio(1, 1)));
    assert_eq!(cohesion_ratio(&ratio(1, 1), &ratio(1, 1)), Some(ratio(1, 2)));
    assert_eq!(cohesion_ratio(&ratio(0, 1), &ratio(2, 1)), Some(ratio(0, 1)));
    assert_eq!(cohesion_ratio(&ratio(0, 1), &ratio(0, 1)), None);
}

/// Friction prices divergence EXACTLY: a measured extractor (lapsed
/// window → reciprocity 0) trading with an unmeasured partner
/// (reading 1) pays φ × 1 × volume — accrued against the mover,
/// sealed at its collect into the vented ledger, receipted.
#[test]
fn friction_prices_divergence_exactly() {
    let (mut world, charted) = furnace_and_coldstore();
    world.absorb(FIRST_ORB, charted, ratio(1, 1)).expect("absorbs"); // debt @19
    for _ in 0..9 {
        world.tick(); // the window lapses at now=27: extracted = 1
    }
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 1)
    );

    // The extractor (reading 0) moves 2 iron to the fresh reader (1):
    // ΔS_ideo = 1/8 × 1 × 2 = 1/4, on the mover.
    world
        .transfer(FIRST_ORB, charted, Resource::Iron, ratio(2, 1))
        .expect("transfers across the divide");
    let vented_before = world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented.clone();
    world.collect().expect("collects — the seal");
    let sealed = world
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            ClosureEvent::FrictionSettled { orb_id, amount } if *orb_id == FIRST_ORB => {
                Some(amount.clone())
            }
            _ => None,
        })
        .expect("the friction is receipted");
    assert_eq!(sealed, ratio(1, 4), "φ × divergence × volume, exact");
    let source = world.orb_by_id(FIRST_ORB).expect("orb");
    // The vented ledger grew by the thermal seal AND the friction.
    assert!(source.entropy_vented > vented_before);
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the friction");
}

/// Aligned partners — any alignment, here two unmeasured orbs both
/// reading 1 — generate exactly ZERO friction: no receipt exists.
#[test]
fn aligned_partners_generate_exactly_zero() {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let charted = world.chart("outpost", FIRST_ORB).expect("charts");
    world
        .transfer(FIRST_ORB, charted, Resource::Iron, ratio(5, 1))
        .expect("transfers");
    for _ in 0..2 {
        world.tick();
    }
    world.collect().expect("collects");
    let frictions = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::FrictionSettled { .. }))
        .count();
    assert_eq!(frictions, 0, "alignment costs nothing — the engine judges nothing");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented,
        ratio(0, 1)
    );
}

/// Tag 38 round-trips byte-exact (the VentSettled shape).
#[test]
fn the_friction_event_round_trips_byte_exact() {
    let event = ClosureEvent::FrictionSettled { orb_id: 3, amount: ratio(1, 4) };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 38);
    let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
    assert_eq!(decoded, event);
    assert_eq!(decoded.wire_bytes(), bytes);
}
