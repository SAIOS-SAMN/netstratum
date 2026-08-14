//! **STELLAR NUCLEOSYNTHESIS ON STAR DEATH** (task #27, ruled 2026-08-14).
//! A star aging into a REMNANT (a white dwarf and beyond) forges heavy
//! elements and scatters them — every sited orb gains an enrichment vein,
//! the veins the trace/catalyst economy then mines. A main-sequence move
//! forges nothing; a world whose star never dies is byte-identical.

use cfe::dop::{positive, unit, ClosureEvent};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

fn sited() -> World {
    let mut world = World::new();
    world.found("nova-hold").expect("founds");
    world.commission_star(StarClass::G).expect("a living sun");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites a planet");
    world
}

fn enrichments(world: &World) -> usize {
    world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::EnrichmentStruck { .. }))
        .count()
}

#[test]
fn the_enrichment_deed_round_trips() {
    let deed = ClosureEvent::EnrichmentStruck { orb_id: 5, stratum: 11, element: [7u8; 32], richness: ratio(1, 3) };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 176, "EnrichmentStruck is tag 176");
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
}

#[test]
fn a_main_sequence_star_forges_no_enrichment() {
    // A living star moving among main-sequence classes seeds nothing —
    // byte-identical to the pre-enrichment law.
    let mut world = sited();
    let veins_before = world.chronicle().receipts().iter().filter(|r| matches!(r.event, ClosureEvent::ProspectStruck { .. } | ClosureEvent::EnrichmentStruck { .. })).count();
    world.age_star(StarClass::K).expect("ages");
    assert_eq!(enrichments(&world), 0, "a main-sequence move forges no vein");
    let veins_after = world.chronicle().receipts().iter().filter(|r| matches!(r.event, ClosureEvent::ProspectStruck { .. } | ClosureEvent::EnrichmentStruck { .. })).count();
    assert_eq!(veins_before, veins_after, "the vein registry is untouched");
}

#[test]
fn a_dying_star_scatters_a_heavy_vein_onto_every_sited_orb() {
    // The star ages into a remnant — its death forges heavy elements and
    // seeds an enrichment vein on the sited orb. Replay-exact.
    let mut world = sited();
    world.age_star(StarClass::NeutronStar).expect("collapses");
    assert_eq!(enrichments(&world), 1, "the death seeded a vein on the sited orb");
    // The vein stands in the world's prospects, ready to mine.
    let struck: Vec<_> = world.chronicle().receipts().iter().filter_map(|r| match &r.event {
        ClosureEvent::EnrichmentStruck { orb_id, element, .. } => Some((*orb_id, *element)),
        _ => None,
    }).collect();
    assert_eq!(struck.len(), 1, "one enrichment for the one sited orb");
    assert_eq!(struck[0].0, FIRST_ORB, "on the sited orb");
    assert_eq!(replay(world.chronicle()), world.committed(), "the enrichment replays MATCHES");

    // Idempotent: dying into the SAME remnant class again seeds nothing new.
    world.age_star(StarClass::NeutronStar).expect("dies again");
    assert_eq!(enrichments(&world), 1, "a second death into the same class enriches once");
}
