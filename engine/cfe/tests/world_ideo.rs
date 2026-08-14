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
//! generate exactly zero. The engine gates nothing.

use cfe::dop::{positive, unit, cohesion_ratio, ClosureEvent, ColonyOrb, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
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
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 2), None, None)
        .expect("sites hot");
    world
        .site_planet(charted, unit(0, 1), positive(100, 1), None, None)
        .expect("sites cold");
    world.tick();
    world.collect().expect("banks the joules");
    world
        .transfer(cfe::dop::between(FIRST_ORB, charted), Resource::Joule, cfe::dop::positive(1, 1))
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
    orb.forwards_fulfilled = ratio(3, 1);
    orb.forwards_breached = ratio(3, 4);
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
/// (reading 1) pays φ × 1 × volume = 1/4, accrued against the mover
/// and sealed at its collect.
///
/// **AMENDED 2026-08-03 — ENCASEMENT FIRST now governs where it goes**
/// (`EQUATIONS` T3; operator ruling: friction entropy is
/// thermodynamically identical to thermal entropy, so the same capture
/// law applies). The PRICE is unchanged and still exact; what changed
/// is the destination. This fixture's furnace carries one Encasement
/// shell — throughput `2 × 1 = 2` — so the whole 1/4 is CAPTURED and
/// nothing vents.
///
/// This pin previously asserted a `FrictionSettled` receipt existed,
/// which encoded the old law: friction went 100% to the void while
/// writing into the very ledger whose own law puts capture FIRST. The
/// assertion is not weakened here — it is turned around to hold BOTH
/// arms, which is the stronger claim: `friction = captured + vented`,
/// each arm receipted, and the total still exactly `1/4`.
#[test]
fn friction_prices_divergence_exactly() {
    let (mut world, charted) = furnace_and_coldstore();
    world.absorb(cfe::dop::between(FIRST_ORB, charted), cfe::dop::positive(1, 1)).expect("absorbs"); // debt @19
    for _ in 0..9 {
        world.tick(); // the window lapses at now=27: extracted = 1
    }
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 1)
    );

    // The extractor (reading 0) moves 2 iron to the fresh reader (1):
    // the friction now prices BOTH the ideological divergence (1) and the
    // #50 physical LOGISTICS distance (the normalized radial separation of
    // the two grain-quantized orbits). MEASURED at 333/536 — up from the
    // pre-logistics 1/4, the added transit cost captured whole.
    world
        .transfer(cfe::dop::between(FIRST_ORB, charted), Resource::Iron, cfe::dop::positive(2, 1))
        .expect("transfers across the divide");
    let vented_before = world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented.clone();
    let joules_before = world
        .orb_by_id(FIRST_ORB)
        .expect("orb")
        .banked_amount(Resource::Joule);
    world.collect().expect("collects — the seal");

    // THE ENTROPIC ARM IS EMPTY, and that is the ruling landing. One
    // shell gives throughput 2; the friction is 1/4; `min(1/4, 2)` is
    // the whole of it, so the overflow is zero and NO `FrictionSettled`
    // is written at all.
    let vented_arm = world.chronicle().receipts().iter().find_map(|r| match &r.event {
        ClosureEvent::FrictionSettled { orb_id, amount } if *orb_id == FIRST_ORB => {
            Some(amount.clone())
        }
        _ => None,
    });
    assert_eq!(
        vented_arm, None,
        "a shell that can encase 2 absorbs 1/4 whole — nothing overflows to the void"
    );

    // THE SYNTROPIC ARM CARRIES IT. Captured friction banks joules and
    // rides the existing YieldCollected receipt — the same arm captured
    // HEAT already uses, which is why this needed no new deed.
    //
    // It carries its OWN receipt rather than merging into the thermal
    // yield: merged, the friction capture would be unattributable, and
    // I6 does not let a difference die without account. So the BANK
    // DELTA IS NOT THE MEASURE — it also holds ten ticks of encased
    // heat (167/8 here) — and the receipt is.
    let joule_yields: Vec<Ratio> = world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|r| match &r.event {
            ClosureEvent::YieldCollected { orb_id, resource: Resource::Joule, amount }
                if *orb_id == FIRST_ORB =>
            {
                Some(amount.clone())
            }
            _ => None,
        })
        .collect();
    assert!(
        joule_yields.contains(&ratio(333, 536)),
        "the price is EXACT — phi x (divergence + logistics) x volume = 333/536 — captured whole. joule yields seen: {joule_yields:?}"
    );
    let joules_after = world
        .orb_by_id(FIRST_ORB)
        .expect("orb")
        .banked_amount(Resource::Joule);
    assert!(
        joules_after > joules_before,
        "captured friction reaches the bank, never the void"
    );

    // AND THE VENTED LEDGER DID NOT GROW BY THE FRICTION. It may still
    // grow by the thermal seal, which is a different arm on the same
    // book; what must not happen is friction reaching the void while a
    // shell stood idle.
    let source = world.orb_by_id(FIRST_ORB).expect("orb");
    let vent_receipts = world.chronicle().receipts().iter().fold(
        ratio(0, 1),
        |acc, r| match &r.event {
            ClosureEvent::VentSettled { orb_id, amount } if *orb_id == FIRST_ORB => {
                acc + amount.clone()
            }
            _ => acc,
        },
    );
    assert_eq!(
        source.entropy_vented, vent_receipts,
        "EVERY unit in the vented ledger is a VentSettled — no friction reached the void while a shell stood idle"
    );
    let _ = vented_before;

    let rebuilt = replay(world.chronicle());
    assert_eq!(
        rebuilt,
        world.committed(),
        "replay MATCHES — the receipt names the vented part, so replay          cannot vent a captured half a second time"
    );
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
        .transfer(cfe::dop::between(FIRST_ORB, charted), Resource::Iron, cfe::dop::positive(5, 1))
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

/// #50: the logistics distance is a bounded, normalized radial separation
/// — zero between an orb and itself (or co-orbital / unsited partners),
/// and strictly between 0 and 1 for orbits truly apart. The moved value
/// itself stays conserved; logistics is a separate transit burn.
#[test]
fn logistics_distance_is_bounded_and_zero_for_co_orbital() {
    let (world, charted) = furnace_and_coldstore();
    assert_eq!(
        world.logistics_distance(FIRST_ORB, FIRST_ORB),
        ratio(0, 1),
        "an orb is zero transit from itself"
    );
    let d = world.logistics_distance(FIRST_ORB, charted);
    assert!(
        d > ratio(0, 1) && d < ratio(1, 1),
        "orbits truly apart carry a bounded transit cost in [0,1): {d}"
    );
}
