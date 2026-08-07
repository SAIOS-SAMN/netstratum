//! THE VERTICAL AXIS — the seat enters the chain.
//!
//! `KERNEL_RELAY` §1.5 named the hole exactly: *who exists, what rank
//! they hold … lives in `var/registry` JSON, files anyone can edit. The
//! game is constitutional; the universe is not.* A station GATES
//! AUTHORITY — the tier-0 verbs check it — so an editable file decided
//! who could act. These pins measure it moving into the chain.

use cfe::dop::{ClosureEvent, ZoneKind};
use cfe::world::World;
use kernel::{admits, Step, Refused};

fn seat(line: &str, station: u8, actor: u64) -> Step {
    Step {
        event: ClosureEvent::StationSeated { line: line.into(), station },
        actor,
        stamp: 1,
    }
}

#[test]
fn the_seat_replays_from_the_receipt_not_from_a_file() {
    // The registry becomes a CACHE from this receipt on — the G1
    // precedent, where GenesisSealed did exactly this for the axes.
    let mut world = World::new();
    world.found("seat-hold").expect("founds");
    world.seat_station_as(cfe::world::SOVEREIGN, cfe::dop::name("abyssal-vocalis"), cfe::dop::Station::from_wire(1).expect("a band office")).expect("seats");

    assert_eq!(world.station_of("abyssal-vocalis"), Some(1), "the seat is held");
    let committed = world.committed();
    assert_eq!(committed.stations, vec![("abyssal-vocalis".to_string(), 1)]);

    // And it survives the boundary: replay alone reconstructs it.
    let replayed = cfe::world::replay(world.chronicle());
    assert_eq!(replayed.stations, committed.stations, "the chain carries the office");
}

#[test]
fn re_seating_replaces_and_the_chain_keeps_the_history() {
    // One seat per line. The COMMITTED reading is the current office;
    // the chain still holds every seating that ever happened, so a
    // promotion is auditable rather than an overwrite.
    let mut world = World::new();
    world.found("climb").expect("founds");
    world.seat_station_as(cfe::world::SOVEREIGN, cfe::dop::name("brine-cortex"), cfe::dop::Station::from_wire(3).expect("a band office")).expect("observer");
    world.seat_station_as(cfe::world::SOVEREIGN, cfe::dop::name("brine-cortex"), cfe::dop::Station::from_wire(2).expect("a band office")).expect("watcher");

    assert_eq!(world.station_of("brine-cortex"), Some(2), "the current office");
    let seatings = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::StationSeated { .. }))
        .count();
    assert_eq!(seatings, 2, "both seatings stand in the record — nothing overwritten");
}

#[test]
fn an_office_outside_the_vocabulary_refuses() {
    // Closed office vocabulary is 1..=10 (BAND-1 + Wave-1 T4–T9).
    // 0 is not one, and neither is 11 — and since C1's Station type,
    // an off-band office is UNREPRESENTABLE: the wire door refuses it
    // before any verb could. Authority from nowhere is no longer a
    // runtime case.
    assert_eq!(cfe::dop::Station::from_wire(0), None, "0 is not an office");
    assert_eq!(cfe::dop::Station::from_wire(11), None, "11 is not an office");
    assert_eq!(
        cfe::dop::Station::from_wire(5),
        Some(cfe::dop::Station::Caravan),
        "5 is caravan (T4) after Wave-1"
    );
    let mut world = World::new();
    world.found("bad-office").expect("founds");
    assert!(cfe::dop::Name::new("").is_none(), "no nameless line — the case cannot be constructed");
    assert!(world.committed().stations.is_empty(), "and nothing was seated");
}

#[test]
fn the_wire_refuses_a_station_it_cannot_name() {
    // Refuse never guess: the decoder rejects an out-of-vocabulary
    // office rather than admitting an office with no name.
    let lawful_seat = ClosureEvent::StationSeated { line: "vitreous-hex".into(), station: 4 };
    // TLV framing is tag ‖ LE32 length ‖ value, so the value starts at 5.
    let bytes = lawful_seat.wire_bytes();
    let value = &bytes[5..];
    assert_eq!(bytes[0], 89, "the deed carries its claimed tag");
    let back = ClosureEvent::from_wire(89, value);
    assert_eq!(back, Some(lawful_seat), "tag 89 round-trips whole");

    let mut forged = value.to_vec();
    forged[0] = 11; // an office that does not exist (1..=10 are lawful)
    assert_eq!(ClosureEvent::from_wire(89, &forged), None, "the wire refuses it");
}

#[test]
fn the_kernel_judges_who_may_seat() {
    // THE POINT OF THE WHOLE LANDING. A station gates authority, so the
    // law that matters is WHO may hand one out — and that law did not
    // exist while the seat lived in a file. It is I2 containment, and
    // the deed is Build-scoped.
    let mut world = World::new();
    world.found("gate").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    let prior = world.committed();

    let mut next = prior.clone();
    next.stations.push(("mantle-anhydro".to_string(), 1));

    // The sovereign holds every scope by construction.
    assert_eq!(
        admits(&prior, &next, &seat("mantle-anhydro", 1, cfe::world::SOVEREIGN)),
        Ok(()),
        "the sovereign may seat"
    );

    // A stranger is not an entity in prior at all — identity precedes
    // the world, so the deed refuses before any state is read.
    assert_eq!(
        admits(&prior, &next, &seat("mantle-anhydro", 1, 4242)),
        Err(Refused::NoSuchEntity { actor: 4242 }),
        "a deed from no one seats no one"
    );
}

#[test]
fn the_seat_moves_no_value() {
    // Banked-neutral: the office changes who may ACT, never what anyone
    // HOLDS. Conservation is vacuous here and that is the correct
    // reading, not an omission.
    let mut world = World::new();
    world.found("neutral").expect("founds");
    let before = world.committed();
    world.seat_station_as(cfe::world::SOVEREIGN, cfe::dop::name("line"), cfe::dop::Station::from_wire(2).expect("a band office")).expect("seats");
    let after = world.committed();

    assert_eq!(before.orbs[0].banked, after.orbs[0].banked, "no bank moved");
    assert_eq!(before.gauge, after.gauge, "no key moved");
}
