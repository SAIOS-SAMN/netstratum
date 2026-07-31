//! NS-2 §2.5 / §3 vector extraction (the N2 precedent): the codec
//! vectors printed from the running engine's own types — inscribe the
//! output into the spec; any implementation verifies from the document
//! alone. Run: `cargo run -p cfe --example ns2_vectors`.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource, Zone, ZoneKind};
use cfe::query::{hit_record_bytes, orb_sigil, query_record_bytes, refusal_record_bytes, NearMiss};
use cfe::ratio::ratio;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn main() {
    // The witness board's planted pair (rqp_scanner.rs): the probe —
    // extractor@10 (1,2), habitat@21 (2,5), banked 10 iron + 5
    // biomass — and its rot90 twin at tiles 22, 45.
    let mut probe = ColonyOrb { orb_id: 2, name: "planted-a".into(), ..ColonyOrb::default() };
    probe.zones = vec![
        Zone { tile: 10, kind: ZoneKind::Extractor },
        Zone { tile: 21, kind: ZoneKind::Habitat },
    ];
    probe.banked = vec![
        (Resource::Iron, ratio(10, 1)),
        (Resource::Biomass, ratio(5, 1)),
    ];
    let mut twin = ColonyOrb { orb_id: 3, name: "planted-b".into(), ..ColonyOrb::default() };
    twin.zones = vec![
        Zone { tile: 22, kind: ZoneKind::Extractor },
        Zone { tile: 45, kind: ZoneKind::Habitat },
    ];
    twin.banked = probe.banked.clone();

    let sigil = orb_sigil(&probe);
    assert_eq!(sigil, orb_sigil(&twin), "the twins project ONE sigil");
    println!("-- the sigil (probe = rot90 twin, one canonical form) --");
    println!("s_canonical: {:?}", sigil.address.s_canonical);
    println!("g_resources: {:?}", sigil.address.g_resources);
    println!("thermal:     {}", sigil.address.thermal);
    println!("joint zones: {:?}", sigil.joint.zones);
    println!();
    println!("-- tag 56, the query record (the address only) --");
    println!("{}", hex(&query_record_bytes(&sigil.address)));
    println!();
    println!("-- tag 57, the hit record (orb 3, stamp 178) --");
    println!("{}", hex(&hit_record_bytes(3, 178, &ratio(3, 4))));
    println!();

    // The kind-swap near-miss (C(Δ) = 0 while the delta is real) and
    // the banked near-miss (iron −5, biomass −2, C(Δ) = |−5| + |−2| = 7,
    // the proportional law) — the
    // witness board's measured refusals, re-stated as records.
    let kind_swap = NearMiss {
        orb: 4,
        banked_residuals: vec![],
        zone_deltas: vec![(10, 1, 2), (21, 2, 1)],
        thermal: (0, 0),
        coherence: ratio(0, 1),
    };
    let banked_miss = NearMiss {
        orb: 5,
        banked_residuals: vec![
            (Resource::Iron, ratio(-5, 1)),
            (Resource::Biomass, ratio(-2, 1)),
        ],
        zone_deltas: vec![],
        thermal: (0, 0),
        coherence: ratio(7, 1),
    };
    println!("-- tag 58, the refusal record (kind swap: delta real, C(Δ) = 0) --");
    println!("{}", hex(&refusal_record_bytes(&kind_swap)));
    println!();
    println!("-- tag 58, the refusal record (banked miss: iron −5, biomass −2, C(Δ) = 7) --");
    println!("{}", hex(&refusal_record_bytes(&banked_miss)));
    println!();

    // Tag 55 — the one chronicle event: the draw, blind to the asking.
    let draw = ClosureEvent::CrystalDrawn { orb_id: 1, amount: ratio(1, 1) };
    println!("-- tag 55, ClosureEvent::CrystalDrawn (orb 1, 1 crystal) --");
    println!("{}", hex(&draw.wire_bytes()));
    let bytes = draw.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), draw, "round-trips");
    println!("(round-trips byte-exact through from_wire)");
}
