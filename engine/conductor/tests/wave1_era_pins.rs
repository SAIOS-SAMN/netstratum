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
use conductor::{element_trade_step, fleet_step, stamp_market_step, Polity, PolitySpec, Policy};
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

/// REFINE-1 — pure NPCs build refineries and convert raw elements.
///
/// **ONT-2b re-pin.** This was a SOLO extractive line that, under ING-2b,
/// refined its geometrically-banked STAPLE at ~beat 91. ONT-2b makes the
/// staple food-only — the extractive line now EATS its main vein
/// (`StapleMined`) instead of banking it, and its small non-staple veins are
/// drained by trace maintenance, so an ISOLATED element-eater no longer
/// accumulates an industrial surplus (measured: solo lines across all three
/// metabolisms refine 0× in 110 beats). Refining is now a NETWORKED / richer-
/// world property: at era scale the diverse roster (Resource-eaters whose
/// element veins are wholly industrial + cross-polity element trade + worlds
/// with many veins) sustains it — measured 2 refining lines / 13 refineries /
/// banks up to 20 element kinds over a seated 10-line, 95-beat era. So the
/// done-bar is witnessed at era scale, early-stopped the instant the first
/// line refines (below the ONT-4 leisure horizon ~b117 that this does not
/// exercise), and every refining line still replays byte-exact (SEAL B).
#[test]
fn pure_npc_refines_elements() {
    let root = scratch("refine");
    let refining = |worlds: &[Polity]| -> bool {
        worlds.iter().any(|p| refined_kinds(p) > 0)
    };
    // A seated 10-line era, capped at 100 beats, STOPPING the instant any line
    // refines — the condition-driven era pattern, cutting the dead tail.
    let worlds = conductor::run_era_until_among(&root.to_string_lossy(), 100, 10, refining);

    let refiners: Vec<&Polity> = worlds.iter().filter(|p| refined_kinds(p) > 0).collect();
    assert!(
        !refiners.is_empty(),
        "pure NPCs must refine at least once at era scale (networked/richer-world industry)"
    );
    // A refine only clears where an ElementRefinery stands, and every line that
    // refined replays byte-exact — the refine wire is SEAL-B-safe end to end.
    for p in &refiners {
        let has_refinery = p.world.orb().is_some_and(|o| {
            o.zones.iter().any(|z| z.kind == cfe::dop::ZoneKind::ElementRefinery)
        });
        assert!(has_refinery, "ElementRefinery must stand for a refine to clear");
        assert_eq!(
            replay(p.world.chronicle()),
            p.world.committed(),
            "a refining line replays MATCHES (SEAL B across the refine)"
        );
    }
}

/// Wave-2 exit — pure NPC element aperture clear (E-APT + E3.3).
///
/// Two mineral lines bank elements via ground_step, then
/// `element_trade_step` hauls open matter under pair_melt. A clear is
/// `ElementApertureReceived` on the importer's chronicle.
#[test]
fn pure_npc_element_route_clears() {
    let root = scratch("eapt");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut exporter = Polity::genesis(
        line("Veinexport", "veinexport", "expansionist", "mineral"),
        &registry,
        &book,
    );
    // Offset on the chart so melt is positive but < 1 (not walled).
    let mut importer_spec = line("Veinimport", "veinimport", "empiricist", "mineral");
    importer_spec.position = (ratio(8, 1), ratio(0, 1), ratio(0, 1));
    let mut importer = Polity::genesis(importer_spec, &registry, &book);

    let mut cleared_on = None;
    for beat in 1u32..=24 {
        let _ = exporter.cycle(&book, &registry);
        let _ = importer.cycle(&book, &registry);
        let mut pair = vec![exporter, importer];
        fleet_step(&mut pair);
        let hits = element_trade_step(&mut pair, u64::from(beat));
        importer = pair.pop().expect("importer");
        exporter = pair.pop().expect("exporter");
        let received = importer.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::ElementApertureReceived { .. })
        }) || exporter.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::ElementApertureReceived { .. })
        });
        if received || !hits.is_empty() {
            cleared_on = Some(beat);
            break;
        }
    }
    assert!(
        cleared_on.is_some(),
        "pure NPC must clear an element aperture within 24 beats; \
         exporter_elems={} importer_elems={}",
        element_kinds(&exporter),
        element_kinds(&importer),
    );
    let received = importer.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::ElementApertureReceived { .. })
    }) || exporter.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::ElementApertureReceived { .. })
    });
    assert!(received, "ElementApertureReceived must stand after trade");
    let sent = exporter.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::ElementApertureSent { .. })
    }) || importer.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::ElementApertureSent { .. })
    });
    assert!(sent, "ElementApertureSent must stand after trade");
    assert_eq!(
        replay(exporter.world.chronicle()),
        exporter.world.committed(),
        "exporter replay MATCHES"
    );
    assert_eq!(
        replay(importer.world.chronicle()),
        importer.world.committed(),
        "importer replay MATCHES"
    );
}

