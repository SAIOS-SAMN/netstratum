//! WAVE-1 DONE-BAR HARNESS (RELAY_WAVE1_IMPLEMENTOR §1).
//!
//! Pure-NPC era targets:
//!   1. refined elements exist          (REFINE-1 — this wave)
//!   2. an element route clears         (Wave 3 — ignored until E3)
//!   3. a STAMP spot clears             (Wave 4 — ignored until S4)
//!
//! Wave-1 exit criterion (T1.4): the conductor seats a T4 Caravan and
//! a T6 Exchange unaided. Built red-first; green when offices + refine
//! land under pure NPC cycles.

use cfe::dop::{ClosureEvent, Station};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::replay;
use conductor::{Polity, PolitySpec, Policy};
use ledger::registry::{tier_of, Civilization, Registry, Tier};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "wave1-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(
    name: &str,
    sigil: &str,
    disposition: &str,
    metabolism: &str,
) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: name.into(),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "the offices open under our own hand".into(),
            disposition: disposition.into(),
            metabolism: metabolism.into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hearth"),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position: (ratio(5, 1), ratio(5, 1), ratio(5, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

fn station_of(polity: &Polity) -> Option<&str> {
    polity.spec.civilization.station.as_deref()
}

fn element_kinds(polity: &Polity) -> usize {
    polity
        .world
        .orb()
        .map(|o| o.elements.len())
        .unwrap_or(0)
}

fn refined_kinds(polity: &Polity) -> usize {
    polity
        .world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::ElementRefined { .. }))
        .count()
}

/// T1.4 EXIT — pure NPC seats a Caravan (T4) and an Exchange (T6).
#[test]
fn pure_npc_seats_caravan_and_exchange() {
    let root = scratch("offices");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));

    // Expansionist → caravan; empiricist → exchange (office_step law).
    let mut caravan = Polity::genesis(
        line("Wayfarer", "wayfarer", "expansionist", "mineral"),
        &registry,
        &book,
    );
    let mut exchange = Polity::genesis(
        line("Broker", "broker", "empiricist", "biotic"),
        &registry,
        &book,
    );

    assert_eq!(station_of(&caravan), None, "starts unseated");
    assert_eq!(station_of(&exchange), None, "starts unseated");

    let mut caravan_on = None;
    let mut exchange_on = None;
    for beat in 1u32..=24 {
        let _ = caravan.cycle(&book, &registry);
        let _ = exchange.cycle(&book, &registry);
        if caravan_on.is_none() && station_of(&caravan) == Some("caravan") {
            caravan_on = Some(beat);
        }
        if exchange_on.is_none() && station_of(&exchange) == Some("exchange") {
            exchange_on = Some(beat);
        }
        if caravan_on.is_some() && exchange_on.is_some() {
            break;
        }
    }

    assert!(
        caravan_on.is_some(),
        "expansionist line must seat caravan under pure NPC; station={:?}",
        station_of(&caravan)
    );
    assert!(
        exchange_on.is_some(),
        "empiricist line must seat exchange under pure NPC; station={:?}",
        station_of(&exchange)
    );
    assert_eq!(tier_of(Some("caravan")), Some(Tier::T4));
    assert_eq!(tier_of(Some("exchange")), Some(Tier::T6));
    assert_eq!(
        Station::from_word("caravan").map(|s| s.as_wire()),
        Some(5)
    );
    assert_eq!(
        Station::from_word("exchange").map(|s| s.as_wire()),
        Some(7)
    );

    // Chain carries the seat (StationSeated), not only the registry card.
    let caravan_receipted = caravan.world.chronicle().receipts().iter().any(|r| {
        matches!(
            &r.event,
            ClosureEvent::StationSeated { station: 5, .. }
        )
    });
    let exchange_receipted = exchange.world.chronicle().receipts().iter().any(|r| {
        matches!(
            &r.event,
            ClosureEvent::StationSeated { station: 7, .. }
        )
    });
    assert!(caravan_receipted, "caravan seat must be receipted");
    assert!(exchange_receipted, "exchange seat must be receipted");

    // SEAL B on both chronicles.
    assert_eq!(
        replay(caravan.world.chronicle()),
        caravan.world.committed(),
        "caravan replay MATCHES"
    );
    assert_eq!(
        replay(exchange.world.chronicle()),
        exchange.world.committed(),
        "exchange replay MATCHES"
    );
}

/// REFINE-1 — pure NPC builds a refinery and converts raw elements.
#[test]
fn pure_npc_refines_elements() {
    let root = scratch("refine");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(
        line("Cruciblewright", "cruciblewright", "expansionist", "mineral"),
        &registry,
        &book,
    );

    let mut refined_on = None;
    for beat in 1u32..=48 {
        let _ = polity.cycle(&book, &registry);
        if refined_kinds(&polity) > 0 {
            refined_on = Some(beat);
            break;
        }
    }

    assert!(
        element_kinds(&polity) >= 1 || refined_kinds(&polity) > 0,
        "ground_step must bank elements before or as refine runs"
    );
    assert!(
        refined_on.is_some(),
        "pure NPC must refine at least once within 48 beats; \
         elements={} refined_receipts={}",
        element_kinds(&polity),
        refined_kinds(&polity)
    );

    let has_refinery = polity.world.orb().is_some_and(|o| {
        o.zones
            .iter()
            .any(|z| z.kind == cfe::dop::ZoneKind::ElementRefinery)
    });
    assert!(has_refinery, "ElementRefinery must stand for a refine to clear");

    assert_eq!(
        replay(polity.world.chronicle()),
        polity.world.committed(),
        "replay MATCHES after refine"
    );
}

/// Wave-3 target — element route clear. Stays ignored until E-ROUTE.
#[test]
#[ignore = "wave-3 E-ROUTE-1 — turns green when element_trade_step clears a route"]
fn pure_npc_element_route_clears() {
    panic!("not yet: element route under pure NPC");
}

/// Wave-4 target — STAMP spot clear. Stays ignored until S4/MM-1.
#[test]
#[ignore = "wave-4 S4/MM-1 — turns green when stamp_market_step clears a spot"]
fn pure_npc_stamp_spot_clears() {
    panic!("not yet: STAMP spot under pure NPC");
}
