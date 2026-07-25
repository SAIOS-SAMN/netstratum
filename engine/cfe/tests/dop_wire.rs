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
            Zone { tile: 4, kind: ZoneKind::Extractor },
            Zone { tile: 9, kind: ZoneKind::Habitat },
        ],
        banked: Vec::new(),
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
        ClosureEvent::ZoneBuilt { orb_id: 1, tile: 4, kind: ZoneKind::Extractor },
        ClosureEvent::YieldCollected {
            orb_id: 1,
            resource: Resource::Iron,
            amount: ratio(3, 2),
        },
    ];
    for event in events {
        let bytes = event.wire_bytes();
        let (tag, value) = tlv::records(&bytes).next().expect("one record");
        let decoded = ClosureEvent::from_wire(tag, value).expect("known tags decode");
        assert_eq!(decoded, event);
    }
}
