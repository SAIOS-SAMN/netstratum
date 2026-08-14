//! NS-1 wire witnesses: the orb codec round-trips byte-exact; the
//! ratio carrier is canonical; readers SKIP unknown tags (SEAL A).

use cfe::dop::{ClosureEvent, ColonyOrb, Resource, Zone, ZoneKind};
use cfe::ratio::{self, ratio};
use cfe::tlv;

fn an_orb() -> ColonyOrb {
    let mut orb = ColonyOrb {
        orb_id: 1,
        name: "meridian".into(),
        zones: vec![
            Zone { tile: 4, kind: ZoneKind::Extractor, base: None },
            Zone { tile: 9, kind: ZoneKind::Habitat, base: None },
        ],
        banked: Vec::new(),
        famine: vec![Resource::Biomass],
        ..ColonyOrb::default()
    };
    orb.bank(Resource::Iron, ratio(6, 1));
    orb.bank(Resource::Biomass, ratio(1, 1));
    orb
}

#[test]
fn the_orb_round_trips_byte_exact() {
    let orb = an_orb();
    let bytes = orb.wire_bytes();
    let decoded = ColonyOrb::from_wire(&bytes).expect("the canonical orb decodes");
    assert_eq!(decoded, orb, "value round trip");
    assert_eq!(decoded.wire_bytes(), bytes, "BYTE round trip");
}

#[test]
fn a_reader_skips_unknown_tags() {
    // SEAL A: a future engine appends a record this version has never
    // heard of; decoding still lands every known field.
    let orb = an_orb();
    let mut extended = orb.wire_bytes();
    tlv::put(200, b"a-future-field", &mut extended);
    let decoded = ColonyOrb::from_wire(&extended).expect("unknown tags skip, never crash");
    assert_eq!(decoded, orb);
}

#[test]
fn the_ratio_carrier_is_canonical() {
    // 2/4 and 1/2 are ONE value with ONE encoding; negatives and zero
    // round-trip; a zero denominator refuses.
    let mut half_reduced = Vec::new();
    let mut half_unreduced = Vec::new();
    ratio::encode(&ratio(1, 2), &mut half_reduced);
    ratio::encode(&ratio(2, 4), &mut half_unreduced);
    assert_eq!(half_reduced, half_unreduced, "one value, one encoding");
    for value in [ratio(0, 1), ratio(-3, 7), ratio(1000003, 20)] {
        let mut bytes = Vec::new();
        ratio::encode(&value, &mut bytes);
        let (decoded, consumed) = ratio::decode(&bytes).expect("canonical carriers decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, bytes.len());
    }
    let zero_denominator = [0u8, 1, 0, 0, 0, 1, 0, 0, 0, 0];
    assert!(ratio::decode(&zero_denominator).is_none(), "0 denominator refuses");
}

#[test]
fn events_round_trip_through_their_wire_records() {
    let events = [
        ClosureEvent::ColonyFounded { orb_id: 1, name: "meridian".into() },
        ClosureEvent::ZoneBuilt { orb_id: 1, tile: 4, kind: ZoneKind::Extractor, base: None },
        ClosureEvent::YieldCollected {
            orb_id: 1,
            resource: Resource::Iron,
            amount: ratio(3, 2),
        },
        ClosureEvent::UpkeepSettled {
            orb_id: 1,
            resource: Resource::Biomass,
            amount: ratio(3, 2),
        },
        ClosureEvent::FamineStruck {
            orb_id: 1,
            resource: Resource::Biomass,
            drawn: ratio(5, 1),
        },
        ClosureEvent::FamineRelieved { orb_id: 1, resource: Resource::Biomass },
        ClosureEvent::ZoneRazed { orb_id: 1, tile: 4 },
        // The union block (tags 39–53), consolidated here — one wire
        // law, one witness (operator ruling 2026-07-30: no per-landing
        // round-trip costumes).
        ClosureEvent::TechDiscovered {
            orb_id: 9,
            node: [7u8; 32],
            condition: 3,
            modifier: 2,
            magnitude: ratio(5, 4),
            name: "radiator doctrine".into(),
        },
        ClosureEvent::TechLicensed {
            deal: 7,
            from_orb: 1,
            council: 1,
            condition: 3,
            node: [9u8; 32],
            name: "radiator doctrine".into(),
        },
        ClosureEvent::GaugeShifted { gauge: ratio(1, 2) },
        ClosureEvent::Diverged { branch: "the-severed-chord".into(), gauge: ratio(2, 3) },
        ClosureEvent::Confluent { partner: "haven".into(), gauge: ratio(5, 6) },
        ClosureEvent::StarAged { class: 9 },
        ClosureEvent::EraFolded { era: 3, gauge: ratio(1, 4) },
        ClosureEvent::ApertureSent {
            deal: 7,
            from_orb: 1,
            resource: Resource::Joule,
            amount: ratio(5, 2),
        },
        ClosureEvent::ApertureReceived {
            deal: 7,
            to_orb: 2,
            resource: Resource::Joule,
            amount: ratio(5, 2),
        },
        ClosureEvent::CouncilSealed {
            council: 9,
            resource: Resource::Joule,
            quota: ratio(5, 2),
            window: 60,
            collateral: ratio(7, 3),
            members: vec![1, 2, 5],
        },
        ClosureEvent::CouncilClosed {
            council: 9,
            member: 5,
            fulfilled: false,
            banked: ratio(0, 1),
        },
        // E3.6: offer sides are Cargo — the Resource form round-trips
        // and an Element side rides the same tag.
        ClosureEvent::OfferPosted {
            offer: 7,
            orb_id: 1,
            give: cfe::dop::Cargo::Resource(Resource::Joule),
            give_amount: ratio(9, 2),
            want: cfe::dop::Cargo::Resource(Resource::Sink),
            want_amount: ratio(9, 2),
        },
        ClosureEvent::OfferPosted {
            offer: 8,
            orb_id: 1,
            give: cfe::dop::Cargo::Element([11u8; 32]),
            give_amount: ratio(3, 2),
            want: cfe::dop::Cargo::Resource(Resource::Iron),
            want_amount: ratio(4, 1),
        },
        // NOVELTY-GATE (tag 166): orbit ‖ cited ‖ opaque expansion.
        ClosureEvent::SolverWordMinted {
            orbit: [7u8; 32],
            expansion: vec![2, 0, 1, 4],
            cited: vec![[1u8; 32], [2u8; 32], [3u8; 32]],
        },
        // CONSILIENCE (tag 177): signature ‖ mass ‖ hardness.
        ClosureEvent::KnowledgeConverged { signature: [9u8; 32], mass: 3, hardness: 2 },
        ClosureEvent::OfferCleared { offer: 7, deal: 100 },
        ClosureEvent::GaugeShiftedAt { grade: 2, gauge: ratio(2, 3) },
        ClosureEvent::InstrumentForged { orb_id: 4, tech_hash: [3u8; 32], grade: 3 },
        ClosureEvent::RouteChartered { deal: 9, eta: cfe::dop::unit(3, 8) },
        ClosureEvent::VoidShare { orb_id: 1, deal: 9, resource: Resource::Iron, amount: ratio(13, 64) },
        ClosureEvent::RouteLapsed { deal: 9 },
        ClosureEvent::HandoffSealed { from: 1, to: 2 },
    ];
    for event in events {
        let bytes = event.wire_bytes();
        let (tag, value) = tlv::records(&bytes).next().expect("one record");
        let decoded = ClosureEvent::from_wire(tag, value).expect("known tags decode");
        assert_eq!(decoded, event);
        assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");
    }
}
