//! **G2 ELEMENT-FED METABOLISM — THE READING LAYER** (ruled 2026-08-12).
//!
//! A minted metabolism value may eat a periodic ELEMENT rather than one
//! of the three closed resource carriers. The element rides the SAME
//! `VocabularyMinted` deed as any vocabulary — no new deed, no new gate
//! — so who may declare a stone-eating line is exactly who may mint at
//! all (the T0 standing gate), and the novelty gate judges it like any
//! other. These witnesses hold the reading layer: the wire round-trips
//! byte-exact and legacy-safe, the fold surfaces the ration, the door
//! refuses a ration off its axis, and the ration resolver returns an
//! ELEMENT where one is declared and the compiled carrier where none is.

use cfe::dop::{metabolic_ration, metabolic_ration_of, metabolic_harvest_of, name, ClosureEvent, Ration, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::vocabulary::{trading, Axis, CHANNEL_UPKEEP, CHANNEL_YIELD};
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// An element id seated at chosen periodic axes (the almanac fixture
/// shape) — a shallow stone the stone-eaters farm.
fn element(line: u8, lane: u8) -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = line - 1;
    id[4] = lane - 1;
    id
}

/// A world whose line holds a progenitor seat, ready to mint.
fn seated() -> World {
    let mut world = World::new();
    world.found("stone-eater-hold").expect("founds");
    world
        .seat_station(
            name("tunborn"),
            cfe::dop::Station::from_wire(1).expect("a band office"),
        )
        .expect("seats the progenitor");
    world
}

#[test]
fn a_minted_metabolism_may_declare_the_element_it_eats() {
    let mut world = seated();
    let stone = element(1, 3);
    let code = world
        .mint_vocabulary_ration_as(
            cfe::world::SOVEREIGN,
            "tunborn",
            Axis::Metabolism,
            "◈",
            name("lithovore"),
            trading(vec![(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))]),
            stone,
        )
        .expect("the progenitor mints a stone-eater");
    // A minted metabolism seats ABOVE the three compiled ones.
    assert!(code >= 4, "the compiled metabolisms are 1..=3; a mint is 4+");

    // The fold surfaces the ration on exactly the minted entry.
    let fold = cfe::vocabulary::fold(world.chronicle(), Axis::Metabolism);
    let minted = fold.iter().find(|e| e.code == code).expect("the mint stands in the fold");
    assert_eq!(minted.ration, Some(stone), "the fold carries the eaten element");
    assert!(
        fold.iter().filter(|e| e.code <= 3).all(|e| e.ration.is_none()),
        "the compiled metabolisms eat resources, never elements"
    );
}

#[test]
fn the_ration_resolver_returns_an_element_only_where_one_is_declared() {
    let stone = element(2, 4);
    // A compiled metabolism (no declared element) reads its closed carrier.
    assert_eq!(
        metabolic_ration_of(1, None),
        Ration::Resource(Resource::Iron),
        "mineral still eats iron"
    );
    assert_eq!(
        metabolic_ration_of(2, None),
        Ration::Resource(metabolic_ration(2)),
        "biotic still eats its v1 carrier"
    );
    // A minted element-eater reads the ELEMENT, bypassing the code map.
    assert_eq!(
        metabolic_ration_of(4, Some(stone)),
        Ration::Element(stone),
        "the stone-eater eats its stone"
    );
    // And it FARMS that same stone at the biotic rate — a stone-eating
    // line grows its stone the way a biotic line grows biomass.
    assert_eq!(
        metabolic_harvest_of(4, Some(stone)),
        (Ration::Element(stone), ratio(1, 1)),
        "the element-eater farms its own element"
    );
}

#[test]
fn a_ration_is_meaningful_only_on_the_metabolism_axis() {
    let mut world = seated();
    let stone = element(1, 2);
    // A ration on any other axis is a malformed declaration — the wire
    // could not tell it from a Zone base — so it dies at the door.
    let off_axis = world.mint_vocabulary_ration_as(
        cfe::world::SOVEREIGN,
        "tunborn",
        Axis::Class,
        "◇",
        name("probe"),
        trading(vec![(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))]),
        stone,
    );
    assert!(off_axis.is_err(), "a ration off the metabolism axis is refused");
}

#[test]
fn the_wire_round_trips_the_ration_and_legacy_mints_decode_unchanged() {
    let stone = element(3, 5);
    // The new form: a metabolism mint carrying its eaten element.
    let with_ration = ClosureEvent::VocabularyMinted {
        axis: Axis::Metabolism.code(),
        code: 4,
        sigil: "◈".into(),
        word: "lithovore".into(),
        binding: vec![(CHANNEL_YIELD, ratio(5, 4))],
        base: None,
        ration: Some(stone),
    };
    let bytes = with_ration.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 96);
    assert_eq!(
        ClosureEvent::from_wire(tag, value).expect("decodes"),
        with_ration,
        "the eaten element rides the wire byte-exact"
    );

    // LEGACY: a base-only (Zone) mint and a plain mint both still
    // round-trip — the trailing region reads by length, so 1 byte is a
    // base and 0 bytes is neither, exactly as before the ration landed.
    for legacy in [
        ClosureEvent::VocabularyMinted {
            axis: Axis::Zone.code(),
            code: 9,
            sigil: "▨".into(),
            word: "arcology".into(),
            binding: vec![(CHANNEL_YIELD, ratio(3, 2))],
            base: Some(1),
            ration: None,
        },
        ClosureEvent::VocabularyMinted {
            axis: Axis::Class.code(),
            code: 7,
            sigil: "✦".into(),
            word: "swarmlike".into(),
            binding: vec![(CHANNEL_UPKEEP, ratio(5, 4))],
            base: None,
            ration: None,
        },
    ] {
        let bytes = legacy.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert_eq!(
            ClosureEvent::from_wire(tag, value).expect("decodes"),
            legacy,
            "legacy mints decode unchanged"
        );
    }
}