/// Wave-4 exit — pure NPC STAMP mint + spot clear (S4 / MM-1).
///
/// Discovery mints crystal; `stamp_market_step` converts half surplus
/// to STAMP 1:1 and clears STAMP for cargo (crystal/iron/biomass)
/// across an unwalled pair. Witness: `StampMinted` + aperture legs,
/// SEAL B on both books.
#[test]
fn pure_npc_stamp_spot_clears() {
    let root = scratch("stamp");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut buyer = Polity::genesis(
        line("Stampbuyer", "stampbuyer", "empiricist", "mineral"),
        &registry,
        &book,
    );
    let mut seller_spec = line("Stampseller", "stampseller", "expansionist", "mineral");
    seller_spec.position = (ratio(8, 1), ratio(0, 1), ratio(0, 1));
    let mut seller = Polity::genesis(seller_spec, &registry, &book);

    let mut cleared_on = None;
    for beat in 1u32..=96 {
        let _ = buyer.cycle(&book, &registry);
        let _ = seller.cycle(&book, &registry);
        let mut pair = vec![buyer, seller];
        fleet_step(&mut pair);
        let hits = stamp_market_step(&mut pair, u64::from(beat));
        seller = pair.pop().expect("seller");
        buyer = pair.pop().expect("buyer");
        let minted = buyer.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::StampMinted { .. })
        }) || seller.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::StampMinted { .. })
        });
        let received = buyer.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::StampApertureReceived { .. })
        }) || seller.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::StampApertureReceived { .. })
        });
        if minted && (received || !hits.is_empty()) {
            cleared_on = Some(beat);
            break;
        }
    }
    assert!(
        cleared_on.is_some(),
        "pure NPC must mint STAMP and clear a spot within 96 beats; \
         buyer_stamp={:?} seller_stamp={:?} buyer_crystal={:?} seller_crystal={:?}",
        buyer.world.orb().map(|o| o.stamp_amount()),
        seller.world.orb().map(|o| o.stamp_amount()),
        buyer.world.orb().map(|o| o.banked_amount(cfe::dop::Resource::Crystal)),
        seller.world.orb().map(|o| o.banked_amount(cfe::dop::Resource::Crystal)),
    );
    let minted = buyer.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::StampMinted { .. })
    }) || seller.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::StampMinted { .. })
    });
    assert!(minted, "StampMinted must stand after mint phase");
    let sent = buyer.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::StampApertureSent { .. })
    }) || seller.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::StampApertureSent { .. })
    });
    let received = buyer.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::StampApertureReceived { .. })
    }) || seller.world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::StampApertureReceived { .. })
    });
    assert!(sent, "StampApertureSent must stand after spot");
    assert!(received, "StampApertureReceived must stand after spot");
    assert_eq!(
        replay(buyer.world.chronicle()),
        buyer.world.committed(),
        "buyer replay MATCHES"
    );
    assert_eq!(
        replay(seller.world.chronicle()),
        seller.world.committed(),
        "seller replay MATCHES"
    );
}


/// Phase-3 exit — pure NPC fleets salvage, mint, and capacity-gate hauls.
#[test]
fn pure_npc_fleet_hauls() {
    let root = scratch("fleet");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut a = Polity::genesis(
        line("Fleetwright", "fleetwright", "expansionist", "mineral"),
        &registry,
        &book,
    );
    let mut b_spec = line("Fleetpeer", "fleetpeer", "empiricist", "mineral");
    b_spec.position = (ratio(8, 1), ratio(0, 1), ratio(0, 1));
    let mut b = Polity::genesis(b_spec, &registry, &book);

    let mut hull_on = None;
    let mut haul_on = None;
    // Pure-fleet path: cycle for iron, fleet_step salvage/mint, trade.
    for beat in 1u32..=24 {
        let _ = a.cycle(&book, &registry);
        let _ = b.cycle(&book, &registry);
        let mut pair = vec![a, b];
        fleet_step(&mut pair);
        let _ = element_trade_step(&mut pair, u64::from(beat));
        b = pair.pop().expect("b");
        a = pair.pop().expect("a");
        if hull_on.is_none() && !a.world.hulls().is_empty() {
            hull_on = Some(beat);
        }
        let hauled = a.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::ElementApertureSent { .. } | ClosureEvent::ApertureSent { .. })
        }) || b.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::ElementApertureSent { .. } | ClosureEvent::ApertureSent { .. })
        });
        if hull_on.is_some() && hauled {
            haul_on = Some(beat);
            break;
        }
        if hull_on.is_some() && haul_on.is_none() {
            // Hulls stand — haul may need another beat of element banking.
            haul_on = haul_on.or(None);
        }
    }
    assert!(
        hull_on.is_some(),
        "NPC must mint a hull within 24 beats; hulls_a={} scrap_a={:?} iron={:?}",
        a.world.hulls().len(),
        a.world.orb().map(|o| o.scrap_amount()),
        a.world.orb().map(|o| o.banked_amount(cfe::dop::Resource::Iron)),
    );
    assert!(
        haul_on.is_some(),
        "with hulls, a cross-world haul must clear; capacity={}",
        a.world.free_capacity(0),
    );
    assert_eq!(
        replay(a.world.chronicle()),
        a.world.committed(),
        "SEAL B after fleet"
    );
}