// ── LAYER 2: the tick, where a stone-eater farms and eats its element ──

/// A world sealed to a MINTED element-metabolism (code 4) that eats
/// `stone`, with an extractor and a habitat standing. No star is
/// commissioned, so the thermal freeze never fires and the only thing
/// that can stop the flows is the element famine under test. `banked`
/// of the stone is laid down by receipt and the world resumed, so the
/// fixture is chain-true.
fn stone_eater(stone: [u8; 32], banked: cfe::ratio::Ratio) -> World {
    let mut world = World::new();
    world.found("stone-hold").expect("founds");
    world
        .seat_station(name("tunborn"), cfe::dop::Station::from_wire(1).expect("office"))
        .expect("seats the progenitor");
    let code = world
        .mint_vocabulary_ration_as(
            SOVEREIGN,
            "tunborn",
            Axis::Metabolism,
            "◈",
            name("lithovore"),
            trading(vec![(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))]),
            stone,
        )
        .expect("mints the stone-eater metabolism");
    // Seal the line to that metabolism (disposition/perception are seed
    // codes). EXPANSIONIST = 1, PLANAR = 1.
    world.seal_genesis("stone-line", 1, code, 1).expect("seals");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    if banked > ratio(0, 1) {
        let stamp = world.now();
        world.chronicle_mut().append(
            ClosureEvent::AssayStruck {
                orb_id: FIRST_ORB,
                element: stone,
                amount: banked,
                richness: None,
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    World::resume(world.chronicle().clone())
}

#[test]
fn a_fed_stone_eater_runs_and_keeps_its_biomass_economy_shut() {
    // A line holding its element meets the food bill, so it is NOT frozen
    // — its non-food flows run verbatim. (The depleting/farming element
    // ECONOMY is a separate landing, task #29; this pins that stasis
    // opens the gate and the biomass economy stays shut.)
    let stone = element(1, 3);
    let mut world = stone_eater(stone, ratio(100, 1));
    for _ in 0..4 {
        world.tick();
    }
    // The extractor still mines iron — production is not suspended.
    assert!(
        world.accrued().iter().any(|(resource, amount)| *resource == Resource::Iron
            && *amount > ratio(0, 1)),
        "a fed line runs its non-food flows: {:?}",
        world.accrued()
    );
    // The FOOD SLOT is element, so BIOMASS never accrues — a stone-eater
    // has no biomass economy at all.
    assert!(
        world.accrued().iter().all(|(resource, _)| *resource != Resource::Biomass),
        "an element-eater touches no biomass: {:?}",
        world.accrued()
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_starved_stone_eater_freezes_whole_and_holds_its_structures() {
    // No stone banked — the food bill cannot be met, so the line is in
    // STASIS: every flow suspended.
    let stone = element(1, 3);
    let mut world = stone_eater(stone, ratio(0, 1));
    let zones_before = world.orb().expect("stands").zones.len();
    for _ in 0..4 {
        world.tick();
    }
    // Production is suspended — no iron mined, no flow of any kind.
    assert!(
        world.accrued().is_empty()
            || world.accrued().iter().all(|(_, amount)| *amount == ratio(0, 1)),
        "a stasis'd world produces nothing: {:?}",
        world.accrued()
    );
    // But the structures HOLD — stasis is not collapse.
    assert_eq!(
        world.orb().expect("stands").zones.len(),
        zones_before,
        "core structures hold through stasis"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn stasis_lifts_the_moment_the_element_bank_is_refilled() {
    // The famine is a per-beat CONDITION, not a stored strike: a starved
    // line that receives its element runs again the very next beat —
    // "requires external trade or new internal minting to resolve."
    let stone = element(1, 3);
    let mut world = stone_eater(stone, ratio(0, 1));
    world.tick();
    assert!(
        world.accrued().iter().all(|(_, amount)| *amount == ratio(0, 1)),
        "frozen while empty"
    );
    // A gift of stone arrives (an assay, by receipt), and the world is
    // resumed to fold it in.
    let stamp = world.now();
    world.chronicle_mut().append(
        ClosureEvent::AssayStruck {
            orb_id: FIRST_ORB,
            element: stone,
            amount: ratio(100, 1),
            richness: None,
        },
        stamp,
        vec![SOVEREIGN],
    );
    let mut world = World::resume(world.chronicle().clone());
    world.tick();
    assert!(
        world.accrued().iter().any(|(resource, amount)| *resource == Resource::Iron
            && *amount > ratio(0, 1)),
        "the refilled line runs again: {:?}",
        world.accrued()
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
